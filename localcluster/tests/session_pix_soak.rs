//! Sustained-throughput PIX Session test (not for CI — run explicitly).
//!
//! The same happy path as [`session_pix`](../session_pix.rs), scaled up to something
//! resembling real use — tens of megabytes crossing the Session in both directions, SSA
//! cycles counted in the tens — and run to its natural end rather than to a target.
//!
//! # The run ends when the Entry runs out of money
//!
//! There is no cycle target and no clock. The Entry is funded with a fixed float, it
//! spends `price_per_byte × quota` of it per SSA, and when the next deposit is one it can
//! no longer afford the deposit fails, the Exit stops seeing money arrive, and its PIX
//! kill switch closes the Session with `ClosureReason::UnrealizedDeposit`. That is the
//! designed behaviour, so this test asserts it happens rather than treating it as a
//! failure — and it makes the run self-limiting:
//!
//! ```text
//! runtime ≈ bootstrap + (float / deposit_per_ssa) × (emissions_per_ssa / packet_rate)
//!                     + kill-switch fuse
//! ```
//!
//! Both terms on the right are knobs: `HOPRD_PIX_SOAK_FLOAT` buys cycles and
//! `HOPRD_PIX_SOAK_RATE` sets how fast each one is consumed. The default float is exactly
//! [`DEFAULT_FUNDED_CYCLES`] cycles' worth, which lands the whole run inside ~6 minutes.
//! To leave the cluster up for observation, fund it for longer:
//!
//! ```bash
//! HOPRD_PIX_SOAK_FLOAT="2000 wxHOPR" cargo nextest run -p hoprd-localcluster \
//!   --test session_pix_soak --run-ignored ignored-only -j 1 --no-capture
//! ```
//!
//! `--no-capture` matters: without it nextest buffers the progress reports until the run
//! ends, which for a large float is hours away.
//!
//! # Why the SSA is wide rather than the rate low
//!
//! An SSA cycle is bounded by *packets*, not by seconds: recovery needs `polys × shares`
//! shares, each riding one return-path SURB, so a cycle lasts `emissions / packet_rate`.
//! Two consequences shape the geometry here:
//!
//!   * **A cycle must outlast a deposit.** The Exit serves data on credit and reconstructs
//!     the key as soon as the shares are in, whether or not the money has arrived. If it
//!     wins that race, `sweep_recovered` finds a zero balance, logs "already swept", drops
//!     the entry, and the deposit is stranded at the stealth address for good. Raising the
//!     rate alone would shorten cycles into that race, so the SSA is widened to match —
//!     32 × 80 rather than 8 × 2, some 3840 packets and ~2.7 MB of billed quota per cycle,
//!     which measures out at ~19 s.
//!   * **The SURB buffer must stay below one SSA.** A share is baked into a SURB when the
//!     SURB is minted, and the Exit spends its buffer roughly in order, so the buffer is a
//!     pipeline delay between generating a share and delivering it. Sized at half an SSA
//!     it is about half a cycle; sized at `session_udp`'s 10 MB it would be ten cycles and
//!     nothing after the first would ever complete. The same reasoning fixes
//!     [`CHUNK_SIZE`] large enough that SURBs cannot piggyback on data packets, leaving
//!     the balancer as the only supply and so the only thing that sets the buffer.
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
//! ```bash
//! cargo build --release -p hoprd
//! export HOPRD_BIN=$(pwd)/target/release/hoprd
//! ```
//!
//! Each test must be run individually — see [`common`] for details.
//!
//! ```bash
//! export HOPRD_BIN=$(pwd)/target/release/hoprd
//! cargo nextest run -p hoprd-localcluster --test session_pix_soak \
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
use common::{ClusterCleanup, ClusterEnv, TempCluster};
use hopr_lib::api::types::primitive::prelude::HoprBalance;
use hoprd_localcluster::{client_helper, identity};
use tokio::net::UdpSocket;

const P2P_HOST: &str = "127.0.0.1";
const P2P_PORT_BASE: u16 = 19500;
const API_PORT_BASE: u16 = 13500;

const NUM_NODES: usize = 3;
const ENTRY: usize = 0;
const EXIT: usize = 2;
/// PIX requires at least one intermediate relay: the share encryption key comes from the
/// first relayer's acknowledgement.
const HOPS: u64 = 1;

