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
use hoprd_localcluster::{
    client_helper::HoprdApiClient,
    control,
    summary::{ClusterState, ClusterSummary},
};
use nix::{
    sys::signal::{self, Signal},
    unistd::Pid,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

const NUM_NODES: usize = 3;
const CHUNK_BYTES: usize = 1024 * 1024;
const DEFAULT_RATE_MIBPS: f64 = 2.0;

/// Payload size in MiB. Override with `HOPRD_LC_PAYLOAD_MIB` (default 500) to bisect the
/// throughput/SURB ceiling.
fn payload_bytes() -> usize {
    std::env::var("HOPRD_LC_PAYLOAD_MIB")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(500)
        * 1024
        * 1024
}

/// Log a progress line every ~5% of the payload.
fn progress_log_bytes() -> usize {
    (payload_bytes() / 20).max(CHUNK_BYTES)
}

/// Number of intermediate relay hops for the session (both directions). Override with
/// `HOPRD_LC_HOPS` (default 1). 0 = direct entry↔exit, no relay.
fn hops() -> u64 {
    std::env::var("HOPRD_LC_HOPS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(1)
}

/// Target offered-load rate in bytes/sec, or `None` to push unthrottled. Defaults to
/// `DEFAULT_RATE_MIBPS`; override with `HOPRD_LC_RATE_MIBPS` (set to `0` to disable
/// throttling).
fn rate_bytes_per_sec() -> Option<f64> {
    let mibps = std::env::var("HOPRD_LC_RATE_MIBPS")
        .ok()
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(DEFAULT_RATE_MIBPS);
    (mibps > 0.0).then_some(mibps * 1024.0 * 1024.0)
}
const FIRST_HOP_LATENCY: &str = "100ms";

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

    // Entry is the first node, exit the last — both resolved from status, never assumed.
    let entry_id = 0;
    let exit_id = summary.nodes.len() - 1;
    let entry = node_client(summary, entry_id)?;
    let exit_address = summary.nodes[exit_id].address.clone().with_context(|| {
        format!("exit node ({exit_id}) has no on-chain address in cluster status")
    })?;

    // Echo server the exit node forwards the session's plaintext to: it mirrors every byte
    // straight back, so the same volume travels up (entry→exit→echo) and back down
    // (echo→exit→entry). Bind before opening the session so the target is reachable.
    let payload = payload_bytes();
    let progress = progress_log_bytes();

    let echo = TcpListener::bind("127.0.0.1:0").await?;
    let echo_addr = echo.local_addr()?;
    tokio::spawn(async move {
        let (mut stream, _) = echo.accept().await?;
        let (mut rd, mut wr) = stream.split();
        let mut buf = vec![0u8; CHUNK_BYTES];
        let mut echoed = 0usize;
        while echoed < payload {
            let n = rd.read(&mut buf).await?;
            if n == 0 {
                break;
            }
            wr.write_all(&buf[..n]).await?;
            echoed += n;
        }
        wr.flush().await?;
        Ok::<usize, std::io::Error>(echoed)
    });

    let hops = hops();
    tracing::info!(
        "opening {hops}-hop TCP session: entry node {entry_id} -> exit node {exit_id} ({}) (echo target {})",
        exit_address,
        echo_addr
    );
    let (listen_ip, listen_port) = entry
        .open_tcp_session(&exit_address, &echo_addr.to_string(), hops)
        .await?;
    tracing::info!("session listener bound at {listen_ip}:{listen_port}");

    let client = TcpStream::connect((listen_ip.as_str(), listen_port))
        .await
        .with_context(|| format!("connecting to session listener {listen_ip}:{listen_port}"))?;
    let (mut rd, mut wr) = client.into_split();

    let rate = rate_bytes_per_sec();
    tracing::info!(
        "round-tripping {} MiB up and back through the echo server (full-duplex){}",
        payload / (1024 * 1024),
        match rate {
            Some(b) => format!(", writer throttled to {:.1} MiB/s", b / (1024.0 * 1024.0)),
            None => ", writer unthrottled".to_string(),
        }
    );
    let start = Instant::now();

    // Writer and reader must run concurrently: serializing them deadlocks once the echoed
    // bytes fill the socket buffers and block the writer's `write_all`.
    let writer = tokio::spawn(async move {
        let chunk = vec![0xABu8; CHUNK_BYTES];
        let mut sent = 0usize;
        let mut next_log = progress;
        while sent < payload {
            let n = CHUNK_BYTES.min(payload - sent);
            wr.write_all(&chunk[..n]).await?;
            sent += n;
            // Pace the offered load: if we're ahead of the target schedule, wait. Slowing
            // the writer keeps the entry's packet pipeline and SURB budget from being
            // overrun, trading throughput for sustainability.
            if let Some(bps) = rate {
                let target = Duration::from_secs_f64(sent as f64 / bps);
                let elapsed = start.elapsed();
                if target > elapsed {
                    tokio::time::sleep(target - elapsed).await;
                }
            }
            if sent >= next_log {
                tracing::info!("  sent {} MiB up", sent / (1024 * 1024));
                next_log += progress;
            }
        }
        wr.flush().await?;
        // Return `wr` instead of letting it drop here: dropping the write half half-closes
        // the session, which tears it down at the entry and makes the still-arriving return
        // data be rejected as "unregistered session". The caller holds it until the reader
        // has drained the full echo.
        Ok::<(usize, tokio::net::tcp::OwnedWriteHalf), std::io::Error>((sent, wr))
    });

    let mut buf = vec![0u8; CHUNK_BYTES];
    let mut received = 0usize;
    let mut next_log = progress;
    let read_fut = async {
        // Stop at the expected byte count rather than relying on EOF.
        while received < payload {
            let n = rd.read(&mut buf).await?;
            if n == 0 {
                break;
            }
            received += n;
            if received >= next_log {
                let mib = received as f64 / (1024.0 * 1024.0);
                tracing::info!(
                    "  echoed back {:.0} MiB ({:.2} MiB/s avg round-trip)",
                    mib,
                    mib / start.elapsed().as_secs_f64()
                );
                next_log += progress;
            }
        }
        Ok::<usize, std::io::Error>(received)
    };

    let received = tokio::time::timeout(TRANSFER_TIMEOUT, read_fut)
        .await
        .context("timed out waiting for echoed data to return")??;
    let elapsed = start.elapsed();

    // The reader has the full echo; only now release the write half.
    let (sent, _wr) = writer
        .await
        .context("writer task panicked")?
        .context("writer failed")?;
    drop(_wr);

    anyhow::ensure!(
        sent == payload,
        "writer sent {sent} bytes, expected {payload}"
    );
    anyhow::ensure!(
        received == payload,
        "echoed back {received} bytes, expected {payload}"
    );

    let mib = payload as f64 / (1024.0 * 1024.0);
    tracing::info!(
        "round-tripped {:.0} MiB up + {:.0} MiB back in {:.1}s = {:.2} MiB/s round-trip (first-hop latency {} each way)",
        mib,
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
    // Diagnostic toggle: HOPRD_LC_NO_LATENCY=1 spawns the cluster without latency relays,
    // to isolate whether a problem is in the relay or in the session itself.
    let with_latency = std::env::var("HOPRD_LC_NO_LATENCY").is_err();

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

    tracing::info!(
        "spawning hoprd-localcluster ({NUM_NODES} nodes, latency {}) at {bin}",
        if with_latency { "on" } else { "off" }
    );
    // Ports are left at the binary's defaults; the test never assumes them — every node
    // endpoint (api_url, api_token, on-chain address) is read back from the cluster status.
    let mut cmd = Command::new(bin);
    cmd.arg("--size")
        .arg(NUM_NODES.to_string())
        .arg("--chain-url")
        .arg(&chain_url)
        .arg("--hoprd-bin")
        .arg(&hoprd_bin)
        .arg("--data-dir")
        .arg(&data_dir);
    if with_latency {
        cmd.arg("--latency-config").arg(&latency_cfg);
    }
    let child = cmd
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
