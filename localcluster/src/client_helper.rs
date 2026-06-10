use std::{
    path::Path,
    process::{Child, Command, Stdio},
};

use anyhow::{Context, Result};
use hoprd_api_client;
use hoprd_api_client::types::{
    OpenChannelBodyRequest, RoutingOptions, SessionClientRequest, SessionTargetSpec,
};
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use tracing::debug;

#[derive(Debug, Clone)]
pub struct HoprdApiClient {
    inner: hoprd_api_client::Client,
}

impl HoprdApiClient {
    pub fn new(base_url: String, token: Option<String>) -> Result<Self> {
        let mut headers = HeaderMap::new();
        if let Some(token) = token {
            let value = format!("Bearer {token}");
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&value).context("invalid api token")?,
            );
        }

        let http_client = reqwest::ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(30))
            .default_headers(headers)
            .build()
            .context("failed to build http client")?;

        Ok(Self {
            inner: hoprd_api_client::Client::new_with_client(base_url.as_ref(), http_client),
        })
    }

    pub async fn wait_started(&self, timeout: std::time::Duration) -> Result<()> {
        self.wait_status("/startedz", timeout).await
    }

    pub async fn wait_ready(&self, timeout: std::time::Duration) -> Result<()> {
        self.wait_status("/readyz", timeout).await
    }

    async fn wait_status(&self, path: &str, timeout: std::time::Duration) -> Result<()> {
        let start = std::time::Instant::now();
        loop {
            let ready = match path {
                "/startedz" => self.inner.startedz().await,
                "/readyz" => self.inner.readyz().await,
                _ => anyhow::bail!("unknown status path: {path}"),
            };
            if ready.is_ok() {
                return Ok(());
            }

            if start.elapsed() > timeout {
                anyhow::bail!("timeout while waiting for {}", path);
            }

            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }

    pub async fn addresses(&self) -> Result<String> {
        let response = self.inner.addresses().await?;
        Ok(response.into_inner().native)
    }

    pub async fn is_outgoing_channel_open(&self, destination: &str) -> Result<bool> {
        let resp = self
            .inner
            .list_channels(None, None)
            .await
            .map_err(|e| anyhow::anyhow!("list_channels: {e}"))?;
        let dest_lower = destination.to_lowercase();
        Ok(resp
            .into_inner()
            .outgoing
            .iter()
            .any(|ch| ch.peer_address.to_lowercase() == dest_lower && ch.status == "Open"))
    }

    pub async fn ping_peer(&self, address: &str) -> Result<()> {
        self.inner.ping_peer(address).await?;
        Ok(())
    }

    pub async fn open_channel(&self, destination: &str, amount: &str) -> Result<()> {
        let body = OpenChannelBodyRequest {
            amount: amount.to_string(),
            destination: destination.to_string(),
        };
        self.inner
            .open_channel(&body)
            .await
            .map_err(|e| anyhow::anyhow!("open_channel to {destination}: {e}"))?;
        Ok(())
    }

    /// Open a session (`protocol` = `"tcp"` or `"udp"`) to `destination` (exit node
    /// on-chain address) using `hops` intermediate relays on both forward and return
    /// paths. The exit forwards the plaintext to `target` (`ip:port`). Returns the
    /// `(ip, port)` of the listener bound on this (entry) node.
    ///
    /// SURB knobs (`response_buffer`, `max_surb_upstream`) are left at protocol defaults.
    pub async fn open_session(
        &self,
        protocol: &str,
        destination: &str,
        target: &str,
        hops: u64,
    ) -> Result<(String, u16)> {
        let body = SessionClientRequest {
            destination: destination.to_string(),
            forward_path: RoutingOptions::Hops(hops),
            return_path: RoutingOptions::Hops(hops),
            target: SessionTargetSpec::Plain(target.to_string()),
            capabilities: None,
            listen_host: None,
            max_client_sessions: None,
            max_surb_upstream: None,
            response_buffer: None,
            session_pool: None,
        };
        let resp = self
            .inner
            .create_client(protocol, &body)
            .await
            .map_err(|e| anyhow::anyhow!("create {protocol} session to {destination}: {e}"))?
            .into_inner();
        Ok((resp.ip, resp.port as u16))
    }
}