// ── SSA geometry ────────────────────────────────────────────────────────────────
//
// Wide rather than deep. `ssa_part_size = 64` is the production default; raising it
// rather than `num_ssa_parts` buys quota without multiplying the number of independent
// polynomials the Exit has to track. Validation allows `num_ssa_parts` 8..=16192 and
// `ssa_part_size` 2..=4096.
const PIX_POLYS: u16 = 32;
const PIX_SHARES: u16 = 80;
/// Emitted beyond the threshold, per polynomial — the production ratio of `shares / 2`.
///
/// The generator's budget is finite: `shares + additional` per polynomial and no more, so
/// a polynomial that loses more than `additional` of its shares can never reach threshold
/// and the SSA stalls for good. `session_pix` can afford 2 because it only moves a few
/// dozen packets; at tens of thousands, a few percent loss would exhaust that on nearly
/// every polynomial. The surplus is delivered unbilled, which lengthens a cycle — here a
/// benefit, since a cycle has to outlast a deposit.
const PIX_ADDITIONAL_SHARES: usize = 40;

/// Datagrams per second in each direction.
///
/// Set just above the ~239/s the return path was measured to sustain here, so the Exit
/// always has something to echo without piling up datagrams it cannot answer. Nowhere
/// near a link limit — `session_udp` does ~4300/s over the same loopback — but the return
/// direction is the one that matters, because share delivery *is* the reply rate.
const DEFAULT_PACKET_RATE: u64 = 250;

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
/// across two runs (521 SURBs → 60/s, 1920 → 239/s) — which [`response_buffer`] sizes.
const CHUNK_SIZE: usize = 900;

// ── Run bounds ──────────────────────────────────────────────────────────────────

/// Cycles the default float pays for, and so the length of a default run.
///
/// The float is set to *exactly* this many deposits, which makes the closing assertions
/// exact: the Entry should spend all of it, and all but the last SSA or two should end up
/// swept into the Exit's Safe.
const DEFAULT_FUNDED_CYCLES: u64 = 10;

/// Ceiling when interpreting a Safe delta as a whole number of cycles; a bound on the
/// division, not an expectation.
const MAX_PLAUSIBLE_CYCLES: u64 = 100_000;
/// Round-trip payload a run must move for "multi-megabyte" to mean anything. Far under
/// what the default float implies (~26 MB), so it fails only on a real collapse in
/// throughput rather than on ordinary variance.
const MIN_TOTAL_BYTES: u64 = 4_000_000;
/// Share of paid-for return packets that must be observed completing the round trip.
/// The shortfall is packet loss downstream of the Exit, which still consumed the SURB.
const MIN_DELIVERED_PERCENT: u32 = 80;
/// SSAs that may legitimately be funded but not yet swept when the run ends.
///
/// The Exit requests the next SSA at the early-recovery threshold, so one is normally in
/// flight at any instant; at the end of the run the final one is cut short by the kill
/// switch before its shares complete.
const MAX_SSAS_IN_FLIGHT: u64 = 2;

/// Worst-case cycle duration used to derive the safety deadline. Deliberately far above
/// the ~8 s a cycle should take: the deadline exists to stop a wedged run, and tripping it
/// on a merely slow one buries the real numbers under a timeout.
const MAX_SECS_PER_CYCLE: u64 = 60;
/// Added to the safety deadline for the tail: the failing deposit's retry chain plus the
/// kill-switch fuse.
const KILL_SWITCH_TAIL: Duration = Duration::from_secs(180);
/// After the kill switch trips, how long to keep polling for in-flight sweeps to land.
const SETTLE_TIMEOUT: Duration = Duration::from_secs(60);
/// Wall-clock ceiling for a run at the default float, bootstrap included.
///
/// Only enforced when `HOPRD_PIX_SOAK_FLOAT` is unset: runtime scales with the float by
/// design, so a deliberately larger one is expected to take longer. Measured at ~400 s,
/// of which roughly half is cluster bootstrap.
const DEFAULT_RUN_BUDGET: Duration = Duration::from_secs(7 * 60);

const WAIT_TIMEOUT: Duration = Duration::from_secs(120);
const SETUP_TIMEOUT: Duration = Duration::from_secs(300);
const REPORT_INTERVAL: Duration = Duration::from_secs(5);

