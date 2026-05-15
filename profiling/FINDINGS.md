# hoprd Memory Leak — Investigation Synthesis

Seven profiling runs were conducted on an OrbStack NixOS VM (aarch64 Linux, jemalloc
profiling enabled). The tooling stack consists of `jeprof`, `scripts/jeprof_plot.py`,
and `scripts/jeprof-vm.sh`. All dump directories live under `hoprnet/jeprof-pull-*/`.

## Current State (TL;DR)

| Layer                                            | Status                                   |
| ------------------------------------------------ | ---------------------------------------- |
| **SURB store** (`MemorySurbStore`)               | **Fixed** — defaults lowered, bounded    |
| **SessionManager cache** (DELETE invalidation)   | **Partially fixed** — 17% reduction only |
| **FrameInspector scaffolding** (Fix 1, deferred) | **Not yet fixed** — still 93–95% of leak |
| Idle baseline (no traffic)                       | **Healthy** — ~73 MB, flat over 6.3 h   |

The primary open issue is that `FrameInspector::new` preallocates approximately 500 KB
per session. The fix is to lower the `SocketConfig::capacity` default from 8192 to 256.

---

## Run Timeline

### Run 1 — Pre-rebase, 1000 iterations

**Directory:** `jeprof-pull-20260507-152514/`  
**Codebase:** pre-master-rebase

| PID       |  Net growth |
| --------- | ----------: |
| 284977    |    308.7 MB |
| 284978    |    133.4 MB |
| 284979    |    210.0 MB |
| **total** | **~652 MB** |

The top suspect was `SessionManager::handle_incoming_session_initiation` at 46% on
PID 1, and `insert_surbs` at 52% on PIDs 2 and 3. The memory profile showed a flat
plateau at 436 MB followed by a sharp ramp when `session_loop` hammered the nodes.
Per-session cost: approximately 217 KB per node.

---

### Run 2 — Post-rebase, 200 iterations

**Directory:** `jeprof-pull-20260508-124517-postrebase/`  
**Codebase:** post-master-rebase

| PID       |  Net growth |
| --------- | ----------: |
| 593633    |    161.4 MB |
| 593634    |    164.9 MB |
| 593635    |    155.4 MB |
| **total** | **~482 MB** |

`insert_surbs` dominated at 85–91% and `decode` at 89–97%. Unlike Run 1, there was
no plateau — growth was monotonically linear throughout. The stack root shifted from
tokio to **rayon workers**, because the rebase changed the SURB insertion codepath
from async to sync/rayon. Per-session cost jumped to **~803 KB per node, a 3.7×
regression compared to pre-rebase**.

---

### Run 3 — Post-rebase, 100 ms sleep between HTTP calls

**Directory:** `jeprof-pull-20260508-131142-sleep100/`  
**Codebase:** same as Run 2

| PID       |   Net growth |
| --------- | -----------: |
| 595904    |     418.7 MB |
| 595905    |     412.8 MB |
| 595906    |     433.8 MB |
| **total** | **~1265 MB** |

Pacing requests more slowly exposed two additional suspects that had been drowned out
by SURB noise in the previous runs:

- `create_session::{{closure}}` REST handler: 11–13% / ~50 MB per node
- `poll@9e67d4` anonymous future: 21–26% / ~100 MB per node

The larger absolute total compared to Run 2 is because more iterations completed over
the longer test duration — per-session cost was unchanged.

---

### Root Cause Confirmed After Runs 1–3: `MemorySurbStore`

`protocols/hopr/src/surb_store.rs:162`. Two `moka::sync::Cache` instances keyed
by `HoprPseudonym`:

```rust
pseudonym_openers:   moka::sync::Cache<HoprPseudonym, moka::sync::Cache<HoprSurbId, ReplyOpener>>
surbs_per_pseudonym: moka::sync::Cache<HoprPseudonym, SurbRingBuffer<HoprSurb>>
```

Each session uses a fresh pseudonym as a privacy guarantee. On the first packet,
the store inserts a `SurbRingBuffer` with capacity `rb_capacity = 15_000` (≈6 MB
when fully loaded) and an inner cache with `max_openers_per_pseudonym = 100_000`.

`DELETE /api/v4/session` does **not** invalidate pseudonym entries. The only eviction
paths are:

- `time_to_idle = 600 s`
- `max_capacity = 10_000` pseudonyms (LRU)

A `session_loop` that creates N sessions in less than 600 seconds accumulates all N
simultaneously. The store is technically bounded, but the bound is enormous:
10,000 pseudonyms × 6 MB = up to 60 GB.

**Hypothesis for Run 4:** Lowering the defaults should produce a plateau once LRU
kicks in. If no plateau appears, a second leak path exists outside the SURB store.

---

