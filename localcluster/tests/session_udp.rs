//! UDP session ping-pong tests (not for CI — run explicitly).
//!
//! Opens a 3-node localcluster, creates a UDP session via the entry node
//! through a relay to an exit node, then sends a payload through the HOPR
//! network and verifies the echo response.
//!
//! Uses the `NoDelay` capability to disable session-layer buffering (preserves
//! UDP datagram boundaries) and `Segmentation` for packets exceeding the
//! SESSION_MTU.  A boosted SURB balancer (10 MB buffer, 50 Mb/s upstream)
//! sustains ~4,300 pkts/sec through the loopback localcluster.
//!
//! Required (at least one chain source):
//!   HOPRD_CHAIN_URL   or HOPRD_CHAIN_IMAGE
//! Optional: HOPRD_BIN, HOPRD_CONTAINER_RUNTIME
//!
//! # Prerequisites
//!
//! The `hoprd` binary must be built in **release** mode.  Debug builds add
//! significant overhead to packet processing, cryptography, and control loops,
//! pushing the 1 MB test past the 5-minute target:
//!
//! ```bash
//! nix develop -c cargo build --release -p hoprd
//! export HOPRD_BIN=$(pwd)/target/release/hoprd
//! ```
//!
//! Each test must be run individually — see [`common`] for details.
//!
//! ```bash
//! export HOPRD_BIN=$(pwd)/target/release/hoprd
//! nix develop -c cargo nextest run -p hoprd-localcluster --test session_udp \
//!   -E 'test(localcluster_udp_session_pingpong_32b)' --run-ignored ignored-only -j 1
//! nix develop -c cargo nextest run -p hoprd-localcluster --test session_udp \
//!   -E 'test(localcluster_udp_session_pingpong_200b)' --run-ignored ignored-only -j 1
//! nix develop -c cargo nextest run -p hoprd-localcluster --test session_udp \
//!   -E 'test(localcluster_udp_session_pingpong_64kb)' --run-ignored ignored-only -j 1
//! nix develop -c cargo nextest run -p hoprd-localcluster --test session_udp \
//!   -E 'test(localcluster_udp_session_pingpong_1mb)' --run-ignored ignored-only -j 1
//! ```

mod common;

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use common::{ClusterCleanup, ClusterEnv, TempCluster};
use hoprd_localcluster::client_helper;
use hoprd_localcluster::identity;
use tokio::net::UdpSocket;

/// Payload data per chunk.  Datagrams stay well under SESSION_MTU = 1020.
const CHUNK_SIZE: usize = 900;
const TAG_SIZE: usize = 4;

const P2P_HOST: &str = "127.0.0.1";
const P2P_PORT_BASE: u16 = 19300;
const TIMEOUT: Duration = Duration::from_secs(300);
const WAIT_TIMEOUT: Duration = Duration::from_secs(120);

#[tokio::test]
#[ignore = "requires external chain container and hoprd binary \u{2014} run explicitly, not in CI"]
async fn localcluster_udp_session_pingpong_32b() {
    go(32).await
}
#[tokio::test]
#[ignore = "requires external chain container and hoprd binary \u{2014} run explicitly, not in CI"]
async fn localcluster_udp_session_pingpong_200b() {
    go(200).await
}

#[tokio::test]
#[ignore = "requires external chain container and hoprd binary \u{2014} run explicitly, not in CI"]
async fn localcluster_udp_session_pingpong_64kb() {
    go(65536).await
}

