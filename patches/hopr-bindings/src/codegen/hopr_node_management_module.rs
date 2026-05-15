///Module containing a contract's types and functions.
/**

```solidity
library Enum {
    type Operation is uint8;
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod Enum {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Operation(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Operation> for u8 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        impl Operation {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(value: u8) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(self) -> u8 {
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
        impl From<u8> for Operation {
            fn from(value: u8) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<Operation> for u8 {
            fn from(value: Operation) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Operation {
            type RustType = u8;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Operation {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`Enum`](self) contract instance.

See the [wrapper's documentation](`EnumInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(address: alloy_sol_types::private::Address, __provider: P) -> EnumInstance<P, N> {
        EnumInstance::<P, N>::new(address, __provider)
    }
    /**A [`Enum`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`Enum`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct EnumInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for EnumInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("EnumInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > EnumInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`Enum`](self) contract instance.

See the [wrapper's documentation](`EnumInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> EnumInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> EnumInstance<P, N> {
            EnumInstance {
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
    > EnumInstance<P, N> {
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
    > EnumInstance<P, N> {
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
library Enum {
    type Operation is uint8;
}

interface HoprNodeManagementModule {
    type GranularPermission is uint8;
    type Target is uint256;

    error AddressEmptyCode(address target);
    error AddressIsZero();
    error ArrayTooLong();
    error ArraysDifferentLength();
    error CalldataOutOfBounds();
    error CannotChangeOwner();
    error DefaultPermissionRejected();
    error DelegateCallNotAllowed();
    error ERC1967InvalidImplementation(address implementation);
    error ERC1967NonPayable();
    error FailedCall();
    error FailedToSendEthToNode();
    error FunctionSignatureTooShort();
    error GranularPermissionRejected();
    error InvalidInitialization();
    error LengthIsZero();
    error NoMembership();
    error NodePermissionRejected();
    error NonExistentKey();
    error NotInitializing();
    error OwnableInvalidOwner(address owner);
    error OwnableUnauthorizedAccount(address account);
    error ParameterNotAllowed();
    error PermissionNotConfigured();
    error PermissionNotFound();
    error SafeMultisendSameAddress();
    error SendNotAllowed();
    error TargetAddressNotAllowed();
    error TargetIsNotScoped();
    error TargetIsScoped();
    error TooManyCapabilities();
    error UUPSUnauthorizedCallContext();
    error UUPSUnsupportedProxiableUUID(bytes32 slot);
    error UnacceptableMultiSendOffset();
    error WithMembership();

    event ExecutionFailure();
    event ExecutionSuccess();
    event Initialized(uint64 version);
    event NodeAdded(address indexed node);
    event NodeRemoved(address indexed node);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event RevokedTarget(address indexed targetAddress);
    event ScopedGranularChannelCapability(address indexed targetAddress, bytes32 indexed channelId, bytes4 selector, GranularPermission permission);
    event ScopedGranularSendCapability(address indexed nodeAddress, address indexed recipientAddress, GranularPermission permission);
    event ScopedGranularTokenCapability(address indexed nodeAddress, address indexed targetAddress, address indexed recipientAddress, bytes4 selector, GranularPermission permission);
    event ScopedTargetChannels(address indexed targetAddress, Target target);
    event ScopedTargetSend(address indexed targetAddress, Target target);
    event ScopedTargetToken(address indexed targetAddress, Target target);
    event SetMultisendAddress(address indexed multisendAddress);
    event Upgraded(address indexed implementation);

    constructor();

    function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
    function VERSION() external view returns (string memory);
    function addChannelsAndTokenTarget(Target defaultTarget) external;
    function addNode(address nodeAddress) external payable;
    function addNodes(address[] memory nodeAddresses) external payable;
    function decodeFunctionSigsAndPermissions(bytes32 encoded, uint256 length) external pure returns (bytes4[] memory functionSigs, GranularPermission[] memory permissions);
    function encodeFunctionSigsAndPermissions(bytes4[] memory functionSigs, GranularPermission[] memory permissions) external pure returns (bytes32 encoded, uint256 length);
    function execTransactionFromModule(address to, uint256 value, bytes memory data, Enum.Operation operation) external returns (bool success);
    function execTransactionFromModuleReturnData(address to, uint256 value, bytes memory data, Enum.Operation operation) external returns (bool, bytes memory);
    function getGranularPermissions(bytes32 capabilityKey, bytes32 pairId) external view returns (GranularPermission);
    function getTargets() external view returns (Target[] memory);
    function includeNode(Target nodeDefaultTarget) external;
    function includeNodes(address[] memory nodeAddresses) external payable;
    function initialize(bytes memory initParams) external;
    function isHoprNodeManagementModule() external view returns (bool);
    function isNode(address nodeAddress) external view returns (bool);
    function multisend() external view returns (address);
    function owner() external view returns (address);
    function proxiableUUID() external view returns (bytes32);
    function removeNode(address nodeAddress) external;
    function renounceOwnership() external;
    function revokeTarget(address targetAddress) external;
    function scopeChannelsCapabilities(address targetAddress, bytes32 channelId, bytes32 encodedSigsPermissions) external;
    function scopeSendCapability(address nodeAddress, address beneficiary, GranularPermission permission) external;
    function scopeTargetChannels(Target defaultTarget) external;
    function scopeTargetSend(Target defaultTarget) external;
    function scopeTargetToken(Target defaultTarget) external;
    function scopeTokenCapabilities(address nodeAddress, address targetAddress, address beneficiary, bytes32 encodedSigsPermissions) external;
    function setMultisend(address _multisend) external;
    function transferOwnership(address newOwner) external;
    function tryGetTarget(address targetAddress) external view returns (bool, Target);
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
    "name": "VERSION",
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
    "name": "addChannelsAndTokenTarget",
    "inputs": [
      {
        "name": "defaultTarget",
        "type": "uint256",
        "internalType": "Target"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "addNode",
    "inputs": [
      {
        "name": "nodeAddress",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "addNodes",
    "inputs": [
      {
        "name": "nodeAddresses",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "decodeFunctionSigsAndPermissions",
    "inputs": [
      {
        "name": "encoded",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "length",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "functionSigs",
        "type": "bytes4[]",
        "internalType": "bytes4[]"
      },
      {
        "name": "permissions",
        "type": "uint8[]",
        "internalType": "enum GranularPermission[]"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "encodeFunctionSigsAndPermissions",
    "inputs": [
      {
        "name": "functionSigs",
        "type": "bytes4[]",
        "internalType": "bytes4[]"
      },
      {
        "name": "permissions",
        "type": "uint8[]",
        "internalType": "enum GranularPermission[]"
      }
    ],
    "outputs": [
      {
        "name": "encoded",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "length",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "execTransactionFromModule",
    "inputs": [
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "value",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "operation",
        "type": "uint8",
        "internalType": "enum Enum.Operation"
      }
    ],
    "outputs": [
      {
        "name": "success",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "execTransactionFromModuleReturnData",
    "inputs": [
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "value",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "operation",
        "type": "uint8",
        "internalType": "enum Enum.Operation"
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
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getGranularPermissions",
    "inputs": [
      {
        "name": "capabilityKey",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "pairId",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "enum GranularPermission"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getTargets",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256[]",
        "internalType": "Target[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "includeNode",
    "inputs": [
      {
        "name": "nodeDefaultTarget",
        "type": "uint256",
        "internalType": "Target"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "includeNodes",
    "inputs": [
      {
        "name": "nodeAddresses",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
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
    "name": "isHoprNodeManagementModule",
    "inputs": [],
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
    "name": "isNode",
    "inputs": [
      {
        "name": "nodeAddress",
        "type": "address",
        "internalType": "address"
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
    "name": "multisend",
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
    "name": "removeNode",
    "inputs": [
      {
        "name": "nodeAddress",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
    "name": "revokeTarget",
    "inputs": [
      {
        "name": "targetAddress",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "scopeChannelsCapabilities",
    "inputs": [
      {
        "name": "targetAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "channelId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "encodedSigsPermissions",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "scopeSendCapability",
    "inputs": [
      {
        "name": "nodeAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "beneficiary",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "permission",
        "type": "uint8",
        "internalType": "enum GranularPermission"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "scopeTargetChannels",
    "inputs": [
      {
        "name": "defaultTarget",
        "type": "uint256",
        "internalType": "Target"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "scopeTargetSend",
    "inputs": [
      {
        "name": "defaultTarget",
        "type": "uint256",
        "internalType": "Target"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "scopeTargetToken",
    "inputs": [
      {
        "name": "defaultTarget",
        "type": "uint256",
        "internalType": "Target"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "scopeTokenCapabilities",
    "inputs": [
      {
        "name": "nodeAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "targetAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "beneficiary",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "encodedSigsPermissions",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setMultisend",
    "inputs": [
      {
        "name": "_multisend",
        "type": "address",
        "internalType": "address"
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
    "name": "tryGetTarget",
    "inputs": [
      {
        "name": "targetAddress",
        "type": "address",
        "internalType": "address"
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
        "type": "uint256",
        "internalType": "Target"
      }
    ],
    "stateMutability": "view"
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
    "name": "ExecutionFailure",
    "inputs": [],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ExecutionSuccess",
    "inputs": [],
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
    "name": "NodeAdded",
    "inputs": [
      {
        "name": "node",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "NodeRemoved",
    "inputs": [
      {
        "name": "node",
        "type": "address",
        "indexed": true,
        "internalType": "address"
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
    "name": "RevokedTarget",
    "inputs": [
      {
        "name": "targetAddress",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ScopedGranularChannelCapability",
    "inputs": [
      {
        "name": "targetAddress",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "channelId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "selector",
        "type": "bytes4",
        "indexed": false,
        "internalType": "bytes4"
      },
      {
        "name": "permission",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum GranularPermission"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ScopedGranularSendCapability",
    "inputs": [
      {
        "name": "nodeAddress",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "recipientAddress",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "permission",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum GranularPermission"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ScopedGranularTokenCapability",
    "inputs": [
      {
        "name": "nodeAddress",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "targetAddress",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "recipientAddress",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "selector",
        "type": "bytes4",
        "indexed": false,
        "internalType": "bytes4"
      },
      {
        "name": "permission",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum GranularPermission"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ScopedTargetChannels",
    "inputs": [
      {
        "name": "targetAddress",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "target",
        "type": "uint256",
        "indexed": false,
        "internalType": "Target"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ScopedTargetSend",
    "inputs": [
      {
        "name": "targetAddress",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "target",
        "type": "uint256",
        "indexed": false,
        "internalType": "Target"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ScopedTargetToken",
    "inputs": [
      {
        "name": "targetAddress",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "target",
        "type": "uint256",
        "indexed": false,
        "internalType": "Target"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SetMultisendAddress",
    "inputs": [
      {
        "name": "multisendAddress",
        "type": "address",
        "indexed": true,
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
    "name": "AddressIsZero",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ArrayTooLong",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ArraysDifferentLength",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CalldataOutOfBounds",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CannotChangeOwner",
    "inputs": []
  },
  {
    "type": "error",
    "name": "DefaultPermissionRejected",
    "inputs": []
  },
  {
    "type": "error",
    "name": "DelegateCallNotAllowed",
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
    "name": "FailedCall",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FailedToSendEthToNode",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FunctionSignatureTooShort",
    "inputs": []
  },
  {
    "type": "error",
    "name": "GranularPermissionRejected",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidInitialization",
    "inputs": []
  },
  {
    "type": "error",
    "name": "LengthIsZero",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NoMembership",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NodePermissionRejected",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NonExistentKey",
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
    "name": "ParameterNotAllowed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "PermissionNotConfigured",
    "inputs": []
  },
  {
    "type": "error",
    "name": "PermissionNotFound",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SafeMultisendSameAddress",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SendNotAllowed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TargetAddressNotAllowed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TargetIsNotScoped",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TargetIsScoped",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TooManyCapabilities",
    "inputs": []
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
    "name": "UnacceptableMultiSendOffset",
    "inputs": []
  },
  {
    "type": "error",
    "name": "WithMembership",
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
pub mod HoprNodeManagementModule {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a060405230608052348015610013575f5ffd5b5061001c610021565b6100d3565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00805468010000000000000000900460ff16156100715760405163f92ee8a960e01b815260040160405180910390fd5b80546001600160401b03908116146100d05780546001600160401b0319166001600160401b0390811782556040519081527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a15b50565b60805161366d6100f95f395f81816115a8015281816115d101526117f8015261366d5ff3fe6080604052600436106101db575f3560e01c80638da5cb5b116100fd578063c68c3a8311610092578063df9620eb11610062578063df9620eb146105bc578063f2fde38b146105cf578063fa19501d146105ee578063ffa1ad741461060d575f5ffd5b8063c68c3a831461051c578063dc06109d1461053b578063dc446a4a1461055a578063df4e6f8a14610586575f5ffd5b8063ad3cb1cc116100cd578063ad3cb1cc14610482578063b2b99ec9146104bf578063b5736962146104de578063c68605c8146104fd575f5ffd5b80638da5cb5b1461041d5780639d95f1cc14610431578063a2450f8914610444578063a76c9a2f14610463575f5ffd5b80635229073f1161017357806363fe3b561161014357806363fe3b56146103aa578063715018a6146103cb578063739c4b08146103df5780638b95eccd146103fe575f5ffd5b80635229073f146102fa57806352d1902d1461032757806356f551171461034957806360976c4b1461037d575f5ffd5b8063439fab91116101ae578063439fab9114610295578063468721a7146102b45780634a1ba408146102d35780634f1ef286146102e7575f5ffd5b806301750152146101df578063110dcee71461022b578063294402cc146102405780633401cde814610276575b5f5ffd5b3480156101ea575f5ffd5b506102166101f9366004612d3c565b6001600160a01b03165f9081526003602052604090205460ff1690565b60405190151581526020015b60405180910390f35b61023e610239366004612d57565b61063d565b005b34801561024b575f5ffd5b505f5461025e906001600160a01b031681565b6040516001600160a01b039091168152602001610222565b348015610281575f5ffd5b5061023e610290366004612d3c565b61080a565b3480156102a0575f5ffd5b5061023e6102af366004612e82565b610820565b3480156102bf575f5ffd5b506102166102ce366004612eb3565b610a10565b3480156102de575f5ffd5b50610216600181565b61023e6102f5366004612f52565b610aa7565b348015610305575f5ffd5b50610319610314366004612eb3565b610ac6565b604051610222929190612fcc565b348015610332575f5ffd5b5061033b610b64565b604051908152602001610222565b348015610354575f5ffd5b50610368610363366004613087565b610b7f565b60408051928352602083019190915201610222565b348015610388575f5ffd5b5061039c61039736600461314c565b610b97565b604051610222929190613194565b3480156103b5575f5ffd5b506103be610ba4565b6040516102229190613226565b3480156103d6575f5ffd5b5061023e610bb5565b3480156103ea575f5ffd5b5061023e6103f9366004613268565b610bc8565b348015610409575f5ffd5b5061023e610418366004612d3c565b610bdb565b348015610428575f5ffd5b5061025e610c2a565b61023e61043f366004612d3c565b610c58565b34801561044f575f5ffd5b5061023e61045e366004613268565b610ce2565b34801561046e575f5ffd5b5061023e61047d366004613268565b610cf3565b34801561048d575f5ffd5b506104b2604051806040016040528060058152602001640352e302e360dc1b81525081565b604051610222919061327f565b3480156104ca575f5ffd5b5061023e6104d9366004612d3c565b610d06565b3480156104e9575f5ffd5b5061023e6104f8366004613268565b610d8e565b348015610508575f5ffd5b5061023e610517366004613291565b610dc5565b348015610527575f5ffd5b5061023e6105363660046132df565b610de1565b348015610546575f5ffd5b5061023e610555366004613268565b610dfb565b348015610565575f5ffd5b5061057961057436600461314c565b610e54565b6040516102229190613323565b348015610591575f5ffd5b506105a56105a0366004612d3c565b610e77565b604080519215158352602083019190915201610222565b61023e6105ca366004612d57565b610e8d565b3480156105da575f5ffd5b5061023e6105e9366004612d3c565b610f8d565b3480156105f9575f5ffd5b5061023e610608366004613331565b610fcc565b348015610618575f5ffd5b506104b2604051806040016040528060058152602001640322e302e360dc1b81525081565b610645610fe1565b80345f82900361066857604051630c71ec1760e01b815260040160405180910390fd5b5f6106738383613377565b90505f5b83811015610802576106ae86868381811061069457610694613396565b90506020020160208101906106a99190612d3c565b611013565b5f60608787848181106106c3576106c3613396565b90506020020160208101906106d89190612d3c565b6001600160a01b0316901b6b0102030000000000000000001790506106fe600182611097565b610758600188888581811061071557610715613396565b905060200201602081019061072a9190612d3c565b89898681811061073c5761073c613396565b90506020020160208101906107519190612d3c565b6001611165565b82156107f9575f87878481811061077157610771613396565b90506020020160208101906107869190612d3c565b6001600160a01b0316845f906040515f60405180830381858888f193505050503d805f81146107d0576040519150601f19603f3d011682016040523d82523d5f602084013e6107d5565b606091505b50509050806107f757604051639a73ee9960e01b815260040160405180910390fd5b505b50600101610677565b505050505050565b610812610fe1565b61081d6001826111ff565b50565b5f610829611263565b805490915060ff600160401b82041615906001600160401b03165f8115801561084f5750825b90505f826001600160401b0316600114801561086a5750303b155b905081158015610878575080155b156108965760405163f92ee8a960e01b815260040160405180910390fd5b845467ffffffffffffffff1916600117855583156108c057845460ff60401b1916600160401b1785555b5f5f5f5f898060200190518101906108d891906133aa565b929650909450925090506001600160a01b03841615806108ff57506001600160a01b038316155b1561091d5760405163867915ab60e01b815260040160405180910390fd5b826001600160a01b0316846001600160a01b03160361094f5760405163598a0e2160e01b815260040160405180910390fd5b5f80546001600160a01b0319166001600160a01b038516179055811561097a5761097a60018361128b565b6109838161134a565b61098c8461140c565b6040516001600160a01b038416907f5fe6aabf4e790843df43ae0e22b58620066fb389295bedc06a92df6c3b28777d905f90a250505050831561080257845460ff60401b19168555604051600181527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a1505050505050565b335f9081526003602052604081205460ff16610a3f57604051631fb1d3e560e31b815260040160405180910390fd5b5f54610a5b906001906001600160a01b03168888888888611414565b610a9d868686868080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152508892506114bd915050565b9695505050505050565b610aaf61159d565b610ab882611641565b610ac28282611649565b5050565b335f9081526003602052604081205460609060ff16610af857604051631fb1d3e560e31b815260040160405180910390fd5b5f54610b14906001906001600160a01b03168989898989611414565b610b56878787878080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250899250611705915050565b915091509550959350505050565b5f610b6d6117ed565b505f5160206136185f395f51905f5290565b5f5f610b8b8484611836565b915091505b9250929050565b606080610b8b8484611936565b6060610bb06001611acc565b905090565b610bbd610fe1565b610bc65f611b25565b565b610bd0610fe1565b61081d600182611b95565b610be3610fe1565b5f80546001600160a01b0319166001600160a01b038316908117825560405190917f5fe6aabf4e790843df43ae0e22b58620066fb389295bedc06a92df6c3b28777d91a250565b7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300546001600160a01b031690565b610c60610fe1565b610c6981611013565b341561081d575f816001600160a01b0316345f906040515f60405180830381858888f193505050503d805f8114610cbb576040519150601f19603f3d011682016040523d82523d5f602084013e610cc0565b606091505b5050905080610ac257604051639a73ee9960e01b815260040160405180910390fd5b610cea610fe1565b61081d8161134a565b610cfb610fe1565b61081d60018261128b565b610d0e610fe1565b6001600160a01b0381165f9081526003602052604090205460ff16610d4657604051631fb1d3e560e31b815260040160405180910390fd5b6001600160a01b0381165f81815260036020526040808220805460ff19169055517fcfc24166db4bb677e857cacabd1541fb2b30645021b27c5130419589b84db52b9190a250565b610d96610fe1565b5f610da18260601c90565b9050610dac81611013565b610db7600183611097565b610ac2600182836001611165565b610dcd610fe1565b610ddb600185858585611c55565b50505050565b610de9610fe1565b610df66001848484611165565b505050565b610e03610fe1565b5f610e0e8260601c90565b6001600160a01b0381165f9081526003602052604090205490915060ff16610e4957604051631fb1d3e560e31b815260040160405180910390fd5b610ac2600183611097565b5f82815260046020908152604080832084845290915290205460ff165b92915050565b5f80610e84600184611dc8565b91509150915091565b610e95610fe1565b80345f829003610eb857604051630c71ec1760e01b815260040160405180910390fd5b5f610ec38383613377565b90505f5b8381101561080257610ee486868381811061069457610694613396565b8115610f85575f868683818110610efd57610efd613396565b9050602002016020810190610f129190612d3c565b6001600160a01b0316835f906040515f60405180830381858888f193505050503d805f8114610f5c576040519150601f19603f3d011682016040523d82523d5f602084013e610f61565b606091505b5050905080610f8357604051639a73ee9960e01b815260040160405180910390fd5b505b600101610ec7565b610f95610fe1565b6001600160a01b038116610fc357604051631e4fbdf760e01b81525f60048201526024015b60405180910390fd5b61081d81611b25565b610fd4610fe1565b610df66001848484611e27565b33610fea610c2a565b6001600160a01b031614610bc65760405163118cdaa760e01b8152336004820152602401610fba565b6001600160a01b0381165f9081526003602052604090205460ff161561104c576040516338e816a560e21b815260040160405180910390fd5b6001600160a01b0381165f81815260036020526040808220805460ff19166001179055517fb25d03aaf308d7291709be1ea28b800463cf3a9a4c4a5555d7333a964c1dfebd9190a250565b5f6110a28260601c90565b90506001600160a01b0381166110cb5760405163867915ab60e01b815260040160405180910390fd5b6001600160a01b0381165f90815260018401602052604090205415611103576040516374603e9560e11b815260040160405180910390fd5b5f61110f836002611f60565b905061111b8482611ff0565b50816001600160a01b03167f1ee2791f2caf0e92a9dc32a37a9ea53ab6ac7a6fb8f2d090e53a067d3a43f6ac8260405161115791815260200190565b60405180910390a250505050565b5f808052600385016020526040812082916111808686612073565b815260208101919091526040015f20805460ff191660018360028111156111a9576111a961316c565b0217905550816001600160a01b0316836001600160a01b03167f7487530ddff120799505e52b1b19b6933f85a9eeae9220c80a7ad7c429b612ae836040516111f19190613323565b60405180910390a350505050565b5f61120a83836120b7565b9050801561124a576040516001600160a01b038316907f0dfce1ea4ba1eeba891ffb2a066790fbc293a9e517fe61d49d156a30165f93f3905f90a2505050565b604051634a89032160e01b815260040160405180910390fd5b5f807ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00610e71565b5f6112968260601c90565b90506001600160a01b0381166112bf5760405163867915ab60e01b815260040160405180910390fd5b6001600160a01b0381165f908152600184016020526040902054156112f7576040516374603e9560e11b815260040160405180910390fd5b5f6113028382611f60565b905061130e8482611ff0565b50816001600160a01b03167faaf26bb12aa89ee96bbe19667a6a055727b75d3f6ed7b8b611ef6519180209d68260405161115791815260200190565b5f6113558260601c90565b90505f816001600160a01b03166382bfefc86040518163ffffffff1660e01b8152600401602060405180830381865afa158015611394573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113b891906133ef565b90506113e360016bffffffffffffffffffffffff8516606085901b6001600160601b03191617611b95565b610df660016bffffffffffffffffffffffff8516606084901b6001600160601b0319161761128b565b610f956121cc565b846001600160a01b0316866001600160a01b0316036114715761146c8784848080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152506121f192505050565b6114b4565b6114b487868686868080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525088925061228f915050565b50505050505050565b5f6114c6610c2a565b6001600160a01b031663468721a7868686866040518563ffffffff1660e01b81526004016114f7949392919061340a565b6020604051808303815f875af1158015611513573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115379190613460565b9050801561156c576040517f4e2e86d21375ebcbf6e93df5ebdd5a915bf830245904c3b54f48adf0170aae4b905f90a1611595565b6040517fc24d93608a03d263ff191d7677141f5e94c496e593108f3aae0cb5b70494c4d3905f90a15b949350505050565b306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016148061162357507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166116175f5160206136185f395f51905f52546001600160a01b031690565b6001600160a01b031614155b15610bc65760405163703e46dd60e11b815260040160405180910390fd5b61081d610fe1565b816001600160a01b03166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa9250505080156116a3575060408051601f3d908101601f191682019092526116a091810190613479565b60015b6116cb57604051634c9c8ce360e01b81526001600160a01b0383166004820152602401610fba565b5f5160206136185f395f51905f5281146116fb57604051632a87526960e21b815260048101829052602401610fba565b610df683836124c8565b5f6060611710610c2a565b6001600160a01b0316635229073f878787876040518563ffffffff1660e01b8152600401611741949392919061340a565b5f604051808303815f875af115801561175c573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526117839190810190613490565b909250905081156117bb576040517f4e2e86d21375ebcbf6e93df5ebdd5a915bf830245904c3b54f48adf0170aae4b905f90a16117e4565b6040517fc24d93608a03d263ff191d7677141f5e94c496e593108f3aae0cb5b70494c4d3905f90a15b94509492505050565b306001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614610bc65760405163703e46dd60e11b815260040160405180910390fd5b81515f908190600781111561185e576040516317a4d98760e31b815260040160405180910390fd5b8351855114611880576040516374f4d53760e01b815260040160405180910390fd5b5f805b828110156118db57611896816020613515565b6118a19060e061352c565b60e08883815181106118b5576118b5613396565b60209081029190910101516001600160e01b031916901c901b9190911790600101611883565b505f5b8281101561192b576118f1816002613515565b86828151811061190357611903613396565b6020026020010151600281111561191c5761191c61316c565b901b91909117906001016118de565b509590945092505050565b606080600783111561195b576040516317a4d98760e31b815260040160405180910390fd5b826001600160401b0381111561197357611973612dc6565b60405190808252806020026020018201604052801561199c578160200160208202803683370190505b509150826001600160401b038111156119b7576119b7612dc6565b6040519080825280602002602001820160405280156119e0578160200160208202803683370190505b5090505f5b83811015611a3d5760e06119fa826020613515565b611a059060e061352c565b86901c901b838281518110611a1c57611a1c613396565b6001600160e01b0319909216602092830291909101909101526001016119e5565b505f5b83811015611ac45760fe611a55826002613515565b611a609060fe61352c565b865f1c901b901c60ff166002811115611a7b57611a7b61316c565b828281518110611a8d57611a8d613396565b60200260200101906002811115611aa657611aa661316c565b90816002811115611ab957611ab961316c565b905250600101611a40565b509250929050565b6060815f01805480602002602001604051908101604052809291908181526020018280548015611b1957602002820191905f5260205f20905b815481526020019060010190808311611b05575b50505050509050919050565b7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930080546001600160a01b031981166001600160a01b03848116918217845560405192169182907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a3505050565b5f611ba08260601c90565b90506001600160a01b038116611bc95760405163867915ab60e01b815260040160405180910390fd5b6001600160a01b0381165f90815260018401602052604090205415611c01576040516374603e9560e11b815260040160405180910390fd5b5f611c0d836001611f60565b9050611c198482611ff0565b50816001600160a01b03167f5ffb06b0b0e8ad6a8f3c5831d499dfa612d9c9d4dc107bbd66f18f61a6492e718260405161115791815260200190565b5f5f611c62836002611936565b90925090505f5b6002811015611dbe5782515f90849083908110611c8857611c88613396565b60200260200101516001600160e01b03191614611db6575f611cc387858481518110611cb657611cb6613396565b602002602001015161251d565b9050828281518110611cd757611cd7613396565b6020026020010151896003015f8381526020019081526020015f205f611cfd8b8b612073565b815260208101919091526040015f20805460ff19166001836002811115611d2657611d2661316c565b0217905550856001600160a01b0316876001600160a01b0316896001600160a01b03167fa3df710420b01cc30ff300309abbc7fadd4630d4ab385b0f5a126fb4babe762b878681518110611d7c57611d7c613396565b6020026020010151878781518110611d9657611d96613396565b6020026020010151604051611dac92919061353f565b60405180910390a4505b600101611c69565b5050505050505050565b6001600160a01b0381165f9081526001830160205260408120548190808203611df7575f5f9250925050610b90565b600185611e04828461352c565b81548110611e1457611e14613396565b905f5260205f2001549250925050610b90565b5f5f611e34836007611936565b90925090505f5b60078110156114b45782515f90849083908110611e5a57611e5a613396565b60200260200101516001600160e01b03191614611f58575f611e8887858481518110611cb657611cb6613396565b9050828281518110611e9c57611e9c613396565b6020908102919091018101515f83815260038b01835260408082208a835290935291909120805460ff19166001836002811115611edb57611edb61316c565b021790555085876001600160a01b03167ff2ffd4f09d58d06824188033d3318d06eb957bfb1a8ffed9af78e1f19168b904868581518110611f1e57611f1e613396565b6020026020010151868681518110611f3857611f38613396565b6020026020010151604051611f4e92919061353f565b60405180910390a3505b600101611e3b565b5f80806001846002811115611f7757611f7761316c565b03611f8f57506aff0000000000000000ffff19611fc9565b5f846002811115611fa257611fa261316c565b03611fba57506aff00ffffffffffffff000019611fc9565b506aff00ffffffffffffffffff195b80851691506050846002811115611fe257611fe261316c565b901b91909117949350505050565b5f61201f83611fff8460601c90565b6001600160a01b03165f9081526001919091016020526040902054151590565b61206c578254600181810185555f858152602081209092018490558454919085019061204b8560601c90565b6001600160a01b0316815260208101919091526040015f2055506001610e71565b505f610e71565b6040516001600160601b0319606084811b8216602084015283901b1660348201525f9060480160405160208183030381529060405280519060200120905092915050565b6001600160a01b0381165f90815260018301602052604081205480156121c3575f6120e360018361352c565b85549091505f906120f69060019061352c565b905081811461216b575f865f01828154811061211457612114613396565b905f5260205f200154905080875f01848154811061213457612134613396565b905f5260205f20018190555083876001015f6121508460601c90565b6001600160a01b0316815260208101919091526040015f2055505b855486908061217c5761217c61355d565b600190038181905f5260205f20015f90559055856001015f866001600160a01b03166001600160a01b031681526020019081526020015f205f905560019350505050610e71565b5f915050610e71565b6121d4612568565b610bc657604051631afcd79f60e31b815260040160405180910390fd5b5f5f5f60605f5f602487015190508060201461222057604051637ed1113760e01b815260040160405180910390fd5b60645b8751811015612284578088015160f81c96506001810188015160601c95506015810188015194506035810188015192506035810188019350612268898787878b61228f565b612273836055613571565b61227d9082613571565b9050612223565b505050505050505050565b8151158015906122a0575060048251105b156122be57604051632342609160e11b815260040160405180910390fd5b5f6122c98686612581565b90506122d68483836125e9565b5f6122e084613584565b90505f6122ef8551848461268a565b90505f8160038111156123045761230461316c565b0361232257604051635872303760e01b815260040160405180910390fd5b60038160038111156123365761233661316c565b03612343575050506124c1565b5f8061234e8561282d565b600281111561235f5761235f61316c565b0361238057612379896123728a8661251d565b8589612847565b90506123e4565b600161238b8561282d565b600281111561239c5761239c61316c565b036123b657612379896123af8a8661251d565b85896128d9565b60026123c18561282d565b60028111156123d2576123d261316c565b036123e4576123e18989612a02565b90505b60028160028111156123f8576123f861316c565b148061243057505f8160028111156124125761241261316c565b1480156124305750600182600381111561242e5761242e61316c565b145b1561244e5760405163864dd1e760e01b815260040160405180910390fd5b60018160028111156124625761246261316c565b148061249a57505f81600281111561247c5761247c61316c565b14801561249a575060028260038111156124985761249861316c565b145b156124a857505050506124c1565b6040516308d5a8b160e31b815260040160405180910390fd5b5050505050565b6124d182612a34565b6040516001600160a01b038316907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b905f90a280511561251557610df68282612a97565b610ac2612b09565b6040516001600160601b0319606084901b1660208201526001600160e01b0319821660348201525f90603801604051602081830303815290604052612561906135c2565b9392505050565b5f612571611263565b54600160401b900460ff16919050565b6001600160a01b0381165f9081526001830160205260408120548082036125bb57604051632d0519ad60e01b815260040160405180910390fd5b836125c760018361352c565b815481106125d7576125d7613396565b905f5260205f20015491505092915050565b60016125f482612b28565b60018111156126055761260561316c565b1461262357604051633bcd102b60e21b815260040160405180910390fd5b60018260018111156126375761263761316c565b03612655576040516306c4a1c760e11b815260040160405180910390fd5b5f8311801561266c575061266a816002612b42565b155b15610df6576040516309e9cd4960e01b815260040160405180910390fd5b5f5f61269584612b77565b90508415806126ac57506001600160e01b03198316155b806126c8575060038160038111156126c6576126c661316c565b145b806126e357505f8160038111156126e1576126e161316c565b145b156126ef579050612561565b5f63d2af4e7560e01b6001600160e01b031985160161271957612712855f612b91565b9050612804565b63ab5d120b60e01b6001600160e01b031985160161273c57612712856002612b91565b634259a0bb60e01b6001600160e01b031985160161275f57612712856003612b91565b639aeaeb4160e01b6001600160e01b031985160161278257612712856004612b91565b63f5413a7160e01b6001600160e01b03198516016127a557612712856005612b91565b63f6a1584d60e01b6001600160e01b03198516016127c857612712856007612b91565b633213221d60e11b6001600160e01b03198516016127eb57612712856008612b91565b6040516318f4c12360e11b815260040160405180910390fd5b5f8160048111156128175761281761316c565b0361282457509050612561565b610a9d81612be3565b5f60ff605083901c166002811115610e7157610e7161316c565b5f6001600160e01b0319831663095ea7b360e01b1480159061287a57506001600160e01b03198316634decdde360e11b14155b15612898576040516318f4c12360e11b815260040160405180910390fd5b5f6128a35f84612c3b565b90505f6128b03383612073565b5f8781526003890160209081526040808320938352929052205460ff1692505050949350505050565b5f5f6128e55f84612c3b565b90506001600160a01b038116331461291057604051636eb0315f60e01b815260040160405180910390fd5b5f63d2af4e7560e01b6001600160e01b031986160161293b57612934600185612c3b565b90506129da565b63ab5d120b60e01b6001600160e01b0319861601612973575f61295f600186612c3b565b905061296b8184612073565b9150506129da565b6001600160e01b0319851663bda65f4560e01b14806129a257506001600160e01b0319851663651514bf60e01b145b806129bd57506001600160e01b03198516630abec58f60e01b145b156127eb575f6129ce600186612c3b565b905061296b8382612073565b5f8681526003880160209081526040808320938352929052205460ff16915050949350505050565b5f5f612a0e3384612073565b5f8080526003860160209081526040808320938352929052205460ff1691505092915050565b806001600160a01b03163b5f03612a6957604051634c9c8ce360e01b81526001600160a01b0382166004820152602401610fba565b5f5160206136185f395f51905f5280546001600160a01b0319166001600160a01b0392909216919091179055565b60605f5f846001600160a01b031684604051612ab391906135e8565b5f60405180830381855af49150503d805f8114612aeb576040519150601f19603f3d011682016040523d82523d5f602084013e612af0565b606091505b5091509150612b00858383612ca4565b95945050505050565b3415610bc65760405163b398979f60e01b815260040160405180910390fd5b5f60ff605883901c166001811115610e7157610e7161316c565b5f816002811115612b5557612b5561316c565b612b5e8461282d565b6002811115612b6f57612b6f61316c565b149392505050565b5f60ff604883901c166003811115610e7157610e7161316c565b5f60098210612bb35760405163b44af9af60e01b815260040160405180910390fd5b5f612bbf836008613515565b612bca9060b8613571565b905083811b60f81c60048111156115955761159561316c565b5f5f826004811115612bf757612bf761316c565b90508060ff165f03612c1c5760405163d8455a1360e01b815260040160405180910390fd5b612c276001826135fe565b60ff1660038111156125615761256161316c565b5f612c47836020613515565b612c52906004613571565b612c5d906020613571565b82511015612c7e57604051631d098e2d60e21b815260040160405180910390fd5b5f612c8a846020613515565b612c95906004613571565b92909201602001519392505050565b606082612cb957612cb482612d00565b612561565b8151158015612cd057506001600160a01b0384163b155b15612cf957604051639996b31560e01b81526001600160a01b0385166004820152602401610fba565b5080612561565b805115612d0f57805160208201fd5b60405163d6bda27560e01b815260040160405180910390fd5b6001600160a01b038116811461081d575f5ffd5b5f60208284031215612d4c575f5ffd5b813561256181612d28565b5f5f60208385031215612d68575f5ffd5b82356001600160401b03811115612d7d575f5ffd5b8301601f81018513612d8d575f5ffd5b80356001600160401b03811115612da2575f5ffd5b8560208260051b8401011115612db6575f5ffd5b6020919091019590945092505050565b634e487b7160e01b5f52604160045260245ffd5b604051601f8201601f191681016001600160401b0381118282101715612e0257612e02612dc6565b604052919050565b5f6001600160401b03821115612e2257612e22612dc6565b50601f01601f191660200190565b5f82601f830112612e3f575f5ffd5b8135612e52612e4d82612e0a565b612dda565b818152846020838601011115612e66575f5ffd5b816020850160208301375f918101602001919091529392505050565b5f60208284031215612e92575f5ffd5b81356001600160401b03811115612ea7575f5ffd5b61159584828501612e30565b5f5f5f5f5f60808688031215612ec7575f5ffd5b8535612ed281612d28565b94506020860135935060408601356001600160401b03811115612ef3575f5ffd5b8601601f81018813612f03575f5ffd5b80356001600160401b03811115612f18575f5ffd5b886020828401011115612f29575f5ffd5b60209190910193509150606086013560028110612f44575f5ffd5b809150509295509295909350565b5f5f60408385031215612f63575f5ffd5b8235612f6e81612d28565b915060208301356001600160401b03811115612f88575f5ffd5b612f9485828601612e30565b9150509250929050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b8215158152604060208201525f6115956040830184612f9e565b5f6001600160401b03821115612ffe57612ffe612dc6565b5060051b60200190565b803560038110613016575f5ffd5b919050565b5f82601f83011261302a575f5ffd5b8135613038612e4d82612fe6565b8082825260208201915060208360051b860101925085831115613059575f5ffd5b602085015b8381101561307d5761306f81613008565b83526020928301920161305e565b5095945050505050565b5f5f60408385031215613098575f5ffd5b82356001600160401b038111156130ad575f5ffd5b8301601f810185136130bd575f5ffd5b80356130cb612e4d82612fe6565b8082825260208201915060208360051b8501019250878311156130ec575f5ffd5b6020840193505b828410156131235783356001600160e01b031981168114613112575f5ffd5b8252602093840193909101906130f3565b945050505060208301356001600160401b03811115613140575f5ffd5b612f948582860161301b565b5f5f6040838503121561315d575f5ffd5b50508035926020909101359150565b634e487b7160e01b5f52602160045260245ffd5b600381106131905761319061316c565b9052565b604080825283519082018190525f9060208501906060840190835b818110156131d75783516001600160e01b0319168352602093840193909201916001016131af565b5050838103602080860191909152855180835291810192508501905f5b8181101561321a57613207848451613180565b60209384019392909201916001016131f4565b50919695505050505050565b602080825282518282018190525f918401906040840190835b8181101561325d57835183526020938401939092019160010161323f565b509095945050505050565b5f60208284031215613278575f5ffd5b5035919050565b602081525f6125616020830184612f9e565b5f5f5f5f608085870312156132a4575f5ffd5b84356132af81612d28565b935060208501356132bf81612d28565b925060408501356132cf81612d28565b9396929550929360600135925050565b5f5f5f606084860312156132f1575f5ffd5b83356132fc81612d28565b9250602084013561330c81612d28565b915061331a60408501613008565b90509250925092565b60208101610e718284613180565b5f5f5f60608486031215613343575f5ffd5b833561334e81612d28565b95602085013595506040909401359392505050565b634e487b7160e01b5f52601160045260245ffd5b5f8261339157634e487b7160e01b5f52601260045260245ffd5b500490565b634e487b7160e01b5f52603260045260245ffd5b5f5f5f5f608085870312156133bd575f5ffd5b84516133c881612d28565b60208601519094506133d981612d28565b6040860151606090960151949790965092505050565b5f602082840312156133ff575f5ffd5b815161256181612d28565b60018060a01b0385168152836020820152608060408201525f6134306080830185612f9e565b9050600283106134425761344261316c565b82606083015295945050505050565b80518015158114613016575f5ffd5b5f60208284031215613470575f5ffd5b61256182613451565b5f60208284031215613489575f5ffd5b5051919050565b5f5f604083850312156134a1575f5ffd5b6134aa83613451565b915060208301516001600160401b038111156134c4575f5ffd5b8301601f810185136134d4575f5ffd5b80516134e2612e4d82612e0a565b8181528660208385010111156134f6575f5ffd5b8160208401602083015e5f602083830101528093505050509250929050565b8082028115828204841417610e7157610e71613363565b81810381811115610e7157610e71613363565b6001600160e01b031983168152604081016125616020830184613180565b634e487b7160e01b5f52603160045260245ffd5b80820180821115610e7157610e71613363565b805160208201516001600160e01b03198116919060048210156135bb576001600160e01b0319600483900360031b81901b82161692505b5050919050565b805160208083015191908110156135e2575f198160200360031b1b821691505b50919050565b5f82518060208501845e5f920191825250919050565b60ff8281168282160390811115610e7157610e7161336356fe360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbca2646970667358221220a97aea9451fac0ce37f9160a64a84553f0e44d4a616ac90ec531b52f5b9e2c9c64736f6c634300081e0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R0`\x80R4\x80\x15a\0\x13W__\xFD[Pa\0\x1Ca\0!V[a\0\xD3V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80Th\x01\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a\0qW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80T`\x01`\x01`@\x1B\x03\x90\x81\x16\x14a\0\xD0W\x80T`\x01`\x01`@\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x90\x81\x17\x82U`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PV[`\x80Qa6ma\0\xF9_9_\x81\x81a\x15\xA8\x01R\x81\x81a\x15\xD1\x01Ra\x17\xF8\x01Ra6m_\xF3\xFE`\x80`@R`\x046\x10a\x01\xDBW_5`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0\xFDW\x80c\xC6\x8C:\x83\x11a\0\x92W\x80c\xDF\x96 \xEB\x11a\0bW\x80c\xDF\x96 \xEB\x14a\x05\xBCW\x80c\xF2\xFD\xE3\x8B\x14a\x05\xCFW\x80c\xFA\x19P\x1D\x14a\x05\xEEW\x80c\xFF\xA1\xADt\x14a\x06\rW__\xFD[\x80c\xC6\x8C:\x83\x14a\x05\x1CW\x80c\xDC\x06\x10\x9D\x14a\x05;W\x80c\xDCDjJ\x14a\x05ZW\x80c\xDFNo\x8A\x14a\x05\x86W__\xFD[\x80c\xAD<\xB1\xCC\x11a\0\xCDW\x80c\xAD<\xB1\xCC\x14a\x04\x82W\x80c\xB2\xB9\x9E\xC9\x14a\x04\xBFW\x80c\xB5sib\x14a\x04\xDEW\x80c\xC6\x86\x05\xC8\x14a\x04\xFDW__\xFD[\x80c\x8D\xA5\xCB[\x14a\x04\x1DW\x80c\x9D\x95\xF1\xCC\x14a\x041W\x80c\xA2E\x0F\x89\x14a\x04DW\x80c\xA7l\x9A/\x14a\x04cW__\xFD[\x80cR)\x07?\x11a\x01sW\x80cc\xFE;V\x11a\x01CW\x80cc\xFE;V\x14a\x03\xAAW\x80cqP\x18\xA6\x14a\x03\xCBW\x80cs\x9CK\x08\x14a\x03\xDFW\x80c\x8B\x95\xEC\xCD\x14a\x03\xFEW__\xFD[\x80cR)\x07?\x14a\x02\xFAW\x80cR\xD1\x90-\x14a\x03'W\x80cV\xF5Q\x17\x14a\x03IW\x80c`\x97lK\x14a\x03}W__\xFD[\x80cC\x9F\xAB\x91\x11a\x01\xAEW\x80cC\x9F\xAB\x91\x14a\x02\x95W\x80cF\x87!\xA7\x14a\x02\xB4W\x80cJ\x1B\xA4\x08\x14a\x02\xD3W\x80cO\x1E\xF2\x86\x14a\x02\xE7W__\xFD[\x80c\x01u\x01R\x14a\x01\xDFW\x80c\x11\r\xCE\xE7\x14a\x02+W\x80c)D\x02\xCC\x14a\x02@W\x80c4\x01\xCD\xE8\x14a\x02vW[__\xFD[4\x80\x15a\x01\xEAW__\xFD[Pa\x02\x16a\x01\xF96`\x04a-<V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x03` R`@\x90 T`\xFF\x16\x90V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02>a\x0296`\x04a-WV[a\x06=V[\0[4\x80\x15a\x02KW__\xFD[P_Ta\x02^\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\"V[4\x80\x15a\x02\x81W__\xFD[Pa\x02>a\x02\x906`\x04a-<V[a\x08\nV[4\x80\x15a\x02\xA0W__\xFD[Pa\x02>a\x02\xAF6`\x04a.\x82V[a\x08 V[4\x80\x15a\x02\xBFW__\xFD[Pa\x02\x16a\x02\xCE6`\x04a.\xB3V[a\n\x10V[4\x80\x15a\x02\xDEW__\xFD[Pa\x02\x16`\x01\x81V[a\x02>a\x02\xF56`\x04a/RV[a\n\xA7V[4\x80\x15a\x03\x05W__\xFD[Pa\x03\x19a\x03\x146`\x04a.\xB3V[a\n\xC6V[`@Qa\x02\"\x92\x91\x90a/\xCCV[4\x80\x15a\x032W__\xFD[Pa\x03;a\x0BdV[`@Q\x90\x81R` \x01a\x02\"V[4\x80\x15a\x03TW__\xFD[Pa\x03ha\x03c6`\x04a0\x87V[a\x0B\x7FV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02\"V[4\x80\x15a\x03\x88W__\xFD[Pa\x03\x9Ca\x03\x976`\x04a1LV[a\x0B\x97V[`@Qa\x02\"\x92\x91\x90a1\x94V[4\x80\x15a\x03\xB5W__\xFD[Pa\x03\xBEa\x0B\xA4V[`@Qa\x02\"\x91\x90a2&V[4\x80\x15a\x03\xD6W__\xFD[Pa\x02>a\x0B\xB5V[4\x80\x15a\x03\xEAW__\xFD[Pa\x02>a\x03\xF96`\x04a2hV[a\x0B\xC8V[4\x80\x15a\x04\tW__\xFD[Pa\x02>a\x04\x186`\x04a-<V[a\x0B\xDBV[4\x80\x15a\x04(W__\xFD[Pa\x02^a\x0C*V[a\x02>a\x04?6`\x04a-<V[a\x0CXV[4\x80\x15a\x04OW__\xFD[Pa\x02>a\x04^6`\x04a2hV[a\x0C\xE2V[4\x80\x15a\x04nW__\xFD[Pa\x02>a\x04}6`\x04a2hV[a\x0C\xF3V[4\x80\x15a\x04\x8DW__\xFD[Pa\x04\xB2`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[`@Qa\x02\"\x91\x90a2\x7FV[4\x80\x15a\x04\xCAW__\xFD[Pa\x02>a\x04\xD96`\x04a-<V[a\r\x06V[4\x80\x15a\x04\xE9W__\xFD[Pa\x02>a\x04\xF86`\x04a2hV[a\r\x8EV[4\x80\x15a\x05\x08W__\xFD[Pa\x02>a\x05\x176`\x04a2\x91V[a\r\xC5V[4\x80\x15a\x05'W__\xFD[Pa\x02>a\x0566`\x04a2\xDFV[a\r\xE1V[4\x80\x15a\x05FW__\xFD[Pa\x02>a\x05U6`\x04a2hV[a\r\xFBV[4\x80\x15a\x05eW__\xFD[Pa\x05ya\x05t6`\x04a1LV[a\x0ETV[`@Qa\x02\"\x91\x90a3#V[4\x80\x15a\x05\x91W__\xFD[Pa\x05\xA5a\x05\xA06`\x04a-<V[a\x0EwV[`@\x80Q\x92\x15\x15\x83R` \x83\x01\x91\x90\x91R\x01a\x02\"V[a\x02>a\x05\xCA6`\x04a-WV[a\x0E\x8DV[4\x80\x15a\x05\xDAW__\xFD[Pa\x02>a\x05\xE96`\x04a-<V[a\x0F\x8DV[4\x80\x15a\x05\xF9W__\xFD[Pa\x02>a\x06\x086`\x04a31V[a\x0F\xCCV[4\x80\x15a\x06\x18W__\xFD[Pa\x04\xB2`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03\"\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[a\x06Ea\x0F\xE1V[\x804_\x82\x90\x03a\x06hW`@Qc\x0Cq\xEC\x17`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x06s\x83\x83a3wV[\x90P_[\x83\x81\x10\x15a\x08\x02Wa\x06\xAE\x86\x86\x83\x81\x81\x10a\x06\x94Wa\x06\x94a3\x96V[\x90P` \x02\x01` \x81\x01\x90a\x06\xA9\x91\x90a-<V[a\x10\x13V[_``\x87\x87\x84\x81\x81\x10a\x06\xC3Wa\x06\xC3a3\x96V[\x90P` \x02\x01` \x81\x01\x90a\x06\xD8\x91\x90a-<V[`\x01`\x01`\xA0\x1B\x03\x16\x90\x1Bk\x01\x02\x03\0\0\0\0\0\0\0\0\0\x17\x90Pa\x06\xFE`\x01\x82a\x10\x97V[a\x07X`\x01\x88\x88\x85\x81\x81\x10a\x07\x15Wa\x07\x15a3\x96V[\x90P` \x02\x01` \x81\x01\x90a\x07*\x91\x90a-<V[\x89\x89\x86\x81\x81\x10a\x07<Wa\x07<a3\x96V[\x90P` \x02\x01` \x81\x01\x90a\x07Q\x91\x90a-<V[`\x01a\x11eV[\x82\x15a\x07\xF9W_\x87\x87\x84\x81\x81\x10a\x07qWa\x07qa3\x96V[\x90P` \x02\x01` \x81\x01\x90a\x07\x86\x91\x90a-<V[`\x01`\x01`\xA0\x1B\x03\x16\x84_\x90`@Q_`@Q\x80\x83\x03\x81\x85\x88\x88\xF1\x93PPPP=\x80_\x81\x14a\x07\xD0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x07\xD5V[``\x91P[PP\x90P\x80a\x07\xF7W`@Qc\x9As\xEE\x99`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P[P`\x01\x01a\x06wV[PPPPPPV[a\x08\x12a\x0F\xE1V[a\x08\x1D`\x01\x82a\x11\xFFV[PV[_a\x08)a\x12cV[\x80T\x90\x91P`\xFF`\x01`@\x1B\x82\x04\x16\x15\x90`\x01`\x01`@\x1B\x03\x16_\x81\x15\x80\x15a\x08OWP\x82[\x90P_\x82`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\x08jWP0;\x15[\x90P\x81\x15\x80\x15a\x08xWP\x80\x15[\x15a\x08\x96W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x17\x85U\x83\x15a\x08\xC0W\x84T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x85U[____\x89\x80` \x01\x90Q\x81\x01\x90a\x08\xD8\x91\x90a3\xAAV[\x92\x96P\x90\x94P\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x84\x16\x15\x80a\x08\xFFWP`\x01`\x01`\xA0\x1B\x03\x83\x16\x15[\x15a\t\x1DW`@Qc\x86y\x15\xAB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x03a\tOW`@QcY\x8A\x0E!`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x17\x90U\x81\x15a\tzWa\tz`\x01\x83a\x12\x8BV[a\t\x83\x81a\x13JV[a\t\x8C\x84a\x14\x0CV[`@Q`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x7F_\xE6\xAA\xBFNy\x08C\xDFC\xAE\x0E\"\xB5\x86 \x06o\xB3\x89)[\xED\xC0j\x92\xDFl;(w}\x90_\x90\xA2PPPP\x83\x15a\x08\x02W\x84T`\xFF`@\x1B\x19\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPPV[3_\x90\x81R`\x03` R`@\x81 T`\xFF\x16a\n?W`@Qc\x1F\xB1\xD3\xE5`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_Ta\n[\x90`\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x88\x88\x88\x88\x88a\x14\x14V[a\n\x9D\x86\x86\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x88\x92Pa\x14\xBD\x91PPV[\x96\x95PPPPPPV[a\n\xAFa\x15\x9DV[a\n\xB8\x82a\x16AV[a\n\xC2\x82\x82a\x16IV[PPV[3_\x90\x81R`\x03` R`@\x81 T``\x90`\xFF\x16a\n\xF8W`@Qc\x1F\xB1\xD3\xE5`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_Ta\x0B\x14\x90`\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x89\x89\x89\x89\x89a\x14\x14V[a\x0BV\x87\x87\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x89\x92Pa\x17\x05\x91PPV[\x91P\x91P\x95P\x95\x93PPPPV[_a\x0Bma\x17\xEDV[P_Q` a6\x18_9_Q\x90_R\x90V[__a\x0B\x8B\x84\x84a\x186V[\x91P\x91P[\x92P\x92\x90PV[``\x80a\x0B\x8B\x84\x84a\x196V[``a\x0B\xB0`\x01a\x1A\xCCV[\x90P\x90V[a\x0B\xBDa\x0F\xE1V[a\x0B\xC6_a\x1B%V[V[a\x0B\xD0a\x0F\xE1V[a\x08\x1D`\x01\x82a\x1B\x95V[a\x0B\xE3a\x0F\xE1V[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x82U`@Q\x90\x91\x7F_\xE6\xAA\xBFNy\x08C\xDFC\xAE\x0E\"\xB5\x86 \x06o\xB3\x89)[\xED\xC0j\x92\xDFl;(w}\x91\xA2PV[\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T`\x01`\x01`\xA0\x1B\x03\x16\x90V[a\x0C`a\x0F\xE1V[a\x0Ci\x81a\x10\x13V[4\x15a\x08\x1DW_\x81`\x01`\x01`\xA0\x1B\x03\x164_\x90`@Q_`@Q\x80\x83\x03\x81\x85\x88\x88\xF1\x93PPPP=\x80_\x81\x14a\x0C\xBBW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x0C\xC0V[``\x91P[PP\x90P\x80a\n\xC2W`@Qc\x9As\xEE\x99`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0C\xEAa\x0F\xE1V[a\x08\x1D\x81a\x13JV[a\x0C\xFBa\x0F\xE1V[a\x08\x1D`\x01\x82a\x12\x8BV[a\r\x0Ea\x0F\xE1V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x03` R`@\x90 T`\xFF\x16a\rFW`@Qc\x1F\xB1\xD3\xE5`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x81\x81R`\x03` R`@\x80\x82 \x80T`\xFF\x19\x16\x90UQ\x7F\xCF\xC2Af\xDBK\xB6w\xE8W\xCA\xCA\xBD\x15A\xFB+0dP!\xB2|Q0A\x95\x89\xB8M\xB5+\x91\x90\xA2PV[a\r\x96a\x0F\xE1V[_a\r\xA1\x82``\x1C\x90V[\x90Pa\r\xAC\x81a\x10\x13V[a\r\xB7`\x01\x83a\x10\x97V[a\n\xC2`\x01\x82\x83`\x01a\x11eV[a\r\xCDa\x0F\xE1V[a\r\xDB`\x01\x85\x85\x85\x85a\x1CUV[PPPPV[a\r\xE9a\x0F\xE1V[a\r\xF6`\x01\x84\x84\x84a\x11eV[PPPV[a\x0E\x03a\x0F\xE1V[_a\x0E\x0E\x82``\x1C\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x03` R`@\x90 T\x90\x91P`\xFF\x16a\x0EIW`@Qc\x1F\xB1\xD3\xE5`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\n\xC2`\x01\x83a\x10\x97V[_\x82\x81R`\x04` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 T`\xFF\x16[\x92\x91PPV[_\x80a\x0E\x84`\x01\x84a\x1D\xC8V[\x91P\x91P\x91P\x91V[a\x0E\x95a\x0F\xE1V[\x804_\x82\x90\x03a\x0E\xB8W`@Qc\x0Cq\xEC\x17`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0E\xC3\x83\x83a3wV[\x90P_[\x83\x81\x10\x15a\x08\x02Wa\x0E\xE4\x86\x86\x83\x81\x81\x10a\x06\x94Wa\x06\x94a3\x96V[\x81\x15a\x0F\x85W_\x86\x86\x83\x81\x81\x10a\x0E\xFDWa\x0E\xFDa3\x96V[\x90P` \x02\x01` \x81\x01\x90a\x0F\x12\x91\x90a-<V[`\x01`\x01`\xA0\x1B\x03\x16\x83_\x90`@Q_`@Q\x80\x83\x03\x81\x85\x88\x88\xF1\x93PPPP=\x80_\x81\x14a\x0F\\W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x0FaV[``\x91P[PP\x90P\x80a\x0F\x83W`@Qc\x9As\xEE\x99`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P[`\x01\x01a\x0E\xC7V[a\x0F\x95a\x0F\xE1V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0F\xC3W`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R_`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[a\x08\x1D\x81a\x1B%V[a\x0F\xD4a\x0F\xE1V[a\r\xF6`\x01\x84\x84\x84a\x1E'V[3a\x0F\xEAa\x0C*V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0B\xC6W`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x0F\xBAV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x03` R`@\x90 T`\xFF\x16\x15a\x10LW`@Qc8\xE8\x16\xA5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x81\x81R`\x03` R`@\x80\x82 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7F\xB2]\x03\xAA\xF3\x08\xD7)\x17\t\xBE\x1E\xA2\x8B\x80\x04c\xCF:\x9ALJUU\xD73:\x96L\x1D\xFE\xBD\x91\x90\xA2PV[_a\x10\xA2\x82``\x1C\x90V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x10\xCBW`@Qc\x86y\x15\xAB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x01\x84\x01` R`@\x90 T\x15a\x11\x03W`@Qct`>\x95`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x11\x0F\x83`\x02a\x1F`V[\x90Pa\x11\x1B\x84\x82a\x1F\xF0V[P\x81`\x01`\x01`\xA0\x1B\x03\x16\x7F\x1E\xE2y\x1F,\xAF\x0E\x92\xA9\xDC2\xA3z\x9E\xA5:\xB6\xACzo\xB8\xF2\xD0\x90\xE5:\x06}:C\xF6\xAC\x82`@Qa\x11W\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPPV[_\x80\x80R`\x03\x85\x01` R`@\x81 \x82\x91a\x11\x80\x86\x86a sV[\x81R` \x81\x01\x91\x90\x91R`@\x01_ \x80T`\xFF\x19\x16`\x01\x83`\x02\x81\x11\x15a\x11\xA9Wa\x11\xA9a1lV[\x02\x17\x90UP\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7Ft\x87S\r\xDF\xF1 y\x95\x05\xE5+\x1B\x19\xB6\x93?\x85\xA9\xEE\xAE\x92 \xC8\nz\xD7\xC4)\xB6\x12\xAE\x83`@Qa\x11\xF1\x91\x90a3#V[`@Q\x80\x91\x03\x90\xA3PPPPV[_a\x12\n\x83\x83a \xB7V[\x90P\x80\x15a\x12JW`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\r\xFC\xE1\xEAK\xA1\xEE\xBA\x89\x1F\xFB*\x06g\x90\xFB\xC2\x93\xA9\xE5\x17\xFEa\xD4\x9D\x15j0\x16_\x93\xF3\x90_\x90\xA2PPPV[`@QcJ\x89\x03!`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0a\x0EqV[_a\x12\x96\x82``\x1C\x90V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x12\xBFW`@Qc\x86y\x15\xAB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x01\x84\x01` R`@\x90 T\x15a\x12\xF7W`@Qct`>\x95`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x13\x02\x83\x82a\x1F`V[\x90Pa\x13\x0E\x84\x82a\x1F\xF0V[P\x81`\x01`\x01`\xA0\x1B\x03\x16\x7F\xAA\xF2k\xB1*\xA8\x9E\xE9k\xBE\x19fzj\x05W'\xB7]?n\xD7\xB8\xB6\x11\xEFe\x19\x18\x02\t\xD6\x82`@Qa\x11W\x91\x81R` \x01\x90V[_a\x13U\x82``\x1C\x90V[\x90P_\x81`\x01`\x01`\xA0\x1B\x03\x16c\x82\xBF\xEF\xC8`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\x94W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xB8\x91\x90a3\xEFV[\x90Pa\x13\xE3`\x01k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16``\x85\x90\x1B`\x01`\x01``\x1B\x03\x19\x16\x17a\x1B\x95V[a\r\xF6`\x01k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16``\x84\x90\x1B`\x01`\x01``\x1B\x03\x19\x16\x17a\x12\x8BV[a\x0F\x95a!\xCCV[\x84`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x03a\x14qWa\x14l\x87\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa!\xF1\x92PPPV[a\x14\xB4V[a\x14\xB4\x87\x86\x86\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x88\x92Pa\"\x8F\x91PPV[PPPPPPPV[_a\x14\xC6a\x0C*V[`\x01`\x01`\xA0\x1B\x03\x16cF\x87!\xA7\x86\x86\x86\x86`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\xF7\x94\x93\x92\x91\x90a4\nV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x15\x13W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x157\x91\x90a4`V[\x90P\x80\x15a\x15lW`@Q\x7FN.\x86\xD2\x13u\xEB\xCB\xF6\xE9=\xF5\xEB\xDDZ\x91[\xF80$Y\x04\xC3\xB5OH\xAD\xF0\x17\n\xAEK\x90_\x90\xA1a\x15\x95V[`@Q\x7F\xC2M\x93`\x8A\x03\xD2c\xFF\x19\x1Dvw\x14\x1F^\x94\xC4\x96\xE5\x93\x10\x8F:\xAE\x0C\xB5\xB7\x04\x94\xC4\xD3\x90_\x90\xA1[\x94\x93PPPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x16#WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x16\x17_Q` a6\x18_9_Q\x90_RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x0B\xC6W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08\x1Da\x0F\xE1V[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x16\xA3WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x16\xA0\x91\x81\x01\x90a4yV[`\x01[a\x16\xCBW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x0F\xBAV[_Q` a6\x18_9_Q\x90_R\x81\x14a\x16\xFBW`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x0F\xBAV[a\r\xF6\x83\x83a$\xC8V[_``a\x17\x10a\x0C*V[`\x01`\x01`\xA0\x1B\x03\x16cR)\x07?\x87\x87\x87\x87`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17A\x94\x93\x92\x91\x90a4\nV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x17\\W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x17\x83\x91\x90\x81\x01\x90a4\x90V[\x90\x92P\x90P\x81\x15a\x17\xBBW`@Q\x7FN.\x86\xD2\x13u\xEB\xCB\xF6\xE9=\xF5\xEB\xDDZ\x91[\xF80$Y\x04\xC3\xB5OH\xAD\xF0\x17\n\xAEK\x90_\x90\xA1a\x17\xE4V[`@Q\x7F\xC2M\x93`\x8A\x03\xD2c\xFF\x19\x1Dvw\x14\x1F^\x94\xC4\x96\xE5\x93\x10\x8F:\xAE\x0C\xB5\xB7\x04\x94\xC4\xD3\x90_\x90\xA1[\x94P\x94\x92PPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0B\xC6W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81Q_\x90\x81\x90`\x07\x81\x11\x15a\x18^W`@Qc\x17\xA4\xD9\x87`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83Q\x85Q\x14a\x18\x80W`@Qct\xF4\xD57`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80[\x82\x81\x10\x15a\x18\xDBWa\x18\x96\x81` a5\x15V[a\x18\xA1\x90`\xE0a5,V[`\xE0\x88\x83\x81Q\x81\x10a\x18\xB5Wa\x18\xB5a3\x96V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x90\x1C\x90\x1B\x91\x90\x91\x17\x90`\x01\x01a\x18\x83V[P_[\x82\x81\x10\x15a\x19+Wa\x18\xF1\x81`\x02a5\x15V[\x86\x82\x81Q\x81\x10a\x19\x03Wa\x19\x03a3\x96V[` \x02` \x01\x01Q`\x02\x81\x11\x15a\x19\x1CWa\x19\x1Ca1lV[\x90\x1B\x91\x90\x91\x17\x90`\x01\x01a\x18\xDEV[P\x95\x90\x94P\x92PPPV[``\x80`\x07\x83\x11\x15a\x19[W`@Qc\x17\xA4\xD9\x87`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19sWa\x19sa-\xC6V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x19\x9CW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\xB7Wa\x19\xB7a-\xC6V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x19\xE0W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83\x81\x10\x15a\x1A=W`\xE0a\x19\xFA\x82` a5\x15V[a\x1A\x05\x90`\xE0a5,V[\x86\x90\x1C\x90\x1B\x83\x82\x81Q\x81\x10a\x1A\x1CWa\x1A\x1Ca3\x96V[`\x01`\x01`\xE0\x1B\x03\x19\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x19\xE5V[P_[\x83\x81\x10\x15a\x1A\xC4W`\xFEa\x1AU\x82`\x02a5\x15V[a\x1A`\x90`\xFEa5,V[\x86_\x1C\x90\x1B\x90\x1C`\xFF\x16`\x02\x81\x11\x15a\x1A{Wa\x1A{a1lV[\x82\x82\x81Q\x81\x10a\x1A\x8DWa\x1A\x8Da3\x96V[` \x02` \x01\x01\x90`\x02\x81\x11\x15a\x1A\xA6Wa\x1A\xA6a1lV[\x90\x81`\x02\x81\x11\x15a\x1A\xB9Wa\x1A\xB9a1lV[\x90RP`\x01\x01a\x1A@V[P\x92P\x92\x90PV[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x1B\x19W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x1B\x05W[PPPPP\x90P\x91\x90PV[\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x82\x17\x84U`@Q\x92\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPPV[_a\x1B\xA0\x82``\x1C\x90V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1B\xC9W`@Qc\x86y\x15\xAB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x01\x84\x01` R`@\x90 T\x15a\x1C\x01W`@Qct`>\x95`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x1C\r\x83`\x01a\x1F`V[\x90Pa\x1C\x19\x84\x82a\x1F\xF0V[P\x81`\x01`\x01`\xA0\x1B\x03\x16\x7F_\xFB\x06\xB0\xB0\xE8\xADj\x8F<X1\xD4\x99\xDF\xA6\x12\xD9\xC9\xD4\xDC\x10{\xBDf\xF1\x8Fa\xA6I.q\x82`@Qa\x11W\x91\x81R` \x01\x90V[__a\x1Cb\x83`\x02a\x196V[\x90\x92P\x90P_[`\x02\x81\x10\x15a\x1D\xBEW\x82Q_\x90\x84\x90\x83\x90\x81\x10a\x1C\x88Wa\x1C\x88a3\x96V[` \x02` \x01\x01Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\x1D\xB6W_a\x1C\xC3\x87\x85\x84\x81Q\x81\x10a\x1C\xB6Wa\x1C\xB6a3\x96V[` \x02` \x01\x01Qa%\x1DV[\x90P\x82\x82\x81Q\x81\x10a\x1C\xD7Wa\x1C\xD7a3\x96V[` \x02` \x01\x01Q\x89`\x03\x01_\x83\x81R` \x01\x90\x81R` \x01_ _a\x1C\xFD\x8B\x8Ba sV[\x81R` \x81\x01\x91\x90\x91R`@\x01_ \x80T`\xFF\x19\x16`\x01\x83`\x02\x81\x11\x15a\x1D&Wa\x1D&a1lV[\x02\x17\x90UP\x85`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x89`\x01`\x01`\xA0\x1B\x03\x16\x7F\xA3\xDFq\x04 \xB0\x1C\xC3\x0F\xF3\x000\x9A\xBB\xC7\xFA\xDDF0\xD4\xAB8[\x0FZ\x12o\xB4\xBA\xBEv+\x87\x86\x81Q\x81\x10a\x1D|Wa\x1D|a3\x96V[` \x02` \x01\x01Q\x87\x87\x81Q\x81\x10a\x1D\x96Wa\x1D\x96a3\x96V[` \x02` \x01\x01Q`@Qa\x1D\xAC\x92\x91\x90a5?V[`@Q\x80\x91\x03\x90\xA4P[`\x01\x01a\x1CiV[PPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x01\x83\x01` R`@\x81 T\x81\x90\x80\x82\x03a\x1D\xF7W__\x92P\x92PPa\x0B\x90V[`\x01\x85a\x1E\x04\x82\x84a5,V[\x81T\x81\x10a\x1E\x14Wa\x1E\x14a3\x96V[\x90_R` _ \x01T\x92P\x92PPa\x0B\x90V[__a\x1E4\x83`\x07a\x196V[\x90\x92P\x90P_[`\x07\x81\x10\x15a\x14\xB4W\x82Q_\x90\x84\x90\x83\x90\x81\x10a\x1EZWa\x1EZa3\x96V[` \x02` \x01\x01Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\x1FXW_a\x1E\x88\x87\x85\x84\x81Q\x81\x10a\x1C\xB6Wa\x1C\xB6a3\x96V[\x90P\x82\x82\x81Q\x81\x10a\x1E\x9CWa\x1E\x9Ca3\x96V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q_\x83\x81R`\x03\x8B\x01\x83R`@\x80\x82 \x8A\x83R\x90\x93R\x91\x90\x91 \x80T`\xFF\x19\x16`\x01\x83`\x02\x81\x11\x15a\x1E\xDBWa\x1E\xDBa1lV[\x02\x17\x90UP\x85\x87`\x01`\x01`\xA0\x1B\x03\x16\x7F\xF2\xFF\xD4\xF0\x9DX\xD0h$\x18\x803\xD31\x8D\x06\xEB\x95{\xFB\x1A\x8F\xFE\xD9\xAFx\xE1\xF1\x91h\xB9\x04\x86\x85\x81Q\x81\x10a\x1F\x1EWa\x1F\x1Ea3\x96V[` \x02` \x01\x01Q\x86\x86\x81Q\x81\x10a\x1F8Wa\x1F8a3\x96V[` \x02` \x01\x01Q`@Qa\x1FN\x92\x91\x90a5?V[`@Q\x80\x91\x03\x90\xA3P[`\x01\x01a\x1E;V[_\x80\x80`\x01\x84`\x02\x81\x11\x15a\x1FwWa\x1Fwa1lV[\x03a\x1F\x8FWPj\xFF\0\0\0\0\0\0\0\0\xFF\xFF\x19a\x1F\xC9V[_\x84`\x02\x81\x11\x15a\x1F\xA2Wa\x1F\xA2a1lV[\x03a\x1F\xBAWPj\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x19a\x1F\xC9V[Pj\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19[\x80\x85\x16\x91P`P\x84`\x02\x81\x11\x15a\x1F\xE2Wa\x1F\xE2a1lV[\x90\x1B\x91\x90\x91\x17\x94\x93PPPPV[_a \x1F\x83a\x1F\xFF\x84``\x1C\x90V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x01\x91\x90\x91\x01` R`@\x90 T\x15\x15\x90V[a lW\x82T`\x01\x81\x81\x01\x85U_\x85\x81R` \x81 \x90\x92\x01\x84\x90U\x84T\x91\x90\x85\x01\x90a K\x85``\x1C\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_ UP`\x01a\x0EqV[P_a\x0EqV[`@Q`\x01`\x01``\x1B\x03\x19``\x84\x81\x1B\x82\x16` \x84\x01R\x83\x90\x1B\x16`4\x82\x01R_\x90`H\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a!\xC3W_a \xE3`\x01\x83a5,V[\x85T\x90\x91P_\x90a \xF6\x90`\x01\x90a5,V[\x90P\x81\x81\x14a!kW_\x86_\x01\x82\x81T\x81\x10a!\x14Wa!\x14a3\x96V[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10a!4Wa!4a3\x96V[\x90_R` _ \x01\x81\x90UP\x83\x87`\x01\x01_a!P\x84``\x1C\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_ UP[\x85T\x86\x90\x80a!|Wa!|a5]V[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa\x0EqV[_\x91PPa\x0EqV[a!\xD4a%hV[a\x0B\xC6W`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[___``__`$\x87\x01Q\x90P\x80` \x14a\" W`@Qc~\xD1\x117`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`d[\x87Q\x81\x10\x15a\"\x84W\x80\x88\x01Q`\xF8\x1C\x96P`\x01\x81\x01\x88\x01Q``\x1C\x95P`\x15\x81\x01\x88\x01Q\x94P`5\x81\x01\x88\x01Q\x92P`5\x81\x01\x88\x01\x93Pa\"h\x89\x87\x87\x87\x8Ba\"\x8FV[a\"s\x83`Ua5qV[a\"}\x90\x82a5qV[\x90Pa\"#V[PPPPPPPPPV[\x81Q\x15\x80\x15\x90a\"\xA0WP`\x04\x82Q\x10[\x15a\"\xBEW`@Qc#B`\x91`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\"\xC9\x86\x86a%\x81V[\x90Pa\"\xD6\x84\x83\x83a%\xE9V[_a\"\xE0\x84a5\x84V[\x90P_a\"\xEF\x85Q\x84\x84a&\x8AV[\x90P_\x81`\x03\x81\x11\x15a#\x04Wa#\x04a1lV[\x03a#\"W`@QcXr07`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03\x81`\x03\x81\x11\x15a#6Wa#6a1lV[\x03a#CWPPPa$\xC1V[_\x80a#N\x85a(-V[`\x02\x81\x11\x15a#_Wa#_a1lV[\x03a#\x80Wa#y\x89a#r\x8A\x86a%\x1DV[\x85\x89a(GV[\x90Pa#\xE4V[`\x01a#\x8B\x85a(-V[`\x02\x81\x11\x15a#\x9CWa#\x9Ca1lV[\x03a#\xB6Wa#y\x89a#\xAF\x8A\x86a%\x1DV[\x85\x89a(\xD9V[`\x02a#\xC1\x85a(-V[`\x02\x81\x11\x15a#\xD2Wa#\xD2a1lV[\x03a#\xE4Wa#\xE1\x89\x89a*\x02V[\x90P[`\x02\x81`\x02\x81\x11\x15a#\xF8Wa#\xF8a1lV[\x14\x80a$0WP_\x81`\x02\x81\x11\x15a$\x12Wa$\x12a1lV[\x14\x80\x15a$0WP`\x01\x82`\x03\x81\x11\x15a$.Wa$.a1lV[\x14[\x15a$NW`@Qc\x86M\xD1\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x81`\x02\x81\x11\x15a$bWa$ba1lV[\x14\x80a$\x9AWP_\x81`\x02\x81\x11\x15a$|Wa$|a1lV[\x14\x80\x15a$\x9AWP`\x02\x82`\x03\x81\x11\x15a$\x98Wa$\x98a1lV[\x14[\x15a$\xA8WPPPPa$\xC1V[`@Qc\x08\xD5\xA8\xB1`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPV[a$\xD1\x82a*4V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15a%\x15Wa\r\xF6\x82\x82a*\x97V[a\n\xC2a+\tV[`@Q`\x01`\x01``\x1B\x03\x19``\x84\x90\x1B\x16` \x82\x01R`\x01`\x01`\xE0\x1B\x03\x19\x82\x16`4\x82\x01R_\x90`8\x01`@Q` \x81\x83\x03\x03\x81R\x90`@Ra%a\x90a5\xC2V[\x93\x92PPPV[_a%qa\x12cV[T`\x01`@\x1B\x90\x04`\xFF\x16\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x01\x83\x01` R`@\x81 T\x80\x82\x03a%\xBBW`@Qc-\x05\x19\xAD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83a%\xC7`\x01\x83a5,V[\x81T\x81\x10a%\xD7Wa%\xD7a3\x96V[\x90_R` _ \x01T\x91PP\x92\x91PPV[`\x01a%\xF4\x82a+(V[`\x01\x81\x11\x15a&\x05Wa&\x05a1lV[\x14a&#W`@Qc;\xCD\x10+`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x82`\x01\x81\x11\x15a&7Wa&7a1lV[\x03a&UW`@Qc\x06\xC4\xA1\xC7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x83\x11\x80\x15a&lWPa&j\x81`\x02a+BV[\x15[\x15a\r\xF6W`@Qc\t\xE9\xCDI`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__a&\x95\x84a+wV[\x90P\x84\x15\x80a&\xACWP`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x15[\x80a&\xC8WP`\x03\x81`\x03\x81\x11\x15a&\xC6Wa&\xC6a1lV[\x14[\x80a&\xE3WP_\x81`\x03\x81\x11\x15a&\xE1Wa&\xE1a1lV[\x14[\x15a&\xEFW\x90Pa%aV[_c\xD2\xAFNu`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x85\x16\x01a'\x19Wa'\x12\x85_a+\x91V[\x90Pa(\x04V[c\xAB]\x12\x0B`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x85\x16\x01a'<Wa'\x12\x85`\x02a+\x91V[cBY\xA0\xBB`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x85\x16\x01a'_Wa'\x12\x85`\x03a+\x91V[c\x9A\xEA\xEBA`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x85\x16\x01a'\x82Wa'\x12\x85`\x04a+\x91V[c\xF5A:q`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x85\x16\x01a'\xA5Wa'\x12\x85`\x05a+\x91V[c\xF6\xA1XM`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x85\x16\x01a'\xC8Wa'\x12\x85`\x07a+\x91V[c2\x13\"\x1D`\xE1\x1B`\x01`\x01`\xE0\x1B\x03\x19\x85\x16\x01a'\xEBWa'\x12\x85`\x08a+\x91V[`@Qc\x18\xF4\xC1#`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81`\x04\x81\x11\x15a(\x17Wa(\x17a1lV[\x03a($WP\x90Pa%aV[a\n\x9D\x81a+\xE3V[_`\xFF`P\x83\x90\x1C\x16`\x02\x81\x11\x15a\x0EqWa\x0Eqa1lV[_`\x01`\x01`\xE0\x1B\x03\x19\x83\x16c\t^\xA7\xB3`\xE0\x1B\x14\x80\x15\x90a(zWP`\x01`\x01`\xE0\x1B\x03\x19\x83\x16cM\xEC\xDD\xE3`\xE1\x1B\x14\x15[\x15a(\x98W`@Qc\x18\xF4\xC1#`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a(\xA3_\x84a,;V[\x90P_a(\xB03\x83a sV[_\x87\x81R`\x03\x89\x01` \x90\x81R`@\x80\x83 \x93\x83R\x92\x90R T`\xFF\x16\x92PPP\x94\x93PPPPV[__a(\xE5_\x84a,;V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a)\x10W`@Qcn\xB01_`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_c\xD2\xAFNu`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x86\x16\x01a);Wa)4`\x01\x85a,;V[\x90Pa)\xDAV[c\xAB]\x12\x0B`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x86\x16\x01a)sW_a)_`\x01\x86a,;V[\x90Pa)k\x81\x84a sV[\x91PPa)\xDAV[`\x01`\x01`\xE0\x1B\x03\x19\x85\x16c\xBD\xA6_E`\xE0\x1B\x14\x80a)\xA2WP`\x01`\x01`\xE0\x1B\x03\x19\x85\x16ce\x15\x14\xBF`\xE0\x1B\x14[\x80a)\xBDWP`\x01`\x01`\xE0\x1B\x03\x19\x85\x16c\n\xBE\xC5\x8F`\xE0\x1B\x14[\x15a'\xEBW_a)\xCE`\x01\x86a,;V[\x90Pa)k\x83\x82a sV[_\x86\x81R`\x03\x88\x01` \x90\x81R`@\x80\x83 \x93\x83R\x92\x90R T`\xFF\x16\x91PP\x94\x93PPPPV[__a*\x0E3\x84a sV[_\x80\x80R`\x03\x86\x01` \x90\x81R`@\x80\x83 \x93\x83R\x92\x90R T`\xFF\x16\x91PP\x92\x91PPV[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03a*iW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x0F\xBAV[_Q` a6\x18_9_Q\x90_R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``__\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa*\xB3\x91\x90a5\xE8V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a*\xEBW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a*\xF0V[``\x91P[P\x91P\x91Pa+\0\x85\x83\x83a,\xA4V[\x95\x94PPPPPV[4\x15a\x0B\xC6W`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`\xFF`X\x83\x90\x1C\x16`\x01\x81\x11\x15a\x0EqWa\x0Eqa1lV[_\x81`\x02\x81\x11\x15a+UWa+Ua1lV[a+^\x84a(-V[`\x02\x81\x11\x15a+oWa+oa1lV[\x14\x93\x92PPPV[_`\xFF`H\x83\x90\x1C\x16`\x03\x81\x11\x15a\x0EqWa\x0Eqa1lV[_`\t\x82\x10a+\xB3W`@Qc\xB4J\xF9\xAF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a+\xBF\x83`\x08a5\x15V[a+\xCA\x90`\xB8a5qV[\x90P\x83\x81\x1B`\xF8\x1C`\x04\x81\x11\x15a\x15\x95Wa\x15\x95a1lV[__\x82`\x04\x81\x11\x15a+\xF7Wa+\xF7a1lV[\x90P\x80`\xFF\x16_\x03a,\x1CW`@Qc\xD8EZ\x13`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a,'`\x01\x82a5\xFEV[`\xFF\x16`\x03\x81\x11\x15a%aWa%aa1lV[_a,G\x83` a5\x15V[a,R\x90`\x04a5qV[a,]\x90` a5qV[\x82Q\x10\x15a,~W`@Qc\x1D\t\x8E-`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a,\x8A\x84` a5\x15V[a,\x95\x90`\x04a5qV[\x92\x90\x92\x01` \x01Q\x93\x92PPPV[``\x82a,\xB9Wa,\xB4\x82a-\0V[a%aV[\x81Q\x15\x80\x15a,\xD0WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a,\xF9W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x0F\xBAV[P\x80a%aV[\x80Q\x15a-\x0FW\x80Q` \x82\x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08\x1DW__\xFD[_` \x82\x84\x03\x12\x15a-LW__\xFD[\x815a%a\x81a-(V[__` \x83\x85\x03\x12\x15a-hW__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a-}W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a-\x8DW__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a-\xA2W__\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a-\xB6W__\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a.\x02Wa.\x02a-\xC6V[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a.\"Wa.\"a-\xC6V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[_\x82`\x1F\x83\x01\x12a.?W__\xFD[\x815a.Ra.M\x82a.\nV[a-\xDAV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a.fW__\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_` \x82\x84\x03\x12\x15a.\x92W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a.\xA7W__\xFD[a\x15\x95\x84\x82\x85\x01a.0V[_____`\x80\x86\x88\x03\x12\x15a.\xC7W__\xFD[\x855a.\xD2\x81a-(V[\x94P` \x86\x015\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a.\xF3W__\xFD[\x86\x01`\x1F\x81\x01\x88\x13a/\x03W__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a/\x18W__\xFD[\x88` \x82\x84\x01\x01\x11\x15a/)W__\xFD[` \x91\x90\x91\x01\x93P\x91P``\x86\x015`\x02\x81\x10a/DW__\xFD[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[__`@\x83\x85\x03\x12\x15a/cW__\xFD[\x825a/n\x81a-(V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a/\x88W__\xFD[a/\x94\x85\x82\x86\x01a.0V[\x91PP\x92P\x92\x90PV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x82\x15\x15\x81R`@` \x82\x01R_a\x15\x95`@\x83\x01\x84a/\x9EV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a/\xFEWa/\xFEa-\xC6V[P`\x05\x1B` \x01\x90V[\x805`\x03\x81\x10a0\x16W__\xFD[\x91\x90PV[_\x82`\x1F\x83\x01\x12a0*W__\xFD[\x815a08a.M\x82a/\xE6V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a0YW__\xFD[` \x85\x01[\x83\x81\x10\x15a0}Wa0o\x81a0\x08V[\x83R` \x92\x83\x01\x92\x01a0^V[P\x95\x94PPPPPV[__`@\x83\x85\x03\x12\x15a0\x98W__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a0\xADW__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a0\xBDW__\xFD[\x805a0\xCBa.M\x82a/\xE6V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15a0\xECW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a1#W\x835`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a1\x12W__\xFD[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a0\xF3V[\x94PPPP` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1@W__\xFD[a/\x94\x85\x82\x86\x01a0\x1BV[__`@\x83\x85\x03\x12\x15a1]W__\xFD[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x03\x81\x10a1\x90Wa1\x90a1lV[\x90RV[`@\x80\x82R\x83Q\x90\x82\x01\x81\x90R_\x90` \x85\x01\x90``\x84\x01\x90\x83[\x81\x81\x10\x15a1\xD7W\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a1\xAFV[PP\x83\x81\x03` \x80\x86\x01\x91\x90\x91R\x85Q\x80\x83R\x91\x81\x01\x92P\x85\x01\x90_[\x81\x81\x10\x15a2\x1AWa2\x07\x84\x84Qa1\x80V[` \x93\x84\x01\x93\x92\x90\x92\x01\x91`\x01\x01a1\xF4V[P\x91\x96\x95PPPPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a2]W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a2?V[P\x90\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a2xW__\xFD[P5\x91\x90PV[` \x81R_a%a` \x83\x01\x84a/\x9EV[____`\x80\x85\x87\x03\x12\x15a2\xA4W__\xFD[\x845a2\xAF\x81a-(V[\x93P` \x85\x015a2\xBF\x81a-(V[\x92P`@\x85\x015a2\xCF\x81a-(V[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[___``\x84\x86\x03\x12\x15a2\xF1W__\xFD[\x835a2\xFC\x81a-(V[\x92P` \x84\x015a3\x0C\x81a-(V[\x91Pa3\x1A`@\x85\x01a0\x08V[\x90P\x92P\x92P\x92V[` \x81\x01a\x0Eq\x82\x84a1\x80V[___``\x84\x86\x03\x12\x15a3CW__\xFD[\x835a3N\x81a-(V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_\x82a3\x91WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[____`\x80\x85\x87\x03\x12\x15a3\xBDW__\xFD[\x84Qa3\xC8\x81a-(V[` \x86\x01Q\x90\x94Pa3\xD9\x81a-(V[`@\x86\x01Q``\x90\x96\x01Q\x94\x97\x90\x96P\x92PPPV[_` \x82\x84\x03\x12\x15a3\xFFW__\xFD[\x81Qa%a\x81a-(V[`\x01\x80`\xA0\x1B\x03\x85\x16\x81R\x83` \x82\x01R`\x80`@\x82\x01R_a40`\x80\x83\x01\x85a/\x9EV[\x90P`\x02\x83\x10a4BWa4Ba1lV[\x82``\x83\x01R\x95\x94PPPPPV[\x80Q\x80\x15\x15\x81\x14a0\x16W__\xFD[_` \x82\x84\x03\x12\x15a4pW__\xFD[a%a\x82a4QV[_` \x82\x84\x03\x12\x15a4\x89W__\xFD[PQ\x91\x90PV[__`@\x83\x85\x03\x12\x15a4\xA1W__\xFD[a4\xAA\x83a4QV[\x91P` \x83\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15a4\xC4W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a4\xD4W__\xFD[\x80Qa4\xE2a.M\x82a.\nV[\x81\x81R\x86` \x83\x85\x01\x01\x11\x15a4\xF6W__\xFD[\x81` \x84\x01` \x83\x01^_` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0EqWa\x0Eqa3cV[\x81\x81\x03\x81\x81\x11\x15a\x0EqWa\x0Eqa3cV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R`@\x81\x01a%a` \x83\x01\x84a1\x80V[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0EqWa\x0Eqa3cV[\x80Q` \x82\x01Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x91\x90`\x04\x82\x10\x15a5\xBBW`\x01`\x01`\xE0\x1B\x03\x19`\x04\x83\x90\x03`\x03\x1B\x81\x90\x1B\x82\x16\x16\x92P[PP\x91\x90PV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a5\xE2W_\x19\x81` \x03`\x03\x1B\x1B\x82\x16\x91P[P\x91\x90PV[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x0EqWa\x0Eqa3cV\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\xA2dipfsX\"\x12 \xA9z\xEA\x94Q\xFA\xC0\xCE7\xF9\x16\nd\xA8ES\xF0\xE4MJaj\xC9\x0E\xC51\xB5/[\x9E,\x9CdsolcC\0\x08\x1E\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080604052600436106101db575f3560e01c80638da5cb5b116100fd578063c68c3a8311610092578063df9620eb11610062578063df9620eb146105bc578063f2fde38b146105cf578063fa19501d146105ee578063ffa1ad741461060d575f5ffd5b8063c68c3a831461051c578063dc06109d1461053b578063dc446a4a1461055a578063df4e6f8a14610586575f5ffd5b8063ad3cb1cc116100cd578063ad3cb1cc14610482578063b2b99ec9146104bf578063b5736962146104de578063c68605c8146104fd575f5ffd5b80638da5cb5b1461041d5780639d95f1cc14610431578063a2450f8914610444578063a76c9a2f14610463575f5ffd5b80635229073f1161017357806363fe3b561161014357806363fe3b56146103aa578063715018a6146103cb578063739c4b08146103df5780638b95eccd146103fe575f5ffd5b80635229073f146102fa57806352d1902d1461032757806356f551171461034957806360976c4b1461037d575f5ffd5b8063439fab91116101ae578063439fab9114610295578063468721a7146102b45780634a1ba408146102d35780634f1ef286146102e7575f5ffd5b806301750152146101df578063110dcee71461022b578063294402cc146102405780633401cde814610276575b5f5ffd5b3480156101ea575f5ffd5b506102166101f9366004612d3c565b6001600160a01b03165f9081526003602052604090205460ff1690565b60405190151581526020015b60405180910390f35b61023e610239366004612d57565b61063d565b005b34801561024b575f5ffd5b505f5461025e906001600160a01b031681565b6040516001600160a01b039091168152602001610222565b348015610281575f5ffd5b5061023e610290366004612d3c565b61080a565b3480156102a0575f5ffd5b5061023e6102af366004612e82565b610820565b3480156102bf575f5ffd5b506102166102ce366004612eb3565b610a10565b3480156102de575f5ffd5b50610216600181565b61023e6102f5366004612f52565b610aa7565b348015610305575f5ffd5b50610319610314366004612eb3565b610ac6565b604051610222929190612fcc565b348015610332575f5ffd5b5061033b610b64565b604051908152602001610222565b348015610354575f5ffd5b50610368610363366004613087565b610b7f565b60408051928352602083019190915201610222565b348015610388575f5ffd5b5061039c61039736600461314c565b610b97565b604051610222929190613194565b3480156103b5575f5ffd5b506103be610ba4565b6040516102229190613226565b3480156103d6575f5ffd5b5061023e610bb5565b3480156103ea575f5ffd5b5061023e6103f9366004613268565b610bc8565b348015610409575f5ffd5b5061023e610418366004612d3c565b610bdb565b348015610428575f5ffd5b5061025e610c2a565b61023e61043f366004612d3c565b610c58565b34801561044f575f5ffd5b5061023e61045e366004613268565b610ce2565b34801561046e575f5ffd5b5061023e61047d366004613268565b610cf3565b34801561048d575f5ffd5b506104b2604051806040016040528060058152602001640352e302e360dc1b81525081565b604051610222919061327f565b3480156104ca575f5ffd5b5061023e6104d9366004612d3c565b610d06565b3480156104e9575f5ffd5b5061023e6104f8366004613268565b610d8e565b348015610508575f5ffd5b5061023e610517366004613291565b610dc5565b348015610527575f5ffd5b5061023e6105363660046132df565b610de1565b348015610546575f5ffd5b5061023e610555366004613268565b610dfb565b348015610565575f5ffd5b5061057961057436600461314c565b610e54565b6040516102229190613323565b348015610591575f5ffd5b506105a56105a0366004612d3c565b610e77565b604080519215158352602083019190915201610222565b61023e6105ca366004612d57565b610e8d565b3480156105da575f5ffd5b5061023e6105e9366004612d3c565b610f8d565b3480156105f9575f5ffd5b5061023e610608366004613331565b610fcc565b348015610618575f5ffd5b506104b2604051806040016040528060058152602001640322e302e360dc1b81525081565b610645610fe1565b80345f82900361066857604051630c71ec1760e01b815260040160405180910390fd5b5f6106738383613377565b90505f5b83811015610802576106ae86868381811061069457610694613396565b90506020020160208101906106a99190612d3c565b611013565b5f60608787848181106106c3576106c3613396565b90506020020160208101906106d89190612d3c565b6001600160a01b0316901b6b0102030000000000000000001790506106fe600182611097565b610758600188888581811061071557610715613396565b905060200201602081019061072a9190612d3c565b89898681811061073c5761073c613396565b90506020020160208101906107519190612d3c565b6001611165565b82156107f9575f87878481811061077157610771613396565b90506020020160208101906107869190612d3c565b6001600160a01b0316845f906040515f60405180830381858888f193505050503d805f81146107d0576040519150601f19603f3d011682016040523d82523d5f602084013e6107d5565b606091505b50509050806107f757604051639a73ee9960e01b815260040160405180910390fd5b505b50600101610677565b505050505050565b610812610fe1565b61081d6001826111ff565b50565b5f610829611263565b805490915060ff600160401b82041615906001600160401b03165f8115801561084f5750825b90505f826001600160401b0316600114801561086a5750303b155b905081158015610878575080155b156108965760405163f92ee8a960e01b815260040160405180910390fd5b845467ffffffffffffffff1916600117855583156108c057845460ff60401b1916600160401b1785555b5f5f5f5f898060200190518101906108d891906133aa565b929650909450925090506001600160a01b03841615806108ff57506001600160a01b038316155b1561091d5760405163867915ab60e01b815260040160405180910390fd5b826001600160a01b0316846001600160a01b03160361094f5760405163598a0e2160e01b815260040160405180910390fd5b5f80546001600160a01b0319166001600160a01b038516179055811561097a5761097a60018361128b565b6109838161134a565b61098c8461140c565b6040516001600160a01b038416907f5fe6aabf4e790843df43ae0e22b58620066fb389295bedc06a92df6c3b28777d905f90a250505050831561080257845460ff60401b19168555604051600181527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a1505050505050565b335f9081526003602052604081205460ff16610a3f57604051631fb1d3e560e31b815260040160405180910390fd5b5f54610a5b906001906001600160a01b03168888888888611414565b610a9d868686868080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152508892506114bd915050565b9695505050505050565b610aaf61159d565b610ab882611641565b610ac28282611649565b5050565b335f9081526003602052604081205460609060ff16610af857604051631fb1d3e560e31b815260040160405180910390fd5b5f54610b14906001906001600160a01b03168989898989611414565b610b56878787878080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250899250611705915050565b915091509550959350505050565b5f610b6d6117ed565b505f5160206136185f395f51905f5290565b5f5f610b8b8484611836565b915091505b9250929050565b606080610b8b8484611936565b6060610bb06001611acc565b905090565b610bbd610fe1565b610bc65f611b25565b565b610bd0610fe1565b61081d600182611b95565b610be3610fe1565b5f80546001600160a01b0319166001600160a01b038316908117825560405190917f5fe6aabf4e790843df43ae0e22b58620066fb389295bedc06a92df6c3b28777d91a250565b7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300546001600160a01b031690565b610c60610fe1565b610c6981611013565b341561081d575f816001600160a01b0316345f906040515f60405180830381858888f193505050503d805f8114610cbb576040519150601f19603f3d011682016040523d82523d5f602084013e610cc0565b606091505b5050905080610ac257604051639a73ee9960e01b815260040160405180910390fd5b610cea610fe1565b61081d8161134a565b610cfb610fe1565b61081d60018261128b565b610d0e610fe1565b6001600160a01b0381165f9081526003602052604090205460ff16610d4657604051631fb1d3e560e31b815260040160405180910390fd5b6001600160a01b0381165f81815260036020526040808220805460ff19169055517fcfc24166db4bb677e857cacabd1541fb2b30645021b27c5130419589b84db52b9190a250565b610d96610fe1565b5f610da18260601c90565b9050610dac81611013565b610db7600183611097565b610ac2600182836001611165565b610dcd610fe1565b610ddb600185858585611c55565b50505050565b610de9610fe1565b610df66001848484611165565b505050565b610e03610fe1565b5f610e0e8260601c90565b6001600160a01b0381165f9081526003602052604090205490915060ff16610e4957604051631fb1d3e560e31b815260040160405180910390fd5b610ac2600183611097565b5f82815260046020908152604080832084845290915290205460ff165b92915050565b5f80610e84600184611dc8565b91509150915091565b610e95610fe1565b80345f829003610eb857604051630c71ec1760e01b815260040160405180910390fd5b5f610ec38383613377565b90505f5b8381101561080257610ee486868381811061069457610694613396565b8115610f85575f868683818110610efd57610efd613396565b9050602002016020810190610f129190612d3c565b6001600160a01b0316835f906040515f60405180830381858888f193505050503d805f8114610f5c576040519150601f19603f3d011682016040523d82523d5f602084013e610f61565b606091505b5050905080610f8357604051639a73ee9960e01b815260040160405180910390fd5b505b600101610ec7565b610f95610fe1565b6001600160a01b038116610fc357604051631e4fbdf760e01b81525f60048201526024015b60405180910390fd5b61081d81611b25565b610fd4610fe1565b610df66001848484611e27565b33610fea610c2a565b6001600160a01b031614610bc65760405163118cdaa760e01b8152336004820152602401610fba565b6001600160a01b0381165f9081526003602052604090205460ff161561104c576040516338e816a560e21b815260040160405180910390fd5b6001600160a01b0381165f81815260036020526040808220805460ff19166001179055517fb25d03aaf308d7291709be1ea28b800463cf3a9a4c4a5555d7333a964c1dfebd9190a250565b5f6110a28260601c90565b90506001600160a01b0381166110cb5760405163867915ab60e01b815260040160405180910390fd5b6001600160a01b0381165f90815260018401602052604090205415611103576040516374603e9560e11b815260040160405180910390fd5b5f61110f836002611f60565b905061111b8482611ff0565b50816001600160a01b03167f1ee2791f2caf0e92a9dc32a37a9ea53ab6ac7a6fb8f2d090e53a067d3a43f6ac8260405161115791815260200190565b60405180910390a250505050565b5f808052600385016020526040812082916111808686612073565b815260208101919091526040015f20805460ff191660018360028111156111a9576111a961316c565b0217905550816001600160a01b0316836001600160a01b03167f7487530ddff120799505e52b1b19b6933f85a9eeae9220c80a7ad7c429b612ae836040516111f19190613323565b60405180910390a350505050565b5f61120a83836120b7565b9050801561124a576040516001600160a01b038316907f0dfce1ea4ba1eeba891ffb2a066790fbc293a9e517fe61d49d156a30165f93f3905f90a2505050565b604051634a89032160e01b815260040160405180910390fd5b5f807ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00610e71565b5f6112968260601c90565b90506001600160a01b0381166112bf5760405163867915ab60e01b815260040160405180910390fd5b6001600160a01b0381165f908152600184016020526040902054156112f7576040516374603e9560e11b815260040160405180910390fd5b5f6113028382611f60565b905061130e8482611ff0565b50816001600160a01b03167faaf26bb12aa89ee96bbe19667a6a055727b75d3f6ed7b8b611ef6519180209d68260405161115791815260200190565b5f6113558260601c90565b90505f816001600160a01b03166382bfefc86040518163ffffffff1660e01b8152600401602060405180830381865afa158015611394573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113b891906133ef565b90506113e360016bffffffffffffffffffffffff8516606085901b6001600160601b03191617611b95565b610df660016bffffffffffffffffffffffff8516606084901b6001600160601b0319161761128b565b610f956121cc565b846001600160a01b0316866001600160a01b0316036114715761146c8784848080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152506121f192505050565b6114b4565b6114b487868686868080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525088925061228f915050565b50505050505050565b5f6114c6610c2a565b6001600160a01b031663468721a7868686866040518563ffffffff1660e01b81526004016114f7949392919061340a565b6020604051808303815f875af1158015611513573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115379190613460565b9050801561156c576040517f4e2e86d21375ebcbf6e93df5ebdd5a915bf830245904c3b54f48adf0170aae4b905f90a1611595565b6040517fc24d93608a03d263ff191d7677141f5e94c496e593108f3aae0cb5b70494c4d3905f90a15b949350505050565b306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016148061162357507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166116175f5160206136185f395f51905f52546001600160a01b031690565b6001600160a01b031614155b15610bc65760405163703e46dd60e11b815260040160405180910390fd5b61081d610fe1565b816001600160a01b03166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa9250505080156116a3575060408051601f3d908101601f191682019092526116a091810190613479565b60015b6116cb57604051634c9c8ce360e01b81526001600160a01b0383166004820152602401610fba565b5f5160206136185f395f51905f5281146116fb57604051632a87526960e21b815260048101829052602401610fba565b610df683836124c8565b5f6060611710610c2a565b6001600160a01b0316635229073f878787876040518563ffffffff1660e01b8152600401611741949392919061340a565b5f604051808303815f875af115801561175c573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526117839190810190613490565b909250905081156117bb576040517f4e2e86d21375ebcbf6e93df5ebdd5a915bf830245904c3b54f48adf0170aae4b905f90a16117e4565b6040517fc24d93608a03d263ff191d7677141f5e94c496e593108f3aae0cb5b70494c4d3905f90a15b94509492505050565b306001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614610bc65760405163703e46dd60e11b815260040160405180910390fd5b81515f908190600781111561185e576040516317a4d98760e31b815260040160405180910390fd5b8351855114611880576040516374f4d53760e01b815260040160405180910390fd5b5f805b828110156118db57611896816020613515565b6118a19060e061352c565b60e08883815181106118b5576118b5613396565b60209081029190910101516001600160e01b031916901c901b9190911790600101611883565b505f5b8281101561192b576118f1816002613515565b86828151811061190357611903613396565b6020026020010151600281111561191c5761191c61316c565b901b91909117906001016118de565b509590945092505050565b606080600783111561195b576040516317a4d98760e31b815260040160405180910390fd5b826001600160401b0381111561197357611973612dc6565b60405190808252806020026020018201604052801561199c578160200160208202803683370190505b509150826001600160401b038111156119b7576119b7612dc6565b6040519080825280602002602001820160405280156119e0578160200160208202803683370190505b5090505f5b83811015611a3d5760e06119fa826020613515565b611a059060e061352c565b86901c901b838281518110611a1c57611a1c613396565b6001600160e01b0319909216602092830291909101909101526001016119e5565b505f5b83811015611ac45760fe611a55826002613515565b611a609060fe61352c565b865f1c901b901c60ff166002811115611a7b57611a7b61316c565b828281518110611a8d57611a8d613396565b60200260200101906002811115611aa657611aa661316c565b90816002811115611ab957611ab961316c565b905250600101611a40565b509250929050565b6060815f01805480602002602001604051908101604052809291908181526020018280548015611b1957602002820191905f5260205f20905b815481526020019060010190808311611b05575b50505050509050919050565b7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930080546001600160a01b031981166001600160a01b03848116918217845560405192169182907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a3505050565b5f611ba08260601c90565b90506001600160a01b038116611bc95760405163867915ab60e01b815260040160405180910390fd5b6001600160a01b0381165f90815260018401602052604090205415611c01576040516374603e9560e11b815260040160405180910390fd5b5f611c0d836001611f60565b9050611c198482611ff0565b50816001600160a01b03167f5ffb06b0b0e8ad6a8f3c5831d499dfa612d9c9d4dc107bbd66f18f61a6492e718260405161115791815260200190565b5f5f611c62836002611936565b90925090505f5b6002811015611dbe5782515f90849083908110611c8857611c88613396565b60200260200101516001600160e01b03191614611db6575f611cc387858481518110611cb657611cb6613396565b602002602001015161251d565b9050828281518110611cd757611cd7613396565b6020026020010151896003015f8381526020019081526020015f205f611cfd8b8b612073565b815260208101919091526040015f20805460ff19166001836002811115611d2657611d2661316c565b0217905550856001600160a01b0316876001600160a01b0316896001600160a01b03167fa3df710420b01cc30ff300309abbc7fadd4630d4ab385b0f5a126fb4babe762b878681518110611d7c57611d7c613396565b6020026020010151878781518110611d9657611d96613396565b6020026020010151604051611dac92919061353f565b60405180910390a4505b600101611c69565b5050505050505050565b6001600160a01b0381165f9081526001830160205260408120548190808203611df7575f5f9250925050610b90565b600185611e04828461352c565b81548110611e1457611e14613396565b905f5260205f2001549250925050610b90565b5f5f611e34836007611936565b90925090505f5b60078110156114b45782515f90849083908110611e5a57611e5a613396565b60200260200101516001600160e01b03191614611f58575f611e8887858481518110611cb657611cb6613396565b9050828281518110611e9c57611e9c613396565b6020908102919091018101515f83815260038b01835260408082208a835290935291909120805460ff19166001836002811115611edb57611edb61316c565b021790555085876001600160a01b03167ff2ffd4f09d58d06824188033d3318d06eb957bfb1a8ffed9af78e1f19168b904868581518110611f1e57611f1e613396565b6020026020010151868681518110611f3857611f38613396565b6020026020010151604051611f4e92919061353f565b60405180910390a3505b600101611e3b565b5f80806001846002811115611f7757611f7761316c565b03611f8f57506aff0000000000000000ffff19611fc9565b5f846002811115611fa257611fa261316c565b03611fba57506aff00ffffffffffffff000019611fc9565b506aff00ffffffffffffffffff195b80851691506050846002811115611fe257611fe261316c565b901b91909117949350505050565b5f61201f83611fff8460601c90565b6001600160a01b03165f9081526001919091016020526040902054151590565b61206c578254600181810185555f858152602081209092018490558454919085019061204b8560601c90565b6001600160a01b0316815260208101919091526040015f2055506001610e71565b505f610e71565b6040516001600160601b0319606084811b8216602084015283901b1660348201525f9060480160405160208183030381529060405280519060200120905092915050565b6001600160a01b0381165f90815260018301602052604081205480156121c3575f6120e360018361352c565b85549091505f906120f69060019061352c565b905081811461216b575f865f01828154811061211457612114613396565b905f5260205f200154905080875f01848154811061213457612134613396565b905f5260205f20018190555083876001015f6121508460601c90565b6001600160a01b0316815260208101919091526040015f2055505b855486908061217c5761217c61355d565b600190038181905f5260205f20015f90559055856001015f866001600160a01b03166001600160a01b031681526020019081526020015f205f905560019350505050610e71565b5f915050610e71565b6121d4612568565b610bc657604051631afcd79f60e31b815260040160405180910390fd5b5f5f5f60605f5f602487015190508060201461222057604051637ed1113760e01b815260040160405180910390fd5b60645b8751811015612284578088015160f81c96506001810188015160601c95506015810188015194506035810188015192506035810188019350612268898787878b61228f565b612273836055613571565b61227d9082613571565b9050612223565b505050505050505050565b8151158015906122a0575060048251105b156122be57604051632342609160e11b815260040160405180910390fd5b5f6122c98686612581565b90506122d68483836125e9565b5f6122e084613584565b90505f6122ef8551848461268a565b90505f8160038111156123045761230461316c565b0361232257604051635872303760e01b815260040160405180910390fd5b60038160038111156123365761233661316c565b03612343575050506124c1565b5f8061234e8561282d565b600281111561235f5761235f61316c565b0361238057612379896123728a8661251d565b8589612847565b90506123e4565b600161238b8561282d565b600281111561239c5761239c61316c565b036123b657612379896123af8a8661251d565b85896128d9565b60026123c18561282d565b60028111156123d2576123d261316c565b036123e4576123e18989612a02565b90505b60028160028111156123f8576123f861316c565b148061243057505f8160028111156124125761241261316c565b1480156124305750600182600381111561242e5761242e61316c565b145b1561244e5760405163864dd1e760e01b815260040160405180910390fd5b60018160028111156124625761246261316c565b148061249a57505f81600281111561247c5761247c61316c565b14801561249a575060028260038111156124985761249861316c565b145b156124a857505050506124c1565b6040516308d5a8b160e31b815260040160405180910390fd5b5050505050565b6124d182612a34565b6040516001600160a01b038316907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b905f90a280511561251557610df68282612a97565b610ac2612b09565b6040516001600160601b0319606084901b1660208201526001600160e01b0319821660348201525f90603801604051602081830303815290604052612561906135c2565b9392505050565b5f612571611263565b54600160401b900460ff16919050565b6001600160a01b0381165f9081526001830160205260408120548082036125bb57604051632d0519ad60e01b815260040160405180910390fd5b836125c760018361352c565b815481106125d7576125d7613396565b905f5260205f20015491505092915050565b60016125f482612b28565b60018111156126055761260561316c565b1461262357604051633bcd102b60e21b815260040160405180910390fd5b60018260018111156126375761263761316c565b03612655576040516306c4a1c760e11b815260040160405180910390fd5b5f8311801561266c575061266a816002612b42565b155b15610df6576040516309e9cd4960e01b815260040160405180910390fd5b5f5f61269584612b77565b90508415806126ac57506001600160e01b03198316155b806126c8575060038160038111156126c6576126c661316c565b145b806126e357505f8160038111156126e1576126e161316c565b145b156126ef579050612561565b5f63d2af4e7560e01b6001600160e01b031985160161271957612712855f612b91565b9050612804565b63ab5d120b60e01b6001600160e01b031985160161273c57612712856002612b91565b634259a0bb60e01b6001600160e01b031985160161275f57612712856003612b91565b639aeaeb4160e01b6001600160e01b031985160161278257612712856004612b91565b63f5413a7160e01b6001600160e01b03198516016127a557612712856005612b91565b63f6a1584d60e01b6001600160e01b03198516016127c857612712856007612b91565b633213221d60e11b6001600160e01b03198516016127eb57612712856008612b91565b6040516318f4c12360e11b815260040160405180910390fd5b5f8160048111156128175761281761316c565b0361282457509050612561565b610a9d81612be3565b5f60ff605083901c166002811115610e7157610e7161316c565b5f6001600160e01b0319831663095ea7b360e01b1480159061287a57506001600160e01b03198316634decdde360e11b14155b15612898576040516318f4c12360e11b815260040160405180910390fd5b5f6128a35f84612c3b565b90505f6128b03383612073565b5f8781526003890160209081526040808320938352929052205460ff1692505050949350505050565b5f5f6128e55f84612c3b565b90506001600160a01b038116331461291057604051636eb0315f60e01b815260040160405180910390fd5b5f63d2af4e7560e01b6001600160e01b031986160161293b57612934600185612c3b565b90506129da565b63ab5d120b60e01b6001600160e01b0319861601612973575f61295f600186612c3b565b905061296b8184612073565b9150506129da565b6001600160e01b0319851663bda65f4560e01b14806129a257506001600160e01b0319851663651514bf60e01b145b806129bd57506001600160e01b03198516630abec58f60e01b145b156127eb575f6129ce600186612c3b565b905061296b8382612073565b5f8681526003880160209081526040808320938352929052205460ff16915050949350505050565b5f5f612a0e3384612073565b5f8080526003860160209081526040808320938352929052205460ff1691505092915050565b806001600160a01b03163b5f03612a6957604051634c9c8ce360e01b81526001600160a01b0382166004820152602401610fba565b5f5160206136185f395f51905f5280546001600160a01b0319166001600160a01b0392909216919091179055565b60605f5f846001600160a01b031684604051612ab391906135e8565b5f60405180830381855af49150503d805f8114612aeb576040519150601f19603f3d011682016040523d82523d5f602084013e612af0565b606091505b5091509150612b00858383612ca4565b95945050505050565b3415610bc65760405163b398979f60e01b815260040160405180910390fd5b5f60ff605883901c166001811115610e7157610e7161316c565b5f816002811115612b5557612b5561316c565b612b5e8461282d565b6002811115612b6f57612b6f61316c565b149392505050565b5f60ff604883901c166003811115610e7157610e7161316c565b5f60098210612bb35760405163b44af9af60e01b815260040160405180910390fd5b5f612bbf836008613515565b612bca9060b8613571565b905083811b60f81c60048111156115955761159561316c565b5f5f826004811115612bf757612bf761316c565b90508060ff165f03612c1c5760405163d8455a1360e01b815260040160405180910390fd5b612c276001826135fe565b60ff1660038111156125615761256161316c565b5f612c47836020613515565b612c52906004613571565b612c5d906020613571565b82511015612c7e57604051631d098e2d60e21b815260040160405180910390fd5b5f612c8a846020613515565b612c95906004613571565b92909201602001519392505050565b606082612cb957612cb482612d00565b612561565b8151158015612cd057506001600160a01b0384163b155b15612cf957604051639996b31560e01b81526001600160a01b0385166004820152602401610fba565b5080612561565b805115612d0f57805160208201fd5b60405163d6bda27560e01b815260040160405180910390fd5b6001600160a01b038116811461081d575f5ffd5b5f60208284031215612d4c575f5ffd5b813561256181612d28565b5f5f60208385031215612d68575f5ffd5b82356001600160401b03811115612d7d575f5ffd5b8301601f81018513612d8d575f5ffd5b80356001600160401b03811115612da2575f5ffd5b8560208260051b8401011115612db6575f5ffd5b6020919091019590945092505050565b634e487b7160e01b5f52604160045260245ffd5b604051601f8201601f191681016001600160401b0381118282101715612e0257612e02612dc6565b604052919050565b5f6001600160401b03821115612e2257612e22612dc6565b50601f01601f191660200190565b5f82601f830112612e3f575f5ffd5b8135612e52612e4d82612e0a565b612dda565b818152846020838601011115612e66575f5ffd5b816020850160208301375f918101602001919091529392505050565b5f60208284031215612e92575f5ffd5b81356001600160401b03811115612ea7575f5ffd5b61159584828501612e30565b5f5f5f5f5f60808688031215612ec7575f5ffd5b8535612ed281612d28565b94506020860135935060408601356001600160401b03811115612ef3575f5ffd5b8601601f81018813612f03575f5ffd5b80356001600160401b03811115612f18575f5ffd5b886020828401011115612f29575f5ffd5b60209190910193509150606086013560028110612f44575f5ffd5b809150509295509295909350565b5f5f60408385031215612f63575f5ffd5b8235612f6e81612d28565b915060208301356001600160401b03811115612f88575f5ffd5b612f9485828601612e30565b9150509250929050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b8215158152604060208201525f6115956040830184612f9e565b5f6001600160401b03821115612ffe57612ffe612dc6565b5060051b60200190565b803560038110613016575f5ffd5b919050565b5f82601f83011261302a575f5ffd5b8135613038612e4d82612fe6565b8082825260208201915060208360051b860101925085831115613059575f5ffd5b602085015b8381101561307d5761306f81613008565b83526020928301920161305e565b5095945050505050565b5f5f60408385031215613098575f5ffd5b82356001600160401b038111156130ad575f5ffd5b8301601f810185136130bd575f5ffd5b80356130cb612e4d82612fe6565b8082825260208201915060208360051b8501019250878311156130ec575f5ffd5b6020840193505b828410156131235783356001600160e01b031981168114613112575f5ffd5b8252602093840193909101906130f3565b945050505060208301356001600160401b03811115613140575f5ffd5b612f948582860161301b565b5f5f6040838503121561315d575f5ffd5b50508035926020909101359150565b634e487b7160e01b5f52602160045260245ffd5b600381106131905761319061316c565b9052565b604080825283519082018190525f9060208501906060840190835b818110156131d75783516001600160e01b0319168352602093840193909201916001016131af565b5050838103602080860191909152855180835291810192508501905f5b8181101561321a57613207848451613180565b60209384019392909201916001016131f4565b50919695505050505050565b602080825282518282018190525f918401906040840190835b8181101561325d57835183526020938401939092019160010161323f565b509095945050505050565b5f60208284031215613278575f5ffd5b5035919050565b602081525f6125616020830184612f9e565b5f5f5f5f608085870312156132a4575f5ffd5b84356132af81612d28565b935060208501356132bf81612d28565b925060408501356132cf81612d28565b9396929550929360600135925050565b5f5f5f606084860312156132f1575f5ffd5b83356132fc81612d28565b9250602084013561330c81612d28565b915061331a60408501613008565b90509250925092565b60208101610e718284613180565b5f5f5f60608486031215613343575f5ffd5b833561334e81612d28565b95602085013595506040909401359392505050565b634e487b7160e01b5f52601160045260245ffd5b5f8261339157634e487b7160e01b5f52601260045260245ffd5b500490565b634e487b7160e01b5f52603260045260245ffd5b5f5f5f5f608085870312156133bd575f5ffd5b84516133c881612d28565b60208601519094506133d981612d28565b6040860151606090960151949790965092505050565b5f602082840312156133ff575f5ffd5b815161256181612d28565b60018060a01b0385168152836020820152608060408201525f6134306080830185612f9e565b9050600283106134425761344261316c565b82606083015295945050505050565b80518015158114613016575f5ffd5b5f60208284031215613470575f5ffd5b61256182613451565b5f60208284031215613489575f5ffd5b5051919050565b5f5f604083850312156134a1575f5ffd5b6134aa83613451565b915060208301516001600160401b038111156134c4575f5ffd5b8301601f810185136134d4575f5ffd5b80516134e2612e4d82612e0a565b8181528660208385010111156134f6575f5ffd5b8160208401602083015e5f602083830101528093505050509250929050565b8082028115828204841417610e7157610e71613363565b81810381811115610e7157610e71613363565b6001600160e01b031983168152604081016125616020830184613180565b634e487b7160e01b5f52603160045260245ffd5b80820180821115610e7157610e71613363565b805160208201516001600160e01b03198116919060048210156135bb576001600160e01b0319600483900360031b81901b82161692505b5050919050565b805160208083015191908110156135e2575f198160200360031b1b821691505b50919050565b5f82518060208501845e5f920191825250919050565b60ff8281168282160390811115610e7157610e7161336356fe360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbca2646970667358221220a97aea9451fac0ce37f9160a64a84553f0e44d4a616ac90ec531b52f5b9e2c9c64736f6c634300081e0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x01\xDBW_5`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0\xFDW\x80c\xC6\x8C:\x83\x11a\0\x92W\x80c\xDF\x96 \xEB\x11a\0bW\x80c\xDF\x96 \xEB\x14a\x05\xBCW\x80c\xF2\xFD\xE3\x8B\x14a\x05\xCFW\x80c\xFA\x19P\x1D\x14a\x05\xEEW\x80c\xFF\xA1\xADt\x14a\x06\rW__\xFD[\x80c\xC6\x8C:\x83\x14a\x05\x1CW\x80c\xDC\x06\x10\x9D\x14a\x05;W\x80c\xDCDjJ\x14a\x05ZW\x80c\xDFNo\x8A\x14a\x05\x86W__\xFD[\x80c\xAD<\xB1\xCC\x11a\0\xCDW\x80c\xAD<\xB1\xCC\x14a\x04\x82W\x80c\xB2\xB9\x9E\xC9\x14a\x04\xBFW\x80c\xB5sib\x14a\x04\xDEW\x80c\xC6\x86\x05\xC8\x14a\x04\xFDW__\xFD[\x80c\x8D\xA5\xCB[\x14a\x04\x1DW\x80c\x9D\x95\xF1\xCC\x14a\x041W\x80c\xA2E\x0F\x89\x14a\x04DW\x80c\xA7l\x9A/\x14a\x04cW__\xFD[\x80cR)\x07?\x11a\x01sW\x80cc\xFE;V\x11a\x01CW\x80cc\xFE;V\x14a\x03\xAAW\x80cqP\x18\xA6\x14a\x03\xCBW\x80cs\x9CK\x08\x14a\x03\xDFW\x80c\x8B\x95\xEC\xCD\x14a\x03\xFEW__\xFD[\x80cR)\x07?\x14a\x02\xFAW\x80cR\xD1\x90-\x14a\x03'W\x80cV\xF5Q\x17\x14a\x03IW\x80c`\x97lK\x14a\x03}W__\xFD[\x80cC\x9F\xAB\x91\x11a\x01\xAEW\x80cC\x9F\xAB\x91\x14a\x02\x95W\x80cF\x87!\xA7\x14a\x02\xB4W\x80cJ\x1B\xA4\x08\x14a\x02\xD3W\x80cO\x1E\xF2\x86\x14a\x02\xE7W__\xFD[\x80c\x01u\x01R\x14a\x01\xDFW\x80c\x11\r\xCE\xE7\x14a\x02+W\x80c)D\x02\xCC\x14a\x02@W\x80c4\x01\xCD\xE8\x14a\x02vW[__\xFD[4\x80\x15a\x01\xEAW__\xFD[Pa\x02\x16a\x01\xF96`\x04a-<V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x03` R`@\x90 T`\xFF\x16\x90V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02>a\x0296`\x04a-WV[a\x06=V[\0[4\x80\x15a\x02KW__\xFD[P_Ta\x02^\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\"V[4\x80\x15a\x02\x81W__\xFD[Pa\x02>a\x02\x906`\x04a-<V[a\x08\nV[4\x80\x15a\x02\xA0W__\xFD[Pa\x02>a\x02\xAF6`\x04a.\x82V[a\x08 V[4\x80\x15a\x02\xBFW__\xFD[Pa\x02\x16a\x02\xCE6`\x04a.\xB3V[a\n\x10V[4\x80\x15a\x02\xDEW__\xFD[Pa\x02\x16`\x01\x81V[a\x02>a\x02\xF56`\x04a/RV[a\n\xA7V[4\x80\x15a\x03\x05W__\xFD[Pa\x03\x19a\x03\x146`\x04a.\xB3V[a\n\xC6V[`@Qa\x02\"\x92\x91\x90a/\xCCV[4\x80\x15a\x032W__\xFD[Pa\x03;a\x0BdV[`@Q\x90\x81R` \x01a\x02\"V[4\x80\x15a\x03TW__\xFD[Pa\x03ha\x03c6`\x04a0\x87V[a\x0B\x7FV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02\"V[4\x80\x15a\x03\x88W__\xFD[Pa\x03\x9Ca\x03\x976`\x04a1LV[a\x0B\x97V[`@Qa\x02\"\x92\x91\x90a1\x94V[4\x80\x15a\x03\xB5W__\xFD[Pa\x03\xBEa\x0B\xA4V[`@Qa\x02\"\x91\x90a2&V[4\x80\x15a\x03\xD6W__\xFD[Pa\x02>a\x0B\xB5V[4\x80\x15a\x03\xEAW__\xFD[Pa\x02>a\x03\xF96`\x04a2hV[a\x0B\xC8V[4\x80\x15a\x04\tW__\xFD[Pa\x02>a\x04\x186`\x04a-<V[a\x0B\xDBV[4\x80\x15a\x04(W__\xFD[Pa\x02^a\x0C*V[a\x02>a\x04?6`\x04a-<V[a\x0CXV[4\x80\x15a\x04OW__\xFD[Pa\x02>a\x04^6`\x04a2hV[a\x0C\xE2V[4\x80\x15a\x04nW__\xFD[Pa\x02>a\x04}6`\x04a2hV[a\x0C\xF3V[4\x80\x15a\x04\x8DW__\xFD[Pa\x04\xB2`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[`@Qa\x02\"\x91\x90a2\x7FV[4\x80\x15a\x04\xCAW__\xFD[Pa\x02>a\x04\xD96`\x04a-<V[a\r\x06V[4\x80\x15a\x04\xE9W__\xFD[Pa\x02>a\x04\xF86`\x04a2hV[a\r\x8EV[4\x80\x15a\x05\x08W__\xFD[Pa\x02>a\x05\x176`\x04a2\x91V[a\r\xC5V[4\x80\x15a\x05'W__\xFD[Pa\x02>a\x0566`\x04a2\xDFV[a\r\xE1V[4\x80\x15a\x05FW__\xFD[Pa\x02>a\x05U6`\x04a2hV[a\r\xFBV[4\x80\x15a\x05eW__\xFD[Pa\x05ya\x05t6`\x04a1LV[a\x0ETV[`@Qa\x02\"\x91\x90a3#V[4\x80\x15a\x05\x91W__\xFD[Pa\x05\xA5a\x05\xA06`\x04a-<V[a\x0EwV[`@\x80Q\x92\x15\x15\x83R` \x83\x01\x91\x90\x91R\x01a\x02\"V[a\x02>a\x05\xCA6`\x04a-WV[a\x0E\x8DV[4\x80\x15a\x05\xDAW__\xFD[Pa\x02>a\x05\xE96`\x04a-<V[a\x0F\x8DV[4\x80\x15a\x05\xF9W__\xFD[Pa\x02>a\x06\x086`\x04a31V[a\x0F\xCCV[4\x80\x15a\x06\x18W__\xFD[Pa\x04\xB2`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03\"\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[a\x06Ea\x0F\xE1V[\x804_\x82\x90\x03a\x06hW`@Qc\x0Cq\xEC\x17`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x06s\x83\x83a3wV[\x90P_[\x83\x81\x10\x15a\x08\x02Wa\x06\xAE\x86\x86\x83\x81\x81\x10a\x06\x94Wa\x06\x94a3\x96V[\x90P` \x02\x01` \x81\x01\x90a\x06\xA9\x91\x90a-<V[a\x10\x13V[_``\x87\x87\x84\x81\x81\x10a\x06\xC3Wa\x06\xC3a3\x96V[\x90P` \x02\x01` \x81\x01\x90a\x06\xD8\x91\x90a-<V[`\x01`\x01`\xA0\x1B\x03\x16\x90\x1Bk\x01\x02\x03\0\0\0\0\0\0\0\0\0\x17\x90Pa\x06\xFE`\x01\x82a\x10\x97V[a\x07X`\x01\x88\x88\x85\x81\x81\x10a\x07\x15Wa\x07\x15a3\x96V[\x90P` \x02\x01` \x81\x01\x90a\x07*\x91\x90a-<V[\x89\x89\x86\x81\x81\x10a\x07<Wa\x07<a3\x96V[\x90P` \x02\x01` \x81\x01\x90a\x07Q\x91\x90a-<V[`\x01a\x11eV[\x82\x15a\x07\xF9W_\x87\x87\x84\x81\x81\x10a\x07qWa\x07qa3\x96V[\x90P` \x02\x01` \x81\x01\x90a\x07\x86\x91\x90a-<V[`\x01`\x01`\xA0\x1B\x03\x16\x84_\x90`@Q_`@Q\x80\x83\x03\x81\x85\x88\x88\xF1\x93PPPP=\x80_\x81\x14a\x07\xD0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x07\xD5V[``\x91P[PP\x90P\x80a\x07\xF7W`@Qc\x9As\xEE\x99`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P[P`\x01\x01a\x06wV[PPPPPPV[a\x08\x12a\x0F\xE1V[a\x08\x1D`\x01\x82a\x11\xFFV[PV[_a\x08)a\x12cV[\x80T\x90\x91P`\xFF`\x01`@\x1B\x82\x04\x16\x15\x90`\x01`\x01`@\x1B\x03\x16_\x81\x15\x80\x15a\x08OWP\x82[\x90P_\x82`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\x08jWP0;\x15[\x90P\x81\x15\x80\x15a\x08xWP\x80\x15[\x15a\x08\x96W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x17\x85U\x83\x15a\x08\xC0W\x84T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x85U[____\x89\x80` \x01\x90Q\x81\x01\x90a\x08\xD8\x91\x90a3\xAAV[\x92\x96P\x90\x94P\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x84\x16\x15\x80a\x08\xFFWP`\x01`\x01`\xA0\x1B\x03\x83\x16\x15[\x15a\t\x1DW`@Qc\x86y\x15\xAB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x03a\tOW`@QcY\x8A\x0E!`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x17\x90U\x81\x15a\tzWa\tz`\x01\x83a\x12\x8BV[a\t\x83\x81a\x13JV[a\t\x8C\x84a\x14\x0CV[`@Q`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x7F_\xE6\xAA\xBFNy\x08C\xDFC\xAE\x0E\"\xB5\x86 \x06o\xB3\x89)[\xED\xC0j\x92\xDFl;(w}\x90_\x90\xA2PPPP\x83\x15a\x08\x02W\x84T`\xFF`@\x1B\x19\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPPV[3_\x90\x81R`\x03` R`@\x81 T`\xFF\x16a\n?W`@Qc\x1F\xB1\xD3\xE5`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_Ta\n[\x90`\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x88\x88\x88\x88\x88a\x14\x14V[a\n\x9D\x86\x86\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x88\x92Pa\x14\xBD\x91PPV[\x96\x95PPPPPPV[a\n\xAFa\x15\x9DV[a\n\xB8\x82a\x16AV[a\n\xC2\x82\x82a\x16IV[PPV[3_\x90\x81R`\x03` R`@\x81 T``\x90`\xFF\x16a\n\xF8W`@Qc\x1F\xB1\xD3\xE5`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_Ta\x0B\x14\x90`\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x89\x89\x89\x89\x89a\x14\x14V[a\x0BV\x87\x87\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x89\x92Pa\x17\x05\x91PPV[\x91P\x91P\x95P\x95\x93PPPPV[_a\x0Bma\x17\xEDV[P_Q` a6\x18_9_Q\x90_R\x90V[__a\x0B\x8B\x84\x84a\x186V[\x91P\x91P[\x92P\x92\x90PV[``\x80a\x0B\x8B\x84\x84a\x196V[``a\x0B\xB0`\x01a\x1A\xCCV[\x90P\x90V[a\x0B\xBDa\x0F\xE1V[a\x0B\xC6_a\x1B%V[V[a\x0B\xD0a\x0F\xE1V[a\x08\x1D`\x01\x82a\x1B\x95V[a\x0B\xE3a\x0F\xE1V[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x82U`@Q\x90\x91\x7F_\xE6\xAA\xBFNy\x08C\xDFC\xAE\x0E\"\xB5\x86 \x06o\xB3\x89)[\xED\xC0j\x92\xDFl;(w}\x91\xA2PV[\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T`\x01`\x01`\xA0\x1B\x03\x16\x90V[a\x0C`a\x0F\xE1V[a\x0Ci\x81a\x10\x13V[4\x15a\x08\x1DW_\x81`\x01`\x01`\xA0\x1B\x03\x164_\x90`@Q_`@Q\x80\x83\x03\x81\x85\x88\x88\xF1\x93PPPP=\x80_\x81\x14a\x0C\xBBW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x0C\xC0V[``\x91P[PP\x90P\x80a\n\xC2W`@Qc\x9As\xEE\x99`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0C\xEAa\x0F\xE1V[a\x08\x1D\x81a\x13JV[a\x0C\xFBa\x0F\xE1V[a\x08\x1D`\x01\x82a\x12\x8BV[a\r\x0Ea\x0F\xE1V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x03` R`@\x90 T`\xFF\x16a\rFW`@Qc\x1F\xB1\xD3\xE5`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x81\x81R`\x03` R`@\x80\x82 \x80T`\xFF\x19\x16\x90UQ\x7F\xCF\xC2Af\xDBK\xB6w\xE8W\xCA\xCA\xBD\x15A\xFB+0dP!\xB2|Q0A\x95\x89\xB8M\xB5+\x91\x90\xA2PV[a\r\x96a\x0F\xE1V[_a\r\xA1\x82``\x1C\x90V[\x90Pa\r\xAC\x81a\x10\x13V[a\r\xB7`\x01\x83a\x10\x97V[a\n\xC2`\x01\x82\x83`\x01a\x11eV[a\r\xCDa\x0F\xE1V[a\r\xDB`\x01\x85\x85\x85\x85a\x1CUV[PPPPV[a\r\xE9a\x0F\xE1V[a\r\xF6`\x01\x84\x84\x84a\x11eV[PPPV[a\x0E\x03a\x0F\xE1V[_a\x0E\x0E\x82``\x1C\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x03` R`@\x90 T\x90\x91P`\xFF\x16a\x0EIW`@Qc\x1F\xB1\xD3\xE5`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\n\xC2`\x01\x83a\x10\x97V[_\x82\x81R`\x04` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 T`\xFF\x16[\x92\x91PPV[_\x80a\x0E\x84`\x01\x84a\x1D\xC8V[\x91P\x91P\x91P\x91V[a\x0E\x95a\x0F\xE1V[\x804_\x82\x90\x03a\x0E\xB8W`@Qc\x0Cq\xEC\x17`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0E\xC3\x83\x83a3wV[\x90P_[\x83\x81\x10\x15a\x08\x02Wa\x0E\xE4\x86\x86\x83\x81\x81\x10a\x06\x94Wa\x06\x94a3\x96V[\x81\x15a\x0F\x85W_\x86\x86\x83\x81\x81\x10a\x0E\xFDWa\x0E\xFDa3\x96V[\x90P` \x02\x01` \x81\x01\x90a\x0F\x12\x91\x90a-<V[`\x01`\x01`\xA0\x1B\x03\x16\x83_\x90`@Q_`@Q\x80\x83\x03\x81\x85\x88\x88\xF1\x93PPPP=\x80_\x81\x14a\x0F\\W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x0FaV[``\x91P[PP\x90P\x80a\x0F\x83W`@Qc\x9As\xEE\x99`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P[`\x01\x01a\x0E\xC7V[a\x0F\x95a\x0F\xE1V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0F\xC3W`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R_`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[a\x08\x1D\x81a\x1B%V[a\x0F\xD4a\x0F\xE1V[a\r\xF6`\x01\x84\x84\x84a\x1E'V[3a\x0F\xEAa\x0C*V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0B\xC6W`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x0F\xBAV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x03` R`@\x90 T`\xFF\x16\x15a\x10LW`@Qc8\xE8\x16\xA5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x81\x81R`\x03` R`@\x80\x82 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7F\xB2]\x03\xAA\xF3\x08\xD7)\x17\t\xBE\x1E\xA2\x8B\x80\x04c\xCF:\x9ALJUU\xD73:\x96L\x1D\xFE\xBD\x91\x90\xA2PV[_a\x10\xA2\x82``\x1C\x90V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x10\xCBW`@Qc\x86y\x15\xAB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x01\x84\x01` R`@\x90 T\x15a\x11\x03W`@Qct`>\x95`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x11\x0F\x83`\x02a\x1F`V[\x90Pa\x11\x1B\x84\x82a\x1F\xF0V[P\x81`\x01`\x01`\xA0\x1B\x03\x16\x7F\x1E\xE2y\x1F,\xAF\x0E\x92\xA9\xDC2\xA3z\x9E\xA5:\xB6\xACzo\xB8\xF2\xD0\x90\xE5:\x06}:C\xF6\xAC\x82`@Qa\x11W\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPPV[_\x80\x80R`\x03\x85\x01` R`@\x81 \x82\x91a\x11\x80\x86\x86a sV[\x81R` \x81\x01\x91\x90\x91R`@\x01_ \x80T`\xFF\x19\x16`\x01\x83`\x02\x81\x11\x15a\x11\xA9Wa\x11\xA9a1lV[\x02\x17\x90UP\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7Ft\x87S\r\xDF\xF1 y\x95\x05\xE5+\x1B\x19\xB6\x93?\x85\xA9\xEE\xAE\x92 \xC8\nz\xD7\xC4)\xB6\x12\xAE\x83`@Qa\x11\xF1\x91\x90a3#V[`@Q\x80\x91\x03\x90\xA3PPPPV[_a\x12\n\x83\x83a \xB7V[\x90P\x80\x15a\x12JW`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\r\xFC\xE1\xEAK\xA1\xEE\xBA\x89\x1F\xFB*\x06g\x90\xFB\xC2\x93\xA9\xE5\x17\xFEa\xD4\x9D\x15j0\x16_\x93\xF3\x90_\x90\xA2PPPV[`@QcJ\x89\x03!`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0a\x0EqV[_a\x12\x96\x82``\x1C\x90V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x12\xBFW`@Qc\x86y\x15\xAB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x01\x84\x01` R`@\x90 T\x15a\x12\xF7W`@Qct`>\x95`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x13\x02\x83\x82a\x1F`V[\x90Pa\x13\x0E\x84\x82a\x1F\xF0V[P\x81`\x01`\x01`\xA0\x1B\x03\x16\x7F\xAA\xF2k\xB1*\xA8\x9E\xE9k\xBE\x19fzj\x05W'\xB7]?n\xD7\xB8\xB6\x11\xEFe\x19\x18\x02\t\xD6\x82`@Qa\x11W\x91\x81R` \x01\x90V[_a\x13U\x82``\x1C\x90V[\x90P_\x81`\x01`\x01`\xA0\x1B\x03\x16c\x82\xBF\xEF\xC8`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\x94W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xB8\x91\x90a3\xEFV[\x90Pa\x13\xE3`\x01k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16``\x85\x90\x1B`\x01`\x01``\x1B\x03\x19\x16\x17a\x1B\x95V[a\r\xF6`\x01k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16``\x84\x90\x1B`\x01`\x01``\x1B\x03\x19\x16\x17a\x12\x8BV[a\x0F\x95a!\xCCV[\x84`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x03a\x14qWa\x14l\x87\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa!\xF1\x92PPPV[a\x14\xB4V[a\x14\xB4\x87\x86\x86\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x88\x92Pa\"\x8F\x91PPV[PPPPPPPV[_a\x14\xC6a\x0C*V[`\x01`\x01`\xA0\x1B\x03\x16cF\x87!\xA7\x86\x86\x86\x86`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\xF7\x94\x93\x92\x91\x90a4\nV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x15\x13W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x157\x91\x90a4`V[\x90P\x80\x15a\x15lW`@Q\x7FN.\x86\xD2\x13u\xEB\xCB\xF6\xE9=\xF5\xEB\xDDZ\x91[\xF80$Y\x04\xC3\xB5OH\xAD\xF0\x17\n\xAEK\x90_\x90\xA1a\x15\x95V[`@Q\x7F\xC2M\x93`\x8A\x03\xD2c\xFF\x19\x1Dvw\x14\x1F^\x94\xC4\x96\xE5\x93\x10\x8F:\xAE\x0C\xB5\xB7\x04\x94\xC4\xD3\x90_\x90\xA1[\x94\x93PPPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x16#WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x16\x17_Q` a6\x18_9_Q\x90_RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x0B\xC6W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08\x1Da\x0F\xE1V[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x16\xA3WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x16\xA0\x91\x81\x01\x90a4yV[`\x01[a\x16\xCBW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x0F\xBAV[_Q` a6\x18_9_Q\x90_R\x81\x14a\x16\xFBW`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x0F\xBAV[a\r\xF6\x83\x83a$\xC8V[_``a\x17\x10a\x0C*V[`\x01`\x01`\xA0\x1B\x03\x16cR)\x07?\x87\x87\x87\x87`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17A\x94\x93\x92\x91\x90a4\nV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x17\\W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x17\x83\x91\x90\x81\x01\x90a4\x90V[\x90\x92P\x90P\x81\x15a\x17\xBBW`@Q\x7FN.\x86\xD2\x13u\xEB\xCB\xF6\xE9=\xF5\xEB\xDDZ\x91[\xF80$Y\x04\xC3\xB5OH\xAD\xF0\x17\n\xAEK\x90_\x90\xA1a\x17\xE4V[`@Q\x7F\xC2M\x93`\x8A\x03\xD2c\xFF\x19\x1Dvw\x14\x1F^\x94\xC4\x96\xE5\x93\x10\x8F:\xAE\x0C\xB5\xB7\x04\x94\xC4\xD3\x90_\x90\xA1[\x94P\x94\x92PPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0B\xC6W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81Q_\x90\x81\x90`\x07\x81\x11\x15a\x18^W`@Qc\x17\xA4\xD9\x87`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83Q\x85Q\x14a\x18\x80W`@Qct\xF4\xD57`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80[\x82\x81\x10\x15a\x18\xDBWa\x18\x96\x81` a5\x15V[a\x18\xA1\x90`\xE0a5,V[`\xE0\x88\x83\x81Q\x81\x10a\x18\xB5Wa\x18\xB5a3\x96V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x90\x1C\x90\x1B\x91\x90\x91\x17\x90`\x01\x01a\x18\x83V[P_[\x82\x81\x10\x15a\x19+Wa\x18\xF1\x81`\x02a5\x15V[\x86\x82\x81Q\x81\x10a\x19\x03Wa\x19\x03a3\x96V[` \x02` \x01\x01Q`\x02\x81\x11\x15a\x19\x1CWa\x19\x1Ca1lV[\x90\x1B\x91\x90\x91\x17\x90`\x01\x01a\x18\xDEV[P\x95\x90\x94P\x92PPPV[``\x80`\x07\x83\x11\x15a\x19[W`@Qc\x17\xA4\xD9\x87`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19sWa\x19sa-\xC6V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x19\x9CW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\xB7Wa\x19\xB7a-\xC6V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x19\xE0W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83\x81\x10\x15a\x1A=W`\xE0a\x19\xFA\x82` a5\x15V[a\x1A\x05\x90`\xE0a5,V[\x86\x90\x1C\x90\x1B\x83\x82\x81Q\x81\x10a\x1A\x1CWa\x1A\x1Ca3\x96V[`\x01`\x01`\xE0\x1B\x03\x19\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x19\xE5V[P_[\x83\x81\x10\x15a\x1A\xC4W`\xFEa\x1AU\x82`\x02a5\x15V[a\x1A`\x90`\xFEa5,V[\x86_\x1C\x90\x1B\x90\x1C`\xFF\x16`\x02\x81\x11\x15a\x1A{Wa\x1A{a1lV[\x82\x82\x81Q\x81\x10a\x1A\x8DWa\x1A\x8Da3\x96V[` \x02` \x01\x01\x90`\x02\x81\x11\x15a\x1A\xA6Wa\x1A\xA6a1lV[\x90\x81`\x02\x81\x11\x15a\x1A\xB9Wa\x1A\xB9a1lV[\x90RP`\x01\x01a\x1A@V[P\x92P\x92\x90PV[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x1B\x19W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x1B\x05W[PPPPP\x90P\x91\x90PV[\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x82\x17\x84U`@Q\x92\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPPV[_a\x1B\xA0\x82``\x1C\x90V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1B\xC9W`@Qc\x86y\x15\xAB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x01\x84\x01` R`@\x90 T\x15a\x1C\x01W`@Qct`>\x95`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x1C\r\x83`\x01a\x1F`V[\x90Pa\x1C\x19\x84\x82a\x1F\xF0V[P\x81`\x01`\x01`\xA0\x1B\x03\x16\x7F_\xFB\x06\xB0\xB0\xE8\xADj\x8F<X1\xD4\x99\xDF\xA6\x12\xD9\xC9\xD4\xDC\x10{\xBDf\xF1\x8Fa\xA6I.q\x82`@Qa\x11W\x91\x81R` \x01\x90V[__a\x1Cb\x83`\x02a\x196V[\x90\x92P\x90P_[`\x02\x81\x10\x15a\x1D\xBEW\x82Q_\x90\x84\x90\x83\x90\x81\x10a\x1C\x88Wa\x1C\x88a3\x96V[` \x02` \x01\x01Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\x1D\xB6W_a\x1C\xC3\x87\x85\x84\x81Q\x81\x10a\x1C\xB6Wa\x1C\xB6a3\x96V[` \x02` \x01\x01Qa%\x1DV[\x90P\x82\x82\x81Q\x81\x10a\x1C\xD7Wa\x1C\xD7a3\x96V[` \x02` \x01\x01Q\x89`\x03\x01_\x83\x81R` \x01\x90\x81R` \x01_ _a\x1C\xFD\x8B\x8Ba sV[\x81R` \x81\x01\x91\x90\x91R`@\x01_ \x80T`\xFF\x19\x16`\x01\x83`\x02\x81\x11\x15a\x1D&Wa\x1D&a1lV[\x02\x17\x90UP\x85`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x89`\x01`\x01`\xA0\x1B\x03\x16\x7F\xA3\xDFq\x04 \xB0\x1C\xC3\x0F\xF3\x000\x9A\xBB\xC7\xFA\xDDF0\xD4\xAB8[\x0FZ\x12o\xB4\xBA\xBEv+\x87\x86\x81Q\x81\x10a\x1D|Wa\x1D|a3\x96V[` \x02` \x01\x01Q\x87\x87\x81Q\x81\x10a\x1D\x96Wa\x1D\x96a3\x96V[` \x02` \x01\x01Q`@Qa\x1D\xAC\x92\x91\x90a5?V[`@Q\x80\x91\x03\x90\xA4P[`\x01\x01a\x1CiV[PPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x01\x83\x01` R`@\x81 T\x81\x90\x80\x82\x03a\x1D\xF7W__\x92P\x92PPa\x0B\x90V[`\x01\x85a\x1E\x04\x82\x84a5,V[\x81T\x81\x10a\x1E\x14Wa\x1E\x14a3\x96V[\x90_R` _ \x01T\x92P\x92PPa\x0B\x90V[__a\x1E4\x83`\x07a\x196V[\x90\x92P\x90P_[`\x07\x81\x10\x15a\x14\xB4W\x82Q_\x90\x84\x90\x83\x90\x81\x10a\x1EZWa\x1EZa3\x96V[` \x02` \x01\x01Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\x1FXW_a\x1E\x88\x87\x85\x84\x81Q\x81\x10a\x1C\xB6Wa\x1C\xB6a3\x96V[\x90P\x82\x82\x81Q\x81\x10a\x1E\x9CWa\x1E\x9Ca3\x96V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q_\x83\x81R`\x03\x8B\x01\x83R`@\x80\x82 \x8A\x83R\x90\x93R\x91\x90\x91 \x80T`\xFF\x19\x16`\x01\x83`\x02\x81\x11\x15a\x1E\xDBWa\x1E\xDBa1lV[\x02\x17\x90UP\x85\x87`\x01`\x01`\xA0\x1B\x03\x16\x7F\xF2\xFF\xD4\xF0\x9DX\xD0h$\x18\x803\xD31\x8D\x06\xEB\x95{\xFB\x1A\x8F\xFE\xD9\xAFx\xE1\xF1\x91h\xB9\x04\x86\x85\x81Q\x81\x10a\x1F\x1EWa\x1F\x1Ea3\x96V[` \x02` \x01\x01Q\x86\x86\x81Q\x81\x10a\x1F8Wa\x1F8a3\x96V[` \x02` \x01\x01Q`@Qa\x1FN\x92\x91\x90a5?V[`@Q\x80\x91\x03\x90\xA3P[`\x01\x01a\x1E;V[_\x80\x80`\x01\x84`\x02\x81\x11\x15a\x1FwWa\x1Fwa1lV[\x03a\x1F\x8FWPj\xFF\0\0\0\0\0\0\0\0\xFF\xFF\x19a\x1F\xC9V[_\x84`\x02\x81\x11\x15a\x1F\xA2Wa\x1F\xA2a1lV[\x03a\x1F\xBAWPj\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x19a\x1F\xC9V[Pj\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19[\x80\x85\x16\x91P`P\x84`\x02\x81\x11\x15a\x1F\xE2Wa\x1F\xE2a1lV[\x90\x1B\x91\x90\x91\x17\x94\x93PPPPV[_a \x1F\x83a\x1F\xFF\x84``\x1C\x90V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x01\x91\x90\x91\x01` R`@\x90 T\x15\x15\x90V[a lW\x82T`\x01\x81\x81\x01\x85U_\x85\x81R` \x81 \x90\x92\x01\x84\x90U\x84T\x91\x90\x85\x01\x90a K\x85``\x1C\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_ UP`\x01a\x0EqV[P_a\x0EqV[`@Q`\x01`\x01``\x1B\x03\x19``\x84\x81\x1B\x82\x16` \x84\x01R\x83\x90\x1B\x16`4\x82\x01R_\x90`H\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a!\xC3W_a \xE3`\x01\x83a5,V[\x85T\x90\x91P_\x90a \xF6\x90`\x01\x90a5,V[\x90P\x81\x81\x14a!kW_\x86_\x01\x82\x81T\x81\x10a!\x14Wa!\x14a3\x96V[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10a!4Wa!4a3\x96V[\x90_R` _ \x01\x81\x90UP\x83\x87`\x01\x01_a!P\x84``\x1C\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_ UP[\x85T\x86\x90\x80a!|Wa!|a5]V[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa\x0EqV[_\x91PPa\x0EqV[a!\xD4a%hV[a\x0B\xC6W`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[___``__`$\x87\x01Q\x90P\x80` \x14a\" W`@Qc~\xD1\x117`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`d[\x87Q\x81\x10\x15a\"\x84W\x80\x88\x01Q`\xF8\x1C\x96P`\x01\x81\x01\x88\x01Q``\x1C\x95P`\x15\x81\x01\x88\x01Q\x94P`5\x81\x01\x88\x01Q\x92P`5\x81\x01\x88\x01\x93Pa\"h\x89\x87\x87\x87\x8Ba\"\x8FV[a\"s\x83`Ua5qV[a\"}\x90\x82a5qV[\x90Pa\"#V[PPPPPPPPPV[\x81Q\x15\x80\x15\x90a\"\xA0WP`\x04\x82Q\x10[\x15a\"\xBEW`@Qc#B`\x91`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\"\xC9\x86\x86a%\x81V[\x90Pa\"\xD6\x84\x83\x83a%\xE9V[_a\"\xE0\x84a5\x84V[\x90P_a\"\xEF\x85Q\x84\x84a&\x8AV[\x90P_\x81`\x03\x81\x11\x15a#\x04Wa#\x04a1lV[\x03a#\"W`@QcXr07`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03\x81`\x03\x81\x11\x15a#6Wa#6a1lV[\x03a#CWPPPa$\xC1V[_\x80a#N\x85a(-V[`\x02\x81\x11\x15a#_Wa#_a1lV[\x03a#\x80Wa#y\x89a#r\x8A\x86a%\x1DV[\x85\x89a(GV[\x90Pa#\xE4V[`\x01a#\x8B\x85a(-V[`\x02\x81\x11\x15a#\x9CWa#\x9Ca1lV[\x03a#\xB6Wa#y\x89a#\xAF\x8A\x86a%\x1DV[\x85\x89a(\xD9V[`\x02a#\xC1\x85a(-V[`\x02\x81\x11\x15a#\xD2Wa#\xD2a1lV[\x03a#\xE4Wa#\xE1\x89\x89a*\x02V[\x90P[`\x02\x81`\x02\x81\x11\x15a#\xF8Wa#\xF8a1lV[\x14\x80a$0WP_\x81`\x02\x81\x11\x15a$\x12Wa$\x12a1lV[\x14\x80\x15a$0WP`\x01\x82`\x03\x81\x11\x15a$.Wa$.a1lV[\x14[\x15a$NW`@Qc\x86M\xD1\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x81`\x02\x81\x11\x15a$bWa$ba1lV[\x14\x80a$\x9AWP_\x81`\x02\x81\x11\x15a$|Wa$|a1lV[\x14\x80\x15a$\x9AWP`\x02\x82`\x03\x81\x11\x15a$\x98Wa$\x98a1lV[\x14[\x15a$\xA8WPPPPa$\xC1V[`@Qc\x08\xD5\xA8\xB1`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPV[a$\xD1\x82a*4V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15a%\x15Wa\r\xF6\x82\x82a*\x97V[a\n\xC2a+\tV[`@Q`\x01`\x01``\x1B\x03\x19``\x84\x90\x1B\x16` \x82\x01R`\x01`\x01`\xE0\x1B\x03\x19\x82\x16`4\x82\x01R_\x90`8\x01`@Q` \x81\x83\x03\x03\x81R\x90`@Ra%a\x90a5\xC2V[\x93\x92PPPV[_a%qa\x12cV[T`\x01`@\x1B\x90\x04`\xFF\x16\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x01\x83\x01` R`@\x81 T\x80\x82\x03a%\xBBW`@Qc-\x05\x19\xAD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83a%\xC7`\x01\x83a5,V[\x81T\x81\x10a%\xD7Wa%\xD7a3\x96V[\x90_R` _ \x01T\x91PP\x92\x91PPV[`\x01a%\xF4\x82a+(V[`\x01\x81\x11\x15a&\x05Wa&\x05a1lV[\x14a&#W`@Qc;\xCD\x10+`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x82`\x01\x81\x11\x15a&7Wa&7a1lV[\x03a&UW`@Qc\x06\xC4\xA1\xC7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x83\x11\x80\x15a&lWPa&j\x81`\x02a+BV[\x15[\x15a\r\xF6W`@Qc\t\xE9\xCDI`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__a&\x95\x84a+wV[\x90P\x84\x15\x80a&\xACWP`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x15[\x80a&\xC8WP`\x03\x81`\x03\x81\x11\x15a&\xC6Wa&\xC6a1lV[\x14[\x80a&\xE3WP_\x81`\x03\x81\x11\x15a&\xE1Wa&\xE1a1lV[\x14[\x15a&\xEFW\x90Pa%aV[_c\xD2\xAFNu`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x85\x16\x01a'\x19Wa'\x12\x85_a+\x91V[\x90Pa(\x04V[c\xAB]\x12\x0B`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x85\x16\x01a'<Wa'\x12\x85`\x02a+\x91V[cBY\xA0\xBB`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x85\x16\x01a'_Wa'\x12\x85`\x03a+\x91V[c\x9A\xEA\xEBA`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x85\x16\x01a'\x82Wa'\x12\x85`\x04a+\x91V[c\xF5A:q`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x85\x16\x01a'\xA5Wa'\x12\x85`\x05a+\x91V[c\xF6\xA1XM`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x85\x16\x01a'\xC8Wa'\x12\x85`\x07a+\x91V[c2\x13\"\x1D`\xE1\x1B`\x01`\x01`\xE0\x1B\x03\x19\x85\x16\x01a'\xEBWa'\x12\x85`\x08a+\x91V[`@Qc\x18\xF4\xC1#`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81`\x04\x81\x11\x15a(\x17Wa(\x17a1lV[\x03a($WP\x90Pa%aV[a\n\x9D\x81a+\xE3V[_`\xFF`P\x83\x90\x1C\x16`\x02\x81\x11\x15a\x0EqWa\x0Eqa1lV[_`\x01`\x01`\xE0\x1B\x03\x19\x83\x16c\t^\xA7\xB3`\xE0\x1B\x14\x80\x15\x90a(zWP`\x01`\x01`\xE0\x1B\x03\x19\x83\x16cM\xEC\xDD\xE3`\xE1\x1B\x14\x15[\x15a(\x98W`@Qc\x18\xF4\xC1#`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a(\xA3_\x84a,;V[\x90P_a(\xB03\x83a sV[_\x87\x81R`\x03\x89\x01` \x90\x81R`@\x80\x83 \x93\x83R\x92\x90R T`\xFF\x16\x92PPP\x94\x93PPPPV[__a(\xE5_\x84a,;V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a)\x10W`@Qcn\xB01_`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_c\xD2\xAFNu`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x86\x16\x01a);Wa)4`\x01\x85a,;V[\x90Pa)\xDAV[c\xAB]\x12\x0B`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x86\x16\x01a)sW_a)_`\x01\x86a,;V[\x90Pa)k\x81\x84a sV[\x91PPa)\xDAV[`\x01`\x01`\xE0\x1B\x03\x19\x85\x16c\xBD\xA6_E`\xE0\x1B\x14\x80a)\xA2WP`\x01`\x01`\xE0\x1B\x03\x19\x85\x16ce\x15\x14\xBF`\xE0\x1B\x14[\x80a)\xBDWP`\x01`\x01`\xE0\x1B\x03\x19\x85\x16c\n\xBE\xC5\x8F`\xE0\x1B\x14[\x15a'\xEBW_a)\xCE`\x01\x86a,;V[\x90Pa)k\x83\x82a sV[_\x86\x81R`\x03\x88\x01` \x90\x81R`@\x80\x83 \x93\x83R\x92\x90R T`\xFF\x16\x91PP\x94\x93PPPPV[__a*\x0E3\x84a sV[_\x80\x80R`\x03\x86\x01` \x90\x81R`@\x80\x83 \x93\x83R\x92\x90R T`\xFF\x16\x91PP\x92\x91PPV[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03a*iW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x0F\xBAV[_Q` a6\x18_9_Q\x90_R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``__\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa*\xB3\x91\x90a5\xE8V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a*\xEBW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a*\xF0V[``\x91P[P\x91P\x91Pa+\0\x85\x83\x83a,\xA4V[\x95\x94PPPPPV[4\x15a\x0B\xC6W`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`\xFF`X\x83\x90\x1C\x16`\x01\x81\x11\x15a\x0EqWa\x0Eqa1lV[_\x81`\x02\x81\x11\x15a+UWa+Ua1lV[a+^\x84a(-V[`\x02\x81\x11\x15a+oWa+oa1lV[\x14\x93\x92PPPV[_`\xFF`H\x83\x90\x1C\x16`\x03\x81\x11\x15a\x0EqWa\x0Eqa1lV[_`\t\x82\x10a+\xB3W`@Qc\xB4J\xF9\xAF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a+\xBF\x83`\x08a5\x15V[a+\xCA\x90`\xB8a5qV[\x90P\x83\x81\x1B`\xF8\x1C`\x04\x81\x11\x15a\x15\x95Wa\x15\x95a1lV[__\x82`\x04\x81\x11\x15a+\xF7Wa+\xF7a1lV[\x90P\x80`\xFF\x16_\x03a,\x1CW`@Qc\xD8EZ\x13`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a,'`\x01\x82a5\xFEV[`\xFF\x16`\x03\x81\x11\x15a%aWa%aa1lV[_a,G\x83` a5\x15V[a,R\x90`\x04a5qV[a,]\x90` a5qV[\x82Q\x10\x15a,~W`@Qc\x1D\t\x8E-`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a,\x8A\x84` a5\x15V[a,\x95\x90`\x04a5qV[\x92\x90\x92\x01` \x01Q\x93\x92PPPV[``\x82a,\xB9Wa,\xB4\x82a-\0V[a%aV[\x81Q\x15\x80\x15a,\xD0WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a,\xF9W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x0F\xBAV[P\x80a%aV[\x80Q\x15a-\x0FW\x80Q` \x82\x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08\x1DW__\xFD[_` \x82\x84\x03\x12\x15a-LW__\xFD[\x815a%a\x81a-(V[__` \x83\x85\x03\x12\x15a-hW__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a-}W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a-\x8DW__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a-\xA2W__\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a-\xB6W__\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a.\x02Wa.\x02a-\xC6V[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a.\"Wa.\"a-\xC6V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[_\x82`\x1F\x83\x01\x12a.?W__\xFD[\x815a.Ra.M\x82a.\nV[a-\xDAV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a.fW__\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_` \x82\x84\x03\x12\x15a.\x92W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a.\xA7W__\xFD[a\x15\x95\x84\x82\x85\x01a.0V[_____`\x80\x86\x88\x03\x12\x15a.\xC7W__\xFD[\x855a.\xD2\x81a-(V[\x94P` \x86\x015\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a.\xF3W__\xFD[\x86\x01`\x1F\x81\x01\x88\x13a/\x03W__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a/\x18W__\xFD[\x88` \x82\x84\x01\x01\x11\x15a/)W__\xFD[` \x91\x90\x91\x01\x93P\x91P``\x86\x015`\x02\x81\x10a/DW__\xFD[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[__`@\x83\x85\x03\x12\x15a/cW__\xFD[\x825a/n\x81a-(V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a/\x88W__\xFD[a/\x94\x85\x82\x86\x01a.0V[\x91PP\x92P\x92\x90PV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x82\x15\x15\x81R`@` \x82\x01R_a\x15\x95`@\x83\x01\x84a/\x9EV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a/\xFEWa/\xFEa-\xC6V[P`\x05\x1B` \x01\x90V[\x805`\x03\x81\x10a0\x16W__\xFD[\x91\x90PV[_\x82`\x1F\x83\x01\x12a0*W__\xFD[\x815a08a.M\x82a/\xE6V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a0YW__\xFD[` \x85\x01[\x83\x81\x10\x15a0}Wa0o\x81a0\x08V[\x83R` \x92\x83\x01\x92\x01a0^V[P\x95\x94PPPPPV[__`@\x83\x85\x03\x12\x15a0\x98W__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a0\xADW__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a0\xBDW__\xFD[\x805a0\xCBa.M\x82a/\xE6V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15a0\xECW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a1#W\x835`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a1\x12W__\xFD[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a0\xF3V[\x94PPPP` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1@W__\xFD[a/\x94\x85\x82\x86\x01a0\x1BV[__`@\x83\x85\x03\x12\x15a1]W__\xFD[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x03\x81\x10a1\x90Wa1\x90a1lV[\x90RV[`@\x80\x82R\x83Q\x90\x82\x01\x81\x90R_\x90` \x85\x01\x90``\x84\x01\x90\x83[\x81\x81\x10\x15a1\xD7W\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a1\xAFV[PP\x83\x81\x03` \x80\x86\x01\x91\x90\x91R\x85Q\x80\x83R\x91\x81\x01\x92P\x85\x01\x90_[\x81\x81\x10\x15a2\x1AWa2\x07\x84\x84Qa1\x80V[` \x93\x84\x01\x93\x92\x90\x92\x01\x91`\x01\x01a1\xF4V[P\x91\x96\x95PPPPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a2]W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a2?V[P\x90\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a2xW__\xFD[P5\x91\x90PV[` \x81R_a%a` \x83\x01\x84a/\x9EV[____`\x80\x85\x87\x03\x12\x15a2\xA4W__\xFD[\x845a2\xAF\x81a-(V[\x93P` \x85\x015a2\xBF\x81a-(V[\x92P`@\x85\x015a2\xCF\x81a-(V[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[___``\x84\x86\x03\x12\x15a2\xF1W__\xFD[\x835a2\xFC\x81a-(V[\x92P` \x84\x015a3\x0C\x81a-(V[\x91Pa3\x1A`@\x85\x01a0\x08V[\x90P\x92P\x92P\x92V[` \x81\x01a\x0Eq\x82\x84a1\x80V[___``\x84\x86\x03\x12\x15a3CW__\xFD[\x835a3N\x81a-(V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_\x82a3\x91WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[____`\x80\x85\x87\x03\x12\x15a3\xBDW__\xFD[\x84Qa3\xC8\x81a-(V[` \x86\x01Q\x90\x94Pa3\xD9\x81a-(V[`@\x86\x01Q``\x90\x96\x01Q\x94\x97\x90\x96P\x92PPPV[_` \x82\x84\x03\x12\x15a3\xFFW__\xFD[\x81Qa%a\x81a-(V[`\x01\x80`\xA0\x1B\x03\x85\x16\x81R\x83` \x82\x01R`\x80`@\x82\x01R_a40`\x80\x83\x01\x85a/\x9EV[\x90P`\x02\x83\x10a4BWa4Ba1lV[\x82``\x83\x01R\x95\x94PPPPPV[\x80Q\x80\x15\x15\x81\x14a0\x16W__\xFD[_` \x82\x84\x03\x12\x15a4pW__\xFD[a%a\x82a4QV[_` \x82\x84\x03\x12\x15a4\x89W__\xFD[PQ\x91\x90PV[__`@\x83\x85\x03\x12\x15a4\xA1W__\xFD[a4\xAA\x83a4QV[\x91P` \x83\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15a4\xC4W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a4\xD4W__\xFD[\x80Qa4\xE2a.M\x82a.\nV[\x81\x81R\x86` \x83\x85\x01\x01\x11\x15a4\xF6W__\xFD[\x81` \x84\x01` \x83\x01^_` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0EqWa\x0Eqa3cV[\x81\x81\x03\x81\x81\x11\x15a\x0EqWa\x0Eqa3cV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R`@\x81\x01a%a` \x83\x01\x84a1\x80V[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0EqWa\x0Eqa3cV[\x80Q` \x82\x01Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x91\x90`\x04\x82\x10\x15a5\xBBW`\x01`\x01`\xE0\x1B\x03\x19`\x04\x83\x90\x03`\x03\x1B\x81\x90\x1B\x82\x16\x16\x92P[PP\x91\x90PV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a5\xE2W_\x19\x81` \x03`\x03\x1B\x1B\x82\x16\x91P[P\x91\x90PV[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x0EqWa\x0Eqa3cV\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\xA2dipfsX\"\x12 \xA9z\xEA\x94Q\xFA\xC0\xCE7\xF9\x16\nd\xA8ES\xF0\xE4MJaj\xC9\x0E\xC51\xB5/[\x9E,\x9CdsolcC\0\x08\x1E\x003",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GranularPermission(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<GranularPermission> for u8 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        impl GranularPermission {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(value: u8) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(self) -> u8 {
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
        impl From<u8> for GranularPermission {
            fn from(value: u8) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<GranularPermission> for u8 {
            fn from(value: GranularPermission) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for GranularPermission {
            type RustType = u8;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for GranularPermission {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Target(alloy::sol_types::private::primitives::aliases::U256);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Target>
        for alloy::sol_types::private::primitives::aliases::U256 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                256,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<256>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        impl Target {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(
                value: alloy::sol_types::private::primitives::aliases::U256,
            ) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(
                self,
            ) -> alloy::sol_types::private::primitives::aliases::U256 {
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
        impl From<alloy::sol_types::private::primitives::aliases::U256> for Target {
            fn from(
                value: alloy::sol_types::private::primitives::aliases::U256,
            ) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<Target> for alloy::sol_types::private::primitives::aliases::U256 {
            fn from(value: Target) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Target {
            type RustType = alloy::sol_types::private::primitives::aliases::U256;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                256,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                256,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                256,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Target {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
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
    /**Custom error with signature `AddressIsZero()` and selector `0x867915ab`.
```solidity
error AddressIsZero();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AddressIsZero;
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
        impl ::core::convert::From<AddressIsZero> for UnderlyingRustTuple<'_> {
            fn from(value: AddressIsZero) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AddressIsZero {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AddressIsZero {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AddressIsZero()";
            const SELECTOR: [u8; 4] = [134u8, 121u8, 21u8, 171u8];
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
    /**Custom error with signature `ArrayTooLong()` and selector `0xbd26cc38`.
```solidity
error ArrayTooLong();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ArrayTooLong;
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
        impl ::core::convert::From<ArrayTooLong> for UnderlyingRustTuple<'_> {
            fn from(value: ArrayTooLong) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ArrayTooLong {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ArrayTooLong {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ArrayTooLong()";
            const SELECTOR: [u8; 4] = [189u8, 38u8, 204u8, 56u8];
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
    /**Custom error with signature `ArraysDifferentLength()` and selector `0x74f4d537`.
```solidity
error ArraysDifferentLength();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ArraysDifferentLength;
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
        impl ::core::convert::From<ArraysDifferentLength> for UnderlyingRustTuple<'_> {
            fn from(value: ArraysDifferentLength) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ArraysDifferentLength {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ArraysDifferentLength {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ArraysDifferentLength()";
            const SELECTOR: [u8; 4] = [116u8, 244u8, 213u8, 55u8];
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
    /**Custom error with signature `CalldataOutOfBounds()` and selector `0x742638b4`.
```solidity
error CalldataOutOfBounds();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CalldataOutOfBounds;
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
        impl ::core::convert::From<CalldataOutOfBounds> for UnderlyingRustTuple<'_> {
            fn from(value: CalldataOutOfBounds) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CalldataOutOfBounds {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CalldataOutOfBounds {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CalldataOutOfBounds()";
            const SELECTOR: [u8; 4] = [116u8, 38u8, 56u8, 180u8];
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
    /**Custom error with signature `CannotChangeOwner()` and selector `0xfd670ebe`.
```solidity
error CannotChangeOwner();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CannotChangeOwner;
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
        impl ::core::convert::From<CannotChangeOwner> for UnderlyingRustTuple<'_> {
            fn from(value: CannotChangeOwner) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CannotChangeOwner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CannotChangeOwner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CannotChangeOwner()";
            const SELECTOR: [u8; 4] = [253u8, 103u8, 14u8, 190u8];
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
    /**Custom error with signature `DefaultPermissionRejected()` and selector `0x58723037`.
```solidity
error DefaultPermissionRejected();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DefaultPermissionRejected;
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
        impl ::core::convert::From<DefaultPermissionRejected>
        for UnderlyingRustTuple<'_> {
            fn from(value: DefaultPermissionRejected) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for DefaultPermissionRejected {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DefaultPermissionRejected {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DefaultPermissionRejected()";
            const SELECTOR: [u8; 4] = [88u8, 114u8, 48u8, 55u8];
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
    /**Custom error with signature `DelegateCallNotAllowed()` and selector `0x0d89438e`.
```solidity
error DelegateCallNotAllowed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DelegateCallNotAllowed;
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
        impl ::core::convert::From<DelegateCallNotAllowed> for UnderlyingRustTuple<'_> {
            fn from(value: DelegateCallNotAllowed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DelegateCallNotAllowed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DelegateCallNotAllowed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DelegateCallNotAllowed()";
            const SELECTOR: [u8; 4] = [13u8, 137u8, 67u8, 142u8];
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
    /**Custom error with signature `FailedToSendEthToNode()` and selector `0x9a73ee99`.
```solidity
error FailedToSendEthToNode();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FailedToSendEthToNode;
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
        impl ::core::convert::From<FailedToSendEthToNode> for UnderlyingRustTuple<'_> {
            fn from(value: FailedToSendEthToNode) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FailedToSendEthToNode {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for FailedToSendEthToNode {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FailedToSendEthToNode()";
            const SELECTOR: [u8; 4] = [154u8, 115u8, 238u8, 153u8];
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
    /**Custom error with signature `FunctionSignatureTooShort()` and selector `0x4684c122`.
```solidity
error FunctionSignatureTooShort();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FunctionSignatureTooShort;
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
        impl ::core::convert::From<FunctionSignatureTooShort>
        for UnderlyingRustTuple<'_> {
            fn from(value: FunctionSignatureTooShort) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for FunctionSignatureTooShort {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for FunctionSignatureTooShort {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FunctionSignatureTooShort()";
            const SELECTOR: [u8; 4] = [70u8, 132u8, 193u8, 34u8];
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
    /**Custom error with signature `GranularPermissionRejected()` and selector `0x864dd1e7`.
```solidity
error GranularPermissionRejected();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GranularPermissionRejected;
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
        impl ::core::convert::From<GranularPermissionRejected>
        for UnderlyingRustTuple<'_> {
            fn from(value: GranularPermissionRejected) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for GranularPermissionRejected {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for GranularPermissionRejected {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GranularPermissionRejected()";
            const SELECTOR: [u8; 4] = [134u8, 77u8, 209u8, 231u8];
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
    /**Custom error with signature `LengthIsZero()` and selector `0x0c71ec17`.
```solidity
error LengthIsZero();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LengthIsZero;
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
        impl ::core::convert::From<LengthIsZero> for UnderlyingRustTuple<'_> {
            fn from(value: LengthIsZero) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for LengthIsZero {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LengthIsZero {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LengthIsZero()";
            const SELECTOR: [u8; 4] = [12u8, 113u8, 236u8, 23u8];
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
    /**Custom error with signature `NoMembership()` and selector `0xfd8e9f28`.
```solidity
error NoMembership();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NoMembership;
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
        impl ::core::convert::From<NoMembership> for UnderlyingRustTuple<'_> {
            fn from(value: NoMembership) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NoMembership {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NoMembership {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NoMembership()";
            const SELECTOR: [u8; 4] = [253u8, 142u8, 159u8, 40u8];
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
    /**Custom error with signature `NodePermissionRejected()` and selector `0x6eb0315f`.
```solidity
error NodePermissionRejected();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NodePermissionRejected;
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
        impl ::core::convert::From<NodePermissionRejected> for UnderlyingRustTuple<'_> {
            fn from(value: NodePermissionRejected) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NodePermissionRejected {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NodePermissionRejected {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NodePermissionRejected()";
            const SELECTOR: [u8; 4] = [110u8, 176u8, 49u8, 95u8];
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
    /**Custom error with signature `NonExistentKey()` and selector `0x2d0519ad`.
```solidity
error NonExistentKey();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NonExistentKey;
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
        impl ::core::convert::From<NonExistentKey> for UnderlyingRustTuple<'_> {
            fn from(value: NonExistentKey) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NonExistentKey {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NonExistentKey {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NonExistentKey()";
            const SELECTOR: [u8; 4] = [45u8, 5u8, 25u8, 173u8];
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
    /**Custom error with signature `ParameterNotAllowed()` and selector `0x31e98246`.
```solidity
error ParameterNotAllowed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ParameterNotAllowed;
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
        impl ::core::convert::From<ParameterNotAllowed> for UnderlyingRustTuple<'_> {
            fn from(value: ParameterNotAllowed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ParameterNotAllowed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ParameterNotAllowed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ParameterNotAllowed()";
            const SELECTOR: [u8; 4] = [49u8, 233u8, 130u8, 70u8];
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
    /**Custom error with signature `PermissionNotConfigured()` and selector `0x46ad4588`.
```solidity
error PermissionNotConfigured();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PermissionNotConfigured;
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
        impl ::core::convert::From<PermissionNotConfigured> for UnderlyingRustTuple<'_> {
            fn from(value: PermissionNotConfigured) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PermissionNotConfigured {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for PermissionNotConfigured {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PermissionNotConfigured()";
            const SELECTOR: [u8; 4] = [70u8, 173u8, 69u8, 136u8];
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
    /**Custom error with signature `PermissionNotFound()` and selector `0xd8455a13`.
```solidity
error PermissionNotFound();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PermissionNotFound;
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
        impl ::core::convert::From<PermissionNotFound> for UnderlyingRustTuple<'_> {
            fn from(value: PermissionNotFound) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PermissionNotFound {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for PermissionNotFound {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PermissionNotFound()";
            const SELECTOR: [u8; 4] = [216u8, 69u8, 90u8, 19u8];
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
    /**Custom error with signature `SafeMultisendSameAddress()` and selector `0x598a0e21`.
```solidity
error SafeMultisendSameAddress();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SafeMultisendSameAddress;
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
        impl ::core::convert::From<SafeMultisendSameAddress>
        for UnderlyingRustTuple<'_> {
            fn from(value: SafeMultisendSameAddress) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for SafeMultisendSameAddress {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SafeMultisendSameAddress {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SafeMultisendSameAddress()";
            const SELECTOR: [u8; 4] = [89u8, 138u8, 14u8, 33u8];
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
    /**Custom error with signature `SendNotAllowed()` and selector `0x09e9cd49`.
```solidity
error SendNotAllowed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SendNotAllowed;
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
        impl ::core::convert::From<SendNotAllowed> for UnderlyingRustTuple<'_> {
            fn from(value: SendNotAllowed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SendNotAllowed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SendNotAllowed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SendNotAllowed()";
            const SELECTOR: [u8; 4] = [9u8, 233u8, 205u8, 73u8];
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
    /**Custom error with signature `TargetAddressNotAllowed()` and selector `0xef3440ac`.
```solidity
error TargetAddressNotAllowed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TargetAddressNotAllowed;
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
        impl ::core::convert::From<TargetAddressNotAllowed> for UnderlyingRustTuple<'_> {
            fn from(value: TargetAddressNotAllowed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TargetAddressNotAllowed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TargetAddressNotAllowed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TargetAddressNotAllowed()";
            const SELECTOR: [u8; 4] = [239u8, 52u8, 64u8, 172u8];
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
    /**Custom error with signature `TargetIsNotScoped()` and selector `0x4a890321`.
```solidity
error TargetIsNotScoped();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TargetIsNotScoped;
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
        impl ::core::convert::From<TargetIsNotScoped> for UnderlyingRustTuple<'_> {
            fn from(value: TargetIsNotScoped) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TargetIsNotScoped {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TargetIsNotScoped {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TargetIsNotScoped()";
            const SELECTOR: [u8; 4] = [74u8, 137u8, 3u8, 33u8];
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
    /**Custom error with signature `TargetIsScoped()` and selector `0xe8c07d2a`.
```solidity
error TargetIsScoped();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TargetIsScoped;
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
        impl ::core::convert::From<TargetIsScoped> for UnderlyingRustTuple<'_> {
            fn from(value: TargetIsScoped) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TargetIsScoped {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TargetIsScoped {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TargetIsScoped()";
            const SELECTOR: [u8; 4] = [232u8, 192u8, 125u8, 42u8];
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
    /**Custom error with signature `TooManyCapabilities()` and selector `0xb44af9af`.
```solidity
error TooManyCapabilities();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TooManyCapabilities;
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
        impl ::core::convert::From<TooManyCapabilities> for UnderlyingRustTuple<'_> {
            fn from(value: TooManyCapabilities) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TooManyCapabilities {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TooManyCapabilities {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TooManyCapabilities()";
            const SELECTOR: [u8; 4] = [180u8, 74u8, 249u8, 175u8];
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
    /**Custom error with signature `UnacceptableMultiSendOffset()` and selector `0x7ed11137`.
```solidity
error UnacceptableMultiSendOffset();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UnacceptableMultiSendOffset;
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
        impl ::core::convert::From<UnacceptableMultiSendOffset>
        for UnderlyingRustTuple<'_> {
            fn from(value: UnacceptableMultiSendOffset) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for UnacceptableMultiSendOffset {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UnacceptableMultiSendOffset {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UnacceptableMultiSendOffset()";
            const SELECTOR: [u8; 4] = [126u8, 209u8, 17u8, 55u8];
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
    /**Custom error with signature `WithMembership()` and selector `0xe3a05a94`.
```solidity
error WithMembership();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WithMembership;
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
        impl ::core::convert::From<WithMembership> for UnderlyingRustTuple<'_> {
            fn from(value: WithMembership) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for WithMembership {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for WithMembership {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WithMembership()";
            const SELECTOR: [u8; 4] = [227u8, 160u8, 90u8, 148u8];
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
    /**Event with signature `ExecutionFailure()` and selector `0xc24d93608a03d263ff191d7677141f5e94c496e593108f3aae0cb5b70494c4d3`.
```solidity
event ExecutionFailure();
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ExecutionFailure;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ExecutionFailure {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ExecutionFailure()";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                194u8, 77u8, 147u8, 96u8, 138u8, 3u8, 210u8, 99u8, 255u8, 25u8, 29u8,
                118u8, 119u8, 20u8, 31u8, 94u8, 148u8, 196u8, 150u8, 229u8, 147u8, 16u8,
                143u8, 58u8, 174u8, 12u8, 181u8, 183u8, 4u8, 148u8, 196u8, 211u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {}
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
        impl alloy_sol_types::private::IntoLogData for ExecutionFailure {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ExecutionFailure> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ExecutionFailure) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ExecutionSuccess()` and selector `0x4e2e86d21375ebcbf6e93df5ebdd5a915bf830245904c3b54f48adf0170aae4b`.
```solidity
event ExecutionSuccess();
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ExecutionSuccess;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ExecutionSuccess {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ExecutionSuccess()";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                78u8, 46u8, 134u8, 210u8, 19u8, 117u8, 235u8, 203u8, 246u8, 233u8, 61u8,
                245u8, 235u8, 221u8, 90u8, 145u8, 91u8, 248u8, 48u8, 36u8, 89u8, 4u8,
                195u8, 181u8, 79u8, 72u8, 173u8, 240u8, 23u8, 10u8, 174u8, 75u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {}
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
        impl alloy_sol_types::private::IntoLogData for ExecutionSuccess {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ExecutionSuccess> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ExecutionSuccess) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `NodeAdded(address)` and selector `0xb25d03aaf308d7291709be1ea28b800463cf3a9a4c4a5555d7333a964c1dfebd`.
```solidity
event NodeAdded(address indexed node);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct NodeAdded {
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
        impl alloy_sol_types::SolEvent for NodeAdded {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "NodeAdded(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                178u8, 93u8, 3u8, 170u8, 243u8, 8u8, 215u8, 41u8, 23u8, 9u8, 190u8, 30u8,
                162u8, 139u8, 128u8, 4u8, 99u8, 207u8, 58u8, 154u8, 76u8, 74u8, 85u8,
                85u8, 215u8, 51u8, 58u8, 150u8, 76u8, 29u8, 254u8, 189u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { node: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.node.clone())
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
                    &self.node,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for NodeAdded {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&NodeAdded> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &NodeAdded) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `NodeRemoved(address)` and selector `0xcfc24166db4bb677e857cacabd1541fb2b30645021b27c5130419589b84db52b`.
```solidity
event NodeRemoved(address indexed node);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct NodeRemoved {
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
        impl alloy_sol_types::SolEvent for NodeRemoved {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "NodeRemoved(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                207u8, 194u8, 65u8, 102u8, 219u8, 75u8, 182u8, 119u8, 232u8, 87u8, 202u8,
                202u8, 189u8, 21u8, 65u8, 251u8, 43u8, 48u8, 100u8, 80u8, 33u8, 178u8,
                124u8, 81u8, 48u8, 65u8, 149u8, 137u8, 184u8, 77u8, 181u8, 43u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { node: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.node.clone())
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
                    &self.node,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for NodeRemoved {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&NodeRemoved> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &NodeRemoved) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `RevokedTarget(address)` and selector `0x0dfce1ea4ba1eeba891ffb2a066790fbc293a9e517fe61d49d156a30165f93f3`.
```solidity
event RevokedTarget(address indexed targetAddress);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RevokedTarget {
        #[allow(missing_docs)]
        pub targetAddress: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for RevokedTarget {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RevokedTarget(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                13u8, 252u8, 225u8, 234u8, 75u8, 161u8, 238u8, 186u8, 137u8, 31u8, 251u8,
                42u8, 6u8, 103u8, 144u8, 251u8, 194u8, 147u8, 169u8, 229u8, 23u8, 254u8,
                97u8, 212u8, 157u8, 21u8, 106u8, 48u8, 22u8, 95u8, 147u8, 243u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { targetAddress: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.targetAddress.clone())
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
                    &self.targetAddress,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RevokedTarget {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RevokedTarget> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RevokedTarget) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ScopedGranularChannelCapability(address,bytes32,bytes4,uint8)` and selector `0xf2ffd4f09d58d06824188033d3318d06eb957bfb1a8ffed9af78e1f19168b904`.
```solidity
event ScopedGranularChannelCapability(address indexed targetAddress, bytes32 indexed channelId, bytes4 selector, GranularPermission permission);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ScopedGranularChannelCapability {
        #[allow(missing_docs)]
        pub targetAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub channelId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub selector: alloy::sol_types::private::FixedBytes<4>,
        #[allow(missing_docs)]
        pub permission: <GranularPermission as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for ScopedGranularChannelCapability {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<4>,
                GranularPermission,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "ScopedGranularChannelCapability(address,bytes32,bytes4,uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                242u8, 255u8, 212u8, 240u8, 157u8, 88u8, 208u8, 104u8, 36u8, 24u8, 128u8,
                51u8, 211u8, 49u8, 141u8, 6u8, 235u8, 149u8, 123u8, 251u8, 26u8, 143u8,
                254u8, 217u8, 175u8, 120u8, 225u8, 241u8, 145u8, 104u8, 185u8, 4u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    targetAddress: topics.1,
                    channelId: topics.2,
                    selector: data.0,
                    permission: data.1,
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
                        4,
                    > as alloy_sol_types::SolType>::tokenize(&self.selector),
                    <GranularPermission as alloy_sol_types::SolType>::tokenize(
                        &self.permission,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.targetAddress.clone(),
                    self.channelId.clone(),
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
                    &self.targetAddress,
                );
                out[2usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.channelId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ScopedGranularChannelCapability {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ScopedGranularChannelCapability>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &ScopedGranularChannelCapability,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ScopedGranularSendCapability(address,address,uint8)` and selector `0x7487530ddff120799505e52b1b19b6933f85a9eeae9220c80a7ad7c429b612ae`.
```solidity
event ScopedGranularSendCapability(address indexed nodeAddress, address indexed recipientAddress, GranularPermission permission);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ScopedGranularSendCapability {
        #[allow(missing_docs)]
        pub nodeAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub recipientAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub permission: <GranularPermission as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for ScopedGranularSendCapability {
            type DataTuple<'a> = (GranularPermission,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ScopedGranularSendCapability(address,address,uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                116u8, 135u8, 83u8, 13u8, 223u8, 241u8, 32u8, 121u8, 149u8, 5u8, 229u8,
                43u8, 27u8, 25u8, 182u8, 147u8, 63u8, 133u8, 169u8, 238u8, 174u8, 146u8,
                32u8, 200u8, 10u8, 122u8, 215u8, 196u8, 41u8, 182u8, 18u8, 174u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    nodeAddress: topics.1,
                    recipientAddress: topics.2,
                    permission: data.0,
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
                    <GranularPermission as alloy_sol_types::SolType>::tokenize(
                        &self.permission,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.nodeAddress.clone(),
                    self.recipientAddress.clone(),
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
                    &self.nodeAddress,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.recipientAddress,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ScopedGranularSendCapability {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ScopedGranularSendCapability> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &ScopedGranularSendCapability,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ScopedGranularTokenCapability(address,address,address,bytes4,uint8)` and selector `0xa3df710420b01cc30ff300309abbc7fadd4630d4ab385b0f5a126fb4babe762b`.
```solidity
event ScopedGranularTokenCapability(address indexed nodeAddress, address indexed targetAddress, address indexed recipientAddress, bytes4 selector, GranularPermission permission);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ScopedGranularTokenCapability {
        #[allow(missing_docs)]
        pub nodeAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub targetAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub recipientAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub selector: alloy::sol_types::private::FixedBytes<4>,
        #[allow(missing_docs)]
        pub permission: <GranularPermission as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for ScopedGranularTokenCapability {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<4>,
                GranularPermission,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ScopedGranularTokenCapability(address,address,address,bytes4,uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                163u8, 223u8, 113u8, 4u8, 32u8, 176u8, 28u8, 195u8, 15u8, 243u8, 0u8,
                48u8, 154u8, 187u8, 199u8, 250u8, 221u8, 70u8, 48u8, 212u8, 171u8, 56u8,
                91u8, 15u8, 90u8, 18u8, 111u8, 180u8, 186u8, 190u8, 118u8, 43u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    nodeAddress: topics.1,
                    targetAddress: topics.2,
                    recipientAddress: topics.3,
                    selector: data.0,
                    permission: data.1,
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
                        4,
                    > as alloy_sol_types::SolType>::tokenize(&self.selector),
                    <GranularPermission as alloy_sol_types::SolType>::tokenize(
                        &self.permission,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.nodeAddress.clone(),
                    self.targetAddress.clone(),
                    self.recipientAddress.clone(),
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
                    &self.nodeAddress,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.targetAddress,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.recipientAddress,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ScopedGranularTokenCapability {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ScopedGranularTokenCapability> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &ScopedGranularTokenCapability,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ScopedTargetChannels(address,uint256)` and selector `0x5ffb06b0b0e8ad6a8f3c5831d499dfa612d9c9d4dc107bbd66f18f61a6492e71`.
```solidity
event ScopedTargetChannels(address indexed targetAddress, Target target);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ScopedTargetChannels {
        #[allow(missing_docs)]
        pub targetAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub target: <Target as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for ScopedTargetChannels {
            type DataTuple<'a> = (Target,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ScopedTargetChannels(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                95u8, 251u8, 6u8, 176u8, 176u8, 232u8, 173u8, 106u8, 143u8, 60u8, 88u8,
                49u8, 212u8, 153u8, 223u8, 166u8, 18u8, 217u8, 201u8, 212u8, 220u8, 16u8,
                123u8, 189u8, 102u8, 241u8, 143u8, 97u8, 166u8, 73u8, 46u8, 113u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    targetAddress: topics.1,
                    target: data.0,
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
                (<Target as alloy_sol_types::SolType>::tokenize(&self.target),)
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.targetAddress.clone())
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
                    &self.targetAddress,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ScopedTargetChannels {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ScopedTargetChannels> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ScopedTargetChannels) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ScopedTargetSend(address,uint256)` and selector `0x1ee2791f2caf0e92a9dc32a37a9ea53ab6ac7a6fb8f2d090e53a067d3a43f6ac`.
```solidity
event ScopedTargetSend(address indexed targetAddress, Target target);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ScopedTargetSend {
        #[allow(missing_docs)]
        pub targetAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub target: <Target as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for ScopedTargetSend {
            type DataTuple<'a> = (Target,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ScopedTargetSend(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                30u8, 226u8, 121u8, 31u8, 44u8, 175u8, 14u8, 146u8, 169u8, 220u8, 50u8,
                163u8, 122u8, 158u8, 165u8, 58u8, 182u8, 172u8, 122u8, 111u8, 184u8,
                242u8, 208u8, 144u8, 229u8, 58u8, 6u8, 125u8, 58u8, 67u8, 246u8, 172u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    targetAddress: topics.1,
                    target: data.0,
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
                (<Target as alloy_sol_types::SolType>::tokenize(&self.target),)
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.targetAddress.clone())
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
                    &self.targetAddress,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ScopedTargetSend {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ScopedTargetSend> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ScopedTargetSend) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ScopedTargetToken(address,uint256)` and selector `0xaaf26bb12aa89ee96bbe19667a6a055727b75d3f6ed7b8b611ef6519180209d6`.
```solidity
event ScopedTargetToken(address indexed targetAddress, Target target);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ScopedTargetToken {
        #[allow(missing_docs)]
        pub targetAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub target: <Target as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for ScopedTargetToken {
            type DataTuple<'a> = (Target,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ScopedTargetToken(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                170u8, 242u8, 107u8, 177u8, 42u8, 168u8, 158u8, 233u8, 107u8, 190u8,
                25u8, 102u8, 122u8, 106u8, 5u8, 87u8, 39u8, 183u8, 93u8, 63u8, 110u8,
                215u8, 184u8, 182u8, 17u8, 239u8, 101u8, 25u8, 24u8, 2u8, 9u8, 214u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    targetAddress: topics.1,
                    target: data.0,
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
                (<Target as alloy_sol_types::SolType>::tokenize(&self.target),)
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.targetAddress.clone())
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
                    &self.targetAddress,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ScopedTargetToken {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ScopedTargetToken> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ScopedTargetToken) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `SetMultisendAddress(address)` and selector `0x5fe6aabf4e790843df43ae0e22b58620066fb389295bedc06a92df6c3b28777d`.
```solidity
event SetMultisendAddress(address indexed multisendAddress);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SetMultisendAddress {
        #[allow(missing_docs)]
        pub multisendAddress: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for SetMultisendAddress {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "SetMultisendAddress(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                95u8, 230u8, 170u8, 191u8, 78u8, 121u8, 8u8, 67u8, 223u8, 67u8, 174u8,
                14u8, 34u8, 181u8, 134u8, 32u8, 6u8, 111u8, 179u8, 137u8, 41u8, 91u8,
                237u8, 192u8, 106u8, 146u8, 223u8, 108u8, 59u8, 40u8, 119u8, 125u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { multisendAddress: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.multisendAddress.clone())
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
                    &self.multisendAddress,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for SetMultisendAddress {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SetMultisendAddress> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &SetMultisendAddress) -> alloy_sol_types::private::LogData {
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
    /**Function with signature `VERSION()` and selector `0xffa1ad74`.
```solidity
function VERSION() external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct VERSIONCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`VERSION()`](VERSIONCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct VERSIONReturn {
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
            impl ::core::convert::From<VERSIONCall> for UnderlyingRustTuple<'_> {
                fn from(value: VERSIONCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for VERSIONCall {
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
            impl ::core::convert::From<VERSIONReturn> for UnderlyingRustTuple<'_> {
                fn from(value: VERSIONReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for VERSIONReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for VERSIONCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::String;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "VERSION()";
            const SELECTOR: [u8; 4] = [255u8, 161u8, 173u8, 116u8];
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
                        let r: VERSIONReturn = r.into();
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
                        let r: VERSIONReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `addChannelsAndTokenTarget(uint256)` and selector `0xa2450f89`.
```solidity
function addChannelsAndTokenTarget(Target defaultTarget) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addChannelsAndTokenTargetCall {
        #[allow(missing_docs)]
        pub defaultTarget: <Target as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`addChannelsAndTokenTarget(uint256)`](addChannelsAndTokenTargetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addChannelsAndTokenTargetReturn {}
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
            type UnderlyingSolTuple<'a> = (Target,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Target as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<addChannelsAndTokenTargetCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: addChannelsAndTokenTargetCall) -> Self {
                    (value.defaultTarget,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addChannelsAndTokenTargetCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { defaultTarget: tuple.0 }
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
            impl ::core::convert::From<addChannelsAndTokenTargetReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: addChannelsAndTokenTargetReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addChannelsAndTokenTargetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl addChannelsAndTokenTargetReturn {
            fn _tokenize(
                &self,
            ) -> <addChannelsAndTokenTargetCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addChannelsAndTokenTargetCall {
            type Parameters<'a> = (Target,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addChannelsAndTokenTargetReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addChannelsAndTokenTarget(uint256)";
            const SELECTOR: [u8; 4] = [162u8, 69u8, 15u8, 137u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<Target as alloy_sol_types::SolType>::tokenize(&self.defaultTarget),)
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                addChannelsAndTokenTargetReturn::_tokenize(ret)
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
    /**Function with signature `addNode(address)` and selector `0x9d95f1cc`.
```solidity
function addNode(address nodeAddress) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addNodeCall {
        #[allow(missing_docs)]
        pub nodeAddress: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`addNode(address)`](addNodeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addNodeReturn {}
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
            impl ::core::convert::From<addNodeCall> for UnderlyingRustTuple<'_> {
                fn from(value: addNodeCall) -> Self {
                    (value.nodeAddress,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addNodeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { nodeAddress: tuple.0 }
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
            impl ::core::convert::From<addNodeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: addNodeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addNodeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl addNodeReturn {
            fn _tokenize(
                &self,
            ) -> <addNodeCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addNodeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addNodeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addNode(address)";
            const SELECTOR: [u8; 4] = [157u8, 149u8, 241u8, 204u8];
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
                        &self.nodeAddress,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                addNodeReturn::_tokenize(ret)
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
    /**Function with signature `addNodes(address[])` and selector `0xdf9620eb`.
```solidity
function addNodes(address[] memory nodeAddresses) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addNodesCall {
        #[allow(missing_docs)]
        pub nodeAddresses: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`addNodes(address[])`](addNodesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addNodesReturn {}
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<addNodesCall> for UnderlyingRustTuple<'_> {
                fn from(value: addNodesCall) -> Self {
                    (value.nodeAddresses,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addNodesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { nodeAddresses: tuple.0 }
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
            impl ::core::convert::From<addNodesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: addNodesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addNodesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl addNodesReturn {
            fn _tokenize(
                &self,
            ) -> <addNodesCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addNodesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addNodesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addNodes(address[])";
            const SELECTOR: [u8; 4] = [223u8, 150u8, 32u8, 235u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.nodeAddresses),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                addNodesReturn::_tokenize(ret)
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
    /**Function with signature `decodeFunctionSigsAndPermissions(bytes32,uint256)` and selector `0x60976c4b`.
```solidity
function decodeFunctionSigsAndPermissions(bytes32 encoded, uint256 length) external pure returns (bytes4[] memory functionSigs, GranularPermission[] memory permissions);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct decodeFunctionSigsAndPermissionsCall {
        #[allow(missing_docs)]
        pub encoded: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub length: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`decodeFunctionSigsAndPermissions(bytes32,uint256)`](decodeFunctionSigsAndPermissionsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct decodeFunctionSigsAndPermissionsReturn {
        #[allow(missing_docs)]
        pub functionSigs: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<4>,
        >,
        #[allow(missing_docs)]
        pub permissions: alloy::sol_types::private::Vec<
            <GranularPermission as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<decodeFunctionSigsAndPermissionsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: decodeFunctionSigsAndPermissionsCall) -> Self {
                    (value.encoded, value.length)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for decodeFunctionSigsAndPermissionsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        encoded: tuple.0,
                        length: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<4>,
                >,
                alloy::sol_types::sol_data::Array<GranularPermission>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<4>>,
                alloy::sol_types::private::Vec<
                    <GranularPermission as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<decodeFunctionSigsAndPermissionsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: decodeFunctionSigsAndPermissionsReturn) -> Self {
                    (value.functionSigs, value.permissions)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for decodeFunctionSigsAndPermissionsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        functionSigs: tuple.0,
                        permissions: tuple.1,
                    }
                }
            }
        }
        impl decodeFunctionSigsAndPermissionsReturn {
            fn _tokenize(
                &self,
            ) -> <decodeFunctionSigsAndPermissionsCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::SolType>::tokenize(&self.functionSigs),
                    <alloy::sol_types::sol_data::Array<
                        GranularPermission,
                    > as alloy_sol_types::SolType>::tokenize(&self.permissions),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for decodeFunctionSigsAndPermissionsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = decodeFunctionSigsAndPermissionsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<4>,
                >,
                alloy::sol_types::sol_data::Array<GranularPermission>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "decodeFunctionSigsAndPermissions(bytes32,uint256)";
            const SELECTOR: [u8; 4] = [96u8, 151u8, 108u8, 75u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.encoded),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.length),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                decodeFunctionSigsAndPermissionsReturn::_tokenize(ret)
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
    /**Function with signature `encodeFunctionSigsAndPermissions(bytes4[],uint8[])` and selector `0x56f55117`.
```solidity
function encodeFunctionSigsAndPermissions(bytes4[] memory functionSigs, GranularPermission[] memory permissions) external pure returns (bytes32 encoded, uint256 length);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct encodeFunctionSigsAndPermissionsCall {
        #[allow(missing_docs)]
        pub functionSigs: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<4>,
        >,
        #[allow(missing_docs)]
        pub permissions: alloy::sol_types::private::Vec<
            <GranularPermission as alloy::sol_types::SolType>::RustType,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`encodeFunctionSigsAndPermissions(bytes4[],uint8[])`](encodeFunctionSigsAndPermissionsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct encodeFunctionSigsAndPermissionsReturn {
        #[allow(missing_docs)]
        pub encoded: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub length: alloy::sol_types::private::primitives::aliases::U256,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<4>,
                >,
                alloy::sol_types::sol_data::Array<GranularPermission>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<4>>,
                alloy::sol_types::private::Vec<
                    <GranularPermission as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<encodeFunctionSigsAndPermissionsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: encodeFunctionSigsAndPermissionsCall) -> Self {
                    (value.functionSigs, value.permissions)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for encodeFunctionSigsAndPermissionsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        functionSigs: tuple.0,
                        permissions: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<encodeFunctionSigsAndPermissionsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: encodeFunctionSigsAndPermissionsReturn) -> Self {
                    (value.encoded, value.length)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for encodeFunctionSigsAndPermissionsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        encoded: tuple.0,
                        length: tuple.1,
                    }
                }
            }
        }
        impl encodeFunctionSigsAndPermissionsReturn {
            fn _tokenize(
                &self,
            ) -> <encodeFunctionSigsAndPermissionsCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.encoded),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.length),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for encodeFunctionSigsAndPermissionsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<4>,
                >,
                alloy::sol_types::sol_data::Array<GranularPermission>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = encodeFunctionSigsAndPermissionsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "encodeFunctionSigsAndPermissions(bytes4[],uint8[])";
            const SELECTOR: [u8; 4] = [86u8, 245u8, 81u8, 23u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::SolType>::tokenize(&self.functionSigs),
                    <alloy::sol_types::sol_data::Array<
                        GranularPermission,
                    > as alloy_sol_types::SolType>::tokenize(&self.permissions),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                encodeFunctionSigsAndPermissionsReturn::_tokenize(ret)
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
    /**Function with signature `execTransactionFromModule(address,uint256,bytes,uint8)` and selector `0x468721a7`.
```solidity
function execTransactionFromModule(address to, uint256 value, bytes memory data, Enum.Operation operation) external returns (bool success);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct execTransactionFromModuleCall {
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub operation: <Enum::Operation as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`execTransactionFromModule(address,uint256,bytes,uint8)`](execTransactionFromModuleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct execTransactionFromModuleReturn {
        #[allow(missing_docs)]
        pub success: bool,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                Enum::Operation,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
                <Enum::Operation as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<execTransactionFromModuleCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: execTransactionFromModuleCall) -> Self {
                    (value.to, value.value, value.data, value.operation)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for execTransactionFromModuleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        to: tuple.0,
                        value: tuple.1,
                        data: tuple.2,
                        operation: tuple.3,
                    }
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
            impl ::core::convert::From<execTransactionFromModuleReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: execTransactionFromModuleReturn) -> Self {
                    (value.success,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for execTransactionFromModuleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { success: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for execTransactionFromModuleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                Enum::Operation,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "execTransactionFromModule(address,uint256,bytes,uint8)";
            const SELECTOR: [u8; 4] = [70u8, 135u8, 33u8, 167u8];
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
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                    <Enum::Operation as alloy_sol_types::SolType>::tokenize(
                        &self.operation,
                    ),
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
                        let r: execTransactionFromModuleReturn = r.into();
                        r.success
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
                        let r: execTransactionFromModuleReturn = r.into();
                        r.success
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `execTransactionFromModuleReturnData(address,uint256,bytes,uint8)` and selector `0x5229073f`.
```solidity
function execTransactionFromModuleReturnData(address to, uint256 value, bytes memory data, Enum.Operation operation) external returns (bool, bytes memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct execTransactionFromModuleReturnDataCall {
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub operation: <Enum::Operation as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`execTransactionFromModuleReturnData(address,uint256,bytes,uint8)`](execTransactionFromModuleReturnDataCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct execTransactionFromModuleReturnDataReturn {
        #[allow(missing_docs)]
        pub _0: bool,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Bytes,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                Enum::Operation,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
                <Enum::Operation as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<execTransactionFromModuleReturnDataCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: execTransactionFromModuleReturnDataCall) -> Self {
                    (value.to, value.value, value.data, value.operation)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for execTransactionFromModuleReturnDataCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        to: tuple.0,
                        value: tuple.1,
                        data: tuple.2,
                        operation: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool, alloy::sol_types::private::Bytes);
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
            impl ::core::convert::From<execTransactionFromModuleReturnDataReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: execTransactionFromModuleReturnDataReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for execTransactionFromModuleReturnDataReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        impl execTransactionFromModuleReturnDataReturn {
            fn _tokenize(
                &self,
            ) -> <execTransactionFromModuleReturnDataCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._1,
                    ),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for execTransactionFromModuleReturnDataCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                Enum::Operation,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = execTransactionFromModuleReturnDataReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Bytes,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "execTransactionFromModuleReturnData(address,uint256,bytes,uint8)";
            const SELECTOR: [u8; 4] = [82u8, 41u8, 7u8, 63u8];
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
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                    <Enum::Operation as alloy_sol_types::SolType>::tokenize(
                        &self.operation,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                execTransactionFromModuleReturnDataReturn::_tokenize(ret)
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
    /**Function with signature `getGranularPermissions(bytes32,bytes32)` and selector `0xdc446a4a`.
```solidity
function getGranularPermissions(bytes32 capabilityKey, bytes32 pairId) external view returns (GranularPermission);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getGranularPermissionsCall {
        #[allow(missing_docs)]
        pub capabilityKey: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub pairId: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getGranularPermissions(bytes32,bytes32)`](getGranularPermissionsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getGranularPermissionsReturn {
        #[allow(missing_docs)]
        pub _0: <GranularPermission as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<getGranularPermissionsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getGranularPermissionsCall) -> Self {
                    (value.capabilityKey, value.pairId)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getGranularPermissionsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        capabilityKey: tuple.0,
                        pairId: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (GranularPermission,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <GranularPermission as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getGranularPermissionsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getGranularPermissionsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getGranularPermissionsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getGranularPermissionsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <GranularPermission as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (GranularPermission,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getGranularPermissions(bytes32,bytes32)";
            const SELECTOR: [u8; 4] = [220u8, 68u8, 106u8, 74u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.capabilityKey),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.pairId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<GranularPermission as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getGranularPermissionsReturn = r.into();
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
                        let r: getGranularPermissionsReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getTargets()` and selector `0x63fe3b56`.
```solidity
function getTargets() external view returns (Target[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTargetsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getTargets()`](getTargetsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTargetsReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
            <Target as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getTargetsCall> for UnderlyingRustTuple<'_> {
                fn from(value: getTargetsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTargetsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Array<Target>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <Target as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getTargetsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getTargetsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTargetsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTargetsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                <Target as alloy::sol_types::SolType>::RustType,
            >;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Array<Target>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTargets()";
            const SELECTOR: [u8; 4] = [99u8, 254u8, 59u8, 86u8];
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
                        Target,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getTargetsReturn = r.into();
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
                        let r: getTargetsReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `includeNode(uint256)` and selector `0xb5736962`.
```solidity
function includeNode(Target nodeDefaultTarget) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct includeNodeCall {
        #[allow(missing_docs)]
        pub nodeDefaultTarget: <Target as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`includeNode(uint256)`](includeNodeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct includeNodeReturn {}
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
            type UnderlyingSolTuple<'a> = (Target,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Target as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<includeNodeCall> for UnderlyingRustTuple<'_> {
                fn from(value: includeNodeCall) -> Self {
                    (value.nodeDefaultTarget,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for includeNodeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { nodeDefaultTarget: tuple.0 }
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
            impl ::core::convert::From<includeNodeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: includeNodeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for includeNodeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl includeNodeReturn {
            fn _tokenize(
                &self,
            ) -> <includeNodeCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for includeNodeCall {
            type Parameters<'a> = (Target,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = includeNodeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "includeNode(uint256)";
            const SELECTOR: [u8; 4] = [181u8, 115u8, 105u8, 98u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <Target as alloy_sol_types::SolType>::tokenize(
                        &self.nodeDefaultTarget,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                includeNodeReturn::_tokenize(ret)
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
    /**Function with signature `includeNodes(address[])` and selector `0x110dcee7`.
```solidity
function includeNodes(address[] memory nodeAddresses) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct includeNodesCall {
        #[allow(missing_docs)]
        pub nodeAddresses: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`includeNodes(address[])`](includeNodesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct includeNodesReturn {}
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<includeNodesCall> for UnderlyingRustTuple<'_> {
                fn from(value: includeNodesCall) -> Self {
                    (value.nodeAddresses,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for includeNodesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { nodeAddresses: tuple.0 }
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
            impl ::core::convert::From<includeNodesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: includeNodesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for includeNodesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl includeNodesReturn {
            fn _tokenize(
                &self,
            ) -> <includeNodesCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for includeNodesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = includeNodesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "includeNodes(address[])";
            const SELECTOR: [u8; 4] = [17u8, 13u8, 206u8, 231u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.nodeAddresses),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                includeNodesReturn::_tokenize(ret)
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
    /**Function with signature `isHoprNodeManagementModule()` and selector `0x4a1ba408`.
```solidity
function isHoprNodeManagementModule() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isHoprNodeManagementModuleCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isHoprNodeManagementModule()`](isHoprNodeManagementModuleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isHoprNodeManagementModuleReturn {
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
            impl ::core::convert::From<isHoprNodeManagementModuleCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isHoprNodeManagementModuleCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isHoprNodeManagementModuleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<isHoprNodeManagementModuleReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isHoprNodeManagementModuleReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isHoprNodeManagementModuleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isHoprNodeManagementModuleCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isHoprNodeManagementModule()";
            const SELECTOR: [u8; 4] = [74u8, 27u8, 164u8, 8u8];
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
                        let r: isHoprNodeManagementModuleReturn = r.into();
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
                        let r: isHoprNodeManagementModuleReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isNode(address)` and selector `0x01750152`.
```solidity
function isNode(address nodeAddress) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isNodeCall {
        #[allow(missing_docs)]
        pub nodeAddress: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isNode(address)`](isNodeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isNodeReturn {
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
            impl ::core::convert::From<isNodeCall> for UnderlyingRustTuple<'_> {
                fn from(value: isNodeCall) -> Self {
                    (value.nodeAddress,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isNodeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { nodeAddress: tuple.0 }
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
            impl ::core::convert::From<isNodeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isNodeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isNodeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isNodeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isNode(address)";
            const SELECTOR: [u8; 4] = [1u8, 117u8, 1u8, 82u8];
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
                        &self.nodeAddress,
                    ),
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
                        let r: isNodeReturn = r.into();
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
                        let r: isNodeReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `multisend()` and selector `0x294402cc`.
```solidity
function multisend() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct multisendCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`multisend()`](multisendCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct multisendReturn {
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
            impl ::core::convert::From<multisendCall> for UnderlyingRustTuple<'_> {
                fn from(value: multisendCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for multisendCall {
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
            impl ::core::convert::From<multisendReturn> for UnderlyingRustTuple<'_> {
                fn from(value: multisendReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for multisendReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for multisendCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "multisend()";
            const SELECTOR: [u8; 4] = [41u8, 68u8, 2u8, 204u8];
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
                        let r: multisendReturn = r.into();
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
                        let r: multisendReturn = r.into();
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
    /**Function with signature `removeNode(address)` and selector `0xb2b99ec9`.
```solidity
function removeNode(address nodeAddress) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeNodeCall {
        #[allow(missing_docs)]
        pub nodeAddress: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`removeNode(address)`](removeNodeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeNodeReturn {}
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
            impl ::core::convert::From<removeNodeCall> for UnderlyingRustTuple<'_> {
                fn from(value: removeNodeCall) -> Self {
                    (value.nodeAddress,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeNodeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { nodeAddress: tuple.0 }
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
            impl ::core::convert::From<removeNodeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: removeNodeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeNodeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl removeNodeReturn {
            fn _tokenize(
                &self,
            ) -> <removeNodeCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removeNodeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeNodeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removeNode(address)";
            const SELECTOR: [u8; 4] = [178u8, 185u8, 158u8, 201u8];
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
                        &self.nodeAddress,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                removeNodeReturn::_tokenize(ret)
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
    /**Function with signature `revokeTarget(address)` and selector `0x3401cde8`.
```solidity
function revokeTarget(address targetAddress) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct revokeTargetCall {
        #[allow(missing_docs)]
        pub targetAddress: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`revokeTarget(address)`](revokeTargetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct revokeTargetReturn {}
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
            impl ::core::convert::From<revokeTargetCall> for UnderlyingRustTuple<'_> {
                fn from(value: revokeTargetCall) -> Self {
                    (value.targetAddress,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for revokeTargetCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { targetAddress: tuple.0 }
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
            impl ::core::convert::From<revokeTargetReturn> for UnderlyingRustTuple<'_> {
                fn from(value: revokeTargetReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for revokeTargetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl revokeTargetReturn {
            fn _tokenize(
                &self,
            ) -> <revokeTargetCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for revokeTargetCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = revokeTargetReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "revokeTarget(address)";
            const SELECTOR: [u8; 4] = [52u8, 1u8, 205u8, 232u8];
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
                        &self.targetAddress,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                revokeTargetReturn::_tokenize(ret)
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
    /**Function with signature `scopeChannelsCapabilities(address,bytes32,bytes32)` and selector `0xfa19501d`.
```solidity
function scopeChannelsCapabilities(address targetAddress, bytes32 channelId, bytes32 encodedSigsPermissions) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct scopeChannelsCapabilitiesCall {
        #[allow(missing_docs)]
        pub targetAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub channelId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub encodedSigsPermissions: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`scopeChannelsCapabilities(address,bytes32,bytes32)`](scopeChannelsCapabilitiesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct scopeChannelsCapabilitiesReturn {}
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
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<scopeChannelsCapabilitiesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: scopeChannelsCapabilitiesCall) -> Self {
                    (value.targetAddress, value.channelId, value.encodedSigsPermissions)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for scopeChannelsCapabilitiesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetAddress: tuple.0,
                        channelId: tuple.1,
                        encodedSigsPermissions: tuple.2,
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
            impl ::core::convert::From<scopeChannelsCapabilitiesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: scopeChannelsCapabilitiesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for scopeChannelsCapabilitiesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl scopeChannelsCapabilitiesReturn {
            fn _tokenize(
                &self,
            ) -> <scopeChannelsCapabilitiesCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for scopeChannelsCapabilitiesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = scopeChannelsCapabilitiesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "scopeChannelsCapabilities(address,bytes32,bytes32)";
            const SELECTOR: [u8; 4] = [250u8, 25u8, 80u8, 29u8];
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
                        &self.targetAddress,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.channelId),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.encodedSigsPermissions,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                scopeChannelsCapabilitiesReturn::_tokenize(ret)
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
    /**Function with signature `scopeSendCapability(address,address,uint8)` and selector `0xc68c3a83`.
```solidity
function scopeSendCapability(address nodeAddress, address beneficiary, GranularPermission permission) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct scopeSendCapabilityCall {
        #[allow(missing_docs)]
        pub nodeAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub beneficiary: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub permission: <GranularPermission as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`scopeSendCapability(address,address,uint8)`](scopeSendCapabilityCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct scopeSendCapabilityReturn {}
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
                GranularPermission,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                <GranularPermission as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<scopeSendCapabilityCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: scopeSendCapabilityCall) -> Self {
                    (value.nodeAddress, value.beneficiary, value.permission)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for scopeSendCapabilityCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        nodeAddress: tuple.0,
                        beneficiary: tuple.1,
                        permission: tuple.2,
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
            impl ::core::convert::From<scopeSendCapabilityReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: scopeSendCapabilityReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for scopeSendCapabilityReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl scopeSendCapabilityReturn {
            fn _tokenize(
                &self,
            ) -> <scopeSendCapabilityCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for scopeSendCapabilityCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                GranularPermission,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = scopeSendCapabilityReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "scopeSendCapability(address,address,uint8)";
            const SELECTOR: [u8; 4] = [198u8, 140u8, 58u8, 131u8];
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
                        &self.nodeAddress,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.beneficiary,
                    ),
                    <GranularPermission as alloy_sol_types::SolType>::tokenize(
                        &self.permission,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                scopeSendCapabilityReturn::_tokenize(ret)
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
    /**Function with signature `scopeTargetChannels(uint256)` and selector `0x739c4b08`.
```solidity
function scopeTargetChannels(Target defaultTarget) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct scopeTargetChannelsCall {
        #[allow(missing_docs)]
        pub defaultTarget: <Target as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`scopeTargetChannels(uint256)`](scopeTargetChannelsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct scopeTargetChannelsReturn {}
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
            type UnderlyingSolTuple<'a> = (Target,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Target as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<scopeTargetChannelsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: scopeTargetChannelsCall) -> Self {
                    (value.defaultTarget,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for scopeTargetChannelsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { defaultTarget: tuple.0 }
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
            impl ::core::convert::From<scopeTargetChannelsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: scopeTargetChannelsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for scopeTargetChannelsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl scopeTargetChannelsReturn {
            fn _tokenize(
                &self,
            ) -> <scopeTargetChannelsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for scopeTargetChannelsCall {
            type Parameters<'a> = (Target,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = scopeTargetChannelsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "scopeTargetChannels(uint256)";
            const SELECTOR: [u8; 4] = [115u8, 156u8, 75u8, 8u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<Target as alloy_sol_types::SolType>::tokenize(&self.defaultTarget),)
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                scopeTargetChannelsReturn::_tokenize(ret)
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
    /**Function with signature `scopeTargetSend(uint256)` and selector `0xdc06109d`.
```solidity
function scopeTargetSend(Target defaultTarget) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct scopeTargetSendCall {
        #[allow(missing_docs)]
        pub defaultTarget: <Target as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`scopeTargetSend(uint256)`](scopeTargetSendCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct scopeTargetSendReturn {}
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
            type UnderlyingSolTuple<'a> = (Target,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Target as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<scopeTargetSendCall> for UnderlyingRustTuple<'_> {
                fn from(value: scopeTargetSendCall) -> Self {
                    (value.defaultTarget,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for scopeTargetSendCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { defaultTarget: tuple.0 }
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
            impl ::core::convert::From<scopeTargetSendReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: scopeTargetSendReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for scopeTargetSendReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl scopeTargetSendReturn {
            fn _tokenize(
                &self,
            ) -> <scopeTargetSendCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for scopeTargetSendCall {
            type Parameters<'a> = (Target,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = scopeTargetSendReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "scopeTargetSend(uint256)";
            const SELECTOR: [u8; 4] = [220u8, 6u8, 16u8, 157u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<Target as alloy_sol_types::SolType>::tokenize(&self.defaultTarget),)
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                scopeTargetSendReturn::_tokenize(ret)
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
    /**Function with signature `scopeTargetToken(uint256)` and selector `0xa76c9a2f`.
```solidity
function scopeTargetToken(Target defaultTarget) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct scopeTargetTokenCall {
        #[allow(missing_docs)]
        pub defaultTarget: <Target as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`scopeTargetToken(uint256)`](scopeTargetTokenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct scopeTargetTokenReturn {}
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
            type UnderlyingSolTuple<'a> = (Target,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Target as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<scopeTargetTokenCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: scopeTargetTokenCall) -> Self {
                    (value.defaultTarget,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for scopeTargetTokenCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { defaultTarget: tuple.0 }
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
            impl ::core::convert::From<scopeTargetTokenReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: scopeTargetTokenReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for scopeTargetTokenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl scopeTargetTokenReturn {
            fn _tokenize(
                &self,
            ) -> <scopeTargetTokenCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for scopeTargetTokenCall {
            type Parameters<'a> = (Target,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = scopeTargetTokenReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "scopeTargetToken(uint256)";
            const SELECTOR: [u8; 4] = [167u8, 108u8, 154u8, 47u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<Target as alloy_sol_types::SolType>::tokenize(&self.defaultTarget),)
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                scopeTargetTokenReturn::_tokenize(ret)
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
    /**Function with signature `scopeTokenCapabilities(address,address,address,bytes32)` and selector `0xc68605c8`.
```solidity
function scopeTokenCapabilities(address nodeAddress, address targetAddress, address beneficiary, bytes32 encodedSigsPermissions) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct scopeTokenCapabilitiesCall {
        #[allow(missing_docs)]
        pub nodeAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub targetAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub beneficiary: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub encodedSigsPermissions: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`scopeTokenCapabilities(address,address,address,bytes32)`](scopeTokenCapabilitiesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct scopeTokenCapabilitiesReturn {}
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
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<scopeTokenCapabilitiesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: scopeTokenCapabilitiesCall) -> Self {
                    (
                        value.nodeAddress,
                        value.targetAddress,
                        value.beneficiary,
                        value.encodedSigsPermissions,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for scopeTokenCapabilitiesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        nodeAddress: tuple.0,
                        targetAddress: tuple.1,
                        beneficiary: tuple.2,
                        encodedSigsPermissions: tuple.3,
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
            impl ::core::convert::From<scopeTokenCapabilitiesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: scopeTokenCapabilitiesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for scopeTokenCapabilitiesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl scopeTokenCapabilitiesReturn {
            fn _tokenize(
                &self,
            ) -> <scopeTokenCapabilitiesCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for scopeTokenCapabilitiesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = scopeTokenCapabilitiesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "scopeTokenCapabilities(address,address,address,bytes32)";
            const SELECTOR: [u8; 4] = [198u8, 134u8, 5u8, 200u8];
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
                        &self.nodeAddress,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.targetAddress,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.beneficiary,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.encodedSigsPermissions,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                scopeTokenCapabilitiesReturn::_tokenize(ret)
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
    /**Function with signature `setMultisend(address)` and selector `0x8b95eccd`.
```solidity
function setMultisend(address _multisend) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setMultisendCall {
        #[allow(missing_docs)]
        pub _multisend: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setMultisend(address)`](setMultisendCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setMultisendReturn {}
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
            impl ::core::convert::From<setMultisendCall> for UnderlyingRustTuple<'_> {
                fn from(value: setMultisendCall) -> Self {
                    (value._multisend,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setMultisendCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _multisend: tuple.0 }
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
            impl ::core::convert::From<setMultisendReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setMultisendReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setMultisendReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setMultisendReturn {
            fn _tokenize(
                &self,
            ) -> <setMultisendCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setMultisendCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setMultisendReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setMultisend(address)";
            const SELECTOR: [u8; 4] = [139u8, 149u8, 236u8, 205u8];
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
                        &self._multisend,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setMultisendReturn::_tokenize(ret)
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
    /**Function with signature `tryGetTarget(address)` and selector `0xdf4e6f8a`.
```solidity
function tryGetTarget(address targetAddress) external view returns (bool, Target);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tryGetTargetCall {
        #[allow(missing_docs)]
        pub targetAddress: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`tryGetTarget(address)`](tryGetTargetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tryGetTargetReturn {
        #[allow(missing_docs)]
        pub _0: bool,
        #[allow(missing_docs)]
        pub _1: <Target as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<tryGetTargetCall> for UnderlyingRustTuple<'_> {
                fn from(value: tryGetTargetCall) -> Self {
                    (value.targetAddress,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for tryGetTargetCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { targetAddress: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool, Target);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                bool,
                <Target as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<tryGetTargetReturn> for UnderlyingRustTuple<'_> {
                fn from(value: tryGetTargetReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for tryGetTargetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        impl tryGetTargetReturn {
            fn _tokenize(
                &self,
            ) -> <tryGetTargetCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                    <Target as alloy_sol_types::SolType>::tokenize(&self._1),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for tryGetTargetCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = tryGetTargetReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool, Target);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "tryGetTarget(address)";
            const SELECTOR: [u8; 4] = [223u8, 78u8, 111u8, 138u8];
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
                        &self.targetAddress,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                tryGetTargetReturn::_tokenize(ret)
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
    ///Container for all the [`HoprNodeManagementModule`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum HoprNodeManagementModuleCalls {
        #[allow(missing_docs)]
        UPGRADE_INTERFACE_VERSION(UPGRADE_INTERFACE_VERSIONCall),
        #[allow(missing_docs)]
        VERSION(VERSIONCall),
        #[allow(missing_docs)]
        addChannelsAndTokenTarget(addChannelsAndTokenTargetCall),
        #[allow(missing_docs)]
        addNode(addNodeCall),
        #[allow(missing_docs)]
        addNodes(addNodesCall),
        #[allow(missing_docs)]
        decodeFunctionSigsAndPermissions(decodeFunctionSigsAndPermissionsCall),
        #[allow(missing_docs)]
        encodeFunctionSigsAndPermissions(encodeFunctionSigsAndPermissionsCall),
        #[allow(missing_docs)]
        execTransactionFromModule(execTransactionFromModuleCall),
        #[allow(missing_docs)]
        execTransactionFromModuleReturnData(execTransactionFromModuleReturnDataCall),
        #[allow(missing_docs)]
        getGranularPermissions(getGranularPermissionsCall),
        #[allow(missing_docs)]
        getTargets(getTargetsCall),
        #[allow(missing_docs)]
        includeNode(includeNodeCall),
        #[allow(missing_docs)]
        includeNodes(includeNodesCall),
        #[allow(missing_docs)]
        initialize(initializeCall),
        #[allow(missing_docs)]
        isHoprNodeManagementModule(isHoprNodeManagementModuleCall),
        #[allow(missing_docs)]
        isNode(isNodeCall),
        #[allow(missing_docs)]
        multisend(multisendCall),
        #[allow(missing_docs)]
        owner(ownerCall),
        #[allow(missing_docs)]
        proxiableUUID(proxiableUUIDCall),
        #[allow(missing_docs)]
        removeNode(removeNodeCall),
        #[allow(missing_docs)]
        renounceOwnership(renounceOwnershipCall),
        #[allow(missing_docs)]
        revokeTarget(revokeTargetCall),
        #[allow(missing_docs)]
        scopeChannelsCapabilities(scopeChannelsCapabilitiesCall),
        #[allow(missing_docs)]
        scopeSendCapability(scopeSendCapabilityCall),
        #[allow(missing_docs)]
        scopeTargetChannels(scopeTargetChannelsCall),
        #[allow(missing_docs)]
        scopeTargetSend(scopeTargetSendCall),
        #[allow(missing_docs)]
        scopeTargetToken(scopeTargetTokenCall),
        #[allow(missing_docs)]
        scopeTokenCapabilities(scopeTokenCapabilitiesCall),
        #[allow(missing_docs)]
        setMultisend(setMultisendCall),
        #[allow(missing_docs)]
        transferOwnership(transferOwnershipCall),
        #[allow(missing_docs)]
        tryGetTarget(tryGetTargetCall),
        #[allow(missing_docs)]
        upgradeToAndCall(upgradeToAndCallCall),
    }
    impl HoprNodeManagementModuleCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [1u8, 117u8, 1u8, 82u8],
            [17u8, 13u8, 206u8, 231u8],
            [41u8, 68u8, 2u8, 204u8],
            [52u8, 1u8, 205u8, 232u8],
            [67u8, 159u8, 171u8, 145u8],
            [70u8, 135u8, 33u8, 167u8],
            [74u8, 27u8, 164u8, 8u8],
            [79u8, 30u8, 242u8, 134u8],
            [82u8, 41u8, 7u8, 63u8],
            [82u8, 209u8, 144u8, 45u8],
            [86u8, 245u8, 81u8, 23u8],
            [96u8, 151u8, 108u8, 75u8],
            [99u8, 254u8, 59u8, 86u8],
            [113u8, 80u8, 24u8, 166u8],
            [115u8, 156u8, 75u8, 8u8],
            [139u8, 149u8, 236u8, 205u8],
            [141u8, 165u8, 203u8, 91u8],
            [157u8, 149u8, 241u8, 204u8],
            [162u8, 69u8, 15u8, 137u8],
            [167u8, 108u8, 154u8, 47u8],
            [173u8, 60u8, 177u8, 204u8],
            [178u8, 185u8, 158u8, 201u8],
            [181u8, 115u8, 105u8, 98u8],
            [198u8, 134u8, 5u8, 200u8],
            [198u8, 140u8, 58u8, 131u8],
            [220u8, 6u8, 16u8, 157u8],
            [220u8, 68u8, 106u8, 74u8],
            [223u8, 78u8, 111u8, 138u8],
            [223u8, 150u8, 32u8, 235u8],
            [242u8, 253u8, 227u8, 139u8],
            [250u8, 25u8, 80u8, 29u8],
            [255u8, 161u8, 173u8, 116u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(isNode),
            ::core::stringify!(includeNodes),
            ::core::stringify!(multisend),
            ::core::stringify!(revokeTarget),
            ::core::stringify!(initialize),
            ::core::stringify!(execTransactionFromModule),
            ::core::stringify!(isHoprNodeManagementModule),
            ::core::stringify!(upgradeToAndCall),
            ::core::stringify!(execTransactionFromModuleReturnData),
            ::core::stringify!(proxiableUUID),
            ::core::stringify!(encodeFunctionSigsAndPermissions),
            ::core::stringify!(decodeFunctionSigsAndPermissions),
            ::core::stringify!(getTargets),
            ::core::stringify!(renounceOwnership),
            ::core::stringify!(scopeTargetChannels),
            ::core::stringify!(setMultisend),
            ::core::stringify!(owner),
            ::core::stringify!(addNode),
            ::core::stringify!(addChannelsAndTokenTarget),
            ::core::stringify!(scopeTargetToken),
            ::core::stringify!(UPGRADE_INTERFACE_VERSION),
            ::core::stringify!(removeNode),
            ::core::stringify!(includeNode),
            ::core::stringify!(scopeTokenCapabilities),
            ::core::stringify!(scopeSendCapability),
            ::core::stringify!(scopeTargetSend),
            ::core::stringify!(getGranularPermissions),
            ::core::stringify!(tryGetTarget),
            ::core::stringify!(addNodes),
            ::core::stringify!(transferOwnership),
            ::core::stringify!(scopeChannelsCapabilities),
            ::core::stringify!(VERSION),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <isNodeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <includeNodesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <multisendCall as alloy_sol_types::SolCall>::SIGNATURE,
            <revokeTargetCall as alloy_sol_types::SolCall>::SIGNATURE,
            <initializeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <execTransactionFromModuleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isHoprNodeManagementModuleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <upgradeToAndCallCall as alloy_sol_types::SolCall>::SIGNATURE,
            <execTransactionFromModuleReturnDataCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proxiableUUIDCall as alloy_sol_types::SolCall>::SIGNATURE,
            <encodeFunctionSigsAndPermissionsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <decodeFunctionSigsAndPermissionsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getTargetsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <renounceOwnershipCall as alloy_sol_types::SolCall>::SIGNATURE,
            <scopeTargetChannelsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setMultisendCall as alloy_sol_types::SolCall>::SIGNATURE,
            <ownerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <addNodeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <addChannelsAndTokenTargetCall as alloy_sol_types::SolCall>::SIGNATURE,
            <scopeTargetTokenCall as alloy_sol_types::SolCall>::SIGNATURE,
            <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SIGNATURE,
            <removeNodeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <includeNodeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <scopeTokenCapabilitiesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <scopeSendCapabilityCall as alloy_sol_types::SolCall>::SIGNATURE,
            <scopeTargetSendCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getGranularPermissionsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <tryGetTargetCall as alloy_sol_types::SolCall>::SIGNATURE,
            <addNodesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <transferOwnershipCall as alloy_sol_types::SolCall>::SIGNATURE,
            <scopeChannelsCapabilitiesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <VERSIONCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for HoprNodeManagementModuleCalls {
        const NAME: &'static str = "HoprNodeManagementModuleCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 32usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::UPGRADE_INTERFACE_VERSION(_) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::VERSION(_) => <VERSIONCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::addChannelsAndTokenTarget(_) => {
                    <addChannelsAndTokenTargetCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addNode(_) => <addNodeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::addNodes(_) => <addNodesCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::decodeFunctionSigsAndPermissions(_) => {
                    <decodeFunctionSigsAndPermissionsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::encodeFunctionSigsAndPermissions(_) => {
                    <encodeFunctionSigsAndPermissionsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::execTransactionFromModule(_) => {
                    <execTransactionFromModuleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::execTransactionFromModuleReturnData(_) => {
                    <execTransactionFromModuleReturnDataCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getGranularPermissions(_) => {
                    <getGranularPermissionsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTargets(_) => {
                    <getTargetsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::includeNode(_) => {
                    <includeNodeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::includeNodes(_) => {
                    <includeNodesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isHoprNodeManagementModule(_) => {
                    <isHoprNodeManagementModuleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isNode(_) => <isNodeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::multisend(_) => {
                    <multisendCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::proxiableUUID(_) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removeNode(_) => {
                    <removeNodeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::revokeTarget(_) => {
                    <revokeTargetCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::scopeChannelsCapabilities(_) => {
                    <scopeChannelsCapabilitiesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::scopeSendCapability(_) => {
                    <scopeSendCapabilityCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::scopeTargetChannels(_) => {
                    <scopeTargetChannelsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::scopeTargetSend(_) => {
                    <scopeTargetSendCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::scopeTargetToken(_) => {
                    <scopeTargetTokenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::scopeTokenCapabilities(_) => {
                    <scopeTokenCapabilitiesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setMultisend(_) => {
                    <setMultisendCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::tryGetTarget(_) => {
                    <tryGetTargetCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls>] = &[
                {
                    fn isNode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <isNodeCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(HoprNodeManagementModuleCalls::isNode)
                    }
                    isNode
                },
                {
                    fn includeNodes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <includeNodesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::includeNodes)
                    }
                    includeNodes
                },
                {
                    fn multisend(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <multisendCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(HoprNodeManagementModuleCalls::multisend)
                    }
                    multisend
                },
                {
                    fn revokeTarget(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <revokeTargetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::revokeTarget)
                    }
                    revokeTarget
                },
                {
                    fn initialize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::initialize)
                    }
                    initialize
                },
                {
                    fn execTransactionFromModule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <execTransactionFromModuleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                HoprNodeManagementModuleCalls::execTransactionFromModule,
                            )
                    }
                    execTransactionFromModule
                },
                {
                    fn isHoprNodeManagementModule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <isHoprNodeManagementModuleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                HoprNodeManagementModuleCalls::isHoprNodeManagementModule,
                            )
                    }
                    isHoprNodeManagementModule
                },
                {
                    fn upgradeToAndCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::upgradeToAndCall)
                    }
                    upgradeToAndCall
                },
                {
                    fn execTransactionFromModuleReturnData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <execTransactionFromModuleReturnDataCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                HoprNodeManagementModuleCalls::execTransactionFromModuleReturnData,
                            )
                    }
                    execTransactionFromModuleReturnData
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn encodeFunctionSigsAndPermissions(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <encodeFunctionSigsAndPermissionsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                HoprNodeManagementModuleCalls::encodeFunctionSigsAndPermissions,
                            )
                    }
                    encodeFunctionSigsAndPermissions
                },
                {
                    fn decodeFunctionSigsAndPermissions(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <decodeFunctionSigsAndPermissionsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                HoprNodeManagementModuleCalls::decodeFunctionSigsAndPermissions,
                            )
                    }
                    decodeFunctionSigsAndPermissions
                },
                {
                    fn getTargets(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <getTargetsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::getTargets)
                    }
                    getTargets
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn scopeTargetChannels(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <scopeTargetChannelsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::scopeTargetChannels)
                    }
                    scopeTargetChannels
                },
                {
                    fn setMultisend(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <setMultisendCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::setMultisend)
                    }
                    setMultisend
                },
                {
                    fn owner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(HoprNodeManagementModuleCalls::owner)
                    }
                    owner
                },
                {
                    fn addNode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <addNodeCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(HoprNodeManagementModuleCalls::addNode)
                    }
                    addNode
                },
                {
                    fn addChannelsAndTokenTarget(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <addChannelsAndTokenTargetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                HoprNodeManagementModuleCalls::addChannelsAndTokenTarget,
                            )
                    }
                    addChannelsAndTokenTarget
                },
                {
                    fn scopeTargetToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <scopeTargetTokenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::scopeTargetToken)
                    }
                    scopeTargetToken
                },
                {
                    fn UPGRADE_INTERFACE_VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                HoprNodeManagementModuleCalls::UPGRADE_INTERFACE_VERSION,
                            )
                    }
                    UPGRADE_INTERFACE_VERSION
                },
                {
                    fn removeNode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <removeNodeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::removeNode)
                    }
                    removeNode
                },
                {
                    fn includeNode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <includeNodeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::includeNode)
                    }
                    includeNode
                },
                {
                    fn scopeTokenCapabilities(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <scopeTokenCapabilitiesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::scopeTokenCapabilities)
                    }
                    scopeTokenCapabilities
                },
                {
                    fn scopeSendCapability(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <scopeSendCapabilityCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::scopeSendCapability)
                    }
                    scopeSendCapability
                },
                {
                    fn scopeTargetSend(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <scopeTargetSendCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::scopeTargetSend)
                    }
                    scopeTargetSend
                },
                {
                    fn getGranularPermissions(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <getGranularPermissionsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::getGranularPermissions)
                    }
                    getGranularPermissions
                },
                {
                    fn tryGetTarget(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <tryGetTargetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::tryGetTarget)
                    }
                    tryGetTarget
                },
                {
                    fn addNodes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <addNodesCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(HoprNodeManagementModuleCalls::addNodes)
                    }
                    addNodes
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn scopeChannelsCapabilities(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <scopeChannelsCapabilitiesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                HoprNodeManagementModuleCalls::scopeChannelsCapabilities,
                            )
                    }
                    scopeChannelsCapabilities
                },
                {
                    fn VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(HoprNodeManagementModuleCalls::VERSION)
                    }
                    VERSION
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
            ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls>] = &[
                {
                    fn isNode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <isNodeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::isNode)
                    }
                    isNode
                },
                {
                    fn includeNodes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <includeNodesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::includeNodes)
                    }
                    includeNodes
                },
                {
                    fn multisend(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <multisendCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::multisend)
                    }
                    multisend
                },
                {
                    fn revokeTarget(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <revokeTargetCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::revokeTarget)
                    }
                    revokeTarget
                },
                {
                    fn initialize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::initialize)
                    }
                    initialize
                },
                {
                    fn execTransactionFromModule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <execTransactionFromModuleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                HoprNodeManagementModuleCalls::execTransactionFromModule,
                            )
                    }
                    execTransactionFromModule
                },
                {
                    fn isHoprNodeManagementModule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <isHoprNodeManagementModuleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                HoprNodeManagementModuleCalls::isHoprNodeManagementModule,
                            )
                    }
                    isHoprNodeManagementModule
                },
                {
                    fn upgradeToAndCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::upgradeToAndCall)
                    }
                    upgradeToAndCall
                },
                {
                    fn execTransactionFromModuleReturnData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <execTransactionFromModuleReturnDataCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                HoprNodeManagementModuleCalls::execTransactionFromModuleReturnData,
                            )
                    }
                    execTransactionFromModuleReturnData
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn encodeFunctionSigsAndPermissions(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <encodeFunctionSigsAndPermissionsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                HoprNodeManagementModuleCalls::encodeFunctionSigsAndPermissions,
                            )
                    }
                    encodeFunctionSigsAndPermissions
                },
                {
                    fn decodeFunctionSigsAndPermissions(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <decodeFunctionSigsAndPermissionsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                HoprNodeManagementModuleCalls::decodeFunctionSigsAndPermissions,
                            )
                    }
                    decodeFunctionSigsAndPermissions
                },
                {
                    fn getTargets(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <getTargetsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::getTargets)
                    }
                    getTargets
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn scopeTargetChannels(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <scopeTargetChannelsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::scopeTargetChannels)
                    }
                    scopeTargetChannels
                },
                {
                    fn setMultisend(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <setMultisendCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::setMultisend)
                    }
                    setMultisend
                },
                {
                    fn owner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::owner)
                    }
                    owner
                },
                {
                    fn addNode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <addNodeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::addNode)
                    }
                    addNode
                },
                {
                    fn addChannelsAndTokenTarget(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <addChannelsAndTokenTargetCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                HoprNodeManagementModuleCalls::addChannelsAndTokenTarget,
                            )
                    }
                    addChannelsAndTokenTarget
                },
                {
                    fn scopeTargetToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <scopeTargetTokenCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::scopeTargetToken)
                    }
                    scopeTargetToken
                },
                {
                    fn UPGRADE_INTERFACE_VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                HoprNodeManagementModuleCalls::UPGRADE_INTERFACE_VERSION,
                            )
                    }
                    UPGRADE_INTERFACE_VERSION
                },
                {
                    fn removeNode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <removeNodeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::removeNode)
                    }
                    removeNode
                },
                {
                    fn includeNode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <includeNodeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::includeNode)
                    }
                    includeNode
                },
                {
                    fn scopeTokenCapabilities(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <scopeTokenCapabilitiesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::scopeTokenCapabilities)
                    }
                    scopeTokenCapabilities
                },
                {
                    fn scopeSendCapability(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <scopeSendCapabilityCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::scopeSendCapability)
                    }
                    scopeSendCapability
                },
                {
                    fn scopeTargetSend(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <scopeTargetSendCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::scopeTargetSend)
                    }
                    scopeTargetSend
                },
                {
                    fn getGranularPermissions(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <getGranularPermissionsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::getGranularPermissions)
                    }
                    getGranularPermissions
                },
                {
                    fn tryGetTarget(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <tryGetTargetCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::tryGetTarget)
                    }
                    tryGetTarget
                },
                {
                    fn addNodes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <addNodesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::addNodes)
                    }
                    addNodes
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn scopeChannelsCapabilities(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <scopeChannelsCapabilitiesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                HoprNodeManagementModuleCalls::scopeChannelsCapabilities,
                            )
                    }
                    scopeChannelsCapabilities
                },
                {
                    fn VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleCalls> {
                        <VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleCalls::VERSION)
                    }
                    VERSION
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
                Self::UPGRADE_INTERFACE_VERSION(inner) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::VERSION(inner) => {
                    <VERSIONCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::addChannelsAndTokenTarget(inner) => {
                    <addChannelsAndTokenTargetCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::addNode(inner) => {
                    <addNodeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::addNodes(inner) => {
                    <addNodesCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::decodeFunctionSigsAndPermissions(inner) => {
                    <decodeFunctionSigsAndPermissionsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::encodeFunctionSigsAndPermissions(inner) => {
                    <encodeFunctionSigsAndPermissionsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::execTransactionFromModule(inner) => {
                    <execTransactionFromModuleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::execTransactionFromModuleReturnData(inner) => {
                    <execTransactionFromModuleReturnDataCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getGranularPermissions(inner) => {
                    <getGranularPermissionsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTargets(inner) => {
                    <getTargetsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::includeNode(inner) => {
                    <includeNodeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::includeNodes(inner) => {
                    <includeNodesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isHoprNodeManagementModule(inner) => {
                    <isHoprNodeManagementModuleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isNode(inner) => {
                    <isNodeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::multisend(inner) => {
                    <multisendCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::proxiableUUID(inner) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::removeNode(inner) => {
                    <removeNodeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::revokeTarget(inner) => {
                    <revokeTargetCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::scopeChannelsCapabilities(inner) => {
                    <scopeChannelsCapabilitiesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::scopeSendCapability(inner) => {
                    <scopeSendCapabilityCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::scopeTargetChannels(inner) => {
                    <scopeTargetChannelsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::scopeTargetSend(inner) => {
                    <scopeTargetSendCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::scopeTargetToken(inner) => {
                    <scopeTargetTokenCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::scopeTokenCapabilities(inner) => {
                    <scopeTokenCapabilitiesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setMultisend(inner) => {
                    <setMultisendCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::tryGetTarget(inner) => {
                    <tryGetTargetCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::UPGRADE_INTERFACE_VERSION(inner) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::VERSION(inner) => {
                    <VERSIONCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::addChannelsAndTokenTarget(inner) => {
                    <addChannelsAndTokenTargetCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::addNode(inner) => {
                    <addNodeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::addNodes(inner) => {
                    <addNodesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::decodeFunctionSigsAndPermissions(inner) => {
                    <decodeFunctionSigsAndPermissionsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::encodeFunctionSigsAndPermissions(inner) => {
                    <encodeFunctionSigsAndPermissionsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::execTransactionFromModule(inner) => {
                    <execTransactionFromModuleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::execTransactionFromModuleReturnData(inner) => {
                    <execTransactionFromModuleReturnDataCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getGranularPermissions(inner) => {
                    <getGranularPermissionsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getTargets(inner) => {
                    <getTargetsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::includeNode(inner) => {
                    <includeNodeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::includeNodes(inner) => {
                    <includeNodesCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::isHoprNodeManagementModule(inner) => {
                    <isHoprNodeManagementModuleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isNode(inner) => {
                    <isNodeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::multisend(inner) => {
                    <multisendCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::removeNode(inner) => {
                    <removeNodeCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::revokeTarget(inner) => {
                    <revokeTargetCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::scopeChannelsCapabilities(inner) => {
                    <scopeChannelsCapabilitiesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::scopeSendCapability(inner) => {
                    <scopeSendCapabilityCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::scopeTargetChannels(inner) => {
                    <scopeTargetChannelsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::scopeTargetSend(inner) => {
                    <scopeTargetSendCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::scopeTargetToken(inner) => {
                    <scopeTargetTokenCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::scopeTokenCapabilities(inner) => {
                    <scopeTokenCapabilitiesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setMultisend(inner) => {
                    <setMultisendCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::tryGetTarget(inner) => {
                    <tryGetTargetCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`HoprNodeManagementModule`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum HoprNodeManagementModuleErrors {
        #[allow(missing_docs)]
        AddressEmptyCode(AddressEmptyCode),
        #[allow(missing_docs)]
        AddressIsZero(AddressIsZero),
        #[allow(missing_docs)]
        ArrayTooLong(ArrayTooLong),
        #[allow(missing_docs)]
        ArraysDifferentLength(ArraysDifferentLength),
        #[allow(missing_docs)]
        CalldataOutOfBounds(CalldataOutOfBounds),
        #[allow(missing_docs)]
        CannotChangeOwner(CannotChangeOwner),
        #[allow(missing_docs)]
        DefaultPermissionRejected(DefaultPermissionRejected),
        #[allow(missing_docs)]
        DelegateCallNotAllowed(DelegateCallNotAllowed),
        #[allow(missing_docs)]
        ERC1967InvalidImplementation(ERC1967InvalidImplementation),
        #[allow(missing_docs)]
        ERC1967NonPayable(ERC1967NonPayable),
        #[allow(missing_docs)]
        FailedCall(FailedCall),
        #[allow(missing_docs)]
        FailedToSendEthToNode(FailedToSendEthToNode),
        #[allow(missing_docs)]
        FunctionSignatureTooShort(FunctionSignatureTooShort),
        #[allow(missing_docs)]
        GranularPermissionRejected(GranularPermissionRejected),
        #[allow(missing_docs)]
        InvalidInitialization(InvalidInitialization),
        #[allow(missing_docs)]
        LengthIsZero(LengthIsZero),
        #[allow(missing_docs)]
        NoMembership(NoMembership),
        #[allow(missing_docs)]
        NodePermissionRejected(NodePermissionRejected),
        #[allow(missing_docs)]
        NonExistentKey(NonExistentKey),
        #[allow(missing_docs)]
        NotInitializing(NotInitializing),
        #[allow(missing_docs)]
        OwnableInvalidOwner(OwnableInvalidOwner),
        #[allow(missing_docs)]
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        #[allow(missing_docs)]
        ParameterNotAllowed(ParameterNotAllowed),
        #[allow(missing_docs)]
        PermissionNotConfigured(PermissionNotConfigured),
        #[allow(missing_docs)]
        PermissionNotFound(PermissionNotFound),
        #[allow(missing_docs)]
        SafeMultisendSameAddress(SafeMultisendSameAddress),
        #[allow(missing_docs)]
        SendNotAllowed(SendNotAllowed),
        #[allow(missing_docs)]
        TargetAddressNotAllowed(TargetAddressNotAllowed),
        #[allow(missing_docs)]
        TargetIsNotScoped(TargetIsNotScoped),
        #[allow(missing_docs)]
        TargetIsScoped(TargetIsScoped),
        #[allow(missing_docs)]
        TooManyCapabilities(TooManyCapabilities),
        #[allow(missing_docs)]
        UUPSUnauthorizedCallContext(UUPSUnauthorizedCallContext),
        #[allow(missing_docs)]
        UUPSUnsupportedProxiableUUID(UUPSUnsupportedProxiableUUID),
        #[allow(missing_docs)]
        UnacceptableMultiSendOffset(UnacceptableMultiSendOffset),
        #[allow(missing_docs)]
        WithMembership(WithMembership),
    }
    impl HoprNodeManagementModuleErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [9u8, 233u8, 205u8, 73u8],
            [12u8, 113u8, 236u8, 23u8],
            [13u8, 137u8, 67u8, 142u8],
            [17u8, 140u8, 218u8, 167u8],
            [30u8, 79u8, 189u8, 247u8],
            [45u8, 5u8, 25u8, 173u8],
            [49u8, 233u8, 130u8, 70u8],
            [70u8, 132u8, 193u8, 34u8],
            [70u8, 173u8, 69u8, 136u8],
            [74u8, 137u8, 3u8, 33u8],
            [76u8, 156u8, 140u8, 227u8],
            [88u8, 114u8, 48u8, 55u8],
            [89u8, 138u8, 14u8, 33u8],
            [110u8, 176u8, 49u8, 95u8],
            [116u8, 38u8, 56u8, 180u8],
            [116u8, 244u8, 213u8, 55u8],
            [126u8, 209u8, 17u8, 55u8],
            [134u8, 77u8, 209u8, 231u8],
            [134u8, 121u8, 21u8, 171u8],
            [153u8, 150u8, 179u8, 21u8],
            [154u8, 115u8, 238u8, 153u8],
            [170u8, 29u8, 73u8, 164u8],
            [179u8, 152u8, 151u8, 159u8],
            [180u8, 74u8, 249u8, 175u8],
            [189u8, 38u8, 204u8, 56u8],
            [214u8, 189u8, 162u8, 117u8],
            [215u8, 230u8, 188u8, 248u8],
            [216u8, 69u8, 90u8, 19u8],
            [224u8, 124u8, 141u8, 186u8],
            [227u8, 160u8, 90u8, 148u8],
            [232u8, 192u8, 125u8, 42u8],
            [239u8, 52u8, 64u8, 172u8],
            [249u8, 46u8, 232u8, 169u8],
            [253u8, 103u8, 14u8, 190u8],
            [253u8, 142u8, 159u8, 40u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(SendNotAllowed),
            ::core::stringify!(LengthIsZero),
            ::core::stringify!(DelegateCallNotAllowed),
            ::core::stringify!(OwnableUnauthorizedAccount),
            ::core::stringify!(OwnableInvalidOwner),
            ::core::stringify!(NonExistentKey),
            ::core::stringify!(ParameterNotAllowed),
            ::core::stringify!(FunctionSignatureTooShort),
            ::core::stringify!(PermissionNotConfigured),
            ::core::stringify!(TargetIsNotScoped),
            ::core::stringify!(ERC1967InvalidImplementation),
            ::core::stringify!(DefaultPermissionRejected),
            ::core::stringify!(SafeMultisendSameAddress),
            ::core::stringify!(NodePermissionRejected),
            ::core::stringify!(CalldataOutOfBounds),
            ::core::stringify!(ArraysDifferentLength),
            ::core::stringify!(UnacceptableMultiSendOffset),
            ::core::stringify!(GranularPermissionRejected),
            ::core::stringify!(AddressIsZero),
            ::core::stringify!(AddressEmptyCode),
            ::core::stringify!(FailedToSendEthToNode),
            ::core::stringify!(UUPSUnsupportedProxiableUUID),
            ::core::stringify!(ERC1967NonPayable),
            ::core::stringify!(TooManyCapabilities),
            ::core::stringify!(ArrayTooLong),
            ::core::stringify!(FailedCall),
            ::core::stringify!(NotInitializing),
            ::core::stringify!(PermissionNotFound),
            ::core::stringify!(UUPSUnauthorizedCallContext),
            ::core::stringify!(WithMembership),
            ::core::stringify!(TargetIsScoped),
            ::core::stringify!(TargetAddressNotAllowed),
            ::core::stringify!(InvalidInitialization),
            ::core::stringify!(CannotChangeOwner),
            ::core::stringify!(NoMembership),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <SendNotAllowed as alloy_sol_types::SolError>::SIGNATURE,
            <LengthIsZero as alloy_sol_types::SolError>::SIGNATURE,
            <DelegateCallNotAllowed as alloy_sol_types::SolError>::SIGNATURE,
            <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::SIGNATURE,
            <OwnableInvalidOwner as alloy_sol_types::SolError>::SIGNATURE,
            <NonExistentKey as alloy_sol_types::SolError>::SIGNATURE,
            <ParameterNotAllowed as alloy_sol_types::SolError>::SIGNATURE,
            <FunctionSignatureTooShort as alloy_sol_types::SolError>::SIGNATURE,
            <PermissionNotConfigured as alloy_sol_types::SolError>::SIGNATURE,
            <TargetIsNotScoped as alloy_sol_types::SolError>::SIGNATURE,
            <ERC1967InvalidImplementation as alloy_sol_types::SolError>::SIGNATURE,
            <DefaultPermissionRejected as alloy_sol_types::SolError>::SIGNATURE,
            <SafeMultisendSameAddress as alloy_sol_types::SolError>::SIGNATURE,
            <NodePermissionRejected as alloy_sol_types::SolError>::SIGNATURE,
            <CalldataOutOfBounds as alloy_sol_types::SolError>::SIGNATURE,
            <ArraysDifferentLength as alloy_sol_types::SolError>::SIGNATURE,
            <UnacceptableMultiSendOffset as alloy_sol_types::SolError>::SIGNATURE,
            <GranularPermissionRejected as alloy_sol_types::SolError>::SIGNATURE,
            <AddressIsZero as alloy_sol_types::SolError>::SIGNATURE,
            <AddressEmptyCode as alloy_sol_types::SolError>::SIGNATURE,
            <FailedToSendEthToNode as alloy_sol_types::SolError>::SIGNATURE,
            <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::SIGNATURE,
            <ERC1967NonPayable as alloy_sol_types::SolError>::SIGNATURE,
            <TooManyCapabilities as alloy_sol_types::SolError>::SIGNATURE,
            <ArrayTooLong as alloy_sol_types::SolError>::SIGNATURE,
            <FailedCall as alloy_sol_types::SolError>::SIGNATURE,
            <NotInitializing as alloy_sol_types::SolError>::SIGNATURE,
            <PermissionNotFound as alloy_sol_types::SolError>::SIGNATURE,
            <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::SIGNATURE,
            <WithMembership as alloy_sol_types::SolError>::SIGNATURE,
            <TargetIsScoped as alloy_sol_types::SolError>::SIGNATURE,
            <TargetAddressNotAllowed as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidInitialization as alloy_sol_types::SolError>::SIGNATURE,
            <CannotChangeOwner as alloy_sol_types::SolError>::SIGNATURE,
            <NoMembership as alloy_sol_types::SolError>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for HoprNodeManagementModuleErrors {
        const NAME: &'static str = "HoprNodeManagementModuleErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 35usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AddressEmptyCode(_) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AddressIsZero(_) => {
                    <AddressIsZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ArrayTooLong(_) => {
                    <ArrayTooLong as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ArraysDifferentLength(_) => {
                    <ArraysDifferentLength as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CalldataOutOfBounds(_) => {
                    <CalldataOutOfBounds as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CannotChangeOwner(_) => {
                    <CannotChangeOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::DefaultPermissionRejected(_) => {
                    <DefaultPermissionRejected as alloy_sol_types::SolError>::SELECTOR
                }
                Self::DelegateCallNotAllowed(_) => {
                    <DelegateCallNotAllowed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC1967InvalidImplementation(_) => {
                    <ERC1967InvalidImplementation as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC1967NonPayable(_) => {
                    <ERC1967NonPayable as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FailedCall(_) => {
                    <FailedCall as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FailedToSendEthToNode(_) => {
                    <FailedToSendEthToNode as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FunctionSignatureTooShort(_) => {
                    <FunctionSignatureTooShort as alloy_sol_types::SolError>::SELECTOR
                }
                Self::GranularPermissionRejected(_) => {
                    <GranularPermissionRejected as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidInitialization(_) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LengthIsZero(_) => {
                    <LengthIsZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NoMembership(_) => {
                    <NoMembership as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NodePermissionRejected(_) => {
                    <NodePermissionRejected as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NonExistentKey(_) => {
                    <NonExistentKey as alloy_sol_types::SolError>::SELECTOR
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
                Self::ParameterNotAllowed(_) => {
                    <ParameterNotAllowed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::PermissionNotConfigured(_) => {
                    <PermissionNotConfigured as alloy_sol_types::SolError>::SELECTOR
                }
                Self::PermissionNotFound(_) => {
                    <PermissionNotFound as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SafeMultisendSameAddress(_) => {
                    <SafeMultisendSameAddress as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SendNotAllowed(_) => {
                    <SendNotAllowed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TargetAddressNotAllowed(_) => {
                    <TargetAddressNotAllowed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TargetIsNotScoped(_) => {
                    <TargetIsNotScoped as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TargetIsScoped(_) => {
                    <TargetIsScoped as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TooManyCapabilities(_) => {
                    <TooManyCapabilities as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UUPSUnauthorizedCallContext(_) => {
                    <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UUPSUnsupportedProxiableUUID(_) => {
                    <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UnacceptableMultiSendOffset(_) => {
                    <UnacceptableMultiSendOffset as alloy_sol_types::SolError>::SELECTOR
                }
                Self::WithMembership(_) => {
                    <WithMembership as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors>] = &[
                {
                    fn SendNotAllowed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <SendNotAllowed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::SendNotAllowed)
                    }
                    SendNotAllowed
                },
                {
                    fn LengthIsZero(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <LengthIsZero as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(HoprNodeManagementModuleErrors::LengthIsZero)
                    }
                    LengthIsZero
                },
                {
                    fn DelegateCallNotAllowed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <DelegateCallNotAllowed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::DelegateCallNotAllowed)
                    }
                    DelegateCallNotAllowed
                },
                {
                    fn OwnableUnauthorizedAccount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                HoprNodeManagementModuleErrors::OwnableUnauthorizedAccount,
                            )
                    }
                    OwnableUnauthorizedAccount
                },
                {
                    fn OwnableInvalidOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::OwnableInvalidOwner)
                    }
                    OwnableInvalidOwner
                },
                {
                    fn NonExistentKey(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <NonExistentKey as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::NonExistentKey)
                    }
                    NonExistentKey
                },
                {
                    fn ParameterNotAllowed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <ParameterNotAllowed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::ParameterNotAllowed)
                    }
                    ParameterNotAllowed
                },
                {
                    fn FunctionSignatureTooShort(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <FunctionSignatureTooShort as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                HoprNodeManagementModuleErrors::FunctionSignatureTooShort,
                            )
                    }
                    FunctionSignatureTooShort
                },
                {
                    fn PermissionNotConfigured(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <PermissionNotConfigured as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::PermissionNotConfigured)
                    }
                    PermissionNotConfigured
                },
                {
                    fn TargetIsNotScoped(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <TargetIsNotScoped as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::TargetIsNotScoped)
                    }
                    TargetIsNotScoped
                },
                {
                    fn ERC1967InvalidImplementation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                HoprNodeManagementModuleErrors::ERC1967InvalidImplementation,
                            )
                    }
                    ERC1967InvalidImplementation
                },
                {
                    fn DefaultPermissionRejected(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <DefaultPermissionRejected as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                HoprNodeManagementModuleErrors::DefaultPermissionRejected,
                            )
                    }
                    DefaultPermissionRejected
                },
                {
                    fn SafeMultisendSameAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <SafeMultisendSameAddress as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                HoprNodeManagementModuleErrors::SafeMultisendSameAddress,
                            )
                    }
                    SafeMultisendSameAddress
                },
                {
                    fn NodePermissionRejected(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <NodePermissionRejected as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::NodePermissionRejected)
                    }
                    NodePermissionRejected
                },
                {
                    fn CalldataOutOfBounds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <CalldataOutOfBounds as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::CalldataOutOfBounds)
                    }
                    CalldataOutOfBounds
                },
                {
                    fn ArraysDifferentLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <ArraysDifferentLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::ArraysDifferentLength)
                    }
                    ArraysDifferentLength
                },
                {
                    fn UnacceptableMultiSendOffset(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <UnacceptableMultiSendOffset as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                HoprNodeManagementModuleErrors::UnacceptableMultiSendOffset,
                            )
                    }
                    UnacceptableMultiSendOffset
                },
                {
                    fn GranularPermissionRejected(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <GranularPermissionRejected as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                HoprNodeManagementModuleErrors::GranularPermissionRejected,
                            )
                    }
                    GranularPermissionRejected
                },
                {
                    fn AddressIsZero(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <AddressIsZero as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::AddressIsZero)
                    }
                    AddressIsZero
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn FailedToSendEthToNode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <FailedToSendEthToNode as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::FailedToSendEthToNode)
                    }
                    FailedToSendEthToNode
                },
                {
                    fn UUPSUnsupportedProxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                HoprNodeManagementModuleErrors::UUPSUnsupportedProxiableUUID,
                            )
                    }
                    UUPSUnsupportedProxiableUUID
                },
                {
                    fn ERC1967NonPayable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <ERC1967NonPayable as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::ERC1967NonPayable)
                    }
                    ERC1967NonPayable
                },
                {
                    fn TooManyCapabilities(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <TooManyCapabilities as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::TooManyCapabilities)
                    }
                    TooManyCapabilities
                },
                {
                    fn ArrayTooLong(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <ArrayTooLong as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(HoprNodeManagementModuleErrors::ArrayTooLong)
                    }
                    ArrayTooLong
                },
                {
                    fn FailedCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(HoprNodeManagementModuleErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn NotInitializing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <NotInitializing as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::NotInitializing)
                    }
                    NotInitializing
                },
                {
                    fn PermissionNotFound(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <PermissionNotFound as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::PermissionNotFound)
                    }
                    PermissionNotFound
                },
                {
                    fn UUPSUnauthorizedCallContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                HoprNodeManagementModuleErrors::UUPSUnauthorizedCallContext,
                            )
                    }
                    UUPSUnauthorizedCallContext
                },
                {
                    fn WithMembership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <WithMembership as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::WithMembership)
                    }
                    WithMembership
                },
                {
                    fn TargetIsScoped(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <TargetIsScoped as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::TargetIsScoped)
                    }
                    TargetIsScoped
                },
                {
                    fn TargetAddressNotAllowed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <TargetAddressNotAllowed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::TargetAddressNotAllowed)
                    }
                    TargetAddressNotAllowed
                },
                {
                    fn InvalidInitialization(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <InvalidInitialization as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::InvalidInitialization)
                    }
                    InvalidInitialization
                },
                {
                    fn CannotChangeOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <CannotChangeOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::CannotChangeOwner)
                    }
                    CannotChangeOwner
                },
                {
                    fn NoMembership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <NoMembership as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(HoprNodeManagementModuleErrors::NoMembership)
                    }
                    NoMembership
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
            ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors>] = &[
                {
                    fn SendNotAllowed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <SendNotAllowed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::SendNotAllowed)
                    }
                    SendNotAllowed
                },
                {
                    fn LengthIsZero(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <LengthIsZero as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::LengthIsZero)
                    }
                    LengthIsZero
                },
                {
                    fn DelegateCallNotAllowed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <DelegateCallNotAllowed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::DelegateCallNotAllowed)
                    }
                    DelegateCallNotAllowed
                },
                {
                    fn OwnableUnauthorizedAccount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                HoprNodeManagementModuleErrors::OwnableUnauthorizedAccount,
                            )
                    }
                    OwnableUnauthorizedAccount
                },
                {
                    fn OwnableInvalidOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::OwnableInvalidOwner)
                    }
                    OwnableInvalidOwner
                },
                {
                    fn NonExistentKey(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <NonExistentKey as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::NonExistentKey)
                    }
                    NonExistentKey
                },
                {
                    fn ParameterNotAllowed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <ParameterNotAllowed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::ParameterNotAllowed)
                    }
                    ParameterNotAllowed
                },
                {
                    fn FunctionSignatureTooShort(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <FunctionSignatureTooShort as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                HoprNodeManagementModuleErrors::FunctionSignatureTooShort,
                            )
                    }
                    FunctionSignatureTooShort
                },
                {
                    fn PermissionNotConfigured(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <PermissionNotConfigured as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::PermissionNotConfigured)
                    }
                    PermissionNotConfigured
                },
                {
                    fn TargetIsNotScoped(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <TargetIsNotScoped as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::TargetIsNotScoped)
                    }
                    TargetIsNotScoped
                },
                {
                    fn ERC1967InvalidImplementation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                HoprNodeManagementModuleErrors::ERC1967InvalidImplementation,
                            )
                    }
                    ERC1967InvalidImplementation
                },
                {
                    fn DefaultPermissionRejected(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <DefaultPermissionRejected as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                HoprNodeManagementModuleErrors::DefaultPermissionRejected,
                            )
                    }
                    DefaultPermissionRejected
                },
                {
                    fn SafeMultisendSameAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <SafeMultisendSameAddress as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                HoprNodeManagementModuleErrors::SafeMultisendSameAddress,
                            )
                    }
                    SafeMultisendSameAddress
                },
                {
                    fn NodePermissionRejected(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <NodePermissionRejected as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::NodePermissionRejected)
                    }
                    NodePermissionRejected
                },
                {
                    fn CalldataOutOfBounds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <CalldataOutOfBounds as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::CalldataOutOfBounds)
                    }
                    CalldataOutOfBounds
                },
                {
                    fn ArraysDifferentLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <ArraysDifferentLength as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::ArraysDifferentLength)
                    }
                    ArraysDifferentLength
                },
                {
                    fn UnacceptableMultiSendOffset(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <UnacceptableMultiSendOffset as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                HoprNodeManagementModuleErrors::UnacceptableMultiSendOffset,
                            )
                    }
                    UnacceptableMultiSendOffset
                },
                {
                    fn GranularPermissionRejected(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <GranularPermissionRejected as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                HoprNodeManagementModuleErrors::GranularPermissionRejected,
                            )
                    }
                    GranularPermissionRejected
                },
                {
                    fn AddressIsZero(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <AddressIsZero as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::AddressIsZero)
                    }
                    AddressIsZero
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn FailedToSendEthToNode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <FailedToSendEthToNode as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::FailedToSendEthToNode)
                    }
                    FailedToSendEthToNode
                },
                {
                    fn UUPSUnsupportedProxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                HoprNodeManagementModuleErrors::UUPSUnsupportedProxiableUUID,
                            )
                    }
                    UUPSUnsupportedProxiableUUID
                },
                {
                    fn ERC1967NonPayable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <ERC1967NonPayable as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::ERC1967NonPayable)
                    }
                    ERC1967NonPayable
                },
                {
                    fn TooManyCapabilities(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <TooManyCapabilities as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::TooManyCapabilities)
                    }
                    TooManyCapabilities
                },
                {
                    fn ArrayTooLong(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <ArrayTooLong as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::ArrayTooLong)
                    }
                    ArrayTooLong
                },
                {
                    fn FailedCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn NotInitializing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <NotInitializing as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::NotInitializing)
                    }
                    NotInitializing
                },
                {
                    fn PermissionNotFound(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <PermissionNotFound as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::PermissionNotFound)
                    }
                    PermissionNotFound
                },
                {
                    fn UUPSUnauthorizedCallContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                HoprNodeManagementModuleErrors::UUPSUnauthorizedCallContext,
                            )
                    }
                    UUPSUnauthorizedCallContext
                },
                {
                    fn WithMembership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <WithMembership as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::WithMembership)
                    }
                    WithMembership
                },
                {
                    fn TargetIsScoped(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <TargetIsScoped as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::TargetIsScoped)
                    }
                    TargetIsScoped
                },
                {
                    fn TargetAddressNotAllowed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <TargetAddressNotAllowed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::TargetAddressNotAllowed)
                    }
                    TargetAddressNotAllowed
                },
                {
                    fn InvalidInitialization(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <InvalidInitialization as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::InvalidInitialization)
                    }
                    InvalidInitialization
                },
                {
                    fn CannotChangeOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <CannotChangeOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::CannotChangeOwner)
                    }
                    CannotChangeOwner
                },
                {
                    fn NoMembership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprNodeManagementModuleErrors> {
                        <NoMembership as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprNodeManagementModuleErrors::NoMembership)
                    }
                    NoMembership
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
                Self::AddressIsZero(inner) => {
                    <AddressIsZero as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ArrayTooLong(inner) => {
                    <ArrayTooLong as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ArraysDifferentLength(inner) => {
                    <ArraysDifferentLength as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CalldataOutOfBounds(inner) => {
                    <CalldataOutOfBounds as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CannotChangeOwner(inner) => {
                    <CannotChangeOwner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DefaultPermissionRejected(inner) => {
                    <DefaultPermissionRejected as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DelegateCallNotAllowed(inner) => {
                    <DelegateCallNotAllowed as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::FailedCall(inner) => {
                    <FailedCall as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::FailedToSendEthToNode(inner) => {
                    <FailedToSendEthToNode as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FunctionSignatureTooShort(inner) => {
                    <FunctionSignatureTooShort as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::GranularPermissionRejected(inner) => {
                    <GranularPermissionRejected as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidInitialization(inner) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LengthIsZero(inner) => {
                    <LengthIsZero as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::NoMembership(inner) => {
                    <NoMembership as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::NodePermissionRejected(inner) => {
                    <NodePermissionRejected as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NonExistentKey(inner) => {
                    <NonExistentKey as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::ParameterNotAllowed(inner) => {
                    <ParameterNotAllowed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::PermissionNotConfigured(inner) => {
                    <PermissionNotConfigured as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::PermissionNotFound(inner) => {
                    <PermissionNotFound as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SafeMultisendSameAddress(inner) => {
                    <SafeMultisendSameAddress as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SendNotAllowed(inner) => {
                    <SendNotAllowed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TargetAddressNotAllowed(inner) => {
                    <TargetAddressNotAllowed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TargetIsNotScoped(inner) => {
                    <TargetIsNotScoped as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TargetIsScoped(inner) => {
                    <TargetIsScoped as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TooManyCapabilities(inner) => {
                    <TooManyCapabilities as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::UnacceptableMultiSendOffset(inner) => {
                    <UnacceptableMultiSendOffset as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::WithMembership(inner) => {
                    <WithMembership as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
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
                Self::AddressIsZero(inner) => {
                    <AddressIsZero as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ArrayTooLong(inner) => {
                    <ArrayTooLong as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ArraysDifferentLength(inner) => {
                    <ArraysDifferentLength as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CalldataOutOfBounds(inner) => {
                    <CalldataOutOfBounds as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CannotChangeOwner(inner) => {
                    <CannotChangeOwner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DefaultPermissionRejected(inner) => {
                    <DefaultPermissionRejected as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DelegateCallNotAllowed(inner) => {
                    <DelegateCallNotAllowed as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::FailedCall(inner) => {
                    <FailedCall as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::FailedToSendEthToNode(inner) => {
                    <FailedToSendEthToNode as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FunctionSignatureTooShort(inner) => {
                    <FunctionSignatureTooShort as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::GranularPermissionRejected(inner) => {
                    <GranularPermissionRejected as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidInitialization(inner) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LengthIsZero(inner) => {
                    <LengthIsZero as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NoMembership(inner) => {
                    <NoMembership as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NodePermissionRejected(inner) => {
                    <NodePermissionRejected as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NonExistentKey(inner) => {
                    <NonExistentKey as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::ParameterNotAllowed(inner) => {
                    <ParameterNotAllowed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::PermissionNotConfigured(inner) => {
                    <PermissionNotConfigured as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::PermissionNotFound(inner) => {
                    <PermissionNotFound as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SafeMultisendSameAddress(inner) => {
                    <SafeMultisendSameAddress as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SendNotAllowed(inner) => {
                    <SendNotAllowed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TargetAddressNotAllowed(inner) => {
                    <TargetAddressNotAllowed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TargetIsNotScoped(inner) => {
                    <TargetIsNotScoped as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TargetIsScoped(inner) => {
                    <TargetIsScoped as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TooManyCapabilities(inner) => {
                    <TooManyCapabilities as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::UnacceptableMultiSendOffset(inner) => {
                    <UnacceptableMultiSendOffset as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::WithMembership(inner) => {
                    <WithMembership as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`HoprNodeManagementModule`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum HoprNodeManagementModuleEvents {
        #[allow(missing_docs)]
        ExecutionFailure(ExecutionFailure),
        #[allow(missing_docs)]
        ExecutionSuccess(ExecutionSuccess),
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        NodeAdded(NodeAdded),
        #[allow(missing_docs)]
        NodeRemoved(NodeRemoved),
        #[allow(missing_docs)]
        OwnershipTransferred(OwnershipTransferred),
        #[allow(missing_docs)]
        RevokedTarget(RevokedTarget),
        #[allow(missing_docs)]
        ScopedGranularChannelCapability(ScopedGranularChannelCapability),
        #[allow(missing_docs)]
        ScopedGranularSendCapability(ScopedGranularSendCapability),
        #[allow(missing_docs)]
        ScopedGranularTokenCapability(ScopedGranularTokenCapability),
        #[allow(missing_docs)]
        ScopedTargetChannels(ScopedTargetChannels),
        #[allow(missing_docs)]
        ScopedTargetSend(ScopedTargetSend),
        #[allow(missing_docs)]
        ScopedTargetToken(ScopedTargetToken),
        #[allow(missing_docs)]
        SetMultisendAddress(SetMultisendAddress),
        #[allow(missing_docs)]
        Upgraded(Upgraded),
    }
    impl HoprNodeManagementModuleEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                13u8, 252u8, 225u8, 234u8, 75u8, 161u8, 238u8, 186u8, 137u8, 31u8, 251u8,
                42u8, 6u8, 103u8, 144u8, 251u8, 194u8, 147u8, 169u8, 229u8, 23u8, 254u8,
                97u8, 212u8, 157u8, 21u8, 106u8, 48u8, 22u8, 95u8, 147u8, 243u8,
            ],
            [
                30u8, 226u8, 121u8, 31u8, 44u8, 175u8, 14u8, 146u8, 169u8, 220u8, 50u8,
                163u8, 122u8, 158u8, 165u8, 58u8, 182u8, 172u8, 122u8, 111u8, 184u8,
                242u8, 208u8, 144u8, 229u8, 58u8, 6u8, 125u8, 58u8, 67u8, 246u8, 172u8,
            ],
            [
                78u8, 46u8, 134u8, 210u8, 19u8, 117u8, 235u8, 203u8, 246u8, 233u8, 61u8,
                245u8, 235u8, 221u8, 90u8, 145u8, 91u8, 248u8, 48u8, 36u8, 89u8, 4u8,
                195u8, 181u8, 79u8, 72u8, 173u8, 240u8, 23u8, 10u8, 174u8, 75u8,
            ],
            [
                95u8, 230u8, 170u8, 191u8, 78u8, 121u8, 8u8, 67u8, 223u8, 67u8, 174u8,
                14u8, 34u8, 181u8, 134u8, 32u8, 6u8, 111u8, 179u8, 137u8, 41u8, 91u8,
                237u8, 192u8, 106u8, 146u8, 223u8, 108u8, 59u8, 40u8, 119u8, 125u8,
            ],
            [
                95u8, 251u8, 6u8, 176u8, 176u8, 232u8, 173u8, 106u8, 143u8, 60u8, 88u8,
                49u8, 212u8, 153u8, 223u8, 166u8, 18u8, 217u8, 201u8, 212u8, 220u8, 16u8,
                123u8, 189u8, 102u8, 241u8, 143u8, 97u8, 166u8, 73u8, 46u8, 113u8,
            ],
            [
                116u8, 135u8, 83u8, 13u8, 223u8, 241u8, 32u8, 121u8, 149u8, 5u8, 229u8,
                43u8, 27u8, 25u8, 182u8, 147u8, 63u8, 133u8, 169u8, 238u8, 174u8, 146u8,
                32u8, 200u8, 10u8, 122u8, 215u8, 196u8, 41u8, 182u8, 18u8, 174u8,
            ],
            [
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8,
                31u8, 208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8,
                218u8, 175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ],
            [
                163u8, 223u8, 113u8, 4u8, 32u8, 176u8, 28u8, 195u8, 15u8, 243u8, 0u8,
                48u8, 154u8, 187u8, 199u8, 250u8, 221u8, 70u8, 48u8, 212u8, 171u8, 56u8,
                91u8, 15u8, 90u8, 18u8, 111u8, 180u8, 186u8, 190u8, 118u8, 43u8,
            ],
            [
                170u8, 242u8, 107u8, 177u8, 42u8, 168u8, 158u8, 233u8, 107u8, 190u8,
                25u8, 102u8, 122u8, 106u8, 5u8, 87u8, 39u8, 183u8, 93u8, 63u8, 110u8,
                215u8, 184u8, 182u8, 17u8, 239u8, 101u8, 25u8, 24u8, 2u8, 9u8, 214u8,
            ],
            [
                178u8, 93u8, 3u8, 170u8, 243u8, 8u8, 215u8, 41u8, 23u8, 9u8, 190u8, 30u8,
                162u8, 139u8, 128u8, 4u8, 99u8, 207u8, 58u8, 154u8, 76u8, 74u8, 85u8,
                85u8, 215u8, 51u8, 58u8, 150u8, 76u8, 29u8, 254u8, 189u8,
            ],
            [
                188u8, 124u8, 215u8, 90u8, 32u8, 238u8, 39u8, 253u8, 154u8, 222u8, 186u8,
                179u8, 32u8, 65u8, 247u8, 85u8, 33u8, 77u8, 188u8, 107u8, 255u8, 169u8,
                12u8, 192u8, 34u8, 91u8, 57u8, 218u8, 46u8, 92u8, 45u8, 59u8,
            ],
            [
                194u8, 77u8, 147u8, 96u8, 138u8, 3u8, 210u8, 99u8, 255u8, 25u8, 29u8,
                118u8, 119u8, 20u8, 31u8, 94u8, 148u8, 196u8, 150u8, 229u8, 147u8, 16u8,
                143u8, 58u8, 174u8, 12u8, 181u8, 183u8, 4u8, 148u8, 196u8, 211u8,
            ],
            [
                199u8, 245u8, 5u8, 178u8, 243u8, 113u8, 174u8, 33u8, 117u8, 238u8, 73u8,
                19u8, 244u8, 73u8, 158u8, 31u8, 38u8, 51u8, 167u8, 181u8, 147u8, 99u8,
                33u8, 238u8, 209u8, 205u8, 174u8, 182u8, 17u8, 81u8, 129u8, 210u8,
            ],
            [
                207u8, 194u8, 65u8, 102u8, 219u8, 75u8, 182u8, 119u8, 232u8, 87u8, 202u8,
                202u8, 189u8, 21u8, 65u8, 251u8, 43u8, 48u8, 100u8, 80u8, 33u8, 178u8,
                124u8, 81u8, 48u8, 65u8, 149u8, 137u8, 184u8, 77u8, 181u8, 43u8,
            ],
            [
                242u8, 255u8, 212u8, 240u8, 157u8, 88u8, 208u8, 104u8, 36u8, 24u8, 128u8,
                51u8, 211u8, 49u8, 141u8, 6u8, 235u8, 149u8, 123u8, 251u8, 26u8, 143u8,
                254u8, 217u8, 175u8, 120u8, 225u8, 241u8, 145u8, 104u8, 185u8, 4u8,
            ],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(RevokedTarget),
            ::core::stringify!(ScopedTargetSend),
            ::core::stringify!(ExecutionSuccess),
            ::core::stringify!(SetMultisendAddress),
            ::core::stringify!(ScopedTargetChannels),
            ::core::stringify!(ScopedGranularSendCapability),
            ::core::stringify!(OwnershipTransferred),
            ::core::stringify!(ScopedGranularTokenCapability),
            ::core::stringify!(ScopedTargetToken),
            ::core::stringify!(NodeAdded),
            ::core::stringify!(Upgraded),
            ::core::stringify!(ExecutionFailure),
            ::core::stringify!(Initialized),
            ::core::stringify!(NodeRemoved),
            ::core::stringify!(ScopedGranularChannelCapability),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <RevokedTarget as alloy_sol_types::SolEvent>::SIGNATURE,
            <ScopedTargetSend as alloy_sol_types::SolEvent>::SIGNATURE,
            <ExecutionSuccess as alloy_sol_types::SolEvent>::SIGNATURE,
            <SetMultisendAddress as alloy_sol_types::SolEvent>::SIGNATURE,
            <ScopedTargetChannels as alloy_sol_types::SolEvent>::SIGNATURE,
            <ScopedGranularSendCapability as alloy_sol_types::SolEvent>::SIGNATURE,
            <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE,
            <ScopedGranularTokenCapability as alloy_sol_types::SolEvent>::SIGNATURE,
            <ScopedTargetToken as alloy_sol_types::SolEvent>::SIGNATURE,
            <NodeAdded as alloy_sol_types::SolEvent>::SIGNATURE,
            <Upgraded as alloy_sol_types::SolEvent>::SIGNATURE,
            <ExecutionFailure as alloy_sol_types::SolEvent>::SIGNATURE,
            <Initialized as alloy_sol_types::SolEvent>::SIGNATURE,
            <NodeRemoved as alloy_sol_types::SolEvent>::SIGNATURE,
            <ScopedGranularChannelCapability as alloy_sol_types::SolEvent>::SIGNATURE,
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
    impl alloy_sol_types::SolEventInterface for HoprNodeManagementModuleEvents {
        const NAME: &'static str = "HoprNodeManagementModuleEvents";
        const COUNT: usize = 15usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<ExecutionFailure as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ExecutionFailure as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ExecutionFailure)
                }
                Some(<ExecutionSuccess as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ExecutionSuccess as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ExecutionSuccess)
                }
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::Initialized)
                }
                Some(<NodeAdded as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <NodeAdded as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::NodeAdded)
                }
                Some(<NodeRemoved as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <NodeRemoved as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::NodeRemoved)
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
                Some(<RevokedTarget as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RevokedTarget as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::RevokedTarget)
                }
                Some(
                    <ScopedGranularChannelCapability as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ScopedGranularChannelCapability as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ScopedGranularChannelCapability)
                }
                Some(
                    <ScopedGranularSendCapability as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ScopedGranularSendCapability as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ScopedGranularSendCapability)
                }
                Some(
                    <ScopedGranularTokenCapability as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ScopedGranularTokenCapability as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ScopedGranularTokenCapability)
                }
                Some(
                    <ScopedTargetChannels as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ScopedTargetChannels as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ScopedTargetChannels)
                }
                Some(<ScopedTargetSend as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ScopedTargetSend as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ScopedTargetSend)
                }
                Some(
                    <ScopedTargetToken as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ScopedTargetToken as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ScopedTargetToken)
                }
                Some(
                    <SetMultisendAddress as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <SetMultisendAddress as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::SetMultisendAddress)
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
    impl alloy_sol_types::private::IntoLogData for HoprNodeManagementModuleEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ExecutionFailure(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ExecutionSuccess(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::NodeAdded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::NodeRemoved(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RevokedTarget(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ScopedGranularChannelCapability(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ScopedGranularSendCapability(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ScopedGranularTokenCapability(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ScopedTargetChannels(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ScopedTargetSend(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ScopedTargetToken(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SetMultisendAddress(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ExecutionFailure(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ExecutionSuccess(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::NodeAdded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::NodeRemoved(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RevokedTarget(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ScopedGranularChannelCapability(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ScopedGranularSendCapability(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ScopedGranularTokenCapability(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ScopedTargetChannels(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ScopedTargetSend(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ScopedTargetToken(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SetMultisendAddress(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`HoprNodeManagementModule`](self) contract instance.

See the [wrapper's documentation](`HoprNodeManagementModuleInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> HoprNodeManagementModuleInstance<P, N> {
        HoprNodeManagementModuleInstance::<P, N>::new(address, __provider)
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
        Output = alloy_contract::Result<HoprNodeManagementModuleInstance<P, N>>,
    > {
        HoprNodeManagementModuleInstance::<P, N>::deploy(__provider)
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
        HoprNodeManagementModuleInstance::<P, N>::deploy_builder(__provider)
    }
    /**A [`HoprNodeManagementModule`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`HoprNodeManagementModule`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct HoprNodeManagementModuleInstance<
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for HoprNodeManagementModuleInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("HoprNodeManagementModuleInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > HoprNodeManagementModuleInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`HoprNodeManagementModule`](self) contract instance.

See the [wrapper's documentation](`HoprNodeManagementModuleInstance`) for more details.*/
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
        ) -> alloy_contract::Result<HoprNodeManagementModuleInstance<P, N>> {
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
    impl<P: ::core::clone::Clone, N> HoprNodeManagementModuleInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> HoprNodeManagementModuleInstance<P, N> {
            HoprNodeManagementModuleInstance {
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
    > HoprNodeManagementModuleInstance<P, N> {
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
        ///Creates a new call builder for the [`UPGRADE_INTERFACE_VERSION`] function.
        pub fn UPGRADE_INTERFACE_VERSION(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, UPGRADE_INTERFACE_VERSIONCall, N> {
            self.call_builder(&UPGRADE_INTERFACE_VERSIONCall)
        }
        ///Creates a new call builder for the [`VERSION`] function.
        pub fn VERSION(&self) -> alloy_contract::SolCallBuilder<&P, VERSIONCall, N> {
            self.call_builder(&VERSIONCall)
        }
        ///Creates a new call builder for the [`addChannelsAndTokenTarget`] function.
        pub fn addChannelsAndTokenTarget(
            &self,
            defaultTarget: <Target as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, addChannelsAndTokenTargetCall, N> {
            self.call_builder(
                &addChannelsAndTokenTargetCall {
                    defaultTarget,
                },
            )
        }
        ///Creates a new call builder for the [`addNode`] function.
        pub fn addNode(
            &self,
            nodeAddress: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, addNodeCall, N> {
            self.call_builder(&addNodeCall { nodeAddress })
        }
        ///Creates a new call builder for the [`addNodes`] function.
        pub fn addNodes(
            &self,
            nodeAddresses: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, addNodesCall, N> {
            self.call_builder(&addNodesCall { nodeAddresses })
        }
        ///Creates a new call builder for the [`decodeFunctionSigsAndPermissions`] function.
        pub fn decodeFunctionSigsAndPermissions(
            &self,
            encoded: alloy::sol_types::private::FixedBytes<32>,
            length: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            decodeFunctionSigsAndPermissionsCall,
            N,
        > {
            self.call_builder(
                &decodeFunctionSigsAndPermissionsCall {
                    encoded,
                    length,
                },
            )
        }
        ///Creates a new call builder for the [`encodeFunctionSigsAndPermissions`] function.
        pub fn encodeFunctionSigsAndPermissions(
            &self,
            functionSigs: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<4>,
            >,
            permissions: alloy::sol_types::private::Vec<
                <GranularPermission as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            encodeFunctionSigsAndPermissionsCall,
            N,
        > {
            self.call_builder(
                &encodeFunctionSigsAndPermissionsCall {
                    functionSigs,
                    permissions,
                },
            )
        }
        ///Creates a new call builder for the [`execTransactionFromModule`] function.
        pub fn execTransactionFromModule(
            &self,
            to: alloy::sol_types::private::Address,
            value: alloy::sol_types::private::primitives::aliases::U256,
            data: alloy::sol_types::private::Bytes,
            operation: <Enum::Operation as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, execTransactionFromModuleCall, N> {
            self.call_builder(
                &execTransactionFromModuleCall {
                    to,
                    value,
                    data,
                    operation,
                },
            )
        }
        ///Creates a new call builder for the [`execTransactionFromModuleReturnData`] function.
        pub fn execTransactionFromModuleReturnData(
            &self,
            to: alloy::sol_types::private::Address,
            value: alloy::sol_types::private::primitives::aliases::U256,
            data: alloy::sol_types::private::Bytes,
            operation: <Enum::Operation as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            execTransactionFromModuleReturnDataCall,
            N,
        > {
            self.call_builder(
                &execTransactionFromModuleReturnDataCall {
                    to,
                    value,
                    data,
                    operation,
                },
            )
        }
        ///Creates a new call builder for the [`getGranularPermissions`] function.
        pub fn getGranularPermissions(
            &self,
            capabilityKey: alloy::sol_types::private::FixedBytes<32>,
            pairId: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, getGranularPermissionsCall, N> {
            self.call_builder(
                &getGranularPermissionsCall {
                    capabilityKey,
                    pairId,
                },
            )
        }
        ///Creates a new call builder for the [`getTargets`] function.
        pub fn getTargets(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getTargetsCall, N> {
            self.call_builder(&getTargetsCall)
        }
        ///Creates a new call builder for the [`includeNode`] function.
        pub fn includeNode(
            &self,
            nodeDefaultTarget: <Target as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, includeNodeCall, N> {
            self.call_builder(
                &includeNodeCall {
                    nodeDefaultTarget,
                },
            )
        }
        ///Creates a new call builder for the [`includeNodes`] function.
        pub fn includeNodes(
            &self,
            nodeAddresses: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, includeNodesCall, N> {
            self.call_builder(&includeNodesCall { nodeAddresses })
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            initParams: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, initializeCall, N> {
            self.call_builder(&initializeCall { initParams })
        }
        ///Creates a new call builder for the [`isHoprNodeManagementModule`] function.
        pub fn isHoprNodeManagementModule(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, isHoprNodeManagementModuleCall, N> {
            self.call_builder(&isHoprNodeManagementModuleCall)
        }
        ///Creates a new call builder for the [`isNode`] function.
        pub fn isNode(
            &self,
            nodeAddress: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, isNodeCall, N> {
            self.call_builder(&isNodeCall { nodeAddress })
        }
        ///Creates a new call builder for the [`multisend`] function.
        pub fn multisend(&self) -> alloy_contract::SolCallBuilder<&P, multisendCall, N> {
            self.call_builder(&multisendCall)
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
        ///Creates a new call builder for the [`removeNode`] function.
        pub fn removeNode(
            &self,
            nodeAddress: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, removeNodeCall, N> {
            self.call_builder(&removeNodeCall { nodeAddress })
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall)
        }
        ///Creates a new call builder for the [`revokeTarget`] function.
        pub fn revokeTarget(
            &self,
            targetAddress: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, revokeTargetCall, N> {
            self.call_builder(&revokeTargetCall { targetAddress })
        }
        ///Creates a new call builder for the [`scopeChannelsCapabilities`] function.
        pub fn scopeChannelsCapabilities(
            &self,
            targetAddress: alloy::sol_types::private::Address,
            channelId: alloy::sol_types::private::FixedBytes<32>,
            encodedSigsPermissions: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, scopeChannelsCapabilitiesCall, N> {
            self.call_builder(
                &scopeChannelsCapabilitiesCall {
                    targetAddress,
                    channelId,
                    encodedSigsPermissions,
                },
            )
        }
        ///Creates a new call builder for the [`scopeSendCapability`] function.
        pub fn scopeSendCapability(
            &self,
            nodeAddress: alloy::sol_types::private::Address,
            beneficiary: alloy::sol_types::private::Address,
            permission: <GranularPermission as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, scopeSendCapabilityCall, N> {
            self.call_builder(
                &scopeSendCapabilityCall {
                    nodeAddress,
                    beneficiary,
                    permission,
                },
            )
        }
        ///Creates a new call builder for the [`scopeTargetChannels`] function.
        pub fn scopeTargetChannels(
            &self,
            defaultTarget: <Target as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, scopeTargetChannelsCall, N> {
            self.call_builder(
                &scopeTargetChannelsCall {
                    defaultTarget,
                },
            )
        }
        ///Creates a new call builder for the [`scopeTargetSend`] function.
        pub fn scopeTargetSend(
            &self,
            defaultTarget: <Target as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, scopeTargetSendCall, N> {
            self.call_builder(
                &scopeTargetSendCall {
                    defaultTarget,
                },
            )
        }
        ///Creates a new call builder for the [`scopeTargetToken`] function.
        pub fn scopeTargetToken(
            &self,
            defaultTarget: <Target as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, scopeTargetTokenCall, N> {
            self.call_builder(
                &scopeTargetTokenCall {
                    defaultTarget,
                },
            )
        }
        ///Creates a new call builder for the [`scopeTokenCapabilities`] function.
        pub fn scopeTokenCapabilities(
            &self,
            nodeAddress: alloy::sol_types::private::Address,
            targetAddress: alloy::sol_types::private::Address,
            beneficiary: alloy::sol_types::private::Address,
            encodedSigsPermissions: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, scopeTokenCapabilitiesCall, N> {
            self.call_builder(
                &scopeTokenCapabilitiesCall {
                    nodeAddress,
                    targetAddress,
                    beneficiary,
                    encodedSigsPermissions,
                },
            )
        }
        ///Creates a new call builder for the [`setMultisend`] function.
        pub fn setMultisend(
            &self,
            _multisend: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, setMultisendCall, N> {
            self.call_builder(&setMultisendCall { _multisend })
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`tryGetTarget`] function.
        pub fn tryGetTarget(
            &self,
            targetAddress: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, tryGetTargetCall, N> {
            self.call_builder(&tryGetTargetCall { targetAddress })
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
    > HoprNodeManagementModuleInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`ExecutionFailure`] event.
        pub fn ExecutionFailure_filter(
            &self,
        ) -> alloy_contract::Event<&P, ExecutionFailure, N> {
            self.event_filter::<ExecutionFailure>()
        }
        ///Creates a new event filter for the [`ExecutionSuccess`] event.
        pub fn ExecutionSuccess_filter(
            &self,
        ) -> alloy_contract::Event<&P, ExecutionSuccess, N> {
            self.event_filter::<ExecutionSuccess>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<&P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`NodeAdded`] event.
        pub fn NodeAdded_filter(&self) -> alloy_contract::Event<&P, NodeAdded, N> {
            self.event_filter::<NodeAdded>()
        }
        ///Creates a new event filter for the [`NodeRemoved`] event.
        pub fn NodeRemoved_filter(&self) -> alloy_contract::Event<&P, NodeRemoved, N> {
            self.event_filter::<NodeRemoved>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<&P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`RevokedTarget`] event.
        pub fn RevokedTarget_filter(
            &self,
        ) -> alloy_contract::Event<&P, RevokedTarget, N> {
            self.event_filter::<RevokedTarget>()
        }
        ///Creates a new event filter for the [`ScopedGranularChannelCapability`] event.
        pub fn ScopedGranularChannelCapability_filter(
            &self,
        ) -> alloy_contract::Event<&P, ScopedGranularChannelCapability, N> {
            self.event_filter::<ScopedGranularChannelCapability>()
        }
        ///Creates a new event filter for the [`ScopedGranularSendCapability`] event.
        pub fn ScopedGranularSendCapability_filter(
            &self,
        ) -> alloy_contract::Event<&P, ScopedGranularSendCapability, N> {
            self.event_filter::<ScopedGranularSendCapability>()
        }
        ///Creates a new event filter for the [`ScopedGranularTokenCapability`] event.
        pub fn ScopedGranularTokenCapability_filter(
            &self,
        ) -> alloy_contract::Event<&P, ScopedGranularTokenCapability, N> {
            self.event_filter::<ScopedGranularTokenCapability>()
        }
        ///Creates a new event filter for the [`ScopedTargetChannels`] event.
        pub fn ScopedTargetChannels_filter(
            &self,
        ) -> alloy_contract::Event<&P, ScopedTargetChannels, N> {
            self.event_filter::<ScopedTargetChannels>()
        }
        ///Creates a new event filter for the [`ScopedTargetSend`] event.
        pub fn ScopedTargetSend_filter(
            &self,
        ) -> alloy_contract::Event<&P, ScopedTargetSend, N> {
            self.event_filter::<ScopedTargetSend>()
        }
        ///Creates a new event filter for the [`ScopedTargetToken`] event.
        pub fn ScopedTargetToken_filter(
            &self,
        ) -> alloy_contract::Event<&P, ScopedTargetToken, N> {
            self.event_filter::<ScopedTargetToken>()
        }
        ///Creates a new event filter for the [`SetMultisendAddress`] event.
        pub fn SetMultisendAddress_filter(
            &self,
        ) -> alloy_contract::Event<&P, SetMultisendAddress, N> {
            self.event_filter::<SetMultisendAddress>()
        }
        ///Creates a new event filter for the [`Upgraded`] event.
        pub fn Upgraded_filter(&self) -> alloy_contract::Event<&P, Upgraded, N> {
            self.event_filter::<Upgraded>()
        }
    }
}
