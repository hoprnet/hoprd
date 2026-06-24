use std::path::PathBuf;

use anyhow::Context;
use hopr_chain_connector::{
    BlockchainConnectorConfig,
    api::*,
    blokli_client::{BlokliClient, BlokliClientConfig, BlokliQueryClient},
    create_trustful_hopr_blokli_connector, create_trustful_safeless_hopr_blokli_connector,
    reexports::chain::exports::alloy::hex,
};
use hopr_lib::{
    HoprKeys,
    api::types::{
        crypto::{
            crypto_traits::Randomizable,
            keypairs::{ChainKeypair, Keypair},
        },
        primitive::prelude::XDaiBalance,
    },
    config::SafeModule,
};
use hopr_session_server_forwarder::config::SessionIpForwardingConfig;
use hopr_strategy::{
    auto_redeeming::AutoRedeemingStrategyConfig,
    channel_lifecycle::{ChannelLifecycleConfig, PopulationConfig},
};
use hoprd::{
    config::{Db, HoprdConfig, Identity, UserHoprLibConfig, UserHoprNetworkConfig},
    strategy::{MultiStrategyConfig, StrategyKind},
};
use hoprd_api::config::{Api, Auth};

pub const DEFAULT_BLOKLI_URL: &str = "http://localhost:8080";
pub const DEFAULT_PRIVATE_KEY: &str =
    "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
pub const DEFAULT_CONFIG_HOME: &str = "/tmp/hopr-nodes";
pub const DEFAULT_IDENTITY_PASSWORD: &str = "password";
pub const DEFAULT_NUM_NODES: usize = 3;
pub const MAX_NUM_NODES: usize = 5;
// Increased tx client timeout multiplier for Anvil
pub const DEFAULT_TX_TIMEOUT_MULTIPLIER: u32 = 10;

pub const DEFAULT_NUM_EXTRA_IDENTITIES: usize = 0;
pub const MAX_EXTRA_IDENTITIES: usize = 5;
/// Base port for latency relays; relay for node `i` listens on `DEFAULT_LATENCY_PORT_BASE + i`.
pub const DEFAULT_LATENCY_PORT_BASE: u16 = 9100;
/// Password for extra identity keystores.
///
/// Intentionally a known constant so external tooling can hardcode it without
/// per-run configuration. Not a secret — this is a local-dev cluster only.
pub const EXTRA_IDENTITY_PASSWORD: &str = "local-cluster";

#[derive(Clone, Debug)]
pub struct GenerationConfig {
    pub blokli_url: String,
    pub private_key: String,
    pub num_nodes: usize,
    pub config_home: PathBuf,
    pub identity_password: String,
    pub random_identities: bool,
    /// Number of extra identities to provision (0–`MAX_EXTRA_IDENTITIES`).
    pub num_extras: usize,
    /// P2P bind/announce host. Used to pre-announce nodes so blokli indexes
    /// the accounts before hoprd starts.
    pub p2p_host: String,
    /// Base P2P port; node `i` listens on `p2p_port_base + i`.
    pub p2p_port_base: u16,
    /// Enables channel lifecycle strategy in generated hoprd configs.
    pub enable_channel_strategy: bool,
    /// When set, each node announces its latency-relay port instead of its real
    /// listen port and disables its own on-chain announce, so peers dial the relay.
    pub latency: Option<crate::latency::LatencyConfig>,
    /// Base port for latency relays; node `i`'s relay listens on `latency_port_base + i`.
    pub latency_port_base: u16,
}

impl Default for GenerationConfig {
    fn default() -> Self {
        Self {
            blokli_url: DEFAULT_BLOKLI_URL.to_string(),
            private_key: DEFAULT_PRIVATE_KEY.to_string(),
            num_nodes: DEFAULT_NUM_NODES,
            config_home: PathBuf::from(DEFAULT_CONFIG_HOME),
            identity_password: DEFAULT_IDENTITY_PASSWORD.to_string(),
            random_identities: false,
            num_extras: DEFAULT_NUM_EXTRA_IDENTITIES,
            p2p_host: "127.0.0.1".to_string(),
            p2p_port_base: 9000,
            enable_channel_strategy: false,
            latency: None,
            latency_port_base: DEFAULT_LATENCY_PORT_BASE,
        }
    }
}

