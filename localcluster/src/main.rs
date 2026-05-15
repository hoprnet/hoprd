//! Orchestrator binary for `hoprd-localcluster`.
//!
//! Lifecycle:
//! 1. Start the Blokli + Anvil chain container via the configured container runtime.
//! 2. Generate node identities and fund Safes on-chain (`identity::generate`).
//! 3. Spawn `hoprd` processes, one per node.
//! 4. Wait for each node to pass `/startedz` then `/readyz`.
//! 5. Wait for the `ChannelLifecycleStrategy` to open the full-mesh channel topology.
//! 6. Block until Ctrl-C, then shut everything down.
//!
//! See `docs/localcluster/README.md` for full setup and usage instructions.

use std::{fs, time::Duration};

use anyhow::{Context, Result};
use clap::Parser;
use hoprd_localcluster::{
    blokli_helper, cli, client_helper,
    identity::{self, GeneratedIdentity},
};
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

    let args = cli::Args::parse();

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

        if args.skip_channels {
            warn!("skipping channel topology wait");
        } else {
            info!("waiting for full-mesh peer visibility");
            client_helper::wait_full_mesh_reachable(&cleanup.nodes, DEFAULT_WAIT_TIMEOUT).await?;
            info!("waiting for channel-lifecycle strategy to open the full mesh");
            client_helper::wait_full_mesh_channels(&cleanup.nodes, DEFAULT_WAIT_TIMEOUT * 4)
                .await?;
        }

        node_summary(&cleanup.nodes, &args, &blokli_url);
        extras_summary(&identities.extras);

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

fn node_summary(nodes: &[client_helper::NodeProcess], args: &cli::Args, blokli_url: &str) {
    println!();
    println!("Chain (Blokli): {blokli_url}");
    println!();

    for node in nodes {
        let addr = node.address.clone().unwrap_or_else(|| "N/A".to_string());
        let api_host = if args.api_host == "0.0.0.0" {
            "127.0.0.1"
        } else {
            &args.api_host
        };
        let api = format!("http://{}:{}", api_host, node.api_port);
        let token = args.api_token.clone().unwrap_or_else(|| "N/A".to_string());
        let mut node_admin = format!("http://localhost:4677/node/info?apiEndpoint={api}");
        if let Some(token) = &args.api_token {
            node_admin.push_str(&format!("&apiToken={token}"));
        }

        let rows = [
            ("Address", addr),
            ("P2P", format!("{}:{}", &args.p2p_host, node.p2p_port)),
            ("API host", api),
            ("API token", token),
            ("Node admin", node_admin),
            ("PID", node.child.id().to_string()),
        ];
        let label_width = rows.iter().map(|(label, _)| label.len()).max().unwrap_or(0);

        println!("Node {}", node.id);
        for (label, value) in rows {
            println!("\t{label:<width$}: {value}", width = label_width);
        }
        println!();
    }
}

fn extras_summary(extras: &[GeneratedIdentity]) {
    if extras.is_empty() {
        return;
    }

    for extra in extras {
        let rows = [
            ("Address", extra.address.clone()),
            ("Safe address", extra.safe_address.clone()),
            ("Module address", extra.module_address.clone()),
            ("Identity file", extra.id_file.display().to_string()),
            ("Password", extra.password.clone()),
        ];
        let label_width = rows.iter().map(|(label, _)| label.len()).max().unwrap_or(0);

        println!("Extra {}", extra.id);
        for (label, value) in rows {
            println!("\t{label:<width$}: {value}", width = label_width);
        }
        println!();
    }
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
        if let Ok(resp) = client.get(&url).send().await {
            if resp.status().is_success() {
                return Ok(());
            }
        }

        if start.elapsed() > timeout {
            anyhow::bail!("timeout waiting for blokli indexer at {url}");
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