// ── Money ───────────────────────────────────────────────────────────────────────

/// A quota of ~1 MB makes this ~1.06 wxHOPR per SSA.
///
/// PIX pricing is its own model, unrelated to channel ticket pricing; it only has to sit
/// above the relay price re-counted per byte, which this does by roughly an order of
/// magnitude.
const PRICE_PER_BYTE: &str = "0.000001 wxHOPR";
/// Ceiling on one deposit. Below `price_per_byte × quota` the strategy refuses to deposit
/// at all, which would end the run on the first cycle instead of on the last.
const MAX_SSA_ALLOCATION: &str = "10 wxHOPR";
const GAS_XDAI_PER_SWEEP: &str = "0.01 xdai";
/// Per channel, out of each Safe's 1000 wxHOPR across two outgoing channels. Tens of
/// megabytes issue a lot of tickets, and a channel draining before the deposit float does
/// would stall packet flow and end the run for the wrong reason.
const CHANNEL_STAKE: &str = "400 wxHOPR";

// ── Exit deadlines ──────────────────────────────────────────────────────────────

/// Together these arm the PIX kill switch with a 45 s fuse — shorter than the 80 s
/// default, because here the fuse is not a safety net but part of the expected path, and
/// every run pays it once at the end. Still an order of magnitude above the couple of
/// seconds a healthy deposit takes.
const MAX_SSA_DELIVERY_TIME: Duration = Duration::from_secs(15);
const MAX_DEPOSIT_WAIT: Duration = Duration::from_secs(30);
/// Also fixes the Exit's deposit poll cadence at a tenth of this.
const MAX_DEPOSIT_TRACKING_TIME: Duration = Duration::from_secs(20);

/// Per-SSA quota in bytes implied by the dimensions above.
///
/// Mirrors [`identity::PixSettings::quota_per_ssa`], which cannot be used here because the
/// settings need the float, the float is derived from the per-cycle deposit, and that is
/// derived from the quota.
fn quota_bytes() -> u64 {
    PIX_POLYS as u64 * PIX_SHARES as u64 * hopr_lib::exports::transport::PACKET_PAYLOAD_SIZE as u64
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

fn packet_rate() -> u64 {
    std::env::var("HOPRD_PIX_SOAK_RATE")
        .ok()
        .and_then(|raw| raw.trim().parse::<u64>().ok())
        .filter(|rate| *rate > 0)
        .unwrap_or(DEFAULT_PACKET_RATE)
}

fn pix_settings(node_deposit_float: HoprBalance) -> identity::PixSettings {
    identity::PixSettings {
        num_ssa_parts: PIX_POLYS as usize,
        ssa_part_size: PIX_SHARES as usize,
        additional_shares: PIX_ADDITIONAL_SHARES,
        // Our ~1 MB quota sits far below the production window's lower bound, so the
        // window has to be opened up.
        quota_range_min: 0,
        quota_range_max: 8 * 1024 * 1024,
        max_ssa_delivery_time: MAX_SSA_DELIVERY_TIME,
        max_deposit_wait: MAX_DEPOSIT_WAIT,
        enforce_on_nodes: vec![EXIT],
        node_deposit_float,
    }
}

fn pix_strategy_env() -> anyhow::Result<client_helper::PixStrategyEnv> {
    Ok(client_helper::PixStrategyEnv {
        price_per_byte: PRICE_PER_BYTE.parse().context("parsing price per byte")?,
        max_ssa_allocation: MAX_SSA_ALLOCATION
            .parse()
            .context("parsing max SSA allocation")?,
        max_deposit_tracking_time: MAX_DEPOSIT_TRACKING_TIME,
        gas_xdai_per_sweep: GAS_XDAI_PER_SWEEP.parse().context("parsing sweep gas")?,
    })
}

/// Emissions the generator produces per SSA, and so the packets a cycle takes to deliver.
fn emissions_per_ssa() -> u64 {
    PIX_POLYS as u64 * (PIX_SHARES as u64 + PIX_ADDITIONAL_SHARES as u64)
}

/// SURB balancer target, at half an SSA's worth of replies.
///
/// hoprd converts this to `target_surb_buffer_size = bytes / SESSION_MTU`
/// (`rest-api::session`, `SessionConfig -> SurbBalancerConfig`), so it is expressed here
/// as a SURB count scaled back up rather than as a byte figure that happens to work out.
///
/// Half an SSA is the sizing rule from the module docs, and it is scale-free: the pipeline
/// delay a buffer imposes is `buffer / drain_rate` while a cycle is
/// `emissions / drain_rate`, so the delay is `buffer / emissions` of a cycle whatever the
/// rate. One full SSA of buffer would mean a share arriving only as its own SSA ends.
fn response_buffer() -> String {
    const SESSION_MTU: u64 = 1020;
    format!("{} B", emissions_per_ssa() / 2 * SESSION_MTU)
}

/// A UDP echo server for the Exit to forward Session payloads to, making Exit → Entry
/// volume mirror Entry → Exit volume.
async fn start_echo_server() -> anyhow::Result<u16> {
    let sock = UdpSocket::bind("127.0.0.1:0")
        .await
        .context("binding echo server")?;
    let port = sock.local_addr().context("echo server address")?.port();
    tokio::spawn(async move {
        let mut buf = vec![0u8; 65535];
        while let Ok((n, src)) = sock.recv_from(&mut buf).await {
            if sock.send_to(&buf[..n], src).await.is_err() {
                break;
            }
        }
    });
    Ok(port)
}

/// Whole SSA deposits represented by `delta`, or `None` when it is not an exact multiple
/// — which would mean something other than PIX moved the balance.
fn completed_cycles(delta: HoprBalance, per_cycle: HoprBalance) -> Option<u64> {
    if per_cycle.is_zero() {
        return None;
    }
    let n = delta.amount() / per_cycle.amount();
    if n > MAX_PLAUSIBLE_CYCLES.into() {
        return None;
    }
    let n = n.as_u64();
    (per_cycle * n == delta).then_some(n)
}

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
}

