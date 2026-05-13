# jemalloc Heap Profiling via Linux VM

This guide covers day-to-day use of the `jeprof-vm.sh` helper script for building, running, and analyzing jemalloc heap profiles of `hoprd` on a Linux VM (or any remote Linux host).

> **Prerequisites:** Complete the one-time VM setup described in [profiling_vm.md](./profiling_vm.md) before proceeding.

jemalloc profiling is Linux-only. This workflow lets macOS developers drive the full profiling cycle: source is synced to a Linux host via SSH, built there with Nix, run with heap profiling enabled, and the resulting dump files are pulled back to macOS for analysis.

The examples below use **OrbStack + NixOS** as the reference setup. Any Linux host reachable by SSH works — set `VM_HOST` and `CHAIN_URL` to match your environment (see the environment overrides table).

---

## Connecting to the VM

All communication with the VM happens over plain SSH. Set `VM_HOST` to your SSH target before running any subcommand (or export it in your shell):

```bash
export VM_HOST=user@<your-vm-ip>   # any Linux host
export VM_HOST=nixos-test@orb      # OrbStack default
```

```bash
ssh "$VM_HOST"                                          # interactive shell
scp file "$VM_HOST":                                    # copy a file to the VM home directory
scp "${VM_HOST}:/tmp/jeprof/*.heap" ./out/              # copy heap dumps back to macOS
```

**OrbStack note:** SSH keys are managed automatically at `~/.orbstack/ssh/id_ed25519` and wired into `~/.ssh/config`. The default VM user is `nixos-test` with passwordless sudo via the `wheel` group.

---

## The `jeprof-vm.sh` Script

Run all subcommands from the **macOS host**, at the repository root.

```bash
./scripts/jeprof-vm.sh sync                  # rsync the repo to the VM (incremental)
./scripts/jeprof-vm.sh build                 # build the hoprd profiling binary on the VM
./scripts/jeprof-vm.sh build-localcluster    # build hoprd-localcluster on the VM
./scripts/jeprof-vm.sh run                   # run a single hoprd node against a remote blokli URL
./scripts/jeprof-vm.sh localcluster N        # run an N-node cluster against a local anvil_blokli
./scripts/jeprof-vm.sh clean                 # wipe transient data and heap dumps on the VM
./scripts/jeprof-vm.sh all                   # sync + build + run (single node, convenience alias)
```

Both `run` and `localcluster` are blocking — they follow the process until you press Ctrl-C. A final heap dump is always written on shutdown (`prof_final:true`).

### Environment overrides

| Variable           | Default                              | Description                                              |
| ------------------ | ------------------------------------ | -------------------------------------------------------- |
| `VM_HOST`          | `nixos-test@orb`                     | SSH target for the NixOS VM                              |
| `BLOKLI_URL`       | `https://blokli.rotsee.hoprnet.link` | Blokli endpoint used by the single-node `run` subcommand |
| `CHAIN_URL`        | `http://host.orb.internal:8080`      | Chain + blokli endpoint used by `localcluster`           |
| `HOPRD_PASSWORD`   | `test-profiling-password`            | Identity file password                                   |
| `PROFILE_DIR`      | `/tmp/jeprof`                        | Directory on the VM where heap dumps are written         |
| `CLUSTER_DIR`      | `/tmp/hoprd-cluster`                 | Data directory for cluster node state on the VM          |
| `LG_PROF_INTERVAL` | `25`                                 | Dump every 2^N bytes of net allocation (25 ≈ 32 MB)      |

### Build artifact locations (on the VM)

```
~/hoprd/result-hoprd/bin/hoprd                    # statically linked (musl), jemalloc-profiling
~/hoprd/result-hoprd/bin/hoprd-cfg
~/hoprd/result-localcluster/bin/hoprd-localcluster
```

Named result symlinks let both binaries coexist on the same filesystem without clobbering each other.

> **First build note:** The cross-toolchain (GCC + binutils + musl) compiles from source the first time because `cache.nixos.org` has no substitutes for these custom musl-cross hashes. Expect 30–60 minutes. Subsequent incremental builds are fast — Crane only recompiles changed crates.

---

## Multi-Node Cluster Workflow

This is the recommended path for realistic memory profiling across multiple nodes.

### 1. Start `anvil_blokli` on macOS

The cluster nodes need a local chain and blokli endpoint, which the `anvil_blokli` Docker image provides on port 8080. From the VM the endpoint is reachable at `http://host.orb.internal:8080`.

```bash
docker rm -f anvil_blokli
docker run --rm --name anvil_blokli --platform linux/amd64 -p 8080:8080 -d \
  europe-west3-docker.pkg.dev/hoprassociation/docker-images/bloklid-anvil:latest
```

### 2. Build on the VM

Do this once, then only when source changes.

```bash
./scripts/jeprof-vm.sh sync
./scripts/jeprof-vm.sh build               # hoprd profiling binary
./scripts/jeprof-vm.sh build-localcluster  # cluster orchestrator
```

### 3. Run the cluster with profiling

```bash
./scripts/jeprof-vm.sh localcluster 3      # start 3 nodes
```

