//! Chain container management for the local cluster.
//!
//! [`ChainHandle`] shells out to any Docker-compatible container CLI to run the
//! Blokli + Anvil chain image.  The runtime binary is supplied by the caller;
//! it must support the following invocations:
//!
//! - `<runtime> run --rm --name <name> --platform linux/amd64 -p 8080:8080 <image>`
//! - `<runtime> rm -f <name>`
//! - `<runtime> ls` (with columns: ID IMAGE OS ARCH STATE ADDR ...)
//!
//! Common compatible runtimes: `docker`, `container` (Apple native), `podman`.
//!
//! ## Chain URL
//!
//! After the container starts, [`ChainHandle::chain_url`] returns the URL to
//! use when connecting to Blokli. When the runtime exposes containers on a
//! routable subnet (e.g. Apple `container` at `192.168.64.x`) the direct
//! container IP is preferred over `localhost:8080`, because some NAT
//! implementations time out long-lived SSE connections (used by the blokli
//! client for on-chain event subscriptions) within 20–30 s.

use std::{
    fs::{self, File},
    path::Path,
    process::{Child, Command, Stdio},
    time::{Duration, Instant},
};

use anyhow::{Context, Result};

const CHAIN_PORT: u16 = 8080;
const CONTAINER_NAME: &str = "hopr-chain";

pub struct ChainHandle {
    runtime: String,
    name: String,
    child: Child,
    /// Direct container URL if we could detect a routable IP; otherwise None
    /// (caller should fall back to `http://localhost:{CHAIN_PORT}`).
    container_ip: Option<String>,
}

impl ChainHandle {
    /// Start the chain container using `runtime` as the container CLI.
    ///
    /// `runtime` is the name (or path) of the container binary, e.g. `"docker"`,
    /// `"container"`, or `"podman"`.  The caller is responsible for ensuring the
    /// runtime daemon is running before calling this.
    pub fn start(runtime: &str, chain_image: &str, log_dir: &Path) -> Result<Self> {
        fs::create_dir_all(log_dir).context("failed to create log directory")?;
        let log_file = log_dir.join("chain.log");
        let log_file = File::create(&log_file).context("failed to create blokli log file")?;
        let log_err = log_file
            .try_clone()
            .context("failed to clone blokli log file handle")?;
        let name = CONTAINER_NAME;

        let mut cmd = Command::new(runtime);
        cmd.arg("run")
            .arg("--rm")
            .arg("--name")
            .arg(name)
            .arg("--platform")
            .arg("linux/amd64")
            .arg("-p")
            .arg(format!("{CHAIN_PORT}:{CHAIN_PORT}"))
            .arg(chain_image)
            .stdout(Stdio::from(log_file))
            .stderr(Stdio::from(log_err));

        let child = cmd.spawn().context("failed to start blokli container")?;

        // Detect the container's routable IP (if any) so callers can bypass
        // port-forwarding NAT for long-lived SSE connections.
        let container_ip = tokio::task::block_in_place(|| detect_container_ip(runtime, name));

        Ok(Self {
            runtime: runtime.to_string(),
            name: name.to_string(),
            child,
            container_ip,
        })
    }

    /// URL to use for the Blokli GraphQL/SSE endpoint.
    ///
    /// Returns the container's direct IP URL when available (preferred for
    /// runtimes like Apple `container` that expose containers on a routable
    /// subnet), otherwise falls back to `http://localhost:{CHAIN_PORT}`.
    pub fn chain_url(&self) -> String {
        self.container_ip
            .as_deref()
            .map(|ip| format!("http://{ip}:{CHAIN_PORT}"))
            .unwrap_or_else(|| format!("http://localhost:{CHAIN_PORT}"))
    }

    pub fn stop(&mut self) {
        let _ = self.child.kill();
        let _ = Command::new(&self.runtime)
            .arg("rm")
            .arg("-f")
            .arg(&self.name)
            .status();
    }
}

/// Try to find the container's routable IP by polling `<runtime> ls`.
///
/// Polls for up to 8 s in 500 ms increments. Returns the IP string (without
/// prefix length) if one is found in a non-loopback subnet, `None` otherwise.
fn detect_container_ip(runtime: &str, name: &str) -> Option<String> {
    let deadline = Instant::now() + Duration::from_secs(8);
    loop {
        if let Some(ip) = try_get_container_ip(runtime, name) {
            return Some(ip);
        }
        if Instant::now() >= deadline {
            return None;
        }
        std::thread::sleep(Duration::from_millis(500));
    }
}

fn try_get_container_ip(runtime: &str, name: &str) -> Option<String> {
    // `container ls` and `podman ps` both emit tabular output where the
    // container ID/name appears in the first column and the address in the 6th.
    // `docker ps` does not emit an IP in its default format, so this naturally
    // falls through to `None` for Docker (which relies on port forwarding).
    let out = Command::new(runtime).arg("ls").output().ok()?;

    let text = String::from_utf8_lossy(&out.stdout);
    for line in text.lines() {
        let cols: Vec<&str> = line.split_whitespace().collect();
        if cols.first().map(|s| *s) == Some(name) && cols.len() >= 6 {
            let addr = cols[5];
            // Strip CIDR prefix (e.g. "192.168.64.2/24" → "192.168.64.2")
            let ip = addr.split('/').next()?;
            if !ip.is_empty() && ip != "127.0.0.1" && ip != "::1" {
                return Some(ip.to_string());
            }
        }
    }
    None
}
