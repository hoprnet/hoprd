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
    /// PIX quota as `(polys_per_ssa, shares_per_poly)`.
    ///
    /// Must equal this node's own `network.pix` generator dimensions, and must be
    /// accompanied by [`SessionCapability::UsePix`] — without the capability the Exit
    /// is never told PIX is in play.
    pub pix_ssa_quota: Option<(u16, u16)>,
}

/// `NonAnonymousPix` strategy configuration, handed to hoprd as environment variables.
///
/// The strategy cannot be configured from YAML (its `HoprBalance` fields do not
/// round-trip through `serde_saphyr`), so hoprd reads these from the environment
/// instead — see `hoprd::strategy::build_strategies`. Balances are emitted in wei so
/// the value hoprd parses is bit-for-bit the one configured here.
#[derive(Clone, Debug)]
pub struct PixStrategyEnv {
    /// Charged per byte of the agreed per-SSA quota; one SSA deposit is
    /// `price_per_byte × quota`.
    pub price_per_byte: HoprBalance,
    /// Ceiling on a single SSA deposit. A larger computed deposit is refused outright,
    /// which starves the Session and lets the Exit's kill switch close it.
    pub max_ssa_allocation: HoprBalance,
    /// How long the Exit keeps polling for the deposit.
    ///
    /// This also sets the poll cadence (`/10`), which must stay comfortably below the
    /// Exit's `max_deposit_wait + max_ssa_delivery_time` deadline — otherwise only the
    /// single immediate balance check happens before the kill switch fires.
    pub max_deposit_tracking_time: std::time::Duration,
    /// xDai moved from the Safe to a recovered stealth address so it can pay gas for
    /// its own sweep. Zero disables the sweep's gas funding entirely.
    pub gas_xdai_per_sweep: XDaiBalance,
}

impl Default for PixStrategyEnv {
    /// Mirrors the hoprd-side fallbacks, so passing `Some(Default::default())` is
    /// equivalent to the old `pix: true`.
    fn default() -> Self {
        Self {
            price_per_byte: "1 wxHOPR".parse().expect("valid static amount"),
            max_ssa_allocation: "100 wxHOPR".parse().expect("valid static amount"),
            max_deposit_tracking_time: std::time::Duration::from_secs(3600),
            gas_xdai_per_sweep: "0.01 xdai".parse().expect("valid static amount"),
        }
    }
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
    pub async fn outgoing_channel_status(&self, destination: &str) -> Result<Option<String>> {
        let resp = self.inner.list_channels(None, None).await?;
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
            listen_host: None,
            max_client_sessions: None,
            max_surb_upstream,
            pix_ssa_quota: pix_ssa_quota
                .map(|(polys, shares)| vec![i32::from(polys), i32::from(shares)]),
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
    /// When set, each hoprd process gets `HOPRD_ENABLE_PIX=1` plus the strategy
    /// configuration; when `None`, PIX is disabled.
    pub pix: Option<PixStrategyEnv>,
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
            .env(
                "HOPRD_ENABLE_PIX",
                if config.pix.is_some() { "1" } else { "0" },
            )
            .stdout(Stdio::from(log_file))
            .stderr(Stdio::from(log_err));

        if let Some(pix) = &config.pix {
            // Balances go over as wei so hoprd reparses exactly this value; the
            // decimal `Display` form would round-trip too, but only because it prints
            // all 18 fractional digits.
            cmd.env(
                "HOPRD_PIX_PRICE_PER_BYTE",
                pix.price_per_byte.format_in_wei(),
            )
            .env(
                "HOPRD_PIX_MAX_SSA_ALLOCATION",
                pix.max_ssa_allocation.format_in_wei(),
            )
            .env(
                "HOPRD_PIX_MAX_DEPOSIT_TRACKING_TIME",
                format!("{}s", pix.max_deposit_tracking_time.as_secs()),
            )
            .env(
                "HOPRD_PIX_GAS_XDAI_PER_SWEEP",
                pix.gas_xdai_per_sweep.format_in_wei(),
            );
        }

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
