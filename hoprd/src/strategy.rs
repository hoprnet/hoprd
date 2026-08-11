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
// `hopr_strategy::pix` lives behind `strategy-pix`, which `runtime-tokio` turns on — the
// same gate as the block that uses these, so the import has to carry it too.
#[cfg(feature = "runtime-tokio")]
use hopr_strategy::pix::{
    non_anonymous_pool::NonAnonymousDepositPoolConfig,
    strategy::{PixStrategy, PixStrategyConfig},
};
// Gated on `strategy-pix` rather than `runtime-tokio`, like the assertion that uses it:
// the former is implied by the latter, not the other way round.
#[cfg(feature = "strategy-pix")]
use hopr_strategy::pix::PoolKeypair;
use hopr_strategy::strategy::{MultiStrategy, Strategy};
use serde::{Deserialize, Serialize};

use smart_default::SmartDefault;
use strum::{Display as StrumDisplay, VariantNames};
use validator::{Validate, ValidationError};

/// The deposit address the PIX spec produces must be the one the selected pool can spend.
///
/// Gated on the same feature as the pool itself, so the assertion exists exactly when the thing
/// it constrains does. `strategy-pix` enables `hopr-lib/pix-secp256k1`, which is what currently
/// makes this hold — this is the backstop for the spec being flipped by something other than
/// that line.
///
/// Which instantiation of `HoprPixSpec` is in play is decided by the *feature graph*, not by
/// anything visible in this file. Today's pool settles with a plain `HoprToken.transfer` signed
/// by the node key, so it can only reach an Ethereum address; a Baby JubJub public key is a curve
/// point, not an account, and no transfer can reach one.
///
/// That combination has already cost a day. hoprnet 27b4b255f9 enabled QUIC by default and, as
/// collateral, dropped `default-features = false` from two workspace dependencies whose `default`
/// set contains `bjj`. Cargo features being additive, every deposit address downstream silently
/// became a curve point. Nothing failed to build. At runtime the only symptom was
/// `pix event failed: input argument to the function is invalid` and a strategy that never
/// deposited — indistinguishable, from the outside, from a Session that had simply stalled.
///
/// Stated against [`PoolKeypair`] rather than against [`Address`] directly, the invariant is
/// *which curve the pool is for* rather than *secp256k1*, so it keeps holding unedited when a
/// Baby JubJub pool is wired in and both sides move together. What it rejects is the two sides
/// moving apart, which is the failure that actually happened.
///
/// It costs nothing at runtime: the function is never called, only type-checked.
///
/// [`Address`]: hopr_lib::api::types::primitive::prelude::Address
#[cfg(feature = "strategy-pix")]
const _: () = {
    type SpecDepositAddress =
        <hopr_lib::exports::transport::HoprPixSpec as hopr_lib::exports::transport::PixSpec>::DepositAddress;
    type PoolDepositAddress =
        <PoolKeypair as hopr_lib::api::types::crypto::prelude::Keypair>::Public;

    #[allow(dead_code)]
    fn pix_spec_and_pool_must_agree_on_the_deposit_address(
        a: SpecDepositAddress,
    ) -> PoolDepositAddress {
        a
    }
};

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

/// Reads `var` and parses it as `T`, falling back to `default` when the variable is
/// unset or does not parse.
///
/// Used for the PIX knobs, which cannot be expressed in YAML (see
/// [`build_strategies`]). A malformed value is a configuration mistake rather than a
/// reason to refuse to start, so it is logged and the default is kept.
#[cfg(feature = "runtime-tokio")]
fn pix_env_or<T>(var: &str, default: T) -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    match std::env::var(var) {
        Ok(raw) => raw.trim().parse().unwrap_or_else(|error| {
            tracing::warn!(%error, var, %raw, "invalid PIX override, keeping the default");
            default
        }),
        Err(_) => default,
    }
}

