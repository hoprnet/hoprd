//! The one-time grant a direct Curvy shield needs, executed for a freshly deployed Safe.
//!
//! The Curvy vault pulls a direct shield's wxHOPR with `safeTransferFrom(msg.sender, …)`, so the
//! shield must be *called by the Safe* — through `HoprNodeManagementModule.execTransactionFromModule`
//! — and the module only forwards calls to targets its owner has scoped. Nothing scopes the
//! aggregator on its own: the chain image grants it on Anvil, and on a real chain every new Safe
//! reverts `NonExistentKey()` on its first shield until someone does. `hopr-strategy`'s
//! `scripts/scope-curvy-aggregator.sh` is the operator's version of this; the cluster's Safes are
//! random per run, so the harness has to do it itself, right after `deploy_safe`.
//!
//! What is sent is a Safe `execTransaction` whose inner call is the module's
//! `scopeTargetToken(uint256)` with the aggregator packed as a `Target` granting
//! `TargetPermission.ALLOW_ALL` — the only permission the module's hardcoded selector whitelist
//! can express for a `directShield`. The Safe is 1-of-1 with the node's chain key as owner, so a
//! Safe "pre-validated" signature (`r` = owner, `s` = 0, `v` = 1) from a transaction the owner
//! sends itself is accepted without any EIP-712 signing.
//!
//! Encodings are done by hand rather than through bindings — `hopr-types` keeps its Safe encoders
//! private and `hopr-api` has no "execute through the Safe" operation — and are pinned by
//! reference vectors produced with `cast`, so the tests check the encoding rather than the code.

use anyhow::Context;
use hopr_chain_connector::{
    Address, ChainKeypair,
    blokli_client::{BlokliQueryClient, BlokliTransactionClient},
};
use hopr_lib::api::types::crypto::keypairs::Keypair;
use tracing::info;

/// Environment variable naming the Curvy aggregator to scope into every node's module. Unset:
/// no grant is made, which is right for the Anvil image (already granted) and for the plain pool.
pub const SCOPE_AGGREGATOR_ENV: &str = "HOPRD_CURVY_SCOPE_AGGREGATOR";

/// `keccak256("scopeTargetToken(uint256)")[..4]` on `HoprNodeManagementModule`.
const SCOPE_TARGET_TOKEN_SELECTOR: [u8; 4] = [0xa7, 0x6c, 0x9a, 0x2f];
/// `keccak256("execTransaction(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,bytes)")[..4]`.
const EXEC_TRANSACTION_SELECTOR: [u8; 4] = [0x6a, 0x76, 0x12, 0x02];
/// Plenty for `execTransaction` → `scopeTargetToken`, which writes one storage slot.
const GAS_LIMIT: u64 = 300_000;
/// Fee fallbacks when Blokli does not quote, in wei: modest even for Gnosis.
const FALLBACK_MAX_FEE_PER_GAS: u128 = 5_000_000_000;
const FALLBACK_MAX_PRIORITY_FEE_PER_GAS: u128 = 1_000_000_000;

/// The aggregator named by [`SCOPE_AGGREGATOR_ENV`], if any.
pub fn aggregator_from_env() -> anyhow::Result<Option<Address>> {
    match std::env::var(SCOPE_AGGREGATOR_ENV) {
        Ok(raw) => {
            raw.trim().parse::<Address>().map(Some).with_context(|| {
                format!("{SCOPE_AGGREGATOR_ENV} must be an EVM address, got {raw:?}")
            })
        }
        Err(_) => Ok(None),
    }
}

/// The module's packed `Target` word for `address`, laid out by `TargetUtils.encodeDefaultPermissions`:
///
/// ```text
/// address << 96 | Clearance << 88 | TargetType << 80 | TargetPermission << 72 | 9 capability bytes
/// ```
///
/// with `Clearance.FUNCTION` (01), `TargetType.TOKEN` (00, a label), `TargetPermission.ALLOW_ALL`
/// (03) and every capability left at NONE so the ALLOW_ALL default applies.
pub fn encode_target(address: Address) -> [u8; 32] {
    let mut word = [0u8; 32];
    let address: [u8; 20] = address.into();
    word[..20].copy_from_slice(&address);
    word[20] = 0x01;
    word[21] = 0x00;
    word[22] = 0x03;
    word
}

