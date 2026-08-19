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
// `hopr_strategy::pix` needs one of the `strategy-pix-*` pairings, which is now an independent
// choice from `runtime-tokio` — so the imports and the build block gate on `pix` below rather
// than on the runtime.
#[cfg(feature = "pix")]
use hopr_strategy::pix::strategy::{PixStrategy, PixStrategyConfig};
// `PoolConfig` is per-pool: each pool module exports its own under that name, and the two share no
// fields. Importing the selected one under a single name is what lets the two build blocks below
// differ only in the fields they set.
#[cfg(all(
    feature = "strategy-pix-curvy",
    not(feature = "strategy-pix-secp256k1")
))]
use hopr_strategy::pix::curvy::PoolConfig;
#[cfg(feature = "strategy-pix-secp256k1")]
use hopr_strategy::pix::secp256k1::PoolConfig;
use hopr_strategy::strategy::{MultiStrategy, Strategy};
use serde::{Deserialize, Serialize};

use smart_default::SmartDefault;
use strum::{Display as StrumDisplay, VariantNames};
use validator::{Validate, ValidationError};

/// The deposit pool this binary was built with, for the startup log line.
///
/// Every other guarantee here is compile-time, and the localcluster launches a *prebuilt* binary
/// from `HOPRD_BIN` — so a stale artifact built with the other pairing would run with none of
/// those checks having seen it. This is the one thing that makes the choice visible at runtime,
/// and `pix-demo.sh` greps the binary for it.
#[cfg(feature = "strategy-pix-secp256k1")]
pub const POOL: &str = "non-anonymous-secp256k1";
#[cfg(all(
    feature = "strategy-pix-curvy",
    not(feature = "strategy-pix-secp256k1")
))]
pub const POOL: &str = "curvy";

