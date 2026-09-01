//! End-to-end PIX Session tests (not for CI — run explicitly).
//!
//! Exercises the whole PIX stack at once — `hopr-lib` Sessions with PIX, `hoprd`
//! Session creation over the REST API, and the `NonAnonymousPix` strategy doing real
//! on-chain deposits and sweeps — which no other test does: `hopr-lib`'s
//! `transport_session_pix` fakes the deposit, and the strategy's own tests run against
//! a mock connector.
//!
//! # The three tests
//!
//! Two run the same body ([`run_pix_cycles`]) over a different [`CycleProfile`], and differ
//! only in how many SSAs the Exit asks for per request. The third arranges the failure that
//! batching introduces.
//!
//! | Test | Exit asks | Entry accepts | Subject |
//! |---|---|---|---|
//! | `…sweeps_recovered_deposits_into_exit_safe` | 1 | 2 | the unbatched exchange |
//! | `…batches_three_ssas_per_request` | 3 | 3 | a batch accepted and paid for in full |
//! | `…is_refused_when_the_batch_exceeds_the_entry_cap` | 3 | 2 | the batch size not being negotiated |
//!
//! The happy path being asserted by the first two:
//!
//!   1. Entry opens a PIX Session to the Exit through one relay.
//!   2. Exit asks the Entry to commit to `ssas_per_request` SSAs in one request.
//!   3. Entry's strategy deposits `price_per_byte × quota` to each SSA stealth address.
//!   4. Exit's strategy observes the deposits and defuses the PIX kill switch, so the
//!      Session survives.
//!   5. Bidirectional traffic delivers SSA shares on the return-path SURBs until the
//!      Exit reconstructs each stealth address private key.
//!   6. Exit's strategy sweeps the deposits into its Safe.
//!   7. Repeats across several requests.
//!
//! The closing assertion is exact rather than approximate: with auto-redeeming off, PIX
//! sweeps are the only thing that moves wxHOPR *into* the Exit's Safe, and sweep gas
//! leaves as xDai. So the Safe's wxHOPR gain must be a whole multiple of
//! `price_per_byte × quota` — which is precisely the statement that recovered funds
//! correspond to the data quota delivered from the Exit back to the Entry. Batching does not
//! change the unit: a batch of three is three whole deposits, not one larger one.
//!
//! Required (at least one chain source):
//!   HOPRD_CHAIN_URL   or HOPRD_CHAIN_IMAGE
//! Optional: HOPRD_BIN, HOPRD_CONTAINER_RUNTIME
//!
//! # Prerequisites
//!
//! The `hoprd` binary must be built in **release** mode. Debug builds slow packet
//! processing and cryptography enough to distort the SSA cycle pacing these tests rely
//! on. It must also carry the **plain deposit pool**, which is not hoprd's default —
//! see [`session_pix_soak`](../session_pix_soak.rs) for why, and for the other pool.
//!
//! ```bash
//! nix develop -c cargo build --release -p hoprd --features strategy-pix-test
//! export HOPRD_BIN=$(pwd)/target/release/hoprd
//! ```
//!
//! Each test must be run individually — see [`common`] for details. All three share one port
//! block and the `hopr-chain` container, so they are selected one at a time. The refusal test
//! is over in seconds once the cluster is up; the other two run for minutes.
//!
//! ```bash
//! export HOPRD_BIN=$(pwd)/target/release/hoprd
//! nix develop -c cargo nextest run -p hoprd-localcluster --test session_pix \
//!   --run-ignored ignored-only -j 1 --no-capture -E 'test(refused)'
//! nix develop -c cargo nextest run -p hoprd-localcluster --test session_pix \
//!   --run-ignored ignored-only -j 1 --no-capture -E 'test(batches_three)'
//! nix develop -c cargo nextest run -p hoprd-localcluster --test session_pix \
//!   --run-ignored ignored-only -j 1 --no-capture -E 'test(sweeps_recovered)'
//! ```

mod common;

use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    time::Duration,
};

use anyhow::Context;
use common::{Cluster, ClusterSpec, pix::completed_cycles, ports};
use hopr_lib::api::types::primitive::prelude::HoprBalance;
use hoprd_localcluster::{client_helper, identity};
use tokio::net::UdpSocket;

const NUM_NODES: usize = 3;
const ENTRY: usize = 0;
const EXIT: usize = 2;
/// Intermediate relays on each path. PIX requires at least one: the share encryption
/// key is derived from the first relayer's acknowledgement, so a zero-hop return path
/// has nothing to derive it from and the Session is refused outright.
const HOPS: u64 = 1;

/// PIX generator dimensions, both sitting on the protocol floor (`PixGlobalConfig`
/// validates `num_ssa_parts >= 8` and `ssa_part_size >= 2`). This makes the per-SSA
/// quota as small as the protocol allows, and therefore the tests as short as possible.
///
/// The `u8`s are the upstream bound: the threshold and the surplus are one byte each of the
/// negotiated `PixParams` word.
const PIX_POLYS: u16 = 8;
const PIX_SHARES: u8 = 2;
/// Shares emitted beyond the threshold. Priced into the quota like any other emitted share,
/// so it costs both test time and wxHOPR; kept small for that reason.
const PIX_ADDITIONAL_SHARES: u8 = 2;

const WAIT_TIMEOUT: Duration = Duration::from_secs(120);
const SETUP_TIMEOUT: Duration = Duration::from_secs(300);
const BALANCE_POLL_INTERVAL: Duration = Duration::from_secs(2);
/// Pushed well past every run here: these tests want the strategies they configure to act on
/// PIX events, not on a timer that might open channels or redeem tickets mid-measurement.
const STRATEGY_INTERVAL: Duration = Duration::from_secs(600);

/// Payload per datagram, comfortably under SESSION_MTU so one datagram is one packet.
const CHUNK_SIZE: usize = 512;

