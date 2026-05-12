# jemalloc profiling on OrbStack NixOS VM — usage

jemalloc profiling is Linux-only. macOS host builds + drives, VM runs.

## Connecting to the VM

SSH key is at `~/.orbstack/ssh/id_ed25519`. Already wired into
`~/.ssh/config` via the OrbStack include.

```bash
ssh nixos-test@orb         # interactive shell
scp file nixos-test@orb:   # copy file to VM home
scp 'nixos-test@orb:/tmp/jeprof/*.heap' ./out/   # copy dumps back
```

VM user: matches your macOS host user (OrbStack default). Sudo: passwordless via `wheel`. No further creds needed.

## Subcommands of `scripts/jeprof-vm.sh` (run from macOS, repo root)

```bash
./scripts/jeprof-vm.sh sync                  # tar-pipe repo to VM
./scripts/jeprof-vm.sh build                 # build hoprd profile binary
./scripts/jeprof-vm.sh build-localcluster    # build hoprd-localcluster
./scripts/jeprof-vm.sh run                   # single-node hoprd vs BLOKLI_URL
./scripts/jeprof-vm.sh localcluster N        # N-node cluster vs CHAIN_URL
./scripts/jeprof-vm.sh all                   # sync + build + run (single node)
```

Env overrides:

```bash
VM_HOST=nixos-test@orb                          # default
BLOKLI_URL=https://blokli.rotsee.hoprnet.link   # single-node remote testnet
CHAIN_URL=http://host.orb.internal:8080         # cluster local anvil_blokli
HOPRD_PASSWORD=test-profiling-password
PROFILE_DIR=/tmp/jeprof                         # heap dumps on VM
CLUSTER_DIR=/tmp/hoprd-cluster                  # cluster data dir on VM
LG_PROF_INTERVAL=25                             # dump every 2^25 ≈ 32MB allocated; lower = more dumps
```

`run` and `localcluster` block; Ctrl-C to stop. Final dump emitted on
shutdown (`prof_final:true`).

Build outputs use named symlinks so they don't clobber each other:

```
~/hoprd/result-hoprd          # hoprd profile binary
~/hoprd/result-localcluster   # hoprd-localcluster binary
```

## Full localcluster workflow (multi-node, jemalloc enabled)

1. **Start anvil_blokli on macOS** (provides chain + blokli on `:8080`):

   ```bash
   docker rm -f anvil_blokli
   docker run --rm --name anvil_blokli --platform linux/amd64 -p 8080:8080 -d \
     europe-west3-docker.pkg.dev/hoprassociation/docker-images/bloklid-anvil:latest
   ```

   Endpoint reachable from VM at `http://host.orb.internal:8080/graphql`.

2. **Build both binaries on VM** (one-time, then incremental):

   ```bash
   ./scripts/jeprof-vm.sh sync
   ./scripts/jeprof-vm.sh build              # hoprd profile
   ./scripts/jeprof-vm.sh build-localcluster # cluster orchestrator
   ```

3. **Run cluster with profiling**:

   ```bash
   ./scripts/jeprof-vm.sh localcluster 3     # 3 nodes
   ```

   Each spawned hoprd inherits `_RJEM_MALLOC_CONF` from the orchestrator
   (Rust `Command::new` inherits parent env by default). PID is baked
   into dump filename (`jeprof.<PID>.<N>.iN.heap`), so all nodes write
   to the same `PROFILE_DIR` without colliding.

   API endpoints: `http://127.0.0.1:3000`, `:3001`, `:3002`, ... (one
   per node). Only reachable from the VM unless you `ssh -L` forward.

## Single-node run (against rotsee testnet)

```bash
./scripts/jeprof-vm.sh run
# or, manual on the VM:
ssh nixos-test@orb
cd ~/hoprd
mkdir -p /tmp/jeprof /tmp/hoprd
HOPRD_PASSWORD=test \
_RJEM_MALLOC_CONF='prof:true,prof_active:true,prof_final:true,prof_prefix:/tmp/jeprof/jeprof,lg_prof_sample:19,lg_prof_interval:20' \
./result-hoprd/bin/hoprd \
  --data /tmp/hoprd \
  --identity /tmp/hoprd/identity \
  --apiHost 127.0.0.1 \
  --blokli-url https://blokli.rotsee.hoprnet.link
```

