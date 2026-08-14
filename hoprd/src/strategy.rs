use std::{sync::Arc, time::Duration};

use hopr_lib::api::{
    chain::{
        ChainReadAccountOperations, ChainReadChannelOperations, ChainReadSafeOperations,
        ChainValues, ChainWriteChannelOperations, ChainWriteTicketOperations,
    },
    node::{
        ActionableEventSource, HasChainApi, HasGraphView, HasNetworkView, HasTicketManagement,
        PacketTransport,
    },
    tickets::TicketManagement,
};
use hopr_strategy::strategy::{MultiStrategy, Strategy};
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display as StrumDisplay, VariantNames};
use validator::{Validate, ValidationError};

#[cfg(all(feature = "telemetry", not(test)))]
lazy_static::lazy_static! {
    static ref METRIC_ENABLED_STRATEGIES: hopr_lib::api::types::telemetry::MultiGauge =
        hopr_lib::api::types::telemetry::MultiGauge::new(
            "hopr_strategy_enabled_strategies",
            "List of enabled strategies",
            &["strategy"],
        )
        .unwrap();
}

#[inline]
fn just_true() -> bool {
    true
}

#[inline]
fn sixty_seconds() -> Duration {
    Duration::from_secs(60)
}

#[inline]
fn empty_strategies() -> Vec<StrategyKind> {
    vec![]
}

fn validate_execution_interval(interval: &Duration) -> std::result::Result<(), ValidationError> {
    if interval < &Duration::from_secs(10) {
        Err(ValidationError::new(
            "strategy execution interval must be at least 10 seconds",
        ))
    } else {
        Ok(())
    }
}

/// Lists all possible strategies with their respective configurations.
///
/// This is a pure serde config type — it is used for YAML deserialization and
/// carries no runtime behaviour. The runtime combinator is [`hopr_strategy::strategy::MultiStrategy`],
/// which accepts any `Box<dyn Strategy + Send>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, StrumDisplay, VariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum StrategyKind {
    #[cfg(feature = "runtime-tokio")]
    AutoRedeeming(hopr_strategy::auto_redeeming::AutoRedeemingStrategyConfig),
    #[cfg(feature = "runtime-tokio")]
    AutoFunding(hopr_strategy::auto_funding::AutoFundingStrategyConfig),
    #[cfg(feature = "runtime-tokio")]
    ClosureFinalizer(hopr_strategy::channel_finalizer::ClosureFinalizerStrategyConfig),
    #[cfg(feature = "runtime-tokio")]
    ChannelLifecycle(Box<hopr_strategy::channel_lifecycle::ChannelLifecycleConfig>),
    Multi(MultiStrategyConfig),
    Passive,
}

impl validator::Validate for StrategyKind {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        match self {
            #[cfg(feature = "runtime-tokio")]
            Self::AutoRedeeming(cfg) => cfg.validate(),
            #[cfg(feature = "runtime-tokio")]
            Self::AutoFunding(cfg) => cfg.validate(),
            #[cfg(feature = "runtime-tokio")]
            Self::ClosureFinalizer(cfg) => cfg.validate(),
            #[cfg(feature = "runtime-tokio")]
            Self::ChannelLifecycle(cfg) => cfg.validate(),
            Self::Multi(cfg) => cfg.validate(),
            Self::Passive => Ok(()),
        }
    }
}

/// Configuration options for the `MultiStrategy` group.
#[derive(Debug, Clone, PartialEq, SmartDefault, Validate, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MultiStrategyConfig {
    /// Indicate whether the `MultiStrategy` can contain another `MultiStrategy`.
    ///
    /// Default is `true`. Nesting is limited to one level: when this is `true`, nested
    /// `Multi` groups have their own `allow_recursive` forced to `false`, so three-deep
    /// nesting is silently flattened.
    #[default = true]
    #[serde(default = "just_true")]
    pub allow_recursive: bool,

    /// Execution interval for periodic scans within each sub-strategy.
    ///
    /// Default is 60 seconds, minimum is 10 seconds.
    #[default(sixty_seconds())]
    #[serde(default = "sixty_seconds", with = "humantime_serde")]
    #[validate(custom(function = "validate_execution_interval"))]
    pub execution_interval: Duration,

    /// Configuration of individual sub-strategies.
    ///
    /// Default is empty, which makes the `MultiStrategy` behave as passive.
    #[default(_code = "vec![]")]
    #[serde(default = "empty_strategies")]
    #[validate(nested)]
    pub strategies: Vec<StrategyKind>,
}

