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

Default config template: [`deploy/compose/hoprd/conf/hoprd.cfg.yaml`](deploy/compose/hoprd/conf/hoprd.cfg.yaml)

Example config: [`hoprd/example_cfg.yaml`](hoprd/example_cfg.yaml)

Key config options:

| Option     | Description                           |
| ---------- | ------------------------------------- |
| `host`     | P2P listen address                    |
| `apiPort`  | REST API port (default 3001)          |
| `apiToken` | Authentication token for REST API     |
| `identity` | Path to node identity file            |
| `data`     | Data directory for node state         |
| `password` | Identity file password                |
| `network`  | Network to connect to (e.g. `dufour`) |

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

The best way to test with multiple HOPR nodes is by using a [local cluster of interconnected nodes](https://github.com/hoprnet/hoprd/blob/main/SETUP_LOCAL_CLUSTER.md).