#[tokio::test]
#[ignore = "requires external chain container and hoprd binary \u{2014} run explicitly, not in CI"]
async fn localcluster_udp_session_pingpong_1mb() {
    go(1048576).await
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

/// Append a 4-byte big-endian sequence tag at the end of the data.
/// The tag survives the echo round-trip because the echo server echoes
/// the whole datagram unchanged.
fn tag_chunk_end(idx: u32, data: &[u8]) -> Vec<u8> {
    let mut v = Vec::with_capacity(data.len() + TAG_SIZE);
    v.extend_from_slice(data);
    v.extend_from_slice(&idx.to_be_bytes());
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
        strategies: identity::StrategySet {
            auto_redeeming: false,
            channel_lifecycle: false,
        },
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

/// Open a cluster, start a UDP session, send `payload_size` bytes through
/// the HOPR network in 900 B datagrams, and verify the echo response.
///
/// The session uses NoDelay + Segmentation capabilities and a boosted SURB
/// balancer to sustain ~4 300 datagrams/sec.
async fn go(payload_size: usize) {
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
        .open_session(client_helper::OpenSessionRequest {
            protocol: "udp",
            destination: exit,
            target: &target,
            hops: 1,
            capabilities: Some(vec![
                hoprd_api_client::types::SessionCapability::Segmentation,
                hoprd_api_client::types::SessionCapability::NoDelay,
            ]),
            response_buffer: Some("10 MB".to_string()),
            max_surb_upstream: Some("50 Mb/s".to_string()),
            pix_ssa_quota: None,
        })
        .await
        .unwrap();
    tracing::info!("session on {ip}:{port} (elapsed={:?})", t0.elapsed());

    let sock = UdpSocket::bind("127.0.0.1:0").await.unwrap();
    sock.connect(format!("{ip}:{port}")).await.unwrap();
    let sock = Arc::new(sock);

    let payload = gen_payload(payload_size);

    // Build tagged chunks: [data][4B tag]
    let chunks: Vec<Vec<u8>> = payload
        .chunks(CHUNK_SIZE)
        .enumerate()
        .map(|(i, c)| tag_chunk_end(i as u32, c))
        .collect();
    let nchunks = chunks.len();
    tracing::info!(
        "{payload_size} B in {nchunks} datagrams (elapsed={:?})",
        t0.elapsed()
    );

    // Steady-paced send loop — forward traffic carries SURBs to the exit.
    // The PID-controlled balancer ramps up the return rate during the first
    // cycle so most responses arrive concurrently with sends.  Duplicates
    // are harmless (recv dedupes by tag).
    const SEND_INTERVAL: Duration = Duration::from_micros(230);

    let n_remaining = Arc::new(AtomicUsize::new(nchunks));
    let done = Arc::new(std::sync::Mutex::new(vec![false; nchunks]));

    let send_sock = sock.clone();
    let send_chunks = chunks.clone();
    let send_rem = n_remaining.clone();
    let send_h = tokio::spawn(async move {
        while send_rem.load(Ordering::Acquire) > 0 {
            for chk in &send_chunks {
                if send_rem.load(Ordering::Acquire) == 0 {
                    break;
                }
                send_sock.send(chk).await.unwrap();
                tokio::time::sleep(SEND_INTERVAL).await;
            }
        }
    });

    // Concurrent receiver: read responses, extract the end-of-datagram tag,
    // verify data integrity, and acknowledge completion.
    let recv_sock = sock.clone();
    let recv_done = done.clone();
    let recv_rem = n_remaining.clone();
    let recv_chunks = chunks.clone();
    let mut recv_h = tokio::spawn(async move {
        let mut buf = vec![0u8; 65536];
        while recv_rem.load(Ordering::Acquire) > 0 {
            let r = tokio::time::timeout(Duration::from_secs(60), recv_sock.recv(&mut buf)).await;
            let n = match r {
                Ok(Ok(n)) if n >= TAG_SIZE => n,
                Ok(Ok(_)) => continue,
                Ok(Err(e)) => panic!("recv err: {e}"),
                Err(_) => continue,
            };
            let tag = u32::from_be_bytes([buf[n - 4], buf[n - 3], buf[n - 2], buf[n - 1]]) as usize;
            if tag >= nchunks {
                continue;
            }
            // The whole datagram, tag included, must match what was sent. Comparing only
            // `..n - TAG_SIZE` slices both sides to the *received* length, so a truncated
            // response carrying the original trailing tag compares equal against its own prefix
            // and is then counted as a complete chunk.
            if buf[..n] != recv_chunks[tag][..] {
                panic!(
                    "data mismatch at chunk {tag}: received {n} B, expected {} B",
                    recv_chunks[tag].len()
                );
            }
            let mut g = recv_done.lock().unwrap();
            if !g[tag] {
                g[tag] = true;
                recv_rem.fetch_sub(1, Ordering::Release);
            }
        }
    });

    // Bound the transfer. The receive loop treats its own 60 s read timeout as "try again", so
    // if responses stop for good `recv_rem` never reaches zero — and because the sender shares
    // that counter it keeps re-sending too, wedging the whole test rather than failing it. There
    // is no outer deadline anywhere else: `TIMEOUT` is only applied during cluster bring-up.
    let receive_result = tokio::time::timeout(TIMEOUT, &mut recv_h).await;
    if receive_result.is_err() {
        recv_h.abort();
    }
    // Stop the sender before closing the session, so nothing is still writing into a Session the
    // next line is tearing down.
    send_h.abort();
    let _ = send_h.await;
    receive_result
        .expect("UDP session receive did not complete before TIMEOUT")
        .expect("UDP session receive task failed");

    tracing::info!(
        "all {payload_size}B ({nchunks} chunks) done in {:?}",
        t0.elapsed()
    );
    entry.api.close_client(&ip, port).await.unwrap();
    tracing::info!("DONE {payload_size}B in {:?}", t0.elapsed());
}
