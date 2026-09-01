//! Shared utilities for localcluster integration tests.
//!
//! # Prerequisites
//!
//! The `hoprd` binary must be built in **release** mode before running any
//! localcluster test.  Each test file documents this in its own `# Prerequisites`
//! section.
//!
//! ```bash
//! nix develop -c cargo build --release -p hoprd
//! export HOPRD_BIN=$(pwd)/target/release/hoprd
//! ```
//!
//! # Serial execution
//!
//! **These tests MUST run one at a time.**  Each test starts a chain container
//! and spawns 3 hoprd processes that use fixed port ranges (P2P, API), on-
//! chain state, and the local loopback — all of which conflict when multiple
//! tests share the machine.  There is no locking or namespace isolation; the
//! cluster is a singleton on the host.
//!
//! All tests are `#[ignore = "..."]` with a description indicating they require
//! a chain container and the `hoprd` binary.  They are not intended for CI —
//! invoke them explicitly by name with `--run-ignored ignored-only`.
//!
//! When invoked via `cargo nextest`, use `-j 1` and an expression that matches
//! exactly one test.  Running all tests of an integration-test binary in
//! parallel (nextest default) will corrupt cluster state and produce spurious
//! failures.
//!
//! ```bash
//! nix develop -c cargo nextest run -p hoprd-localcluster --test smoke --run-ignored ignored-only -j 1
//! ```
//!
//! Each test sets up a temporary directory, resolves the chain source (an
//! existing Blokli URL or a Docker container launched from `HOPRD_CHAIN_IMAGE`),
//! and tracks `Cleanup` to kill processes on drop.

// Six integration binaries compile this module separately and each uses a different subset of
// it, so anything not used by all six reads as dead code in the other five. One allow for the
// module rather than an allow per item; the cost is that a helper nobody uses at all stops
// being reported.
#![allow(dead_code)]

pub mod pix;

use std::{path::PathBuf, time::Duration};

use anyhow::{Context, Result};
use hoprd_localcluster::{blokli_helper, client_helper};

/// Environment-derived configuration for a test cluster run.
///
/// Read once by [`Cluster::start`]; no test constructs one.
#[derive(Clone, Debug)]
struct ClusterEnv {
    hoprd_bin: PathBuf,
    chain_url: Option<String>,
    chain_image: Option<String>,
    container_runtime: String,
}

impl ClusterEnv {
    /// Read configuration from environment variables, using sensible defaults.
    fn from_env() -> Result<Self> {
        let chain_url = std::env::var("HOPRD_CHAIN_URL").ok();
        let chain_image = std::env::var("HOPRD_CHAIN_IMAGE").ok();
        anyhow::ensure!(
            chain_url.is_some() || chain_image.is_some(),
            "set HOPRD_CHAIN_URL (existing chain) or HOPRD_CHAIN_IMAGE (to start a container)"
        );

        Ok(Self {
            hoprd_bin: PathBuf::from(
                std::env::var("HOPRD_BIN").unwrap_or_else(|_| "hoprd".to_string()),
            ),
            chain_url,
            chain_image,
            container_runtime: std::env::var("HOPRD_CONTAINER_RUNTIME")
                .unwrap_or_else(|_| "docker".to_string()),
        })
    }
}

/// A temporary localcluster working directory. Owned by [`Cluster`].
struct TempCluster {
    /// Kept alive for the lifetime of the cluster; dropped last to clean up
    /// the on-disk directory tree.
    _temp_dir: tempfile::TempDir,
    data_dir: PathBuf,
    log_dir: PathBuf,
}

impl TempCluster {
    fn new() -> Result<Self> {
        let temp_dir = tempfile::tempdir()?;
        let data_dir = temp_dir.path().to_path_buf();
        let log_dir = data_dir.join("logs");
        std::fs::create_dir_all(&log_dir)?;
        Ok(Self {
            _temp_dir: temp_dir,
            data_dir,
            log_dir,
        })
    }
}

