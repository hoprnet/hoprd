# hoprd local cluster

A local HOPR cluster spins up N `hoprd` nodes on your workstation, backed by an in-memory Ethereum chain (Anvil) and a HOPR indexer (Blokli), and opens payment channels between every pair of nodes. It is the fastest way to develop and test against a realistic multi-node HOPR topology without connecting to a live network.

This is **not** production-equivalent — the chain state is ephemeral and the node configuration is simplified.

---

## Prerequisites

| Requirement         | Notes                                                                    |
| ------------------- | ------------------------------------------------------------------------ |
| Nix (with flakes)   | `experimental-features = nix-command flakes` in `~/.config/nix/nix.conf` |
| A container runtime | See options below                                                        |

### Container runtime options

The `hoprd-localcluster` orchestrator delegates to any Docker-compatible container CLI. Pick the one installed on your machine:

| Runtime                                     | Flag / env                                                             | Notes                                                                                                                               |
| ------------------------------------------- | ---------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| **Docker** (Docker Desktop, OrbStack, etc.) | default — no flag needed                                               | Standard path on Linux and most macOS setups                                                                                        |
| **Apple `container`** (macOS native)        | `--container-runtime container` or `HOPRD_CONTAINER_RUNTIME=container` | Run `container system start` once per boot. The orchestrator already passes `--platform linux/amd64` for the amd64-only chain image |
| **Podman**                                  | `--container-runtime podman` or `HOPRD_CONTAINER_RUNTIME=podman`       | Must have a running Podman machine                                                                                                  |

Any runtime that accepts `run --rm --name <n> --platform linux/amd64 -p 8080:8080 <image>` and `rm -f <n>` will work.

---

## Build

```bash
# Nix (preferred — reproducible, no toolchain setup needed)
nix build -L --out-link result-hoprd .#binary-hoprd
nix build -L --out-link result-localcluster .#binary-hoprd-localcluster
# binaries: ./result-hoprd/bin/hoprd  and  ./result-localcluster/bin/hoprd-localcluster

# Cargo (inside the dev shell)
nix develop -c cargo build -p hoprd -p hoprd-localcluster
# binaries: target/debug/hoprd  and  target/debug/hoprd-localcluster
```

---

## Run

### Default (Docker)

```bash
rm -rf /tmp/hopr-nodes   # clear any stale state

CHAIN_IMAGE=europe-west3-docker.pkg.dev/hoprassociation/docker-images/bloklid-anvil:latest

RUST_LOG=info \
./result-localcluster/bin/hoprd-localcluster \
  --hoprd-bin ./result-hoprd/bin/hoprd \
  --chain-image "$CHAIN_IMAGE" \
  --size 3
```

### Apple `container` (macOS)

```bash
container system start   # once per boot

rm -rf /tmp/hopr-nodes

CHAIN_IMAGE=europe-west3-docker.pkg.dev/hoprassociation/docker-images/bloklid-anvil:latest

RUST_LOG=info \
HOPRD_CONTAINER_RUNTIME=container \
./result-localcluster/bin/hoprd-localcluster \
  --hoprd-bin ./result-hoprd/bin/hoprd \
  --chain-image "$CHAIN_IMAGE" \
  --size 3
```

> **macOS NAT note**: the orchestrator detects the container's direct IP (e.g. `192.168.64.x`) and uses it instead of `localhost:8080`. This avoids macOS port-forwarding NAT, which drops long-lived SSE connections used by the chain indexer client.

### External chain (skip the container)

If you already have Blokli running at a known URL, pass it directly and the container step is skipped entirely:

```bash
HOPRD_CHAIN_URL=http://localhost:8080 \
./result-localcluster/bin/hoprd-localcluster \
  --hoprd-bin ./result-hoprd/bin/hoprd \
  --size 3
```

Press **Ctrl-C** to stop — the orchestrator kills all `hoprd` processes and removes the chain container on exit.

### Docker Compose

`localcluster/docker-compose.yml` bundles the chain and the orchestrator into a single `docker compose up`. The `hoprd-localcluster` image must be built first (it is not published; it bundles both `hoprd` and `hoprd-localcluster`):

```bash
# Build and load the image (x86_64 Linux only)
docker load < $(nix build -L .#docker-hoprd-localcluster-x86_64-linux --print-out-paths)

# Start the cluster (default: 3 nodes)
docker compose -f localcluster/docker-compose.yml up -d

# Override the node count
CLUSTER_SIZE=5 docker compose -f localcluster/docker-compose.yml up -d
```