/// A provisioned HOPR identity: an on-disk encrypted keystore and an on-chain
/// Safe + Module. Used for both cluster nodes and extra identities.
pub struct GeneratedIdentity {
    pub id: usize,
    /// EVM address derived from the chain key (hex string with 0x prefix).
    pub address: String,
    pub safe_address: String,
    pub module_address: String,
    pub id_file: PathBuf,
    pub password: String,
}

pub struct GenerationOutput {
    pub nodes: Vec<GeneratedIdentity>,
    pub extras: Vec<GeneratedIdentity>,
}

lazy_static::lazy_static! {
    static ref NODE_KEYS: [HoprKeys; MAX_NUM_NODES] = [
        (
            hex!("76a4edbc3f595d4d07671779a0055e30b2b8477ecfd5d23c37afd7b5aa83781d"),
            hex!("71bf1f42ebbfcd89c3e197a3fd7cda79b92499e509b6fefa0fe44d02821d146a")
        ).try_into().unwrap(),
        (
            hex!("c90f09e849aa512be3dd007452977e32c7cfdc1e3de1a62bd92ba6592bcc9e90"),
            hex!("c3659450e994f3ad086373440e4e7070629a1bfbd555387237ccb28d17acbfc8")
        ).try_into().unwrap(),
        (
            hex!("40d4749a620d1a4278d030a3153b5b94d6fcd4f9677f6ce8e37e6ebb1987ad53"),
            hex!("4a14c5aeb53629a2dd45058a8d233f24dd90192189e8200a1e5f10069868f963")
        ).try_into().unwrap(),
        (
            hex!("e539f1ac48270be4e84b6acfe35252df5e141a29b50ddb07b50670271bb574ee"),
            hex!("8c1edcdebfe508031e4124168bb4a133180e8ee68207a7946fcdc4ad0068ef0d")
        ).try_into().unwrap(),
        (
            hex!("9ab557eb14d8b081c7e1750eb87407d8c421aa79bdeb420f38980829e7dbf936"),
            hex!("6075c595103667537c33cdb954e3e5189921cab942e5fc0ba9ec27fe6d7787d1")
        ).try_into().unwrap()
    ];

    /// Hardcoded keys for `--extra-identities`.
    ///
    /// Frozen at compile time so the EVM addresses, Safe addresses, and Module
    /// addresses remain identical across cluster runs (given the same Anvil
    /// chain). Must not overlap with `NODE_KEYS`.
    static ref EXTRA_KEYS: [HoprKeys; MAX_EXTRA_IDENTITIES] = [
        (
            hex!("a8c2179d4f2e5b1a0c9d8e7f6a5b4c3d2e1f0a9b8c7d6e5f4a3b2c1d0e9f8a7b"),
            hex!("b7d3286ae0f3c4b5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9")
        ).try_into().unwrap(),
        (
            hex!("c8e4397bf1a4d5c6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0"),
            hex!("d9f54a8c02b5e6d7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1")
        ).try_into().unwrap(),
        (
            hex!("ea065b9d13c6f7e8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2"),
            hex!("fb176cae24d7a8f9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a3")
        ).try_into().unwrap(),
        (
            hex!("0c287dbf35e8b9a0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a3b4"),
            hex!("1d398ec046f9cab1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5")
        ).try_into().unwrap(),
        (
            hex!("2e4a9fd157a0dbc2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6"),
            hex!("3f5ba0e268b1ecd3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7")
        ).try_into().unwrap(),
    ];
}

