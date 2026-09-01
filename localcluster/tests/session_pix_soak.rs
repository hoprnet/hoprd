//! Sustained-throughput PIX Session test (not for CI — run explicitly).
//!
//! The same happy path as [`session_pix`](../session_pix.rs), scaled up to something
//! resembling real use — tens of megabytes crossing the Session in both directions, SSA
//! cycles counted in the tens — and run to its natural end rather than to a target.
//!
//! # Four nodes, two of them relays
//!
//! ```text
//!            ┌── node1 ──┐
//!   Entry ───┤           ├─── Exit
//!            └── node2 ──┘
//! ```
//!
//! One Session at [`HOPS`] = 1, and *both* relays carry it. Nothing in this test alternates
//! between them: hoprd re-resolves the route for every outgoing packet, so a second viable
//! relay is a second candidate the planner draws from. `resolve_routing_stage` ("resolves the
//! routing of every outgoing packet") feeds `PathPlanner::resolve_routing`, whose `Hops` arm
//! picks from a cached weighted collection of validated paths with `pick_one()`; return paths
//! go through `resolve_diverse_return_paths`, which draws from γ-tempered weights with a share
//! of uniform exploration on top. Both live in `hopr-transport::path::planner`.
//!
//! So the split is statistical rather than round-robin, which is what
//! [`MIN_RELAY_SHARE_PERCENT`] is a floor for. Two relays instead of one changes neither the
//! aggregate packet rate nor the SSA geometry — each simply relays about half — so every figure
//! below is unaffected by the topology.
//!
//! It does buy one thing no 3-node run can: a relay that never reaches the path selector at all
//! — an unopened channel, a missed announcement, a graph that only ever found one path — leaves
//! every *other* assertion in this test passing. Here it fails the share floor.
//!
//! # The run ends when the Entry runs out of budget
//!
//! There is no cycle target and no clock. The Entry is given a fixed deposit budget, it
//! commits `price_per_byte × quota` of it per SSA, and when the next deposit would cross the
//! budget the strategy refuses it, the Exit stops seeing money arrive, and its PIX kill switch
//! closes the Session with `ClosureReason::UnrealizedDeposit`. That is the designed behaviour,
//! so this test asserts it happens rather than treating it as a failure — and it makes the run
//! self-limiting:
//!
//! The budget is `PixStrategyConfig::max_spend_per_window`, not an empty account. It used to be
//! the latter: deposits were paid by the node's own account, so funding that account with
//! exactly N cycles' worth ended the run after N. `hopr-types` 4.0.0 moved the payer to the
//! Safe, which also holds the channel stakes — so "the money ran out" would now mean "the
//! stakes' leftovers ran out too", and the cycle count would depend on stake arithmetic that
//! has nothing to do with PIX. The budget states the number instead. The Safe is still funded
//! with the float, comfortably above the budget, so it is never what binds.
//!
//! ```text
//! runtime ≈ bootstrap + (float / deposit_per_ssa) × (emissions_per_ssa / packet_rate)
//!                     + kill-switch fuse
//! ```
//!
//! Both terms on the right are knobs: `HOPRD_PIX_SOAK_FLOAT` buys cycles and
//! `HOPRD_PIX_SOAK_RATE` sets how fast each one is consumed. The default float is exactly
//! [`DEFAULT_FUNDED_CYCLES`] cycles' worth, which lands the whole run inside ~7 minutes —
//! measured 407 s, of which 244 s is cluster bootstrap, so the traffic itself is the shorter
//! half.
//! To leave the cluster up for observation, fund it for longer:
//!
//! ```bash
//! HOPRD_PIX_SOAK_FLOAT="2000 wxHOPR" nix develop -c cargo nextest run -p hoprd-localcluster \
//!   --test session_pix_soak --run-ignored ignored-only -j 1 --no-capture
//! ```
//!
//! `--no-capture` matters: without it nextest buffers the progress reports until the run
//! ends, which for a large float is hours away.
//!
//! # Why the geometry is sized to the packet rate
//!
//! An SSA cycle is bounded by *packets*, not by seconds: a cycle emits
//! `polys × (shares + surplus)` shares, each riding one return-path SURB, so it lasts
//! `emissions / share_rate`. That same product is the per-SSA quota, so a cycle's length and
//! its price are the same number scaled — the Exit is paid per return packet it sends.
//! Three things follow, and together they fix the geometry once a target rate is chosen:
//!
//!   * **A cycle must outlast a deposit.** The Exit serves data on credit and reconstructs
//!     the key as soon as the shares are in, whether or not the money has arrived. If it
//!     wins that race, `sweep_recovered` finds a zero balance, logs "already swept", drops
//!     the entry, and the deposit is stranded at the stealth address for good. Raising the
//!     rate alone would shorten cycles into that race, so the SSA is widened in step:
//!     `emissions ≈ TARGET_CYCLE_SECS × share_rate`.
//!   * **The return SURB budget is shared between shares and data.** A reply carries either
//!     a share or echoed payload, never both, and each costs one SURB. So the buffer has to
//!     be sized for the sum — see [`response_buffer`]. Sizing it from the SSA alone is
//!     exactly what broke when the rate was first raised to 1000/s against an unscaled SSA:
//!     the share stream took the whole budget, the echo starved to nothing, and the run got
//!     *longer* rather than shorter. The Exit's own egress shaping
//!     (`balancer_minimum_surb_buffer_duration`) is what enforces this.
//!   * **A share is baked into its SURB at mint time**, and the Exit spends its buffer
//!     roughly in order, so the buffer is a pipeline delay between generating a share and
//!     delivering it. Because it drains at the *combined* rate, that delay is
//!     `SURB_RUNWAY_SECS` seconds regardless of scale. Sized at `session_udp`'s 10 MB it
//!     would be several cycles and nothing after the first would ever complete. The same
//!     reasoning fixes [`CHUNK_SIZE`] large enough that SURBs cannot piggyback on data
//!     packets, leaving the balancer as the only supply and so the only thing that sets the
//!     buffer.
//!
//! The rate is not unbounded: the buffer must stay under `rb_capacity × 2/3` = 66 666 SURBs
//! ([`SURB_BUFFER_CEILING`]) or the balancer's overshoot evicts shares, which caps this shape at
//! roughly 7000 datagrams/s — `SURB_RUNWAY_SECS × (share_rate + rate)` with a share rate of 1296.
//! [`DEFAULT_PACKET_RATE`] is 4000 and the ladder below saturates at 6500, both inside it, and
//! the test refuses to start above it rather than running into eviction. Beyond that needs the
//! `NoRateControl` capability or a larger ring buffer, neither of which this test uses.
//!
//! # Live observation
//!
//! The startup banner prints each node's scrape URL. Note that hoprd's `/metrics` strips
//! every `hopr_session_*` series, since they are labelled by session id: node-wide
//! counters — `hopr_packets_count`, `hopr_strategy_pix_*` — are there, while per-session
//! packet and SURB counters go over OTLP only, to `HOPRD_OTLP_ENDPOINT` (default
//! `http://localhost:4318`).
//!
//! Required (at least one chain source):
//!   HOPRD_CHAIN_URL   or HOPRD_CHAIN_IMAGE
//! Optional: HOPRD_BIN, HOPRD_CONTAINER_RUNTIME, HOPRD_PIX_SOAK_FLOAT, HOPRD_PIX_SOAK_RATE
//!
//! # Prerequisites
//!
//! The `hoprd` binary must be built in **release** mode. This test moves tens of
//! megabytes through the packet pipeline; a debug build will not reach the packet rate the
//! SSA sizing assumes, and cycles will stretch until the run hits its safety deadline.
//!
//! It must also name a **deposit pool**, which no default supplies:
//!
//! ```bash
//! nix develop -c cargo build --release -p hoprd --features strategy-pix-test
//! export HOPRD_BIN=$(pwd)/target/release/hoprd
//! ```
//!
//! # Which deposit pool
//!
//! A PIX deposit address is whatever `HoprPixSpec` produces, and a deposit pool can only settle
//! to the address type its own keypair produces. hoprd bundles the two into one feature so they
//! cannot disagree:
//!
//! | hoprd feature | pool | deposit address | status |
//! |---|---|---|---|
//! | `strategy-pix-test` | `NonAnonymousDepositPool` | Ethereum `Address` | implemented — **what this test needs** |
//! | `strategy-pix-curvy` | `CurvyDepositPool` | `BjjPublicKey` | stub, methods panic — production's eventual choice |
//!
//! They are mutually exclusive and `hoprd::strategy` rejects both with a `compile_error!` —
//! `hopr-strategy` itself compiles both pools and lets the call site choose, since features are
//! additive and two consumers in one graph may each want a different one. *Neither* is in
//! hoprd's `default`: Cargo unifies features across a workspace build, and a default pairing
//! would collide with the one this crate selects. A plain
//! `cargo build --release -p hoprd` therefore produces a binary with no PIX at all — one that
//! refuses to start against a config carrying a `Pix` strategy stanza, rather than silently
//! running without the strategy it was asked for.
//!
//! Either way the binary is wrong for this test in a way that costs a full bootstrap to
//! discover, which is why `pix-demo.sh` checks it before starting the cluster.
//!
//! Two separate builds are involved and nothing links them: this crate's own
//! `strategy-pix-test` (in `localcluster/Cargo.toml`) selects the pool the *harness*
//! compiles against, while `HOPRD_BIN` is a *prebuilt binary* carrying whichever pool it was
//! built with. When `CurvyDepositPool` is implemented and this test moves to Baby JubJub, both
//! have to change together.
//!
//! Each test must be run individually — see [`common`] for details.
//!
//! ```bash
//! export HOPRD_BIN=$(pwd)/target/release/hoprd
//! nix develop -c cargo nextest run -p hoprd-localcluster --test session_pix_soak \
//!   --run-ignored ignored-only -j 1
//! ```

