use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand, ValueEnum};

use crate::identity::{
    DEFAULT_CONFIG_HOME, DEFAULT_IDENTITY_PASSWORD, DEFAULT_LATENCY_PORT_BASE,
    DEFAULT_NUM_EXTRA_IDENTITIES, DEFAULT_NUM_NODES, MAX_EXTRA_IDENTITIES, MAX_NUM_NODES,
};

fn parse_size(s: &str) -> Result<usize, String> {
    let n: usize = s
        .parse()
        .map_err(|_| format!("'{s}' is not a valid number"))?;
    if !(1..=MAX_NUM_NODES).contains(&n) {
        return Err(format!(
            "size must be between 1 and {MAX_NUM_NODES}, got {n}"
        ));
    }
    Ok(n)
}

/// Top-level entry point.
///
/// With no subcommand, the cluster is started (the flattened [`Args`]). The
/// `status` subcommand instead reads the summary file written by a (possibly still
/// starting) cluster, so its presence negates the run-only requirements (e.g.
/// `--chain-image`).
#[derive(Parser, Debug)]
#[command(
    name = "hoprd-localcluster",
    about = "Run a local HOPR cluster using external processes.\n\nLifecycle: start chain container → generate identities & fund Safes → spawn hoprd nodes → open channels → wait for Ctrl-C.\n\nSee docs/localcluster/README.md for full setup instructions.",
    args_conflicts_with_subcommands = true,
    subcommand_negates_reqs = true
)]
pub struct Cli {
    #[command(flatten)]
    pub run: Args,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Query a running cluster's live status and print it as JSON.
    ///
    /// Always exits 0 with a parseable answer: the live state of a running/starting
    /// cluster, or `not_running` when nothing is listening on the control socket.
    Status(StatusArgs),
}

#[derive(Parser, Debug)]
pub struct StatusArgs {
    /// Data directory of the cluster to inspect (used to locate the control socket).
    #[arg(long, default_value = DEFAULT_CONFIG_HOME)]
    pub data_dir: PathBuf,

    /// Control-base prefix; the socket is read from `<base>.sock`. Defaults to `<data-dir>/cluster`.
    #[arg(long)]
    pub control_base: Option<PathBuf>,
}

impl StatusArgs {
    /// Resolve the control socket path from the control base.
    pub fn socket_path(&self) -> PathBuf {
        crate::control::socket_path(&resolve_control_base(
            self.control_base.as_deref(),
            &self.data_dir,
        ))
    }
}

/// Control-base prefix: explicit override or `<data-dir>/cluster`.
fn resolve_control_base(explicit: Option<&Path>, data_dir: &Path) -> PathBuf {
    explicit
        .map(Path::to_path_buf)
        .unwrap_or_else(|| data_dir.join("cluster"))
}

#[derive(Parser, Debug)]
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

    /// Path prefix for the lock (`<base>.lock`) and status socket (`<base>.sock`). Override
    /// onto a local FS when `--data-dir` is a bind mount/NFS. Defaults to `<data-dir>/cluster`.
    #[arg(long)]
    pub control_base: Option<PathBuf>,

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

    /// Inject artificial latency on inter-node P2P traffic via per-node UDP relays.
    ///
    /// One value selects the delay source; an optional `@<port>` suffix overrides the
    /// relay base port (node `i`'s relay listens on `port + i`):
    /// - a global delay spec — `100ms`, `100ms±30ms`, `uniform:50ms,150ms`, `normal:100ms,30ms`
    /// - `config:<path>` — YAML file with per-node / per-link overrides (see docs)
    ///
    /// e.g. `--latency 100ms±30ms`, `--latency config:lat.yaml@9100`.
    #[arg(long, value_parser = parse_latency)]
    pub latency: Option<Latency>,
}

impl Args {
    /// Resolve the control base: explicit `--control-base` or `<data-dir>/cluster`.
    pub fn control_base(&self) -> PathBuf {
        resolve_control_base(self.control_base.as_deref(), &self.data_dir)
    }
}

/// Where the latency delays come from. The two forms are mutually exclusive.
#[derive(Clone, Debug)]
pub enum LatencyKind {
    /// A single global delay, parsed at CLI-parse time.
    Fixed(crate::latency::DelayDist),
    /// A YAML file with per-node / per-link overrides (read lazily by [`Latency::resolve`]).
    Config(PathBuf),
}

/// Latency shaping requested on the CLI: the delay source plus the relay base port.
#[derive(Clone, Debug)]
pub struct Latency {
    pub kind: LatencyKind,
    pub port_base: u16,
}

impl Latency {
    /// Resolve the delay model the relays apply. Errors on an unreadable/invalid config
    /// file or a config that yields no delays.
    pub fn resolve(&self) -> Result<crate::latency::LatencyConfig, String> {
        match &self.kind {
            LatencyKind::Fixed(dist) => Ok(crate::latency::LatencyConfig::global(*dist)),
            LatencyKind::Config(path) => {
                let contents = std::fs::read_to_string(path)
                    .map_err(|e| format!("failed to read {}: {e}", path.display()))?;
                let cfg = crate::latency::LatencyConfig::from_yaml(&contents)?;
                if cfg.is_empty() {
                    return Err(format!(
                        "latency config {} produced no delays",
                        path.display()
                    ));
                }
                Ok(cfg)
            }
        }
    }
}

/// Relay base port; node `i`'s relay listens on `port_base + i`.
impl From<&Latency> for u16 {
    fn from(l: &Latency) -> u16 {
        l.port_base
    }
}

/// Parse `--latency`: `<spec-or-config>[@<port>]`. The body is either a delay spec or a
/// `config:<path>`; the optional `@<port>` overrides the relay base port.
fn parse_latency(s: &str) -> Result<Latency, String> {
    let (body, port_base) = match s.rsplit_once('@') {
        Some((body, port)) => (
            body,
            port.parse()
                .map_err(|_| format!("invalid relay base port '{port}'"))?,
        ),
        None => (s, DEFAULT_LATENCY_PORT_BASE),
    };
    let kind = match body.strip_prefix("config:") {
        Some(path) => LatencyKind::Config(PathBuf::from(path)),
        None => LatencyKind::Fixed(crate::latency::parse_delay(body)?),
    };
    Ok(Latency { kind, port_base })
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
