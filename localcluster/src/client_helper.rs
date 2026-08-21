use std::{
    path::Path,
    process::{Child, Command, Stdio},
};

use anyhow::{Context, Result};
use hopr_lib::api::types::primitive::prelude::{HoprBalance, XDaiBalance};
use hoprd_api_client;
use hoprd_api_client::types::{
    IpProtocol, OpenChannelBodyRequest, RoutingOptions, SessionCapability, SessionClientRequest,
    SessionTargetSpec,
};
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use tracing::debug;

/// Parameters for [`HoprdApiClient::open_session`].
///
/// Grouped into a struct rather than passed positionally: at eight parameters the
/// call site stops being readable, and several are `Option<String>` and so
/// indistinguishable to the type checker if transposed.
pub struct OpenSessionRequest<'a> {
    /// `"tcp"` or `"udp"`.
    pub protocol: &'a str,
    /// On-chain address of the Exit node.
    pub destination: &'a str,
    /// `ip:port` the Exit forwards the plaintext to.
    pub target: &'a str,
    /// Number of intermediate relays, applied to both the forward and the return path.
    ///
    /// PIX Sessions require at least one: the share encryption key is derived from the
    /// first relayer's acknowledgement, so a zero-hop return path has nothing to derive
    /// it from and the Session is rejected.
    pub hops: u64,
    /// When `None`, no capabilities are requested and the server applies its per-protocol
    /// default (UDP gets `Segmentation` only).
    pub capabilities: Option<Vec<SessionCapability>>,
    /// SURB balancer: how much response data the Exit may deliver before needing more
    /// SURBs, e.g. `"10 MB"`. `None` leaves the protocol default.
    pub response_buffer: Option<String>,
    /// SURB balancer: ceiling on artificial SURB generation, e.g. `"50 Mb/s"`.
    /// `None` leaves the protocol default.
    pub max_surb_upstream: Option<String>,
    /// PIX parameters as `(polys_per_ssa, shares_per_poly, surplus_shares)`.
    ///
    /// Must equal this node's own `network.pix` generator dimensions — all three of them,
    /// since the surplus is priced into the per-SSA quota — and must be accompanied by
    /// [`SessionCapability::UsePix`]; without the capability the Exit is never told PIX
    /// is in play.
    pub pix_ssa_quota: Option<(u16, u8, u8)>,
}

/// Balances reported by `GET /account/balances`, split by holder.
#[derive(Clone, Copy, Debug)]
pub struct NodeBalances {
    /// wxHOPR held by the node's own account. PIX deposits are paid from here.
    pub node_hopr: HoprBalance,
    /// xDai held by the node's own account; pays gas for everything the node signs.
    pub node_native: XDaiBalance,
    /// wxHOPR held by the Safe. Channel stakes and swept PIX deposits land here.
    pub safe_hopr: HoprBalance,
    /// xDai held by the Safe.
    pub safe_native: XDaiBalance,
}

/// A parsed Prometheus text-format scrape of one node's `/metrics`.
///
/// Note that hoprd deliberately strips every `hopr_session_*` series from this endpoint
/// (`rest-api::root::collect_hopr_metrics`) because they are labelled by session id and
/// so unbounded in cardinality. Per-session counters are exported over OTLP only; what
/// remains here is node-wide, including `hopr_packets_count` and the
/// `hopr_strategy_pix_*` lifecycle counters.
#[derive(Clone, Debug, Default)]
pub struct MetricsSnapshot {
    /// `(name, label block including braces or empty, value)` per sample line.
    samples: Vec<(String, String, f64)>,
}

impl MetricsSnapshot {
    /// Parse the Prometheus text exposition format, skipping `# HELP` / `# TYPE`.
    ///
    /// Label values containing whitespace would split wrongly here; none of the series
    /// this is used for have any.
    pub fn parse(body: &str) -> Self {
        let samples = body
            .lines()
            .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
            .filter_map(|line| {
                let mut parts = line.split_whitespace();
                let series = parts.next()?;
                // Trailing token after the value is an optional timestamp; ignore it.
                let value = parts.next()?.parse::<f64>().ok()?;
                let (name, labels) = match series.split_once('{') {
                    Some((name, rest)) => (name, format!("{{{rest}")),
                    None => (series, String::new()),
                };
                Some((name.to_string(), labels, value))
            })
            .collect();
        Self { samples }
    }

    /// Sum of `name` across every label set, or 0.0 when the series is absent.
    pub fn sum(&self, name: &str) -> f64 {
        self.sum_where(name, "")
    }