mod common;

use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    time::{Duration, Instant},
};

use anyhow::Context;
use common::{
    Cluster, ClusterSpec,
    pix::{MAX_PLAUSIBLE_CYCLES, completed_cycles},
    ports,
};
use hopr_lib::api::types::primitive::prelude::HoprBalance;
use hoprd_localcluster::{client_helper, identity};
use tokio::net::UdpSocket;

const NUM_NODES: usize = 4;
const ENTRY: usize = 0;
/// Both are candidates for every packet — see the module docs.
const RELAYS: [usize; 2] = [1, 2];
const EXIT: usize = 3;
/// PIX requires at least one intermediate relay: the share encryption key comes from the
/// first relayer's acknowledgement.
///
/// One hop, but not one relay — the planner redraws it per packet, so which relayer supplies
/// that key changes constantly. PIX is built for that: the Exit files each pending share under
/// the peer it sent the reply to (`insert_encrypted_share(peer, ack_challenge, share)`) and the
/// awaiting-acks structure is a per-peer cache, so an acknowledgement from either relay resolves
/// its own share and nothing is shared between them.
///
/// `Hops(1)` also cannot degenerate into the direct path, even though the full mesh gives the
/// Entry a channel straight to the Exit: the selector searches for paths of exactly `hops + 1`
/// edges. That is already load-bearing today — the 3-node version of this test had the same
/// direct channel and never routed around its relay.
const HOPS: u64 = 1;

// ── SSA geometry ────────────────────────────────────────────────────────────────
//
// Sized to the packet rate, not chosen for its own sake — see the module docs. The
// product with [`PIX_SHARES`] is separately bounded at `4 × 8192 × 64` upstream
// (`validate_pix_dimension_product`); 128 × 108 = 13 824 uses well under a percent of
// that. Validation allows `num_ssa_parts` 8..=16192 and `ssa_part_size` 2..=255.
//
// The `u8`s are the upstream bound rather than a local choice: the threshold and the
// surplus are one byte each of the negotiated `PixParams` word, so a value above 255 is
// unrepresentable on the wire and is now a compile error here rather than a rejected
// Session.
//
// Split as polys rather than shares because `num_ssa_parts` is documented upstream as
// scaling with CPU parallelism while `ssa_part_size` does not.
const PIX_POLYS: u16 = 128;
const PIX_SHARES: u8 = 108;
/// Emitted beyond the threshold, per polynomial — the production ratio of `shares / 2`.
///
/// The generator's budget is finite: `shares + additional` per polynomial and no more, so
/// a polynomial that loses more than `additional` of its shares can never reach threshold
/// and the SSA stalls for good. `session_pix` can afford 2 because it only moves a few
/// dozen packets; at tens of thousands, a few percent loss would exhaust that on nearly
/// every polynomial.
///
/// It is delivered in every cycle whether or not anything is lost, and since it travels to
/// the Exit as part of the negotiated `PixParams` it is also *priced*: the quota counts it
/// and the deposit pays for it. So raising it lengthens a cycle — here a benefit, since a
/// cycle has to outlast a deposit — and costs money in proportion, which is the way round
/// it should be.
///
/// An absolute count upstream, not a ratio, so it has to move with [`PIX_SHARES`] to keep
/// the 1.5× surplus factor.
const PIX_ADDITIONAL_SHARES: u8 = 54;

/// Datagrams per second in each direction.
///
/// The forward rate is not free: it is only sustainable while [`response_buffer`] scales with
/// it, because share delivery and the echo spend the same return SURBs. Raised against an
/// unscaled buffer it collapses — at 250 → 1000 the echo went from 47 MB to 124 KB and the run
/// got *longer*, the Exit having spent its whole budget on shares with none left to reply with.
///
/// Measured ladder, three cycles per rung, everything else derived:
///
/// | requested | achieved | loss | buffer (% of ceiling) | cycle |
/// |---|---|---|---|---|
/// | 1000 | 1000 | 0% | 18 368 (28%) | 33 s |
/// | 2000 | 1997 | 0% | 26 368 (40%) | 21 s |
/// | 3000 | 2998 | 0% | 34 368 (52%) | 16 s |
/// | **4000** | **3992** | **0%** | **42 368 (64%)** | **14 s** |
/// | 5000 | 4990 | 0.3% | 50 368 (76%) | 14 s |
/// | 6000 | 6010 | 0% | 58 368 (88%) | 11 s |
/// | 6500 | 5660 | 6.3% | 62 368 (94%) | 12 s |
///
/// So the flow sustains **6000 datagrams/s each way** and saturates by 6500. 4000 is committed
/// rather than 6000 on three grounds, all of which are about a demo run rather than a probe:
/// the buffer keeps real overshoot headroom below the ring-buffer ceiling, where the penalty is
/// silent permanent share loss; the cycle keeps 3-7× the deposit round trip instead of 2×; and
/// there is 50% of margin to the knee, so a loaded machine degrades the number on screen rather
/// than the `sent == echoed` invariant behind it. Nowhere near a link limit either way —
/// `session_udp` does ~4300/s over the same loopback with no SURB machinery at all.
const DEFAULT_PACKET_RATE: u64 = 4000;

/// Payload per datagram, and the least obvious constant in this file.
///
/// A packet's payload is `HoprPacket::PAYLOAD_SIZE` = 1038 B and a `HoprSurb` is 401 B, so
/// what a forward packet has room to carry alongside its data is:
///
/// | payload | SURBs carried |
/// |---|---|
/// | ≤ 236 B | 2 (`MAX_SURBS_IN_PACKET`) |
/// | 237–637 B | 1 |
/// | ≥ 638 B | 0 — SURBs need dedicated keep-alive packets |
///
/// 900 B lands in the last row deliberately, so **the balancer is the only source of
/// SURBs**. That looks like a handicap and measuring it suggested as much: at 900 B the
/// Exit replied at ~60/s against a 200/s forward rate. But 200 B, which piggybacks two
/// SURBs on every datagram, was worse. Supply then runs at twice the forward rate against
/// a demand of one SURB per reply, and the surplus is not free: a share is bound to a SURB
/// when the SURB is minted, so over-minting burns the SSA's emission budget into a buffer
/// the Exit is not draining. The buffer inflates, the pipeline delay grows with it, and
/// cycles that started at ~16 s had stretched to ~90 s by the sixth.
///
/// Keeping supply under closed-loop control is what makes the run stable. The reply rate
/// then tracks the balancer's target buffer — measured at roughly `target / 8` per second
/// across two runs (521 SURBs → 60/s, 1920 → 239/s), which is what [`SURB_RUNWAY_SECS`]
/// encodes and [`response_buffer`] inverts to get a buffer from a rate.
///
/// Note this also makes the payload a fixed 900 of the 1038 B a packet is *billed* for, so
/// the datagram rate and the HOPR packet rate are one to one in the forward direction —
/// which is what lets the demo quote a packet rate at all.
const CHUNK_SIZE: usize = 900;

