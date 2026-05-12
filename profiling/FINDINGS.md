# hoprd memory leak — investigation synthesis

7 profiling runs on OrbStack NixOS VM (aarch64 Linux, jemalloc profiling
enabled). Tool stack: `jeprof`, `scripts/jeprof_plot.py`, `scripts/jeprof-vm.sh`.
All dump dirs under `hoprnet/jeprof-pull-*/`.

## TL;DR current state

| Layer                                            | Status                                   |
| ------------------------------------------------ | ---------------------------------------- |
| **SURB store** (`MemorySurbStore`)               | **Fixed** — defaults lowered, bounded    |
| **SessionManager cache** (DELETE invalidation)   | **Partially fixed** — 17% reduction only |
| **FrameInspector scaffolding** (Fix 1, deferred) | **Not yet fixed** — still 93-95% of leak |
| Idle baseline (no traffic)                       | **Healthy** — ~73 MB, flat for 6.3 h     |

Primary open issue: `FrameInspector::new` preallocates ~500 KB per session.
Fix: lower `SocketConfig::capacity` default 8192 → 256.

---

## Run timeline

### Run 1 — pre-rebase, 1000 iterations

**Dir:** `jeprof-pull-20260507-152514/`
**Code:** pre-master-rebase

| PID       |  Net growth |
| --------- | ----------: |
| 284977    |    308.7 MB |
| 284978    |    133.4 MB |
| 284979    |    210.0 MB |
| **total** | **~652 MB** |

Top suspect: `SessionManager::handle_incoming_session_initiation` 46% on PID 1,
`insert_surbs` 52% on PIDs 2/3. Shape: flat plateau at 436 MB, then sharp ramp
when session_loop hammered. Per-session cost: ~217 KB/node.

---

### Run 2 — post-rebase, 200 iterations

**Dir:** `jeprof-pull-20260508-124517-postrebase/`
**Code:** post-master-rebase

| PID       |  Net growth |
| --------- | ----------: |
| 593633    |    161.4 MB |
| 593634    |    164.9 MB |
| 593635    |    155.4 MB |
| **total** | **~482 MB** |

Top: `insert_surbs` 85–91%, `decode` 89–97%. No plateau — monotonically linear.
Stack root shifted from tokio to **rayon workers** (rebase changed SURB insertion
codepath from async to sync/rayon). Per-session cost jumped to **~803 KB/node
(3.7× worse than pre-rebase)**.

---

### Run 3 — post-rebase, 100 ms sleep between HTTP calls

**Dir:** `jeprof-pull-20260508-131142-sleep100/`
**Code:** same as Run 2

| PID       |   Net growth |
| --------- | -----------: |
| 595904    |     418.7 MB |
| 595905    |     412.8 MB |
| 595906    |     433.8 MB |
| **total** | **~1265 MB** |

Slowing requests exposed two additional suspects previously drowned by SURB noise:

- `create_session::{{closure}}` REST handler: 11–13% / ~50 MB per node
- `poll@9e67d4` anonymous future: 21–26% / ~100 MB per node

Larger total than Run 2 because more iterations ran (longer test), not a
regression — per-session cost unchanged.

---

### Root cause confirmed after Runs 1–3: `MemorySurbStore`

`protocols/hopr/src/surb_store.rs:162`. Two `moka::sync::Cache` instances keyed
by `HoprPseudonym`:

```rust
pseudonym_openers:   moka::sync::Cache<HoprPseudonym, moka::sync::Cache<HoprSurbId, ReplyOpener>>
surbs_per_pseudonym: moka::sync::Cache<HoprPseudonym, SurbRingBuffer<HoprSurb>>
```

Each session uses a new pseudonym (privacy guarantee). On first packet:

- Inserts a `SurbRingBuffer` with capacity `rb_capacity = 15_000` (≈6 MB fully loaded)
- Inserts an inner cache with `max_openers_per_pseudonym = 100_000`

`DELETE /api/v4/session` does **not** invalidate pseudonym entries. Only eviction:

- `time_to_idle = 600 s`
- `max_capacity = 10_000` pseudonyms LRU

A session_loop creating N sessions in T < 600 s accumulates all N simultaneously.
Bounded but the bound is enormous (10k × 6 MB = 60 GB potential).

**Experiment:** Lowering defaults should produce a plateau (LRU caps). If no
plateau → second leak path exists outside the SURB store.

---

### Run 4 — lowered SURB defaults (2000 iterations, 200 ms sleep)

