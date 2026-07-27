use std::{sync::Arc, time::Duration};

use hopr_lib::api::{
    chain::{
        ChainReadAccountOperations, ChainReadChannelOperations, ChainReadSafeOperations,
        ChainValues, ChainWriteChannelOperations, ChainWriteTicketOperations,
    },
    node::{ActionableEventSource, HasChainApi, HasGraphView, HasNetworkView, HasTicketManagement},
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
        use hopr_strategy::auto_redeeming::AutoRedeemingStrategyConfig;
        return MultiStrategyConfig {
            allow_recursive: false,
            execution_interval: Duration::from_secs(60),
            strategies: vec![
                StrategyKind::AutoRedeeming(AutoRedeemingStrategyConfig {
                    redeem_on_winning: true,
                    ..Default::default()
                }),
                StrategyKind::ChannelLifecycle(Box::default()),
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
/// When the `HOPRD_ENABLE_PIX` environment variable is set to `1`, the
/// `NonAnonymousPix` strategy is added programmatically (it is intentionally
/// not a YAML-configurable [`StrategyKind`] because its config type's serde
/// representation is incompatible with `serde_saphyr`).
///
/// External strategies can be composed by building this result first, then wrapping
/// it with additional strategies in a new `MultiStrategy::new(...)` call at the
/// call site.
pub fn build_strategies<N>(cfg: &MultiStrategyConfig, node: Arc<N>) -> Box<dyn Strategy + Send>
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

    let mut multi = build_strategies_inner(cfg, Arc::clone(&node));

    // NonAnonymousPix is not a YAML-configurable StrategyKind because its
    // HoprBalance fields don't round-trip through serde_saphyr. Instead it's
    // enabled via environment variable for test/development use.
    #[cfg(feature = "runtime-tokio")]
    if std::env::var("HOPRD_ENABLE_PIX")
        .ok()
        .map_or(false, |v| v == "1" || v.eq_ignore_ascii_case("true"))
    {
        let pix_cfg = hopr_strategy::non_anonymous_pix::NonAnonymousPixStrategyConfig {
            price_per_byte: "1 wxHOPR".parse().expect("valid static amount"),
            max_ssa_allocation: "100 wxHOPR".parse().expect("valid static amount"),
            max_deposit_tracking_time: std::time::Duration::from_secs(3600),
            pix_recovery_db_path: None,
            pix_recovery_password_env: None,
            gas_xdai_per_sweep: Default::default(),
        };
        match hopr_strategy::non_anonymous_pix::NonAnonymousPixStrategy::new(pix_cfg)
            .build(Arc::clone(&node))
        {
            Ok(pix) => {
                multi = Box::new(MultiStrategy::new(vec![multi, pix]));
                #[cfg(all(feature = "telemetry", not(test)))]
                METRIC_ENABLED_STRATEGIES.set(&["non_anonymous_pix"], 1_f64);
            }
            Err(e) => {
                tracing::error!(error = %e, "failed to build NonAnonymousPixStrategy");
            }
        }
    }

    multi
}

fn build_strategies_inner<N>(cfg: &MultiStrategyConfig, node: Arc<N>) -> Box<dyn Strategy + Send>
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
                .build(Arc::clone(&node)),
            ),
            #[cfg(feature = "runtime-tokio")]
            StrategyKind::AutoFunding(sub_cfg) => strategies.push(
                hopr_strategy::auto_funding::AutoFundingStrategy::new(
                    *sub_cfg,
                    cfg.execution_interval,
                )
                .build(Arc::clone(&node)),
            ),
            #[cfg(feature = "runtime-tokio")]
            StrategyKind::ClosureFinalizer(sub_cfg) => strategies.push(
                hopr_strategy::channel_finalizer::ClosureFinalizerStrategy::new(
                    *sub_cfg,
                    cfg.execution_interval,
                )
                .build(Arc::clone(&node)),
            ),
            // ChannelLifecycle owns its own tick cadence via
            // ChannelLifecycleConfig::tick_interval and runs as an independent
            // async task; cfg.execution_interval does not apply to it.
            #[cfg(feature = "runtime-tokio")]
            StrategyKind::ChannelLifecycle(sub_cfg) => strategies.push(
                hopr_strategy::channel_lifecycle::ChannelLifecycleStrategy::new(
                    (**sub_cfg).clone(),
                )
                .build(Arc::clone(&node)),
            ),
            StrategyKind::Multi(sub_cfg) => {
                if cfg.allow_recursive {
                    let mut sub = sub_cfg.clone();
                    sub.allow_recursive = false;
                    strategies.push(build_strategies_inner(&sub, Arc::clone(&node)));
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

    Box::new(MultiStrategy::new(strategies))
}