// ── Run bounds ──────────────────────────────────────────────────────────────────

/// Cycles the default float pays for, and so the length of a default run.
///
/// The float is set to *exactly* this many deposits, which makes the closing assertions
/// exact: the Entry should spend all of it, and all but the last SSA or two should end up
/// swept into the Exit's Safe.
///
/// Ten fits again at [`DEFAULT_PACKET_RATE`] of 4000. A cycle is bounded by packets rather than
/// seconds, so four times the rate is roughly a third of the cycle time (~13 s against ~33 s at
/// 1000/s), and ten of them land inside [`DEFAULT_RUN_BUDGET`] where at 1000/s only five did.
const DEFAULT_FUNDED_CYCLES: u64 = 10;

/// Round-trip payload a run must move for "multi-megabyte" to mean anything. Far under
/// what the default float implies (~140 MB at the committed rate), so it fails only on a
/// real collapse in throughput rather than on ordinary variance — which is exactly how the
/// first 1000/s attempt was caught, at 124 KB.
const MIN_TOTAL_BYTES: u64 = 4_000_000;
/// Share of paid-for return packets that must be observed completing the round trip.
/// The shortfall is packet loss downstream of the Exit, which still consumed the SURB.
const MIN_DELIVERED_PERCENT: u32 = 80;
/// Least share of the relayed packets each of the two [`RELAYS`] must carry.
///
/// Not 50: the forward path is drawn per packet by weighted random selection over the planner's
/// candidates — weighted by latency, channel capacity and ack rate — and return paths come from
/// tempered weights with a share of uniform exploration on top. So an even split is what two
/// identical loopback relays tend to, not what any single run is owed, and a loaded machine skews
/// it further.
///
/// The floor is set low because it is not measuring the split. It is measuring that a relay is
/// carrying traffic *at all*, which is the failure a 3-node run cannot see: an unopened channel,
/// a node that never announced, or a graph that only ever found one path leaves every other
/// assertion here passing. Raise it only against measurements from several runs.
const MIN_RELAY_SHARE_PERCENT: u64 = 15;
/// SSAs that may legitimately be funded but not yet swept when the run ends.
///
/// The Exit requests the next SSA at the early-recovery threshold, so one is normally in
/// flight at any instant; at the end of the run the final one is cut short by the kill
/// switch before its shares complete.
const MAX_SSAS_IN_FLIGHT: u64 = 2;

/// Worst-case cycle duration used to derive the safety deadline. Deliberately far above
/// the [`TARGET_CYCLE_SECS`] a cycle is sized for: the deadline exists to stop a wedged run,
/// and tripping it on a merely slow one buries the real numbers under a timeout.
const MAX_SECS_PER_CYCLE: u64 = 60;
/// Added to the safety deadline for the tail: the failing deposit's retry chain plus the
/// kill-switch fuse.
const KILL_SWITCH_TAIL: Duration = Duration::from_secs(180);
/// After the kill switch trips, how long to keep polling for in-flight sweeps to land.
const SETTLE_TIMEOUT: Duration = Duration::from_secs(60);
/// Wall-clock ceiling for a run at the default float, bootstrap included.
///
/// Only enforced when `HOPRD_PIX_SOAK_FLOAT` is unset: runtime scales with the float by
/// design, so a deliberately larger one is expected to take longer.
///
/// The traffic phase does not care how many relays there are — it is the same aggregate rate over
/// the same geometry — but the bootstrap does, and it is the larger half of a default run. The
/// fourth node adds a serial round of identity provisioning (Safe deployment, Safe registration,
/// pre-announce and two PIX top-ups, all sequential per node) and the full mesh goes from 6
/// channel opens to 12, also sequential: measured 244 s to channels-ready against 190 s at three
/// nodes, for a 407 s run against the 350 s one this replaces.
///
/// So 9 minutes is 130 s of headroom over a measured run, a little more than the 70 s the old
/// budget left. Bootstrap is where a loaded machine costs the most and it is now the part that
/// grew, which is what the extra margin is for.
const DEFAULT_RUN_BUDGET: Duration = Duration::from_secs(9 * 60);

const WAIT_TIMEOUT: Duration = Duration::from_secs(120);
const SETUP_TIMEOUT: Duration = Duration::from_secs(300);
const REPORT_INTERVAL: Duration = Duration::from_secs(5);

// ── Money ───────────────────────────────────────────────────────────────────────

/// At the committed ~21.52 MB quota this makes a deposit ~21.52 wxHOPR.
///
/// Held constant as the geometry scales, so the deposit tracks the data rather than staying
/// put — which is the point being demonstrated. [`MAX_SSA_ALLOCATION`] has to stay above it.
///
/// PIX pricing is its own model, unrelated to channel ticket pricing; it only has to sit
/// above the relay price re-counted per byte, which this does by roughly an order of
/// magnitude.
const PRICE_PER_BYTE: &str = "0.000001 wxHOPR";
/// Ceiling on one deposit. Below `price_per_byte × quota` the strategy refuses to deposit
/// at all, which would end the run on the first cycle instead of on the last. The quota
/// grows with the packet rate, so this has to leave room above it — at the committed
/// geometry a deposit is ~21.52 wxHOPR.
///
/// It moved 20 → 30 when the surplus was priced into the quota upstream: the dimensions did
/// not change, but what they cost went up by the 1.5× surplus factor, and 20 had become a
/// ceiling *below* the deposit. The symptom is unmistakable and immediate — every deposit
/// refused, `deposits_failed` climbing from the first cycle, and the kill switch ending the
/// run before any money moves.
const MAX_SSA_ALLOCATION: &str = "30 wxHOPR";
const GAS_XDAI_PER_SWEEP: &str = "0.01 xdai";
/// Per channel, out of each Safe's 1000 wxHOPR across **three** outgoing channels. Tens of
/// megabytes issue a lot of tickets, and a channel draining before the deposit float does
/// would stall packet flow and end the run for the wrong reason.
///
/// Down from 400 because a four-node full mesh gives every node one more outgoing channel to
/// fund: 3 × 300 = 900 of the 1000 wxHOPR each Safe holds. Against the traffic it is a *rise*
/// rather than a cut — the forward leg now splits across two relays, so a channel carries about
/// half the tickets it used to for three quarters of the stake.
const CHANNEL_STAKE: &str = "300 wxHOPR";

// ── Exit deadlines ──────────────────────────────────────────────────────────────

/// Together these arm the PIX kill switch with a 23 s fuse, against the 80 s product default.
///
/// **Sized for the bloklid-anvil container, not for a real chain**, and deliberately not shared
/// with [`session_pix`](../session_pix.rs), which keeps the product defaults so that one of the
/// two tests exercises the shipped configuration end to end. The product values live upstream in
/// `IncomingSessionPixConfig` and are untouched by either.
///
/// Here the fuse is not a safety net but part of the expected path — every run pays it once, at
/// the end, and at [`DEFAULT_PACKET_RATE`] it is the most expensive thing in the run after the
/// traffic itself. Measured across ten cycles at the committed geometry:
///
/// | component | measured | budget |
/// |---|---|---|
/// | `SsaCommit` delivery | 0.025–0.035 s | 3 s |
/// | on-chain settlement | 2.03–9.85 s (median 7.0) | 20 s |
///
/// Delivery is the one that was badly wrong: 15 s for a 30 ms operation. 3 s is still a hundred
/// times over. The deposit wait keeps 2× the observed worst case rather than the 3× it had,
/// because the 5× spread on *instant* mining is the thing the budget buys, not the median — and
/// the error is asymmetric, since firing early kills a Session that would have paid and strands
/// its deposit at a stealth address the Entry cannot spend from.
///
/// The cost of the fuse is bytes, not seconds, and bytes scale with the rate while the fuse does
/// not: 45 s cost ~10 MB at the original 250 datagrams/s and 145 MB at 4000. So this has to be
/// revisited whenever [`DEFAULT_PACKET_RATE`] moves, which is not otherwise obvious.
const MAX_SSA_DELIVERY_TIME: Duration = Duration::from_secs(3);
const MAX_DEPOSIT_WAIT: Duration = Duration::from_secs(20);
/// Also fixes the Exit's deposit poll cadence at a tenth of this.
const MAX_DEPOSIT_TRACKING_TIME: Duration = Duration::from_secs(20);

