//! Live status control socket.
//!
//! The running orchestrator serves the current [`ClusterSummary`] on a unix domain
//! socket at `<data-dir>/cluster.sock`. The `status` subcommand connects, reads the
//! JSON snapshot, and prints it — always querying the live in-memory state rather than
//! a file that could go stale. When no instance is listening, [`query`] synthesizes a
//! [`ClusterState::NotRunning`](crate::summary::ClusterState::NotRunning) reply so callers always get a parseable answer.
//!
//! The summary is shared as `Arc<Mutex<ClusterSummary>>`; the orchestrator mutates it
//! through the lifecycle while the accept loop serves read-only snapshots.

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::{Context, Result};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{UnixListener, UnixStream},
    sync::Mutex,
    task::JoinHandle,
};
use tracing::{debug, warn};

use crate::summary::ClusterSummary;

/// Shared, mutable cluster summary served over the control socket.
pub type SharedSummary = Arc<Mutex<ClusterSummary>>;

/// Default control socket path for a data directory.
pub fn socket_path(data_dir: &Path) -> PathBuf {
    data_dir.join("cluster.sock")
}

/// A running control server. Dropping it stops the accept loop and removes the socket.
pub struct ControlServer {
    path: PathBuf,
    handle: JoinHandle<()>,
}

impl ControlServer {
    /// Bind the control socket and start serving snapshots of `shared`.
    ///
    /// The caller must already hold the single-instance lock, so any pre-existing socket
    /// file is guaranteed stale and is removed before binding.
    pub fn start(path: PathBuf, shared: SharedSummary) -> Result<Self> {
        let _ = std::fs::remove_file(&path);
        let listener = UnixListener::bind(&path)
            .with_context(|| format!("failed to bind control socket {}", path.display()))?;

        let handle = tokio::spawn(async move {
            loop {
                match listener.accept().await {
                    Ok((stream, _)) => {
                        let json = { shared.lock().await.to_json() };
                        match json {
                            Ok(json) => serve_one(stream, json).await,
                            Err(err) => warn!(error = %err, "failed to serialize cluster summary"),
                        }
                    }
                    Err(err) => {
                        warn!(error = %err, "control socket accept failed; stopping server");
                        break;
                    }
                }
            }
        });

        Ok(Self { path, handle })
    }
}

impl Drop for ControlServer {
    fn drop(&mut self) {
        self.handle.abort();
        let _ = std::fs::remove_file(&self.path);
    }
}

async fn serve_one(mut stream: UnixStream, json: String) {
    if let Err(err) = async {
        stream.write_all(json.as_bytes()).await?;
        stream.shutdown().await
    }
    .await
    {
        debug!(error = %err, "failed to write status to control socket client");
    }
}

/// Query a running cluster's live summary as JSON.
///
/// Returns a [`ClusterState::NotRunning`](crate::summary::ClusterState::NotRunning) JSON document when nothing is listening
/// (socket missing or connection refused).
pub async fn query(path: &Path) -> Result<String> {
    match UnixStream::connect(path).await {
        Ok(mut stream) => {
            let mut buf = String::new();
            stream
                .read_to_string(&mut buf)
                .await
                .with_context(|| format!("failed to read status from {}", path.display()))?;
            Ok(buf)
        }
        Err(err)
            if matches!(
                err.kind(),
                std::io::ErrorKind::NotFound | std::io::ErrorKind::ConnectionRefused
            ) =>
        {
            serde_json::to_string_pretty(&serde_json::json!({ "state": "not_running" }))
                .context("failed to serialize not_running status")
        }
        Err(err) => Err(err)
            .with_context(|| format!("failed to connect to control socket {}", path.display())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::summary::ClusterState;

    #[tokio::test]
    async fn query_missing_socket_reports_not_running() {
        let dir = tempfile::tempdir().unwrap();
        let path = socket_path(dir.path());
        let json = query(&path).await.unwrap();
        assert!(json.contains("\"state\": \"not_running\""));
    }

    #[tokio::test]
    async fn serves_live_snapshot() {
        let dir = tempfile::tempdir().unwrap();
        let path = socket_path(dir.path());
        let shared: SharedSummary = Arc::new(Mutex::new(ClusterSummary::not_running()));
        {
            let mut s = shared.lock().await;
            s.state = ClusterState::Running;
        }
        let _server = ControlServer::start(path.clone(), shared).unwrap();

        let json = query(&path).await.unwrap();
        assert!(json.contains("\"state\": \"running\""));
    }
}
