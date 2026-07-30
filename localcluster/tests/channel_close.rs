//! Integration test: start a 3-node local cluster, open a full mesh of outgoing
//! channels, then initiate closure on every channel and verify the transition
//! from `Open` to `PendingToClose`.
//!
//! This test is `#[ignore]` because it requires external binaries and services.
//!
//! Required (at least one chain source):
//!   HOPRD_CHAIN_URL        – Blokli URL of a running Anvil+Blokli stack
//!   HOPRD_CHAIN_IMAGE      – container image to launch (used when HOPRD_CHAIN_URL is absent)
//!
//! Optional:
//!   HOPRD_BIN              – path to the hoprd binary (default: "hoprd" on PATH)
//!   HOPRD_CONTAINER_RUNTIME – container runtime CLI (default: "docker")

mod common;

use std::time::Duration;

use common::{ClusterCleanup, ClusterEnv, TempCluster};
use hoprd_localcluster::{client_helper, identity};

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

const P2P_HOST: &str = "127.0.0.1";
const P2P_PORT_BASE: u16 = 19200;

#[tokio::test]
#[ignore]
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
                    let status = src
                        .api
                        .outgoing_channel_status(dst_addr)
                        .await?
                        .unwrap_or_else(|| "<none>".to_string());
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

    let env = ClusterEnv::from_env()?;
    let cluster = TempCluster::new()?;

    let mut cleanup = ClusterCleanup {
        chain: None,
        nodes: vec![],
    };

    let blokli_url = common::start_chain(&env, &cluster.log_dir, &mut cleanup).await?;

    // Wait for chain to be ready.
    common::wait_for_blokli_ready(&blokli_url, env.wait_timeout).await?;

    let num_nodes = 3;
    let gen_cfg = identity::GenerationConfig {
        blokli_url: blokli_url.clone(),
        num_nodes,
        config_home: cluster.data_dir.clone(),
        random_identities: true,
        p2p_host: P2P_HOST.to_string(),
        p2p_port_base: P2P_PORT_BASE,
        strategies: identity::StrategySet {
            auto_redeeming: true,
            channel_lifecycle: false,
            pix: false,
        },
        ..Default::default()
    };
    identity::generate(&gen_cfg).await?;

    // Spawn hoprd processes.
    let start_cfg = client_helper::NodeStartConfig {
        num_nodes,
        hoprd_bin: &env.hoprd_bin,
        data_dir: &cluster.data_dir,
        log_dir: &cluster.log_dir,
        api_host: "127.0.0.1",
        api_port_base: 13200,
        p2p_host: P2P_HOST,
        p2p_port_base: P2P_PORT_BASE,
        identity_password: identity::DEFAULT_IDENTITY_PASSWORD,
        api_token: None,
        pix: false,
    };
    cleanup.nodes = client_helper::start_nodes(&start_cfg).await?;

    // Wait for all nodes to be started.
    futures::future::try_join_all(
        cleanup
            .nodes
            .iter()
            .map(|n| n.api.wait_started(env.wait_timeout)),
    )
    .await?;

    // Fetch on-chain addresses so we can identify peers.
    for node in &mut cleanup.nodes {
        node.address = Some(node.api.addresses().await?);
    }

    // ── Phase 1: Open full mesh ────────────────────────────────────────

    tracing::info!("opening full-mesh channels…");
    client_helper::open_full_mesh_channels(&cleanup.nodes, CHANNEL_AMOUNT, TIMEOUT).await?;
    wait_full_mesh_status(&cleanup.nodes, &["Open"], TIMEOUT).await?;
    tracing::info!("all channels are Open");

    // ── Phase 2: Initiate closure ──────────────────────────────────────

    tracing::info!("initiating channel closure…");
    close_and_poll(&cleanup.nodes, &["PendingToClose"], TIMEOUT).await?;
    tracing::info!("all channels transitioned to PendingToClose");

    Ok(())
}
