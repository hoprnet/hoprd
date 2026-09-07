//! Integration test: start a 3-node local cluster and verify that every node's
//! `/account/addresses` REST API response matches the identity generated for it.
//!
//! This test is `#[ignore]` (long runtime, external chain container + `hoprd`
//! binary) and is not intended for CI — run it explicitly by name.
//!
//! Required (at least one chain source):
//!   HOPRD_CHAIN_URL        – Blokli URL of a running Anvil+Blokli stack
//!   HOPRD_CHAIN_IMAGE      – container image to launch (used when HOPRD_CHAIN_URL is absent)
//!
//! Optional:
//!   HOPRD_BIN              – path to the hoprd binary (default: "hoprd" on PATH)
//!   HOPRD_CONTAINER_RUNTIME – container runtime CLI (default: "docker")
//!
//! # Prerequisites
//!
//! The `hoprd` binary must be built in **release** mode.  Debug builds incur
//! significant overhead that can push the test past the default timeout:
//!
//! ```bash
//! nix develop -c cargo build --release -p hoprd
//! export HOPRD_BIN=$(pwd)/target/release/hoprd
//! ```

mod common;

use anyhow::Context;
use common::{Cluster, ClusterSpec, ports};

#[tokio::test]
#[ignore = "requires external chain container and hoprd binary — run explicitly, not in CI"]
async fn localcluster_addresses_match_generated_identities() {
    run().await.expect("address-identity verification failed");
}

async fn run() -> anyhow::Result<()> {
    common::init_tracing();

    // Every setting this suite needs is a bring-up default: three nodes, random identities,
    // AutoRedeeming on and ChannelLifecycle off. It asserts on identity alone, so it wants no
    // channels, no readiness check and no peer graph.
    let cluster = Cluster::start(ClusterSpec::new(ports::ADDRESS_IDENTITY)).await?;

    // Verify every node's API-reported address matches its generated identity.
    // The generated identity stores the address as lowercase, while the API
    // returns it in EIP-55 checksummed format — compare case-insensitively.
    for node in cluster.nodes() {
        let actual = node
            .address
            .as_deref()
            .with_context(|| format!("node {} address unresolved", node.id))?;
        let expected = &cluster.identities.nodes[node.id].address;
        anyhow::ensure!(
            actual.eq_ignore_ascii_case(expected),
            "node {} address mismatch: got {actual}, expected {expected}",
            node.id,
        );
        tracing::info!("node {id} address verified: {actual}", id = node.id);
    }

    tracing::info!("address-identity verification test passed");
    Ok(())
}