/// The deposit address type this build's `HoprPixSpec` produces.
///
/// Naming this in the `build_*` call below *is* the assertion that the spec and the selected pool
/// agree: each builder is bound on `A: DepositAddressOf<PoolKeypair>`, which holds only for the
/// address its own pool settles to. A mismatched pairing therefore stops at that call site with a
/// message naming both the offending type and the two features that fix it, instead of failing
/// once per event at runtime having deposited nothing.
///
/// Which instantiation of `HoprPixSpec` is in play is decided by the *feature graph*, not by
/// anything visible in this file. The secp256k1 pool settles with a plain `HoprToken.transfer`
/// signed by the node key, so it can only reach an Ethereum address; a Baby JubJub public key is
/// a curve point, not an account, and no transfer can reach one.
///
/// That combination has already cost a day. hoprnet 27b4b255f9 enabled QUIC by default and, as
/// collateral, dropped `default-features = false` from two workspace dependencies whose `default`
/// set contains `bjj`. Cargo features being additive, every deposit address downstream silently
/// became a curve point. Nothing failed to build. At runtime the only symptom was
/// `pix event failed: input argument to the function is invalid` and a strategy that never
/// deposited — indistinguishable, from the outside, from a Session that had simply stalled.
///
/// This replaces a free-standing `const _: () = { … }` assertion against `PoolKeypair::Public`.
/// The two checked the same thing, but a separate assertion can drift out of step with the
/// builder actually called; passing the witness to the builder cannot, because the assertion and
/// the choice are then one expression. It costs nothing at runtime — the parameter appears only
/// in a bound.
#[cfg(feature = "pix")]
type SpecDepositAddress =
    <hopr_lib::exports::transport::HoprPixSpec as hopr_lib::exports::transport::PixSpec>::DepositAddress;

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
#[cfg(feature = "pix")]
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
#[cfg(feature = "pix")]
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
                    funding: FundingConfig {
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
pub fn build_strategies<N>(
    cfg: &MultiStrategyConfig,
    node: Arc<N>,
) -> anyhow::Result<Box<dyn Strategy + Send>>
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

    // `mut` only when a PIX pool is selected — the block below is the sole mutator, and it is
    // gated on `pix`. A no-pool build genuinely never writes to this, so the lint is right there.
    #[cfg_attr(not(feature = "pix"), allow(unused_mut))]
    let mut multi = build_strategies_inner(cfg, Arc::clone(&node))?;

    // PIX is not a YAML-configurable StrategyKind because its
    // HoprBalance fields don't round-trip through serde_saphyr. Instead it's
    // enabled via environment variable for test/development use.
    #[cfg(feature = "pix")]
    if std::env::var("HOPRD_ENABLE_PIX")
        .ok()
        .is_some_and(|v| v == "1" || v.eq_ignore_ascii_case("true"))
    {
        // Built once per pool rather than once with a `cfg`-ed field, because the two configs
        // share no fields by contract — the pools settle by different means, so neither one's
        // knobs are evidence that the other needs them. Writing them separately is what stops a
        // value meant for one from silently reaching the other; a shared literal would make the
        // overlap load-bearing and quietly break when either config moves.
        //
        // `gas_xdai_per_sweep` is the clearest case: it funds a recovered stealth address so it
        // can pay for its own `withdraw_from_signer` transaction, which is a fact about settling
        // on-chain from an EOA. Reading `HOPRD_PIX_GAS_XDAI_PER_SWEEP` under the other pool and
        // dropping it would be exactly the silent misconfiguration this arrangement exists to
        // avoid, so that variable is only read here.
        //
        // Retry budgets are left at the upstream defaults: this block exists to wire the handful
        // of values hoprd exposes as environment variables, and every other field is better
        // served by whatever upstream currently documents.
        #[cfg(feature = "strategy-pix-secp256k1")]
        let pool_cfg = PoolConfig {
            max_deposit_tracking_time: pix_env_duration_or(
                "HOPRD_PIX_MAX_DEPOSIT_TRACKING_TIME",
                Duration::from_secs(3600),
            ),
            // Not `Default::default()`: `Balance<XDai>::default()` is zero, which makes
            // `fund_sweep_gas_impl` a no-op and leaves the address without gas.
            gas_xdai_per_sweep: pix_env_or(
                "HOPRD_PIX_GAS_XDAI_PER_SWEEP",
                "0.01 xdai".parse().expect("valid static amount"),
            ),
            ..Default::default()
        };
        // The curvy pool's settlement design is not written, so it has nothing to configure
        // beyond the deadline the `DepositPool` contract makes it own. New knobs belong here,
        // read from their own variables — not borrowed from the block above.
        #[cfg(all(
            feature = "strategy-pix-curvy",
            not(feature = "strategy-pix-secp256k1")
        ))]
        let pool_cfg = PoolConfig {
            max_deposit_tracking_time: pix_env_duration_or(
                "HOPRD_PIX_MAX_DEPOSIT_TRACKING_TIME",
                Duration::from_secs(3600),
            ),
        };

        let pix_cfg = PixStrategyConfig {
            price_per_byte: pix_env_or(
                "HOPRD_PIX_PRICE_PER_BYTE",
                "1 wxHOPR".parse().expect("valid static amount"),
            ),
            max_ssa_allocation: pix_env_or(
                "HOPRD_PIX_MAX_SSA_ALLOCATION",
                "100 wxHOPR".parse().expect("valid static amount"),
            ),
            pix_recovery_db_path: None,
            pix_recovery_password_env: None,
            // Likewise the deposit/withdrawal batching windows.
            ..Default::default()
        };
        // The pool is a build-time choice with no runtime trace, and the localcluster runs a
        // prebuilt binary — so without this line there is nothing in a log to say which pool a
        // running node actually has. `POOL` is the name the test harness asserts against.
        tracing::info!(
            pool = POOL,
            price_per_byte = %pix_cfg.price_per_byte,
            max_ssa_allocation = %pix_cfg.max_ssa_allocation,
            max_deposit_tracking_time = ?pool_cfg.max_deposit_tracking_time,
            "enabling the PIX strategy"
        );
        // One builder per pool rather than one call that dispatches: the pool is named here, and
        // `SpecDepositAddress` is the witness that it can settle what this build's spec produces.
        // The generic `build_with_pool` exists for supplying a pool from outside this crate.
        #[cfg(feature = "strategy-pix-secp256k1")]
        let built = PixStrategy::new(pix_cfg)
            .build_non_anonymous::<_, SpecDepositAddress>(Arc::clone(&node), pool_cfg);
        #[cfg(all(
            feature = "strategy-pix-curvy",
            not(feature = "strategy-pix-secp256k1")
        ))]
        let built = PixStrategy::new(pix_cfg)
            .build_curvy::<_, SpecDepositAddress>(Arc::clone(&node), pool_cfg);

        match built {
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

    // Without a `strategy-pix-*` pairing there is no pool to build, so the block above is
    // compiled out entirely and `HOPRD_ENABLE_PIX=1` would otherwise do nothing at all — no
    // strategy, no log line, no error. Silence is the one outcome this must not have.
    #[cfg(not(feature = "pix"))]
    if std::env::var("HOPRD_ENABLE_PIX")
        .ok()
        .is_some_and(|v| v == "1" || v.eq_ignore_ascii_case("true"))
    {
        tracing::error!(
            "HOPRD_ENABLE_PIX is set but this binary was built without a PIX deposit pool, so \
             the PIX strategy is not available. Rebuild with `--features strategy-pix-curvy` \
             (production) or `--features strategy-pix-secp256k1` (tests and demo)."
        );
    }

    Ok(multi)
}

fn build_strategies_inner<N>(
    cfg: &MultiStrategyConfig,
    node: Arc<N>,
) -> anyhow::Result<Box<dyn Strategy + Send>>
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
        + PacketTransport
        + Send
        + Sync
        + 'static,
{
    let mut strategies = Vec::<Box<dyn Strategy + Send>>::new();

    // `build` is fallible in hopr-strategy 1.0: it validates the strategy's own config and returns
    // `StrategyError::InvalidConfiguration` rather than constructing something that cannot work.
    // The `?` propagates that out through `build_strategies` to `hoprd::run`, so a strategy stanza
    // that cannot be honoured stops the node from starting rather than being silently dropped.
    //
    // The PIX block in `build_strategies` deliberately does *not* do this. Its configuration comes
    // from environment variables rather than the YAML strategy list, and the surrounding
    // `pix_env_or` policy is already log-and-default; aborting startup over one malformed variable
    // would be a different contract from the one that block's other values have.
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
