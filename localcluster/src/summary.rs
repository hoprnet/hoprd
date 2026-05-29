//! Machine-readable cluster summary.
//!
//! Once the cluster reaches the "running" state, its metadata (blokli URL, per-node
//! addresses/endpoints, extra identity details) is captured into [`ClusterSummary`].
//! This is the single source of truth for both the human-readable stdout table and
//! the JSON consumed by external tooling via `--summary-file` and the `status`
//! subcommand. It replaces fragile stdout scraping.

use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::{cli, client_helper::NodeProcess, identity::GeneratedIdentity};

/// Full snapshot of a running cluster.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterSummary {
    /// Base URL of the Blokli indexer / chain services.
    pub blokli_url: String,
    pub nodes: Vec<NodeSummary>,
    /// Extra (non-node) pre-funded identities. Empty when none were requested.
    #[serde(default)]
    pub extras: Vec<ExtraSummary>,
}

/// A single running hoprd node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeSummary {
    pub id: usize,
    /// On-chain peer (EVM) address, or `null` if it could not be fetched.
    pub address: Option<String>,
    /// REST API base URL.
    pub api_url: String,
    /// API bearer token, or `null` if authentication is disabled.
    pub api_token: Option<String>,
    /// P2P `host:port` the node listens on.
    pub p2p: String,
    /// Convenience URL opening this node in the hopr-admin UI.
    pub node_admin_url: String,
    /// OS process id of the spawned hoprd.
    pub pid: u32,
}

/// A pre-funded extra identity (Safe + Module + on-disk keystore), not run as a node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtraSummary {
    pub id: usize,
    pub address: String,
    pub safe_address: String,
    pub module_address: String,
    /// Path to the encrypted keystore file.
    pub keystore_path: String,
    /// Password protecting the keystore.
    pub password: String,
}

impl ClusterSummary {
    /// Build the summary from live cluster state.
    pub fn build(
        nodes: &[NodeProcess],
        args: &cli::Args,
        blokli_url: &str,
        extras: &[GeneratedIdentity],
    ) -> Self {
        // 0.0.0.0 is not routable for clients; advertise loopback instead.
        let api_host = if args.api_host == "0.0.0.0" {
            "127.0.0.1"
        } else {
            &args.api_host
        };

        let nodes = nodes
            .iter()
            .map(|node| {
                let api_url = format!("http://{}:{}", api_host, node.api_port);
                let mut node_admin_url =
                    format!("http://localhost:4677/node/info?apiEndpoint={api_url}");
                if let Some(token) = &args.api_token {
                    node_admin_url.push_str(&format!("&apiToken={token}"));
                }
                NodeSummary {
                    id: node.id,
                    address: node.address.clone(),
                    api_url,
                    api_token: args.api_token.clone(),
                    p2p: format!("{}:{}", &args.p2p_host, node.p2p_port),
                    node_admin_url,
                    pid: node.child.id(),
                }
            })
            .collect();

        let extras = extras
            .iter()
            .map(|extra| ExtraSummary {
                id: extra.id,
                address: extra.address.clone(),
                safe_address: extra.safe_address.clone(),
                module_address: extra.module_address.clone(),
                keystore_path: extra.id_file.display().to_string(),
                password: extra.password.clone(),
            })
            .collect();

        Self {
            blokli_url: blokli_url.to_string(),
            nodes,
            extras,
        }
    }