/// Resources that must be cleaned up at the end of a test (chain container +
/// spawned hoprd processes).
struct ClusterCleanup {
    chain: Option<blokli_helper::ChainHandle>,
    nodes: Vec<client_helper::NodeProcess>,
}

impl Drop for ClusterCleanup {
    fn drop(&mut self) {
        for n in &mut self.nodes {
            let _ = n.child.kill();
        }
        if let Some(c) = &mut self.chain {
            c.stop();
        }
    }
}

/// Start a chain container (or use an existing one), returning the Blokli URL.
async fn start_chain(
    env: &ClusterEnv,
    log_dir: &std::path::Path,
    cleanup: &mut ClusterCleanup,
) -> Result<String> {
    if let Some(url) = &env.chain_url {
        Ok(url.trim_end_matches('/').to_string())
    } else {
        let img = env.chain_image.as_deref().unwrap();
        let handle = blokli_helper::ChainHandle::start(&env.container_runtime, img, log_dir)
            .context("failed to start chain container")?;
        let url = handle.chain_url();
        cleanup.chain = Some(handle);
        Ok(url)
    }
}

/// Poll the Blokli `/readyz` endpoint until it returns success.
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

/// Copies every node log out of `from` and into `to` when it drops, so the logs outlive the
/// [`TempCluster`] directory that is deleted on the way out.
///
/// A named struct rather than a closure guard so a [`Cluster`] can own one, which is what
/// makes it impossible to arm the copy *after* bring-up — the ordering that used to discard
/// the logs of exactly the failure the copy exists for. The chain container writes into the
/// same directory, so its log is preserved too.
///
/// Failures are reported rather than swallowed. This runs precisely when a test has failed
/// and someone is about to go looking, and "the nodes logged nothing" and "the harness could
/// not copy the logs" are indistinguishable from the destination directory. It cannot fail
/// the test — it may be dropping during an unwind, where a panic would abort the process —
/// so it warns and carries on.
///
/// `to` is a fixed per-suite path so the logs are findable without knowing where the temp
/// directory was; the drop logs it on the way out. Nothing is cleared first, so same-named
/// files are overwritten but a file left by a longer earlier run of the same suite survives
/// alongside this one's.
///
/// A fixed path under a world-writable `/tmp` is somebody else's to create first, and
/// `create_dir_all` succeeds happily against a directory that is already theirs. hoprd logs
/// carry on-chain addresses, peer ids and Session detail, so the destination is made `0700`
/// and its owner is checked against the owner of `from` — a [`tempfile::TempDir`] this
/// process created, hence this user. A destination that fails the check is left untouched
/// and the logs are simply not copied.
pub struct NodeLogs {
    from: PathBuf,
    to: &'static str,
}

impl NodeLogs {
    pub fn new(from: PathBuf, to: &'static str) -> Self {
        Self { from, to }
    }
}