impl NodeMetrics {
    /// A node that cannot be scraped reports zeroes rather than failing the run: this
    /// feeds a progress display, and the authoritative signal is the on-chain balance.
    async fn scrape(api: &client_helper::HoprdApiClient) -> Self {
        let Ok(m) = api.metrics().await else {
            return Self::default();
        };
        let tracking = |outcome: &str| {
            m.sum_where(
                "hopr_strategy_pix_deposit_tracking_total",
                &format!(r#"outcome="{outcome}""#),
            ) as u64
        };
        let packets =
            |kind: &str| m.sum_where("hopr_packets_count", &format!(r#"type="{kind}""#)) as u64;
        Self {
            deposits: m.sum("hopr_strategy_pix_deposits_total") as u64,
            deposits_rejected: m.sum("hopr_strategy_pix_deposits_rejected_total") as u64,
            deposits_failed: m.sum("hopr_strategy_pix_deposits_failed_total") as u64,
            deposits_confirmed: tracking("confirmed"),
            deposits_timed_out: tracking("timeout"),
            keys_recovered: m.sum("hopr_strategy_pix_keys_recovered_total") as u64,
            sweeps: m.sum("hopr_strategy_pix_sweeps_total") as u64,
            packets_sent: packets("sent"),
            packets_received: packets("received"),
        }
    }
}

async fn setup_cluster(
    env: &ClusterEnv,
    cluster: &TempCluster,
    cleanup: &mut ClusterCleanup,
    settings: identity::PixSettings,
) -> anyhow::Result<()> {
    let t0 = Instant::now();
    let blk = common::start_chain(env, &cluster.log_dir, cleanup)
        .await
        .context("starting chain")?;
    common::wait_for_blokli_ready(&blk, WAIT_TIMEOUT)
        .await
        .context("waiting for blokli")?;
    tracing::info!("chain ready after {:?}", t0.elapsed());

    identity::generate(&identity::GenerationConfig {
        blokli_url: blk,
        num_nodes: NUM_NODES,
        config_home: cluster.data_dir.clone(),
        random_identities: true,
        p2p_host: P2P_HOST.to_string(),
        p2p_port_base: P2P_PORT_BASE,
        // AutoRedeeming stays off: redeemed tickets also credit the Safe in wxHOPR, which
        // would make the closing balance assertions ambiguous.
        strategies: identity::StrategySet {
            auto_redeeming: false,
            channel_lifecycle: false,
            pix: true,
        },
        strategy_execution_interval: Some(Duration::from_secs(600)),
        pix: Some(settings),
        ..Default::default()
    })
    .await
    .context("generating identities")?;
    tracing::info!("identities generated after {:?}", t0.elapsed());

    cleanup.nodes = client_helper::start_nodes(&client_helper::NodeStartConfig {
        num_nodes: NUM_NODES,
        hoprd_bin: &env.hoprd_bin,
        data_dir: &cluster.data_dir,
        log_dir: &cluster.log_dir,
        api_host: "127.0.0.1",
        api_port_base: API_PORT_BASE,
        p2p_host: P2P_HOST,
        p2p_port_base: P2P_PORT_BASE,
        identity_password: identity::DEFAULT_IDENTITY_PASSWORD,
        api_token: None,
        pix: Some(pix_strategy_env()?),
    })
    .await
    .context("starting nodes")?;
    tracing::info!("nodes started after {:?}", t0.elapsed());

    futures::future::try_join_all(
        cleanup
            .nodes
            .iter()
            .map(|n| n.api.wait_started(WAIT_TIMEOUT)),
    )
    .await
    .context("waiting for nodes to start")?;
    for n in &mut cleanup.nodes {
        n.address = Some(n.api.addresses().await.context("resolving node address")?);
    }
    futures::future::try_join_all(cleanup.nodes.iter().map(|n| n.api.wait_ready(WAIT_TIMEOUT)))
        .await
        .context("waiting for nodes to become ready")?;
    tracing::info!("nodes ready after {:?}", t0.elapsed());

    client_helper::open_full_mesh_channels(&cleanup.nodes, CHANNEL_STAKE, SETUP_TIMEOUT)
        .await
        .context("opening channels")?;
    client_helper::wait_full_mesh_channels(&cleanup.nodes, SETUP_TIMEOUT)
        .await
        .context("waiting for channels")?;
    client_helper::wait_full_mesh_reachable(&cleanup.nodes, SETUP_TIMEOUT)
        .await
        .context("waiting for peer reachability")?;
    tracing::info!("channels ready after {:?}", t0.elapsed());
    Ok(())
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

    let env = ClusterEnv::from_env().context("reading cluster environment")?;
    let cluster = TempCluster::new().context("creating temp cluster")?;
    let mut cleanup = ClusterCleanup {
        chain: None,
        nodes: vec![],
    };
    let t0 = Instant::now();

    let log_dir = cluster.log_dir.clone();
    let _log_guard = scopeguard::guard(log_dir.clone(), |logs| {
        let dest = std::path::Path::new("/tmp/pix-soak-logs");
        let _ = std::fs::create_dir_all(dest);
        if let Ok(entries) = std::fs::read_dir(&logs) {
            for e in entries.flatten() {
                let _ = std::fs::copy(e.path(), dest.join(e.file_name()));
            }
        }
    });

    // `ssa_polys`/`ssa_shares` are deliberately not named `polys`/`shares`: `pix-demo.sh`
    // greps this banner for `<name>=<number>` and takes the first match in the whole log, so
    // a field name that could occur in any other line would be read out of that line instead.
    tracing::info!(
        rate, quota, %price_per_byte, %per_cycle, %float, funded_cycles,
        ssa_polys = PIX_POLYS,
        ssa_shares = PIX_SHARES,
        emissions_per_ssa = emissions_per_ssa(),
        response_buffer = %response_buffer(),
        est_cycle_secs = emissions_per_ssa() as f64 / rate as f64,
        "PIX geometry: {PIX_POLYS} polys x {PIX_SHARES} shares = {quota} B per SSA at \
         {price_per_byte}/B = {per_cycle} per deposit; the run ends when {funded_cycles} \
         deposits have drained the float"
    );

    setup_cluster(&env, &cluster, &mut cleanup, pix_settings(float)).await?;

    let echo_port = start_echo_server().await?;
    let entry = &cleanup.nodes[ENTRY];
    let exit_node = &cleanup.nodes[EXIT];
    let exit_addr = exit_node
        .address
        .as_ref()
        .context("exit node address unresolved")?;
    let target = format!("127.0.0.1:{echo_port}");

    for node in &cleanup.nodes {
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
    // Interrupting a long run kills the process without unwinding, so the scopeguard that
    // copies these out never runs — say where they are while the run is still live.
    tracing::info!("node logs: {}", log_dir.display());

    // Snapshot after channel funding, so the stakes are already out of the Safes and the
    // only movement left is PIX.
    //
    // The two sides use different accounts: `SafePayloadGenerator::transfer` signs a
    // direct token transfer with the node key, so deposits leave the Entry's *node*
    // account, while `sweep_recovered` withdraws to the Exit's *Safe*.
    let entry_before = entry.api.balances().await.context("entry balances")?;
    let exit_before = exit_node.api.balances().await.context("exit balances")?;
    let entry_metrics_before = NodeMetrics::scrape(&entry.api).await;
    let exit_metrics_before = NodeMetrics::scrape(&exit_node.api).await;
    assert_eq!(
        entry_before.node_hopr, float,
        "the Entry node account holds {} rather than the {float} float it was configured \
         with; the run length would not match the funding",
        entry_before.node_hopr
    );
    tracing::info!(
        entry_node_hopr = %entry_before.node_hopr,
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
            pix_ssa_quota: Some((PIX_POLYS, PIX_SHARES)),
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

        let sent_n = sent.load(Ordering::Acquire);
        let echoed_n = echoed.load(Ordering::Acquire);
        let secs = traffic_started.elapsed().as_secs_f64().max(1.0);
        tracing::info!(
            elapsed = ?t0.elapsed(),
            cycles,
            funded_cycles,
            %recovered,
            entry_float = %entry_now.node_hopr,
            sent_mb = sent_n * CHUNK_SIZE as u64 / 1_000_000,
            echoed_mb = echoed_n * CHUNK_SIZE as u64 / 1_000_000,
            echo_pkt_s = format!("{:.0}", echoed_n as f64 / secs),
            entry_pkts_sent = entry_metrics
                .packets_sent
                .saturating_sub(entry_metrics_before.packets_sent),
            exit_pkts_recv = exit_metrics
                .packets_received
                .saturating_sub(exit_metrics_before.packets_received),
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
    // The condition is "no new sweep since the last poll" rather than
    // "keys_recovered == sweeps": the SSAs whose deposits failed still have their shares
    // completed by the traffic that keeps flowing during the kill-switch fuse, so their
    // keys are recovered but there is nothing at those addresses to sweep. Waiting for the
    // counts to converge would always burn the full timeout.
    let settle_until = Instant::now() + SETTLE_TIMEOUT;
    loop {
        let before = exit_metrics.sweeps;
        tokio::time::sleep(REPORT_INTERVAL).await;
        exit_metrics = NodeMetrics::scrape(&exit_node.api).await;
        if exit_metrics.sweeps == before || Instant::now() >= settle_until {
            break;
        }
    }
    entry_metrics = NodeMetrics::scrape(&entry.api).await;

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
    let spent = entry_before.node_hopr - entry_after.node_hopr;
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

    // The Entry spent its float down to what it could no longer afford, and every wxHOPR
    // left as a whole SSA deposit.
    let deposited_cycles = completed_cycles(spent, per_cycle).unwrap_or_else(|| {
        panic!(
            "the Entry node account paid out {spent}, which is not a whole multiple of the \
             {per_cycle} per-SSA deposit — something other than PIX deposits moved it"
        )
    });
    assert_eq!(
        deposited_cycles, funded_cycles,
        "the Entry was funded for {funded_cycles} deposits but made {deposited_cycles}, leaving \
         {} unspent. The run should end only once the float can no longer cover a deposit.",
        entry_after.node_hopr
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
    // Compared in packets rather than bytes: a quota is `polys × shares` packets priced at
    // the full `HoprPacket::PAYLOAD_SIZE`, whereas each datagram here carries CHUNK_SIZE,
    // so the two are not commensurable as byte counts. Nor is this an equality — a reply
    // dropped after leaving the Exit has still consumed its SURB and unlocked its share,
    // so paid-for volume legitimately exceeds observed volume by the loss rate. What it
    // rules out is the Exit being paid for traffic it never sent.
    let paid_packets = cycles * PIX_POLYS as u64 * PIX_SHARES as u64;
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
