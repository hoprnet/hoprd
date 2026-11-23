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

### Artifacts: relay and exit

hoprd builds two artifacts. Features are additive, so the relay-only one needs `--no-default-features`.

| Artifact | Cargo features                                                                                        | Nix package                                                          | What it links                                                       |
| -------- | ----------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------- | ------------------------------------------------------------------- |
| exit     | default (`exit`, which implies `relay`)                                                               | `binary-hoprd-x86_64-linux`, `docker-hoprd-x86_64-linux`             | The relay plus the Session server, the Session REST API and PIX.    |
| relay    | `--no-default-features -F relay,runtime-tokio,telemetry,transport-quic,allocator-jemalloc,state-dump` | `binary-hoprd-relay-x86_64-linux`, `docker-hoprd-relay-x86_64-linux` | Ticket and acknowledgement processing only. No Session code at all. |

The relay build rejects the Session and PIX configuration keys (`api.session_flow_control`,
`api.enable_explicit_path_sessions`, `hopr.network.maximum_sessions`, `hopr.network.pix`,
`hopr.network.incoming_session_pix`, `session_ip_forwarding`) and has no `/session` endpoints.
`session-server` is a deprecated alias of `exit`. `scripts/check-entity-fencing.sh` verifies from the
dependency graph that the relay links no Session crate.

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