`_RJEM_MALLOC_CONF` knobs:

- `lg_prof_sample:19` → sample every 2^19 ≈ 512KB allocations
- `lg_prof_interval:25` → dump heap every 2^25 ≈ 32MB net allocations (default)
- `prof_final:true` → dump on exit
- override via `LG_PROF_INTERVAL=N`; 30 ≈ 1GB, 20 ≈ 1MB (warning: 1MB
  generates 100k+ dumps and 7+GB in a few minutes for a 3-node cluster)

## Observing memory live

```bash
# RSS / VSZ snapshot
ssh nixos-test@orb 'ps -o pid,rss,vsz,comm -p $(pgrep -x hoprd | tr "\n" ",")'

# top
ssh nixos-test@orb 'top -p $(pgrep -x hoprd | paste -sd,)'

# jemalloc stats line is logged automatically by hoprd::jemalloc_stats:
#   allocated=… active=… mapped=… retained=… arenas_active=… cache_efficiency=…
```

## Analyzing dumps via the helper (recommended)

```bash
./scripts/jeprof-vm.sh analyze summary       # top inuse_space allocators per PID, latest dump
./scripts/jeprof-vm.sh analyze diff          # latest vs earliest snapshot per PID (growth)
./scripts/jeprof-vm.sh analyze top           # cumulative alloc_space, latest dump
./scripts/jeprof-vm.sh analyze svg           # write /tmp/jeprof_<PID>_diff.svg per PID

./scripts/jeprof-vm.sh analyze summary 284977   # restrict to one PID
./scripts/jeprof-vm.sh pull ./jeprof-out        # scp earliest+latest+binary to macOS
```

How to read the text output:

| col 1 | col 2 | col 3 | col 4 | col 5 | col 6    |
| ----- | ----- | ----- | ----- | ----- | -------- |
| self  | self% | cum%  | cum   | cum%  | function |

- The first row is almost always `_rjem_je_prof_backtrace` at ~100%. That
  is jemalloc's own per-sample metadata, not your code. Skip it and
  read the cumulative percentages of user functions below.
- For "what grew?" use `analyze diff` — non-zero rows are the delta.
- For "what allocates total bytes?" use `analyze top` (alloc_space).
- For "what is currently held?" use `analyze summary` (inuse_space).

## Analyzing dumps with jeprof (on the VM, manual)

`jeprof` + `graphviz` already installed in user profile. Binary path
matters — must match the one that produced the dumps.

```bash
ssh nixos-test@orb
BIN=~/hoprd/result-hoprd/bin/hoprd
H=/tmp/jeprof

# Top allocators (text):
jeprof --text "$BIN" "$H/jeprof.<PID>.<N>.iN.heap" | head -30

# SVG call graph:
jeprof --svg "$BIN" "$H/jeprof.<PID>.<N>.iN.heap" > /tmp/heap.svg

# Diff (growth between snapshots A → B):
jeprof --text --base="$H/jeprof.<PID>.<A>.iA.heap" \
       "$BIN" "$H/jeprof.<PID>.<B>.iB.heap"

# Interactive shell:
jeprof "$BIN" "$H"/jeprof.*.heap
# commands inside: top, list <fn>, web, disasm <fn>, peek <fn>
```

Filter by node when running a cluster — each node's PID is distinct:

```bash
ls /tmp/jeprof/                          # group by PID
jeprof --text "$BIN" /tmp/jeprof/jeprof.12345.*.heap | head
```

Pull artifacts to macOS for offline analysis:

```bash
mkdir -p ~/jeprof-out
scp 'nixos-test@orb:/tmp/jeprof/*.heap' ~/jeprof-out/
scp nixos-test@orb:~/hoprd/result-hoprd/bin/hoprd ~/jeprof-out/hoprd
# install jemalloc on macOS (brew install jemalloc) then run jeprof
# the same way against ~/jeprof-out/hoprd + heap files.
```

## One-time VM setup (already done)

- `git` installed: `nix-env -iA nixos.git` (build sandbox needs it for
  cargo git deps).
- `jemalloc` + `graphviz` + `perl` + `binutils` + `openssl` installed:
  `nix-env -iA nixos.jemalloc nixos.graphviz nixos.perl nixos.binutils
nixos.openssl nixos.python313`
  (provides `jeprof`, `dot`, `objdump`, `addr2line`, `nm`, `libssl.so.3`,
  `python3`).