/// Build the multiaddr that hoprd will announce on chain for a given host/port.
///
/// The format must match what hoprd derives from its `--host` argument so that
/// blokli sees the same multiaddr from the pre-announce and hoprd startup,
/// allowing hoprd to receive `AlreadyAnnounced` and skip the on-chain announce.
///
/// IP addresses use `/ip4/<addr>/udp/<port>/quic-v1`; all other values
/// (including "localhost") use `/dns4/<host>/udp/<port>/quic-v1`.
fn build_announce_multiaddr(host: &str, port: u16) -> anyhow::Result<Multiaddr> {
    let s = if host.parse::<std::net::IpAddr>().is_ok() {
        format!("/ip4/{host}/udp/{port}/quic-v1")
    } else {
        format!("/dns4/{host}/udp/{port}/quic-v1")
    };
    s.parse().context("invalid pre-announce multiaddr")
}

/// Generate test node Safes, hoprd configuration files, and optional extra
/// identities for external tooling.
///
/// Each cluster node is pre-announced on-chain using a module-aware connector
/// before hoprd starts, ensuring blokli indexes the account during its
/// catch-up phase rather than the live phase (where announcement events are
/// not monitored).
pub async fn generate(config: &GenerationConfig) -> anyhow::Result<GenerationOutput> {
    std::fs::create_dir_all(&config.config_home)?;
    let home_path = &config.config_home;
    let private_key = hex::decode(&config.private_key).context("invalid private key")?;

    let blokli_client = BlokliClient::new(
        config.blokli_url.parse()?,
        BlokliClientConfig {
            auto_compatibility_check: false,
            ..Default::default()
        },
    );
    let status = blokli_client.query_health().await?;
    if !status.eq_ignore_ascii_case("ok") {
        return Err(anyhow::anyhow!("Blokli is not usable: {status}"));
    }

    // Create connector for the deployer account
    let mut anvil_connector = create_trustful_safeless_hopr_blokli_connector(
        &ChainKeypair::from_secret(&private_key)?,
        BlockchainConnectorConfig {
            tx_timeout_multiplier: DEFAULT_TX_TIMEOUT_MULTIPLIER,
            ..Default::default()
        },
        blokli_client.clone(),
    )
    .await?;
    anvil_connector.connect().await?;

    let initial_token_balance: HoprBalance = "1000 wxHOPR".parse()?;
    let initial_native_balance: XDaiBalance = "1 xDai".parse()?;
    let p2p_host = &config.p2p_host;

    let effective_num_nodes = config.num_nodes.clamp(1, NODE_KEYS.len());
    let mut strategies = vec![StrategyKind::AutoRedeeming(AutoRedeemingStrategyConfig {
        redeem_on_winning: true,
        ..Default::default()
    })];
    if config.enable_channel_strategy {
        let mesh_target = effective_num_nodes.saturating_sub(1);
        strategies.push(StrategyKind::ChannelLifecycle(Box::new(
            ChannelLifecycleConfig {
                population: PopulationConfig {
                    min_open_channels: mesh_target,
                    target_open_channels: mesh_target,
                    ..Default::default()
                },
                // probe_recheck_threshold=3s → first probe within 3s → EMA converges
                // immediately → peer_score ≥ 0.5 well before this 10s tick fires.
                tick_interval: std::time::Duration::from_secs(10),
                ..Default::default()
            },
        )));
    }
    let node_strategy = MultiStrategyConfig {
        allow_recursive: false,
        execution_interval: std::time::Duration::from_secs(60),
        strategies,
    };

    let mut nodes = Vec::with_capacity(effective_num_nodes);

    for id in 0..effective_num_nodes {
        let kp = if config.random_identities {
            HoprKeys::random()
        } else {
            NODE_KEYS[id].clone()
        };
        let node_address = kp.chain_key.public().to_address();
        eprintln!("Node {id}: Address {node_address}");

        let node_connector = std::sync::Arc::new(
            create_trustful_safeless_hopr_blokli_connector(
                &kp.chain_key,
                BlockchainConnectorConfig {
                    tx_timeout_multiplier: DEFAULT_TX_TIMEOUT_MULTIPLIER,
                    ..Default::default()
                },
                blokli_client.clone(),
            )
            .await?,
        );

        eprint!("Node {id}: Checking balances...");

        // Send 1 xDai to the new node address from Anvil 0 account
        let node_native_balance: XDaiBalance = node_connector.balance(node_address).await?;
        if node_native_balance < initial_native_balance {
            let top_up = initial_native_balance - node_native_balance;
            if anvil_connector.balance(*anvil_connector.me()).await? < top_up {
                return Err(anyhow::anyhow!(
                    "Account {} must have at least {top_up}.",
                    anvil_connector.me()
                ));
            }

            anvil_connector
                .withdraw(top_up, &node_address)
                .await?
                .await?;
            eprint!("\x1b[2K\rNode {id}: {top_up} transferred to {node_address}");
        } else {
            eprint!(
                "\x1b[2K\rNode {id}: {node_address} already has {node_native_balance} xDai tokens"
            );
        }

        eprint!("\x1b[2K\rNode {id}: Checking Safe deployment...");
        let safe = if let Some(safe) = node_connector
            .safe_info(SafeSelector::Owner(node_address))
            .await?
        {
            safe
        } else {
            // Send 1000 wxHOPR tokens to the new node address from Anvil 0 account
            eprint!("\x1b[2K\rNode {id}: Topping up to {initial_token_balance}...");
            let node_token_balance: HoprBalance = node_connector.balance(node_address).await?;
            if node_token_balance < initial_token_balance {
                let top_up = initial_token_balance - node_token_balance;
                if anvil_connector.balance(*anvil_connector.me()).await? < top_up {
                    return Err(anyhow::anyhow!(
                        "Account {} must have at least {top_up}.",
                        anvil_connector.me()
                    ));
                }

                anvil_connector
                    .withdraw(top_up, &node_address)
                    .await?
                    .await?;
                eprint!("\x1b[2K\rNode {id}: {top_up} transferred to {node_address}");
            } else {
                eprint!(
                    "\x1b[2K\rNode {id}: {node_address} already has {node_token_balance} wxHOPR tokens"
                );
            }

            eprint!("\x1b[2K\rNode {id}: Deploying Safe...");
            // Subscribe before submitting the tx so the SafeDeployed event is not
            // missed if blokli indexes the block before our subscription opens.
            let node_connector_clone = node_connector.clone();
            let poll_handle = tokio::task::spawn(async move {
                let deadline = std::time::Instant::now() + std::time::Duration::from_secs(120);
                loop {
                    if let Some(s) = node_connector_clone
                        .safe_info(SafeSelector::Owner(node_address))
                        .await?
                    {
                        return Ok::<_, anyhow::Error>(s);
                    }
                    if std::time::Instant::now() >= deadline {
                        anyhow::bail!("Node {id}: safe not indexed after 120s");
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                }
            });
            let deploy_result: anyhow::Result<()> = async {
                node_connector
                    .deploy_safe(initial_token_balance)
                    .await?
                    .await?;
                Ok(())
            }
            .await;
            if let Err(e) = deploy_result {
                poll_handle.abort();
                return Err(e);
            }
            poll_handle.await??
        };

        // Pre-announce the node's KeyBinding + multiaddr before hoprd starts.
        //
        // Blokli's live-phase filter does not include the announcements contract, so
        // any KeyBinding/Announcement txs submitted by hoprd at startup may be missed.
        // By pre-announcing here (while blokli is still processing blocks from the
        // Safe-deployment epoch), the events land in a range that blokli's partial
        // re-index (triggered by hoprd's RegisteredNodeSafe tx) will cover.
        eprint!("\x1b[2K\rNode {id}: Pre-announcing on chain...");
        let mut module_connector = create_trustful_hopr_blokli_connector(
            &kp.chain_key,
            BlockchainConnectorConfig {
                tx_timeout_multiplier: DEFAULT_TX_TIMEOUT_MULTIPLIER,
                ..Default::default()
            },
            blokli_client.clone(),
            safe.module,
        )
        .await?;
        module_connector.connect().await?;

        // Register node↔Safe binding in HoprNodeSafeRegistry before announcing.
        // hoprd does the same on startup; omitting this causes InvalidTokenSender()
        // in the HoprAnnouncements keyBind call.
        match module_connector.register_safe(&safe.address).await {
            Ok(awaiter) => {
                awaiter.await.context("safe registration")?;
            }
            Err(SafeRegistrationError::AlreadyRegistered(_)) => {}
            Err(e) => return Err(anyhow::anyhow!("safe registration failed: {e}")),
        }

        // With latency enabled, peers must dial the relay, so announce the relay port
        // (the node still binds its real port; its own announce is disabled below).
        let announce_port = if config.latency.is_some() {
            config.latency_port_base.checked_add(id as u16)
        } else {
            config.p2p_port_base.checked_add(id as u16)
        }
        .ok_or_else(|| {
            anyhow::anyhow!("announce port overflow: port base + node id {id} exceeds u16")
        })?;
        let multiaddr = build_announce_multiaddr(p2p_host, announce_port)?;
        match module_connector
            .announce(&[multiaddr], &kp.packet_key)
            .await
        {
            Ok(awaiter) => {
                awaiter.await.context("pre-announce confirmation")?;
                eprintln!("\x1b[2K\rNode {id}: Pre-announced");
            }
            Err(AnnouncementError::AlreadyAnnounced) => {
                eprintln!("\x1b[2K\rNode {id}: Already announced, skipping pre-announce");
            }
            Err(e) => return Err(anyhow::anyhow!("pre-announce failed: {e}")),
        }

        let id_file = home_path.join(format!("node_id_{id}.id"));
        let id_file_str = id_file
            .to_str()
            .ok_or(anyhow::anyhow!("Invalid path"))?
            .to_owned();

        let node_cfg = HoprdConfig {
            hopr: UserHoprLibConfig {
                // When relaying through the latency proxy, the relay port is pre-announced
                // here; the node must not self-announce its real port (it would publish a
                // second, undelayed address peers could dial).
                announce: config.latency.is_none(),
                network: UserHoprNetworkConfig {
                    announce_local_addresses: true,
                    prefer_local_addresses: true,
                    probe_recheck_threshold: std::time::Duration::from_secs(3),
                    probe_interval: std::time::Duration::from_secs(3),
                    ..Default::default()
                },
                safe_module: SafeModule {
                    safe_address: safe.address,
                    module_address: safe.module,
                },
                ..Default::default()
            },
            identity: Identity {
                file: id_file_str.clone(),
                password: config.identity_password.clone(),
                private_key: None,
            },
            db: Db {
                data: home_path
                    .join(format!("db_{id}"))
                    .to_str()
                    .ok_or(anyhow::anyhow!("Invalid path"))?
                    .to_owned(),
                initialize: true,
                force_initialize: true,
            },
            api: Api {
                enable: true,
                auth: Auth::None,
                ..Default::default()
            },
            blokli_url: config.blokli_url.clone(),
            session_ip_forwarding: SessionIpForwardingConfig {
                use_target_allow_list: false,
                ..Default::default()
            },
            strategy: node_strategy.clone(),
        };

        let cfg_file = home_path
            .join(format!("hoprd_cfg_{id}.yaml"))
            .to_str()
            .ok_or(anyhow::anyhow!("Invalid path"))?
            .to_owned();
        std::fs::write(&cfg_file, serde_saphyr::to_string(&node_cfg)?)?;
        kp.write_eth_keystore(&id_file_str, &config.identity_password)?;

        eprintln!("\x1b[2K\rNode {id}: Node config written to {cfg_file}");

        nodes.push(GeneratedIdentity {
            id,
            address: node_address.to_string(),
            safe_address: safe.address.to_string(),
            module_address: safe.module.to_string(),
            id_file,
            password: config.identity_password.clone(),
        });
    }

    let mut extras = Vec::with_capacity(config.num_extras);

    for id in 0..config.num_extras.clamp(0, EXTRA_KEYS.len()) {
        let kp = EXTRA_KEYS[id].clone();
        let node_address = kp.chain_key.public().to_address();
        eprintln!("Extra {id}: Address {node_address}");

        let node_connector = std::sync::Arc::new(
            create_trustful_safeless_hopr_blokli_connector(
                &kp.chain_key,
                BlockchainConnectorConfig {
                    tx_timeout_multiplier: DEFAULT_TX_TIMEOUT_MULTIPLIER,
                    ..Default::default()
                },
                blokli_client.clone(),
            )
            .await?,
        );

        eprint!("Extra {id}: Checking balances...");

        let node_native_balance: XDaiBalance = node_connector.balance(node_address).await?;
        if node_native_balance < initial_native_balance {
            let top_up = initial_native_balance - node_native_balance;
            if anvil_connector.balance(*anvil_connector.me()).await? < top_up {
                return Err(anyhow::anyhow!(
                    "Account {} must have at least {top_up}.",
                    anvil_connector.me()
                ));
            }

            anvil_connector
                .withdraw(top_up, &node_address)
                .await?
                .await?;
            eprint!("\x1b[2K\rExtra {id}: {top_up} transferred to {node_address}");
        } else {
            eprint!(
                "\x1b[2K\rExtra {id}: {node_address} already has {node_native_balance} xDai tokens"
            );
        }

        eprint!("\x1b[2K\rExtra {id}: Checking Safe deployment...");
        let safe = if let Some(safe) = node_connector
            .safe_info(SafeSelector::Owner(node_address))
            .await?
        {
            safe
        } else {
            eprint!("\x1b[2K\rExtra {id}: Topping up to {initial_token_balance}...");
            let node_token_balance: HoprBalance = node_connector.balance(node_address).await?;
            if node_token_balance < initial_token_balance {
                let top_up = initial_token_balance - node_token_balance;
                if anvil_connector.balance(*anvil_connector.me()).await? < top_up {
                    return Err(anyhow::anyhow!(
                        "Account {} must have at least {top_up}.",
                        anvil_connector.me()
                    ));
                }

                anvil_connector
                    .withdraw(top_up, &node_address)
                    .await?
                    .await?;
                eprint!("\x1b[2K\rExtra {id}: {top_up} transferred to {node_address}");
            } else {
                eprint!(
                    "\x1b[2K\rExtra {id}: {node_address} already has {node_token_balance} wxHOPR tokens"
                );
            }

            eprint!("\x1b[2K\rExtra {id}: Deploying Safe...");
            let node_connector_clone = node_connector.clone();
            let poll_handle = tokio::task::spawn(async move {
                let deadline = std::time::Instant::now() + std::time::Duration::from_secs(120);
                loop {
                    if let Some(s) = node_connector_clone
                        .safe_info(SafeSelector::Owner(node_address))
                        .await?
                    {
                        return Ok::<_, anyhow::Error>(s);
                    }
                    if std::time::Instant::now() >= deadline {
                        anyhow::bail!("Extra {id}: safe not indexed after 120s");
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                }
            });
            let deploy_result: anyhow::Result<()> = async {
                node_connector
                    .deploy_safe(initial_token_balance)
                    .await?
                    .await?;
                Ok(())
            }
            .await;
            if let Err(e) = deploy_result {
                poll_handle.abort();
                return Err(e);
            }
            poll_handle.await??
        };

        let id_file = home_path.join(format!("extra_id_{id}.id"));
        let id_file_str = id_file
            .to_str()
            .ok_or(anyhow::anyhow!("Invalid path"))?
            .to_owned();
        kp.write_eth_keystore(&id_file_str, EXTRA_IDENTITY_PASSWORD)?;

        eprintln!("\x1b[2K\rExtra {id}: Identity written to {id_file_str}");

        extras.push(GeneratedIdentity {
            id,
            address: node_address.to_string(),
            safe_address: safe.address.to_string(),
            module_address: safe.module.to_string(),
            id_file,
            password: EXTRA_IDENTITY_PASSWORD.to_string(),
        });
    }

    Ok(GenerationOutput { nodes, extras })
}
