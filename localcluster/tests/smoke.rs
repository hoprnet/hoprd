//! Smoke test: start a 3-node local cluster and verify the ChannelLifecycleStrategy
//! opens a full-mesh topology without any explicit REST open_channel calls.
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
//! cargo build --release -p hoprd
//! export HOPRD_BIN=$(pwd)/target/release/hoprd
//! ```

mod common;

use std::time::Duration;

use anyhow::Result;
use common::{ClusterCleanup, ClusterEnv, TempCluster};
use hoprd_localcluster::{client_helper, identity};

const WAIT_TIMEOUT: Duration = Duration::from_secs(120);

#[tokio::test]
#[ignore = "requires external chain container and hoprd binary — run explicitly, not in CI"]
async fn localcluster_channels_opened_by_strategy() {
    run().await.expect("localcluster smoke test failed");
}

async fn run() -> Result<()> {
    common::init_tracing();

    let env = ClusterEnv::from_env()?;
    let cluster = TempCluster::new()?;

    let mut cleanup = ClusterCleanup {
        chain: None,
        nodes: vec![],
    };

    let blokli_url = common::start_chain(&env, &cluster.log_dir, &mut cleanup).await?;

    // Wait for chain to be ready.
    common::wait_for_blokli_ready(&blokli_url, WAIT_TIMEOUT).await?;

    // Generate identities and per-node configs.  The ChannelLifecycleStrategy
    // population thresholds are set to num_nodes-1 inside `generate` so the
    // strategy will open a full mesh.
    const P2P_HOST: &str = "127.0.0.1";
    const P2P_PORT_BASE: u16 = 19000;

    let num_nodes = 3;
    let gen_cfg = identity::GenerationConfig {
        blokli_url: blokli_url.clone(),
        num_nodes,
        config_home: cluster.data_dir.clone(),
        // Frozen identities on purpose: they are what a real cluster run uses, so this test
        // also covers the frozen key derivation. See `scripts/localcluster-smoke.sh` for the
        // cheaper 2-node variant that CI runs on every PR.
        random_identities: false,
        p2p_host: P2P_HOST.to_string(),
        p2p_port_base: P2P_PORT_BASE,
        strategies: identity::StrategySet {
            auto_redeeming: true,
            channel_lifecycle: true,
        },
        // PIX off. It used to be on here, but only as `HOPRD_ENABLE_PIX=1`, which a binary
        // without a deposit pool logged and ignored — so this test stayed runnable against a
        // plain `cargo build -p hoprd`. A `Pix` stanza is not ignorable: it would now refuse
        // to start that same binary. Nothing here opens a PIX Session, so there is nothing to
        // keep; `session_pix` covers the strategy properly.
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
        api_port_base: 13000,
        p2p_host: P2P_HOST,
        p2p_port_base: P2P_PORT_BASE,
        identity_password: identity::DEFAULT_IDENTITY_PASSWORD,
        api_token: None,
    };
    cleanup.nodes = client_helper::start_nodes(&start_cfg).await?;

    // Wait for all nodes to be started (API up, HoprState::Running).
    // We intentionally skip the `wait_ready` (readyz) check here: on Apple
    // Container the blokli SSE subscription drops every ~10 s, cycling the
    // chain health through Degraded→Connecting states. During reconnection
    // HoprState briefly leaves Running, so /readyz oscillates between 200 and
    // 412 — making the check flaky. Peer connectivity is verified by
    // wait_full_mesh_reachable below, which is a stronger guarantee anyway.
    futures::future::try_join_all(
        cleanup
            .nodes
            .iter()
            .map(|n| n.api.wait_started(2 * WAIT_TIMEOUT)),
    )
    .await?;

    // Fetch on-chain addresses so we can identify peers.
    for node in &mut cleanup.nodes {
        node.address = Some(node.api.addresses().await?);
    }

    // Wait for every node to be reachable by every other (strategy precondition:
    // require_currently_connected = true).
    client_helper::wait_full_mesh_reachable(&cleanup.nodes, WAIT_TIMEOUT).await?;

    // Key assertion: the ChannelLifecycleStrategy must open the full mesh of
    // outgoing channels without any explicit REST open_channel calls.
    client_helper::wait_full_mesh_channels(&cleanup.nodes, WAIT_TIMEOUT * 4).await?;

    // Double-check every expected pair via direct API call.
    for src in &cleanup.nodes {
        for dst in &cleanup.nodes {
            if let (Some(src_addr), Some(dst_addr)) = (&src.address, &dst.address)
                && src_addr != dst_addr
            {
                assert!(
                    src.api.is_outgoing_channel_open(dst_addr).await?,
                    "node {} missing open outgoing channel to node {}",
                    src.id,
                    dst.id,
                );
            }
        }
    }

    tracing::info!("smoke test passed: full mesh established by strategy");
    Ok(())
}