    /// Sum of `name` restricted to label sets carrying `label_filter` as a whole
    /// `key="value"` pair, e.g. `sum_where("hopr_packets_count", r#"type="sent""#)`.
    /// An empty filter matches every label set.
    ///
    /// Metric names are compared with any trailing `_total` segments removed on both
    /// sides: OpenTelemetry's Prometheus exporter appends `_total` to counters, and
    /// whether a name that already ends in `_total` gets a second one is exporter- and
    /// version-dependent.
    pub fn sum_where(&self, name: &str, label_filter: &str) -> f64 {
        let wanted = strip_total_suffixes(name);
        self.samples
            .iter()
            .filter(|(sample, labels, _)| {
                strip_total_suffixes(sample) == wanted && has_label(labels, label_filter)
            })
            .map(|(_, _, value)| value)
            .sum()
    }
}

fn strip_total_suffixes(name: &str) -> &str {
    let mut name = name;
    while let Some(stripped) = name.strip_suffix("_total") {
        name = stripped;
    }
    name
}

/// Whether `labels` — a label block including its braces, or empty for an unlabelled series —
/// carries `filter` as one whole `key="value"` pair.
///
/// Deliberately not `labels.contains(filter)`. A substring test is unanchored on the left, so
/// `type="sent"` would also match a `packet_type="sent"` label, and the result is a silently
/// doubled reading rather than an error. Both series read through this today carry exactly one
/// label, so nothing collides yet — this is what keeps the next label from corrupting a number
/// instead of failing a test.
fn has_label(labels: &str, filter: &str) -> bool {
    filter.is_empty() || split_labels(labels).any(|pair| pair == filter)
}

/// Split a label block into its `key="value"` pairs, honouring commas and escapes inside
/// quoted values.
fn split_labels(labels: &str) -> impl Iterator<Item = &str> {
    let inner = labels.trim().trim_start_matches('{').trim_end_matches('}');
    let mut pairs = Vec::new();
    let (mut in_quotes, mut escaped, mut start) = (false, false, 0);
    for (i, c) in inner.char_indices() {
        match c {
            _ if escaped => escaped = false,
            '\\' if in_quotes => escaped = true,
            '"' => in_quotes = !in_quotes,
            ',' if !in_quotes => {
                pairs.push(inner[start..i].trim());
                start = i + 1;
            }
            _ => {}
        }
    }
    pairs.push(inner[start..].trim());
    pairs.into_iter().filter(|pair| !pair.is_empty())
}

