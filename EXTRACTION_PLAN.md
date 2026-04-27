# hoprd Extraction Plan

Extract `hoprd` from `hoprnet` monorepo into standalone repo at `/Users/emil/Documents/hopr/hoprd/`.

## Decisions

1. **Dependency strategy**: git deps (reference hoprnet by SHA, migrate to crates.io later)
2. **localcluster**: moves to hoprd repo; hoprnet doesn't need it (hoprnet integration tests use in-process clusters via hopr-reference, not hoprd-localcluster)
3. **deploy/**: moves to hoprd repo; removed from hoprnet
4. **Target**: `/Users/emil/Documents/hopr/hoprd/` (already initialized git repo with LICENSE, .gitignore, README stub)

## Source → Destination Mapping

| Source (`hoprnet/`) | Dest (`hoprd/`) |
|---|---|
| `hoprd/hoprd/` | `hoprd/` |
| `hoprd/rest-api/` | `rest-api/` |
| `hoprd/rest-api-client/` | `rest-api-client/` |
| `localcluster/` | `localcluster/` |
| `deploy/` | `deploy/` |

## Target Structure

```
hoprd/
├── Cargo.toml           ← new workspace (4 members)
├── Cargo.lock
├── flake.nix            ← adapted from hoprnet (standalone, not using hoprnet as flake input)
├── flake.lock
├── hoprd/               ← from hoprnet/hoprd/hoprd/
├── rest-api/            ← from hoprnet/hoprd/rest-api/
├── rest-api-client/     ← from hoprnet/hoprd/rest-api-client/
├── localcluster/        ← from hoprnet/localcluster/
├── deploy/              ← from hoprnet/deploy/
├── .github/
│   └── workflows/
│       ├── pr.yaml
│       ├── merge.yaml
│       └── release.yaml
├── .gitignore           (exists)
├── LICENSE              (exists)
└── README.md            (stub → fill)
```

## Phase 1 — Copy Source Files

Copy (keep hoprnet intact until hoprd repo verified green):

```bash
cp -r /Users/emil/Documents/hopr/hoprnet/hoprd/hoprd      /Users/emil/Documents/hopr/hoprd/hoprd
cp -r /Users/emil/Documents/hopr/hoprnet/hoprd/rest-api   /Users/emil/Documents/hopr/hoprd/rest-api
cp -r /Users/emil/Documents/hopr/hoprnet/hoprd/rest-api-client /Users/emil/Documents/hopr/hoprd/rest-api-client
cp -r /Users/emil/Documents/hopr/hoprnet/localcluster     /Users/emil/Documents/hopr/hoprd/localcluster
cp -r /Users/emil/Documents/hopr/hoprnet/deploy           /Users/emil/Documents/hopr/hoprd/deploy
```

## Phase 2 — Cargo Workspace

New root `Cargo.toml` with 4 members: `hoprd`, `rest-api`, `rest-api-client`, `localcluster`.

`[workspace.dependencies]` needs:
- **Intra-repo path deps**:
  - `hoprd-api = { path = "rest-api", default-features = false }`
  - `hoprd-api-client = { path = "rest-api-client" }`
- **hoprnet git deps** (all `{ workspace = true }` deps in the 4 crate Cargo.tomls that are path deps in hoprnet):
  - `hopr-lib`
  - `hopr-strategy`
  - `hopr-reference`
  - `hopr-chain-connector`
  - `hopr-ct-full-network`
  - `hopr-network-graph`
  - `hopr-transport-p2p`
  - `hopr-metrics`
  - `hopr-async-runtime`
  - `hopr-utils-session`
  - All others found by scanning the 4 Cargo.tomls for `{ workspace = true }` entries that map to `path = "..."` in hoprnet's workspace
  - Pattern: `hopr-xxx = { git = "https://github.com/hoprnet/hoprnet", rev = "<SHA>", default-features = false }`
- **Third-party deps**: copy all matching entries from hoprnet `[workspace.dependencies]` (non-path entries)

Note: `hopr-types = { version = "1.5.4" }` is already a crates.io dep — copy version entry as-is.

SHA to use: pin to the last hoprnet commit before hoprd crates are removed.

Verify: `cargo build`, `cargo nextest run --lib`

## Phase 3 — Nix Flake

Standalone `flake.nix` (no hoprnet flake input — hoprnet flake builds all 42 crates, overkill).
Use crane + cross-compilation, adapted from hoprnet's `flake.nix` (822 lines → ~200 lines for 4 crates).

**Outputs to include:**

| Output | Notes |
|---|---|
| `binary-hoprd-{x86_64,aarch64}-{linux,darwin}` | release builds |
| `binary-hoprd-dev` | debug build |
| `binary-hoprd-candidate` | opt-level 2, lto=false (fast iteration) |
| `binary-hoprd-localcluster-x86_64-linux` | test harness |
| `docker-hoprd-{x86_64,aarch64}-linux` | production Docker |
| `docker-hoprd-dev-x86_64-linux` | dev Docker |
| `test-unit` | `cargo nextest --lib` |
| `devShells.{default,ci,test}` | dev environments |
| `docs` | rustdoc |

Risk: crane vendoring of git deps. Test `nix build .#hoprd-candidate` early.

## Phase 4 — CI Workflows

Reusable workflows from `hoprnet/hopr-workflows` work unchanged — reference them with current SHAs.

**`pr.yaml`** jobs:
- `validate-pr-title` — copy as-is
- `label` — copy as-is
- `checks` → `uses: hoprnet/hopr-workflows/.github/workflows/checks.yaml@<sha>`
- `tests` → `uses: hoprnet/hopr-workflows/.github/workflows/tests.yaml@<sha>` (unit enabled; integration TBD)
- `build-binaries` → `uses: hoprnet/hopr-workflows/.github/workflows/build-binaries.yaml@<sha>`
- `build-docker` → `uses: hoprnet/hopr-workflows/.github/workflows/build-docker.yaml@<sha>`
- `docs` — copy as-is

Drop: anything blokli/hoprnet-protocol-specific.

**`merge.yaml`**: build + push Docker images + deploy to Gnosis Dev.

**`release.yaml`**: copy, adapt artifact names.

Source workflows to adapt from: `/Users/emil/Documents/hopr/hoprnet/.github/workflows/`

## Phase 5 — README

Content:
- What: hoprd daemon + REST API for HOPR protocol
- Quick start: Docker (`docker run`) and binary
- Build: `nix build .#hoprd-candidate`, `cargo build --release`
- Config: reference `deploy/compose/hoprd/conf/hoprd.cfg.yaml`
- REST API: OpenAPI spec location
- Dev: link to hoprnet for protocol-level contributions

## Phase 6 — hoprnet Cleanup (after hoprd repo verified green)

1. Remove from `hoprnet/Cargo.toml` members: `hoprd/hoprd`, `hoprd/rest-api`, `hoprd/rest-api-client`, `localcluster`
2. Remove from `[workspace.dependencies]`: `hoprd-api`, `hoprd-api-client`
3. Remove from `flake.nix`: all `hoprd*` and `localcluster` outputs (~lines 160-200, 433-485 area)
4. Remove from hoprnet CI: hoprd binary + Docker build jobs
5. Delete directories: `hoprnet/hoprd/`, `hoprnet/localcluster/`, `hoprnet/deploy/`
6. Update `hoprnet/.claude/INSTRUCTIONS.md` to reflect hoprd lives in separate repo

## Phase 7 — Verification Checklist

- [ ] `cargo build` in `./hoprd`
- [ ] `cargo nextest run --lib` passes
- [ ] `cargo nextest run --test '*' -j 1` passes
- [ ] `nix build .#hoprd-candidate` builds
- [ ] `nix build .#docker-hoprd-x86_64-linux` produces runnable image
- [ ] `docker run` smoke test (node starts, API responds)
- [ ] CI push to draft PR — all jobs green
- [ ] hoprnet `cargo build` passes after cleanup
- [ ] hoprnet CI passes after cleanup

## Key File References

- hoprnet workspace: `/Users/emil/Documents/hopr/hoprnet/Cargo.toml`
- hoprnet flake: `/Users/emil/Documents/hopr/hoprnet/flake.nix` (822 lines)
- hoprnet CI: `/Users/emil/Documents/hopr/hoprnet/.github/workflows/pr.yaml`
- hoprd main Cargo.toml: `/Users/emil/Documents/hopr/hoprnet/hoprd/hoprd/Cargo.toml`
- rest-api Cargo.toml: `/Users/emil/Documents/hopr/hoprnet/hoprd/rest-api/Cargo.toml`
- localcluster Cargo.toml: `/Users/emil/Documents/hopr/hoprnet/localcluster/Cargo.toml`
- hopr-workflows: `/Users/emil/Documents/hopr/hopr-workflows/.github/workflows/`

## Effort Estimate

| Phase | Effort |
|---|---|
| 1 — Copy files | 0.5h |
| 2 — Cargo workspace + git deps | 2-4h |
| 3 — flake.nix | 1-2d |
| 4 — CI workflows | 4-6h |
| 5 — README | 1h |
| 6 — hoprnet cleanup | 2-4h |
| 7 — Verification | 1d |
