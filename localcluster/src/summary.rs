//! Machine-readable, progressively-updated cluster summary.
//!
//! The summary is the single source of truth for both the human-readable stdout table
//! and the JSON served to external tooling. The orchestrator holds it in memory and
//! updates it on every lifecycle transition — chain ready, each node spawned/started/
//! ready, channels open, shutdown. The live snapshot is served over a unix socket (see
//! [`crate::control`]); the `status` subcommand queries it and keys off the structured
//! `state` fields instead of scraping logs, with a deterministic stop criterion
//! (`state == "running"`).
//!
//! When no instance is listening, `status` reports `not_running`, so callers always get
//! a parseable answer.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::{cli, client_helper::NodeProcess, identity::GeneratedIdentity, latency::LatencyConfig};

/// Overall lifecycle state of the cluster.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClusterState {
    /// No instance is listening for this data directory; nothing is running.
    NotRunning,
    /// Booting chain services and generating identities.
    Initializing,
    /// Nodes are spawning and coming up; not all are ready yet.
    Starting,
    /// All nodes are ready and channels (if any) are open. Stop criterion for tooling.
    Running,
    /// Shutdown was requested; processes are being torn down.
    ShuttingDown,
    /// Startup failed; see `error`.
    Failed,
}

/// Lifecycle state of a single node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeState {
    /// Not yet spawned.
    Pending,
    /// Process spawned; not yet answering `/startedz`.
    Spawned,
    /// Passed `/startedz`.
    Started,
    /// Passed `/readyz`; address known.
    Ready,
    /// Outgoing channels to all peers are open.
    ChannelsOpen,
    /// This node failed to come up.
    Failed,
}

