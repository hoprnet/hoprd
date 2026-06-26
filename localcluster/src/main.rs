//! Orchestrator binary for `hoprd-localcluster`.
//!
//! Lifecycle:
//! 1. Acquire the single-instance lock on the data directory.
//! 2. Start the Blokli + Anvil chain container via the configured container runtime.
//! 3. Generate node identities and fund Safes on-chain (`identity::generate`).
//! 4. Spawn `hoprd` processes, one per node.
//! 5. Wait for each node to pass `/startedz` then `/readyz`.
//! 6. Manage channels according to `--channel-management`.
//! 7. Block until Ctrl-C, then shut everything down.
//!
//! Throughout, the live [`ClusterSummary`] is updated and served on a unix control
//! socket so external tooling can poll structured status (see `summary` and `control`).
//!
//! See `docs/localcluster/README.md` for full setup and usage instructions.

use std::{fs, net::SocketAddr, sync::Arc, time::Duration};

use anyhow::{Context, Result};
use clap::Parser;
use futures::stream::{FuturesUnordered, StreamExt};
use hoprd_localcluster::{
    blokli_helper, cli, client_helper, control,
    control::{ControlServer, SharedSummary},
    identity,
    lock::ClusterLock,
    relay::{self, RelayConfig, RelayHandle},
    summary::{ClusterState, ClusterSummary, NodeState},
};
use tokio::sync::Mutex;
use tracing::{error, info, warn};

const DEFAULT_WAIT_TIMEOUT: Duration = Duration::from_secs(120);

#[derive(Default)]
struct Cleanup {
    nodes: Vec<client_helper::NodeProcess>,
    relays: Vec<RelayHandle>,
    chain: Option<blokli_helper::ChainHandle>,
}

impl Cleanup {
    fn shutdown(&mut self) {
        for relay in self.relays.iter() {
            relay.abort();
        }
        for node in self.nodes.iter_mut() {
            let _ = node.child.kill();
        }
        if let Some(chain) = self.chain.as_mut() {
            chain.stop();
        }
    }
}

/// Resolve a `host:port` (e.g. `localhost`, `127.0.0.1`) to a single socket address.
///
/// Prefers IPv4: hoprd binds its P2P host as IPv4 (it resolves domains like `localhost`
/// to `127.0.0.1` and rejects IPv6 literals), so the relay must stay on the same address
/// family — otherwise it would forward to `[::1]` while the node listens on `127.0.0.1`.
async fn resolve_socket(host: &str, port: u16) -> Result<SocketAddr> {
    let mut addrs: Vec<SocketAddr> = tokio::net::lookup_host((host, port))
        .await
        .with_context(|| format!("failed to resolve {host}:{port}"))?
        .collect();
    addrs.sort_by_key(SocketAddr::is_ipv6);
    addrs
        .into_iter()
        .next()
        .with_context(|| format!("no address resolved for {host}:{port}"))
}

