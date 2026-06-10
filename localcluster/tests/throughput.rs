//! Manual throughput test: push data through a HOPR **session** (TCP or UDP) between two
//! endpoints of a local cluster, optionally with artificial first-hop latency, and report
//! throughput. `#[ignore]` — run it explicitly; it spins up real `hoprd` processes against
//! a real chain.
//!
//! The cluster is reused if one is already running at the control base, otherwise the
//! `hoprd-localcluster` binary is spawned and torn down on exit. Node API endpoints and
//! on-chain addresses are read from the cluster **status** (nothing hardcoded). Entry =
//! first node, exit = last node; intermediate nodes relay.
//!
//! ## Required environment
//!   HOPRD_BIN          path to the hoprd binary (default `hoprd` on PATH).
//!                      A relative path is resolved against the workspace root.
//!   HOPRD_CHAIN_URL    Blokli URL of a running chain (default http://localhost:8080).
//!
//! ## Scenario options (all optional, with defaults)
//!   HOPRD_LC_PROTO        tcp | udp            session protocol            (default tcp)
//!   HOPRD_LC_DOWNLOAD     set to any value     download shape (tiny request up, bulk
//!                                              stream down) instead of symmetric echo
//!   HOPRD_LC_HOPS         <n>                  intermediate relay hops      (default 1;
//!                                              0 = direct entry↔exit, no relay)
//!   HOPRD_LC_PAYLOAD_MIB  <n>                  payload size in MiB          (default 500)
//!   HOPRD_LC_RATE_MIBPS   <f>                  cap offered load, MiB/s; 0 = unthrottled
//!                                              (default 0). Applies to the TCP echo writer
//!                                              and the UDP send side.
//!   HOPRD_LC_NO_LATENCY   set to any value     disable the latency relays (default: on,
//!                                              ~100 ms each way on the first hop, links 0↔1)
//!   HOPRD_LC_DATA_DIR     <path>               cluster data dir   (default /tmp/hopr-localcluster-throughput)
//!   RUST_LOG              tracing filter       (default info)
//!
//! Notes:
//! - TCP asserts the full payload is transferred; UDP is lossy and only reports delivered
//!   bytes / throughput (no exact-count assert).
//! - Echo = symmetric (payload up AND back); download = payload one way (down).
//!
//! ## Examples
//!   # default: TCP echo, 500 MiB, 1 hop, 100 ms latency
//!   HOPRD_BIN=./target/release/hoprd HOPRD_CHAIN_URL=http://localhost:8080 RUST_LOG=info \
//!     cargo nextest run -p hoprd-localcluster --test throughput --run-ignored all -j1 --no-capture
//!
//!   # UDP download, 0-hop, no latency, 50 MiB
//!   HOPRD_LC_PROTO=udp HOPRD_LC_DOWNLOAD=1 HOPRD_LC_HOPS=0 HOPRD_LC_NO_LATENCY=1 \
//!   HOPRD_LC_PAYLOAD_MIB=50 HOPRD_BIN=./target/release/hoprd HOPRD_CHAIN_URL=http://localhost:8080 \
//!     cargo nextest run -p hoprd-localcluster --test throughput --run-ignored all -j1 --no-capture
//!
//!   # TCP echo, 1-hop, 100 ms latency, paced to 2 MiB/s
//!   HOPRD_LC_RATE_MIBPS=2 HOPRD_BIN=./target/release/hoprd HOPRD_CHAIN_URL=http://localhost:8080 \
//!     cargo nextest run -p hoprd-localcluster --test throughput --run-ignored all -j1 --no-capture

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
const DEFAULT_RATE_MIBPS: f64 = 0.0;

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

/// Session protocol: `"tcp"` (default) or `"udp"`. Override with `HOPRD_LC_PROTO`.
fn proto() -> String {
    std::env::var("HOPRD_LC_PROTO").unwrap_or_else(|_| "tcp".to_string())
}

