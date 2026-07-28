//! Integration test: start a 3-node local cluster and verify that every node's
//! `/account/addresses` REST API response matches the identity generated for it.
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

use common::{ClusterCleanup, ClusterEnv, TempCluster};
use hoprd_localcluster::{client_helper, identity};

#[tokio::test]
#[ignore]
async fn localcluster_addresses_match_generated_identities() {
    run().await.expect("address-identity verification failed");
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

    const P2P_HOST: &str = "127.0.0.1";
    const P2P_PORT_BASE: u16 = 19100; // different base to avoid port conflicts

    let num_nodes = 3;
    let gen_cfg = identity::GenerationConfig {
        blokli_url: blokli_url.clone(),
        num_nodes,
        config_home: cluster.data_dir.clone(),
        random_identities: true,
        p2p_host: P2P_HOST.to_string(),
        p2p_port_base: P2P_PORT_BASE,
        enable_channel_strategy: false,
        enable_pix: false,
        ..Default::default()
    };
    let gen_output = identity::generate(&gen_cfg).await?;

    // Spawn hoprd processes.
    let start_cfg = client_helper::NodeStartConfig {
        num_nodes,
        hoprd_bin: &env.hoprd_bin,
        data_dir: &cluster.data_dir,
        log_dir: &cluster.log_dir,
        api_host: "127.0.0.1",
        api_port_base: 13100,
        p2p_host: P2P_HOST,
        p2p_port_base: P2P_PORT_BASE,
        identity_password: identity::DEFAULT_IDENTITY_PASSWORD,
        api_token: None,
        enable_pix: false,
    };
    cleanup.nodes = client_helper::start_nodes(&start_cfg).await?;

    // Wait for all nodes to be started (API up, HoprState::Running).
    futures::future::try_join_all(
        cleanup
            .nodes
            .iter()
            .map(|n| n.api.wait_started(env.wait_timeout)),
    )
    .await?;

    // Verify every node's API-reported address matches its generated identity.
    // The generated identity stores the address as lowercase, while the API
    // returns it in EIP-55 checksummed format — compare case-insensitively.
    for node in &cleanup.nodes {
        let actual = node.api.addresses().await?;
        let expected = &gen_output.nodes[node.id].address;
        anyhow::ensure!(
            actual.eq_ignore_ascii_case(expected),
            "node {} address mismatch: got {actual}, expected {expected}",
            node.id,
        );
        tracing::info!("node {id} address verified: {actual}", id = node.id);
    }

    tracing::info!("address-identity verification test passed");
    Ok(())
}