#[derive(Debug, Clone)]
pub struct HoprdApiClient {
    inner: hoprd_api_client::Client,
    /// Kept alongside `inner` for the `/metrics` scrape: that endpoint returns a
    /// Prometheus text body, and the generated client hands back an opaque byte stream.
    http: reqwest::Client,
    base_url: String,
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
            inner: hoprd_api_client::Client::new_with_client(
                base_url.as_ref(),
                http_client.clone(),
            ),
            http: http_client,
            base_url,
        })
    }

    /// Scrape this node's Prometheus `/metrics` endpoint.
    ///
    /// Returns an empty snapshot rather than an error for exactly one answer: a node compiled
    /// without the `telemetry` feature responds `422 BUILT WITHOUT METRICS SUPPORT`, and that
    /// should degrade a live progress report rather than fail it.
    ///
    /// Every other non-2xx is a real scrape failure and is reported as one. A wrong API token
    /// (401), a wrong base URL (404), a node in trouble (500) and a node still starting (503)
    /// all used to arrive here as an empty snapshot, which reads downstream as "the node did
    /// nothing" — and an assertion written against a counter that must stay at zero then passes
    /// for having observed nothing at all.
    pub async fn metrics(&self) -> Result<MetricsSnapshot> {
        let url = format!("{}/metrics", self.base_url);
        let resp = self
            .http
            .get(&url)
            .send()
            .await
            .with_context(|| format!("scraping {url}"))?;
        let status = resp.status();
        if status == reqwest::StatusCode::UNPROCESSABLE_ENTITY {
            return Ok(MetricsSnapshot::default());
        }
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("scraping {url}: HTTP {status} - {body}");
        }
        Ok(MetricsSnapshot::parse(
            &resp.text().await.context("reading metrics body")?,
        ))
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

    pub async fn close_channel(&self, destination: &str) -> Result<()> {
        match self
            .inner
            .close_channel(
                destination,
                Some(hoprd_api_client::types::ChannelDirection::Outgoing),
            )
            .await
        {
            Ok(_) => Ok(()),
            Err(hoprd_api_client::Error::UnexpectedResponse(resp)) => {
                let status = resp.status();
                let body = resp.text().await.unwrap_or_default();
                anyhow::bail!("close_channel to {destination}: HTTP {status} - {body}")
            }
            Err(e) => anyhow::bail!("close_channel to {destination}: {e}"),
        }
    }

    /// Return the channel status string for the outgoing channel to `destination`,
    /// or `None` if no such channel exists.
    ///
    /// `includingClosed` is on: without it the server filters `Closed` out of the listing
    /// entirely, so a channel that finished closing is indistinguishable from one that was never
    /// opened. A closure poll would then read `None` and report "no such channel" for the exact
    /// outcome it was waiting for.
    pub async fn outgoing_channel_status(&self, destination: &str) -> Result<Option<String>> {
        let resp = self.inner.list_channels(None, Some(true)).await?;
        let dest_lower = destination.to_lowercase();
        Ok(resp
            .into_inner()
            .outgoing
            .into_iter()
            .find(|ch| ch.peer_address.to_lowercase() == dest_lower)
            .map(|ch| ch.status))
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

    /// Open a session described by `req`. Returns the `(ip, port)` of the listener
    /// bound on this (entry) node.
    pub async fn open_session(&self, req: OpenSessionRequest<'_>) -> Result<(String, u16)> {
        let OpenSessionRequest {
            protocol,
            destination,
            target,
            hops,
            capabilities,
            response_buffer,
            max_surb_upstream,
            pix_ssa_quota,
        } = req;

        let body = SessionClientRequest {
            destination: destination.to_string(),
            forward_path: RoutingOptions::Hops(hops),
            return_path: RoutingOptions::Hops(hops),
            target: SessionTargetSpec::Plain(target.to_string()),
            capabilities,
            flow_control: None,
            listen_host: None,
            max_client_sessions: None,
            max_surb_upstream,
            // The generated client takes a fixed `[u64; 3]`, so only the widths are converted
            // here; the arity is the array type's own business. The named triple stays on
            // `OpenSessionRequest` — this is the one place it becomes positional.
            pix_ssa_quota: pix_ssa_quota.map(|(polys, shares, surplus)| {
                [u64::from(polys), u64::from(shares), u64::from(surplus)]
            }),
            response_buffer,
            session_pool: None,
        };
        let resp = self
            .inner
            .create_client(protocol, &body)
            .await
            .map_err(|e| anyhow::anyhow!("create {protocol} session to {destination}: {e}"))?
            .into_inner();
        let port = u16::try_from(resp.port)
            .map_err(|_| anyhow::anyhow!("session port {} out of u16 range", resp.port))?;
        Ok((resp.ip, port))
    }

    /// wxHOPR and xDai held by this node's own account and by its Safe.
    ///
    /// Both matter for PIX: `SafePayloadGenerator::transfer` signs a direct token
    /// transfer with the node key, so outgoing deposits leave the *node* account, while
    /// `withdraw_from_signer` sweeps recovered deposits into the *Safe*.
    pub async fn balances(&self) -> Result<NodeBalances> {
        let b = self
            .inner
            .balances()
            .await
            .map_err(|e| anyhow::anyhow!("balances: {e}"))?
            .into_inner();

        let parse_hopr = |raw: &str| -> Result<HoprBalance> {
            raw.parse()
                .with_context(|| format!("unparseable wxHOPR balance {raw}"))
        };
        let parse_native = |raw: &str| -> Result<XDaiBalance> {
            raw.parse()
                .with_context(|| format!("unparseable xDai balance {raw}"))
        };

        Ok(NodeBalances {
            node_hopr: parse_hopr(&b.hopr)?,
            node_native: parse_native(&b.native)?,
            safe_hopr: parse_hopr(&b.safe_hopr)?,
            safe_native: parse_native(&b.safe_native)?,
        })
    }

    /// Close a UDP session listener identified by its listening IP and port.
    pub async fn close_client(&self, ip: &str, port: u16) -> Result<()> {
        self.inner
            .close_client(IpProtocol::Udp, ip, port as i32)
            .await?;
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
    /// Development-only Curvy operator key. When present, nodes are started one at a
    /// time so their independent SDK clients cannot race on the shared EVM nonce.
    pub curvy_operator_private_key: Option<&'a str>,
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
            .stdout(Stdio::from(log_file))
            .stderr(Stdio::from(log_err));

        if let Some(operator_private_key) = config.curvy_operator_private_key {
            cmd.env("HOPRD_CURVY_OPERATOR_PRIVATE_KEY", operator_private_key);
            // Proof timings are also persisted independently of tracing. Each node owns a
            // separate JSONL sink, so blocking prover threads cannot interleave records and the
            // acceptance report remains complete even when hoprd is terminated immediately
            // after the target settlement.
            cmd.env(
                "CURVY_PROOF_TIMINGS_PATH",
                config
                    .log_dir
                    .join(format!("curvy_proof_timings_{id}.jsonl")),
            );
            // The full-system Curvy scenario consumes these events to produce its proof-phase
            // report. Preserve any caller-supplied filter, but make the instrumentation target
            // explicit so a filter such as `hopr=info` cannot silently produce empty metrics.
            let rust_log = std::env::var("RUST_LOG")
                .ok()
                .filter(|filter| !filter.trim().is_empty())
                .unwrap_or_else(|| "info".to_string());
            cmd.env("RUST_LOG", format!("{rust_log},curvy_witnesscalc=info"));
        }

        if let Some(token) = &config.api_token {
            cmd.arg("--apiToken").arg(token);
        }

        // Do not debug-print `cmd`: its environment may contain the Curvy operator key.
        debug!(node_id = id, config = %cfg_file.display(), "starting hoprd node");
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

        if config.curvy_operator_private_key.is_some() {
            nodes
                .last()
                .expect("node was just appended")
                .api
                .wait_started(std::time::Duration::from_secs(120))
                .await
                .with_context(|| format!("waiting for Curvy node {id} startup"))?;
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Shaped like a real scrape: `_total` on the counters, one label on the series that
    /// carry one, and a decoy whose label *name* ends in the one being filtered for.
    const SCRAPE: &str = r#"
# HELP hopr_packets_count Number of processed packets
# TYPE hopr_packets_count counter
hopr_packets_count_total{type="sent"} 12
hopr_packets_count_total{type="received"} 7
hopr_packets_count_total{packet_type="sent"} 1000
hopr_strategy_pix_sweeps_total 3
hopr_strategy_pix_deposit_tracking_total{outcome="confirmed"} 5 1699999999
hopr_strategy_pix_deposit_tracking_total{outcome="timeout"} 2
"#;

    #[test]
    fn sum_where_matches_a_whole_label_pair_and_not_a_suffix_of_one() {
        let m = MetricsSnapshot::parse(SCRAPE);
        // 1000, not 1012: `packet_type="sent"` is a different label.
        assert_eq!(m.sum_where("hopr_packets_count", r#"type="sent""#), 12.0);
        assert_eq!(
            m.sum_where("hopr_packets_count", r#"packet_type="sent""#),
            1000.0
        );
    }

    #[test]
    fn sum_adds_every_label_set_and_tolerates_a_trailing_timestamp() {
        let m = MetricsSnapshot::parse(SCRAPE);
        assert_eq!(m.sum("hopr_packets_count"), 1019.0);
        assert_eq!(m.sum("hopr_strategy_pix_deposit_tracking_total"), 7.0);
        assert_eq!(
            m.sum_where(
                "hopr_strategy_pix_deposit_tracking_total",
                r#"outcome="confirmed""#
            ),
            5.0
        );
    }

    #[test]
    fn an_unlabelled_series_is_summed_but_never_matches_a_filter() {
        let m = MetricsSnapshot::parse(SCRAPE);
        assert_eq!(m.sum("hopr_strategy_pix_sweeps"), 3.0);
        assert_eq!(
            m.sum_where("hopr_strategy_pix_sweeps", r#"outcome="confirmed""#),
            0.0
        );
    }

    #[test]
    fn an_absent_series_reads_zero_rather_than_failing() {
        let m = MetricsSnapshot::parse(SCRAPE);
        assert_eq!(m.sum("hopr_nothing_like_this"), 0.0);
    }

    #[test]
    fn label_splitting_survives_commas_and_escaped_quotes_inside_values() {
        let m = MetricsSnapshot::parse(r#"weird{a="x,y",b="say\"hi\"",c="z"} 4"#);
        for filter in [r#"a="x,y""#, r#"b="say\"hi\"""#, r#"c="z""#] {
            assert_eq!(m.sum_where("weird", filter), 4.0, "filter {filter}");
        }
        // The comma inside `a` is not a separator, so its tail is not a pair of its own.
        assert_eq!(m.sum_where("weird", r#"y""#), 0.0);
    }

    /// Exercised directly rather than through [`MetricsSnapshot::parse`], which splits the
    /// line on whitespace and so cannot deliver a label value containing a space in the
    /// first place. Nothing hoprd exports has one; the splitter handles it regardless, and
    /// this is the only way to say so.
    #[test]
    fn label_splitting_survives_whitespace_inside_values() {
        let labels = r#"{a="x y",b="z"}"#;
        assert_eq!(
            split_labels(labels).collect::<Vec<_>>(),
            vec![r#"a="x y""#, r#"b="z""#]
        );
        assert!(has_label(labels, r#"a="x y""#));
        assert!(!has_label(labels, r#"a="x""#));
    }
}
