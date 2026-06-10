//! Manual throughput test: push 500 MiB through a TCP connection layered on a HOPR
//! session, between two endpoints of a 3-node local cluster, with ~100 ms of artificial
//! latency on the first hop (each direction) — simulating a Warsaw–California link.
//!
//! This test is `#[ignore]` (run it explicitly); it spins up real `hoprd` processes and
//! talks to a real chain. It either reuses a cluster already running at the control base
//! or spawns the `hoprd-localcluster` binary itself and tears it down on exit.
//!
//! Topology (1-hop session): node 0 (entry) → node 1 (relay) → node 2 (exit). Only the
//! two endpoints exchange application data; node 1 only relays. Latency is applied to the
//! first physical hop, links 0↔1.
//!
//! Run with:
//!   HOPRD_BIN=./target/release/hoprd \
//!   HOPRD_CHAIN_URL=http://localhost:8080 \
//!   RUST_LOG=info \
//!   cargo nextest run -p hoprd-localcluster --test throughput --run-ignored all -j 1 --no-capture

use std::{
    path::PathBuf,
    process::{Child, Command, Stdio},
    time::{Duration, Instant},
};

use anyhow::{Context, Result, bail};
use nix::{sys::signal::{self, Signal}, unistd::Pid};
use hoprd_localcluster::{
    client_helper::HoprdApiClient,
    control,
    summary::{ClusterState, ClusterSummary},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

const NUM_NODES: usize = 3;
const PAYLOAD_BYTES: usize = 500 * 1024 * 1024;
const CHUNK_BYTES: usize = 1024 * 1024;
const PROGRESS_LOG_BYTES: usize = 25 * 1024 * 1024;
const FIRST_HOP_LATENCY: &str = "100ms";

const API_PORT_BASE: u16 = 3010;
const P2P_PORT_BASE: u16 = 9010;
const LATENCY_PORT_BASE: u16 = 9110;

const PROVISION_TIMEOUT: Duration = Duration::from_secs(900);
const TRANSFER_TIMEOUT: Duration = Duration::from_secs(600);

/// Stable, persistent data directory (NOT `std::env::temp_dir()`: under `nix develop` that
/// resolves to an ephemeral `/tmp/nix-shell.*` wiped when the shell exits, which would
/// delete the cluster logs and break cross-run reuse). Override with `HOPRD_LC_DATA_DIR`.
fn data_dir() -> PathBuf {
    std::env::var("HOPRD_LC_DATA_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp/hopr-localcluster-throughput"))
}

/// Resolve `HOPRD_BIN` (default `hoprd`) to a path the spawned orchestrator can execute.
/// A *relative* path like `./target/release/hoprd` is meaningless to the child: cargo/
/// nextest run this test with CWD set to the crate dir (`localcluster/`), so it is resolved
/// against the workspace root (parent of `CARGO_MANIFEST_DIR`). Absolute paths and bare
/// command names (resolved via `PATH`) are passed through unchanged.
fn resolve_hoprd_bin() -> PathBuf {
    let raw = std::env::var("HOPRD_BIN").unwrap_or_else(|_| "hoprd".to_string());
    let path = PathBuf::from(&raw);
    if path.is_absolute() || !raw.contains('/') {
        return path;
    }
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crate dir has a parent")
        .to_path_buf();
    let joined = workspace_root.join(path);
    std::fs::canonicalize(&joined).unwrap_or(joined)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore]
async fn tcp_session_500mib_one_hop_with_latency() {
    run().await.expect("throughput test failed");
}