/// Default HOPRd strategy configuration.
///
/// ## Strategies included
/// - `AutoRedeeming` *(requires `runtime-tokio` feature)*: redeems single tickets on channel close if worth at least 1
///   wxHOPR.
/// - `ChannelLifecycle` *(requires `runtime-tokio` feature)*: unified strategy that automatically opens, funds,
///   tops up, closes, and finalizes outgoing payment channels based on peer connectivity and quality.
///
/// When `runtime-tokio` is not enabled, returns an empty `MultiStrategyConfig` (passive behaviour).
pub fn hopr_default_strategies() -> MultiStrategyConfig {
    #[cfg(feature = "runtime-tokio")]
    {
        use hopr_strategy::{
            auto_redeeming::AutoRedeemingStrategyConfig,
            channel_lifecycle::{CapacitySizingMode, ChannelLifecycleConfig, FundingConfig},
        };
        return MultiStrategyConfig {
            allow_recursive: false,
            execution_interval: Duration::from_secs(60),
            strategies: vec![
                StrategyKind::AutoRedeeming(AutoRedeemingStrategyConfig {
                    redeem_on_winning: true,
                    ..Default::default()
                }),
                StrategyKind::ChannelLifecycle(Box::new(ChannelLifecycleConfig {
                    // Probabilistic sizing comes from upstream; the capacities are ours. The 1 GiB
                    // default exhausted mid-session: the exit began rejecting consecutive tickets
                    // with "ticket value is greater than remaining unrealized balance", which
                    // presents as a throughput collapse and reads exactly like a return-path
                    // failure.
                    //
                    // Probabilistic rather than deterministic because payouts are lumpy -- winning
                    // tickets are Binomial(N, win_prob) -- so a channel sized to the mean drain
                    // rests near one ticket face value and starves on variance alone. 0.99 adds
                    // k*sigma above the mean, k = 2.33.
                    funding: FundingConfig {
                        initial_capacity: bytesize::ByteSize::gib(10),
                        topup_capacity: bytesize::ByteSize::gib(10),
                        lower_capacity_threshold: bytesize::ByteSize::gib(1),
                        sizing_mode: CapacitySizingMode::Probabilistic {
                            success_probability: 0.99,
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                })),
            ],
        };
    }
    #[allow(unreachable_code)]
    MultiStrategyConfig::default()
}

/// Builds a [`MultiStrategy`] from a [`MultiStrategyConfig`] and a node reference.
///
/// Maps each [`StrategyKind`] variant to its concrete builder, wires in the node,
/// and returns a single `Box<dyn Strategy + Send>` that runs all sub-strategies
/// concurrently.
///
/// External strategies can be composed by building this result first, then wrapping
/// it with additional strategies in a new `MultiStrategy::new(...)` call at the
/// call site.
pub fn build_strategies<N>(
    cfg: &MultiStrategyConfig,
    node: Arc<N>,
) -> anyhow::Result<Box<dyn Strategy + Send>>
where
    N: ActionableEventSource
        + HasChainApi<
            ChainApi: ChainReadAccountOperations
                          + ChainReadChannelOperations
                          + ChainReadSafeOperations
                          + ChainValues
                          + ChainWriteChannelOperations
                          + ChainWriteTicketOperations
                          + Clone
                          + Send
                          + Sync
                          + 'static,
        > + HasGraphView
        + HasNetworkView
        + HasTicketManagement<TicketManager: TicketManagement + Clone + Send + Sync + 'static>
        + PacketTransport
        + Send
        + Sync
        + 'static,
{
    // Seed all gauges to 0 exactly once at the top level — recursive calls via
    // StrategyKind::Multi must not reset them or they would clobber values set
    // by earlier iterations of the outer loop.
    #[cfg(all(feature = "telemetry", not(test)))]
    StrategyKind::VARIANTS
        .iter()
        .for_each(|s| METRIC_ENABLED_STRATEGIES.set(&[*s], 0_f64));

    build_strategies_inner(cfg, node)
}

fn build_strategies_inner<N>(
    cfg: &MultiStrategyConfig,
    node: Arc<N>,
) -> anyhow::Result<Box<dyn Strategy + Send>>
where
    N: ActionableEventSource
        + HasChainApi<
            ChainApi: ChainReadAccountOperations
                          + ChainReadChannelOperations
                          + ChainReadSafeOperations
                          + ChainValues
                          + ChainWriteChannelOperations
                          + ChainWriteTicketOperations
                          + Clone
                          + Send
                          + Sync
                          + 'static,
        > + HasGraphView
        + HasNetworkView
        + HasTicketManagement<TicketManager: TicketManagement + Clone + Send + Sync + 'static>
        + PacketTransport
        + Send
        + Sync
        + 'static,
{
    let mut strategies = Vec::<Box<dyn Strategy + Send>>::new();

    for strategy in cfg.strategies.iter() {
        match strategy {
            #[cfg(feature = "runtime-tokio")]
            StrategyKind::AutoRedeeming(sub_cfg) => strategies.push(
                hopr_strategy::auto_redeeming::AutoRedeemingStrategy::new(
                    *sub_cfg,
                    cfg.execution_interval,
                )
                .build(Arc::clone(&node))?,
            ),
            #[cfg(feature = "runtime-tokio")]
            StrategyKind::AutoFunding(sub_cfg) => strategies.push(
                hopr_strategy::auto_funding::AutoFundingStrategy::new(
                    *sub_cfg,
                    cfg.execution_interval,
                )
                .build(Arc::clone(&node))?,
            ),
            #[cfg(feature = "runtime-tokio")]
            StrategyKind::ClosureFinalizer(sub_cfg) => strategies.push(
                hopr_strategy::channel_finalizer::ClosureFinalizerStrategy::new(
                    *sub_cfg,
                    cfg.execution_interval,
                )
                .build(Arc::clone(&node))?,
            ),
            // ChannelLifecycle owns its own tick cadence via
            // ChannelLifecycleConfig::tick_interval and runs as an independent
            // async task; cfg.execution_interval does not apply to it.
            #[cfg(feature = "runtime-tokio")]
            StrategyKind::ChannelLifecycle(sub_cfg) => strategies.push(
                hopr_strategy::channel_lifecycle::ChannelLifecycleStrategy::new(
                    (**sub_cfg).clone(),
                )
                .build(Arc::clone(&node))?,
            ),
            StrategyKind::Multi(sub_cfg) => {
                if cfg.allow_recursive {
                    let mut sub = sub_cfg.clone();
                    sub.allow_recursive = false;
                    strategies.push(build_strategies_inner(&sub, Arc::clone(&node))?);
                } else {
                    tracing::error!("recursive multi-strategy not allowed and skipped");
                    continue; // skip the telemetry update: nothing was actually built
                }
            }
            StrategyKind::Passive => {} // passive = empty sub-strategy list
        }

        #[cfg(all(feature = "telemetry", not(test)))]
        if !matches!(strategy, StrategyKind::Passive) {
            METRIC_ENABLED_STRATEGIES.set(&[&strategy.to_string()], 1_f64);
        }
    }

    Ok(Box::new(MultiStrategy::new(strategies)))
}

#[cfg(all(test, feature = "runtime-tokio"))]
mod tests {
    use hopr_lib::api::types::primitive::prelude::{HoprBalance, U256};

    use super::*;

    /// A single HOPR packet's usable payload, in bytes, at the pinned hopr-lib rev
    /// (`10f6d80c…`): `HoprPacket::PAYLOAD_SIZE`. Hardcoded rather than pulled from
    /// `hopr_lib::exports::transport::PACKET_PAYLOAD_SIZE` (a `#[doc(hidden)]`
    /// re-export) to keep this test's dependency surface minimal.
    struct TestTransport;

    impl PacketTransport for TestTransport {
        fn packet_payload_size() -> usize {
            1038
        }
    }

    fn wei(w: u128) -> HoprBalance {
        HoprBalance::from(U256::from(w))
    }

    /// Pins the wxHOPR amounts the shipped `probabilistic{0.99}` sizing mode resolves
    /// to on rotsee and jura, given each network's live on-chain ticket price and
    /// winning probability (read via `cast call` against the ticket-price and
    /// winning-probability oracles on Gnosis at the time this was written). Fails if
    /// the shipped capacities or sizing mode drift from the figures reviewed in the PR.
    #[test]
    fn shipped_channel_lifecycle_funding_resolves_to_reviewed_stakes() -> anyhow::Result<()> {
        let cfg = serde_saphyr::from_str::<crate::config::HoprdConfig>(include_str!(
            "../../deploy/compose/hoprd/conf/hoprd.cfg.yaml"
        ))?;
        let funding = cfg
            .strategy
            .strategies
            .iter()
            .find_map(|s| match s {
                StrategyKind::ChannelLifecycle(c) => Some(c.funding.clone()),
                _ => None,
            })
            .expect("shipped config must declare a ChannelLifecycle strategy");

        // rotsee: ticket price 100 wei, win_prob 1.25e-4 (1/8 000)
        let rotsee = funding.resolve::<TestTransport>(wei(100), 1.25e-4);
        assert_eq!(rotsee.initial_balance, wei(374_400_000));
        assert_eq!(rotsee.topup_balance, wei(201_600_000));
        assert_eq!(rotsee.lower_balance_threshold, wei(201_600_000));
        assert_eq!(rotsee.min_safe_balance_required, wei(201_600_000));

        // jura: ticket price 1e13 wei, win_prob 4e-6 (1/250 000)
        let jura = funding.resolve::<TestTransport>(wei(10_000_000_000_000), 4.0e-6);
        assert_eq!(jura.initial_balance, wei(67_500_000_000_000_000_000));
        assert_eq!(jura.topup_balance, wei(45_000_000_000_000_000_000));
        assert_eq!(
            jura.lower_balance_threshold,
            wei(45_000_000_000_000_000_000)
        );
        assert_eq!(
            jura.min_safe_balance_required,
            wei(45_000_000_000_000_000_000)
        );

        Ok(())
    }
}