### Run 4 — Lowered SURB Defaults (2000 iterations, 200 ms sleep)

**Directories:** `jeprof-pull-20260508-150153-lowdefaults/` (dumps) and
`jeprof-pull-20260508-150400-lowdefaults/` (annotated findings)  
**Changes applied:** `max_pseudonyms 10000→100`, `rb_capacity 15000→1024`,
`pseudonyms_lifetime 600s→60s`, `reply_opener_lifetime 3600s→300s`

| PID       |   Net growth |
| --------- | -----------: |
| 601074    |      1180 MB |
| 601075    |      1127 MB |
| 601076    |      1204 MB |
| **total** | **~3511 MB** |

**The SURB store leak was eliminated** — `insert_surbs`, `insert_reply_opener`, and
`moka::value_initializer` all disappeared from the top of the profile. However, a
new dominant leak became visible:

```text
FrameInspector::new                            93–94%  ~1100 MB/node
handle_incoming_session_initiation             61–65%   ~730 MB/node
new_session / create_session REST              35–40%   ~460 MB/node
```

The larger absolute total compared to Run 3 is because the test ran longer (2000
iterations vs. ~700) and `FrameInspector` is more expensive per session than the
now-bounded SURB path.

**Diagnosis — `protocols/session/src/processing/types.rs:120`:**

```rust
pub fn new(capacity: usize) -> Self {
    Self(FrameDashMap::with_capacity(Self::INCOMPLETE_FRAME_RATIO * capacity + 1))
    // INCOMPLETE_FRAME_RATIO = 2 → with_capacity(16385)
}
```

The default capacity is 8192 (`protocols/session/src/socket/mod.rs:62–63`). DashMap
preallocates shards for 16,385 slots, which comes to approximately **500 KB per
session**. At 2000 sessions — all fitting within `SessionManager.sessions.max_capacity
= 2048`, so none have been idle-evicted — that is roughly 1 GB per node, which
matches the observed numbers exactly.

This is the same anti-pattern as the SURB store: the session slot is held in a moka
cache with a 3-minute idle TTL and is never explicitly invalidated when
`DELETE /session` is called.

---

### Run 5 — Fix 2 + Fix 3 (Session Cache Invalidation on DELETE)

**Directory:** `jeprof-pull-20260508-172539-fix2/`  
**Fixes applied:**

- **Fix 2:** `transport/session/src/manager.rs` — added `close_session()`, which
  calls `self.sessions.remove(id).await` and then the existing private close with
  `ClosureReason::Eviction`.
- `transport/api/src/lib.rs` — `HoprSessionConfigurator::close()` proxies to it.
- `hoprd/rest-api/src/session.rs` — the DELETE handler now calls `cfg.close().await`
  on every configurator before aborting the listener task.
- **Fix 3:** Audited all 20 moka cache sites; only `SessionManager.sessions` is
  per-session. All other caches (per-peer, per-key) have appropriate TTL and capacity.

| PID       |   Net growth |
| --------- | -----------: |
| 606257    |     944.1 MB |
| 606258    |    1102.9 MB |
| 606261    |     864.7 MB |
| **total** | **~2912 MB** |

Fix 2 achieved a **−17%** reduction compared to Run 4. `FrameInspector::new` still
accounts for 93–95% of growth and is effectively unchanged.

**Why the impact was modest:** Fix 2 removes the `SessionSlot` from the moka cache
on DELETE, but `FrameInspector` lives inside per-session tokio tasks spawned by
`new_session` and `handle_incoming_session_initiation`. Aborting those tasks does not
synchronously drop the DashMap — the runtime schedules the Drop, and the
high-frequency session-creation loop outpaces drop reclamation. The accumulation is
either in-flight aborted futures whose Drop has not yet completed, or Arc cycles that
keep `FrameInspector` alive after `sessions.remove()`.

---

### Run 6 — Idle Baseline, 6.3 Hours (No Session Traffic)

**Directory:** `jeprof-pull-20260508-235000-longrun/`  
**Codebase:** same as Run 5 (Fix 2+3 with lowered SURB defaults; FrameInspector
capacity still 8192)

| PID       | Net delta (6.3 h) |
| --------- | ----------------: |
| 608117    |          +13.8 MB |
| 608118    |          +12.2 MB |
| 608121    |          +13.0 MB |
| **total** |        **~39 MB** |

`jemalloc_stats` from node 0: allocated = 72.75 MB at t=60s, 73.11 MB at t=6113s.
**The baseline is flat.** No `POST /api/v4/session` calls were issued during the
entire run.

The small growth seen in the diff comes from: tokio worker spawns (~7 MB), OTLP
metric buffers retained because the collector was unreachable (~5–8 MB), and
chain-key warmup (`insert_reply_opener` < 1 MB). There is no `FrameInspector`,
no `handle_incoming_session_initiation`, and no `create_session` in the profile.

