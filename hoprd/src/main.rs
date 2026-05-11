use std::{num::NonZeroUsize, process::ExitCode, str::FromStr};

use hopr_lib::{
    HoprKeys, IdentityRetrievalModes,
    api::types::{crypto::keypairs::Keypair, primitive::traits::ToHex},
};
use hoprd::{cli::CliArgs, config::HoprdConfig};
use validator::Validate;

// Avoid musl's default allocator due to degraded performance
//
// https://nickb.dev/blog/default-musl-allocator-considered-harmful-to-performance
#[cfg(all(feature = "allocator-mimalloc", feature = "allocator-jemalloc"))]
compile_error!(
    "feature \"allocator-jemalloc\" and feature \"allocator-mimalloc\" cannot be enabled at the same time"
);
#[cfg(all(target_os = "linux", feature = "allocator-mimalloc"))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
#[cfg(all(target_os = "linux", feature = "allocator-jemalloc"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[cfg(all(target_os = "linux", feature = "allocator-jemalloc-stats"))]
mod jemalloc_stats;

#[cfg(feature = "telemetry")]
mod telemetry;
mod telemetry_common;

#[cfg(not(feature = "runtime-tokio"))]
compile_error!("The 'runtime-tokio' feature must be enabled");

#[cfg(feature = "runtime-tokio")]
fn main() -> ExitCode {
    if let Err(e) = telemetry_common::install_base_subscriber() {
        eprintln!("ERROR: failed to initialize base log subscriber: {e}");
        return ExitCode::FAILURE;
    }

    let num_cpu_threads = std::env::var("HOPRD_NUM_CPU_THREADS").ok().and_then(|v| {
        usize::from_str(&v)
            .map_err(anyhow::Error::from)
            .and_then(|v| NonZeroUsize::try_from(v).map_err(anyhow::Error::from))
            .inspect_err(|error| tracing::error!(%error, "failed to parse HOPRD_NUM_CPU_THREADS"))
            .ok()
    });

    let num_io_threads = std::env::var("HOPRD_NUM_IO_THREADS").ok().and_then(|v| {
        usize::from_str(&v)
            .map_err(anyhow::Error::from)
            .and_then(|v| NonZeroUsize::try_from(v).map_err(anyhow::Error::from))
            .inspect_err(|error| tracing::error!(%error, "failed to parse HOPRD_NUM_IO_THREADS"))
            .ok()
    });

    let args = <CliArgs as clap::Parser>::parse();
    let cfg = match HoprdConfig::try_from(args) {
        Ok(cfg) => cfg,
        Err(error) => {
            tracing::error!(%error, "hoprd exited with an error");
            return ExitCode::FAILURE;
        }
    };
    if let Err(error) = cfg.validate() {
        tracing::error!(%error, "hoprd exited with an error");
        return ExitCode::FAILURE;
    }

    let maybe_keys = match &cfg.identity.private_key {
        Some(private_key) => IdentityRetrievalModes::FromPrivateKey { private_key },
        None => IdentityRetrievalModes::FromFile {
            password: &cfg.identity.password,
            id_path: &cfg.identity.file,
        },
    };

    let hopr_keys: HoprKeys = match maybe_keys.try_into() {
        Ok(hopr_keys) => hopr_keys,
        Err(error) => {
            tracing::error!(%error, "hoprd exited with an error");
            return ExitCode::FAILURE;
        }
    };

    #[cfg(feature = "telemetry")]
    let node_identity = telemetry::NodeTelemetryIdentity {
        node_address: Keypair::public(&hopr_keys.chain_key).to_address().to_hex(),
        node_peer_id: Keypair::public(&hopr_keys.packet_key).to_peerid_str(),
        extra_labels: std::env::var("HOPRD_OTEL_EXPORT_LABELS")
            .unwrap_or_default()
            .split(',')
            .filter_map(|pair| {
                let (k, v) = pair.trim().split_once('=')?;
                Some((k.trim().to_string(), v.trim().to_string()))
            })
            .collect(),
    };

    hopr_lib::prepare_tokio_runtime(num_cpu_threads, num_io_threads)
        .and_then(|runtime| {
            runtime.block_on(async move {
                use hoprd::main_inner;

                #[cfg(feature = "telemetry")]
                let _telemetry = telemetry::init_logger(node_identity)?;

                main_inner(cfg, hopr_keys).await
            })
        })
        .map(|_| {
            tracing::info!("hoprd exited successfully");
            ExitCode::SUCCESS
        })
        .unwrap_or_else(|error| {
            tracing::error!(%error, backtrace = ?error.backtrace(), "hoprd exited with an error");
            ExitCode::FAILURE
        })
}
