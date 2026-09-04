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
    types::crypto::keypairs::ChainKeypair,
};
// The two invariants the `strategy-pix-*` feature set relies on and Cargo cannot state. Both were
// documented in prose in `Cargo.toml` and enforced by nothing; each produced a wrong binary or an
// unrelated error cascade rather than a message naming the rule it broke.
//
// Features are additive, so a third crate in the graph turning on the other pairing is enough to
// reach the first case — and it does not fail, it silently picks the plain pool by the `cfg` precedence
// every block below is written with. The pool decides which address type deposits settle to, so
// the wrong one deposits into an address the Exit can never sweep.
#[cfg(all(feature = "strategy-pix-curvy", feature = "strategy-pix-test"))]
compile_error!(
    "the `strategy-pix-curvy` and `strategy-pix-test` features are mutually exclusive: they \
     select conflicting `hopr-lib/pix-*` features and `HoprPixSpec` has one deposit-address type. \
     Enable exactly one."
);
// `pix` on its own selects no pool, so `hopr_strategy::pix` is not enabled and neither `POOL` nor
// `pool_cfg` below exists. Without this the build fails with a handful of unresolved names that
// say nothing about the feature that is missing.
#[cfg(all(
    feature = "pix",
    not(any(feature = "strategy-pix-curvy", feature = "strategy-pix-test"))
))]
compile_error!(
    "the `pix` feature selects no deposit pool on its own and is not meant to be enabled \
     directly. Enable `strategy-pix-curvy` (production) or `strategy-pix-test` (tests and \
     demo), each of which turns on `pix` as well."
);

// `hopr_strategy::pix` needs one of the `strategy-pix-*` pairings, which is now an independent
// choice from `runtime-tokio` — so the imports and the build block gate on `pix` below rather
// than on the runtime.
#[cfg(feature = "pix")]
use hopr_strategy::pix::strategy::{PixStrategy, PixStrategyConfig};
// `PoolConfig` is per-pool: each pool module exports its own under that name, and the two share no
// fields. Importing the selected one under a single name is what lets the two build blocks below
// differ only in the fields they set.
#[cfg(all(feature = "strategy-pix-curvy", not(feature = "strategy-pix-test")))]
use hopr_strategy::pix::pools::curvy::PoolConfig;
#[cfg(feature = "strategy-pix-test")]
use hopr_strategy::pix::pools::plain::PoolConfig;
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
#[cfg(feature = "strategy-pix-test")]
pub const POOL: &str = "non-anonymous-secp256k1";
#[cfg(all(feature = "strategy-pix-curvy", not(feature = "strategy-pix-test")))]
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
/// anything visible in this file. The plain pool settles with an ordinary `HoprToken.transfer`
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

fn validate_execution_interval(interval: &Duration) -> std::result::Result<(), ValidationError> {
    if interval < &Duration::from_secs(10) {
        Err(ValidationError::new(
            "strategy execution interval must be at least 10 seconds",
        ))
    } else {
        Ok(())
    }
}

/// PIX settlement configuration, as it appears in the `Pix` strategy stanza.
///
/// Two nested sections rather than one flat one. `PixStrategyConfig` is pool-agnostic by
/// design — upstream keeps settlement config out of it so that both pools can be compiled
/// together — and `PoolConfig` is whichever pool this binary's `strategy-pix-*` feature
/// selected. They are also not flattenable: `PixStrategyConfig` carries
/// `deny_unknown_fields`, and `#[serde(flatten)]` routes sibling keys into the flattened
/// type, so `pool` would be rejected as unknown.
///
/// ```yaml
/// strategy:
///   strategies:
///     - Pix:
///         strategy:
///           price_per_byte: "0.0001 wxHOPR"
///           max_ssa_allocation: "10 wxHOPR"
///         pool:
///           max_deposit_tracking_time: 30s
///           gas_xdai_per_sweep: "0.01 xDai"
/// ```
///
/// Both sections may be omitted; each falls back to the upstream defaults documented on the
/// respective type. Note that the accepted keys under `pool` follow the build: a
/// `strategy-pix-curvy` binary has only `max_deposit_tracking_time`, and — because
/// `CurvyDepositPoolConfig` does not set `deny_unknown_fields` — it *ignores* the plain pool's
/// keys rather than rejecting them.
#[cfg(feature = "pix")]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Validate)]
#[serde(default, deny_unknown_fields)]
pub struct PixConfig {
    /// Pool-agnostic settlement knobs: pricing, the per-deposit ceiling, the recovery store,
    /// and the deposit/withdrawal batching windows.
    #[validate(nested)]
    pub strategy: PixStrategyConfig,
    /// The selected deposit pool's own knobs.
    #[validate(nested)]
    pub pool: PoolConfig,
}

