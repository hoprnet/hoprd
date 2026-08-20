//! End-to-end PIX Session test (not for CI — run explicitly).
//!
//! Exercises the whole PIX stack at once — `hopr-lib` Sessions with PIX, `hoprd`
//! Session creation over the REST API, and the `NonAnonymousPix` strategy doing real
//! on-chain deposits and sweeps — which no other test does: `hopr-lib`'s
//! `transport_session_pix` fakes the deposit, and the strategy's own tests run against
//! a mock connector.
//!
//! The happy path being asserted:
//!
//!   1. Entry opens a PIX Session to the Exit through one relay.
//!   2. Entry's strategy deposits `price_per_byte × quota` to the SSA stealth address.
//!   3. Exit's strategy observes the deposit and defuses the PIX kill switch, so the
//!      Session survives.
//!   4. Bidirectional traffic delivers SSA shares on the return-path SURBs until the
//!      Exit reconstructs the stealth address private key.
//!   5. Exit's strategy sweeps the deposit into its Safe.
//!   6. Repeats across several SSA cycles.
//!
//! The closing assertion is exact rather than approximate: with auto-redeeming off, PIX
//! sweeps are the only thing that moves wxHOPR *into* the Exit's Safe, and sweep gas
//! leaves as xDai. So the Safe's wxHOPR gain must be a whole multiple of
//! `price_per_byte × quota` — which is precisely the statement that recovered funds
//! correspond to the data quota delivered from the Exit back to the Entry.
//!
//! Required (at least one chain source):
//!   HOPRD_CHAIN_URL   or HOPRD_CHAIN_IMAGE
//! Optional: HOPRD_BIN, HOPRD_CONTAINER_RUNTIME
//!
//! # Prerequisites
//!
//! The `hoprd` binary must be built in **release** mode. Debug builds slow packet
//! processing and cryptography enough to distort the SSA cycle pacing this test relies
//! on. It must also carry the **secp256k1 deposit pool**, which is not hoprd's default —
//! see [`session_pix_soak`](../session_pix_soak.rs) for why, and for the other pool.
//!
//! ```bash
//! cargo build --release -p hoprd --features strategy-pix-secp256k1
//! export HOPRD_BIN=$(pwd)/target/release/hoprd
//! ```
//!
//! Each test must be run individually — see [`common`] for details.
//!
//! ```bash
//! export HOPRD_BIN=$(pwd)/target/release/hoprd
//! cargo nextest run -p hoprd-localcluster --test session_pix \
//!   --run-ignored ignored-only -j 1
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
use common::{ClusterCleanup, ClusterEnv, TempCluster};
use hopr_lib::api::types::primitive::prelude::HoprBalance;
use hoprd_localcluster::{client_helper, identity};
use tokio::net::UdpSocket;

const P2P_HOST: &str = "127.0.0.1";
const P2P_PORT_BASE: u16 = 19400;
const API_PORT_BASE: u16 = 13400;

const NUM_NODES: usize = 3;
const ENTRY: usize = 0;
const EXIT: usize = 2;
/// Intermediate relays on each path. PIX requires at least one: the share encryption
/// key is derived from the first relayer's acknowledgement, so a zero-hop return path
/// has nothing to derive it from and the Session is refused outright.
const HOPS: u64 = 1;

/// PIX generator dimensions, both sitting on the protocol floor (`PixGlobalConfig`
/// validates `num_ssa_parts >= 8` and `ssa_part_size >= 2`). This makes the per-SSA
/// quota as small as the protocol allows, and therefore the test as short as possible.
///
/// The `u8`s are the upstream bound: the threshold and the surplus are one byte each of the
/// negotiated `PixParams` word.
const PIX_POLYS: u16 = 8;
const PIX_SHARES: u8 = 2;
/// Shares emitted beyond the threshold. Priced into the quota like any other emitted share,
/// so it costs both test time and wxHOPR; kept small for that reason.
const PIX_ADDITIONAL_SHARES: u8 = 2;

/// SSA cycles that must fully complete — deposit made, key recovered, funds swept.
const TARGET_CYCLES: u64 = 4;
/// Upper bound when interpreting a Safe balance delta as a whole number of cycles.
/// Only a sanity limit for the search; the test never expects to approach it.
const MAX_PLAUSIBLE_CYCLES: u64 = 1_000;
/// How far the Entry's deposits may legitimately run ahead of the Exit's recoveries.
///
/// The Exit requests the next SSA once the current one passes the early-recovery
/// threshold, and the Entry deposits for it immediately, so at any instant one SSA is
/// normally funded but not yet recovered. Two allows for the sample landing mid-handover.
const MAX_SSAS_IN_FLIGHT: u64 = 2;

