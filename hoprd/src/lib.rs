//! HOPR node daemon binary. Runs the HOPR protocol (via [`hopr-lib`](https://github.com/hoprnet/hoprnet))
//! and exposes a REST API for node management.
//!
//! When the REST API is enabled, interactive API docs are available at:
//! - `http://localhost:3001/scalar` (Scalar UI)
//! - `http://localhost:3001/swagger-ui` (Swagger UI)
//!
//! ## Usage
//! See `hoprd --help` for the full list of options.
pub mod cli;
pub mod config;
pub mod strategy;

#[cfg(all(target_os = "linux", feature = "allocator-jemalloc-stats"))]
mod jemalloc_stats;

use async_signal::{Signal, Signals};
use futures::{FutureExt, StreamExt, future::abortable};
use signal_hook::low_level;

use hoprd_api::{RestApiParameters, serve_api};
use std::sync::Arc;

use anyhow::Context;

use crate::config::HoprdConfig;
use hopr_chain_connector::{
    BlockchainConnectorConfig, blokli_client, create_trustful_hopr_blokli_connector,
};
use hopr_chain_connector::{
    HoprBlockchainSafeConnector,
    blokli_client::{BlokliClient, BlokliDnsOverride},
};
use hopr_lib::builder::HoprBuilder;
use hopr_lib::config::HoprLibConfig;
use hopr_lib::{AbortableList, HoprKeys, api::types::crypto::keypairs::Keypair};
use hopr_network_graph::{ChannelGraph, SharedChannelGraph};
#[cfg(feature = "session-server")]
use hopr_session_server_forwarder::HoprServerIpForwardingReactor;
use hopr_ticket_manager::{
    HoprTicketManager, RedbStore, RedbTicketQueue, ticket_manager_from_chain,
};
use hopr_transport_p2p::{HoprLibp2pNetworkBuilder, HoprNetwork, PeerDiscovery};

type HoprBlokliConnector = HoprBlockchainSafeConnector<BlokliClient>;
type SharedTicketManager = Arc<HoprTicketManager<RedbStore, RedbTicketQueue>>;
type HoprNode =
    hopr_lib::Hopr<Arc<HoprBlokliConnector>, SharedChannelGraph, HoprNetwork, SharedTicketManager>;

#[derive(Clone, Debug, PartialEq, Eq, Hash, strum::Display)]
enum HoprdProcess {
    #[strum(to_string = "session listener sockets")]
    ListenerSocket,
    #[strum(to_string = "hopr strategies process")]
    Strategies,
    #[strum(to_string = "REST API process")]
    RestApi,
}

