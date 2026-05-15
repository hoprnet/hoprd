//! HOPR Smart Contracts Addresses dumper.
//!
//! This executable dumps the content of the build state of the contracts-addresses.json file.
//!
//! ## Help
//! ```shell
//! ➜   hopr-contract-addresses
//! {
//!   "networks": {
//!     "rotsee": {
//!      "addresses": {
//!        "announcements": "0xe45A8DBDADafd86AB7e2368b4749864083331cFC",
//!        "channels": "0x9680F21d4583ad27F2e6bc3005EDF120FFf057B7",
//!        "module_implementation": "0x79C3bF06E96A9373765284aFb0a35e9529E2B3F2",
//!        "node_safe_migration": "0xe9670B5D87c87111C9050D915971B73b9f5021a9",
//!        "node_safe_registry": "0xaFa257f2799835D2E29e7eC7ee448530F9d8Cb20",
//!        "node_stake_factory": "0xcB0841cc3DBDE97aC52B945F02Ee4F3D8707d977",
//!        "ticket_price_oracle": "0xFA251d4C367683d6181531afd5964E660aCf43A0",
//!        "token": "0xD4fdec44DB9D44B8f2b6d529620f9C0C7066A2c1",
//!        "winning_probability_oracle": "0xa641822a52AcbDc0c0123337f715C1d9756c21bD"
//!      },
//!      "chain_id": 100,
//!      "environment_type": "staging",
//!      "indexer_start_block_number": 42671336
//!    },
//!     "debug-staging": {
//!       "addresses": {
//!         "announcements": "0x5cE40E4b330eA20d9217f0120E3329b98017f9Fb",
//!         "channels": "0x09b77F370c2F379F3dB42B799fa7EDBB37D9E51a",
//!         "module_implementation": "0x4db69d5D0cCaD110a6F327bc837fe5B7e1ffBDAe",
//!         "node_safe_migration": "0x6E3Ef29b17FB33C1db372b06b19CE937894B317A",
//!         "node_safe_registry": "0x549DCe2Ce8ba1c177B080D372580fa4d85123671",
//!         "node_stake_factory": "0x87e1EBD42F0F79d88E5af212D0c24dc155EA78C0",
//!         "ticket_price_oracle": "0x344f8DD6dBCd5Bc0B396dDD47F20FDc9A89c3c90",
//!         "token": "0x02e1009fd222917Ee7bdfdBF8AE1e56c4ae3F6f3",
//!         "winning_probability_oracle": "0xF2d64bb29A5207DFCF6185161e43364D3B2D9A27"
//!       },
//!       "chain_id": 100,
//!       "environment_type": "staging",
//!       "indexer_start_block_number": 29690235
//!     }
//!   }
//! }
//! ```

fn main() -> anyhow::Result<()> {
    println!("{}", hopr_bindings::CONTRACTS_ADDRESSES_FILE_CONTENT);

    Ok(())
}
