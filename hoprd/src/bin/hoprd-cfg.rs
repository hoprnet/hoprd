//! Hoprd configuration utility `hoprd-cfg`
//!
//! This executable offers functionalities associated with configuration management
//! of the HOPRd node configuration.
//!
//! ## Help
//! ```shell
//! ➜   hoprd-cfg --help
//! Usage: hoprd-cfg [OPTIONS]
//!
//! Options:
//!   -d, --default              Print the default YAML config for the hoprd
//!   -v, --validate <VALIDATE>  Validate the config at this path
//!   -h, --help                 Print help
//!   -V, --version              Print version
//! ```
//!
//! ## Dump a default configuration file
//! ```shell
//! ➜   hoprd-cfg -d     
//! hopr:
//! host:
//!   address: !IPv4 0.0.0.0
//!   port: 9091
//!
//! ... <snip>
//! ```
//!
//! ## Validate an existing configuration YAML
//!
//! Validation reflects the *effective* configuration hoprd builds at startup:
//! the YAML file with `HOPRD_*` environment variables (and CLI-equivalent
//! overrides) layered on top. Values supplied via the environment are honored,
//! so they are not falsely reported as missing or invalid.
//!
//! All validation errors found in the config are reported at once
//! (one per line under `Caused by:`); the binary exits with code 1.
//!
//! ```shell
//! ➜   hoprd-cfg -v /tmp/bad-config.yaml
//! Error: config validation failed
//!
//! Caused by:
//!     blokli_url: Validation error: url [{"value": String("not-a-valid-url")}]
//!     identity.password: Validation error: No password could be found [{"value": String("")}]
//!
//! ➜   echo $?
//! 1
//! ```
//!
//! Note: YAML parsing errors (unknown fields, type mismatches) are
//! reported by `serde` and stop at the first occurrence.
//!
//! ## Validate the effective config from hoprd arguments
//!
//! The container entrypoint uses `--validate-args` to validate the exact argument
//! vector hoprd will receive (everything after `--`), so CLI-flag overrides are
//! honored in addition to env vars and the YAML file:
//!
//! ```shell
//! ➜   hoprd-cfg --validate-args -- --configurationFilePath /cfg.yaml --password s3cr3t
//! ```
//!
//! A `--help`/`--version` request among the forwarded arguments is a no-op success,
//! since hoprd itself handles those.

use std::path::PathBuf;

use anyhow::{Context, anyhow};
use clap::Parser;
use hoprd::config::HoprdConfig;
use validator::Validate as _;

#[derive(Parser, Default)]
#[clap(author, version, about, long_about = None)]
struct CliArgs {
    /// Print the default YAML config for the hoprd
    #[clap(short = 'd', long, conflicts_with_all = ["validate", "validate_args"])]
    default: bool,
    /// Validate the config at this path
    #[clap(short, long, conflicts_with_all = ["default", "validate_args"])]
    validate: Option<PathBuf>,
    /// Validate the effective config built from the forwarded hoprd arguments
    /// (everything after `--`), mirroring hoprd's startup. Honors env-var and
    /// CLI-flag overrides, not just the YAML file.
    #[clap(long = "validate-args", conflicts_with_all = ["default", "validate"])]
    validate_args: bool,
    /// hoprd arguments to validate; everything after `--`.
    #[clap(last = true)]
    forwarded_args: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();

    if args.default {
        println!(
            "{}",
            serde_saphyr::to_string(&hoprd::config::HoprdConfig::default())
                .context("failed to serialize default config")?
        );
    } else if args.validate_args {
        validate_effective_config(&args.forwarded_args)?;
    } else if let Some(cfg_path) = args.validate {
        let cfg_path = cfg_path
            .into_os_string()
            .into_string()
            .map_err(|_| anyhow!("file path not convertible"))?;

        validate_effective_config(&["--configurationFilePath".to_string(), cfg_path])?;
    }