**Dirs:** `jeprof-pull-20260508-150153-lowdefaults/` (dumps) +
`jeprof-pull-20260508-150400-lowdefaults/` (annotated findings)
**Code:** SURB defaults: `max_pseudonyms 10000→100`, `rb_capacity 15000→1024`,
`pseudonyms_lifetime 600s→60s`, `reply_opener_lifetime 3600s→300s`

| PID       |   Net growth |
| --------- | -----------: |
| 601074    |      1180 MB |
| 601075    |      1127 MB |
| 601076    |      1204 MB |
| **total** | **~3511 MB** |

**SURB store leak eliminated** — `insert_surbs`, `insert_reply_opener`, moka
`value_initializer` all gone from top. But a new dominant leak appeared:

```
FrameInspector::new                            93–94%  ~1100 MB/node
handle_incoming_session_initiation             61–65%   ~730 MB/node
new_session / create_session REST              35–40%   ~460 MB/node
```

Larger absolute total because the test ran longer (2000 iter vs ~700 in Run 3)
and `FrameInspector` is more expensive per session than the now-bounded SURB path.

**Diagnosis:** `protocols/session/src/processing/types.rs:120`:

```rust
pub fn new(capacity: usize) -> Self {
    Self(FrameDashMap::with_capacity(Self::INCOMPLETE_FRAME_RATIO * capacity + 1))
    // INCOMPLETE_FRAME_RATIO = 2 → with_capacity(16385)
}
```

Default capacity = 8192 (`protocols/session/src/socket/mod.rs:62–63`). DashMap
preallocates shards for 16385 slots ≈ **~500 KB per session**. At 2000 sessions
(all under `SessionManager.sessions.max_capacity = 2048` cap, not idle-timed out)
= ~1 GB per node. Matches exactly.

Same anti-pattern as SURB store: session slot kept in moka cache (3 min idle TTL),
never explicitly invalidated on `DELETE /session`.

---

### Run 5 — Fix 2 + Fix 3 (session-cache invalidation on DELETE)

**Dir:** `jeprof-pull-20260508-172539-fix2/`
**Fixes applied:**

- Fix 2: `transport/session/src/manager.rs` — added `close_session()` calling
  `self.sessions.remove(id).await` and the existing private close with `ClosureReason::Eviction`.
- `transport/api/src/lib.rs` — `HoprSessionConfigurator::close()` proxies to it.
- `hoprd/rest-api/src/session.rs` — DELETE handler calls `cfg.close().await` on each
  configurator before aborting the listener task.
- Fix 3: moka cache audit — 20 sites checked; only `SessionManager.sessions` is
  per-session. All others (per-peer, per-key) have appropriate TTL+capacity.

| PID       |   Net growth |
| --------- | -----------: |
| 606257    |     944.1 MB |
| 606258    |    1102.9 MB |
| 606261    |     864.7 MB |
| **total** | **~2912 MB** |

Fix 2 saved **−17 %** vs Run 4. `FrameInspector::new` still 93–95% — unchanged.

**Why modest impact:** Fix 2 removes the SessionSlot from the moka cache on DELETE,
but `FrameInspector` lives inside per-session tokio tasks spawned by
`new_session` / `handle_incoming_session_initiation`. Aborting them doesn't
synchronously drop the DashMap — runtime polls the Drop, and the hot session-create
loop outpaces drop reclamation. The accumulation is in-flight aborted futures whose
Drop hasn't completed yet, or Arc cycles keeping `FrameInspector` alive.

---

### Run 6 — idle baseline, 6.3 h (no session traffic)

**Dir:** `jeprof-pull-20260508-235000-longrun/`
**Code:** same as Run 5 (Fix 2+3 + lowered SURB defaults; FrameInspector capacity = 8192)

| PID       | Net delta (6.3 h) |
| --------- | ----------------: |
| 608117    |          +13.8 MB |
| 608118    |          +12.2 MB |
| 608121    |          +13.0 MB |
| **total** |        **~39 MB** |

`jemalloc_stats` from node 0: allocated = 72.75 MB at 60s, 73.11 MB at 6113s.
**Flat.** No `POST /api/v4/session` calls in the entire run.

Top diff contributors: tokio worker spawns (~7 MB), OTLP metric buffers (collector
unreachable → retained, ~5–8 MB), chain-key warmup (`insert_reply_opener` < 1 MB).
No `FrameInspector`, no `handle_incoming_session_initiation`, no `create_session`.

