use std::{path::PathBuf, str::FromStr};

use clap::Parser;
use hopr_types::primitive::prelude::HoprBalance;

use crate::identity::{DEFAULT_CONFIG_HOME, DEFAULT_IDENTITY_PASSWORD, DEFAULT_NUM_NODES};

#[derive(Parser, Debug)]
#[command(
    name = "hoprd-localcluster",
    about = "Run a local HOPR cluster using external processes.\n\nLifecycle: start chain container → generate identities & fund Safes → spawn hoprd nodes → open channels → wait for Ctrl-C.\n\nSee docs/localcluster/README.md for full setup instructions."
)]
pub struct Args {
    /// Number of nodes to start
    #[arg(long, default_value_t = DEFAULT_NUM_NODES)]
    pub size: usize,

    /// Channel funding amount in base units (per channel)
    #[arg(long, default_value = "1 wxHOPR", value_parser = HoprBalance::from_str)]
    pub funding_amount: HoprBalance,

    /// Skip channel creation
    #[arg(long, default_value_t = false)]
    pub skip_channels: bool,

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
}