    Ok(())
}

/// Validates the effective hoprd configuration built from `args` (a hoprd CLI argument
/// vector, without the leading program name).
///
/// This mirrors hoprd's own startup exactly: the YAML file with env-var and CLI-flag
/// overrides layered on top, so values legitimately supplied via the environment or CLI
/// are not falsely reported as missing or invalid. A `--help`/`--version` request is
/// treated as success without validating, since hoprd itself renders those.
fn validate_effective_config(args: &[String]) -> anyhow::Result<()> {
    let argv = std::iter::once("hoprd".to_string()).chain(args.iter().cloned());

    let hoprd_args = match hoprd::cli::CliArgs::try_parse_from(argv) {
        Ok(parsed) => parsed,
        Err(error)
            if matches!(
                error.kind(),
                clap::error::ErrorKind::DisplayHelp
                    | clap::error::ErrorKind::DisplayVersion
                    | clap::error::ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
            ) =>
        {
            return Ok(());
        }
        Err(error) => {
            return Err(anyhow::Error::new(error).context("failed to parse args for validation"));
        }
    };

    let cfg = HoprdConfig::try_from(hoprd_args).context("failed to build config")?;
    cfg.validate().context("config validation failed")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use anyhow::Context;
    use tempfile::NamedTempFile;

    use super::validate_effective_config;
    use hoprd::config::HoprdConfig;

    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    fn config_file_without_password() -> anyhow::Result<NamedTempFile> {
        // Serialize the default config (identity.password is "" by default)
        // to get a valid YAML that fails validation only on the missing password.
        let yaml = serde_saphyr::to_string(&HoprdConfig::default())
            .context("failed to serialize default config")?;
        let mut file = NamedTempFile::new()?;
        file.write_all(yaml.as_bytes())?;
        Ok(file)
    }

    #[test]
    fn validate_passes_when_password_supplied_via_cli_flag() -> anyhow::Result<()> {
        let file = config_file_without_password()?;
        let path = file.path().to_str().unwrap().to_string();

        // CLI flags are part of the forwarded argument vector, so they must be honored
        // exactly as hoprd would at startup (no env mutation needed).
        validate_effective_config(&[
            "--configurationFilePath".to_string(),
            path,
            "--password".to_string(),
            "a-securely-provided-password".to_string(),
        ])
        .context("expected validation to pass with --password supplied")
    }

    #[test]
    fn validate_passes_when_password_supplied_via_env() -> anyhow::Result<()> {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let file = config_file_without_password()?;
        let path = file.path().to_str().unwrap().to_string();

        // Safety: ENV_LOCK serializes all env-var access across tests in this module.
        unsafe { std::env::set_var("HOPRD_PASSWORD", "s3cr3tpassword") };
        let result =
            validate_effective_config(&["--configurationFilePath".to_string(), path.clone()]);
        unsafe { std::env::remove_var("HOPRD_PASSWORD") };

        result.context("expected validation to pass with HOPRD_PASSWORD set")
    }

    #[test]
    fn validate_fails_without_password_in_config_or_env() -> anyhow::Result<()> {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let file = config_file_without_password()?;
        let path = file.path().to_str().unwrap().to_string();

        unsafe { std::env::remove_var("HOPRD_PASSWORD") };
        let result = validate_effective_config(&["--configurationFilePath".to_string(), path]);

        let err = result.expect_err("expected validation to fail without a password");
        assert!(
            err.to_string().contains("config validation failed"),
            "unexpected error: {err}"
        );
        Ok(())
    }

    #[test]
    fn help_and_version_requests_are_a_noop_success() -> anyhow::Result<()> {
        // hoprd itself renders --help/--version; the validation gate must not block them.
        validate_effective_config(&["--help".to_string()])
            .context("--help should not fail validation")?;
        validate_effective_config(&["--version".to_string()])
            .context("--version should not fail validation")?;
        Ok(())
    }
}