- `nix.settings.trusted-users = [ "root" "@wheel" ]` added to
  `/etc/nixos/configuration.nix` and `nixos-rebuild switch` run.
  (Optional — only needed if VM is later used as a remote nix builder.)

## Build artifact locations

```
VM:
  ~/hoprd/result-hoprd/bin/hoprd            # 50MB ELF aarch64 musl, jemalloc-profiling
  ~/hoprd/result-hoprd/bin/hoprd-cfg
  ~/hoprd/result-localcluster/bin/hoprd-localcluster
```

First build of the cross-toolchain compiles GCC + binutils + musl from
source on the VM (cache.nixos.org has no substitutes for these custom
musl-cross hashes). Expect 30–60 min on first run; later builds reuse
`/nix/store` and are fast.

## Re-syncing source after edits on macOS

```bash
./scripts/jeprof-vm.sh sync     # full re-tar
./scripts/jeprof-vm.sh build    # incremental nix build
```

Crane diffs cargo inputs; only changed crates recompile.

## Changelog (relative to original `just test-jeprof` workflow)

- **Original `just test-jeprof` is unchanged.** Still expects a Linux
  build environment; broken on macOS by design (`nixLib.mkDockerImage`
  hard-requires x86_64-linux).
- **Added** `scripts/jeprof-vm.sh` with subcommands:
  `sync` / `build` / `build-localcluster` / `run` / `localcluster N` /
  `all`.
- **Added** `scripts/JEPROF-VM-USAGE.md` (this file).
- **VM env**: installed `git`, `jemalloc`, `graphviz`, `perl` via
  `nix-env`. Updated NixOS config to trust `@wheel`.
- **Build outputs** moved from `result/` to `result-hoprd/` and
  `result-localcluster/` to coexist on the same VM filesystem.
- **`localcluster` subcommand** wires `hoprd-localcluster` to the
  jemalloc env so all child hoprd processes profile with PID-distinct
  dump filenames. Default chain URL is `http://host.orb.internal:8080`
  (anvil_blokli docker on macOS host).
- **No `flake.nix` changes kept.** Earlier exploratory edits were
  reverted; chosen strategy does not need them.
- **`hoprd-localcluster` runtime fix:** the orchestrator is dynamically
  linked (glibc) and pulls in `libssl.so.3`/`libcrypto.so.3`. The
  `localcluster` subcommand sets `LD_LIBRARY_PATH` to the
  `nixos.openssl` store path before `exec`. (`hoprd` itself is
  statically linked against musl, so it has no such dep.)
- **Default `lg_prof_interval=25`** (32MB). The original
  `test-jeprof.sh` used `20` (1MB), which is fine for a 10-second
  smoke test but produces ~7GB and 400k+ files for a 3-node cluster
  running 3 minutes.

## Verified end-to-end (2026-05-07)

Smoke test of full localcluster path on the VM:

- `anvil_blokli` Docker on macOS host, `:8080/graphql` reachable from
  VM via `host.orb.internal`.
- 3-node `hoprd-localcluster` spawned on VM, each child got distinct
  PID, all wrote heap dumps to `/tmp/jeprof/jeprof.<PID>.*.heap`.
- `hoprd::jemalloc_stats` log lines emitted in each
  `/tmp/hoprd-cluster/logs/hoprd_<N>.log` (proves jemalloc is the
  active allocator and stats poller is running).
- Env propagation confirmed: `_RJEM_MALLOC_CONF` set on the
  orchestrator parent → all 3 children inherited and profiled.

## Known caveats

- **Ctrl-C in interactive run propagates correctly** through `ssh -t`'s
  PTY. If you start the cluster from a script that backgrounds the
  process, signals do _not_ always propagate; clean up manually with
  `ssh nixos-test@orb 'pkill -INT hoprd-localcluster; pkill -INT hoprd'`.
- **OpenTelemetry export errors** (`Connection refused 127.0.0.1:4318`)
  appear in node logs because `hoprd-localcluster` always sets
  `HOPRD_OTLP_ENDPOINT=http://localhost:4318` and there is no collector
  running. Harmless; ignore.
- **Dumps stay on the VM**; pull with `scp` for off-VM analysis.
