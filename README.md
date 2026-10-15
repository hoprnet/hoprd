# hoprd

Full HOPR node daemon and REST API. Runs a HOPR node that participates in the privacy-preserving messaging network and exposes a REST API for interaction.

## Quick Start

### Docker

```bash
docker run --rm -it \
  -v $(pwd)/hoprd-data:/app/hoprd-data \
  -p 9091:9091/udp \
  -p 3001:3001 \
  gcr.io/hoprassociation/hoprd:stable \
  --identity /app/hoprd-data/hopr.id \
  --data /app/hoprd-data \
  --password yourpassword \
  --apiPort 3001 \
  --apiToken yourtoken
```

### Binary

Download from [releases](https://github.com/hoprnet/hoprd/releases) or build from source:

```bash
nix build .#hoprd-candidate
./result/bin/hoprd --help
```

## Build

### Nix (recommended)

```bash
# Fast iterative build (opt-level 2, no LTO)
nix build .#hoprd-candidate

# Release build
nix build .#binary-hoprd

# Docker image
nix build .#docker-hoprd-x86_64-linux
```

### Cargo

```bash
# Debug build
nix develop -c cargo build

# Release build
cargo build --profile release
```

## Configuration

Generate default config:

```bash
# Print default YAML config
cargo run --release -p hoprd --bin hoprd-cfg -- --default

# Validate existing config
cargo run --release -p hoprd --bin hoprd-cfg -- --validate /path/to/hoprd.yaml
```

Key config options:

| Option                       | Description                                                |
| ---------------------------- | ---------------------------------------------------------- |
| `host`                       | P2P listen address                                         |
| `apiPort`                    | REST API port (default 3001)                               |
| `apiToken`                   | Authentication token for REST API                          |
| `enableExplicitPathSessions` | Enables the deprecated explicit-path REST session endpoint |
| `identity`                   | Path to node identity file                                 |
| `data`                       | Data directory for node state                              |
| `password`                   | Identity file password                                     |
| `network`                    | Network to connect to (e.g. `dufour`)                      |

## REST API

OpenAPI spec is generated at build time. Served at `http://localhost:3001/scalar` (Scalar UI) and `http://localhost:3001/swagger-ui` when the node is running.

### Relay throughput benchmark

Measures how many packets per second this node's CPU can relay. The node prepares synthetic traffic routed through itself, then relays it at full speed: SPHINX processing, ticket validation and re-issue, and acknowledgements in both directions. It never touches the node's keys, channels, tickets or packet metrics, and network I/O is not measured. It shares the CPU pool with live traffic, so relaying slows down while it runs.

```bash
curl -X POST -H "X-Auth-Token: $TOKEN" http://localhost:3001/api/v4/node/benchmark/relay \
  -H 'Content-Type: application/json' -d '{"packets": 20000}'
curl -H "X-Auth-Token: $TOKEN" http://localhost:3001/api/v4/node/benchmark/relay
```

The body is optional (`packets` defaults to 20000, at most 100000; `concurrency` defaults to the packet pipeline's own). Poll the `GET` until `state` is `completed` or `failed`; `result.packetsPerSecond` and `result.megabitsPerSecond` give the throughput.

## Development

```bash
# Enter dev shell
nix develop

# Unit tests
cargo nextest run --lib

# Integration tests (single-threaded — shared cluster resources)
cargo nextest run --test '*' -j 1

# Lints
nix run -L .#check
```

## Protocol contributions

Protocol libraries (`hopr-lib`, `hopr-transport-*`, `hopr-crypto-*`, etc.) live in [hoprnet/hoprnet](https://github.com/hoprnet/hoprnet). Submit protocol-level changes there.

## Local cluster

Spin up a multi-node HOPR network locally with `hoprd-localcluster`. See [`docs/localcluster/README.md`](docs/localcluster/README.md) for full setup instructions.

## Memory Profiling

`hoprd` supports detailed heap profiling using `jemalloc` and `jeprof`.

- **[General Profiling Guide](profiling/memory-profiling.md)**: How to build and run the profiling variants.
- **[VM Setup](profiling/profiling_vm.md)**: Setting up a Linux VM (OrbStack example) for profiling on macOS.
- **[VM Usage Helper](profiling/jeprof-vm-usage.md)**: Using the `jeprof-vm.sh` script to drive builds and analysis.