async fn run() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()))
        .with_target(false)
        .try_init()
        .ok();

    let cluster = ensure_cluster().await?;
    let summary = &cluster.summary;

    anyhow::ensure!(
        summary.nodes.len() >= NUM_NODES,
        "cluster has {} nodes, need at least {NUM_NODES}",
        summary.nodes.len()
    );

    let entry = node_client(summary, 0)?;
    let exit_address = summary.nodes[2]
        .address
        .clone()
        .context("exit node (2) has no on-chain address in cluster status")?;

    // TCP sink the exit node forwards the session's plaintext to. Bind before opening the
    // session so the target is reachable the moment data starts flowing.
    let sink = TcpListener::bind("127.0.0.1:0").await?;
    let sink_addr = sink.local_addr()?;
    let sink_task = tokio::spawn(async move {
        let (mut stream, _) = sink.accept().await?;
        let mut buf = vec![0u8; CHUNK_BYTES];
        let mut total = 0usize;
        let mut next_log = PROGRESS_LOG_BYTES;
        // Stop at the expected byte count rather than waiting for EOF: the session may not
        // propagate the entry-side write shutdown to the exit→target connection, so a
        // read-until-EOF loop could block forever after all data has arrived.
        while total < PAYLOAD_BYTES {
            let n = stream.read(&mut buf).await?;
            if n == 0 {
                break;
            }
            total += n;
            if total >= next_log {
                tracing::info!("  sink received {} MiB", total / (1024 * 1024));
                next_log += PROGRESS_LOG_BYTES;
            }
        }
        Ok::<usize, std::io::Error>(total)
    });

    tracing::info!(
        "opening 1-hop TCP session: entry node 0 -> exit {} (target {})",
        exit_address,
        sink_addr
    );
    let (listen_ip, listen_port) = entry
        .open_tcp_session(&exit_address, &sink_addr.to_string(), 1)
        .await?;
    tracing::info!("session listener bound at {listen_ip}:{listen_port}");

    let mut client = TcpStream::connect((listen_ip.as_str(), listen_port))
        .await
        .with_context(|| format!("connecting to session listener {listen_ip}:{listen_port}"))?;

    tracing::info!(
        "pushing {} MiB through the session",
        PAYLOAD_BYTES / (1024 * 1024)
    );
    let start = Instant::now();
    let chunk = vec![0xABu8; CHUNK_BYTES];
    let mut sent = 0usize;
    let mut next_log = PROGRESS_LOG_BYTES;
    while sent < PAYLOAD_BYTES {
        let n = CHUNK_BYTES.min(PAYLOAD_BYTES - sent);
        client.write_all(&chunk[..n]).await?;
        sent += n;
        if sent >= next_log {
            let mib = sent as f64 / (1024.0 * 1024.0);
            let secs = start.elapsed().as_secs_f64();
            tracing::info!("  sent {:.0} MiB ({:.2} MiB/s avg)", mib, mib / secs);
            next_log += PROGRESS_LOG_BYTES;
        }
    }
    client.flush().await?;
    tracing::info!(
        "all {} MiB written into the session in {:.1}s; waiting for the sink to drain",
        PAYLOAD_BYTES / (1024 * 1024),
        start.elapsed().as_secs_f64()
    );

    // Do NOT shut down the write half here: `write_all` only means the bytes are buffered
    // in the entry node's session, not delivered. Closing now would tear the session down
    // and drop the still-in-flight tail. The sink stops at exactly PAYLOAD_BYTES, so we
    // don't need EOF — just keep the stream open until it has drained.
    let received = tokio::time::timeout(TRANSFER_TIMEOUT, sink_task)
        .await
        .context("timed out waiting for the sink to drain the session")???;
    let elapsed = start.elapsed();
    let _ = client.shutdown().await;

    anyhow::ensure!(
        received == PAYLOAD_BYTES,
        "sink received {received} bytes, expected {PAYLOAD_BYTES}"
    );

    let mib = PAYLOAD_BYTES as f64 / (1024.0 * 1024.0);
    tracing::info!(
        "transferred {:.0} MiB in {:.1}s = {:.2} MiB/s (first-hop latency {} each way)",
        mib,
        elapsed.as_secs_f64(),
        mib / elapsed.as_secs_f64(),
        FIRST_HOP_LATENCY,
    );

    Ok(())
}

fn node_client(summary: &ClusterSummary, id: usize) -> Result<HoprdApiClient> {
    let node = &summary.nodes[id];
    HoprdApiClient::new(node.api_url.clone(), node.api_token.clone())
}

/// A provisioned cluster plus the live status snapshot used to address its nodes. When we
/// spawned the binary ourselves, the child is killed on drop; a reused cluster is left
/// running.
struct Cluster {
    summary: ClusterSummary,
    _child: Option<ChildGuard>,
}

