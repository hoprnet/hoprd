use std::process::Child;

use anyhow::{Context, Result};
use hopr_lib::api::types::primitive::prelude::HoprBalance;
use hoprd_api_client;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use tracing::{debug, warn};

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

    pub async fn open_channel(&self, destination: &str, amount: &str) -> Result<()> {
        debug!(destination, amount, "opening channel");
        let req = hoprd_api_client::types::OpenChannelBodyRequest {
            amount: amount.to_string(),
            destination: destination.to_string(),
        };
        for attempt in 1..=2u32 {
            match self.inner.open_channel(&req).await {
                Ok(resp) => {
                    let inner = resp.into_inner();
                    debug!(destination, channel_id = %inner.channel_id, tx = %inner.transaction_receipt, "channel opened");
                    return Ok(());
                }
                Err(hoprd_api_client::Error::UnexpectedResponse(resp)) => {
                    let status = resp.status();
                    let body = resp
                        .text()
                        .await
                        .unwrap_or_else(|_| "<body unavailable>".to_string());

                    // hoprd times out after 2*expectedBlockTime waiting for blokli to deliver
                    // the on-chain "channel opened" event. The tx was submitted (the error body
                    // contains its hash) and is almost certainly mined. Give blokli 5s to catch
                    // up, then verify via the channels list before deciding whether to retry.
                    if status.as_u16() == 422 && body.contains("timed out") {
                        warn!(
                            destination,
                            attempt,
                            "channel open confirmation timed out; waiting 5s then checking channel state"
                        );
                        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                        if self.is_outgoing_channel_open(destination).await? {
                            tracing::info!(
                                destination,
                                "channel confirmed open (blokli event arrived during 5s wait)"
                            );
                            return Ok(());
                        }
                        if attempt < 2 {
                            continue;
                        }
                    }

                    anyhow::bail!("open_channel to {destination} HTTP {status}: {body}");
                }
                Err(e) => return Err(anyhow::anyhow!(e)),
            }
        }
        unreachable!()
    }

    async fn is_outgoing_channel_open(&self, destination: &str) -> Result<bool> {
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

pub async fn open_full_mesh_channels(nodes: &[NodeProcess], amount: &HoprBalance) -> Result<()> {
    let amount = amount.to_string();

    // Serialize all channel opens globally: hoprd waits ~2s for the on-chain
    // confirmation event; concurrent submissions overwhelm blokli's event delivery.
    // open_channel() handles the "timed out" case by retrying after verifying
    // the channel state via the channels list API.
    for src in nodes {
        let Some(src_addr) = src.address.as_deref() else {
            anyhow::bail!("node {} address missing", src.id);
        };
        for dst in nodes {
            let Some(dst_addr) = dst.address.as_deref() else {
                anyhow::bail!("node {} address missing", dst.id);
            };
            if src_addr == dst_addr {
                continue;
            }
            src.api
                .open_channel(dst_addr, &amount)
                .await
                .with_context(|| format!("node {}→{}", src.id, dst.id))?;
            tracing::info!(src = src.id, dst = dst.id, "channel opened");
        }
    }
    Ok(())
}