#[tokio::main]
async fn main() -> Result<()> {
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();

    let cli = cli::Cli::parse();

    if let Some(cli::Command::Status(status)) = &cli.command {
        let json = control::query(&status.socket_path()).await?;
        println!("{json}");
        return Ok(());
    }

    let args = cli.run;

    // Resolve the relay delay model once; `args.latency` keeps the source + relay base port.
    let latency_model = args
        .latency
        .as_ref()
        .map(|l| l.resolve())
        .transpose()
        .map_err(|e| anyhow::anyhow!("invalid latency configuration: {e}"))?;

    let data_dir = args.data_dir.clone();
    fs::create_dir_all(&data_dir).context("failed to create data directory")?;
    let log_dir = data_dir.join("logs");
    fs::create_dir_all(&log_dir).context("failed to create log directory")?;

    let control_base = args.control_base();
    if let Some(parent) = control_base.parent() {
        fs::create_dir_all(parent).context("failed to create control directory")?;
    }

    // Refuse to run a second instance against the same control base. Held for the
    // whole process lifetime; released automatically on exit (including a crash).
    let _lock = ClusterLock::acquire(&control_base)?;

    // Live status, updated through the lifecycle and served on the control socket.
    let summary: SharedSummary = Arc::new(Mutex::new(ClusterSummary::initial(
        &args,
        latency_model.as_ref(),
    )));
    let _control = ControlServer::start(control::socket_path(&control_base), summary.clone())?;

    let explicit_chain_url = args.chain_url.clone();

    let mut cleanup = Cleanup::default();

    let result: Result<()> = async {
        // Determine the effective blokli URL: explicit --chain-url or container-detected IP.
        let blokli_url = if let Some(ref url) = explicit_chain_url {
            let url = url.trim_end_matches('/').to_string();
            info!("using external chain services at {url}");
            url
        } else {
            let chain_image = args
                .chain_image
                .as_deref()
                .context("missing chain image (set --chain-image or HOPRD_CHAIN_IMAGE)")?;
            info!("starting chain services (anvil + blokli)");
            let handle =
                blokli_helper::ChainHandle::start(&args.container_runtime, chain_image, &log_dir)?;
            let url = handle.chain_url();
            info!("chain container URL: {url}");
            cleanup.chain = Some(handle);
            url
        };
        summary.lock().await.blokli_url = Some(blokli_url.clone());

        let config = identity::GenerationConfig {
            blokli_url: blokli_url.clone(),
            num_nodes: args.size,
            config_home: data_dir.to_path_buf(),
            identity_password: args.identity_password.clone(),
            random_identities: true,
            num_extras: args.extra_identities,
            p2p_host: args.p2p_host.clone(),
            p2p_port_base: args.p2p_port_base,
            enable_channel_strategy: matches!(
                args.channel_management,
                cli::ChannelManagement::Strategy | cli::ChannelManagement::Both
            ),
            latency: args.latency.clone(),
            ..Default::default()
        };

        info!("waiting for blokli indexer to be ready");
        wait_for_blokli_ready(&blokli_url, DEFAULT_WAIT_TIMEOUT).await?;
        info!("blokli indexer is ready");

        info!("generating identities and configs via hoprd-gen-test library");
        let identities = identity::generate(&config).await?;
        summary.lock().await.set_extras(&identities.extras);

        if let (Some(latency_cfg), Some(latency)) = (&latency_model, &args.latency) {
            // Relays must be listening before nodes start dialing the announced relay ports.
            let latency_cfg = Arc::new(latency_cfg.clone());
            let port_base = latency.port_base;
            info!("latency relays enabled (relay base port {port_base})");
            // `auto`/`0.0.0.0` aren't resolvable hostnames; map to the same loopback the
            // rest of localcluster advertises so lookup_host never sees the sentinel.
            let relay_host = hoprd_localcluster::summary::advertised_host(&args.p2p_host);
            for id in 0..args.size {
                let listen_port = port_base.checked_add(id as u16).ok_or_else(|| {
                    anyhow::anyhow!("relay listen port overflow: base + node id {id} exceeds u16")
                })?;
                let target_port = args.p2p_port_base.checked_add(id as u16).ok_or_else(|| {
                    anyhow::anyhow!("relay target port overflow: base + node id {id} exceeds u16")
                })?;
                let listen = resolve_socket(relay_host, listen_port)
                    .await
                    .context("resolving relay listen address")?;
                let target = resolve_socket(relay_host, target_port)
                    .await
                    .context("resolving relay target address")?;
                let handle = relay::spawn_relay(RelayConfig {
                    node_id: id,
                    listen,
                    target,
                    p2p_port_base: args.p2p_port_base,
                    latency: latency_cfg.clone(),
                })
                .await
                .with_context(|| format!("failed to start latency relay for node {id}"))?;
                info!(
                    "node {id} latency relay {} -> {}",
                    handle.listen_addr(),
                    target
                );
                cleanup.relays.push(handle);
            }
        }

        info!("starting hoprd nodes");
        let start_cfg = client_helper::NodeStartConfig {
            num_nodes: args.size,
            hoprd_bin: &args.hoprd_bin,
            data_dir: &data_dir,
            log_dir: &log_dir,
            api_host: &args.api_host,
            api_port_base: args.api_port_base,
            p2p_host: &args.p2p_host,
            p2p_port_base: args.p2p_port_base,
            identity_password: &args.identity_password,
            api_token: args.api_token.clone(),
        };
        cleanup.nodes = client_helper::start_nodes(&start_cfg).await?;
        {
            let mut s = summary.lock().await;
            s.mark_spawned(&cleanup.nodes);
            s.state = ClusterState::Starting;
        }

        info!("waiting for nodes to start");
        wait_nodes(
            &cleanup.nodes,
            &summary,
            NodeState::Started,
            |api| async move { api.wait_started(2 * DEFAULT_WAIT_TIMEOUT).await },
        )
        .await?;

        info!("waiting for nodes to be ready");
        wait_nodes(
            &cleanup.nodes,
            &summary,
            NodeState::Ready,
            |api| async move { api.wait_ready(DEFAULT_WAIT_TIMEOUT).await },
        )
        .await?;

        info!("fetching node addresses");
        for node in cleanup.nodes.iter_mut() {
            let address = node.api.addresses().await?;
            node.address = Some(address.clone());
            summary.lock().await.set_node_address(node.id, address);
        }

        match args.channel_management {
            cli::ChannelManagement::None => {
                warn!("channel management is disabled");
            }
            cli::ChannelManagement::Api => {
                info!("opening full-mesh channels through REST API");
                client_helper::open_full_mesh_channels(
                    &cleanup.nodes,
                    &args.funding_amount,
                    DEFAULT_WAIT_TIMEOUT * 4,
                )
                .await?;
                info!("waiting for full-mesh channels to be open");
                client_helper::wait_full_mesh_channels(&cleanup.nodes, DEFAULT_WAIT_TIMEOUT * 4)
                    .await?;
            }
            cli::ChannelManagement::Strategy => {
                info!("waiting for full-mesh peer visibility");
                client_helper::wait_full_mesh_reachable(&cleanup.nodes, DEFAULT_WAIT_TIMEOUT)
                    .await?;
                info!("waiting for strategy-managed full-mesh channels");
                client_helper::wait_full_mesh_channels(&cleanup.nodes, DEFAULT_WAIT_TIMEOUT * 4)
                    .await?;
            }
            cli::ChannelManagement::Both => {
                info!("opening full-mesh channels through REST API");
                client_helper::open_full_mesh_channels(
                    &cleanup.nodes,
                    &args.funding_amount,
                    DEFAULT_WAIT_TIMEOUT * 4,
                )
                .await?;
                info!("waiting for full-mesh channels to be open");
                client_helper::wait_full_mesh_channels(&cleanup.nodes, DEFAULT_WAIT_TIMEOUT * 4)
                    .await?;
            }
        }

        {
            let mut s = summary.lock().await;
            if !matches!(args.channel_management, cli::ChannelManagement::None) {
                s.set_all_node_states(NodeState::ChannelsOpen);
            }
            s.state = ClusterState::Running;
            print!("{}", s.render_human());
        }

        info!(
            "localcluster running; query status via `hoprd-localcluster status --control-base {}`",
            control_base.display()
        );
        info!("press Ctrl+C to stop");
        tokio::select! {
            res = tokio::signal::ctrl_c() => {
                res.context("failed to await Ctrl+C")?;
            }
            _ = wait_sigterm() => {}
        }
        info!("shutdown requested");
        summary.lock().await.state = ClusterState::ShuttingDown;

        Ok(())
    }
    .await;

    if let Err(err) = &result {
        let mut s = summary.lock().await;
        s.state = ClusterState::Failed;
        s.error = Some(err.to_string());
    }

    cleanup.shutdown();

    if let Err(err) = result {
        error!(error = %err, "localcluster failed");
        return Err(err);
    }

    Ok(())
}

