//! Integration test: start a 3-node local cluster, open a full-mesh of funded
//! channels, create a UDP session (entry → relay → exit), send UDP packets back
//! and forth through an echo server, verify packet integrity, and close the session.
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
use tokio::net::UdpSocket;

/// Amount of wxHOPR to fund each channel with.
///
/// Each node receives 1000 wxHOPR and needs to open 2 outgoing channels,
/// so 400 wxHOPR per channel leaves room for both plus ticket redemption buffer.
const CHANNEL_AMOUNT: &str = "400 wxHOPR";

const P2P_HOST: &str = "127.0.0.1";
const P2P_PORT_BASE: u16 = 19300;

/// Client-facing constants.
const TIMEOUT: Duration = Duration::from_secs(300);
const WAIT_TIMEOUT: Duration = Duration::from_secs(120);

#[tokio::test]
#[ignore]
async fn localcluster_udp_session_pingpong() {
    run().await.expect("UDP session ping-pong test failed");
}

/// Start a UDP echo server and return its port.
///
/// The spawned task reads datagrams and writes them back to the sender.
async fn start_echo_server() -> anyhow::Result<u16> {
    let sock = UdpSocket::bind("127.0.0.1:0").await?;
    let port = sock.local_addr()?.port();
    tokio::spawn(async move {
        let mut buf = vec![0u8; 2048];
        loop {
            match sock.recv_from(&mut buf).await {
                Ok((n, src)) => {
                    if let Err(e) = sock.send_to(&buf[..n], src).await {
                        tracing::warn!("echo server send_to error: {e}");
                        break;
                    }
                }
                Err(e) => {
                    tracing::warn!("echo server recv_from error: {e}");
                    break;
                }
            }
        }
    });
    Ok(port)
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
    common::wait_for_blokli_ready(&blokli_url, WAIT_TIMEOUT).await?;

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
    identity::generate(&gen_cfg).await?;

    // Spawn hoprd processes.
    let start_cfg = client_helper::NodeStartConfig {
        num_nodes,
        hoprd_bin: &env.hoprd_bin,
        data_dir: &cluster.data_dir,
        log_dir: &cluster.log_dir,
        api_host: "127.0.0.1",
        api_port_base: 13300,
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
            .map(|n| n.api.wait_started(WAIT_TIMEOUT)),
    )
    .await?;

    // Fetch on-chain addresses so we can identify peers.
    for node in &mut cleanup.nodes {
        node.address = Some(node.api.addresses().await?);
    }

    // ── Phase 1: Open full mesh with generous funding ──────────────────

    // Wait for chain health before opening channels.  At startup the blokli
    // SSE subscription drops frequently, causing hoprd to enter a Degraded
    // state.  Channel-opening transactions require a healthy connection.
    tracing::info!("waiting for chain health…");
    futures::future::try_join_all(
        cleanup
            .nodes
            .iter()
            .map(|n| n.api.wait_ready(WAIT_TIMEOUT)),
    )
    .await?;
    tracing::info!("all nodes ready");

    tracing::info!("opening full-mesh channels ({} each)…", CHANNEL_AMOUNT);
    client_helper::open_full_mesh_channels(&cleanup.nodes, CHANNEL_AMOUNT, TIMEOUT).await?;
    client_helper::wait_full_mesh_channels(&cleanup.nodes, TIMEOUT).await?;
    tracing::info!("all channels are Open");

    // Wait for peer connectivity (strategy precondition not active here, but
    // the session routing needs nodes to be mutually reachable).
    client_helper::wait_full_mesh_reachable(&cleanup.nodes, TIMEOUT).await?;

    // ── Phase 2: Start UDP echo server & open session ──────────────────

    let echo_port = start_echo_server().await?;
    tracing::info!("UDP echo server listening on port {echo_port}");

    // Use node[0] as the entry node, node[2] as the exit node.
    // node[1] will be the relay (1 hop forward, 1 hop return).
    let entry = &cleanup.nodes[0];
    let exit_addr = cleanup.nodes[2]
        .address
        .as_ref()
        .expect("exit node address not resolved");

    let target = format!("127.0.0.1:{echo_port}");

    tracing::info!(
        "creating UDP session: entry=node{} exit={exit_addr} target={target}",
        entry.id,
    );
    let (session_ip, session_port) = entry
        .api
        .open_session("udp", exit_addr, &target, 1)
        .await?;
    tracing::info!("session listening on {session_ip}:{session_port}");

    // ── Phase 3: Send / receive UDP packets ────────────────────────────

    let test_sock = UdpSocket::bind("127.0.0.1:0").await?;
    test_sock
        .connect(format!("{session_ip}:{session_port}"))
        .await?;

    let payloads: Vec<&[u8]> = vec![b"ping-0", b"ping-1", b"ping-2"];
    let mut responses = Vec::new();

    for (i, payload) in payloads.iter().enumerate() {
        tracing::info!("sending: {}", std::str::from_utf8(payload).unwrap());
        test_sock.send(payload).await?;

        // Read the echoed response.
        let mut buf = vec![0u8; 2048];
        let n = tokio::time::timeout(Duration::from_secs(30), test_sock.recv(&mut buf)).await??;
        let received = buf[..n].to_vec();
        tracing::info!(
            "received #{i}: {}",
            std::str::from_utf8(&received).unwrap_or("<bin>")
        );
        anyhow::ensure!(
            received == *payload,
            "packet #{i} mismatch: sent {payload:?}, received {received:?}",
        );
        responses.push(received);
    }

    anyhow::ensure!(
        responses.len() == payloads.len(),
        "expected {} responses, got {}",
        payloads.len(),
        responses.len(),
    );
    tracing::info!("all {} packets echoed correctly", responses.len());

    // ── Phase 4: Close the session ─────────────────────────────────────

    tracing::info!("closing UDP session at {session_ip}:{session_port}");
    entry.api.close_client(&session_ip, session_port).await?;

    // Verify the session socket is unreachable.
    let result = tokio::time::timeout(
        Duration::from_secs(5),
        test_sock.send(b"should-not-arrive"),
    )
    .await;
    // Sending to a closed socket may succeed at the UDP level (no connection
    // state), but there's no listener to process it — just log it.
    tracing::info!("session closed (send after close: {result:?})");

    tracing::info!("UDP session ping-pong test passed");
    Ok(())
}
