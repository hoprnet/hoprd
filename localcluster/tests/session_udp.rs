//! UDP session ping-pong tests.
//!
//! Required (at least one chain source):
//!   HOPRD_CHAIN_URL   or HOPRD_CHAIN_IMAGE
//! Optional: HOPRD_BIN, HOPRD_CONTAINER_RUNTIME

mod common;

use std::time::Duration;

use common::{ClusterCleanup, ClusterEnv, TempCluster};
use hoprd_localcluster::client_helper;
use hoprd_localcluster::identity;
use tokio::net::UdpSocket;

/// Effective data per chunk: SESSION_MTU minus 4-byte sequence tag.
const CHUNK_SIZE: usize = 900;
const TAG_SIZE: usize = 4;
const DATA_PER_CHUNK: usize = CHUNK_SIZE - TAG_SIZE; // 896

const P2P_HOST: &str = "127.0.0.1";
const P2P_PORT_BASE: u16 = 19300;
const TIMEOUT: Duration = Duration::from_secs(300);
const WAIT_TIMEOUT: Duration = Duration::from_secs(120);

#[tokio::test]
#[ignore]
async fn localcluster_udp_session_pingpong_32b() {
    go(32, Duration::from_secs(120)).await
}
#[tokio::test]
#[ignore]
async fn localcluster_udp_session_pingpong_200b() {
    go(200, Duration::from_secs(120)).await
}

#[tokio::test]
#[ignore]
async fn localcluster_udp_session_pingpong_64kb() {
    go(65536, Duration::from_secs(180)).await
}

/// 1 MiB session test — should complete within ~5 min of session time.
/// Uses the keep-alive send pump to maintain forward-path SURB flow.
#[tokio::test]
#[ignore]
async fn localcluster_udp_session_pingpong_1mb() {
    go(1048576, Duration::from_secs(300)).await
}

async fn start_echo_server() -> anyhow::Result<u16> {
    let sock = UdpSocket::bind("127.0.0.1:0").await?;
    let port = sock.local_addr()?.port();
    tokio::spawn(async move {
        let mut buf = vec![0u8; 65535];
        loop {
            if let Ok((n, src)) = sock.recv_from(&mut buf).await {
                let _ = sock.send_to(&buf[..n], src).await;
            }
        }
    });
    Ok(port)
}

fn gen_payload(size: usize) -> Vec<u8> {
    let mut d = vec![0u8; size];
    for (i, b) in d.iter_mut().enumerate() {
        *b = (i % 256) as u8;
    }
    d
}

fn tag_chunk(idx: u32, data: &[u8]) -> Vec<u8> {
    let mut v = Vec::with_capacity(TAG_SIZE + data.len());
    v.extend_from_slice(&idx.to_be_bytes());
    v.extend_from_slice(data);
    v
}

async fn setup_cluster(env: &ClusterEnv, cluster: &TempCluster, cleanup: &mut ClusterCleanup) {
    let t0 = std::time::Instant::now();
    let blk = common::start_chain(env, &cluster.log_dir, cleanup)
        .await
        .unwrap();
    common::wait_for_blokli_ready(&blk, WAIT_TIMEOUT)
        .await
        .unwrap();
    tracing::info!("chain ready after {:?}", t0.elapsed());

    identity::generate(&identity::GenerationConfig {
        blokli_url: blk,
        num_nodes: 3,
        config_home: cluster.data_dir.clone(),
        random_identities: true,
        p2p_host: P2P_HOST.to_string(),
        p2p_port_base: P2P_PORT_BASE,
        enable_channel_strategy: false,
        enable_pix: false,
        disable_strategies: false,
        strategy_execution_interval: Some(Duration::from_secs(600)),
        ..Default::default()
    })
    .await
    .unwrap();
    tracing::info!("identities generated after {:?}", t0.elapsed());

    cleanup.nodes = client_helper::start_nodes(&client_helper::NodeStartConfig {
        num_nodes: 3,
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
    })
    .await
    .unwrap();
    tracing::info!("nodes started after {:?}", t0.elapsed());

    futures::future::try_join_all(
        cleanup
            .nodes
            .iter()
            .map(|n| n.api.wait_started(WAIT_TIMEOUT)),
    )
    .await
    .unwrap();
    for n in &mut cleanup.nodes {
        n.address = Some(n.api.addresses().await.unwrap());
    }
    futures::future::try_join_all(cleanup.nodes.iter().map(|n| n.api.wait_ready(WAIT_TIMEOUT)))
        .await
        .unwrap();
    tracing::info!("nodes ready after {:?}", t0.elapsed());

    client_helper::open_full_mesh_channels(&cleanup.nodes, "10 wxHOPR", TIMEOUT)
        .await
        .unwrap();
    client_helper::wait_full_mesh_channels(&cleanup.nodes, TIMEOUT)
        .await
        .unwrap();
    client_helper::wait_full_mesh_reachable(&cleanup.nodes, TIMEOUT)
        .await
        .unwrap();
    tracing::info!("channels ready after {:?}", t0.elapsed());
}

