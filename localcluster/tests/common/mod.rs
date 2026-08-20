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
    #[allow(dead_code)]
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
