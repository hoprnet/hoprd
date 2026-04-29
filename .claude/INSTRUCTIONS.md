# hoprd Development Guidelines

## Project Overview

`hoprd` is the Full HOPR node daemon. It runs the HOPR protocol (sourced from [hoprnet/hoprnet](https://github.com/hoprnet/hoprnet) as git dependencies) and exposes a REST API for interaction.

**Workspace members**:

- [`hoprd/`](hoprd/) — daemon binary + lib (Axum REST server, config, telemetry)
- [`rest-api/`](rest-api/) — REST API handlers (utoipa/OpenAPI)
- [`rest-api-client/`](rest-api-client/) — generated Rust client for the REST API
- [`localcluster/`](localcluster/) — local multi-node test harness

**Protocol libs live in [hoprnet/hoprnet](https://github.com/hoprnet/hoprnet)** — `hopr-lib`, `hopr-transport-*`, `hopr-crypto-*`, etc. are consumed as git deps. Do not modify them here; protocol changes must be made in the hoprnet repo and then the SHA updated here.

## Build & Test

### Essential (run before committing)

```bash
nix fmt                        # Format all code
nix run -L .#check             # Clippy + all linters
```

### Build

Nix is the primary build system. Cargo commands run inside `nix develop` to get the correct toolchain and dependencies.

```bash
# Nix builds (preferred)
nix build .#hoprd-candidate             # Fast build (opt-level 2, lto=false)
nix build .#binary-hoprd                # Release build
nix build .#docker-hoprd-x86_64-linux   # Docker image (Linux only)

# Cargo (inside nix dev shell)
nix develop -c cargo build              # Debug build
cargo build --profile release           # Production (opt-level 3, lto="fat")
```

### Test

```bash
# Unit tests
nix develop -c cargo nextest run --lib
nix develop -c cargo nextest run --lib -p <crate>

# Integration tests (single-threaded — shared cluster state)
nix develop -c cargo nextest run --test '*' -j 1
nix develop -c cargo nextest run -p <crate> --test <test_name> -j 1
```

### Coverage

```bash
nix build -L .#coverage-unit     # LCOV report → ./result
```

### Setup

1. Nix with flakes: `experimental-features = nix-command flakes` in `~/.config/nix/nix.conf`
2. `direnv allow .` — auto-loads dev environment

## Technology Stack

- **Rust 1.95** (stable), edition 2024
- **Async**: Tokio (`runtime-tokio` feature gates throughout)
- **REST API**: Axum + utoipa (OpenAPI 3), token auth via `X-Auth-Token` / `Bearer`
- **Telemetry**: OpenTelemetry (OTLP), Prometheus metrics
- **Testing**: cargo-nextest, insta (snapshot tests)

## Code Style

### Critical Rules

- `tracing::info!()` not `info!()` — explicit prefix required
- `parking_lot::Mutex` (sync) or `tokio::sync::Mutex` (async) — never `std::sync::Mutex`
- `thiserror` for library errors, `anyhow` for application errors
- All channels must be bounded
- `tracing::info!()` not `info!()`

For language-specific rules see [rust.md](rust.md).

### REST API ([rest-api/](rest-api/))

- Auth: `X-Auth-Token` header or `Bearer` token
- OpenAPI: served at `/scalar` and `/swagger-ui`
- Patterns: `#[utoipa::path]` macros, `#[derive(ToSchema)]` on types
- Entry: [rest-api/src/lib.rs](rest-api/src/lib.rs)

### Configuration ([hoprd/src/config.rs](hoprd/src/config.rs))

- Validation: `#[derive(Validate)]` + `validator` crate
- Defaults: `#[derive(SmartDefault)]`
- Default config: [deploy/compose/hoprd/conf/hoprd.cfg.yaml](deploy/compose/hoprd/conf/hoprd.cfg.yaml)
- Example config: [hoprd/example_cfg.yaml](hoprd/example_cfg.yaml)

## Common Mistakes (AVOID)

1. `std::sync::Mutex` in async → use `parking_lot::Mutex` or `tokio::sync::Mutex`
2. Unbounded channels → always specify capacity
3. Integration tests in parallel → `-j 1` required
4. Missing tracing prefix → `tracing::info!()` not `info!()`
5. `.unwrap()` in libraries → propagate with `?`
6. Protocol-level changes here → those belong in the [hoprnet](https://github.com/hoprnet/hoprnet) repo

## Dependency Strategy

hoprnet crates are consumed as git deps pinned to a SHA in the root `Cargo.toml`. To update:

1. Find the target SHA in [hoprnet/hoprnet](https://github.com/hoprnet/hoprnet)
2. Update all `rev = "..."` entries in the root `Cargo.toml`
3. Run `cargo check` to verify
4. Run `nix build .#hoprd-candidate` to verify Nix vendoring