/// Delay between datagrams.
///
/// This paces the Exit → Entry packet rate, which is what drives SSA share delivery:
/// the Exit consumes one return-path SURB per reply, and each SURB carries one share.
/// The generator emits shares polynomial-major at `PIX_SHARES + PIX_ADDITIONAL_SHARES`
/// per polynomial, so recovering an SSA takes on the order of
/// `PIX_POLYS × (PIX_SHARES + PIX_ADDITIONAL_SHARES)` replies — 32 here. Batching does not
/// change that per-SSA cost; a batch of three simply takes three times as many replies to
/// work through.
///
/// The pacing matters because share collection and the deposit run *concurrently*: the
/// Exit serves data on credit and only the kill switch enforces payment. Were a cycle
/// to finish before its deposit transaction was mined, the Exit would recover the key,
/// find a zero balance, log "already swept", and the funds would stay stranded at the
/// stealth address. At 400 ms a cycle takes roughly 13 s, comfortably longer than an
/// Anvil transaction.
const SEND_INTERVAL: Duration = Duration::from_millis(400);

/// SURB balancer target: how much response data the Exit can deliver before needing
/// more SURBs.
///
/// Deliberately small, and the least obvious knob in these tests. A PIX share is baked
/// into a SURB *when the SURB is minted*, and the Exit spends its SURB buffer roughly in
/// order — so the buffer is a pipeline delay between share generation and share
/// delivery. Sized at 2 MB (≈2000 SURBs, the value `session_udp` uses for throughput)
/// the Exit spends ~13 minutes working through SURBs minted during SSA #1 before it ever
/// touches one carrying an SSA #2 share, and the test sees exactly one cycle complete and
/// then nothing. 16 kB is ~16 SURBs, so a new SSA's shares start landing within seconds
/// of its commitment.
const RESPONSE_BUFFER: &str = "16 kB";
/// Ceiling on artificial SURB generation. Kept generous: with a small buffer the
/// balancer needs to refill promptly, and this caps the rate, not the depth.
const MAX_SURB_UPSTREAM: &str = "20 Mb/s";

/// Charged per byte of the agreed quota. With the dimensions above the quota is
/// `8 × (2 + 2) × 1038` ≈ 33.2 kB, so one SSA deposit is ~3.32 wxHOPR — small against the
/// 1000 wxHOPR each Safe is provisioned with, but large enough to be unambiguous in a
/// balance delta.
const PRICE_PER_BYTE: &str = "0.0001 wxHOPR";
/// Ceiling on a single deposit. Must exceed `PRICE_PER_BYTE × quota` or the strategy
/// refuses to deposit at all and the Exit's kill switch closes the Session.
///
/// Per *deposit*, not per batch: a batch is several whole deposits of this size, so batching
/// does not push against this.
const MAX_SSA_ALLOCATION: &str = "10 wxHOPR";
const GAS_XDAI_PER_SWEEP: &str = "0.01 xdai";
/// Generous relative to what the traffic actually consumes; an underfunded channel
/// would stall packet flow and starve share delivery.
const CHANNEL_STAKE: &str = "50 wxHOPR";
/// wxHOPR the Safe holds for deposits, and the ceiling on what the strategy will commit.
///
/// The same number twice so neither binds before the other: these tests end on their own
/// cycle count, not on either budget, and a run that hits one of them has gone wrong. ~30 SSA
/// deposits at this configuration's ~3.32 wxHOPR per cycle, against a worst case of nine
/// (three full batches at [`BATCHED`]).
const DEPOSIT_BUDGET: &str = "100 wxHOPR";
/// Must outlast the run, or the window rolls mid-run and the budget silently refills.
const SPEND_WINDOW: Duration = Duration::from_secs(3600);

/// Exit-side deadlines. Together these give the PIX kill switch an 80 s fuse *per SSA in a
/// batch* — upstream scales the deadline by `ssas_per_request`, and scales the deposit
/// awaiter's timeout by the same factor, so a batch is judged as a whole rather than per
/// cycle. Batching therefore buys slack here rather than spending it.
const MAX_SSA_DELIVERY_TIME: Duration = Duration::from_secs(20);
const MAX_DEPOSIT_WAIT: Duration = Duration::from_secs(60);
/// How long the Exit's strategy keeps polling for a deposit. Also fixes the poll
/// cadence at a tenth of this, so 30 s means a 3 s cadence — the hoprd default of 1 h
/// would poll every 6 min and never beat the 80 s fuse.
const MAX_DEPOSIT_TRACKING_TIME: Duration = Duration::from_secs(30);

// ── hoprd log lines the assertions read ──────────────────────────────────────────────
//
// These are other people's log messages, so they are named here rather than spelled inline:
// an upstream rewording then breaks one constant instead of scattered string literals, and
// each can carry what it actually means. All are `info!` or louder, which matters — hoprd's
// child processes log at `info` by default, and a `RUST_LOG` that filtered it out would blank
// every count below.