API ports `3001–3005` are mapped to the host; node `i` listens on port `3001 + i - 1`. Stop with `docker compose -f localcluster/docker-compose.yml down`.

---

## Verify

Once the cluster is running (look for `localcluster running; press Ctrl+C to stop` in the log), check readiness from a separate terminal:

```bash
for port in 3000 3001 3002; do
  printf "node @%d: " "$port"
  curl -s -o /dev/null -w "%{http_code}\n" "http://localhost:${port}/readyz"
done
```

All three should print `200`. The endpoints (defined in `rest-api/src/checks.rs`) return:

| Endpoint         | 200 when                                        |
| ---------------- | ----------------------------------------------- |
| `GET /startedz`  | Node process is `Running`                       |
| `GET /readyz`    | Running + minimally connected + chain reachable |
| `GET /healthyz`  | Running + minimally connected (no chain check)  |
| `GET /eligiblez` | Always                                          |

---

## Machine-readable status

For CI and integration tooling, query the structured status instead of scraping stdout. A running cluster serves its **live** state on a unix domain socket at `<data-dir>/cluster.sock`. The `status` subcommand connects to it and prints the current snapshot as JSON:

```bash
./result-localcluster/bin/hoprd-localcluster status                       # reads <data-dir>/cluster.sock
./result-localcluster/bin/hoprd-localcluster status --data-dir /tmp/hopr-nodes
./result-localcluster/bin/hoprd-localcluster status --control-base /var/run/hopr/cluster   # reads <base>.sock
```

`status` always exits `0` with a parseable answer:

- a live snapshot while the cluster is `initializing` / `starting` / `running` / `shutting_down`,
- `{"state": "not_running"}` when nothing is listening (no cluster, or it already exited).

The status is updated **as the cluster comes up**, so tooling can poll `status` and key off the structured `state` fields instead of grepping logs. The deterministic stop criterion is `.state == "running"`. Each node also carries its own `state`, advancing `pending → spawned → started → ready → channels_open` as it becomes available.

Example (`jq`-friendly) wait loop:

```bash
until [ "$(./result-localcluster/bin/hoprd-localcluster status | jq -r .state)" = "running" ]; do sleep 1; done
```

Shape:

```json
{
  "state": "running",
  "pid": 12000,
  "blokli_url": "http://127.0.0.1:8080",
  "nodes": [
    {
      "id": 0,
      "state": "channels_open",
      "address": "0x1234…",
      "api_url": "http://127.0.0.1:3000",
      "api_token": null,
      "p2p": "127.0.0.1:9000",
      "latency": null,
      "node_admin_url": "http://localhost:4677/node/info?apiEndpoint=http://127.0.0.1:3000",
      "pid": 12345
    }
  ],
  "extras": [
    {
      "id": 0,
      "address": "0xabcd…",
      "safe_address": "0x…",
      "module_address": "0x…",
      "keystore_path": "/tmp/hopr-nodes/extra_id_0.id",
      "password": "local-cluster"
    }
  ]
}
```

`api_token` is `null` unless `--api-token` is set; `address`/`pid` are `null` until a node reaches that point; `extras` is empty unless `--extra-identities` is greater than 0. If startup fails, `state` becomes `failed` and an `error` field describes why.

> Note: by default the socket and lock live at `<data-dir>/cluster.{sock,lock}`. Across a Docker bind mount (gRPC-FUSE/virtiofs on macOS) or NFS they are unreliable — point `--control-base` at a local path (e.g. `/var/run/hopr/cluster`) and pass the same value to `status`.

---

## Single-instance lock

A running cluster holds an exclusive advisory lock on `<control-base>.lock` (default `<data-dir>/cluster.lock`) for its whole lifetime. Starting a second cluster against the same control base fails fast with the offending pid:

```text
another localcluster instance (pid 12000) is already using control base /tmp/hopr-nodes/cluster; stop it (e.g. `kill 12000`) or pass a different --control-base
```

For tooling, the same rejection is emitted on **stdout** as JSON (the human message above goes to stderr):

```json
{
  "error": "lock_held",
  "control_base": "/tmp/hopr-nodes/cluster",
  "holder_pid": 12000
}
```

The lock is released automatically by the OS when the owner exits — including on a crash or `kill -9` — so a stale lock never wedges a future run.

> Note: the guarantee is keyed on the **control base**, not the data directory. By default they map one-to-one (`<data-dir>/cluster`). If you override `--control-base` while reusing the same `--data-dir`, a second cluster _can_ start against that data directory and clobber it — keep `--control-base` paired one-to-one with `--data-dir`.