/// Stand-in for `PixConfig` in a binary built without a PIX deposit pool.
///
/// (Not an intra-doc link: `PixConfig` needs the `pix` feature, and this type exists only when
/// that feature is off, so the two are never in scope together.)
///
/// Exists only so a `Pix` stanza still *parses* here, which is what lets [`StrategyKind`]'s
/// `Validate` impl answer with the two features that would fix it. Without the variant, serde
/// rejects the stanza as an unknown one and lists the variants that do exist — true, but no
/// help at all to someone who just wants to know why PIX is missing.
///
/// No `deny_unknown_fields`, so it swallows whatever the stanza contained.
#[cfg(not(feature = "pix"))]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PixNotBuilt {}

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
    /// Makes an Exit paid for the traffic it delivers: the Entry deposits to per-Session
    /// stealth addresses, the Exit recovers each key from the shares its spent SURBs carried
    /// and sweeps the deposit into its Safe. Not in [`hopr_default_strategies`] — opt-in.
    #[cfg(feature = "pix")]
    Pix(PixConfig),
    /// See [`PixNotBuilt`]: parses so that validation can explain itself.
    #[cfg(not(feature = "pix"))]
    Pix(PixNotBuilt),
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
            #[cfg(feature = "pix")]
            Self::Pix(cfg) => cfg.validate(),
            // The stanza asks for a strategy this binary cannot provide. Refusing here stops
            // the node before it starts rather than letting it relay with no deposit path.
            #[cfg(not(feature = "pix"))]
            Self::Pix(_) => {
                let mut errors = validator::ValidationErrors::new();
                errors.add(
                    "Pix",
                    ValidationError::new(
                        "the configuration contains a `Pix` strategy but this binary was built \
                         without a PIX deposit pool. Rebuild with `--features \
                         strategy-pix-curvy` (production) or `--features \
                         strategy-pix-test` (tests and demo).",
                    ),
                );
                Err(errors)
            }
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
/// PIX is one of those variants ([`StrategyKind::Pix`]) rather than a special case. It used
/// to be enabled by `HOPRD_ENABLE_PIX=1` and tuned by four more environment variables,
/// because a [`hopr_lib::api::types::primitive::prelude::HoprBalance`] had no readable serde
/// form — it serialized as a positional `[U256, currency]` pair, so `1 wxHOPR` was not
/// something a config file could say. `hopr-strategy` 1.0.1 gave those fields
/// `DisplayFromStr` and the special case went away with them.
///
/// External strategies can be composed by building this result first, then wrapping
/// it with additional strategies in a new `MultiStrategy::new(...)` call at the
/// call site.
///
/// `chain_key` is the node's own chain keypair. Only the `strategy-pix-test` pairing reads
/// it: since `hopr-types` 4.0.0 routes `SafePayloadGenerator::transfer` through the Safe module,
/// the plain pool signs its sweeps and gas top-ups with short-lived EOA connectors instead, and
/// the top-up is the one movement the *node* pays for. It is taken unconditionally so that the
/// signature does not move with the feature set.
pub fn build_strategies<N>(
    cfg: &MultiStrategyConfig,
    node: Arc<N>,
    chain_key: &ChainKeypair,
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

    build_strategies_inner(cfg, node, chain_key)
}