/// Exit, once per `SsaRequest`, carrying the `batch_size` it asked for.
const EXIT_BATCH_REQUEST: &str = "generated exit commitments for the SSA batch";
/// Exit, once per SSA whose deposit it confirmed — which is also what defuses that cycle's
/// kill switch. `hopr_transport_session`'s wording, not the Entry's deposit-flush lines below.
const EXIT_DEPOSIT_SEEN: &str = "SSA deposit successful";
/// Exit, once per SSA whose deposit did not arrive inside the (batch-scaled) window, letting
/// the kill switch fire.
const EXIT_DEPOSIT_MISSED: &str = "deposit confirmation timed out";
/// Exit, once per SSA cycle whose stealth-address key it reconstructed.
const EXIT_KEY_RECOVERED: &str = "private key recovered";
/// Exit, once per kill switch that fired.
///
/// A strict prefix of `"pix session deposit timeout set"`, which is the switch being *armed* and
/// happens on every request. Counting it needs the second needle below to tell them apart — see
/// [`Cluster::count_log_lines`].
const EXIT_KILL_SWITCH_FIRED: &str = "pix session deposit timeout";
/// Field present on the fired line and absent from the armed one.
const EXIT_KILL_SWITCH_INDEX: &str = "ssa_index=";
/// Exit, when the SSA index space ran out mid-batch and the request was shortened. Reaching it
/// needs 2^32 cycles in one Session, so it is a legibility guard: without it, a short batch
/// would only show up as the batch-size assertion failing for no visible reason.
const EXIT_BATCH_TRUNCATED: &str = "ssa batch truncated";
/// Entry, once per SSA of an accepted batch, emitted only after that SSA's commitment is on
/// the wire and its deposit has been handed to the strategy. This is the line that says the
/// Entry *proceeded* with a batch entry rather than merely receiving it.
const ENTRY_SSA_COMMITTED: &str = "generated client SSA commitment and deposit address";
/// Entry, when it refused a batch and tore the Session down.
const ENTRY_REFUSED_BATCH: &str = "closed session after refusing the Exit's SSA request";
/// Entry, carrying the reason a Start message was rejected — including the over-cap batch
/// message, which names both the size asked for and the cap.
const ENTRY_START_MSG_FAILED: &str = "failed to process Start protocol message";
/// Exit, on receiving the Entry's `SessionError`. Paired with the reason below, which
/// `StartErrorReason` renders through `strum::Display`.
const EXIT_POST_ESTABLISHMENT_ERROR: &str = "received post-establishment session error";
const EXIT_UNACCEPTABLE_PIX_PARAMS: &str = "reason=UnacceptablePixParams";
/// Entry, `hopr_strategy`'s two deposit-flush lines. Which one appears is a matter of whether
/// the strategy's 500 ms debounce happened to coalesce a protocol batch into one on-chain
/// flush, so neither is asserted on — see [`run_pix_cycles`].
const ENTRY_SINGLE_DEPOSIT: &str = "single deposit flushed successfully";
const ENTRY_BATCH_DEPOSIT: &str = "batch deposit flushed";

/// Everything [`run_pix_cycles`] varies on.
struct CycleProfile {
    /// `incoming_session_pix.ssas_per_request` on every node: SSAs the Exit asks the Entry to
    /// commit to in one request.
    ssas_per_request: usize,
    /// `pix.max_ssas_per_request` on every node: SSA commitments the Entry accepts in one
    /// request. Must be at least [`Self::ssas_per_request`] here, since these profiles are the
    /// ones that expect the batch to be *served*.
    entry_cap: usize,
    /// SSA cycles that must fully complete — deposit made, key recovered, funds swept.
    target_cycles: u64,
    /// Budget for [`Self::target_cycles`] deposits to make it all the way into the Exit's Safe,
    /// measured from the moment the Session opens.
    recovery_timeout: Duration,
    /// Where the node logs are copied when the cluster drops. One per profile, or the second
    /// run to use this port block would overwrite the first one's post-mortem.
    logs_to: &'static str,
}

impl CycleProfile {
    /// How far the Entry's deposits may legitimately run ahead of the Exit's recoveries.
    ///
    /// The Exit asks for the next batch once the *last* SSA of the current one passes the
    /// early-recovery threshold (earlier indices take the "index already advanced" path), and the
    /// Entry deposits for the whole new batch immediately. So a full batch may be funded while
    /// one cycle is still outstanding.
    fn max_ssas_in_flight(&self) -> u64 {
        self.ssas_per_request as u64 + 1
    }

    /// Requests needed to reach [`Self::target_cycles`], and therefore the fewest the run must
    /// be able to show for itself.
    fn min_requests(&self) -> usize {
        self.target_cycles.div_ceil(self.ssas_per_request as u64) as usize
    }
}

/// The unbatched exchange: one SSA per request, against hoprd's own default Entry cap.
///
/// Four cycles is enough to show the request/deposit/recover/sweep loop repeating rather than
/// working once.
const UNBATCHED: CycleProfile = CycleProfile {
    ssas_per_request: 1,
    entry_cap: 2,
    target_cycles: 4,
    recovery_timeout: Duration::from_secs(240),
    logs_to: "/tmp/pix-session-logs",
};

/// Three SSAs per request, with the Entry's cap raised to match.
///
/// Six cycles rather than three, so the run spans *two* full batches: one batch would never
/// exercise the request at a batch boundary, which is where the index advances by three at once
/// and a second over-default-cap batch has to be accepted.
///
/// The traffic is three times as long per request — 6 cycles × 32 replies × [`SEND_INTERVAL`]
/// ≈ 77 s — so the recovery budget is scaled to keep the same slack [`UNBATCHED`] has.
const BATCHED: CycleProfile = CycleProfile {
    ssas_per_request: 3,
    entry_cap: 3,
    target_cycles: 6,
    recovery_timeout: Duration::from_secs(360),
    logs_to: "/tmp/pix-session-batched-logs",
};

/// The Exit asks for three against an Entry that accepts two.
///
/// Not a [`CycleProfile`]: no cycle ever completes, so there is nothing for the fields about
/// cycle counts and recovery budgets to mean.
const REFUSED_SSAS_PER_REQUEST: usize = 3;
const REFUSED_ENTRY_CAP: usize = 2;
const REFUSED_LOGS: &str = "/tmp/pix-session-refused-logs";
/// How long the refusal is waited for. The Exit sends its first `SsaRequest` during
/// establishment, so this is a round trip plus the Entry's commitment-count check — seconds.
/// The budget is generous only because the alternative to waiting long enough is a test that
/// fails by looking too early.
const REFUSAL_TIMEOUT: Duration = Duration::from_secs(60);
const REFUSAL_POLL_INTERVAL: Duration = Duration::from_secs(1);