## Configuration reference

Flags take precedence over env vars. Only the flags marked with an env var below support one.

| Flag                   | Env var                   | Default              | Description                                                                                                               |
| ---------------------- | ------------------------- | -------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| `--size`               | —                         | `3`                  | Number of nodes to start (1–5)                                                                                            |
| `--api-host`           | —                         | `localhost`          | Host to bind the REST API on                                                                                              |
| `--api-port-base`      | —                         | `3000`               | First API port (each node gets base + id)                                                                                 |
| `--p2p-host`           | —                         | `localhost`          | Host to bind P2P on (use an IP address for local clusters; hostname-based multiaddrs require DNS resolution at dial time) |
| `--p2p-port-base`      | —                         | `9000`               | First P2P port                                                                                                            |
| `--data-dir`           | —                         | `/tmp/hopr-nodes`    | Root for configs, identities, DBs, logs                                                                                   |
| `--control-base`       | —                         | `<data-dir>/cluster` | Path prefix for the lock (`<base>.lock`) and status socket (`<base>.sock`)                                                |
| `--chain-image`        | `HOPRD_CHAIN_IMAGE`       | —                    | Container image for Blokli + Anvil                                                                                        |
| `--chain-url`          | `HOPRD_CHAIN_URL`         | —                    | External Blokli URL; skips the container step                                                                             |
| `--container-runtime`  | `HOPRD_CONTAINER_RUNTIME` | `docker`             | Container CLI (`docker`, `container`, `podman`, …)                                                                        |
| `--hoprd-bin`          | —                         | `hoprd`              | Path to the `hoprd` binary                                                                                                |
| `--identity-password`  | —                         | `password`           | Password for identity encryption                                                                                          |
| `--api-token`          | —                         | none                 | Bearer token for the REST API                                                                                             |
| `--funding-amount`     | —                         | `1 wxHOPR`           | Per-channel funding amount                                                                                                |
| `--channel-management` | —                         | `api`                | Channel management mode: `api` (manual REST open), `strategy` (channel strategy only), `both`, or `none`                  |
| `--extra-identities`   | —                         | `0`                  | Extra pre-funded identities for external tooling (0–5)                                                                    |
| `--latency`            | —                         | none                 | Global artificial latency on inter-node traffic (e.g. `100ms`, `100ms±30ms`, `uniform:50ms,150ms`, `normal:100ms,30ms`)   |
| `--latency-config`     | —                         | none                 | Path to a YAML file with per-node / per-link latency overrides (enables relays even without `--latency`)                  |
| `--latency-port-base`  | —                         | `9100`               | First latency-relay port (node `i`'s relay listens on base + id)                                                          |

### Artificial latency

`--latency` / `--latency-config` inject artificial delay on the P2P traffic between
nodes, cross-platform (Linux + macOS) and without modifying `hoprd`.

When enabled, each node `X` runs a small userspace **UDP relay**: the relay's port is
announced on chain instead of the node's real listen port (and the node's own
self-announce is disabled), so peers dial the relay, which forwards datagrams to the
node after a sampled delay. Granularity:

- **Global** — one delay for all links: `--latency 150ms±50ms`.
- **Per-node / per-link** — a YAML file via `--latency-config`. Resolution order is
  `per_link` → `per_node` (keyed by destination) → `default`:

  ```yaml
  default: "100ms±30ms" # all links unless overridden
  per_node:
    2: "300ms" # any link whose destination is node 2
  per_link:
    - { from: 0, to: 1, delay: "500ms" } # directed link 0 → 1
    - { from: 1, to: 0, delay: "10ms" }
  ```

Delay spec forms: `100ms` (fixed), `100ms±30ms` / `100ms+-30ms` (uniform
`[mean-jitter, mean+jitter]`), `uniform:min,max`, `normal:mean,stddev`. Durations accept
`us`/`µs`, `ms` (default), `s`.

Caveats:

- Delay is modelled physically: each packet is released at `arrival + sampled_delay`. A
  **fixed** delay preserves packet order (like a real fixed-latency link); **jitter** lets
  packets overtake one another, so they **reorder** — exactly as on the real internet.
  Reordering stresses the HOPR session layer (segment reassembly), so heavily jittered
  links will see slower / failing session establishment. That is realistic behaviour, not
  a relay defect.
- Delay is applied per hop; a multi-hop HOPR path accumulates delay at each relayed node.
- Latency mode flips `announce=false` and announces the relay port — only meaningful for
  the local Anvil chain. Disabled by default, so normal runs are unaffected.