    /// Serialize to pretty JSON.
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).context("failed to serialize cluster summary")
    }

    /// Write the summary as pretty JSON to `path`.
    pub fn write_file(&self, path: &Path) -> Result<()> {
        let json = self.to_json()?;
        std::fs::write(path, json)
            .with_context(|| format!("failed to write summary file {}", path.display()))
    }

    /// Read a previously written summary from `path`.
    pub fn read_file(path: &Path) -> Result<Self> {
        let contents = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read summary file {}", path.display()))?;
        serde_json::from_str(&contents)
            .with_context(|| format!("failed to parse summary file {}", path.display()))
    }

    /// Render the human-readable table printed to stdout.
    pub fn render_human(&self) -> String {
        let mut out = String::new();
        out.push('\n');
        out.push_str(&format!("Chain (Blokli): {}\n\n", self.blokli_url));

        for node in &self.nodes {
            let rows = [
                (
                    "Address",
                    node.address.clone().unwrap_or_else(|| "N/A".to_string()),
                ),
                ("P2P", node.p2p.clone()),
                ("API host", node.api_url.clone()),
                (
                    "API token",
                    node.api_token.clone().unwrap_or_else(|| "N/A".to_string()),
                ),
                ("Node admin", node.node_admin_url.clone()),
                ("PID", node.pid.to_string()),
            ];
            let label_width = rows.iter().map(|(label, _)| label.len()).max().unwrap_or(0);

            out.push_str(&format!("Node {}\n", node.id));
            for (label, value) in rows {
                out.push_str(&format!(
                    "\t{label:<width$}: {value}\n",
                    width = label_width
                ));
            }
            out.push('\n');
        }

        for extra in &self.extras {
            let rows = [
                ("Address", extra.address.clone()),
                ("Safe address", extra.safe_address.clone()),
                ("Module address", extra.module_address.clone()),
                ("Identity file", extra.keystore_path.clone()),
                ("Password", extra.password.clone()),
            ];
            let label_width = rows.iter().map(|(label, _)| label.len()).max().unwrap_or(0);

            out.push_str(&format!("Extra {}\n", extra.id));
            for (label, value) in rows {
                out.push_str(&format!(
                    "\t{label:<width$}: {value}\n",
                    width = label_width
                ));
            }
            out.push('\n');
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> ClusterSummary {
        ClusterSummary {
            blokli_url: "http://chain:8080".to_string(),
            nodes: vec![NodeSummary {
                id: 0,
                address: Some("0xabc".to_string()),
                api_url: "http://127.0.0.1:3000".to_string(),
                api_token: Some("tok".to_string()),
                p2p: "localhost:9000".to_string(),
                node_admin_url:
                    "http://localhost:4677/node/info?apiEndpoint=http://127.0.0.1:3000&apiToken=tok"
                        .to_string(),
                pid: 4242,
            }],
            extras: vec![ExtraSummary {
                id: 0,
                address: "0xdef".to_string(),
                safe_address: "0xsafe".to_string(),
                module_address: "0xmod".to_string(),
                keystore_path: "/tmp/extra_id_0.id".to_string(),
                password: "local-cluster".to_string(),
            }],
        }
    }

    #[test]
    fn json_round_trip() {
        let original = sample();
        let json = original.to_json().unwrap();
        let parsed: ClusterSummary = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.blokli_url, original.blokli_url);
        assert_eq!(parsed.nodes.len(), 1);
        assert_eq!(parsed.nodes[0].pid, 4242);
        assert_eq!(parsed.extras[0].keystore_path, "/tmp/extra_id_0.id");
    }

    #[test]
    fn extras_default_when_missing() {
        let json = r#"{"blokli_url":"http://x","nodes":[]}"#;
        let parsed: ClusterSummary = serde_json::from_str(json).unwrap();
        assert!(parsed.extras.is_empty());
    }

    #[test]
    fn write_then_read_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("summary.json");
        let original = sample();
        original.write_file(&path).unwrap();

        let parsed = ClusterSummary::read_file(&path).unwrap();
        assert_eq!(parsed.nodes[0].address, Some("0xabc".to_string()));
        assert_eq!(parsed.extras[0].safe_address, "0xsafe");
    }

    #[test]
    fn render_human_contains_key_fields() {
        let rendered = sample().render_human();
        assert!(rendered.contains("Chain (Blokli): http://chain:8080"));
        assert!(rendered.contains("Node 0"));
        assert!(rendered.contains("0xabc"));
        assert!(rendered.contains("Extra 0"));
        assert!(rendered.contains("/tmp/extra_id_0.id"));
    }
}
