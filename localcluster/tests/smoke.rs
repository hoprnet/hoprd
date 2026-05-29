//! Smoke test: start a 3-node local cluster and verify the ChannelLifecycleStrategy
//! opens a full-mesh topology without any explicit REST open_channel calls.
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

use std::{path::PathBuf, time::Duration};

use anyhow::Result;
use hoprd_localcluster::{blokli_helper, client_helper, identity};

const WAIT_TIMEOUT: Duration = Duration::from_secs(120);

#[tokio::test]
#[ignore]
async fn localcluster_channels_opened_by_strategy() {
    run().await.expect("localcluster smoke test failed");
}

async fn run() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()))
        .with_target(false)
        .try_init()
        .ok();

    let hoprd_bin =
        PathBuf::from(std::env::var("HOPRD_BIN").unwrap_or_else(|_| "hoprd".to_string()));
    let chain_url_env = std::env::var("HOPRD_CHAIN_URL").ok();
    let chain_image = std::env::var("HOPRD_CHAIN_IMAGE").ok();
    let container_runtime =
        std::env::var("HOPRD_CONTAINER_RUNTIME").unwrap_or_else(|_| "docker".to_string());

    anyhow::ensure!(
        chain_url_env.is_some() || chain_image.is_some(),
        "set HOPRD_CHAIN_URL (existing chain) or HOPRD_CHAIN_IMAGE (to start a container)"
    );

    let temp_dir = tempfile::tempdir()?;
    let data_dir = temp_dir.path().to_path_buf();
    let log_dir = data_dir.join("logs");
    std::fs::create_dir_all(&log_dir)?;

    struct Cleanup {
        chain: Option<blokli_helper::ChainHandle>,
        nodes: Vec<client_helper::NodeProcess>,
    }
    impl Drop for Cleanup {
        fn drop(&mut self) {
            for n in &mut self.nodes {
                let _ = n.child.kill();
            }
            if let Some(c) = &mut self.chain {
                c.stop();
            }
        }
    }

    let mut cleanup = Cleanup {
        chain: None,
        nodes: vec![],
    };

    let blokli_url = if let Some(url) = chain_url_env {
        url.trim_end_matches('/').to_string()
    } else {
        let img = chain_image.as_deref().unwrap();
        let handle = blokli_helper::ChainHandle::start(&container_runtime, img, &log_dir)?;
        let url = handle.chain_url();
        cleanup.chain = Some(handle);
        url
    };

    // Wait for chain to be ready.
    wait_for_blokli_ready(&blokli_url, WAIT_TIMEOUT).await?;

    // Generate identities and per-node configs.  The ChannelLifecycleStrategy
    // population thresholds are set to num_nodes-1 inside `generate` so the
    // strategy will open a full mesh.
    const P2P_HOST: &str = "127.0.0.1";
    const P2P_PORT_BASE: u16 = 19000;

    let num_nodes = 3;
    let gen_cfg = identity::GenerationConfig {
        blokli_url: blokli_url.clone(),
        num_nodes,
        config_home: data_dir.clone(),
        random_identities: true,
        p2p_host: P2P_HOST.to_string(),
        p2p_port_base: P2P_PORT_BASE,
        ..Default::default()
    };
    identity::generate(&gen_cfg).await?;

    // Spawn hoprd processes.
    let start_cfg = client_helper::NodeStartConfig {
        num_nodes,
        hoprd_bin: &hoprd_bin,
        data_dir: &data_dir,
        log_dir: &log_dir,
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

async fn wait_for_blokli_ready(url: &str, timeout: Duration) -> Result<()> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;
    let readyz = format!("{url}/readyz");
    let start = std::time::Instant::now();
    loop {
        if let Ok(resp) = client.get(&readyz).send().await
            && resp.status().is_success()
        {
            return Ok(());
        }
        if start.elapsed() > timeout {
            anyhow::bail!("timeout waiting for blokli at {readyz}");
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