const WAIT_TIMEOUT: Duration = Duration::from_secs(120);
const SETUP_TIMEOUT: Duration = Duration::from_secs(300);
/// Budget for `TARGET_CYCLES` deposits to make it all the way into the Exit's Safe,
/// measured from the moment the Session opens.
const RECOVERY_TIMEOUT: Duration = Duration::from_secs(240);
const BALANCE_POLL_INTERVAL: Duration = Duration::from_secs(2);

/// Payload per datagram, comfortably under SESSION_MTU so one datagram is one packet.
const CHUNK_SIZE: usize = 512;

/// Delay between datagrams.
///
/// This paces the Exit → Entry packet rate, which is what drives SSA share delivery:
/// the Exit consumes one return-path SURB per reply, and each SURB carries one share.
/// The generator emits shares polynomial-major at `PIX_SHARES + PIX_ADDITIONAL_SHARES`
/// per polynomial, so recovering an SSA takes on the order of
/// `PIX_POLYS × (PIX_SHARES + PIX_ADDITIONAL_SHARES)` replies — 32 here.
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
/// Deliberately small, and the least obvious knob in this test. A PIX share is baked
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
const MAX_SSA_ALLOCATION: &str = "10 wxHOPR";
const GAS_XDAI_PER_SWEEP: &str = "0.01 xdai";
/// Generous relative to what the traffic actually consumes; an underfunded channel
/// would stall packet flow and starve share delivery.
const CHANNEL_STAKE: &str = "50 wxHOPR";

/// Exit-side deadlines. Together these give the PIX kill switch an 80 s fuse.
const MAX_SSA_DELIVERY_TIME: Duration = Duration::from_secs(20);
const MAX_DEPOSIT_WAIT: Duration = Duration::from_secs(60);
/// How long the Exit's strategy keeps polling for a deposit. Also fixes the poll
/// cadence at a tenth of this, so 30 s means a 3 s cadence — the hoprd default of 1 h
/// would poll every 6 min and never beat the 80 s fuse.
const MAX_DEPOSIT_TRACKING_TIME: Duration = Duration::from_secs(30);