/// `scopeTargetToken(uint256)` calldata for the module.
pub fn encode_scope_target_token(target: [u8; 32]) -> Vec<u8> {
    let mut data = Vec::with_capacity(36);
    data.extend_from_slice(&SCOPE_TARGET_TOKEN_SELECTOR);
    data.extend_from_slice(&target);
    data
}

/// A Safe pre-validated signature for `owner`: `r` = the owner address, `s` = 0, `v` = 1. Accepted
/// only when the transaction's sender is that owner.
pub fn prevalidated_signature(owner: Address) -> Vec<u8> {
    let mut signature = vec![0u8; 65];
    let owner: [u8; 20] = owner.into();
    signature[12..32].copy_from_slice(&owner);
    signature[64] = 0x01;
    signature
}

/// Safe `execTransaction(to, 0, data, CALL, 0, 0, 0, address(0), address(0), signatures)` calldata.
pub fn encode_exec_transaction(to: Address, data: &[u8], signatures: &[u8]) -> Vec<u8> {
    fn word_u64(value: u64) -> [u8; 32] {
        let mut word = [0u8; 32];
        word[24..].copy_from_slice(&value.to_be_bytes());
        word
    }
    fn word_address(address: Address) -> [u8; 32] {
        let mut word = [0u8; 32];
        let address: [u8; 20] = address.into();
        word[12..].copy_from_slice(&address);
        word
    }
    fn padded(bytes: &[u8]) -> Vec<u8> {
        let mut out = word_u64(bytes.len() as u64).to_vec();
        out.extend_from_slice(bytes);
        out.resize(out.len().div_ceil(32) * 32, 0);
        out
    }
    // Ten head words; the two `bytes` arguments are offsets into the tail.
    let head_len = 10 * 32;
    let data_tail = padded(data);
    let signatures_offset = head_len + data_tail.len();
    let mut out = Vec::with_capacity(4 + head_len + data_tail.len() + 32 + signatures.len() + 32);
    out.extend_from_slice(&EXEC_TRANSACTION_SELECTOR);
    out.extend_from_slice(&word_address(to)); // to
    out.extend_from_slice(&word_u64(0)); // value
    out.extend_from_slice(&word_u64(head_len as u64)); // data (offset)
    out.extend_from_slice(&word_u64(0)); // operation: CALL
    out.extend_from_slice(&word_u64(0)); // safeTxGas
    out.extend_from_slice(&word_u64(0)); // baseGas
    out.extend_from_slice(&word_u64(0)); // gasPrice
    out.extend_from_slice(&[0u8; 32]); // gasToken
    out.extend_from_slice(&[0u8; 32]); // refundReceiver
    out.extend_from_slice(&word_u64(signatures_offset as u64)); // signatures (offset)
    out.extend_from_slice(&data_tail);
    out.extend_from_slice(&padded(signatures));
    out
}

/// Whether a submission error says the target is already in the module's set — the module's
/// `TargetIsScoped()` revert — which is the outcome we wanted, reached earlier.
fn is_already_scoped(message: &str) -> bool {
    message.contains("TargetIsScoped")
}