#[cfg(feature = "runtime-tokio")]
pub async fn main_inner(cfg: HoprdConfig, hopr_keys: HoprKeys) -> anyhow::Result<()> {
    use hopr_lib::api::types::primitive::traits::ToHex as _;

    #[cfg(all(target_os = "linux", feature = "allocator-jemalloc-stats"))]
    let _jemalloc_stats = jemalloc_stats::JemallocStats::start().await;

    if cfg!(debug_assertions) {
        tracing::warn!("Executable was built using the DEBUG profile.");
    } else {
        tracing::info!("Executable was built using the RELEASE profile.");
    }

    let git_hash = option_env!("VERGEN_GIT_SHA").unwrap_or("unknown");
    tracing::info!(
        version = env!("CARGO_PKG_VERSION"),
        hash = git_hash,
        cfg = cfg.as_redacted_string()?,
        "Starting HOPR daemon"
    );

    if std::env::var("DAPPNODE")
        .map(|v| v.to_lowercase() == "true")
        .unwrap_or(false)
    {
        tracing::info!("The HOPRd node appears to run on DappNode");
    }

    let mut hopr_lib_cfg: HoprLibConfig = cfg.hopr.clone().into();
    update_hopr_lib_config_from_env_vars(&mut hopr_lib_cfg)?;

    tracing::info!(
        packet_key = Keypair::public(&hopr_keys.packet_key).to_peerid_str(),
        blockchain_address = Keypair::public(&hopr_keys.chain_key).to_address().to_hex(),
        "Node public identifiers"
    );

    // Create the node instance
    tracing::info!("creating the HOPRd node instance from hopr-lib");

    let mut processes = AbortableList::<HoprdProcess>::default();

    let mut chain_connector = create_trustful_hopr_blokli_connector(
        &hopr_keys.chain_key,
        BlockchainConnectorConfig {
            connection_sync_timeout: std::time::Duration::from_mins(1),
            sync_tolerance: 90,
            tx_timeout_multiplier: std::env::var("HOPR_TX_TIMEOUT_MULTIPLIER")
                .ok()
                .and_then(|p| {
                    p.parse()
                        .inspect_err(|error| tracing::warn!(%error, "failed to parse HOPR_TX_TIMEOUT_MULTIPLIER"))
                        .ok()
                })
                .unwrap_or_else(|| BlockchainConnectorConfig::default().tx_timeout_multiplier),
        },
        BlokliClient::new(
            cfg.blokli_url.parse()?,
            blokli_client::BlokliClientConfig {
                timeout: std::time::Duration::from_secs(30),
                stream_reconnect_timeout: std::time::Duration::from_secs(30),
                subscription_stream_restart_delay: Some(std::time::Duration::from_secs(1)),
                dns_override: cfg.blokli_dns_override.map(|(ip, port)| BlokliDnsOverride { ip, port }),
                // Allow local clusters to skip the blokli version compatibility check:
                // some dev images (e.g. bloklid-anvil) do index Safe events but don't
                // yet advertise the IndexesSafeEvents feature flag in their API response.
                auto_compatibility_check: std::env::var("HOPR_BLOKLI_NO_COMPAT_CHECK").is_err(),
                ..Default::default()
            },
        ),
        cfg.hopr.safe_module.module_address,
    )
    .await?;
    chain_connector.connect().await?;
    let chain_connector = Arc::new(chain_connector);

    let prober_cfg = hopr_ct_full_network::ProberConfig {
        interval: cfg.hopr.network.probe_interval,
        shuffle_ttl: cfg.hopr.network.probe_interval * 2,
        ..Default::default()
    };
    prober_cfg.validate_against_probe_timeout(hopr_lib_cfg.protocol.probe.timeout)?;

    let (ticket_manager, ticket_factory) = ticket_manager_from_chain(&chain_connector)
        .await
        .map_err(|e| anyhow::anyhow!("failed to seed ticket manager: {e}"))?;

    let path_cfg = hopr_lib_cfg.protocol.path_planner;
    let graph: SharedChannelGraph = Arc::new(ChannelGraph::with_edge_params(
        *hopr_keys.packet_key.public(),
        path_cfg.edge_penalty,
        path_cfg.min_ack_rate,
    ));
    let graph_for_ct = graph.clone();
    let safe_address = hopr_lib_cfg.safe_module.safe_address;
    let module_address = hopr_lib_cfg.safe_module.module_address;
    #[cfg(feature = "session-server")]
    let server = HoprServerIpForwardingReactor::new(
        hopr_keys.packet_key.clone(),
        cfg.session_ip_forwarding.clone(),
    );

    let builder = HoprBuilder
        .with_identity(&hopr_keys.chain_key, &hopr_keys.packet_key)
        .with_config(hopr_lib_cfg)
        .with_safe_module(&safe_address, &module_address)
        .with_chain_api(move |_ctx| chain_connector)
        .with_graph(move |_ctx| graph)
        .with_network(move |ctx| {
            Box::pin(async move {
                let peer_discovery_rx = ctx.take_peer_discovery_rx().ok_or(
                    hopr_lib::errors::HoprLibError::BuilderError("peer_discovery_rx already taken"),
                )?;
                let multiaddresses = vec![
                    (&ctx.cfg.host)
                        .try_into()
                        .map_err(hopr_lib::errors::HoprLibError::TransportError)?,
                ];
                let nb = HoprLibp2pNetworkBuilder::new(
                    peer_discovery_rx
                        .map(|(peer_id, addrs)| PeerDiscovery::Announce(peer_id, addrs)),
                );
                nb.build(
                    &ctx.packet_key,
                    multiaddresses,
                    "/hopr/mix/1.1.0",
                    ctx.cfg.protocol.transport.prefer_local_addresses,
                )
                .await
                .map_err(|e| hopr_lib::errors::HoprLibError::GeneralError(e.to_string()))
            })
        })
        .with_cover_traffic(move |ctx| {
            hopr_ct_full_network::FullNetworkDiscovery::new(
                *ctx.packet_key.public(),
                prober_cfg,
                graph_for_ct,
            )
        });

    #[cfg(feature = "session-server")]
    let node = Arc::new(
        builder
            .with_session_server(server)
            .build_full(ticket_manager, ticket_factory)
            .await?,
    );
    #[cfg(not(feature = "session-server"))]
    let node = Arc::new(builder.build_full(ticket_manager, ticket_factory).await?);

    if cfg.api.enable {
        let list = init_rest_api(&cfg, node.clone()).await?;
        processes.extend_from(list);
    }

    tracing::debug!("initializing strategies");
    let mut multi_strategy = crate::strategy::build_strategies(&cfg.strategy, Arc::clone(&node));
    tracing::debug!(strategy = %multi_strategy, "initialized strategies");

    tracing::debug!("starting up strategies");
    processes.insert(
        HoprdProcess::Strategies,
        tokio::spawn(async move {
            if let Err(e) = multi_strategy.run().await {
                tracing::error!(%e, "strategy terminated with error");
            }
        }),
    );

    let mut signals = Signals::new([Signal::Hup, Signal::Int, Signal::Term])
        .context("failed to register signal handlers")?;
    while let Some(Ok(signal)) = signals.next().await {
        match signal {
            Signal::Hup => {
                tracing::info!("Received the HUP signal... not doing anything");
            }
            Signal::Int | Signal::Term => {
                tracing::error!(signal = ?signal, "Received a termination signal... tearing down the node");
                // Explicitly tear down running processes here
                drop(node);
                drop(processes);

                tracing::error!(signal = ?signal, "All processes stopped... emulating the default signal handler...");
                low_level::emulate_default_handler(signal as i32)?;
                tracing::error!("Shutting down!");
                break;
            }
            _ => {
                tracing::error!(signal = ?signal, "Received an unhandled signal... emulating the default signal handler...");
                low_level::emulate_default_handler(signal as i32)?;
            }
        }
    }

    Ok(())
}