**Conclusion:** Background housekeeping does not leak. All observed growth in
prior runs is session-allocation–driven.

---

## Progression summary

| Run | Dir                                  | Code         | Load               |    Total Δ | Top suspect                            | Status                        |
| --- | ------------------------------------ | ------------ | ------------------ | ---------: | -------------------------------------- | ----------------------------- |
| 1   | `20260507-152514`                    | pre-rebase   | 1000 iter          |    ~652 MB | `handle_incoming` 46% + `insert_surbs` | leak found                    |
| 2   | `20260508-124517-postrebase`         | post-rebase  | 200 iter           |    ~482 MB | `insert_surbs` 85–91%                  | leak worsened 3.7×/session    |
| 3   | `20260508-131142-sleep100`           | post-rebase  | longer, 100ms pace |   ~1265 MB | `insert_surbs` 47–52% + REST 12%       | REST path visible             |
| 4   | `20260508-150153/150400-lowdefaults` | low SURB cfg | 2000 iter          |   ~3511 MB | **`FrameInspector::new` 93–94%**       | SURB fixed; new leak unmasked |
| 5   | `20260508-172539-fix2`               | Fix 2+3      | 2000 iter, 200ms   |   ~2912 MB | `FrameInspector::new` 93–95%           | −17% only                     |
| 6   | `20260508-235000-longrun`            | Fix 2+3      | **idle 6.3 h**     | **~39 MB** | otel buffers + tokio warmup            | baseline healthy              |

---

## Two root causes

### RC-1: `MemorySurbStore` — per-pseudonym caches never invalidated on session close

**File:** `protocols/hopr/src/surb_store.rs`  
**Fixed by:** lowering defaults (Run 4). Should also add explicit invalidation on
session close for correctness, not just bounded growth.

```bash
rg -n 'invalidate' protocols/hopr/src/surb_store.rs
# Add: pub fn invalidate_pseudonym(&self, p: &HoprPseudonym)
# Wire into DELETE /session close path
```

### RC-2: `FrameInspector` — 500 KB DashMap scaffolding per session, slow Drop

**File:** `protocols/session/src/processing/types.rs`,
`protocols/session/src/socket/mod.rs`  
**Not yet fixed.** Fix 2 (cache invalidation) helps 17%; Fix 1 (capacity) needed.

```rust
// protocols/session/src/socket/mod.rs:62
/// Default is 8192.
#[default(8192)]
pub capacity: usize,
```

Change to `#[default(256)]`. Each `FrameDashMap::with_capacity(2*256+1 = 513)`
≈ 16 KB vs ~500 KB. Per-node cost for 2000 sessions: ~32 MB vs ~1 GB.

Secondary: audit Drop ordering for spawned tasks from `SessionManager::new_session`
and `handle_incoming_session_initiation` — ensure no `Arc` cycle keeps
`FrameInspector` alive after `sessions.remove()`.

```bash
rg -n 'spawn|Arc.*FrameInspector\|FrameInspector.*Arc' \
   transport/session/src/manager.rs \
   protocols/session/src/socket/ \
   protocols/session/src/processing/
```

---

## Next experiment

Apply Fix 1 (`#[default(256)]`), rebuild, rerun 2000 iter + 200 ms sleep.

Expected:

- Total leak ~100 MB cluster-wide (32 MB × 3 nodes ≈ 96 MB).
- If still higher, per-session state outside `FrameInspector` accumulates
  (reassembly buffers, control-channel buffers of size 2048, segment channel
  of size 8192 in `socket/mod.rs`). Audit all `with_capacity` calls there.

```bash
rg -n 'with_capacity\|channel.*capacity\|bounded' \
   protocols/session/src/socket/mod.rs \
   protocols/session/src/processing/
```

---

## Quick reference — open artifacts

```bash
BASE=/Users/emil/Documents/hopr/hoprnet

# Idle baseline (healthy)
open "$BASE/jeprof-pull-20260508-235000-longrun/plots/total_inuse.png"

# Fix 2+3 run (FrameInspector still dominant)
open "$BASE/jeprof-pull-20260508-172539-fix2/plots/total_inuse.png"
open "$BASE/jeprof-pull-20260508-172539-fix2/leak/leak_606257.pdf"

# Lowered SURB defaults (FrameInspector first visible)
open "$BASE/jeprof-pull-20260508-150153-lowdefaults/plots/total_inuse.png"
open "$BASE/jeprof-pull-20260508-150153-lowdefaults/leak/leak_601074.pdf"
```