impl Drop for NodeLogs {
    fn drop(&mut self) {
        use std::os::unix::fs::{DirBuilderExt, MetadataExt, PermissionsExt};

        let (logs, to) = (&self.from, self.to);
        let dest = std::path::Path::new(to);
        let owner = match std::fs::metadata(logs) {
            Ok(meta) => meta.uid(),
            Err(error) => {
                tracing::warn!(dir = %logs.display(), %error, "cannot stat the node log directory");
                return;
            }
        };
        // A fresh directory is 0700 from the moment it exists, before anything is written to it.
        if let Err(error) = std::fs::DirBuilder::new()
            .recursive(true)
            .mode(0o700)
            .create(dest)
        {
            tracing::warn!(dest = to, %error, "cannot create the log destination");
            return;
        }
        match std::fs::symlink_metadata(dest) {
            Ok(meta) if meta.is_dir() && meta.uid() == owner => {
                // `recursive` above left an existing directory's mode alone, and earlier runs of
                // this harness created it 0755. Tighten it now that it is known to be ours, so
                // the guarantee holds for the second run and not only the first. The copies
                // themselves keep the source's 0644; the directory is what gates access.
                let mode = std::fs::Permissions::from_mode(0o700);
                if let Err(error) = std::fs::set_permissions(dest, mode) {
                    tracing::warn!(dest = to, %error, "cannot restrict the log destination");
                }
            }
            Ok(_) => {
                tracing::warn!(
                    dest = to,
                    "log destination is not a directory this user owns"
                );
                return;
            }
            Err(error) => {
                tracing::warn!(dest = to, %error, "cannot stat the log destination");
                return;
            }
        }
        let entries = match std::fs::read_dir(logs) {
            Ok(entries) => entries,
            Err(error) => {
                tracing::warn!(dir = %logs.display(), %error, "cannot read the node log directory");
                return;
            }
        };
        for entry in entries {
            match entry {
                Ok(entry) => {
                    let (src, dst) = (entry.path(), dest.join(entry.file_name()));
                    if let Err(error) = std::fs::copy(&src, &dst) {
                        tracing::warn!(src = %src.display(), dst = %dst.display(), %error, "cannot copy a node log");
                    }
                }
                Err(error) => {
                    tracing::warn!(dir = %logs.display(), %error, "cannot read a log directory entry")
                }
            }
        }
        tracing::info!(dest = to, "node logs copied");
    }
}

/// The P2P and API port bases one integration binary owns.
#[derive(Clone, Copy, Debug)]
pub struct PortBlock {
    pub p2p: u16,
    pub api: u16,
}

/// One block per integration binary. Node `i` of a suite binds `p2p + i` and `api + i`.
///
/// Stated here rather than in each test file because the invariant is a global one — no two
/// suites may overlap — and it cannot be checked from inside a single test. The suites are not
/// isolated from each other in any other way (see the serial-execution note in the module
/// docs), so a collision is a corrupted run rather than a bind error.
pub mod ports {
    use super::PortBlock;

    pub const SMOKE: PortBlock = PortBlock {
        p2p: 19000,
        api: 13000,
    };
    pub const ADDRESS_IDENTITY: PortBlock = PortBlock {
        p2p: 19100,
        api: 13100,
    };
    pub const CHANNEL_CLOSE: PortBlock = PortBlock {
        p2p: 19200,
        api: 13200,
    };
    pub const SESSION_UDP: PortBlock = PortBlock {
        p2p: 19300,
        api: 13300,
    };
    pub const SESSION_PIX: PortBlock = PortBlock {
        p2p: 19400,
        api: 13400,
    };
    /// Pinned. `scripts/pix-demo.sh` hard-codes `API_PORT_BASE=13500` and identifies every
    /// node — to scrape it, and to `pkill` it — by `--apiPort $((API_PORT_BASE + i))`.
    /// Moving this silently breaks the demo's dashboard and its teardown.
    pub const SESSION_PIX_SOAK: PortBlock = PortBlock {
        p2p: 19500,
        api: 13500,
    };

    const ALL: [PortBlock; 6] = [
        SMOKE,
        ADDRESS_IDENTITY,
        CHANNEL_CLOSE,
        SESSION_UDP,
        SESSION_PIX,
        SESSION_PIX_SOAK,
    ];

    /// No two blocks may come within `MAX_NUM_NODES` of each other, in either dimension.
    const _: () = {
        const fn apart(a: u16, b: u16) -> bool {
            a.abs_diff(b) as usize >= hoprd_localcluster::identity::MAX_NUM_NODES
        }
        let mut i = 0;
        while i < ALL.len() {
            let mut j = i + 1;
            while j < ALL.len() {
                assert!(apart(ALL[i].p2p, ALL[j].p2p), "P2P port blocks overlap");
                assert!(apart(ALL[i].api, ALL[j].api), "API port blocks overlap");
                j += 1;
            }
            i += 1;
        }
    };
}