/// PIX settings for a run in which the Exit asks for `ssas_per_request` SSAs at a time and the
/// Entry accepts up to `entry_cap` of them.
///
/// The two are arguments rather than constants because the batch size is not negotiated: every
/// node here is configured as both halves, and whether they agree is the subject of one of the
/// tests below.
fn pix_settings(
    ssas_per_request: usize,
    entry_cap: usize,
) -> anyhow::Result<identity::PixSettings> {
    Ok(identity::PixSettings {
        num_ssa_parts: PIX_POLYS as usize,
        ssa_part_size: PIX_SHARES as usize,
        additional_shares: PIX_ADDITIONAL_SHARES as usize,
        // The Exit rejects any quota outside this window. Ours is ~33.2 kB — the surplus is
        // in the product, see `quota_per_ssa` — against a production default window of
        // ~130 MiB–519 MiB, so it has to be widened. Per SSA, so batching does not move it.
        quota_range_min: 0,
        quota_range_max: 1024 * 1024,
        max_ssa_delivery_time: MAX_SSA_DELIVERY_TIME,
        max_deposit_wait: MAX_DEPOSIT_WAIT,
        // Only the Exit refuses non-PIX Sessions; the relay never terminates one.
        enforce_on_nodes: vec![EXIT],
        ssas_per_request,
        max_ssas_per_request: entry_cap,
        safe_deposit_float: DEPOSIT_BUDGET.parse().context("parsing deposit float")?,
        // Settlement knobs. These used to travel as environment variables; they are written
        // into the generated node config's `Pix` strategy stanza now.
        price_per_byte: PRICE_PER_BYTE.parse().context("parsing price per byte")?,
        max_ssa_allocation: MAX_SSA_ALLOCATION
            .parse()
            .context("parsing max SSA allocation")?,
        max_spend_per_window: DEPOSIT_BUDGET.parse().context("parsing spend ceiling")?,
        spend_window: SPEND_WINDOW,
        max_deposit_tracking_time: MAX_DEPOSIT_TRACKING_TIME,
        gas_xdai_per_sweep: GAS_XDAI_PER_SWEEP.parse().context("parsing sweep gas")?,
    })
}

/// Bring up a three-node cluster with PIX configured, channels funded and every peer reachable.
///
/// AutoRedeeming is deliberately off: ticket redemption also credits the Safe in wxHOPR, which
/// would make the closing balance assertions ambiguous. ChannelLifecycle is off because these
/// tests open the channels themselves.
async fn pix_cluster(
    settings: identity::PixSettings,
    logs_to: &'static str,
) -> anyhow::Result<Cluster> {
    let cluster = Cluster::start(ClusterSpec {
        num_nodes: NUM_NODES,
        strategies: identity::StrategySet {
            auto_redeeming: false,
            channel_lifecycle: false,
        },
        strategy_execution_interval: Some(STRATEGY_INTERVAL),
        pix: Some(settings),
        logs_to: Some(logs_to),
        ..ClusterSpec::new(ports::SESSION_PIX)
    })
    .await?;
    cluster.wait_ready(WAIT_TIMEOUT).await?;
    cluster.open_channels(CHANNEL_STAKE, SETUP_TIMEOUT).await?;
    cluster.wait_channels(SETUP_TIMEOUT).await?;
    cluster.wait_reachable(SETUP_TIMEOUT).await?;
    Ok(cluster)
}

/// A PIX Session request for this configuration's dimensions.
fn open_request<'a>(exit_addr: &'a str, target: &'a str) -> client_helper::OpenSessionRequest<'a> {
    client_helper::OpenSessionRequest {
        protocol: "udp",
        destination: exit_addr,
        target,
        hops: HOPS,
        capabilities: Some(vec![
            hoprd_api_client::types::SessionCapability::Segmentation,
            hoprd_api_client::types::SessionCapability::NoDelay,
            hoprd_api_client::types::SessionCapability::UsePix,
        ]),
        // The SURB balancer is what carries SSA shares to the Exit. See
        // RESPONSE_BUFFER: oversizing it delays share delivery by a whole buffer's
        // worth of replies, which stalls every cycle after the first.
        response_buffer: Some(RESPONSE_BUFFER.to_string()),
        max_surb_upstream: Some(MAX_SURB_UPSTREAM.to_string()),
        // Must match this node's own generator dimensions, or the Session is refused.
        pix_ssa_quota: Some(hoprd_api_client::types::PixSsaQuota {
            polys_per_ssa: PIX_POLYS,
            shares_per_poly: PIX_SHARES,
            surplus_shares: PIX_ADDITIONAL_SHARES,
        }),
    }
}