/// `HOPRD_LC_DOWNLOAD=1` switches from the symmetric echo to a VPN-like download (tiny
/// request up, `payload` streamed down) — the asymmetric shape the SURB balancer targets.
fn download_mode() -> bool {
    std::env::var("HOPRD_LC_DOWNLOAD").is_ok()
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

    let payload = payload_bytes();
    let progress = progress_log_bytes();
    let download = download_mode();
    let hops = hops();

    // UDP session path is fully separate (datagrams, no stream/window/retransmit) — used to
    // isolate whether a stall is in the TCP session's reliability layer. Honours the same
    // echo/download and rate options.
    if proto() == "udp" {
        return run_udp(
            &entry,
            entry_id,
            exit_id,
            &exit_address,
            hops,
            payload,
            progress,
            download,
            rate_bytes_per_sec(),
        )
        .await;
    }

    // Target the exit node forwards the session's plaintext to. Two shapes:
    //   echo     – mirror every byte back (symmetric up == down).
    //   download – ignore the (tiny) request, stream `payload` bytes back (VPN-like:
    //              small up, bulk down — the case the SURB balancer is built for).
    // Bind before opening the session so the target is reachable the moment data flows.
    let target = TcpListener::bind("127.0.0.1:0").await?;
    let target_addr = target.local_addr()?;
    tokio::spawn(async move {
        let (mut stream, _) = target.accept().await?;
        if download {
            let (_rd, mut wr) = stream.into_split();
            let chunk = vec![0xCDu8; CHUNK_BYTES];
            let mut written = 0usize;
            while written < payload {
                let n = CHUNK_BYTES.min(payload - written);
                wr.write_all(&chunk[..n]).await?;
                written += n;
            }
            wr.flush().await?;
            // Hold the read half so the connection isn't closed before the bulk send drains.
            drop(_rd);
            Ok::<usize, std::io::Error>(written)
        } else {
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
        }
    });

    tracing::info!(
        "opening {hops}-hop TCP session: entry node {entry_id} -> exit node {exit_id} ({}) (target {})",
        exit_address,
        target_addr
    );
    let (listen_ip, listen_port) = entry
        .open_session("tcp", &exit_address, &target_addr.to_string(), hops)
        .await?;
    tracing::info!("session listener bound at {listen_ip}:{listen_port}");

    let client = TcpStream::connect((listen_ip.as_str(), listen_port))
        .await
        .with_context(|| format!("connecting to session listener {listen_ip}:{listen_port}"))?;
    let (mut rd, mut wr) = client.into_split();

    let start = Instant::now();

    if download {
        // VPN download shape: send a tiny request to open the exit→target connection, then
        // pull the bulk stream. The write half stays open for the whole download.
        tracing::info!(
            "downloading {} MiB (tiny request up, bulk stream down)",
            payload / (1024 * 1024)
        );
        wr.write_all(b"GET\n").await?;
        wr.flush().await?;

        let mut buf = vec![0u8; CHUNK_BYTES];
        let mut received = 0usize;
        let mut next_log = progress;
        let read_fut = async {
            while received < payload {
                let n = rd.read(&mut buf).await?;
                if n == 0 {
                    break;
                }
                received += n;
                if received >= next_log {
                    let mib = received as f64 / (1024.0 * 1024.0);
                    tracing::info!(
                        "  downloaded {:.0} MiB ({:.2} MiB/s)",
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
            .context("timed out waiting for the download")??;
        let elapsed = start.elapsed();
        drop(wr);

        anyhow::ensure!(
            received == payload,
            "downloaded {received} bytes, expected {payload}"
        );
        let mib = payload as f64 / (1024.0 * 1024.0);
        tracing::info!(
            "downloaded {:.0} MiB in {:.1}s = {:.2} MiB/s",
            mib,
            elapsed.as_secs_f64(),
            mib / elapsed.as_secs_f64()
        );
        return Ok(());
    }

    let rate = rate_bytes_per_sec();
    tracing::info!(
        "round-tripping {} MiB up and back through the echo server (full-duplex){}",
        payload / (1024 * 1024),
        match rate {
            Some(b) => format!(", writer throttled to {:.1} MiB/s", b / (1024.0 * 1024.0)),
            None => ", writer unthrottled".to_string(),
        }
    );

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

/// UDP-session data plane for both scenarios (`download` = bulk stream down after a tiny
/// request; otherwise `echo` = datagrams mirrored back). UDP is unreliable (no
/// stream/window/retransmit), so this reports delivered bytes / throughput rather than
/// asserting an exact count — it isolates whether a stall is in the TCP reliability layer.
#[allow(clippy::too_many_arguments)]
async fn run_udp(
    entry: &HoprdApiClient,
    entry_id: usize,
    exit_id: usize,
    exit_address: &str,
    hops: u64,
    payload: usize,
    progress: usize,
    download: bool,
    rate: Option<f64>,
) -> Result<()> {
    use std::sync::Arc;
    const DGRAM: usize = 1000; // stay under the session MTU

    let target = Arc::new(tokio::net::UdpSocket::bind("127.0.0.1:0").await?);
    let target_addr = target.local_addr()?;
    {
        let target = target.clone();
        tokio::spawn(async move {
            let mut buf = vec![0u8; 2048];
            if download {
                // Learn the exit's source address from the request, then stream back.
                let (_, peer) = target.recv_from(&mut buf).await?;
                send_datagrams(&target, Some(peer), payload, DGRAM, rate).await?;
            } else {
                // Echo: bounce every datagram back to its sender.
                loop {
                    let (n, peer) = target.recv_from(&mut buf).await?;
                    target.send_to(&buf[..n], peer).await?;
                }
            }
            Ok::<(), std::io::Error>(())
        });
    }

    tracing::info!(
        "opening {hops}-hop UDP session ({}): entry node {entry_id} -> exit node {exit_id} ({exit_address}) (target {target_addr})",
        if download { "download" } else { "echo" }
    );
    let (listen_ip, listen_port) = entry
        .open_session("udp", exit_address, &target_addr.to_string(), hops)
        .await?;
    tracing::info!("session listener bound at {listen_ip}:{listen_port}");

    let sock = Arc::new(tokio::net::UdpSocket::bind("127.0.0.1:0").await?);
    sock.connect((listen_ip.as_str(), listen_port)).await?;

    let label = if download { "downloading" } else { "echoing" };
    tracing::info!("UDP {label} {} MiB", payload / (1024 * 1024));

    if download {
        sock.send(b"GET").await?;
    } else {
        // Push the payload up as datagrams; the target mirrors them back.
        let s = sock.clone();
        tokio::spawn(async move { send_datagrams(&s, None, payload, DGRAM, rate).await });
    }

    let start = Instant::now();
    let mut buf = vec![0u8; 2048];
    let mut received = 0usize;
    let mut next_log = progress;
    loop {
        match tokio::time::timeout(Duration::from_secs(15), sock.recv(&mut buf)).await {
            Ok(Ok(n)) => {
                received += n;
                if received >= next_log {
                    let mib = received as f64 / (1024.0 * 1024.0);
                    tracing::info!(
                        "  received {:.0} MiB ({:.2} MiB/s)",
                        mib,
                        mib / start.elapsed().as_secs_f64()
                    );
                    next_log += progress;
                }
                if received >= payload {
                    break;
                }
            }
            Ok(Err(e)) => return Err(e.into()),
            Err(_) => {
                tracing::warn!("UDP recv idle 15s — stopping");
                break;
            }
        }
    }
    let elapsed = start.elapsed();
    let mib = received as f64 / (1024.0 * 1024.0);
    tracing::info!(
        "UDP {label}: received {:.1} of {} MiB in {:.1}s = {:.2} MiB/s ({} datagrams; UDP loss expected)",
        mib,
        payload / (1024 * 1024),
        elapsed.as_secs_f64(),
        if elapsed.as_secs_f64() > 0.0 {
            mib / elapsed.as_secs_f64()
        } else {
            0.0
        },
        received / DGRAM,
    );
    Ok(())
}

/// Send `payload` bytes as `dgram`-sized datagrams on a UDP socket — to `peer` via
/// `send_to` when given, else on the connected socket — optionally paced to `rate` B/s.
async fn send_datagrams(
    sock: &tokio::net::UdpSocket,
    peer: Option<std::net::SocketAddr>,
    payload: usize,
    dgram: usize,
    rate: Option<f64>,
) -> std::io::Result<()> {
    let chunk = vec![0xCDu8; dgram];
    let start = Instant::now();
    let mut sent = 0usize;
    while sent < payload {
        match peer {
            Some(p) => sock.send_to(&chunk, p).await?,
            None => sock.send(&chunk).await?,
        };
        sent += dgram;
        if let Some(bps) = rate {
            let target = Duration::from_secs_f64(sent as f64 / bps);
            let elapsed = start.elapsed();
            if target > elapsed {
                tokio::time::sleep(target - elapsed).await;
            }
        }
    }
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
