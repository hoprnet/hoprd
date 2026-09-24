//! Backwards-compatible deserialization of the `ChannelLifecycle` strategy config.
//!
//! `hopr-strategy` rejects unknown keys, so a config file that still carries a
//! `funding` option removed upstream would stop the node from starting. This module
//! mirrors [`ChannelLifecycleConfig`] for deserialization only, accepts the removed
//! options as deprecated, logs a warning for each one set, and discards them.
//!
//! Serialization is not affected: [`StrategyKind`](super::StrategyKind) serializes the
//! upstream type, so a re-emitted config never contains the deprecated options.

use std::time::Duration;

use bytesize::ByteSize;
use hopr_strategy::channel_lifecycle::{
    CapacitySizingMode, ChannelLifecycleConfig, ClosureConfig, ConcurrencyConfig,
    EligibilityConfig, FinalizerConfig, FundingConfig, PopulationConfig, ProactiveFundingConfig,
    RestartGuardConfig, SelectorProfile,
};
use serde::{Deserialize, Deserializer};

/// Deserializes a [`ChannelLifecycleConfig`], tolerating the deprecated `funding` options.
pub(super) fn deserialize_channel_lifecycle<'de, D>(
    deserializer: D,
) -> Result<Box<ChannelLifecycleConfig>, D::Error>
where
    D: Deserializer<'de>,
{
    let cfg = LegacyChannelLifecycleConfig::deserialize(deserializer)?;

    for key in cfg.funding.deprecated_keys_set() {
        tracing::warn!(
            key,
            "ignoring deprecated channel lifecycle funding option: it has no effect and should \
             be removed from the configuration"
        );
    }

    Ok(Box::new(cfg.into()))
}

/// Deserialization mirror of [`ChannelLifecycleConfig`] whose only difference is the
/// `funding` section.
///
/// Conversions to and from the upstream type list every field explicitly, so a field
/// added upstream fails compilation here instead of being silently dropped.
#[derive(Deserialize)]
#[serde(default, deny_unknown_fields)]
struct LegacyChannelLifecycleConfig {
    #[serde(with = "humantime_serde")]
    tick_interval: Duration,
    #[serde(with = "humantime_serde")]
    jitter: Duration,
    population: PopulationConfig,
    eligibility: EligibilityConfig,
    funding: LegacyFundingConfig,
    proactive_funding: ProactiveFundingConfig,
    closure: ClosureConfig,
    finalizer: FinalizerConfig,
    restart: RestartGuardConfig,
    concurrency: ConcurrencyConfig,
    selector: SelectorProfile,
}

impl Default for LegacyChannelLifecycleConfig {
    fn default() -> Self {
        ChannelLifecycleConfig::default().into()
    }
}

impl From<ChannelLifecycleConfig> for LegacyChannelLifecycleConfig {
    fn from(cfg: ChannelLifecycleConfig) -> Self {
        let ChannelLifecycleConfig {
            tick_interval,
            jitter,
            population,
            eligibility,
            funding,
            proactive_funding,
            closure,
            finalizer,
            restart,
            concurrency,
            selector,
        } = cfg;

        Self {
            tick_interval,
            jitter,
            population,
            eligibility,
            funding: funding.into(),
            proactive_funding,
            closure,
            finalizer,
            restart,
            concurrency,
            selector,
        }
    }
}

impl From<LegacyChannelLifecycleConfig> for ChannelLifecycleConfig {
    fn from(cfg: LegacyChannelLifecycleConfig) -> Self {
        Self {
            tick_interval: cfg.tick_interval,
            jitter: cfg.jitter,
            population: cfg.population,
            eligibility: cfg.eligibility,
            funding: cfg.funding.into(),
            proactive_funding: cfg.proactive_funding,
            closure: cfg.closure,
            finalizer: cfg.finalizer,
            restart: cfg.restart,
            concurrency: cfg.concurrency,
            selector: cfg.selector,
        }
    }
}

/// [`FundingConfig`] plus the options `hopr-strategy` no longer accepts.
#[derive(Deserialize)]
#[serde(default, deny_unknown_fields)]
struct LegacyFundingConfig {
    initial_capacity: ByteSize,
    topup_capacity: ByteSize,
    lower_capacity_threshold: ByteSize,
    sizing_mode: CapacitySizingMode,

    /// Replaced by a safe balance requirement derived from live channel demand.
    #[deprecated(note = "has no effect: the required safe balance is derived from live demand")]
    min_safe_capacity_required: Option<ByteSize>,

    /// Replaced by fund and open passes that each gate on exactly what they spend.
    #[deprecated(note = "has no effect: funding passes gate on exactly what they spend")]
    stop_when_unfunded: Option<bool>,
}

impl LegacyFundingConfig {
    /// Names of the deprecated options present in the parsed config.
    #[allow(deprecated)]
    fn deprecated_keys_set(&self) -> Vec<&'static str> {
        let mut keys = Vec::new();
        if self.min_safe_capacity_required.is_some() {
            keys.push("min_safe_capacity_required");
        }
        if self.stop_when_unfunded.is_some() {
            keys.push("stop_when_unfunded");
        }
        keys
    }
}

impl Default for LegacyFundingConfig {
    fn default() -> Self {
        FundingConfig::default().into()
    }
}

impl From<FundingConfig> for LegacyFundingConfig {
    #[allow(deprecated)]
    fn from(cfg: FundingConfig) -> Self {
        let FundingConfig {
            initial_capacity,
            topup_capacity,
            lower_capacity_threshold,
            sizing_mode,
        } = cfg;

        Self {
            initial_capacity,
            topup_capacity,
            lower_capacity_threshold,
            sizing_mode,
            min_safe_capacity_required: None,
            stop_when_unfunded: None,
        }
    }
}

impl From<LegacyFundingConfig> for FundingConfig {
    fn from(cfg: LegacyFundingConfig) -> Self {
        Self {
            initial_capacity: cfg.initial_capacity,
            topup_capacity: cfg.topup_capacity,
            lower_capacity_threshold: cfg.lower_capacity_threshold,
            sizing_mode: cfg.sizing_mode,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn funding(yaml: &str) -> anyhow::Result<LegacyFundingConfig> {
        Ok(serde_saphyr::from_str::<LegacyFundingConfig>(yaml)?)
    }

    #[test]
    fn deprecated_keys_set_is_empty_without_deprecated_options() -> anyhow::Result<()> {
        let cfg = funding("initial_capacity: \"1 GiB\"\n")?;

        assert!(cfg.deprecated_keys_set().is_empty());

        Ok(())
    }

    #[test]
    fn deprecated_keys_set_names_each_deprecated_option() -> anyhow::Result<()> {
        let cfg = funding("min_safe_capacity_required: \"512 MiB\"\nstop_when_unfunded: false\n")?;

        assert_eq!(
            cfg.deprecated_keys_set(),
            vec!["min_safe_capacity_required", "stop_when_unfunded"]
        );

        Ok(())
    }
}