pub struct NodeProcess {
    pub id: usize,
    pub api_port: u16,
    pub p2p_port: u16,
    pub api: HoprdApiClient,
    pub child: Child,
    pub address: Option<String>,
}

pub async fn wait_full_mesh_reachable(
    nodes: &[NodeProcess],
    timeout: std::time::Duration,
) -> Result<()> {
    let start = std::time::Instant::now();
    loop {
        let pairs: Vec<_> = nodes
            .iter()
            .flat_map(|src| {
                nodes.iter().filter_map(move |dst| {
                    let src_addr = src.address.as_deref()?;
                    let dst_addr = dst.address.as_deref()?;
                    if src_addr == dst_addr {
                        return None;
                    }
                    Some((src.id, dst.id, src.api.clone(), dst_addr.to_string()))
                })
            })
            .collect();

        let results = futures::future::join_all(
            pairs
                .iter()
                .map(|(_, _, api, dst)| api.ping_peer(dst.as_str())),
        )
        .await;

        let failed: Vec<_> = pairs
            .iter()
            .zip(results.iter())
            .filter(|(_, r)| r.is_err())
            .map(|((src, dst, _, _), _)| (*src, *dst))
            .collect();

        if failed.is_empty() {
            return Ok(());
        }

        if start.elapsed() > timeout {
            let pairs_str: Vec<_> = failed.iter().map(|(s, d)| format!("{s}→{d}")).collect();
            anyhow::bail!(
                "timeout waiting for peer visibility: {}",
                pairs_str.join(", ")
            );
        }

        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }
}

/// Parameters for [`start_nodes`].
pub struct NodeStartConfig<'a> {
    pub num_nodes: usize,
    pub hoprd_bin: &'a Path,
    pub data_dir: &'a Path,
    pub log_dir: &'a Path,
    pub api_host: &'a str,
    pub api_port_base: u16,
    pub p2p_host: &'a str,
    pub p2p_port_base: u16,
    pub identity_password: &'a str,
    pub api_token: Option<String>,
}

/// Spawn `config.num_nodes` hoprd processes and return their handles.
pub async fn start_nodes(config: &NodeStartConfig<'_>) -> Result<Vec<NodeProcess>> {
    use std::fs;

    let api_client_host = if config.api_host == "0.0.0.0" {
        "127.0.0.1"
    } else {
        config.api_host
    };

    let mut nodes = Vec::new();

    let effective_num_nodes = config.num_nodes.clamp(1, crate::identity::MAX_NUM_NODES);
    for id in 0..effective_num_nodes {
        let api_port = config.api_port_base + id as u16;
        let p2p_port = config.p2p_port_base + id as u16;
        let cfg_file = config.data_dir.join(format!("hoprd_cfg_{id}.yaml"));
        if !cfg_file.exists() {
            anyhow::bail!("missing hoprd config file: {}", cfg_file.display());
        }
        let db_dir = config.data_dir.join(format!("db_{id}"));
        fs::create_dir_all(db_dir.join("node_db")).with_context(|| {
            format!(
                "failed to create db directory {}",
                db_dir.join("node_db").display()
            )
        })?;
        let log_file_path = config.log_dir.join(format!("hoprd_{id}.log"));
        let log_file =
            std::fs::File::create(&log_file_path).context("failed to create hoprd log file")?;
        let log_err = log_file
            .try_clone()
            .context("failed to clone hoprd log file handle")?;

        let mut cmd = Command::new(config.hoprd_bin);
        cmd.arg("--configurationFilePath")
            .arg(&cfg_file)
            .arg("--api")
            .arg("--apiHost")
            .arg(config.api_host)
            .arg("--apiPort")
            .arg(api_port.to_string())
            .arg("--host")
            .arg(format!("{}:{}", config.p2p_host, p2p_port))
            .arg("--password")
            .arg(config.identity_password)
            .env(
                "HOPRD_OTEL_SIGNALS",
                std::env::var("HOPRD_OTEL_SIGNALS").unwrap_or_else(|_| "metrics".to_string()),
            )
            .env(
                "HOPRD_OTLP_ENDPOINT",
                std::env::var("HOPRD_OTLP_ENDPOINT")
                    .unwrap_or_else(|_| "http://localhost:4318".to_string()),
            )
            .env(
                "HOPRD_METRIC_EXPORT_INTERVAL",
                std::env::var("HOPRD_METRIC_EXPORT_INTERVAL")
                    .unwrap_or_else(|_| "15000,hopr_session=1000".to_string()),
            )
            .env(
                "HOPR_TX_TIMEOUT_MULTIPLIER",
                crate::identity::DEFAULT_TX_TIMEOUT_MULTIPLIER.to_string(),
            )
            .env("HOPR_BLOKLI_NO_COMPAT_CHECK", "1")
            .stdout(Stdio::from(log_file))
            .stderr(Stdio::from(log_err));

        if let Some(token) = &config.api_token {
            cmd.arg("--apiToken").arg(token);
        }

        debug!("starting hoprd node {} with command: {:?}", id, cmd);
        let child = cmd.spawn().context("failed to start hoprd")?;
        let api = HoprdApiClient::new(
            format!("http://{}:{}", api_client_host, api_port),
            config.api_token.clone(),
        )?;

        nodes.push(NodeProcess {
            id,
            api_port,
            p2p_port,
            api,
            child,
            address: None,
        });
    }

    Ok(nodes)
}