/// Full snapshot of a cluster at one point in its lifecycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterSummary {
    pub state: ClusterState,
    /// OS process id of the orchestrator that owns this summary.
    pub pid: u32,
    /// Base URL of the Blokli indexer / chain services, once known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blokli_url: Option<String>,
    pub nodes: Vec<NodeSummary>,
    /// Extra (non-node) pre-funded identities. Empty when none were requested.
    #[serde(default)]
    pub extras: Vec<ExtraSummary>,
    /// Failure detail, set only when `state == Failed`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// A single hoprd node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeSummary {
    pub id: usize,
    pub state: NodeState,
    /// On-chain peer (EVM) address, or `null` until fetched.
    pub address: Option<String>,
    /// REST API base URL.
    pub api_url: String,
    /// API bearer token, or `null` if authentication is disabled.
    pub api_token: Option<String>,
    /// P2P `host:port` peers dial to reach the node — the latency relay when latency is
    /// enabled, otherwise the node's own listen address.
    pub p2p: String,
    /// Artificial latency applied to traffic arriving at this node, or `null` when latency
    /// is disabled. A single value (e.g. `200ms`, `70-130ms uniform`) when uniform across
    /// sources, otherwise a per-source breakdown.
    #[serde(default)]
    pub latency: Option<String>,
    /// Convenience URL opening this node in the hopr-admin UI.
    pub node_admin_url: String,
    /// OS process id of the spawned hoprd, or `null` until spawned.
    pub pid: Option<u32>,
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
    /// Build the initial summary from CLI args, before anything has started.
    ///
    /// Endpoints are deterministic from the args (`base + id`), so they are populated
    /// up front; per-node `address`/`pid` and the chain URL are filled in as they become
    /// known.
    ///
    /// When `latency` is `Some`, relays are active: the `p2p` field reports the relay port
    /// peers actually dial (not the real listen port), and each node's `latency` field
    /// describes the delay applied to its inbound traffic.
    pub fn initial(args: &cli::Args, latency: Option<&LatencyConfig>) -> Self {
        let p2p_port_base = match &args.latency {
            Some(l) => l.port_base,
            None => args.p2p_port_base,
        } as usize;
        let nodes = (0..args.size)
            .map(|id| {
                let api_url = format!(
                    "http://{}:{}",
                    advertised_host(&args.api_host),
                    args.api_port_base as usize + id
                );
                let mut node_admin_url =
                    format!("http://localhost:4677/node/info?apiEndpoint={api_url}");
                if let Some(token) = &args.api_token {
                    node_admin_url.push_str(&format!("&apiToken={token}"));
                }
                NodeSummary {
                    id,
                    state: NodeState::Pending,
                    address: None,
                    api_url,
                    api_token: args.api_token.clone(),
                    p2p: format!("{}:{}", advertised_host(&args.p2p_host), p2p_port_base + id),
                    latency: latency.and_then(|cfg| cfg.describe_inbound(id, args.size)),
                    node_admin_url,
                    pid: None,
                }
            })
            .collect();

        Self {
            state: ClusterState::Initializing,
            pid: std::process::id(),
            blokli_url: None,
            nodes,
            extras: Vec::new(),
            error: None,
        }
    }

    /// Summary returned when no cluster is running for the queried data directory.
    pub fn not_running() -> Self {
        Self {
            state: ClusterState::NotRunning,
            pid: 0,
            blokli_url: None,
            nodes: Vec::new(),
            extras: Vec::new(),
            error: None,
        }
    }

    /// Replace the extra-identity list (known after identity generation).
    pub fn set_extras(&mut self, extras: &[GeneratedIdentity]) {
        self.extras = extras
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
    }

    /// Record the live pid for each spawned node and mark it [`NodeState::Spawned`].
    pub fn mark_spawned(&mut self, nodes: &[NodeProcess]) {
        for proc in nodes {
            if let Some(node) = self.node_mut(proc.id) {
                node.pid = Some(proc.child.id());
                node.state = NodeState::Spawned;
            }
        }
    }

    /// Set a single node's lifecycle state.
    pub fn set_node_state(&mut self, id: usize, state: NodeState) {
        if let Some(node) = self.node_mut(id) {
            node.state = state;
        }
    }

    /// Set every node's lifecycle state.
    pub fn set_all_node_states(&mut self, state: NodeState) {
        for node in &mut self.nodes {
            node.state = state;
        }
    }

    /// Record a fetched node address.
    pub fn set_node_address(&mut self, id: usize, address: String) {
        if let Some(node) = self.node_mut(id) {
            node.address = Some(address);
        }
    }

    fn node_mut(&mut self, id: usize) -> Option<&mut NodeSummary> {
        self.nodes.iter_mut().find(|n| n.id == id)
    }

    /// Serialize to pretty JSON.
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).context("failed to serialize cluster summary")
    }

    /// Render the human-readable table printed to stdout.
    pub fn render_human(&self) -> String {
        let mut out = String::new();
        out.push('\n');
        out.push_str(&format!("State: {}\n", self.state_label()));
        out.push_str(&format!(
            "Chain (Blokli): {}\n\n",
            self.blokli_url.as_deref().unwrap_or("N/A")
        ));

        for node in &self.nodes {
            let rows = [
                ("State", node.state_label().to_string()),
                (
                    "Address",
                    node.address.clone().unwrap_or_else(|| "N/A".to_string()),
                ),
                ("P2P", node.p2p.clone()),
                (
                    "Latency",
                    node.latency.clone().unwrap_or_else(|| "none".to_string()),
                ),
                ("API host", node.api_url.clone()),
                (
                    "API token",
                    node.api_token.clone().unwrap_or_else(|| "N/A".to_string()),
                ),
                ("Node admin", node.node_admin_url.clone()),
                (
                    "PID",
                    node.pid
                        .map(|p| p.to_string())
                        .unwrap_or_else(|| "N/A".to_string()),
                ),
            ];
            let label_width = rows.iter().map(|(label, _)| label.len()).max().unwrap_or(0);

            out.push_str(&format!("Node {}\n", node.id));
            for (label, value) in rows {
                out.push_str(&format!("\t{label:<label_width$}: {value}\n"));
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
                out.push_str(&format!("\t{label:<label_width$}: {value}\n"));
            }
            out.push('\n');
        }

        out
    }

    fn state_label(&self) -> &'static str {
        match self.state {
            ClusterState::NotRunning => "not_running",
            ClusterState::Initializing => "initializing",
            ClusterState::Starting => "starting",
            ClusterState::Running => "running",
            ClusterState::ShuttingDown => "shutting_down",
            ClusterState::Failed => "failed",
        }
    }
}