struct ChildGuard(Child);
impl Drop for ChildGuard {
    fn drop(&mut self) {
        // SIGTERM (not SIGKILL): the orchestrator traps it and runs its own cleanup, which
        // tears down the spawned hoprd nodes and latency relays. A hard kill would orphan
        // them and leave the API/P2P ports bound, blocking the next run.
        let pid = Pid::from_raw(self.0.id() as i32);
        let _ = signal::kill(pid, Signal::SIGTERM);
        for _ in 0..50 {
            match self.0.try_wait() {
                Ok(Some(_)) => return,
                _ => std::thread::sleep(Duration::from_millis(200)),
            }
        }
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

async fn ensure_cluster() -> Result<Cluster> {
    let data_dir = data_dir();
    std::fs::create_dir_all(&data_dir)?;
    let control_base = data_dir.join("cluster");
    let socket = control::socket_path(&control_base);

    if let Some(summary) = running_summary(&socket).await? {
        tracing::warn!(
            "reusing cluster already running at {} (latency/topology taken as-is)",
            control_base.display()
        );
        return Ok(Cluster {
            summary,
            _child: None,
        });
    }

    let hoprd_bin = resolve_hoprd_bin();
    let chain_url =
        std::env::var("HOPRD_CHAIN_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());

    let latency_cfg = data_dir.join("latency.yaml");
    std::fs::write(
        &latency_cfg,
        format!(
            "per_link:\n  - {{ from: 0, to: 1, delay: \"{FIRST_HOP_LATENCY}\" }}\n  - {{ from: 1, to: 0, delay: \"{FIRST_HOP_LATENCY}\" }}\n"
        ),
    )?;

    let bin = env!("CARGO_BIN_EXE_hoprd-localcluster");
    let log = std::fs::File::create(data_dir.join("orchestrator.log"))?;
    let log_err = log.try_clone()?;

    tracing::info!("spawning hoprd-localcluster ({NUM_NODES} nodes) at {bin}");
    let child = Command::new(bin)
        .arg("--size")
        .arg(NUM_NODES.to_string())
        .arg("--chain-url")
        .arg(&chain_url)
        .arg("--hoprd-bin")
        .arg(&hoprd_bin)
        .arg("--data-dir")
        .arg(&data_dir)
        .arg("--api-port-base")
        .arg(API_PORT_BASE.to_string())
        .arg("--p2p-port-base")
        .arg(P2P_PORT_BASE.to_string())
        .arg("--latency-port-base")
        .arg(LATENCY_PORT_BASE.to_string())
        .arg("--latency-config")
        .arg(&latency_cfg)
        .stdout(Stdio::from(log))
        .stderr(Stdio::from(log_err))
        .spawn()
        .context("failed to spawn hoprd-localcluster binary")?;
    let mut guard = ChildGuard(child);

    let summary = wait_running(&socket, &mut guard.0, PROVISION_TIMEOUT).await?;
    Ok(Cluster {
        summary,
        _child: Some(guard),
    })
}

/// Return the live summary iff a cluster is currently `running`.
async fn running_summary(socket: &std::path::Path) -> Result<Option<ClusterSummary>> {
    let json = control::query(socket).await?;
    let value: serde_json::Value = serde_json::from_str(&json)?;
    if value.get("state").and_then(|s| s.as_str()) != Some("running") {
        return Ok(None);
    }
    Ok(Some(serde_json::from_str(&json)?))
}

async fn wait_running(
    socket: &std::path::Path,
    child: &mut Child,
    timeout: Duration,
) -> Result<ClusterSummary> {
    let start = Instant::now();
    loop {
        // Fail fast if the orchestrator died (e.g. couldn't spawn hoprd) instead of
        // polling a dead socket until the timeout elapses.
        if let Some(status) = child.try_wait()? {
            bail!(
                "hoprd-localcluster exited early ({status}); see {}/orchestrator.log",
                data_dir().display()
            );
        }
        let json = control::query(socket).await?;
        if let Ok(summary) = serde_json::from_str::<ClusterSummary>(&json) {
            match summary.state {
                ClusterState::Running => return Ok(summary),
                ClusterState::Failed => {
                    bail!("cluster failed to start: {:?}", summary.error)
                }
                _ => {}
            }
        }
        if start.elapsed() > timeout {
            bail!("timeout waiting for cluster to reach 'running'");
        }
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}