- When latency is enabled, the `status` JSON `p2p` field reports the **relay** port (the
  address peers dial, `latency_port_base + id`), not the node's real listen port, and each
  node's `latency` field describes the delay applied to its inbound traffic (a single value,
  or a per-source breakdown when links differ). It is `null` when latency is disabled.

### Channel management modes

`--channel-management` controls how payment channels are opened during cluster startup:

- `api` (default): Localcluster opens channels explicitly via REST API calls (`POST /api/v4/channels`) and waits for full-mesh channels to become open.
- `strategy`: Localcluster enables the node channel strategy in generated `hoprd` configs, does not make manual REST `open_channel` calls, and waits for full-mesh channels to become open.
- `both`: Localcluster enables strategy and also performs manual REST channel opening, then waits for full-mesh channels.
- `none`: Localcluster disables both strategy-driven and manual startup channel opening, and skips channel topology waiting.

Use `api` for deterministic startup behavior, `strategy` for strategy-only testing, `both` for mixed behavior checks, and `none` when you want to manage channels manually after startup.

---

## Cluster data layout

Everything lands under `--data-dir` (default `/tmp/hopr-nodes`):

```text
/tmp/hopr-nodes/
  hoprd_cfg_0.yaml       # generated hoprd config for node 0
  hoprd_cfg_1.yaml
  hoprd_cfg_2.yaml
  node_id_0.id           # encrypted identity keystore
  node_id_1.id
  node_id_2.id
  extra_id_0.id          # extra identity keystore (if --extra-identities > 0)
  extra_id_1.id
  cluster.lock           # single-instance advisory lock (holds owner pid, removed on exit)
  cluster.sock           # control socket serving live `status` (removed on exit)
  db_0/                  # node 0 database
  db_1/
  db_2/
  logs/
    hoprd_0.log          # stdout+stderr for node 0
    hoprd_1.log
    hoprd_2.log
    chain.log            # chain container output
```

---

## Extra identities

Pass `--extra-identities <N>` (1–5) to provision additional HOPR identities alongside the cluster. Each extra identity:

- Uses a **hardcoded keypair** (`EXTRA_KEYS` in `localcluster/src/identity.rs`), so the EVM address, Safe address, and Module address are the **same on every run** against a fresh Anvil chain.
- Gets funded with xDai and wxHOPR, and has its own Safe + Module deployed.
- Is written to `--data-dir` as `extra_id_{i}.id` — an encrypted Ethereum keystore.
- Uses the password `local-cluster` (a known constant; safe to hardcode in tooling).
- Is **not** started as a `hoprd` node.

When the cluster is ready the orchestrator prints a summary for each extra identity:

```
Extra 0
    Address       : 0x…
    Safe address  : 0x…
    Module address: 0x…
    Identity file : /tmp/hopr-nodes/extra_id_0.id
    Password      : local-cluster
```

Example:

```bash
RUST_LOG=info \
./result-localcluster/bin/hoprd-localcluster \
  --hoprd-bin ./result-hoprd/bin/hoprd \
  --chain-image "$CHAIN_IMAGE" \
  --size 3 \
  --extra-identities 2
```

---

## Troubleshooting

**Chain image pull is slow on first run** — The `/graphql` readiness poll has a 60s timeout. Pre-pull the image to avoid the race:

```bash
# Docker
docker pull --platform linux/amd64 <chain-image>

# Apple container
container image pull --platform linux/amd64 <chain-image>
```

**Apple `container` system is not running** — `container run` fails immediately. Run `container system start` once after each reboot.

**`/readyz` returns 412 forever** — Two possible causes:

1. _Network health Red_ — Nodes are not becoming minimally connected. This usually means P2P peers can't dial each other. Verify `--p2p-host` is an IP address (e.g. `127.0.0.1`), not a hostname. libp2p resolves hostname-based multiaddrs (like `/dns4/localhost/...`) at dial time — if DNS lookup fails or is slow the dial is silently dropped.
2. _Chain check failing_ — The chain indexer (blokli) is unreachable. Inspect `logs/hoprd_*.log` and `logs/chain.log` for errors. The indexer may need more than the 10s warmup.

**Port collisions** — Use `lsof -i :<port>` to find conflicts. Override with `--api-port-base` and `--p2p-port-base`.

**Stale state on re-run** — Remove `/tmp/hopr-nodes` between runs: `rm -rf /tmp/hopr-nodes`.