/// Everything a test varies about cluster bring-up.
///
/// Construct with [`ClusterSpec::new`], which takes the one field that has no safe default,
/// and override only what differs:
///
/// ```ignore
/// Cluster::start(ClusterSpec {
///     num_nodes: 4,
///     pix: Some(pix_settings()?),
///     ..ClusterSpec::new(ports::SESSION_PIX_SOAK)
/// }).await?
/// ```
#[derive(Clone, Debug)]
pub struct ClusterSpec {
    pub ports: PortBlock,
    pub num_nodes: usize,
    pub random_identities: bool,
    pub strategies: hoprd_localcluster::identity::StrategySet,
    /// `None` leaves hoprd's own 60 s default, which is what a test relying on a strategy to
    /// act *during* bring-up needs. A Session test that does not want strategies interfering
    /// pushes it out instead.
    pub strategy_execution_interval: Option<Duration>,
    pub pix: Option<hoprd_localcluster::identity::PixSettings>,
    /// Deadline for every node to report `HoprState::Running`.
    pub start_timeout: Duration,
    /// Where to copy the node logs when the cluster drops. `None` discards them along with
    /// the temp directory.
    pub logs_to: Option<&'static str>,
}

impl ClusterSpec {
    /// Defaults matching the majority of the suites: three nodes, random identities,
    /// AutoRedeeming on and ChannelLifecycle off, hoprd's own strategy interval, no PIX.
    pub fn new(ports: PortBlock) -> Self {
        Self {
            ports,
            num_nodes: 3,
            random_identities: true,
            strategies: hoprd_localcluster::identity::StrategySet::default(),
            strategy_execution_interval: None,
            pix: None,
            start_timeout: Duration::from_secs(120),
            logs_to: None,
        }
    }
}

/// A cluster that is up: chain running, identities generated, nodes spawned and started,
/// every node's on-chain address resolved.
///
/// Nothing beyond that. Readiness, channels and peer reachability are separate steps because
/// the suites want different ones in different orders — `smoke` checks reachability *before*
/// channels because its strategy opens them, the Session suites check it after because they
/// opened the channels themselves. A composite would hide that difference, and would grow a
/// boolean the first time one suite needed to skip a step.
#[must_use = "dropping the Cluster kills the nodes and stops the chain"]
pub struct Cluster {
    // Field order is drop order: the logs are copied out first, while the temp directory that
    // holds them still exists, and `_temp` is deleted last.
    _logs: Option<NodeLogs>,
    cleanup: ClusterCleanup,
    _temp: TempCluster,
    /// The generated identities, for a test that checks a node against its own.
    pub identities: hoprd_localcluster::identity::GenerationOutput,
    log_dir: PathBuf,
}

