use std::path::PathBuf;

use clap::{Parser, ValueEnum};

use crate::identity::{
    DEFAULT_CONFIG_HOME, DEFAULT_IDENTITY_PASSWORD, DEFAULT_NUM_EXTRA_IDENTITIES,
    DEFAULT_NUM_NODES, MAX_EXTRA_IDENTITIES, MAX_NUM_NODES,
};

fn parse_size(s: &str) -> Result<usize, String> {
    let n: usize = s
        .parse()
        .map_err(|_| format!("'{s}' is not a valid number"))?;
    if n < 1 || n > MAX_NUM_NODES {
        return Err(format!(
            "size must be between 1 and {MAX_NUM_NODES}, got {n}"
        ));
    }
    Ok(n)
}

#[derive(Parser, Debug)]
#[command(
    name = "hoprd-localcluster",
    about = "Run a local HOPR cluster using external processes.\n\nLifecycle: start chain container → generate identities & fund Safes → spawn hoprd nodes → open channels → wait for Ctrl-C.\n\nSee docs/localcluster/README.md for full setup instructions."
)]
pub struct Args {
    /// Number of nodes to start (1–5)
    #[arg(long, default_value_t = DEFAULT_NUM_NODES, value_parser = parse_size)]
    pub size: usize,

    /// Channel management mode: `api`, `strategy`, `both`, or `none`
    #[arg(long, value_enum, default_value_t = ChannelManagement::Api)]
    pub channel_management: ChannelManagement,

    /// REST API host to bind (use "auto" to bind 0.0.0.0 and advertise the container IP)
    #[arg(long, default_value = "localhost")]
    pub api_host: String,

    /// REST API base port (node index is added)
    #[arg(long, default_value_t = 3000)]
    pub api_port_base: u16,

    /// P2P host to bind (use "auto" to detect the container interface IP)
    #[arg(long, default_value = "localhost")]
    pub p2p_host: String,

    /// P2P base port (node index is added)
    #[arg(long, default_value_t = 9000)]
    pub p2p_port_base: u16,

    /// Base directory for generated configs, identities, DBs, and logs
    #[arg(long, default_value = DEFAULT_CONFIG_HOME)]
    pub data_dir: PathBuf,

    /// Container image containing both Anvil and Blokli (required unless --chain-url is set)
    #[arg(long, env = "HOPRD_CHAIN_IMAGE", required_unless_present = "chain_url")]
    pub chain_image: Option<String>,

    /// Base URL for Blokli (e.g. http://chain:8080). If set, localcluster will not start the chain container.
    #[arg(long, env = "HOPRD_CHAIN_URL")]
    pub chain_url: Option<String>,

    /// Container runtime CLI used to start the chain container.
    /// Must support `run --rm --name --platform -p` and `rm -f`.
    /// `container` (Apple native) additionally supports `ls` for direct IP
    /// detection, which bypasses macOS NAT for long-lived SSE connections.
    /// Common values: `docker` (default), `container` (Apple native), `podman`.
    #[arg(long, env = "HOPRD_CONTAINER_RUNTIME", default_value = "docker")]
    pub container_runtime: String,

    /// Path to the hoprd binary
    #[arg(long, default_value = "hoprd")]
    pub hoprd_bin: PathBuf,

    /// Password used to encrypt identities
    #[arg(long, default_value = DEFAULT_IDENTITY_PASSWORD)]
    pub identity_password: String,

    /// API token for hoprd REST API (enables authentication)
    #[arg(long)]
    pub api_token: Option<String>,

    /// Per-channel funding amount used for REST API channel opening
    #[arg(long, default_value = "1 wxHOPR", value_parser = parse_funding_amount)]
    pub funding_amount: String,

    /// Number of pre-funded extra identities to create alongside the cluster (0–5).
    /// Each gets its own Safe + Module, is written to `--data-dir` as an encrypted
    /// keystore (`extra_id_{i}.id`, password "local-cluster"), and is NOT run as a
    /// hoprd node. Useful for external tooling that needs a funded HOPR identity.
    #[arg(long, default_value_t = DEFAULT_NUM_EXTRA_IDENTITIES, value_parser = parse_extras)]
    pub extra_identities: usize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum ChannelManagement {
    Api,
    Strategy,
    Both,
    None,
}

fn parse_extras(s: &str) -> Result<usize, String> {
    let n: usize = s
        .parse()
        .map_err(|_| format!("'{s}' is not a valid number"))?;
    if n > MAX_EXTRA_IDENTITIES {
        return Err(format!(
            "extra-identities must be between 0 and {MAX_EXTRA_IDENTITIES}, got {n}"
        ));
    }
    Ok(n)
}

fn parse_funding_amount(s: &str) -> Result<String, String> {
    let value = s.trim();
    if value.is_empty() {
        return Err("funding-amount must not be empty".to_string());
    }
    Ok(value.to_string())
}
