use std::time::Duration;

use hopr_lib::{
    api::types::{internal::tickets::WinningProbability, primitive::prelude::HoprBalance},
    config::{
        HoprLibConfig, HoprPacketPipelineConfig, HostConfig, HostType, MixerConfig, ProbeConfig,
        SafeModule, SessionGlobalConfig, TransportConfig,
    },
    exports::transport::{HoprProtocolConfig, TagAllocatorConfig, config::HoprCodecConfig},
};
use hopr_reference::config::SessionIpForwardingConfig;
use hoprd_api::config::{Api, Auth};
use proc_macro_regex::regex;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError, ValidationErrors};

pub const DEFAULT_HOST: &str = "0.0.0.0";
pub const DEFAULT_PORT: u16 = 9091;

// Validate that the path is a valid UTF-8 path.
//
// Also used to perform the identity file existence check on the
// specified path, which is now circumvented but could
// return in the future workflows of setting up a node.
fn validate_file_path(_s: &str) -> Result<(), ValidationError> {
    Ok(())

    // if std::path::Path::new(_s).is_file() {
    //     Ok(())
    // } else {
    //     Err(ValidationError::new(
    //         "Invalid file path specified, the file does not exist or is not a file",
    //     ))
    // }
}

fn validate_password(s: &str) -> Result<(), ValidationError> {
    if !s.is_empty() {
        Ok(())
    } else {
        Err(ValidationError::new("No password could be found"))
    }
}

regex!(is_private_key "^(0[xX])?[a-fA-F0-9]{128}$");

pub(crate) fn validate_private_key(s: &str) -> Result<(), ValidationError> {
    if is_private_key(s) {
        Ok(())
    } else {
        Err(ValidationError::new("No valid private key could be found"))
    }
}

fn validate_optional_private_key(s: &str) -> Result<(), ValidationError> {
    validate_private_key(s)
}

// Ensures the node has an identity source to load or create keys from: either a
// non-empty identity file path or a private key. Without this, an empty file path
// (the default) only surfaces as an opaque filesystem error at keypair creation.
fn validate_identity_source(identity: &Identity) -> Result<(), ValidationError> {
    let has_file = !identity.file.trim().is_empty();
    let has_key = identity
        .private_key
        .as_ref()
        .is_some_and(|k| !k.trim().is_empty());

    if has_file || has_key {
        return Ok(());
    }

    let mut error = ValidationError::new("identity_source_missing");
    error.message = Some(
        "no identity source configured: provide an identity file via --identity \
         (HOPRD_IDENTITY) or a private key via --privateKey (HOPRD_PRIVATE_KEY)"
            .into(),
    );
    Err(error)
}

#[derive(Default, Serialize, Deserialize, Validate, Clone, PartialEq)]
#[validate(schema(function = "validate_identity_source", skip_on_field_errors = false))]
#[serde(deny_unknown_fields)]
pub struct Identity {
    #[validate(custom(function = "validate_file_path"))]
    #[serde(default)]
    pub file: String,
    #[validate(custom(function = "validate_password"))]
    #[serde(default)]
    pub password: String,
    #[validate(custom(function = "validate_optional_private_key"))]
    #[serde(default)]
    pub private_key: Option<String>,
}

impl std::fmt::Debug for Identity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let obfuscated: String = "<REDACTED>".into();

        f.debug_struct("Identity")
            .field("file", &self.file)
            .field("password", &obfuscated)
            .field("private_key", &obfuscated)
            .finish()
    }
}

#[derive(
    Debug, Clone, PartialEq, smart_default::SmartDefault, Serialize, Deserialize, Validate,
)]
#[serde(deny_unknown_fields)]
pub struct Db {
    /// Path to the directory containing the database
    #[serde(default)]
    pub data: String,
    /// Determines whether the database should be initialized upon startup.
    #[serde(default = "just_true")]
    #[default = true]
    pub initialize: bool,
    /// Determines whether the database should be forcibly-initialized if it exists upon startup.
    #[serde(default)]
    pub force_initialize: bool,
}

fn default_session_idle_timeout() -> Duration {
    HoprLibConfig::default().protocol.session.idle_timeout
}

fn default_max_sessions() -> usize {
    HoprLibConfig::default()
        .protocol
        .session
        .tag_allocator
        .session as usize
}

fn default_session_establish_max_retries() -> usize {
    HoprLibConfig::default()
        .protocol
        .session
        .establish_max_retries as usize
}

fn default_probe_recheck_threshold() -> Duration {
    Duration::from_secs(10)
}

fn default_probe_interval() -> Duration {
    Duration::from_secs(3)
}