fn pix_settings() -> identity::PixSettings {
    identity::PixSettings {
        num_ssa_parts: PIX_POLYS as usize,
        ssa_part_size: PIX_SHARES as usize,
        additional_shares: PIX_ADDITIONAL_SHARES as usize,
        // The Exit rejects any quota outside this window. Ours is ~33.2 kB — the surplus is
        // in the product, see `quota_per_ssa` — against a production default window of
        // ~130 MiB–519 MiB, so it has to be widened.
        quota_range_min: 0,
        quota_range_max: 1024 * 1024,
        max_ssa_delivery_time: MAX_SSA_DELIVERY_TIME,
        max_deposit_wait: MAX_DEPOSIT_WAIT,
        // Only the Exit refuses non-PIX Sessions; the relay never terminates one.
        enforce_on_nodes: vec![EXIT],
        // ~60 SSA deposits at this test's ~1.66 wxHOPR per cycle; it needs four.
        node_deposit_float: "100 wxHOPR".parse().expect("valid static amount"),
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

/// A UDP echo server for the Exit to forward Session payloads to. Echoing means the
/// Exit → Entry volume matches the Entry → Exit volume, so pacing the sender paces
/// share delivery.
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

/// Number of whole SSA deposits that `delta` represents, or `None` when it is not an
/// exact multiple — which would mean something other than PIX sweeps moved the Safe.
fn completed_cycles(delta: HoprBalance, per_cycle: HoprBalance) -> Option<u64> {
    (0..=MAX_PLAUSIBLE_CYCLES).find(|n| per_cycle * *n == delta)
}

/// Count occurrences of `needle` in node `id`'s hoprd log.
///
/// Fallible on purpose. Two of the assertions below *are* primary assertions on these counts, and
/// a missing or unreadable log would otherwise report zero — which reads as "the Exit never gave
/// up waiting" when the truth is "the test never looked", and makes the companion assertion fail
/// while blaming the Exit for a file-system problem.
fn count_in_node_log(log_dir: &std::path::Path, id: usize, needle: &str) -> anyhow::Result<usize> {
    let path = log_dir.join(format!("hoprd_{id}.log"));
    let log = std::fs::read_to_string(&path)
        .with_context(|| format!("reading {} to count {needle:?}", path.display()))?;
    Ok(log.matches(needle).count())
}

async fn setup_cluster(
    env: &ClusterEnv,
    cluster: &TempCluster,
    cleanup: &mut ClusterCleanup,
) -> anyhow::Result<()> {
    let t0 = std::time::Instant::now();
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
        // AutoRedeeming is deliberately off: ticket redemption also credits the Safe in
        // wxHOPR, which would make the closing balance assertion ambiguous.
        strategies: identity::StrategySet {
            auto_redeeming: false,
            channel_lifecycle: false,
            pix: true,
        },
        strategy_execution_interval: Some(Duration::from_secs(600)),
        pix: Some(pix_settings()),
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

#[tokio::test]
#[ignore = "requires external chain container and hoprd binary \u{2014} run explicitly, not in CI"]
async fn localcluster_pix_session_sweeps_recovered_deposits_into_exit_safe() -> anyhow::Result<()> {
    common::init_tracing();
    let env = ClusterEnv::from_env().context("reading cluster environment")?;
    let cluster = TempCluster::new().context("creating temp cluster")?;
    let mut cleanup = ClusterCleanup {
        chain: None,
        nodes: vec![],
    };
    let t0 = std::time::Instant::now();

    let log_dir = cluster.log_dir.clone();
    let _log_guard = scopeguard::guard(log_dir.clone(), |logs| {
        let dest = std::path::Path::new("/tmp/pix-session-logs");
        let _ = std::fs::create_dir_all(dest);
        if let Ok(entries) = std::fs::read_dir(&logs) {
            for e in entries.flatten() {
                let _ = std::fs::copy(e.path(), dest.join(e.file_name()));
            }
        }
    });

    setup_cluster(&env, &cluster, &mut cleanup).await?;

    let settings = pix_settings();
    let quota = settings.quota_per_ssa();
    let price_per_byte: HoprBalance = PRICE_PER_BYTE.parse().context("parsing price per byte")?;
    let per_cycle = price_per_byte * quota;
    let target_total = per_cycle * TARGET_CYCLES;
    tracing::info!(
        %price_per_byte, quota, %per_cycle, %target_total, TARGET_CYCLES,
        "PIX accounting: one SSA cycle costs price_per_byte x quota"
    );

    let echo_port = start_echo_server().await?;
    let entry = &cleanup.nodes[ENTRY];
    let exit_node = &cleanup.nodes[EXIT];
    let exit_addr = exit_node
        .address
        .as_ref()
        .context("exit node address unresolved")?;
    let target = format!("127.0.0.1:{echo_port}");
    tracing::info!(entry = entry.id, exit = %exit_addr, %target, "topology");

    // Snapshot after channel funding so the stakes are already out of the Safes and the
    // only subsequent movement is PIX.
    //
    // The two sides use different accounts, which is not symmetric and easy to get
    // wrong: `SafePayloadGenerator::transfer` signs a direct `HoprToken.transfer` with
    // the node key, so the Entry's deposits leave its *node* account, while
    // `sweep_recovered` calls `withdraw_from_signer(.., &safe_address)`, so the Exit's
    // recoveries land in its *Safe*.
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
            // The SURB balancer is what carries SSA shares to the Exit. See
            // RESPONSE_BUFFER: oversizing it delays share delivery by a whole buffer's
            // worth of replies, which stalls every cycle after the first.
            response_buffer: Some(RESPONSE_BUFFER.to_string()),
            max_surb_upstream: Some(MAX_SURB_UPSTREAM.to_string()),
            // Must match this node's own generator dimensions, or the Session is refused.
            pix_ssa_quota: Some((PIX_POLYS, PIX_SHARES, PIX_ADDITIONAL_SHARES)),
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
    let deadline = std::time::Instant::now() + RECOVERY_TIMEOUT;
    let mut recovered = HoprBalance::default();
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
            cycles = completed_cycles(recovered, per_cycle).unwrap_or(0),
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

    let deposits_made = count_in_node_log(&log_dir, ENTRY, "deposit successful")?;
    let keys_recovered = count_in_node_log(&log_dir, EXIT, "private key recovered")?;
    let deposits_seen = count_in_node_log(&log_dir, EXIT, "SSA deposit successful")?;
    let deposits_missed = count_in_node_log(&log_dir, EXIT, "deposit confirmation timed out")?;
    let echoed = echoed.load(Ordering::Acquire);

    // ── Assertions ──────────────────────────────────────────────────────────────
    assert!(
        echoed > 0,
        "no datagram completed the Entry -> Exit -> echo -> Entry round trip, so the PIX \
         Session never carried traffic (deposits made: {deposits_made}, keys recovered: \
         {keys_recovered})"
    );

    // "The Exit sees the deposit and does not kill the Session": the deposit awaiter
    // logs one line per SSA when it confirms a deposit and defuses the kill switch, and
    // a different one when it gives up and lets the switch fire.
    //
    // These read hoprd's own logs, which default to `info`; a `RUST_LOG` that filters
    // info out of the child processes would blank both counts.
    assert_eq!(
        deposits_missed, 0,
        "the Exit gave up waiting for {deposits_missed} deposit(s) and let the PIX kill \
         switch close the Session (it confirmed {deposits_seen}). Either the Entry's \
         strategy never deposited, or the deposit landed outside the \
         max_deposit_wait + max_ssa_delivery_time window."
    );
    assert!(
        deposits_seen >= TARGET_CYCLES as usize,
        "the Exit confirmed only {deposits_seen} deposits, expected at least \
         {TARGET_CYCLES} (Entry logged {deposits_made} deposits made)"
    );

    // An exact multiple is the real check: it says every wxHOPR that entered the Exit's
    // Safe arrived as a whole SSA deposit of `price_per_byte x quota`, i.e. the recovered
    // funds correspond exactly to the data quota delivered back to the Entry.
    let cycles = completed_cycles(recovered, per_cycle).unwrap_or_else(|| {
        panic!(
            "Exit Safe gained {recovered}, which is not a whole multiple of the {per_cycle} \
             per-SSA deposit — something other than PIX sweeps moved the balance \
             (deposits made: {deposits_made}, keys recovered: {keys_recovered})"
        )
    });

    assert!(
        cycles >= TARGET_CYCLES,
        "expected at least {TARGET_CYCLES} completed SSA cycles, got {cycles} ({recovered} of \
         the {target_total} target) after {:?}. Entry logged {deposits_made} deposits, Exit \
         recovered {keys_recovered} keys, {echoed} datagrams echoed. A recovered-key count \
         above the cycle count means keys were reconstructed before their deposits were \
         mined and the funds are stranded at the stealth addresses — slow SEND_INTERVAL down.",
        t0.elapsed()
    );

    // The Entry funded every one of those deposits out of its own node account.
    //
    // Not an equality: the Exit requests the next SSA at the early-recovery threshold
    // (~85% of shares), so the Entry has already deposited for SSAs still in flight by
    // the time this samples. `spent` therefore runs a cycle or two ahead of `recovered`.
    // What must hold is that the outflow is *only* whole PIX deposits, and that the
    // Entry is not paying for SSAs that never complete.
    let entry_after = entry
        .api
        .balances()
        .await
        .context("reading entry balances after")?;
    let spent = entry_before.node_hopr - entry_after.node_hopr;
    let deposited_cycles = completed_cycles(spent, per_cycle).unwrap_or_else(|| {
        panic!(
            "the Entry node account paid out {spent}, which is not a whole multiple of the \
             {per_cycle} per-SSA deposit — something other than PIX deposits moved it"
        )
    });
    assert!(
        (cycles..=cycles + MAX_SSAS_IN_FLIGHT).contains(&deposited_cycles),
        "the Entry deposited for {deposited_cycles} SSAs but only {cycles} were recovered and \
         swept. Up to {MAX_SSAS_IN_FLIGHT} may legitimately be in flight thanks to \
         early-recovery pipelining; more than that means deposits are being made for SSAs \
         that never complete."
    );

    entry
        .api
        .close_client(&ip, port)
        .await
        .context("closing the session listener")?;

    tracing::info!(
        cycles, %recovered, deposits_made, deposits_seen, keys_recovered, echoed,
        "PIX session test PASSED in {:?}", t0.elapsed()
    );
    Ok(())
}