/// Run `profile.target_cycles` SSA cycles through a PIX Session and assert that every one of
/// them was requested at the configured batch size, committed to by the Entry, paid for, and
/// swept into the Exit's Safe.
async fn run_pix_cycles(profile: &CycleProfile) -> anyhow::Result<()> {
    let t0 = std::time::Instant::now();
    let batch = profile.ssas_per_request;
    let target_cycles = profile.target_cycles;

    let settings = pix_settings(batch, profile.entry_cap)?;
    let quota = settings.quota_per_ssa();
    let cluster = pix_cluster(settings, profile.logs_to).await?;
    tracing::info!("channels ready after {:?}", t0.elapsed());

    let price_per_byte: HoprBalance = PRICE_PER_BYTE.parse().context("parsing price per byte")?;
    let per_cycle = price_per_byte * quota;
    let target_total = per_cycle * target_cycles;
    tracing::info!(
        %price_per_byte, quota, %per_cycle, %target_total, target_cycles, batch,
        "PIX accounting: one SSA cycle costs price_per_byte x quota, whether batched or not"
    );

    let echo_port = common::echo_server().await?;
    let entry = cluster.node(ENTRY);
    let exit_node = cluster.node(EXIT);
    let exit_addr = exit_node
        .address
        .as_ref()
        .context("exit node address unresolved")?;
    let target = format!("127.0.0.1:{echo_port}");
    tracing::info!(entry = entry.id, exit = %exit_addr, %target, "topology");

    // Snapshot after channel funding so the stakes are already out of the Safes and the
    // only subsequent movement is PIX.
    //
    // Both sides move wxHOPR through their *Safe*, and for the same reason: `hopr-types` 4.0.0
    // routes `SafePayloadGenerator::transfer` through the Safe module, so the Entry's deposits
    // are debited from its Safe however they are signed, and the Exit's sweeps are credited to
    // its own. Until then the Entry's transfer was direct and left its node account instead,
    // which is what this used to sample.
    //
    // The node accounts are not idle — they still pay gas, and on the Exit they pay the sweep's
    // xDai top-up — but nothing moves *wxHOPR* through them any more.
    let entry_before = entry
        .api
        .balances()
        .await
        .context("reading entry balances")?;
    let exit_before = exit_node
        .api
        .balances()
        .await
        .context("reading exit balances")?;
    tracing::info!(
        entry_safe_hopr = %entry_before.safe_hopr,
        exit_safe_hopr = %exit_before.safe_hopr,
        exit_safe_native = %exit_before.safe_native,
        "balances before the Session"
    );

    let (ip, port) = entry
        .api
        .open_session(open_request(exit_addr, &target))
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
    let echoed = Arc::new(AtomicU64::new(0));

    let payload: Vec<u8> = (0..CHUNK_SIZE).map(|i| (i % 256) as u8).collect();
    let sender = tokio::spawn({
        let (sock, stop, payload) = (sock.clone(), stop.clone(), payload.clone());
        async move {
            while !stop.load(Ordering::Acquire) {
                if sock.send(&payload).await.is_err() {
                    break;
                }
                tokio::time::sleep(SEND_INTERVAL).await;
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
                    // balance assertion below is the real verdict — just note it.
                    Ok(Ok(n)) => tracing::warn!(n, "unexpected echo payload"),
                    Ok(Err(error)) => {
                        tracing::warn!(%error, "session socket recv failed");
                        break;
                    }
                    Err(_) => tracing::warn!("no echo for 30s"),
                }
            }
        }
    });

    // ── Wait for the deposits to land in the Exit's Safe ────────────────────────
    // `Balance` subtraction saturates, so a Safe that somehow shrank reads as zero
    // rather than wrapping.
    let deadline = std::time::Instant::now() + profile.recovery_timeout;
    let mut recovered = HoprBalance::default();
    // Display only — the loop exits on `recovered`, not on this. Kept across iterations so a
    // sample that lands mid-sweep, and is therefore not a whole multiple, reports the last
    // consistent reading rather than 0. Printing 0 after three completed cycles reads as "the
    // money went away", which is the most alarming thing this test can say and is false.
    let mut cycles_seen = 0u64;
    while std::time::Instant::now() < deadline {
        tokio::time::sleep(BALANCE_POLL_INTERVAL).await;

        let exit_now = exit_node
            .api
            .balances()
            .await
            .context("polling exit balances")?;
        recovered = exit_now.safe_hopr - exit_before.safe_hopr;
        tracing::info!(
            %recovered,
            cycles = {
                cycles_seen = completed_cycles(recovered, per_cycle).unwrap_or(cycles_seen);
                cycles_seen
            },
            echoed = echoed.load(Ordering::Acquire),
            elapsed = ?t0.elapsed(),
            "waiting for SSA cycles"
        );
        if recovered >= target_total {
            break;
        }
    }

    stop.store(true, Ordering::Release);
    sender.abort();
    receiver.abort();

    // The two Entry-side deposit-flush lines are read but not asserted on. Which of them the
    // strategy emits depends on whether its 500 ms debounce coalesced this profile's batch into
    // one on-chain flush, which is a timing question and is covered by upstream's own unit
    // tests; the authoritative deposit count here is the Entry Safe delta below. They are still
    // worth reporting, because "the Entry deposited nothing" and "the Entry deposited and the
    // Exit could not recover" look identical from the Exit's side.
    let single_deposits = cluster.count_log_lines(ENTRY, &[ENTRY_SINGLE_DEPOSIT])?;
    let batch_deposits = cluster.count_log_lines(ENTRY, &[ENTRY_BATCH_DEPOSIT])?;
    let entry_commitments = cluster.count_log_lines(ENTRY, &[ENTRY_SSA_COMMITTED])?;
    let refusals = cluster.count_log_lines(ENTRY, &[ENTRY_REFUSED_BATCH])?;
    let exit_requests = cluster.count_log_lines(EXIT, &[EXIT_BATCH_REQUEST])?;
    let full_batches =
        cluster.count_log_lines(EXIT, &[EXIT_BATCH_REQUEST, &format!("batch_size={batch}")])?;
    let truncated = cluster.count_log_lines(EXIT, &[EXIT_BATCH_TRUNCATED])?;
    let keys_recovered = cluster.count_log_lines(EXIT, &[EXIT_KEY_RECOVERED])?;
    let deposits_seen = cluster.count_log_lines(EXIT, &[EXIT_DEPOSIT_SEEN])?;
    let deposits_missed = cluster.count_log_lines(EXIT, &[EXIT_DEPOSIT_MISSED])?;
    let kill_switches_fired =
        cluster.count_log_lines(EXIT, &[EXIT_KILL_SWITCH_FIRED, EXIT_KILL_SWITCH_INDEX])?;
    let echoed = echoed.load(Ordering::Acquire);

    // ── Assertions ──────────────────────────────────────────────────────────────
    assert!(
        echoed > 0,
        "no datagram completed the Entry -> Exit -> echo -> Entry round trip, so the PIX \
         Session never carried traffic (Entry commitments: {entry_commitments}, deposits: \
         {single_deposits} single / {batch_deposits} batched, keys recovered: {keys_recovered})"
    );

    // ── The batch size actually in effect ───────────────────────────────────────
    // Asserted for both profiles rather than only the batched one, so the unbatched run states
    // that it *is* unbatched instead of assuming it. `batch_size` is what the Exit allocated,
    // not what it was configured with, so this also catches an upstream clamp.
    assert_eq!(
        truncated, 0,
        "the Exit shortened {truncated} batch(es) for want of SSA index space, which needs 2^32 \
         cycles in one Session and means something is very wrong"
    );
    assert_eq!(
        full_batches, exit_requests,
        "the Exit made {exit_requests} SSA requests but only {full_batches} of them asked for \
         the configured {batch} SSAs — the rest were a different size, so this run did not \
         exercise the batch size it was configured with"
    );
    assert!(
        exit_requests >= profile.min_requests(),
        "the Exit made {exit_requests} SSA requests, expected at least {} to reach \
         {target_cycles} cycles at {batch} per request",
        profile.min_requests()
    );

    // ── The Entry accepted the batch and proceeded with all of it ───────────────
    // The batch size is not negotiated, so an Entry whose `max_ssas_per_request` is below the
    // Exit's `ssas_per_request` refuses every request and the Session dies. This is the
    // assertion that says so in those terms rather than as a mystery deposit timeout.
    assert_eq!(
        refusals, 0,
        "the Entry refused {refusals} of the Exit's SSA requests as unacceptable. A batch of \
         {batch} needs the Entry's pix.max_ssas_per_request to be at least {batch}; it is \
         configured at {}.",
        profile.entry_cap
    );
    // One line per SSA of an accepted batch, emitted only once that SSA's commitment is on the
    // wire and its deposit is armed. So this is the Entry proceeding with every entry of every
    // batch, not just the first.
    assert!(
        entry_commitments >= target_cycles as usize,
        "the Entry committed to {entry_commitments} SSAs, expected at least {target_cycles}. A \
         count short of the target but a multiple of the {exit_requests} requests means the \
         Entry served only part of each batch."
    );

    // ── The Exit saw the deposits and did not kill the Session ──────────────────
    // The deposit awaiter logs one line per SSA when it confirms a deposit and defuses that
    // cycle's kill switch, and a different one when it gives up. Both are per SSA, not per
    // batch, so they stay comparable to the cycle count under batching. Upstream scales the
    // awaiter's window by `ssas_per_request` for exactly this reason: the N-th deposit of a
    // batch the Entry is funding in order legitimately arrives late.
    assert_eq!(
        deposits_missed, 0,
        "the Exit gave up waiting for {deposits_missed} deposit(s) (it confirmed \
         {deposits_seen}). Either the Entry's strategy never deposited, or a deposit landed \
         outside the {batch} x (max_deposit_wait + max_ssa_delivery_time) window."
    );
    assert_eq!(
        kill_switches_fired, 0,
        "the PIX kill switch fired {kill_switches_fired} time(s), closing the Session for an \
         unrealized deposit"
    );
    assert!(
        deposits_seen >= target_cycles as usize,
        "the Exit confirmed only {deposits_seen} deposits, expected at least {target_cycles} \
         (Entry committed to {entry_commitments} SSAs and flushed {single_deposits} single / \
         {batch_deposits} batched deposits)"
    );

    // An exact multiple is the real check: it says every wxHOPR that entered the Exit's
    // Safe arrived as a whole SSA deposit of `price_per_byte x quota`, i.e. the recovered
    // funds correspond exactly to the data quota delivered back to the Entry. A batch is
    // several such deposits, so the unit is unchanged.
    let cycles = completed_cycles(recovered, per_cycle).unwrap_or_else(|| {
        panic!(
            "Exit Safe gained {recovered}, which is not a whole multiple of the {per_cycle} \
             per-SSA deposit — something other than PIX sweeps moved the balance (Entry \
             commitments: {entry_commitments}, keys recovered: {keys_recovered})"
        )
    });

    assert!(
        cycles >= target_cycles,
        "expected at least {target_cycles} completed SSA cycles, got {cycles} ({recovered} of \
         the {target_total} target) after {:?}. The Exit made {exit_requests} requests of \
         {batch}, the Entry committed to {entry_commitments} SSAs, the Exit recovered \
         {keys_recovered} keys, {echoed} datagrams echoed. A recovered-key count above the \
         cycle count means keys were reconstructed before their deposits were mined and the \
         funds are stranded at the stealth addresses — slow SEND_INTERVAL down.",
        t0.elapsed()
    );

    // The Entry funded every one of those deposits out of its Safe.
    //
    // That the delta is *only* deposits is arranged, not assumed: both strategies that move Safe
    // wxHOPR are off (see `pix_cluster`), and the channel stakes left the Safe before
    // `entry_before` was sampled. So nothing else here debits or credits it.
    //
    // Not an equality: the Exit requests the next batch at the early-recovery threshold
    // (~85% of the last SSA's shares), so the Entry has already deposited for SSAs still in
    // flight by the time this samples. `spent` therefore runs a batch or so ahead of
    // `recovered`. What must hold is that the outflow is *only* whole PIX deposits — a
    // half-mined batch is still a whole number of them, since each entry is its own transfer —
    // and that the Entry is not paying for SSAs that never complete.
    let entry_after = entry
        .api
        .balances()
        .await
        .context("reading entry balances after")?;
    let spent = entry_before.safe_hopr - entry_after.safe_hopr;
    let deposited_cycles = completed_cycles(spent, per_cycle).unwrap_or_else(|| {
        panic!(
            "the Entry Safe paid out {spent}, which is not a whole multiple of the \
             {per_cycle} per-SSA deposit — something other than PIX deposits moved it"
        )
    });
    let in_flight = profile.max_ssas_in_flight();
    assert!(
        (cycles..=cycles + in_flight).contains(&deposited_cycles),
        "the Entry deposited for {deposited_cycles} SSAs but only {cycles} were recovered and \
         swept. Up to {in_flight} may legitimately be in flight at {batch} SSAs per request — a \
         whole new batch funded plus the one still recovering — and more than that means \
         deposits are being made for SSAs that never complete."
    );

    entry
        .api
        .close_client(&ip, port)
        .await
        .context("closing the session listener")?;

    tracing::info!(
        cycles, %recovered, batch, exit_requests, entry_commitments, deposits_seen,
        keys_recovered, echoed, single_deposits, batch_deposits,
        "PIX session test PASSED in {:?}", t0.elapsed()
    );
    Ok(())
}

