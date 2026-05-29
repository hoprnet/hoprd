//! Orchestrator binary for `hoprd-localcluster`.
//!
//! Lifecycle:
//! 1. Start the Blokli + Anvil chain container via the configured container runtime.
//! 2. Generate node identities and fund Safes on-chain (`identity::generate`).
//! 3. Spawn `hoprd` processes, one per node.
//! 4. Wait for each node to pass `/startedz` then `/readyz`.
//! 5. Manage channels according to `--channel-management`.
//! 6. Block until Ctrl-C, then shut everything down.
//!
//! See `docs/localcluster/README.md` for full setup and usage instructions.

use std::{fs, time::Duration};

use anyhow::{Context, Result};
use clap::Parser;
use hoprd_localcluster::{blokli_helper, cli, client_helper, identity, summary::ClusterSummary};
use tracing::{error, info, warn};

const DEFAULT_WAIT_TIMEOUT: Duration = Duration::from_secs(120);

#[derive(Default)]
struct Cleanup {
    nodes: Vec<client_helper::NodeProcess>,
    chain: Option<blokli_helper::ChainHandle>,
}

impl Cleanup {
    fn shutdown(&mut self) {
        for node in self.nodes.iter_mut() {
            let _ = node.child.kill();
        }
        if let Some(chain) = self.chain.as_mut() {
            chain.stop();
        }
    }
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
        let summary = ClusterSummary::read_file(&status.summary_file)?;
        println!("{}", summary.to_json()?);
        return Ok(());
    }

    let args = cli.run;

    let data_dir = args.data_dir.clone();
    fs::create_dir_all(&data_dir).context("failed to create data directory")?;
    let log_dir = data_dir.join("logs");
    fs::create_dir_all(&log_dir).context("failed to create log directory")?;

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
            ..Default::default()
        };

        info!("waiting for blokli indexer to be ready");
        wait_for_blokli_ready(&blokli_url, DEFAULT_WAIT_TIMEOUT).await?;
        info!("blokli indexer is ready");

        info!("generating identities and configs via hoprd-gen-test library");
        let identities = identity::generate(&config).await?;

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

        info!("waiting for nodes to start");
        futures::future::try_join_all(
            cleanup
                .nodes
                .iter()
                .map(|n| n.api.wait_started(2 * DEFAULT_WAIT_TIMEOUT)),
        )
        .await?;
        info!("waiting for nodes to be ready");
        futures::future::try_join_all(
            cleanup
                .nodes
                .iter()
                .map(|n| n.api.wait_ready(DEFAULT_WAIT_TIMEOUT)),
        )
        .await?;

        info!("fetching node addresses");
        for node in cleanup.nodes.iter_mut() {
            node.address = Some(node.api.addresses().await?);
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

        let summary = ClusterSummary::build(&cleanup.nodes, &args, &blokli_url, &identities.extras);
        print!("{}", summary.render_human());
        // Default to `<data_dir>/summary.json` so external tooling has a stable,
        // discoverable location even when `--summary-file` is not passed.
        let summary_path = args
            .summary_file
            .clone()
            .unwrap_or_else(|| data_dir.join("summary.json"));
        summary
            .write_file(&summary_path)
            .with_context(|| format!("failed to write summary file {}", summary_path.display()))?;
        info!("wrote cluster summary to {}", summary_path.display());

        info!("localcluster running; press Ctrl+C to stop");
        tokio::select! {
            res = tokio::signal::ctrl_c() => {
                res.context("failed to await Ctrl+C")?;
            }
            _ = wait_sigterm() => {}
        }
        info!("shutdown requested");

        Ok(())
    }
    .await;

    cleanup.shutdown();

    if let Err(err) = result {
        error!(error = %err, "localcluster failed");
        return Err(err);
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
