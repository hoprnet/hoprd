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
nix build -L .#binary-hoprd .#binary-hoprd-localcluster
# binaries: ./result/bin/hoprd  and  ./result-1/bin/hoprd-localcluster

# Cargo (inside the dev shell)
nix develop -c cargo build -p hoprd -p hoprd-localcluster
# binaries: target/debug/hoprd  and  target/debug/hoprd-localcluster
```

---

## Run

The chain image version must match the `blokli-client` version pinned in `Cargo.lock`. Check `Cargo.lock` for the `blokli-client` entry and use the image tag that was built from the same commit. The `latest` tag may have breaking GraphQL schema changes relative to the pinned client.

To find the compatible tag for the currently pinned client:

```bash
grep -A3 'name = "blokli-client"' Cargo.lock
# note the source commit, then find the matching image tag in the registry
```

### Default (Docker)

```bash
rm -rf /tmp/hopr-nodes   # clear any stale state

CHAIN_IMAGE=europe-west3-docker.pkg.dev/hoprassociation/docker-images/bloklid-anvil:0.10.3-pr.339

RUST_LOG=info \
./result-1/bin/hoprd-localcluster \
  --hoprd-bin ./result/bin/hoprd \
  --chain-image "$CHAIN_IMAGE" \
  --size 3
```

### Apple `container` (macOS)

```bash
container system start   # once per boot

rm -rf /tmp/hopr-nodes

CHAIN_IMAGE=europe-west3-docker.pkg.dev/hoprassociation/docker-images/bloklid-anvil:0.10.3-pr.339

RUST_LOG=info \
HOPRD_CONTAINER_RUNTIME=container \
./result-1/bin/hoprd-localcluster \
  --hoprd-bin ./result/bin/hoprd \
  --chain-image "$CHAIN_IMAGE" \
  --size 3
```

> **macOS NAT note**: the orchestrator detects the container's direct IP (e.g. `192.168.64.x`) and uses it instead of `localhost:8080`. This avoids macOS port-forwarding NAT, which drops long-lived SSE connections used by the chain indexer client.

### External chain (skip the container)

If you already have Blokli running at a known URL, pass it directly and the container step is skipped entirely:

```bash
HOPRD_CHAIN_URL=http://localhost:8080 \
./result-1/bin/hoprd-localcluster \
  --hoprd-bin ./result/bin/hoprd \
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

## Configuration reference

Flags take precedence over env vars. Only the flags marked with an env var below support one.

| Flag                  | Env var                   | Default           | Description                                            |
| --------------------- | ------------------------- | ----------------- | ------------------------------------------------------ |
| `--size`              | —                         | `3`               | Number of nodes to start (1–5)                         |
| `--api-host`          | —                         | `localhost`       | Host to bind the REST API on                           |
| `--api-port-base`     | —                         | `3000`            | First API port (each node gets base + id)              |
| `--p2p-host`          | —                         | `localhost`       | Host to bind P2P on                                    |
| `--p2p-port-base`     | —                         | `9000`            | First P2P port                                         |
| `--data-dir`          | —                         | `/tmp/hopr-nodes` | Root for configs, identities, DBs, logs                |
| `--chain-image`       | `HOPRD_CHAIN_IMAGE`       | —                 | Container image for Blokli + Anvil                     |
| `--chain-url`         | `HOPRD_CHAIN_URL`         | —                 | External Blokli URL; skips the container step          |
| `--container-runtime` | `HOPRD_CONTAINER_RUNTIME` | `docker`          | Container CLI (`docker`, `container`, `podman`, …)     |
| `--hoprd-bin`         | —                         | `hoprd`           | Path to the `hoprd` binary                             |
| `--identity-password` | —                         | `password`        | Password for identity encryption                       |
| `--api-token`         | —                         | none              | Bearer token for the REST API                          |
| `--funding-amount`    | —                         | `1 wxHOPR`        | Per-channel funding amount                             |
| `--skip-channels`     | —                         | `false`           | Skip opening payment channels                          |
| `--extra-identities`  | —                         | `0`               | Extra pre-funded identities for external tooling (0–5) |

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
./result-1/bin/hoprd-localcluster \
  --hoprd-bin ./result/bin/hoprd \
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

**`bloklid-anvil:latest` fails with a schema error** — The `latest` tag may have a breaking GraphQL schema change relative to the `blokli-client` version pinned in `Cargo.lock`. Use a version-pinned tag (e.g. `0.10.3-pr.339`) that matches the client's source commit. See the [Run](#run) section for how to identify the compatible tag.

**Apple `container` system is not running** — `container run` fails immediately. Run `container system start` once after each reboot.

**`/readyz` returns 412 forever** — The chain check is failing. The indexer may need more than the 10s warmup. Inspect `logs/hoprd_*.log` and `logs/chain.log` for errors.

**Port collisions** — Use `lsof -i :<port>` to find conflicts. Override with `--api-port-base` and `--p2p-port-base`.

**Stale state on re-run** — Remove `/tmp/hopr-nodes` between runs: `rm -rf /tmp/hopr-nodes`.