async fn init_rest_api(
    cfg: &HoprdConfig,
    hopr: Arc<HoprNode>,
) -> anyhow::Result<AbortableList<HoprdProcess>> {
    let node_cfg_value =
        serde_json::to_value(cfg.as_redacted()).context("failed to serialize redacted config")?;

    let api_cfg = cfg.api.clone();

    let listen_address = match &cfg.api.host.address {
        hopr_lib::config::HostType::IPv4(a) | hopr_lib::config::HostType::Domain(a) => {
            format!("{a}:{}", cfg.api.host.port)
        }
    };

    let api_listener = tokio::net::TcpListener::bind(&listen_address)
        .await
        .map_err(|e| {
            hopr_lib::errors::HoprLibError::GeneralError(format!(
                "REST API bind failed for {listen_address}: {e}"
            ))
        })?;

    tracing::info!(listen_address, "Running a REST API");

    let session_listener_sockets = Arc::new(hopr_utils_session::ListenerJoinHandles::default());

    let mut processes = AbortableList::<HoprdProcess>::default();

    processes.insert(
        HoprdProcess::ListenerSocket,
        session_listener_sockets.clone(),
    );

    let cfg_clone = cfg.clone();
    let (proc, abort_handle) = abortable(
        async move {
            if let Err(e) = serve_api(RestApiParameters {
                listener: api_listener,
                hoprd_cfg: node_cfg_value,
                cfg: api_cfg,
                hopr,
                session_listener_sockets,
                default_session_listen_host: cfg_clone
                    .session_ip_forwarding
                    .default_entry_listen_host,
            })
            .await
            {
                tracing::error!(error = %e, "the REST API server could not start")
            }
        }
        .inspect(|_| {
            tracing::warn!(
                task = "hoprd - REST API",
                "long-running background task finished"
            )
        }),
    );
    let _jh = tokio::spawn(proc);
    processes.insert(HoprdProcess::RestApi, abort_handle);

    Ok(processes)
}
fn update_hopr_lib_config_from_env_vars(cfg: &mut HoprLibConfig) -> anyhow::Result<()> {
    cfg.protocol.packet.pipeline.output_concurrency = std::env::var("HOPR_INTERNAL_OUT_PACKET_PIPELINE_CONCURRENCY")
        .ok()
        .and_then(|p| {
            p.parse()
                .inspect_err(
                    |error| tracing::warn!(%error, "failed to parse HOPR_INTERNAL_OUT_PACKET_PIPELINE_CONCURRENCY"),
                )
                .ok()
        });

    cfg.protocol.packet.pipeline.input_concurrency = std::env::var("HOPR_INTERNAL_IN_PACKET_PIPELINE_CONCURRENCY")
        .ok()
        .and_then(|p| {
            p.parse()
                .inspect_err(
                    |error| tracing::warn!(%error, "failed to parse HOPR_INTERNAL_IN_PACKET_PIPELINE_CONCURRENCY"),
                )
                .ok()
        });

    #[cfg(feature = "session-server")]
    if let Some(cap) = std::env::var("HOPR_INTERNAL_SESSION_INCOMING_CAPACITY")
        .ok()
        .and_then(|s| {
            s.trim()
                .parse::<usize>()
                .inspect_err(
                    |error| tracing::warn!(%error, "failed to parse HOPR_INTERNAL_SESSION_INCOMING_CAPACITY"),
                )
                .ok()
        })
        .filter(|&c| c > 0)
    {
        cfg.incoming_session_capacity = cap;
    }

    #[cfg(debug_assertions)]
    if let Ok(v) = std::env::var("HOPR_TEST_DISABLE_CHECKS") {
        cfg.disable_protocol_checks = v.to_lowercase() == "true";
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use hopr_lib::config::HoprLibConfig;

    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    const OUT_VAR: &str = "HOPR_INTERNAL_OUT_PACKET_PIPELINE_CONCURRENCY";
    const IN_VAR: &str = "HOPR_INTERNAL_IN_PACKET_PIPELINE_CONCURRENCY";

    struct EnvGuard(Vec<&'static str>);

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            for k in &self.0 {
                // Safety: ENV_LOCK is held for the duration of each test, serializing all env access
                unsafe { std::env::remove_var(k) };
            }
        }
    }

    fn setup_env(
        vars: &[(&'static str, Option<&str>)],
    ) -> (std::sync::MutexGuard<'static, ()>, EnvGuard) {
        let guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        for (k, v) in vars {
            match v {
                // Safety: ENV_LOCK serializes all env access across tests in this module
                Some(val) => unsafe { std::env::set_var(k, val) },
                None => unsafe { std::env::remove_var(k) },
            }
        }
        let keys = vars.iter().map(|(k, _)| *k).collect();
        (guard, EnvGuard(keys))
    }

    #[test]
    fn hoprd_process_display_strings() -> anyhow::Result<()> {
        assert_eq!(
            HoprdProcess::ListenerSocket.to_string(),
            "session listener sockets"
        );
        assert_eq!(
            HoprdProcess::Strategies.to_string(),
            "hopr strategies process"
        );
        assert_eq!(HoprdProcess::RestApi.to_string(), "REST API process");
        Ok(())
    }

    #[test]
    fn env_vars_absent_leaves_config_unchanged() -> anyhow::Result<()> {
        let (_g, _e) = setup_env(&[(OUT_VAR, None), (IN_VAR, None)]);
        let default = HoprLibConfig::default();
        let mut cfg = HoprLibConfig::default();

        update_hopr_lib_config_from_env_vars(&mut cfg)?;

        assert_eq!(
            cfg.protocol.packet.pipeline.output_concurrency,
            default.protocol.packet.pipeline.output_concurrency
        );
        assert_eq!(
            cfg.protocol.packet.pipeline.input_concurrency,
            default.protocol.packet.pipeline.input_concurrency
        );
        Ok(())
    }

    #[test]
    fn out_concurrency_set_to_valid_value() -> anyhow::Result<()> {
        let (_g, _e) = setup_env(&[(OUT_VAR, Some("4")), (IN_VAR, None)]);
        let mut cfg = HoprLibConfig::default();

        update_hopr_lib_config_from_env_vars(&mut cfg)?;

        assert_eq!(cfg.protocol.packet.pipeline.output_concurrency, Some(4));
        assert_eq!(
            cfg.protocol.packet.pipeline.input_concurrency,
            HoprLibConfig::default()
                .protocol
                .packet
                .pipeline
                .input_concurrency
        );
        Ok(())
    }

    #[test]
    fn in_concurrency_set_to_valid_value() -> anyhow::Result<()> {
        let (_g, _e) = setup_env(&[(OUT_VAR, None), (IN_VAR, Some("8"))]);
        let mut cfg = HoprLibConfig::default();

        update_hopr_lib_config_from_env_vars(&mut cfg)?;

        assert_eq!(
            cfg.protocol.packet.pipeline.output_concurrency,
            HoprLibConfig::default()
                .protocol
                .packet
                .pipeline
                .output_concurrency
        );
        assert_eq!(cfg.protocol.packet.pipeline.input_concurrency, Some(8));
        Ok(())
    }

    #[test]
    fn both_vars_set_updates_both_fields() -> anyhow::Result<()> {
        let (_g, _e) = setup_env(&[(OUT_VAR, Some("16")), (IN_VAR, Some("32"))]);
        let mut cfg = HoprLibConfig::default();

        update_hopr_lib_config_from_env_vars(&mut cfg)?;

        assert_eq!(cfg.protocol.packet.pipeline.output_concurrency, Some(16));
        assert_eq!(cfg.protocol.packet.pipeline.input_concurrency, Some(32));
        Ok(())
    }

    #[test]
    fn non_numeric_value_is_silently_ignored() -> anyhow::Result<()> {
        let (_g, _e) = setup_env(&[
            (OUT_VAR, Some("not_a_number")),
            (IN_VAR, Some("also!invalid")),
        ]);
        let default = HoprLibConfig::default();
        let mut cfg = HoprLibConfig::default();

        update_hopr_lib_config_from_env_vars(&mut cfg)?;

        assert_eq!(
            cfg.protocol.packet.pipeline.output_concurrency,
            default.protocol.packet.pipeline.output_concurrency
        );
        assert_eq!(
            cfg.protocol.packet.pipeline.input_concurrency,
            default.protocol.packet.pipeline.input_concurrency
        );
        Ok(())
    }

    #[test]
    fn zero_concurrency_is_accepted() -> anyhow::Result<()> {
        // 0 is valid usize; pipeline layer treats Some(0) the same as None (use default parallelism)
        let (_g, _e) = setup_env(&[(OUT_VAR, Some("0")), (IN_VAR, Some("0"))]);
        let mut cfg = HoprLibConfig::default();

        update_hopr_lib_config_from_env_vars(&mut cfg)?;

        assert_eq!(cfg.protocol.packet.pipeline.output_concurrency, Some(0));
        assert_eq!(cfg.protocol.packet.pipeline.input_concurrency, Some(0));
        Ok(())
    }

    #[test]
    fn negative_value_is_silently_ignored() -> anyhow::Result<()> {
        // "-1" cannot parse as usize, so fields stay unchanged
        let (_g, _e) = setup_env(&[(OUT_VAR, Some("-1")), (IN_VAR, Some("-5"))]);
        let default = HoprLibConfig::default();
        let mut cfg = HoprLibConfig::default();

        update_hopr_lib_config_from_env_vars(&mut cfg)?;

        assert_eq!(
            cfg.protocol.packet.pipeline.output_concurrency,
            default.protocol.packet.pipeline.output_concurrency
        );
        assert_eq!(
            cfg.protocol.packet.pipeline.input_concurrency,
            default.protocol.packet.pipeline.input_concurrency
        );
        Ok(())
    }

    const DISABLE_CHECKS_VAR: &str = "HOPR_TEST_DISABLE_CHECKS";

    #[test]
    fn disable_checks_absent_leaves_config_unchanged() -> anyhow::Result<()> {
        let (_g, _e) = setup_env(&[(DISABLE_CHECKS_VAR, None)]);
        let mut cfg = HoprLibConfig::default();
        cfg.disable_protocol_checks = true;

        update_hopr_lib_config_from_env_vars(&mut cfg)?;

        assert!(
            cfg.disable_protocol_checks,
            "prior value must not be clobbered when var is unset"
        );
        Ok(())
    }

    #[test]
    fn disable_checks_true_values() -> anyhow::Result<()> {
        for val in ["true", "TRUE", "True"] {
            let (_g, _e) = setup_env(&[(DISABLE_CHECKS_VAR, Some(val))]);
            let mut cfg = HoprLibConfig::default();

            update_hopr_lib_config_from_env_vars(&mut cfg)?;

            assert!(
                cfg.disable_protocol_checks,
                "expected true for value {val:?}"
            );
        }
        Ok(())
    }

    #[test]
    fn disable_checks_false_and_other_values() -> anyhow::Result<()> {
        for val in ["false", "FALSE", "0", "yes", ""] {
            let (_g, _e) = setup_env(&[(DISABLE_CHECKS_VAR, Some(val))]);
            let mut cfg = HoprLibConfig::default();

            update_hopr_lib_config_from_env_vars(&mut cfg)?;

            assert!(
                !cfg.disable_protocol_checks,
                "expected false for value {val:?}"
            );
        }
        Ok(())
    }
}