Each child `hoprd` process inherits `_RJEM_MALLOC_CONF` from the orchestrator (Rust's `Command::new` inherits the parent environment by default). The PID is embedded in every dump filename (`jeprof.<PID>.<seq>.iN.heap`), so all nodes write to the same `PROFILE_DIR` without filename collisions.

Node REST API endpoints are `http://127.0.0.1:3000`, `:3001`, `:3002`, … (one per node). These are only reachable from inside the VM unless you set up `ssh -L` port forwarding.

---

## Single-Node Run (against the rotsee testnet)

```bash
./scripts/jeprof-vm.sh run
```

Or manually, directly on the VM:

```bash
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

### Tuning `_RJEM_MALLOC_CONF`

| Parameter          | Value          | Effect                                                                                             |
| ------------------ | -------------- | -------------------------------------------------------------------------------------------------- |
| `lg_prof_sample`   | `19`           | Sample every 2^19 ≈ 512 KB of allocations                                                          |
| `lg_prof_interval` | `25` (default) | Dump every 2^25 ≈ 32 MB of net allocations                                                         |
| `lg_prof_interval` | `30`           | Dump every ~1 GB — sparse dumps for long runs                                                      |
| `lg_prof_interval` | `20`           | Dump every ~1 MB — high frequency; generates 100k+ files and 7+ GB in minutes for a 3-node cluster |
| `prof_final`       | `true`         | Always write a final dump on exit                                                                  |

Override per-run with the `LG_PROF_INTERVAL` env variable.

---

## Observing Memory Live

```bash
# RSS / VSZ snapshot
ssh nixos-test@orb 'pids=$(pgrep -d, -x hoprd); [ -n "$pids" ] && ps -o pid,rss,vsz,comm -p "$pids" || echo "no hoprd processes"'

# Interactive top
ssh nixos-test@orb 'pids=$(pgrep -d, -x hoprd); [ -n "$pids" ] && top -p "$pids" || echo "no hoprd processes"'
```

When built with the `allocator-jemalloc-stats` feature, `hoprd` also logs jemalloc statistics every 60 seconds at the `INFO` level:

```
allocated=… active=… mapped=… retained=… arenas_active=… cache_efficiency=…
```

---

## Analyzing Heap Dumps

### Using the helper script (recommended)

Run from macOS, at the repository root. The script SSHes into the VM, runs `jeprof`, and prints or writes results locally.

```bash
./scripts/jeprof-vm.sh analyze summary        # top in-use allocators per PID, latest dump
./scripts/jeprof-vm.sh analyze diff           # latest vs. earliest snapshot per PID (growth delta)
./scripts/jeprof-vm.sh analyze top            # cumulative alloc_space, latest dump
./scripts/jeprof-vm.sh analyze svg            # write /tmp/jeprof_<PID>_diff.svg per PID

./scripts/jeprof-vm.sh analyze summary 284977  # restrict to a specific PID
./scripts/jeprof-vm.sh pull ./jeprof-out       # scp earliest + latest dumps and binary to macOS
```

#### Reading the text output

| Column 1 | Column 2 | Column 3 | Column 4 | Column 5 | Column 6 |
| -------- | -------- | -------- | -------- | -------- | -------- |
| self     | self%    | cum      | cum%     | (unused) | function |

- The first row is almost always `_rjem_je_prof_backtrace` at ~100%. This is jemalloc's own per-sample metadata overhead — skip it and read the cumulative percentages of the user functions below.
- **"What grew?"** → use `analyze diff`. Non-zero rows are the delta between the earliest and latest snapshot.
- **"What allocates total bytes?"** → use `analyze top` (alloc_space).
- **"What is currently held?"** → use `analyze summary` (inuse_space).

### Manual analysis on the VM

`jeprof` and `graphviz` are already installed in the VM user profile. The binary path must match the one that produced the dumps.

```bash
ssh nixos-test@orb
BIN=~/hoprd/result-hoprd/bin/hoprd
H=/tmp/jeprof

# Top allocators (text output):
jeprof --text "$BIN" "$H/jeprof.<PID>.<N>.iN.heap" | head -30

# SVG call graph:
jeprof --svg "$BIN" "$H/jeprof.<PID>.<N>.iN.heap" > /tmp/heap.svg

# Growth diff between snapshots A and B:
jeprof --text --base="$H/jeprof.<PID>.<A>.iA.heap" \
       "$BIN" "$H/jeprof.<PID>.<B>.iB.heap"

# Interactive shell:
jeprof "$BIN" "$H"/jeprof.*.heap
# Inside: top, list <fn>, web, disasm <fn>, peek <fn>
```

When running a cluster, filter by PID — each node's dumps are distinct:

```bash
ls /tmp/jeprof/                                        # group by PID prefix
jeprof --text "$BIN" /tmp/jeprof/jeprof.12345.*.heap | head
```

### Pulling dumps to macOS for offline analysis

```bash
mkdir -p ~/jeprof-out
scp 'nixos-test@orb:/tmp/jeprof/*.heap' ~/jeprof-out/
scp nixos-test@orb:~/hoprd/result-hoprd/bin/hoprd ~/jeprof-out/hoprd

# Install jemalloc on macOS (brew install jemalloc), then run jeprof the same way:
jeprof --text ~/jeprof-out/hoprd ~/jeprof-out/jeprof.*.heap | head -30
```

---

## Re-syncing After Source Changes

```bash
./scripts/jeprof-vm.sh sync    # rsync the repo to the VM (incremental)
./scripts/jeprof-vm.sh build   # incremental Nix build (only changed crates recompile)
```

---

## Known Caveats

- **Ctrl-C propagation:** Works correctly through `ssh -t`'s PTY in interactive runs. If the cluster is started from a background script, signals may not propagate. Clean up manually:
  ```bash
  ssh nixos-test@orb 'pkill -INT hoprd-localcluster; pkill -INT hoprd'
  ```
- **OpenTelemetry errors in logs:** `Connection refused 127.0.0.1:4318` appears because `hoprd-localcluster` always sets `HOPRD_OTLP_ENDPOINT=http://localhost:4318` and no collector is running. This is harmless — ignore it.
- **Dumps stay on the VM** until you pull them with `scp` or the `pull` subcommand.
