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
//! sweeps are the only thing that moves wxHOPR *into* the Exit's Safe. The Safe's gain
//! must therefore be a whole multiple of the net per-cycle sweep. That equals
//! `price_per_byte × quota` for the transparent pool and the same gross allocation minus
//! the live Curvy withdrawal fees for the private pool.
//!
//! Successful Nextest output is normally captured. The Curvy scenario therefore also writes
//! `/tmp/pix-session-logs/pix-measurements.json`, including bootstrap, TTFB, settlement,
//! traffic and per-circuit proof-phase timings. Use `--success-output final` to print the
//! human-readable tracing output for a passing run.
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
//! nix develop -c cargo build --release -p hoprd --features strategy-pix-secp256k1
//! export HOPRD_BIN=$(pwd)/target/release/hoprd
//! ```
//!
//! Each test must be run individually — see [`common`] for details.
//!
//! ```bash
//! export HOPRD_BIN=$(pwd)/target/release/hoprd
//! nix develop -c cargo nextest run -p hoprd-localcluster --test session_pix \
//!   --run-ignored ignored-only -j 1
//! ```

#[path = "common/mod.rs"]
mod common;

use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    time::Duration,
};

use anyhow::Context;
#[cfg(feature = "strategy-pix-curvy")]
use anyhow::ensure;
use common::{Cluster, ClusterSpec, pix::completed_cycles, ports};
use hopr_lib::api::types::primitive::prelude::HoprBalance;
#[cfg(feature = "strategy-pix-curvy")]
use hopr_lib::api::types::primitive::prelude::U256;
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
/// How far the Entry's deposits may legitimately run ahead of the Exit's recoveries.
///
/// The Exit requests the next SSA once the current one passes the early-recovery
/// threshold, and the Entry deposits for it immediately, so at any instant one SSA is
/// normally funded but not yet recovered. Two allows for the sample landing mid-handover.
#[cfg(feature = "strategy-pix-secp256k1")]
const MAX_SSAS_IN_FLIGHT: u64 = 2;

const WAIT_TIMEOUT: Duration = Duration::from_secs(120);
const SETUP_TIMEOUT: Duration = Duration::from_secs(300);
/// Budget for `TARGET_CYCLES` deposits to make it all the way into the Exit's Safe,
/// measured from the moment the Session opens.
const RECOVERY_TIMEOUT: Duration = Duration::from_secs(240);
const BALANCE_POLL_INTERVAL: Duration = Duration::from_secs(2);
/// HOPR's acceptance requirement, measured from the client's session-open request until
/// the first non-empty payload completes Entry -> Exit -> echo -> Entry. Chain, identity,
/// node and channel bootstrap are reported separately and deliberately excluded.
const TIME_TO_FIRST_BYTE_SLO: Duration = Duration::from_secs(60);
const TTFB_NOT_OBSERVED: u64 = u64::MAX;
const PIX_LOG_DEST: &str = "/tmp/pix-session-logs";

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
#[cfg(feature = "strategy-pix-secp256k1")]
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

#[cfg(feature = "strategy-pix-curvy")]
const CURVY_TOKEN_ID: u64 = 3;

#[cfg(feature = "strategy-pix-curvy")]
fn decimal_u128_field(node: &serde_json::Value, field: &str) -> anyhow::Result<u128> {
    // Blokli's UInt256 scalar is emitted as a decimal string. Do not parse it through
    // Alloy's U256 `FromStr`: an unprefixed "20" is accepted there as hexadecimal and
    // becomes 32, which made this test charge 32 bps for a configured 20-bps fee.
    node.get(field)
        .and_then(serde_json::Value::as_str)
        .with_context(|| format!("Blokli response has no decimal {field}: {node}"))?
        .parse::<u128>()
        .with_context(|| format!("Blokli returned an invalid {field}"))
}