// `multi_thread`, matching `session_pix_soak`: the correctness of these tests rests on packet
// pacing. `SEND_INTERVAL` is a floor, not a rate, and a current-thread runtime multiplexes the
// sender, the echo server task and the balance poller onto one thread — contention stretches the
// interval, and a cycle that outruns its deposit leaves the Exit recovering a key against a zero
// balance, logging "already swept" and stranding the funds. `recovery_timeout` absorbs the
// stretch until it does not.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires external chain container and hoprd binary \u{2014} run explicitly, not in CI"]
async fn localcluster_pix_session_sweeps_recovered_deposits_into_exit_safe() -> anyhow::Result<()> {
    common::init_tracing();
    run_pix_cycles(&UNBATCHED).await
}

/// The same exchange with the Exit asking for three SSAs at a time and the Entry's cap raised to
/// match, over two full batches.
///
/// Everything batching scales is exercised by construction: the Entry produces three commitment
/// sets and three deposits per request, the Exit holds three live reconstructor cycles and fronts
/// three SSA quotas of service before the first deposit lands, and both the kill switch and the
/// deposit awaiter are running on their tripled windows. The assertions that make it a test of
/// batching rather than a slower test of the same thing are the batch-size count on the Exit, the
/// zero-refusal count on the Entry, and the per-SSA commitment count that has to reach the target
/// through batches rather than one at a time.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires external chain container and hoprd binary \u{2014} run explicitly, not in CI"]
async fn localcluster_pix_session_batches_three_ssas_per_request() -> anyhow::Result<()> {
    common::init_tracing();
    run_pix_cycles(&BATCHED).await
}