/// Grants `safe`'s module the `aggregator` target, sending from the Safe's owner `owner_key`.
///
/// Fees are taken from Blokli the way the node's own connector takes them, so this transaction
/// is priced like every other one the node sends; the nonce is queried, never guessed.
pub async fn scope_aggregator<C>(
    client: &C,
    owner_key: &ChainKeypair,
    safe: Address,
    module: Address,
    aggregator: Address,
) -> anyhow::Result<()>
where
    C: BlokliQueryClient + BlokliTransactionClient + Send + Sync,
{
    let owner = owner_key.public().to_address();
    let scope = encode_scope_target_token(encode_target(aggregator));
    let calldata = encode_exec_transaction(module, &scope, &prevalidated_signature(owner));

    let info = client
        .query_chain_info()
        .await
        .context("querying chain info")?;
    let chain_id = u64::try_from(info.chain_id).context("Blokli reported a negative chain id")?;
    let max_fee_per_gas = info
        .max_fee_per_gas
        .as_deref()
        .and_then(|raw| raw.parse::<u128>().ok())
        .unwrap_or(FALLBACK_MAX_FEE_PER_GAS);
    let max_priority_fee_per_gas = info
        .max_priority_fee_per_gas
        .as_deref()
        .and_then(|raw| raw.parse::<u128>().ok())
        .unwrap_or(FALLBACK_MAX_PRIORITY_FEE_PER_GAS)
        .min(max_fee_per_gas);
    let nonce = client
        .query_transaction_count(&owner.into())
        .await
        .context("querying the owner's nonce")?;

    let signed = curvy_abi::sign_eip1559_call(curvy_abi::Eip1559Call {
        signer_secret: owner_key.secret().as_ref(),
        to: safe.into(),
        calldata,
        value: 0,
        nonce,
        gas_limit: GAS_LIMIT,
        max_fee_per_gas,
        max_priority_fee_per_gas,
        chain_id,
    })
    .context("signing the Safe transaction")?;

    match client.submit_and_confirm_transaction(&signed.0, 1).await {
        Ok(receipt) => {
            info!(%safe, %module, %aggregator, ?receipt, "scoped the Curvy aggregator into the node's module");
            Ok(())
        }
        Err(error) if is_already_scoped(&error.to_string()) => {
            info!(%safe, %module, %aggregator, "the Curvy aggregator was already scoped");
            Ok(())
        }
        Err(error) => Err(anyhow::anyhow!("submitting the Safe transaction: {error}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn addr(byte: u8) -> Address {
        Address::from([byte; 20])
    }

    fn hex(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{b:02x}")).collect()
    }

    // Vectors from `cast` (foundry 1.7.1) and from `scope-curvy-aggregator.sh --self-test`, not
    // from this code.

    #[test]
    fn the_target_word_matches_the_scripts_reference_vector() {
        assert_eq!(
            hex(&encode_target(addr(0xcc))),
            "cccccccccccccccccccccccccccccccccccccccc010003000000000000000000"
        );
        let mut one = [0u8; 20];
        one[19] = 1;
        assert_eq!(
            hex(&encode_target(Address::from(one))),
            "0000000000000000000000000000000000000001010003000000000000000000"
        );
    }

    #[test]
    fn the_selectors_match_cast_sig() {
        // cast sig 'scopeTargetToken(uint256)' / cast sig 'execTransaction(…)'
        assert_eq!(hex(&SCOPE_TARGET_TOKEN_SELECTOR), "a76c9a2f");
        assert_eq!(hex(&EXEC_TRANSACTION_SELECTOR), "6a761202");
    }

    #[test]
    fn exec_transaction_matches_cast_calldata() {
        // cast calldata 'execTransaction(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,bytes)' \
        //   0x1111…1111 0 0xa76c9a2f<target(0xcc…cc)> 0 0 0 0 0x0 0x0 <prevalidated(0xaa…aa)>
        let expected = concat!(
            "6a761202",
            "0000000000000000000000001111111111111111111111111111111111111111",
            "0000000000000000000000000000000000000000000000000000000000000000",
            "0000000000000000000000000000000000000000000000000000000000000140",
            "0000000000000000000000000000000000000000000000000000000000000000",
            "0000000000000000000000000000000000000000000000000000000000000000",
            "0000000000000000000000000000000000000000000000000000000000000000",
            "0000000000000000000000000000000000000000000000000000000000000000",
            "0000000000000000000000000000000000000000000000000000000000000000",
            "0000000000000000000000000000000000000000000000000000000000000000",
            "00000000000000000000000000000000000000000000000000000000000001a0",
            "0000000000000000000000000000000000000000000000000000000000000024",
            "a76c9a2fcccccccccccccccccccccccccccccccccccccccc010003000000000000000000",
            "00000000000000000000000000000000000000000000000000000000",
            "0000000000000000000000000000000000000000000000000000000000000041",
            "000000000000000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "0000000000000000000000000000000000000000000000000000000000000000",
            "0100000000000000000000000000000000000000000000000000000000000000",
        );
        let scope = encode_scope_target_token(encode_target(addr(0xcc)));
        let actual =
            encode_exec_transaction(addr(0x11), &scope, &prevalidated_signature(addr(0xaa)));
        assert_eq!(hex(&actual), expected);
    }

    #[test]
    fn a_prevalidated_signature_is_65_bytes_naming_the_owner() {
        let signature = prevalidated_signature(addr(0xaa));
        assert_eq!(signature.len(), 65);
        assert_eq!(&signature[..12], &[0u8; 12]);
        assert_eq!(&signature[12..32], &[0xaa; 20]);
        assert_eq!(&signature[32..64], &[0u8; 32]);
        assert_eq!(signature[64], 1);
    }
}
