//! Smoke test: start a 3-node local cluster and verify the ChannelLifecycleStrategy
//! opens a full-mesh topology without any explicit REST open_channel calls.
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

use std::time::Duration;

use anyhow::Result;
use common::{Cluster, ClusterSpec, ports};
use hoprd_localcluster::identity;

const WAIT_TIMEOUT: Duration = Duration::from_secs(120);

#[tokio::test]
#[ignore = "requires external chain container and hoprd binary — run explicitly, not in CI"]
async fn localcluster_channels_opened_by_strategy() {
    run().await.expect("localcluster smoke test failed");
}

async fn run() -> Result<()> {
    common::init_tracing();

    // The ChannelLifecycleStrategy population thresholds are set to num_nodes-1 inside
    // `generate`, so the strategy opens a full mesh on its own.
    let cluster = Cluster::start(ClusterSpec {
        // Frozen identities on purpose: they are what a real cluster run uses, so this test
        // also covers the frozen key derivation. See `scripts/localcluster-smoke.sh` for the
        // cheaper 2-node variant that CI runs on every PR.
        random_identities: false,
        strategies: identity::StrategySet {
            auto_redeeming: true,
            channel_lifecycle: true,
        },
        // `strategy_execution_interval` stays unset, i.e. hoprd's 60 s default. This is the
        // only suite whose mesh is opened *by* a strategy, so the strategy has to actually run
        // during the test; the 600 s the Session suites use would stall it.
        //
        // PIX off. It used to be on here, but only as `HOPRD_ENABLE_PIX=1`, which a binary
        // without a deposit pool logged and ignored — so this test stayed runnable against a
        // plain `cargo build -p hoprd`. A `Pix` stanza is not ignorable: it would now refuse
        // to start that same binary. Nothing here opens a PIX Session, so there is nothing to
        // keep; `session_pix` covers the strategy properly.
        start_timeout: 2 * WAIT_TIMEOUT,
        ..ClusterSpec::new(ports::SMOKE)
    })
    .await?;

    // `wait_ready` is intentionally not called: on Apple Container the blokli SSE
    // subscription drops every ~10 s, cycling the chain health through Degraded→Connecting.
    // During reconnection HoprState briefly leaves Running, so /readyz oscillates between 200
    // and 412 — making the check flaky. `wait_reachable` below is a stronger guarantee anyway,
    // and it is the strategy's actual precondition (require_currently_connected = true), which
    // is why it comes before the channels here rather than after.
    cluster.wait_reachable(WAIT_TIMEOUT).await?;

    // Key assertion: the ChannelLifecycleStrategy must open the full mesh of outgoing channels
    // without any explicit REST open_channel calls.
    cluster.wait_channels(WAIT_TIMEOUT * 4).await?;

    // Double-check every expected pair via direct API call.
    for src in cluster.nodes() {
        for dst in cluster.nodes() {
            if let (Some(src_addr), Some(dst_addr)) = (&src.address, &dst.address)
                && src_addr != dst_addr
            {
                assert!(
                    src.api.is_outgoing_channel_open(dst_addr).await?,
                    "node {} missing open outgoing channel to node {}",
                    src.id,
                    dst.id,
                );
            }
        }
    }

    tracing::info!("smoke test passed: full mesh established by strategy");
    Ok(())
}