impl Cluster {
    /// Start the chain, generate identities and configs, spawn the nodes, wait for each to
    /// reach `HoprState::Running`, and resolve every node's on-chain address.
    pub async fn start(spec: ClusterSpec) -> Result<Self> {
        use hoprd_localcluster::identity;

        let env = ClusterEnv::from_env().context("reading cluster environment")?;
        let temp = TempCluster::new().context("creating temp cluster")?;
        let log_dir = temp.log_dir.clone();
        // Armed before anything is started, so a failure during bring-up — the case the copy
        // exists for — still leaves the node and chain logs behind.
        let logs = spec.logs_to.map(|to| NodeLogs::new(log_dir.clone(), to));
        let mut cleanup = ClusterCleanup {
            chain: None,
            nodes: vec![],
        };
        let t0 = std::time::Instant::now();

        let blokli_url = start_chain(&env, &temp.log_dir, &mut cleanup)
            .await
            .context("starting chain")?;
        wait_for_blokli_ready(&blokli_url, CHAIN_READY_TIMEOUT)
            .await
            .context("waiting for blokli")?;
        tracing::info!("chain ready after {:?}", t0.elapsed());

        let identities = identity::generate(&identity::GenerationConfig {
            blokli_url,
            num_nodes: spec.num_nodes,
            config_home: temp.data_dir.clone(),
            random_identities: spec.random_identities,
            p2p_host: P2P_HOST.to_string(),
            p2p_port_base: spec.ports.p2p,
            strategies: spec.strategies,
            strategy_execution_interval: spec.strategy_execution_interval,
            pix: spec.pix,
            ..Default::default()
        })
        .await
        .context("generating identities")?;
        tracing::info!("identities generated after {:?}", t0.elapsed());

        // The ten fields are spelled out rather than defaulted, and that is deliberate:
        // `NodeStartConfig` holds `&Path`s, which have no `Default`, and a hand-written one
        // would turn "you forgot `api_port_base`" from a compile error into a silent
        // collision with another suite's port block. This is the only construction site in
        // the test tree, so the cost is paid once.
        cleanup.nodes = client_helper::start_nodes(&client_helper::NodeStartConfig {
            num_nodes: spec.num_nodes,
            hoprd_bin: &env.hoprd_bin,
            data_dir: &temp.data_dir,
            log_dir: &temp.log_dir,
            api_host: API_HOST,
            api_port_base: spec.ports.api,
            p2p_host: P2P_HOST,
            p2p_port_base: spec.ports.p2p,
            identity_password: identity::DEFAULT_IDENTITY_PASSWORD,
            api_token: None,
        })
        .await
        .context("starting nodes")?;
        tracing::info!("nodes started after {:?}", t0.elapsed());

        futures::future::try_join_all(
            cleanup
                .nodes
                .iter()
                .map(|n| n.api.wait_started(spec.start_timeout)),
        )
        .await
        .context("waiting for nodes to start")?;
        for n in &mut cleanup.nodes {
            n.address = Some(n.api.addresses().await.context("resolving node address")?);
        }
        tracing::info!("nodes started and addressed after {:?}", t0.elapsed());

        Ok(Self {
            _logs: logs,
            cleanup,
            _temp: temp,
            identities,
            log_dir,
        })
    }

    pub fn nodes(&self) -> &[client_helper::NodeProcess] {
        &self.cleanup.nodes
    }

    pub fn node(&self, id: usize) -> &client_helper::NodeProcess {
        &self.cleanup.nodes[id]
    }

    /// The live log directory, inside the temp tree. Valid until the cluster drops.
    pub fn log_dir(&self) -> &std::path::Path {
        &self.log_dir
    }

    /// Lines of node `id`'s hoprd log that contain *every* needle.
    ///
    /// Line-scoped rather than a count of substring occurrences over the whole file, which is
    /// what a suite asserting on a `tracing` line actually needs. Two reasons, both load-bearing
    /// for the PIX suites:
    ///
    /// - A message and one of its own fields — `"…for the SSA batch"` and `batch_size=3` — are only
    ///   the same event if they are on the same line.
    /// - One PIX message is a strict prefix of another: `"pix session deposit timeout"` is the kill
    ///   switch firing, `"pix session deposit timeout set"` is it being armed. A substring count of
    ///   the former reports both, so "the kill switch never fired" would be unassertable.
    ///
    /// Colour is stripped first, and that is not cosmetic. hoprd colours its output whether or not
    /// stdout is a terminal, and `tracing` wraps a field's *name*, its `=` and its value in separate
    /// escape sequences — a line displaying `batch_size=3` holds
    /// `ESC[3mbatch_size ESC[0m ESC[2m= ESC[0m 3`. So no `field=value` needle matches the raw bytes,
    /// and a suite asserting on one silently counts zero for every line.
    ///
    /// Fallible on purpose. Some of these counts *are* primary assertions, and a missing or
    /// unreadable log would otherwise report zero — which reads as "the node never did this" when
    /// the truth is "the test never looked", and fails the companion assertion while blaming the
    /// node for a file-system problem.
    pub fn count_log_lines(&self, id: usize, needles: &[&str]) -> Result<usize> {
        let path = self.log_dir.join(format!("hoprd_{id}.log"));
        let log = std::fs::read_to_string(&path)
            .with_context(|| format!("reading {} to count {needles:?}", path.display()))?;
        Ok(strip_ansi(&log)
            .lines()
            .filter(|line| needles.iter().all(|needle| line.contains(needle)))
            .count())
    }

