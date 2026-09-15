//! Empty a kept cluster's Safes back to one address.
//!
//! On a real chain every run leaves wxHOPR behind: each node's Safe holds whatever the channel
//! stakes and the PIX float did not spend. The identities are kept on disk
//! ([`KEEP_CLUSTER_DIR_ENV`](crate::identity) in the tests; `--data-dir` for the CLI), and each
//! node's chain key is the sole owner of its Safe, so this walks a cluster directory and moves
//! every Safe's wxHOPR to `--to` — through the node's management module, which is how
//! `withdraw` on a Safe-aware connector spends: the node key signs, the Safe pays.
//!
//! Only wxHOPR moves. A Safe holds no xDai (the cluster never gives it any), and the node
//! accounts' xDai is gas money not worth a transaction each.

use std::path::{Path, PathBuf};

use anyhow::Context;
use hopr_chain_connector::{
    Address, BlockchainConnectorConfig,
    api::*,
    blokli_client::{BlokliClient, BlokliClientConfig},
    create_trustful_hopr_blokli_connector,
};
use hopr_lib::{HoprKeys, api::types::crypto::keypairs::Keypair};
use tracing::info;

use crate::identity::DEFAULT_TX_TIMEOUT_MULTIPLIER;

/// A node of a kept cluster: its keystore and the Safe/module its config names.
#[derive(Debug)]
pub struct KeptNode {
    pub id: usize,
    pub keystore: PathBuf,
    pub safe: Address,
    pub module: Address,
}

/// The nodes a cluster directory holds, from `node_id_<i>.id` + `hoprd_cfg_<i>.yaml` pairs.
pub fn kept_nodes(cluster_dir: &Path) -> anyhow::Result<Vec<KeptNode>> {
    let mut nodes = Vec::new();
    for id in 0..16 {
        let keystore = cluster_dir.join(format!("node_id_{id}.id"));
        let cfg = cluster_dir.join(format!("hoprd_cfg_{id}.yaml"));
        if !keystore.is_file() {
            continue;
        }
        anyhow::ensure!(
            cfg.is_file(),
            "node {id}: {} exists but {} does not",
            keystore.display(),
            cfg.display()
        );
        let yaml =
            std::fs::read_to_string(&cfg).with_context(|| format!("reading {}", cfg.display()))?;
        let value: serde_json::Value =
            serde_saphyr::from_str(&yaml).with_context(|| format!("parsing {}", cfg.display()))?;
        let safe_module = &value["hopr"]["safe_module"];
        let parse = |key: &str| -> anyhow::Result<Address> {
            safe_module[key]
                .as_str()
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "node {id}: hopr.safe_module.{key} missing in {}",
                        cfg.display()
                    )
                })?
                .parse()
                .with_context(|| format!("node {id}: hopr.safe_module.{key}"))
        };
        nodes.push(KeptNode {
            id,
            keystore,
            safe: parse("safe_address")?,
            module: parse("module_address")?,
        });
    }
    anyhow::ensure!(
        !nodes.is_empty(),
        "no node_id_<i>.id files under {}",
        cluster_dir.display()
    );
    Ok(nodes)
}

/// Moves every Safe's wxHOPR to `to`. Returns what each node moved.
pub async fn sweep_safes(
    cluster_dir: &Path,
    password: &str,
    blokli_url: &str,
    to: Address,
) -> anyhow::Result<Vec<(usize, Address, HoprBalance)>> {
    let blokli_client = BlokliClient::new(blokli_url.parse()?, BlokliClientConfig::default());
    let mut moved = Vec::new();
    for node in kept_nodes(cluster_dir)? {
        let path = node
            .keystore
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("non-UTF-8 keystore path"))?;
        let (keys, _) = HoprKeys::read_eth_keystore(path, password)
            .with_context(|| format!("node {}: reading {}", node.id, node.keystore.display()))?;
        let owner = keys.chain_key.public().to_address();
        let mut connector = create_trustful_hopr_blokli_connector(
            &keys.chain_key,
            BlockchainConnectorConfig {
                tx_timeout_multiplier: DEFAULT_TX_TIMEOUT_MULTIPLIER,
                ..Default::default()
            },
            blokli_client.clone(),
            node.module,
        )
        .await?;
        connector.connect().await?;
        let balance: HoprBalance = connector.balance(node.safe).await?;
        if balance.is_zero() {
            info!(node = node.id, safe = %node.safe, "Safe is empty; nothing to sweep");
            eprintln!("Node {}: Safe {} is empty", node.id, node.safe);
            continue;
        }
        eprintln!(
            "Node {}: sweeping {balance} from Safe {} (owner {owner}, module {}) to {to}...",
            node.id, node.safe, node.module
        );
        connector
            .withdraw(balance, &to)
            .await
            .with_context(|| format!("node {}: submitting the sweep", node.id))?
            .await
            .with_context(|| format!("node {}: confirming the sweep", node.id))?;
        eprintln!("Node {}: {balance} moved to {to}", node.id);
        moved.push((node.id, node.safe, balance));
    }
    Ok(moved)
}
