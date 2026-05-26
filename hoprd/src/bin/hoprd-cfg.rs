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

use std::path::PathBuf;

use anyhow::{Context, anyhow};
use clap::Parser;
use hoprd::config::HoprdConfig;
use validator::Validate as _;

#[derive(Parser, Default)]
#[clap(author, version, about, long_about = None)]
struct CliArgs {
    /// Print the default YAML config for the hoprd
    #[clap(short = 'd', long, conflicts_with = "validate")]
    default: bool,
    /// Validate the config at this path
    #[clap(short, long, conflicts_with = "default")]
    validate: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();

    if args.default {
        println!(
            "{}",
            serde_saphyr::to_string(&hoprd::config::HoprdConfig::default())
                .context("failed to serialize default config")?
        );
    } else if let Some(cfg_path) = args.validate {
        let cfg_path = cfg_path
            .into_os_string()
            .into_string()
            .map_err(|_| anyhow!("file path not convertible"))?;

        // Use hoprd's own CliArgs so that env-var overrides (e.g. HOPRD_PASSWORD)
        // are applied exactly as hoprd would apply them at startup.  Without this,
        // fields legitimately supplied via environment variables (the standard Docker
        // Compose pattern) would always fail validation.
        let hoprd_args = hoprd::cli::CliArgs::try_parse_from([
            "hoprd",
            "--configurationFilePath",
            &cfg_path,
        ])
        .context("failed to parse args for validation")?;
        let cfg = HoprdConfig::try_from(hoprd_args).context("failed to build config")?;

        cfg.validate().context("config validation failed")?;
    }

    Ok(())
}
