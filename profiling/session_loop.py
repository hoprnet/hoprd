#!/usr/bin/env python3
"""
Hammer a running hoprd cluster with sessions.

Loop N times:
  1. Pick a random entry node and a random destination peer.
  2. POST /api/v4/session/{protocol} to open a session listener.
  3. Connect to the returned (ip, port), write random bytes, close socket.
  4. DELETE /api/v4/session/{protocol}/{ip}/{port} to close the session.
  5. Record timing + outcome.

Run from macOS with port-forwards (3000/3001/3002 → VM), or from the VM.

  ssh -N -L 3000:localhost:3000 -L 3001:localhost:3001 -L 3002:localhost:3002 nixos-test@orb &
  ./scripts/session_loop.py --iterations 1000

Or directly on the VM:
  ssh nixos-test@orb 'cd ~/hoprd && ./scripts/session_loop.py'

Defaults assume 3-node cluster from `scripts/jeprof-vm.sh localcluster 3`.
"""
from __future__ import annotations

import argparse
import json
import os
import random
import socket
import sys
import time
import urllib.error
import urllib.request
from concurrent.futures import ThreadPoolExecutor, as_completed
from dataclasses import dataclass, field

DEFAULT_NODES = ["http://[::1]:3000", "http://[::1]:3001", "http://[::1]:3002"]
PROTOCOLS = ["tcp", "udp"]
HOPS_CHOICES = [0, 1, 2]
PAYLOAD_MIN = 32
PAYLOAD_MAX = 4096


@dataclass
class Stats:
    ok: int = 0
    fail_open: int = 0
    fail_send: int = 0
    fail_close: int = 0
    bytes_sent: int = 0
    durations_ms: list[float] = field(default_factory=list)

    def merge(self, other: "Stats") -> None:
        self.ok += other.ok
        self.fail_open += other.fail_open
        self.fail_send += other.fail_send
        self.fail_close += other.fail_close
        self.bytes_sent += other.bytes_sent
        self.durations_ms.extend(other.durations_ms)


class HttpError(Exception):
    pass


def _request(method: str, url: str, *, body: dict | None = None,
             token: str | None = None, timeout: float = 10.0) -> tuple[int, bytes]:
    data = json.dumps(body).encode() if body is not None else None
    req = urllib.request.Request(url, data=data, method=method)
    if data is not None:
        req.add_header("content-type", "application/json")
    if token:
        req.add_header("x-auth-token", token)
    try:
        with urllib.request.urlopen(req, timeout=timeout) as resp:
            return resp.status, resp.read()
    except urllib.error.HTTPError as e:
        return e.code, e.read() if e.fp else b""
    except urllib.error.URLError as e:
        raise HttpError(f"{method} {url}: {e.reason}") from e


def discover_addresses(nodes: list[str], token: str | None) -> list[str]:
    addrs: list[str] = []
    for url in nodes:
        code, body = _request("GET", f"{url}/api/v4/account/addresses", token=token)
        if code != 200:
            raise HttpError(f"GET {url}/api/v4/account/addresses: HTTP {code}: {body[:200]!r}")
        addrs.append(json.loads(body)["native"])
    return addrs


def open_session(entry: str, protocol: str, destination: str, hops: int,
                 token: str | None, timeout: float) -> tuple[str, int]:
    body: dict = {
        "destination": destination,
        "forwardPath": {"Hops": hops},
        "returnPath":  {"Hops": hops},
        "target":      {"Service": 0},
    }
    if protocol == "tcp":
        body["capabilities"] = ["Segmentation", "Retransmission"]
    code, payload = _request("POST", f"{entry}/api/v4/session/{protocol}",
                             body=body, token=token, timeout=timeout)
    if code != 200:
        raise HttpError(f"POST session: HTTP {code}: {payload[:200]!r}")
    j = json.loads(payload)
    return j["ip"], int(j["port"])


def close_session(entry: str, protocol: str, ip: str, port: int,
                  token: str | None, timeout: float) -> None:
    code, payload = _request("DELETE", f"{entry}/api/v4/session/{protocol}/{ip}/{port}",
                             token=token, timeout=timeout)
    if code not in (204, 404):
        raise HttpError(f"DELETE session: HTTP {code}: {payload[:200]!r}")


def push_traffic(ip: str, port: int, protocol: str, payload: bytes,
                 connect_timeout: float = 3.0, send_timeout: float = 3.0) -> int:
    """Open socket, write payload, close. Returns bytes written."""
    sock_type = socket.SOCK_STREAM if protocol == "tcp" else socket.SOCK_DGRAM
    sent = 0
    if protocol == "tcp":
        with socket.create_connection((ip, port), timeout=connect_timeout) as s:
            s.settimeout(send_timeout)
            s.sendall(payload)
            sent = len(payload)
            try:
                s.shutdown(socket.SHUT_WR)
                # Drain any echo so the peer doesn't see RST.
                s.settimeout(0.2)
                while s.recv(65536):
                    pass
            except (TimeoutError, OSError):
                pass
    else:  # udp
        with socket.socket(socket.AF_INET, sock_type) as s:
            s.settimeout(send_timeout)
            # UDP MTU concern; keep payload bounded by caller (PAYLOAD_MAX < 1500 typically).
            s.sendto(payload, (ip, port))
            sent = len(payload)
    return sent


