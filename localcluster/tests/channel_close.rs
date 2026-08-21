//! Integration test: start a 3-node local cluster, open a full mesh of outgoing
//! channels, then initiate closure on every channel and verify the transition
//! from `Open` to `PendingToClose`.
//!
//! This test is `#[ignore]` (long runtime, external chain container + `hoprd`
//! binary) and is not intended for CI — run it explicitly by name.
//!
//! Required (at least one chain source):
//!   HOPRD_CHAIN_URL        – Blokli URL of a running Anvil+Blokli stack
//!   HOPRD_CHAIN_IMAGE      – container image to launch (used when HOPRD_CHAIN_URL is absent)
//!
//! Optional:
//!   HOPRD_BIN              – path to the hoprd binary (default: "hoprd" on PATH)
//!   HOPRD_CONTAINER_RUNTIME – container runtime CLI (default: "docker")
//!
//! # Prerequisites
//!
//! The `hoprd` binary must be built in **release** mode.  Debug builds incur
//! significant overhead that can push the test past the default timeout:
//!
//! ```bash
//! nix develop -c cargo build --release -p hoprd
//! export HOPRD_BIN=$(pwd)/target/release/hoprd
//! ```

mod common;

use std::time::Duration;

use common::{Cluster, ClusterSpec, ports};
use hoprd_localcluster::client_helper;

/// Amount of wxHOPR to fund each channel with.
const CHANNEL_AMOUNT: &str = "10 wxHOPR";
/// General timeout for chain operations.
const TIMEOUT: Duration = Duration::from_secs(180);
/// How long to wait for a close_channel REST call to succeed.  On the
/// local Blokli the chain subscription drops frequently, causing the
/// event-waiter inside `close_channel_by_id` to time out even though the
/// on-chain transaction went through.  We give the call a generous window
/// and fall back to polling for the status change.
const CLOSE_TX_TIMEOUT: Duration = Duration::from_secs(30);

#[tokio::test]
#[ignore = "requires external chain container and hoprd binary — run explicitly, not in CI"]
async fn localcluster_channel_initiate_closure() {
    run().await.expect("channel initiate closure test failed");
}

/// Poll every node's outgoing channel and return once *all* of them have a
/// status from `acceptable`.
async fn wait_full_mesh_status(
    nodes: &[client_helper::NodeProcess],
    acceptable: &[&str],
    timeout: Duration,
) -> anyhow::Result<()> {
    let start = std::time::Instant::now();
    loop {
        let mut mismatched = Vec::new();
        for src in nodes {
            for dst in nodes {
                if let (Some(src_addr), Some(dst_addr)) = (&src.address, &dst.address)
                    && src_addr != dst_addr
                {
                    // A failed read is a transient subscription drop, not a verdict. This is the
                    // same flakiness the module doc above predicts and that `close_and_poll`
                    // already tolerates on the write side; propagating it here would end the
                    // whole test on one dropped GET, two seconds before the retry that would
                    // have succeeded. Record it as a mismatch and let the timeout bound it.
                    let status = match src.api.outgoing_channel_status(dst_addr).await {
                        Ok(status) => status.unwrap_or_else(|| "<none>".to_string()),
                        Err(error) => format!("<unreadable: {error}>"),
                    };
                    if !acceptable.iter().any(|a| status == *a) {
                        mismatched.push((src.id, dst.id, status));
                    }
                }
            }
        }

        if mismatched.is_empty() {
            return Ok(());
        }

        if start.elapsed() > timeout {
            let pairs: Vec<_> = mismatched
                .iter()
                .map(|(s, d, st)| format!("{s}→{d} is {st}"))
                .collect();
            anyhow::bail!(
                "timeout waiting for full-mesh channels to be in {:?}: {}",
                acceptable,
                pairs.join(", ")
            );
        }

        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}

/// Fire a close_channel request for every outgoing pair and then poll until
/// every channel reaches an `acceptable` status.
///
/// The REST handler internally submits the on-chain transaction; even if the
/// HTTP response is an error (chain subscription flakiness), the tx may have
/// gone through.  We therefore issue the requests optimistically and rely on
/// polling to confirm the state transition.
async fn close_and_poll(
    nodes: &[client_helper::NodeProcess],
    acceptable: &[&str],
    timeout: Duration,
) -> anyhow::Result<()> {
    // First pass: fire close_channel on every pair in parallel.
    let mut futures = Vec::new();
    for src in nodes {
        for dst in nodes {
            if let (Some(src_addr), Some(dst_addr)) = (&src.address, &dst.address)
                && src_addr != dst_addr
            {
                let api = src.api.clone();
                let addr = dst_addr.clone();
                futures.push(async move {
                    // We give each call its own deadline but don't bail on
                    // failure — the on-chain action may already be done.
                    let _ = tokio::time::timeout(CLOSE_TX_TIMEOUT, api.close_channel(&addr)).await;
                });
            }
        }
    }
    futures::future::join_all(futures).await;

    // Now poll until every pair has an acceptable status.
    wait_full_mesh_status(nodes, acceptable, timeout).await
}

async fn run() -> anyhow::Result<()> {
    common::init_tracing();

    // Bring-up defaults suffice; this suite opens its channels itself rather than leaving them
    // to a strategy.
    let cluster = Cluster::start(ClusterSpec::new(ports::CHANNEL_CLOSE)).await?;

    // ── Phase 1: Open full mesh ────────────────────────────────────────

    // Neither `wait_ready` nor `wait_reachable` is called, deliberately. Opening a channel is
    // an on-chain action that needs neither /readyz nor a converged peer graph, and
    // `open_channels` re-checks and retries every 5 s until TIMEOUT, so a node still catching
    // up is absorbed rather than raced. `wait_full_mesh_status` below is the real precondition
    // check for phase 2, and it is status-generic where the shared waiter is not.
    tracing::info!("opening full-mesh channels…");
    cluster.open_channels(CHANNEL_AMOUNT, TIMEOUT).await?;
    wait_full_mesh_status(cluster.nodes(), &["Open"], TIMEOUT).await?;
    tracing::info!("all channels are Open");

    // ── Phase 2: Initiate closure ──────────────────────────────────────

    // `Closed` is accepted alongside `PendingToClose`: what this phase asserts is that closure
    // was initiated and observed, and a channel that got all the way to `Closed` between two
    // polls satisfies that just as well. Excluding it would turn a faster-than-expected chain
    // into a timeout.
    tracing::info!("initiating channel closure…");
    close_and_poll(cluster.nodes(), &["PendingToClose", "Closed"], TIMEOUT).await?;
    tracing::info!("all channels transitioned to PendingToClose or Closed");

    Ok(())
}