/// Per-SSA quota in bytes implied by the dimensions above.
///
/// Mirrors [`identity::PixSettings::quota_per_ssa`], which cannot be used here because the
/// settings need the float, the float is derived from the per-cycle deposit, and that is
/// derived from the quota.
///
/// Every emitted share is charged for, surplus included, so this is exactly
/// [`emissions_per_ssa`] priced at the full packet payload — the Exit is paid for each
/// return packet it sends rather than for the subset that happened to be needed.
fn quota_bytes() -> u64 {
    emissions_per_ssa() * hopr_lib::exports::transport::PACKET_PAYLOAD_SIZE as u64
}

/// wxHOPR the Entry gets to spend on deposits, from `HOPRD_PIX_SOAK_FLOAT` or
/// [`DEFAULT_FUNDED_CYCLES`] cycles' worth.
fn deposit_float(per_cycle: HoprBalance) -> anyhow::Result<HoprBalance> {
    match std::env::var("HOPRD_PIX_SOAK_FLOAT") {
        Ok(raw) => raw
            .trim()
            .parse()
            .with_context(|| format!("HOPRD_PIX_SOAK_FLOAT must be a wxHOPR amount, got {raw:?}")),
        Err(_) => Ok(per_cycle * DEFAULT_FUNDED_CYCLES),
    }
}

/// Datagrams per second the sender paces at, from `HOPRD_PIX_SOAK_RATE` or
/// [`DEFAULT_PACKET_RATE`].
///
/// A rejected override is announced rather than swallowed. The rate is not cosmetic — it sizes
/// [`surb_buffer_target`], [`response_buffer`], and every figure in the run's own measurement
/// table — so a run that quietly ignored `HOPRD_PIX_SOAK_RATE=4o00` would print a table
/// describing something other than what was asked for.
///
/// Called from [`surb_buffer_target`], which is infallible and two layers below the test body,
/// so this warns and falls back instead of returning a `Result`. The ceiling check in the test
/// body is the hard stop for a rate that is *valid but unsafe*.
fn packet_rate() -> u64 {
    match std::env::var("HOPRD_PIX_SOAK_RATE") {
        Ok(raw) => match raw.trim().parse::<u64>() {
            Ok(rate) if rate > 0 => rate,
            Ok(_) => {
                tracing::warn!(
                    "HOPRD_PIX_SOAK_RATE must be greater than zero, got {raw:?}; \
                     falling back to {DEFAULT_PACKET_RATE}"
                );
                DEFAULT_PACKET_RATE
            }
            Err(e) => {
                tracing::warn!(
                    "HOPRD_PIX_SOAK_RATE must be a positive integer, got {raw:?} ({e}); \
                     falling back to {DEFAULT_PACKET_RATE}"
                );
                DEFAULT_PACKET_RATE
            }
        },
        Err(_) => DEFAULT_PACKET_RATE,
    }
}

/// `budget` is what actually ends the run: the strategy refuses the deposit that would cross it,
/// which starves the Session exactly as an empty account used to. `safe_deposit_float` is sized
/// to cover it with room to spare, so the Safe's balance is never what binds — see
/// [`identity::PixSettings::max_spend_per_window`] for why the run can no longer be bounded by
/// balance alone.
fn pix_settings(
    safe_deposit_float: HoprBalance,
    budget: HoprBalance,
) -> anyhow::Result<identity::PixSettings> {
    Ok(identity::PixSettings {
        num_ssa_parts: PIX_POLYS as usize,
        ssa_part_size: PIX_SHARES as usize,
        additional_shares: PIX_ADDITIONAL_SHARES as usize,
        // Our quota sits far below the production window's lower bound, so the window has
        // to be opened up. The upper bound is left well clear of the committed quota so a
        // `HOPRD_PIX_SOAK_RATE` override does not have to move it too — the Exit rejects
        // the Session outright if the offered quota falls outside this.
        quota_range_min: 0,
        quota_range_max: 64 * 1024 * 1024,
        max_ssa_delivery_time: MAX_SSA_DELIVERY_TIME,
        max_deposit_wait: MAX_DEPOSIT_WAIT,
        enforce_on_nodes: vec![EXIT],
        // Unbatched, with hoprd's own Entry cap. The soak's subject is a long run at a fixed
        // per-cycle cost, and batching would multiply the cycles in flight, the kill-switch
        // window and the unincentivized service fronted before the first deposit — all of which
        // this file's budget and pacing arithmetic is written against one cycle at a time.
        // `session_pix.rs` is where the batched exchange is exercised.
        ssas_per_request: 1,
        max_ssas_per_request: 2,
        safe_deposit_float,
        // Settlement knobs. These used to travel as environment variables; they are written
        // into the generated node config's `Pix` strategy stanza now.
        price_per_byte: PRICE_PER_BYTE.parse().context("parsing price per byte")?,
        max_ssa_allocation: MAX_SSA_ALLOCATION
            .parse()
            .context("parsing max SSA allocation")?,
        max_spend_per_window: budget,
        // Far past `DEFAULT_RUN_BUDGET` and past any plausible `HOPRD_PIX_SOAK_FLOAT` override.
        // A window that rolled mid-run would refill the budget and the run would never end.
        spend_window: Duration::from_secs(24 * 3600),
        max_deposit_tracking_time: MAX_DEPOSIT_TRACKING_TIME,
        gas_xdai_per_sweep: GAS_XDAI_PER_SWEEP.parse().context("parsing sweep gas")?,
    })
}

/// Emissions the generator produces per SSA, and so the packets a cycle takes to deliver.
fn emissions_per_ssa() -> u64 {
    PIX_POLYS as u64 * (PIX_SHARES as u64 + PIX_ADDITIONAL_SHARES as u64)
}

/// SURB balancer target: enough runway to keep both return streams fed, and no more.
///
/// hoprd converts this to `target_surb_buffer_size = bytes / SESSION_MTU`
/// (`rest-api::session`, `SessionConfig -> SurbBalancerConfig`), so it is expressed here
/// as a SURB count scaled back up rather than as a byte figure that happens to work out.
///
/// The Exit spends one return SURB per reply packet, and a reply carries *either* a share
/// *or* echoed data — one budget, two streams. So the buffer has to cover both:
/// `emissions_per_ssa()` shares plus a cycle's worth of echo at the forward rate. Sizing it
/// from the SSA alone is what broke at 1000/s: the share stream took the whole budget and the
/// echo starved to nothing. Sizing it *generously* costs cycle time instead — see
/// [`SURB_RUNWAY_SECS`] for the measurements. It is a floor to be met, not a budget to spend.
///
/// The resulting pipeline delay is `buffer / (shares + echo)` = [`SURB_RUNWAY_SECS`] seconds
/// whatever the scale, which has to stay well inside a cycle: at the committed geometry it is
/// about a third of one.
///
/// Must stay under `rb_capacity × 2/3` = 66 666 SURBs (`surb_buffer_target_ceiling` in
/// `hopr-transport`), because the balancer overshooting into a full ring buffer evicts the
/// oldest SURBs, and an evicted SURB is a permanently lost share rather than a wasted SURB.
fn response_buffer() -> String {
    const SESSION_MTU: u64 = 1020;
    format!("{} B", surb_buffer_target() * SESSION_MTU)
}