/// Net amount one Curvy withdrawal credits to the Exit Safe.
///
/// PIX allocates the gross `price_per_byte * quota` inside Curvy, but the Vault deducts
/// its proportional withdrawal fee and the token-specific withdrawal gas reimbursement
/// before transferring to the Safe. Read both values from the same chain the test uses so
/// the accounting assertion follows deployment configuration instead of localnet constants.
#[cfg(feature = "strategy-pix-curvy")]
async fn curvy_sweep_per_cycle(
    blokli_url: &str,
    gross: HoprBalance,
) -> anyhow::Result<HoprBalance> {
    const FEE_DENOMINATOR: u128 = 10_000;
    const QUERY: &str = r#"
query ($tokenId: UInt256!) {
  curvyVaultFees {
    __typename
    ... on CurvyVaultFees { withdrawalFee }
    ... on QueryFailedError { code message }
  }
  curvyVaultToken(tokenId: $tokenId) {
    __typename
    ... on CurvyVaultToken { gasFees { withdrawal } }
    ... on QueryFailedError { code message }
  }
}
"#;

    let response = reqwest::Client::new()
        .post(format!("{}/graphql", blokli_url.trim_end_matches('/')))
        .json(&serde_json::json!({
            "query": QUERY,
            "variables": { "tokenId": CURVY_TOKEN_ID.to_string() },
        }))
        .send()
        .await
        .context("querying Blokli for Curvy withdrawal fees")?
        .error_for_status()
        .context("Blokli rejected the Curvy withdrawal-fee query")?
        .json::<serde_json::Value>()
        .await
        .context("decoding Blokli Curvy withdrawal-fee response")?;
    ensure!(
        response.get("errors").is_none(),
        "Blokli Curvy withdrawal-fee query failed: {response}"
    );
    let data = response
        .get("data")
        .context("Blokli Curvy withdrawal-fee response has no data")?;
    let vault = data
        .get("curvyVaultFees")
        .context("Blokli response has no curvyVaultFees")?;
    let token = data
        .get("curvyVaultToken")
        .context("Blokli response has no curvyVaultToken")?;
    ensure!(
        vault.get("__typename").and_then(serde_json::Value::as_str) == Some("CurvyVaultFees"),
        "Blokli could not read Curvy Vault fees: {vault}"
    );
    ensure!(
        token.get("__typename").and_then(serde_json::Value::as_str) == Some("CurvyVaultToken"),
        "Blokli could not read Curvy token {CURVY_TOKEN_ID}: {token}"
    );

    let withdrawal_fee_bps = decimal_u128_field(vault, "withdrawalFee")?;
    let withdrawal_gas = decimal_u128_field(
        token
            .get("gasFees")
            .context("Blokli Curvy token response has no gasFees")?,
        "withdrawal",
    )?;
    ensure!(
        withdrawal_fee_bps <= FEE_DENOMINATOR,
        "Curvy withdrawal fee {withdrawal_fee_bps} exceeds the denominator"
    );
    let proportional_fee =
        gross.amount() * U256::from(withdrawal_fee_bps) / U256::from(FEE_DENOMINATOR);
    let total_fee = proportional_fee + U256::from(withdrawal_gas);
    ensure!(
        gross.amount() > total_fee,
        "Curvy withdrawal fees {total_fee} consume the {gross} SSA allocation"
    );
    Ok(HoprBalance::from(gross.amount() - total_fee))
}

fn pix_settings() -> anyhow::Result<identity::PixSettings> {
    Ok(identity::PixSettings {
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
        // Settlement knobs. These used to travel as environment variables; they are written
        // into the generated node config's `Pix` strategy stanza now.
        price_per_byte: PRICE_PER_BYTE.parse().context("parsing price per byte")?,
        max_ssa_allocation: MAX_SSA_ALLOCATION
            .parse()
            .context("parsing max SSA allocation")?,
        max_deposit_tracking_time: MAX_DEPOSIT_TRACKING_TIME,
        #[cfg(feature = "strategy-pix-secp256k1")]
        gas_xdai_per_sweep: GAS_XDAI_PER_SWEEP.parse().context("parsing sweep gas")?,
    })
}

/// A UDP echo server for the Exit to forward Session payloads to. Echoing means the
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