fn default_outgoing_ticket_winning_prob() -> Option<f64> {
    HoprLibConfig::default()
        .protocol
        .packet
        .codec
        .outgoing_win_prob
        .map(|p| p.as_f64())
}

fn build_mixer_cfg_from_env() -> MixerConfig {
    let defaults = MixerConfig::default();
    MixerConfig {
        min_delay: std::env::var("HOPR_INTERNAL_MIXER_MINIMUM_DELAY_IN_MS")
            .ok()
            .and_then(|v| v.trim().parse::<u64>().ok())
            .map(Duration::from_millis)
            .unwrap_or(defaults.min_delay),
        delay_range: std::env::var("HOPR_INTERNAL_MIXER_DELAY_RANGE_IN_MS")
            .ok()
            .and_then(|v| v.trim().parse::<u64>().ok())
            .map(Duration::from_millis)
            .unwrap_or(defaults.delay_range),
        capacity: std::env::var("HOPR_INTERNAL_MIXER_CAPACITY")
            .ok()
            .and_then(|v| v.trim().parse::<usize>().ok())
            .filter(|&c| c > 0)
            .unwrap_or(defaults.capacity),
        ..defaults
    }
}

/// Subset of various selected HOPR library network-related configuration options.
#[derive(Debug, Clone, PartialEq, smart_default::SmartDefault, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UserHoprNetworkConfig {
    /// How long it takes before HOPR Session is considered idle and is closed automatically
    #[default(default_session_idle_timeout())]
    #[serde(default = "default_session_idle_timeout", with = "humantime_serde")]
    pub session_idle_timeout: Duration,
    /// Maximum number of outgoing or incoming Sessions allowed by the Session manager
    #[default(default_max_sessions())]
    #[serde(default = "default_max_sessions")]
    pub maximum_sessions: usize,
    /// How many retries are made to establish an outgoing HOPR Session
    #[default(default_session_establish_max_retries())]
    #[serde(default = "default_session_establish_max_retries")]
    pub session_establish_max_retries: usize,
    /// The time interval for which to consider peer re-probing in seconds
    #[default(default_probe_recheck_threshold())]
    #[serde(default = "default_probe_recheck_threshold", with = "humantime_serde")]
    pub probe_recheck_threshold: Duration,
    /// The delay between individual probing rounds for neighbor discovery
    #[default(default_probe_interval())]
    #[serde(default = "default_probe_interval", with = "humantime_serde")]
    pub probe_interval: Duration,
    /// Should local addresses be announced on-chain?
    #[serde(default)]
    pub announce_local_addresses: bool,
    /// Should local addresses be preferred when dialing a peer?
    #[serde(default)]
    pub prefer_local_addresses: bool,
    /// Outgoing ticket winning probability.
    #[default(default_outgoing_ticket_winning_prob())]
    #[serde(default = "default_outgoing_ticket_winning_prob")]
    pub outgoing_ticket_winning_prob: Option<f64>,
    /// Minimum incoming ticket price.
    ///
    /// The value cannot be lower than the minimum network ticket price multiplied by the node's path position,
    /// and will default to that value whenever it is lower.
    #[serde(default)]
    pub min_incoming_ticket_price: Option<HoprBalance>,
    /// Packet mixer configuration.
    ///
    /// Controls the minimum delay, delay spread, and buffer capacity of the HOPR packet mixer.
    /// When omitted from the config file, falls back to `HOPR_INTERNAL_MIXER_*` env vars,
    /// then to compiled-in defaults (0 ms min, 20 ms spread, 20 000 capacity).
    #[default(build_mixer_cfg_from_env())]
    #[serde(default = "build_mixer_cfg_from_env")]
    pub mixer: MixerConfig,
}

/// Subset of the [`HoprLibConfig`] that is tuned to be user-facing and more user-friendly.
#[derive(Debug, Clone, PartialEq, smart_default::SmartDefault, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UserHoprLibConfig {
    /// Determines whether the node should be advertised publicly on-chain.
    #[default(just_true())]
    #[serde(default = "just_true")]
    pub announce: bool,
    /// Configuration related to host specifics
    #[default(default_host())]
    #[serde(default = "default_host")]
    pub host: HostConfig,
    /// Safe and Module configuration
    #[serde(default)]
    pub safe_module: SafeModule,
    /// Various HOPR-network and transport-related configuration options.
    #[serde(default)]
    pub network: UserHoprNetworkConfig,
    /// Path to a file that acts as incoming ticket storage.
    ///
    /// The file will be in the `redb` file format and can contain already existing tickets.
    /// If the file does not exist, it will be created.
    ///
    /// If omitted, a temporary file will be created and deleted on application exit.
    ///
    /// Make sure the file is secure and not accessible by unauthorized users on production.
    #[serde(default)]
    pub ticket_storage_file: Option<String>,
}

