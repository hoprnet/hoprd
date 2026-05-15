# Memory Profiling with jemalloc

`hoprd` uses [jemalloc](http://jemalloc.net/) as its memory allocator on Linux, with optional heap profiling support via `jeprof`. On macOS, `hoprd` uses the system allocator and jemalloc profiling is not available. All profiling workflows described here target Linux (including Docker on macOS hosts).

jemalloc heap profiling generates detailed memory allocation reports and SVG call-graph visualizations to diagnose memory usage patterns and leaks.

## Quick start

The fastest way to run memory profiling end-to-end is via the provided just recipe:

```bash
just test-jeprof
```

This builds the profiling Docker image, starts `hoprd` with heap profiling enabled, collects heap dumps, and generates SVG analysis files in `/tmp/hoprd-jeprof-profiles-*/analysis/`.

## How it works

The profiling setup has three layers, controlled via Cargo feature flags on the `hoprd` package. These features only take effect on Linux (`cfg(target_os = "linux")`); on other platforms the system allocator is used regardless of feature flags.

| Feature                        | What it does                                                                                     |
| ------------------------------ | ------------------------------------------------------------------------------------------------ |
| `allocator-jemalloc`           | Swaps the system allocator for jemalloc (Linux only)                                             |
| `allocator-jemalloc-profiling` | Compiles jemalloc with `--enable-prof` for heap sampling (Linux only)                            |
| `allocator-jemalloc-stats`     | Enables periodic runtime statistics logging (allocated, active, mapped memory, cache efficiency) |

A dedicated Cargo build profile `memprof` (defined in the workspace `Cargo.toml`) inherits from `release` but preserves debug symbols (`debug = 2`, `strip = false`) so that `jeprof` can resolve function names in the heap dumps.

The Nix flake provides pre-configured profiling build targets:

```bash
# Build the profiling Docker image (includes jeprof, graphviz, gdb, etc.)
nix build .#docker-hoprd-profile-x86_64-linux

# Build just the profiling binary (without Docker packaging)
nix build .#binary-hoprd-profile-x86_64-linux    # x86_64
nix build .#binary-hoprd-profile-aarch64-linux   # ARM64
```

## Running the profiling Docker image manually

1. Build and load the profiling Docker image:

   ```bash
   nix build .#hoprd-profile-docker
   docker load <result
   ```

2. Run `hoprd` with profiling enabled via `_RJEM_MALLOC_CONF`:

   ```bash
   docker run -d --name hoprd-profile \
     -e "_RJEM_MALLOC_CONF=prof:true,prof_active:true,prof_final:true,prof_prefix:/app/.tmp/jeprof,lg_prof_sample:19,lg_prof_interval:26" \
     -e "HOPRD_PASSWORD=my-password" \
     hoprd:latest \
     bash -c "mkdir -p /tmp/hoprd /app/.tmp && exec hoprd --data /tmp/hoprd --identity /tmp/hoprd/identity --apiHost 0.0.0.0 --blokli-url https://your-blokli-url"
   ```

   > **Note:** The `_RJEM_` prefix is required because `tikv-jemallocator` renames jemalloc symbols. The standard `MALLOC_CONF` variable will not work.

3. After collecting enough data, stop the container and extract the heap files:

   ```bash
   docker stop hoprd-profile
   docker cp hoprd-profile:/app/.tmp ./heap-dumps
   ```

## Analyzing heap dumps

The `scripts/analyze_memory.sh` script generates multiple SVG visualizations from a heap dump file:

```bash
# Inside the profiling Docker container (which has jeprof and graphviz):
docker run --rm \
  -v ./heap-dumps:/profiles:ro \
  -v ./scripts/analyze_memory.sh:/analyze_memory.sh:ro \
  -v ./output:/output \
  hoprd:latest \
  bash /analyze_memory.sh /bin/hoprd /profiles/jeprof.12345.0.heap /output/analysis
```

This produces several SVGs in the output directory:

| File                  | Description                                               |
| --------------------- | --------------------------------------------------------- |
| `*_overview.svg`      | Full memory usage call graph                              |
| `*_top20.svg`         | Top 20 memory consumers                                   |
| `*_significant.svg`   | Only allocations above 1% of total                        |
| `*_detailed.svg`      | Call graph with source line numbers                       |
| `*_objects.svg`       | Allocation count (number of objects, not bytes)           |
| `*_rust_specific.svg` | Focused on Rust runtime allocations (tokio, serde, std)   |
| `*_rust_logic.svg`    | Application logic only (excludes panic/runtime internals) |
| `*_large_allocs.svg`  | Only allocations above 5% of total                        |

## Tuning profiling parameters

The `_RJEM_MALLOC_CONF` environment variable controls profiling behavior:

| Parameter              | Description                                                              | Example                            |
| ---------------------- | ------------------------------------------------------------------------ | ---------------------------------- |
| `prof:true`            | Enable the profiling subsystem                                           | Required                           |
| `prof_active:true`     | Start sampling immediately (set `false` to activate later via `mallctl`) | Required for auto-profiling        |
| `prof_final:true`      | Dump a final heap profile on process exit                                | Useful for leak detection          |
| `prof_prefix:<path>`   | Path prefix for heap dump files                                          | `/app/.tmp/jeprof`                 |
| `lg_prof_sample:<N>`   | Sample every 2^N bytes of allocation (19 = ~512KB)                       | Lower = more detail, more overhead |
| `lg_prof_interval:<N>` | Dump a heap profile every 2^N bytes of allocation                        | 26 = ~64MB, 20 = ~1MB              |

For the full list of available options, see the [jemalloc manual — `MALLOC_CONF`](http://jemalloc.net/jemalloc.3.html#tuning).

## Runtime statistics

When built with the `allocator-jemalloc-stats` feature, `hoprd` logs jemalloc statistics periodically (every 60 seconds) at the `INFO` level. The logged metrics include:

- **allocated** / **active** / **mapped** / **retained**: Memory usage breakdown
- **cache_efficiency**: Ratio of allocated to active memory (higher is better)
- **narenas**, **tcache_max**, **background_thread**: Allocator configuration

The base jemalloc configuration is baked in at compile time via `JEMALLOC_SYS_WITH_MALLOC_CONF` in `.cargo/config.toml` and includes tuned parameters for production use (4 arenas, 64KB tcache, aggressive page decay).