impl NodeSummary {
    fn state_label(&self) -> &'static str {
        match self.state {
            NodeState::Pending => "pending",
            NodeState::Spawned => "spawned",
            NodeState::Started => "started",
            NodeState::Ready => "ready",
            NodeState::ChannelsOpen => "channels_open",
            NodeState::Failed => "failed",
        }
    }
}

/// `0.0.0.0` and `auto` (bind-all) are not routable for clients; advertise loopback instead.
pub fn advertised_host(host: &str) -> &str {
    match host {
        "0.0.0.0" | "auto" => "127.0.0.1",
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> ClusterSummary {
        ClusterSummary {
            state: ClusterState::Running,
            pid: 4242,
            blokli_url: Some("http://chain:8080".to_string()),
            nodes: vec![NodeSummary {
                id: 0,
                state: NodeState::ChannelsOpen,
                address: Some("0xabc".to_string()),
                api_url: "http://127.0.0.1:3000".to_string(),
                api_token: Some("tok".to_string()),
                p2p: "localhost:9000".to_string(),
                latency: Some("200ms".to_string()),
                node_admin_url:
                    "http://localhost:4677/node/info?apiEndpoint=http://127.0.0.1:3000&apiToken=tok"
                        .to_string(),
                pid: Some(5000),
            }],
            extras: vec![ExtraSummary {
                id: 0,
                address: "0xdef".to_string(),
                safe_address: "0xsafe".to_string(),
                module_address: "0xmod".to_string(),
                keystore_path: "/tmp/extra_id_0.id".to_string(),
                password: "local-cluster".to_string(),
            }],
            error: None,
        }
    }

    #[test]
    fn json_round_trip() {
        let original = sample();
        let json = original.to_json().unwrap();
        let parsed: ClusterSummary = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.state, ClusterState::Running);
        assert_eq!(parsed.nodes.len(), 1);
        assert_eq!(parsed.nodes[0].state, NodeState::ChannelsOpen);
        assert_eq!(parsed.nodes[0].pid, Some(5000));
        assert_eq!(parsed.extras[0].keystore_path, "/tmp/extra_id_0.id");
    }

    #[test]
    fn state_serializes_snake_case() {
        let json = sample().to_json().unwrap();
        assert!(json.contains("\"state\": \"running\""));
        assert!(json.contains("\"state\": \"channels_open\""));
    }

    #[test]
    fn extras_default_when_missing() {
        let json = r#"{"state":"running","pid":1,"nodes":[]}"#;
        let parsed: ClusterSummary = serde_json::from_str(json).unwrap();
        assert!(parsed.extras.is_empty());
    }

    #[test]
    fn not_running_serializes() {
        let json = ClusterSummary::not_running().to_json().unwrap();
        assert!(json.contains("\"state\": \"not_running\""));
    }

    #[test]
    fn render_human_contains_key_fields() {
        let rendered = sample().render_human();
        assert!(rendered.contains("State: running"));
        assert!(rendered.contains("Chain (Blokli): http://chain:8080"));
        assert!(rendered.contains("Node 0"));
        assert!(rendered.contains("0xabc"));
        assert!(rendered.contains("Extra 0"));
        assert!(rendered.contains("/tmp/extra_id_0.id"));
    }
}