/// Batch-retransmit send/recv with tagged chunks.
///
/// Phase 1: initial burst sends everything. Phase 2: drain responses; when
/// the flow stalls, retransmit all missing chunks to pump forward-path SURBs.
/// Tags resolve out-of-order delivery.
async fn go(payload_size: usize, chunk_timeout: Duration) {
    common::init_tracing();
    let env = ClusterEnv::from_env().unwrap();
    let cluster = TempCluster::new().unwrap();
    let mut cleanup = ClusterCleanup {
        chain: None,
        nodes: vec![],
    };
    let t0 = std::time::Instant::now();

    setup_cluster(&env, &cluster, &mut cleanup).await;

    let log_dir = cluster.log_dir.clone();
    let _log_guard = {
        let ld = log_dir.clone();
        scopeguard::guard(ld, |logs| {
            let dest = std::path::Path::new("/tmp/udp-session-logs");
            let _ = std::fs::create_dir_all(dest);
            if let Ok(entries) = std::fs::read_dir(&logs) {
                for e in entries.flatten() {
                    let dst = dest.join(e.file_name());
                    let _ = std::fs::copy(&e.path(), &dst);
                }
            }
        })
    };

    let echo_port = start_echo_server().await.unwrap();
    let entry = &cleanup.nodes[0];
    let exit = cleanup.nodes[2].address.as_ref().unwrap();
    let target = format!("127.0.0.1:{echo_port}");

    tracing::info!("entry=node{} exit={exit} target={target}", entry.id);

    let (ip, port) = entry
        .api
        .open_session("udp", exit, &target, 1, None, None, None)
        .await
        .unwrap();
    tracing::info!("session on {ip}:{port} (elapsed={:?})", t0.elapsed());

    let sock = UdpSocket::bind("127.0.0.1:0").await.unwrap();
    sock.connect(format!("{ip}:{port}")).await.unwrap();

    let payload = gen_payload(payload_size);
    let nchunks = payload.len().div_ceil(DATA_PER_CHUNK);
    tracing::info!(
        "{payload_size} B in {nchunks} chunks of {DATA_PER_CHUNK}B + {TAG_SIZE}B tag (elapsed={:?})",
        t0.elapsed()
    );

    let deadline = std::time::Instant::now() + chunk_timeout;

    // Pre-compute tagged chunks
    let chunks: Vec<Vec<u8>> = payload
        .chunks(DATA_PER_CHUNK)
        .enumerate()
        .map(|(i, c)| tag_chunk(i as u32, c))
        .collect();

    let mut done: Vec<bool> = vec![false; nchunks];
    let mut n_done = 0usize;

    // Phase 1: initial burst — send everything once
    for tagged in &chunks {
        assert!(
            std::time::Instant::now() < deadline,
            "timeout during init burst"
        );
        sock.send(tagged).await.unwrap();
        tokio::time::sleep(Duration::from_millis(3)).await;
    }
    tracing::info!("burst {nchunks} chunks");

    // Phase 2: drain with retransmit. On recv timeout, pump all missing chunks
    // to re-energize the forward SURB path.
    let mut buf = vec![0u8; CHUNK_SIZE];
    let mut recv_to = Duration::from_millis(500);

    loop {
        if n_done >= nchunks {
            break;
        }
        assert!(
            std::time::Instant::now() < deadline,
            "timeout (recvd={n_done}/{nchunks})"
        );

        let r = tokio::time::timeout(recv_to, sock.recv(&mut buf)).await;
        match r {
            Ok(Ok(n)) if n >= TAG_SIZE => {
                let tag = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as usize;
                if tag < nchunks && !done[tag] {
                    let off = tag * DATA_PER_CHUNK;
                    let chunk_end = payload.len().min(off + DATA_PER_CHUNK);
                    let data = &buf[TAG_SIZE..n];
                    assert_eq!(data.len(), chunk_end - off, "size mismatch tag {tag}");
                    assert_eq!(data, &payload[off..chunk_end], "data mismatch tag {tag}");
                    done[tag] = true;
                    n_done += 1;
                    if n_done <= 3 || n_done % 50 == 0 || n_done >= nchunks {
                        tracing::info!("resp tag={tag} recvd={n_done}/{nchunks}");
                    }
                }
                recv_to = Duration::from_millis(500);
            }
            Ok(Ok(_)) => {}
            Ok(Err(e)) => panic!("recv error: {e}"),
            Err(_) => {
                // Pump all missing chunks
                recv_to = (recv_to * 2).min(Duration::from_secs(10));
                for (i, tagged) in chunks.iter().enumerate() {
                    if !done[i] {
                        sock.send(tagged).await.unwrap();
                        tokio::time::sleep(Duration::from_millis(1)).await;
                    }
                }
                let missing = nchunks - n_done;
                tracing::info!("retx {missing} (recv_to={recv_to:?})");
            }
        }
    }

    assert_eq!(
        n_done, nchunks,
        "expected {nchunks} responses, got {n_done}"
    );
    tracing::info!("all {payload_size}B sent+recv'd after {:?}", t0.elapsed());
    entry.api.close_client(&ip, port).await.unwrap();
    tracing::info!("DONE {payload_size}B in {:?}", t0.elapsed());
}