/// Poll until every node has an outgoing `Open` channel to every other node.
pub async fn wait_full_mesh_channels(
    nodes: &[NodeProcess],
    timeout: std::time::Duration,
) -> Result<()> {
    if let Some(node) = nodes.iter().find(|n| n.address.is_none()) {
        anyhow::bail!(
            "node {} address not resolved before waiting for full-mesh channels",
            node.id
        );
    }

    let start = std::time::Instant::now();
    loop {
        let pairs: Vec<_> = nodes
            .iter()
            .flat_map(|src| {
                nodes.iter().filter_map(move |dst| {
                    let src_addr = src.address.as_deref()?;
                    let dst_addr = dst.address.as_deref()?;
                    if src_addr == dst_addr {
                        return None;
                    }
                    Some((src.id, dst.id, src.api.clone(), dst_addr.to_string()))
                })
            })
            .collect();

        let results = futures::future::join_all(
            pairs
                .iter()
                .map(|(_, _, api, dst)| api.is_outgoing_channel_open(dst.as_str())),
        )
        .await;

        let missing: Vec<_> = pairs
            .iter()
            .zip(results.iter())
            .filter(|(_, r)| !matches!(r, Ok(true)))
            .map(|((src, dst, _, _), _)| (*src, *dst))
            .collect();

        if missing.is_empty() {
            return Ok(());
        }

        if start.elapsed() > timeout {
            let pairs_str: Vec<_> = missing.iter().map(|(s, d)| format!("{s}→{d}")).collect();
            anyhow::bail!(
                "timeout waiting for full-mesh channels: {}",
                pairs_str.join(", ")
            );
        }

        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}

pub async fn open_full_mesh_channels(
    nodes: &[NodeProcess],
    amount: &str,
    timeout: std::time::Duration,
) -> Result<()> {
    if let Some(node) = nodes.iter().find(|n| n.address.is_none()) {
        anyhow::bail!(
            "node {} address not resolved before opening full-mesh channels",
            node.id
        );
    }

    let start = std::time::Instant::now();
    loop {
        let pairs: Vec<_> = nodes
            .iter()
            .flat_map(|src| {
                nodes.iter().filter_map(move |dst| {
                    let src_addr = src.address.as_deref()?;
                    let dst_addr = dst.address.as_deref()?;
                    if src_addr == dst_addr {
                        return None;
                    }
                    Some((src.id, dst.id, src.api.clone(), dst_addr.to_string()))
                })
            })
            .collect();

        let mut missing = Vec::new();
        for (src, dst, api, addr) in pairs {
            if api.is_outgoing_channel_open(addr.as_str()).await? {
                continue;
            }

            let open_result = api.open_channel(addr.as_str(), amount).await;
            if open_result.is_err() && !api.is_outgoing_channel_open(addr.as_str()).await? {
                missing.push((src, dst));
            }
        }

        if missing.is_empty() {
            return Ok(());
        }

        if start.elapsed() > timeout {
            let pairs_str: Vec<_> = missing.iter().map(|(s, d)| format!("{s}→{d}")).collect();
            anyhow::bail!(
                "timeout opening full-mesh channels: {}",
                pairs_str.join(", ")
            );
        }

        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}
