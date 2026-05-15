use std::{env, fs, path::Path};

use anyhow::Context;

const CONTRACTS_ADDRESSES_FILENAME: &str = "contracts-addresses.json";

fn main() -> anyhow::Result<()> {
    let out_dir = env::var("OUT_DIR").context("OUT_DIR environment variable should be set")?;
    let src_dir = env::var("CARGO_MANIFEST_DIR").context("CARGO_MANIFEST_DIR environment variable should be set")?;

    let dest_path = Path::new(&out_dir).join(CONTRACTS_ADDRESSES_FILENAME);
    let config_path = Path::new(&src_dir).join(CONTRACTS_ADDRESSES_FILENAME);

    if !config_path.exists() {
        return Err(anyhow::anyhow!(
            "{} not found at expected path: {:?}",
            CONTRACTS_ADDRESSES_FILENAME,
            config_path
        ));
    } else {
        fs::create_dir_all(out_dir)?;
        fs::copy(&config_path, &dest_path).context(format!(
            "Failed to copy {} to OUT_DIR: {}",
            config_path.display(),
            dest_path.display()
        ))?;
    }

    // Tell Cargo to rerun this build script if the config file changes
    println!("cargo:rerun-if-changed={}", config_path.display());

    Ok(())
}