/// An Exit batching above the Entry's cap loses the Session, and pays nothing for it.
///
/// The batch size is not negotiated — `StartSession.additional_data` is fully allocated, so the
/// Entry cannot advertise its cap and the Exit cannot learn it — which makes a mismatched pair a
/// configuration error that only shows up at run time. Upstream's answer is to refuse the whole
/// request with `UnacceptablePixParams` rather than truncate it, so the failure is immediate and
/// reported instead of surfacing as a deposit timeout a whole batch-scaled window later. This is
/// the test of that answer, and it is the companion to the zero-refusal assertion in
/// [`run_pix_cycles`]: that one says a matched pair is never refused, this one says a mismatched
/// pair always is.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires external chain container and hoprd binary \u{2014} run explicitly, not in CI"]
async fn localcluster_pix_session_is_refused_when_the_batch_exceeds_the_entry_cap()
-> anyhow::Result<()> {
    common::init_tracing();
    let t0 = std::time::Instant::now();

    let settings = pix_settings(REFUSED_SSAS_PER_REQUEST, REFUSED_ENTRY_CAP)?;
    let cluster = pix_cluster(settings, REFUSED_LOGS).await?;
    tracing::info!("channels ready after {:?}", t0.elapsed());

    let echo_port = common::echo_server().await?;
    let entry = cluster.node(ENTRY);
    let exit_node = cluster.node(EXIT);
    let exit_addr = exit_node
        .address
        .as_ref()
        .context("exit node address unresolved")?;
    let target = format!("127.0.0.1:{echo_port}");

    let entry_before = entry
        .api
        .balances()
        .await
        .context("reading entry balances")?;
    let exit_before = exit_node
        .api
        .balances()
        .await
        .context("reading exit balances")?;

    // The Exit sends `SessionEstablished` to the Entry *before* it runs its PIX setup and the
    // `SsaRequest` that setup sends, so this normally succeeds and the Session is torn down a
    // round trip later. It can also lose that race — the API call waits for SURB readiness, and
    // the closure may land first — and return an error instead. Both are consistent with the
    // refusal, which the logs are the verdict on, so the outcome is recorded rather than
    // asserted, and the listener is only closed if there is one.
    let opened = entry
        .api
        .open_session(open_request(exit_addr, &target))
        .await;
    match &opened {
        Ok((ip, port)) => tracing::info!(
            %ip, port, elapsed = ?t0.elapsed(),
            "listener opened; the Entry's refusal is expected to close the Session under it"
        ),
        Err(error) => tracing::info!(
            %error, elapsed = ?t0.elapsed(),
            "open_session failed, which an immediate refusal can legitimately cause"
        ),
    }

    // Polled rather than slept through: the refusal takes a round trip, and the only thing a
    // fixed wait buys is a test that fails by having looked too early.
    let deadline = std::time::Instant::now() + REFUSAL_TIMEOUT;
    let refusals = loop {
        let refusals = cluster.count_log_lines(ENTRY, &[ENTRY_REFUSED_BATCH])?;
        if refusals > 0 || std::time::Instant::now() >= deadline {
            break refusals;
        }
        tokio::time::sleep(REFUSAL_POLL_INTERVAL).await;
    };

    let over_cap = cluster.count_log_lines(
        ENTRY,
        &[
            ENTRY_START_MSG_FAILED,
            &format!("at most {REFUSED_ENTRY_CAP} allowed"),
        ],
    )?;
    let exit_notified = cluster.count_log_lines(
        EXIT,
        &[EXIT_POST_ESTABLISHMENT_ERROR, EXIT_UNACCEPTABLE_PIX_PARAMS],
    )?;
    let entry_commitments = cluster.count_log_lines(ENTRY, &[ENTRY_SSA_COMMITTED])?;
    let deposits_seen = cluster.count_log_lines(EXIT, &[EXIT_DEPOSIT_SEEN])?;

    // ── Assertions ──────────────────────────────────────────────────────────────
    // The most specific line first: it names both the size asked for and the cap, so a failure
    // here distinguishes "refused for some other reason" from "refused over the cap".
    assert!(
        over_cap > 0,
        "the Entry never rejected an over-cap batch. Expected it to refuse the Exit's request \
         for {REFUSED_SSAS_PER_REQUEST} SSA commitments against its own cap of \
         {REFUSED_ENTRY_CAP} (it refused {refusals} request(s) for any reason, and committed to \
         {entry_commitments} SSAs). A zero commitment count with a zero refusal count means no \
         SsaRequest ever reached the Entry, which is a different failure — check that the \
         Session was established at all."
    );
    assert!(
        refusals > 0,
        "the Entry rejected {over_cap} over-cap batch(es) but never closed the Session for it, \
         so the refusal did not take the Session down with it"
    );
    assert!(
        exit_notified > 0,
        "the Exit was never told why its request was refused. Expected an \
         UnacceptablePixParams SessionError from the Entry, which is what makes the failure \
         immediate rather than a deposit timeout {REFUSED_SSAS_PER_REQUEST} deadlines later."
    );

    // Nothing was committed to and nothing was paid: the whole request is rejected, not the
    // surplus over the cap. A truncating Entry would show up here as commitments and deposits
    // for the first two of every three.
    assert_eq!(
        entry_commitments, 0,
        "the Entry committed to {entry_commitments} SSAs of a batch it refused — the refusal is \
         supposed to reject the whole request, not serve it up to the cap"
    );
    assert_eq!(
        deposits_seen, 0,
        "the Exit confirmed {deposits_seen} deposit(s) for a Session that was refused"
    );

    let entry_after = entry
        .api
        .balances()
        .await
        .context("reading entry balances after")?;
    let exit_after = exit_node
        .api
        .balances()
        .await
        .context("reading exit balances after")?;
    let spent = entry_before.safe_hopr - entry_after.safe_hopr;
    let recovered = exit_after.safe_hopr - exit_before.safe_hopr;
    assert!(
        spent.is_zero(),
        "the Entry Safe paid out {spent} for a Session whose SSA request it refused itself"
    );
    assert!(
        recovered.is_zero(),
        "the Exit Safe gained {recovered} from a Session that never got past its SSA request"
    );

    if let Ok((ip, port)) = &opened {
        // Best-effort: the Session is already gone, so the listener may have been reaped with it.
        if let Err(error) = entry.api.close_client(ip, *port).await {
            tracing::info!(%error, "closing the listener failed, which a refused Session can cause");
        }
    }

    tracing::info!(
        refusals,
        over_cap,
        exit_notified,
        "PIX refusal test PASSED in {:?}",
        t0.elapsed()
    );
    Ok(())
}