def one_iteration(entry: str, dest: str, protocol: str, hops: int,
                  payload_size: int, token: str | None,
                  http_timeout: float = 10.0,
                  sleep_s: float = 0.0) -> tuple[Stats, str | None]:
    s = Stats()
    t0 = time.monotonic()
    try:
        ip, port = open_session(entry, protocol, dest, hops, token, http_timeout)
    except Exception as e:
        s.fail_open += 1
        s.durations_ms.append((time.monotonic() - t0) * 1000.0)
        return s, f"open: {e!r}"
    if sleep_s:
        time.sleep(sleep_s)
    try:
        payload = os.urandom(payload_size)
        try:
            s.bytes_sent = push_traffic(ip, port, protocol, payload)
        except Exception as e:
            s.fail_send += 1
            return s, f"send: {e!r}"
        if sleep_s:
            time.sleep(sleep_s)
    finally:
        try:
            close_session(entry, protocol, ip, port, token, http_timeout)
        except Exception as e:
            s.fail_close += 1
            s.durations_ms.append((time.monotonic() - t0) * 1000.0)
            return s, f"close: {e!r}"
        if sleep_s:
            time.sleep(sleep_s)
    s.ok += 1
    s.durations_ms.append((time.monotonic() - t0) * 1000.0)
    return s, None


def summary(s: Stats, total: int, wall_s: float) -> str:
    if not s.durations_ms:
        return "no iterations completed"
    d = sorted(s.durations_ms)
    p = lambda q: d[min(len(d) - 1, int(q * len(d)))]
    return (
        f"iterations={total} ok={s.ok} fail_open={s.fail_open} "
        f"fail_send={s.fail_send} fail_close={s.fail_close}\n"
        f"bytes_sent={s.bytes_sent} wall={wall_s:.1f}s "
        f"throughput={s.ok/wall_s:.1f} sess/s\n"
        f"latency_ms p50={p(0.50):.1f} p90={p(0.90):.1f} "
        f"p99={p(0.99):.1f} max={d[-1]:.1f}"
    )


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--nodes", nargs="+", default=DEFAULT_NODES,
                    help="Cluster API base URLs (default: %(default)s)")
    ap.add_argument("-n", "--iterations", type=int, default=1000)
    ap.add_argument("-c", "--concurrency", type=int, default=4,
                    help="Parallel workers (each runs sequential iterations)")
    ap.add_argument("--protocols", nargs="+", choices=PROTOCOLS, default=PROTOCOLS)
    ap.add_argument("--hops", nargs="+", type=int, choices=HOPS_CHOICES, default=HOPS_CHOICES)
    ap.add_argument("--payload-min", type=int, default=PAYLOAD_MIN)
    ap.add_argument("--payload-max", type=int, default=PAYLOAD_MAX)
    ap.add_argument("--token", default=os.environ.get("HOPRD_API_TOKEN"))
    ap.add_argument("--seed", type=int)
    ap.add_argument("--progress-every", type=int, default=50)
    ap.add_argument("--sleep-ms", type=int, default=100,
                    help="Sleep between each HTTP call (open/send/close). Default 100ms.")
    args = ap.parse_args()

    if args.seed is not None:
        random.seed(args.seed)

    print(f"discovering addresses for {len(args.nodes)} nodes...", file=sys.stderr)
    addrs = discover_addresses(args.nodes, args.token)
    nodes_with_addrs = list(zip(args.nodes, addrs, strict=True))
    print(f"  {json.dumps(dict(nodes_with_addrs), indent=2)}", file=sys.stderr)
    if len(nodes_with_addrs) < 2:
        print("ERROR: need at least 2 nodes for source/destination pairs", file=sys.stderr)
        return 2

    sleep_s = args.sleep_ms / 1000.0

    def gen_one(_i: int) -> tuple[Stats, str | None]:
        entry_url, _ = random.choice(nodes_with_addrs)
        dest_choice = random.choice([(u, a) for (u, a) in nodes_with_addrs if u != entry_url])
        _, dest_addr = dest_choice
        protocol = random.choice(args.protocols)
        hops = random.choice(args.hops)
        size = random.randint(args.payload_min, args.payload_max)
        return one_iteration(entry_url, dest_addr, protocol, hops, size, args.token,
                             sleep_s=sleep_s)

    agg = Stats()
    errors_seen: dict[str, int] = {}
    t_start = time.monotonic()
    completed = 0
    with ThreadPoolExecutor(max_workers=args.concurrency) as ex:
        futures = [ex.submit(gen_one, i) for i in range(args.iterations)]
        for fut in as_completed(futures):
            s, err = fut.result()
            agg.merge(s)
            if err:
                # bucket errors by short prefix
                key = err.split(":", 1)[0] + ": " + err.split(":", 1)[1].strip().split("\n")[0][:120]
                errors_seen[key] = errors_seen.get(key, 0) + 1
            completed += 1
            if completed % args.progress_every == 0 or completed == args.iterations:
                wall = time.monotonic() - t_start
                print(f"[{completed}/{args.iterations}] {summary(agg, completed, wall)}",
                      file=sys.stderr)

    wall = time.monotonic() - t_start
    print("=" * 60)
    print(summary(agg, args.iterations, wall))
    if errors_seen:
        print("--- top errors ---")
        for k, v in sorted(errors_seen.items(), key=lambda kv: -kv[1])[:10]:
            print(f"  {v:5d}  {k}")
    return 0 if agg.ok > 0 and agg.fail_open == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