**Conclusion:** Background housekeeping does not leak. All growth observed in the
previous runs is driven entirely by session allocation.

---

## Progression Summary

| Run | Directory                            | Code         | Load                |    Total Δ | Top suspect                            | Outcome                       |
| --- | ------------------------------------ | ------------ | ------------------- | ---------: | -------------------------------------- | ----------------------------- |
| 1   | `20260507-152514`                    | pre-rebase   | 1000 iter           |    ~652 MB | `handle_incoming` 46% + `insert_surbs` | Leak found                    |
| 2   | `20260508-124517-postrebase`         | post-rebase  | 200 iter            |    ~482 MB | `insert_surbs` 85–91%                  | Leak worsened 3.7×/session    |
| 3   | `20260508-131142-sleep100`           | post-rebase  | longer, 100ms pace  |   ~1265 MB | `insert_surbs` 47–52% + REST 12%       | REST path visible             |
| 4   | `20260508-150153/150400-lowdefaults` | low SURB cfg | 2000 iter           |   ~3511 MB | **`FrameInspector::new` 93–94%**       | SURB fixed; new leak unmasked |
| 5   | `20260508-172539-fix2`               | Fix 2+3      | 2000 iter, 200ms    |   ~2912 MB | `FrameInspector::new` 93–95%           | −17% only                     |
| 6   | `20260508-235000-longrun`            | Fix 2+3      | **idle 6.3 h**      | **~39 MB** | OTel buffers + tokio warmup            | Baseline healthy              |

---

## Two Root Causes

### RC-1: `MemorySurbStore` — Per-pseudonym Caches Never Invalidated on Session Close

**File:** `protocols/hopr/src/surb_store.rs`  
**Status:** Fixed by lowering defaults (Run 4). Explicit invalidation on session
close should also be added for correctness — the current fix only bounds growth
rather than eliminating it.

```bash
rg -n 'invalidate' protocols/hopr/src/surb_store.rs
# Add: pub fn invalidate_pseudonym(&self, p: &HoprPseudonym)
# Wire into the DELETE /session close path
```

### RC-2: `FrameInspector` — 500 KB DashMap Scaffolding per Session, Slow Drop

**Files:** `protocols/session/src/processing/types.rs`,
`protocols/session/src/socket/mod.rs`  
**Status:** Not yet fixed. Fix 2 (cache invalidation) contributes −17%; Fix 1
(capacity reduction) is still required.

```rust
// protocols/session/src/socket/mod.rs:62
/// Default is 8192.
#[default(8192)]
pub capacity: usize,
```

Change to `#[default(256)]`. Each `FrameDashMap::with_capacity(2×256+1 = 513)` will
occupy approximately 16 KB instead of ~500 KB. For 2000 sessions per node, that
reduces the footprint from ~1 GB to ~32 MB.

As a secondary step, audit the Drop ordering for tasks spawned by
`SessionManager::new_session` and `handle_incoming_session_initiation` to confirm
there are no Arc cycles keeping `FrameInspector` alive after `sessions.remove()`.

```bash
rg -n 'spawn|Arc.*FrameInspector|FrameInspector.*Arc' \
   transport/session/src/manager.rs \
   protocols/session/src/socket/ \
   protocols/session/src/processing/
```

---

## Next Experiment

Apply Fix 1 (`#[default(256)]`), rebuild, and rerun with 2000 iterations at 200 ms
pacing.

Expected outcome:
- Total leak approximately 100 MB cluster-wide (32 MB × 3 nodes ≈ 96 MB).
- If growth remains higher, there is per-session state accumulating outside
  `FrameInspector` — reassembly buffers, control-channel buffers (size 2048), or
  the segment channel (size 8192 in `socket/mod.rs`). Audit all `with_capacity`
  calls there.

```bash
rg -n 'with_capacity|channel.*capacity|bounded' \
   protocols/session/src/socket/mod.rs \
   protocols/session/src/processing/
```

---

## Quick Reference — Open Artifacts

```bash
BASE="${REPO_ROOT:-$(git rev-parse --show-toplevel)}"

# Idle baseline (healthy)
open "$BASE/jeprof-pull-20260508-235000-longrun/plots/total_inuse.png"

# Fix 2+3 run (FrameInspector still dominant)
open "$BASE/jeprof-pull-20260508-172539-fix2/plots/total_inuse.png"
open "$BASE/jeprof-pull-20260508-172539-fix2/leak/leak_606257.pdf"

# Lowered SURB defaults (FrameInspector first visible)
open "$BASE/jeprof-pull-20260508-150153-lowdefaults/plots/total_inuse.png"
open "$BASE/jeprof-pull-20260508-150153-lowdefaults/leak/leak_601074.pdf"
```
