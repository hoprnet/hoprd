# Investigation: artificial latency between localcluster nodes

This document records the investigation behind localcluster's `--latency` feature: the
problem, the constraints, the options considered with their trade-offs, and why the
userspace UDP relay was chosen.

## Problem

We want to inject artificial / random latency on the P2P traffic between localcluster
nodes to simulate realistic or degraded network conditions for testing.

### Constraints

1. **No `hoprd` change.** The daemon under test must stay untouched; shaping lives in the
   test harness.
2. **Cross-platform.** Must work on both Linux and macOS (developer laptops + CI).
3. **Configurable granularity.** Global, per-node, and per-link delays.
4. **Random latency.** Jitter / distributions, not just a fixed offset.

## Established facts

These shaped the option analysis (all verified in-tree):

- Nodes are native `hoprd` processes talking **QUIC over UDP**. Default
  `p2p_host = 127.0.0.1`; node `i` listens on `p2p_port_base + i` (default `9000 + i`).
  Same-host traffic is therefore **loopback**.
- hoprd uses stock **libp2p-quic** (`.with_quic()` in
  `impls/transport/p2p/src/swarm.rs`), which uses one quinn endpoint per listen socket.
  Outbound dials reuse that socket, so a node's **source UDP port equals its listen
  port** (fixed, not ephemeral). This lets a middlebox identify the sending node by
  source port — the key enabler for per-link shaping.
- localcluster already **pre-announces** each node's multiaddr on chain
  (`localcluster/src/identity.rs`) so blokli indexes it during catch-up. hoprd's own
  announce (`hopr.announce=true`) just re-announces the same address.
- **`hopr.announce=false` keeps a node fully functional and reachable** as long as its
  address is announced on chain by another party (verified in
  `hopr/hopr-lib/src/builder.rs` and the discovery behavior). Safe-registration and
  key-binding happen regardless of the flag.
- A peer dials **all** announced multiaddrs for a target, and any packet destined to node
  `X` must arrive at `X`'s announced address — so a middlebox on `X`'s announced port sees
  **every** directed link `Y → X`.

## Options considered

### 1. `tc` / netem (Linux)

Linux kernel traffic control with the `netem` qdisc.

- **+** Native delay + jitter + distribution + loss + reorder in one command.
- **+** Transparent — no config change; shapes the real ports in place.
- **+** Per-link achievable via network namespaces / veth per node.
- **−** **Linux only.** Fails constraint 2.
- **−** Needs `sudo`; loopback shaping needs filters or per-node netns.

### 2. dummynet + pf (macOS)

macOS/BSD traffic shaping: `dnctl` builds delay "pipes", `pfctl` steers matched packets
into them. (This is what Apple's Network Link Conditioner uses internally.)

- **+** Transparent — no config change. Per-link works because source ports are fixed
  (match `(src port, dst port)`).
- **−** **macOS only.** Fails constraint 2.
- **−** Needs `sudo`.
- **−** Default `/etc/pf.conf` has `set skip on lo0`, so **loopback bypasses pf** — must
  load a ruleset that processes `lo0`; known-finicky.
- **−** **No native random jitter** — `delay` is fixed; jitter needs hacks (multiple pipes
  selected by `prob`, or periodically reconfiguring the pipe).

### 3. ns-3 emulation

The ns-3 discrete-event network simulator in real-time emulation mode (`TapBridge` /
`FdNetDevice`): real packets enter the simulator, traverse simulated channels with rich
delay/loss/queue models, and exit.

- **+** Most realistic — topology, queueing, congestion, correlated loss.
- **+** Transparent to hoprd.
- **−** **Linux only** (tap devices, netns, raw sockets). Fails constraint 2.
- **−** Needs `sudo`; one tap + netns per node; real-time scheduler must keep pace.
- **−** Heavyweight: build ns-3, author a C++/Python topology script. Massive overkill for
  "add latency" — sits on the same Linux tap/netns plumbing as netem but adds a simulator.

### 4. TCP fault-injection proxies (toxiproxy, comcast, …)

- **−** **Ruled out:** these are TCP-only. HOPR P2P is QUIC over **UDP**.

### 5. Userspace UDP relay (chosen)

A small Rust/tokio UDP relay per node, in front of its real listen port. The relay's port
is announced on chain instead of the node's real port, and the node's self-announce is
disabled (`announce=false`), so peers dial the relay. The relay forwards datagrams to the
node after a sampled per-link delay and relays replies back. The sending node is
identified by source UDP port (fixed = its listen port), giving per-link control.

- **+** **Cross-platform** — pure Rust/tokio, identical on Linux and macOS. Meets the hard
  constraint.
- **+** No `sudo`; no kernel/loopback fragility.
- **+** Native random jitter / distributions in code; full global / per-node / per-link
  granularity from one resolution rule.
- **+** No `hoprd` change.
- **−** One functional change in localcluster's **generated test config**:
  `announce=false` + announce the relay port. Confined to the local Anvil chain;
  production hoprd is untouched.
- **−** ~300–400 LOC of harness code. Delay is modelled physically (release =
  `arrival + sampled_delay`, delivered in release-time order): fixed delay preserves order,
  jitter reorders — realistic, though jitter stresses HOPR session reassembly.

## Comparison

| Option              | Cross-platform       | Transparent (no cfg change) | Random jitter | sudo | Effort  |
| ------------------- | -------------------- | --------------------------- | ------------- | ---- | ------- |
| tc / netem (Linux)  | ❌ Linux only        | ✅                          | ✅ native     | yes  | low     |
| dummynet+pf (macOS) | ❌ macOS only        | ✅                          | ⚠️ hacky      | yes  | low–med |
| ns-3 emulation      | ❌ Linux only        | ✅                          | ✅ + models   | yes  | high    |
| TCP proxies         | n/a — UDP, ruled out |                             |               |      |
| **UDP relay**       | ✅                   | ❌ (`announce=false`)       | ✅ native     | no   | medium  |

## Decision

The cross-platform mandate (constraint 2) eliminates every OS-level shaper — each is
single-OS, and supporting both would mean maintaining two fragile backends. The
**userspace UDP relay** is the only single-codebase solution and additionally avoids
`sudo` and gives native jitter and clean per-link control. Its one cost — `announce=false`
in the generated test config — is confined to the local test chain and does not touch
production `hoprd`.

## Implementation

- `localcluster/src/latency.rs` — `DelayDist` (Fixed / Uniform / Normal) + `LatencyConfig`
  with `resolve(src, dst)` (precedence `per_link` → `per_node` → `default`); spec + YAML
  parsers.
- `localcluster/src/relay.rs` — per-node UDP relay: binds the announced relay port,
  maintains one upstream socket per peer, applies a sampled delay per datagram in both
  directions.
- `localcluster/src/identity.rs` — when latency is enabled, pre-announce the relay port and
  set `announce=false` in the generated config.
- `localcluster/src/main.rs` — spawn one relay per node before nodes start; abort on
  shutdown.
- CLI: `--latency`, `--latency-config`, `--latency-port-base` (see README).
