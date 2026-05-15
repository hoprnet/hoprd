///Module containing a contract's types and functions.
/**

```solidity
library HoprLedger {
    struct RootStruct { bytes28 rootHash; uint32 timestamp; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod HoprLedger {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct RootStruct { bytes28 rootHash; uint32 timestamp; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RootStruct {
        #[allow(missing_docs)]
        pub rootHash: alloy::sol_types::private::FixedBytes<28>,
        #[allow(missing_docs)]
        pub timestamp: u32,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::FixedBytes<28>,
            alloy::sol_types::sol_data::Uint<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<28>, u32);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<RootStruct> for UnderlyingRustTuple<'_> {
            fn from(value: RootStruct) -> Self {
                (value.rootHash, value.timestamp)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RootStruct {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    rootHash: tuple.0,
                    timestamp: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for RootStruct {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for RootStruct {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        28,
                    > as alloy_sol_types::SolType>::tokenize(&self.rootHash),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.timestamp),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for RootStruct {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for RootStruct {
            const NAME: &'static str = "RootStruct";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "RootStruct(bytes28 rootHash,uint32 timestamp)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::FixedBytes<
                        28,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.rootHash)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.timestamp)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for RootStruct {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        28,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.rootHash,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.timestamp,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    28,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.rootHash,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.timestamp,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`HoprLedger`](self) contract instance.

See the [wrapper's documentation](`HoprLedgerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> HoprLedgerInstance<P, N> {
        HoprLedgerInstance::<P, N>::new(address, __provider)
    }
    /**A [`HoprLedger`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`HoprLedger`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct HoprLedgerInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for HoprLedgerInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("HoprLedgerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > HoprLedgerInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`HoprLedger`](self) contract instance.

See the [wrapper's documentation](`HoprLedgerInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            __provider: P,
        ) -> Self {
            Self {
                address,
                provider: __provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<P: ::core::clone::Clone, N> HoprLedgerInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> HoprLedgerInstance<P, N> {
            HoprLedgerInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > HoprLedgerInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > HoprLedgerInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library HoprLedger {
    struct RootStruct {
        bytes28 rootHash;
        uint32 timestamp;
    }
}

interface HoprAnnouncements {
    type KeyId is uint32;
    struct KeyBindingWithSignatureTimestamp {
        bytes32 ed25519_sig_0;
        bytes32 ed25519_sig_1;
        bytes32 ed25519_pub_key;
        address chain_key;
        uint256 boundTimestamp;
    }

    error AddressEmptyCode(address target);
    error AlreadyInitialized();
    error ContractNotResponsible();
    error ERC1967InvalidImplementation(address implementation);
    error ERC1967NonPayable();
    error EmptyMultiaddr();
    error ExistingKeyBinding();
    error FailedCall();
    error InvalidInitialization();
    error InvalidKeyBindingFeeAmount();
    error InvalidSafeAddress();
    error InvalidTokenRecipient();
    error InvalidTokenSender();
    error InvalidTokensReceivedUsage();
    error KeyIdOutOfRange();
    error MultiSigUninitialized();
    error NoNeedToProvideKeyBindingFee();
    error NotInitializing();
    error OwnableInvalidOwner(address owner);
    error OwnableUnauthorizedAccount(address account);
    error UUPSUnauthorizedCallContext();
    error UUPSUnsupportedProxiableUUID(bytes32 slot);
    error WrongToken();
    error ZeroAddress(string reason);
    error ZeroInterval();

    event AddressAnnouncement(address node, string baseMultiaddr);
    event Initialized(uint64 version);
    event KeyBinding(bytes32 ed25519_sig_0, bytes32 ed25519_sig_1, bytes32 ed25519_pub_key, address chain_key, uint256 key_id);
    event KeyBindingFeeUpdate(uint256 newFee, uint256 oldFee);
    event LedgerDomainSeparatorUpdated(bytes32 indexed ledgerDomainSeparator);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event RevokeAnnouncement(address node);
    event Upgraded(address indexed implementation);

    constructor();

    function ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZE() external view returns (uint256);
    function LEDGER_VERSION() external view returns (string memory);
    function SNAPSHOT_INTERVAL() external view returns (uint256);
    function TOKEN() external view returns (address);
    function TOKENS_RECIPIENT_INTERFACE_HASH() external view returns (bytes32);
    function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
    function announce(string memory baseMultiaddr) external;
    function announceSafe(address selfAddress, string memory baseMultiaddr) external;
    function getAllKeyBindings() external view returns (KeyBindingWithSignatureTimestamp[] memory);
    function getKeyBindingCount() external view returns (uint256);
    function getKeyBindingWithKeyId(KeyId keyId) external view returns (KeyBindingWithSignatureTimestamp memory);
    function getKeyIdRange() external pure returns (uint32 minKeyId, uint32 maxKeyId);
    function getKeyIdWithOffchainKey(bytes32 ed25519_pub_key) external view returns (bool, KeyId);
    function getOffchainKeyWithKeyId(KeyId keyId) external view returns (bytes32 ed25519_pub_key);
    function initialize(bytes memory initParams) external;
    function isOffchainKeyBound(bytes32 ed25519_pub_key) external view returns (bool);
    function keyBindingFee() external view returns (uint256);
    function latestRoot() external view returns (HoprLedger.RootStruct memory);
    function latestSnapshotRoot() external view returns (HoprLedger.RootStruct memory);
    function ledgerDomainSeparator() external view returns (bytes32);
    function multiaddrOf(address) external view returns (string memory);
    function owner() external view returns (address);
    function proxiableUUID() external view returns (bytes32);
    function renounceOwnership() external;
    function revoke() external;
    function revokeSafe(address selfAddress) external;
    function tokensReceived(address, address from, address to, uint256 amount, bytes memory userData, bytes memory) external;
    function transferOwnership(address newOwner) external;
    function tryGetKeyBinding(bytes32 ed25519_pub_key) external view returns (bool, KeyId, KeyBindingWithSignatureTimestamp memory);
    function updateKeyBindingFee(uint256 newFee) external;
    function updateLedgerDomainSeparator() external;
    function upgradeToAndCall(address newImplementation, bytes memory data) external payable;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZE",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "LEDGER_VERSION",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "SNAPSHOT_INTERVAL",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "TOKEN",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IERC777"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "TOKENS_RECIPIENT_INTERFACE_HASH",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "UPGRADE_INTERFACE_VERSION",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "announce",
    "inputs": [
      {
        "name": "baseMultiaddr",
        "type": "string",
        "internalType": "string"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "announceSafe",
    "inputs": [
      {
        "name": "selfAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "baseMultiaddr",
        "type": "string",
        "internalType": "string"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getAllKeyBindings",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct KeyBindingWithSignatureTimestamp[]",
        "components": [
          {
            "name": "ed25519_sig_0",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "ed25519_sig_1",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "ed25519_pub_key",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "chain_key",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "boundTimestamp",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getKeyBindingCount",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getKeyBindingWithKeyId",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint32",
        "internalType": "KeyId"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct KeyBindingWithSignatureTimestamp",
        "components": [
          {
            "name": "ed25519_sig_0",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "ed25519_sig_1",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "ed25519_pub_key",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "chain_key",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "boundTimestamp",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getKeyIdRange",
    "inputs": [],
    "outputs": [
      {
        "name": "minKeyId",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "maxKeyId",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "getKeyIdWithOffchainKey",
    "inputs": [
      {
        "name": "ed25519_pub_key",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "",
        "type": "uint32",
        "internalType": "KeyId"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOffchainKeyWithKeyId",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint32",
        "internalType": "KeyId"
      }
    ],
    "outputs": [
      {
        "name": "ed25519_pub_key",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initialize",
    "inputs": [
      {
        "name": "initParams",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "isOffchainKeyBound",
    "inputs": [
      {
        "name": "ed25519_pub_key",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "keyBindingFee",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "latestRoot",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct HoprLedger.RootStruct",
        "components": [
          {
            "name": "rootHash",
            "type": "bytes28",
            "internalType": "bytes28"
          },
          {
            "name": "timestamp",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "latestSnapshotRoot",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct HoprLedger.RootStruct",
        "components": [
          {
            "name": "rootHash",
            "type": "bytes28",
            "internalType": "bytes28"
          },
          {
            "name": "timestamp",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "ledgerDomainSeparator",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "multiaddrOf",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "owner",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "proxiableUUID",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "renounceOwnership",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "revoke",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "revokeSafe",
    "inputs": [
      {
        "name": "selfAddress",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "tokensReceived",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "from",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "userData",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "transferOwnership",
    "inputs": [
      {
        "name": "newOwner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "tryGetKeyBinding",
    "inputs": [
      {
        "name": "ed25519_pub_key",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "",
        "type": "uint32",
        "internalType": "KeyId"
      },
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct KeyBindingWithSignatureTimestamp",
        "components": [
          {
            "name": "ed25519_sig_0",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "ed25519_sig_1",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "ed25519_pub_key",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "chain_key",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "boundTimestamp",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "updateKeyBindingFee",
    "inputs": [
      {
        "name": "newFee",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateLedgerDomainSeparator",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "upgradeToAndCall",
    "inputs": [
      {
        "name": "newImplementation",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "event",
    "name": "AddressAnnouncement",
    "inputs": [
      {
        "name": "node",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "baseMultiaddr",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Initialized",
    "inputs": [
      {
        "name": "version",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "KeyBinding",
    "inputs": [
      {
        "name": "ed25519_sig_0",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "ed25519_sig_1",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "ed25519_pub_key",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "chain_key",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "key_id",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "KeyBindingFeeUpdate",
    "inputs": [
      {
        "name": "newFee",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "oldFee",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "LedgerDomainSeparatorUpdated",
    "inputs": [
      {
        "name": "ledgerDomainSeparator",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OwnershipTransferred",
    "inputs": [
      {
        "name": "previousOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RevokeAnnouncement",
    "inputs": [
      {
        "name": "node",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Upgraded",
    "inputs": [
      {
        "name": "implementation",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "AddressEmptyCode",
    "inputs": [
      {
        "name": "target",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "AlreadyInitialized",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ContractNotResponsible",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ERC1967InvalidImplementation",
    "inputs": [
      {
        "name": "implementation",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC1967NonPayable",
    "inputs": []
  },
  {
    "type": "error",
    "name": "EmptyMultiaddr",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ExistingKeyBinding",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FailedCall",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidInitialization",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidKeyBindingFeeAmount",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidSafeAddress",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidTokenRecipient",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidTokenSender",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidTokensReceivedUsage",
    "inputs": []
  },
  {
    "type": "error",
    "name": "KeyIdOutOfRange",
    "inputs": []
  },
  {
    "type": "error",
    "name": "MultiSigUninitialized",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NoNeedToProvideKeyBindingFee",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotInitializing",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OwnableInvalidOwner",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "OwnableUnauthorizedAccount",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "UUPSUnauthorizedCallContext",
    "inputs": []
  },
  {
    "type": "error",
    "name": "UUPSUnsupportedProxiableUUID",
    "inputs": [
      {
        "name": "slot",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "WrongToken",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ZeroAddress",
    "inputs": [
      {
        "name": "reason",
        "type": "string",
        "internalType": "string"
      }
    ]
  },
  {
    "type": "error",
    "name": "ZeroInterval",
    "inputs": []
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod HoprAnnouncements {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x306080525f805460ff60a01b1916815560e081905261010081905261012081905261014081905260a06101608190526101809190915260c08080526101a06040529052662386f26fc10000600755348015610058575f5ffd5b506201518061006681610074565b5061006f61010b565b6102b5565b805f036100945760405163346ff60760e01b815260040160405180910390fd5b60018190556040516001600160601b03193060601b16602082015260340160408051808303601f190181529190528051602091820120901c600160e01b4263ffffffff90811682029290921760028190556001600160e01b038082169183900490931690910217600355610108906101bc16565b50565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00805468010000000000000000900460ff161561015b5760405163f92ee8a960e01b815260040160405180910390fd5b80546001600160401b03908116146101085780546001600160401b0319166001600160401b0390811782556040519081527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a150565b604080518082018252600a8152692437b8392632b233b2b960b11b6020918201528151808301835260058152640322e302e360dc1b9082015281517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f81527f6cd681790c78c220517b099a737f8e85f69e797abe4e2910fb189b61db4bf2cd918101919091527fb4bcb154e38601c389396fa918314da42d4626f13ef6d0ceb07e5f5d26b2fbc39181019190915246606082015230608082015260a09020600454811461010857600481905560405181907fa43fad83920fd09445855e854e73c9c532e17402c9ceb09993a2392843a5bdb9905f90a250565b60805160a0516128076102eb5f395f8181610622015261072001525f81816118ef015281816119180152611a5c01526128075ff3fe6080604052600436106101da575f3560e01c80638da5cb5b116100fd578063dc96fd5011610092578063f2fde38b11610062578063f2fde38b14610663578063f83e429214610682578063fad0e5a214610696578063fd153acf146106b5575f5ffd5b8063dc96fd50146105cd578063ddad1902146105e1578063e9d3ee9514610611578063ea0a523714610644575f5ffd5b8063ad3cb1cc116100cd578063ad3cb1cc14610528578063b6549f7514610558578063c966c4fe1461056c578063d7b0fef114610581575f5ffd5b80638da5cb5b14610463578063a2a077561461049f578063a8b4eec9146104cd578063a969635a146104f9575f5ffd5b806352d1902d11610173578063715018a611610143578063715018a6146103c257806372581cc0146103d6578063773b4a331461040957806382bfefc81461042c575f5ffd5b806352d1902d1461033257806353665aaa14610346578063604634c9146103725780636d2beef1146103ad575f5ffd5b8063308c712e116101ae578063308c712e146102b4578063439fab91146102d35780634ac3e4f2146102f25780634f1ef2861461031f575f5ffd5b806223de29146101de5780630df18f94146101ff57806310ab529714610274578063244d496e14610295575b5f5ffd5b3480156101e9575f5ffd5b506101fd6101f836600461209a565b6106ca565b005b34801561020a575f5ffd5b506040805180820182525f808252602091820152815180830190925260035463ffffffff1981831b168352600160e01b900463ffffffff16908201525b60408051825163ffffffff1916815260209283015163ffffffff1692810192909252015b60405180910390f35b34801561027f575f5ffd5b5061028861094f565b60405161026b919061217d565b3480156102a0575f5ffd5b506101fd6102af3660046121ca565b610960565b3480156102bf575f5ffd5b506101fd6102ce3660046121e1565b610974565b3480156102de575f5ffd5b506101fd6102ed3660046122a3565b610a3e565b3480156102fd575f5ffd5b5061031161030c3660046122dc565b610d2a565b60405190815260200161026b565b6101fd61032d3660046122ff565b610d4e565b34801561033d575f5ffd5b50610311610d69565b348015610351575f5ffd5b506103656103603660046121e1565b610d84565b60405161026b9190612379565b34801561037d575f5ffd5b5061039161038c3660046121ca565b610e1b565b60408051921515835263ffffffff90911660208301520161026b565b3480156103b8575f5ffd5b5061031160015481565b3480156103cd575f5ffd5b506101fd610e37565b3480156103e1575f5ffd5b506103117fb281fc8c12954d22544db45de3159a39272895b169a852b314f9cc762e44c53b81565b348015610414575f5ffd5b50604080515f815263ffffffff60208201520161026b565b348015610437575f5ffd5b5060085461044b906001600160a01b031681565b6040516001600160a01b03909116815260200161026b565b34801561046e575f5ffd5b507f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300546001600160a01b031661044b565b3480156104aa575f5ffd5b506104be6104b93660046121ca565b610e4a565b60405161026b9392919061238b565b3480156104d8575f5ffd5b506104ec6104e73660046122dc565b610e72565b60405161026b91906123ad565b348015610504575f5ffd5b506105186105133660046121ca565b610e99565b604051901515815260200161026b565b348015610533575f5ffd5b50610365604051806040016040528060058152602001640352e302e360dc1b81525081565b348015610563575f5ffd5b506101fd610eb0565b348015610577575f5ffd5b5061031160045481565b34801561058c575f5ffd5b506040805180820182525f808252602091820152815180830190925260025463ffffffff1981831b168352600160e01b900463ffffffff1690820152610247565b3480156105d8575f5ffd5b506101fd610f73565b3480156105ec575f5ffd5b50610365604051806040016040528060058152602001640322e302e360dc1b81525081565b34801561061c575f5ffd5b506103117f000000000000000000000000000000000000000000000000000000000000000081565b34801561064f575f5ffd5b506101fd61065e3660046123bb565b61106c565b34801561066e575f5ffd5b506101fd61067d3660046121e1565b611165565b34801561068d575f5ffd5b5061031161119f565b3480156106a1575f5ffd5b506101fd6106b03660046123f9565b6111a9565b3480156106c0575f5ffd5b5061031160075481565b6008546001600160a01b031633146106f557604051635079ff7560e11b815260040160405180910390fd5b6001600160a01b038616301461071e57604051631738922160e31b815260040160405180910390fd5b7f000000000000000000000000000000000000000000000000000000000000000083101561075f57604051630d3dcde560e31b815260040160405180910390fd5b5f80808080610770888a018a612449565b5f80546040516302265e3160e61b81526001600160a01b038089166004830152979c50959a5093985091965094509216906389978c4090602401602060405180830381865afa1580156107c5573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107e991906124c3565b90506001600160a01b0381161580156108145750856001600160a01b03168d6001600160a01b031614155b8061084457506001600160a01b038116158015906108445750806001600160a01b03168d6001600160a01b031614155b1561086257604051634a93b62160e11b815260040160405180910390fd5b5f61086f878787876112ab565b509050801561089f576007548c1461089a57604051634732cefd60e01b815260040160405180910390fd5b6108be565b8b156108be576040516303689fb160e21b815260040160405180910390fd5b8251156108cf576108cf87846113ec565b8b1561093e576008546040805163fe9d930360e01b8152600481018f905260248101919091525f60448201526001600160a01b039091169063fe9d9303906064015f604051808303815f87803b158015610927575f5ffd5b505af1158015610939573d5f5f3e3d5ffd5b505050505b505050505050505050505050505050565b606061095b600561151a565b905090565b6109686115af565b6109718161160a565b50565b5f548190600160a01b900460ff1661099f576040516308a9441960e31b815260040160405180910390fd5b5f546040516302265e3160e61b81526001600160a01b038381166004830152339216906389978c4090602401602060405180830381865afa1580156109e6573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a0a91906124c3565b6001600160a01b031614610a315760405163acd5a82360e01b815260040160405180910390fd5b610a3a8261168b565b5050565b5f610a47611736565b805490915060ff600160401b82041615906001600160401b03165f81158015610a6d5750825b90505f826001600160401b03166001148015610a885750303b155b905081158015610a96575080155b15610ab45760405163f92ee8a960e01b815260040160405180910390fd5b845467ffffffffffffffff191660011785558315610ade57845460ff60401b1916600160401b1785555b610aea6201518061175e565b5f5f5f5f89806020019051810190610b0291906124de565b929650909450925090506001600160a01b038416610b685760405163eac0d38960e01b815260206004820152601860248201527f5f746f6b656e206d757374206e6f7420626520656d707479000000000000000060448201526064015b60405180910390fd5b6001600160a01b038316610bbf5760405163eac0d38960e01b815260206004820152601f60248201527f5f736166655265676973747279206d757374206e6f7420626520656d707479006044820152606401610b5f565b6001600160a01b038116610c165760405163eac0d38960e01b815260206004820152601860248201527f5f6f776e6572206d757374206e6f7420626520656d70747900000000000000006044820152606401610b5f565b600880546001600160a01b0319166001600160a01b038616179055610c3a836117ed565b6040516329965a1d60e01b815230600482018190527fb281fc8c12954d22544db45de3159a39272895b169a852b314f9cc762e44c53b60248301526044820152731820a4b7618bde71dce8cdc73aab6c95905fad24906329965a1d906064015f604051808303815f87803b158015610cb0575f5ffd5b505af1158015610cc2573d5f5f3e3d5ffd5b50505050610ccf8261160a565b610cd881611865565b505050508315610d2257845460ff60401b19168555604051600181527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a15b505050505050565b5f63ffffffff80831690610d4390600590839061186d16565b604001519392505050565b610d566118e4565b610d5f82611988565b610a3a8282611990565b5f610d72611a51565b505f5160206127b25f395f51905f5290565b60096020525f908152604090208054610d9c90612530565b80601f0160208091040260200160405190810160405280929190818152602001828054610dc890612530565b8015610e135780601f10610dea57610100808354040283529160200191610e13565b820191905f5260205f20905b815481529060010190602001808311610df657829003601f168201915b505050505081565b5f808080610e2a600586611a9a565b5090969095509350505050565b610e3f6115af565b610e485f611b82565b565b5f5f610e54611fc5565b5f8080610e62600588611a9a565b9199909850909650945050505050565b610e7a611fc5565b63ffffffff80831690610e9290600590839061186d16565b9392505050565b5f8181526006602052604081205415155b92915050565b5f54600160a01b900460ff16610ed9576040516308a9441960e31b815260040160405180910390fd5b5f80546040516302265e3160e61b81523360048201526001600160a01b03909116906389978c4090602401602060405180830381865afa158015610f1f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f4391906124c3565b6001600160a01b031614610f6a5760405163acd5a82360e01b815260040160405180910390fd5b610e483361168b565b604080518082018252600a8152692437b8392632b233b2b960b11b6020918201528151808301835260058152640322e302e360dc1b9082015281517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f81527f6cd681790c78c220517b099a737f8e85f69e797abe4e2910fb189b61db4bf2cd918101919091527fb4bcb154e38601c389396fa918314da42d4626f13ef6d0ceb07e5f5d26b2fbc39181019190915246606082015230608082015260a09020600454811461097157600481905560405181907fa43fad83920fd09445855e854e73c9c532e17402c9ceb09993a2392843a5bdb9905f90a250565b5f54600160a01b900460ff16611095576040516308a9441960e31b815260040160405180910390fd5b5f80546040516302265e3160e61b81523360048201526001600160a01b03909116906389978c4090602401602060405180830381865afa1580156110db573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110ff91906124c3565b6001600160a01b0316146111265760405163acd5a82360e01b815260040160405180910390fd5b610a3a3383838080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152506113ec92505050565b61116d6115af565b6001600160a01b03811661119657604051631e4fbdf760e01b81525f6004820152602401610b5f565b61097181611b82565b5f61095b60055490565b5f548390600160a01b900460ff166111d4576040516308a9441960e31b815260040160405180910390fd5b5f546040516302265e3160e61b81526001600160a01b038381166004830152339216906389978c4090602401602060405180830381865afa15801561121b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061123f91906124c3565b6001600160a01b0316146112665760405163acd5a82360e01b815260040160405180910390fd5b6112a58484848080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152506113ec92505050565b50505050565b5f8080806112ba600586611a9a565b509150915081156112d1575f935091506113e39050565b61130e60405180608001604052808981526020018881526020018781526020018a6001600160a01b03168152506005611bf290919063ffffffff16565b604080517f11d94653de52fc143887956fe4b5e4120a850b1300ece0533dee6bd8ff91f73260208201529081018990526060808201899052608082018890528a901b6001600160601b03191660a082015260b481018290529093506113859060d4015b604051602081830303815290604052611cf4565b60408051888152602081018890529081018690526001600160a01b0389166060820152608081018490527f11d94653de52fc143887956fe4b5e4120a850b1300ece0533dee6bd8ff91f7329060a00160405180910390a16001935050505b94509492505050565b80515f0361140c57604051629ceac760e41b815260040160405180910390fd5b6001600160a01b0382165f908152600960205260409020805461142e90612530565b158015915061147a5750808051906020012060095f846001600160a01b03166001600160a01b031681526020019081526020015f206040516114709190612568565b6040518091039020145b15611483575050565b6001600160a01b0382165f9081526009602052604090206114a48282612624565b506114dd7fc4df5ba16814838ab2618829d68f8623bb897302f24dbdba2279dbe45adb3d148383604051602001611371939291906126f5565b7fc4df5ba16814838ab2618829d68f8623bb897302f24dbdba2279dbe45adb3d14828260405161150e92919061271e565b60405180910390a15050565b6060815f01805480602002602001604051908101604052809291908181526020015f905b828210156115a4575f8481526020908190206040805160a08101825260058602909201805483526001808201548486015260028201549284019290925260038101546001600160a01b03166060840152600401546080830152908352909201910161153e565b505050509050919050565b336115e17f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300546001600160a01b031690565b6001600160a01b031614610e485760405163118cdaa760e01b8152336004820152602401610b5f565b6007805490829055604080517f6925945cb4c6b597a361809d5b51107e4e03720404a82483853e91faa176b88e60208201529081018390526060810182905261165590608001611371565b60408051838152602081018390527f6925945cb4c6b597a361809d5b51107e4e03720404a82483853e91faa176b88e910161150e565b6001600160a01b0381165f9081526009602052604081206116ab91611ff8565b6116f77fa4de30a528becadf82649d1395c0e30dd18ae35b5a96ce71e9295bb14bc9f3bc8260405160200161137192919091825260601b6001600160601b031916602082015260340190565b6040516001600160a01b03821681527fa4de30a528becadf82649d1395c0e30dd18ae35b5a96ce71e9295bb14bc9f3bc9060200160405180910390a150565b5f807ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00610eaa565b805f0361177e5760405163346ff60760e01b815260040160405180910390fd5b60018190556040516001600160601b03193060601b16602082015260340160408051808303601f190181529190528051602091820120901c600160e01b4263ffffffff90811682029290921760028190556001600160e01b038116908290049092160217600355610971610f73565b5f54600160a01b900460ff16156118165760405162dc149f60e41b815260040160405180910390fd5b6001600160a01b03811661183d5760405163474ebe2f60e11b815260040160405180910390fd5b5f80546001600160a01b039092166001600160a81b031990921691909117600160a01b179055565b61116d611dba565b611875611fc5565b825f01828154811061188957611889612741565b5f9182526020918290206040805160a08101825260059093029091018054835260018101549383019390935260028301549082015260038201546001600160a01b031660608201526004909101546080820152905092915050565b306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016148061196a57507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031661195e5f5160206127b25f395f51905f52546001600160a01b031690565b6001600160a01b031614155b15610e485760405163703e46dd60e11b815260040160405180910390fd5b6109716115af565b816001600160a01b03166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa9250505080156119ea575060408051601f3d908101601f191682019092526119e791810190612755565b60015b611a1257604051634c9c8ce360e01b81526001600160a01b0383166004820152602401610b5f565b5f5160206127b25f395f51905f528114611a4257604051632a87526960e21b815260048101829052602401610b5f565b611a4c8383611ddf565b505050565b306001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614610e485760405163703e46dd60e11b815260040160405180910390fd5b5f5f611aa4611fc5565b5f84815260018601602052604081205490819003611af25750506040805160a0810182525f808252602082018190529181018290526060810182905260808101829052909250829150611b7b565b6001611afe8183612780565b87611b0a600185612780565b81548110611b1a57611b1a612741565b5f9182526020918290206040805160a08101825260059093029091018054835260018101549383019390935260028301549082015260038201546001600160a01b03166060820152600490910154608082015291955093509150611b7b9050565b9250925092565b7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930080546001600160a01b031981166001600160a01b03848116918217845560405192169182907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a3505050565b81545f9063ffffffff11611c19576040516310953bff60e11b815260040160405180910390fd5b6040808301515f90815260018501602052205415611c495760405162584c6f60e51b815260040160405180910390fd5b6040805160a0810182528351815260208085015181830190815285840180518486019081526060808901516001600160a01b0390811691870191825242608088019081528b5460018082018e555f8e81528981209a516005909302909a019182559651818801559351600285015591516003840180546001600160a01b031916919092161790555160049091015587549051845281880190925292909120819055610e929190612780565b6001546002545f91611d1291600160e01b900463ffffffff16612793565b421115611d1d575060015b600454600254835160208086019190912060408051808401959095524360e01b6001600160e01b0319169085015291901b63ffffffff19166044830152606082015260800160408051601f19818403018152919052805160209182012063ffffffff4216600160e01b02911c176002558015610a3a5750506002546001600160e01b038116600160e01b9182900463ffffffff1690910217600355565b611dc2611e34565b610e4857604051631afcd79f60e31b815260040160405180910390fd5b611de882611e4d565b6040516001600160a01b038316907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b905f90a2805115611e2c57611a4c8282611eb0565b610a3a611f22565b5f611e3d611736565b54600160401b900460ff16919050565b806001600160a01b03163b5f03611e8257604051634c9c8ce360e01b81526001600160a01b0382166004820152602401610b5f565b5f5160206127b25f395f51905f5280546001600160a01b0319166001600160a01b0392909216919091179055565b60605f5f846001600160a01b031684604051611ecc91906127a6565b5f60405180830381855af49150503d805f8114611f04576040519150601f19603f3d011682016040523d82523d5f602084013e611f09565b606091505b5091509150611f19858383611f41565b95945050505050565b3415610e485760405163b398979f60e01b815260040160405180910390fd5b606082611f5657611f5182611f9d565b610e92565b8151158015611f6d57506001600160a01b0384163b155b15611f9657604051639996b31560e01b81526001600160a01b0385166004820152602401610b5f565b5092915050565b805115611fac57805160208201fd5b60405163d6bda27560e01b815260040160405180910390fd5b6040518060a001604052805f81526020015f81526020015f81526020015f6001600160a01b031681526020015f81525090565b50805461200490612530565b5f825580601f10612013575050565b601f0160209004905f5260205f209081019061097191905b8082111561203e575f815560010161202b565b5090565b6001600160a01b0381168114610971575f5ffd5b5f5f83601f840112612066575f5ffd5b5081356001600160401b0381111561207c575f5ffd5b602083019150836020828501011115612093575f5ffd5b9250929050565b5f5f5f5f5f5f5f5f60c0898b0312156120b1575f5ffd5b88356120bc81612042565b975060208901356120cc81612042565b965060408901356120dc81612042565b95506060890135945060808901356001600160401b038111156120fd575f5ffd5b6121098b828c01612056565b90955093505060a08901356001600160401b03811115612127575f5ffd5b6121338b828c01612056565b999c989b5096995094979396929594505050565b8051825260208082015190830152604080820151908301526060808201516001600160a01b031690830152608090810151910152565b602080825282518282018190525f918401906040840190835b818110156121bf576121a9838551612147565b6020939093019260a09290920191600101612196565b509095945050505050565b5f602082840312156121da575f5ffd5b5035919050565b5f602082840312156121f1575f5ffd5b8135610e9281612042565b634e487b7160e01b5f52604160045260245ffd5b5f5f6001600160401b03841115612229576122296121fc565b50604051601f19601f85018116603f011681018181106001600160401b0382111715612257576122576121fc565b60405283815290508082840185101561226e575f5ffd5b838360208301375f60208583010152509392505050565b5f82601f830112612294575f5ffd5b610e9283833560208501612210565b5f602082840312156122b3575f5ffd5b81356001600160401b038111156122c8575f5ffd5b6122d484828501612285565b949350505050565b5f602082840312156122ec575f5ffd5b813563ffffffff81168114610e92575f5ffd5b5f5f60408385031215612310575f5ffd5b823561231b81612042565b915060208301356001600160401b03811115612335575f5ffd5b61234185828601612285565b9150509250929050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b602081525f610e92602083018461234b565b831515815263ffffffff8316602082015260e081016122d46040830184612147565b60a08101610eaa8284612147565b5f5f602083850312156123cc575f5ffd5b82356001600160401b038111156123e1575f5ffd5b6123ed85828601612056565b90969095509350505050565b5f5f5f6040848603121561240b575f5ffd5b833561241681612042565b925060208401356001600160401b03811115612430575f5ffd5b61243c86828701612056565b9497909650939450505050565b5f5f5f5f5f60a0868803121561245d575f5ffd5b853561246881612042565b945060208601359350604086013592506060860135915060808601356001600160401b03811115612497575f5ffd5b8601601f810188136124a7575f5ffd5b6124b688823560208401612210565b9150509295509295909350565b5f602082840312156124d3575f5ffd5b8151610e9281612042565b5f5f5f5f608085870312156124f1575f5ffd5b84516124fc81612042565b602086015190945061250d81612042565b60408601516060870151919450925061252581612042565b939692955090935050565b600181811c9082168061254457607f821691505b60208210810361256257634e487b7160e01b5f52602260045260245ffd5b50919050565b5f5f835461257581612530565b60018216801561258c57600181146125a1576125ce565b60ff19831686528115158202860193506125ce565b865f5260205f205f5b838110156125c6578154888201526001909101906020016125aa565b505081860193505b509195945050505050565b601f821115611a4c57805f5260205f20601f840160051c810160208510156125fe5750805b601f840160051c820191505b8181101561261d575f815560010161260a565b5050505050565b81516001600160401b0381111561263d5761263d6121fc565b6126518161264b8454612530565b846125d9565b6020601f821160018114612683575f831561266c5750848201515b5f19600385901b1c1916600184901b17845561261d565b5f84815260208120601f198516915b828110156126b25787850151825560209485019460019092019101612692565b50848210156126cf57868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b5f81518060208401855e5f93019283525090919050565b8381526bffffffffffffffffffffffff198360601b1660208201525f611f1960348301846126de565b6001600160a01b03831681526040602082018190525f906122d49083018461234b565b634e487b7160e01b5f52603260045260245ffd5b5f60208284031215612765575f5ffd5b5051919050565b634e487b7160e01b5f52601160045260245ffd5b81810381811115610eaa57610eaa61276c565b80820180821115610eaa57610eaa61276c565b5f610e9282846126de56fe360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbca264697066735822122056536b14d553d1f74e6ef43eb222325ae846761f66804cdc0b48a8eb321c01d764736f6c634300081e0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"0`\x80R_\x80T`\xFF`\xA0\x1B\x19\x16\x81U`\xE0\x81\x90Ra\x01\0\x81\x90Ra\x01 \x81\x90Ra\x01@\x81\x90R`\xA0a\x01`\x81\x90Ra\x01\x80\x91\x90\x91R`\xC0\x80\x80Ra\x01\xA0`@R\x90Rf#\x86\xF2o\xC1\0\0`\x07U4\x80\x15a\0XW__\xFD[Pb\x01Q\x80a\0f\x81a\0tV[Pa\0oa\x01\x0BV[a\x02\xB5V[\x80_\x03a\0\x94W`@Qc4o\xF6\x07`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x81\x90U`@Q`\x01`\x01``\x1B\x03\x190``\x1B\x16` \x82\x01R`4\x01`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x91\x82\x01 \x90\x1C`\x01`\xE0\x1BBc\xFF\xFF\xFF\xFF\x90\x81\x16\x82\x02\x92\x90\x92\x17`\x02\x81\x90U`\x01`\x01`\xE0\x1B\x03\x80\x82\x16\x91\x83\x90\x04\x90\x93\x16\x90\x91\x02\x17`\x03Ua\x01\x08\x90a\x01\xBC\x16V[PV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80Th\x01\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a\x01[W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80T`\x01`\x01`@\x1B\x03\x90\x81\x16\x14a\x01\x08W\x80T`\x01`\x01`@\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x90\x81\x17\x82U`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`@\x80Q\x80\x82\x01\x82R`\n\x81Ri$7\xB89&2\xB23\xB2\xB9`\xB1\x1B` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`\x05\x81Rd\x03\"\xE3\x02\xE3`\xDC\x1B\x90\x82\x01R\x81Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81R\x7Fl\xD6\x81y\x0Cx\xC2 Q{\t\x9As\x7F\x8E\x85\xF6\x9Eyz\xBEN)\x10\xFB\x18\x9Ba\xDBK\xF2\xCD\x91\x81\x01\x91\x90\x91R\x7F\xB4\xBC\xB1T\xE3\x86\x01\xC3\x899o\xA9\x181M\xA4-F&\xF1>\xF6\xD0\xCE\xB0~_]&\xB2\xFB\xC3\x91\x81\x01\x91\x90\x91RF``\x82\x01R0`\x80\x82\x01R`\xA0\x90 `\x04T\x81\x14a\x01\x08W`\x04\x81\x90U`@Q\x81\x90\x7F\xA4?\xAD\x83\x92\x0F\xD0\x94E\x85^\x85Ns\xC9\xC52\xE1t\x02\xC9\xCE\xB0\x99\x93\xA29(C\xA5\xBD\xB9\x90_\x90\xA2PV[`\x80Q`\xA0Qa(\x07a\x02\xEB_9_\x81\x81a\x06\"\x01Ra\x07 \x01R_\x81\x81a\x18\xEF\x01R\x81\x81a\x19\x18\x01Ra\x1A\\\x01Ra(\x07_\xF3\xFE`\x80`@R`\x046\x10a\x01\xDAW_5`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0\xFDW\x80c\xDC\x96\xFDP\x11a\0\x92W\x80c\xF2\xFD\xE3\x8B\x11a\0bW\x80c\xF2\xFD\xE3\x8B\x14a\x06cW\x80c\xF8>B\x92\x14a\x06\x82W\x80c\xFA\xD0\xE5\xA2\x14a\x06\x96W\x80c\xFD\x15:\xCF\x14a\x06\xB5W__\xFD[\x80c\xDC\x96\xFDP\x14a\x05\xCDW\x80c\xDD\xAD\x19\x02\x14a\x05\xE1W\x80c\xE9\xD3\xEE\x95\x14a\x06\x11W\x80c\xEA\nR7\x14a\x06DW__\xFD[\x80c\xAD<\xB1\xCC\x11a\0\xCDW\x80c\xAD<\xB1\xCC\x14a\x05(W\x80c\xB6T\x9Fu\x14a\x05XW\x80c\xC9f\xC4\xFE\x14a\x05lW\x80c\xD7\xB0\xFE\xF1\x14a\x05\x81W__\xFD[\x80c\x8D\xA5\xCB[\x14a\x04cW\x80c\xA2\xA0wV\x14a\x04\x9FW\x80c\xA8\xB4\xEE\xC9\x14a\x04\xCDW\x80c\xA9icZ\x14a\x04\xF9W__\xFD[\x80cR\xD1\x90-\x11a\x01sW\x80cqP\x18\xA6\x11a\x01CW\x80cqP\x18\xA6\x14a\x03\xC2W\x80crX\x1C\xC0\x14a\x03\xD6W\x80cw;J3\x14a\x04\tW\x80c\x82\xBF\xEF\xC8\x14a\x04,W__\xFD[\x80cR\xD1\x90-\x14a\x032W\x80cSfZ\xAA\x14a\x03FW\x80c`F4\xC9\x14a\x03rW\x80cm+\xEE\xF1\x14a\x03\xADW__\xFD[\x80c0\x8Cq.\x11a\x01\xAEW\x80c0\x8Cq.\x14a\x02\xB4W\x80cC\x9F\xAB\x91\x14a\x02\xD3W\x80cJ\xC3\xE4\xF2\x14a\x02\xF2W\x80cO\x1E\xF2\x86\x14a\x03\x1FW__\xFD[\x80b#\xDE)\x14a\x01\xDEW\x80c\r\xF1\x8F\x94\x14a\x01\xFFW\x80c\x10\xABR\x97\x14a\x02tW\x80c$MIn\x14a\x02\x95W[__\xFD[4\x80\x15a\x01\xE9W__\xFD[Pa\x01\xFDa\x01\xF86`\x04a \x9AV[a\x06\xCAV[\0[4\x80\x15a\x02\nW__\xFD[P`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x03Tc\xFF\xFF\xFF\xFF\x19\x81\x83\x1B\x16\x83R`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90\x82\x01R[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x19\x16\x81R` \x92\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x92\x81\x01\x92\x90\x92R\x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x7FW__\xFD[Pa\x02\x88a\tOV[`@Qa\x02k\x91\x90a!}V[4\x80\x15a\x02\xA0W__\xFD[Pa\x01\xFDa\x02\xAF6`\x04a!\xCAV[a\t`V[4\x80\x15a\x02\xBFW__\xFD[Pa\x01\xFDa\x02\xCE6`\x04a!\xE1V[a\ttV[4\x80\x15a\x02\xDEW__\xFD[Pa\x01\xFDa\x02\xED6`\x04a\"\xA3V[a\n>V[4\x80\x15a\x02\xFDW__\xFD[Pa\x03\x11a\x03\x0C6`\x04a\"\xDCV[a\r*V[`@Q\x90\x81R` \x01a\x02kV[a\x01\xFDa\x03-6`\x04a\"\xFFV[a\rNV[4\x80\x15a\x03=W__\xFD[Pa\x03\x11a\riV[4\x80\x15a\x03QW__\xFD[Pa\x03ea\x03`6`\x04a!\xE1V[a\r\x84V[`@Qa\x02k\x91\x90a#yV[4\x80\x15a\x03}W__\xFD[Pa\x03\x91a\x03\x8C6`\x04a!\xCAV[a\x0E\x1BV[`@\x80Q\x92\x15\x15\x83Rc\xFF\xFF\xFF\xFF\x90\x91\x16` \x83\x01R\x01a\x02kV[4\x80\x15a\x03\xB8W__\xFD[Pa\x03\x11`\x01T\x81V[4\x80\x15a\x03\xCDW__\xFD[Pa\x01\xFDa\x0E7V[4\x80\x15a\x03\xE1W__\xFD[Pa\x03\x11\x7F\xB2\x81\xFC\x8C\x12\x95M\"TM\xB4]\xE3\x15\x9A9'(\x95\xB1i\xA8R\xB3\x14\xF9\xCCv.D\xC5;\x81V[4\x80\x15a\x04\x14W__\xFD[P`@\x80Q_\x81Rc\xFF\xFF\xFF\xFF` \x82\x01R\x01a\x02kV[4\x80\x15a\x047W__\xFD[P`\x08Ta\x04K\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02kV[4\x80\x15a\x04nW__\xFD[P\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T`\x01`\x01`\xA0\x1B\x03\x16a\x04KV[4\x80\x15a\x04\xAAW__\xFD[Pa\x04\xBEa\x04\xB96`\x04a!\xCAV[a\x0EJV[`@Qa\x02k\x93\x92\x91\x90a#\x8BV[4\x80\x15a\x04\xD8W__\xFD[Pa\x04\xECa\x04\xE76`\x04a\"\xDCV[a\x0ErV[`@Qa\x02k\x91\x90a#\xADV[4\x80\x15a\x05\x04W__\xFD[Pa\x05\x18a\x05\x136`\x04a!\xCAV[a\x0E\x99V[`@Q\x90\x15\x15\x81R` \x01a\x02kV[4\x80\x15a\x053W__\xFD[Pa\x03e`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[4\x80\x15a\x05cW__\xFD[Pa\x01\xFDa\x0E\xB0V[4\x80\x15a\x05wW__\xFD[Pa\x03\x11`\x04T\x81V[4\x80\x15a\x05\x8CW__\xFD[P`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x02Tc\xFF\xFF\xFF\xFF\x19\x81\x83\x1B\x16\x83R`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90\x82\x01Ra\x02GV[4\x80\x15a\x05\xD8W__\xFD[Pa\x01\xFDa\x0FsV[4\x80\x15a\x05\xECW__\xFD[Pa\x03e`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03\"\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[4\x80\x15a\x06\x1CW__\xFD[Pa\x03\x11\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x06OW__\xFD[Pa\x01\xFDa\x06^6`\x04a#\xBBV[a\x10lV[4\x80\x15a\x06nW__\xFD[Pa\x01\xFDa\x06}6`\x04a!\xE1V[a\x11eV[4\x80\x15a\x06\x8DW__\xFD[Pa\x03\x11a\x11\x9FV[4\x80\x15a\x06\xA1W__\xFD[Pa\x01\xFDa\x06\xB06`\x04a#\xF9V[a\x11\xA9V[4\x80\x15a\x06\xC0W__\xFD[Pa\x03\x11`\x07T\x81V[`\x08T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06\xF5W`@QcPy\xFFu`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x86\x160\x14a\x07\x1EW`@Qc\x178\x92!`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x10\x15a\x07_W`@Qc\r=\xCD\xE5`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80\x80\x80\x80a\x07p\x88\x8A\x01\x8Aa$IV[_\x80T`@Qc\x02&^1`\xE6\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x89\x16`\x04\x83\x01R\x97\x9CP\x95\x9AP\x93\x98P\x91\x96P\x94P\x92\x16\x90c\x89\x97\x8C@\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xC5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xE9\x91\x90a$\xC3V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15a\x08\x14WP\x85`\x01`\x01`\xA0\x1B\x03\x16\x8D`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x80a\x08DWP`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x08DWP\x80`\x01`\x01`\xA0\x1B\x03\x16\x8D`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x08bW`@QcJ\x93\xB6!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x08o\x87\x87\x87\x87a\x12\xABV[P\x90P\x80\x15a\x08\x9FW`\x07T\x8C\x14a\x08\x9AW`@QcG2\xCE\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08\xBEV[\x8B\x15a\x08\xBEW`@Qc\x03h\x9F\xB1`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82Q\x15a\x08\xCFWa\x08\xCF\x87\x84a\x13\xECV[\x8B\x15a\t>W`\x08T`@\x80Qc\xFE\x9D\x93\x03`\xE0\x1B\x81R`\x04\x81\x01\x8F\x90R`$\x81\x01\x91\x90\x91R_`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xFE\x9D\x93\x03\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\t'W__\xFD[PZ\xF1\x15\x80\x15a\t9W=__>=_\xFD[PPPP[PPPPPPPPPPPPPPPV[``a\t[`\x05a\x15\x1AV[\x90P\x90V[a\tha\x15\xAFV[a\tq\x81a\x16\nV[PV[_T\x81\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16a\t\x9FW`@Qc\x08\xA9D\x19`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_T`@Qc\x02&^1`\xE6\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R3\x92\x16\x90c\x89\x97\x8C@\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xE6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\n\x91\x90a$\xC3V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\n1W`@Qc\xAC\xD5\xA8#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\n:\x82a\x16\x8BV[PPV[_a\nGa\x176V[\x80T\x90\x91P`\xFF`\x01`@\x1B\x82\x04\x16\x15\x90`\x01`\x01`@\x1B\x03\x16_\x81\x15\x80\x15a\nmWP\x82[\x90P_\x82`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\n\x88WP0;\x15[\x90P\x81\x15\x80\x15a\n\x96WP\x80\x15[\x15a\n\xB4W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x17\x85U\x83\x15a\n\xDEW\x84T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x85U[a\n\xEAb\x01Q\x80a\x17^V[____\x89\x80` \x01\x90Q\x81\x01\x90a\x0B\x02\x91\x90a$\xDEV[\x92\x96P\x90\x94P\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x84\x16a\x0BhW`@Qc\xEA\xC0\xD3\x89`\xE0\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7F_token must not be empty\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x0B\xBFW`@Qc\xEA\xC0\xD3\x89`\xE0\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7F_safeRegistry must not be empty\0`D\x82\x01R`d\x01a\x0B_V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0C\x16W`@Qc\xEA\xC0\xD3\x89`\xE0\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7F_owner must not be empty\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0B_V[`\x08\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x86\x16\x17\x90Ua\x0C:\x83a\x17\xEDV[`@Qc)\x96Z\x1D`\xE0\x1B\x81R0`\x04\x82\x01\x81\x90R\x7F\xB2\x81\xFC\x8C\x12\x95M\"TM\xB4]\xE3\x15\x9A9'(\x95\xB1i\xA8R\xB3\x14\xF9\xCCv.D\xC5;`$\x83\x01R`D\x82\x01Rs\x18 \xA4\xB7a\x8B\xDEq\xDC\xE8\xCD\xC7:\xABl\x95\x90_\xAD$\x90c)\x96Z\x1D\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0C\xB0W__\xFD[PZ\xF1\x15\x80\x15a\x0C\xC2W=__>=_\xFD[PPPPa\x0C\xCF\x82a\x16\nV[a\x0C\xD8\x81a\x18eV[PPPP\x83\x15a\r\"W\x84T`\xFF`@\x1B\x19\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPV[_c\xFF\xFF\xFF\xFF\x80\x83\x16\x90a\rC\x90`\x05\x90\x83\x90a\x18m\x16V[`@\x01Q\x93\x92PPPV[a\rVa\x18\xE4V[a\r_\x82a\x19\x88V[a\n:\x82\x82a\x19\x90V[_a\rra\x1AQV[P_Q` a'\xB2_9_Q\x90_R\x90V[`\t` R_\x90\x81R`@\x90 \x80Ta\r\x9C\x90a%0V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\r\xC8\x90a%0V[\x80\x15a\x0E\x13W\x80`\x1F\x10a\r\xEAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0E\x13V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\r\xF6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[_\x80\x80\x80a\x0E*`\x05\x86a\x1A\x9AV[P\x90\x96\x90\x95P\x93PPPPV[a\x0E?a\x15\xAFV[a\x0EH_a\x1B\x82V[V[__a\x0ETa\x1F\xC5V[_\x80\x80a\x0Eb`\x05\x88a\x1A\x9AV[\x91\x99\x90\x98P\x90\x96P\x94PPPPPV[a\x0Eza\x1F\xC5V[c\xFF\xFF\xFF\xFF\x80\x83\x16\x90a\x0E\x92\x90`\x05\x90\x83\x90a\x18m\x16V[\x93\x92PPPV[_\x81\x81R`\x06` R`@\x81 T\x15\x15[\x92\x91PPV[_T`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x0E\xD9W`@Qc\x08\xA9D\x19`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80T`@Qc\x02&^1`\xE6\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x89\x97\x8C@\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x1FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0FC\x91\x90a$\xC3V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0FjW`@Qc\xAC\xD5\xA8#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0EH3a\x16\x8BV[`@\x80Q\x80\x82\x01\x82R`\n\x81Ri$7\xB89&2\xB23\xB2\xB9`\xB1\x1B` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`\x05\x81Rd\x03\"\xE3\x02\xE3`\xDC\x1B\x90\x82\x01R\x81Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81R\x7Fl\xD6\x81y\x0Cx\xC2 Q{\t\x9As\x7F\x8E\x85\xF6\x9Eyz\xBEN)\x10\xFB\x18\x9Ba\xDBK\xF2\xCD\x91\x81\x01\x91\x90\x91R\x7F\xB4\xBC\xB1T\xE3\x86\x01\xC3\x899o\xA9\x181M\xA4-F&\xF1>\xF6\xD0\xCE\xB0~_]&\xB2\xFB\xC3\x91\x81\x01\x91\x90\x91RF``\x82\x01R0`\x80\x82\x01R`\xA0\x90 `\x04T\x81\x14a\tqW`\x04\x81\x90U`@Q\x81\x90\x7F\xA4?\xAD\x83\x92\x0F\xD0\x94E\x85^\x85Ns\xC9\xC52\xE1t\x02\xC9\xCE\xB0\x99\x93\xA29(C\xA5\xBD\xB9\x90_\x90\xA2PV[_T`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x10\x95W`@Qc\x08\xA9D\x19`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80T`@Qc\x02&^1`\xE6\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x89\x97\x8C@\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xDBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xFF\x91\x90a$\xC3V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x11&W`@Qc\xAC\xD5\xA8#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\n:3\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x13\xEC\x92PPPV[a\x11ma\x15\xAFV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x11\x96W`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R_`\x04\x82\x01R`$\x01a\x0B_V[a\tq\x81a\x1B\x82V[_a\t[`\x05T\x90V[_T\x83\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x11\xD4W`@Qc\x08\xA9D\x19`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_T`@Qc\x02&^1`\xE6\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R3\x92\x16\x90c\x89\x97\x8C@\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\x1BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12?\x91\x90a$\xC3V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x12fW`@Qc\xAC\xD5\xA8#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x12\xA5\x84\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x13\xEC\x92PPPV[PPPPV[_\x80\x80\x80a\x12\xBA`\x05\x86a\x1A\x9AV[P\x91P\x91P\x81\x15a\x12\xD1W_\x93P\x91Pa\x13\xE3\x90PV[a\x13\x0E`@Q\x80`\x80\x01`@R\x80\x89\x81R` \x01\x88\x81R` \x01\x87\x81R` \x01\x8A`\x01`\x01`\xA0\x1B\x03\x16\x81RP`\x05a\x1B\xF2\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`@\x80Q\x7F\x11\xD9FS\xDER\xFC\x148\x87\x95o\xE4\xB5\xE4\x12\n\x85\x0B\x13\0\xEC\xE0S=\xEEk\xD8\xFF\x91\xF72` \x82\x01R\x90\x81\x01\x89\x90R``\x80\x82\x01\x89\x90R`\x80\x82\x01\x88\x90R\x8A\x90\x1B`\x01`\x01``\x1B\x03\x19\x16`\xA0\x82\x01R`\xB4\x81\x01\x82\x90R\x90\x93Pa\x13\x85\x90`\xD4\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x1C\xF4V[`@\x80Q\x88\x81R` \x81\x01\x88\x90R\x90\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x89\x16``\x82\x01R`\x80\x81\x01\x84\x90R\x7F\x11\xD9FS\xDER\xFC\x148\x87\x95o\xE4\xB5\xE4\x12\n\x85\x0B\x13\0\xEC\xE0S=\xEEk\xD8\xFF\x91\xF72\x90`\xA0\x01`@Q\x80\x91\x03\x90\xA1`\x01\x93PPP[\x94P\x94\x92PPPV[\x80Q_\x03a\x14\x0CW`@Qb\x9C\xEA\xC7`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\t` R`@\x90 \x80Ta\x14.\x90a%0V[\x15\x80\x15\x91Pa\x14zWP\x80\x80Q\x90` \x01 `\t_\x84`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ `@Qa\x14p\x91\x90a%hV[`@Q\x80\x91\x03\x90 \x14[\x15a\x14\x83WPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\t` R`@\x90 a\x14\xA4\x82\x82a&$V[Pa\x14\xDD\x7F\xC4\xDF[\xA1h\x14\x83\x8A\xB2a\x88)\xD6\x8F\x86#\xBB\x89s\x02\xF2M\xBD\xBA\"y\xDB\xE4Z\xDB=\x14\x83\x83`@Q` \x01a\x13q\x93\x92\x91\x90a&\xF5V[\x7F\xC4\xDF[\xA1h\x14\x83\x8A\xB2a\x88)\xD6\x8F\x86#\xBB\x89s\x02\xF2M\xBD\xBA\"y\xDB\xE4Z\xDB=\x14\x82\x82`@Qa\x15\x0E\x92\x91\x90a'\x1EV[`@Q\x80\x91\x03\x90\xA1PPV[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x15\xA4W_\x84\x81R` \x90\x81\x90 `@\x80Q`\xA0\x81\x01\x82R`\x05\x86\x02\x90\x92\x01\x80T\x83R`\x01\x80\x82\x01T\x84\x86\x01R`\x02\x82\x01T\x92\x84\x01\x92\x90\x92R`\x03\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16``\x84\x01R`\x04\x01T`\x80\x83\x01R\x90\x83R\x90\x92\x01\x91\x01a\x15>V[PPPP\x90P\x91\x90PV[3a\x15\xE1\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0EHW`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x0B_V[`\x07\x80T\x90\x82\x90U`@\x80Q\x7Fi%\x94\\\xB4\xC6\xB5\x97\xA3a\x80\x9D[Q\x10~N\x03r\x04\x04\xA8$\x83\x85>\x91\xFA\xA1v\xB8\x8E` \x82\x01R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90Ra\x16U\x90`\x80\x01a\x13qV[`@\x80Q\x83\x81R` \x81\x01\x83\x90R\x7Fi%\x94\\\xB4\xC6\xB5\x97\xA3a\x80\x9D[Q\x10~N\x03r\x04\x04\xA8$\x83\x85>\x91\xFA\xA1v\xB8\x8E\x91\x01a\x15\x0EV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\t` R`@\x81 a\x16\xAB\x91a\x1F\xF8V[a\x16\xF7\x7F\xA4\xDE0\xA5(\xBE\xCA\xDF\x82d\x9D\x13\x95\xC0\xE3\r\xD1\x8A\xE3[Z\x96\xCEq\xE9)[\xB1K\xC9\xF3\xBC\x82`@Q` \x01a\x13q\x92\x91\x90\x91\x82R``\x1B`\x01`\x01``\x1B\x03\x19\x16` \x82\x01R`4\x01\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x81R\x7F\xA4\xDE0\xA5(\xBE\xCA\xDF\x82d\x9D\x13\x95\xC0\xE3\r\xD1\x8A\xE3[Z\x96\xCEq\xE9)[\xB1K\xC9\xF3\xBC\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[_\x80\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0a\x0E\xAAV[\x80_\x03a\x17~W`@Qc4o\xF6\x07`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x81\x90U`@Q`\x01`\x01``\x1B\x03\x190``\x1B\x16` \x82\x01R`4\x01`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x91\x82\x01 \x90\x1C`\x01`\xE0\x1BBc\xFF\xFF\xFF\xFF\x90\x81\x16\x82\x02\x92\x90\x92\x17`\x02\x81\x90U`\x01`\x01`\xE0\x1B\x03\x81\x16\x90\x82\x90\x04\x90\x92\x16\x02\x17`\x03Ua\tqa\x0FsV[_T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x18\x16W`@Qb\xDC\x14\x9F`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x18=W`@QcGN\xBE/`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x01`\x01`\xA8\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17`\x01`\xA0\x1B\x17\x90UV[a\x11ma\x1D\xBAV[a\x18ua\x1F\xC5V[\x82_\x01\x82\x81T\x81\x10a\x18\x89Wa\x18\x89a'AV[_\x91\x82R` \x91\x82\x90 `@\x80Q`\xA0\x81\x01\x82R`\x05\x90\x93\x02\x90\x91\x01\x80T\x83R`\x01\x81\x01T\x93\x83\x01\x93\x90\x93R`\x02\x83\x01T\x90\x82\x01R`\x03\x82\x01T`\x01`\x01`\xA0\x1B\x03\x16``\x82\x01R`\x04\x90\x91\x01T`\x80\x82\x01R\x90P\x92\x91PPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x19jWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x19^_Q` a'\xB2_9_Q\x90_RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x0EHW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\tqa\x15\xAFV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x19\xEAWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x19\xE7\x91\x81\x01\x90a'UV[`\x01[a\x1A\x12W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x0B_V[_Q` a'\xB2_9_Q\x90_R\x81\x14a\x1ABW`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x0B_V[a\x1AL\x83\x83a\x1D\xDFV[PPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0EHW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__a\x1A\xA4a\x1F\xC5V[_\x84\x81R`\x01\x86\x01` R`@\x81 T\x90\x81\x90\x03a\x1A\xF2WPP`@\x80Q`\xA0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R\x90\x92P\x82\x91Pa\x1B{V[`\x01a\x1A\xFE\x81\x83a'\x80V[\x87a\x1B\n`\x01\x85a'\x80V[\x81T\x81\x10a\x1B\x1AWa\x1B\x1Aa'AV[_\x91\x82R` \x91\x82\x90 `@\x80Q`\xA0\x81\x01\x82R`\x05\x90\x93\x02\x90\x91\x01\x80T\x83R`\x01\x81\x01T\x93\x83\x01\x93\x90\x93R`\x02\x83\x01T\x90\x82\x01R`\x03\x82\x01T`\x01`\x01`\xA0\x1B\x03\x16``\x82\x01R`\x04\x90\x91\x01T`\x80\x82\x01R\x91\x95P\x93P\x91Pa\x1B{\x90PV[\x92P\x92P\x92V[\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x82\x17\x84U`@Q\x92\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPPV[\x81T_\x90c\xFF\xFF\xFF\xFF\x11a\x1C\x19W`@Qc\x10\x95;\xFF`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80\x83\x01Q_\x90\x81R`\x01\x85\x01` R T\x15a\x1CIW`@QbXLo`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\xA0\x81\x01\x82R\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01\x90\x81R\x85\x84\x01\x80Q\x84\x86\x01\x90\x81R``\x80\x89\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x87\x01\x91\x82RB`\x80\x88\x01\x90\x81R\x8BT`\x01\x80\x82\x01\x8EU_\x8E\x81R\x89\x81 \x9AQ`\x05\x90\x93\x02\x90\x9A\x01\x91\x82U\x96Q\x81\x88\x01U\x93Q`\x02\x85\x01U\x91Q`\x03\x84\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x92\x16\x17\x90UQ`\x04\x90\x91\x01U\x87T\x90Q\x84R\x81\x88\x01\x90\x92R\x92\x90\x91 \x81\x90Ua\x0E\x92\x91\x90a'\x80V[`\x01T`\x02T_\x91a\x1D\x12\x91`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a'\x93V[B\x11\x15a\x1D\x1DWP`\x01[`\x04T`\x02T\x83Q` \x80\x86\x01\x91\x90\x91 `@\x80Q\x80\x84\x01\x95\x90\x95RC`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x90\x85\x01R\x91\x90\x1Bc\xFF\xFF\xFF\xFF\x19\x16`D\x83\x01R``\x82\x01R`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x91\x82\x01 c\xFF\xFF\xFF\xFFB\x16`\x01`\xE0\x1B\x02\x91\x1C\x17`\x02U\x80\x15a\n:WPP`\x02T`\x01`\x01`\xE0\x1B\x03\x81\x16`\x01`\xE0\x1B\x91\x82\x90\x04c\xFF\xFF\xFF\xFF\x16\x90\x91\x02\x17`\x03UV[a\x1D\xC2a\x1E4V[a\x0EHW`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1D\xE8\x82a\x1EMV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15a\x1E,Wa\x1AL\x82\x82a\x1E\xB0V[a\n:a\x1F\"V[_a\x1E=a\x176V[T`\x01`@\x1B\x90\x04`\xFF\x16\x91\x90PV[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03a\x1E\x82W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x0B_V[_Q` a'\xB2_9_Q\x90_R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``__\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x1E\xCC\x91\x90a'\xA6V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a\x1F\x04W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x1F\tV[``\x91P[P\x91P\x91Pa\x1F\x19\x85\x83\x83a\x1FAV[\x95\x94PPPPPV[4\x15a\x0EHW`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x82a\x1FVWa\x1FQ\x82a\x1F\x9DV[a\x0E\x92V[\x81Q\x15\x80\x15a\x1FmWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x1F\x96W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x0B_V[P\x92\x91PPV[\x80Q\x15a\x1F\xACW\x80Q` \x82\x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x80`\xA0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81RP\x90V[P\x80Ta \x04\x90a%0V[_\x82U\x80`\x1F\x10a \x13WPPV[`\x1F\x01` \x90\x04\x90_R` _ \x90\x81\x01\x90a\tq\x91\x90[\x80\x82\x11\x15a >W_\x81U`\x01\x01a +V[P\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\tqW__\xFD[__\x83`\x1F\x84\x01\x12a fW__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a |W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a \x93W__\xFD[\x92P\x92\x90PV[________`\xC0\x89\x8B\x03\x12\x15a \xB1W__\xFD[\x885a \xBC\x81a BV[\x97P` \x89\x015a \xCC\x81a BV[\x96P`@\x89\x015a \xDC\x81a BV[\x95P``\x89\x015\x94P`\x80\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a \xFDW__\xFD[a!\t\x8B\x82\x8C\x01a VV[\x90\x95P\x93PP`\xA0\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a!'W__\xFD[a!3\x8B\x82\x8C\x01a VV[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x80\x82\x01Q\x90\x83\x01R``\x80\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90\x83\x01R`\x80\x90\x81\x01Q\x91\x01RV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a!\xBFWa!\xA9\x83\x85Qa!GV[` \x93\x90\x93\x01\x92`\xA0\x92\x90\x92\x01\x91`\x01\x01a!\x96V[P\x90\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a!\xDAW__\xFD[P5\x91\x90PV[_` \x82\x84\x03\x12\x15a!\xF1W__\xFD[\x815a\x0E\x92\x81a BV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[__`\x01`\x01`@\x1B\x03\x84\x11\x15a\")Wa\")a!\xFCV[P`@Q`\x1F\x19`\x1F\x85\x01\x81\x16`?\x01\x16\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\"WWa\"Wa!\xFCV[`@R\x83\x81R\x90P\x80\x82\x84\x01\x85\x10\x15a\"nW__\xFD[\x83\x83` \x83\x017_` \x85\x83\x01\x01RP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\"\x94W__\xFD[a\x0E\x92\x83\x835` \x85\x01a\"\x10V[_` \x82\x84\x03\x12\x15a\"\xB3W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xC8W__\xFD[a\"\xD4\x84\x82\x85\x01a\"\x85V[\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a\"\xECW__\xFD[\x815c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0E\x92W__\xFD[__`@\x83\x85\x03\x12\x15a#\x10W__\xFD[\x825a#\x1B\x81a BV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a#5W__\xFD[a#A\x85\x82\x86\x01a\"\x85V[\x91PP\x92P\x92\x90PV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x0E\x92` \x83\x01\x84a#KV[\x83\x15\x15\x81Rc\xFF\xFF\xFF\xFF\x83\x16` \x82\x01R`\xE0\x81\x01a\"\xD4`@\x83\x01\x84a!GV[`\xA0\x81\x01a\x0E\xAA\x82\x84a!GV[__` \x83\x85\x03\x12\x15a#\xCCW__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a#\xE1W__\xFD[a#\xED\x85\x82\x86\x01a VV[\x90\x96\x90\x95P\x93PPPPV[___`@\x84\x86\x03\x12\x15a$\x0BW__\xFD[\x835a$\x16\x81a BV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a$0W__\xFD[a$<\x86\x82\x87\x01a VV[\x94\x97\x90\x96P\x93\x94PPPPV[_____`\xA0\x86\x88\x03\x12\x15a$]W__\xFD[\x855a$h\x81a BV[\x94P` \x86\x015\x93P`@\x86\x015\x92P``\x86\x015\x91P`\x80\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a$\x97W__\xFD[\x86\x01`\x1F\x81\x01\x88\x13a$\xA7W__\xFD[a$\xB6\x88\x825` \x84\x01a\"\x10V[\x91PP\x92\x95P\x92\x95\x90\x93PV[_` \x82\x84\x03\x12\x15a$\xD3W__\xFD[\x81Qa\x0E\x92\x81a BV[____`\x80\x85\x87\x03\x12\x15a$\xF1W__\xFD[\x84Qa$\xFC\x81a BV[` \x86\x01Q\x90\x94Pa%\r\x81a BV[`@\x86\x01Q``\x87\x01Q\x91\x94P\x92Pa%%\x81a BV[\x93\x96\x92\x95P\x90\x93PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a%DW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a%bWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[__\x83Ta%u\x81a%0V[`\x01\x82\x16\x80\x15a%\x8CW`\x01\x81\x14a%\xA1Wa%\xCEV[`\xFF\x19\x83\x16\x86R\x81\x15\x15\x82\x02\x86\x01\x93Pa%\xCEV[\x86_R` _ _[\x83\x81\x10\x15a%\xC6W\x81T\x88\x82\x01R`\x01\x90\x91\x01\x90` \x01a%\xAAV[PP\x81\x86\x01\x93P[P\x91\x95\x94PPPPPV[`\x1F\x82\x11\x15a\x1ALW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a%\xFEWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a&\x1DW_\x81U`\x01\x01a&\nV[PPPPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a&=Wa&=a!\xFCV[a&Q\x81a&K\x84Ta%0V[\x84a%\xD9V[` `\x1F\x82\x11`\x01\x81\x14a&\x83W_\x83\x15a&lWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua&\x1DV[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a&\xB2W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a&\x92V[P\x84\x82\x10\x15a&\xCFW\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[\x83\x81Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x83``\x1B\x16` \x82\x01R_a\x1F\x19`4\x83\x01\x84a&\xDEV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90a\"\xD4\x90\x83\x01\x84a#KV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a'eW__\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0E\xAAWa\x0E\xAAa'lV[\x80\x82\x01\x80\x82\x11\x15a\x0E\xAAWa\x0E\xAAa'lV[_a\x0E\x92\x82\x84a&\xDEV\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\xA2dipfsX\"\x12 VSk\x14\xD5S\xD1\xF7Nn\xF4>\xB2\"2Z\xE8Fv\x1Ff\x80L\xDC\x0BH\xA8\xEB2\x1C\x01\xD7dsolcC\0\x08\x1E\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080604052600436106101da575f3560e01c80638da5cb5b116100fd578063dc96fd5011610092578063f2fde38b11610062578063f2fde38b14610663578063f83e429214610682578063fad0e5a214610696578063fd153acf146106b5575f5ffd5b8063dc96fd50146105cd578063ddad1902146105e1578063e9d3ee9514610611578063ea0a523714610644575f5ffd5b8063ad3cb1cc116100cd578063ad3cb1cc14610528578063b6549f7514610558578063c966c4fe1461056c578063d7b0fef114610581575f5ffd5b80638da5cb5b14610463578063a2a077561461049f578063a8b4eec9146104cd578063a969635a146104f9575f5ffd5b806352d1902d11610173578063715018a611610143578063715018a6146103c257806372581cc0146103d6578063773b4a331461040957806382bfefc81461042c575f5ffd5b806352d1902d1461033257806353665aaa14610346578063604634c9146103725780636d2beef1146103ad575f5ffd5b8063308c712e116101ae578063308c712e146102b4578063439fab91146102d35780634ac3e4f2146102f25780634f1ef2861461031f575f5ffd5b806223de29146101de5780630df18f94146101ff57806310ab529714610274578063244d496e14610295575b5f5ffd5b3480156101e9575f5ffd5b506101fd6101f836600461209a565b6106ca565b005b34801561020a575f5ffd5b506040805180820182525f808252602091820152815180830190925260035463ffffffff1981831b168352600160e01b900463ffffffff16908201525b60408051825163ffffffff1916815260209283015163ffffffff1692810192909252015b60405180910390f35b34801561027f575f5ffd5b5061028861094f565b60405161026b919061217d565b3480156102a0575f5ffd5b506101fd6102af3660046121ca565b610960565b3480156102bf575f5ffd5b506101fd6102ce3660046121e1565b610974565b3480156102de575f5ffd5b506101fd6102ed3660046122a3565b610a3e565b3480156102fd575f5ffd5b5061031161030c3660046122dc565b610d2a565b60405190815260200161026b565b6101fd61032d3660046122ff565b610d4e565b34801561033d575f5ffd5b50610311610d69565b348015610351575f5ffd5b506103656103603660046121e1565b610d84565b60405161026b9190612379565b34801561037d575f5ffd5b5061039161038c3660046121ca565b610e1b565b60408051921515835263ffffffff90911660208301520161026b565b3480156103b8575f5ffd5b5061031160015481565b3480156103cd575f5ffd5b506101fd610e37565b3480156103e1575f5ffd5b506103117fb281fc8c12954d22544db45de3159a39272895b169a852b314f9cc762e44c53b81565b348015610414575f5ffd5b50604080515f815263ffffffff60208201520161026b565b348015610437575f5ffd5b5060085461044b906001600160a01b031681565b6040516001600160a01b03909116815260200161026b565b34801561046e575f5ffd5b507f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300546001600160a01b031661044b565b3480156104aa575f5ffd5b506104be6104b93660046121ca565b610e4a565b60405161026b9392919061238b565b3480156104d8575f5ffd5b506104ec6104e73660046122dc565b610e72565b60405161026b91906123ad565b348015610504575f5ffd5b506105186105133660046121ca565b610e99565b604051901515815260200161026b565b348015610533575f5ffd5b50610365604051806040016040528060058152602001640352e302e360dc1b81525081565b348015610563575f5ffd5b506101fd610eb0565b348015610577575f5ffd5b5061031160045481565b34801561058c575f5ffd5b506040805180820182525f808252602091820152815180830190925260025463ffffffff1981831b168352600160e01b900463ffffffff1690820152610247565b3480156105d8575f5ffd5b506101fd610f73565b3480156105ec575f5ffd5b50610365604051806040016040528060058152602001640322e302e360dc1b81525081565b34801561061c575f5ffd5b506103117f000000000000000000000000000000000000000000000000000000000000000081565b34801561064f575f5ffd5b506101fd61065e3660046123bb565b61106c565b34801561066e575f5ffd5b506101fd61067d3660046121e1565b611165565b34801561068d575f5ffd5b5061031161119f565b3480156106a1575f5ffd5b506101fd6106b03660046123f9565b6111a9565b3480156106c0575f5ffd5b5061031160075481565b6008546001600160a01b031633146106f557604051635079ff7560e11b815260040160405180910390fd5b6001600160a01b038616301461071e57604051631738922160e31b815260040160405180910390fd5b7f000000000000000000000000000000000000000000000000000000000000000083101561075f57604051630d3dcde560e31b815260040160405180910390fd5b5f80808080610770888a018a612449565b5f80546040516302265e3160e61b81526001600160a01b038089166004830152979c50959a5093985091965094509216906389978c4090602401602060405180830381865afa1580156107c5573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107e991906124c3565b90506001600160a01b0381161580156108145750856001600160a01b03168d6001600160a01b031614155b8061084457506001600160a01b038116158015906108445750806001600160a01b03168d6001600160a01b031614155b1561086257604051634a93b62160e11b815260040160405180910390fd5b5f61086f878787876112ab565b509050801561089f576007548c1461089a57604051634732cefd60e01b815260040160405180910390fd5b6108be565b8b156108be576040516303689fb160e21b815260040160405180910390fd5b8251156108cf576108cf87846113ec565b8b1561093e576008546040805163fe9d930360e01b8152600481018f905260248101919091525f60448201526001600160a01b039091169063fe9d9303906064015f604051808303815f87803b158015610927575f5ffd5b505af1158015610939573d5f5f3e3d5ffd5b505050505b505050505050505050505050505050565b606061095b600561151a565b905090565b6109686115af565b6109718161160a565b50565b5f548190600160a01b900460ff1661099f576040516308a9441960e31b815260040160405180910390fd5b5f546040516302265e3160e61b81526001600160a01b038381166004830152339216906389978c4090602401602060405180830381865afa1580156109e6573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a0a91906124c3565b6001600160a01b031614610a315760405163acd5a82360e01b815260040160405180910390fd5b610a3a8261168b565b5050565b5f610a47611736565b805490915060ff600160401b82041615906001600160401b03165f81158015610a6d5750825b90505f826001600160401b03166001148015610a885750303b155b905081158015610a96575080155b15610ab45760405163f92ee8a960e01b815260040160405180910390fd5b845467ffffffffffffffff191660011785558315610ade57845460ff60401b1916600160401b1785555b610aea6201518061175e565b5f5f5f5f89806020019051810190610b0291906124de565b929650909450925090506001600160a01b038416610b685760405163eac0d38960e01b815260206004820152601860248201527f5f746f6b656e206d757374206e6f7420626520656d707479000000000000000060448201526064015b60405180910390fd5b6001600160a01b038316610bbf5760405163eac0d38960e01b815260206004820152601f60248201527f5f736166655265676973747279206d757374206e6f7420626520656d707479006044820152606401610b5f565b6001600160a01b038116610c165760405163eac0d38960e01b815260206004820152601860248201527f5f6f776e6572206d757374206e6f7420626520656d70747900000000000000006044820152606401610b5f565b600880546001600160a01b0319166001600160a01b038616179055610c3a836117ed565b6040516329965a1d60e01b815230600482018190527fb281fc8c12954d22544db45de3159a39272895b169a852b314f9cc762e44c53b60248301526044820152731820a4b7618bde71dce8cdc73aab6c95905fad24906329965a1d906064015f604051808303815f87803b158015610cb0575f5ffd5b505af1158015610cc2573d5f5f3e3d5ffd5b50505050610ccf8261160a565b610cd881611865565b505050508315610d2257845460ff60401b19168555604051600181527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a15b505050505050565b5f63ffffffff80831690610d4390600590839061186d16565b604001519392505050565b610d566118e4565b610d5f82611988565b610a3a8282611990565b5f610d72611a51565b505f5160206127b25f395f51905f5290565b60096020525f908152604090208054610d9c90612530565b80601f0160208091040260200160405190810160405280929190818152602001828054610dc890612530565b8015610e135780601f10610dea57610100808354040283529160200191610e13565b820191905f5260205f20905b815481529060010190602001808311610df657829003601f168201915b505050505081565b5f808080610e2a600586611a9a565b5090969095509350505050565b610e3f6115af565b610e485f611b82565b565b5f5f610e54611fc5565b5f8080610e62600588611a9a565b9199909850909650945050505050565b610e7a611fc5565b63ffffffff80831690610e9290600590839061186d16565b9392505050565b5f8181526006602052604081205415155b92915050565b5f54600160a01b900460ff16610ed9576040516308a9441960e31b815260040160405180910390fd5b5f80546040516302265e3160e61b81523360048201526001600160a01b03909116906389978c4090602401602060405180830381865afa158015610f1f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f4391906124c3565b6001600160a01b031614610f6a5760405163acd5a82360e01b815260040160405180910390fd5b610e483361168b565b604080518082018252600a8152692437b8392632b233b2b960b11b6020918201528151808301835260058152640322e302e360dc1b9082015281517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f81527f6cd681790c78c220517b099a737f8e85f69e797abe4e2910fb189b61db4bf2cd918101919091527fb4bcb154e38601c389396fa918314da42d4626f13ef6d0ceb07e5f5d26b2fbc39181019190915246606082015230608082015260a09020600454811461097157600481905560405181907fa43fad83920fd09445855e854e73c9c532e17402c9ceb09993a2392843a5bdb9905f90a250565b5f54600160a01b900460ff16611095576040516308a9441960e31b815260040160405180910390fd5b5f80546040516302265e3160e61b81523360048201526001600160a01b03909116906389978c4090602401602060405180830381865afa1580156110db573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110ff91906124c3565b6001600160a01b0316146111265760405163acd5a82360e01b815260040160405180910390fd5b610a3a3383838080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152506113ec92505050565b61116d6115af565b6001600160a01b03811661119657604051631e4fbdf760e01b81525f6004820152602401610b5f565b61097181611b82565b5f61095b60055490565b5f548390600160a01b900460ff166111d4576040516308a9441960e31b815260040160405180910390fd5b5f546040516302265e3160e61b81526001600160a01b038381166004830152339216906389978c4090602401602060405180830381865afa15801561121b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061123f91906124c3565b6001600160a01b0316146112665760405163acd5a82360e01b815260040160405180910390fd5b6112a58484848080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152506113ec92505050565b50505050565b5f8080806112ba600586611a9a565b509150915081156112d1575f935091506113e39050565b61130e60405180608001604052808981526020018881526020018781526020018a6001600160a01b03168152506005611bf290919063ffffffff16565b604080517f11d94653de52fc143887956fe4b5e4120a850b1300ece0533dee6bd8ff91f73260208201529081018990526060808201899052608082018890528a901b6001600160601b03191660a082015260b481018290529093506113859060d4015b604051602081830303815290604052611cf4565b60408051888152602081018890529081018690526001600160a01b0389166060820152608081018490527f11d94653de52fc143887956fe4b5e4120a850b1300ece0533dee6bd8ff91f7329060a00160405180910390a16001935050505b94509492505050565b80515f0361140c57604051629ceac760e41b815260040160405180910390fd5b6001600160a01b0382165f908152600960205260409020805461142e90612530565b158015915061147a5750808051906020012060095f846001600160a01b03166001600160a01b031681526020019081526020015f206040516114709190612568565b6040518091039020145b15611483575050565b6001600160a01b0382165f9081526009602052604090206114a48282612624565b506114dd7fc4df5ba16814838ab2618829d68f8623bb897302f24dbdba2279dbe45adb3d148383604051602001611371939291906126f5565b7fc4df5ba16814838ab2618829d68f8623bb897302f24dbdba2279dbe45adb3d14828260405161150e92919061271e565b60405180910390a15050565b6060815f01805480602002602001604051908101604052809291908181526020015f905b828210156115a4575f8481526020908190206040805160a08101825260058602909201805483526001808201548486015260028201549284019290925260038101546001600160a01b03166060840152600401546080830152908352909201910161153e565b505050509050919050565b336115e17f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300546001600160a01b031690565b6001600160a01b031614610e485760405163118cdaa760e01b8152336004820152602401610b5f565b6007805490829055604080517f6925945cb4c6b597a361809d5b51107e4e03720404a82483853e91faa176b88e60208201529081018390526060810182905261165590608001611371565b60408051838152602081018390527f6925945cb4c6b597a361809d5b51107e4e03720404a82483853e91faa176b88e910161150e565b6001600160a01b0381165f9081526009602052604081206116ab91611ff8565b6116f77fa4de30a528becadf82649d1395c0e30dd18ae35b5a96ce71e9295bb14bc9f3bc8260405160200161137192919091825260601b6001600160601b031916602082015260340190565b6040516001600160a01b03821681527fa4de30a528becadf82649d1395c0e30dd18ae35b5a96ce71e9295bb14bc9f3bc9060200160405180910390a150565b5f807ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00610eaa565b805f0361177e5760405163346ff60760e01b815260040160405180910390fd5b60018190556040516001600160601b03193060601b16602082015260340160408051808303601f190181529190528051602091820120901c600160e01b4263ffffffff90811682029290921760028190556001600160e01b038116908290049092160217600355610971610f73565b5f54600160a01b900460ff16156118165760405162dc149f60e41b815260040160405180910390fd5b6001600160a01b03811661183d5760405163474ebe2f60e11b815260040160405180910390fd5b5f80546001600160a01b039092166001600160a81b031990921691909117600160a01b179055565b61116d611dba565b611875611fc5565b825f01828154811061188957611889612741565b5f9182526020918290206040805160a08101825260059093029091018054835260018101549383019390935260028301549082015260038201546001600160a01b031660608201526004909101546080820152905092915050565b306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016148061196a57507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031661195e5f5160206127b25f395f51905f52546001600160a01b031690565b6001600160a01b031614155b15610e485760405163703e46dd60e11b815260040160405180910390fd5b6109716115af565b816001600160a01b03166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa9250505080156119ea575060408051601f3d908101601f191682019092526119e791810190612755565b60015b611a1257604051634c9c8ce360e01b81526001600160a01b0383166004820152602401610b5f565b5f5160206127b25f395f51905f528114611a4257604051632a87526960e21b815260048101829052602401610b5f565b611a4c8383611ddf565b505050565b306001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614610e485760405163703e46dd60e11b815260040160405180910390fd5b5f5f611aa4611fc5565b5f84815260018601602052604081205490819003611af25750506040805160a0810182525f808252602082018190529181018290526060810182905260808101829052909250829150611b7b565b6001611afe8183612780565b87611b0a600185612780565b81548110611b1a57611b1a612741565b5f9182526020918290206040805160a08101825260059093029091018054835260018101549383019390935260028301549082015260038201546001600160a01b03166060820152600490910154608082015291955093509150611b7b9050565b9250925092565b7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930080546001600160a01b031981166001600160a01b03848116918217845560405192169182907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a3505050565b81545f9063ffffffff11611c19576040516310953bff60e11b815260040160405180910390fd5b6040808301515f90815260018501602052205415611c495760405162584c6f60e51b815260040160405180910390fd5b6040805160a0810182528351815260208085015181830190815285840180518486019081526060808901516001600160a01b0390811691870191825242608088019081528b5460018082018e555f8e81528981209a516005909302909a019182559651818801559351600285015591516003840180546001600160a01b031916919092161790555160049091015587549051845281880190925292909120819055610e929190612780565b6001546002545f91611d1291600160e01b900463ffffffff16612793565b421115611d1d575060015b600454600254835160208086019190912060408051808401959095524360e01b6001600160e01b0319169085015291901b63ffffffff19166044830152606082015260800160408051601f19818403018152919052805160209182012063ffffffff4216600160e01b02911c176002558015610a3a5750506002546001600160e01b038116600160e01b9182900463ffffffff1690910217600355565b611dc2611e34565b610e4857604051631afcd79f60e31b815260040160405180910390fd5b611de882611e4d565b6040516001600160a01b038316907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b905f90a2805115611e2c57611a4c8282611eb0565b610a3a611f22565b5f611e3d611736565b54600160401b900460ff16919050565b806001600160a01b03163b5f03611e8257604051634c9c8ce360e01b81526001600160a01b0382166004820152602401610b5f565b5f5160206127b25f395f51905f5280546001600160a01b0319166001600160a01b0392909216919091179055565b60605f5f846001600160a01b031684604051611ecc91906127a6565b5f60405180830381855af49150503d805f8114611f04576040519150601f19603f3d011682016040523d82523d5f602084013e611f09565b606091505b5091509150611f19858383611f41565b95945050505050565b3415610e485760405163b398979f60e01b815260040160405180910390fd5b606082611f5657611f5182611f9d565b610e92565b8151158015611f6d57506001600160a01b0384163b155b15611f9657604051639996b31560e01b81526001600160a01b0385166004820152602401610b5f565b5092915050565b805115611fac57805160208201fd5b60405163d6bda27560e01b815260040160405180910390fd5b6040518060a001604052805f81526020015f81526020015f81526020015f6001600160a01b031681526020015f81525090565b50805461200490612530565b5f825580601f10612013575050565b601f0160209004905f5260205f209081019061097191905b8082111561203e575f815560010161202b565b5090565b6001600160a01b0381168114610971575f5ffd5b5f5f83601f840112612066575f5ffd5b5081356001600160401b0381111561207c575f5ffd5b602083019150836020828501011115612093575f5ffd5b9250929050565b5f5f5f5f5f5f5f5f60c0898b0312156120b1575f5ffd5b88356120bc81612042565b975060208901356120cc81612042565b965060408901356120dc81612042565b95506060890135945060808901356001600160401b038111156120fd575f5ffd5b6121098b828c01612056565b90955093505060a08901356001600160401b03811115612127575f5ffd5b6121338b828c01612056565b999c989b5096995094979396929594505050565b8051825260208082015190830152604080820151908301526060808201516001600160a01b031690830152608090810151910152565b602080825282518282018190525f918401906040840190835b818110156121bf576121a9838551612147565b6020939093019260a09290920191600101612196565b509095945050505050565b5f602082840312156121da575f5ffd5b5035919050565b5f602082840312156121f1575f5ffd5b8135610e9281612042565b634e487b7160e01b5f52604160045260245ffd5b5f5f6001600160401b03841115612229576122296121fc565b50604051601f19601f85018116603f011681018181106001600160401b0382111715612257576122576121fc565b60405283815290508082840185101561226e575f5ffd5b838360208301375f60208583010152509392505050565b5f82601f830112612294575f5ffd5b610e9283833560208501612210565b5f602082840312156122b3575f5ffd5b81356001600160401b038111156122c8575f5ffd5b6122d484828501612285565b949350505050565b5f602082840312156122ec575f5ffd5b813563ffffffff81168114610e92575f5ffd5b5f5f60408385031215612310575f5ffd5b823561231b81612042565b915060208301356001600160401b03811115612335575f5ffd5b61234185828601612285565b9150509250929050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b602081525f610e92602083018461234b565b831515815263ffffffff8316602082015260e081016122d46040830184612147565b60a08101610eaa8284612147565b5f5f602083850312156123cc575f5ffd5b82356001600160401b038111156123e1575f5ffd5b6123ed85828601612056565b90969095509350505050565b5f5f5f6040848603121561240b575f5ffd5b833561241681612042565b925060208401356001600160401b03811115612430575f5ffd5b61243c86828701612056565b9497909650939450505050565b5f5f5f5f5f60a0868803121561245d575f5ffd5b853561246881612042565b945060208601359350604086013592506060860135915060808601356001600160401b03811115612497575f5ffd5b8601601f810188136124a7575f5ffd5b6124b688823560208401612210565b9150509295509295909350565b5f602082840312156124d3575f5ffd5b8151610e9281612042565b5f5f5f5f608085870312156124f1575f5ffd5b84516124fc81612042565b602086015190945061250d81612042565b60408601516060870151919450925061252581612042565b939692955090935050565b600181811c9082168061254457607f821691505b60208210810361256257634e487b7160e01b5f52602260045260245ffd5b50919050565b5f5f835461257581612530565b60018216801561258c57600181146125a1576125ce565b60ff19831686528115158202860193506125ce565b865f5260205f205f5b838110156125c6578154888201526001909101906020016125aa565b505081860193505b509195945050505050565b601f821115611a4c57805f5260205f20601f840160051c810160208510156125fe5750805b601f840160051c820191505b8181101561261d575f815560010161260a565b5050505050565b81516001600160401b0381111561263d5761263d6121fc565b6126518161264b8454612530565b846125d9565b6020601f821160018114612683575f831561266c5750848201515b5f19600385901b1c1916600184901b17845561261d565b5f84815260208120601f198516915b828110156126b25787850151825560209485019460019092019101612692565b50848210156126cf57868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b5f81518060208401855e5f93019283525090919050565b8381526bffffffffffffffffffffffff198360601b1660208201525f611f1960348301846126de565b6001600160a01b03831681526040602082018190525f906122d49083018461234b565b634e487b7160e01b5f52603260045260245ffd5b5f60208284031215612765575f5ffd5b5051919050565b634e487b7160e01b5f52601160045260245ffd5b81810381811115610eaa57610eaa61276c565b80820180821115610eaa57610eaa61276c565b5f610e9282846126de56fe360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbca264697066735822122056536b14d553d1f74e6ef43eb222325ae846761f66804cdc0b48a8eb321c01d764736f6c634300081e0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x01\xDAW_5`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0\xFDW\x80c\xDC\x96\xFDP\x11a\0\x92W\x80c\xF2\xFD\xE3\x8B\x11a\0bW\x80c\xF2\xFD\xE3\x8B\x14a\x06cW\x80c\xF8>B\x92\x14a\x06\x82W\x80c\xFA\xD0\xE5\xA2\x14a\x06\x96W\x80c\xFD\x15:\xCF\x14a\x06\xB5W__\xFD[\x80c\xDC\x96\xFDP\x14a\x05\xCDW\x80c\xDD\xAD\x19\x02\x14a\x05\xE1W\x80c\xE9\xD3\xEE\x95\x14a\x06\x11W\x80c\xEA\nR7\x14a\x06DW__\xFD[\x80c\xAD<\xB1\xCC\x11a\0\xCDW\x80c\xAD<\xB1\xCC\x14a\x05(W\x80c\xB6T\x9Fu\x14a\x05XW\x80c\xC9f\xC4\xFE\x14a\x05lW\x80c\xD7\xB0\xFE\xF1\x14a\x05\x81W__\xFD[\x80c\x8D\xA5\xCB[\x14a\x04cW\x80c\xA2\xA0wV\x14a\x04\x9FW\x80c\xA8\xB4\xEE\xC9\x14a\x04\xCDW\x80c\xA9icZ\x14a\x04\xF9W__\xFD[\x80cR\xD1\x90-\x11a\x01sW\x80cqP\x18\xA6\x11a\x01CW\x80cqP\x18\xA6\x14a\x03\xC2W\x80crX\x1C\xC0\x14a\x03\xD6W\x80cw;J3\x14a\x04\tW\x80c\x82\xBF\xEF\xC8\x14a\x04,W__\xFD[\x80cR\xD1\x90-\x14a\x032W\x80cSfZ\xAA\x14a\x03FW\x80c`F4\xC9\x14a\x03rW\x80cm+\xEE\xF1\x14a\x03\xADW__\xFD[\x80c0\x8Cq.\x11a\x01\xAEW\x80c0\x8Cq.\x14a\x02\xB4W\x80cC\x9F\xAB\x91\x14a\x02\xD3W\x80cJ\xC3\xE4\xF2\x14a\x02\xF2W\x80cO\x1E\xF2\x86\x14a\x03\x1FW__\xFD[\x80b#\xDE)\x14a\x01\xDEW\x80c\r\xF1\x8F\x94\x14a\x01\xFFW\x80c\x10\xABR\x97\x14a\x02tW\x80c$MIn\x14a\x02\x95W[__\xFD[4\x80\x15a\x01\xE9W__\xFD[Pa\x01\xFDa\x01\xF86`\x04a \x9AV[a\x06\xCAV[\0[4\x80\x15a\x02\nW__\xFD[P`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x03Tc\xFF\xFF\xFF\xFF\x19\x81\x83\x1B\x16\x83R`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90\x82\x01R[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x19\x16\x81R` \x92\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x92\x81\x01\x92\x90\x92R\x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x7FW__\xFD[Pa\x02\x88a\tOV[`@Qa\x02k\x91\x90a!}V[4\x80\x15a\x02\xA0W__\xFD[Pa\x01\xFDa\x02\xAF6`\x04a!\xCAV[a\t`V[4\x80\x15a\x02\xBFW__\xFD[Pa\x01\xFDa\x02\xCE6`\x04a!\xE1V[a\ttV[4\x80\x15a\x02\xDEW__\xFD[Pa\x01\xFDa\x02\xED6`\x04a\"\xA3V[a\n>V[4\x80\x15a\x02\xFDW__\xFD[Pa\x03\x11a\x03\x0C6`\x04a\"\xDCV[a\r*V[`@Q\x90\x81R` \x01a\x02kV[a\x01\xFDa\x03-6`\x04a\"\xFFV[a\rNV[4\x80\x15a\x03=W__\xFD[Pa\x03\x11a\riV[4\x80\x15a\x03QW__\xFD[Pa\x03ea\x03`6`\x04a!\xE1V[a\r\x84V[`@Qa\x02k\x91\x90a#yV[4\x80\x15a\x03}W__\xFD[Pa\x03\x91a\x03\x8C6`\x04a!\xCAV[a\x0E\x1BV[`@\x80Q\x92\x15\x15\x83Rc\xFF\xFF\xFF\xFF\x90\x91\x16` \x83\x01R\x01a\x02kV[4\x80\x15a\x03\xB8W__\xFD[Pa\x03\x11`\x01T\x81V[4\x80\x15a\x03\xCDW__\xFD[Pa\x01\xFDa\x0E7V[4\x80\x15a\x03\xE1W__\xFD[Pa\x03\x11\x7F\xB2\x81\xFC\x8C\x12\x95M\"TM\xB4]\xE3\x15\x9A9'(\x95\xB1i\xA8R\xB3\x14\xF9\xCCv.D\xC5;\x81V[4\x80\x15a\x04\x14W__\xFD[P`@\x80Q_\x81Rc\xFF\xFF\xFF\xFF` \x82\x01R\x01a\x02kV[4\x80\x15a\x047W__\xFD[P`\x08Ta\x04K\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02kV[4\x80\x15a\x04nW__\xFD[P\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T`\x01`\x01`\xA0\x1B\x03\x16a\x04KV[4\x80\x15a\x04\xAAW__\xFD[Pa\x04\xBEa\x04\xB96`\x04a!\xCAV[a\x0EJV[`@Qa\x02k\x93\x92\x91\x90a#\x8BV[4\x80\x15a\x04\xD8W__\xFD[Pa\x04\xECa\x04\xE76`\x04a\"\xDCV[a\x0ErV[`@Qa\x02k\x91\x90a#\xADV[4\x80\x15a\x05\x04W__\xFD[Pa\x05\x18a\x05\x136`\x04a!\xCAV[a\x0E\x99V[`@Q\x90\x15\x15\x81R` \x01a\x02kV[4\x80\x15a\x053W__\xFD[Pa\x03e`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[4\x80\x15a\x05cW__\xFD[Pa\x01\xFDa\x0E\xB0V[4\x80\x15a\x05wW__\xFD[Pa\x03\x11`\x04T\x81V[4\x80\x15a\x05\x8CW__\xFD[P`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x02Tc\xFF\xFF\xFF\xFF\x19\x81\x83\x1B\x16\x83R`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90\x82\x01Ra\x02GV[4\x80\x15a\x05\xD8W__\xFD[Pa\x01\xFDa\x0FsV[4\x80\x15a\x05\xECW__\xFD[Pa\x03e`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03\"\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[4\x80\x15a\x06\x1CW__\xFD[Pa\x03\x11\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x06OW__\xFD[Pa\x01\xFDa\x06^6`\x04a#\xBBV[a\x10lV[4\x80\x15a\x06nW__\xFD[Pa\x01\xFDa\x06}6`\x04a!\xE1V[a\x11eV[4\x80\x15a\x06\x8DW__\xFD[Pa\x03\x11a\x11\x9FV[4\x80\x15a\x06\xA1W__\xFD[Pa\x01\xFDa\x06\xB06`\x04a#\xF9V[a\x11\xA9V[4\x80\x15a\x06\xC0W__\xFD[Pa\x03\x11`\x07T\x81V[`\x08T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06\xF5W`@QcPy\xFFu`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x86\x160\x14a\x07\x1EW`@Qc\x178\x92!`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x10\x15a\x07_W`@Qc\r=\xCD\xE5`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80\x80\x80\x80a\x07p\x88\x8A\x01\x8Aa$IV[_\x80T`@Qc\x02&^1`\xE6\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x89\x16`\x04\x83\x01R\x97\x9CP\x95\x9AP\x93\x98P\x91\x96P\x94P\x92\x16\x90c\x89\x97\x8C@\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xC5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xE9\x91\x90a$\xC3V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15a\x08\x14WP\x85`\x01`\x01`\xA0\x1B\x03\x16\x8D`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x80a\x08DWP`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x08DWP\x80`\x01`\x01`\xA0\x1B\x03\x16\x8D`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x08bW`@QcJ\x93\xB6!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x08o\x87\x87\x87\x87a\x12\xABV[P\x90P\x80\x15a\x08\x9FW`\x07T\x8C\x14a\x08\x9AW`@QcG2\xCE\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08\xBEV[\x8B\x15a\x08\xBEW`@Qc\x03h\x9F\xB1`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82Q\x15a\x08\xCFWa\x08\xCF\x87\x84a\x13\xECV[\x8B\x15a\t>W`\x08T`@\x80Qc\xFE\x9D\x93\x03`\xE0\x1B\x81R`\x04\x81\x01\x8F\x90R`$\x81\x01\x91\x90\x91R_`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xFE\x9D\x93\x03\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\t'W__\xFD[PZ\xF1\x15\x80\x15a\t9W=__>=_\xFD[PPPP[PPPPPPPPPPPPPPPV[``a\t[`\x05a\x15\x1AV[\x90P\x90V[a\tha\x15\xAFV[a\tq\x81a\x16\nV[PV[_T\x81\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16a\t\x9FW`@Qc\x08\xA9D\x19`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_T`@Qc\x02&^1`\xE6\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R3\x92\x16\x90c\x89\x97\x8C@\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xE6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\n\x91\x90a$\xC3V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\n1W`@Qc\xAC\xD5\xA8#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\n:\x82a\x16\x8BV[PPV[_a\nGa\x176V[\x80T\x90\x91P`\xFF`\x01`@\x1B\x82\x04\x16\x15\x90`\x01`\x01`@\x1B\x03\x16_\x81\x15\x80\x15a\nmWP\x82[\x90P_\x82`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\n\x88WP0;\x15[\x90P\x81\x15\x80\x15a\n\x96WP\x80\x15[\x15a\n\xB4W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x17\x85U\x83\x15a\n\xDEW\x84T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x85U[a\n\xEAb\x01Q\x80a\x17^V[____\x89\x80` \x01\x90Q\x81\x01\x90a\x0B\x02\x91\x90a$\xDEV[\x92\x96P\x90\x94P\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x84\x16a\x0BhW`@Qc\xEA\xC0\xD3\x89`\xE0\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7F_token must not be empty\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x0B\xBFW`@Qc\xEA\xC0\xD3\x89`\xE0\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7F_safeRegistry must not be empty\0`D\x82\x01R`d\x01a\x0B_V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0C\x16W`@Qc\xEA\xC0\xD3\x89`\xE0\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7F_owner must not be empty\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0B_V[`\x08\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x86\x16\x17\x90Ua\x0C:\x83a\x17\xEDV[`@Qc)\x96Z\x1D`\xE0\x1B\x81R0`\x04\x82\x01\x81\x90R\x7F\xB2\x81\xFC\x8C\x12\x95M\"TM\xB4]\xE3\x15\x9A9'(\x95\xB1i\xA8R\xB3\x14\xF9\xCCv.D\xC5;`$\x83\x01R`D\x82\x01Rs\x18 \xA4\xB7a\x8B\xDEq\xDC\xE8\xCD\xC7:\xABl\x95\x90_\xAD$\x90c)\x96Z\x1D\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0C\xB0W__\xFD[PZ\xF1\x15\x80\x15a\x0C\xC2W=__>=_\xFD[PPPPa\x0C\xCF\x82a\x16\nV[a\x0C\xD8\x81a\x18eV[PPPP\x83\x15a\r\"W\x84T`\xFF`@\x1B\x19\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPV[_c\xFF\xFF\xFF\xFF\x80\x83\x16\x90a\rC\x90`\x05\x90\x83\x90a\x18m\x16V[`@\x01Q\x93\x92PPPV[a\rVa\x18\xE4V[a\r_\x82a\x19\x88V[a\n:\x82\x82a\x19\x90V[_a\rra\x1AQV[P_Q` a'\xB2_9_Q\x90_R\x90V[`\t` R_\x90\x81R`@\x90 \x80Ta\r\x9C\x90a%0V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\r\xC8\x90a%0V[\x80\x15a\x0E\x13W\x80`\x1F\x10a\r\xEAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0E\x13V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\r\xF6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[_\x80\x80\x80a\x0E*`\x05\x86a\x1A\x9AV[P\x90\x96\x90\x95P\x93PPPPV[a\x0E?a\x15\xAFV[a\x0EH_a\x1B\x82V[V[__a\x0ETa\x1F\xC5V[_\x80\x80a\x0Eb`\x05\x88a\x1A\x9AV[\x91\x99\x90\x98P\x90\x96P\x94PPPPPV[a\x0Eza\x1F\xC5V[c\xFF\xFF\xFF\xFF\x80\x83\x16\x90a\x0E\x92\x90`\x05\x90\x83\x90a\x18m\x16V[\x93\x92PPPV[_\x81\x81R`\x06` R`@\x81 T\x15\x15[\x92\x91PPV[_T`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x0E\xD9W`@Qc\x08\xA9D\x19`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80T`@Qc\x02&^1`\xE6\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x89\x97\x8C@\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x1FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0FC\x91\x90a$\xC3V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0FjW`@Qc\xAC\xD5\xA8#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0EH3a\x16\x8BV[`@\x80Q\x80\x82\x01\x82R`\n\x81Ri$7\xB89&2\xB23\xB2\xB9`\xB1\x1B` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`\x05\x81Rd\x03\"\xE3\x02\xE3`\xDC\x1B\x90\x82\x01R\x81Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81R\x7Fl\xD6\x81y\x0Cx\xC2 Q{\t\x9As\x7F\x8E\x85\xF6\x9Eyz\xBEN)\x10\xFB\x18\x9Ba\xDBK\xF2\xCD\x91\x81\x01\x91\x90\x91R\x7F\xB4\xBC\xB1T\xE3\x86\x01\xC3\x899o\xA9\x181M\xA4-F&\xF1>\xF6\xD0\xCE\xB0~_]&\xB2\xFB\xC3\x91\x81\x01\x91\x90\x91RF``\x82\x01R0`\x80\x82\x01R`\xA0\x90 `\x04T\x81\x14a\tqW`\x04\x81\x90U`@Q\x81\x90\x7F\xA4?\xAD\x83\x92\x0F\xD0\x94E\x85^\x85Ns\xC9\xC52\xE1t\x02\xC9\xCE\xB0\x99\x93\xA29(C\xA5\xBD\xB9\x90_\x90\xA2PV[_T`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x10\x95W`@Qc\x08\xA9D\x19`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80T`@Qc\x02&^1`\xE6\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x89\x97\x8C@\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xDBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xFF\x91\x90a$\xC3V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x11&W`@Qc\xAC\xD5\xA8#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\n:3\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x13\xEC\x92PPPV[a\x11ma\x15\xAFV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x11\x96W`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R_`\x04\x82\x01R`$\x01a\x0B_V[a\tq\x81a\x1B\x82V[_a\t[`\x05T\x90V[_T\x83\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x11\xD4W`@Qc\x08\xA9D\x19`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_T`@Qc\x02&^1`\xE6\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R3\x92\x16\x90c\x89\x97\x8C@\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\x1BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12?\x91\x90a$\xC3V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x12fW`@Qc\xAC\xD5\xA8#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x12\xA5\x84\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x13\xEC\x92PPPV[PPPPV[_\x80\x80\x80a\x12\xBA`\x05\x86a\x1A\x9AV[P\x91P\x91P\x81\x15a\x12\xD1W_\x93P\x91Pa\x13\xE3\x90PV[a\x13\x0E`@Q\x80`\x80\x01`@R\x80\x89\x81R` \x01\x88\x81R` \x01\x87\x81R` \x01\x8A`\x01`\x01`\xA0\x1B\x03\x16\x81RP`\x05a\x1B\xF2\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`@\x80Q\x7F\x11\xD9FS\xDER\xFC\x148\x87\x95o\xE4\xB5\xE4\x12\n\x85\x0B\x13\0\xEC\xE0S=\xEEk\xD8\xFF\x91\xF72` \x82\x01R\x90\x81\x01\x89\x90R``\x80\x82\x01\x89\x90R`\x80\x82\x01\x88\x90R\x8A\x90\x1B`\x01`\x01``\x1B\x03\x19\x16`\xA0\x82\x01R`\xB4\x81\x01\x82\x90R\x90\x93Pa\x13\x85\x90`\xD4\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x1C\xF4V[`@\x80Q\x88\x81R` \x81\x01\x88\x90R\x90\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x89\x16``\x82\x01R`\x80\x81\x01\x84\x90R\x7F\x11\xD9FS\xDER\xFC\x148\x87\x95o\xE4\xB5\xE4\x12\n\x85\x0B\x13\0\xEC\xE0S=\xEEk\xD8\xFF\x91\xF72\x90`\xA0\x01`@Q\x80\x91\x03\x90\xA1`\x01\x93PPP[\x94P\x94\x92PPPV[\x80Q_\x03a\x14\x0CW`@Qb\x9C\xEA\xC7`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\t` R`@\x90 \x80Ta\x14.\x90a%0V[\x15\x80\x15\x91Pa\x14zWP\x80\x80Q\x90` \x01 `\t_\x84`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ `@Qa\x14p\x91\x90a%hV[`@Q\x80\x91\x03\x90 \x14[\x15a\x14\x83WPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\t` R`@\x90 a\x14\xA4\x82\x82a&$V[Pa\x14\xDD\x7F\xC4\xDF[\xA1h\x14\x83\x8A\xB2a\x88)\xD6\x8F\x86#\xBB\x89s\x02\xF2M\xBD\xBA\"y\xDB\xE4Z\xDB=\x14\x83\x83`@Q` \x01a\x13q\x93\x92\x91\x90a&\xF5V[\x7F\xC4\xDF[\xA1h\x14\x83\x8A\xB2a\x88)\xD6\x8F\x86#\xBB\x89s\x02\xF2M\xBD\xBA\"y\xDB\xE4Z\xDB=\x14\x82\x82`@Qa\x15\x0E\x92\x91\x90a'\x1EV[`@Q\x80\x91\x03\x90\xA1PPV[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x15\xA4W_\x84\x81R` \x90\x81\x90 `@\x80Q`\xA0\x81\x01\x82R`\x05\x86\x02\x90\x92\x01\x80T\x83R`\x01\x80\x82\x01T\x84\x86\x01R`\x02\x82\x01T\x92\x84\x01\x92\x90\x92R`\x03\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16``\x84\x01R`\x04\x01T`\x80\x83\x01R\x90\x83R\x90\x92\x01\x91\x01a\x15>V[PPPP\x90P\x91\x90PV[3a\x15\xE1\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0EHW`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x0B_V[`\x07\x80T\x90\x82\x90U`@\x80Q\x7Fi%\x94\\\xB4\xC6\xB5\x97\xA3a\x80\x9D[Q\x10~N\x03r\x04\x04\xA8$\x83\x85>\x91\xFA\xA1v\xB8\x8E` \x82\x01R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90Ra\x16U\x90`\x80\x01a\x13qV[`@\x80Q\x83\x81R` \x81\x01\x83\x90R\x7Fi%\x94\\\xB4\xC6\xB5\x97\xA3a\x80\x9D[Q\x10~N\x03r\x04\x04\xA8$\x83\x85>\x91\xFA\xA1v\xB8\x8E\x91\x01a\x15\x0EV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\t` R`@\x81 a\x16\xAB\x91a\x1F\xF8V[a\x16\xF7\x7F\xA4\xDE0\xA5(\xBE\xCA\xDF\x82d\x9D\x13\x95\xC0\xE3\r\xD1\x8A\xE3[Z\x96\xCEq\xE9)[\xB1K\xC9\xF3\xBC\x82`@Q` \x01a\x13q\x92\x91\x90\x91\x82R``\x1B`\x01`\x01``\x1B\x03\x19\x16` \x82\x01R`4\x01\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x81R\x7F\xA4\xDE0\xA5(\xBE\xCA\xDF\x82d\x9D\x13\x95\xC0\xE3\r\xD1\x8A\xE3[Z\x96\xCEq\xE9)[\xB1K\xC9\xF3\xBC\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[_\x80\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0a\x0E\xAAV[\x80_\x03a\x17~W`@Qc4o\xF6\x07`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x81\x90U`@Q`\x01`\x01``\x1B\x03\x190``\x1B\x16` \x82\x01R`4\x01`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x91\x82\x01 \x90\x1C`\x01`\xE0\x1BBc\xFF\xFF\xFF\xFF\x90\x81\x16\x82\x02\x92\x90\x92\x17`\x02\x81\x90U`\x01`\x01`\xE0\x1B\x03\x81\x16\x90\x82\x90\x04\x90\x92\x16\x02\x17`\x03Ua\tqa\x0FsV[_T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x18\x16W`@Qb\xDC\x14\x9F`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x18=W`@QcGN\xBE/`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x01`\x01`\xA8\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17`\x01`\xA0\x1B\x17\x90UV[a\x11ma\x1D\xBAV[a\x18ua\x1F\xC5V[\x82_\x01\x82\x81T\x81\x10a\x18\x89Wa\x18\x89a'AV[_\x91\x82R` \x91\x82\x90 `@\x80Q`\xA0\x81\x01\x82R`\x05\x90\x93\x02\x90\x91\x01\x80T\x83R`\x01\x81\x01T\x93\x83\x01\x93\x90\x93R`\x02\x83\x01T\x90\x82\x01R`\x03\x82\x01T`\x01`\x01`\xA0\x1B\x03\x16``\x82\x01R`\x04\x90\x91\x01T`\x80\x82\x01R\x90P\x92\x91PPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x19jWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x19^_Q` a'\xB2_9_Q\x90_RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x0EHW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\tqa\x15\xAFV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x19\xEAWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x19\xE7\x91\x81\x01\x90a'UV[`\x01[a\x1A\x12W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x0B_V[_Q` a'\xB2_9_Q\x90_R\x81\x14a\x1ABW`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x0B_V[a\x1AL\x83\x83a\x1D\xDFV[PPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0EHW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__a\x1A\xA4a\x1F\xC5V[_\x84\x81R`\x01\x86\x01` R`@\x81 T\x90\x81\x90\x03a\x1A\xF2WPP`@\x80Q`\xA0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R\x90\x92P\x82\x91Pa\x1B{V[`\x01a\x1A\xFE\x81\x83a'\x80V[\x87a\x1B\n`\x01\x85a'\x80V[\x81T\x81\x10a\x1B\x1AWa\x1B\x1Aa'AV[_\x91\x82R` \x91\x82\x90 `@\x80Q`\xA0\x81\x01\x82R`\x05\x90\x93\x02\x90\x91\x01\x80T\x83R`\x01\x81\x01T\x93\x83\x01\x93\x90\x93R`\x02\x83\x01T\x90\x82\x01R`\x03\x82\x01T`\x01`\x01`\xA0\x1B\x03\x16``\x82\x01R`\x04\x90\x91\x01T`\x80\x82\x01R\x91\x95P\x93P\x91Pa\x1B{\x90PV[\x92P\x92P\x92V[\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x82\x17\x84U`@Q\x92\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPPV[\x81T_\x90c\xFF\xFF\xFF\xFF\x11a\x1C\x19W`@Qc\x10\x95;\xFF`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80\x83\x01Q_\x90\x81R`\x01\x85\x01` R T\x15a\x1CIW`@QbXLo`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\xA0\x81\x01\x82R\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01\x90\x81R\x85\x84\x01\x80Q\x84\x86\x01\x90\x81R``\x80\x89\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x87\x01\x91\x82RB`\x80\x88\x01\x90\x81R\x8BT`\x01\x80\x82\x01\x8EU_\x8E\x81R\x89\x81 \x9AQ`\x05\x90\x93\x02\x90\x9A\x01\x91\x82U\x96Q\x81\x88\x01U\x93Q`\x02\x85\x01U\x91Q`\x03\x84\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x92\x16\x17\x90UQ`\x04\x90\x91\x01U\x87T\x90Q\x84R\x81\x88\x01\x90\x92R\x92\x90\x91 \x81\x90Ua\x0E\x92\x91\x90a'\x80V[`\x01T`\x02T_\x91a\x1D\x12\x91`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a'\x93V[B\x11\x15a\x1D\x1DWP`\x01[`\x04T`\x02T\x83Q` \x80\x86\x01\x91\x90\x91 `@\x80Q\x80\x84\x01\x95\x90\x95RC`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x90\x85\x01R\x91\x90\x1Bc\xFF\xFF\xFF\xFF\x19\x16`D\x83\x01R``\x82\x01R`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x91\x82\x01 c\xFF\xFF\xFF\xFFB\x16`\x01`\xE0\x1B\x02\x91\x1C\x17`\x02U\x80\x15a\n:WPP`\x02T`\x01`\x01`\xE0\x1B\x03\x81\x16`\x01`\xE0\x1B\x91\x82\x90\x04c\xFF\xFF\xFF\xFF\x16\x90\x91\x02\x17`\x03UV[a\x1D\xC2a\x1E4V[a\x0EHW`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1D\xE8\x82a\x1EMV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15a\x1E,Wa\x1AL\x82\x82a\x1E\xB0V[a\n:a\x1F\"V[_a\x1E=a\x176V[T`\x01`@\x1B\x90\x04`\xFF\x16\x91\x90PV[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03a\x1E\x82W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x0B_V[_Q` a'\xB2_9_Q\x90_R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``__\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x1E\xCC\x91\x90a'\xA6V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a\x1F\x04W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x1F\tV[``\x91P[P\x91P\x91Pa\x1F\x19\x85\x83\x83a\x1FAV[\x95\x94PPPPPV[4\x15a\x0EHW`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x82a\x1FVWa\x1FQ\x82a\x1F\x9DV[a\x0E\x92V[\x81Q\x15\x80\x15a\x1FmWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x1F\x96W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x0B_V[P\x92\x91PPV[\x80Q\x15a\x1F\xACW\x80Q` \x82\x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x80`\xA0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81RP\x90V[P\x80Ta \x04\x90a%0V[_\x82U\x80`\x1F\x10a \x13WPPV[`\x1F\x01` \x90\x04\x90_R` _ \x90\x81\x01\x90a\tq\x91\x90[\x80\x82\x11\x15a >W_\x81U`\x01\x01a +V[P\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\tqW__\xFD[__\x83`\x1F\x84\x01\x12a fW__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a |W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a \x93W__\xFD[\x92P\x92\x90PV[________`\xC0\x89\x8B\x03\x12\x15a \xB1W__\xFD[\x885a \xBC\x81a BV[\x97P` \x89\x015a \xCC\x81a BV[\x96P`@\x89\x015a \xDC\x81a BV[\x95P``\x89\x015\x94P`\x80\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a \xFDW__\xFD[a!\t\x8B\x82\x8C\x01a VV[\x90\x95P\x93PP`\xA0\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a!'W__\xFD[a!3\x8B\x82\x8C\x01a VV[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x80\x82\x01Q\x90\x83\x01R``\x80\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90\x83\x01R`\x80\x90\x81\x01Q\x91\x01RV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a!\xBFWa!\xA9\x83\x85Qa!GV[` \x93\x90\x93\x01\x92`\xA0\x92\x90\x92\x01\x91`\x01\x01a!\x96V[P\x90\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a!\xDAW__\xFD[P5\x91\x90PV[_` \x82\x84\x03\x12\x15a!\xF1W__\xFD[\x815a\x0E\x92\x81a BV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[__`\x01`\x01`@\x1B\x03\x84\x11\x15a\")Wa\")a!\xFCV[P`@Q`\x1F\x19`\x1F\x85\x01\x81\x16`?\x01\x16\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\"WWa\"Wa!\xFCV[`@R\x83\x81R\x90P\x80\x82\x84\x01\x85\x10\x15a\"nW__\xFD[\x83\x83` \x83\x017_` \x85\x83\x01\x01RP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\"\x94W__\xFD[a\x0E\x92\x83\x835` \x85\x01a\"\x10V[_` \x82\x84\x03\x12\x15a\"\xB3W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xC8W__\xFD[a\"\xD4\x84\x82\x85\x01a\"\x85V[\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a\"\xECW__\xFD[\x815c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0E\x92W__\xFD[__`@\x83\x85\x03\x12\x15a#\x10W__\xFD[\x825a#\x1B\x81a BV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a#5W__\xFD[a#A\x85\x82\x86\x01a\"\x85V[\x91PP\x92P\x92\x90PV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x0E\x92` \x83\x01\x84a#KV[\x83\x15\x15\x81Rc\xFF\xFF\xFF\xFF\x83\x16` \x82\x01R`\xE0\x81\x01a\"\xD4`@\x83\x01\x84a!GV[`\xA0\x81\x01a\x0E\xAA\x82\x84a!GV[__` \x83\x85\x03\x12\x15a#\xCCW__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a#\xE1W__\xFD[a#\xED\x85\x82\x86\x01a VV[\x90\x96\x90\x95P\x93PPPPV[___`@\x84\x86\x03\x12\x15a$\x0BW__\xFD[\x835a$\x16\x81a BV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a$0W__\xFD[a$<\x86\x82\x87\x01a VV[\x94\x97\x90\x96P\x93\x94PPPPV[_____`\xA0\x86\x88\x03\x12\x15a$]W__\xFD[\x855a$h\x81a BV[\x94P` \x86\x015\x93P`@\x86\x015\x92P``\x86\x015\x91P`\x80\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a$\x97W__\xFD[\x86\x01`\x1F\x81\x01\x88\x13a$\xA7W__\xFD[a$\xB6\x88\x825` \x84\x01a\"\x10V[\x91PP\x92\x95P\x92\x95\x90\x93PV[_` \x82\x84\x03\x12\x15a$\xD3W__\xFD[\x81Qa\x0E\x92\x81a BV[____`\x80\x85\x87\x03\x12\x15a$\xF1W__\xFD[\x84Qa$\xFC\x81a BV[` \x86\x01Q\x90\x94Pa%\r\x81a BV[`@\x86\x01Q``\x87\x01Q\x91\x94P\x92Pa%%\x81a BV[\x93\x96\x92\x95P\x90\x93PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a%DW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a%bWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[__\x83Ta%u\x81a%0V[`\x01\x82\x16\x80\x15a%\x8CW`\x01\x81\x14a%\xA1Wa%\xCEV[`\xFF\x19\x83\x16\x86R\x81\x15\x15\x82\x02\x86\x01\x93Pa%\xCEV[\x86_R` _ _[\x83\x81\x10\x15a%\xC6W\x81T\x88\x82\x01R`\x01\x90\x91\x01\x90` \x01a%\xAAV[PP\x81\x86\x01\x93P[P\x91\x95\x94PPPPPV[`\x1F\x82\x11\x15a\x1ALW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a%\xFEWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a&\x1DW_\x81U`\x01\x01a&\nV[PPPPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a&=Wa&=a!\xFCV[a&Q\x81a&K\x84Ta%0V[\x84a%\xD9V[` `\x1F\x82\x11`\x01\x81\x14a&\x83W_\x83\x15a&lWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua&\x1DV[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a&\xB2W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a&\x92V[P\x84\x82\x10\x15a&\xCFW\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[\x83\x81Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x83``\x1B\x16` \x82\x01R_a\x1F\x19`4\x83\x01\x84a&\xDEV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90a\"\xD4\x90\x83\x01\x84a#KV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a'eW__\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0E\xAAWa\x0E\xAAa'lV[\x80\x82\x01\x80\x82\x11\x15a\x0E\xAAWa\x0E\xAAa'lV[_a\x0E\x92\x82\x84a&\xDEV\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\xA2dipfsX\"\x12 VSk\x14\xD5S\xD1\xF7Nn\xF4>\xB2\"2Z\xE8Fv\x1Ff\x80L\xDC\x0BH\xA8\xEB2\x1C\x01\xD7dsolcC\0\x08\x1E\x003",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KeyId(u32);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<KeyId> for u32 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                32,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<32>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        impl KeyId {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(value: u32) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(self) -> u32 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl From<u32> for KeyId {
            fn from(value: u32) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<KeyId> for u32 {
            fn from(value: KeyId) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for KeyId {
            type RustType = u32;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                32,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                32,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                32,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for KeyId {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct KeyBindingWithSignatureTimestamp { bytes32 ed25519_sig_0; bytes32 ed25519_sig_1; bytes32 ed25519_pub_key; address chain_key; uint256 boundTimestamp; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KeyBindingWithSignatureTimestamp {
        #[allow(missing_docs)]
        pub ed25519_sig_0: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub ed25519_sig_1: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub ed25519_pub_key: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub chain_key: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub boundTimestamp: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<KeyBindingWithSignatureTimestamp>
        for UnderlyingRustTuple<'_> {
            fn from(value: KeyBindingWithSignatureTimestamp) -> Self {
                (
                    value.ed25519_sig_0,
                    value.ed25519_sig_1,
                    value.ed25519_pub_key,
                    value.chain_key,
                    value.boundTimestamp,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for KeyBindingWithSignatureTimestamp {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    ed25519_sig_0: tuple.0,
                    ed25519_sig_1: tuple.1,
                    ed25519_pub_key: tuple.2,
                    chain_key: tuple.3,
                    boundTimestamp: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for KeyBindingWithSignatureTimestamp {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self>
        for KeyBindingWithSignatureTimestamp {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.ed25519_sig_0),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.ed25519_sig_1),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.ed25519_pub_key),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.chain_key,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.boundTimestamp),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for KeyBindingWithSignatureTimestamp {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for KeyBindingWithSignatureTimestamp {
            const NAME: &'static str = "KeyBindingWithSignatureTimestamp";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "KeyBindingWithSignatureTimestamp(bytes32 ed25519_sig_0,bytes32 ed25519_sig_1,bytes32 ed25519_pub_key,address chain_key,uint256 boundTimestamp)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.ed25519_sig_0)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.ed25519_sig_1)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.ed25519_pub_key,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.chain_key,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.boundTimestamp,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for KeyBindingWithSignatureTimestamp {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.ed25519_sig_0,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.ed25519_sig_1,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.ed25519_pub_key,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.chain_key,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.boundTimestamp,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.ed25519_sig_0,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.ed25519_sig_1,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.ed25519_pub_key,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.chain_key,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.boundTimestamp,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `AddressEmptyCode(address)` and selector `0x9996b315`.
```solidity
error AddressEmptyCode(address target);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AddressEmptyCode {
        #[allow(missing_docs)]
        pub target: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<AddressEmptyCode> for UnderlyingRustTuple<'_> {
            fn from(value: AddressEmptyCode) -> Self {
                (value.target,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AddressEmptyCode {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { target: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AddressEmptyCode {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AddressEmptyCode(address)";
            const SELECTOR: [u8; 4] = [153u8, 150u8, 179u8, 21u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.target,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `AlreadyInitialized()` and selector `0x0dc149f0`.
```solidity
error AlreadyInitialized();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AlreadyInitialized;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<AlreadyInitialized> for UnderlyingRustTuple<'_> {
            fn from(value: AlreadyInitialized) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AlreadyInitialized {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AlreadyInitialized {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AlreadyInitialized()";
            const SELECTOR: [u8; 4] = [13u8, 193u8, 73u8, 240u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ContractNotResponsible()` and selector `0xacd5a823`.
```solidity
error ContractNotResponsible();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ContractNotResponsible;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ContractNotResponsible> for UnderlyingRustTuple<'_> {
            fn from(value: ContractNotResponsible) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ContractNotResponsible {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ContractNotResponsible {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ContractNotResponsible()";
            const SELECTOR: [u8; 4] = [172u8, 213u8, 168u8, 35u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ERC1967InvalidImplementation(address)` and selector `0x4c9c8ce3`.
```solidity
error ERC1967InvalidImplementation(address implementation);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC1967InvalidImplementation {
        #[allow(missing_docs)]
        pub implementation: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ERC1967InvalidImplementation>
        for UnderlyingRustTuple<'_> {
            fn from(value: ERC1967InvalidImplementation) -> Self {
                (value.implementation,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ERC1967InvalidImplementation {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { implementation: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC1967InvalidImplementation {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC1967InvalidImplementation(address)";
            const SELECTOR: [u8; 4] = [76u8, 156u8, 140u8, 227u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.implementation,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ERC1967NonPayable()` and selector `0xb398979f`.
```solidity
error ERC1967NonPayable();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC1967NonPayable;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ERC1967NonPayable> for UnderlyingRustTuple<'_> {
            fn from(value: ERC1967NonPayable) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC1967NonPayable {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC1967NonPayable {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC1967NonPayable()";
            const SELECTOR: [u8; 4] = [179u8, 152u8, 151u8, 159u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `EmptyMultiaddr()` and selector `0x09ceac70`.
```solidity
error EmptyMultiaddr();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyMultiaddr;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<EmptyMultiaddr> for UnderlyingRustTuple<'_> {
            fn from(value: EmptyMultiaddr) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptyMultiaddr {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyMultiaddr {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyMultiaddr()";
            const SELECTOR: [u8; 4] = [9u8, 206u8, 172u8, 112u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ExistingKeyBinding()` and selector `0x0b098de0`.
```solidity
error ExistingKeyBinding();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ExistingKeyBinding;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ExistingKeyBinding> for UnderlyingRustTuple<'_> {
            fn from(value: ExistingKeyBinding) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ExistingKeyBinding {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ExistingKeyBinding {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ExistingKeyBinding()";
            const SELECTOR: [u8; 4] = [11u8, 9u8, 141u8, 224u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `FailedCall()` and selector `0xd6bda275`.
```solidity
error FailedCall();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FailedCall;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<FailedCall> for UnderlyingRustTuple<'_> {
            fn from(value: FailedCall) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FailedCall {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for FailedCall {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FailedCall()";
            const SELECTOR: [u8; 4] = [214u8, 189u8, 162u8, 117u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidInitialization()` and selector `0xf92ee8a9`.
```solidity
error InvalidInitialization();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidInitialization;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidInitialization> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidInitialization) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidInitialization {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidInitialization {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidInitialization()";
            const SELECTOR: [u8; 4] = [249u8, 46u8, 232u8, 169u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidKeyBindingFeeAmount()` and selector `0x4732cefd`.
```solidity
error InvalidKeyBindingFeeAmount();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidKeyBindingFeeAmount;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidKeyBindingFeeAmount>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidKeyBindingFeeAmount) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidKeyBindingFeeAmount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidKeyBindingFeeAmount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidKeyBindingFeeAmount()";
            const SELECTOR: [u8; 4] = [71u8, 50u8, 206u8, 253u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidSafeAddress()` and selector `0x8e9d7c5e`.
```solidity
error InvalidSafeAddress();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidSafeAddress;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidSafeAddress> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidSafeAddress) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidSafeAddress {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidSafeAddress {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidSafeAddress()";
            const SELECTOR: [u8; 4] = [142u8, 157u8, 124u8, 94u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidTokenRecipient()` and selector `0xb9c49108`.
```solidity
error InvalidTokenRecipient();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidTokenRecipient;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidTokenRecipient> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidTokenRecipient) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidTokenRecipient {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidTokenRecipient {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidTokenRecipient()";
            const SELECTOR: [u8; 4] = [185u8, 196u8, 145u8, 8u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidTokenSender()` and selector `0x95276c42`.
```solidity
error InvalidTokenSender();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidTokenSender;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidTokenSender> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidTokenSender) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidTokenSender {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidTokenSender {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidTokenSender()";
            const SELECTOR: [u8; 4] = [149u8, 39u8, 108u8, 66u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidTokensReceivedUsage()` and selector `0x69ee6f28`.
```solidity
error InvalidTokensReceivedUsage();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidTokensReceivedUsage;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidTokensReceivedUsage>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidTokensReceivedUsage) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidTokensReceivedUsage {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidTokensReceivedUsage {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidTokensReceivedUsage()";
            const SELECTOR: [u8; 4] = [105u8, 238u8, 111u8, 40u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `KeyIdOutOfRange()` and selector `0x212a77fe`.
```solidity
error KeyIdOutOfRange();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KeyIdOutOfRange;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<KeyIdOutOfRange> for UnderlyingRustTuple<'_> {
            fn from(value: KeyIdOutOfRange) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for KeyIdOutOfRange {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KeyIdOutOfRange {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KeyIdOutOfRange()";
            const SELECTOR: [u8; 4] = [33u8, 42u8, 119u8, 254u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `MultiSigUninitialized()` and selector `0x454a20c8`.
```solidity
error MultiSigUninitialized();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MultiSigUninitialized;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<MultiSigUninitialized> for UnderlyingRustTuple<'_> {
            fn from(value: MultiSigUninitialized) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MultiSigUninitialized {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MultiSigUninitialized {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MultiSigUninitialized()";
            const SELECTOR: [u8; 4] = [69u8, 74u8, 32u8, 200u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `NoNeedToProvideKeyBindingFee()` and selector `0x0da27ec4`.
```solidity
error NoNeedToProvideKeyBindingFee();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NoNeedToProvideKeyBindingFee;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NoNeedToProvideKeyBindingFee>
        for UnderlyingRustTuple<'_> {
            fn from(value: NoNeedToProvideKeyBindingFee) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for NoNeedToProvideKeyBindingFee {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NoNeedToProvideKeyBindingFee {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NoNeedToProvideKeyBindingFee()";
            const SELECTOR: [u8; 4] = [13u8, 162u8, 126u8, 196u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `NotInitializing()` and selector `0xd7e6bcf8`.
```solidity
error NotInitializing();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotInitializing;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NotInitializing> for UnderlyingRustTuple<'_> {
            fn from(value: NotInitializing) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotInitializing {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotInitializing {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotInitializing()";
            const SELECTOR: [u8; 4] = [215u8, 230u8, 188u8, 248u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `OwnableInvalidOwner(address)` and selector `0x1e4fbdf7`.
```solidity
error OwnableInvalidOwner(address owner);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OwnableInvalidOwner {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OwnableInvalidOwner> for UnderlyingRustTuple<'_> {
            fn from(value: OwnableInvalidOwner) -> Self {
                (value.owner,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OwnableInvalidOwner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { owner: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OwnableInvalidOwner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OwnableInvalidOwner(address)";
            const SELECTOR: [u8; 4] = [30u8, 79u8, 189u8, 247u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `OwnableUnauthorizedAccount(address)` and selector `0x118cdaa7`.
```solidity
error OwnableUnauthorizedAccount(address account);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OwnableUnauthorizedAccount {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OwnableUnauthorizedAccount>
        for UnderlyingRustTuple<'_> {
            fn from(value: OwnableUnauthorizedAccount) -> Self {
                (value.account,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OwnableUnauthorizedAccount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { account: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OwnableUnauthorizedAccount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OwnableUnauthorizedAccount(address)";
            const SELECTOR: [u8; 4] = [17u8, 140u8, 218u8, 167u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `UUPSUnauthorizedCallContext()` and selector `0xe07c8dba`.
```solidity
error UUPSUnauthorizedCallContext();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UUPSUnauthorizedCallContext;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UUPSUnauthorizedCallContext>
        for UnderlyingRustTuple<'_> {
            fn from(value: UUPSUnauthorizedCallContext) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for UUPSUnauthorizedCallContext {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UUPSUnauthorizedCallContext {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UUPSUnauthorizedCallContext()";
            const SELECTOR: [u8; 4] = [224u8, 124u8, 141u8, 186u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `UUPSUnsupportedProxiableUUID(bytes32)` and selector `0xaa1d49a4`.
```solidity
error UUPSUnsupportedProxiableUUID(bytes32 slot);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UUPSUnsupportedProxiableUUID {
        #[allow(missing_docs)]
        pub slot: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UUPSUnsupportedProxiableUUID>
        for UnderlyingRustTuple<'_> {
            fn from(value: UUPSUnsupportedProxiableUUID) -> Self {
                (value.slot,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for UUPSUnsupportedProxiableUUID {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { slot: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UUPSUnsupportedProxiableUUID {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UUPSUnsupportedProxiableUUID(bytes32)";
            const SELECTOR: [u8; 4] = [170u8, 29u8, 73u8, 164u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.slot),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `WrongToken()` and selector `0xa0f3feea`.
```solidity
error WrongToken();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WrongToken;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<WrongToken> for UnderlyingRustTuple<'_> {
            fn from(value: WrongToken) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for WrongToken {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for WrongToken {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WrongToken()";
            const SELECTOR: [u8; 4] = [160u8, 243u8, 254u8, 234u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ZeroAddress(string)` and selector `0xeac0d389`.
```solidity
error ZeroAddress(string reason);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroAddress {
        #[allow(missing_docs)]
        pub reason: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ZeroAddress> for UnderlyingRustTuple<'_> {
            fn from(value: ZeroAddress) -> Self {
                (value.reason,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ZeroAddress {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { reason: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ZeroAddress {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ZeroAddress(string)";
            const SELECTOR: [u8; 4] = [234u8, 192u8, 211u8, 137u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.reason,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ZeroInterval()` and selector `0x346ff607`.
```solidity
error ZeroInterval();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroInterval;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ZeroInterval> for UnderlyingRustTuple<'_> {
            fn from(value: ZeroInterval) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ZeroInterval {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ZeroInterval {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ZeroInterval()";
            const SELECTOR: [u8; 4] = [52u8, 111u8, 246u8, 7u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `AddressAnnouncement(address,string)` and selector `0xc4df5ba16814838ab2618829d68f8623bb897302f24dbdba2279dbe45adb3d14`.
```solidity
event AddressAnnouncement(address node, string baseMultiaddr);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct AddressAnnouncement {
        #[allow(missing_docs)]
        pub node: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub baseMultiaddr: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for AddressAnnouncement {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::String,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "AddressAnnouncement(address,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                196u8, 223u8, 91u8, 161u8, 104u8, 20u8, 131u8, 138u8, 178u8, 97u8, 136u8,
                41u8, 214u8, 143u8, 134u8, 35u8, 187u8, 137u8, 115u8, 2u8, 242u8, 77u8,
                189u8, 186u8, 34u8, 121u8, 219u8, 228u8, 90u8, 219u8, 61u8, 20u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    node: data.0,
                    baseMultiaddr: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.node,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.baseMultiaddr,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for AddressAnnouncement {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&AddressAnnouncement> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &AddressAnnouncement) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Initialized(uint64)` and selector `0xc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2`.
```solidity
event Initialized(uint64 version);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Initialized {
        #[allow(missing_docs)]
        pub version: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Initialized {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Initialized(uint64)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                199u8, 245u8, 5u8, 178u8, 243u8, 113u8, 174u8, 33u8, 117u8, 238u8, 73u8,
                19u8, 244u8, 73u8, 158u8, 31u8, 38u8, 51u8, 167u8, 181u8, 147u8, 99u8,
                33u8, 238u8, 209u8, 205u8, 174u8, 182u8, 17u8, 81u8, 129u8, 210u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { version: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.version),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Initialized {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Initialized> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Initialized) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `KeyBinding(bytes32,bytes32,bytes32,address,uint256)` and selector `0x11d94653de52fc143887956fe4b5e4120a850b1300ece0533dee6bd8ff91f732`.
```solidity
event KeyBinding(bytes32 ed25519_sig_0, bytes32 ed25519_sig_1, bytes32 ed25519_pub_key, address chain_key, uint256 key_id);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct KeyBinding {
        #[allow(missing_docs)]
        pub ed25519_sig_0: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub ed25519_sig_1: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub ed25519_pub_key: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub chain_key: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub key_id: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for KeyBinding {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "KeyBinding(bytes32,bytes32,bytes32,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                17u8, 217u8, 70u8, 83u8, 222u8, 82u8, 252u8, 20u8, 56u8, 135u8, 149u8,
                111u8, 228u8, 181u8, 228u8, 18u8, 10u8, 133u8, 11u8, 19u8, 0u8, 236u8,
                224u8, 83u8, 61u8, 238u8, 107u8, 216u8, 255u8, 145u8, 247u8, 50u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    ed25519_sig_0: data.0,
                    ed25519_sig_1: data.1,
                    ed25519_pub_key: data.2,
                    chain_key: data.3,
                    key_id: data.4,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.ed25519_sig_0),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.ed25519_sig_1),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.ed25519_pub_key),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.chain_key,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.key_id),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for KeyBinding {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&KeyBinding> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &KeyBinding) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `KeyBindingFeeUpdate(uint256,uint256)` and selector `0x6925945cb4c6b597a361809d5b51107e4e03720404a82483853e91faa176b88e`.
```solidity
event KeyBindingFeeUpdate(uint256 newFee, uint256 oldFee);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct KeyBindingFeeUpdate {
        #[allow(missing_docs)]
        pub newFee: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub oldFee: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for KeyBindingFeeUpdate {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "KeyBindingFeeUpdate(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                105u8, 37u8, 148u8, 92u8, 180u8, 198u8, 181u8, 151u8, 163u8, 97u8, 128u8,
                157u8, 91u8, 81u8, 16u8, 126u8, 78u8, 3u8, 114u8, 4u8, 4u8, 168u8, 36u8,
                131u8, 133u8, 62u8, 145u8, 250u8, 161u8, 118u8, 184u8, 142u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    newFee: data.0,
                    oldFee: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newFee),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.oldFee),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for KeyBindingFeeUpdate {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&KeyBindingFeeUpdate> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &KeyBindingFeeUpdate) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `LedgerDomainSeparatorUpdated(bytes32)` and selector `0xa43fad83920fd09445855e854e73c9c532e17402c9ceb09993a2392843a5bdb9`.
```solidity
event LedgerDomainSeparatorUpdated(bytes32 indexed ledgerDomainSeparator);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct LedgerDomainSeparatorUpdated {
        #[allow(missing_docs)]
        pub ledgerDomainSeparator: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for LedgerDomainSeparatorUpdated {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "LedgerDomainSeparatorUpdated(bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                164u8, 63u8, 173u8, 131u8, 146u8, 15u8, 208u8, 148u8, 69u8, 133u8, 94u8,
                133u8, 78u8, 115u8, 201u8, 197u8, 50u8, 225u8, 116u8, 2u8, 201u8, 206u8,
                176u8, 153u8, 147u8, 162u8, 57u8, 40u8, 67u8, 165u8, 189u8, 185u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    ledgerDomainSeparator: topics.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.ledgerDomainSeparator.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(
                    &self.ledgerDomainSeparator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for LedgerDomainSeparatorUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&LedgerDomainSeparatorUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &LedgerDomainSeparatorUpdated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `OwnershipTransferred(address,address)` and selector `0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0`.
```solidity
event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OwnershipTransferred {
        #[allow(missing_docs)]
        pub previousOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OwnershipTransferred {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8,
                31u8, 208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8,
                218u8, 175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    previousOwner: topics.1,
                    newOwner: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.previousOwner.clone(),
                    self.newOwner.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.previousOwner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newOwner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnershipTransferred {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnershipTransferred> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OwnershipTransferred) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `RevokeAnnouncement(address)` and selector `0xa4de30a528becadf82649d1395c0e30dd18ae35b5a96ce71e9295bb14bc9f3bc`.
```solidity
event RevokeAnnouncement(address node);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RevokeAnnouncement {
        #[allow(missing_docs)]
        pub node: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RevokeAnnouncement {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "RevokeAnnouncement(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                164u8, 222u8, 48u8, 165u8, 40u8, 190u8, 202u8, 223u8, 130u8, 100u8,
                157u8, 19u8, 149u8, 192u8, 227u8, 13u8, 209u8, 138u8, 227u8, 91u8, 90u8,
                150u8, 206u8, 113u8, 233u8, 41u8, 91u8, 177u8, 75u8, 201u8, 243u8, 188u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { node: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.node,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RevokeAnnouncement {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RevokeAnnouncement> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RevokeAnnouncement) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Upgraded(address)` and selector `0xbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b`.
```solidity
event Upgraded(address indexed implementation);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Upgraded {
        #[allow(missing_docs)]
        pub implementation: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Upgraded {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Upgraded(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                188u8, 124u8, 215u8, 90u8, 32u8, 238u8, 39u8, 253u8, 154u8, 222u8, 186u8,
                179u8, 32u8, 65u8, 247u8, 85u8, 33u8, 77u8, 188u8, 107u8, 255u8, 169u8,
                12u8, 192u8, 34u8, 91u8, 57u8, 218u8, 46u8, 92u8, 45u8, 59u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { implementation: topics.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.implementation.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.implementation,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Upgraded {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Upgraded> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Upgraded) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {}
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZE()` and selector `0xe9d3ee95`.
```solidity
function ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZE() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZECall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZE()`](ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZEReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZECall>
            for UnderlyingRustTuple<'_> {
                fn from(value: ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZEReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZE()";
            const SELECTOR: [u8; 4] = [233u8, 211u8, 238u8, 149u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZEReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZEReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `LEDGER_VERSION()` and selector `0xddad1902`.
```solidity
function LEDGER_VERSION() external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LEDGER_VERSIONCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`LEDGER_VERSION()`](LEDGER_VERSIONCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LEDGER_VERSIONReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<LEDGER_VERSIONCall> for UnderlyingRustTuple<'_> {
                fn from(value: LEDGER_VERSIONCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for LEDGER_VERSIONCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<LEDGER_VERSIONReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: LEDGER_VERSIONReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for LEDGER_VERSIONReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for LEDGER_VERSIONCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::String;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LEDGER_VERSION()";
            const SELECTOR: [u8; 4] = [221u8, 173u8, 25u8, 2u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: LEDGER_VERSIONReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: LEDGER_VERSIONReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `SNAPSHOT_INTERVAL()` and selector `0x6d2beef1`.
```solidity
function SNAPSHOT_INTERVAL() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SNAPSHOT_INTERVALCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`SNAPSHOT_INTERVAL()`](SNAPSHOT_INTERVALCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SNAPSHOT_INTERVALReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<SNAPSHOT_INTERVALCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: SNAPSHOT_INTERVALCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for SNAPSHOT_INTERVALCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<SNAPSHOT_INTERVALReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: SNAPSHOT_INTERVALReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for SNAPSHOT_INTERVALReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for SNAPSHOT_INTERVALCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SNAPSHOT_INTERVAL()";
            const SELECTOR: [u8; 4] = [109u8, 43u8, 238u8, 241u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: SNAPSHOT_INTERVALReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: SNAPSHOT_INTERVALReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `TOKEN()` and selector `0x82bfefc8`.
```solidity
function TOKEN() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TOKENCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`TOKEN()`](TOKENCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TOKENReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<TOKENCall> for UnderlyingRustTuple<'_> {
                fn from(value: TOKENCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for TOKENCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<TOKENReturn> for UnderlyingRustTuple<'_> {
                fn from(value: TOKENReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for TOKENReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for TOKENCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TOKEN()";
            const SELECTOR: [u8; 4] = [130u8, 191u8, 239u8, 200u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: TOKENReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: TOKENReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `TOKENS_RECIPIENT_INTERFACE_HASH()` and selector `0x72581cc0`.
```solidity
function TOKENS_RECIPIENT_INTERFACE_HASH() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TOKENS_RECIPIENT_INTERFACE_HASHCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`TOKENS_RECIPIENT_INTERFACE_HASH()`](TOKENS_RECIPIENT_INTERFACE_HASHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TOKENS_RECIPIENT_INTERFACE_HASHReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<TOKENS_RECIPIENT_INTERFACE_HASHCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: TOKENS_RECIPIENT_INTERFACE_HASHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for TOKENS_RECIPIENT_INTERFACE_HASHCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<TOKENS_RECIPIENT_INTERFACE_HASHReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: TOKENS_RECIPIENT_INTERFACE_HASHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for TOKENS_RECIPIENT_INTERFACE_HASHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for TOKENS_RECIPIENT_INTERFACE_HASHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TOKENS_RECIPIENT_INTERFACE_HASH()";
            const SELECTOR: [u8; 4] = [114u8, 88u8, 28u8, 192u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: TOKENS_RECIPIENT_INTERFACE_HASHReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: TOKENS_RECIPIENT_INTERFACE_HASHReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `UPGRADE_INTERFACE_VERSION()` and selector `0xad3cb1cc`.
```solidity
function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UPGRADE_INTERFACE_VERSIONCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`UPGRADE_INTERFACE_VERSION()`](UPGRADE_INTERFACE_VERSIONCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UPGRADE_INTERFACE_VERSIONReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UPGRADE_INTERFACE_VERSIONCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: UPGRADE_INTERFACE_VERSIONCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for UPGRADE_INTERFACE_VERSIONCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UPGRADE_INTERFACE_VERSIONReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: UPGRADE_INTERFACE_VERSIONReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for UPGRADE_INTERFACE_VERSIONReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for UPGRADE_INTERFACE_VERSIONCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::String;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UPGRADE_INTERFACE_VERSION()";
            const SELECTOR: [u8; 4] = [173u8, 60u8, 177u8, 204u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: UPGRADE_INTERFACE_VERSIONReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: UPGRADE_INTERFACE_VERSIONReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `announce(string)` and selector `0xea0a5237`.
```solidity
function announce(string memory baseMultiaddr) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct announceCall {
        #[allow(missing_docs)]
        pub baseMultiaddr: alloy::sol_types::private::String,
    }
    ///Container type for the return parameters of the [`announce(string)`](announceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct announceReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<announceCall> for UnderlyingRustTuple<'_> {
                fn from(value: announceCall) -> Self {
                    (value.baseMultiaddr,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for announceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { baseMultiaddr: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<announceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: announceReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for announceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl announceReturn {
            fn _tokenize(
                &self,
            ) -> <announceCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for announceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::String,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = announceReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "announce(string)";
            const SELECTOR: [u8; 4] = [234u8, 10u8, 82u8, 55u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.baseMultiaddr,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                announceReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `announceSafe(address,string)` and selector `0xfad0e5a2`.
```solidity
function announceSafe(address selfAddress, string memory baseMultiaddr) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct announceSafeCall {
        #[allow(missing_docs)]
        pub selfAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub baseMultiaddr: alloy::sol_types::private::String,
    }
    ///Container type for the return parameters of the [`announceSafe(address,string)`](announceSafeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct announceSafeReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::String,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::String,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<announceSafeCall> for UnderlyingRustTuple<'_> {
                fn from(value: announceSafeCall) -> Self {
                    (value.selfAddress, value.baseMultiaddr)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for announceSafeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        selfAddress: tuple.0,
                        baseMultiaddr: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<announceSafeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: announceSafeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for announceSafeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl announceSafeReturn {
            fn _tokenize(
                &self,
            ) -> <announceSafeCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for announceSafeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::String,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = announceSafeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "announceSafe(address,string)";
            const SELECTOR: [u8; 4] = [250u8, 208u8, 229u8, 162u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.selfAddress,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.baseMultiaddr,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                announceSafeReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getAllKeyBindings()` and selector `0x10ab5297`.
```solidity
function getAllKeyBindings() external view returns (KeyBindingWithSignatureTimestamp[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllKeyBindingsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getAllKeyBindings()`](getAllKeyBindingsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllKeyBindingsReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
            <KeyBindingWithSignatureTimestamp as alloy::sol_types::SolType>::RustType,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getAllKeyBindingsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAllKeyBindingsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllKeyBindingsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<KeyBindingWithSignatureTimestamp>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <KeyBindingWithSignatureTimestamp as alloy::sol_types::SolType>::RustType,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getAllKeyBindingsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAllKeyBindingsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllKeyBindingsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAllKeyBindingsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                <KeyBindingWithSignatureTimestamp as alloy::sol_types::SolType>::RustType,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<KeyBindingWithSignatureTimestamp>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getAllKeyBindings()";
            const SELECTOR: [u8; 4] = [16u8, 171u8, 82u8, 151u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        KeyBindingWithSignatureTimestamp,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getAllKeyBindingsReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getAllKeyBindingsReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getKeyBindingCount()` and selector `0xf83e4292`.
```solidity
function getKeyBindingCount() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKeyBindingCountCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getKeyBindingCount()`](getKeyBindingCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKeyBindingCountReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getKeyBindingCountCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKeyBindingCountCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKeyBindingCountCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getKeyBindingCountReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKeyBindingCountReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKeyBindingCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getKeyBindingCountCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getKeyBindingCount()";
            const SELECTOR: [u8; 4] = [248u8, 62u8, 66u8, 146u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getKeyBindingCountReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getKeyBindingCountReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getKeyBindingWithKeyId(uint32)` and selector `0xa8b4eec9`.
```solidity
function getKeyBindingWithKeyId(KeyId keyId) external view returns (KeyBindingWithSignatureTimestamp memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKeyBindingWithKeyIdCall {
        #[allow(missing_docs)]
        pub keyId: <KeyId as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getKeyBindingWithKeyId(uint32)`](getKeyBindingWithKeyIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKeyBindingWithKeyIdReturn {
        #[allow(missing_docs)]
        pub _0: <KeyBindingWithSignatureTimestamp as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (KeyId,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <KeyId as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getKeyBindingWithKeyIdCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKeyBindingWithKeyIdCall) -> Self {
                    (value.keyId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKeyBindingWithKeyIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { keyId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (KeyBindingWithSignatureTimestamp,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <KeyBindingWithSignatureTimestamp as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getKeyBindingWithKeyIdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKeyBindingWithKeyIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKeyBindingWithKeyIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getKeyBindingWithKeyIdCall {
            type Parameters<'a> = (KeyId,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <KeyBindingWithSignatureTimestamp as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (KeyBindingWithSignatureTimestamp,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getKeyBindingWithKeyId(uint32)";
            const SELECTOR: [u8; 4] = [168u8, 180u8, 238u8, 201u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<KeyId as alloy_sol_types::SolType>::tokenize(&self.keyId),)
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <KeyBindingWithSignatureTimestamp as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getKeyBindingWithKeyIdReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getKeyBindingWithKeyIdReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getKeyIdRange()` and selector `0x773b4a33`.
```solidity
function getKeyIdRange() external pure returns (uint32 minKeyId, uint32 maxKeyId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKeyIdRangeCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getKeyIdRange()`](getKeyIdRangeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKeyIdRangeReturn {
        #[allow(missing_docs)]
        pub minKeyId: u32,
        #[allow(missing_docs)]
        pub maxKeyId: u32,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getKeyIdRangeCall> for UnderlyingRustTuple<'_> {
                fn from(value: getKeyIdRangeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getKeyIdRangeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32, u32);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getKeyIdRangeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getKeyIdRangeReturn) -> Self {
                    (value.minKeyId, value.maxKeyId)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getKeyIdRangeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        minKeyId: tuple.0,
                        maxKeyId: tuple.1,
                    }
                }
            }
        }
        impl getKeyIdRangeReturn {
            fn _tokenize(
                &self,
            ) -> <getKeyIdRangeCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.minKeyId),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxKeyId),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getKeyIdRangeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getKeyIdRangeReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getKeyIdRange()";
            const SELECTOR: [u8; 4] = [119u8, 59u8, 74u8, 51u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                getKeyIdRangeReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getKeyIdWithOffchainKey(bytes32)` and selector `0x604634c9`.
```solidity
function getKeyIdWithOffchainKey(bytes32 ed25519_pub_key) external view returns (bool, KeyId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKeyIdWithOffchainKeyCall {
        #[allow(missing_docs)]
        pub ed25519_pub_key: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getKeyIdWithOffchainKey(bytes32)`](getKeyIdWithOffchainKeyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKeyIdWithOffchainKeyReturn {
        #[allow(missing_docs)]
        pub _0: bool,
        #[allow(missing_docs)]
        pub _1: <KeyId as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getKeyIdWithOffchainKeyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKeyIdWithOffchainKeyCall) -> Self {
                    (value.ed25519_pub_key,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKeyIdWithOffchainKeyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { ed25519_pub_key: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool, KeyId);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                bool,
                <KeyId as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getKeyIdWithOffchainKeyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKeyIdWithOffchainKeyReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKeyIdWithOffchainKeyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        impl getKeyIdWithOffchainKeyReturn {
            fn _tokenize(
                &self,
            ) -> <getKeyIdWithOffchainKeyCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                    <KeyId as alloy_sol_types::SolType>::tokenize(&self._1),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getKeyIdWithOffchainKeyCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getKeyIdWithOffchainKeyReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool, KeyId);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getKeyIdWithOffchainKey(bytes32)";
            const SELECTOR: [u8; 4] = [96u8, 70u8, 52u8, 201u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.ed25519_pub_key),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                getKeyIdWithOffchainKeyReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getOffchainKeyWithKeyId(uint32)` and selector `0x4ac3e4f2`.
```solidity
function getOffchainKeyWithKeyId(KeyId keyId) external view returns (bytes32 ed25519_pub_key);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOffchainKeyWithKeyIdCall {
        #[allow(missing_docs)]
        pub keyId: <KeyId as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getOffchainKeyWithKeyId(uint32)`](getOffchainKeyWithKeyIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOffchainKeyWithKeyIdReturn {
        #[allow(missing_docs)]
        pub ed25519_pub_key: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (KeyId,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <KeyId as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOffchainKeyWithKeyIdCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOffchainKeyWithKeyIdCall) -> Self {
                    (value.keyId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOffchainKeyWithKeyIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { keyId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOffchainKeyWithKeyIdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOffchainKeyWithKeyIdReturn) -> Self {
                    (value.ed25519_pub_key,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOffchainKeyWithKeyIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { ed25519_pub_key: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOffchainKeyWithKeyIdCall {
            type Parameters<'a> = (KeyId,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOffchainKeyWithKeyId(uint32)";
            const SELECTOR: [u8; 4] = [74u8, 195u8, 228u8, 242u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<KeyId as alloy_sol_types::SolType>::tokenize(&self.keyId),)
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getOffchainKeyWithKeyIdReturn = r.into();
                        r.ed25519_pub_key
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getOffchainKeyWithKeyIdReturn = r.into();
                        r.ed25519_pub_key
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `initialize(bytes)` and selector `0x439fab91`.
```solidity
function initialize(bytes memory initParams) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        #[allow(missing_docs)]
        pub initParams: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`initialize(bytes)`](initializeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (value.initParams,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { initParams: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initializeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl initializeReturn {
            fn _tokenize(
                &self,
            ) -> <initializeCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(bytes)";
            const SELECTOR: [u8; 4] = [67u8, 159u8, 171u8, 145u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.initParams,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                initializeReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isOffchainKeyBound(bytes32)` and selector `0xa969635a`.
```solidity
function isOffchainKeyBound(bytes32 ed25519_pub_key) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOffchainKeyBoundCall {
        #[allow(missing_docs)]
        pub ed25519_pub_key: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isOffchainKeyBound(bytes32)`](isOffchainKeyBoundCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOffchainKeyBoundReturn {
        #[allow(missing_docs)]
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isOffchainKeyBoundCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isOffchainKeyBoundCall) -> Self {
                    (value.ed25519_pub_key,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isOffchainKeyBoundCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { ed25519_pub_key: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isOffchainKeyBoundReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isOffchainKeyBoundReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isOffchainKeyBoundReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isOffchainKeyBoundCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isOffchainKeyBound(bytes32)";
            const SELECTOR: [u8; 4] = [169u8, 105u8, 99u8, 90u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.ed25519_pub_key),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: isOffchainKeyBoundReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: isOffchainKeyBoundReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `keyBindingFee()` and selector `0xfd153acf`.
```solidity
function keyBindingFee() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct keyBindingFeeCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`keyBindingFee()`](keyBindingFeeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct keyBindingFeeReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<keyBindingFeeCall> for UnderlyingRustTuple<'_> {
                fn from(value: keyBindingFeeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for keyBindingFeeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<keyBindingFeeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: keyBindingFeeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for keyBindingFeeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for keyBindingFeeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "keyBindingFee()";
            const SELECTOR: [u8; 4] = [253u8, 21u8, 58u8, 207u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: keyBindingFeeReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: keyBindingFeeReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `latestRoot()` and selector `0xd7b0fef1`.
```solidity
function latestRoot() external view returns (HoprLedger.RootStruct memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct latestRootCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`latestRoot()`](latestRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct latestRootReturn {
        #[allow(missing_docs)]
        pub _0: <HoprLedger::RootStruct as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<latestRootCall> for UnderlyingRustTuple<'_> {
                fn from(value: latestRootCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for latestRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (HoprLedger::RootStruct,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <HoprLedger::RootStruct as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<latestRootReturn> for UnderlyingRustTuple<'_> {
                fn from(value: latestRootReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for latestRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for latestRootCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <HoprLedger::RootStruct as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (HoprLedger::RootStruct,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "latestRoot()";
            const SELECTOR: [u8; 4] = [215u8, 176u8, 254u8, 241u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<HoprLedger::RootStruct as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: latestRootReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: latestRootReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `latestSnapshotRoot()` and selector `0x0df18f94`.
```solidity
function latestSnapshotRoot() external view returns (HoprLedger.RootStruct memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct latestSnapshotRootCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`latestSnapshotRoot()`](latestSnapshotRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct latestSnapshotRootReturn {
        #[allow(missing_docs)]
        pub _0: <HoprLedger::RootStruct as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<latestSnapshotRootCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: latestSnapshotRootCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for latestSnapshotRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (HoprLedger::RootStruct,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <HoprLedger::RootStruct as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<latestSnapshotRootReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: latestSnapshotRootReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for latestSnapshotRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for latestSnapshotRootCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <HoprLedger::RootStruct as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (HoprLedger::RootStruct,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "latestSnapshotRoot()";
            const SELECTOR: [u8; 4] = [13u8, 241u8, 143u8, 148u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<HoprLedger::RootStruct as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: latestSnapshotRootReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: latestSnapshotRootReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `ledgerDomainSeparator()` and selector `0xc966c4fe`.
```solidity
function ledgerDomainSeparator() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ledgerDomainSeparatorCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`ledgerDomainSeparator()`](ledgerDomainSeparatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ledgerDomainSeparatorReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ledgerDomainSeparatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: ledgerDomainSeparatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ledgerDomainSeparatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ledgerDomainSeparatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: ledgerDomainSeparatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ledgerDomainSeparatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ledgerDomainSeparatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ledgerDomainSeparator()";
            const SELECTOR: [u8; 4] = [201u8, 102u8, 196u8, 254u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: ledgerDomainSeparatorReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: ledgerDomainSeparatorReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `multiaddrOf(address)` and selector `0x53665aaa`.
```solidity
function multiaddrOf(address) external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct multiaddrOfCall(pub alloy::sol_types::private::Address);
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`multiaddrOf(address)`](multiaddrOfCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct multiaddrOfReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<multiaddrOfCall> for UnderlyingRustTuple<'_> {
                fn from(value: multiaddrOfCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for multiaddrOfCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<multiaddrOfReturn> for UnderlyingRustTuple<'_> {
                fn from(value: multiaddrOfReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for multiaddrOfReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for multiaddrOfCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::String;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "multiaddrOf(address)";
            const SELECTOR: [u8; 4] = [83u8, 102u8, 90u8, 170u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.0,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: multiaddrOfReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: multiaddrOfReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
```solidity
function owner() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`owner()`](ownerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerCall> for UnderlyingRustTuple<'_> {
                fn from(value: ownerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ownerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ownerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "owner()";
            const SELECTOR: [u8; 4] = [141u8, 165u8, 203u8, 91u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: ownerReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: ownerReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `proxiableUUID()` and selector `0x52d1902d`.
```solidity
function proxiableUUID() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proxiableUUIDCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`proxiableUUID()`](proxiableUUIDCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proxiableUUIDReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proxiableUUIDCall> for UnderlyingRustTuple<'_> {
                fn from(value: proxiableUUIDCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proxiableUUIDCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proxiableUUIDReturn> for UnderlyingRustTuple<'_> {
                fn from(value: proxiableUUIDReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proxiableUUIDReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proxiableUUIDCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proxiableUUID()";
            const SELECTOR: [u8; 4] = [82u8, 209u8, 144u8, 45u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: proxiableUUIDReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: proxiableUUIDReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `renounceOwnership()` and selector `0x715018a6`.
```solidity
function renounceOwnership() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipCall;
    ///Container type for the return parameters of the [`renounceOwnership()`](renounceOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl renounceOwnershipReturn {
            fn _tokenize(
                &self,
            ) -> <renounceOwnershipCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "renounceOwnership()";
            const SELECTOR: [u8; 4] = [113u8, 80u8, 24u8, 166u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                renounceOwnershipReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `revoke()` and selector `0xb6549f75`.
```solidity
function revoke() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct revokeCall;
    ///Container type for the return parameters of the [`revoke()`](revokeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct revokeReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<revokeCall> for UnderlyingRustTuple<'_> {
                fn from(value: revokeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for revokeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<revokeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: revokeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for revokeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl revokeReturn {
            fn _tokenize(
                &self,
            ) -> <revokeCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for revokeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = revokeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "revoke()";
            const SELECTOR: [u8; 4] = [182u8, 84u8, 159u8, 117u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                revokeReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `revokeSafe(address)` and selector `0x308c712e`.
```solidity
function revokeSafe(address selfAddress) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct revokeSafeCall {
        #[allow(missing_docs)]
        pub selfAddress: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`revokeSafe(address)`](revokeSafeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct revokeSafeReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<revokeSafeCall> for UnderlyingRustTuple<'_> {
                fn from(value: revokeSafeCall) -> Self {
                    (value.selfAddress,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for revokeSafeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { selfAddress: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<revokeSafeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: revokeSafeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for revokeSafeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl revokeSafeReturn {
            fn _tokenize(
                &self,
            ) -> <revokeSafeCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for revokeSafeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = revokeSafeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "revokeSafe(address)";
            const SELECTOR: [u8; 4] = [48u8, 140u8, 113u8, 46u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.selfAddress,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                revokeSafeReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `tokensReceived(address,address,address,uint256,bytes,bytes)` and selector `0x0023de29`.
```solidity
function tokensReceived(address, address from, address to, uint256 amount, bytes memory userData, bytes memory) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tokensReceivedCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub userData: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub _5: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`tokensReceived(address,address,address,uint256,bytes,bytes)`](tokensReceivedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tokensReceivedReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<tokensReceivedCall> for UnderlyingRustTuple<'_> {
                fn from(value: tokensReceivedCall) -> Self {
                    (
                        value._0,
                        value.from,
                        value.to,
                        value.amount,
                        value.userData,
                        value._5,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for tokensReceivedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        from: tuple.1,
                        to: tuple.2,
                        amount: tuple.3,
                        userData: tuple.4,
                        _5: tuple.5,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<tokensReceivedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: tokensReceivedReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for tokensReceivedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl tokensReceivedReturn {
            fn _tokenize(
                &self,
            ) -> <tokensReceivedCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for tokensReceivedCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = tokensReceivedReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "tokensReceived(address,address,address,uint256,bytes,bytes)";
            const SELECTOR: [u8; 4] = [0u8, 35u8, 222u8, 41u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.from,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.userData,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._5,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                tokensReceivedReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `transferOwnership(address)` and selector `0xf2fde38b`.
```solidity
function transferOwnership(address newOwner) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipCall {
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`transferOwnership(address)`](transferOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipCall) -> Self {
                    (value.newOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newOwner: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl transferOwnershipReturn {
            fn _tokenize(
                &self,
            ) -> <transferOwnershipCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferOwnership(address)";
            const SELECTOR: [u8; 4] = [242u8, 253u8, 227u8, 139u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newOwner,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                transferOwnershipReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `tryGetKeyBinding(bytes32)` and selector `0xa2a07756`.
```solidity
function tryGetKeyBinding(bytes32 ed25519_pub_key) external view returns (bool, KeyId, KeyBindingWithSignatureTimestamp memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tryGetKeyBindingCall {
        #[allow(missing_docs)]
        pub ed25519_pub_key: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`tryGetKeyBinding(bytes32)`](tryGetKeyBindingCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tryGetKeyBindingReturn {
        #[allow(missing_docs)]
        pub _0: bool,
        #[allow(missing_docs)]
        pub _1: <KeyId as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _2: <KeyBindingWithSignatureTimestamp as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<tryGetKeyBindingCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: tryGetKeyBindingCall) -> Self {
                    (value.ed25519_pub_key,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for tryGetKeyBindingCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { ed25519_pub_key: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                KeyId,
                KeyBindingWithSignatureTimestamp,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                bool,
                <KeyId as alloy::sol_types::SolType>::RustType,
                <KeyBindingWithSignatureTimestamp as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<tryGetKeyBindingReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: tryGetKeyBindingReturn) -> Self {
                    (value._0, value._1, value._2)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for tryGetKeyBindingReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                        _2: tuple.2,
                    }
                }
            }
        }
        impl tryGetKeyBindingReturn {
            fn _tokenize(
                &self,
            ) -> <tryGetKeyBindingCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                    <KeyId as alloy_sol_types::SolType>::tokenize(&self._1),
                    <KeyBindingWithSignatureTimestamp as alloy_sol_types::SolType>::tokenize(
                        &self._2,
                    ),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for tryGetKeyBindingCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = tryGetKeyBindingReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                KeyId,
                KeyBindingWithSignatureTimestamp,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "tryGetKeyBinding(bytes32)";
            const SELECTOR: [u8; 4] = [162u8, 160u8, 119u8, 86u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.ed25519_pub_key),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                tryGetKeyBindingReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `updateKeyBindingFee(uint256)` and selector `0x244d496e`.
```solidity
function updateKeyBindingFee(uint256 newFee) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateKeyBindingFeeCall {
        #[allow(missing_docs)]
        pub newFee: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`updateKeyBindingFee(uint256)`](updateKeyBindingFeeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateKeyBindingFeeReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateKeyBindingFeeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateKeyBindingFeeCall) -> Self {
                    (value.newFee,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateKeyBindingFeeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newFee: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateKeyBindingFeeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateKeyBindingFeeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateKeyBindingFeeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl updateKeyBindingFeeReturn {
            fn _tokenize(
                &self,
            ) -> <updateKeyBindingFeeCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateKeyBindingFeeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateKeyBindingFeeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateKeyBindingFee(uint256)";
            const SELECTOR: [u8; 4] = [36u8, 77u8, 73u8, 110u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newFee),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                updateKeyBindingFeeReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `updateLedgerDomainSeparator()` and selector `0xdc96fd50`.
```solidity
function updateLedgerDomainSeparator() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateLedgerDomainSeparatorCall;
    ///Container type for the return parameters of the [`updateLedgerDomainSeparator()`](updateLedgerDomainSeparatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateLedgerDomainSeparatorReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateLedgerDomainSeparatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateLedgerDomainSeparatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateLedgerDomainSeparatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateLedgerDomainSeparatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateLedgerDomainSeparatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateLedgerDomainSeparatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl updateLedgerDomainSeparatorReturn {
            fn _tokenize(
                &self,
            ) -> <updateLedgerDomainSeparatorCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateLedgerDomainSeparatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateLedgerDomainSeparatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateLedgerDomainSeparator()";
            const SELECTOR: [u8; 4] = [220u8, 150u8, 253u8, 80u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                updateLedgerDomainSeparatorReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `upgradeToAndCall(address,bytes)` and selector `0x4f1ef286`.
```solidity
function upgradeToAndCall(address newImplementation, bytes memory data) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct upgradeToAndCallCall {
        #[allow(missing_docs)]
        pub newImplementation: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`upgradeToAndCall(address,bytes)`](upgradeToAndCallCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct upgradeToAndCallReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<upgradeToAndCallCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: upgradeToAndCallCall) -> Self {
                    (value.newImplementation, value.data)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for upgradeToAndCallCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newImplementation: tuple.0,
                        data: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<upgradeToAndCallReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: upgradeToAndCallReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for upgradeToAndCallReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl upgradeToAndCallReturn {
            fn _tokenize(
                &self,
            ) -> <upgradeToAndCallCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for upgradeToAndCallCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = upgradeToAndCallReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "upgradeToAndCall(address,bytes)";
            const SELECTOR: [u8; 4] = [79u8, 30u8, 242u8, 134u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newImplementation,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                upgradeToAndCallReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`HoprAnnouncements`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum HoprAnnouncementsCalls {
        #[allow(missing_docs)]
        ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZE(
            ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZECall,
        ),
        #[allow(missing_docs)]
        LEDGER_VERSION(LEDGER_VERSIONCall),
        #[allow(missing_docs)]
        SNAPSHOT_INTERVAL(SNAPSHOT_INTERVALCall),
        #[allow(missing_docs)]
        TOKEN(TOKENCall),
        #[allow(missing_docs)]
        TOKENS_RECIPIENT_INTERFACE_HASH(TOKENS_RECIPIENT_INTERFACE_HASHCall),
        #[allow(missing_docs)]
        UPGRADE_INTERFACE_VERSION(UPGRADE_INTERFACE_VERSIONCall),
        #[allow(missing_docs)]
        announce(announceCall),
        #[allow(missing_docs)]
        announceSafe(announceSafeCall),
        #[allow(missing_docs)]
        getAllKeyBindings(getAllKeyBindingsCall),
        #[allow(missing_docs)]
        getKeyBindingCount(getKeyBindingCountCall),
        #[allow(missing_docs)]
        getKeyBindingWithKeyId(getKeyBindingWithKeyIdCall),
        #[allow(missing_docs)]
        getKeyIdRange(getKeyIdRangeCall),
        #[allow(missing_docs)]
        getKeyIdWithOffchainKey(getKeyIdWithOffchainKeyCall),
        #[allow(missing_docs)]
        getOffchainKeyWithKeyId(getOffchainKeyWithKeyIdCall),
        #[allow(missing_docs)]
        initialize(initializeCall),
        #[allow(missing_docs)]
        isOffchainKeyBound(isOffchainKeyBoundCall),
        #[allow(missing_docs)]
        keyBindingFee(keyBindingFeeCall),
        #[allow(missing_docs)]
        latestRoot(latestRootCall),
        #[allow(missing_docs)]
        latestSnapshotRoot(latestSnapshotRootCall),
        #[allow(missing_docs)]
        ledgerDomainSeparator(ledgerDomainSeparatorCall),
        #[allow(missing_docs)]
        multiaddrOf(multiaddrOfCall),
        #[allow(missing_docs)]
        owner(ownerCall),
        #[allow(missing_docs)]
        proxiableUUID(proxiableUUIDCall),
        #[allow(missing_docs)]
        renounceOwnership(renounceOwnershipCall),
        #[allow(missing_docs)]
        revoke(revokeCall),
        #[allow(missing_docs)]
        revokeSafe(revokeSafeCall),
        #[allow(missing_docs)]
        tokensReceived(tokensReceivedCall),
        #[allow(missing_docs)]
        transferOwnership(transferOwnershipCall),
        #[allow(missing_docs)]
        tryGetKeyBinding(tryGetKeyBindingCall),
        #[allow(missing_docs)]
        updateKeyBindingFee(updateKeyBindingFeeCall),
        #[allow(missing_docs)]
        updateLedgerDomainSeparator(updateLedgerDomainSeparatorCall),
        #[allow(missing_docs)]
        upgradeToAndCall(upgradeToAndCallCall),
    }
    impl HoprAnnouncementsCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [0u8, 35u8, 222u8, 41u8],
            [13u8, 241u8, 143u8, 148u8],
            [16u8, 171u8, 82u8, 151u8],
            [36u8, 77u8, 73u8, 110u8],
            [48u8, 140u8, 113u8, 46u8],
            [67u8, 159u8, 171u8, 145u8],
            [74u8, 195u8, 228u8, 242u8],
            [79u8, 30u8, 242u8, 134u8],
            [82u8, 209u8, 144u8, 45u8],
            [83u8, 102u8, 90u8, 170u8],
            [96u8, 70u8, 52u8, 201u8],
            [109u8, 43u8, 238u8, 241u8],
            [113u8, 80u8, 24u8, 166u8],
            [114u8, 88u8, 28u8, 192u8],
            [119u8, 59u8, 74u8, 51u8],
            [130u8, 191u8, 239u8, 200u8],
            [141u8, 165u8, 203u8, 91u8],
            [162u8, 160u8, 119u8, 86u8],
            [168u8, 180u8, 238u8, 201u8],
            [169u8, 105u8, 99u8, 90u8],
            [173u8, 60u8, 177u8, 204u8],
            [182u8, 84u8, 159u8, 117u8],
            [201u8, 102u8, 196u8, 254u8],
            [215u8, 176u8, 254u8, 241u8],
            [220u8, 150u8, 253u8, 80u8],
            [221u8, 173u8, 25u8, 2u8],
            [233u8, 211u8, 238u8, 149u8],
            [234u8, 10u8, 82u8, 55u8],
            [242u8, 253u8, 227u8, 139u8],
            [248u8, 62u8, 66u8, 146u8],
            [250u8, 208u8, 229u8, 162u8],
            [253u8, 21u8, 58u8, 207u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(tokensReceived),
            ::core::stringify!(latestSnapshotRoot),
            ::core::stringify!(getAllKeyBindings),
            ::core::stringify!(updateKeyBindingFee),
            ::core::stringify!(revokeSafe),
            ::core::stringify!(initialize),
            ::core::stringify!(getOffchainKeyWithKeyId),
            ::core::stringify!(upgradeToAndCall),
            ::core::stringify!(proxiableUUID),
            ::core::stringify!(multiaddrOf),
            ::core::stringify!(getKeyIdWithOffchainKey),
            ::core::stringify!(SNAPSHOT_INTERVAL),
            ::core::stringify!(renounceOwnership),
            ::core::stringify!(TOKENS_RECIPIENT_INTERFACE_HASH),
            ::core::stringify!(getKeyIdRange),
            ::core::stringify!(TOKEN),
            ::core::stringify!(owner),
            ::core::stringify!(tryGetKeyBinding),
            ::core::stringify!(getKeyBindingWithKeyId),
            ::core::stringify!(isOffchainKeyBound),
            ::core::stringify!(UPGRADE_INTERFACE_VERSION),
            ::core::stringify!(revoke),
            ::core::stringify!(ledgerDomainSeparator),
            ::core::stringify!(latestRoot),
            ::core::stringify!(updateLedgerDomainSeparator),
            ::core::stringify!(LEDGER_VERSION),
            ::core::stringify!(ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZE),
            ::core::stringify!(announce),
            ::core::stringify!(transferOwnership),
            ::core::stringify!(getKeyBindingCount),
            ::core::stringify!(announceSafe),
            ::core::stringify!(keyBindingFee),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <tokensReceivedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <latestSnapshotRootCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getAllKeyBindingsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <updateKeyBindingFeeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <revokeSafeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <initializeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getOffchainKeyWithKeyIdCall as alloy_sol_types::SolCall>::SIGNATURE,
            <upgradeToAndCallCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proxiableUUIDCall as alloy_sol_types::SolCall>::SIGNATURE,
            <multiaddrOfCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getKeyIdWithOffchainKeyCall as alloy_sol_types::SolCall>::SIGNATURE,
            <SNAPSHOT_INTERVALCall as alloy_sol_types::SolCall>::SIGNATURE,
            <renounceOwnershipCall as alloy_sol_types::SolCall>::SIGNATURE,
            <TOKENS_RECIPIENT_INTERFACE_HASHCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getKeyIdRangeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <TOKENCall as alloy_sol_types::SolCall>::SIGNATURE,
            <ownerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <tryGetKeyBindingCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getKeyBindingWithKeyIdCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isOffchainKeyBoundCall as alloy_sol_types::SolCall>::SIGNATURE,
            <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SIGNATURE,
            <revokeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <ledgerDomainSeparatorCall as alloy_sol_types::SolCall>::SIGNATURE,
            <latestRootCall as alloy_sol_types::SolCall>::SIGNATURE,
            <updateLedgerDomainSeparatorCall as alloy_sol_types::SolCall>::SIGNATURE,
            <LEDGER_VERSIONCall as alloy_sol_types::SolCall>::SIGNATURE,
            <ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZECall as alloy_sol_types::SolCall>::SIGNATURE,
            <announceCall as alloy_sol_types::SolCall>::SIGNATURE,
            <transferOwnershipCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getKeyBindingCountCall as alloy_sol_types::SolCall>::SIGNATURE,
            <announceSafeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <keyBindingFeeCall as alloy_sol_types::SolCall>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for HoprAnnouncementsCalls {
        const NAME: &'static str = "HoprAnnouncementsCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 32usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZE(_) => {
                    <ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::LEDGER_VERSION(_) => {
                    <LEDGER_VERSIONCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::SNAPSHOT_INTERVAL(_) => {
                    <SNAPSHOT_INTERVALCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::TOKEN(_) => <TOKENCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::TOKENS_RECIPIENT_INTERFACE_HASH(_) => {
                    <TOKENS_RECIPIENT_INTERFACE_HASHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::UPGRADE_INTERFACE_VERSION(_) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::announce(_) => <announceCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::announceSafe(_) => {
                    <announceSafeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAllKeyBindings(_) => {
                    <getAllKeyBindingsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getKeyBindingCount(_) => {
                    <getKeyBindingCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getKeyBindingWithKeyId(_) => {
                    <getKeyBindingWithKeyIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getKeyIdRange(_) => {
                    <getKeyIdRangeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getKeyIdWithOffchainKey(_) => {
                    <getKeyIdWithOffchainKeyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOffchainKeyWithKeyId(_) => {
                    <getOffchainKeyWithKeyIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isOffchainKeyBound(_) => {
                    <isOffchainKeyBoundCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::keyBindingFee(_) => {
                    <keyBindingFeeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::latestRoot(_) => {
                    <latestRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::latestSnapshotRoot(_) => {
                    <latestSnapshotRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::ledgerDomainSeparator(_) => {
                    <ledgerDomainSeparatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::multiaddrOf(_) => {
                    <multiaddrOfCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::proxiableUUID(_) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::revoke(_) => <revokeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::revokeSafe(_) => {
                    <revokeSafeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::tokensReceived(_) => {
                    <tokensReceivedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::tryGetKeyBinding(_) => {
                    <tryGetKeyBindingCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateKeyBindingFee(_) => {
                    <updateKeyBindingFeeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateLedgerDomainSeparator(_) => {
                    <updateLedgerDomainSeparatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::upgradeToAndCall(_) => {
                    <upgradeToAndCallCall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<HoprAnnouncementsCalls>] = &[
                {
                    fn tokensReceived(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <tokensReceivedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::tokensReceived)
                    }
                    tokensReceived
                },
                {
                    fn latestSnapshotRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <latestSnapshotRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::latestSnapshotRoot)
                    }
                    latestSnapshotRoot
                },
                {
                    fn getAllKeyBindings(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <getAllKeyBindingsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::getAllKeyBindings)
                    }
                    getAllKeyBindings
                },
                {
                    fn updateKeyBindingFee(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <updateKeyBindingFeeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::updateKeyBindingFee)
                    }
                    updateKeyBindingFee
                },
                {
                    fn revokeSafe(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <revokeSafeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::revokeSafe)
                    }
                    revokeSafe
                },
                {
                    fn initialize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::initialize)
                    }
                    initialize
                },
                {
                    fn getOffchainKeyWithKeyId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <getOffchainKeyWithKeyIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::getOffchainKeyWithKeyId)
                    }
                    getOffchainKeyWithKeyId
                },
                {
                    fn upgradeToAndCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::upgradeToAndCall)
                    }
                    upgradeToAndCall
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn multiaddrOf(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <multiaddrOfCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::multiaddrOf)
                    }
                    multiaddrOf
                },
                {
                    fn getKeyIdWithOffchainKey(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <getKeyIdWithOffchainKeyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::getKeyIdWithOffchainKey)
                    }
                    getKeyIdWithOffchainKey
                },
                {
                    fn SNAPSHOT_INTERVAL(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <SNAPSHOT_INTERVALCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::SNAPSHOT_INTERVAL)
                    }
                    SNAPSHOT_INTERVAL
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn TOKENS_RECIPIENT_INTERFACE_HASH(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <TOKENS_RECIPIENT_INTERFACE_HASHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::TOKENS_RECIPIENT_INTERFACE_HASH)
                    }
                    TOKENS_RECIPIENT_INTERFACE_HASH
                },
                {
                    fn getKeyIdRange(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <getKeyIdRangeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::getKeyIdRange)
                    }
                    getKeyIdRange
                },
                {
                    fn TOKEN(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <TOKENCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(HoprAnnouncementsCalls::TOKEN)
                    }
                    TOKEN
                },
                {
                    fn owner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(HoprAnnouncementsCalls::owner)
                    }
                    owner
                },
                {
                    fn tryGetKeyBinding(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <tryGetKeyBindingCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::tryGetKeyBinding)
                    }
                    tryGetKeyBinding
                },
                {
                    fn getKeyBindingWithKeyId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <getKeyBindingWithKeyIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::getKeyBindingWithKeyId)
                    }
                    getKeyBindingWithKeyId
                },
                {
                    fn isOffchainKeyBound(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <isOffchainKeyBoundCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::isOffchainKeyBound)
                    }
                    isOffchainKeyBound
                },
                {
                    fn UPGRADE_INTERFACE_VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::UPGRADE_INTERFACE_VERSION)
                    }
                    UPGRADE_INTERFACE_VERSION
                },
                {
                    fn revoke(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <revokeCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(HoprAnnouncementsCalls::revoke)
                    }
                    revoke
                },
                {
                    fn ledgerDomainSeparator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <ledgerDomainSeparatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::ledgerDomainSeparator)
                    }
                    ledgerDomainSeparator
                },
                {
                    fn latestRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <latestRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::latestRoot)
                    }
                    latestRoot
                },
                {
                    fn updateLedgerDomainSeparator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <updateLedgerDomainSeparatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::updateLedgerDomainSeparator)
                    }
                    updateLedgerDomainSeparator
                },
                {
                    fn LEDGER_VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <LEDGER_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::LEDGER_VERSION)
                    }
                    LEDGER_VERSION
                },
                {
                    fn ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                HoprAnnouncementsCalls::ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZE,
                            )
                    }
                    ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZE
                },
                {
                    fn announce(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <announceCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(HoprAnnouncementsCalls::announce)
                    }
                    announce
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn getKeyBindingCount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <getKeyBindingCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::getKeyBindingCount)
                    }
                    getKeyBindingCount
                },
                {
                    fn announceSafe(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <announceSafeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::announceSafe)
                    }
                    announceSafe
                },
                {
                    fn keyBindingFee(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <keyBindingFeeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::keyBindingFee)
                    }
                    keyBindingFee
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<HoprAnnouncementsCalls>] = &[
                {
                    fn tokensReceived(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <tokensReceivedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::tokensReceived)
                    }
                    tokensReceived
                },
                {
                    fn latestSnapshotRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <latestSnapshotRootCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::latestSnapshotRoot)
                    }
                    latestSnapshotRoot
                },
                {
                    fn getAllKeyBindings(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <getAllKeyBindingsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::getAllKeyBindings)
                    }
                    getAllKeyBindings
                },
                {
                    fn updateKeyBindingFee(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <updateKeyBindingFeeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::updateKeyBindingFee)
                    }
                    updateKeyBindingFee
                },
                {
                    fn revokeSafe(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <revokeSafeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::revokeSafe)
                    }
                    revokeSafe
                },
                {
                    fn initialize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::initialize)
                    }
                    initialize
                },
                {
                    fn getOffchainKeyWithKeyId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <getOffchainKeyWithKeyIdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::getOffchainKeyWithKeyId)
                    }
                    getOffchainKeyWithKeyId
                },
                {
                    fn upgradeToAndCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::upgradeToAndCall)
                    }
                    upgradeToAndCall
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn multiaddrOf(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <multiaddrOfCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::multiaddrOf)
                    }
                    multiaddrOf
                },
                {
                    fn getKeyIdWithOffchainKey(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <getKeyIdWithOffchainKeyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::getKeyIdWithOffchainKey)
                    }
                    getKeyIdWithOffchainKey
                },
                {
                    fn SNAPSHOT_INTERVAL(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <SNAPSHOT_INTERVALCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::SNAPSHOT_INTERVAL)
                    }
                    SNAPSHOT_INTERVAL
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn TOKENS_RECIPIENT_INTERFACE_HASH(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <TOKENS_RECIPIENT_INTERFACE_HASHCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::TOKENS_RECIPIENT_INTERFACE_HASH)
                    }
                    TOKENS_RECIPIENT_INTERFACE_HASH
                },
                {
                    fn getKeyIdRange(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <getKeyIdRangeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::getKeyIdRange)
                    }
                    getKeyIdRange
                },
                {
                    fn TOKEN(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <TOKENCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::TOKEN)
                    }
                    TOKEN
                },
                {
                    fn owner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::owner)
                    }
                    owner
                },
                {
                    fn tryGetKeyBinding(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <tryGetKeyBindingCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::tryGetKeyBinding)
                    }
                    tryGetKeyBinding
                },
                {
                    fn getKeyBindingWithKeyId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <getKeyBindingWithKeyIdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::getKeyBindingWithKeyId)
                    }
                    getKeyBindingWithKeyId
                },
                {
                    fn isOffchainKeyBound(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <isOffchainKeyBoundCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::isOffchainKeyBound)
                    }
                    isOffchainKeyBound
                },
                {
                    fn UPGRADE_INTERFACE_VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::UPGRADE_INTERFACE_VERSION)
                    }
                    UPGRADE_INTERFACE_VERSION
                },
                {
                    fn revoke(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <revokeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::revoke)
                    }
                    revoke
                },
                {
                    fn ledgerDomainSeparator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <ledgerDomainSeparatorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::ledgerDomainSeparator)
                    }
                    ledgerDomainSeparator
                },
                {
                    fn latestRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <latestRootCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::latestRoot)
                    }
                    latestRoot
                },
                {
                    fn updateLedgerDomainSeparator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <updateLedgerDomainSeparatorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::updateLedgerDomainSeparator)
                    }
                    updateLedgerDomainSeparator
                },
                {
                    fn LEDGER_VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <LEDGER_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::LEDGER_VERSION)
                    }
                    LEDGER_VERSION
                },
                {
                    fn ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                HoprAnnouncementsCalls::ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZE,
                            )
                    }
                    ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZE
                },
                {
                    fn announce(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <announceCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::announce)
                    }
                    announce
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn getKeyBindingCount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <getKeyBindingCountCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::getKeyBindingCount)
                    }
                    getKeyBindingCount
                },
                {
                    fn announceSafe(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <announceSafeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::announceSafe)
                    }
                    announceSafe
                },
                {
                    fn keyBindingFee(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsCalls> {
                        <keyBindingFeeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsCalls::keyBindingFee)
                    }
                    keyBindingFee
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_VALIDATE_SHIMS[idx](data)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZE(inner) => {
                    <ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LEDGER_VERSION(inner) => {
                    <LEDGER_VERSIONCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SNAPSHOT_INTERVAL(inner) => {
                    <SNAPSHOT_INTERVALCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TOKEN(inner) => {
                    <TOKENCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::TOKENS_RECIPIENT_INTERFACE_HASH(inner) => {
                    <TOKENS_RECIPIENT_INTERFACE_HASHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UPGRADE_INTERFACE_VERSION(inner) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::announce(inner) => {
                    <announceCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::announceSafe(inner) => {
                    <announceSafeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAllKeyBindings(inner) => {
                    <getAllKeyBindingsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getKeyBindingCount(inner) => {
                    <getKeyBindingCountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getKeyBindingWithKeyId(inner) => {
                    <getKeyBindingWithKeyIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getKeyIdRange(inner) => {
                    <getKeyIdRangeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getKeyIdWithOffchainKey(inner) => {
                    <getKeyIdWithOffchainKeyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOffchainKeyWithKeyId(inner) => {
                    <getOffchainKeyWithKeyIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isOffchainKeyBound(inner) => {
                    <isOffchainKeyBoundCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::keyBindingFee(inner) => {
                    <keyBindingFeeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::latestRoot(inner) => {
                    <latestRootCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::latestSnapshotRoot(inner) => {
                    <latestSnapshotRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ledgerDomainSeparator(inner) => {
                    <ledgerDomainSeparatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::multiaddrOf(inner) => {
                    <multiaddrOfCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::proxiableUUID(inner) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::revoke(inner) => {
                    <revokeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::revokeSafe(inner) => {
                    <revokeSafeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::tokensReceived(inner) => {
                    <tokensReceivedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::tryGetKeyBinding(inner) => {
                    <tryGetKeyBindingCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateKeyBindingFee(inner) => {
                    <updateKeyBindingFeeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateLedgerDomainSeparator(inner) => {
                    <updateLedgerDomainSeparatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::upgradeToAndCall(inner) => {
                    <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZE(inner) => {
                    <ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LEDGER_VERSION(inner) => {
                    <LEDGER_VERSIONCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SNAPSHOT_INTERVAL(inner) => {
                    <SNAPSHOT_INTERVALCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TOKEN(inner) => {
                    <TOKENCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::TOKENS_RECIPIENT_INTERFACE_HASH(inner) => {
                    <TOKENS_RECIPIENT_INTERFACE_HASHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UPGRADE_INTERFACE_VERSION(inner) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::announce(inner) => {
                    <announceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::announceSafe(inner) => {
                    <announceSafeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getAllKeyBindings(inner) => {
                    <getAllKeyBindingsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getKeyBindingCount(inner) => {
                    <getKeyBindingCountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getKeyBindingWithKeyId(inner) => {
                    <getKeyBindingWithKeyIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getKeyIdRange(inner) => {
                    <getKeyIdRangeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getKeyIdWithOffchainKey(inner) => {
                    <getKeyIdWithOffchainKeyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOffchainKeyWithKeyId(inner) => {
                    <getOffchainKeyWithKeyIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isOffchainKeyBound(inner) => {
                    <isOffchainKeyBoundCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::keyBindingFee(inner) => {
                    <keyBindingFeeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::latestRoot(inner) => {
                    <latestRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::latestSnapshotRoot(inner) => {
                    <latestSnapshotRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ledgerDomainSeparator(inner) => {
                    <ledgerDomainSeparatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::multiaddrOf(inner) => {
                    <multiaddrOfCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::proxiableUUID(inner) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::revoke(inner) => {
                    <revokeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::revokeSafe(inner) => {
                    <revokeSafeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::tokensReceived(inner) => {
                    <tokensReceivedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::tryGetKeyBinding(inner) => {
                    <tryGetKeyBindingCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateKeyBindingFee(inner) => {
                    <updateKeyBindingFeeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateLedgerDomainSeparator(inner) => {
                    <updateLedgerDomainSeparatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::upgradeToAndCall(inner) => {
                    <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`HoprAnnouncements`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum HoprAnnouncementsErrors {
        #[allow(missing_docs)]
        AddressEmptyCode(AddressEmptyCode),
        #[allow(missing_docs)]
        AlreadyInitialized(AlreadyInitialized),
        #[allow(missing_docs)]
        ContractNotResponsible(ContractNotResponsible),
        #[allow(missing_docs)]
        ERC1967InvalidImplementation(ERC1967InvalidImplementation),
        #[allow(missing_docs)]
        ERC1967NonPayable(ERC1967NonPayable),
        #[allow(missing_docs)]
        EmptyMultiaddr(EmptyMultiaddr),
        #[allow(missing_docs)]
        ExistingKeyBinding(ExistingKeyBinding),
        #[allow(missing_docs)]
        FailedCall(FailedCall),
        #[allow(missing_docs)]
        InvalidInitialization(InvalidInitialization),
        #[allow(missing_docs)]
        InvalidKeyBindingFeeAmount(InvalidKeyBindingFeeAmount),
        #[allow(missing_docs)]
        InvalidSafeAddress(InvalidSafeAddress),
        #[allow(missing_docs)]
        InvalidTokenRecipient(InvalidTokenRecipient),
        #[allow(missing_docs)]
        InvalidTokenSender(InvalidTokenSender),
        #[allow(missing_docs)]
        InvalidTokensReceivedUsage(InvalidTokensReceivedUsage),
        #[allow(missing_docs)]
        KeyIdOutOfRange(KeyIdOutOfRange),
        #[allow(missing_docs)]
        MultiSigUninitialized(MultiSigUninitialized),
        #[allow(missing_docs)]
        NoNeedToProvideKeyBindingFee(NoNeedToProvideKeyBindingFee),
        #[allow(missing_docs)]
        NotInitializing(NotInitializing),
        #[allow(missing_docs)]
        OwnableInvalidOwner(OwnableInvalidOwner),
        #[allow(missing_docs)]
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        #[allow(missing_docs)]
        UUPSUnauthorizedCallContext(UUPSUnauthorizedCallContext),
        #[allow(missing_docs)]
        UUPSUnsupportedProxiableUUID(UUPSUnsupportedProxiableUUID),
        #[allow(missing_docs)]
        WrongToken(WrongToken),
        #[allow(missing_docs)]
        ZeroAddress(ZeroAddress),
        #[allow(missing_docs)]
        ZeroInterval(ZeroInterval),
    }
    impl HoprAnnouncementsErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [9u8, 206u8, 172u8, 112u8],
            [11u8, 9u8, 141u8, 224u8],
            [13u8, 162u8, 126u8, 196u8],
            [13u8, 193u8, 73u8, 240u8],
            [17u8, 140u8, 218u8, 167u8],
            [30u8, 79u8, 189u8, 247u8],
            [33u8, 42u8, 119u8, 254u8],
            [52u8, 111u8, 246u8, 7u8],
            [69u8, 74u8, 32u8, 200u8],
            [71u8, 50u8, 206u8, 253u8],
            [76u8, 156u8, 140u8, 227u8],
            [105u8, 238u8, 111u8, 40u8],
            [142u8, 157u8, 124u8, 94u8],
            [149u8, 39u8, 108u8, 66u8],
            [153u8, 150u8, 179u8, 21u8],
            [160u8, 243u8, 254u8, 234u8],
            [170u8, 29u8, 73u8, 164u8],
            [172u8, 213u8, 168u8, 35u8],
            [179u8, 152u8, 151u8, 159u8],
            [185u8, 196u8, 145u8, 8u8],
            [214u8, 189u8, 162u8, 117u8],
            [215u8, 230u8, 188u8, 248u8],
            [224u8, 124u8, 141u8, 186u8],
            [234u8, 192u8, 211u8, 137u8],
            [249u8, 46u8, 232u8, 169u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(EmptyMultiaddr),
            ::core::stringify!(ExistingKeyBinding),
            ::core::stringify!(NoNeedToProvideKeyBindingFee),
            ::core::stringify!(AlreadyInitialized),
            ::core::stringify!(OwnableUnauthorizedAccount),
            ::core::stringify!(OwnableInvalidOwner),
            ::core::stringify!(KeyIdOutOfRange),
            ::core::stringify!(ZeroInterval),
            ::core::stringify!(MultiSigUninitialized),
            ::core::stringify!(InvalidKeyBindingFeeAmount),
            ::core::stringify!(ERC1967InvalidImplementation),
            ::core::stringify!(InvalidTokensReceivedUsage),
            ::core::stringify!(InvalidSafeAddress),
            ::core::stringify!(InvalidTokenSender),
            ::core::stringify!(AddressEmptyCode),
            ::core::stringify!(WrongToken),
            ::core::stringify!(UUPSUnsupportedProxiableUUID),
            ::core::stringify!(ContractNotResponsible),
            ::core::stringify!(ERC1967NonPayable),
            ::core::stringify!(InvalidTokenRecipient),
            ::core::stringify!(FailedCall),
            ::core::stringify!(NotInitializing),
            ::core::stringify!(UUPSUnauthorizedCallContext),
            ::core::stringify!(ZeroAddress),
            ::core::stringify!(InvalidInitialization),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <EmptyMultiaddr as alloy_sol_types::SolError>::SIGNATURE,
            <ExistingKeyBinding as alloy_sol_types::SolError>::SIGNATURE,
            <NoNeedToProvideKeyBindingFee as alloy_sol_types::SolError>::SIGNATURE,
            <AlreadyInitialized as alloy_sol_types::SolError>::SIGNATURE,
            <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::SIGNATURE,
            <OwnableInvalidOwner as alloy_sol_types::SolError>::SIGNATURE,
            <KeyIdOutOfRange as alloy_sol_types::SolError>::SIGNATURE,
            <ZeroInterval as alloy_sol_types::SolError>::SIGNATURE,
            <MultiSigUninitialized as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidKeyBindingFeeAmount as alloy_sol_types::SolError>::SIGNATURE,
            <ERC1967InvalidImplementation as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidTokensReceivedUsage as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidSafeAddress as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidTokenSender as alloy_sol_types::SolError>::SIGNATURE,
            <AddressEmptyCode as alloy_sol_types::SolError>::SIGNATURE,
            <WrongToken as alloy_sol_types::SolError>::SIGNATURE,
            <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::SIGNATURE,
            <ContractNotResponsible as alloy_sol_types::SolError>::SIGNATURE,
            <ERC1967NonPayable as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidTokenRecipient as alloy_sol_types::SolError>::SIGNATURE,
            <FailedCall as alloy_sol_types::SolError>::SIGNATURE,
            <NotInitializing as alloy_sol_types::SolError>::SIGNATURE,
            <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::SIGNATURE,
            <ZeroAddress as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidInitialization as alloy_sol_types::SolError>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for HoprAnnouncementsErrors {
        const NAME: &'static str = "HoprAnnouncementsErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 25usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AddressEmptyCode(_) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AlreadyInitialized(_) => {
                    <AlreadyInitialized as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ContractNotResponsible(_) => {
                    <ContractNotResponsible as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC1967InvalidImplementation(_) => {
                    <ERC1967InvalidImplementation as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC1967NonPayable(_) => {
                    <ERC1967NonPayable as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptyMultiaddr(_) => {
                    <EmptyMultiaddr as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ExistingKeyBinding(_) => {
                    <ExistingKeyBinding as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FailedCall(_) => {
                    <FailedCall as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidInitialization(_) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidKeyBindingFeeAmount(_) => {
                    <InvalidKeyBindingFeeAmount as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSafeAddress(_) => {
                    <InvalidSafeAddress as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidTokenRecipient(_) => {
                    <InvalidTokenRecipient as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidTokenSender(_) => {
                    <InvalidTokenSender as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidTokensReceivedUsage(_) => {
                    <InvalidTokensReceivedUsage as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KeyIdOutOfRange(_) => {
                    <KeyIdOutOfRange as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MultiSigUninitialized(_) => {
                    <MultiSigUninitialized as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NoNeedToProvideKeyBindingFee(_) => {
                    <NoNeedToProvideKeyBindingFee as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotInitializing(_) => {
                    <NotInitializing as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OwnableInvalidOwner(_) => {
                    <OwnableInvalidOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OwnableUnauthorizedAccount(_) => {
                    <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UUPSUnauthorizedCallContext(_) => {
                    <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UUPSUnsupportedProxiableUUID(_) => {
                    <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::SELECTOR
                }
                Self::WrongToken(_) => {
                    <WrongToken as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ZeroAddress(_) => {
                    <ZeroAddress as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ZeroInterval(_) => {
                    <ZeroInterval as alloy_sol_types::SolError>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<HoprAnnouncementsErrors>] = &[
                {
                    fn EmptyMultiaddr(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <EmptyMultiaddr as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::EmptyMultiaddr)
                    }
                    EmptyMultiaddr
                },
                {
                    fn ExistingKeyBinding(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <ExistingKeyBinding as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::ExistingKeyBinding)
                    }
                    ExistingKeyBinding
                },
                {
                    fn NoNeedToProvideKeyBindingFee(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <NoNeedToProvideKeyBindingFee as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::NoNeedToProvideKeyBindingFee)
                    }
                    NoNeedToProvideKeyBindingFee
                },
                {
                    fn AlreadyInitialized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <AlreadyInitialized as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::AlreadyInitialized)
                    }
                    AlreadyInitialized
                },
                {
                    fn OwnableUnauthorizedAccount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::OwnableUnauthorizedAccount)
                    }
                    OwnableUnauthorizedAccount
                },
                {
                    fn OwnableInvalidOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::OwnableInvalidOwner)
                    }
                    OwnableInvalidOwner
                },
                {
                    fn KeyIdOutOfRange(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <KeyIdOutOfRange as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::KeyIdOutOfRange)
                    }
                    KeyIdOutOfRange
                },
                {
                    fn ZeroInterval(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <ZeroInterval as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(HoprAnnouncementsErrors::ZeroInterval)
                    }
                    ZeroInterval
                },
                {
                    fn MultiSigUninitialized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <MultiSigUninitialized as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::MultiSigUninitialized)
                    }
                    MultiSigUninitialized
                },
                {
                    fn InvalidKeyBindingFeeAmount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <InvalidKeyBindingFeeAmount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::InvalidKeyBindingFeeAmount)
                    }
                    InvalidKeyBindingFeeAmount
                },
                {
                    fn ERC1967InvalidImplementation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::ERC1967InvalidImplementation)
                    }
                    ERC1967InvalidImplementation
                },
                {
                    fn InvalidTokensReceivedUsage(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <InvalidTokensReceivedUsage as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::InvalidTokensReceivedUsage)
                    }
                    InvalidTokensReceivedUsage
                },
                {
                    fn InvalidSafeAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <InvalidSafeAddress as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::InvalidSafeAddress)
                    }
                    InvalidSafeAddress
                },
                {
                    fn InvalidTokenSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <InvalidTokenSender as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::InvalidTokenSender)
                    }
                    InvalidTokenSender
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn WrongToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <WrongToken as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(HoprAnnouncementsErrors::WrongToken)
                    }
                    WrongToken
                },
                {
                    fn UUPSUnsupportedProxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::UUPSUnsupportedProxiableUUID)
                    }
                    UUPSUnsupportedProxiableUUID
                },
                {
                    fn ContractNotResponsible(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <ContractNotResponsible as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::ContractNotResponsible)
                    }
                    ContractNotResponsible
                },
                {
                    fn ERC1967NonPayable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <ERC1967NonPayable as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::ERC1967NonPayable)
                    }
                    ERC1967NonPayable
                },
                {
                    fn InvalidTokenRecipient(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <InvalidTokenRecipient as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::InvalidTokenRecipient)
                    }
                    InvalidTokenRecipient
                },
                {
                    fn FailedCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(HoprAnnouncementsErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn NotInitializing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <NotInitializing as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::NotInitializing)
                    }
                    NotInitializing
                },
                {
                    fn UUPSUnauthorizedCallContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::UUPSUnauthorizedCallContext)
                    }
                    UUPSUnauthorizedCallContext
                },
                {
                    fn ZeroAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <ZeroAddress as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(HoprAnnouncementsErrors::ZeroAddress)
                    }
                    ZeroAddress
                },
                {
                    fn InvalidInitialization(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <InvalidInitialization as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::InvalidInitialization)
                    }
                    InvalidInitialization
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<HoprAnnouncementsErrors>] = &[
                {
                    fn EmptyMultiaddr(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <EmptyMultiaddr as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::EmptyMultiaddr)
                    }
                    EmptyMultiaddr
                },
                {
                    fn ExistingKeyBinding(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <ExistingKeyBinding as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::ExistingKeyBinding)
                    }
                    ExistingKeyBinding
                },
                {
                    fn NoNeedToProvideKeyBindingFee(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <NoNeedToProvideKeyBindingFee as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::NoNeedToProvideKeyBindingFee)
                    }
                    NoNeedToProvideKeyBindingFee
                },
                {
                    fn AlreadyInitialized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <AlreadyInitialized as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::AlreadyInitialized)
                    }
                    AlreadyInitialized
                },
                {
                    fn OwnableUnauthorizedAccount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::OwnableUnauthorizedAccount)
                    }
                    OwnableUnauthorizedAccount
                },
                {
                    fn OwnableInvalidOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::OwnableInvalidOwner)
                    }
                    OwnableInvalidOwner
                },
                {
                    fn KeyIdOutOfRange(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <KeyIdOutOfRange as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::KeyIdOutOfRange)
                    }
                    KeyIdOutOfRange
                },
                {
                    fn ZeroInterval(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <ZeroInterval as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::ZeroInterval)
                    }
                    ZeroInterval
                },
                {
                    fn MultiSigUninitialized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <MultiSigUninitialized as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::MultiSigUninitialized)
                    }
                    MultiSigUninitialized
                },
                {
                    fn InvalidKeyBindingFeeAmount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <InvalidKeyBindingFeeAmount as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::InvalidKeyBindingFeeAmount)
                    }
                    InvalidKeyBindingFeeAmount
                },
                {
                    fn ERC1967InvalidImplementation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::ERC1967InvalidImplementation)
                    }
                    ERC1967InvalidImplementation
                },
                {
                    fn InvalidTokensReceivedUsage(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <InvalidTokensReceivedUsage as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::InvalidTokensReceivedUsage)
                    }
                    InvalidTokensReceivedUsage
                },
                {
                    fn InvalidSafeAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <InvalidSafeAddress as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::InvalidSafeAddress)
                    }
                    InvalidSafeAddress
                },
                {
                    fn InvalidTokenSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <InvalidTokenSender as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::InvalidTokenSender)
                    }
                    InvalidTokenSender
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn WrongToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <WrongToken as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::WrongToken)
                    }
                    WrongToken
                },
                {
                    fn UUPSUnsupportedProxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::UUPSUnsupportedProxiableUUID)
                    }
                    UUPSUnsupportedProxiableUUID
                },
                {
                    fn ContractNotResponsible(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <ContractNotResponsible as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::ContractNotResponsible)
                    }
                    ContractNotResponsible
                },
                {
                    fn ERC1967NonPayable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <ERC1967NonPayable as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::ERC1967NonPayable)
                    }
                    ERC1967NonPayable
                },
                {
                    fn InvalidTokenRecipient(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <InvalidTokenRecipient as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::InvalidTokenRecipient)
                    }
                    InvalidTokenRecipient
                },
                {
                    fn FailedCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn NotInitializing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <NotInitializing as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::NotInitializing)
                    }
                    NotInitializing
                },
                {
                    fn UUPSUnauthorizedCallContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::UUPSUnauthorizedCallContext)
                    }
                    UUPSUnauthorizedCallContext
                },
                {
                    fn ZeroAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <ZeroAddress as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::ZeroAddress)
                    }
                    ZeroAddress
                },
                {
                    fn InvalidInitialization(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprAnnouncementsErrors> {
                        <InvalidInitialization as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprAnnouncementsErrors::InvalidInitialization)
                    }
                    InvalidInitialization
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_VALIDATE_SHIMS[idx](data)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::AddressEmptyCode(inner) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::AlreadyInitialized(inner) => {
                    <AlreadyInitialized as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ContractNotResponsible(inner) => {
                    <ContractNotResponsible as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC1967InvalidImplementation(inner) => {
                    <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC1967NonPayable(inner) => {
                    <ERC1967NonPayable as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EmptyMultiaddr(inner) => {
                    <EmptyMultiaddr as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ExistingKeyBinding(inner) => {
                    <ExistingKeyBinding as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FailedCall(inner) => {
                    <FailedCall as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidInitialization(inner) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidKeyBindingFeeAmount(inner) => {
                    <InvalidKeyBindingFeeAmount as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidSafeAddress(inner) => {
                    <InvalidSafeAddress as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidTokenRecipient(inner) => {
                    <InvalidTokenRecipient as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidTokenSender(inner) => {
                    <InvalidTokenSender as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidTokensReceivedUsage(inner) => {
                    <InvalidTokensReceivedUsage as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::KeyIdOutOfRange(inner) => {
                    <KeyIdOutOfRange as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MultiSigUninitialized(inner) => {
                    <MultiSigUninitialized as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NoNeedToProvideKeyBindingFee(inner) => {
                    <NoNeedToProvideKeyBindingFee as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotInitializing(inner) => {
                    <NotInitializing as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OwnableInvalidOwner(inner) => {
                    <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OwnableUnauthorizedAccount(inner) => {
                    <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UUPSUnauthorizedCallContext(inner) => {
                    <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UUPSUnsupportedProxiableUUID(inner) => {
                    <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::WrongToken(inner) => {
                    <WrongToken as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ZeroAddress(inner) => {
                    <ZeroAddress as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ZeroInterval(inner) => {
                    <ZeroInterval as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AddressEmptyCode(inner) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::AlreadyInitialized(inner) => {
                    <AlreadyInitialized as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ContractNotResponsible(inner) => {
                    <ContractNotResponsible as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC1967InvalidImplementation(inner) => {
                    <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC1967NonPayable(inner) => {
                    <ERC1967NonPayable as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EmptyMultiaddr(inner) => {
                    <EmptyMultiaddr as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ExistingKeyBinding(inner) => {
                    <ExistingKeyBinding as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FailedCall(inner) => {
                    <FailedCall as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidInitialization(inner) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidKeyBindingFeeAmount(inner) => {
                    <InvalidKeyBindingFeeAmount as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidSafeAddress(inner) => {
                    <InvalidSafeAddress as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidTokenRecipient(inner) => {
                    <InvalidTokenRecipient as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidTokenSender(inner) => {
                    <InvalidTokenSender as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidTokensReceivedUsage(inner) => {
                    <InvalidTokensReceivedUsage as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::KeyIdOutOfRange(inner) => {
                    <KeyIdOutOfRange as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MultiSigUninitialized(inner) => {
                    <MultiSigUninitialized as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NoNeedToProvideKeyBindingFee(inner) => {
                    <NoNeedToProvideKeyBindingFee as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotInitializing(inner) => {
                    <NotInitializing as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OwnableInvalidOwner(inner) => {
                    <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OwnableUnauthorizedAccount(inner) => {
                    <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UUPSUnauthorizedCallContext(inner) => {
                    <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UUPSUnsupportedProxiableUUID(inner) => {
                    <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::WrongToken(inner) => {
                    <WrongToken as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::ZeroAddress(inner) => {
                    <ZeroAddress as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ZeroInterval(inner) => {
                    <ZeroInterval as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`HoprAnnouncements`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum HoprAnnouncementsEvents {
        #[allow(missing_docs)]
        AddressAnnouncement(AddressAnnouncement),
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        KeyBinding(KeyBinding),
        #[allow(missing_docs)]
        KeyBindingFeeUpdate(KeyBindingFeeUpdate),
        #[allow(missing_docs)]
        LedgerDomainSeparatorUpdated(LedgerDomainSeparatorUpdated),
        #[allow(missing_docs)]
        OwnershipTransferred(OwnershipTransferred),
        #[allow(missing_docs)]
        RevokeAnnouncement(RevokeAnnouncement),
        #[allow(missing_docs)]
        Upgraded(Upgraded),
    }
    impl HoprAnnouncementsEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                17u8, 217u8, 70u8, 83u8, 222u8, 82u8, 252u8, 20u8, 56u8, 135u8, 149u8,
                111u8, 228u8, 181u8, 228u8, 18u8, 10u8, 133u8, 11u8, 19u8, 0u8, 236u8,
                224u8, 83u8, 61u8, 238u8, 107u8, 216u8, 255u8, 145u8, 247u8, 50u8,
            ],
            [
                105u8, 37u8, 148u8, 92u8, 180u8, 198u8, 181u8, 151u8, 163u8, 97u8, 128u8,
                157u8, 91u8, 81u8, 16u8, 126u8, 78u8, 3u8, 114u8, 4u8, 4u8, 168u8, 36u8,
                131u8, 133u8, 62u8, 145u8, 250u8, 161u8, 118u8, 184u8, 142u8,
            ],
            [
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8,
                31u8, 208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8,
                218u8, 175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ],
            [
                164u8, 63u8, 173u8, 131u8, 146u8, 15u8, 208u8, 148u8, 69u8, 133u8, 94u8,
                133u8, 78u8, 115u8, 201u8, 197u8, 50u8, 225u8, 116u8, 2u8, 201u8, 206u8,
                176u8, 153u8, 147u8, 162u8, 57u8, 40u8, 67u8, 165u8, 189u8, 185u8,
            ],
            [
                164u8, 222u8, 48u8, 165u8, 40u8, 190u8, 202u8, 223u8, 130u8, 100u8,
                157u8, 19u8, 149u8, 192u8, 227u8, 13u8, 209u8, 138u8, 227u8, 91u8, 90u8,
                150u8, 206u8, 113u8, 233u8, 41u8, 91u8, 177u8, 75u8, 201u8, 243u8, 188u8,
            ],
            [
                188u8, 124u8, 215u8, 90u8, 32u8, 238u8, 39u8, 253u8, 154u8, 222u8, 186u8,
                179u8, 32u8, 65u8, 247u8, 85u8, 33u8, 77u8, 188u8, 107u8, 255u8, 169u8,
                12u8, 192u8, 34u8, 91u8, 57u8, 218u8, 46u8, 92u8, 45u8, 59u8,
            ],
            [
                196u8, 223u8, 91u8, 161u8, 104u8, 20u8, 131u8, 138u8, 178u8, 97u8, 136u8,
                41u8, 214u8, 143u8, 134u8, 35u8, 187u8, 137u8, 115u8, 2u8, 242u8, 77u8,
                189u8, 186u8, 34u8, 121u8, 219u8, 228u8, 90u8, 219u8, 61u8, 20u8,
            ],
            [
                199u8, 245u8, 5u8, 178u8, 243u8, 113u8, 174u8, 33u8, 117u8, 238u8, 73u8,
                19u8, 244u8, 73u8, 158u8, 31u8, 38u8, 51u8, 167u8, 181u8, 147u8, 99u8,
                33u8, 238u8, 209u8, 205u8, 174u8, 182u8, 17u8, 81u8, 129u8, 210u8,
            ],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(KeyBinding),
            ::core::stringify!(KeyBindingFeeUpdate),
            ::core::stringify!(OwnershipTransferred),
            ::core::stringify!(LedgerDomainSeparatorUpdated),
            ::core::stringify!(RevokeAnnouncement),
            ::core::stringify!(Upgraded),
            ::core::stringify!(AddressAnnouncement),
            ::core::stringify!(Initialized),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <KeyBinding as alloy_sol_types::SolEvent>::SIGNATURE,
            <KeyBindingFeeUpdate as alloy_sol_types::SolEvent>::SIGNATURE,
            <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE,
            <LedgerDomainSeparatorUpdated as alloy_sol_types::SolEvent>::SIGNATURE,
            <RevokeAnnouncement as alloy_sol_types::SolEvent>::SIGNATURE,
            <Upgraded as alloy_sol_types::SolEvent>::SIGNATURE,
            <AddressAnnouncement as alloy_sol_types::SolEvent>::SIGNATURE,
            <Initialized as alloy_sol_types::SolEvent>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 32usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 32usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for HoprAnnouncementsEvents {
        const NAME: &'static str = "HoprAnnouncementsEvents";
        const COUNT: usize = 8usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <AddressAnnouncement as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <AddressAnnouncement as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::AddressAnnouncement)
                }
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::Initialized)
                }
                Some(<KeyBinding as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <KeyBinding as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::KeyBinding)
                }
                Some(
                    <KeyBindingFeeUpdate as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <KeyBindingFeeUpdate as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::KeyBindingFeeUpdate)
                }
                Some(
                    <LedgerDomainSeparatorUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <LedgerDomainSeparatorUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::LedgerDomainSeparatorUpdated)
                }
                Some(
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OwnershipTransferred)
                }
                Some(
                    <RevokeAnnouncement as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <RevokeAnnouncement as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::RevokeAnnouncement)
                }
                Some(<Upgraded as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Upgraded as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Upgraded)
                }
                _ => {
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for HoprAnnouncementsEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::AddressAnnouncement(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::KeyBinding(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::KeyBindingFeeUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::LedgerDomainSeparatorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RevokeAnnouncement(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::AddressAnnouncement(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::KeyBinding(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::KeyBindingFeeUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::LedgerDomainSeparatorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RevokeAnnouncement(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`HoprAnnouncements`](self) contract instance.

See the [wrapper's documentation](`HoprAnnouncementsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> HoprAnnouncementsInstance<P, N> {
        HoprAnnouncementsInstance::<P, N>::new(address, __provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        __provider: P,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<HoprAnnouncementsInstance<P, N>>,
    > {
        HoprAnnouncementsInstance::<P, N>::deploy(__provider)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(__provider: P) -> alloy_contract::RawCallBuilder<P, N> {
        HoprAnnouncementsInstance::<P, N>::deploy_builder(__provider)
    }
    /**A [`HoprAnnouncements`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`HoprAnnouncements`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct HoprAnnouncementsInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for HoprAnnouncementsInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("HoprAnnouncementsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > HoprAnnouncementsInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`HoprAnnouncements`](self) contract instance.

See the [wrapper's documentation](`HoprAnnouncementsInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            __provider: P,
        ) -> Self {
            Self {
                address,
                provider: __provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            __provider: P,
        ) -> alloy_contract::Result<HoprAnnouncementsInstance<P, N>> {
            let call_builder = Self::deploy_builder(__provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(__provider: P) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                __provider,
                ::core::clone::Clone::clone(&BYTECODE),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<P: ::core::clone::Clone, N> HoprAnnouncementsInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> HoprAnnouncementsInstance<P, N> {
            HoprAnnouncementsInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > HoprAnnouncementsInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZE`] function.
        pub fn ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZE(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZECall,
            N,
        > {
            self.call_builder(&ERC777_HOOK_BIND_KEY_MAYBE_ANNOUNCE_SIZECall)
        }
        ///Creates a new call builder for the [`LEDGER_VERSION`] function.
        pub fn LEDGER_VERSION(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, LEDGER_VERSIONCall, N> {
            self.call_builder(&LEDGER_VERSIONCall)
        }
        ///Creates a new call builder for the [`SNAPSHOT_INTERVAL`] function.
        pub fn SNAPSHOT_INTERVAL(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, SNAPSHOT_INTERVALCall, N> {
            self.call_builder(&SNAPSHOT_INTERVALCall)
        }
        ///Creates a new call builder for the [`TOKEN`] function.
        pub fn TOKEN(&self) -> alloy_contract::SolCallBuilder<&P, TOKENCall, N> {
            self.call_builder(&TOKENCall)
        }
        ///Creates a new call builder for the [`TOKENS_RECIPIENT_INTERFACE_HASH`] function.
        pub fn TOKENS_RECIPIENT_INTERFACE_HASH(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, TOKENS_RECIPIENT_INTERFACE_HASHCall, N> {
            self.call_builder(&TOKENS_RECIPIENT_INTERFACE_HASHCall)
        }
        ///Creates a new call builder for the [`UPGRADE_INTERFACE_VERSION`] function.
        pub fn UPGRADE_INTERFACE_VERSION(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, UPGRADE_INTERFACE_VERSIONCall, N> {
            self.call_builder(&UPGRADE_INTERFACE_VERSIONCall)
        }
        ///Creates a new call builder for the [`announce`] function.
        pub fn announce(
            &self,
            baseMultiaddr: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<&P, announceCall, N> {
            self.call_builder(&announceCall { baseMultiaddr })
        }
        ///Creates a new call builder for the [`announceSafe`] function.
        pub fn announceSafe(
            &self,
            selfAddress: alloy::sol_types::private::Address,
            baseMultiaddr: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<&P, announceSafeCall, N> {
            self.call_builder(
                &announceSafeCall {
                    selfAddress,
                    baseMultiaddr,
                },
            )
        }
        ///Creates a new call builder for the [`getAllKeyBindings`] function.
        pub fn getAllKeyBindings(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getAllKeyBindingsCall, N> {
            self.call_builder(&getAllKeyBindingsCall)
        }
        ///Creates a new call builder for the [`getKeyBindingCount`] function.
        pub fn getKeyBindingCount(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getKeyBindingCountCall, N> {
            self.call_builder(&getKeyBindingCountCall)
        }
        ///Creates a new call builder for the [`getKeyBindingWithKeyId`] function.
        pub fn getKeyBindingWithKeyId(
            &self,
            keyId: <KeyId as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, getKeyBindingWithKeyIdCall, N> {
            self.call_builder(
                &getKeyBindingWithKeyIdCall {
                    keyId,
                },
            )
        }
        ///Creates a new call builder for the [`getKeyIdRange`] function.
        pub fn getKeyIdRange(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getKeyIdRangeCall, N> {
            self.call_builder(&getKeyIdRangeCall)
        }
        ///Creates a new call builder for the [`getKeyIdWithOffchainKey`] function.
        pub fn getKeyIdWithOffchainKey(
            &self,
            ed25519_pub_key: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, getKeyIdWithOffchainKeyCall, N> {
            self.call_builder(
                &getKeyIdWithOffchainKeyCall {
                    ed25519_pub_key,
                },
            )
        }
        ///Creates a new call builder for the [`getOffchainKeyWithKeyId`] function.
        pub fn getOffchainKeyWithKeyId(
            &self,
            keyId: <KeyId as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, getOffchainKeyWithKeyIdCall, N> {
            self.call_builder(
                &getOffchainKeyWithKeyIdCall {
                    keyId,
                },
            )
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            initParams: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, initializeCall, N> {
            self.call_builder(&initializeCall { initParams })
        }
        ///Creates a new call builder for the [`isOffchainKeyBound`] function.
        pub fn isOffchainKeyBound(
            &self,
            ed25519_pub_key: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, isOffchainKeyBoundCall, N> {
            self.call_builder(
                &isOffchainKeyBoundCall {
                    ed25519_pub_key,
                },
            )
        }
        ///Creates a new call builder for the [`keyBindingFee`] function.
        pub fn keyBindingFee(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, keyBindingFeeCall, N> {
            self.call_builder(&keyBindingFeeCall)
        }
        ///Creates a new call builder for the [`latestRoot`] function.
        pub fn latestRoot(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, latestRootCall, N> {
            self.call_builder(&latestRootCall)
        }
        ///Creates a new call builder for the [`latestSnapshotRoot`] function.
        pub fn latestSnapshotRoot(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, latestSnapshotRootCall, N> {
            self.call_builder(&latestSnapshotRootCall)
        }
        ///Creates a new call builder for the [`ledgerDomainSeparator`] function.
        pub fn ledgerDomainSeparator(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, ledgerDomainSeparatorCall, N> {
            self.call_builder(&ledgerDomainSeparatorCall)
        }
        ///Creates a new call builder for the [`multiaddrOf`] function.
        pub fn multiaddrOf(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, multiaddrOfCall, N> {
            self.call_builder(&multiaddrOfCall(_0))
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<&P, ownerCall, N> {
            self.call_builder(&ownerCall)
        }
        ///Creates a new call builder for the [`proxiableUUID`] function.
        pub fn proxiableUUID(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, proxiableUUIDCall, N> {
            self.call_builder(&proxiableUUIDCall)
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall)
        }
        ///Creates a new call builder for the [`revoke`] function.
        pub fn revoke(&self) -> alloy_contract::SolCallBuilder<&P, revokeCall, N> {
            self.call_builder(&revokeCall)
        }
        ///Creates a new call builder for the [`revokeSafe`] function.
        pub fn revokeSafe(
            &self,
            selfAddress: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, revokeSafeCall, N> {
            self.call_builder(&revokeSafeCall { selfAddress })
        }
        ///Creates a new call builder for the [`tokensReceived`] function.
        pub fn tokensReceived(
            &self,
            _0: alloy::sol_types::private::Address,
            from: alloy::sol_types::private::Address,
            to: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
            userData: alloy::sol_types::private::Bytes,
            _5: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, tokensReceivedCall, N> {
            self.call_builder(
                &tokensReceivedCall {
                    _0,
                    from,
                    to,
                    amount,
                    userData,
                    _5,
                },
            )
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`tryGetKeyBinding`] function.
        pub fn tryGetKeyBinding(
            &self,
            ed25519_pub_key: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, tryGetKeyBindingCall, N> {
            self.call_builder(
                &tryGetKeyBindingCall {
                    ed25519_pub_key,
                },
            )
        }
        ///Creates a new call builder for the [`updateKeyBindingFee`] function.
        pub fn updateKeyBindingFee(
            &self,
            newFee: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, updateKeyBindingFeeCall, N> {
            self.call_builder(&updateKeyBindingFeeCall { newFee })
        }
        ///Creates a new call builder for the [`updateLedgerDomainSeparator`] function.
        pub fn updateLedgerDomainSeparator(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, updateLedgerDomainSeparatorCall, N> {
            self.call_builder(&updateLedgerDomainSeparatorCall)
        }
        ///Creates a new call builder for the [`upgradeToAndCall`] function.
        pub fn upgradeToAndCall(
            &self,
            newImplementation: alloy::sol_types::private::Address,
            data: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, upgradeToAndCallCall, N> {
            self.call_builder(
                &upgradeToAndCallCall {
                    newImplementation,
                    data,
                },
            )
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > HoprAnnouncementsInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`AddressAnnouncement`] event.
        pub fn AddressAnnouncement_filter(
            &self,
        ) -> alloy_contract::Event<&P, AddressAnnouncement, N> {
            self.event_filter::<AddressAnnouncement>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<&P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`KeyBinding`] event.
        pub fn KeyBinding_filter(&self) -> alloy_contract::Event<&P, KeyBinding, N> {
            self.event_filter::<KeyBinding>()
        }
        ///Creates a new event filter for the [`KeyBindingFeeUpdate`] event.
        pub fn KeyBindingFeeUpdate_filter(
            &self,
        ) -> alloy_contract::Event<&P, KeyBindingFeeUpdate, N> {
            self.event_filter::<KeyBindingFeeUpdate>()
        }
        ///Creates a new event filter for the [`LedgerDomainSeparatorUpdated`] event.
        pub fn LedgerDomainSeparatorUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, LedgerDomainSeparatorUpdated, N> {
            self.event_filter::<LedgerDomainSeparatorUpdated>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<&P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`RevokeAnnouncement`] event.
        pub fn RevokeAnnouncement_filter(
            &self,
        ) -> alloy_contract::Event<&P, RevokeAnnouncement, N> {
            self.event_filter::<RevokeAnnouncement>()
        }
        ///Creates a new event filter for the [`Upgraded`] event.
        pub fn Upgraded_filter(&self) -> alloy_contract::Event<&P, Upgraded, N> {
            self.event_filter::<Upgraded>()
        }
    }
}