/// Runway the Exit's SURB buffer is sized to hold, and so the buffer / rate conversion.
///
/// Matches `balancer_minimum_surb_buffer_duration` (default 5 s), which is what shapes the
/// Exit's egress: it will not drain faster than its buffer represents that many seconds of
/// replies. Sized above the default because the balancer's own loop costs the difference —
/// measured at buffer/8 at 250 datagrams/s (521 SURBs → 60/s, 1920 → 239/s).
///
/// **This is a floor, not a throughput dial, and raising it costs cycle time.** Measured at
/// 1000 datagrams/s with everything else fixed:
///
/// | runway | buffer | cycle | share rate |
/// |---|---|---|---|
/// | 8 | 18 368 SURBs | 32 s | 648/s |
/// | 11 | 25 256 SURBs | 40 s | 518/s |
///
/// More buffer made cycles *longer*. A share is bound to its SURB when the SURB is minted and
/// the Exit spends the buffer roughly in order, so buffer depth is latency between generating
/// a share and delivering it — the share stream does not speed up to fill it. So the runway is
/// set to the smallest value that keeps the echo stream fed: below it the echo starves outright
/// (at 1920 SURBs against a 1000/s forward rate it collapsed to nothing), above it every cycle
/// pays for depth it does not use.
const SURB_RUNWAY_SECS: u64 = 8;

/// Return-path rate the buffer is provisioned against: the share stream plus the echo.
///
/// The share rate is `emissions_per_ssa() / cycle`, and the cycle is what
/// [`TARGET_CYCLE_SECS`] fixes, so it is derived rather than measured.
///
/// Adding the two is deliberate over-provisioning, not a claim that they are separate
/// packets — measurement says they are not. Over a 1000/s run the Exit sent 179 974 packets
/// against ~177 000 echo replies, so essentially every share travelled *on* a reply rather
/// than in a packet of its own, which is what "the Exit unlocks one share per SURB it spends
/// replying" means. Sizing against the sum therefore leaves roughly 2× headroom. That is the
/// direction to err in — the buffer only has to be a floor — and it is the value measured to
/// work; the tighter `SURB_RUNWAY_SECS * packet_rate()` has never been run.
fn surb_buffer_target() -> u64 {
    let share_rate = emissions_per_ssa() / TARGET_CYCLE_SECS;
    SURB_RUNWAY_SECS * (share_rate + packet_rate())
}

/// `rb_capacity × 2/3`, the ceiling [`response_buffer`]'s doc names — `surb_buffer_target_ceiling`
/// in `hopr-transport`, which is not a workspace member, so nothing clamps it on this side.
///
/// Crossing it is not a degradation: the balancer overshoots into a full ring buffer, the oldest
/// SURBs are evicted, and a share bound to an evicted SURB can never be delivered. Checked in the
/// test body rather than here, because [`surb_buffer_target`] is called from infallible helpers
/// and the point is to refuse the run before it starts, not to clamp silently.
///
/// With the committed geometry the ceiling is reached at a packet rate of about 7000/s, which is
/// the real bound on a `HOPRD_PIX_SOAK_RATE` override.
const SURB_BUFFER_CEILING: u64 = 66_666;

/// Cycle length the geometry is *sized* for. The cycle actually achieved is about twice it.
///
/// Not a timeout — the cycle is bounded by packets, not seconds — but the figure the SSA width
/// and the SURB buffer are both derived from, and the one number that has to stay comfortably
/// above a deposit round trip (measured 2.2 s for the Exit to observe one, 5.0 s for the Entry
/// to confirm). Below that the Exit reconstructs the key before the money lands,
/// `sweep_recovered` finds a zero balance and logs "already swept", and the deposit is
/// stranded for good.
///
/// It is a sizing figure, not a prediction, and how far the achieved cycle lands from it depends
/// on the rate: ~33 s at 1000 datagrams/s, ~13 s at 4000. The share stream does not scale
/// linearly with the reply rate — shares per reply fell from ~0.7 at 1000/s to ~0.3 at 5000 —
/// so the modelled share rate of `emissions / this` is an over-estimate at low rates and an
/// under-estimate at high ones. Both directions are safe: the buffer it sizes only has to be a
/// floor, and the error is a factor of two either way against a ceiling four times clear.
///
/// The consequence worth knowing is that the Exit is billed for less than it delivers, by
/// `shares_per_reply / (1 + additional/threshold)` — about half here, and the same under-billing
/// the test had at 250/s. Closing it means either pricing against delivered bytes or finding the
/// fixed point between buffer depth and share rate; neither is needed to demonstrate throughput.
const TARGET_CYCLE_SECS: u64 = 16;

/// A UDP echo server for the Exit to forward Session payloads to, making Exit → Entry
/// volume mirror Entry → Exit volume.
/// Deposits `float` can pay for, rounding down.
fn affordable_cycles(float: HoprBalance, per_cycle: HoprBalance) -> u64 {
    if per_cycle.is_zero() {
        return 0;
    }
    let n = float.amount() / per_cycle.amount();
    if n > MAX_PLAUSIBLE_CYCLES.into() {
        MAX_PLAUSIBLE_CYCLES
    } else {
        n.as_u64()
    }
}

/// One node's `/metrics`, reduced to the counters this test reports on.
#[derive(Clone, Copy, Debug, Default)]
struct NodeMetrics {
    // PIX lifecycle, in the order an SSA cycle passes through it.
    deposits: u64,
    deposits_rejected: u64,
    deposits_failed: u64,
    deposits_confirmed: u64,
    deposits_timed_out: u64,
    keys_recovered: u64,
    sweeps: u64,
    // Node-wide packet counts. Per-session counters are not here: hoprd strips every
    // `hopr_session_*` series from this endpoint and exports them over OTLP instead.
    packets_sent: u64,
    packets_received: u64,
    /// Relayed on behalf of someone else, both directions of the Session together. Only a relay
    /// reports any, which is what makes it the per-relay traffic share.
    packets_forwarded: u64,
}

impl NodeMetrics {
    /// Lenient read. A node that cannot be scraped reports zeroes rather than failing the run:
    /// this feeds a progress display, and the authoritative signal is the on-chain balance.
    ///
    /// Only for the reporting loop. The closing assertions use [`try_scrape`](Self::try_scrape),
    /// because there the same zeroes are indistinguishable from a node that genuinely did
    /// nothing, and several of those assertions are satisfied by zero.
    async fn scrape(api: &client_helper::HoprdApiClient) -> Self {
        Self::try_scrape(api).await.unwrap_or_default()
    }