/// Every `field=value` needle above has to survive hoprd's colouring.
///
/// Not `#[ignore]`d and needing no cluster, because it guards the assertions that do. hoprd
/// colours its output whether or not stdout is a terminal, and `tracing` wraps a field's name, its
/// `=` and its value in three separate escape sequences — so the first version of the refusal test
/// asserted on `reason=UnacceptablePixParams` against a line that displays exactly that, and
/// counted zero. The batch-size assertion in [`run_pix_cycles`] would have failed the same way,
/// and `EXIT_KILL_SWITCH_INDEX` would have passed for the wrong reason.
///
/// Both lines are the bytes hoprd actually wrote during that run, not a reconstruction.
#[test]
fn log_field_needles_survive_hoprds_colouring() {
    const EXIT_BATCH_LINE: &str = concat!(
        "\u{1b}[2m2026-09-01T13:53:12.654844Z\u{1b}[0m \u{1b}[32m INFO\u{1b}[0m ThreadId(34) ",
        "\u{1b}[2mhopr_transport_session::manager\u{1b}[0m\u{1b}[2m:\u{1b}[0m ",
        "generated exit commitments for the SSA batch ",
        "\u{1b}[3msession_id\u{1b}[0m\u{1b}[2m=\u{1b}[0m5c40eaf71aeae516a43b ",
        "\u{1b}[3mcurrent_ssa_state\u{1b}[0m\u{1b}[2m=\u{1b}[0mSessionSsaState { current_index: 1, ",
        "num_errors: 0, polys_per_ssa: 8, shares_per_poly: 2, surplus_shares: 2 } ",
        "\u{1b}[3mbatch_size\u{1b}[0m\u{1b}[2m=\u{1b}[0m3 ",
        "\u{1b}[3mfirst_ssa_index\u{1b}[0m\u{1b}[2m=\u{1b}[0m1",
    );
    const EXIT_REFUSED_LINE: &str = concat!(
        "\u{1b}[2m2026-09-01T13:53:12.698437Z\u{1b}[0m \u{1b}[31mERROR\u{1b}[0m ThreadId(34) ",
        "\u{1b}[2mhopr_transport_session::manager\u{1b}[0m\u{1b}[2m:\u{1b}[0m ",
        "received post-establishment session error — closing session ",
        "\u{1b}[3msession_id\u{1b}[0m\u{1b}[2m=\u{1b}[0m5c40eaf71aeae516a43b ",
        "\u{1b}[3mreason\u{1b}[0m\u{1b}[2m=\u{1b}[0mUnacceptablePixParams",
    );

    let batch = common::strip_ansi(EXIT_BATCH_LINE);
    assert!(
        !batch.contains('\u{1b}'),
        "an escape sequence survived stripping: {batch:?}"
    );
    assert!(
        batch.contains(EXIT_BATCH_REQUEST) && batch.contains("batch_size=3"),
        "the batch-size needle must match the line that displays it: {batch:?}"
    );
    // `first_ssa_index=1`, not the batch size — the two-needle form has to see the whole line, or
    // a request of one would count as a request of three.
    assert!(
        !batch.contains("batch_size=1"),
        "the batch-size needle must not match another field's value: {batch:?}"
    );

    let refused = common::strip_ansi(EXIT_REFUSED_LINE);
    assert!(
        refused.contains(EXIT_POST_ESTABLISHMENT_ERROR)
            && refused.contains(EXIT_UNACCEPTABLE_PIX_PARAMS),
        "the refusal reason must match the line that displays it: {refused:?}"
    );
}