#[cfg(feature = "strategy-pix-curvy")]
fn duration_millis(duration: Duration) -> u64 {
    duration.as_millis().try_into().unwrap_or(u64::MAX)
}

#[cfg(feature = "strategy-pix-curvy")]
#[derive(Clone, Debug, serde::Deserialize)]
struct ProofTiming {
    #[serde(default)]
    circuit: String,
    graph_load_ms: u64,
    prover_load_ms: u64,
    witness_ms: u64,
    groth16_ms: u64,
    total_ms: u64,
}

#[cfg(feature = "strategy-pix-curvy")]
fn proof_timings_in_node_sidecar(
    log_dir: &std::path::Path,
    id: usize,
    circuit: &str,
    after_circuit: Option<&str>,
) -> anyhow::Result<Vec<ProofTiming>> {
    let path = log_dir.join(format!("curvy_proof_timings_{id}.jsonl"));
    let log = std::fs::read_to_string(&path).with_context(|| {
        format!(
            "reading {} for persisted {circuit} proof timings",
            path.display()
        )
    })?;
    let records = log
        .lines()
        .enumerate()
        .filter(|(_, line)| !line.trim().is_empty())
        .map(|(line_number, line)| {
            serde_json::from_str::<ProofTiming>(line).with_context(|| {
                format!(
                    "decoding Curvy proof timing record {}:{}",
                    path.display(),
                    line_number + 1
                )
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let start = after_circuit
        .map(|marker| {
            records
                .iter()
                .position(|record| record.circuit == marker)
                .with_context(|| {
                    format!(
                        "{} has no {marker} session proof before {circuit} timings",
                        path.display()
                    )
                })
        })
        .transpose()?
        .unwrap_or_default();
    Ok(records
        .into_iter()
        .skip(start)
        .filter(|record| record.circuit == circuit)
        .collect())
}

#[cfg(feature = "strategy-pix-curvy")]
fn millisecond_stats(values: impl Iterator<Item = u64>) -> serde_json::Value {
    let mut values = values.collect::<Vec<_>>();
    values.sort_unstable();
    let total = values.iter().copied().sum::<u64>();
    let percentile = |percent: usize| {
        (!values.is_empty()).then(|| {
            let rank = values.len().saturating_mul(percent).div_ceil(100);
            values[rank.saturating_sub(1).min(values.len() - 1)]
        })
    };
    serde_json::json!({
        "count": values.len(),
        "min": values.iter().copied().min(),
        "mean": (!values.is_empty()).then(|| total as f64 / values.len() as f64),
        "p50": percentile(50),
        "p95": percentile(95),
        "max": values.iter().copied().max(),
        "total": total,
    })
}

#[cfg(feature = "strategy-pix-curvy")]
fn proof_timing_report(samples: &[ProofTiming]) -> serde_json::Value {
    serde_json::json!({
        "phases_ms": {
            "graph_load": millisecond_stats(samples.iter().map(|sample| sample.graph_load_ms)),
            "prover_load_and_authenticate": millisecond_stats(samples.iter().map(|sample| sample.prover_load_ms)),
            "witness_generation": millisecond_stats(samples.iter().map(|sample| sample.witness_ms)),
            "groth16_proving": millisecond_stats(samples.iter().map(|sample| sample.groth16_ms)),
            "total": millisecond_stats(samples.iter().map(|sample| sample.total_ms)),
        },
        "samples_ms": samples.iter().map(|sample| serde_json::json!({
            "graph_load": sample.graph_load_ms,
            "prover_load_and_authenticate": sample.prover_load_ms,
            "witness_generation": sample.witness_ms,
            "groth16_proving": sample.groth16_ms,
            "total": sample.total_ms,
        })).collect::<Vec<_>>(),
    })
}

#[cfg(feature = "strategy-pix-curvy")]
fn log_proof_timing_summary(circuit: &str, samples: &[ProofTiming]) {
    if samples.is_empty() {
        return;
    }
    let mean = |value: fn(&ProofTiming) -> u64| {
        samples.iter().map(value).sum::<u64>() as f64 / samples.len() as f64
    };
    let mut totals = samples
        .iter()
        .map(|sample| sample.total_ms)
        .collect::<Vec<_>>();
    totals.sort_unstable();
    let percentile = |percent: usize| {
        let rank = totals.len().saturating_mul(percent).div_ceil(100);
        totals[rank.saturating_sub(1).min(totals.len() - 1)]
    };
    tracing::info!(
        circuit,
        samples = samples.len(),
        graph_load_mean_ms = mean(|sample| sample.graph_load_ms),
        prover_load_mean_ms = mean(|sample| sample.prover_load_ms),
        witness_mean_ms = mean(|sample| sample.witness_ms),
        groth16_mean_ms = mean(|sample| sample.groth16_ms),
        total_mean_ms = mean(|sample| sample.total_ms),
        total_p50_ms = percentile(50),
        total_p95_ms = percentile(95),
        total_max_ms = totals.last().copied().unwrap_or_default(),
        "Curvy proof timing summary"
    );
}

#[cfg(feature = "strategy-pix-curvy")]
fn finalize_proof_timing_report(log_dir: &std::path::Path) -> anyhow::Result<()> {
    // Entry performs one startup pending-note proof while shielding its private-pool float.
    // The first PIX aggregation unambiguously marks the session phase, so pending timings are
    // selected only from that record onward. PIX aggregation and withdrawal are session-only.
    let pix_aggregation = proof_timings_in_node_sidecar(log_dir, ENTRY, "pix-aggregation", None)?;
    let pending_commitment =
        proof_timings_in_node_sidecar(log_dir, ENTRY, "pending", Some("pix-aggregation"))?;
    let pix_withdrawal = proof_timings_in_node_sidecar(log_dir, EXIT, "pix-withdrawal", None)?;

    ensure!(
        !pix_aggregation.is_empty() && !pending_commitment.is_empty() && !pix_withdrawal.is_empty(),
        "finalized Curvy proof timing is incomplete: {} aggregation, {} pending and {} \
         withdrawal samples; verify that hoprd sets `CURVY_PROOF_TIMINGS_PATH` and uses the \
         instrumented `curvy-witnesscalc`",
        pix_aggregation.len(),
        pending_commitment.len(),
        pix_withdrawal.len(),
    );
    log_proof_timing_summary("entry-pix-aggregation", &pix_aggregation);
    log_proof_timing_summary("entry-pending-commitment", &pending_commitment);
    log_proof_timing_summary("exit-pix-withdrawal", &pix_withdrawal);

    let path = log_dir.join("pix-measurements.json");
    let mut measurements =
        serde_json::from_slice::<serde_json::Value>(&std::fs::read(&path).with_context(|| {
            format!(
                "reading preliminary PIX measurements from {}",
                path.display()
            )
        })?)
        .context("decoding preliminary PIX measurements")?;
    measurements
        .as_object_mut()
        .context("PIX measurements root is not an object")?
        .insert(
            "proof_timing".to_owned(),
            serde_json::json!({
                "entry_pix_aggregation": proof_timing_report(&pix_aggregation),
                "entry_pending_commitment": proof_timing_report(&pending_commitment),
                "exit_pix_withdrawal": proof_timing_report(&pix_withdrawal),
            }),
        );
    std::fs::write(
        &path,
        serde_json::to_vec_pretty(&measurements).context("encoding finalized PIX measurements")?,
    )
    .with_context(|| format!("writing finalized PIX measurements to {}", path.display()))?;
    tracing::info!(
        path = %path.display(),
        pix_aggregation_proofs = pix_aggregation.len(),
        pending_commitment_proofs = pending_commitment.len(),
        pix_withdrawal_proofs = pix_withdrawal.len(),
        "finalized detailed PIX timing report"
    );
    Ok(())
}

// `multi_thread`, matching `session_pix_soak`: the correctness of this test rests on packet
// pacing. `SEND_INTERVAL` is a floor, not a rate, and a current-thread runtime multiplexes the
// sender, the echo server task and the balance poller onto one thread — contention stretches the
// interval, and a cycle that outruns its deposit leaves the Exit recovering a key against a zero
// balance, logging "already swept" and stranding the funds. `RECOVERY_TIMEOUT` absorbs the
// stretch until it does not.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires external chain container and hoprd binary \u{2014} run explicitly, not in CI"]
async fn localcluster_pix_session_sweeps_recovered_deposits_into_exit_safe() -> anyhow::Result<()> {
    common::init_tracing();
    let t0 = std::time::Instant::now();

    let cluster = Cluster::start(ClusterSpec {
        num_nodes: NUM_NODES,
        // AutoRedeeming is deliberately off: ticket redemption also credits the Safe in
        // wxHOPR, which would make the closing balance assertion ambiguous.
        strategies: identity::StrategySet {
            auto_redeeming: false,
            channel_lifecycle: false,
        },
        strategy_execution_interval: Some(Duration::from_secs(600)),
        pix: Some(pix_settings()?),
        logs_to: Some(PIX_LOG_DEST),
        ..ClusterSpec::new(ports::SESSION_PIX)
    })
    .await?;
    let cluster_started = t0.elapsed();
    cluster.wait_ready(WAIT_TIMEOUT).await?;
    let nodes_ready = t0.elapsed();
    cluster.open_channels(CHANNEL_STAKE, SETUP_TIMEOUT).await?;
    cluster.wait_channels(SETUP_TIMEOUT).await?;
    cluster.wait_reachable(SETUP_TIMEOUT).await?;
    let channels_ready = t0.elapsed();
    tracing::info!("channels ready after {:?}", t0.elapsed());
    let log_dir = cluster.log_dir().to_path_buf();

    let settings = pix_settings()?;
    let quota = settings.quota_per_ssa();
    let price_per_byte: HoprBalance = PRICE_PER_BYTE.parse().context("parsing price per byte")?;
    let per_cycle = price_per_byte * quota;
    #[cfg(feature = "strategy-pix-secp256k1")]
    let swept_per_cycle = per_cycle;
    #[cfg(feature = "strategy-pix-curvy")]
    let swept_per_cycle = curvy_sweep_per_cycle(cluster.blokli_url(), per_cycle).await?;
    let target_total = swept_per_cycle * TARGET_CYCLES;
    tracing::info!(
        %price_per_byte, quota, %per_cycle, %swept_per_cycle, %target_total, TARGET_CYCLES,
        "PIX accounting: one SSA cycle costs price_per_byte x quota"
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

    let session_requested = std::time::Instant::now();
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
            pix_ssa_quota: Some(hoprd_api_client::types::PixSsaQuota {
                polys_per_ssa: PIX_POLYS,
                shares_per_poly: PIX_SHARES,
                surplus_shares: PIX_ADDITIONAL_SHARES,
            }),
        })
        .await
        .context("opening PIX session")?;
    let session_established = session_requested.elapsed();
    tracing::info!(
        ?session_established,
        total_elapsed = ?t0.elapsed(),
        "PIX session listening on {ip}:{port}"
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
    let sent_datagrams = Arc::new(AtomicU64::new(0));
    let sent_bytes = Arc::new(AtomicU64::new(0));
    let received_datagrams = Arc::new(AtomicU64::new(0));
    let received_bytes = Arc::new(AtomicU64::new(0));
    let echoed = Arc::new(AtomicU64::new(0));
    let first_send_millis = Arc::new(AtomicU64::new(TTFB_NOT_OBSERVED));
    let first_byte_millis = Arc::new(AtomicU64::new(TTFB_NOT_OBSERVED));

    let payload: Vec<u8> = (0..CHUNK_SIZE).map(|i| (i % 256) as u8).collect();
    let sender = tokio::spawn({
        let (sock, stop, sent_datagrams, sent_bytes, first_send_millis, payload) = (
            sock.clone(),
            stop.clone(),
            sent_datagrams.clone(),
            sent_bytes.clone(),
            first_send_millis.clone(),
            payload.clone(),
        );
        async move {
            while !stop.load(Ordering::Acquire) {
                match sock.send(&payload).await {
                    Ok(n) => {
                        let elapsed = session_requested.elapsed().as_millis() as u64;
                        let _ = first_send_millis.compare_exchange(
                            TTFB_NOT_OBSERVED,
                            elapsed,
                            Ordering::AcqRel,
                            Ordering::Acquire,
                        );
                        sent_datagrams.fetch_add(1, Ordering::Release);
                        sent_bytes.fetch_add(n as u64, Ordering::Release);
                    }
                    Err(_) => break,
                }
                tokio::time::sleep(SEND_INTERVAL).await;
            }
        }
    });
    let receiver = tokio::spawn({
        let (sock, stop, received_datagrams, received_bytes, echoed, first_byte_millis) = (
            sock.clone(),
            stop.clone(),
            received_datagrams.clone(),
            received_bytes.clone(),
            echoed.clone(),
            first_byte_millis.clone(),
        );
        async move {
            let mut buf = vec![0u8; 65535];
            while !stop.load(Ordering::Acquire) {
                match tokio::time::timeout(Duration::from_secs(30), sock.recv(&mut buf)).await {
                    Ok(Ok(n)) => {
                        received_datagrams.fetch_add(1, Ordering::Release);
                        received_bytes.fetch_add(n as u64, Ordering::Release);
                        if n > 0 {
                            let elapsed = session_requested.elapsed().as_millis() as u64;
                            let _ = first_byte_millis.compare_exchange(
                                TTFB_NOT_OBSERVED,
                                elapsed,
                                Ordering::AcqRel,
                                Ordering::Acquire,
                            );
                        }
                        if buf[..n] == payload[..] {
                            echoed.fetch_add(1, Ordering::Release);
                        } else {
                            // A short, combined or corrupted echo is a Session-layer failure,
                            // but the balance assertion below is the real verdict — just note it.
                            tracing::warn!(n, "unexpected echo payload");
                        }
                    }
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
    let mut first_safe_credit = None;
    let mut target_recovered = None;
    #[cfg(feature = "strategy-pix-curvy")]
    let mut settlement_milestones = Vec::new();
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
        if !recovered.is_zero() && first_safe_credit.is_none() {
            first_safe_credit = Some(session_requested.elapsed());
        }
        if let Some(observed_cycles) = completed_cycles(recovered, swept_per_cycle)
            && observed_cycles > cycles_seen
        {
            #[cfg(feature = "strategy-pix-curvy")]
            settlement_milestones.push((observed_cycles, session_requested.elapsed()));
            tracing::info!(
                cycles = observed_cycles,
                %recovered,
                since_session_request = ?session_requested.elapsed(),
                "PIX settlement milestone"
            );
            cycles_seen = observed_cycles;
        }
        tracing::info!(
            %recovered,
            cycles = cycles_seen,
            echoed = echoed.load(Ordering::Acquire),
            elapsed = ?t0.elapsed(),
            "waiting for SSA cycles"
        );
        if recovered >= target_total {
            target_recovered = Some(session_requested.elapsed());
            break;
        }
    }

    stop.store(true, Ordering::Release);
    sender.abort();
    receiver.abort();

    let deposits_made = count_in_node_log(&log_dir, ENTRY, "single deposit flushed successfully")?;
    let keys_recovered = count_in_node_log(&log_dir, EXIT, "private key recovered")?;
    let deposits_seen = count_in_node_log(&log_dir, EXIT, "SSA deposit successful")?;
    let deposits_missed = count_in_node_log(&log_dir, EXIT, "deposit confirmation timed out")?;
    #[cfg(feature = "strategy-pix-curvy")]
    let curvy_pending = count_in_node_log(
        &log_dir,
        EXIT,
        "discovered Curvy PIX pending note through Blokli",
    )?;
    #[cfg(feature = "strategy-pix-curvy")]
    let curvy_committed = count_in_node_log(
        &log_dir,
        EXIT,
        "correlated committed Curvy PIX note through Blokli",
    )?;
    let echoed = echoed.load(Ordering::Acquire);
    let sent_datagrams = sent_datagrams.load(Ordering::Acquire);
    let sent_bytes = sent_bytes.load(Ordering::Acquire);
    let received_datagrams = received_datagrams.load(Ordering::Acquire);
    let received_bytes = received_bytes.load(Ordering::Acquire);
    let first_send_millis = first_send_millis.load(Ordering::Acquire);
    let first_byte_millis = first_byte_millis.load(Ordering::Acquire);
    let time_to_first_send =
        (first_send_millis != TTFB_NOT_OBSERVED).then(|| Duration::from_millis(first_send_millis));
    let time_to_first_byte =
        (first_byte_millis != TTFB_NOT_OBSERVED).then(|| Duration::from_millis(first_byte_millis));
    let established_to_first_byte =
        time_to_first_byte.map(|elapsed| elapsed.saturating_sub(session_established));

    #[cfg(feature = "strategy-pix-curvy")]
    {
        let measurements = serde_json::json!({
            "schema_version": 1,
            "units": {
                "timing": "milliseconds",
                "traffic": "bytes",
                "balances": "HoprBalance display units",
            },
            "acceptance": {
                "time_to_first_byte_slo_ms": duration_millis(TIME_TO_FIRST_BYTE_SLO),
                "time_to_first_byte_ms": time_to_first_byte.map(duration_millis),
                "time_to_first_byte_passed": time_to_first_byte
                    .is_some_and(|elapsed| elapsed < TIME_TO_FIRST_BYTE_SLO),
                "target_cycles": TARGET_CYCLES,
                "target_balance_recovered": recovered >= target_total,
            },
            "bootstrap_timing_ms": {
                "cluster_start": duration_millis(cluster_started),
                "nodes_ready": duration_millis(nodes_ready),
                "channels_ready": duration_millis(channels_ready),
                "ready_wait_after_cluster_start": duration_millis(nodes_ready.saturating_sub(cluster_started)),
                "channel_setup_after_nodes_ready": duration_millis(channels_ready.saturating_sub(nodes_ready)),
            },
            "session_timing_ms": {
                "establishment": duration_millis(session_established),
                "first_datagram_sent_from_request": time_to_first_send.map(duration_millis),
                "first_byte_from_request": time_to_first_byte.map(duration_millis),
                "first_byte_after_establishment": established_to_first_byte.map(duration_millis),
                "first_safe_credit_from_request": first_safe_credit.map(duration_millis),
                "target_recovered_from_request": target_recovered.map(duration_millis),
                "settlement_milestones": settlement_milestones.iter().map(|(cycles, elapsed)| {
                    serde_json::json!({
                        "completed_cycles": cycles,
                        "from_session_request": duration_millis(*elapsed),
                    })
                }).collect::<Vec<_>>(),
            },
            // Child proof sidecars are still owned by the running hoprd processes here. This
            // placeholder is replaced after Cluster teardown has copied their finalized files.
            "proof_timing": { "status": "pending_child_log_finalization" },
            "traffic": {
                "sent_datagrams": sent_datagrams,
                "sent_bytes": sent_bytes,
                "received_datagrams": received_datagrams,
                "received_bytes": received_bytes,
                "exact_echoes": echoed,
            },
            "protocol_counts": {
                "entry_deposits_made": deposits_made,
                "exit_deposits_seen": deposits_seen,
                "exit_deposit_timeouts": deposits_missed,
                "exit_keys_recovered": keys_recovered,
                "curvy_pending_notes_discovered": curvy_pending,
                "curvy_committed_notes_correlated": curvy_committed,
            },
            "accounting": {
                "gross_per_cycle": per_cycle.to_string(),
                "net_swept_per_cycle": swept_per_cycle.to_string(),
                "target_total": target_total.to_string(),
                "recovered": recovered.to_string(),
            },
        });
        let measurements_path = log_dir.join("pix-measurements.json");
        std::fs::write(
            &measurements_path,
            serde_json::to_vec_pretty(&measurements).context("encoding PIX measurements")?,
        )
        .with_context(|| {
            format!(
                "writing PIX measurements to {}",
                measurements_path.display()
            )
        })?;
        tracing::info!(
            path = %measurements_path.display(),
            "wrote preliminary PIX timing report"
        );
    }

    tracing::info!(
        ?cluster_started,
        ?nodes_ready,
        ?channels_ready,
        ?session_established,
        ?time_to_first_send,
        ?time_to_first_byte,
        ?established_to_first_byte,
        ?first_safe_credit,
        ?target_recovered,
        sent_datagrams,
        sent_bytes,
        received_datagrams,
        received_bytes,
        exact_echoes = echoed,
        deposits_made,
        deposits_seen,
        keys_recovered,
        %recovered,
        "PIX end-to-end timing and traffic measurements"
    );

    // ── Assertions ──────────────────────────────────────────────────────────────
    let time_to_first_byte = time_to_first_byte
        .unwrap_or_else(|| panic!("no byte completed the Entry -> Exit -> echo -> Entry path"));
    assert!(
        time_to_first_byte < TIME_TO_FIRST_BYTE_SLO,
        "time to first byte was {time_to_first_byte:?}, exceeding the HOPR requirement of \
         {TIME_TO_FIRST_BYTE_SLO:?}; session establishment itself took {session_established:?}"
    );
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
    assert!(
        keys_recovered >= TARGET_CYCLES as usize,
        "the Exit reconstructed only {keys_recovered} SSA private keys, expected at least \
         {TARGET_CYCLES}; the BJJ Shamir-share recovery path did not complete"
    );
    #[cfg(feature = "strategy-pix-curvy")]
    {
        assert!(
            curvy_pending >= TARGET_CYCLES as usize,
            "Blokli yielded only {curvy_pending} owned pending Curvy notes, expected at least {TARGET_CYCLES}"
        );
        assert!(
            curvy_committed >= TARGET_CYCLES as usize,
            "Blokli yielded only {curvy_committed} correlated committed Curvy notes, expected at least {TARGET_CYCLES}"
        );
    }

    // An exact multiple is the real check: it says every wxHOPR that entered the Exit's
    // Safe arrived as a whole SSA sweep. Curvy deducts its configured withdrawal fees;
    // `swept_per_cycle` is therefore net while the Entry's `per_cycle` allocation is gross.
    let cycles = completed_cycles(recovered, swept_per_cycle).unwrap_or_else(|| {
        panic!(
            "Exit Safe gained {recovered}, which is not a whole multiple of the {swept_per_cycle} \
             net per-SSA sweep — something other than PIX sweeps moved the balance \
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
    #[cfg(feature = "strategy-pix-secp256k1")]
    {
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
    }
    #[cfg(feature = "strategy-pix-curvy")]
    {
        // Curvy allocations spend the private note shielded during node startup,
        // not the Entry's public node account after this test's balance snapshot.
        let funding_events =
            count_in_node_log(&log_dir, ENTRY, "Curvy PIX private pool is funded")?;
        assert_eq!(
            funding_events, 1,
            "the Entry should shield its durable Curvy funding exactly once, observed {funding_events} times"
        );
        assert!(
            deposits_made >= TARGET_CYCLES as usize,
            "the Entry completed only {deposits_made} Curvy allocations, expected at least {TARGET_CYCLES}"
        );
    }

    entry
        .api
        .close_client(&ip, port)
        .await
        .context("closing the session listener")?;

    tracing::info!(
        cycles, %recovered, deposits_made, deposits_seen, keys_recovered, echoed,
        "PIX session test PASSED in {:?}", t0.elapsed()
    );
    #[cfg(feature = "strategy-pix-curvy")]
    {
        // Finalize child output before parsing proof timings. Cluster teardown closes every
        // per-node JSONL sink and copies it to the stable diagnostics destination.
        drop(cluster);
        finalize_proof_timing_report(std::path::Path::new(PIX_LOG_DEST))?;
    }
    Ok(())
}