    /// Strict read, for anything an assertion depends on.
    async fn try_scrape(api: &client_helper::HoprdApiClient) -> anyhow::Result<Self> {
        let m = api.metrics().await.context("scraping node metrics")?;
        let tracking = |outcome: &str| {
            m.sum_where(
                "hopr_strategy_pix_deposit_tracking_total",
                &format!(r#"outcome="{outcome}""#),
            ) as u64
        };
        let packets =
            |kind: &str| m.sum_where("hopr_packets_count", &format!(r#"type="{kind}""#)) as u64;
        Ok(Self {
            deposits: m.sum("hopr_strategy_pix_deposits_total") as u64,
            deposits_rejected: m.sum("hopr_strategy_pix_deposits_rejected_total") as u64,
            deposits_failed: m.sum("hopr_strategy_pix_deposits_failed_total") as u64,
            deposits_confirmed: tracking("confirmed"),
            deposits_timed_out: tracking("timeout"),
            keys_recovered: m.sum("hopr_strategy_pix_keys_recovered_total") as u64,
            sweeps: m.sum("hopr_strategy_pix_sweeps_total") as u64,
            packets_sent: packets("sent"),
            packets_received: packets("received"),
            packets_forwarded: packets("forwarded"),
        })
    }
}

/// Packets each relay forwarded since `before`, in [`RELAYS`] order.
///
/// Measured as a delta because bootstrap is not quiet: probes are relayed too, and 1-hop probe
/// paths keep running for the whole life of the cluster. They are a rounding error against a
/// traffic phase of millions of packets, but the baseline is free to subtract.
async fn relay_forwarded(nodes: &[client_helper::NodeProcess], before: &[NodeMetrics]) -> Vec<u64> {
    let mut out = Vec::with_capacity(RELAYS.len());
    for (relay, before) in RELAYS.iter().zip(before) {
        let now = NodeMetrics::scrape(&nodes[*relay].api).await;
        out.push(
            now.packets_forwarded
                .saturating_sub(before.packets_forwarded),
        );
    }
    out
}

/// Each relay's percentage of everything the relays forwarded between them.
fn relay_shares(forwarded: &[u64]) -> Vec<u64> {
    let total: u64 = forwarded.iter().sum();
    forwarded
        .iter()
        .map(|n| (n * 100).checked_div(total).unwrap_or(0))
        .collect()
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires external chain container and hoprd binary \u{2014} run explicitly, not in CI"]
async fn localcluster_pix_session_runs_until_the_entry_cannot_deposit() -> anyhow::Result<()> {
    common::init_tracing();
    let rate = packet_rate();
    let quota = quota_bytes();
    let price_per_byte: HoprBalance = PRICE_PER_BYTE.parse().context("parsing price per byte")?;
    let per_cycle = price_per_byte * quota;
    let float_overridden = std::env::var("HOPRD_PIX_SOAK_FLOAT").is_ok();
    let float = deposit_float(per_cycle)?;
    let funded_cycles = affordable_cycles(float, per_cycle);
    anyhow::ensure!(
        funded_cycles > 0,
        "the float {float} does not cover even one {per_cycle} deposit"
    );
    // Refuse a rate whose buffer would not fit, rather than running it and reporting the damage
    // hundreds of lines later as an unexplained stranded deposit. Evicted SURBs lose their shares
    // permanently, so there is no partial result worth having here.
    let surb_buffer = surb_buffer_target();
    anyhow::ensure!(
        surb_buffer < SURB_BUFFER_CEILING,
        "a rate of {rate} datagrams/s needs a {surb_buffer} SURB buffer, over the \
         {SURB_BUFFER_CEILING} ring-buffer ceiling; the balancer would evict SURBs and \
         permanently lose their shares"
    );

    let t0 = Instant::now();

    // `ssa_polys`/`ssa_shares` are deliberately not named `polys`/`shares`: `pix-demo.sh`
    // greps this banner for `<name>=<number>` and takes the first match in the whole log, so
    // a field name that could occur in any other line would be read out of that line instead.
    tracing::info!(
        rate, quota, %price_per_byte, %per_cycle, %float, funded_cycles,
        ssa_polys = PIX_POLYS,
        ssa_shares = PIX_SHARES,
        ssa_surplus = PIX_ADDITIONAL_SHARES,
        emissions_per_ssa = emissions_per_ssa(),
        response_buffer = %response_buffer(),
        surb_buffer_target = surb_buffer_target(),
        // The cycle is `emissions / share_rate`, and the share rate is what
        // `TARGET_CYCLE_SECS` fixes — so this is the geometry's own target, restated. It is
        // logged so a run whose measured cadence drifts from it is obvious in the log.
        est_cycle_secs = TARGET_CYCLE_SECS,
        est_share_rate = emissions_per_ssa() / TARGET_CYCLE_SECS,
        "PIX geometry: {PIX_POLYS} polys x ({PIX_SHARES} + {PIX_ADDITIONAL_SHARES}) shares = \
         {quota} B per SSA at {price_per_byte}/B = {per_cycle} per deposit; {rate} datagrams/s \
         each way, the run ends when {funded_cycles} deposits have spent the budget"
    );

    let cluster = Cluster::start(ClusterSpec {
        // Four: an Entry, two relays and an Exit. `ENTRY`/`RELAYS`/`EXIT` above and
        // `pix-demo.sh`'s own index→role mapping both assume this count.
        num_nodes: NUM_NODES,
        // AutoRedeeming stays off: redeemed tickets also credit the Safe in wxHOPR, which
        // would make the closing balance assertions ambiguous.
        strategies: identity::StrategySet {
            auto_redeeming: false,
            channel_lifecycle: false,
        },
        strategy_execution_interval: Some(Duration::from_secs(600)),
        // The same figure twice, deliberately: the Safe is funded with the float *and* the
        // strategy is budgeted for it. The budget is what binds — the Safe additionally holds
        // whatever the channel stakes left behind, so its balance alone would run the Entry
        // several cycles past `funded_cycles`.
        pix: Some(pix_settings(float, float)?),
        logs_to: Some("/tmp/pix-soak-logs"),
        ..ClusterSpec::new(ports::SESSION_PIX_SOAK)
    })
    .await?;
    cluster.wait_ready(WAIT_TIMEOUT).await?;
    cluster.open_channels(CHANNEL_STAKE, SETUP_TIMEOUT).await?;
    cluster.wait_channels(SETUP_TIMEOUT).await?;
    cluster.wait_reachable(SETUP_TIMEOUT).await?;
    tracing::info!("channels ready after {:?}", t0.elapsed());
    let log_dir = cluster.log_dir().to_path_buf();

    let echo_port = common::echo_server().await?;
    let entry = cluster.node(ENTRY);
    let exit_node = cluster.node(EXIT);
    let exit_addr = exit_node
        .address
        .as_ref()
        .context("exit node address unresolved")?;
    let target = format!("127.0.0.1:{echo_port}");

    for node in cluster.nodes() {
        tracing::info!(
            "node{} metrics: http://127.0.0.1:{}/metrics",
            node.id,
            node.api_port
        );
    }
    tracing::info!(
        "per-session hopr_session_* counters go to OTLP only: {} (the /metrics endpoint \
         strips them)",
        std::env::var("HOPRD_OTLP_ENDPOINT")
            .unwrap_or_else(|_| "http://localhost:4318".to_string())
    );
    // Interrupting a long run kills the process without unwinding, so the guard that copies
    // these out never drops — say where they are while the run is still live.
    tracing::info!("node logs: {}", log_dir.display());

    // Snapshot after channel funding, so the stakes are already out of the Safes and the
    // only movement left is PIX.
    //
    // Both sides move wxHOPR through their *Safe*: `hopr-types` 4.0.0 routes
    // `SafePayloadGenerator::transfer` through the Safe module, so a deposit debits the Entry's
    // Safe however it is signed, and a sweep credits the Exit's. Until then the Entry's transfer
    // was direct and left its node account, which is what this used to sample.
    let entry_before = entry.api.balances().await.context("entry balances")?;
    let exit_before = exit_node.api.balances().await.context("exit balances")?;
    let entry_metrics_before = NodeMetrics::scrape(&entry.api).await;
    let exit_metrics_before = NodeMetrics::scrape(&exit_node.api).await;
    let mut relays_metrics_before = Vec::with_capacity(RELAYS.len());
    for relay in RELAYS {
        relays_metrics_before.push(NodeMetrics::scrape(&cluster.node(relay).api).await);
    }
    // The budget is what ends the run, so what matters is that the Safe can cover it — not that
    // it holds some exact figure. It holds the float plus whatever the channel stakes left, and
    // an equality here would be asserting the arithmetic of the stakes rather than anything
    // about PIX.
    assert!(
        entry_before.safe_hopr >= float,
        "the Entry Safe holds {} against a {float} deposit budget, so it would run dry before \
         the budget bound and the run would end for the wrong reason",
        entry_before.safe_hopr
    );
    tracing::info!(
        entry_safe_hopr = %entry_before.safe_hopr,
        exit_safe_hopr = %exit_before.safe_hopr,
        exit_safe_native = %exit_before.safe_native,
        "balances before the Session"
    );

    let (ip, port) = entry
        .api
        .open_session(client_helper::OpenSessionRequest {
            protocol: "udp",
            destination: exit_addr,
            target: &target,
            hops: HOPS,
            capabilities: Some(vec![
                hoprd_api_client::types::SessionCapability::Segmentation,
                hoprd_api_client::types::SessionCapability::NoDelay,
                hoprd_api_client::types::SessionCapability::UsePix,
            ]),
            response_buffer: Some(response_buffer()),
            max_surb_upstream: Some("50 Mb/s".to_string()),
            // Must match this node's own generator dimensions or the Session is refused.
            pix_ssa_quota: Some(hoprd_api_client::types::PixSsaQuota {
                polys_per_ssa: PIX_POLYS,
                shares_per_poly: PIX_SHARES,
                surplus_shares: PIX_ADDITIONAL_SHARES,
            }),
        })
        .await
        .context("opening PIX session")?;
    tracing::info!(
        "PIX session listening on {ip}:{port} (elapsed={:?})",
        t0.elapsed()
    );

    // ── Background bidirectional traffic ────────────────────────────────────────
    let sock = UdpSocket::bind("127.0.0.1:0")
        .await
        .context("binding session client socket")?;
    sock.connect(format!("{ip}:{port}"))
        .await
        .context("connecting to session listener")?;
    let sock = Arc::new(sock);

    let stop = Arc::new(AtomicBool::new(false));
    let sent = Arc::new(AtomicU64::new(0));
    let echoed = Arc::new(AtomicU64::new(0));

    // `tokio::time::interval` panics on a zero period, which an absurd rate override
    // would otherwise produce.
    let send_interval = Duration::from_micros(1_000_000 / rate).max(Duration::from_micros(1));
    let payload: Vec<u8> = (0..CHUNK_SIZE).map(|i| (i % 256) as u8).collect();
    let sender = tokio::spawn({
        let (sock, stop, sent, payload) =
            (sock.clone(), stop.clone(), sent.clone(), payload.clone());
        async move {
            // Default `MissedTickBehavior::Burst` keeps the long-run average on target
            // when a tick is late, which matters because the cycle length is derived from
            // this rate.
            let mut ticker = tokio::time::interval(send_interval);
            while !stop.load(Ordering::Acquire) {
                ticker.tick().await;
                // Once the kill switch closes the Session the listener goes away and this
                // starts erroring; that is the expected end of the run, not a fault.
                if sock.send(&payload).await.is_err() {
                    break;
                }
                sent.fetch_add(1, Ordering::Release);
            }
        }
    });
    let receiver = tokio::spawn({
        let (sock, stop, echoed) = (sock.clone(), stop.clone(), echoed.clone());
        async move {
            let mut buf = vec![0u8; 65535];
            while !stop.load(Ordering::Acquire) {
                match tokio::time::timeout(Duration::from_secs(30), sock.recv(&mut buf)).await {
                    Ok(Ok(n)) if buf[..n] == payload[..] => {
                        echoed.fetch_add(1, Ordering::Release);
                    }
                    // A short or corrupted echo is a Session-layer failure, but the
                    // balance assertions below are the real verdict — just note it.
                    Ok(Ok(n)) => tracing::warn!(n, "unexpected echo payload"),
                    Ok(Err(error)) => {
                        tracing::debug!(%error, "session socket recv failed");
                        break;
                    }
                    Err(_) => tracing::warn!("no echo for 30s"),
                }
            }
        }
    });

    // ── Run until the Entry cannot afford the next deposit ──────────────────────
    //
    // The Exit's tracker timing out is the definitive end signal: it means a deposit it
    // was promised never arrived, which is what arms the kill switch. The deadline below
    // is a safety net for the case where that never happens.
    let deadline =
        Instant::now() + Duration::from_secs(funded_cycles * MAX_SECS_PER_CYCLE) + KILL_SWITCH_TAIL;
    let traffic_started = Instant::now();
    let mut recovered;
    let mut exit_metrics;
    let mut entry_metrics;
    // Carried across iterations: a delta that is momentarily not a whole multiple (a
    // sweep landing mid-poll) keeps the last good reading rather than reporting zero.
    let mut cycles = 0u64;
    let mut killed = false;
    loop {
        tokio::time::sleep(REPORT_INTERVAL).await;

        let exit_now = exit_node
            .api
            .balances()
            .await
            .context("polling exit balances")?;
        let entry_now = entry
            .api
            .balances()
            .await
            .context("polling entry balances")?;
        recovered = exit_now.safe_hopr - exit_before.safe_hopr;
        cycles = completed_cycles(recovered, per_cycle).unwrap_or(cycles);
        entry_metrics = NodeMetrics::scrape(&entry.api).await;
        exit_metrics = NodeMetrics::scrape(&exit_node.api).await;
        let relayed = relay_forwarded(cluster.nodes(), &relays_metrics_before).await;

        let sent_n = sent.load(Ordering::Acquire);
        let echoed_n = echoed.load(Ordering::Acquire);
        let secs = traffic_started.elapsed().as_secs_f64().max(1.0);
        tracing::info!(
            elapsed = ?t0.elapsed(),
            cycles,
            funded_cycles,
            %recovered,
            entry_safe = %entry_now.safe_hopr,
            sent_mb = sent_n * CHUNK_SIZE as u64 / 1_000_000,
            echoed_mb = echoed_n * CHUNK_SIZE as u64 / 1_000_000,
            echo_pkt_s = format!("{:.0}", echoed_n as f64 / secs),
            entry_pkts_sent = entry_metrics
                .packets_sent
                .saturating_sub(entry_metrics_before.packets_sent),
            exit_pkts_recv = exit_metrics
                .packets_received
                .saturating_sub(exit_metrics_before.packets_received),
            relayed = ?relayed,
            relay_split_pct = ?relay_shares(&relayed),
            deposits = entry_metrics.deposits,
            deposits_failed = entry_metrics.deposits_failed,
            confirmed = exit_metrics.deposits_confirmed,
            keys = exit_metrics.keys_recovered,
            sweeps = exit_metrics.sweeps,
            "live"
        );

        if exit_metrics.deposits_timed_out > 0 {
            killed = true;
            tracing::info!(
                "the Entry ran out of deposit funds and the Exit's kill switch tripped after \
                 {:?}",
                t0.elapsed()
            );
            break;
        }
        if Instant::now() >= deadline {
            break;
        }
    }

    // Sweeps of already-recovered keys outlive the Session, so give the last ones time to
    // land before reading the Safe for the final time.
    //
    // The condition is "no new sweep for a while" rather than "keys_recovered == sweeps": the
    // SSAs whose deposits failed still have their shares completed by the traffic that keeps
    // flowing during the kill-switch fuse, so their keys are recovered but there is nothing at
    // those addresses to sweep. Waiting for the counts to converge would always burn the full
    // timeout.
    //
    // "A while" is several polls, not one. `REPORT_INTERVAL` is 5 s and the Entry takes a
    // measured ~5.0 s to confirm a sweep, so a single quiet interval is ordinary rather than
    // evidence of settlement — and treating it as evidence undercounts `exit_metrics.sweeps`,
    // which then reports funds "stranded at stealth addresses" for deposits swept a second after
    // the test stopped looking. `SETTLE_TIMEOUT` (60 s) has the budget; spend it.
    const QUIET_POLLS_REQUIRED: u32 = 3;
    let settle_until = Instant::now() + SETTLE_TIMEOUT;
    let mut quiet_polls = 0u32;
    loop {
        let before = exit_metrics.sweeps;
        tokio::time::sleep(REPORT_INTERVAL).await;
        // Strict from here on: everything below feeds an assertion, and a zeroed snapshot would
        // both end this loop immediately and satisfy the assertions it ends up in.
        exit_metrics = NodeMetrics::try_scrape(&exit_node.api)
            .await
            .context("scraping exit metrics while waiting for sweeps to settle")?;
        quiet_polls = if exit_metrics.sweeps == before {
            quiet_polls + 1
        } else {
            0
        };
        if quiet_polls >= QUIET_POLLS_REQUIRED || Instant::now() >= settle_until {
            break;
        }
    }
    entry_metrics = NodeMetrics::try_scrape(&entry.api)
        .await
        .context("scraping entry metrics for the closing assertions")?;
    let relayed = relay_forwarded(cluster.nodes(), &relays_metrics_before).await;
    let relay_split = relay_shares(&relayed);

    stop.store(true, Ordering::Release);
    sender.abort();
    receiver.abort();

    let entry_after = entry.api.balances().await.context("entry balances after")?;
    let exit_after = exit_node
        .api
        .balances()
        .await
        .context("exit balances after")?;
    let recovered = exit_after.safe_hopr - exit_before.safe_hopr;
    let spent = entry_before.safe_hopr - entry_after.safe_hopr;
    let sent_n = sent.load(Ordering::Acquire);
    let echoed_n = echoed.load(Ordering::Acquire);
    let echoed_bytes = echoed_n * CHUNK_SIZE as u64;

    // ── Assertions ──────────────────────────────────────────────────────────────
    assert!(
        echoed_n > 0,
        "no datagram completed the Entry -> Exit -> echo -> Entry round trip, so the PIX \
         Session never carried traffic (Entry deposits: {}, Exit keys recovered: {})",
        entry_metrics.deposits,
        exit_metrics.keys_recovered
    );

    // Both relays carried the Session. Nothing here distributes the traffic — hoprd redraws the
    // route per packet — so what this asserts is that the cluster is wired the way the topology
    // says and that a Session still spreads over the candidates it is given.
    let total_relayed: u64 = relayed.iter().sum();
    assert!(
        total_relayed > 0,
        "neither relay forwarded a packet, yet {echoed_n} datagrams completed the round trip — \
         a {HOPS}-hop Session cannot have delivered them without one"
    );
    for ((relay, forwarded), share) in RELAYS.iter().zip(&relayed).zip(&relay_split) {
        assert!(
            forwarded * 100 >= total_relayed * MIN_RELAY_SHARE_PERCENT,
            "node{relay} forwarded {forwarded} of the {total_relayed} packets the relays carried \
             between them ({share}%), under the {MIN_RELAY_SHARE_PERCENT}% floor: the Session is \
             not spread over both relays. Check that node{relay}'s channels to the Entry and to \
             the Exit are open and funded and that it announced on chain — a relay missing any of \
             those never enters the path selector's candidate set, and every other assertion in \
             this test passes without it."
        );
    }

    // The run ended the way it is designed to.
    assert!(
        killed,
        "the Exit's kill switch never tripped within {:?}: the Entry was funded for \
         {funded_cycles} deposits and made {}, spending {spent} of its {float}. Either cycles \
         are running far slower than the {MAX_SECS_PER_CYCLE}s/cycle the deadline assumes, or \
         deposits stopped for a reason other than running out of money.",
        t0.elapsed(),
        entry_metrics.deposits
    );
    assert_eq!(
        entry_metrics.deposits_rejected, 0,
        "the Entry rejected {} deposit(s) as exceeding max_ssa_allocation ({MAX_SSA_ALLOCATION}), \
         which ends the run for the wrong reason",
        entry_metrics.deposits_rejected
    );

    // Every recovered key that had money behind it must have produced a sweep.
    // `sweep_recovered` returns early *without* counting when the stealth address holds
    // nothing, so a gap here is the deposit race the SSA geometry is sized to avoid:
    // shares complete, the key is reconstructed, the deposit has not been mined, and the
    // funds are stranded at that address permanently.
    //
    // Two kinds of gap are legitimate, and both cluster at the end of the run. An SSA
    // whose deposit *failed* for want of funds has nothing to sweep by construction, and
    // traffic keeps flowing through the kill-switch fuse, so its shares usually finish
    // anyway. On top of that one SSA is normally in flight at any instant.
    let unswept_allowance =
        exit_metrics.sweeps + entry_metrics.deposits_failed + MAX_SSAS_IN_FLIGHT;
    assert!(
        exit_metrics.keys_recovered <= unswept_allowance,
        "the Exit recovered {} keys but swept only {}, beyond the {} unfunded ({} failed \
         deposits) and {MAX_SSAS_IN_FLIGHT} in-flight SSAs that may legitimately have no \
         sweep. The excess is funds stranded at stealth addresses: shares completed before \
         the deposit was mined. Lower HOPRD_PIX_SOAK_RATE or widen the SSA so a cycle \
         outlasts a deposit.",
        exit_metrics.keys_recovered,
        exit_metrics.sweeps,
        entry_metrics.deposits_failed + MAX_SSAS_IN_FLIGHT,
        entry_metrics.deposits_failed
    );

    // The Entry spent its budget down to what it could no longer afford, and every wxHOPR left
    // as a whole SSA deposit. That the Safe delta is *only* deposits is arranged rather than
    // assumed: both strategies that move Safe wxHOPR are off, and the channel stakes left before
    // `entry_before` was sampled.
    let deposited_cycles = completed_cycles(spent, per_cycle).unwrap_or_else(|| {
        panic!(
            "the Entry Safe paid out {spent}, which is not a whole multiple of the \
             {per_cycle} per-SSA deposit — something other than PIX deposits moved it"
        )
    });
    assert_eq!(
        deposited_cycles, funded_cycles,
        "the Entry was budgeted for {funded_cycles} deposits but made {deposited_cycles}. The \
         run should end only once the next deposit would cross `max_spend_per_window` ({float}); \
         its Safe still holds {}, so a shortfall here is the budget being hit early or a deposit \
         being refused for some other reason.",
        entry_after.safe_hopr
    );

    // And all of it, bar the SSAs cut short at the end, reached the Exit's Safe as whole
    // quota-sized deposits — recovered funds correspond to the data quota delivered.
    let cycles = completed_cycles(recovered, per_cycle).unwrap_or_else(|| {
        panic!(
            "Exit Safe gained {recovered}, which is not a whole multiple of the {per_cycle} \
             per-SSA deposit — something other than PIX sweeps moved the balance (Entry \
             deposits: {deposited_cycles}, Exit keys recovered: {})",
            exit_metrics.keys_recovered
        )
    });
    assert!(
        cycles + MAX_SSAS_IN_FLIGHT >= deposited_cycles,
        "the Entry funded {deposited_cycles} SSAs but only {cycles} were recovered and swept, \
         leaving more than the {MAX_SSAS_IN_FLIGHT} that may be cut short by the kill switch. \
         {} wxHOPR is stranded at stealth addresses.",
        per_cycle * (deposited_cycles - cycles)
    );

    assert!(
        echoed_bytes >= MIN_TOTAL_BYTES,
        "only {echoed_bytes} bytes completed the round trip, short of the {MIN_TOTAL_BYTES} \
         this test exists to move"
    );

    // Recovered funds correspond to data actually delivered back to the Entry.
    //
    // Compared in packets rather than bytes: a quota is `polys × (shares + surplus)` packets
    // priced at the full `HoprPacket::PAYLOAD_SIZE`, whereas each datagram here carries
    // CHUNK_SIZE, so the two are not commensurable as byte counts. Nor is this an equality —
    // a reply dropped after leaving the Exit has still consumed its SURB and unlocked its
    // share, so paid-for volume legitimately exceeds observed volume by the loss rate. What
    // it rules out is the Exit being paid for traffic it never sent.
    //
    // This binds far harder than it used to. While the surplus was unpriced, `paid_packets`
    // was two thirds of what a cycle actually delivered and the ratio sat near 150% against
    // a 80% floor — the assertion could not have failed short of total collapse. Now that
    // every emitted share is charged for, paid volume and delivered volume are the same
    // quantity and the margin here really is the loss rate.
    let paid_packets = cycles * emissions_per_ssa();
    assert!(
        echoed_n * 100 >= paid_packets * u64::from(MIN_DELIVERED_PERCENT),
        "the Exit was paid for {paid_packets} return packets across {cycles} cycles but only \
         {echoed_n} completed the round trip — over {}% of paid-for volume never arrived",
        100 - MIN_DELIVERED_PERCENT
    );

    // Runtime is a function of the float, so this only binds the default.
    if !float_overridden {
        assert!(
            t0.elapsed() <= DEFAULT_RUN_BUDGET,
            "a default run took {:?}, over its {DEFAULT_RUN_BUDGET:?} budget: {cycles} cycles \
             at {:.1}s each plus bootstrap. Either the machine is loaded or a cycle has got \
             slower — lower DEFAULT_FUNDED_CYCLES or check the return packet rate.",
            t0.elapsed(),
            t0.elapsed().as_secs_f64() / cycles.max(1) as f64
        );
    }

    tracing::info!(
        cycles,
        funded_cycles,
        %recovered,
        %spent,
        sent_mb = sent_n * CHUNK_SIZE as u64 / 1_000_000,
        echoed_mb = echoed_bytes / 1_000_000,
        relayed = ?relayed,
        relay_split_pct = ?relay_split,
        deposits = entry_metrics.deposits,
        deposits_failed = entry_metrics.deposits_failed,
        confirmed = exit_metrics.deposits_confirmed,
        keys = exit_metrics.keys_recovered,
        sweeps = exit_metrics.sweeps,
        "PIX soak test PASSED in {:?} \u{2014} ran until the Entry could not deposit",
        t0.elapsed()
    );
    Ok(())
}