/// Await a lifecycle check on every node, advancing each node's state in the shared
/// summary the moment it passes — so a poller observes peers coming up one by one.
async fn wait_nodes<F, Fut>(
    nodes: &[client_helper::NodeProcess],
    summary: &SharedSummary,
    reached: NodeState,
    check: F,
) -> Result<()>
where
    F: Fn(client_helper::HoprdApiClient) -> Fut,
    Fut: std::future::Future<Output = Result<()>>,
{
    let mut pending: FuturesUnordered<_> = nodes
        .iter()
        .map(|node| {
            let id = node.id;
            let fut = check(node.api.clone());
            async move { fut.await.map(|()| id) }
        })
        .collect();

    while let Some(result) = pending.next().await {
        let id = result?;
        summary.lock().await.set_node_state(id, reached);
    }
    Ok(())
}

async fn wait_sigterm() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};
        if let Ok(mut stream) = signal(SignalKind::terminate()) {
            stream.recv().await;
        }
    }
    #[cfg(not(unix))]
    {
        std::future::pending::<()>().await;
    }
}

async fn wait_for_blokli_ready(blokli_url: &str, timeout: Duration) -> Result<()> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .context("failed to build http client")?;
    let url = format!("{blokli_url}/readyz");
    let start = std::time::Instant::now();

    loop {
        if let Ok(resp) = client.get(&url).send().await
            && resp.status().is_success()
        {
            return Ok(());
        }

        if start.elapsed() > timeout {
            anyhow::bail!("timeout waiting for blokli indexer at {url}");
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