    /// Wait for every node's `/readyz`.
    pub async fn wait_ready(&self, timeout: Duration) -> Result<()> {
        futures::future::try_join_all(self.nodes().iter().map(|n| n.api.wait_ready(timeout)))
            .await
            .context("waiting for nodes to become ready")?;
        Ok(())
    }

    /// Open and fund an outgoing channel from every node to every other.
    pub async fn open_channels(&self, stake: &str, timeout: Duration) -> Result<()> {
        client_helper::open_full_mesh_channels(self.nodes(), stake, timeout)
            .await
            .context("opening channels")
    }

    /// Wait until every node has an `Open` outgoing channel to every other.
    pub async fn wait_channels(&self, timeout: Duration) -> Result<()> {
        client_helper::wait_full_mesh_channels(self.nodes(), timeout)
            .await
            .context("waiting for channels")
    }

    /// Wait until every node can reach every other.
    pub async fn wait_reachable(&self, timeout: Duration) -> Result<()> {
        client_helper::wait_full_mesh_reachable(self.nodes(), timeout)
            .await
            .context("waiting for peer reachability")
    }
}

/// `text` with ANSI CSI escape sequences removed. See [`Cluster::count_log_lines`] for why a
/// harness reading hoprd's log files needs this at all.
///
/// Hand-rolled rather than a dependency: this models one sequence family — `ESC [` … final byte in
/// `@`..=`~` — which is all `tracing`'s colour output emits. Anything else the escape starts is
/// dropped along with the `ESC`, so a sequence this does not model degrades to a stray character
/// rather than to swallowed text.
pub fn strip_ansi(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(esc) = rest.find('\u{1b}') {
        out.push_str(&rest[..esc]);
        let after_esc = &rest[esc + 1..];
        rest = match after_esc.strip_prefix('[') {
            Some(params) => match params.find(|c: char| ('\u{40}'..='\u{7e}').contains(&c)) {
                // The final byte is ASCII, so one past its start is a char boundary.
                Some(end) => &params[end + 1..],
                // Unterminated: the rest of the input is sequence, not text.
                None => "",
            },
            None => after_esc,
        };
    }
    out.push_str(rest);
    out
}

/// Loopback, for both the API and the P2P listeners. Not a knob: a localcluster is a
/// single-host arrangement by construction.
const API_HOST: &str = "127.0.0.1";
const P2P_HOST: &str = "127.0.0.1";

/// Every suite allowed blokli the same 120 s, so this is a constant rather than a spec field.
const CHAIN_READY_TIMEOUT: Duration = Duration::from_secs(120);

/// A UDP echo server that echoes each datagram back to its sender. Returns the bound port.
///
/// Exits its task if a receive fails, rather than looping on the error: a socket that cannot
/// receive will not start doing so, and spinning on it burns a core inside tests whose subject
/// is packet rate. A dead echo server shows up immediately as a receive timeout.
pub async fn echo_server() -> Result<u16> {
    let sock = tokio::net::UdpSocket::bind("127.0.0.1:0")
        .await
        .context("binding the echo server")?;
    let port = sock.local_addr().context("echo server address")?.port();
    tokio::spawn(async move {
        let mut buf = vec![0u8; 65535];
        while let Ok((n, src)) = sock.recv_from(&mut buf).await {
            if sock.send_to(&buf[..n], src).await.is_err() {
                break;
            }
        }
    });
    Ok(port)
}

/// Initialise a tracing subscriber with `RUST_LOG` (default: `"info"`).
///
/// Safe to call multiple times — duplicate calls are silently ignored.
pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()))
        .with_target(false)
        .try_init()
        .ok();
}