/// [`pix_env_or`] for [`Duration`], which has no `FromStr`; accepts humantime syntax
/// such as `30s` or `2m`.
#[cfg(feature = "runtime-tokio")]
fn pix_env_duration_or(var: &str, default: Duration) -> Duration {
    match std::env::var(var) {
        Ok(raw) => humantime_serde::re::humantime::parse_duration(raw.trim()).unwrap_or_else(
            |error| {
                tracing::warn!(%error, var, %raw, "invalid PIX duration override, keeping the default");
                default
            },
        ),
        Err(_) => default,
    }
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
/// PIX strategy is added programmatically (it is intentionally
/// not a YAML-configurable [`StrategyKind`] because its config type's serde
/// representation is incompatible with `serde_saphyr`).
///
/// For the same reason its knobs are read from the environment rather than from the
/// config file. All are optional and fall back to the defaults shown:
///
/// | Variable | Default |
/// |---|---|
/// | `HOPRD_PIX_PRICE_PER_BYTE` | `1 wxHOPR` |
/// | `HOPRD_PIX_MAX_SSA_ALLOCATION` | `100 wxHOPR` |
/// | `HOPRD_PIX_MAX_DEPOSIT_TRACKING_TIME` | `1h` |
/// | `HOPRD_PIX_GAS_XDAI_PER_SWEEP` | `0.01 xdai` |
///
/// Note that `max_deposit_tracking_time` drives the Exit's deposit poll cadence
/// (`tracking_time / 10`), which must stay below the Exit's
/// `max_deposit_wait + max_ssa_delivery_time` kill-switch deadline — otherwise only the
/// single immediate balance check can land in time.
///
/// External strategies can be composed by building this result first, then wrapping
/// it with additional strategies in a new `MultiStrategy::new(...)` call at the
/// call site.
pub fn build_strategies<N>(cfg: &MultiStrategyConfig, node: Arc<N>) -> Box<dyn Strategy + Send>
where
    N: ActionableEventSource
        + PacketTransport
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

    // PIX is not a YAML-configurable StrategyKind because its
    // HoprBalance fields don't round-trip through serde_saphyr. Instead it's
    // enabled via environment variable for test/development use.
    #[cfg(feature = "runtime-tokio")]
    if std::env::var("HOPRD_ENABLE_PIX")
        .ok()
        .is_some_and(|v| v == "1" || v.eq_ignore_ascii_case("true"))
    {
        let pix_cfg = PixStrategyConfig {
            price_per_byte: pix_env_or(
                "HOPRD_PIX_PRICE_PER_BYTE",
                "1 wxHOPR".parse().expect("valid static amount"),
            ),
            max_ssa_allocation: pix_env_or(
                "HOPRD_PIX_MAX_SSA_ALLOCATION",
                "100 wxHOPR".parse().expect("valid static amount"),
            ),
            pool: NonAnonymousDepositPoolConfig {
                max_deposit_tracking_time: pix_env_duration_or(
                    "HOPRD_PIX_MAX_DEPOSIT_TRACKING_TIME",
                    Duration::from_secs(3600),
                ),
                // Not `Default::default()`: `Balance<XDai>::default()` is zero, which makes
                // `fund_sweep_gas_impl` a no-op and leaves the recovered stealth address
                // without gas to pay for its own `withdraw_from_signer` sweep.
                gas_xdai_per_sweep: pix_env_or(
                    "HOPRD_PIX_GAS_XDAI_PER_SWEEP",
                    "0.01 xdai".parse().expect("valid static amount"),
                ),
                // Retry budgets are left at the upstream defaults: this block exists to wire
                // the handful of values hoprd exposes as environment variables, and every
                // other field is better served by whatever upstream currently documents.
                ..Default::default()
            },
            pix_recovery_db_path: None,
            pix_recovery_password_env: None,
            // Likewise the deposit/withdrawal batching windows.
            ..Default::default()
        };
        tracing::info!(
            price_per_byte = %pix_cfg.price_per_byte,
            max_ssa_allocation = %pix_cfg.max_ssa_allocation,
            max_deposit_tracking_time = ?pix_cfg.pool.max_deposit_tracking_time,
            gas_xdai_per_sweep = %pix_cfg.pool.gas_xdai_per_sweep,
            "enabling the PIX strategy"
        );
        // `build_non_anonymous` picks the default on-chain deposit pool; the generic
        // `build_with_pool` exists for alternative (anonymous) pool implementations.
        match PixStrategy::new(pix_cfg).build_non_anonymous(Arc::clone(&node)) {
            Ok(pix) => {
                multi = Box::new(MultiStrategy::new(vec![multi, pix]));
                #[cfg(all(feature = "telemetry", not(test)))]
                METRIC_ENABLED_STRATEGIES.set(&["pix"], 1_f64);
            }
            Err(e) => {
                tracing::error!(error = %e, "failed to build PixStrategy");
            }
        }
    }

    multi
}

fn build_strategies_inner<N>(cfg: &MultiStrategyConfig, node: Arc<N>) -> Box<dyn Strategy + Send>
where
    N: ActionableEventSource
        + PacketTransport
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
