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

use std::{path::PathBuf, time::Duration};

use anyhow::{Context, Result};
use hoprd_localcluster::{blokli_helper, client_helper};

/// Environment-derived configuration for a test cluster run.
#[derive(Clone, Debug)]
pub struct ClusterEnv {
    pub hoprd_bin: PathBuf,
    pub chain_url: Option<String>,
    pub chain_image: Option<String>,
    pub container_runtime: String,
    pub wait_timeout: Duration,
}

impl ClusterEnv {
    /// Read configuration from environment variables, using sensible defaults.
    pub fn from_env() -> Result<Self> {
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
            wait_timeout: Duration::from_secs(120),
        })
    }
}

/// A temporary localcluster working directory.
pub struct TempCluster {
    /// Kept alive for the lifetime of the cluster; dropped last to clean up
    /// the on-disk directory tree.
    pub _temp_dir: tempfile::TempDir,
    pub data_dir: PathBuf,
    pub log_dir: PathBuf,
}

impl TempCluster {
    pub fn new() -> Result<Self> {
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
pub struct ClusterCleanup {
    pub chain: Option<blokli_helper::ChainHandle>,
    pub nodes: Vec<client_helper::NodeProcess>,
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
pub async fn start_chain(
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
pub async fn wait_for_blokli_ready(url: &str, timeout: Duration) -> Result<()> {
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