// `chain_key` is unread unless the plain pool is compiled in; see `build_strategies`.
// Without the plain pool there is no `build_non_anonymous` call, so `chain_key` reaches nothing
// but the recursive call below — which is what both of these lints are pointing at. Gated on the
// feature rather than blanket-allowed, so the day something else here needs the key, an unused
// one is still caught in the builds that have it.
#[cfg_attr(
    not(feature = "strategy-pix-test"),
    allow(
        unused_variables,
        clippy::only_used_in_recursion,
        reason = "only the plain PIX pool signs with the node key"
    )
)]
fn build_strategies_inner<N>(
    cfg: &MultiStrategyConfig,
    node: Arc<N>,
    chain_key: &ChainKeypair,
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

    // `build` is fallible in hopr-strategy 1.0: it validates the strategy's own config and returns
    // `StrategyError::InvalidConfiguration` rather than constructing something that cannot work.
    // The `?` propagates that out through `build_strategies` to `hoprd::run`, so a strategy stanza
    // that cannot be honoured stops the node from starting rather than being silently dropped.
    // PIX included: a stanza asking for the strategy that makes an Exit paid, on a node that then
    // relays with no deposit path, is the one failure that must not be survivable.
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
            #[cfg(feature = "pix")]
            StrategyKind::Pix(sub_cfg) => {
                // The pool is a build-time choice with no runtime trace, and the localcluster
                // runs a prebuilt binary — so without this line there is nothing in a log to say
                // which pool a running node actually has. `POOL` is the name the test harness
                // asserts against.
                tracing::info!(
                    pool = POOL,
                    price_per_byte = %sub_cfg.strategy.price_per_byte,
                    max_ssa_allocation = %sub_cfg.strategy.max_ssa_allocation,
                    max_deposit_tracking_time = ?sub_cfg.pool.max_deposit_tracking_time,
                    "enabling the PIX strategy"
                );
                // One builder per pool rather than one call that dispatches: the pool is named
                // here, and `SpecDepositAddress` is the witness that it can settle what this
                // build's spec produces. The generic `build_with_pool` exists for supplying a
                // pool from outside this crate.
                #[cfg(feature = "strategy-pix-test")]
                let built = PixStrategy::new(sub_cfg.strategy.clone())
                    .build_non_anonymous::<_, SpecDepositAddress>(
                        Arc::clone(&node),
                        chain_key.clone(),
                        sub_cfg.pool.clone(),
                    )?;
                #[cfg(all(feature = "strategy-pix-curvy", not(feature = "strategy-pix-test")))]
                let built = PixStrategy::new(sub_cfg.strategy.clone())
                    .build_curvy::<_, SpecDepositAddress>(
                        Arc::clone(&node),
                        sub_cfg.pool.clone(),
                    )?;
                strategies.push(built);
            }
            // Unreachable: `StrategyKind::validate` rejects this stanza before a node with it in
            // the config can start. Matched anyway so the arm list stays exhaustive.
            #[cfg(not(feature = "pix"))]
            StrategyKind::Pix(_) => anyhow::bail!(
                "the configuration contains a `Pix` strategy but this binary was built without a \
                 PIX deposit pool. Rebuild with `--features strategy-pix-curvy` (production) or \
                 `--features strategy-pix-test` (tests and demo)."
            ),
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
                    strategies.push(build_strategies_inner(&sub, Arc::clone(&node), chain_key)?);
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

    /// The `Pix` stanza in the form the docs promise. This is the whole point of the change:
    /// before `hopr-strategy` 1.0.1 a balance was a positional `[U256, currency]` pair and
    /// `price_per_byte: 0.0001 wxHOPR` failed to parse with "expected sequence start".
    #[cfg(feature = "pix")]
    #[test]
    fn pix_stanza_parses_from_yaml() -> anyhow::Result<()> {
        let cfg: MultiStrategyConfig = serde_saphyr::from_str(
            r#"
strategies:
  - Pix:
      strategy:
        price_per_byte: 0.0001 wxHOPR
        max_ssa_allocation: 10 wxHOPR
      pool:
        max_deposit_tracking_time: 30s
"#,
        )?;

        let StrategyKind::Pix(pix) = &cfg.strategies[0] else {
            anyhow::bail!("expected a Pix stanza, got {:?}", cfg.strategies[0]);
        };
        assert_eq!(pix.strategy.price_per_byte, "0.0001 wxHOPR".parse()?);
        assert_eq!(pix.strategy.max_ssa_allocation, "10 wxHOPR".parse()?);
        assert_eq!(pix.pool.max_deposit_tracking_time, Duration::from_secs(30));
        Ok(())
    }

    /// Both halves are optional, and omitting them must not silently zero anything. The
    /// buffer periods are the ones to watch: until 1.0.1 they carried a bare
    /// `#[serde(default)]`, so the deserialize path produced `Duration::default()` — 0ns —
    /// rather than the 500ms the type's own `Default` documents.
    #[cfg(feature = "pix")]
    #[test]
    fn omitted_pix_sections_fall_back_to_documented_defaults() -> anyhow::Result<()> {
        let cfg: MultiStrategyConfig = serde_saphyr::from_str("strategies:\n  - Pix: {}\n")?;

        let StrategyKind::Pix(pix) = &cfg.strategies[0] else {
            anyhow::bail!("expected a Pix stanza, got {:?}", cfg.strategies[0]);
        };
        assert_eq!(pix.strategy.price_per_byte, "1 wxHOPR".parse()?);
        assert_eq!(pix.strategy.max_ssa_allocation, "100 wxHOPR".parse()?);
        assert_eq!(
            pix.strategy.deposit_buffer_period,
            Duration::from_millis(500),
            "a bare serde(default) here would give 0ns and disable debouncing"
        );
        assert_eq!(
            pix.strategy.withdrawal_buffer_period,
            Duration::from_millis(500)
        );
        assert_eq!(pix.pool, Default::default());
        Ok(())
    }

    /// `deny_unknown_fields` upstream: a typo has to be an error, not a value that looks set
    /// and is not.
    #[cfg(feature = "pix")]
    #[test]
    fn a_misspelled_pix_key_is_rejected() {
        let err = serde_saphyr::from_str::<MultiStrategyConfig>(
            "strategies:\n  - Pix:\n      strategy:\n        price_per_bite: 1 wxHOPR\n",
        );
        assert!(err.is_err(), "expected a parse error, got {err:?}");
    }

    /// The stanza has to survive a write-read cycle, since `hoprd-cfg` dumps the running
    /// config and the localcluster generates node configs the same way.
    #[cfg(feature = "pix")]
    #[test]
    fn pix_stanza_round_trips() -> anyhow::Result<()> {
        let before = MultiStrategyConfig {
            strategies: vec![StrategyKind::Pix(PixConfig::default())],
            ..hopr_default_strategies()
        };
        let after: MultiStrategyConfig =
            serde_saphyr::from_str(&serde_saphyr::to_string(&before)?)?;
        assert_eq!(before, after);
        Ok(())
    }

    /// A pool-less binary must reject the stanza rather than start without the strategy it
    /// names. The variant exists only so the message can say *which features* would provide
    /// it; serde's own "unknown variant" error would not.
    #[cfg(not(feature = "pix"))]
    #[test]
    fn a_pix_stanza_is_refused_by_a_binary_without_a_pool() -> anyhow::Result<()> {
        let cfg: MultiStrategyConfig = serde_saphyr::from_str(
            "strategies:\n  - Pix:\n      strategy:\n        price_per_byte: 1 wxHOPR\n",
        )?;

        let err = cfg
            .validate()
            .expect_err("a Pix stanza must not validate without a deposit pool");
        let rendered = format!("{err:?}");
        assert!(
            rendered.contains("strategy-pix-test"),
            "the error should name the features that fix it, got {rendered}"
        );
        Ok(())
    }
}