// NOTE: this intentionally does not validate (0.0.0.0) to force user to specify
// their external IP.
#[inline]
fn default_host() -> HostConfig {
    HostConfig {
        address: HostType::IPv4(hopr_lib::config::DEFAULT_HOST.to_owned()),
        port: hopr_lib::config::DEFAULT_PORT,
    }
}

impl From<UserHoprLibConfig> for HoprLibConfig {
    fn from(value: UserHoprLibConfig) -> Self {
        HoprLibConfig {
            host: value.host,
            publish: value.announce,
            safe_module: value.safe_module,
            protocol: HoprProtocolConfig {
                transport: TransportConfig {
                    announce_local_addresses: value.network.announce_local_addresses,
                    prefer_local_addresses: value.network.prefer_local_addresses,
                },
                packet: HoprPacketPipelineConfig {
                    codec: HoprCodecConfig {
                        outgoing_win_prob: value
                            .network
                            .outgoing_ticket_winning_prob
                            .and_then(|v| WinningProbability::try_from_f64(v).ok()),
                        min_incoming_ticket_price: value.network.min_incoming_ticket_price,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                probe: ProbeConfig {
                    interval: value.network.probe_interval,
                    recheck_threshold: value.network.probe_recheck_threshold,
                    ..Default::default()
                },
                session: SessionGlobalConfig {
                    idle_timeout: value.network.session_idle_timeout,
                    establish_max_retries: value.network.session_establish_max_retries as u32,
                    tag_allocator: TagAllocatorConfig {
                        session: value.network.maximum_sessions as u64,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                path_planner: Default::default(),
                counter_flush_interval: HoprProtocolConfig::default().counter_flush_interval,
                mixer: value.network.mixer,
            },
            ..Default::default()
        }
    }
}

impl Validate for UserHoprLibConfig {
    fn validate(&self) -> Result<(), ValidationErrors> {
        HoprLibConfig::from(self.clone()).validate()
    }
}

/// The main configuration object of the entire node.
///
/// The configuration is composed of individual configurations of corresponding
/// component configuration objects.
///
/// A default configuration YAML can be generated via `hoprd-cfg --default`.
#[derive(
    Debug, Serialize, Deserialize, Validate, Clone, PartialEq, smart_default::SmartDefault,
)]
#[serde(deny_unknown_fields)]
pub struct HoprdConfig {
    /// Configuration related to hopr-lib functionality
    #[validate(nested)]
    #[serde(default)]
    pub hopr: UserHoprLibConfig,
    /// Configuration regarding the identity of the node
    #[validate(nested)]
    #[serde(default)]
    pub identity: Identity,
    /// Configuration of the underlying database engine
    #[validate(nested)]
    #[serde(default)]
    pub db: Db,
    /// Configuration relevant for the API of the node
    #[validate(nested)]
    #[serde(default)]
    pub api: Api,
    /// Configuration of the Session entry/exit node IP protocol forwarding.
    #[validate(nested)]
    #[serde(default)]
    pub session_ip_forwarding: SessionIpForwardingConfig,
    /// Blokli provider URL to connect to.
    #[validate(url)]
    #[default(default_blokli_url())]
    pub blokli_url: String,
    /// Configuration of underlying node behavior in the form strategies
    ///
    /// Strategies represent automatically executable behavior performed by
    /// the node given pre-configured triggers.
    #[validate(nested)]
    #[serde(default = "crate::strategy::hopr_default_strategies")]
    #[default(crate::strategy::hopr_default_strategies())]
    pub strategy: crate::strategy::MultiStrategyConfig,
}

impl HoprdConfig {
    pub fn as_redacted(&self) -> Self {
        let mut ret = self.clone();
        // redacting sensitive information
        match ret.api.auth {
            Auth::None => {}
            Auth::Token(_) => ret.api.auth = Auth::Token("<REDACTED>".to_owned()),
        }

        if ret.identity.private_key.is_some() {
            ret.identity.private_key = Some("<REDACTED>".to_owned());
        }

        "<REDACTED>".clone_into(&mut ret.identity.password);

        ret
    }

    pub fn as_redacted_string(&self) -> anyhow::Result<String> {
        let redacted_cfg = self.as_redacted();
        Ok(serde_json::to_string(&redacted_cfg)?)
    }
}

fn just_true() -> bool {
    true
}

// Local Blokli endpoint default; suitable for development. Production deployments must override via config or CLI.
fn default_blokli_url() -> String {
    "http://localhost:8080".to_string()
}

#[cfg(test)]
mod tests {
    use std::{
        io::{Read, Write},
        str::FromStr,
    };

    use anyhow::Context;
    use clap::{Args, Command, FromArgMatches, Parser};
    use hopr_lib::api::types::primitive::prelude::Address;
    use tempfile::NamedTempFile;

    use super::*;

    pub fn example_cfg() -> anyhow::Result<HoprdConfig> {
        let safe_module = hopr_lib::config::SafeModule {
            safe_address: Address::from_str("0x0000000000000000000000000000000000000000")?,
            module_address: Address::from_str("0x0000000000000000000000000000000000000000")?,
        };

        let identity = Identity {
            file: "path/to/identity.file".to_string(),
            password: "change_me".to_owned(),
            private_key: None,
        };

        let host = HostConfig {
            address: HostType::IPv4("1.2.3.4".into()),
            port: 9091,
        };

        Ok(HoprdConfig {
            hopr: UserHoprLibConfig {
                host,
                safe_module,
                ..Default::default()
            },
            db: Db {
                data: "/app/db".to_owned(),
                ..Default::default()
            },
            identity,
            ..HoprdConfig::default()
        })
    }

    #[test]
    fn test_config_should_be_serializable_into_string() -> anyhow::Result<()> {
        let cfg = example_cfg()?;
        let yaml = serde_saphyr::to_string(&cfg)?;
        let from_yaml: HoprdConfig = serde_saphyr::from_str(&yaml)?;
        assert_eq!(cfg, from_yaml);
        Ok(())
    }

    #[test]
    fn example_config_should_be_serializable_into_string() -> anyhow::Result<()> {
        serde_saphyr::from_str::<HoprdConfig>(include_str!(
            "../../deploy/compose/hoprd/conf/hoprd.cfg.yaml"
        ))?;
        Ok(())
    }

    #[test]
    fn test_config_should_be_deserializable_from_a_string_in_a_file() -> anyhow::Result<()> {
        let mut config_file = NamedTempFile::new()?;
        let mut prepared_config_file = config_file.reopen()?;

        let cfg = example_cfg()?;
        let yaml = serde_saphyr::to_string(&cfg)?;
        config_file.write_all(yaml.as_bytes())?;

        let mut buf = String::new();
        prepared_config_file.read_to_string(&mut buf)?;
        let deserialized_cfg: HoprdConfig = serde_saphyr::from_str(&buf)?;

        assert_eq!(deserialized_cfg, cfg);

        Ok(())
    }

    /// TODO: This test attempts to deserialize the data structure incorrectly in the native build
    /// (`confirmations`` are an extra field), as well as misses the native implementation for the
    /// version satisfies check
    #[test]
    #[ignore]
    fn test_config_is_extractable_from_the_cli_arguments() -> anyhow::Result<()> {
        let pwnd = "rpc://pawned!";

        let mut config_file = NamedTempFile::new()?;

        let mut cfg = example_cfg()?;
        cfg.blokli_url = pwnd.to_owned();

        let yaml = serde_saphyr::to_string(&cfg)?;
        config_file.write_all(yaml.as_bytes())?;
        let cfg_file_path = config_file
            .path()
            .to_str()
            .context("file path should have a string representation")?
            .to_string();

        let cli_args = vec!["hoprd", "--configurationFilePath", cfg_file_path.as_str()];

        let mut cmd = Command::new("hoprd").version("0.0.0");
        cmd = crate::cli::CliArgs::augment_args(cmd);
        let derived_matches = cmd.try_get_matches_from(cli_args)?;
        let args = crate::cli::CliArgs::from_arg_matches(&derived_matches)?;

        // skipping validation
        let cfg = HoprdConfig::try_from(args)?;

        assert_eq!(cfg.blokli_url, pwnd.to_owned());

        Ok(())
    }

    /// Writes `cfg` to a temporary YAML file and returns the file (kept alive by
    /// the caller) together with its path.
    fn write_cfg_file(cfg: &HoprdConfig) -> anyhow::Result<(NamedTempFile, String)> {
        let mut config_file = NamedTempFile::new()?;
        let yaml = serde_saphyr::to_string(cfg)?;
        config_file.write_all(yaml.as_bytes())?;
        let path = config_file
            .path()
            .to_str()
            .context("file path should have a string representation")?
            .to_string();
        Ok((config_file, path))
    }

    #[test]
    fn validation_should_fail_when_required_values_are_missing_from_the_file() -> anyhow::Result<()>
    {
        let mut cfg = example_cfg()?;
        // Blank the password so the file on its own is genuinely invalid.
        cfg.identity.password = String::new();

        let (_file, cfg_path) = write_cfg_file(&cfg)?;

        let cli_args = crate::cli::CliArgs::try_parse_from([
            "hoprd",
            "--configurationFilePath",
            cfg_path.as_str(),
        ])?;
        let effective = HoprdConfig::try_from(cli_args)?;

        assert!(
            effective.validate().is_err(),
            "config with an empty password must fail validation on its own"
        );

        Ok(())
    }

    #[test]
    fn validation_should_pass_when_required_values_are_supplied_via_overrides() -> anyhow::Result<()>
    {
        let mut cfg = example_cfg()?;
        // Same file as the negative test: invalid on its own due to the empty password.
        cfg.identity.password = String::new();

        let (_file, cfg_path) = write_cfg_file(&cfg)?;

        // The password is supplied as a CLI/environment override (the exact code
        // path that `HOPRD_PASSWORD` feeds into), so the effective config is valid.
        let cli_args = crate::cli::CliArgs::try_parse_from([
            "hoprd",
            "--configurationFilePath",
            cfg_path.as_str(),
            "--password",
            "a-securely-provided-password",
        ])?;
        let effective = HoprdConfig::try_from(cli_args)?;

        assert!(
            effective.validate().is_ok(),
            "config must validate once the missing password is supplied via override"
        );

        Ok(())
    }

    #[test]
    fn validation_fails_when_no_identity_source() -> anyhow::Result<()> {
        let mut cfg = example_cfg()?;
        cfg.identity.file = String::new();
        cfg.identity.private_key = None;

        let err = cfg
            .validate()
            .expect_err("a config without any identity source must fail validation");
        let rendered = format!("{err:?}");
        assert!(
            rendered.contains("identity_source_missing"),
            "unexpected error: {rendered}"
        );

        Ok(())
    }

    #[test]
    fn validation_passes_with_private_key_and_no_file() -> anyhow::Result<()> {
        let mut cfg = example_cfg()?;
        cfg.identity.file = String::new();
        // A valid 128-hex private key satisfies both the source requirement and the key format.
        cfg.identity.private_key = Some(format!("0x{}", "a".repeat(128)));

        cfg.validate()
            .context("a private key alone should satisfy the identity source requirement")?;

        Ok(())
    }

    #[test]
    fn validation_passes_with_identity_file_and_no_key() -> anyhow::Result<()> {
        // `example_cfg` sets `identity.file` and leaves `private_key` unset.
        let cfg = example_cfg()?;

        cfg.validate()
            .context("an identity file alone should satisfy the identity source requirement")?;

        Ok(())
    }

    #[test]
    fn explicit_mixer_section_round_trips() -> anyhow::Result<()> {
        use std::time::Duration;

        let mut cfg = example_cfg()?;
        cfg.hopr.network.mixer = hopr_lib::config::MixerConfig {
            min_delay: Duration::from_millis(0),
            delay_range: Duration::from_millis(15),
            capacity: 5_000,
            ..Default::default()
        };
        let yaml = serde_saphyr::to_string(&cfg)?;
        let from_yaml: HoprdConfig = serde_saphyr::from_str(&yaml)?;
        assert_eq!(cfg, from_yaml);
        assert!(!yaml.contains("metric_delay_window"));
        let lib_cfg: HoprLibConfig = cfg.hopr.into();
        assert_eq!(
            lib_cfg.protocol.mixer.delay_range,
            Duration::from_millis(15)
        );

        Ok(())
    }

    #[test]
    fn mixer_env_var_fallback_is_read() {
        use std::time::Duration;

        // Baseline: no env vars → compiled-in defaults.
        let base = build_mixer_cfg_from_env();
        assert_eq!(base.delay_range, MixerConfig::default().delay_range);

        // With env var set → value is picked up.
        unsafe { std::env::set_var("HOPR_INTERNAL_MIXER_DELAY_RANGE_IN_MS", "42") };
        let with_env = build_mixer_cfg_from_env();
        unsafe { std::env::remove_var("HOPR_INTERNAL_MIXER_DELAY_RANGE_IN_MS") };
        assert_eq!(with_env.delay_range, Duration::from_millis(42));

        // SmartDefault path (hopr.network absent from YAML) also reads env vars.
        unsafe { std::env::set_var("HOPR_INTERNAL_MIXER_DELAY_RANGE_IN_MS", "7") };
        let default_net = UserHoprNetworkConfig::default();
        unsafe { std::env::remove_var("HOPR_INTERNAL_MIXER_DELAY_RANGE_IN_MS") };
        assert_eq!(default_net.mixer.delay_range, Duration::from_millis(7));
    }
}
