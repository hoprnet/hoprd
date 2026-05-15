///Module containing a contract's types and functions.
/**

```solidity
library HoprChannelsType {
    type ChannelStatus is uint8;
    type Balance is uint96;
    type ChannelEpoch is uint24;
    type TicketIndex is uint48;
    type Timestamp is uint32;
    type WinProb is uint56;
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod HoprChannelsType {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ChannelStatus(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<ChannelStatus> for u8 {
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
        impl ChannelStatus {
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
        impl From<u8> for ChannelStatus {
            fn from(value: u8) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<ChannelStatus> for u8 {
            fn from(value: ChannelStatus) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for ChannelStatus {
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
        impl alloy_sol_types::EventTopic for ChannelStatus {
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
    pub struct Balance(alloy::sol_types::private::primitives::aliases::U96);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Balance>
        for alloy::sol_types::private::primitives::aliases::U96 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                96,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<96>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    96,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    96,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    96,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        impl Balance {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(
                value: alloy::sol_types::private::primitives::aliases::U96,
            ) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(
                self,
            ) -> alloy::sol_types::private::primitives::aliases::U96 {
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
        impl From<alloy::sol_types::private::primitives::aliases::U96> for Balance {
            fn from(value: alloy::sol_types::private::primitives::aliases::U96) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<Balance> for alloy::sol_types::private::primitives::aliases::U96 {
            fn from(value: Balance) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Balance {
            type RustType = alloy::sol_types::private::primitives::aliases::U96;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                96,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                96,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                96,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    96,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    96,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Balance {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    96,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    96,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    96,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ChannelEpoch(alloy::sol_types::private::primitives::aliases::U24);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<ChannelEpoch>
        for alloy::sol_types::private::primitives::aliases::U24 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                24,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<24>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    24,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    24,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    24,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        impl ChannelEpoch {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(
                value: alloy::sol_types::private::primitives::aliases::U24,
            ) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(
                self,
            ) -> alloy::sol_types::private::primitives::aliases::U24 {
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
        impl From<alloy::sol_types::private::primitives::aliases::U24> for ChannelEpoch {
            fn from(value: alloy::sol_types::private::primitives::aliases::U24) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<ChannelEpoch> for alloy::sol_types::private::primitives::aliases::U24 {
            fn from(value: ChannelEpoch) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for ChannelEpoch {
            type RustType = alloy::sol_types::private::primitives::aliases::U24;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                24,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                24,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                24,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    24,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    24,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ChannelEpoch {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    24,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    24,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    24,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TicketIndex(alloy::sol_types::private::primitives::aliases::U48);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<TicketIndex>
        for alloy::sol_types::private::primitives::aliases::U48 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                48,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<48>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    48,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    48,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    48,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        impl TicketIndex {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(
                value: alloy::sol_types::private::primitives::aliases::U48,
            ) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(
                self,
            ) -> alloy::sol_types::private::primitives::aliases::U48 {
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
        impl From<alloy::sol_types::private::primitives::aliases::U48> for TicketIndex {
            fn from(value: alloy::sol_types::private::primitives::aliases::U48) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<TicketIndex> for alloy::sol_types::private::primitives::aliases::U48 {
            fn from(value: TicketIndex) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for TicketIndex {
            type RustType = alloy::sol_types::private::primitives::aliases::U48;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                48,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                48,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                48,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    48,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    48,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for TicketIndex {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    48,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    48,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    48,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Timestamp(u32);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Timestamp> for u32 {
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
        impl Timestamp {
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
        impl From<u32> for Timestamp {
            fn from(value: u32) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<Timestamp> for u32 {
            fn from(value: Timestamp) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Timestamp {
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
        impl alloy_sol_types::EventTopic for Timestamp {
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
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WinProb(alloy::sol_types::private::primitives::aliases::U56);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<WinProb>
        for alloy::sol_types::private::primitives::aliases::U56 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                56,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<56>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    56,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    56,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    56,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        impl WinProb {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(
                value: alloy::sol_types::private::primitives::aliases::U56,
            ) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(
                self,
            ) -> alloy::sol_types::private::primitives::aliases::U56 {
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
        impl From<alloy::sol_types::private::primitives::aliases::U56> for WinProb {
            fn from(value: alloy::sol_types::private::primitives::aliases::U56) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<WinProb> for alloy::sol_types::private::primitives::aliases::U56 {
            fn from(value: WinProb) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for WinProb {
            type RustType = alloy::sol_types::private::primitives::aliases::U56;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                56,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                56,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                56,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    56,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    56,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for WinProb {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    56,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    56,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    56,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`HoprChannelsType`](self) contract instance.

See the [wrapper's documentation](`HoprChannelsTypeInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> HoprChannelsTypeInstance<P, N> {
        HoprChannelsTypeInstance::<P, N>::new(address, __provider)
    }
    /**A [`HoprChannelsType`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`HoprChannelsType`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct HoprChannelsTypeInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for HoprChannelsTypeInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("HoprChannelsTypeInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > HoprChannelsTypeInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`HoprChannelsType`](self) contract instance.

See the [wrapper's documentation](`HoprChannelsTypeInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> HoprChannelsTypeInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> HoprChannelsTypeInstance<P, N> {
            HoprChannelsTypeInstance {
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
    > HoprChannelsTypeInstance<P, N> {
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
    > HoprChannelsTypeInstance<P, N> {
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
///Module containing a contract's types and functions.
/**

```solidity
library HoprCrypto {
    struct CompactSignature { bytes32 r; bytes32 vs; }
    struct VRFParameters { uint256 vx; uint256 vy; uint256 s; uint256 h; uint256 sBx; uint256 sBy; uint256 hVx; uint256 hVy; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod HoprCrypto {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct CompactSignature { bytes32 r; bytes32 vs; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CompactSignature {
        #[allow(missing_docs)]
        pub r: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub vs: alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<CompactSignature> for UnderlyingRustTuple<'_> {
            fn from(value: CompactSignature) -> Self {
                (value.r, value.vs)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CompactSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { r: tuple.0, vs: tuple.1 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for CompactSignature {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for CompactSignature {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.r),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.vs),
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
        impl alloy_sol_types::SolType for CompactSignature {
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
        impl alloy_sol_types::SolStruct for CompactSignature {
            const NAME: &'static str = "CompactSignature";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "CompactSignature(bytes32 r,bytes32 vs)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.r)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.vs)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for CompactSignature {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.r)
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.vs)
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
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.r, out);
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.vs, out);
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
    /**```solidity
struct VRFParameters { uint256 vx; uint256 vy; uint256 s; uint256 h; uint256 sBx; uint256 sBy; uint256 hVx; uint256 hVy; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct VRFParameters {
        #[allow(missing_docs)]
        pub vx: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub vy: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub s: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub h: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub sBx: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub sBy: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub hVx: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub hVy: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<VRFParameters> for UnderlyingRustTuple<'_> {
            fn from(value: VRFParameters) -> Self {
                (
                    value.vx,
                    value.vy,
                    value.s,
                    value.h,
                    value.sBx,
                    value.sBy,
                    value.hVx,
                    value.hVy,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for VRFParameters {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    vx: tuple.0,
                    vy: tuple.1,
                    s: tuple.2,
                    h: tuple.3,
                    sBx: tuple.4,
                    sBy: tuple.5,
                    hVx: tuple.6,
                    hVy: tuple.7,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for VRFParameters {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for VRFParameters {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.vx),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.vy),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.s),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.h),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.sBx),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.sBy),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.hVx),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.hVy),
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
        impl alloy_sol_types::SolType for VRFParameters {
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
        impl alloy_sol_types::SolStruct for VRFParameters {
            const NAME: &'static str = "VRFParameters";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "VRFParameters(uint256 vx,uint256 vy,uint256 s,uint256 h,uint256 sBx,uint256 sBy,uint256 hVx,uint256 hVy)",
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.vx)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.vy)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.s)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.h)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.sBx)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.sBy)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.hVx)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.hVy)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for VRFParameters {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.vx)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.vy)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.s)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.h)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.sBx)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.sBy)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.hVx)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.hVy)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.vx, out);
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.vy, out);
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.s, out);
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.h, out);
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.sBx, out);
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.sBy, out);
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.hVx, out);
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.hVy, out);
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
    /**Creates a new wrapper around an on-chain [`HoprCrypto`](self) contract instance.

See the [wrapper's documentation](`HoprCryptoInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> HoprCryptoInstance<P, N> {
        HoprCryptoInstance::<P, N>::new(address, __provider)
    }
    /**A [`HoprCrypto`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`HoprCrypto`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct HoprCryptoInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for HoprCryptoInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("HoprCryptoInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > HoprCryptoInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`HoprCrypto`](self) contract instance.

See the [wrapper's documentation](`HoprCryptoInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> HoprCryptoInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> HoprCryptoInstance<P, N> {
            HoprCryptoInstance {
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
    > HoprCryptoInstance<P, N> {
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
    > HoprCryptoInstance<P, N> {
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
library HoprChannelsType {
    type ChannelStatus is uint8;
    type Balance is uint96;
    type ChannelEpoch is uint24;
    type TicketIndex is uint48;
    type Timestamp is uint32;
    type WinProb is uint56;
}

library HoprCrypto {
    struct CompactSignature {
        bytes32 r;
        bytes32 vs;
    }
    struct VRFParameters {
        uint256 vx;
        uint256 vy;
        uint256 s;
        uint256 h;
        uint256 sBx;
        uint256 sBy;
        uint256 hVx;
        uint256 hVy;
    }
}

library HoprLedger {
    struct RootStruct {
        bytes28 rootHash;
        uint32 timestamp;
    }
}

interface HoprChannels {
    struct RedeemableTicket {
        TicketData data;
        HoprCrypto.CompactSignature signature;
        uint256 porSecret;
    }
    struct TicketData {
        bytes32 channelId;
        HoprChannelsType.Balance amount;
        HoprChannelsType.TicketIndex ticketIndex;
        HoprChannelsType.ChannelEpoch epoch;
        HoprChannelsType.WinProb winProb;
    }

    error AddressEmptyCode(address target);
    error AlreadyInitialized();
    error BalanceExceedsGlobalPerChannelAllowance();
    error ContractNotResponsible();
    error ECDSAInvalidSignature();
    error ECDSAInvalidSignatureLength(uint256 length);
    error ECDSAInvalidSignatureS(bytes32 s);
    error FailedCall();
    error InsufficientChannelBalance();
    error InvalidBalance();
    error InvalidCurvePoint();
    error InvalidFieldElement();
    error InvalidNoticePeriod();
    error InvalidPointWitness();
    error InvalidSafeAddress();
    error InvalidTicketIndex();
    error InvalidTicketSignature();
    error InvalidTokenRecipient();
    error InvalidTokensReceivedUsage();
    error InvalidVRFProof();
    error MultiSigUninitialized();
    error NoticePeriodNotDue();
    error SourceEqualsDestination();
    error TicketIsNotAWin();
    error TokenTransferFailed();
    error WrongChannelState(string reason);
    error WrongToken();
    error ZeroAddress(string reason);
    error ZeroInterval();

    event ChannelBalanceDecreased(bytes32 indexed channelId, bytes32 channel);
    event ChannelBalanceIncreased(bytes32 indexed channelId, bytes32 channel);
    event ChannelClosed(bytes32 indexed channelId, bytes32 channel);
    event ChannelOpened(bytes32 indexed channelId, address indexed source, address indexed destination, bytes32 channel);
    event DomainSeparatorUpdated(bytes32 indexed domainSeparator);
    event LedgerDomainSeparatorUpdated(bytes32 indexed ledgerDomainSeparator);
    event OutgoingChannelClosureInitiated(bytes32 indexed channelId, bytes32 channel);
    event TicketRedeemed(bytes32 indexed channelId, bytes32 channel);

    constructor(address _token, HoprChannelsType.Timestamp _noticePeriodChannelClosure, address _safeRegistry);

    function ERC777_HOOK_FUND_CHANNEL_MULTI_SIZE() external view returns (uint256);
    function ERC777_HOOK_FUND_CHANNEL_SIZE() external view returns (uint256);
    function LEDGER_VERSION() external view returns (string memory);
    function MAX_USED_BALANCE() external view returns (HoprChannelsType.Balance);
    function MIN_USED_BALANCE() external view returns (HoprChannelsType.Balance);
    function NOTICE_PERIOD_CHANNEL_CLOSURE() external view returns (HoprChannelsType.Timestamp);
    function SNAPSHOT_INTERVAL() external view returns (uint256);
    function TOKEN() external view returns (address);
    function TOKENS_RECIPIENT_INTERFACE_HASH() external view returns (bytes32);
    function VERSION() external view returns (string memory);
    function _currentBlockTimestamp() external view returns (HoprChannelsType.Timestamp);
    function _getChannelId(address source, address destination) external pure returns (bytes32);
    function _getTicketHash(RedeemableTicket memory redeemable) external view returns (bytes32);
    function _isWinningTicket(bytes32 ticketHash, RedeemableTicket memory redeemable, HoprCrypto.VRFParameters memory params) external pure returns (bool);
    function channelState(bytes32 channelId) external view returns (bytes32 state);
    function channels(bytes32) external view returns (HoprChannelsType.Balance balance, HoprChannelsType.TicketIndex ticketIndex, HoprChannelsType.Timestamp closureTime, HoprChannelsType.ChannelEpoch epoch, HoprChannelsType.ChannelStatus status);
    function closeIncomingChannel(address source) external;
    function closeIncomingChannelSafe(address selfAddress, address source) external;
    function domainSeparator() external view returns (bytes32);
    function finalizeOutgoingChannelClosure(address destination) external;
    function finalizeOutgoingChannelClosureSafe(address selfAddress, address destination) external;
    function fundChannel(address account, HoprChannelsType.Balance amount) external;
    function fundChannelSafe(address selfAddress, address account, HoprChannelsType.Balance amount) external;
    function initiateOutgoingChannelClosure(address destination) external;
    function initiateOutgoingChannelClosureSafe(address selfAddress, address destination) external;
    function latestRoot() external view returns (HoprLedger.RootStruct memory);
    function latestSnapshotRoot() external view returns (HoprLedger.RootStruct memory);
    function ledgerDomainSeparator() external view returns (bytes32);
    function multicall(bytes[] memory data) external returns (bytes[] memory results);
    function redeemTicket(RedeemableTicket memory redeemable, HoprCrypto.VRFParameters memory params) external;
    function redeemTicketSafe(address selfAddress, RedeemableTicket memory redeemable, HoprCrypto.VRFParameters memory params) external;
    function tokensReceived(address, address from, address to, uint256 amount, bytes memory userData, bytes memory) external;
    function updateDomainSeparator() external;
    function updateLedgerDomainSeparator() external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_token",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_noticePeriodChannelClosure",
        "type": "uint32",
        "internalType": "HoprChannelsType.Timestamp"
      },
      {
        "name": "_safeRegistry",
        "type": "address",
        "internalType": "contract HoprNodeSafeRegistry"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "ERC777_HOOK_FUND_CHANNEL_MULTI_SIZE",
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
    "name": "ERC777_HOOK_FUND_CHANNEL_SIZE",
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
    "name": "MAX_USED_BALANCE",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint96",
        "internalType": "HoprChannelsType.Balance"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "MIN_USED_BALANCE",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint96",
        "internalType": "HoprChannelsType.Balance"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "NOTICE_PERIOD_CHANNEL_CLOSURE",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "HoprChannelsType.Timestamp"
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
        "internalType": "contract IERC20"
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
    "name": "_currentBlockTimestamp",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "HoprChannelsType.Timestamp"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "_getChannelId",
    "inputs": [
      {
        "name": "source",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "destination",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "_getTicketHash",
    "inputs": [
      {
        "name": "redeemable",
        "type": "tuple",
        "internalType": "struct HoprChannels.RedeemableTicket",
        "components": [
          {
            "name": "data",
            "type": "tuple",
            "internalType": "struct HoprChannels.TicketData",
            "components": [
              {
                "name": "channelId",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "amount",
                "type": "uint96",
                "internalType": "HoprChannelsType.Balance"
              },
              {
                "name": "ticketIndex",
                "type": "uint48",
                "internalType": "HoprChannelsType.TicketIndex"
              },
              {
                "name": "epoch",
                "type": "uint24",
                "internalType": "HoprChannelsType.ChannelEpoch"
              },
              {
                "name": "winProb",
                "type": "uint56",
                "internalType": "HoprChannelsType.WinProb"
              }
            ]
          },
          {
            "name": "signature",
            "type": "tuple",
            "internalType": "struct HoprCrypto.CompactSignature",
            "components": [
              {
                "name": "r",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "vs",
                "type": "bytes32",
                "internalType": "bytes32"
              }
            ]
          },
          {
            "name": "porSecret",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
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
    "name": "_isWinningTicket",
    "inputs": [
      {
        "name": "ticketHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "redeemable",
        "type": "tuple",
        "internalType": "struct HoprChannels.RedeemableTicket",
        "components": [
          {
            "name": "data",
            "type": "tuple",
            "internalType": "struct HoprChannels.TicketData",
            "components": [
              {
                "name": "channelId",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "amount",
                "type": "uint96",
                "internalType": "HoprChannelsType.Balance"
              },
              {
                "name": "ticketIndex",
                "type": "uint48",
                "internalType": "HoprChannelsType.TicketIndex"
              },
              {
                "name": "epoch",
                "type": "uint24",
                "internalType": "HoprChannelsType.ChannelEpoch"
              },
              {
                "name": "winProb",
                "type": "uint56",
                "internalType": "HoprChannelsType.WinProb"
              }
            ]
          },
          {
            "name": "signature",
            "type": "tuple",
            "internalType": "struct HoprCrypto.CompactSignature",
            "components": [
              {
                "name": "r",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "vs",
                "type": "bytes32",
                "internalType": "bytes32"
              }
            ]
          },
          {
            "name": "porSecret",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct HoprCrypto.VRFParameters",
        "components": [
          {
            "name": "vx",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "vy",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "s",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "h",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "sBx",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "sBy",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "hVx",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "hVy",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "channelState",
    "inputs": [
      {
        "name": "channelId",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "state",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "channels",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "balance",
        "type": "uint96",
        "internalType": "HoprChannelsType.Balance"
      },
      {
        "name": "ticketIndex",
        "type": "uint48",
        "internalType": "HoprChannelsType.TicketIndex"
      },
      {
        "name": "closureTime",
        "type": "uint32",
        "internalType": "HoprChannelsType.Timestamp"
      },
      {
        "name": "epoch",
        "type": "uint24",
        "internalType": "HoprChannelsType.ChannelEpoch"
      },
      {
        "name": "status",
        "type": "uint8",
        "internalType": "enum HoprChannelsType.ChannelStatus"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "closeIncomingChannel",
    "inputs": [
      {
        "name": "source",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "closeIncomingChannelSafe",
    "inputs": [
      {
        "name": "selfAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "source",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "domainSeparator",
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
    "name": "finalizeOutgoingChannelClosure",
    "inputs": [
      {
        "name": "destination",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "finalizeOutgoingChannelClosureSafe",
    "inputs": [
      {
        "name": "selfAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "destination",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "fundChannel",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint96",
        "internalType": "HoprChannelsType.Balance"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "fundChannelSafe",
    "inputs": [
      {
        "name": "selfAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint96",
        "internalType": "HoprChannelsType.Balance"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "initiateOutgoingChannelClosure",
    "inputs": [
      {
        "name": "destination",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "initiateOutgoingChannelClosureSafe",
    "inputs": [
      {
        "name": "selfAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "destination",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
    "name": "multicall",
    "inputs": [
      {
        "name": "data",
        "type": "bytes[]",
        "internalType": "bytes[]"
      }
    ],
    "outputs": [
      {
        "name": "results",
        "type": "bytes[]",
        "internalType": "bytes[]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "redeemTicket",
    "inputs": [
      {
        "name": "redeemable",
        "type": "tuple",
        "internalType": "struct HoprChannels.RedeemableTicket",
        "components": [
          {
            "name": "data",
            "type": "tuple",
            "internalType": "struct HoprChannels.TicketData",
            "components": [
              {
                "name": "channelId",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "amount",
                "type": "uint96",
                "internalType": "HoprChannelsType.Balance"
              },
              {
                "name": "ticketIndex",
                "type": "uint48",
                "internalType": "HoprChannelsType.TicketIndex"
              },
              {
                "name": "epoch",
                "type": "uint24",
                "internalType": "HoprChannelsType.ChannelEpoch"
              },
              {
                "name": "winProb",
                "type": "uint56",
                "internalType": "HoprChannelsType.WinProb"
              }
            ]
          },
          {
            "name": "signature",
            "type": "tuple",
            "internalType": "struct HoprCrypto.CompactSignature",
            "components": [
              {
                "name": "r",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "vs",
                "type": "bytes32",
                "internalType": "bytes32"
              }
            ]
          },
          {
            "name": "porSecret",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct HoprCrypto.VRFParameters",
        "components": [
          {
            "name": "vx",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "vy",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "s",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "h",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "sBx",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "sBy",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "hVx",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "hVy",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "redeemTicketSafe",
    "inputs": [
      {
        "name": "selfAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "redeemable",
        "type": "tuple",
        "internalType": "struct HoprChannels.RedeemableTicket",
        "components": [
          {
            "name": "data",
            "type": "tuple",
            "internalType": "struct HoprChannels.TicketData",
            "components": [
              {
                "name": "channelId",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "amount",
                "type": "uint96",
                "internalType": "HoprChannelsType.Balance"
              },
              {
                "name": "ticketIndex",
                "type": "uint48",
                "internalType": "HoprChannelsType.TicketIndex"
              },
              {
                "name": "epoch",
                "type": "uint24",
                "internalType": "HoprChannelsType.ChannelEpoch"
              },
              {
                "name": "winProb",
                "type": "uint56",
                "internalType": "HoprChannelsType.WinProb"
              }
            ]
          },
          {
            "name": "signature",
            "type": "tuple",
            "internalType": "struct HoprCrypto.CompactSignature",
            "components": [
              {
                "name": "r",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "vs",
                "type": "bytes32",
                "internalType": "bytes32"
              }
            ]
          },
          {
            "name": "porSecret",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct HoprCrypto.VRFParameters",
        "components": [
          {
            "name": "vx",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "vy",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "s",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "h",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "sBx",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "sBy",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "hVx",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "hVy",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
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
    "name": "updateDomainSeparator",
    "inputs": [],
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
    "type": "event",
    "name": "ChannelBalanceDecreased",
    "inputs": [
      {
        "name": "channelId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "channel",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ChannelBalanceIncreased",
    "inputs": [
      {
        "name": "channelId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "channel",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ChannelClosed",
    "inputs": [
      {
        "name": "channelId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "channel",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ChannelOpened",
    "inputs": [
      {
        "name": "channelId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "source",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "destination",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "channel",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "DomainSeparatorUpdated",
    "inputs": [
      {
        "name": "domainSeparator",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
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
    "name": "OutgoingChannelClosureInitiated",
    "inputs": [
      {
        "name": "channelId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "channel",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TicketRedeemed",
    "inputs": [
      {
        "name": "channelId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "channel",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
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
    "name": "BalanceExceedsGlobalPerChannelAllowance",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ContractNotResponsible",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ECDSAInvalidSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ECDSAInvalidSignatureLength",
    "inputs": [
      {
        "name": "length",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ECDSAInvalidSignatureS",
    "inputs": [
      {
        "name": "s",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "FailedCall",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InsufficientChannelBalance",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidBalance",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidCurvePoint",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidFieldElement",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidNoticePeriod",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidPointWitness",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidSafeAddress",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidTicketIndex",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidTicketSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidTokenRecipient",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidTokensReceivedUsage",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidVRFProof",
    "inputs": []
  },
  {
    "type": "error",
    "name": "MultiSigUninitialized",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NoticePeriodNotDue",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SourceEqualsDestination",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TicketIsNotAWin",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TokenTransferFailed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "WrongChannelState",
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
pub mod HoprChannels {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6005805460ff60a01b191690555f610120819052610134819052610140819052610154819052604061010081905260808190526101808290526101949190915260286101608190526101a890915260a05234801561005b575f5ffd5b5060405161480338038061480383398101604081905261007a916105de565b6201518061008781610223565b508163ffffffff165f036100ae5760405163f9ee910760e01b815260040160405180910390fd5b6001600160a01b03831661010a5760405163eac0d38960e01b815260206004820152601860248201527f5f746f6b656e206d757374206e6f7420626520656d707479000000000000000060448201526064015b60405180910390fd5b6001600160a01b0381166101615760405163eac0d38960e01b815260206004820152601f60248201527f5f736166655265676973747279206d757374206e6f7420626520656d707479006044820152606401610101565b61016a816102ba565b6001600160a01b03831660c05263ffffffff821660e0526040516329965a1d60e01b815230600482018190527fb281fc8c12954d22544db45de3159a39272895b169a852b314f9cc762e44c53b60248301526044820152731820a4b7618bde71dce8cdc73aab6c95905fad24906329965a1d906064015f604051808303815f87803b1580156101f7575f5ffd5b505af1158015610209573d5f5f3e3d5ffd5b5050505061021b61033460201b60201c565b505050610655565b805f036102435760405163346ff60760e01b815260040160405180910390fd5b60018190556040516001600160601b03193060601b16602082015260340160408051808303601f190181529190528051602091820120901c600160e01b4263ffffffff90811682029290921760028190556001600160e01b0380821691839004909316909102176003556102b79061043016565b50565b600554600160a01b900460ff16156102e45760405162dc149f60e41b815260040160405180910390fd5b6001600160a01b03811661030b5760405163474ebe2f60e11b815260040160405180910390fd5b600580546001600160a01b039092166001600160a81b031990921691909117600160a01b179055565b604080518082018252600c81526b486f70724368616e6e656c7360a01b6020918201528151808301835260058152640322e302e360dc1b9082015281515f5160206147e35f395f51905f5281527f84e6908f343601d9ce9fb60d8250394eb8a51c56f1876bc1e017c97acd6567f2918101919091525f5160206147c35f395f51905f529181019190915246606082015230608082015260a0902060065481146102b7576006819055604080515f5160206147a35f395f51905f5260208201529081018290526104159060600160408051601f19818403018152919052610503565b6006546040515f5160206147a35f395f51905f52905f90a250565b604080518082018252600a8152692437b8392632b233b2b960b11b6020918201528151808301835260058152640322e302e360dc1b9082015281515f5160206147e35f395f51905f5281527f6cd681790c78c220517b099a737f8e85f69e797abe4e2910fb189b61db4bf2cd918101919091525f5160206147c35f395f51905f529181019190915246606082015230608082015260a0902060045481146102b757600481905560405181907fa43fad83920fd09445855e854e73c9c532e17402c9ceb09993a2392843a5bdb9905f90a250565b6001546002545f9161052191600160e01b900463ffffffff16610630565b42111561052c575060015b600454600254835160208086019190912060408051808401959095524360e01b6001600160e01b0319169085015291901b63ffffffff19166044830152606082015260800160408051601f19818403018152919052805160209182012063ffffffff4216600160e01b02911c1760025580156105c6576002546001600160e01b038116600160e01b9182900463ffffffff16909102176003555b5050565b6001600160a01b03811681146102b7575f5ffd5b5f5f5f606084860312156105f0575f5ffd5b83516105fb816105ca565b602085015190935063ffffffff81168114610614575f5ffd5b6040850151909250610625816105ca565b809150509250925092565b8082018082111561064f57634e487b7160e01b5f52601160045260245ffd5b92915050565b60805160a05160c05160e0516140dc6106c75f395f818161057f015261254d01525f818161047f01528181610601015281816109a3015281816116d001528181611c4901528181611e42015261236701525f81816102ef015261067001525f81816103d501526107c801526140dc5ff3fe608060405234801561000f575f5ffd5b50600436106101fc575f3560e01c80637a7ebd7b11610114578063be9babdc116100a9578063dc96fd5011610079578063dc96fd50146105a1578063ddad1902146105a9578063f698da25146105da578063fc55309a146105e3578063ffa1ad74146105a9575f5ffd5b8063be9babdc1461051e578063c966c4fe14610531578063d7b0fef11461053a578063d94cc1b41461057a575f5ffd5b8063ab66ccab116100e4578063ab66ccab146104c1578063ac9650d8146104d4578063b920deed146104f4578063bda65f451461050b575f5ffd5b80637a7ebd7b146103f75780637c8e28da1461046757806382bfefc81461047a57806389ccfe89146104b9575f5ffd5b806354a2edf5116101955780636d2beef1116101655780636d2beef11461036a57806372581cc014610373578063762bd76b1461039a57806378d149a8146103ad57806378d8016d146103d0575f5ffd5b806354a2edf51461031f5780635d2f07c514610332578063651514bf1461034457806365e3fa7214610357575f5ffd5b806323cb3ac0116101d057806323cb3ac0146102a457806329392e32146102b75780632d50b18b146102d757806344dae6f8146102ea575f5ffd5b806223de29146102005780630abec58f146102155780630df18f94146102285780631a7ffe7a14610291575b5f5ffd5b61021361020e366004613933565b6105f6565b005b6102136102233660046139fd565b6108af565b6040805180820182525f808252602091820152815180830190925260035463ffffffff1981831b168352600160e01b900463ffffffff16908201525b60408051825163ffffffff1916815260209283015163ffffffff1692810192909252015b60405180910390f35b61021361029f366004613a41565b610a3d565b6102136102b2366004613a41565b610b05565b6102bf600181565b6040516001600160601b039091168152602001610288565b6102136102e5366004613a73565b610bca565b6103117f000000000000000000000000000000000000000000000000000000000000000081565b604051908152602001610288565b61021361032d366004613ab0565b610c94565b6102bf6a084595161401484a00000081565b610213610352366004613ab0565b610d62565b610213610365366004613ae7565b610e2b565b61031160015481565b6103117fb281fc8c12954d22544db45de3159a39272895b169a852b314f9cc762e44c53b81565b6103116103a8366004613b1c565b610ef5565b6103c06103bb366004613b33565b610f0a565b6040519015158152602001610288565b6103117f000000000000000000000000000000000000000000000000000000000000000081565b610456610405366004613b1c565b5f602081905290815260409020546001600160601b03811690600160601b810465ffffffffffff1690600160901b810463ffffffff1690600160b01b810462ffffff1690600160c81b900460ff1685565b604051610288959493929190613b6b565b610213610475366004613a41565b610f99565b6104a17f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610288565b61021361105e565b6103116104cf366004613bcc565b6111a6565b6104e76104e2366004613be7565b6112f4565b6040516102889190613c86565b425b60405163ffffffff9091168152602001610288565b610213610519366004613ab0565b6113da565b61031161052c366004613ab0565b6114a3565b61031160045481565b6040805180820182525f808252602091820152815180830190925260025463ffffffff1981831b168352600160e01b900463ffffffff1690820152610264565b6104f67f000000000000000000000000000000000000000000000000000000000000000081565b6102136114e7565b6105cd604051806040016040528060058152602001640322e302e360dc1b81525081565b6040516102889190613ce9565b61031160065481565b6102136105f1366004613cfb565b6115e0565b336001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161461063f57604051635079ff7560e11b815260040160405180910390fd5b6001600160a01b038616301461066857604051631738922160e31b815260040160405180910390fd5b82156108a5577f000000000000000000000000000000000000000000000000000000000000000083036107c6576001600160601b038511156106bd5760405163293ceef960e21b815260040160405180910390fd5b6005546040516302265e3160e61b81528535606090811c6004830181905292601488013590911c915f916001600160a01b0316906389978c4090602401602060405180830381865afa158015610715573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107399190613d25565b9050826001600160a01b03168a6001600160a01b031603610781576001600160a01b0381161561077c5760405163acd5a82360e01b815260040160405180910390fd5b6107b3565b896001600160a01b0316816001600160a01b0316146107b35760405163acd5a82360e01b815260040160405180910390fd5b6107be83838a611764565b5050506108a5565b7f0000000000000000000000000000000000000000000000000000000000000000830361088c578335606090811c90601486013560a090811c916020880135901c906034880135901c881580610831575061082d6001600160601b03808316908516613d54565b8914155b1561084f5760405163c52e3eff60e01b815260040160405180910390fd5b6001600160601b0383161561086957610869848385611764565b6001600160601b0381161561088357610883828583611764565b505050506108a5565b604051630d3dcde560e31b815260040160405180910390fd5b5050505050505050565b6005548390600160a01b900460ff166108db576040516308a9441960e31b815260040160405180910390fd5b6005546040516302265e3160e61b81526001600160a01b038381166004830152339216906389978c4090602401602060405180830381865afa158015610923573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109479190613d25565b6001600160a01b03161461096e5760405163acd5a82360e01b815260040160405180910390fd5b610979848484611764565b6040516323b872dd60e01b81523360048201523060248201526001600160601b03831660448201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906323b872dd906064016020604051808303815f875af11580156109f1573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a159190613d67565b1515600114610a375760405163022e258160e11b815260040160405180910390fd5b50505050565b600554600160a01b900460ff16610a67576040516308a9441960e31b815260040160405180910390fd5b6005546040516302265e3160e61b81523360048201525f916001600160a01b0316906389978c4090602401602060405180830381865afa158015610aad573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ad19190613d25565b6001600160a01b031614610af85760405163acd5a82360e01b815260040160405180910390fd5b610b023382611b4c565b50565b600554600160a01b900460ff16610b2f576040516308a9441960e31b815260040160405180910390fd5b6005546040516302265e3160e61b81523360048201525f916001600160a01b0316906389978c4090602401602060405180830381865afa158015610b75573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b999190613d25565b6001600160a01b031614610bc05760405163acd5a82360e01b815260040160405180910390fd5b610b023382611cdd565b6005548390600160a01b900460ff16610bf6576040516308a9441960e31b815260040160405180910390fd5b6005546040516302265e3160e61b81526001600160a01b038381166004830152339216906389978c4090602401602060405180830381865afa158015610c3e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c629190613d25565b6001600160a01b031614610c895760405163acd5a82360e01b815260040160405180910390fd5b610a37848484611e79565b6005548290600160a01b900460ff16610cc0576040516308a9441960e31b815260040160405180910390fd5b6005546040516302265e3160e61b81526001600160a01b038381166004830152339216906389978c4090602401602060405180830381865afa158015610d08573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d2c9190613d25565b6001600160a01b031614610d535760405163acd5a82360e01b815260040160405180910390fd5b610d5d8383611b4c565b505050565b6005548290600160a01b900460ff16610d8e576040516308a9441960e31b815260040160405180910390fd5b6005546040516302265e3160e61b81526001600160a01b038381166004830152339216906389978c4090602401602060405180830381865afa158015610dd6573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610dfa9190613d25565b6001600160a01b031614610e215760405163acd5a82360e01b815260040160405180910390fd5b610d5d8383611cdd565b600554600160a01b900460ff16610e55576040516308a9441960e31b815260040160405180910390fd5b6005546040516302265e3160e61b81523360048201525f916001600160a01b0316906389978c4090602401602060405180830381865afa158015610e9b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ebf9190613d25565b6001600160a01b031614610ee65760405163acd5a82360e01b815260040160405180910390fd5b610ef1338383611e79565b5050565b5f818152602081905260408120545b92915050565b60408051602080820186905283359282019290925290820135606082015260e083810135608083015260a0808501359083015260c080850135908301525f9182910160408051601f19818403018152919052805160209091012060c81c9050610f7960a0850160808601613d86565b66ffffffffffffff168166ffffffffffffff1611159150505b9392505050565b600554600160a01b900460ff16610fc3576040516308a9441960e31b815260040160405180910390fd5b6005546040516302265e3160e61b81523360048201525f916001600160a01b0316906389978c4090602401602060405180830381865afa158015611009573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061102d9190613d25565b6001600160a01b0316146110545760405163acd5a82360e01b815260040160405180910390fd5b610b0233826124f2565b604080518082018252600c81526b486f70724368616e6e656c7360a01b6020918201528151808301835260058152640322e302e360dc1b9082015281517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f81527f84e6908f343601d9ce9fb60d8250394eb8a51c56f1876bc1e017c97acd6567f2918101919091527fb4bcb154e38601c389396fa918314da42d4626f13ef6d0ceb07e5f5d26b2fbc39181019190915246606082015230608082015260a090206006548114610b02576006819055604080517f771f5240ae5fd8a7640d3fb82fa70aab2fb1dbf35f2ef464f8509946717664c56020820152908101829052611178906060015b60405160208183030381529060405261262a565b6006546040517f771f5240ae5fd8a7640d3fb82fa70aab2fb1dbf35f2ef464f8509946717664c5905f90a250565b5f5f6111b58360e001356126f0565b90505f6111c860a0850160808601613d86565b66ffffffffffffff1660386111e36080870160608801613dac565b62ffffff16901b60506111fc6060880160408901613dce565b65ffffffffffff16901b60806112186040890160208a01613df3565b6001600160601b0316901b17171790505f6112a56365e3fa7260e01b6001600160e01b031916865f015f0135848660405160200161128193929190928352602091821b63ffffffff19169183019190915260601b6001600160601b031916603c82015260500190565b604051602081830303815290604052805190602001205f9182526020526040902090565b600654604051601960f81b6020820152600160f81b6021820152602281019190915260428101829052909150606201604051602081830303815290604052805190602001209350505050919050565b604080515f8152602081019091526060908267ffffffffffffffff81111561131e5761131e613e1f565b60405190808252806020026020018201604052801561135157816020015b606081526020019060019003908161133c5790505b5091505f5b838110156113d2576113ad3086868481811061137457611374613e33565b90506020028101906113869190613e47565b8560405160200161139993929190613ea1565b6040516020818303038152906040526127a9565b8382815181106113bf576113bf613e33565b6020908102919091010152600101611356565b505092915050565b6005548290600160a01b900460ff16611406576040516308a9441960e31b815260040160405180910390fd5b6005546040516302265e3160e61b81526001600160a01b038381166004830152339216906389978c4090602401602060405180830381865afa15801561144e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114729190613d25565b6001600160a01b0316146114995760405163acd5a82360e01b815260040160405180910390fd5b610d5d83836124f2565b6040516001600160601b0319606084811b8216602084015283901b1660348201525f9060480160405160208183030381529060405280519060200120905092915050565b604080518082018252600a8152692437b8392632b233b2b960b11b6020918201528151808301835260058152640322e302e360dc1b9082015281517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f81527f6cd681790c78c220517b099a737f8e85f69e797abe4e2910fb189b61db4bf2cd918101919091527fb4bcb154e38601c389396fa918314da42d4626f13ef6d0ceb07e5f5d26b2fbc39181019190915246606082015230608082015260a090206004548114610b0257600481905560405181907fa43fad83920fd09445855e854e73c9c532e17402c9ceb09993a2392843a5bdb9905f90a250565b600554600160a01b900460ff1661160a576040516308a9441960e31b815260040160405180910390fd5b6005546040516302265e3160e61b81523360048201525f916001600160a01b0316906389978c4090602401602060405180830381865afa158015611650573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116749190613d25565b6001600160a01b03161461169b5760405163acd5a82360e01b815260040160405180910390fd5b6116a6338383611764565b6040516323b872dd60e01b81523360048201523060248201526001600160601b03821660448201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906323b872dd906064016020604051808303815f875af115801561171e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906117429190613d67565b1515600114610ef15760405163022e258160e11b815260040160405180910390fd5b8060016001600160601b03821610156117905760405163c52e3eff60e01b815260040160405180910390fd5b6a084595161401484a0000006001600160601b03821611156117c55760405163293ceef960e21b815260040160405180910390fd5b8383806001600160a01b0316826001600160a01b0316036117f957604051634bd1d76960e11b815260040160405180910390fd5b6001600160a01b0382166118555760405163eac0d38960e01b815260206004820152601860248201527f736f75726365206d757374206e6f7420626520656d707479000000000000000060448201526064015b60405180910390fd5b6001600160a01b0381166118ac5760405163eac0d38960e01b815260206004820152601d60248201527f64657374696e6174696f6e206d757374206e6f7420626520656d707479000000604482015260640161184c565b5f6118b787876114a3565b5f81815260208190526040902090915060028154600160c81b900460ff1660028111156118e6576118e6613b57565b036119475760405163499463c160e01b815260206004820152602a60248201527f63616e6e6f742066756e642061206368616e6e656c20746861742077696c6c2060448201526931b637b9b29039b7b7b760b11b606482015260840161184c565b805461195d9087906001600160601b0316613ec0565b81546001600160601b0319166001600160601b03919091161781555f8154600160c81b900460ff16600281111561199657611996613b57565b03611ac75780546119b490600160b01b900462ffffff166001613edf565b815462ffffff91909116600160b01b026dff00000000000000ffffffffffff60601b19166dffffffff00000000ffffffffffff60601b1990911617600160c81b1781555f82815260208190526040902054611a6b907e4c0177ad15bb302b4419e9ac96065d6f436e0f61972f97bbe272834c40f9349084908b908b906040805160208101969096528501939093526001600160601b0319606092831b811683860152911b166074830152608882015260a801611164565b866001600160a01b0316886001600160a01b0316837e4c0177ad15bb302b4419e9ac96065d6f436e0f61972f97bbe272834c40f934611ab5865f9081526020819052604090205490565b60405190815260200160405180910390a45b611b0d5f5160206140675f395f51905f5283611aee855f9081526020819052604090205490565b6040805160208101949094528301919091526060820152608001611164565b815f5160206140675f395f51905f52611b31845f9081526020819052604090205490565b60405190815260200160405180910390a25050505050505050565b5f611b5782846114a3565b5f8181526020819052604081209192508154600160c81b900460ff166002811115611b8457611b84613b57565b03611ba25760405163499463c160e01b815260040161184c90613efa565b8054600163ff00000160b01b0319811682555f838152602081905260409020546001600160601b0390911690611be8905f5160206140875f395f51905f52908590611aee565b825f5160206140875f395f51905f52611c0c855f9081526020819052604090205490565b60405190815260200160405180910390a28015611cd65760405163a9059cbb60e01b81526001600160a01b038581166004830152602482018390527f0000000000000000000000000000000000000000000000000000000000000000169063a9059cbb906044015b6020604051808303815f875af1158015611c90573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611cb49190613d67565b1515600114611cd65760405163022e258160e11b815260040160405180910390fd5b5050505050565b5f611ce883836114a3565b5f81815260208190526040902090915060028154600160c81b900460ff166002811115611d1757611d17613b57565b14611d745760405163499463c160e01b815260206004820152602660248201527f6368616e6e656c207374617465206d7573742062652050454e44494e475f544f6044820152655f434c4f534560d01b606482015260840161184c565b805463ffffffff428116600160901b9092041610611da5576040516338b2019560e11b815260040160405180910390fd5b8054600163ff00000160b01b0319811682555f838152602081905260409020546001600160601b0390911690611deb905f5160206140875f395f51905f52908590611aee565b825f5160206140875f395f51905f52611e0f855f9081526020819052604090205490565b60405190815260200160405180910390a28015611cd65760405163a9059cbb60e01b8152336004820152602481018290527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063a9059cbb90604401611c74565b611e896040830160208401613df3565b60016001600160601b0382161015611eb45760405163c52e3eff60e01b815260040160405180910390fd5b6a084595161401484a0000006001600160601b0382161115611ee95760405163293ceef960e21b815260040160405180910390fd5b8260e00135611ef78161281b565b611f1457604051633ae4ed6b60e01b815260040160405180910390fd5b83355f90815260208190526040902060018154600160c81b900460ff166002811115611f4257611f42613b57565b14158015611f6d575060028154600160c81b900460ff166002811115611f6a57611f6a613b57565b14155b15611fd55760405163499463c160e01b815260206004820152603160248201527f7370656e64696e67206368616e6e656c206d757374206265204f50454e206f726044820152702050454e44494e475f544f5f434c4f534560781b606482015260840161184c565b611fe56080860160608701613dac565b8154600160b01b900462ffffff9081169116146120455760405163499463c160e01b815260206004820152601860248201527f6368616e6e656c2065706f6368206d757374206d617463680000000000000000604482015260640161184c565b5f6120566060870160408801613dce565b825490915065ffffffffffff600160601b9091048116908216101561208e576040516331de484760e01b815260040160405180910390fd5b61209e6040870160208801613df3565b82546001600160601b03918216911610156120cc57604051632c51d8db60e21b815260040160405180910390fd5b5f6120d6876111a6565b90506120e3818888610f0a565b6121005760405163ee835c8960e01b815260040160405180910390fd5b5f60405180606001604052808381526020018a6001600160a01b0316815260200160065460405160200161213691815260200190565b60408051601f198184030181529190529052905061216261215c36899003890189613f4a565b8261283c565b61217f576040516312bfb7b760e31b815260040160405180910390fd5b5f6121938360a08b013560c08c0135612abb565b905088356121a1828c6114a3565b146121bf576040516366eea9ab60e11b815260040160405180910390fd5b6121ca600185613fe3565b855465ffffffffffff91909116600160601b0265ffffffffffff60601b199091161785556121fe60408a0160208b01613df3565b855461221391906001600160601b0316614001565b85546001600160601b0319166001600160601b039190911617855588355f81815260208190526040902054612269917f0c4672f14b63bb6354fac475ee498055a2784455673af224717b9770fd68d8d191611aee565b88355f818152602081905260409020547f0c4672f14b63bb6354fac475ee498055a2784455673af224717b9770fd68d8d19060405190815260200160405180910390a25f6122b78b836114a3565b5f818152602081905260408082208d35808452919092205492935090916122ff917fed22f34d154d253a7f6fd477439be59080a1554aa0d0642686d64223ab544c8a91611aee565b8a355f818152602081905260409020547fed22f34d154d253a7f6fd477439be59080a1554aa0d0642686d64223ab544c8a9060405190815260200160405180910390a25f8154600160c81b900460ff16600281111561236057612360613b57565b03612446577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663a9059cbb338d5f0160200160208101906123aa9190613df3565b6040516001600160e01b031960e085901b1681526001600160a01b0390921660048301526001600160601b031660248201526044016020604051808303815f875af11580156123fb573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061241f9190613d67565b15156001146124415760405163022e258160e11b815260040160405180910390fd5b6124e4565b61245660408c0160208d01613df3565b815461246b91906001600160601b0316613ec0565b81546001600160601b0319166001600160601b03919091161781555f828152602081905260409020546124ae905f5160206140675f395f51905f52908490611aee565b815f5160206140675f395f51905f526124d2845f9081526020819052604090205490565b60405190815260200160405180910390a25b505050505050505050505050565b5f6124fd83836114a3565b5f8181526020819052604081209192508154600160c81b900460ff16600281111561252a5761252a613b57565b036125485760405163499463c160e01b815260040161184c90613efa565b6125727f000000000000000000000000000000000000000000000000000000000000000042614020565b815463ffffffff91909116600160901b0260ff60c81b191667ff000000ffffffff60901b1990911617600160c91b1781555f828152602081905260409020546125dd907e61e8037197b4d91cf8add2a0736613459081f236d1c8ab58549ebbc330b4e3908490611aee565b817e61e8037197b4d91cf8add2a0736613459081f236d1c8ab58549ebbc330b4e3612613845f9081526020819052604090205490565b60405190815260200160405180910390a250505050565b6001546002545f9161264891600160e01b900463ffffffff16613d54565b421115612653575060015b600454600254835160208086019190912060408051808401959095524360e01b6001600160e01b0319169085015291901b63ffffffff19166044830152606082015260800160408051601f19818403018152919052805160209182012063ffffffff4216600160e01b02911c176002558015610ef15750506002546001600160e01b038116600160e01b9182900463ffffffff1690910217600355565b5f600181601b7f79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f8179870014551231950b75fc4402da1732fc9bebe197f79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f817988709604080515f8152602081018083529590955260ff909316928401929092526060830152608082015260a0016020604051602081039080840390855afa158015612798573d5f5f3e3d5ffd5b5050604051601f1901519392505050565b60605f5f846001600160a01b0316846040516127c5919061403c565b5f60405180830381855af49150503d805f81146127fd576040519150601f19603f3d011682016040523d82523d5f602084013e612802565b606091505b5091509150612812858383612ae5565b95945050505050565b5f811580610f0457505070014551231950b75fc4402da1732fc9bebe191190565b5f6401000003d019836060015110158061286057506401000003d019836040015110155b1561287e57604051633ae4ed6b60e01b815260040160405180910390fd5b61288f835f01518460200151612b41565b6128ac57604051633922a54160e11b815260040160405180910390fd5b5f5f6128fc8460200151855f01516040516020016128e392919060609290921b6001600160601b0319168252601482015260340190565b6040516020818303038152906040528560400151612b6b565b915091505f61291086604001518484612bec565b905061294b86608001518760a00151604080516020808201949094528082019290925280518083038201815260609092019052805191012090565b6001600160a01b0316816001600160a01b03161461297c57604051631dbfb9b360e31b815260040160405180910390fd5b5f6129938760600151885f01518960200151612bec565b90506129ce8760c001518860e00151604080516020808201949094528082019290925280518083038201815260609092019052805191012090565b6001600160a01b0316816001600160a01b0316146129ff57604051631dbfb9b360e31b815260040160405180910390fd5b5f5f612a2f89608001518a60a001518b60c001518c60e001516401000003d019612a299190613e0c565b5f612c85565b6020808b01518c518d8301518d516040519698509496505f95612aa695612a8d958a928a92910160609690961b6001600160601b03191686526014860194909452603485019290925260548401526074830152609482015260b40190565b6040516020818303038152906040528a60400151612e07565b60608b01511497505050505050505092915050565b5f5f5f5f612aca878787612e74565b925092509250612ada8282612ea6565b509095945050505050565b606082612afa57612af582612f5e565b610f92565b8151158015612b1157506001600160a01b0384163b155b15612b3a57604051639996b31560e01b81526001600160a01b038516600482015260240161184c565b5080610f92565b5f6401000003d01980846401000003d019868709096007086401000003d019838409149392505050565b5f5f5f5f612b798686612f86565b915091505f5f612b888461303b565b915091505f5f612b978561303b565b915091505f5f612bca868686867f3f8731abdd661adca08a5558f0f5d272e953d363cb6f0e5d405447c01a444533612c85565b91509150612bd882826132f5565b9950995050505050505050505b9250929050565b5f80612bf9600284614047565b5f03612c075750601b612c0b565b50601c5b60015f828670014551231950b75fc4402da1732fc9bebe19888a09604080515f8152602081018083529590955260ff909316928401929092526060830152608082015260a0016020604051602081039080840390855afa158015612c71573d5f5f3e3d5ffd5b5050604051601f1901519695505050505050565b5f5f838614198588141615612c98575f5ffd5b5f5f858814878a141660018114612cb4578015612d3157612daa565b6401000003d01989600209915060405160208152602080820152602060408201528260608201526401000003d21960808201526401000003d01960a082015260208160c08360055f19fa612d06575f5ffd5b6401000003d019876401000003d019808e8f09600309086401000003d0198251820994505050612daa565b6401000003d0198a6401000003d019038908915060405160208152602080820152602060408201528260608201526401000003d21960808201526401000003d01960a082015260208160c08360055f19fa612d8a575f5ffd5b6401000003d01981516401000003d0198c6401000003d019038b08099350505b50506401000003d01980896401000003d01903886401000003d01903086401000003d0198384090892506401000003d019876401000003d019036401000003d01980866401000003d019038c088409089150509550959350505050565b5f5f5f612e1485856135dd565b9150915060405160308152602080820152602060408201528260608201528160808201526001609082015270014551231950b75fc4402da1732fc9bebe1960b082015260208160d08360055f19fa612e6a575f5ffd5b5195945050505050565b5f80806001600160ff1b038416601b60ff86901c01612e95888289856136f0565b945094509450505093509350939050565b5f826003811115612eb957612eb9613b57565b03612ec2575050565b6001826003811115612ed657612ed6613b57565b03612ef45760405163f645eedf60e01b815260040160405180910390fd5b6002826003811115612f0857612f08613b57565b03612f295760405163fce698f760e01b81526004810182905260240161184c565b6003826003811115612f3d57612f3d613b57565b03610ef1576040516335e2f38360e21b81526004810182905260240161184c565b805115612f6d57805160208201fd5b60405163d6bda27560e01b815260040160405180910390fd5b5f5f5f5f5f612f9587876137b8565b9250925092506040516030815260208082015260206040820152836060820152826080820152600160908201526401000003d01960b082015260208160d08360055f19fa612fe1575f5ffd5b80519550506040516030815260208082015282605082015260206040820152816070820152600160908201526401000003d01960b082015260208160d08360055f19fa61302c575f5ffd5b80519450505050509250929050565b5f5f6401000003d0198384096401000003d019816401000003db190990506401000003d0198182096401000003d01982820890506401000003d019600182086401000003d0196106eb820990505f82156001811461309e5780156130ac576130b8565b6401000003db1991506130b8565b836401000003d0190391505b506401000003d019817f3f8731abdd661adca08a5558f0f5d272e953d363cb6f0e5d405447c01a4445330990506401000003d01982830992506401000003d0198182096401000003d019817f3f8731abdd661adca08a5558f0f5d272e953d363cb6f0e5d405447c01a444533096401000003d01981860894506401000003d01984860994506401000003d01983830991506401000003d019826106eb0990506401000003d0198186089450506401000003d01983860996505f5f6401000003d0198384096401000003d0198488096401000003d0198183099150604051602081526020808201526020604082015282606082015263400000f5600160fe1b0360808201526401000003d01960a082015260208160c08360055f19fa6131db575f5ffd5b6401000003d01982825109925050506401000003d0197f31fdf302724013e57ad13fb38f842afeec184f00a74789dd286729c8303c4a5982096401000003d0198283096401000003d01986820990508881146001811461324057801561324c57613253565b60019450839550613253565b5f94508295505b505050506401000003d0198a880997506401000003d0198289099750801561327c578498508197505b50505060028506600288061461329857846401000003d0190394505b604051935060208452602080850152602060408501528060608501525050506401000003d21960808201526401000003d01960a082015260208160c08360055f19fa6132e2575f5ffd5b6401000003d01981518409925050915091565b5f5f6401000003d0198485096401000003d0198186096401000003d019807f8e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38daaaaa8c76401000003d019897f07d3d4c80bc321d5b9f315cea7fd44c5d595d2fc0bf63b92dfff1044f17c658109086401000003d01980857f534c328d23f234e6e2a413deca25caece4506144037c40314ecbd0b53d9dd262096401000003d019857f8e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38daaaaa88c0908086401000003d0197fd35771193d94918a9ca34ccbb7b640dd86cd409542f8487d9fe6b745781eb49b6401000003d019808a7fedadc6f64383dc1df7c4b2d51b54225406d36b641f5e41bbc52a56612a8c6d140986080860405160208152602080820152602060408201528160608201526401000003d21960808201526401000003d01960a082015260208160c08360055f19fa613451575f5ffd5b805191506401000003d01982840996506401000003d019807f4bda12f684bda12f684bda12f684bda12f684bda12f684bda12f684b8e38e23c6401000003d0198c7fc75e0c32d5cb7c0fa9d0a54b12a0a6d5647ab046d686da6fdffc90fc201d71a309086401000003d01980887f29a6194691f91a73715209ef6512e576722830a201be2018a765e85a9ecee931096401000003d019887f2f684bda12f684bda12f684bda12f684bda12f684bda12f684bda12f38e38d8409080892506401000003d019806401000006c4196401000003d0198c7f7a06534bb8bdb49fd5e9e6632722c2989467c1bfc8e8d978dfb425d2685c257309086401000003d01980887f6484aa716545ca2cf3a70c3fa8fe337e0a3d21162f0d6299a7bf8192bfd2a76f098708089450604051905060208152602080820152602060408201528460608201526401000003d21960808201526401000003d01960a082015260208160c08360055f19fa6135bf575f5ffd5b5193506401000003d019905083818389090993505050509250929050565b5f5f60ff835111156135ed575f5ffd5b5f6040515f5b6088811015613609575f828201526020016135f3565b50608860205f5b88518110156136315788820151848401526020928301929182019101613610565b50506089875101905060308183015360020160205f5b87518110156136685787820151848401526020928301929182019101613647565b5050608b8651885101019050855181830153508551855101608c01812091505060405181815260016020820153602160205f5b87518110156136bc578782015184840152602092830192918201910161369b565b5050508451855160210182015384516022018120935083821881526002602082015384516022018120925050509250929050565b5f80807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a084111561372957505f915060039050826137ae565b604080515f808252602082018084528a905260ff891692820192909252606081018790526080810186905260019060a0016020604051602081039080840390855afa15801561377a573d5f5f3e3d5ffd5b5050604051601f1901519150506001600160a01b0381166137a557505f9250600191508290506137ae565b92505f91508190505b9450945094915050565b5f5f5f60ff845111156137c9575f5ffd5b5f6040515f5b60888110156137e5575f828201526020016137cf565b50608860205f5b895181101561380d57898201518484015260209283019291820191016137ec565b50506089885101905060608183015360020160205f5b88518110156138445788820151848401526020928301929182019101613823565b5050608b8751895101019050865181830153508651865101608c01812091505060405181815260016020820153602160205f5b88518110156138985788820151848401526020928301929182019101613877565b5050508551865160210182015385516022018120945084821881526002602082015385516022018120935083821881526003602082015385516022018120925050509250925092565b6001600160a01b0381168114610b02575f5ffd5b5f5f83601f840112613905575f5ffd5b50813567ffffffffffffffff81111561391c575f5ffd5b602083019150836020828501011115612be5575f5ffd5b5f5f5f5f5f5f5f5f60c0898b03121561394a575f5ffd5b8835613955816138e1565b97506020890135613965816138e1565b96506040890135613975816138e1565b955060608901359450608089013567ffffffffffffffff811115613997575f5ffd5b6139a38b828c016138f5565b90955093505060a089013567ffffffffffffffff8111156139c2575f5ffd5b6139ce8b828c016138f5565b999c989b5096995094979396929594505050565b80356001600160601b03811681146139f8575f5ffd5b919050565b5f5f5f60608486031215613a0f575f5ffd5b8335613a1a816138e1565b92506020840135613a2a816138e1565b9150613a38604085016139e2565b90509250925092565b5f60208284031215613a51575f5ffd5b8135610f92816138e1565b5f6101008284031215613a6d575f5ffd5b50919050565b5f5f5f6102208486031215613a86575f5ffd5b8335613a91816138e1565b9250613aa08560208601613a5c565b9150613a38856101208601613a5c565b5f5f60408385031215613ac1575f5ffd5b8235613acc816138e1565b91506020830135613adc816138e1565b809150509250929050565b5f5f6102008385031215613af9575f5ffd5b613b038484613a5c565b9150613b13846101008501613a5c565b90509250929050565b5f60208284031215613b2c575f5ffd5b5035919050565b5f5f5f6102208486031215613b46575f5ffd5b83359250613aa08560208601613a5c565b634e487b7160e01b5f52602160045260245ffd5b6001600160601b038616815265ffffffffffff8516602082015263ffffffff8416604082015262ffffff8316606082015260a0810160038310613bbc57634e487b7160e01b5f52602160045260245ffd5b8260808301529695505050505050565b5f6101008284031215613bdd575f5ffd5b610f928383613a5c565b5f5f60208385031215613bf8575f5ffd5b823567ffffffffffffffff811115613c0e575f5ffd5b8301601f81018513613c1e575f5ffd5b803567ffffffffffffffff811115613c34575f5ffd5b8560208260051b8401011115613c48575f5ffd5b6020919091019590945092505050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015613cdd57603f19878603018452613cc8858351613c58565b94506020938401939190910190600101613cac565b50929695505050505050565b602081525f610f926020830184613c58565b5f5f60408385031215613d0c575f5ffd5b8235613d17816138e1565b9150613b13602084016139e2565b5f60208284031215613d35575f5ffd5b8151610f92816138e1565b634e487b7160e01b5f52601160045260245ffd5b80820180821115610f0457610f04613d40565b5f60208284031215613d77575f5ffd5b81518015158114610f92575f5ffd5b5f60208284031215613d96575f5ffd5b813566ffffffffffffff81168114610f92575f5ffd5b5f60208284031215613dbc575f5ffd5b813562ffffff81168114610f92575f5ffd5b5f60208284031215613dde575f5ffd5b813565ffffffffffff81168114610f92575f5ffd5b5f60208284031215613e03575f5ffd5b610f92826139e2565b81810381811115610f0457610f04613d40565b634e487b7160e01b5f52604160045260245ffd5b634e487b7160e01b5f52603260045260245ffd5b5f5f8335601e19843603018112613e5c575f5ffd5b83018035915067ffffffffffffffff821115613e76575f5ffd5b602001915036819003821315612be5575f5ffd5b5f81518060208401855e5f93019283525090919050565b828482375f8382015f8152613eb68185613e8a565b9695505050505050565b6001600160601b038181168382160190811115610f0457610f04613d40565b62ffffff8181168382160190811115610f0457610f04613d40565b60208082526030908201527f6368616e6e656c206d7573742068617665207374617465204f50454e206f722060408201526f50454e44494e475f544f5f434c4f534560801b606082015260800190565b5f610100828403128015613f5c575f5ffd5b50604051610100810167ffffffffffffffff81118282101715613f8d57634e487b7160e01b5f52604160045260245ffd5b604090815283358252602080850135908301528381013590820152606080840135908201526080808401359082015260a0808401359082015260c0808401359082015260e0928301359281019290925250919050565b65ffffffffffff8181168382160190811115610f0457610f04613d40565b6001600160601b038281168282160390811115610f0457610f04613d40565b63ffffffff8181168382160190811115610f0457610f04613d40565b5f610f928284613e8a565b5f8261406157634e487b7160e01b5f52601260045260245ffd5b50069056feec21547ca1e22edc3f9b4f4e0da94638b5b94dcb18d52dcc072abe5801a8013c197bba684a6afb7ba24f1d265605414b1d0051a6e295d6ff7b6e78e870d7a7f0a26469706673582212205b87f29e9f5a1eff814c1c2274c276ee1d2d56f541e2f217d56c2f69e2030e7564736f6c634300081e0033771f5240ae5fd8a7640d3fb82fa70aab2fb1dbf35f2ef464f8509946717664c5b4bcb154e38601c389396fa918314da42d4626f13ef6d0ceb07e5f5d26b2fbc38b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x05\x80T`\xFF`\xA0\x1B\x19\x16\x90U_a\x01 \x81\x90Ra\x014\x81\x90Ra\x01@\x81\x90Ra\x01T\x81\x90R`@a\x01\0\x81\x90R`\x80\x81\x90Ra\x01\x80\x82\x90Ra\x01\x94\x91\x90\x91R`(a\x01`\x81\x90Ra\x01\xA8\x90\x91R`\xA0R4\x80\x15a\0[W__\xFD[P`@QaH\x038\x03\x80aH\x03\x839\x81\x01`@\x81\x90Ra\0z\x91a\x05\xDEV[b\x01Q\x80a\0\x87\x81a\x02#V[P\x81c\xFF\xFF\xFF\xFF\x16_\x03a\0\xAEW`@Qc\xF9\xEE\x91\x07`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x01\nW`@Qc\xEA\xC0\xD3\x89`\xE0\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7F_token must not be empty\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x01aW`@Qc\xEA\xC0\xD3\x89`\xE0\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7F_safeRegistry must not be empty\0`D\x82\x01R`d\x01a\x01\x01V[a\x01j\x81a\x02\xBAV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\xC0Rc\xFF\xFF\xFF\xFF\x82\x16`\xE0R`@Qc)\x96Z\x1D`\xE0\x1B\x81R0`\x04\x82\x01\x81\x90R\x7F\xB2\x81\xFC\x8C\x12\x95M\"TM\xB4]\xE3\x15\x9A9'(\x95\xB1i\xA8R\xB3\x14\xF9\xCCv.D\xC5;`$\x83\x01R`D\x82\x01Rs\x18 \xA4\xB7a\x8B\xDEq\xDC\xE8\xCD\xC7:\xABl\x95\x90_\xAD$\x90c)\x96Z\x1D\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x01\xF7W__\xFD[PZ\xF1\x15\x80\x15a\x02\tW=__>=_\xFD[PPPPa\x02\x1Ba\x034` \x1B` \x1CV[PPPa\x06UV[\x80_\x03a\x02CW`@Qc4o\xF6\x07`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x81\x90U`@Q`\x01`\x01``\x1B\x03\x190``\x1B\x16` \x82\x01R`4\x01`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x91\x82\x01 \x90\x1C`\x01`\xE0\x1BBc\xFF\xFF\xFF\xFF\x90\x81\x16\x82\x02\x92\x90\x92\x17`\x02\x81\x90U`\x01`\x01`\xE0\x1B\x03\x80\x82\x16\x91\x83\x90\x04\x90\x93\x16\x90\x91\x02\x17`\x03Ua\x02\xB7\x90a\x040\x16V[PV[`\x05T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x02\xE4W`@Qb\xDC\x14\x9F`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x03\x0BW`@QcGN\xBE/`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x01`\x01`\xA8\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17`\x01`\xA0\x1B\x17\x90UV[`@\x80Q\x80\x82\x01\x82R`\x0C\x81RkHoprChannels`\xA0\x1B` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`\x05\x81Rd\x03\"\xE3\x02\xE3`\xDC\x1B\x90\x82\x01R\x81Q_Q` aG\xE3_9_Q\x90_R\x81R\x7F\x84\xE6\x90\x8F46\x01\xD9\xCE\x9F\xB6\r\x82P9N\xB8\xA5\x1CV\xF1\x87k\xC1\xE0\x17\xC9z\xCDeg\xF2\x91\x81\x01\x91\x90\x91R_Q` aG\xC3_9_Q\x90_R\x91\x81\x01\x91\x90\x91RF``\x82\x01R0`\x80\x82\x01R`\xA0\x90 `\x06T\x81\x14a\x02\xB7W`\x06\x81\x90U`@\x80Q_Q` aG\xA3_9_Q\x90_R` \x82\x01R\x90\x81\x01\x82\x90Ra\x04\x15\x90``\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90Ra\x05\x03V[`\x06T`@Q_Q` aG\xA3_9_Q\x90_R\x90_\x90\xA2PV[`@\x80Q\x80\x82\x01\x82R`\n\x81Ri$7\xB89&2\xB23\xB2\xB9`\xB1\x1B` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`\x05\x81Rd\x03\"\xE3\x02\xE3`\xDC\x1B\x90\x82\x01R\x81Q_Q` aG\xE3_9_Q\x90_R\x81R\x7Fl\xD6\x81y\x0Cx\xC2 Q{\t\x9As\x7F\x8E\x85\xF6\x9Eyz\xBEN)\x10\xFB\x18\x9Ba\xDBK\xF2\xCD\x91\x81\x01\x91\x90\x91R_Q` aG\xC3_9_Q\x90_R\x91\x81\x01\x91\x90\x91RF``\x82\x01R0`\x80\x82\x01R`\xA0\x90 `\x04T\x81\x14a\x02\xB7W`\x04\x81\x90U`@Q\x81\x90\x7F\xA4?\xAD\x83\x92\x0F\xD0\x94E\x85^\x85Ns\xC9\xC52\xE1t\x02\xC9\xCE\xB0\x99\x93\xA29(C\xA5\xBD\xB9\x90_\x90\xA2PV[`\x01T`\x02T_\x91a\x05!\x91`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a\x060V[B\x11\x15a\x05,WP`\x01[`\x04T`\x02T\x83Q` \x80\x86\x01\x91\x90\x91 `@\x80Q\x80\x84\x01\x95\x90\x95RC`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x90\x85\x01R\x91\x90\x1Bc\xFF\xFF\xFF\xFF\x19\x16`D\x83\x01R``\x82\x01R`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x91\x82\x01 c\xFF\xFF\xFF\xFFB\x16`\x01`\xE0\x1B\x02\x91\x1C\x17`\x02U\x80\x15a\x05\xC6W`\x02T`\x01`\x01`\xE0\x1B\x03\x81\x16`\x01`\xE0\x1B\x91\x82\x90\x04c\xFF\xFF\xFF\xFF\x16\x90\x91\x02\x17`\x03U[PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02\xB7W__\xFD[___``\x84\x86\x03\x12\x15a\x05\xF0W__\xFD[\x83Qa\x05\xFB\x81a\x05\xCAV[` \x85\x01Q\x90\x93Pc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x06\x14W__\xFD[`@\x85\x01Q\x90\x92Pa\x06%\x81a\x05\xCAV[\x80\x91PP\x92P\x92P\x92V[\x80\x82\x01\x80\x82\x11\x15a\x06OWcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x92\x91PPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa@\xDCa\x06\xC7_9_\x81\x81a\x05\x7F\x01Ra%M\x01R_\x81\x81a\x04\x7F\x01R\x81\x81a\x06\x01\x01R\x81\x81a\t\xA3\x01R\x81\x81a\x16\xD0\x01R\x81\x81a\x1CI\x01R\x81\x81a\x1EB\x01Ra#g\x01R_\x81\x81a\x02\xEF\x01Ra\x06p\x01R_\x81\x81a\x03\xD5\x01Ra\x07\xC8\x01Ra@\xDC_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\xFCW_5`\xE0\x1C\x80cz~\xBD{\x11a\x01\x14W\x80c\xBE\x9B\xAB\xDC\x11a\0\xA9W\x80c\xDC\x96\xFDP\x11a\0yW\x80c\xDC\x96\xFDP\x14a\x05\xA1W\x80c\xDD\xAD\x19\x02\x14a\x05\xA9W\x80c\xF6\x98\xDA%\x14a\x05\xDAW\x80c\xFCU0\x9A\x14a\x05\xE3W\x80c\xFF\xA1\xADt\x14a\x05\xA9W__\xFD[\x80c\xBE\x9B\xAB\xDC\x14a\x05\x1EW\x80c\xC9f\xC4\xFE\x14a\x051W\x80c\xD7\xB0\xFE\xF1\x14a\x05:W\x80c\xD9L\xC1\xB4\x14a\x05zW__\xFD[\x80c\xABf\xCC\xAB\x11a\0\xE4W\x80c\xABf\xCC\xAB\x14a\x04\xC1W\x80c\xAC\x96P\xD8\x14a\x04\xD4W\x80c\xB9 \xDE\xED\x14a\x04\xF4W\x80c\xBD\xA6_E\x14a\x05\x0BW__\xFD[\x80cz~\xBD{\x14a\x03\xF7W\x80c|\x8E(\xDA\x14a\x04gW\x80c\x82\xBF\xEF\xC8\x14a\x04zW\x80c\x89\xCC\xFE\x89\x14a\x04\xB9W__\xFD[\x80cT\xA2\xED\xF5\x11a\x01\x95W\x80cm+\xEE\xF1\x11a\x01eW\x80cm+\xEE\xF1\x14a\x03jW\x80crX\x1C\xC0\x14a\x03sW\x80cv+\xD7k\x14a\x03\x9AW\x80cx\xD1I\xA8\x14a\x03\xADW\x80cx\xD8\x01m\x14a\x03\xD0W__\xFD[\x80cT\xA2\xED\xF5\x14a\x03\x1FW\x80c]/\x07\xC5\x14a\x032W\x80ce\x15\x14\xBF\x14a\x03DW\x80ce\xE3\xFAr\x14a\x03WW__\xFD[\x80c#\xCB:\xC0\x11a\x01\xD0W\x80c#\xCB:\xC0\x14a\x02\xA4W\x80c)9.2\x14a\x02\xB7W\x80c-P\xB1\x8B\x14a\x02\xD7W\x80cD\xDA\xE6\xF8\x14a\x02\xEAW__\xFD[\x80b#\xDE)\x14a\x02\0W\x80c\n\xBE\xC5\x8F\x14a\x02\x15W\x80c\r\xF1\x8F\x94\x14a\x02(W\x80c\x1A\x7F\xFEz\x14a\x02\x91W[__\xFD[a\x02\x13a\x02\x0E6`\x04a93V[a\x05\xF6V[\0[a\x02\x13a\x02#6`\x04a9\xFDV[a\x08\xAFV[`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x03Tc\xFF\xFF\xFF\xFF\x19\x81\x83\x1B\x16\x83R`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90\x82\x01R[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x19\x16\x81R` \x92\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x92\x81\x01\x92\x90\x92R\x01[`@Q\x80\x91\x03\x90\xF3[a\x02\x13a\x02\x9F6`\x04a:AV[a\n=V[a\x02\x13a\x02\xB26`\x04a:AV[a\x0B\x05V[a\x02\xBF`\x01\x81V[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x88V[a\x02\x13a\x02\xE56`\x04a:sV[a\x0B\xCAV[a\x03\x11\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01a\x02\x88V[a\x02\x13a\x03-6`\x04a:\xB0V[a\x0C\x94V[a\x02\xBFj\x08E\x95\x16\x14\x01HJ\0\0\0\x81V[a\x02\x13a\x03R6`\x04a:\xB0V[a\rbV[a\x02\x13a\x03e6`\x04a:\xE7V[a\x0E+V[a\x03\x11`\x01T\x81V[a\x03\x11\x7F\xB2\x81\xFC\x8C\x12\x95M\"TM\xB4]\xE3\x15\x9A9'(\x95\xB1i\xA8R\xB3\x14\xF9\xCCv.D\xC5;\x81V[a\x03\x11a\x03\xA86`\x04a;\x1CV[a\x0E\xF5V[a\x03\xC0a\x03\xBB6`\x04a;3V[a\x0F\nV[`@Q\x90\x15\x15\x81R` \x01a\x02\x88V[a\x03\x11\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04Va\x04\x056`\x04a;\x1CV[_` \x81\x90R\x90\x81R`@\x90 T`\x01`\x01``\x1B\x03\x81\x16\x90`\x01``\x1B\x81\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90`\x01`\x90\x1B\x81\x04c\xFF\xFF\xFF\xFF\x16\x90`\x01`\xB0\x1B\x81\x04b\xFF\xFF\xFF\x16\x90`\x01`\xC8\x1B\x90\x04`\xFF\x16\x85V[`@Qa\x02\x88\x95\x94\x93\x92\x91\x90a;kV[a\x02\x13a\x04u6`\x04a:AV[a\x0F\x99V[a\x04\xA1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x88V[a\x02\x13a\x10^V[a\x03\x11a\x04\xCF6`\x04a;\xCCV[a\x11\xA6V[a\x04\xE7a\x04\xE26`\x04a;\xE7V[a\x12\xF4V[`@Qa\x02\x88\x91\x90a<\x86V[B[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x88V[a\x02\x13a\x05\x196`\x04a:\xB0V[a\x13\xDAV[a\x03\x11a\x05,6`\x04a:\xB0V[a\x14\xA3V[a\x03\x11`\x04T\x81V[`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x02Tc\xFF\xFF\xFF\xFF\x19\x81\x83\x1B\x16\x83R`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90\x82\x01Ra\x02dV[a\x04\xF6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\x13a\x14\xE7V[a\x05\xCD`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03\"\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[`@Qa\x02\x88\x91\x90a<\xE9V[a\x03\x11`\x06T\x81V[a\x02\x13a\x05\xF16`\x04a<\xFBV[a\x15\xE0V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x06?W`@QcPy\xFFu`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x86\x160\x14a\x06hW`@Qc\x178\x92!`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x15a\x08\xA5W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x03a\x07\xC6W`\x01`\x01``\x1B\x03\x85\x11\x15a\x06\xBDW`@Qc)<\xEE\xF9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05T`@Qc\x02&^1`\xE6\x1B\x81R\x855``\x90\x81\x1C`\x04\x83\x01\x81\x90R\x92`\x14\x88\x015\x90\x91\x1C\x91_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x89\x97\x8C@\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\x15W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x079\x91\x90a=%V[\x90P\x82`\x01`\x01`\xA0\x1B\x03\x16\x8A`\x01`\x01`\xA0\x1B\x03\x16\x03a\x07\x81W`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x07|W`@Qc\xAC\xD5\xA8#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07\xB3V[\x89`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07\xB3W`@Qc\xAC\xD5\xA8#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07\xBE\x83\x83\x8Aa\x17dV[PPPa\x08\xA5V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x03a\x08\x8CW\x835``\x90\x81\x1C\x90`\x14\x86\x015`\xA0\x90\x81\x1C\x91` \x88\x015\x90\x1C\x90`4\x88\x015\x90\x1C\x88\x15\x80a\x081WPa\x08-`\x01`\x01``\x1B\x03\x80\x83\x16\x90\x85\x16a=TV[\x89\x14\x15[\x15a\x08OW`@Qc\xC5.>\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01``\x1B\x03\x83\x16\x15a\x08iWa\x08i\x84\x83\x85a\x17dV[`\x01`\x01``\x1B\x03\x81\x16\x15a\x08\x83Wa\x08\x83\x82\x85\x83a\x17dV[PPPPa\x08\xA5V[`@Qc\r=\xCD\xE5`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPV[`\x05T\x83\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x08\xDBW`@Qc\x08\xA9D\x19`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05T`@Qc\x02&^1`\xE6\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R3\x92\x16\x90c\x89\x97\x8C@\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t#W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tG\x91\x90a=%V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\tnW`@Qc\xAC\xD5\xA8#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\ty\x84\x84\x84a\x17dV[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`\x01`\x01``\x1B\x03\x83\x16`D\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\t\xF1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x15\x91\x90a=gV[\x15\x15`\x01\x14a\n7W`@Qc\x02.%\x81`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPV[`\x05T`\x01`\xA0\x1B\x90\x04`\xFF\x16a\ngW`@Qc\x08\xA9D\x19`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05T`@Qc\x02&^1`\xE6\x1B\x81R3`\x04\x82\x01R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x89\x97\x8C@\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xADW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xD1\x91\x90a=%V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\n\xF8W`@Qc\xAC\xD5\xA8#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B\x023\x82a\x1BLV[PV[`\x05T`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x0B/W`@Qc\x08\xA9D\x19`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05T`@Qc\x02&^1`\xE6\x1B\x81R3`\x04\x82\x01R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x89\x97\x8C@\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BuW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x99\x91\x90a=%V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0B\xC0W`@Qc\xAC\xD5\xA8#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B\x023\x82a\x1C\xDDV[`\x05T\x83\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x0B\xF6W`@Qc\x08\xA9D\x19`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05T`@Qc\x02&^1`\xE6\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R3\x92\x16\x90c\x89\x97\x8C@\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C>W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Cb\x91\x90a=%V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0C\x89W`@Qc\xAC\xD5\xA8#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\n7\x84\x84\x84a\x1EyV[`\x05T\x82\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x0C\xC0W`@Qc\x08\xA9D\x19`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05T`@Qc\x02&^1`\xE6\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R3\x92\x16\x90c\x89\x97\x8C@\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x08W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r,\x91\x90a=%V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\rSW`@Qc\xAC\xD5\xA8#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\r]\x83\x83a\x1BLV[PPPV[`\x05T\x82\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16a\r\x8EW`@Qc\x08\xA9D\x19`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05T`@Qc\x02&^1`\xE6\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R3\x92\x16\x90c\x89\x97\x8C@\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xD6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xFA\x91\x90a=%V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0E!W`@Qc\xAC\xD5\xA8#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\r]\x83\x83a\x1C\xDDV[`\x05T`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x0EUW`@Qc\x08\xA9D\x19`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05T`@Qc\x02&^1`\xE6\x1B\x81R3`\x04\x82\x01R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x89\x97\x8C@\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\x9BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xBF\x91\x90a=%V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0E\xE6W`@Qc\xAC\xD5\xA8#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0E\xF13\x83\x83a\x1EyV[PPV[_\x81\x81R` \x81\x90R`@\x81 T[\x92\x91PPV[`@\x80Q` \x80\x82\x01\x86\x90R\x835\x92\x82\x01\x92\x90\x92R\x90\x82\x015``\x82\x01R`\xE0\x83\x81\x015`\x80\x83\x01R`\xA0\x80\x85\x015\x90\x83\x01R`\xC0\x80\x85\x015\x90\x83\x01R_\x91\x82\x91\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 `\xC8\x1C\x90Pa\x0Fy`\xA0\x85\x01`\x80\x86\x01a=\x86V[f\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81f\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15\x91PP[\x93\x92PPPV[`\x05T`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x0F\xC3W`@Qc\x08\xA9D\x19`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05T`@Qc\x02&^1`\xE6\x1B\x81R3`\x04\x82\x01R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x89\x97\x8C@\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\tW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10-\x91\x90a=%V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x10TW`@Qc\xAC\xD5\xA8#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B\x023\x82a$\xF2V[`@\x80Q\x80\x82\x01\x82R`\x0C\x81RkHoprChannels`\xA0\x1B` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`\x05\x81Rd\x03\"\xE3\x02\xE3`\xDC\x1B\x90\x82\x01R\x81Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81R\x7F\x84\xE6\x90\x8F46\x01\xD9\xCE\x9F\xB6\r\x82P9N\xB8\xA5\x1CV\xF1\x87k\xC1\xE0\x17\xC9z\xCDeg\xF2\x91\x81\x01\x91\x90\x91R\x7F\xB4\xBC\xB1T\xE3\x86\x01\xC3\x899o\xA9\x181M\xA4-F&\xF1>\xF6\xD0\xCE\xB0~_]&\xB2\xFB\xC3\x91\x81\x01\x91\x90\x91RF``\x82\x01R0`\x80\x82\x01R`\xA0\x90 `\x06T\x81\x14a\x0B\x02W`\x06\x81\x90U`@\x80Q\x7Fw\x1FR@\xAE_\xD8\xA7d\r?\xB8/\xA7\n\xAB/\xB1\xDB\xF3_.\xF4d\xF8P\x99Fqvd\xC5` \x82\x01R\x90\x81\x01\x82\x90Ra\x11x\x90``\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra&*V[`\x06T`@Q\x7Fw\x1FR@\xAE_\xD8\xA7d\r?\xB8/\xA7\n\xAB/\xB1\xDB\xF3_.\xF4d\xF8P\x99Fqvd\xC5\x90_\x90\xA2PV[__a\x11\xB5\x83`\xE0\x015a&\xF0V[\x90P_a\x11\xC8`\xA0\x85\x01`\x80\x86\x01a=\x86V[f\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`8a\x11\xE3`\x80\x87\x01``\x88\x01a=\xACV[b\xFF\xFF\xFF\x16\x90\x1B`Pa\x11\xFC``\x88\x01`@\x89\x01a=\xCEV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x1B`\x80a\x12\x18`@\x89\x01` \x8A\x01a=\xF3V[`\x01`\x01``\x1B\x03\x16\x90\x1B\x17\x17\x17\x90P_a\x12\xA5ce\xE3\xFAr`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x86_\x01_\x015\x84\x86`@Q` \x01a\x12\x81\x93\x92\x91\x90\x92\x83R` \x91\x82\x1Bc\xFF\xFF\xFF\xFF\x19\x16\x91\x83\x01\x91\x90\x91R``\x1B`\x01`\x01``\x1B\x03\x19\x16`<\x82\x01R`P\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 _\x91\x82R` R`@\x90 \x90V[`\x06T`@Q`\x19`\xF8\x1B` \x82\x01R`\x01`\xF8\x1B`!\x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x82\x90R\x90\x91P`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x93PPPP\x91\x90PV[`@\x80Q_\x81R` \x81\x01\x90\x91R``\x90\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\x1EWa\x13\x1Ea>\x1FV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x13QW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x13<W\x90P[P\x91P_[\x83\x81\x10\x15a\x13\xD2Wa\x13\xAD0\x86\x86\x84\x81\x81\x10a\x13tWa\x13ta>3V[\x90P` \x02\x81\x01\x90a\x13\x86\x91\x90a>GV[\x85`@Q` \x01a\x13\x99\x93\x92\x91\x90a>\xA1V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra'\xA9V[\x83\x82\x81Q\x81\x10a\x13\xBFWa\x13\xBFa>3V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x13VV[PP\x92\x91PPV[`\x05T\x82\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x14\x06W`@Qc\x08\xA9D\x19`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05T`@Qc\x02&^1`\xE6\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R3\x92\x16\x90c\x89\x97\x8C@\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14NW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14r\x91\x90a=%V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x14\x99W`@Qc\xAC\xD5\xA8#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\r]\x83\x83a$\xF2V[`@Q`\x01`\x01``\x1B\x03\x19``\x84\x81\x1B\x82\x16` \x84\x01R\x83\x90\x1B\x16`4\x82\x01R_\x90`H\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`@\x80Q\x80\x82\x01\x82R`\n\x81Ri$7\xB89&2\xB23\xB2\xB9`\xB1\x1B` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`\x05\x81Rd\x03\"\xE3\x02\xE3`\xDC\x1B\x90\x82\x01R\x81Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81R\x7Fl\xD6\x81y\x0Cx\xC2 Q{\t\x9As\x7F\x8E\x85\xF6\x9Eyz\xBEN)\x10\xFB\x18\x9Ba\xDBK\xF2\xCD\x91\x81\x01\x91\x90\x91R\x7F\xB4\xBC\xB1T\xE3\x86\x01\xC3\x899o\xA9\x181M\xA4-F&\xF1>\xF6\xD0\xCE\xB0~_]&\xB2\xFB\xC3\x91\x81\x01\x91\x90\x91RF``\x82\x01R0`\x80\x82\x01R`\xA0\x90 `\x04T\x81\x14a\x0B\x02W`\x04\x81\x90U`@Q\x81\x90\x7F\xA4?\xAD\x83\x92\x0F\xD0\x94E\x85^\x85Ns\xC9\xC52\xE1t\x02\xC9\xCE\xB0\x99\x93\xA29(C\xA5\xBD\xB9\x90_\x90\xA2PV[`\x05T`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x16\nW`@Qc\x08\xA9D\x19`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05T`@Qc\x02&^1`\xE6\x1B\x81R3`\x04\x82\x01R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x89\x97\x8C@\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16PW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16t\x91\x90a=%V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x16\x9BW`@Qc\xAC\xD5\xA8#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x16\xA63\x83\x83a\x17dV[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`\x01`\x01``\x1B\x03\x82\x16`D\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x17\x1EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17B\x91\x90a=gV[\x15\x15`\x01\x14a\x0E\xF1W`@Qc\x02.%\x81`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x01`\x01`\x01``\x1B\x03\x82\x16\x10\x15a\x17\x90W`@Qc\xC5.>\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[j\x08E\x95\x16\x14\x01HJ\0\0\0`\x01`\x01``\x1B\x03\x82\x16\x11\x15a\x17\xC5W`@Qc)<\xEE\xF9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83\x83\x80`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x03a\x17\xF9W`@QcK\xD1\xD7i`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x18UW`@Qc\xEA\xC0\xD3\x89`\xE0\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7Fsource must not be empty\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x18\xACW`@Qc\xEA\xC0\xD3\x89`\xE0\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7Fdestination must not be empty\0\0\0`D\x82\x01R`d\x01a\x18LV[_a\x18\xB7\x87\x87a\x14\xA3V[_\x81\x81R` \x81\x90R`@\x90 \x90\x91P`\x02\x81T`\x01`\xC8\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x18\xE6Wa\x18\xE6a;WV[\x03a\x19GW`@QcI\x94c\xC1`\xE0\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7Fcannot fund a channel that will `D\x82\x01Ri1\xB67\xB9\xB2\x909\xB7\xB7\xB7`\xB1\x1B`d\x82\x01R`\x84\x01a\x18LV[\x80Ta\x19]\x90\x87\x90`\x01`\x01``\x1B\x03\x16a>\xC0V[\x81T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x91\x90\x91\x16\x17\x81U_\x81T`\x01`\xC8\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x19\x96Wa\x19\x96a;WV[\x03a\x1A\xC7W\x80Ta\x19\xB4\x90`\x01`\xB0\x1B\x90\x04b\xFF\xFF\xFF\x16`\x01a>\xDFV[\x81Tb\xFF\xFF\xFF\x91\x90\x91\x16`\x01`\xB0\x1B\x02m\xFF\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF``\x1B\x19\x16m\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF``\x1B\x19\x90\x91\x16\x17`\x01`\xC8\x1B\x17\x81U_\x82\x81R` \x81\x90R`@\x90 Ta\x1Ak\x90~L\x01w\xAD\x15\xBB0+D\x19\xE9\xAC\x96\x06]oCn\x0Fa\x97/\x97\xBB\xE2r\x83L@\xF94\x90\x84\x90\x8B\x90\x8B\x90`@\x80Q` \x81\x01\x96\x90\x96R\x85\x01\x93\x90\x93R`\x01`\x01``\x1B\x03\x19``\x92\x83\x1B\x81\x16\x83\x86\x01R\x91\x1B\x16`t\x83\x01R`\x88\x82\x01R`\xA8\x01a\x11dV[\x86`\x01`\x01`\xA0\x1B\x03\x16\x88`\x01`\x01`\xA0\x1B\x03\x16\x83~L\x01w\xAD\x15\xBB0+D\x19\xE9\xAC\x96\x06]oCn\x0Fa\x97/\x97\xBB\xE2r\x83L@\xF94a\x1A\xB5\x86_\x90\x81R` \x81\x90R`@\x90 T\x90V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA4[a\x1B\r_Q` a@g_9_Q\x90_R\x83a\x1A\xEE\x85_\x90\x81R` \x81\x90R`@\x90 T\x90V[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01R`\x80\x01a\x11dV[\x81_Q` a@g_9_Q\x90_Ra\x1B1\x84_\x90\x81R` \x81\x90R`@\x90 T\x90V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA2PPPPPPPPV[_a\x1BW\x82\x84a\x14\xA3V[_\x81\x81R` \x81\x90R`@\x81 \x91\x92P\x81T`\x01`\xC8\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x1B\x84Wa\x1B\x84a;WV[\x03a\x1B\xA2W`@QcI\x94c\xC1`\xE0\x1B\x81R`\x04\x01a\x18L\x90a>\xFAV[\x80T`\x01c\xFF\0\0\x01`\xB0\x1B\x03\x19\x81\x16\x82U_\x83\x81R` \x81\x90R`@\x90 T`\x01`\x01``\x1B\x03\x90\x91\x16\x90a\x1B\xE8\x90_Q` a@\x87_9_Q\x90_R\x90\x85\x90a\x1A\xEEV[\x82_Q` a@\x87_9_Q\x90_Ra\x1C\x0C\x85_\x90\x81R` \x81\x90R`@\x90 T\x90V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA2\x80\x15a\x1C\xD6W`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1C\x90W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xB4\x91\x90a=gV[\x15\x15`\x01\x14a\x1C\xD6W`@Qc\x02.%\x81`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPV[_a\x1C\xE8\x83\x83a\x14\xA3V[_\x81\x81R` \x81\x90R`@\x90 \x90\x91P`\x02\x81T`\x01`\xC8\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x1D\x17Wa\x1D\x17a;WV[\x14a\x1DtW`@QcI\x94c\xC1`\xE0\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7Fchannel state must be PENDING_TO`D\x82\x01Re_CLOSE`\xD0\x1B`d\x82\x01R`\x84\x01a\x18LV[\x80Tc\xFF\xFF\xFF\xFFB\x81\x16`\x01`\x90\x1B\x90\x92\x04\x16\x10a\x1D\xA5W`@Qc8\xB2\x01\x95`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80T`\x01c\xFF\0\0\x01`\xB0\x1B\x03\x19\x81\x16\x82U_\x83\x81R` \x81\x90R`@\x90 T`\x01`\x01``\x1B\x03\x90\x91\x16\x90a\x1D\xEB\x90_Q` a@\x87_9_Q\x90_R\x90\x85\x90a\x1A\xEEV[\x82_Q` a@\x87_9_Q\x90_Ra\x1E\x0F\x85_\x90\x81R` \x81\x90R`@\x90 T\x90V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA2\x80\x15a\x1C\xD6W`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01a\x1CtV[a\x1E\x89`@\x83\x01` \x84\x01a=\xF3V[`\x01`\x01`\x01``\x1B\x03\x82\x16\x10\x15a\x1E\xB4W`@Qc\xC5.>\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[j\x08E\x95\x16\x14\x01HJ\0\0\0`\x01`\x01``\x1B\x03\x82\x16\x11\x15a\x1E\xE9W`@Qc)<\xEE\xF9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`\xE0\x015a\x1E\xF7\x81a(\x1BV[a\x1F\x14W`@Qc:\xE4\xEDk`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x835_\x90\x81R` \x81\x90R`@\x90 `\x01\x81T`\x01`\xC8\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x1FBWa\x1FBa;WV[\x14\x15\x80\x15a\x1FmWP`\x02\x81T`\x01`\xC8\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x1FjWa\x1Fja;WV[\x14\x15[\x15a\x1F\xD5W`@QcI\x94c\xC1`\xE0\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7Fspending channel must be OPEN or`D\x82\x01Rp PENDING_TO_CLOSE`x\x1B`d\x82\x01R`\x84\x01a\x18LV[a\x1F\xE5`\x80\x86\x01``\x87\x01a=\xACV[\x81T`\x01`\xB0\x1B\x90\x04b\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14a EW`@QcI\x94c\xC1`\xE0\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7Fchannel epoch must match\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x18LV[_a V``\x87\x01`@\x88\x01a=\xCEV[\x82T\x90\x91Pe\xFF\xFF\xFF\xFF\xFF\xFF`\x01``\x1B\x90\x91\x04\x81\x16\x90\x82\x16\x10\x15a \x8EW`@Qc1\xDEHG`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a \x9E`@\x87\x01` \x88\x01a=\xF3V[\x82T`\x01`\x01``\x1B\x03\x91\x82\x16\x91\x16\x10\x15a \xCCW`@Qc,Q\xD8\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a \xD6\x87a\x11\xA6V[\x90Pa \xE3\x81\x88\x88a\x0F\nV[a!\0W`@Qc\xEE\x83\\\x89`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`@Q\x80``\x01`@R\x80\x83\x81R` \x01\x8A`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x06T`@Q` \x01a!6\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x90R\x90Pa!ba!\\6\x89\x90\x03\x89\x01\x89a?JV[\x82a(<V[a!\x7FW`@Qc\x12\xBF\xB7\xB7`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a!\x93\x83`\xA0\x8B\x015`\xC0\x8C\x015a*\xBBV[\x90P\x885a!\xA1\x82\x8Ca\x14\xA3V[\x14a!\xBFW`@Qcf\xEE\xA9\xAB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a!\xCA`\x01\x85a?\xE3V[\x85Te\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\x01``\x1B\x02e\xFF\xFF\xFF\xFF\xFF\xFF``\x1B\x19\x90\x91\x16\x17\x85Ua!\xFE`@\x8A\x01` \x8B\x01a=\xF3V[\x85Ta\"\x13\x91\x90`\x01`\x01``\x1B\x03\x16a@\x01V[\x85T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x91\x90\x91\x16\x17\x85U\x885_\x81\x81R` \x81\x90R`@\x90 Ta\"i\x91\x7F\x0CFr\xF1Kc\xBBcT\xFA\xC4u\xEEI\x80U\xA2xDUg:\xF2$q{\x97p\xFDh\xD8\xD1\x91a\x1A\xEEV[\x885_\x81\x81R` \x81\x90R`@\x90 T\x7F\x0CFr\xF1Kc\xBBcT\xFA\xC4u\xEEI\x80U\xA2xDUg:\xF2$q{\x97p\xFDh\xD8\xD1\x90`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA2_a\"\xB7\x8B\x83a\x14\xA3V[_\x81\x81R` \x81\x90R`@\x80\x82 \x8D5\x80\x84R\x91\x90\x92 T\x92\x93P\x90\x91a\"\xFF\x91\x7F\xED\"\xF3M\x15M%:\x7Fo\xD4wC\x9B\xE5\x90\x80\xA1UJ\xA0\xD0d&\x86\xD6B#\xABTL\x8A\x91a\x1A\xEEV[\x8A5_\x81\x81R` \x81\x90R`@\x90 T\x7F\xED\"\xF3M\x15M%:\x7Fo\xD4wC\x9B\xE5\x90\x80\xA1UJ\xA0\xD0d&\x86\xD6B#\xABTL\x8A\x90`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA2_\x81T`\x01`\xC8\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a#`Wa#`a;WV[\x03a$FW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBB3\x8D_\x01` \x01` \x81\x01\x90a#\xAA\x91\x90a=\xF3V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`\x01`\x01``\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a#\xFBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\x1F\x91\x90a=gV[\x15\x15`\x01\x14a$AW`@Qc\x02.%\x81`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a$\xE4V[a$V`@\x8C\x01` \x8D\x01a=\xF3V[\x81Ta$k\x91\x90`\x01`\x01``\x1B\x03\x16a>\xC0V[\x81T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x91\x90\x91\x16\x17\x81U_\x82\x81R` \x81\x90R`@\x90 Ta$\xAE\x90_Q` a@g_9_Q\x90_R\x90\x84\x90a\x1A\xEEV[\x81_Q` a@g_9_Q\x90_Ra$\xD2\x84_\x90\x81R` \x81\x90R`@\x90 T\x90V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA2[PPPPPPPPPPPPV[_a$\xFD\x83\x83a\x14\xA3V[_\x81\x81R` \x81\x90R`@\x81 \x91\x92P\x81T`\x01`\xC8\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a%*Wa%*a;WV[\x03a%HW`@QcI\x94c\xC1`\xE0\x1B\x81R`\x04\x01a\x18L\x90a>\xFAV[a%r\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Ba@ V[\x81Tc\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\x01`\x90\x1B\x02`\xFF`\xC8\x1B\x19\x16g\xFF\0\0\0\xFF\xFF\xFF\xFF`\x90\x1B\x19\x90\x91\x16\x17`\x01`\xC9\x1B\x17\x81U_\x82\x81R` \x81\x90R`@\x90 Ta%\xDD\x90~a\xE8\x03q\x97\xB4\xD9\x1C\xF8\xAD\xD2\xA0sf\x13E\x90\x81\xF26\xD1\xC8\xABXT\x9E\xBB\xC30\xB4\xE3\x90\x84\x90a\x1A\xEEV[\x81~a\xE8\x03q\x97\xB4\xD9\x1C\xF8\xAD\xD2\xA0sf\x13E\x90\x81\xF26\xD1\xC8\xABXT\x9E\xBB\xC30\xB4\xE3a&\x13\x84_\x90\x81R` \x81\x90R`@\x90 T\x90V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA2PPPPV[`\x01T`\x02T_\x91a&H\x91`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a=TV[B\x11\x15a&SWP`\x01[`\x04T`\x02T\x83Q` \x80\x86\x01\x91\x90\x91 `@\x80Q\x80\x84\x01\x95\x90\x95RC`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x90\x85\x01R\x91\x90\x1Bc\xFF\xFF\xFF\xFF\x19\x16`D\x83\x01R``\x82\x01R`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x91\x82\x01 c\xFF\xFF\xFF\xFFB\x16`\x01`\xE0\x1B\x02\x91\x1C\x17`\x02U\x80\x15a\x0E\xF1WPP`\x02T`\x01`\x01`\xE0\x1B\x03\x81\x16`\x01`\xE0\x1B\x91\x82\x90\x04c\xFF\xFF\xFF\xFF\x16\x90\x91\x02\x17`\x03UV[_`\x01\x81`\x1B\x7Fy\xBEf~\xF9\xDC\xBB\xACU\xA0b\x95\xCE\x87\x0B\x07\x02\x9B\xFC\xDB-\xCE(\xD9Y\xF2\x81[\x16\xF8\x17\x98p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19\x7Fy\xBEf~\xF9\xDC\xBB\xACU\xA0b\x95\xCE\x87\x0B\x07\x02\x9B\xFC\xDB-\xCE(\xD9Y\xF2\x81[\x16\xF8\x17\x98\x87\t`@\x80Q_\x81R` \x81\x01\x80\x83R\x95\x90\x95R`\xFF\x90\x93\x16\x92\x84\x01\x92\x90\x92R``\x83\x01R`\x80\x82\x01R`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a'\x98W=__>=_\xFD[PP`@Q`\x1F\x19\x01Q\x93\x92PPPV[``__\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa'\xC5\x91\x90a@<V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a'\xFDW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a(\x02V[``\x91P[P\x91P\x91Pa(\x12\x85\x83\x83a*\xE5V[\x95\x94PPPPPV[_\x81\x15\x80a\x0F\x04WPPp\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19\x11\x90V[_d\x01\0\0\x03\xD0\x19\x83``\x01Q\x10\x15\x80a(`WPd\x01\0\0\x03\xD0\x19\x83`@\x01Q\x10\x15[\x15a(~W`@Qc:\xE4\xEDk`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a(\x8F\x83_\x01Q\x84` \x01Qa+AV[a(\xACW`@Qc9\"\xA5A`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__a(\xFC\x84` \x01Q\x85_\x01Q`@Q` \x01a(\xE3\x92\x91\x90``\x92\x90\x92\x1B`\x01`\x01``\x1B\x03\x19\x16\x82R`\x14\x82\x01R`4\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85`@\x01Qa+kV[\x91P\x91P_a)\x10\x86`@\x01Q\x84\x84a+\xECV[\x90Pa)K\x86`\x80\x01Q\x87`\xA0\x01Q`@\x80Q` \x80\x82\x01\x94\x90\x94R\x80\x82\x01\x92\x90\x92R\x80Q\x80\x83\x03\x82\x01\x81R``\x90\x92\x01\x90R\x80Q\x91\x01 \x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a)|W`@Qc\x1D\xBF\xB9\xB3`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a)\x93\x87``\x01Q\x88_\x01Q\x89` \x01Qa+\xECV[\x90Pa)\xCE\x87`\xC0\x01Q\x88`\xE0\x01Q`@\x80Q` \x80\x82\x01\x94\x90\x94R\x80\x82\x01\x92\x90\x92R\x80Q\x80\x83\x03\x82\x01\x81R``\x90\x92\x01\x90R\x80Q\x91\x01 \x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a)\xFFW`@Qc\x1D\xBF\xB9\xB3`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__a*/\x89`\x80\x01Q\x8A`\xA0\x01Q\x8B`\xC0\x01Q\x8C`\xE0\x01Qd\x01\0\0\x03\xD0\x19a*)\x91\x90a>\x0CV[_a,\x85V[` \x80\x8B\x01Q\x8CQ\x8D\x83\x01Q\x8DQ`@Q\x96\x98P\x94\x96P_\x95a*\xA6\x95a*\x8D\x95\x8A\x92\x8A\x92\x91\x01``\x96\x90\x96\x1B`\x01`\x01``\x1B\x03\x19\x16\x86R`\x14\x86\x01\x94\x90\x94R`4\x85\x01\x92\x90\x92R`T\x84\x01R`t\x83\x01R`\x94\x82\x01R`\xB4\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x8A`@\x01Qa.\x07V[``\x8B\x01Q\x14\x97PPPPPPPP\x92\x91PPV[____a*\xCA\x87\x87\x87a.tV[\x92P\x92P\x92Pa*\xDA\x82\x82a.\xA6V[P\x90\x95\x94PPPPPV[``\x82a*\xFAWa*\xF5\x82a/^V[a\x0F\x92V[\x81Q\x15\x80\x15a+\x11WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a+:W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x18LV[P\x80a\x0F\x92V[_d\x01\0\0\x03\xD0\x19\x80\x84d\x01\0\0\x03\xD0\x19\x86\x87\t\t`\x07\x08d\x01\0\0\x03\xD0\x19\x83\x84\t\x14\x93\x92PPPV[____a+y\x86\x86a/\x86V[\x91P\x91P__a+\x88\x84a0;V[\x91P\x91P__a+\x97\x85a0;V[\x91P\x91P__a+\xCA\x86\x86\x86\x86\x7F?\x871\xAB\xDDf\x1A\xDC\xA0\x8AUX\xF0\xF5\xD2r\xE9S\xD3c\xCBo\x0E]@TG\xC0\x1ADE3a,\x85V[\x91P\x91Pa+\xD8\x82\x82a2\xF5V[\x99P\x99PPPPPPPPP[\x92P\x92\x90PV[_\x80a+\xF9`\x02\x84a@GV[_\x03a,\x07WP`\x1Ba,\x0BV[P`\x1C[`\x01_\x82\x86p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19\x88\x8A\t`@\x80Q_\x81R` \x81\x01\x80\x83R\x95\x90\x95R`\xFF\x90\x93\x16\x92\x84\x01\x92\x90\x92R``\x83\x01R`\x80\x82\x01R`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a,qW=__>=_\xFD[PP`@Q`\x1F\x19\x01Q\x96\x95PPPPPPV[__\x83\x86\x14\x19\x85\x88\x14\x16\x15a,\x98W__\xFD[__\x85\x88\x14\x87\x8A\x14\x16`\x01\x81\x14a,\xB4W\x80\x15a-1Wa-\xAAV[d\x01\0\0\x03\xD0\x19\x89`\x02\t\x91P`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x82``\x82\x01Rd\x01\0\0\x03\xD2\x19`\x80\x82\x01Rd\x01\0\0\x03\xD0\x19`\xA0\x82\x01R` \x81`\xC0\x83`\x05_\x19\xFAa-\x06W__\xFD[d\x01\0\0\x03\xD0\x19\x87d\x01\0\0\x03\xD0\x19\x80\x8E\x8F\t`\x03\t\x08d\x01\0\0\x03\xD0\x19\x82Q\x82\t\x94PPPa-\xAAV[d\x01\0\0\x03\xD0\x19\x8Ad\x01\0\0\x03\xD0\x19\x03\x89\x08\x91P`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x82``\x82\x01Rd\x01\0\0\x03\xD2\x19`\x80\x82\x01Rd\x01\0\0\x03\xD0\x19`\xA0\x82\x01R` \x81`\xC0\x83`\x05_\x19\xFAa-\x8AW__\xFD[d\x01\0\0\x03\xD0\x19\x81Qd\x01\0\0\x03\xD0\x19\x8Cd\x01\0\0\x03\xD0\x19\x03\x8B\x08\t\x93PP[PPd\x01\0\0\x03\xD0\x19\x80\x89d\x01\0\0\x03\xD0\x19\x03\x88d\x01\0\0\x03\xD0\x19\x03\x08d\x01\0\0\x03\xD0\x19\x83\x84\t\x08\x92Pd\x01\0\0\x03\xD0\x19\x87d\x01\0\0\x03\xD0\x19\x03d\x01\0\0\x03\xD0\x19\x80\x86d\x01\0\0\x03\xD0\x19\x03\x8C\x08\x84\t\x08\x91PP\x95P\x95\x93PPPPV[___a.\x14\x85\x85a5\xDDV[\x91P\x91P`@Q`0\x81R` \x80\x82\x01R` `@\x82\x01R\x82``\x82\x01R\x81`\x80\x82\x01R`\x01`\x90\x82\x01Rp\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19`\xB0\x82\x01R` \x81`\xD0\x83`\x05_\x19\xFAa.jW__\xFD[Q\x95\x94PPPPPV[_\x80\x80`\x01`\x01`\xFF\x1B\x03\x84\x16`\x1B`\xFF\x86\x90\x1C\x01a.\x95\x88\x82\x89\x85a6\xF0V[\x94P\x94P\x94PPP\x93P\x93P\x93\x90PV[_\x82`\x03\x81\x11\x15a.\xB9Wa.\xB9a;WV[\x03a.\xC2WPPV[`\x01\x82`\x03\x81\x11\x15a.\xD6Wa.\xD6a;WV[\x03a.\xF4W`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a/\x08Wa/\x08a;WV[\x03a/)W`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x18LV[`\x03\x82`\x03\x81\x11\x15a/=Wa/=a;WV[\x03a\x0E\xF1W`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x18LV[\x80Q\x15a/mW\x80Q` \x82\x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_____a/\x95\x87\x87a7\xB8V[\x92P\x92P\x92P`@Q`0\x81R` \x80\x82\x01R` `@\x82\x01R\x83``\x82\x01R\x82`\x80\x82\x01R`\x01`\x90\x82\x01Rd\x01\0\0\x03\xD0\x19`\xB0\x82\x01R` \x81`\xD0\x83`\x05_\x19\xFAa/\xE1W__\xFD[\x80Q\x95PP`@Q`0\x81R` \x80\x82\x01R\x82`P\x82\x01R` `@\x82\x01R\x81`p\x82\x01R`\x01`\x90\x82\x01Rd\x01\0\0\x03\xD0\x19`\xB0\x82\x01R` \x81`\xD0\x83`\x05_\x19\xFAa0,W__\xFD[\x80Q\x94PPPPP\x92P\x92\x90PV[__d\x01\0\0\x03\xD0\x19\x83\x84\td\x01\0\0\x03\xD0\x19\x81d\x01\0\0\x03\xDB\x19\t\x90Pd\x01\0\0\x03\xD0\x19\x81\x82\td\x01\0\0\x03\xD0\x19\x82\x82\x08\x90Pd\x01\0\0\x03\xD0\x19`\x01\x82\x08d\x01\0\0\x03\xD0\x19a\x06\xEB\x82\t\x90P_\x82\x15`\x01\x81\x14a0\x9EW\x80\x15a0\xACWa0\xB8V[d\x01\0\0\x03\xDB\x19\x91Pa0\xB8V[\x83d\x01\0\0\x03\xD0\x19\x03\x91P[Pd\x01\0\0\x03\xD0\x19\x81\x7F?\x871\xAB\xDDf\x1A\xDC\xA0\x8AUX\xF0\xF5\xD2r\xE9S\xD3c\xCBo\x0E]@TG\xC0\x1ADE3\t\x90Pd\x01\0\0\x03\xD0\x19\x82\x83\t\x92Pd\x01\0\0\x03\xD0\x19\x81\x82\td\x01\0\0\x03\xD0\x19\x81\x7F?\x871\xAB\xDDf\x1A\xDC\xA0\x8AUX\xF0\xF5\xD2r\xE9S\xD3c\xCBo\x0E]@TG\xC0\x1ADE3\td\x01\0\0\x03\xD0\x19\x81\x86\x08\x94Pd\x01\0\0\x03\xD0\x19\x84\x86\t\x94Pd\x01\0\0\x03\xD0\x19\x83\x83\t\x91Pd\x01\0\0\x03\xD0\x19\x82a\x06\xEB\t\x90Pd\x01\0\0\x03\xD0\x19\x81\x86\x08\x94PPd\x01\0\0\x03\xD0\x19\x83\x86\t\x96P__d\x01\0\0\x03\xD0\x19\x83\x84\td\x01\0\0\x03\xD0\x19\x84\x88\td\x01\0\0\x03\xD0\x19\x81\x83\t\x91P`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x82``\x82\x01Rc@\0\0\xF5`\x01`\xFE\x1B\x03`\x80\x82\x01Rd\x01\0\0\x03\xD0\x19`\xA0\x82\x01R` \x81`\xC0\x83`\x05_\x19\xFAa1\xDBW__\xFD[d\x01\0\0\x03\xD0\x19\x82\x82Q\t\x92PPPd\x01\0\0\x03\xD0\x19\x7F1\xFD\xF3\x02r@\x13\xE5z\xD1?\xB3\x8F\x84*\xFE\xEC\x18O\0\xA7G\x89\xDD(g)\xC80<JY\x82\td\x01\0\0\x03\xD0\x19\x82\x83\td\x01\0\0\x03\xD0\x19\x86\x82\t\x90P\x88\x81\x14`\x01\x81\x14a2@W\x80\x15a2LWa2SV[`\x01\x94P\x83\x95Pa2SV[_\x94P\x82\x95P[PPPPd\x01\0\0\x03\xD0\x19\x8A\x88\t\x97Pd\x01\0\0\x03\xD0\x19\x82\x89\t\x97P\x80\x15a2|W\x84\x98P\x81\x97P[PPP`\x02\x85\x06`\x02\x88\x06\x14a2\x98W\x84d\x01\0\0\x03\xD0\x19\x03\x94P[`@Q\x93P` \x84R` \x80\x85\x01R` `@\x85\x01R\x80``\x85\x01RPPPd\x01\0\0\x03\xD2\x19`\x80\x82\x01Rd\x01\0\0\x03\xD0\x19`\xA0\x82\x01R` \x81`\xC0\x83`\x05_\x19\xFAa2\xE2W__\xFD[d\x01\0\0\x03\xD0\x19\x81Q\x84\t\x92PP\x91P\x91V[__d\x01\0\0\x03\xD0\x19\x84\x85\td\x01\0\0\x03\xD0\x19\x81\x86\td\x01\0\0\x03\xD0\x19\x80\x7F\x8E8\xE3\x8E8\xE3\x8E8\xE3\x8E8\xE3\x8E8\xE3\x8E8\xE3\x8E8\xE3\x8E8\xE3\x8E8\xE3\x8D\xAA\xAA\xA8\xC7d\x01\0\0\x03\xD0\x19\x89\x7F\x07\xD3\xD4\xC8\x0B\xC3!\xD5\xB9\xF3\x15\xCE\xA7\xFDD\xC5\xD5\x95\xD2\xFC\x0B\xF6;\x92\xDF\xFF\x10D\xF1|e\x81\t\x08d\x01\0\0\x03\xD0\x19\x80\x85\x7FSL2\x8D#\xF24\xE6\xE2\xA4\x13\xDE\xCA%\xCA\xEC\xE4PaD\x03|@1N\xCB\xD0\xB5=\x9D\xD2b\td\x01\0\0\x03\xD0\x19\x85\x7F\x8E8\xE3\x8E8\xE3\x8E8\xE3\x8E8\xE3\x8E8\xE3\x8E8\xE3\x8E8\xE3\x8E8\xE3\x8E8\xE3\x8D\xAA\xAA\xA8\x8C\t\x08\x08d\x01\0\0\x03\xD0\x19\x7F\xD3Wq\x19=\x94\x91\x8A\x9C\xA3L\xCB\xB7\xB6@\xDD\x86\xCD@\x95B\xF8H}\x9F\xE6\xB7Ex\x1E\xB4\x9Bd\x01\0\0\x03\xD0\x19\x80\x8A\x7F\xED\xAD\xC6\xF6C\x83\xDC\x1D\xF7\xC4\xB2\xD5\x1BT\"T\x06\xD3kd\x1F^A\xBB\xC5*Va*\x8Cm\x14\t\x86\x08\x08`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x81``\x82\x01Rd\x01\0\0\x03\xD2\x19`\x80\x82\x01Rd\x01\0\0\x03\xD0\x19`\xA0\x82\x01R` \x81`\xC0\x83`\x05_\x19\xFAa4QW__\xFD[\x80Q\x91Pd\x01\0\0\x03\xD0\x19\x82\x84\t\x96Pd\x01\0\0\x03\xD0\x19\x80\x7FK\xDA\x12\xF6\x84\xBD\xA1/hK\xDA\x12\xF6\x84\xBD\xA1/hK\xDA\x12\xF6\x84\xBD\xA1/hK\x8E8\xE2<d\x01\0\0\x03\xD0\x19\x8C\x7F\xC7^\x0C2\xD5\xCB|\x0F\xA9\xD0\xA5K\x12\xA0\xA6\xD5dz\xB0F\xD6\x86\xDAo\xDF\xFC\x90\xFC \x1Dq\xA3\t\x08d\x01\0\0\x03\xD0\x19\x80\x88\x7F)\xA6\x19F\x91\xF9\x1AsqR\t\xEFe\x12\xE5vr(0\xA2\x01\xBE \x18\xA7e\xE8Z\x9E\xCE\xE91\td\x01\0\0\x03\xD0\x19\x88\x7F/hK\xDA\x12\xF6\x84\xBD\xA1/hK\xDA\x12\xF6\x84\xBD\xA1/hK\xDA\x12\xF6\x84\xBD\xA1/8\xE3\x8D\x84\t\x08\x08\x92Pd\x01\0\0\x03\xD0\x19\x80d\x01\0\0\x06\xC4\x19d\x01\0\0\x03\xD0\x19\x8C\x7Fz\x06SK\xB8\xBD\xB4\x9F\xD5\xE9\xE6c'\"\xC2\x98\x94g\xC1\xBF\xC8\xE8\xD9x\xDF\xB4%\xD2h\\%s\t\x08d\x01\0\0\x03\xD0\x19\x80\x88\x7Fd\x84\xAAqeE\xCA,\xF3\xA7\x0C?\xA8\xFE3~\n=!\x16/\rb\x99\xA7\xBF\x81\x92\xBF\xD2\xA7o\t\x87\x08\x08\x94P`@Q\x90P` \x81R` \x80\x82\x01R` `@\x82\x01R\x84``\x82\x01Rd\x01\0\0\x03\xD2\x19`\x80\x82\x01Rd\x01\0\0\x03\xD0\x19`\xA0\x82\x01R` \x81`\xC0\x83`\x05_\x19\xFAa5\xBFW__\xFD[Q\x93Pd\x01\0\0\x03\xD0\x19\x90P\x83\x81\x83\x89\t\t\x93PPPP\x92P\x92\x90PV[__`\xFF\x83Q\x11\x15a5\xEDW__\xFD[_`@Q_[`\x88\x81\x10\x15a6\tW_\x82\x82\x01R` \x01a5\xF3V[P`\x88` _[\x88Q\x81\x10\x15a61W\x88\x82\x01Q\x84\x84\x01R` \x92\x83\x01\x92\x91\x82\x01\x91\x01a6\x10V[PP`\x89\x87Q\x01\x90P`0\x81\x83\x01S`\x02\x01` _[\x87Q\x81\x10\x15a6hW\x87\x82\x01Q\x84\x84\x01R` \x92\x83\x01\x92\x91\x82\x01\x91\x01a6GV[PP`\x8B\x86Q\x88Q\x01\x01\x90P\x85Q\x81\x83\x01SP\x85Q\x85Q\x01`\x8C\x01\x81 \x91PP`@Q\x81\x81R`\x01` \x82\x01S`!` _[\x87Q\x81\x10\x15a6\xBCW\x87\x82\x01Q\x84\x84\x01R` \x92\x83\x01\x92\x91\x82\x01\x91\x01a6\x9BV[PPP\x84Q\x85Q`!\x01\x82\x01S\x84Q`\"\x01\x81 \x93P\x83\x82\x18\x81R`\x02` \x82\x01S\x84Q`\"\x01\x81 \x92PPP\x92P\x92\x90PV[_\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a7)WP_\x91P`\x03\x90P\x82a7\xAEV[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a7zW=__>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a7\xA5WP_\x92P`\x01\x91P\x82\x90Pa7\xAEV[\x92P_\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[___`\xFF\x84Q\x11\x15a7\xC9W__\xFD[_`@Q_[`\x88\x81\x10\x15a7\xE5W_\x82\x82\x01R` \x01a7\xCFV[P`\x88` _[\x89Q\x81\x10\x15a8\rW\x89\x82\x01Q\x84\x84\x01R` \x92\x83\x01\x92\x91\x82\x01\x91\x01a7\xECV[PP`\x89\x88Q\x01\x90P``\x81\x83\x01S`\x02\x01` _[\x88Q\x81\x10\x15a8DW\x88\x82\x01Q\x84\x84\x01R` \x92\x83\x01\x92\x91\x82\x01\x91\x01a8#V[PP`\x8B\x87Q\x89Q\x01\x01\x90P\x86Q\x81\x83\x01SP\x86Q\x86Q\x01`\x8C\x01\x81 \x91PP`@Q\x81\x81R`\x01` \x82\x01S`!` _[\x88Q\x81\x10\x15a8\x98W\x88\x82\x01Q\x84\x84\x01R` \x92\x83\x01\x92\x91\x82\x01\x91\x01a8wV[PPP\x85Q\x86Q`!\x01\x82\x01S\x85Q`\"\x01\x81 \x94P\x84\x82\x18\x81R`\x02` \x82\x01S\x85Q`\"\x01\x81 \x93P\x83\x82\x18\x81R`\x03` \x82\x01S\x85Q`\"\x01\x81 \x92PPP\x92P\x92P\x92V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0B\x02W__\xFD[__\x83`\x1F\x84\x01\x12a9\x05W__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a9\x1CW__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a+\xE5W__\xFD[________`\xC0\x89\x8B\x03\x12\x15a9JW__\xFD[\x885a9U\x81a8\xE1V[\x97P` \x89\x015a9e\x81a8\xE1V[\x96P`@\x89\x015a9u\x81a8\xE1V[\x95P``\x89\x015\x94P`\x80\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a9\x97W__\xFD[a9\xA3\x8B\x82\x8C\x01a8\xF5V[\x90\x95P\x93PP`\xA0\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a9\xC2W__\xFD[a9\xCE\x8B\x82\x8C\x01a8\xF5V[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a9\xF8W__\xFD[\x91\x90PV[___``\x84\x86\x03\x12\x15a:\x0FW__\xFD[\x835a:\x1A\x81a8\xE1V[\x92P` \x84\x015a:*\x81a8\xE1V[\x91Pa:8`@\x85\x01a9\xE2V[\x90P\x92P\x92P\x92V[_` \x82\x84\x03\x12\x15a:QW__\xFD[\x815a\x0F\x92\x81a8\xE1V[_a\x01\0\x82\x84\x03\x12\x15a:mW__\xFD[P\x91\x90PV[___a\x02 \x84\x86\x03\x12\x15a:\x86W__\xFD[\x835a:\x91\x81a8\xE1V[\x92Pa:\xA0\x85` \x86\x01a:\\V[\x91Pa:8\x85a\x01 \x86\x01a:\\V[__`@\x83\x85\x03\x12\x15a:\xC1W__\xFD[\x825a:\xCC\x81a8\xE1V[\x91P` \x83\x015a:\xDC\x81a8\xE1V[\x80\x91PP\x92P\x92\x90PV[__a\x02\0\x83\x85\x03\x12\x15a:\xF9W__\xFD[a;\x03\x84\x84a:\\V[\x91Pa;\x13\x84a\x01\0\x85\x01a:\\V[\x90P\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a;,W__\xFD[P5\x91\x90PV[___a\x02 \x84\x86\x03\x12\x15a;FW__\xFD[\x835\x92Pa:\xA0\x85` \x86\x01a:\\V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x01`\x01``\x1B\x03\x86\x16\x81Re\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16` \x82\x01Rc\xFF\xFF\xFF\xFF\x84\x16`@\x82\x01Rb\xFF\xFF\xFF\x83\x16``\x82\x01R`\xA0\x81\x01`\x03\x83\x10a;\xBCWcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x82`\x80\x83\x01R\x96\x95PPPPPPV[_a\x01\0\x82\x84\x03\x12\x15a;\xDDW__\xFD[a\x0F\x92\x83\x83a:\\V[__` \x83\x85\x03\x12\x15a;\xF8W__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a<\x0EW__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a<\x1EW__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a<4W__\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a<HW__\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a<\xDDW`?\x19\x87\x86\x03\x01\x84Ra<\xC8\x85\x83Qa<XV[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a<\xACV[P\x92\x96\x95PPPPPPV[` \x81R_a\x0F\x92` \x83\x01\x84a<XV[__`@\x83\x85\x03\x12\x15a=\x0CW__\xFD[\x825a=\x17\x81a8\xE1V[\x91Pa;\x13` \x84\x01a9\xE2V[_` \x82\x84\x03\x12\x15a=5W__\xFD[\x81Qa\x0F\x92\x81a8\xE1V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0F\x04Wa\x0F\x04a=@V[_` \x82\x84\x03\x12\x15a=wW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x0F\x92W__\xFD[_` \x82\x84\x03\x12\x15a=\x96W__\xFD[\x815f\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0F\x92W__\xFD[_` \x82\x84\x03\x12\x15a=\xBCW__\xFD[\x815b\xFF\xFF\xFF\x81\x16\x81\x14a\x0F\x92W__\xFD[_` \x82\x84\x03\x12\x15a=\xDEW__\xFD[\x815e\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0F\x92W__\xFD[_` \x82\x84\x03\x12\x15a>\x03W__\xFD[a\x0F\x92\x82a9\xE2V[\x81\x81\x03\x81\x81\x11\x15a\x0F\x04Wa\x0F\x04a=@V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[__\x835`\x1E\x19\x846\x03\x01\x81\x12a>\\W__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a>vW__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a+\xE5W__\xFD[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[\x82\x84\x827_\x83\x82\x01_\x81Ra>\xB6\x81\x85a>\x8AV[\x96\x95PPPPPPV[`\x01`\x01``\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0F\x04Wa\x0F\x04a=@V[b\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0F\x04Wa\x0F\x04a=@V[` \x80\x82R`0\x90\x82\x01R\x7Fchannel must have state OPEN or `@\x82\x01RoPENDING_TO_CLOSE`\x80\x1B``\x82\x01R`\x80\x01\x90V[_a\x01\0\x82\x84\x03\x12\x80\x15a?\\W__\xFD[P`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a?\x8DWcNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x90\x81R\x835\x82R` \x80\x85\x015\x90\x83\x01R\x83\x81\x015\x90\x82\x01R``\x80\x84\x015\x90\x82\x01R`\x80\x80\x84\x015\x90\x82\x01R`\xA0\x80\x84\x015\x90\x82\x01R`\xC0\x80\x84\x015\x90\x82\x01R`\xE0\x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[e\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0F\x04Wa\x0F\x04a=@V[`\x01`\x01``\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x0F\x04Wa\x0F\x04a=@V[c\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0F\x04Wa\x0F\x04a=@V[_a\x0F\x92\x82\x84a>\x8AV[_\x82a@aWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x06\x90V\xFE\xEC!T|\xA1\xE2.\xDC?\x9BON\r\xA9F8\xB5\xB9M\xCB\x18\xD5-\xCC\x07*\xBEX\x01\xA8\x01<\x19{\xBAhJj\xFB{\xA2O\x1D&V\x05AK\x1D\0Q\xA6\xE2\x95\xD6\xFF{nx\xE8p\xD7\xA7\xF0\xA2dipfsX\"\x12 [\x87\xF2\x9E\x9FZ\x1E\xFF\x81L\x1C\"t\xC2v\xEE\x1D-V\xF5A\xE2\xF2\x17\xD5l/i\xE2\x03\x0EudsolcC\0\x08\x1E\x003w\x1FR@\xAE_\xD8\xA7d\r?\xB8/\xA7\n\xAB/\xB1\xDB\xF3_.\xF4d\xF8P\x99Fqvd\xC5\xB4\xBC\xB1T\xE3\x86\x01\xC3\x899o\xA9\x181M\xA4-F&\xF1>\xF6\xD0\xCE\xB0~_]&\xB2\xFB\xC3\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b50600436106101fc575f3560e01c80637a7ebd7b11610114578063be9babdc116100a9578063dc96fd5011610079578063dc96fd50146105a1578063ddad1902146105a9578063f698da25146105da578063fc55309a146105e3578063ffa1ad74146105a9575f5ffd5b8063be9babdc1461051e578063c966c4fe14610531578063d7b0fef11461053a578063d94cc1b41461057a575f5ffd5b8063ab66ccab116100e4578063ab66ccab146104c1578063ac9650d8146104d4578063b920deed146104f4578063bda65f451461050b575f5ffd5b80637a7ebd7b146103f75780637c8e28da1461046757806382bfefc81461047a57806389ccfe89146104b9575f5ffd5b806354a2edf5116101955780636d2beef1116101655780636d2beef11461036a57806372581cc014610373578063762bd76b1461039a57806378d149a8146103ad57806378d8016d146103d0575f5ffd5b806354a2edf51461031f5780635d2f07c514610332578063651514bf1461034457806365e3fa7214610357575f5ffd5b806323cb3ac0116101d057806323cb3ac0146102a457806329392e32146102b75780632d50b18b146102d757806344dae6f8146102ea575f5ffd5b806223de29146102005780630abec58f146102155780630df18f94146102285780631a7ffe7a14610291575b5f5ffd5b61021361020e366004613933565b6105f6565b005b6102136102233660046139fd565b6108af565b6040805180820182525f808252602091820152815180830190925260035463ffffffff1981831b168352600160e01b900463ffffffff16908201525b60408051825163ffffffff1916815260209283015163ffffffff1692810192909252015b60405180910390f35b61021361029f366004613a41565b610a3d565b6102136102b2366004613a41565b610b05565b6102bf600181565b6040516001600160601b039091168152602001610288565b6102136102e5366004613a73565b610bca565b6103117f000000000000000000000000000000000000000000000000000000000000000081565b604051908152602001610288565b61021361032d366004613ab0565b610c94565b6102bf6a084595161401484a00000081565b610213610352366004613ab0565b610d62565b610213610365366004613ae7565b610e2b565b61031160015481565b6103117fb281fc8c12954d22544db45de3159a39272895b169a852b314f9cc762e44c53b81565b6103116103a8366004613b1c565b610ef5565b6103c06103bb366004613b33565b610f0a565b6040519015158152602001610288565b6103117f000000000000000000000000000000000000000000000000000000000000000081565b610456610405366004613b1c565b5f602081905290815260409020546001600160601b03811690600160601b810465ffffffffffff1690600160901b810463ffffffff1690600160b01b810462ffffff1690600160c81b900460ff1685565b604051610288959493929190613b6b565b610213610475366004613a41565b610f99565b6104a17f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610288565b61021361105e565b6103116104cf366004613bcc565b6111a6565b6104e76104e2366004613be7565b6112f4565b6040516102889190613c86565b425b60405163ffffffff9091168152602001610288565b610213610519366004613ab0565b6113da565b61031161052c366004613ab0565b6114a3565b61031160045481565b6040805180820182525f808252602091820152815180830190925260025463ffffffff1981831b168352600160e01b900463ffffffff1690820152610264565b6104f67f000000000000000000000000000000000000000000000000000000000000000081565b6102136114e7565b6105cd604051806040016040528060058152602001640322e302e360dc1b81525081565b6040516102889190613ce9565b61031160065481565b6102136105f1366004613cfb565b6115e0565b336001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161461063f57604051635079ff7560e11b815260040160405180910390fd5b6001600160a01b038616301461066857604051631738922160e31b815260040160405180910390fd5b82156108a5577f000000000000000000000000000000000000000000000000000000000000000083036107c6576001600160601b038511156106bd5760405163293ceef960e21b815260040160405180910390fd5b6005546040516302265e3160e61b81528535606090811c6004830181905292601488013590911c915f916001600160a01b0316906389978c4090602401602060405180830381865afa158015610715573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107399190613d25565b9050826001600160a01b03168a6001600160a01b031603610781576001600160a01b0381161561077c5760405163acd5a82360e01b815260040160405180910390fd5b6107b3565b896001600160a01b0316816001600160a01b0316146107b35760405163acd5a82360e01b815260040160405180910390fd5b6107be83838a611764565b5050506108a5565b7f0000000000000000000000000000000000000000000000000000000000000000830361088c578335606090811c90601486013560a090811c916020880135901c906034880135901c881580610831575061082d6001600160601b03808316908516613d54565b8914155b1561084f5760405163c52e3eff60e01b815260040160405180910390fd5b6001600160601b0383161561086957610869848385611764565b6001600160601b0381161561088357610883828583611764565b505050506108a5565b604051630d3dcde560e31b815260040160405180910390fd5b5050505050505050565b6005548390600160a01b900460ff166108db576040516308a9441960e31b815260040160405180910390fd5b6005546040516302265e3160e61b81526001600160a01b038381166004830152339216906389978c4090602401602060405180830381865afa158015610923573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109479190613d25565b6001600160a01b03161461096e5760405163acd5a82360e01b815260040160405180910390fd5b610979848484611764565b6040516323b872dd60e01b81523360048201523060248201526001600160601b03831660448201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906323b872dd906064016020604051808303815f875af11580156109f1573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a159190613d67565b1515600114610a375760405163022e258160e11b815260040160405180910390fd5b50505050565b600554600160a01b900460ff16610a67576040516308a9441960e31b815260040160405180910390fd5b6005546040516302265e3160e61b81523360048201525f916001600160a01b0316906389978c4090602401602060405180830381865afa158015610aad573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ad19190613d25565b6001600160a01b031614610af85760405163acd5a82360e01b815260040160405180910390fd5b610b023382611b4c565b50565b600554600160a01b900460ff16610b2f576040516308a9441960e31b815260040160405180910390fd5b6005546040516302265e3160e61b81523360048201525f916001600160a01b0316906389978c4090602401602060405180830381865afa158015610b75573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b999190613d25565b6001600160a01b031614610bc05760405163acd5a82360e01b815260040160405180910390fd5b610b023382611cdd565b6005548390600160a01b900460ff16610bf6576040516308a9441960e31b815260040160405180910390fd5b6005546040516302265e3160e61b81526001600160a01b038381166004830152339216906389978c4090602401602060405180830381865afa158015610c3e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c629190613d25565b6001600160a01b031614610c895760405163acd5a82360e01b815260040160405180910390fd5b610a37848484611e79565b6005548290600160a01b900460ff16610cc0576040516308a9441960e31b815260040160405180910390fd5b6005546040516302265e3160e61b81526001600160a01b038381166004830152339216906389978c4090602401602060405180830381865afa158015610d08573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d2c9190613d25565b6001600160a01b031614610d535760405163acd5a82360e01b815260040160405180910390fd5b610d5d8383611b4c565b505050565b6005548290600160a01b900460ff16610d8e576040516308a9441960e31b815260040160405180910390fd5b6005546040516302265e3160e61b81526001600160a01b038381166004830152339216906389978c4090602401602060405180830381865afa158015610dd6573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610dfa9190613d25565b6001600160a01b031614610e215760405163acd5a82360e01b815260040160405180910390fd5b610d5d8383611cdd565b600554600160a01b900460ff16610e55576040516308a9441960e31b815260040160405180910390fd5b6005546040516302265e3160e61b81523360048201525f916001600160a01b0316906389978c4090602401602060405180830381865afa158015610e9b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ebf9190613d25565b6001600160a01b031614610ee65760405163acd5a82360e01b815260040160405180910390fd5b610ef1338383611e79565b5050565b5f818152602081905260408120545b92915050565b60408051602080820186905283359282019290925290820135606082015260e083810135608083015260a0808501359083015260c080850135908301525f9182910160408051601f19818403018152919052805160209091012060c81c9050610f7960a0850160808601613d86565b66ffffffffffffff168166ffffffffffffff1611159150505b9392505050565b600554600160a01b900460ff16610fc3576040516308a9441960e31b815260040160405180910390fd5b6005546040516302265e3160e61b81523360048201525f916001600160a01b0316906389978c4090602401602060405180830381865afa158015611009573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061102d9190613d25565b6001600160a01b0316146110545760405163acd5a82360e01b815260040160405180910390fd5b610b0233826124f2565b604080518082018252600c81526b486f70724368616e6e656c7360a01b6020918201528151808301835260058152640322e302e360dc1b9082015281517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f81527f84e6908f343601d9ce9fb60d8250394eb8a51c56f1876bc1e017c97acd6567f2918101919091527fb4bcb154e38601c389396fa918314da42d4626f13ef6d0ceb07e5f5d26b2fbc39181019190915246606082015230608082015260a090206006548114610b02576006819055604080517f771f5240ae5fd8a7640d3fb82fa70aab2fb1dbf35f2ef464f8509946717664c56020820152908101829052611178906060015b60405160208183030381529060405261262a565b6006546040517f771f5240ae5fd8a7640d3fb82fa70aab2fb1dbf35f2ef464f8509946717664c5905f90a250565b5f5f6111b58360e001356126f0565b90505f6111c860a0850160808601613d86565b66ffffffffffffff1660386111e36080870160608801613dac565b62ffffff16901b60506111fc6060880160408901613dce565b65ffffffffffff16901b60806112186040890160208a01613df3565b6001600160601b0316901b17171790505f6112a56365e3fa7260e01b6001600160e01b031916865f015f0135848660405160200161128193929190928352602091821b63ffffffff19169183019190915260601b6001600160601b031916603c82015260500190565b604051602081830303815290604052805190602001205f9182526020526040902090565b600654604051601960f81b6020820152600160f81b6021820152602281019190915260428101829052909150606201604051602081830303815290604052805190602001209350505050919050565b604080515f8152602081019091526060908267ffffffffffffffff81111561131e5761131e613e1f565b60405190808252806020026020018201604052801561135157816020015b606081526020019060019003908161133c5790505b5091505f5b838110156113d2576113ad3086868481811061137457611374613e33565b90506020028101906113869190613e47565b8560405160200161139993929190613ea1565b6040516020818303038152906040526127a9565b8382815181106113bf576113bf613e33565b6020908102919091010152600101611356565b505092915050565b6005548290600160a01b900460ff16611406576040516308a9441960e31b815260040160405180910390fd5b6005546040516302265e3160e61b81526001600160a01b038381166004830152339216906389978c4090602401602060405180830381865afa15801561144e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114729190613d25565b6001600160a01b0316146114995760405163acd5a82360e01b815260040160405180910390fd5b610d5d83836124f2565b6040516001600160601b0319606084811b8216602084015283901b1660348201525f9060480160405160208183030381529060405280519060200120905092915050565b604080518082018252600a8152692437b8392632b233b2b960b11b6020918201528151808301835260058152640322e302e360dc1b9082015281517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f81527f6cd681790c78c220517b099a737f8e85f69e797abe4e2910fb189b61db4bf2cd918101919091527fb4bcb154e38601c389396fa918314da42d4626f13ef6d0ceb07e5f5d26b2fbc39181019190915246606082015230608082015260a090206004548114610b0257600481905560405181907fa43fad83920fd09445855e854e73c9c532e17402c9ceb09993a2392843a5bdb9905f90a250565b600554600160a01b900460ff1661160a576040516308a9441960e31b815260040160405180910390fd5b6005546040516302265e3160e61b81523360048201525f916001600160a01b0316906389978c4090602401602060405180830381865afa158015611650573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116749190613d25565b6001600160a01b03161461169b5760405163acd5a82360e01b815260040160405180910390fd5b6116a6338383611764565b6040516323b872dd60e01b81523360048201523060248201526001600160601b03821660448201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906323b872dd906064016020604051808303815f875af115801561171e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906117429190613d67565b1515600114610ef15760405163022e258160e11b815260040160405180910390fd5b8060016001600160601b03821610156117905760405163c52e3eff60e01b815260040160405180910390fd5b6a084595161401484a0000006001600160601b03821611156117c55760405163293ceef960e21b815260040160405180910390fd5b8383806001600160a01b0316826001600160a01b0316036117f957604051634bd1d76960e11b815260040160405180910390fd5b6001600160a01b0382166118555760405163eac0d38960e01b815260206004820152601860248201527f736f75726365206d757374206e6f7420626520656d707479000000000000000060448201526064015b60405180910390fd5b6001600160a01b0381166118ac5760405163eac0d38960e01b815260206004820152601d60248201527f64657374696e6174696f6e206d757374206e6f7420626520656d707479000000604482015260640161184c565b5f6118b787876114a3565b5f81815260208190526040902090915060028154600160c81b900460ff1660028111156118e6576118e6613b57565b036119475760405163499463c160e01b815260206004820152602a60248201527f63616e6e6f742066756e642061206368616e6e656c20746861742077696c6c2060448201526931b637b9b29039b7b7b760b11b606482015260840161184c565b805461195d9087906001600160601b0316613ec0565b81546001600160601b0319166001600160601b03919091161781555f8154600160c81b900460ff16600281111561199657611996613b57565b03611ac75780546119b490600160b01b900462ffffff166001613edf565b815462ffffff91909116600160b01b026dff00000000000000ffffffffffff60601b19166dffffffff00000000ffffffffffff60601b1990911617600160c81b1781555f82815260208190526040902054611a6b907e4c0177ad15bb302b4419e9ac96065d6f436e0f61972f97bbe272834c40f9349084908b908b906040805160208101969096528501939093526001600160601b0319606092831b811683860152911b166074830152608882015260a801611164565b866001600160a01b0316886001600160a01b0316837e4c0177ad15bb302b4419e9ac96065d6f436e0f61972f97bbe272834c40f934611ab5865f9081526020819052604090205490565b60405190815260200160405180910390a45b611b0d5f5160206140675f395f51905f5283611aee855f9081526020819052604090205490565b6040805160208101949094528301919091526060820152608001611164565b815f5160206140675f395f51905f52611b31845f9081526020819052604090205490565b60405190815260200160405180910390a25050505050505050565b5f611b5782846114a3565b5f8181526020819052604081209192508154600160c81b900460ff166002811115611b8457611b84613b57565b03611ba25760405163499463c160e01b815260040161184c90613efa565b8054600163ff00000160b01b0319811682555f838152602081905260409020546001600160601b0390911690611be8905f5160206140875f395f51905f52908590611aee565b825f5160206140875f395f51905f52611c0c855f9081526020819052604090205490565b60405190815260200160405180910390a28015611cd65760405163a9059cbb60e01b81526001600160a01b038581166004830152602482018390527f0000000000000000000000000000000000000000000000000000000000000000169063a9059cbb906044015b6020604051808303815f875af1158015611c90573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611cb49190613d67565b1515600114611cd65760405163022e258160e11b815260040160405180910390fd5b5050505050565b5f611ce883836114a3565b5f81815260208190526040902090915060028154600160c81b900460ff166002811115611d1757611d17613b57565b14611d745760405163499463c160e01b815260206004820152602660248201527f6368616e6e656c207374617465206d7573742062652050454e44494e475f544f6044820152655f434c4f534560d01b606482015260840161184c565b805463ffffffff428116600160901b9092041610611da5576040516338b2019560e11b815260040160405180910390fd5b8054600163ff00000160b01b0319811682555f838152602081905260409020546001600160601b0390911690611deb905f5160206140875f395f51905f52908590611aee565b825f5160206140875f395f51905f52611e0f855f9081526020819052604090205490565b60405190815260200160405180910390a28015611cd65760405163a9059cbb60e01b8152336004820152602481018290527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063a9059cbb90604401611c74565b611e896040830160208401613df3565b60016001600160601b0382161015611eb45760405163c52e3eff60e01b815260040160405180910390fd5b6a084595161401484a0000006001600160601b0382161115611ee95760405163293ceef960e21b815260040160405180910390fd5b8260e00135611ef78161281b565b611f1457604051633ae4ed6b60e01b815260040160405180910390fd5b83355f90815260208190526040902060018154600160c81b900460ff166002811115611f4257611f42613b57565b14158015611f6d575060028154600160c81b900460ff166002811115611f6a57611f6a613b57565b14155b15611fd55760405163499463c160e01b815260206004820152603160248201527f7370656e64696e67206368616e6e656c206d757374206265204f50454e206f726044820152702050454e44494e475f544f5f434c4f534560781b606482015260840161184c565b611fe56080860160608701613dac565b8154600160b01b900462ffffff9081169116146120455760405163499463c160e01b815260206004820152601860248201527f6368616e6e656c2065706f6368206d757374206d617463680000000000000000604482015260640161184c565b5f6120566060870160408801613dce565b825490915065ffffffffffff600160601b9091048116908216101561208e576040516331de484760e01b815260040160405180910390fd5b61209e6040870160208801613df3565b82546001600160601b03918216911610156120cc57604051632c51d8db60e21b815260040160405180910390fd5b5f6120d6876111a6565b90506120e3818888610f0a565b6121005760405163ee835c8960e01b815260040160405180910390fd5b5f60405180606001604052808381526020018a6001600160a01b0316815260200160065460405160200161213691815260200190565b60408051601f198184030181529190529052905061216261215c36899003890189613f4a565b8261283c565b61217f576040516312bfb7b760e31b815260040160405180910390fd5b5f6121938360a08b013560c08c0135612abb565b905088356121a1828c6114a3565b146121bf576040516366eea9ab60e11b815260040160405180910390fd5b6121ca600185613fe3565b855465ffffffffffff91909116600160601b0265ffffffffffff60601b199091161785556121fe60408a0160208b01613df3565b855461221391906001600160601b0316614001565b85546001600160601b0319166001600160601b039190911617855588355f81815260208190526040902054612269917f0c4672f14b63bb6354fac475ee498055a2784455673af224717b9770fd68d8d191611aee565b88355f818152602081905260409020547f0c4672f14b63bb6354fac475ee498055a2784455673af224717b9770fd68d8d19060405190815260200160405180910390a25f6122b78b836114a3565b5f818152602081905260408082208d35808452919092205492935090916122ff917fed22f34d154d253a7f6fd477439be59080a1554aa0d0642686d64223ab544c8a91611aee565b8a355f818152602081905260409020547fed22f34d154d253a7f6fd477439be59080a1554aa0d0642686d64223ab544c8a9060405190815260200160405180910390a25f8154600160c81b900460ff16600281111561236057612360613b57565b03612446577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663a9059cbb338d5f0160200160208101906123aa9190613df3565b6040516001600160e01b031960e085901b1681526001600160a01b0390921660048301526001600160601b031660248201526044016020604051808303815f875af11580156123fb573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061241f9190613d67565b15156001146124415760405163022e258160e11b815260040160405180910390fd5b6124e4565b61245660408c0160208d01613df3565b815461246b91906001600160601b0316613ec0565b81546001600160601b0319166001600160601b03919091161781555f828152602081905260409020546124ae905f5160206140675f395f51905f52908490611aee565b815f5160206140675f395f51905f526124d2845f9081526020819052604090205490565b60405190815260200160405180910390a25b505050505050505050505050565b5f6124fd83836114a3565b5f8181526020819052604081209192508154600160c81b900460ff16600281111561252a5761252a613b57565b036125485760405163499463c160e01b815260040161184c90613efa565b6125727f000000000000000000000000000000000000000000000000000000000000000042614020565b815463ffffffff91909116600160901b0260ff60c81b191667ff000000ffffffff60901b1990911617600160c91b1781555f828152602081905260409020546125dd907e61e8037197b4d91cf8add2a0736613459081f236d1c8ab58549ebbc330b4e3908490611aee565b817e61e8037197b4d91cf8add2a0736613459081f236d1c8ab58549ebbc330b4e3612613845f9081526020819052604090205490565b60405190815260200160405180910390a250505050565b6001546002545f9161264891600160e01b900463ffffffff16613d54565b421115612653575060015b600454600254835160208086019190912060408051808401959095524360e01b6001600160e01b0319169085015291901b63ffffffff19166044830152606082015260800160408051601f19818403018152919052805160209182012063ffffffff4216600160e01b02911c176002558015610ef15750506002546001600160e01b038116600160e01b9182900463ffffffff1690910217600355565b5f600181601b7f79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f8179870014551231950b75fc4402da1732fc9bebe197f79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f817988709604080515f8152602081018083529590955260ff909316928401929092526060830152608082015260a0016020604051602081039080840390855afa158015612798573d5f5f3e3d5ffd5b5050604051601f1901519392505050565b60605f5f846001600160a01b0316846040516127c5919061403c565b5f60405180830381855af49150503d805f81146127fd576040519150601f19603f3d011682016040523d82523d5f602084013e612802565b606091505b5091509150612812858383612ae5565b95945050505050565b5f811580610f0457505070014551231950b75fc4402da1732fc9bebe191190565b5f6401000003d019836060015110158061286057506401000003d019836040015110155b1561287e57604051633ae4ed6b60e01b815260040160405180910390fd5b61288f835f01518460200151612b41565b6128ac57604051633922a54160e11b815260040160405180910390fd5b5f5f6128fc8460200151855f01516040516020016128e392919060609290921b6001600160601b0319168252601482015260340190565b6040516020818303038152906040528560400151612b6b565b915091505f61291086604001518484612bec565b905061294b86608001518760a00151604080516020808201949094528082019290925280518083038201815260609092019052805191012090565b6001600160a01b0316816001600160a01b03161461297c57604051631dbfb9b360e31b815260040160405180910390fd5b5f6129938760600151885f01518960200151612bec565b90506129ce8760c001518860e00151604080516020808201949094528082019290925280518083038201815260609092019052805191012090565b6001600160a01b0316816001600160a01b0316146129ff57604051631dbfb9b360e31b815260040160405180910390fd5b5f5f612a2f89608001518a60a001518b60c001518c60e001516401000003d019612a299190613e0c565b5f612c85565b6020808b01518c518d8301518d516040519698509496505f95612aa695612a8d958a928a92910160609690961b6001600160601b03191686526014860194909452603485019290925260548401526074830152609482015260b40190565b6040516020818303038152906040528a60400151612e07565b60608b01511497505050505050505092915050565b5f5f5f5f612aca878787612e74565b925092509250612ada8282612ea6565b509095945050505050565b606082612afa57612af582612f5e565b610f92565b8151158015612b1157506001600160a01b0384163b155b15612b3a57604051639996b31560e01b81526001600160a01b038516600482015260240161184c565b5080610f92565b5f6401000003d01980846401000003d019868709096007086401000003d019838409149392505050565b5f5f5f5f612b798686612f86565b915091505f5f612b888461303b565b915091505f5f612b978561303b565b915091505f5f612bca868686867f3f8731abdd661adca08a5558f0f5d272e953d363cb6f0e5d405447c01a444533612c85565b91509150612bd882826132f5565b9950995050505050505050505b9250929050565b5f80612bf9600284614047565b5f03612c075750601b612c0b565b50601c5b60015f828670014551231950b75fc4402da1732fc9bebe19888a09604080515f8152602081018083529590955260ff909316928401929092526060830152608082015260a0016020604051602081039080840390855afa158015612c71573d5f5f3e3d5ffd5b5050604051601f1901519695505050505050565b5f5f838614198588141615612c98575f5ffd5b5f5f858814878a141660018114612cb4578015612d3157612daa565b6401000003d01989600209915060405160208152602080820152602060408201528260608201526401000003d21960808201526401000003d01960a082015260208160c08360055f19fa612d06575f5ffd5b6401000003d019876401000003d019808e8f09600309086401000003d0198251820994505050612daa565b6401000003d0198a6401000003d019038908915060405160208152602080820152602060408201528260608201526401000003d21960808201526401000003d01960a082015260208160c08360055f19fa612d8a575f5ffd5b6401000003d01981516401000003d0198c6401000003d019038b08099350505b50506401000003d01980896401000003d01903886401000003d01903086401000003d0198384090892506401000003d019876401000003d019036401000003d01980866401000003d019038c088409089150509550959350505050565b5f5f5f612e1485856135dd565b9150915060405160308152602080820152602060408201528260608201528160808201526001609082015270014551231950b75fc4402da1732fc9bebe1960b082015260208160d08360055f19fa612e6a575f5ffd5b5195945050505050565b5f80806001600160ff1b038416601b60ff86901c01612e95888289856136f0565b945094509450505093509350939050565b5f826003811115612eb957612eb9613b57565b03612ec2575050565b6001826003811115612ed657612ed6613b57565b03612ef45760405163f645eedf60e01b815260040160405180910390fd5b6002826003811115612f0857612f08613b57565b03612f295760405163fce698f760e01b81526004810182905260240161184c565b6003826003811115612f3d57612f3d613b57565b03610ef1576040516335e2f38360e21b81526004810182905260240161184c565b805115612f6d57805160208201fd5b60405163d6bda27560e01b815260040160405180910390fd5b5f5f5f5f5f612f9587876137b8565b9250925092506040516030815260208082015260206040820152836060820152826080820152600160908201526401000003d01960b082015260208160d08360055f19fa612fe1575f5ffd5b80519550506040516030815260208082015282605082015260206040820152816070820152600160908201526401000003d01960b082015260208160d08360055f19fa61302c575f5ffd5b80519450505050509250929050565b5f5f6401000003d0198384096401000003d019816401000003db190990506401000003d0198182096401000003d01982820890506401000003d019600182086401000003d0196106eb820990505f82156001811461309e5780156130ac576130b8565b6401000003db1991506130b8565b836401000003d0190391505b506401000003d019817f3f8731abdd661adca08a5558f0f5d272e953d363cb6f0e5d405447c01a4445330990506401000003d01982830992506401000003d0198182096401000003d019817f3f8731abdd661adca08a5558f0f5d272e953d363cb6f0e5d405447c01a444533096401000003d01981860894506401000003d01984860994506401000003d01983830991506401000003d019826106eb0990506401000003d0198186089450506401000003d01983860996505f5f6401000003d0198384096401000003d0198488096401000003d0198183099150604051602081526020808201526020604082015282606082015263400000f5600160fe1b0360808201526401000003d01960a082015260208160c08360055f19fa6131db575f5ffd5b6401000003d01982825109925050506401000003d0197f31fdf302724013e57ad13fb38f842afeec184f00a74789dd286729c8303c4a5982096401000003d0198283096401000003d01986820990508881146001811461324057801561324c57613253565b60019450839550613253565b5f94508295505b505050506401000003d0198a880997506401000003d0198289099750801561327c578498508197505b50505060028506600288061461329857846401000003d0190394505b604051935060208452602080850152602060408501528060608501525050506401000003d21960808201526401000003d01960a082015260208160c08360055f19fa6132e2575f5ffd5b6401000003d01981518409925050915091565b5f5f6401000003d0198485096401000003d0198186096401000003d019807f8e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38daaaaa8c76401000003d019897f07d3d4c80bc321d5b9f315cea7fd44c5d595d2fc0bf63b92dfff1044f17c658109086401000003d01980857f534c328d23f234e6e2a413deca25caece4506144037c40314ecbd0b53d9dd262096401000003d019857f8e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38e38daaaaa88c0908086401000003d0197fd35771193d94918a9ca34ccbb7b640dd86cd409542f8487d9fe6b745781eb49b6401000003d019808a7fedadc6f64383dc1df7c4b2d51b54225406d36b641f5e41bbc52a56612a8c6d140986080860405160208152602080820152602060408201528160608201526401000003d21960808201526401000003d01960a082015260208160c08360055f19fa613451575f5ffd5b805191506401000003d01982840996506401000003d019807f4bda12f684bda12f684bda12f684bda12f684bda12f684bda12f684b8e38e23c6401000003d0198c7fc75e0c32d5cb7c0fa9d0a54b12a0a6d5647ab046d686da6fdffc90fc201d71a309086401000003d01980887f29a6194691f91a73715209ef6512e576722830a201be2018a765e85a9ecee931096401000003d019887f2f684bda12f684bda12f684bda12f684bda12f684bda12f684bda12f38e38d8409080892506401000003d019806401000006c4196401000003d0198c7f7a06534bb8bdb49fd5e9e6632722c2989467c1bfc8e8d978dfb425d2685c257309086401000003d01980887f6484aa716545ca2cf3a70c3fa8fe337e0a3d21162f0d6299a7bf8192bfd2a76f098708089450604051905060208152602080820152602060408201528460608201526401000003d21960808201526401000003d01960a082015260208160c08360055f19fa6135bf575f5ffd5b5193506401000003d019905083818389090993505050509250929050565b5f5f60ff835111156135ed575f5ffd5b5f6040515f5b6088811015613609575f828201526020016135f3565b50608860205f5b88518110156136315788820151848401526020928301929182019101613610565b50506089875101905060308183015360020160205f5b87518110156136685787820151848401526020928301929182019101613647565b5050608b8651885101019050855181830153508551855101608c01812091505060405181815260016020820153602160205f5b87518110156136bc578782015184840152602092830192918201910161369b565b5050508451855160210182015384516022018120935083821881526002602082015384516022018120925050509250929050565b5f80807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a084111561372957505f915060039050826137ae565b604080515f808252602082018084528a905260ff891692820192909252606081018790526080810186905260019060a0016020604051602081039080840390855afa15801561377a573d5f5f3e3d5ffd5b5050604051601f1901519150506001600160a01b0381166137a557505f9250600191508290506137ae565b92505f91508190505b9450945094915050565b5f5f5f60ff845111156137c9575f5ffd5b5f6040515f5b60888110156137e5575f828201526020016137cf565b50608860205f5b895181101561380d57898201518484015260209283019291820191016137ec565b50506089885101905060608183015360020160205f5b88518110156138445788820151848401526020928301929182019101613823565b5050608b8751895101019050865181830153508651865101608c01812091505060405181815260016020820153602160205f5b88518110156138985788820151848401526020928301929182019101613877565b5050508551865160210182015385516022018120945084821881526002602082015385516022018120935083821881526003602082015385516022018120925050509250925092565b6001600160a01b0381168114610b02575f5ffd5b5f5f83601f840112613905575f5ffd5b50813567ffffffffffffffff81111561391c575f5ffd5b602083019150836020828501011115612be5575f5ffd5b5f5f5f5f5f5f5f5f60c0898b03121561394a575f5ffd5b8835613955816138e1565b97506020890135613965816138e1565b96506040890135613975816138e1565b955060608901359450608089013567ffffffffffffffff811115613997575f5ffd5b6139a38b828c016138f5565b90955093505060a089013567ffffffffffffffff8111156139c2575f5ffd5b6139ce8b828c016138f5565b999c989b5096995094979396929594505050565b80356001600160601b03811681146139f8575f5ffd5b919050565b5f5f5f60608486031215613a0f575f5ffd5b8335613a1a816138e1565b92506020840135613a2a816138e1565b9150613a38604085016139e2565b90509250925092565b5f60208284031215613a51575f5ffd5b8135610f92816138e1565b5f6101008284031215613a6d575f5ffd5b50919050565b5f5f5f6102208486031215613a86575f5ffd5b8335613a91816138e1565b9250613aa08560208601613a5c565b9150613a38856101208601613a5c565b5f5f60408385031215613ac1575f5ffd5b8235613acc816138e1565b91506020830135613adc816138e1565b809150509250929050565b5f5f6102008385031215613af9575f5ffd5b613b038484613a5c565b9150613b13846101008501613a5c565b90509250929050565b5f60208284031215613b2c575f5ffd5b5035919050565b5f5f5f6102208486031215613b46575f5ffd5b83359250613aa08560208601613a5c565b634e487b7160e01b5f52602160045260245ffd5b6001600160601b038616815265ffffffffffff8516602082015263ffffffff8416604082015262ffffff8316606082015260a0810160038310613bbc57634e487b7160e01b5f52602160045260245ffd5b8260808301529695505050505050565b5f6101008284031215613bdd575f5ffd5b610f928383613a5c565b5f5f60208385031215613bf8575f5ffd5b823567ffffffffffffffff811115613c0e575f5ffd5b8301601f81018513613c1e575f5ffd5b803567ffffffffffffffff811115613c34575f5ffd5b8560208260051b8401011115613c48575f5ffd5b6020919091019590945092505050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015613cdd57603f19878603018452613cc8858351613c58565b94506020938401939190910190600101613cac565b50929695505050505050565b602081525f610f926020830184613c58565b5f5f60408385031215613d0c575f5ffd5b8235613d17816138e1565b9150613b13602084016139e2565b5f60208284031215613d35575f5ffd5b8151610f92816138e1565b634e487b7160e01b5f52601160045260245ffd5b80820180821115610f0457610f04613d40565b5f60208284031215613d77575f5ffd5b81518015158114610f92575f5ffd5b5f60208284031215613d96575f5ffd5b813566ffffffffffffff81168114610f92575f5ffd5b5f60208284031215613dbc575f5ffd5b813562ffffff81168114610f92575f5ffd5b5f60208284031215613dde575f5ffd5b813565ffffffffffff81168114610f92575f5ffd5b5f60208284031215613e03575f5ffd5b610f92826139e2565b81810381811115610f0457610f04613d40565b634e487b7160e01b5f52604160045260245ffd5b634e487b7160e01b5f52603260045260245ffd5b5f5f8335601e19843603018112613e5c575f5ffd5b83018035915067ffffffffffffffff821115613e76575f5ffd5b602001915036819003821315612be5575f5ffd5b5f81518060208401855e5f93019283525090919050565b828482375f8382015f8152613eb68185613e8a565b9695505050505050565b6001600160601b038181168382160190811115610f0457610f04613d40565b62ffffff8181168382160190811115610f0457610f04613d40565b60208082526030908201527f6368616e6e656c206d7573742068617665207374617465204f50454e206f722060408201526f50454e44494e475f544f5f434c4f534560801b606082015260800190565b5f610100828403128015613f5c575f5ffd5b50604051610100810167ffffffffffffffff81118282101715613f8d57634e487b7160e01b5f52604160045260245ffd5b604090815283358252602080850135908301528381013590820152606080840135908201526080808401359082015260a0808401359082015260c0808401359082015260e0928301359281019290925250919050565b65ffffffffffff8181168382160190811115610f0457610f04613d40565b6001600160601b038281168282160390811115610f0457610f04613d40565b63ffffffff8181168382160190811115610f0457610f04613d40565b5f610f928284613e8a565b5f8261406157634e487b7160e01b5f52601260045260245ffd5b50069056feec21547ca1e22edc3f9b4f4e0da94638b5b94dcb18d52dcc072abe5801a8013c197bba684a6afb7ba24f1d265605414b1d0051a6e295d6ff7b6e78e870d7a7f0a26469706673582212205b87f29e9f5a1eff814c1c2274c276ee1d2d56f541e2f217d56c2f69e2030e7564736f6c634300081e0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\xFCW_5`\xE0\x1C\x80cz~\xBD{\x11a\x01\x14W\x80c\xBE\x9B\xAB\xDC\x11a\0\xA9W\x80c\xDC\x96\xFDP\x11a\0yW\x80c\xDC\x96\xFDP\x14a\x05\xA1W\x80c\xDD\xAD\x19\x02\x14a\x05\xA9W\x80c\xF6\x98\xDA%\x14a\x05\xDAW\x80c\xFCU0\x9A\x14a\x05\xE3W\x80c\xFF\xA1\xADt\x14a\x05\xA9W__\xFD[\x80c\xBE\x9B\xAB\xDC\x14a\x05\x1EW\x80c\xC9f\xC4\xFE\x14a\x051W\x80c\xD7\xB0\xFE\xF1\x14a\x05:W\x80c\xD9L\xC1\xB4\x14a\x05zW__\xFD[\x80c\xABf\xCC\xAB\x11a\0\xE4W\x80c\xABf\xCC\xAB\x14a\x04\xC1W\x80c\xAC\x96P\xD8\x14a\x04\xD4W\x80c\xB9 \xDE\xED\x14a\x04\xF4W\x80c\xBD\xA6_E\x14a\x05\x0BW__\xFD[\x80cz~\xBD{\x14a\x03\xF7W\x80c|\x8E(\xDA\x14a\x04gW\x80c\x82\xBF\xEF\xC8\x14a\x04zW\x80c\x89\xCC\xFE\x89\x14a\x04\xB9W__\xFD[\x80cT\xA2\xED\xF5\x11a\x01\x95W\x80cm+\xEE\xF1\x11a\x01eW\x80cm+\xEE\xF1\x14a\x03jW\x80crX\x1C\xC0\x14a\x03sW\x80cv+\xD7k\x14a\x03\x9AW\x80cx\xD1I\xA8\x14a\x03\xADW\x80cx\xD8\x01m\x14a\x03\xD0W__\xFD[\x80cT\xA2\xED\xF5\x14a\x03\x1FW\x80c]/\x07\xC5\x14a\x032W\x80ce\x15\x14\xBF\x14a\x03DW\x80ce\xE3\xFAr\x14a\x03WW__\xFD[\x80c#\xCB:\xC0\x11a\x01\xD0W\x80c#\xCB:\xC0\x14a\x02\xA4W\x80c)9.2\x14a\x02\xB7W\x80c-P\xB1\x8B\x14a\x02\xD7W\x80cD\xDA\xE6\xF8\x14a\x02\xEAW__\xFD[\x80b#\xDE)\x14a\x02\0W\x80c\n\xBE\xC5\x8F\x14a\x02\x15W\x80c\r\xF1\x8F\x94\x14a\x02(W\x80c\x1A\x7F\xFEz\x14a\x02\x91W[__\xFD[a\x02\x13a\x02\x0E6`\x04a93V[a\x05\xF6V[\0[a\x02\x13a\x02#6`\x04a9\xFDV[a\x08\xAFV[`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x03Tc\xFF\xFF\xFF\xFF\x19\x81\x83\x1B\x16\x83R`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90\x82\x01R[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x19\x16\x81R` \x92\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x92\x81\x01\x92\x90\x92R\x01[`@Q\x80\x91\x03\x90\xF3[a\x02\x13a\x02\x9F6`\x04a:AV[a\n=V[a\x02\x13a\x02\xB26`\x04a:AV[a\x0B\x05V[a\x02\xBF`\x01\x81V[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x88V[a\x02\x13a\x02\xE56`\x04a:sV[a\x0B\xCAV[a\x03\x11\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01a\x02\x88V[a\x02\x13a\x03-6`\x04a:\xB0V[a\x0C\x94V[a\x02\xBFj\x08E\x95\x16\x14\x01HJ\0\0\0\x81V[a\x02\x13a\x03R6`\x04a:\xB0V[a\rbV[a\x02\x13a\x03e6`\x04a:\xE7V[a\x0E+V[a\x03\x11`\x01T\x81V[a\x03\x11\x7F\xB2\x81\xFC\x8C\x12\x95M\"TM\xB4]\xE3\x15\x9A9'(\x95\xB1i\xA8R\xB3\x14\xF9\xCCv.D\xC5;\x81V[a\x03\x11a\x03\xA86`\x04a;\x1CV[a\x0E\xF5V[a\x03\xC0a\x03\xBB6`\x04a;3V[a\x0F\nV[`@Q\x90\x15\x15\x81R` \x01a\x02\x88V[a\x03\x11\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04Va\x04\x056`\x04a;\x1CV[_` \x81\x90R\x90\x81R`@\x90 T`\x01`\x01``\x1B\x03\x81\x16\x90`\x01``\x1B\x81\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90`\x01`\x90\x1B\x81\x04c\xFF\xFF\xFF\xFF\x16\x90`\x01`\xB0\x1B\x81\x04b\xFF\xFF\xFF\x16\x90`\x01`\xC8\x1B\x90\x04`\xFF\x16\x85V[`@Qa\x02\x88\x95\x94\x93\x92\x91\x90a;kV[a\x02\x13a\x04u6`\x04a:AV[a\x0F\x99V[a\x04\xA1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x88V[a\x02\x13a\x10^V[a\x03\x11a\x04\xCF6`\x04a;\xCCV[a\x11\xA6V[a\x04\xE7a\x04\xE26`\x04a;\xE7V[a\x12\xF4V[`@Qa\x02\x88\x91\x90a<\x86V[B[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x88V[a\x02\x13a\x05\x196`\x04a:\xB0V[a\x13\xDAV[a\x03\x11a\x05,6`\x04a:\xB0V[a\x14\xA3V[a\x03\x11`\x04T\x81V[`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x02Tc\xFF\xFF\xFF\xFF\x19\x81\x83\x1B\x16\x83R`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90\x82\x01Ra\x02dV[a\x04\xF6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\x13a\x14\xE7V[a\x05\xCD`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03\"\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[`@Qa\x02\x88\x91\x90a<\xE9V[a\x03\x11`\x06T\x81V[a\x02\x13a\x05\xF16`\x04a<\xFBV[a\x15\xE0V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x06?W`@QcPy\xFFu`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x86\x160\x14a\x06hW`@Qc\x178\x92!`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x15a\x08\xA5W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x03a\x07\xC6W`\x01`\x01``\x1B\x03\x85\x11\x15a\x06\xBDW`@Qc)<\xEE\xF9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05T`@Qc\x02&^1`\xE6\x1B\x81R\x855``\x90\x81\x1C`\x04\x83\x01\x81\x90R\x92`\x14\x88\x015\x90\x91\x1C\x91_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x89\x97\x8C@\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\x15W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x079\x91\x90a=%V[\x90P\x82`\x01`\x01`\xA0\x1B\x03\x16\x8A`\x01`\x01`\xA0\x1B\x03\x16\x03a\x07\x81W`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x07|W`@Qc\xAC\xD5\xA8#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07\xB3V[\x89`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07\xB3W`@Qc\xAC\xD5\xA8#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07\xBE\x83\x83\x8Aa\x17dV[PPPa\x08\xA5V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x03a\x08\x8CW\x835``\x90\x81\x1C\x90`\x14\x86\x015`\xA0\x90\x81\x1C\x91` \x88\x015\x90\x1C\x90`4\x88\x015\x90\x1C\x88\x15\x80a\x081WPa\x08-`\x01`\x01``\x1B\x03\x80\x83\x16\x90\x85\x16a=TV[\x89\x14\x15[\x15a\x08OW`@Qc\xC5.>\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01``\x1B\x03\x83\x16\x15a\x08iWa\x08i\x84\x83\x85a\x17dV[`\x01`\x01``\x1B\x03\x81\x16\x15a\x08\x83Wa\x08\x83\x82\x85\x83a\x17dV[PPPPa\x08\xA5V[`@Qc\r=\xCD\xE5`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPV[`\x05T\x83\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x08\xDBW`@Qc\x08\xA9D\x19`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05T`@Qc\x02&^1`\xE6\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R3\x92\x16\x90c\x89\x97\x8C@\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t#W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tG\x91\x90a=%V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\tnW`@Qc\xAC\xD5\xA8#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\ty\x84\x84\x84a\x17dV[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`\x01`\x01``\x1B\x03\x83\x16`D\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\t\xF1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x15\x91\x90a=gV[\x15\x15`\x01\x14a\n7W`@Qc\x02.%\x81`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPV[`\x05T`\x01`\xA0\x1B\x90\x04`\xFF\x16a\ngW`@Qc\x08\xA9D\x19`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05T`@Qc\x02&^1`\xE6\x1B\x81R3`\x04\x82\x01R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x89\x97\x8C@\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xADW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xD1\x91\x90a=%V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\n\xF8W`@Qc\xAC\xD5\xA8#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B\x023\x82a\x1BLV[PV[`\x05T`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x0B/W`@Qc\x08\xA9D\x19`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05T`@Qc\x02&^1`\xE6\x1B\x81R3`\x04\x82\x01R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x89\x97\x8C@\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BuW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x99\x91\x90a=%V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0B\xC0W`@Qc\xAC\xD5\xA8#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B\x023\x82a\x1C\xDDV[`\x05T\x83\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x0B\xF6W`@Qc\x08\xA9D\x19`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05T`@Qc\x02&^1`\xE6\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R3\x92\x16\x90c\x89\x97\x8C@\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C>W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Cb\x91\x90a=%V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0C\x89W`@Qc\xAC\xD5\xA8#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\n7\x84\x84\x84a\x1EyV[`\x05T\x82\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x0C\xC0W`@Qc\x08\xA9D\x19`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05T`@Qc\x02&^1`\xE6\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R3\x92\x16\x90c\x89\x97\x8C@\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x08W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r,\x91\x90a=%V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\rSW`@Qc\xAC\xD5\xA8#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\r]\x83\x83a\x1BLV[PPPV[`\x05T\x82\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16a\r\x8EW`@Qc\x08\xA9D\x19`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05T`@Qc\x02&^1`\xE6\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R3\x92\x16\x90c\x89\x97\x8C@\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xD6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xFA\x91\x90a=%V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0E!W`@Qc\xAC\xD5\xA8#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\r]\x83\x83a\x1C\xDDV[`\x05T`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x0EUW`@Qc\x08\xA9D\x19`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05T`@Qc\x02&^1`\xE6\x1B\x81R3`\x04\x82\x01R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x89\x97\x8C@\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\x9BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xBF\x91\x90a=%V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0E\xE6W`@Qc\xAC\xD5\xA8#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0E\xF13\x83\x83a\x1EyV[PPV[_\x81\x81R` \x81\x90R`@\x81 T[\x92\x91PPV[`@\x80Q` \x80\x82\x01\x86\x90R\x835\x92\x82\x01\x92\x90\x92R\x90\x82\x015``\x82\x01R`\xE0\x83\x81\x015`\x80\x83\x01R`\xA0\x80\x85\x015\x90\x83\x01R`\xC0\x80\x85\x015\x90\x83\x01R_\x91\x82\x91\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 `\xC8\x1C\x90Pa\x0Fy`\xA0\x85\x01`\x80\x86\x01a=\x86V[f\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81f\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15\x91PP[\x93\x92PPPV[`\x05T`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x0F\xC3W`@Qc\x08\xA9D\x19`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05T`@Qc\x02&^1`\xE6\x1B\x81R3`\x04\x82\x01R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x89\x97\x8C@\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\tW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10-\x91\x90a=%V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x10TW`@Qc\xAC\xD5\xA8#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B\x023\x82a$\xF2V[`@\x80Q\x80\x82\x01\x82R`\x0C\x81RkHoprChannels`\xA0\x1B` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`\x05\x81Rd\x03\"\xE3\x02\xE3`\xDC\x1B\x90\x82\x01R\x81Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81R\x7F\x84\xE6\x90\x8F46\x01\xD9\xCE\x9F\xB6\r\x82P9N\xB8\xA5\x1CV\xF1\x87k\xC1\xE0\x17\xC9z\xCDeg\xF2\x91\x81\x01\x91\x90\x91R\x7F\xB4\xBC\xB1T\xE3\x86\x01\xC3\x899o\xA9\x181M\xA4-F&\xF1>\xF6\xD0\xCE\xB0~_]&\xB2\xFB\xC3\x91\x81\x01\x91\x90\x91RF``\x82\x01R0`\x80\x82\x01R`\xA0\x90 `\x06T\x81\x14a\x0B\x02W`\x06\x81\x90U`@\x80Q\x7Fw\x1FR@\xAE_\xD8\xA7d\r?\xB8/\xA7\n\xAB/\xB1\xDB\xF3_.\xF4d\xF8P\x99Fqvd\xC5` \x82\x01R\x90\x81\x01\x82\x90Ra\x11x\x90``\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra&*V[`\x06T`@Q\x7Fw\x1FR@\xAE_\xD8\xA7d\r?\xB8/\xA7\n\xAB/\xB1\xDB\xF3_.\xF4d\xF8P\x99Fqvd\xC5\x90_\x90\xA2PV[__a\x11\xB5\x83`\xE0\x015a&\xF0V[\x90P_a\x11\xC8`\xA0\x85\x01`\x80\x86\x01a=\x86V[f\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`8a\x11\xE3`\x80\x87\x01``\x88\x01a=\xACV[b\xFF\xFF\xFF\x16\x90\x1B`Pa\x11\xFC``\x88\x01`@\x89\x01a=\xCEV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x1B`\x80a\x12\x18`@\x89\x01` \x8A\x01a=\xF3V[`\x01`\x01``\x1B\x03\x16\x90\x1B\x17\x17\x17\x90P_a\x12\xA5ce\xE3\xFAr`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x86_\x01_\x015\x84\x86`@Q` \x01a\x12\x81\x93\x92\x91\x90\x92\x83R` \x91\x82\x1Bc\xFF\xFF\xFF\xFF\x19\x16\x91\x83\x01\x91\x90\x91R``\x1B`\x01`\x01``\x1B\x03\x19\x16`<\x82\x01R`P\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 _\x91\x82R` R`@\x90 \x90V[`\x06T`@Q`\x19`\xF8\x1B` \x82\x01R`\x01`\xF8\x1B`!\x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x82\x90R\x90\x91P`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x93PPPP\x91\x90PV[`@\x80Q_\x81R` \x81\x01\x90\x91R``\x90\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\x1EWa\x13\x1Ea>\x1FV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x13QW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x13<W\x90P[P\x91P_[\x83\x81\x10\x15a\x13\xD2Wa\x13\xAD0\x86\x86\x84\x81\x81\x10a\x13tWa\x13ta>3V[\x90P` \x02\x81\x01\x90a\x13\x86\x91\x90a>GV[\x85`@Q` \x01a\x13\x99\x93\x92\x91\x90a>\xA1V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra'\xA9V[\x83\x82\x81Q\x81\x10a\x13\xBFWa\x13\xBFa>3V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x13VV[PP\x92\x91PPV[`\x05T\x82\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x14\x06W`@Qc\x08\xA9D\x19`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05T`@Qc\x02&^1`\xE6\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R3\x92\x16\x90c\x89\x97\x8C@\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14NW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14r\x91\x90a=%V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x14\x99W`@Qc\xAC\xD5\xA8#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\r]\x83\x83a$\xF2V[`@Q`\x01`\x01``\x1B\x03\x19``\x84\x81\x1B\x82\x16` \x84\x01R\x83\x90\x1B\x16`4\x82\x01R_\x90`H\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`@\x80Q\x80\x82\x01\x82R`\n\x81Ri$7\xB89&2\xB23\xB2\xB9`\xB1\x1B` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`\x05\x81Rd\x03\"\xE3\x02\xE3`\xDC\x1B\x90\x82\x01R\x81Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81R\x7Fl\xD6\x81y\x0Cx\xC2 Q{\t\x9As\x7F\x8E\x85\xF6\x9Eyz\xBEN)\x10\xFB\x18\x9Ba\xDBK\xF2\xCD\x91\x81\x01\x91\x90\x91R\x7F\xB4\xBC\xB1T\xE3\x86\x01\xC3\x899o\xA9\x181M\xA4-F&\xF1>\xF6\xD0\xCE\xB0~_]&\xB2\xFB\xC3\x91\x81\x01\x91\x90\x91RF``\x82\x01R0`\x80\x82\x01R`\xA0\x90 `\x04T\x81\x14a\x0B\x02W`\x04\x81\x90U`@Q\x81\x90\x7F\xA4?\xAD\x83\x92\x0F\xD0\x94E\x85^\x85Ns\xC9\xC52\xE1t\x02\xC9\xCE\xB0\x99\x93\xA29(C\xA5\xBD\xB9\x90_\x90\xA2PV[`\x05T`\x01`\xA0\x1B\x90\x04`\xFF\x16a\x16\nW`@Qc\x08\xA9D\x19`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05T`@Qc\x02&^1`\xE6\x1B\x81R3`\x04\x82\x01R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x89\x97\x8C@\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16PW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16t\x91\x90a=%V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x16\x9BW`@Qc\xAC\xD5\xA8#`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x16\xA63\x83\x83a\x17dV[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`\x01`\x01``\x1B\x03\x82\x16`D\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x17\x1EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17B\x91\x90a=gV[\x15\x15`\x01\x14a\x0E\xF1W`@Qc\x02.%\x81`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x01`\x01`\x01``\x1B\x03\x82\x16\x10\x15a\x17\x90W`@Qc\xC5.>\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[j\x08E\x95\x16\x14\x01HJ\0\0\0`\x01`\x01``\x1B\x03\x82\x16\x11\x15a\x17\xC5W`@Qc)<\xEE\xF9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83\x83\x80`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x03a\x17\xF9W`@QcK\xD1\xD7i`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x18UW`@Qc\xEA\xC0\xD3\x89`\xE0\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7Fsource must not be empty\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x18\xACW`@Qc\xEA\xC0\xD3\x89`\xE0\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7Fdestination must not be empty\0\0\0`D\x82\x01R`d\x01a\x18LV[_a\x18\xB7\x87\x87a\x14\xA3V[_\x81\x81R` \x81\x90R`@\x90 \x90\x91P`\x02\x81T`\x01`\xC8\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x18\xE6Wa\x18\xE6a;WV[\x03a\x19GW`@QcI\x94c\xC1`\xE0\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7Fcannot fund a channel that will `D\x82\x01Ri1\xB67\xB9\xB2\x909\xB7\xB7\xB7`\xB1\x1B`d\x82\x01R`\x84\x01a\x18LV[\x80Ta\x19]\x90\x87\x90`\x01`\x01``\x1B\x03\x16a>\xC0V[\x81T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x91\x90\x91\x16\x17\x81U_\x81T`\x01`\xC8\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x19\x96Wa\x19\x96a;WV[\x03a\x1A\xC7W\x80Ta\x19\xB4\x90`\x01`\xB0\x1B\x90\x04b\xFF\xFF\xFF\x16`\x01a>\xDFV[\x81Tb\xFF\xFF\xFF\x91\x90\x91\x16`\x01`\xB0\x1B\x02m\xFF\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF``\x1B\x19\x16m\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF``\x1B\x19\x90\x91\x16\x17`\x01`\xC8\x1B\x17\x81U_\x82\x81R` \x81\x90R`@\x90 Ta\x1Ak\x90~L\x01w\xAD\x15\xBB0+D\x19\xE9\xAC\x96\x06]oCn\x0Fa\x97/\x97\xBB\xE2r\x83L@\xF94\x90\x84\x90\x8B\x90\x8B\x90`@\x80Q` \x81\x01\x96\x90\x96R\x85\x01\x93\x90\x93R`\x01`\x01``\x1B\x03\x19``\x92\x83\x1B\x81\x16\x83\x86\x01R\x91\x1B\x16`t\x83\x01R`\x88\x82\x01R`\xA8\x01a\x11dV[\x86`\x01`\x01`\xA0\x1B\x03\x16\x88`\x01`\x01`\xA0\x1B\x03\x16\x83~L\x01w\xAD\x15\xBB0+D\x19\xE9\xAC\x96\x06]oCn\x0Fa\x97/\x97\xBB\xE2r\x83L@\xF94a\x1A\xB5\x86_\x90\x81R` \x81\x90R`@\x90 T\x90V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA4[a\x1B\r_Q` a@g_9_Q\x90_R\x83a\x1A\xEE\x85_\x90\x81R` \x81\x90R`@\x90 T\x90V[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01R`\x80\x01a\x11dV[\x81_Q` a@g_9_Q\x90_Ra\x1B1\x84_\x90\x81R` \x81\x90R`@\x90 T\x90V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA2PPPPPPPPV[_a\x1BW\x82\x84a\x14\xA3V[_\x81\x81R` \x81\x90R`@\x81 \x91\x92P\x81T`\x01`\xC8\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x1B\x84Wa\x1B\x84a;WV[\x03a\x1B\xA2W`@QcI\x94c\xC1`\xE0\x1B\x81R`\x04\x01a\x18L\x90a>\xFAV[\x80T`\x01c\xFF\0\0\x01`\xB0\x1B\x03\x19\x81\x16\x82U_\x83\x81R` \x81\x90R`@\x90 T`\x01`\x01``\x1B\x03\x90\x91\x16\x90a\x1B\xE8\x90_Q` a@\x87_9_Q\x90_R\x90\x85\x90a\x1A\xEEV[\x82_Q` a@\x87_9_Q\x90_Ra\x1C\x0C\x85_\x90\x81R` \x81\x90R`@\x90 T\x90V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA2\x80\x15a\x1C\xD6W`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1C\x90W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xB4\x91\x90a=gV[\x15\x15`\x01\x14a\x1C\xD6W`@Qc\x02.%\x81`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPV[_a\x1C\xE8\x83\x83a\x14\xA3V[_\x81\x81R` \x81\x90R`@\x90 \x90\x91P`\x02\x81T`\x01`\xC8\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x1D\x17Wa\x1D\x17a;WV[\x14a\x1DtW`@QcI\x94c\xC1`\xE0\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7Fchannel state must be PENDING_TO`D\x82\x01Re_CLOSE`\xD0\x1B`d\x82\x01R`\x84\x01a\x18LV[\x80Tc\xFF\xFF\xFF\xFFB\x81\x16`\x01`\x90\x1B\x90\x92\x04\x16\x10a\x1D\xA5W`@Qc8\xB2\x01\x95`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80T`\x01c\xFF\0\0\x01`\xB0\x1B\x03\x19\x81\x16\x82U_\x83\x81R` \x81\x90R`@\x90 T`\x01`\x01``\x1B\x03\x90\x91\x16\x90a\x1D\xEB\x90_Q` a@\x87_9_Q\x90_R\x90\x85\x90a\x1A\xEEV[\x82_Q` a@\x87_9_Q\x90_Ra\x1E\x0F\x85_\x90\x81R` \x81\x90R`@\x90 T\x90V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA2\x80\x15a\x1C\xD6W`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01a\x1CtV[a\x1E\x89`@\x83\x01` \x84\x01a=\xF3V[`\x01`\x01`\x01``\x1B\x03\x82\x16\x10\x15a\x1E\xB4W`@Qc\xC5.>\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[j\x08E\x95\x16\x14\x01HJ\0\0\0`\x01`\x01``\x1B\x03\x82\x16\x11\x15a\x1E\xE9W`@Qc)<\xEE\xF9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`\xE0\x015a\x1E\xF7\x81a(\x1BV[a\x1F\x14W`@Qc:\xE4\xEDk`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x835_\x90\x81R` \x81\x90R`@\x90 `\x01\x81T`\x01`\xC8\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x1FBWa\x1FBa;WV[\x14\x15\x80\x15a\x1FmWP`\x02\x81T`\x01`\xC8\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x1FjWa\x1Fja;WV[\x14\x15[\x15a\x1F\xD5W`@QcI\x94c\xC1`\xE0\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7Fspending channel must be OPEN or`D\x82\x01Rp PENDING_TO_CLOSE`x\x1B`d\x82\x01R`\x84\x01a\x18LV[a\x1F\xE5`\x80\x86\x01``\x87\x01a=\xACV[\x81T`\x01`\xB0\x1B\x90\x04b\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14a EW`@QcI\x94c\xC1`\xE0\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7Fchannel epoch must match\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x18LV[_a V``\x87\x01`@\x88\x01a=\xCEV[\x82T\x90\x91Pe\xFF\xFF\xFF\xFF\xFF\xFF`\x01``\x1B\x90\x91\x04\x81\x16\x90\x82\x16\x10\x15a \x8EW`@Qc1\xDEHG`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a \x9E`@\x87\x01` \x88\x01a=\xF3V[\x82T`\x01`\x01``\x1B\x03\x91\x82\x16\x91\x16\x10\x15a \xCCW`@Qc,Q\xD8\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a \xD6\x87a\x11\xA6V[\x90Pa \xE3\x81\x88\x88a\x0F\nV[a!\0W`@Qc\xEE\x83\\\x89`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`@Q\x80``\x01`@R\x80\x83\x81R` \x01\x8A`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x06T`@Q` \x01a!6\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x90R\x90Pa!ba!\\6\x89\x90\x03\x89\x01\x89a?JV[\x82a(<V[a!\x7FW`@Qc\x12\xBF\xB7\xB7`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a!\x93\x83`\xA0\x8B\x015`\xC0\x8C\x015a*\xBBV[\x90P\x885a!\xA1\x82\x8Ca\x14\xA3V[\x14a!\xBFW`@Qcf\xEE\xA9\xAB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a!\xCA`\x01\x85a?\xE3V[\x85Te\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\x01``\x1B\x02e\xFF\xFF\xFF\xFF\xFF\xFF``\x1B\x19\x90\x91\x16\x17\x85Ua!\xFE`@\x8A\x01` \x8B\x01a=\xF3V[\x85Ta\"\x13\x91\x90`\x01`\x01``\x1B\x03\x16a@\x01V[\x85T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x91\x90\x91\x16\x17\x85U\x885_\x81\x81R` \x81\x90R`@\x90 Ta\"i\x91\x7F\x0CFr\xF1Kc\xBBcT\xFA\xC4u\xEEI\x80U\xA2xDUg:\xF2$q{\x97p\xFDh\xD8\xD1\x91a\x1A\xEEV[\x885_\x81\x81R` \x81\x90R`@\x90 T\x7F\x0CFr\xF1Kc\xBBcT\xFA\xC4u\xEEI\x80U\xA2xDUg:\xF2$q{\x97p\xFDh\xD8\xD1\x90`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA2_a\"\xB7\x8B\x83a\x14\xA3V[_\x81\x81R` \x81\x90R`@\x80\x82 \x8D5\x80\x84R\x91\x90\x92 T\x92\x93P\x90\x91a\"\xFF\x91\x7F\xED\"\xF3M\x15M%:\x7Fo\xD4wC\x9B\xE5\x90\x80\xA1UJ\xA0\xD0d&\x86\xD6B#\xABTL\x8A\x91a\x1A\xEEV[\x8A5_\x81\x81R` \x81\x90R`@\x90 T\x7F\xED\"\xF3M\x15M%:\x7Fo\xD4wC\x9B\xE5\x90\x80\xA1UJ\xA0\xD0d&\x86\xD6B#\xABTL\x8A\x90`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA2_\x81T`\x01`\xC8\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a#`Wa#`a;WV[\x03a$FW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBB3\x8D_\x01` \x01` \x81\x01\x90a#\xAA\x91\x90a=\xF3V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`\x01`\x01``\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a#\xFBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\x1F\x91\x90a=gV[\x15\x15`\x01\x14a$AW`@Qc\x02.%\x81`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a$\xE4V[a$V`@\x8C\x01` \x8D\x01a=\xF3V[\x81Ta$k\x91\x90`\x01`\x01``\x1B\x03\x16a>\xC0V[\x81T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x91\x90\x91\x16\x17\x81U_\x82\x81R` \x81\x90R`@\x90 Ta$\xAE\x90_Q` a@g_9_Q\x90_R\x90\x84\x90a\x1A\xEEV[\x81_Q` a@g_9_Q\x90_Ra$\xD2\x84_\x90\x81R` \x81\x90R`@\x90 T\x90V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA2[PPPPPPPPPPPPV[_a$\xFD\x83\x83a\x14\xA3V[_\x81\x81R` \x81\x90R`@\x81 \x91\x92P\x81T`\x01`\xC8\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a%*Wa%*a;WV[\x03a%HW`@QcI\x94c\xC1`\xE0\x1B\x81R`\x04\x01a\x18L\x90a>\xFAV[a%r\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Ba@ V[\x81Tc\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\x01`\x90\x1B\x02`\xFF`\xC8\x1B\x19\x16g\xFF\0\0\0\xFF\xFF\xFF\xFF`\x90\x1B\x19\x90\x91\x16\x17`\x01`\xC9\x1B\x17\x81U_\x82\x81R` \x81\x90R`@\x90 Ta%\xDD\x90~a\xE8\x03q\x97\xB4\xD9\x1C\xF8\xAD\xD2\xA0sf\x13E\x90\x81\xF26\xD1\xC8\xABXT\x9E\xBB\xC30\xB4\xE3\x90\x84\x90a\x1A\xEEV[\x81~a\xE8\x03q\x97\xB4\xD9\x1C\xF8\xAD\xD2\xA0sf\x13E\x90\x81\xF26\xD1\xC8\xABXT\x9E\xBB\xC30\xB4\xE3a&\x13\x84_\x90\x81R` \x81\x90R`@\x90 T\x90V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA2PPPPV[`\x01T`\x02T_\x91a&H\x91`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a=TV[B\x11\x15a&SWP`\x01[`\x04T`\x02T\x83Q` \x80\x86\x01\x91\x90\x91 `@\x80Q\x80\x84\x01\x95\x90\x95RC`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x90\x85\x01R\x91\x90\x1Bc\xFF\xFF\xFF\xFF\x19\x16`D\x83\x01R``\x82\x01R`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x91\x82\x01 c\xFF\xFF\xFF\xFFB\x16`\x01`\xE0\x1B\x02\x91\x1C\x17`\x02U\x80\x15a\x0E\xF1WPP`\x02T`\x01`\x01`\xE0\x1B\x03\x81\x16`\x01`\xE0\x1B\x91\x82\x90\x04c\xFF\xFF\xFF\xFF\x16\x90\x91\x02\x17`\x03UV[_`\x01\x81`\x1B\x7Fy\xBEf~\xF9\xDC\xBB\xACU\xA0b\x95\xCE\x87\x0B\x07\x02\x9B\xFC\xDB-\xCE(\xD9Y\xF2\x81[\x16\xF8\x17\x98p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19\x7Fy\xBEf~\xF9\xDC\xBB\xACU\xA0b\x95\xCE\x87\x0B\x07\x02\x9B\xFC\xDB-\xCE(\xD9Y\xF2\x81[\x16\xF8\x17\x98\x87\t`@\x80Q_\x81R` \x81\x01\x80\x83R\x95\x90\x95R`\xFF\x90\x93\x16\x92\x84\x01\x92\x90\x92R``\x83\x01R`\x80\x82\x01R`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a'\x98W=__>=_\xFD[PP`@Q`\x1F\x19\x01Q\x93\x92PPPV[``__\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa'\xC5\x91\x90a@<V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a'\xFDW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a(\x02V[``\x91P[P\x91P\x91Pa(\x12\x85\x83\x83a*\xE5V[\x95\x94PPPPPV[_\x81\x15\x80a\x0F\x04WPPp\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19\x11\x90V[_d\x01\0\0\x03\xD0\x19\x83``\x01Q\x10\x15\x80a(`WPd\x01\0\0\x03\xD0\x19\x83`@\x01Q\x10\x15[\x15a(~W`@Qc:\xE4\xEDk`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a(\x8F\x83_\x01Q\x84` \x01Qa+AV[a(\xACW`@Qc9\"\xA5A`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__a(\xFC\x84` \x01Q\x85_\x01Q`@Q` \x01a(\xE3\x92\x91\x90``\x92\x90\x92\x1B`\x01`\x01``\x1B\x03\x19\x16\x82R`\x14\x82\x01R`4\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85`@\x01Qa+kV[\x91P\x91P_a)\x10\x86`@\x01Q\x84\x84a+\xECV[\x90Pa)K\x86`\x80\x01Q\x87`\xA0\x01Q`@\x80Q` \x80\x82\x01\x94\x90\x94R\x80\x82\x01\x92\x90\x92R\x80Q\x80\x83\x03\x82\x01\x81R``\x90\x92\x01\x90R\x80Q\x91\x01 \x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a)|W`@Qc\x1D\xBF\xB9\xB3`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a)\x93\x87``\x01Q\x88_\x01Q\x89` \x01Qa+\xECV[\x90Pa)\xCE\x87`\xC0\x01Q\x88`\xE0\x01Q`@\x80Q` \x80\x82\x01\x94\x90\x94R\x80\x82\x01\x92\x90\x92R\x80Q\x80\x83\x03\x82\x01\x81R``\x90\x92\x01\x90R\x80Q\x91\x01 \x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a)\xFFW`@Qc\x1D\xBF\xB9\xB3`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__a*/\x89`\x80\x01Q\x8A`\xA0\x01Q\x8B`\xC0\x01Q\x8C`\xE0\x01Qd\x01\0\0\x03\xD0\x19a*)\x91\x90a>\x0CV[_a,\x85V[` \x80\x8B\x01Q\x8CQ\x8D\x83\x01Q\x8DQ`@Q\x96\x98P\x94\x96P_\x95a*\xA6\x95a*\x8D\x95\x8A\x92\x8A\x92\x91\x01``\x96\x90\x96\x1B`\x01`\x01``\x1B\x03\x19\x16\x86R`\x14\x86\x01\x94\x90\x94R`4\x85\x01\x92\x90\x92R`T\x84\x01R`t\x83\x01R`\x94\x82\x01R`\xB4\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x8A`@\x01Qa.\x07V[``\x8B\x01Q\x14\x97PPPPPPPP\x92\x91PPV[____a*\xCA\x87\x87\x87a.tV[\x92P\x92P\x92Pa*\xDA\x82\x82a.\xA6V[P\x90\x95\x94PPPPPV[``\x82a*\xFAWa*\xF5\x82a/^V[a\x0F\x92V[\x81Q\x15\x80\x15a+\x11WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a+:W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x18LV[P\x80a\x0F\x92V[_d\x01\0\0\x03\xD0\x19\x80\x84d\x01\0\0\x03\xD0\x19\x86\x87\t\t`\x07\x08d\x01\0\0\x03\xD0\x19\x83\x84\t\x14\x93\x92PPPV[____a+y\x86\x86a/\x86V[\x91P\x91P__a+\x88\x84a0;V[\x91P\x91P__a+\x97\x85a0;V[\x91P\x91P__a+\xCA\x86\x86\x86\x86\x7F?\x871\xAB\xDDf\x1A\xDC\xA0\x8AUX\xF0\xF5\xD2r\xE9S\xD3c\xCBo\x0E]@TG\xC0\x1ADE3a,\x85V[\x91P\x91Pa+\xD8\x82\x82a2\xF5V[\x99P\x99PPPPPPPPP[\x92P\x92\x90PV[_\x80a+\xF9`\x02\x84a@GV[_\x03a,\x07WP`\x1Ba,\x0BV[P`\x1C[`\x01_\x82\x86p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19\x88\x8A\t`@\x80Q_\x81R` \x81\x01\x80\x83R\x95\x90\x95R`\xFF\x90\x93\x16\x92\x84\x01\x92\x90\x92R``\x83\x01R`\x80\x82\x01R`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a,qW=__>=_\xFD[PP`@Q`\x1F\x19\x01Q\x96\x95PPPPPPV[__\x83\x86\x14\x19\x85\x88\x14\x16\x15a,\x98W__\xFD[__\x85\x88\x14\x87\x8A\x14\x16`\x01\x81\x14a,\xB4W\x80\x15a-1Wa-\xAAV[d\x01\0\0\x03\xD0\x19\x89`\x02\t\x91P`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x82``\x82\x01Rd\x01\0\0\x03\xD2\x19`\x80\x82\x01Rd\x01\0\0\x03\xD0\x19`\xA0\x82\x01R` \x81`\xC0\x83`\x05_\x19\xFAa-\x06W__\xFD[d\x01\0\0\x03\xD0\x19\x87d\x01\0\0\x03\xD0\x19\x80\x8E\x8F\t`\x03\t\x08d\x01\0\0\x03\xD0\x19\x82Q\x82\t\x94PPPa-\xAAV[d\x01\0\0\x03\xD0\x19\x8Ad\x01\0\0\x03\xD0\x19\x03\x89\x08\x91P`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x82``\x82\x01Rd\x01\0\0\x03\xD2\x19`\x80\x82\x01Rd\x01\0\0\x03\xD0\x19`\xA0\x82\x01R` \x81`\xC0\x83`\x05_\x19\xFAa-\x8AW__\xFD[d\x01\0\0\x03\xD0\x19\x81Qd\x01\0\0\x03\xD0\x19\x8Cd\x01\0\0\x03\xD0\x19\x03\x8B\x08\t\x93PP[PPd\x01\0\0\x03\xD0\x19\x80\x89d\x01\0\0\x03\xD0\x19\x03\x88d\x01\0\0\x03\xD0\x19\x03\x08d\x01\0\0\x03\xD0\x19\x83\x84\t\x08\x92Pd\x01\0\0\x03\xD0\x19\x87d\x01\0\0\x03\xD0\x19\x03d\x01\0\0\x03\xD0\x19\x80\x86d\x01\0\0\x03\xD0\x19\x03\x8C\x08\x84\t\x08\x91PP\x95P\x95\x93PPPPV[___a.\x14\x85\x85a5\xDDV[\x91P\x91P`@Q`0\x81R` \x80\x82\x01R` `@\x82\x01R\x82``\x82\x01R\x81`\x80\x82\x01R`\x01`\x90\x82\x01Rp\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19`\xB0\x82\x01R` \x81`\xD0\x83`\x05_\x19\xFAa.jW__\xFD[Q\x95\x94PPPPPV[_\x80\x80`\x01`\x01`\xFF\x1B\x03\x84\x16`\x1B`\xFF\x86\x90\x1C\x01a.\x95\x88\x82\x89\x85a6\xF0V[\x94P\x94P\x94PPP\x93P\x93P\x93\x90PV[_\x82`\x03\x81\x11\x15a.\xB9Wa.\xB9a;WV[\x03a.\xC2WPPV[`\x01\x82`\x03\x81\x11\x15a.\xD6Wa.\xD6a;WV[\x03a.\xF4W`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a/\x08Wa/\x08a;WV[\x03a/)W`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x18LV[`\x03\x82`\x03\x81\x11\x15a/=Wa/=a;WV[\x03a\x0E\xF1W`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x18LV[\x80Q\x15a/mW\x80Q` \x82\x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_____a/\x95\x87\x87a7\xB8V[\x92P\x92P\x92P`@Q`0\x81R` \x80\x82\x01R` `@\x82\x01R\x83``\x82\x01R\x82`\x80\x82\x01R`\x01`\x90\x82\x01Rd\x01\0\0\x03\xD0\x19`\xB0\x82\x01R` \x81`\xD0\x83`\x05_\x19\xFAa/\xE1W__\xFD[\x80Q\x95PP`@Q`0\x81R` \x80\x82\x01R\x82`P\x82\x01R` `@\x82\x01R\x81`p\x82\x01R`\x01`\x90\x82\x01Rd\x01\0\0\x03\xD0\x19`\xB0\x82\x01R` \x81`\xD0\x83`\x05_\x19\xFAa0,W__\xFD[\x80Q\x94PPPPP\x92P\x92\x90PV[__d\x01\0\0\x03\xD0\x19\x83\x84\td\x01\0\0\x03\xD0\x19\x81d\x01\0\0\x03\xDB\x19\t\x90Pd\x01\0\0\x03\xD0\x19\x81\x82\td\x01\0\0\x03\xD0\x19\x82\x82\x08\x90Pd\x01\0\0\x03\xD0\x19`\x01\x82\x08d\x01\0\0\x03\xD0\x19a\x06\xEB\x82\t\x90P_\x82\x15`\x01\x81\x14a0\x9EW\x80\x15a0\xACWa0\xB8V[d\x01\0\0\x03\xDB\x19\x91Pa0\xB8V[\x83d\x01\0\0\x03\xD0\x19\x03\x91P[Pd\x01\0\0\x03\xD0\x19\x81\x7F?\x871\xAB\xDDf\x1A\xDC\xA0\x8AUX\xF0\xF5\xD2r\xE9S\xD3c\xCBo\x0E]@TG\xC0\x1ADE3\t\x90Pd\x01\0\0\x03\xD0\x19\x82\x83\t\x92Pd\x01\0\0\x03\xD0\x19\x81\x82\td\x01\0\0\x03\xD0\x19\x81\x7F?\x871\xAB\xDDf\x1A\xDC\xA0\x8AUX\xF0\xF5\xD2r\xE9S\xD3c\xCBo\x0E]@TG\xC0\x1ADE3\td\x01\0\0\x03\xD0\x19\x81\x86\x08\x94Pd\x01\0\0\x03\xD0\x19\x84\x86\t\x94Pd\x01\0\0\x03\xD0\x19\x83\x83\t\x91Pd\x01\0\0\x03\xD0\x19\x82a\x06\xEB\t\x90Pd\x01\0\0\x03\xD0\x19\x81\x86\x08\x94PPd\x01\0\0\x03\xD0\x19\x83\x86\t\x96P__d\x01\0\0\x03\xD0\x19\x83\x84\td\x01\0\0\x03\xD0\x19\x84\x88\td\x01\0\0\x03\xD0\x19\x81\x83\t\x91P`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x82``\x82\x01Rc@\0\0\xF5`\x01`\xFE\x1B\x03`\x80\x82\x01Rd\x01\0\0\x03\xD0\x19`\xA0\x82\x01R` \x81`\xC0\x83`\x05_\x19\xFAa1\xDBW__\xFD[d\x01\0\0\x03\xD0\x19\x82\x82Q\t\x92PPPd\x01\0\0\x03\xD0\x19\x7F1\xFD\xF3\x02r@\x13\xE5z\xD1?\xB3\x8F\x84*\xFE\xEC\x18O\0\xA7G\x89\xDD(g)\xC80<JY\x82\td\x01\0\0\x03\xD0\x19\x82\x83\td\x01\0\0\x03\xD0\x19\x86\x82\t\x90P\x88\x81\x14`\x01\x81\x14a2@W\x80\x15a2LWa2SV[`\x01\x94P\x83\x95Pa2SV[_\x94P\x82\x95P[PPPPd\x01\0\0\x03\xD0\x19\x8A\x88\t\x97Pd\x01\0\0\x03\xD0\x19\x82\x89\t\x97P\x80\x15a2|W\x84\x98P\x81\x97P[PPP`\x02\x85\x06`\x02\x88\x06\x14a2\x98W\x84d\x01\0\0\x03\xD0\x19\x03\x94P[`@Q\x93P` \x84R` \x80\x85\x01R` `@\x85\x01R\x80``\x85\x01RPPPd\x01\0\0\x03\xD2\x19`\x80\x82\x01Rd\x01\0\0\x03\xD0\x19`\xA0\x82\x01R` \x81`\xC0\x83`\x05_\x19\xFAa2\xE2W__\xFD[d\x01\0\0\x03\xD0\x19\x81Q\x84\t\x92PP\x91P\x91V[__d\x01\0\0\x03\xD0\x19\x84\x85\td\x01\0\0\x03\xD0\x19\x81\x86\td\x01\0\0\x03\xD0\x19\x80\x7F\x8E8\xE3\x8E8\xE3\x8E8\xE3\x8E8\xE3\x8E8\xE3\x8E8\xE3\x8E8\xE3\x8E8\xE3\x8E8\xE3\x8D\xAA\xAA\xA8\xC7d\x01\0\0\x03\xD0\x19\x89\x7F\x07\xD3\xD4\xC8\x0B\xC3!\xD5\xB9\xF3\x15\xCE\xA7\xFDD\xC5\xD5\x95\xD2\xFC\x0B\xF6;\x92\xDF\xFF\x10D\xF1|e\x81\t\x08d\x01\0\0\x03\xD0\x19\x80\x85\x7FSL2\x8D#\xF24\xE6\xE2\xA4\x13\xDE\xCA%\xCA\xEC\xE4PaD\x03|@1N\xCB\xD0\xB5=\x9D\xD2b\td\x01\0\0\x03\xD0\x19\x85\x7F\x8E8\xE3\x8E8\xE3\x8E8\xE3\x8E8\xE3\x8E8\xE3\x8E8\xE3\x8E8\xE3\x8E8\xE3\x8E8\xE3\x8D\xAA\xAA\xA8\x8C\t\x08\x08d\x01\0\0\x03\xD0\x19\x7F\xD3Wq\x19=\x94\x91\x8A\x9C\xA3L\xCB\xB7\xB6@\xDD\x86\xCD@\x95B\xF8H}\x9F\xE6\xB7Ex\x1E\xB4\x9Bd\x01\0\0\x03\xD0\x19\x80\x8A\x7F\xED\xAD\xC6\xF6C\x83\xDC\x1D\xF7\xC4\xB2\xD5\x1BT\"T\x06\xD3kd\x1F^A\xBB\xC5*Va*\x8Cm\x14\t\x86\x08\x08`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x81``\x82\x01Rd\x01\0\0\x03\xD2\x19`\x80\x82\x01Rd\x01\0\0\x03\xD0\x19`\xA0\x82\x01R` \x81`\xC0\x83`\x05_\x19\xFAa4QW__\xFD[\x80Q\x91Pd\x01\0\0\x03\xD0\x19\x82\x84\t\x96Pd\x01\0\0\x03\xD0\x19\x80\x7FK\xDA\x12\xF6\x84\xBD\xA1/hK\xDA\x12\xF6\x84\xBD\xA1/hK\xDA\x12\xF6\x84\xBD\xA1/hK\x8E8\xE2<d\x01\0\0\x03\xD0\x19\x8C\x7F\xC7^\x0C2\xD5\xCB|\x0F\xA9\xD0\xA5K\x12\xA0\xA6\xD5dz\xB0F\xD6\x86\xDAo\xDF\xFC\x90\xFC \x1Dq\xA3\t\x08d\x01\0\0\x03\xD0\x19\x80\x88\x7F)\xA6\x19F\x91\xF9\x1AsqR\t\xEFe\x12\xE5vr(0\xA2\x01\xBE \x18\xA7e\xE8Z\x9E\xCE\xE91\td\x01\0\0\x03\xD0\x19\x88\x7F/hK\xDA\x12\xF6\x84\xBD\xA1/hK\xDA\x12\xF6\x84\xBD\xA1/hK\xDA\x12\xF6\x84\xBD\xA1/8\xE3\x8D\x84\t\x08\x08\x92Pd\x01\0\0\x03\xD0\x19\x80d\x01\0\0\x06\xC4\x19d\x01\0\0\x03\xD0\x19\x8C\x7Fz\x06SK\xB8\xBD\xB4\x9F\xD5\xE9\xE6c'\"\xC2\x98\x94g\xC1\xBF\xC8\xE8\xD9x\xDF\xB4%\xD2h\\%s\t\x08d\x01\0\0\x03\xD0\x19\x80\x88\x7Fd\x84\xAAqeE\xCA,\xF3\xA7\x0C?\xA8\xFE3~\n=!\x16/\rb\x99\xA7\xBF\x81\x92\xBF\xD2\xA7o\t\x87\x08\x08\x94P`@Q\x90P` \x81R` \x80\x82\x01R` `@\x82\x01R\x84``\x82\x01Rd\x01\0\0\x03\xD2\x19`\x80\x82\x01Rd\x01\0\0\x03\xD0\x19`\xA0\x82\x01R` \x81`\xC0\x83`\x05_\x19\xFAa5\xBFW__\xFD[Q\x93Pd\x01\0\0\x03\xD0\x19\x90P\x83\x81\x83\x89\t\t\x93PPPP\x92P\x92\x90PV[__`\xFF\x83Q\x11\x15a5\xEDW__\xFD[_`@Q_[`\x88\x81\x10\x15a6\tW_\x82\x82\x01R` \x01a5\xF3V[P`\x88` _[\x88Q\x81\x10\x15a61W\x88\x82\x01Q\x84\x84\x01R` \x92\x83\x01\x92\x91\x82\x01\x91\x01a6\x10V[PP`\x89\x87Q\x01\x90P`0\x81\x83\x01S`\x02\x01` _[\x87Q\x81\x10\x15a6hW\x87\x82\x01Q\x84\x84\x01R` \x92\x83\x01\x92\x91\x82\x01\x91\x01a6GV[PP`\x8B\x86Q\x88Q\x01\x01\x90P\x85Q\x81\x83\x01SP\x85Q\x85Q\x01`\x8C\x01\x81 \x91PP`@Q\x81\x81R`\x01` \x82\x01S`!` _[\x87Q\x81\x10\x15a6\xBCW\x87\x82\x01Q\x84\x84\x01R` \x92\x83\x01\x92\x91\x82\x01\x91\x01a6\x9BV[PPP\x84Q\x85Q`!\x01\x82\x01S\x84Q`\"\x01\x81 \x93P\x83\x82\x18\x81R`\x02` \x82\x01S\x84Q`\"\x01\x81 \x92PPP\x92P\x92\x90PV[_\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a7)WP_\x91P`\x03\x90P\x82a7\xAEV[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a7zW=__>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a7\xA5WP_\x92P`\x01\x91P\x82\x90Pa7\xAEV[\x92P_\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[___`\xFF\x84Q\x11\x15a7\xC9W__\xFD[_`@Q_[`\x88\x81\x10\x15a7\xE5W_\x82\x82\x01R` \x01a7\xCFV[P`\x88` _[\x89Q\x81\x10\x15a8\rW\x89\x82\x01Q\x84\x84\x01R` \x92\x83\x01\x92\x91\x82\x01\x91\x01a7\xECV[PP`\x89\x88Q\x01\x90P``\x81\x83\x01S`\x02\x01` _[\x88Q\x81\x10\x15a8DW\x88\x82\x01Q\x84\x84\x01R` \x92\x83\x01\x92\x91\x82\x01\x91\x01a8#V[PP`\x8B\x87Q\x89Q\x01\x01\x90P\x86Q\x81\x83\x01SP\x86Q\x86Q\x01`\x8C\x01\x81 \x91PP`@Q\x81\x81R`\x01` \x82\x01S`!` _[\x88Q\x81\x10\x15a8\x98W\x88\x82\x01Q\x84\x84\x01R` \x92\x83\x01\x92\x91\x82\x01\x91\x01a8wV[PPP\x85Q\x86Q`!\x01\x82\x01S\x85Q`\"\x01\x81 \x94P\x84\x82\x18\x81R`\x02` \x82\x01S\x85Q`\"\x01\x81 \x93P\x83\x82\x18\x81R`\x03` \x82\x01S\x85Q`\"\x01\x81 \x92PPP\x92P\x92P\x92V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0B\x02W__\xFD[__\x83`\x1F\x84\x01\x12a9\x05W__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a9\x1CW__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a+\xE5W__\xFD[________`\xC0\x89\x8B\x03\x12\x15a9JW__\xFD[\x885a9U\x81a8\xE1V[\x97P` \x89\x015a9e\x81a8\xE1V[\x96P`@\x89\x015a9u\x81a8\xE1V[\x95P``\x89\x015\x94P`\x80\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a9\x97W__\xFD[a9\xA3\x8B\x82\x8C\x01a8\xF5V[\x90\x95P\x93PP`\xA0\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a9\xC2W__\xFD[a9\xCE\x8B\x82\x8C\x01a8\xF5V[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a9\xF8W__\xFD[\x91\x90PV[___``\x84\x86\x03\x12\x15a:\x0FW__\xFD[\x835a:\x1A\x81a8\xE1V[\x92P` \x84\x015a:*\x81a8\xE1V[\x91Pa:8`@\x85\x01a9\xE2V[\x90P\x92P\x92P\x92V[_` \x82\x84\x03\x12\x15a:QW__\xFD[\x815a\x0F\x92\x81a8\xE1V[_a\x01\0\x82\x84\x03\x12\x15a:mW__\xFD[P\x91\x90PV[___a\x02 \x84\x86\x03\x12\x15a:\x86W__\xFD[\x835a:\x91\x81a8\xE1V[\x92Pa:\xA0\x85` \x86\x01a:\\V[\x91Pa:8\x85a\x01 \x86\x01a:\\V[__`@\x83\x85\x03\x12\x15a:\xC1W__\xFD[\x825a:\xCC\x81a8\xE1V[\x91P` \x83\x015a:\xDC\x81a8\xE1V[\x80\x91PP\x92P\x92\x90PV[__a\x02\0\x83\x85\x03\x12\x15a:\xF9W__\xFD[a;\x03\x84\x84a:\\V[\x91Pa;\x13\x84a\x01\0\x85\x01a:\\V[\x90P\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a;,W__\xFD[P5\x91\x90PV[___a\x02 \x84\x86\x03\x12\x15a;FW__\xFD[\x835\x92Pa:\xA0\x85` \x86\x01a:\\V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x01`\x01``\x1B\x03\x86\x16\x81Re\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16` \x82\x01Rc\xFF\xFF\xFF\xFF\x84\x16`@\x82\x01Rb\xFF\xFF\xFF\x83\x16``\x82\x01R`\xA0\x81\x01`\x03\x83\x10a;\xBCWcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x82`\x80\x83\x01R\x96\x95PPPPPPV[_a\x01\0\x82\x84\x03\x12\x15a;\xDDW__\xFD[a\x0F\x92\x83\x83a:\\V[__` \x83\x85\x03\x12\x15a;\xF8W__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a<\x0EW__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a<\x1EW__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a<4W__\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a<HW__\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a<\xDDW`?\x19\x87\x86\x03\x01\x84Ra<\xC8\x85\x83Qa<XV[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a<\xACV[P\x92\x96\x95PPPPPPV[` \x81R_a\x0F\x92` \x83\x01\x84a<XV[__`@\x83\x85\x03\x12\x15a=\x0CW__\xFD[\x825a=\x17\x81a8\xE1V[\x91Pa;\x13` \x84\x01a9\xE2V[_` \x82\x84\x03\x12\x15a=5W__\xFD[\x81Qa\x0F\x92\x81a8\xE1V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0F\x04Wa\x0F\x04a=@V[_` \x82\x84\x03\x12\x15a=wW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x0F\x92W__\xFD[_` \x82\x84\x03\x12\x15a=\x96W__\xFD[\x815f\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0F\x92W__\xFD[_` \x82\x84\x03\x12\x15a=\xBCW__\xFD[\x815b\xFF\xFF\xFF\x81\x16\x81\x14a\x0F\x92W__\xFD[_` \x82\x84\x03\x12\x15a=\xDEW__\xFD[\x815e\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0F\x92W__\xFD[_` \x82\x84\x03\x12\x15a>\x03W__\xFD[a\x0F\x92\x82a9\xE2V[\x81\x81\x03\x81\x81\x11\x15a\x0F\x04Wa\x0F\x04a=@V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[__\x835`\x1E\x19\x846\x03\x01\x81\x12a>\\W__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a>vW__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a+\xE5W__\xFD[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[\x82\x84\x827_\x83\x82\x01_\x81Ra>\xB6\x81\x85a>\x8AV[\x96\x95PPPPPPV[`\x01`\x01``\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0F\x04Wa\x0F\x04a=@V[b\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0F\x04Wa\x0F\x04a=@V[` \x80\x82R`0\x90\x82\x01R\x7Fchannel must have state OPEN or `@\x82\x01RoPENDING_TO_CLOSE`\x80\x1B``\x82\x01R`\x80\x01\x90V[_a\x01\0\x82\x84\x03\x12\x80\x15a?\\W__\xFD[P`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a?\x8DWcNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x90\x81R\x835\x82R` \x80\x85\x015\x90\x83\x01R\x83\x81\x015\x90\x82\x01R``\x80\x84\x015\x90\x82\x01R`\x80\x80\x84\x015\x90\x82\x01R`\xA0\x80\x84\x015\x90\x82\x01R`\xC0\x80\x84\x015\x90\x82\x01R`\xE0\x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[e\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0F\x04Wa\x0F\x04a=@V[`\x01`\x01``\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x0F\x04Wa\x0F\x04a=@V[c\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0F\x04Wa\x0F\x04a=@V[_a\x0F\x92\x82\x84a>\x8AV[_\x82a@aWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x06\x90V\xFE\xEC!T|\xA1\xE2.\xDC?\x9BON\r\xA9F8\xB5\xB9M\xCB\x18\xD5-\xCC\x07*\xBEX\x01\xA8\x01<\x19{\xBAhJj\xFB{\xA2O\x1D&V\x05AK\x1D\0Q\xA6\xE2\x95\xD6\xFF{nx\xE8p\xD7\xA7\xF0\xA2dipfsX\"\x12 [\x87\xF2\x9E\x9FZ\x1E\xFF\x81L\x1C\"t\xC2v\xEE\x1D-V\xF5A\xE2\xF2\x17\xD5l/i\xE2\x03\x0EudsolcC\0\x08\x1E\x003",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct RedeemableTicket { TicketData data; HoprCrypto.CompactSignature signature; uint256 porSecret; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RedeemableTicket {
        #[allow(missing_docs)]
        pub data: <TicketData as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub signature: <HoprCrypto::CompactSignature as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub porSecret: alloy::sol_types::private::primitives::aliases::U256,
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
            TicketData,
            HoprCrypto::CompactSignature,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <TicketData as alloy::sol_types::SolType>::RustType,
            <HoprCrypto::CompactSignature as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<RedeemableTicket> for UnderlyingRustTuple<'_> {
            fn from(value: RedeemableTicket) -> Self {
                (value.data, value.signature, value.porSecret)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RedeemableTicket {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    data: tuple.0,
                    signature: tuple.1,
                    porSecret: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for RedeemableTicket {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for RedeemableTicket {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <TicketData as alloy_sol_types::SolType>::tokenize(&self.data),
                    <HoprCrypto::CompactSignature as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.porSecret),
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
        impl alloy_sol_types::SolType for RedeemableTicket {
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
        impl alloy_sol_types::SolStruct for RedeemableTicket {
            const NAME: &'static str = "RedeemableTicket";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "RedeemableTicket(TicketData data,CompactSignature signature,uint256 porSecret)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(2);
                components
                    .push(
                        <TicketData as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <TicketData as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(
                        <HoprCrypto::CompactSignature as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <HoprCrypto::CompactSignature as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <TicketData as alloy_sol_types::SolType>::eip712_data_word(
                            &self.data,
                        )
                        .0,
                    <HoprCrypto::CompactSignature as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signature,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.porSecret)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for RedeemableTicket {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <TicketData as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.data,
                    )
                    + <HoprCrypto::CompactSignature as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signature,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.porSecret,
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
                <TicketData as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.data,
                    out,
                );
                <HoprCrypto::CompactSignature as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signature,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.porSecret,
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
    /**```solidity
struct TicketData { bytes32 channelId; HoprChannelsType.Balance amount; HoprChannelsType.TicketIndex ticketIndex; HoprChannelsType.ChannelEpoch epoch; HoprChannelsType.WinProb winProb; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TicketData {
        #[allow(missing_docs)]
        pub channelId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub amount: <HoprChannelsType::Balance as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub ticketIndex: <HoprChannelsType::TicketIndex as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub epoch: <HoprChannelsType::ChannelEpoch as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub winProb: <HoprChannelsType::WinProb as alloy::sol_types::SolType>::RustType,
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
            HoprChannelsType::Balance,
            HoprChannelsType::TicketIndex,
            HoprChannelsType::ChannelEpoch,
            HoprChannelsType::WinProb,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            <HoprChannelsType::Balance as alloy::sol_types::SolType>::RustType,
            <HoprChannelsType::TicketIndex as alloy::sol_types::SolType>::RustType,
            <HoprChannelsType::ChannelEpoch as alloy::sol_types::SolType>::RustType,
            <HoprChannelsType::WinProb as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<TicketData> for UnderlyingRustTuple<'_> {
            fn from(value: TicketData) -> Self {
                (
                    value.channelId,
                    value.amount,
                    value.ticketIndex,
                    value.epoch,
                    value.winProb,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TicketData {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    channelId: tuple.0,
                    amount: tuple.1,
                    ticketIndex: tuple.2,
                    epoch: tuple.3,
                    winProb: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for TicketData {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for TicketData {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.channelId),
                    <HoprChannelsType::Balance as alloy_sol_types::SolType>::tokenize(
                        &self.amount,
                    ),
                    <HoprChannelsType::TicketIndex as alloy_sol_types::SolType>::tokenize(
                        &self.ticketIndex,
                    ),
                    <HoprChannelsType::ChannelEpoch as alloy_sol_types::SolType>::tokenize(
                        &self.epoch,
                    ),
                    <HoprChannelsType::WinProb as alloy_sol_types::SolType>::tokenize(
                        &self.winProb,
                    ),
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
        impl alloy_sol_types::SolType for TicketData {
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
        impl alloy_sol_types::SolStruct for TicketData {
            const NAME: &'static str = "TicketData";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "TicketData(bytes32 channelId,uint96 amount,uint48 ticketIndex,uint24 epoch,uint56 winProb)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.channelId)
                        .0,
                    <HoprChannelsType::Balance as alloy_sol_types::SolType>::eip712_data_word(
                            &self.amount,
                        )
                        .0,
                    <HoprChannelsType::TicketIndex as alloy_sol_types::SolType>::eip712_data_word(
                            &self.ticketIndex,
                        )
                        .0,
                    <HoprChannelsType::ChannelEpoch as alloy_sol_types::SolType>::eip712_data_word(
                            &self.epoch,
                        )
                        .0,
                    <HoprChannelsType::WinProb as alloy_sol_types::SolType>::eip712_data_word(
                            &self.winProb,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for TicketData {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.channelId,
                    )
                    + <HoprChannelsType::Balance as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amount,
                    )
                    + <HoprChannelsType::TicketIndex as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.ticketIndex,
                    )
                    + <HoprChannelsType::ChannelEpoch as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.epoch,
                    )
                    + <HoprChannelsType::WinProb as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.winProb,
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
                    &rust.channelId,
                    out,
                );
                <HoprChannelsType::Balance as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amount,
                    out,
                );
                <HoprChannelsType::TicketIndex as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.ticketIndex,
                    out,
                );
                <HoprChannelsType::ChannelEpoch as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.epoch,
                    out,
                );
                <HoprChannelsType::WinProb as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.winProb,
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
    /**Custom error with signature `BalanceExceedsGlobalPerChannelAllowance()` and selector `0xa4f3bbe4`.
```solidity
error BalanceExceedsGlobalPerChannelAllowance();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BalanceExceedsGlobalPerChannelAllowance;
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
        impl ::core::convert::From<BalanceExceedsGlobalPerChannelAllowance>
        for UnderlyingRustTuple<'_> {
            fn from(value: BalanceExceedsGlobalPerChannelAllowance) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for BalanceExceedsGlobalPerChannelAllowance {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BalanceExceedsGlobalPerChannelAllowance {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BalanceExceedsGlobalPerChannelAllowance()";
            const SELECTOR: [u8; 4] = [164u8, 243u8, 187u8, 228u8];
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
    /**Custom error with signature `ECDSAInvalidSignature()` and selector `0xf645eedf`.
```solidity
error ECDSAInvalidSignature();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAInvalidSignature;
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
        impl ::core::convert::From<ECDSAInvalidSignature> for UnderlyingRustTuple<'_> {
            fn from(value: ECDSAInvalidSignature) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECDSAInvalidSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECDSAInvalidSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECDSAInvalidSignature()";
            const SELECTOR: [u8; 4] = [246u8, 69u8, 238u8, 223u8];
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
    /**Custom error with signature `ECDSAInvalidSignatureLength(uint256)` and selector `0xfce698f7`.
```solidity
error ECDSAInvalidSignatureLength(uint256 length);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAInvalidSignatureLength {
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
        impl ::core::convert::From<ECDSAInvalidSignatureLength>
        for UnderlyingRustTuple<'_> {
            fn from(value: ECDSAInvalidSignatureLength) -> Self {
                (value.length,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ECDSAInvalidSignatureLength {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { length: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECDSAInvalidSignatureLength {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECDSAInvalidSignatureLength(uint256)";
            const SELECTOR: [u8; 4] = [252u8, 230u8, 152u8, 247u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.length),
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
    /**Custom error with signature `ECDSAInvalidSignatureS(bytes32)` and selector `0xd78bce0c`.
```solidity
error ECDSAInvalidSignatureS(bytes32 s);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAInvalidSignatureS {
        #[allow(missing_docs)]
        pub s: alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<ECDSAInvalidSignatureS> for UnderlyingRustTuple<'_> {
            fn from(value: ECDSAInvalidSignatureS) -> Self {
                (value.s,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECDSAInvalidSignatureS {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { s: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECDSAInvalidSignatureS {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECDSAInvalidSignatureS(bytes32)";
            const SELECTOR: [u8; 4] = [215u8, 139u8, 206u8, 12u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.s),
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
    /**Custom error with signature `InsufficientChannelBalance()` and selector `0xb147636c`.
```solidity
error InsufficientChannelBalance();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientChannelBalance;
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
        impl ::core::convert::From<InsufficientChannelBalance>
        for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientChannelBalance) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InsufficientChannelBalance {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientChannelBalance {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientChannelBalance()";
            const SELECTOR: [u8; 4] = [177u8, 71u8, 99u8, 108u8];
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
    /**Custom error with signature `InvalidBalance()` and selector `0xc52e3eff`.
```solidity
error InvalidBalance();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidBalance;
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
        impl ::core::convert::From<InvalidBalance> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidBalance) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidBalance {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidBalance {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidBalance()";
            const SELECTOR: [u8; 4] = [197u8, 46u8, 62u8, 255u8];
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
    /**Custom error with signature `InvalidCurvePoint()` and selector `0x72454a82`.
```solidity
error InvalidCurvePoint();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidCurvePoint;
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
        impl ::core::convert::From<InvalidCurvePoint> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidCurvePoint) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidCurvePoint {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidCurvePoint {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidCurvePoint()";
            const SELECTOR: [u8; 4] = [114u8, 69u8, 74u8, 130u8];
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
    /**Custom error with signature `InvalidFieldElement()` and selector `0x3ae4ed6b`.
```solidity
error InvalidFieldElement();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidFieldElement;
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
        impl ::core::convert::From<InvalidFieldElement> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidFieldElement) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidFieldElement {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidFieldElement {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidFieldElement()";
            const SELECTOR: [u8; 4] = [58u8, 228u8, 237u8, 107u8];
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
    /**Custom error with signature `InvalidNoticePeriod()` and selector `0xf9ee9107`.
```solidity
error InvalidNoticePeriod();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidNoticePeriod;
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
        impl ::core::convert::From<InvalidNoticePeriod> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidNoticePeriod) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidNoticePeriod {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidNoticePeriod {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidNoticePeriod()";
            const SELECTOR: [u8; 4] = [249u8, 238u8, 145u8, 7u8];
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
    /**Custom error with signature `InvalidPointWitness()` and selector `0xedfdcd98`.
```solidity
error InvalidPointWitness();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidPointWitness;
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
        impl ::core::convert::From<InvalidPointWitness> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidPointWitness) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidPointWitness {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidPointWitness {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidPointWitness()";
            const SELECTOR: [u8; 4] = [237u8, 253u8, 205u8, 152u8];
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
    /**Custom error with signature `InvalidTicketIndex()` and selector `0x31de4847`.
```solidity
error InvalidTicketIndex();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidTicketIndex;
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
        impl ::core::convert::From<InvalidTicketIndex> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidTicketIndex) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidTicketIndex {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidTicketIndex {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidTicketIndex()";
            const SELECTOR: [u8; 4] = [49u8, 222u8, 72u8, 71u8];
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
    /**Custom error with signature `InvalidTicketSignature()` and selector `0xcddd5356`.
```solidity
error InvalidTicketSignature();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidTicketSignature;
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
        impl ::core::convert::From<InvalidTicketSignature> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidTicketSignature) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidTicketSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidTicketSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidTicketSignature()";
            const SELECTOR: [u8; 4] = [205u8, 221u8, 83u8, 86u8];
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
    /**Custom error with signature `InvalidVRFProof()` and selector `0x95fdbdb8`.
```solidity
error InvalidVRFProof();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidVRFProof;
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
        impl ::core::convert::From<InvalidVRFProof> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidVRFProof) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidVRFProof {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidVRFProof {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidVRFProof()";
            const SELECTOR: [u8; 4] = [149u8, 253u8, 189u8, 184u8];
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
    /**Custom error with signature `NoticePeriodNotDue()` and selector `0x7164032a`.
```solidity
error NoticePeriodNotDue();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NoticePeriodNotDue;
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
        impl ::core::convert::From<NoticePeriodNotDue> for UnderlyingRustTuple<'_> {
            fn from(value: NoticePeriodNotDue) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NoticePeriodNotDue {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NoticePeriodNotDue {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NoticePeriodNotDue()";
            const SELECTOR: [u8; 4] = [113u8, 100u8, 3u8, 42u8];
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
    /**Custom error with signature `SourceEqualsDestination()` and selector `0x97a3aed2`.
```solidity
error SourceEqualsDestination();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SourceEqualsDestination;
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
        impl ::core::convert::From<SourceEqualsDestination> for UnderlyingRustTuple<'_> {
            fn from(value: SourceEqualsDestination) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SourceEqualsDestination {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SourceEqualsDestination {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SourceEqualsDestination()";
            const SELECTOR: [u8; 4] = [151u8, 163u8, 174u8, 210u8];
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
    /**Custom error with signature `TicketIsNotAWin()` and selector `0xee835c89`.
```solidity
error TicketIsNotAWin();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TicketIsNotAWin;
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
        impl ::core::convert::From<TicketIsNotAWin> for UnderlyingRustTuple<'_> {
            fn from(value: TicketIsNotAWin) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TicketIsNotAWin {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TicketIsNotAWin {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TicketIsNotAWin()";
            const SELECTOR: [u8; 4] = [238u8, 131u8, 92u8, 137u8];
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
    /**Custom error with signature `TokenTransferFailed()` and selector `0x045c4b02`.
```solidity
error TokenTransferFailed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TokenTransferFailed;
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
        impl ::core::convert::From<TokenTransferFailed> for UnderlyingRustTuple<'_> {
            fn from(value: TokenTransferFailed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TokenTransferFailed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TokenTransferFailed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TokenTransferFailed()";
            const SELECTOR: [u8; 4] = [4u8, 92u8, 75u8, 2u8];
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
    /**Custom error with signature `WrongChannelState(string)` and selector `0x499463c1`.
```solidity
error WrongChannelState(string reason);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WrongChannelState {
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
        impl ::core::convert::From<WrongChannelState> for UnderlyingRustTuple<'_> {
            fn from(value: WrongChannelState) -> Self {
                (value.reason,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for WrongChannelState {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { reason: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for WrongChannelState {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WrongChannelState(string)";
            const SELECTOR: [u8; 4] = [73u8, 148u8, 99u8, 193u8];
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
    /**Event with signature `ChannelBalanceDecreased(bytes32,bytes32)` and selector `0x0c4672f14b63bb6354fac475ee498055a2784455673af224717b9770fd68d8d1`.
```solidity
event ChannelBalanceDecreased(bytes32 indexed channelId, bytes32 channel);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ChannelBalanceDecreased {
        #[allow(missing_docs)]
        pub channelId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub channel: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for ChannelBalanceDecreased {
            type DataTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "ChannelBalanceDecreased(bytes32,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                12u8, 70u8, 114u8, 241u8, 75u8, 99u8, 187u8, 99u8, 84u8, 250u8, 196u8,
                117u8, 238u8, 73u8, 128u8, 85u8, 162u8, 120u8, 68u8, 85u8, 103u8, 58u8,
                242u8, 36u8, 113u8, 123u8, 151u8, 112u8, 253u8, 104u8, 216u8, 209u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    channelId: topics.1,
                    channel: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.channel),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.channelId.clone())
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.channelId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ChannelBalanceDecreased {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ChannelBalanceDecreased> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &ChannelBalanceDecreased,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ChannelBalanceIncreased(bytes32,bytes32)` and selector `0xec21547ca1e22edc3f9b4f4e0da94638b5b94dcb18d52dcc072abe5801a8013c`.
```solidity
event ChannelBalanceIncreased(bytes32 indexed channelId, bytes32 channel);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ChannelBalanceIncreased {
        #[allow(missing_docs)]
        pub channelId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub channel: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for ChannelBalanceIncreased {
            type DataTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "ChannelBalanceIncreased(bytes32,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                236u8, 33u8, 84u8, 124u8, 161u8, 226u8, 46u8, 220u8, 63u8, 155u8, 79u8,
                78u8, 13u8, 169u8, 70u8, 56u8, 181u8, 185u8, 77u8, 203u8, 24u8, 213u8,
                45u8, 204u8, 7u8, 42u8, 190u8, 88u8, 1u8, 168u8, 1u8, 60u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    channelId: topics.1,
                    channel: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.channel),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.channelId.clone())
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.channelId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ChannelBalanceIncreased {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ChannelBalanceIncreased> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &ChannelBalanceIncreased,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ChannelClosed(bytes32,bytes32)` and selector `0x197bba684a6afb7ba24f1d265605414b1d0051a6e295d6ff7b6e78e870d7a7f0`.
```solidity
event ChannelClosed(bytes32 indexed channelId, bytes32 channel);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ChannelClosed {
        #[allow(missing_docs)]
        pub channelId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub channel: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for ChannelClosed {
            type DataTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "ChannelClosed(bytes32,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                25u8, 123u8, 186u8, 104u8, 74u8, 106u8, 251u8, 123u8, 162u8, 79u8, 29u8,
                38u8, 86u8, 5u8, 65u8, 75u8, 29u8, 0u8, 81u8, 166u8, 226u8, 149u8, 214u8,
                255u8, 123u8, 110u8, 120u8, 232u8, 112u8, 215u8, 167u8, 240u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    channelId: topics.1,
                    channel: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.channel),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.channelId.clone())
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.channelId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ChannelClosed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ChannelClosed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ChannelClosed) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ChannelOpened(bytes32,address,address,bytes32)` and selector `0x004c0177ad15bb302b4419e9ac96065d6f436e0f61972f97bbe272834c40f934`.
```solidity
event ChannelOpened(bytes32 indexed channelId, address indexed source, address indexed destination, bytes32 channel);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ChannelOpened {
        #[allow(missing_docs)]
        pub channelId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub source: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub destination: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub channel: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for ChannelOpened {
            type DataTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ChannelOpened(bytes32,address,address,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                0u8, 76u8, 1u8, 119u8, 173u8, 21u8, 187u8, 48u8, 43u8, 68u8, 25u8, 233u8,
                172u8, 150u8, 6u8, 93u8, 111u8, 67u8, 110u8, 15u8, 97u8, 151u8, 47u8,
                151u8, 187u8, 226u8, 114u8, 131u8, 76u8, 64u8, 249u8, 52u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    channelId: topics.1,
                    source: topics.2,
                    destination: topics.3,
                    channel: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.channel),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.channelId.clone(),
                    self.source.clone(),
                    self.destination.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.channelId);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.source,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.destination,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ChannelOpened {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ChannelOpened> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ChannelOpened) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `DomainSeparatorUpdated(bytes32)` and selector `0x771f5240ae5fd8a7640d3fb82fa70aab2fb1dbf35f2ef464f8509946717664c5`.
```solidity
event DomainSeparatorUpdated(bytes32 indexed domainSeparator);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct DomainSeparatorUpdated {
        #[allow(missing_docs)]
        pub domainSeparator: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for DomainSeparatorUpdated {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "DomainSeparatorUpdated(bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                119u8, 31u8, 82u8, 64u8, 174u8, 95u8, 216u8, 167u8, 100u8, 13u8, 63u8,
                184u8, 47u8, 167u8, 10u8, 171u8, 47u8, 177u8, 219u8, 243u8, 95u8, 46u8,
                244u8, 100u8, 248u8, 80u8, 153u8, 70u8, 113u8, 118u8, 100u8, 197u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { domainSeparator: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.domainSeparator.clone())
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.domainSeparator);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for DomainSeparatorUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DomainSeparatorUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &DomainSeparatorUpdated) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `OutgoingChannelClosureInitiated(bytes32,bytes32)` and selector `0x0061e8037197b4d91cf8add2a0736613459081f236d1c8ab58549ebbc330b4e3`.
```solidity
event OutgoingChannelClosureInitiated(bytes32 indexed channelId, bytes32 channel);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OutgoingChannelClosureInitiated {
        #[allow(missing_docs)]
        pub channelId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub channel: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for OutgoingChannelClosureInitiated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "OutgoingChannelClosureInitiated(bytes32,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                0u8, 97u8, 232u8, 3u8, 113u8, 151u8, 180u8, 217u8, 28u8, 248u8, 173u8,
                210u8, 160u8, 115u8, 102u8, 19u8, 69u8, 144u8, 129u8, 242u8, 54u8, 209u8,
                200u8, 171u8, 88u8, 84u8, 158u8, 187u8, 195u8, 48u8, 180u8, 227u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    channelId: topics.1,
                    channel: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.channel),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.channelId.clone())
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.channelId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OutgoingChannelClosureInitiated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OutgoingChannelClosureInitiated>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &OutgoingChannelClosureInitiated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `TicketRedeemed(bytes32,bytes32)` and selector `0xed22f34d154d253a7f6fd477439be59080a1554aa0d0642686d64223ab544c8a`.
```solidity
event TicketRedeemed(bytes32 indexed channelId, bytes32 channel);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TicketRedeemed {
        #[allow(missing_docs)]
        pub channelId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub channel: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for TicketRedeemed {
            type DataTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "TicketRedeemed(bytes32,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                237u8, 34u8, 243u8, 77u8, 21u8, 77u8, 37u8, 58u8, 127u8, 111u8, 212u8,
                119u8, 67u8, 155u8, 229u8, 144u8, 128u8, 161u8, 85u8, 74u8, 160u8, 208u8,
                100u8, 38u8, 134u8, 214u8, 66u8, 35u8, 171u8, 84u8, 76u8, 138u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    channelId: topics.1,
                    channel: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.channel),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.channelId.clone())
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.channelId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TicketRedeemed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TicketRedeemed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TicketRedeemed) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address _token, HoprChannelsType.Timestamp _noticePeriodChannelClosure, address _safeRegistry);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _noticePeriodChannelClosure: <HoprChannelsType::Timestamp as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _safeRegistry: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                HoprChannelsType::Timestamp,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <HoprChannelsType::Timestamp as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (
                        value._token,
                        value._noticePeriodChannelClosure,
                        value._safeRegistry,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _token: tuple.0,
                        _noticePeriodChannelClosure: tuple.1,
                        _safeRegistry: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                HoprChannelsType::Timestamp,
                alloy::sol_types::sol_data::Address,
            );
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
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._token,
                    ),
                    <HoprChannelsType::Timestamp as alloy_sol_types::SolType>::tokenize(
                        &self._noticePeriodChannelClosure,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._safeRegistry,
                    ),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `ERC777_HOOK_FUND_CHANNEL_MULTI_SIZE()` and selector `0x78d8016d`.
```solidity
function ERC777_HOOK_FUND_CHANNEL_MULTI_SIZE() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC777_HOOK_FUND_CHANNEL_MULTI_SIZECall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`ERC777_HOOK_FUND_CHANNEL_MULTI_SIZE()`](ERC777_HOOK_FUND_CHANNEL_MULTI_SIZECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC777_HOOK_FUND_CHANNEL_MULTI_SIZEReturn {
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
            impl ::core::convert::From<ERC777_HOOK_FUND_CHANNEL_MULTI_SIZECall>
            for UnderlyingRustTuple<'_> {
                fn from(value: ERC777_HOOK_FUND_CHANNEL_MULTI_SIZECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ERC777_HOOK_FUND_CHANNEL_MULTI_SIZECall {
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
            impl ::core::convert::From<ERC777_HOOK_FUND_CHANNEL_MULTI_SIZEReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: ERC777_HOOK_FUND_CHANNEL_MULTI_SIZEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ERC777_HOOK_FUND_CHANNEL_MULTI_SIZEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ERC777_HOOK_FUND_CHANNEL_MULTI_SIZECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC777_HOOK_FUND_CHANNEL_MULTI_SIZE()";
            const SELECTOR: [u8; 4] = [120u8, 216u8, 1u8, 109u8];
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
                        let r: ERC777_HOOK_FUND_CHANNEL_MULTI_SIZEReturn = r.into();
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
                        let r: ERC777_HOOK_FUND_CHANNEL_MULTI_SIZEReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `ERC777_HOOK_FUND_CHANNEL_SIZE()` and selector `0x44dae6f8`.
```solidity
function ERC777_HOOK_FUND_CHANNEL_SIZE() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC777_HOOK_FUND_CHANNEL_SIZECall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`ERC777_HOOK_FUND_CHANNEL_SIZE()`](ERC777_HOOK_FUND_CHANNEL_SIZECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC777_HOOK_FUND_CHANNEL_SIZEReturn {
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
            impl ::core::convert::From<ERC777_HOOK_FUND_CHANNEL_SIZECall>
            for UnderlyingRustTuple<'_> {
                fn from(value: ERC777_HOOK_FUND_CHANNEL_SIZECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ERC777_HOOK_FUND_CHANNEL_SIZECall {
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
            impl ::core::convert::From<ERC777_HOOK_FUND_CHANNEL_SIZEReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: ERC777_HOOK_FUND_CHANNEL_SIZEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ERC777_HOOK_FUND_CHANNEL_SIZEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ERC777_HOOK_FUND_CHANNEL_SIZECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC777_HOOK_FUND_CHANNEL_SIZE()";
            const SELECTOR: [u8; 4] = [68u8, 218u8, 230u8, 248u8];
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
                        let r: ERC777_HOOK_FUND_CHANNEL_SIZEReturn = r.into();
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
                        let r: ERC777_HOOK_FUND_CHANNEL_SIZEReturn = r.into();
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
    /**Function with signature `MAX_USED_BALANCE()` and selector `0x5d2f07c5`.
```solidity
function MAX_USED_BALANCE() external view returns (HoprChannelsType.Balance);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_USED_BALANCECall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`MAX_USED_BALANCE()`](MAX_USED_BALANCECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_USED_BALANCEReturn {
        #[allow(missing_docs)]
        pub _0: <HoprChannelsType::Balance as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<MAX_USED_BALANCECall>
            for UnderlyingRustTuple<'_> {
                fn from(value: MAX_USED_BALANCECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MAX_USED_BALANCECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (HoprChannelsType::Balance,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <HoprChannelsType::Balance as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<MAX_USED_BALANCEReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: MAX_USED_BALANCEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MAX_USED_BALANCEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for MAX_USED_BALANCECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <HoprChannelsType::Balance as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (HoprChannelsType::Balance,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MAX_USED_BALANCE()";
            const SELECTOR: [u8; 4] = [93u8, 47u8, 7u8, 197u8];
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
                (<HoprChannelsType::Balance as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: MAX_USED_BALANCEReturn = r.into();
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
                        let r: MAX_USED_BALANCEReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `MIN_USED_BALANCE()` and selector `0x29392e32`.
```solidity
function MIN_USED_BALANCE() external view returns (HoprChannelsType.Balance);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MIN_USED_BALANCECall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`MIN_USED_BALANCE()`](MIN_USED_BALANCECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MIN_USED_BALANCEReturn {
        #[allow(missing_docs)]
        pub _0: <HoprChannelsType::Balance as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<MIN_USED_BALANCECall>
            for UnderlyingRustTuple<'_> {
                fn from(value: MIN_USED_BALANCECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MIN_USED_BALANCECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (HoprChannelsType::Balance,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <HoprChannelsType::Balance as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<MIN_USED_BALANCEReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: MIN_USED_BALANCEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MIN_USED_BALANCEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for MIN_USED_BALANCECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <HoprChannelsType::Balance as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (HoprChannelsType::Balance,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MIN_USED_BALANCE()";
            const SELECTOR: [u8; 4] = [41u8, 57u8, 46u8, 50u8];
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
                (<HoprChannelsType::Balance as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: MIN_USED_BALANCEReturn = r.into();
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
                        let r: MIN_USED_BALANCEReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `NOTICE_PERIOD_CHANNEL_CLOSURE()` and selector `0xd94cc1b4`.
```solidity
function NOTICE_PERIOD_CHANNEL_CLOSURE() external view returns (HoprChannelsType.Timestamp);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NOTICE_PERIOD_CHANNEL_CLOSURECall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`NOTICE_PERIOD_CHANNEL_CLOSURE()`](NOTICE_PERIOD_CHANNEL_CLOSURECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NOTICE_PERIOD_CHANNEL_CLOSUREReturn {
        #[allow(missing_docs)]
        pub _0: <HoprChannelsType::Timestamp as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<NOTICE_PERIOD_CHANNEL_CLOSURECall>
            for UnderlyingRustTuple<'_> {
                fn from(value: NOTICE_PERIOD_CHANNEL_CLOSURECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for NOTICE_PERIOD_CHANNEL_CLOSURECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (HoprChannelsType::Timestamp,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <HoprChannelsType::Timestamp as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<NOTICE_PERIOD_CHANNEL_CLOSUREReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: NOTICE_PERIOD_CHANNEL_CLOSUREReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for NOTICE_PERIOD_CHANNEL_CLOSUREReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for NOTICE_PERIOD_CHANNEL_CLOSURECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <HoprChannelsType::Timestamp as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (HoprChannelsType::Timestamp,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NOTICE_PERIOD_CHANNEL_CLOSURE()";
            const SELECTOR: [u8; 4] = [217u8, 76u8, 193u8, 180u8];
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
                    <HoprChannelsType::Timestamp as alloy_sol_types::SolType>::tokenize(
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
                        let r: NOTICE_PERIOD_CHANNEL_CLOSUREReturn = r.into();
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
                        let r: NOTICE_PERIOD_CHANNEL_CLOSUREReturn = r.into();
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
    /**Function with signature `_currentBlockTimestamp()` and selector `0xb920deed`.
```solidity
function _currentBlockTimestamp() external view returns (HoprChannelsType.Timestamp);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct _currentBlockTimestampCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`_currentBlockTimestamp()`](_currentBlockTimestampCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct _currentBlockTimestampReturn {
        #[allow(missing_docs)]
        pub _0: <HoprChannelsType::Timestamp as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<_currentBlockTimestampCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: _currentBlockTimestampCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for _currentBlockTimestampCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (HoprChannelsType::Timestamp,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <HoprChannelsType::Timestamp as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<_currentBlockTimestampReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: _currentBlockTimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for _currentBlockTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for _currentBlockTimestampCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <HoprChannelsType::Timestamp as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (HoprChannelsType::Timestamp,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "_currentBlockTimestamp()";
            const SELECTOR: [u8; 4] = [185u8, 32u8, 222u8, 237u8];
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
                    <HoprChannelsType::Timestamp as alloy_sol_types::SolType>::tokenize(
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
                        let r: _currentBlockTimestampReturn = r.into();
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
                        let r: _currentBlockTimestampReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `_getChannelId(address,address)` and selector `0xbe9babdc`.
```solidity
function _getChannelId(address source, address destination) external pure returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct _getChannelIdCall {
        #[allow(missing_docs)]
        pub source: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub destination: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`_getChannelId(address,address)`](_getChannelIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct _getChannelIdReturn {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<_getChannelIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: _getChannelIdCall) -> Self {
                    (value.source, value.destination)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for _getChannelIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        source: tuple.0,
                        destination: tuple.1,
                    }
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
            impl ::core::convert::From<_getChannelIdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: _getChannelIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for _getChannelIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for _getChannelIdCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "_getChannelId(address,address)";
            const SELECTOR: [u8; 4] = [190u8, 155u8, 171u8, 220u8];
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
                        &self.source,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.destination,
                    ),
                )
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
                        let r: _getChannelIdReturn = r.into();
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
                        let r: _getChannelIdReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `_getTicketHash(((bytes32,uint96,uint48,uint24,uint56),(bytes32,bytes32),uint256))` and selector `0xab66ccab`.
```solidity
function _getTicketHash(RedeemableTicket memory redeemable) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct _getTicketHashCall {
        #[allow(missing_docs)]
        pub redeemable: <RedeemableTicket as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`_getTicketHash(((bytes32,uint96,uint48,uint24,uint56),(bytes32,bytes32),uint256))`](_getTicketHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct _getTicketHashReturn {
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
            type UnderlyingSolTuple<'a> = (RedeemableTicket,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <RedeemableTicket as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<_getTicketHashCall> for UnderlyingRustTuple<'_> {
                fn from(value: _getTicketHashCall) -> Self {
                    (value.redeemable,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for _getTicketHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { redeemable: tuple.0 }
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
            impl ::core::convert::From<_getTicketHashReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: _getTicketHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for _getTicketHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for _getTicketHashCall {
            type Parameters<'a> = (RedeemableTicket,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "_getTicketHash(((bytes32,uint96,uint48,uint24,uint56),(bytes32,bytes32),uint256))";
            const SELECTOR: [u8; 4] = [171u8, 102u8, 204u8, 171u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <RedeemableTicket as alloy_sol_types::SolType>::tokenize(
                        &self.redeemable,
                    ),
                )
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
                        let r: _getTicketHashReturn = r.into();
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
                        let r: _getTicketHashReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `_isWinningTicket(bytes32,((bytes32,uint96,uint48,uint24,uint56),(bytes32,bytes32),uint256),(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256))` and selector `0x78d149a8`.
```solidity
function _isWinningTicket(bytes32 ticketHash, RedeemableTicket memory redeemable, HoprCrypto.VRFParameters memory params) external pure returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct _isWinningTicketCall {
        #[allow(missing_docs)]
        pub ticketHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub redeemable: <RedeemableTicket as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub params: <HoprCrypto::VRFParameters as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`_isWinningTicket(bytes32,((bytes32,uint96,uint48,uint24,uint56),(bytes32,bytes32),uint256),(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256))`](_isWinningTicketCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct _isWinningTicketReturn {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                RedeemableTicket,
                HoprCrypto::VRFParameters,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                <RedeemableTicket as alloy::sol_types::SolType>::RustType,
                <HoprCrypto::VRFParameters as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<_isWinningTicketCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: _isWinningTicketCall) -> Self {
                    (value.ticketHash, value.redeemable, value.params)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for _isWinningTicketCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        ticketHash: tuple.0,
                        redeemable: tuple.1,
                        params: tuple.2,
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
            impl ::core::convert::From<_isWinningTicketReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: _isWinningTicketReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for _isWinningTicketReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for _isWinningTicketCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                RedeemableTicket,
                HoprCrypto::VRFParameters,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "_isWinningTicket(bytes32,((bytes32,uint96,uint48,uint24,uint56),(bytes32,bytes32),uint256),(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256))";
            const SELECTOR: [u8; 4] = [120u8, 209u8, 73u8, 168u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.ticketHash),
                    <RedeemableTicket as alloy_sol_types::SolType>::tokenize(
                        &self.redeemable,
                    ),
                    <HoprCrypto::VRFParameters as alloy_sol_types::SolType>::tokenize(
                        &self.params,
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
                        let r: _isWinningTicketReturn = r.into();
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
                        let r: _isWinningTicketReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `channelState(bytes32)` and selector `0x762bd76b`.
```solidity
function channelState(bytes32 channelId) external view returns (bytes32 state);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct channelStateCall {
        #[allow(missing_docs)]
        pub channelId: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`channelState(bytes32)`](channelStateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct channelStateReturn {
        #[allow(missing_docs)]
        pub state: alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<channelStateCall> for UnderlyingRustTuple<'_> {
                fn from(value: channelStateCall) -> Self {
                    (value.channelId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for channelStateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { channelId: tuple.0 }
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
            impl ::core::convert::From<channelStateReturn> for UnderlyingRustTuple<'_> {
                fn from(value: channelStateReturn) -> Self {
                    (value.state,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for channelStateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { state: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for channelStateCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "channelState(bytes32)";
            const SELECTOR: [u8; 4] = [118u8, 43u8, 215u8, 107u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.channelId),
                )
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
                        let r: channelStateReturn = r.into();
                        r.state
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
                        let r: channelStateReturn = r.into();
                        r.state
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `channels(bytes32)` and selector `0x7a7ebd7b`.
```solidity
function channels(bytes32) external view returns (HoprChannelsType.Balance balance, HoprChannelsType.TicketIndex ticketIndex, HoprChannelsType.Timestamp closureTime, HoprChannelsType.ChannelEpoch epoch, HoprChannelsType.ChannelStatus status);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct channelsCall(pub alloy::sol_types::private::FixedBytes<32>);
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`channels(bytes32)`](channelsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct channelsReturn {
        #[allow(missing_docs)]
        pub balance: <HoprChannelsType::Balance as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub ticketIndex: <HoprChannelsType::TicketIndex as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub closureTime: <HoprChannelsType::Timestamp as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub epoch: <HoprChannelsType::ChannelEpoch as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub status: <HoprChannelsType::ChannelStatus as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<channelsCall> for UnderlyingRustTuple<'_> {
                fn from(value: channelsCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for channelsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                HoprChannelsType::Balance,
                HoprChannelsType::TicketIndex,
                HoprChannelsType::Timestamp,
                HoprChannelsType::ChannelEpoch,
                HoprChannelsType::ChannelStatus,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <HoprChannelsType::Balance as alloy::sol_types::SolType>::RustType,
                <HoprChannelsType::TicketIndex as alloy::sol_types::SolType>::RustType,
                <HoprChannelsType::Timestamp as alloy::sol_types::SolType>::RustType,
                <HoprChannelsType::ChannelEpoch as alloy::sol_types::SolType>::RustType,
                <HoprChannelsType::ChannelStatus as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<channelsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: channelsReturn) -> Self {
                    (
                        value.balance,
                        value.ticketIndex,
                        value.closureTime,
                        value.epoch,
                        value.status,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for channelsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        balance: tuple.0,
                        ticketIndex: tuple.1,
                        closureTime: tuple.2,
                        epoch: tuple.3,
                        status: tuple.4,
                    }
                }
            }
        }
        impl channelsReturn {
            fn _tokenize(
                &self,
            ) -> <channelsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <HoprChannelsType::Balance as alloy_sol_types::SolType>::tokenize(
                        &self.balance,
                    ),
                    <HoprChannelsType::TicketIndex as alloy_sol_types::SolType>::tokenize(
                        &self.ticketIndex,
                    ),
                    <HoprChannelsType::Timestamp as alloy_sol_types::SolType>::tokenize(
                        &self.closureTime,
                    ),
                    <HoprChannelsType::ChannelEpoch as alloy_sol_types::SolType>::tokenize(
                        &self.epoch,
                    ),
                    <HoprChannelsType::ChannelStatus as alloy_sol_types::SolType>::tokenize(
                        &self.status,
                    ),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for channelsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = channelsReturn;
            type ReturnTuple<'a> = (
                HoprChannelsType::Balance,
                HoprChannelsType::TicketIndex,
                HoprChannelsType::Timestamp,
                HoprChannelsType::ChannelEpoch,
                HoprChannelsType::ChannelStatus,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "channels(bytes32)";
            const SELECTOR: [u8; 4] = [122u8, 126u8, 189u8, 123u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.0),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                channelsReturn::_tokenize(ret)
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
    /**Function with signature `closeIncomingChannel(address)` and selector `0x1a7ffe7a`.
```solidity
function closeIncomingChannel(address source) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct closeIncomingChannelCall {
        #[allow(missing_docs)]
        pub source: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`closeIncomingChannel(address)`](closeIncomingChannelCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct closeIncomingChannelReturn {}
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
            impl ::core::convert::From<closeIncomingChannelCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: closeIncomingChannelCall) -> Self {
                    (value.source,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for closeIncomingChannelCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { source: tuple.0 }
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
            impl ::core::convert::From<closeIncomingChannelReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: closeIncomingChannelReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for closeIncomingChannelReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl closeIncomingChannelReturn {
            fn _tokenize(
                &self,
            ) -> <closeIncomingChannelCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for closeIncomingChannelCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = closeIncomingChannelReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "closeIncomingChannel(address)";
            const SELECTOR: [u8; 4] = [26u8, 127u8, 254u8, 122u8];
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
                        &self.source,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                closeIncomingChannelReturn::_tokenize(ret)
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
    /**Function with signature `closeIncomingChannelSafe(address,address)` and selector `0x54a2edf5`.
```solidity
function closeIncomingChannelSafe(address selfAddress, address source) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct closeIncomingChannelSafeCall {
        #[allow(missing_docs)]
        pub selfAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub source: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`closeIncomingChannelSafe(address,address)`](closeIncomingChannelSafeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct closeIncomingChannelSafeReturn {}
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
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<closeIncomingChannelSafeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: closeIncomingChannelSafeCall) -> Self {
                    (value.selfAddress, value.source)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for closeIncomingChannelSafeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        selfAddress: tuple.0,
                        source: tuple.1,
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
            impl ::core::convert::From<closeIncomingChannelSafeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: closeIncomingChannelSafeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for closeIncomingChannelSafeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl closeIncomingChannelSafeReturn {
            fn _tokenize(
                &self,
            ) -> <closeIncomingChannelSafeCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for closeIncomingChannelSafeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = closeIncomingChannelSafeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "closeIncomingChannelSafe(address,address)";
            const SELECTOR: [u8; 4] = [84u8, 162u8, 237u8, 245u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.source,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                closeIncomingChannelSafeReturn::_tokenize(ret)
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
    /**Function with signature `domainSeparator()` and selector `0xf698da25`.
```solidity
function domainSeparator() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct domainSeparatorCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`domainSeparator()`](domainSeparatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct domainSeparatorReturn {
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
            impl ::core::convert::From<domainSeparatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: domainSeparatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for domainSeparatorCall {
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
            impl ::core::convert::From<domainSeparatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: domainSeparatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for domainSeparatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for domainSeparatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "domainSeparator()";
            const SELECTOR: [u8; 4] = [246u8, 152u8, 218u8, 37u8];
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
                        let r: domainSeparatorReturn = r.into();
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
                        let r: domainSeparatorReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `finalizeOutgoingChannelClosure(address)` and selector `0x23cb3ac0`.
```solidity
function finalizeOutgoingChannelClosure(address destination) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct finalizeOutgoingChannelClosureCall {
        #[allow(missing_docs)]
        pub destination: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`finalizeOutgoingChannelClosure(address)`](finalizeOutgoingChannelClosureCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct finalizeOutgoingChannelClosureReturn {}
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
            impl ::core::convert::From<finalizeOutgoingChannelClosureCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: finalizeOutgoingChannelClosureCall) -> Self {
                    (value.destination,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for finalizeOutgoingChannelClosureCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { destination: tuple.0 }
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
            impl ::core::convert::From<finalizeOutgoingChannelClosureReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: finalizeOutgoingChannelClosureReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for finalizeOutgoingChannelClosureReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl finalizeOutgoingChannelClosureReturn {
            fn _tokenize(
                &self,
            ) -> <finalizeOutgoingChannelClosureCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for finalizeOutgoingChannelClosureCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = finalizeOutgoingChannelClosureReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "finalizeOutgoingChannelClosure(address)";
            const SELECTOR: [u8; 4] = [35u8, 203u8, 58u8, 192u8];
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
                        &self.destination,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                finalizeOutgoingChannelClosureReturn::_tokenize(ret)
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
    /**Function with signature `finalizeOutgoingChannelClosureSafe(address,address)` and selector `0x651514bf`.
```solidity
function finalizeOutgoingChannelClosureSafe(address selfAddress, address destination) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct finalizeOutgoingChannelClosureSafeCall {
        #[allow(missing_docs)]
        pub selfAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub destination: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`finalizeOutgoingChannelClosureSafe(address,address)`](finalizeOutgoingChannelClosureSafeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct finalizeOutgoingChannelClosureSafeReturn {}
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
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<finalizeOutgoingChannelClosureSafeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: finalizeOutgoingChannelClosureSafeCall) -> Self {
                    (value.selfAddress, value.destination)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for finalizeOutgoingChannelClosureSafeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        selfAddress: tuple.0,
                        destination: tuple.1,
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
            impl ::core::convert::From<finalizeOutgoingChannelClosureSafeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: finalizeOutgoingChannelClosureSafeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for finalizeOutgoingChannelClosureSafeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl finalizeOutgoingChannelClosureSafeReturn {
            fn _tokenize(
                &self,
            ) -> <finalizeOutgoingChannelClosureSafeCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for finalizeOutgoingChannelClosureSafeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = finalizeOutgoingChannelClosureSafeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "finalizeOutgoingChannelClosureSafe(address,address)";
            const SELECTOR: [u8; 4] = [101u8, 21u8, 20u8, 191u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.destination,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                finalizeOutgoingChannelClosureSafeReturn::_tokenize(ret)
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
    /**Function with signature `fundChannel(address,uint96)` and selector `0xfc55309a`.
```solidity
function fundChannel(address account, HoprChannelsType.Balance amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct fundChannelCall {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: <HoprChannelsType::Balance as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`fundChannel(address,uint96)`](fundChannelCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct fundChannelReturn {}
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
                HoprChannelsType::Balance,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <HoprChannelsType::Balance as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<fundChannelCall> for UnderlyingRustTuple<'_> {
                fn from(value: fundChannelCall) -> Self {
                    (value.account, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for fundChannelCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        account: tuple.0,
                        amount: tuple.1,
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
            impl ::core::convert::From<fundChannelReturn> for UnderlyingRustTuple<'_> {
                fn from(value: fundChannelReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for fundChannelReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl fundChannelReturn {
            fn _tokenize(
                &self,
            ) -> <fundChannelCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for fundChannelCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                HoprChannelsType::Balance,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = fundChannelReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "fundChannel(address,uint96)";
            const SELECTOR: [u8; 4] = [252u8, 85u8, 48u8, 154u8];
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
                    <HoprChannelsType::Balance as alloy_sol_types::SolType>::tokenize(
                        &self.amount,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                fundChannelReturn::_tokenize(ret)
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
    /**Function with signature `fundChannelSafe(address,address,uint96)` and selector `0x0abec58f`.
```solidity
function fundChannelSafe(address selfAddress, address account, HoprChannelsType.Balance amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct fundChannelSafeCall {
        #[allow(missing_docs)]
        pub selfAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: <HoprChannelsType::Balance as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`fundChannelSafe(address,address,uint96)`](fundChannelSafeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct fundChannelSafeReturn {}
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
                HoprChannelsType::Balance,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                <HoprChannelsType::Balance as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<fundChannelSafeCall> for UnderlyingRustTuple<'_> {
                fn from(value: fundChannelSafeCall) -> Self {
                    (value.selfAddress, value.account, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for fundChannelSafeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        selfAddress: tuple.0,
                        account: tuple.1,
                        amount: tuple.2,
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
            impl ::core::convert::From<fundChannelSafeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: fundChannelSafeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for fundChannelSafeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl fundChannelSafeReturn {
            fn _tokenize(
                &self,
            ) -> <fundChannelSafeCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for fundChannelSafeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                HoprChannelsType::Balance,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = fundChannelSafeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "fundChannelSafe(address,address,uint96)";
            const SELECTOR: [u8; 4] = [10u8, 190u8, 197u8, 143u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                    <HoprChannelsType::Balance as alloy_sol_types::SolType>::tokenize(
                        &self.amount,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                fundChannelSafeReturn::_tokenize(ret)
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
    /**Function with signature `initiateOutgoingChannelClosure(address)` and selector `0x7c8e28da`.
```solidity
function initiateOutgoingChannelClosure(address destination) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initiateOutgoingChannelClosureCall {
        #[allow(missing_docs)]
        pub destination: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`initiateOutgoingChannelClosure(address)`](initiateOutgoingChannelClosureCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initiateOutgoingChannelClosureReturn {}
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
            impl ::core::convert::From<initiateOutgoingChannelClosureCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: initiateOutgoingChannelClosureCall) -> Self {
                    (value.destination,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initiateOutgoingChannelClosureCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { destination: tuple.0 }
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
            impl ::core::convert::From<initiateOutgoingChannelClosureReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: initiateOutgoingChannelClosureReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initiateOutgoingChannelClosureReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl initiateOutgoingChannelClosureReturn {
            fn _tokenize(
                &self,
            ) -> <initiateOutgoingChannelClosureCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initiateOutgoingChannelClosureCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initiateOutgoingChannelClosureReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initiateOutgoingChannelClosure(address)";
            const SELECTOR: [u8; 4] = [124u8, 142u8, 40u8, 218u8];
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
                        &self.destination,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                initiateOutgoingChannelClosureReturn::_tokenize(ret)
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
    /**Function with signature `initiateOutgoingChannelClosureSafe(address,address)` and selector `0xbda65f45`.
```solidity
function initiateOutgoingChannelClosureSafe(address selfAddress, address destination) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initiateOutgoingChannelClosureSafeCall {
        #[allow(missing_docs)]
        pub selfAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub destination: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`initiateOutgoingChannelClosureSafe(address,address)`](initiateOutgoingChannelClosureSafeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initiateOutgoingChannelClosureSafeReturn {}
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
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<initiateOutgoingChannelClosureSafeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: initiateOutgoingChannelClosureSafeCall) -> Self {
                    (value.selfAddress, value.destination)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initiateOutgoingChannelClosureSafeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        selfAddress: tuple.0,
                        destination: tuple.1,
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
            impl ::core::convert::From<initiateOutgoingChannelClosureSafeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: initiateOutgoingChannelClosureSafeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initiateOutgoingChannelClosureSafeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl initiateOutgoingChannelClosureSafeReturn {
            fn _tokenize(
                &self,
            ) -> <initiateOutgoingChannelClosureSafeCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initiateOutgoingChannelClosureSafeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initiateOutgoingChannelClosureSafeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initiateOutgoingChannelClosureSafe(address,address)";
            const SELECTOR: [u8; 4] = [189u8, 166u8, 95u8, 69u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.destination,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                initiateOutgoingChannelClosureSafeReturn::_tokenize(ret)
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
    /**Function with signature `multicall(bytes[])` and selector `0xac9650d8`.
```solidity
function multicall(bytes[] memory data) external returns (bytes[] memory results);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct multicallCall {
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`multicall(bytes[])`](multicallCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct multicallReturn {
        #[allow(missing_docs)]
        pub results: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
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
            impl ::core::convert::From<multicallCall> for UnderlyingRustTuple<'_> {
                fn from(value: multicallCall) -> Self {
                    (value.data,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for multicallCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { data: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
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
            impl ::core::convert::From<multicallReturn> for UnderlyingRustTuple<'_> {
                fn from(value: multicallReturn) -> Self {
                    (value.results,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for multicallReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { results: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for multicallCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::Bytes,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "multicall(bytes[])";
            const SELECTOR: [u8; 4] = [172u8, 150u8, 80u8, 216u8];
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
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::tokenize(&self.data),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: multicallReturn = r.into();
                        r.results
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
                        let r: multicallReturn = r.into();
                        r.results
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `redeemTicket(((bytes32,uint96,uint48,uint24,uint56),(bytes32,bytes32),uint256),(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256))` and selector `0x65e3fa72`.
```solidity
function redeemTicket(RedeemableTicket memory redeemable, HoprCrypto.VRFParameters memory params) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct redeemTicketCall {
        #[allow(missing_docs)]
        pub redeemable: <RedeemableTicket as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub params: <HoprCrypto::VRFParameters as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`redeemTicket(((bytes32,uint96,uint48,uint24,uint56),(bytes32,bytes32),uint256),(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256))`](redeemTicketCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct redeemTicketReturn {}
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
            type UnderlyingSolTuple<'a> = (RedeemableTicket, HoprCrypto::VRFParameters);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <RedeemableTicket as alloy::sol_types::SolType>::RustType,
                <HoprCrypto::VRFParameters as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<redeemTicketCall> for UnderlyingRustTuple<'_> {
                fn from(value: redeemTicketCall) -> Self {
                    (value.redeemable, value.params)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for redeemTicketCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        redeemable: tuple.0,
                        params: tuple.1,
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
            impl ::core::convert::From<redeemTicketReturn> for UnderlyingRustTuple<'_> {
                fn from(value: redeemTicketReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for redeemTicketReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl redeemTicketReturn {
            fn _tokenize(
                &self,
            ) -> <redeemTicketCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for redeemTicketCall {
            type Parameters<'a> = (RedeemableTicket, HoprCrypto::VRFParameters);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = redeemTicketReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "redeemTicket(((bytes32,uint96,uint48,uint24,uint56),(bytes32,bytes32),uint256),(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256))";
            const SELECTOR: [u8; 4] = [101u8, 227u8, 250u8, 114u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <RedeemableTicket as alloy_sol_types::SolType>::tokenize(
                        &self.redeemable,
                    ),
                    <HoprCrypto::VRFParameters as alloy_sol_types::SolType>::tokenize(
                        &self.params,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                redeemTicketReturn::_tokenize(ret)
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
    /**Function with signature `redeemTicketSafe(address,((bytes32,uint96,uint48,uint24,uint56),(bytes32,bytes32),uint256),(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256))` and selector `0x2d50b18b`.
```solidity
function redeemTicketSafe(address selfAddress, RedeemableTicket memory redeemable, HoprCrypto.VRFParameters memory params) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct redeemTicketSafeCall {
        #[allow(missing_docs)]
        pub selfAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub redeemable: <RedeemableTicket as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub params: <HoprCrypto::VRFParameters as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`redeemTicketSafe(address,((bytes32,uint96,uint48,uint24,uint56),(bytes32,bytes32),uint256),(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256))`](redeemTicketSafeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct redeemTicketSafeReturn {}
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
                RedeemableTicket,
                HoprCrypto::VRFParameters,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <RedeemableTicket as alloy::sol_types::SolType>::RustType,
                <HoprCrypto::VRFParameters as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<redeemTicketSafeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: redeemTicketSafeCall) -> Self {
                    (value.selfAddress, value.redeemable, value.params)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for redeemTicketSafeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        selfAddress: tuple.0,
                        redeemable: tuple.1,
                        params: tuple.2,
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
            impl ::core::convert::From<redeemTicketSafeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: redeemTicketSafeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for redeemTicketSafeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl redeemTicketSafeReturn {
            fn _tokenize(
                &self,
            ) -> <redeemTicketSafeCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for redeemTicketSafeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                RedeemableTicket,
                HoprCrypto::VRFParameters,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = redeemTicketSafeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "redeemTicketSafe(address,((bytes32,uint96,uint48,uint24,uint56),(bytes32,bytes32),uint256),(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256))";
            const SELECTOR: [u8; 4] = [45u8, 80u8, 177u8, 139u8];
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
                    <RedeemableTicket as alloy_sol_types::SolType>::tokenize(
                        &self.redeemable,
                    ),
                    <HoprCrypto::VRFParameters as alloy_sol_types::SolType>::tokenize(
                        &self.params,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                redeemTicketSafeReturn::_tokenize(ret)
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
    /**Function with signature `updateDomainSeparator()` and selector `0x89ccfe89`.
```solidity
function updateDomainSeparator() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateDomainSeparatorCall;
    ///Container type for the return parameters of the [`updateDomainSeparator()`](updateDomainSeparatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateDomainSeparatorReturn {}
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
            impl ::core::convert::From<updateDomainSeparatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateDomainSeparatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateDomainSeparatorCall {
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
            impl ::core::convert::From<updateDomainSeparatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateDomainSeparatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateDomainSeparatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl updateDomainSeparatorReturn {
            fn _tokenize(
                &self,
            ) -> <updateDomainSeparatorCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateDomainSeparatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateDomainSeparatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateDomainSeparator()";
            const SELECTOR: [u8; 4] = [137u8, 204u8, 254u8, 137u8];
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
                updateDomainSeparatorReturn::_tokenize(ret)
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
    ///Container for all the [`HoprChannels`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum HoprChannelsCalls {
        #[allow(missing_docs)]
        ERC777_HOOK_FUND_CHANNEL_MULTI_SIZE(ERC777_HOOK_FUND_CHANNEL_MULTI_SIZECall),
        #[allow(missing_docs)]
        ERC777_HOOK_FUND_CHANNEL_SIZE(ERC777_HOOK_FUND_CHANNEL_SIZECall),
        #[allow(missing_docs)]
        LEDGER_VERSION(LEDGER_VERSIONCall),
        #[allow(missing_docs)]
        MAX_USED_BALANCE(MAX_USED_BALANCECall),
        #[allow(missing_docs)]
        MIN_USED_BALANCE(MIN_USED_BALANCECall),
        #[allow(missing_docs)]
        NOTICE_PERIOD_CHANNEL_CLOSURE(NOTICE_PERIOD_CHANNEL_CLOSURECall),
        #[allow(missing_docs)]
        SNAPSHOT_INTERVAL(SNAPSHOT_INTERVALCall),
        #[allow(missing_docs)]
        TOKEN(TOKENCall),
        #[allow(missing_docs)]
        TOKENS_RECIPIENT_INTERFACE_HASH(TOKENS_RECIPIENT_INTERFACE_HASHCall),
        #[allow(missing_docs)]
        VERSION(VERSIONCall),
        #[allow(missing_docs)]
        _currentBlockTimestamp(_currentBlockTimestampCall),
        #[allow(missing_docs)]
        _getChannelId(_getChannelIdCall),
        #[allow(missing_docs)]
        _getTicketHash(_getTicketHashCall),
        #[allow(missing_docs)]
        _isWinningTicket(_isWinningTicketCall),
        #[allow(missing_docs)]
        channelState(channelStateCall),
        #[allow(missing_docs)]
        channels(channelsCall),
        #[allow(missing_docs)]
        closeIncomingChannel(closeIncomingChannelCall),
        #[allow(missing_docs)]
        closeIncomingChannelSafe(closeIncomingChannelSafeCall),
        #[allow(missing_docs)]
        domainSeparator(domainSeparatorCall),
        #[allow(missing_docs)]
        finalizeOutgoingChannelClosure(finalizeOutgoingChannelClosureCall),
        #[allow(missing_docs)]
        finalizeOutgoingChannelClosureSafe(finalizeOutgoingChannelClosureSafeCall),
        #[allow(missing_docs)]
        fundChannel(fundChannelCall),
        #[allow(missing_docs)]
        fundChannelSafe(fundChannelSafeCall),
        #[allow(missing_docs)]
        initiateOutgoingChannelClosure(initiateOutgoingChannelClosureCall),
        #[allow(missing_docs)]
        initiateOutgoingChannelClosureSafe(initiateOutgoingChannelClosureSafeCall),
        #[allow(missing_docs)]
        latestRoot(latestRootCall),
        #[allow(missing_docs)]
        latestSnapshotRoot(latestSnapshotRootCall),
        #[allow(missing_docs)]
        ledgerDomainSeparator(ledgerDomainSeparatorCall),
        #[allow(missing_docs)]
        multicall(multicallCall),
        #[allow(missing_docs)]
        redeemTicket(redeemTicketCall),
        #[allow(missing_docs)]
        redeemTicketSafe(redeemTicketSafeCall),
        #[allow(missing_docs)]
        tokensReceived(tokensReceivedCall),
        #[allow(missing_docs)]
        updateDomainSeparator(updateDomainSeparatorCall),
        #[allow(missing_docs)]
        updateLedgerDomainSeparator(updateLedgerDomainSeparatorCall),
    }
    impl HoprChannelsCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [0u8, 35u8, 222u8, 41u8],
            [10u8, 190u8, 197u8, 143u8],
            [13u8, 241u8, 143u8, 148u8],
            [26u8, 127u8, 254u8, 122u8],
            [35u8, 203u8, 58u8, 192u8],
            [41u8, 57u8, 46u8, 50u8],
            [45u8, 80u8, 177u8, 139u8],
            [68u8, 218u8, 230u8, 248u8],
            [84u8, 162u8, 237u8, 245u8],
            [93u8, 47u8, 7u8, 197u8],
            [101u8, 21u8, 20u8, 191u8],
            [101u8, 227u8, 250u8, 114u8],
            [109u8, 43u8, 238u8, 241u8],
            [114u8, 88u8, 28u8, 192u8],
            [118u8, 43u8, 215u8, 107u8],
            [120u8, 209u8, 73u8, 168u8],
            [120u8, 216u8, 1u8, 109u8],
            [122u8, 126u8, 189u8, 123u8],
            [124u8, 142u8, 40u8, 218u8],
            [130u8, 191u8, 239u8, 200u8],
            [137u8, 204u8, 254u8, 137u8],
            [171u8, 102u8, 204u8, 171u8],
            [172u8, 150u8, 80u8, 216u8],
            [185u8, 32u8, 222u8, 237u8],
            [189u8, 166u8, 95u8, 69u8],
            [190u8, 155u8, 171u8, 220u8],
            [201u8, 102u8, 196u8, 254u8],
            [215u8, 176u8, 254u8, 241u8],
            [217u8, 76u8, 193u8, 180u8],
            [220u8, 150u8, 253u8, 80u8],
            [221u8, 173u8, 25u8, 2u8],
            [246u8, 152u8, 218u8, 37u8],
            [252u8, 85u8, 48u8, 154u8],
            [255u8, 161u8, 173u8, 116u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(tokensReceived),
            ::core::stringify!(fundChannelSafe),
            ::core::stringify!(latestSnapshotRoot),
            ::core::stringify!(closeIncomingChannel),
            ::core::stringify!(finalizeOutgoingChannelClosure),
            ::core::stringify!(MIN_USED_BALANCE),
            ::core::stringify!(redeemTicketSafe),
            ::core::stringify!(ERC777_HOOK_FUND_CHANNEL_SIZE),
            ::core::stringify!(closeIncomingChannelSafe),
            ::core::stringify!(MAX_USED_BALANCE),
            ::core::stringify!(finalizeOutgoingChannelClosureSafe),
            ::core::stringify!(redeemTicket),
            ::core::stringify!(SNAPSHOT_INTERVAL),
            ::core::stringify!(TOKENS_RECIPIENT_INTERFACE_HASH),
            ::core::stringify!(channelState),
            ::core::stringify!(_isWinningTicket),
            ::core::stringify!(ERC777_HOOK_FUND_CHANNEL_MULTI_SIZE),
            ::core::stringify!(channels),
            ::core::stringify!(initiateOutgoingChannelClosure),
            ::core::stringify!(TOKEN),
            ::core::stringify!(updateDomainSeparator),
            ::core::stringify!(_getTicketHash),
            ::core::stringify!(multicall),
            ::core::stringify!(_currentBlockTimestamp),
            ::core::stringify!(initiateOutgoingChannelClosureSafe),
            ::core::stringify!(_getChannelId),
            ::core::stringify!(ledgerDomainSeparator),
            ::core::stringify!(latestRoot),
            ::core::stringify!(NOTICE_PERIOD_CHANNEL_CLOSURE),
            ::core::stringify!(updateLedgerDomainSeparator),
            ::core::stringify!(LEDGER_VERSION),
            ::core::stringify!(domainSeparator),
            ::core::stringify!(fundChannel),
            ::core::stringify!(VERSION),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <tokensReceivedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <fundChannelSafeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <latestSnapshotRootCall as alloy_sol_types::SolCall>::SIGNATURE,
            <closeIncomingChannelCall as alloy_sol_types::SolCall>::SIGNATURE,
            <finalizeOutgoingChannelClosureCall as alloy_sol_types::SolCall>::SIGNATURE,
            <MIN_USED_BALANCECall as alloy_sol_types::SolCall>::SIGNATURE,
            <redeemTicketSafeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <ERC777_HOOK_FUND_CHANNEL_SIZECall as alloy_sol_types::SolCall>::SIGNATURE,
            <closeIncomingChannelSafeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <MAX_USED_BALANCECall as alloy_sol_types::SolCall>::SIGNATURE,
            <finalizeOutgoingChannelClosureSafeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <redeemTicketCall as alloy_sol_types::SolCall>::SIGNATURE,
            <SNAPSHOT_INTERVALCall as alloy_sol_types::SolCall>::SIGNATURE,
            <TOKENS_RECIPIENT_INTERFACE_HASHCall as alloy_sol_types::SolCall>::SIGNATURE,
            <channelStateCall as alloy_sol_types::SolCall>::SIGNATURE,
            <_isWinningTicketCall as alloy_sol_types::SolCall>::SIGNATURE,
            <ERC777_HOOK_FUND_CHANNEL_MULTI_SIZECall as alloy_sol_types::SolCall>::SIGNATURE,
            <channelsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <initiateOutgoingChannelClosureCall as alloy_sol_types::SolCall>::SIGNATURE,
            <TOKENCall as alloy_sol_types::SolCall>::SIGNATURE,
            <updateDomainSeparatorCall as alloy_sol_types::SolCall>::SIGNATURE,
            <_getTicketHashCall as alloy_sol_types::SolCall>::SIGNATURE,
            <multicallCall as alloy_sol_types::SolCall>::SIGNATURE,
            <_currentBlockTimestampCall as alloy_sol_types::SolCall>::SIGNATURE,
            <initiateOutgoingChannelClosureSafeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <_getChannelIdCall as alloy_sol_types::SolCall>::SIGNATURE,
            <ledgerDomainSeparatorCall as alloy_sol_types::SolCall>::SIGNATURE,
            <latestRootCall as alloy_sol_types::SolCall>::SIGNATURE,
            <NOTICE_PERIOD_CHANNEL_CLOSURECall as alloy_sol_types::SolCall>::SIGNATURE,
            <updateLedgerDomainSeparatorCall as alloy_sol_types::SolCall>::SIGNATURE,
            <LEDGER_VERSIONCall as alloy_sol_types::SolCall>::SIGNATURE,
            <domainSeparatorCall as alloy_sol_types::SolCall>::SIGNATURE,
            <fundChannelCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for HoprChannelsCalls {
        const NAME: &'static str = "HoprChannelsCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 34usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::ERC777_HOOK_FUND_CHANNEL_MULTI_SIZE(_) => {
                    <ERC777_HOOK_FUND_CHANNEL_MULTI_SIZECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::ERC777_HOOK_FUND_CHANNEL_SIZE(_) => {
                    <ERC777_HOOK_FUND_CHANNEL_SIZECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::LEDGER_VERSION(_) => {
                    <LEDGER_VERSIONCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::MAX_USED_BALANCE(_) => {
                    <MAX_USED_BALANCECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::MIN_USED_BALANCE(_) => {
                    <MIN_USED_BALANCECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::NOTICE_PERIOD_CHANNEL_CLOSURE(_) => {
                    <NOTICE_PERIOD_CHANNEL_CLOSURECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::SNAPSHOT_INTERVAL(_) => {
                    <SNAPSHOT_INTERVALCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::TOKEN(_) => <TOKENCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::TOKENS_RECIPIENT_INTERFACE_HASH(_) => {
                    <TOKENS_RECIPIENT_INTERFACE_HASHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::VERSION(_) => <VERSIONCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::_currentBlockTimestamp(_) => {
                    <_currentBlockTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::_getChannelId(_) => {
                    <_getChannelIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::_getTicketHash(_) => {
                    <_getTicketHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::_isWinningTicket(_) => {
                    <_isWinningTicketCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::channelState(_) => {
                    <channelStateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::channels(_) => <channelsCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::closeIncomingChannel(_) => {
                    <closeIncomingChannelCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::closeIncomingChannelSafe(_) => {
                    <closeIncomingChannelSafeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::domainSeparator(_) => {
                    <domainSeparatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::finalizeOutgoingChannelClosure(_) => {
                    <finalizeOutgoingChannelClosureCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::finalizeOutgoingChannelClosureSafe(_) => {
                    <finalizeOutgoingChannelClosureSafeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::fundChannel(_) => {
                    <fundChannelCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::fundChannelSafe(_) => {
                    <fundChannelSafeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initiateOutgoingChannelClosure(_) => {
                    <initiateOutgoingChannelClosureCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initiateOutgoingChannelClosureSafe(_) => {
                    <initiateOutgoingChannelClosureSafeCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::multicall(_) => {
                    <multicallCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::redeemTicket(_) => {
                    <redeemTicketCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::redeemTicketSafe(_) => {
                    <redeemTicketSafeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::tokensReceived(_) => {
                    <tokensReceivedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateDomainSeparator(_) => {
                    <updateDomainSeparatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateLedgerDomainSeparator(_) => {
                    <updateLedgerDomainSeparatorCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<HoprChannelsCalls>] = &[
                {
                    fn tokensReceived(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <tokensReceivedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsCalls::tokensReceived)
                    }
                    tokensReceived
                },
                {
                    fn fundChannelSafe(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <fundChannelSafeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsCalls::fundChannelSafe)
                    }
                    fundChannelSafe
                },
                {
                    fn latestSnapshotRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <latestSnapshotRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsCalls::latestSnapshotRoot)
                    }
                    latestSnapshotRoot
                },
                {
                    fn closeIncomingChannel(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <closeIncomingChannelCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsCalls::closeIncomingChannel)
                    }
                    closeIncomingChannel
                },
                {
                    fn finalizeOutgoingChannelClosure(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <finalizeOutgoingChannelClosureCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsCalls::finalizeOutgoingChannelClosure)
                    }
                    finalizeOutgoingChannelClosure
                },
                {
                    fn MIN_USED_BALANCE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <MIN_USED_BALANCECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsCalls::MIN_USED_BALANCE)
                    }
                    MIN_USED_BALANCE
                },
                {
                    fn redeemTicketSafe(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <redeemTicketSafeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsCalls::redeemTicketSafe)
                    }
                    redeemTicketSafe
                },
                {
                    fn ERC777_HOOK_FUND_CHANNEL_SIZE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <ERC777_HOOK_FUND_CHANNEL_SIZECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsCalls::ERC777_HOOK_FUND_CHANNEL_SIZE)
                    }
                    ERC777_HOOK_FUND_CHANNEL_SIZE
                },
                {
                    fn closeIncomingChannelSafe(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <closeIncomingChannelSafeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsCalls::closeIncomingChannelSafe)
                    }
                    closeIncomingChannelSafe
                },
                {
                    fn MAX_USED_BALANCE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <MAX_USED_BALANCECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsCalls::MAX_USED_BALANCE)
                    }
                    MAX_USED_BALANCE
                },
                {
                    fn finalizeOutgoingChannelClosureSafe(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <finalizeOutgoingChannelClosureSafeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsCalls::finalizeOutgoingChannelClosureSafe)
                    }
                    finalizeOutgoingChannelClosureSafe
                },
                {
                    fn redeemTicket(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <redeemTicketCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsCalls::redeemTicket)
                    }
                    redeemTicket
                },
                {
                    fn SNAPSHOT_INTERVAL(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <SNAPSHOT_INTERVALCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsCalls::SNAPSHOT_INTERVAL)
                    }
                    SNAPSHOT_INTERVAL
                },
                {
                    fn TOKENS_RECIPIENT_INTERFACE_HASH(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <TOKENS_RECIPIENT_INTERFACE_HASHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsCalls::TOKENS_RECIPIENT_INTERFACE_HASH)
                    }
                    TOKENS_RECIPIENT_INTERFACE_HASH
                },
                {
                    fn channelState(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <channelStateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsCalls::channelState)
                    }
                    channelState
                },
                {
                    fn _isWinningTicket(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <_isWinningTicketCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsCalls::_isWinningTicket)
                    }
                    _isWinningTicket
                },
                {
                    fn ERC777_HOOK_FUND_CHANNEL_MULTI_SIZE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <ERC777_HOOK_FUND_CHANNEL_MULTI_SIZECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsCalls::ERC777_HOOK_FUND_CHANNEL_MULTI_SIZE)
                    }
                    ERC777_HOOK_FUND_CHANNEL_MULTI_SIZE
                },
                {
                    fn channels(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <channelsCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(HoprChannelsCalls::channels)
                    }
                    channels
                },
                {
                    fn initiateOutgoingChannelClosure(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <initiateOutgoingChannelClosureCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsCalls::initiateOutgoingChannelClosure)
                    }
                    initiateOutgoingChannelClosure
                },
                {
                    fn TOKEN(data: &[u8]) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <TOKENCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(HoprChannelsCalls::TOKEN)
                    }
                    TOKEN
                },
                {
                    fn updateDomainSeparator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <updateDomainSeparatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsCalls::updateDomainSeparator)
                    }
                    updateDomainSeparator
                },
                {
                    fn _getTicketHash(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <_getTicketHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsCalls::_getTicketHash)
                    }
                    _getTicketHash
                },
                {
                    fn multicall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <multicallCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(HoprChannelsCalls::multicall)
                    }
                    multicall
                },
                {
                    fn _currentBlockTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <_currentBlockTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsCalls::_currentBlockTimestamp)
                    }
                    _currentBlockTimestamp
                },
                {
                    fn initiateOutgoingChannelClosureSafe(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <initiateOutgoingChannelClosureSafeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsCalls::initiateOutgoingChannelClosureSafe)
                    }
                    initiateOutgoingChannelClosureSafe
                },
                {
                    fn _getChannelId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <_getChannelIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsCalls::_getChannelId)
                    }
                    _getChannelId
                },
                {
                    fn ledgerDomainSeparator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <ledgerDomainSeparatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsCalls::ledgerDomainSeparator)
                    }
                    ledgerDomainSeparator
                },
                {
                    fn latestRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <latestRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsCalls::latestRoot)
                    }
                    latestRoot
                },
                {
                    fn NOTICE_PERIOD_CHANNEL_CLOSURE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <NOTICE_PERIOD_CHANNEL_CLOSURECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsCalls::NOTICE_PERIOD_CHANNEL_CLOSURE)
                    }
                    NOTICE_PERIOD_CHANNEL_CLOSURE
                },
                {
                    fn updateLedgerDomainSeparator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <updateLedgerDomainSeparatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsCalls::updateLedgerDomainSeparator)
                    }
                    updateLedgerDomainSeparator
                },
                {
                    fn LEDGER_VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <LEDGER_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsCalls::LEDGER_VERSION)
                    }
                    LEDGER_VERSION
                },
                {
                    fn domainSeparator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <domainSeparatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsCalls::domainSeparator)
                    }
                    domainSeparator
                },
                {
                    fn fundChannel(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <fundChannelCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsCalls::fundChannel)
                    }
                    fundChannel
                },
                {
                    fn VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(HoprChannelsCalls::VERSION)
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
            ) -> alloy_sol_types::Result<HoprChannelsCalls>] = &[
                {
                    fn tokensReceived(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <tokensReceivedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsCalls::tokensReceived)
                    }
                    tokensReceived
                },
                {
                    fn fundChannelSafe(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <fundChannelSafeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsCalls::fundChannelSafe)
                    }
                    fundChannelSafe
                },
                {
                    fn latestSnapshotRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <latestSnapshotRootCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsCalls::latestSnapshotRoot)
                    }
                    latestSnapshotRoot
                },
                {
                    fn closeIncomingChannel(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <closeIncomingChannelCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsCalls::closeIncomingChannel)
                    }
                    closeIncomingChannel
                },
                {
                    fn finalizeOutgoingChannelClosure(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <finalizeOutgoingChannelClosureCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsCalls::finalizeOutgoingChannelClosure)
                    }
                    finalizeOutgoingChannelClosure
                },
                {
                    fn MIN_USED_BALANCE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <MIN_USED_BALANCECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsCalls::MIN_USED_BALANCE)
                    }
                    MIN_USED_BALANCE
                },
                {
                    fn redeemTicketSafe(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <redeemTicketSafeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsCalls::redeemTicketSafe)
                    }
                    redeemTicketSafe
                },
                {
                    fn ERC777_HOOK_FUND_CHANNEL_SIZE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <ERC777_HOOK_FUND_CHANNEL_SIZECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsCalls::ERC777_HOOK_FUND_CHANNEL_SIZE)
                    }
                    ERC777_HOOK_FUND_CHANNEL_SIZE
                },
                {
                    fn closeIncomingChannelSafe(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <closeIncomingChannelSafeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsCalls::closeIncomingChannelSafe)
                    }
                    closeIncomingChannelSafe
                },
                {
                    fn MAX_USED_BALANCE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <MAX_USED_BALANCECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsCalls::MAX_USED_BALANCE)
                    }
                    MAX_USED_BALANCE
                },
                {
                    fn finalizeOutgoingChannelClosureSafe(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <finalizeOutgoingChannelClosureSafeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsCalls::finalizeOutgoingChannelClosureSafe)
                    }
                    finalizeOutgoingChannelClosureSafe
                },
                {
                    fn redeemTicket(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <redeemTicketCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsCalls::redeemTicket)
                    }
                    redeemTicket
                },
                {
                    fn SNAPSHOT_INTERVAL(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <SNAPSHOT_INTERVALCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsCalls::SNAPSHOT_INTERVAL)
                    }
                    SNAPSHOT_INTERVAL
                },
                {
                    fn TOKENS_RECIPIENT_INTERFACE_HASH(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <TOKENS_RECIPIENT_INTERFACE_HASHCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsCalls::TOKENS_RECIPIENT_INTERFACE_HASH)
                    }
                    TOKENS_RECIPIENT_INTERFACE_HASH
                },
                {
                    fn channelState(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <channelStateCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsCalls::channelState)
                    }
                    channelState
                },
                {
                    fn _isWinningTicket(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <_isWinningTicketCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsCalls::_isWinningTicket)
                    }
                    _isWinningTicket
                },
                {
                    fn ERC777_HOOK_FUND_CHANNEL_MULTI_SIZE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <ERC777_HOOK_FUND_CHANNEL_MULTI_SIZECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsCalls::ERC777_HOOK_FUND_CHANNEL_MULTI_SIZE)
                    }
                    ERC777_HOOK_FUND_CHANNEL_MULTI_SIZE
                },
                {
                    fn channels(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <channelsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsCalls::channels)
                    }
                    channels
                },
                {
                    fn initiateOutgoingChannelClosure(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <initiateOutgoingChannelClosureCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsCalls::initiateOutgoingChannelClosure)
                    }
                    initiateOutgoingChannelClosure
                },
                {
                    fn TOKEN(data: &[u8]) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <TOKENCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsCalls::TOKEN)
                    }
                    TOKEN
                },
                {
                    fn updateDomainSeparator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <updateDomainSeparatorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsCalls::updateDomainSeparator)
                    }
                    updateDomainSeparator
                },
                {
                    fn _getTicketHash(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <_getTicketHashCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsCalls::_getTicketHash)
                    }
                    _getTicketHash
                },
                {
                    fn multicall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <multicallCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsCalls::multicall)
                    }
                    multicall
                },
                {
                    fn _currentBlockTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <_currentBlockTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsCalls::_currentBlockTimestamp)
                    }
                    _currentBlockTimestamp
                },
                {
                    fn initiateOutgoingChannelClosureSafe(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <initiateOutgoingChannelClosureSafeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsCalls::initiateOutgoingChannelClosureSafe)
                    }
                    initiateOutgoingChannelClosureSafe
                },
                {
                    fn _getChannelId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <_getChannelIdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsCalls::_getChannelId)
                    }
                    _getChannelId
                },
                {
                    fn ledgerDomainSeparator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <ledgerDomainSeparatorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsCalls::ledgerDomainSeparator)
                    }
                    ledgerDomainSeparator
                },
                {
                    fn latestRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <latestRootCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsCalls::latestRoot)
                    }
                    latestRoot
                },
                {
                    fn NOTICE_PERIOD_CHANNEL_CLOSURE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <NOTICE_PERIOD_CHANNEL_CLOSURECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsCalls::NOTICE_PERIOD_CHANNEL_CLOSURE)
                    }
                    NOTICE_PERIOD_CHANNEL_CLOSURE
                },
                {
                    fn updateLedgerDomainSeparator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <updateLedgerDomainSeparatorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsCalls::updateLedgerDomainSeparator)
                    }
                    updateLedgerDomainSeparator
                },
                {
                    fn LEDGER_VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <LEDGER_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsCalls::LEDGER_VERSION)
                    }
                    LEDGER_VERSION
                },
                {
                    fn domainSeparator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <domainSeparatorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsCalls::domainSeparator)
                    }
                    domainSeparator
                },
                {
                    fn fundChannel(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <fundChannelCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsCalls::fundChannel)
                    }
                    fundChannel
                },
                {
                    fn VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsCalls> {
                        <VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsCalls::VERSION)
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
                Self::ERC777_HOOK_FUND_CHANNEL_MULTI_SIZE(inner) => {
                    <ERC777_HOOK_FUND_CHANNEL_MULTI_SIZECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC777_HOOK_FUND_CHANNEL_SIZE(inner) => {
                    <ERC777_HOOK_FUND_CHANNEL_SIZECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LEDGER_VERSION(inner) => {
                    <LEDGER_VERSIONCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MAX_USED_BALANCE(inner) => {
                    <MAX_USED_BALANCECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MIN_USED_BALANCE(inner) => {
                    <MIN_USED_BALANCECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NOTICE_PERIOD_CHANNEL_CLOSURE(inner) => {
                    <NOTICE_PERIOD_CHANNEL_CLOSURECall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::VERSION(inner) => {
                    <VERSIONCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::_currentBlockTimestamp(inner) => {
                    <_currentBlockTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::_getChannelId(inner) => {
                    <_getChannelIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::_getTicketHash(inner) => {
                    <_getTicketHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::_isWinningTicket(inner) => {
                    <_isWinningTicketCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::channelState(inner) => {
                    <channelStateCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::channels(inner) => {
                    <channelsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::closeIncomingChannel(inner) => {
                    <closeIncomingChannelCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::closeIncomingChannelSafe(inner) => {
                    <closeIncomingChannelSafeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::domainSeparator(inner) => {
                    <domainSeparatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::finalizeOutgoingChannelClosure(inner) => {
                    <finalizeOutgoingChannelClosureCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::finalizeOutgoingChannelClosureSafe(inner) => {
                    <finalizeOutgoingChannelClosureSafeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::fundChannel(inner) => {
                    <fundChannelCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::fundChannelSafe(inner) => {
                    <fundChannelSafeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initiateOutgoingChannelClosure(inner) => {
                    <initiateOutgoingChannelClosureCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initiateOutgoingChannelClosureSafe(inner) => {
                    <initiateOutgoingChannelClosureSafeCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::multicall(inner) => {
                    <multicallCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::redeemTicket(inner) => {
                    <redeemTicketCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::redeemTicketSafe(inner) => {
                    <redeemTicketSafeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::tokensReceived(inner) => {
                    <tokensReceivedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateDomainSeparator(inner) => {
                    <updateDomainSeparatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateLedgerDomainSeparator(inner) => {
                    <updateLedgerDomainSeparatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::ERC777_HOOK_FUND_CHANNEL_MULTI_SIZE(inner) => {
                    <ERC777_HOOK_FUND_CHANNEL_MULTI_SIZECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC777_HOOK_FUND_CHANNEL_SIZE(inner) => {
                    <ERC777_HOOK_FUND_CHANNEL_SIZECall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::MAX_USED_BALANCE(inner) => {
                    <MAX_USED_BALANCECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MIN_USED_BALANCE(inner) => {
                    <MIN_USED_BALANCECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NOTICE_PERIOD_CHANNEL_CLOSURE(inner) => {
                    <NOTICE_PERIOD_CHANNEL_CLOSURECall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::VERSION(inner) => {
                    <VERSIONCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::_currentBlockTimestamp(inner) => {
                    <_currentBlockTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::_getChannelId(inner) => {
                    <_getChannelIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::_getTicketHash(inner) => {
                    <_getTicketHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::_isWinningTicket(inner) => {
                    <_isWinningTicketCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::channelState(inner) => {
                    <channelStateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::channels(inner) => {
                    <channelsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::closeIncomingChannel(inner) => {
                    <closeIncomingChannelCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::closeIncomingChannelSafe(inner) => {
                    <closeIncomingChannelSafeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::domainSeparator(inner) => {
                    <domainSeparatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::finalizeOutgoingChannelClosure(inner) => {
                    <finalizeOutgoingChannelClosureCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::finalizeOutgoingChannelClosureSafe(inner) => {
                    <finalizeOutgoingChannelClosureSafeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::fundChannel(inner) => {
                    <fundChannelCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::fundChannelSafe(inner) => {
                    <fundChannelSafeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initiateOutgoingChannelClosure(inner) => {
                    <initiateOutgoingChannelClosureCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initiateOutgoingChannelClosureSafe(inner) => {
                    <initiateOutgoingChannelClosureSafeCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::multicall(inner) => {
                    <multicallCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::redeemTicket(inner) => {
                    <redeemTicketCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::redeemTicketSafe(inner) => {
                    <redeemTicketSafeCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::updateDomainSeparator(inner) => {
                    <updateDomainSeparatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
            }
        }
    }
    ///Container for all the [`HoprChannels`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum HoprChannelsErrors {
        #[allow(missing_docs)]
        AddressEmptyCode(AddressEmptyCode),
        #[allow(missing_docs)]
        AlreadyInitialized(AlreadyInitialized),
        #[allow(missing_docs)]
        BalanceExceedsGlobalPerChannelAllowance(BalanceExceedsGlobalPerChannelAllowance),
        #[allow(missing_docs)]
        ContractNotResponsible(ContractNotResponsible),
        #[allow(missing_docs)]
        ECDSAInvalidSignature(ECDSAInvalidSignature),
        #[allow(missing_docs)]
        ECDSAInvalidSignatureLength(ECDSAInvalidSignatureLength),
        #[allow(missing_docs)]
        ECDSAInvalidSignatureS(ECDSAInvalidSignatureS),
        #[allow(missing_docs)]
        FailedCall(FailedCall),
        #[allow(missing_docs)]
        InsufficientChannelBalance(InsufficientChannelBalance),
        #[allow(missing_docs)]
        InvalidBalance(InvalidBalance),
        #[allow(missing_docs)]
        InvalidCurvePoint(InvalidCurvePoint),
        #[allow(missing_docs)]
        InvalidFieldElement(InvalidFieldElement),
        #[allow(missing_docs)]
        InvalidNoticePeriod(InvalidNoticePeriod),
        #[allow(missing_docs)]
        InvalidPointWitness(InvalidPointWitness),
        #[allow(missing_docs)]
        InvalidSafeAddress(InvalidSafeAddress),
        #[allow(missing_docs)]
        InvalidTicketIndex(InvalidTicketIndex),
        #[allow(missing_docs)]
        InvalidTicketSignature(InvalidTicketSignature),
        #[allow(missing_docs)]
        InvalidTokenRecipient(InvalidTokenRecipient),
        #[allow(missing_docs)]
        InvalidTokensReceivedUsage(InvalidTokensReceivedUsage),
        #[allow(missing_docs)]
        InvalidVRFProof(InvalidVRFProof),
        #[allow(missing_docs)]
        MultiSigUninitialized(MultiSigUninitialized),
        #[allow(missing_docs)]
        NoticePeriodNotDue(NoticePeriodNotDue),
        #[allow(missing_docs)]
        SourceEqualsDestination(SourceEqualsDestination),
        #[allow(missing_docs)]
        TicketIsNotAWin(TicketIsNotAWin),
        #[allow(missing_docs)]
        TokenTransferFailed(TokenTransferFailed),
        #[allow(missing_docs)]
        WrongChannelState(WrongChannelState),
        #[allow(missing_docs)]
        WrongToken(WrongToken),
        #[allow(missing_docs)]
        ZeroAddress(ZeroAddress),
        #[allow(missing_docs)]
        ZeroInterval(ZeroInterval),
    }
    impl HoprChannelsErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [4u8, 92u8, 75u8, 2u8],
            [13u8, 193u8, 73u8, 240u8],
            [49u8, 222u8, 72u8, 71u8],
            [52u8, 111u8, 246u8, 7u8],
            [58u8, 228u8, 237u8, 107u8],
            [69u8, 74u8, 32u8, 200u8],
            [73u8, 148u8, 99u8, 193u8],
            [105u8, 238u8, 111u8, 40u8],
            [113u8, 100u8, 3u8, 42u8],
            [114u8, 69u8, 74u8, 130u8],
            [142u8, 157u8, 124u8, 94u8],
            [149u8, 253u8, 189u8, 184u8],
            [151u8, 163u8, 174u8, 210u8],
            [153u8, 150u8, 179u8, 21u8],
            [160u8, 243u8, 254u8, 234u8],
            [164u8, 243u8, 187u8, 228u8],
            [172u8, 213u8, 168u8, 35u8],
            [177u8, 71u8, 99u8, 108u8],
            [185u8, 196u8, 145u8, 8u8],
            [197u8, 46u8, 62u8, 255u8],
            [205u8, 221u8, 83u8, 86u8],
            [214u8, 189u8, 162u8, 117u8],
            [215u8, 139u8, 206u8, 12u8],
            [234u8, 192u8, 211u8, 137u8],
            [237u8, 253u8, 205u8, 152u8],
            [238u8, 131u8, 92u8, 137u8],
            [246u8, 69u8, 238u8, 223u8],
            [249u8, 238u8, 145u8, 7u8],
            [252u8, 230u8, 152u8, 247u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(TokenTransferFailed),
            ::core::stringify!(AlreadyInitialized),
            ::core::stringify!(InvalidTicketIndex),
            ::core::stringify!(ZeroInterval),
            ::core::stringify!(InvalidFieldElement),
            ::core::stringify!(MultiSigUninitialized),
            ::core::stringify!(WrongChannelState),
            ::core::stringify!(InvalidTokensReceivedUsage),
            ::core::stringify!(NoticePeriodNotDue),
            ::core::stringify!(InvalidCurvePoint),
            ::core::stringify!(InvalidSafeAddress),
            ::core::stringify!(InvalidVRFProof),
            ::core::stringify!(SourceEqualsDestination),
            ::core::stringify!(AddressEmptyCode),
            ::core::stringify!(WrongToken),
            ::core::stringify!(BalanceExceedsGlobalPerChannelAllowance),
            ::core::stringify!(ContractNotResponsible),
            ::core::stringify!(InsufficientChannelBalance),
            ::core::stringify!(InvalidTokenRecipient),
            ::core::stringify!(InvalidBalance),
            ::core::stringify!(InvalidTicketSignature),
            ::core::stringify!(FailedCall),
            ::core::stringify!(ECDSAInvalidSignatureS),
            ::core::stringify!(ZeroAddress),
            ::core::stringify!(InvalidPointWitness),
            ::core::stringify!(TicketIsNotAWin),
            ::core::stringify!(ECDSAInvalidSignature),
            ::core::stringify!(InvalidNoticePeriod),
            ::core::stringify!(ECDSAInvalidSignatureLength),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <TokenTransferFailed as alloy_sol_types::SolError>::SIGNATURE,
            <AlreadyInitialized as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidTicketIndex as alloy_sol_types::SolError>::SIGNATURE,
            <ZeroInterval as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidFieldElement as alloy_sol_types::SolError>::SIGNATURE,
            <MultiSigUninitialized as alloy_sol_types::SolError>::SIGNATURE,
            <WrongChannelState as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidTokensReceivedUsage as alloy_sol_types::SolError>::SIGNATURE,
            <NoticePeriodNotDue as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidCurvePoint as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidSafeAddress as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidVRFProof as alloy_sol_types::SolError>::SIGNATURE,
            <SourceEqualsDestination as alloy_sol_types::SolError>::SIGNATURE,
            <AddressEmptyCode as alloy_sol_types::SolError>::SIGNATURE,
            <WrongToken as alloy_sol_types::SolError>::SIGNATURE,
            <BalanceExceedsGlobalPerChannelAllowance as alloy_sol_types::SolError>::SIGNATURE,
            <ContractNotResponsible as alloy_sol_types::SolError>::SIGNATURE,
            <InsufficientChannelBalance as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidTokenRecipient as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidBalance as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidTicketSignature as alloy_sol_types::SolError>::SIGNATURE,
            <FailedCall as alloy_sol_types::SolError>::SIGNATURE,
            <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::SIGNATURE,
            <ZeroAddress as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidPointWitness as alloy_sol_types::SolError>::SIGNATURE,
            <TicketIsNotAWin as alloy_sol_types::SolError>::SIGNATURE,
            <ECDSAInvalidSignature as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidNoticePeriod as alloy_sol_types::SolError>::SIGNATURE,
            <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for HoprChannelsErrors {
        const NAME: &'static str = "HoprChannelsErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 29usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AddressEmptyCode(_) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AlreadyInitialized(_) => {
                    <AlreadyInitialized as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BalanceExceedsGlobalPerChannelAllowance(_) => {
                    <BalanceExceedsGlobalPerChannelAllowance as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ContractNotResponsible(_) => {
                    <ContractNotResponsible as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECDSAInvalidSignature(_) => {
                    <ECDSAInvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECDSAInvalidSignatureLength(_) => {
                    <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECDSAInvalidSignatureS(_) => {
                    <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FailedCall(_) => {
                    <FailedCall as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientChannelBalance(_) => {
                    <InsufficientChannelBalance as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidBalance(_) => {
                    <InvalidBalance as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidCurvePoint(_) => {
                    <InvalidCurvePoint as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidFieldElement(_) => {
                    <InvalidFieldElement as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidNoticePeriod(_) => {
                    <InvalidNoticePeriod as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidPointWitness(_) => {
                    <InvalidPointWitness as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSafeAddress(_) => {
                    <InvalidSafeAddress as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidTicketIndex(_) => {
                    <InvalidTicketIndex as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidTicketSignature(_) => {
                    <InvalidTicketSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidTokenRecipient(_) => {
                    <InvalidTokenRecipient as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidTokensReceivedUsage(_) => {
                    <InvalidTokensReceivedUsage as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidVRFProof(_) => {
                    <InvalidVRFProof as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MultiSigUninitialized(_) => {
                    <MultiSigUninitialized as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NoticePeriodNotDue(_) => {
                    <NoticePeriodNotDue as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SourceEqualsDestination(_) => {
                    <SourceEqualsDestination as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TicketIsNotAWin(_) => {
                    <TicketIsNotAWin as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TokenTransferFailed(_) => {
                    <TokenTransferFailed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::WrongChannelState(_) => {
                    <WrongChannelState as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<HoprChannelsErrors>] = &[
                {
                    fn TokenTransferFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <TokenTransferFailed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsErrors::TokenTransferFailed)
                    }
                    TokenTransferFailed
                },
                {
                    fn AlreadyInitialized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <AlreadyInitialized as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsErrors::AlreadyInitialized)
                    }
                    AlreadyInitialized
                },
                {
                    fn InvalidTicketIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <InvalidTicketIndex as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsErrors::InvalidTicketIndex)
                    }
                    InvalidTicketIndex
                },
                {
                    fn ZeroInterval(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <ZeroInterval as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(HoprChannelsErrors::ZeroInterval)
                    }
                    ZeroInterval
                },
                {
                    fn InvalidFieldElement(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <InvalidFieldElement as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsErrors::InvalidFieldElement)
                    }
                    InvalidFieldElement
                },
                {
                    fn MultiSigUninitialized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <MultiSigUninitialized as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsErrors::MultiSigUninitialized)
                    }
                    MultiSigUninitialized
                },
                {
                    fn WrongChannelState(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <WrongChannelState as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsErrors::WrongChannelState)
                    }
                    WrongChannelState
                },
                {
                    fn InvalidTokensReceivedUsage(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <InvalidTokensReceivedUsage as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsErrors::InvalidTokensReceivedUsage)
                    }
                    InvalidTokensReceivedUsage
                },
                {
                    fn NoticePeriodNotDue(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <NoticePeriodNotDue as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsErrors::NoticePeriodNotDue)
                    }
                    NoticePeriodNotDue
                },
                {
                    fn InvalidCurvePoint(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <InvalidCurvePoint as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsErrors::InvalidCurvePoint)
                    }
                    InvalidCurvePoint
                },
                {
                    fn InvalidSafeAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <InvalidSafeAddress as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsErrors::InvalidSafeAddress)
                    }
                    InvalidSafeAddress
                },
                {
                    fn InvalidVRFProof(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <InvalidVRFProof as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsErrors::InvalidVRFProof)
                    }
                    InvalidVRFProof
                },
                {
                    fn SourceEqualsDestination(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <SourceEqualsDestination as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsErrors::SourceEqualsDestination)
                    }
                    SourceEqualsDestination
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn WrongToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <WrongToken as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(HoprChannelsErrors::WrongToken)
                    }
                    WrongToken
                },
                {
                    fn BalanceExceedsGlobalPerChannelAllowance(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <BalanceExceedsGlobalPerChannelAllowance as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                HoprChannelsErrors::BalanceExceedsGlobalPerChannelAllowance,
                            )
                    }
                    BalanceExceedsGlobalPerChannelAllowance
                },
                {
                    fn ContractNotResponsible(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <ContractNotResponsible as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsErrors::ContractNotResponsible)
                    }
                    ContractNotResponsible
                },
                {
                    fn InsufficientChannelBalance(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <InsufficientChannelBalance as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsErrors::InsufficientChannelBalance)
                    }
                    InsufficientChannelBalance
                },
                {
                    fn InvalidTokenRecipient(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <InvalidTokenRecipient as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsErrors::InvalidTokenRecipient)
                    }
                    InvalidTokenRecipient
                },
                {
                    fn InvalidBalance(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <InvalidBalance as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsErrors::InvalidBalance)
                    }
                    InvalidBalance
                },
                {
                    fn InvalidTicketSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <InvalidTicketSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsErrors::InvalidTicketSignature)
                    }
                    InvalidTicketSignature
                },
                {
                    fn FailedCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(HoprChannelsErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn ECDSAInvalidSignatureS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsErrors::ECDSAInvalidSignatureS)
                    }
                    ECDSAInvalidSignatureS
                },
                {
                    fn ZeroAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <ZeroAddress as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(HoprChannelsErrors::ZeroAddress)
                    }
                    ZeroAddress
                },
                {
                    fn InvalidPointWitness(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <InvalidPointWitness as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsErrors::InvalidPointWitness)
                    }
                    InvalidPointWitness
                },
                {
                    fn TicketIsNotAWin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <TicketIsNotAWin as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsErrors::TicketIsNotAWin)
                    }
                    TicketIsNotAWin
                },
                {
                    fn ECDSAInvalidSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsErrors::ECDSAInvalidSignature)
                    }
                    ECDSAInvalidSignature
                },
                {
                    fn InvalidNoticePeriod(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <InvalidNoticePeriod as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsErrors::InvalidNoticePeriod)
                    }
                    InvalidNoticePeriod
                },
                {
                    fn ECDSAInvalidSignatureLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(HoprChannelsErrors::ECDSAInvalidSignatureLength)
                    }
                    ECDSAInvalidSignatureLength
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
            ) -> alloy_sol_types::Result<HoprChannelsErrors>] = &[
                {
                    fn TokenTransferFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <TokenTransferFailed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsErrors::TokenTransferFailed)
                    }
                    TokenTransferFailed
                },
                {
                    fn AlreadyInitialized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <AlreadyInitialized as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsErrors::AlreadyInitialized)
                    }
                    AlreadyInitialized
                },
                {
                    fn InvalidTicketIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <InvalidTicketIndex as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsErrors::InvalidTicketIndex)
                    }
                    InvalidTicketIndex
                },
                {
                    fn ZeroInterval(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <ZeroInterval as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsErrors::ZeroInterval)
                    }
                    ZeroInterval
                },
                {
                    fn InvalidFieldElement(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <InvalidFieldElement as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsErrors::InvalidFieldElement)
                    }
                    InvalidFieldElement
                },
                {
                    fn MultiSigUninitialized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <MultiSigUninitialized as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsErrors::MultiSigUninitialized)
                    }
                    MultiSigUninitialized
                },
                {
                    fn WrongChannelState(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <WrongChannelState as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsErrors::WrongChannelState)
                    }
                    WrongChannelState
                },
                {
                    fn InvalidTokensReceivedUsage(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <InvalidTokensReceivedUsage as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsErrors::InvalidTokensReceivedUsage)
                    }
                    InvalidTokensReceivedUsage
                },
                {
                    fn NoticePeriodNotDue(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <NoticePeriodNotDue as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsErrors::NoticePeriodNotDue)
                    }
                    NoticePeriodNotDue
                },
                {
                    fn InvalidCurvePoint(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <InvalidCurvePoint as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsErrors::InvalidCurvePoint)
                    }
                    InvalidCurvePoint
                },
                {
                    fn InvalidSafeAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <InvalidSafeAddress as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsErrors::InvalidSafeAddress)
                    }
                    InvalidSafeAddress
                },
                {
                    fn InvalidVRFProof(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <InvalidVRFProof as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsErrors::InvalidVRFProof)
                    }
                    InvalidVRFProof
                },
                {
                    fn SourceEqualsDestination(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <SourceEqualsDestination as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsErrors::SourceEqualsDestination)
                    }
                    SourceEqualsDestination
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn WrongToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <WrongToken as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsErrors::WrongToken)
                    }
                    WrongToken
                },
                {
                    fn BalanceExceedsGlobalPerChannelAllowance(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <BalanceExceedsGlobalPerChannelAllowance as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                HoprChannelsErrors::BalanceExceedsGlobalPerChannelAllowance,
                            )
                    }
                    BalanceExceedsGlobalPerChannelAllowance
                },
                {
                    fn ContractNotResponsible(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <ContractNotResponsible as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsErrors::ContractNotResponsible)
                    }
                    ContractNotResponsible
                },
                {
                    fn InsufficientChannelBalance(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <InsufficientChannelBalance as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsErrors::InsufficientChannelBalance)
                    }
                    InsufficientChannelBalance
                },
                {
                    fn InvalidTokenRecipient(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <InvalidTokenRecipient as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsErrors::InvalidTokenRecipient)
                    }
                    InvalidTokenRecipient
                },
                {
                    fn InvalidBalance(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <InvalidBalance as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsErrors::InvalidBalance)
                    }
                    InvalidBalance
                },
                {
                    fn InvalidTicketSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <InvalidTicketSignature as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsErrors::InvalidTicketSignature)
                    }
                    InvalidTicketSignature
                },
                {
                    fn FailedCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn ECDSAInvalidSignatureS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsErrors::ECDSAInvalidSignatureS)
                    }
                    ECDSAInvalidSignatureS
                },
                {
                    fn ZeroAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <ZeroAddress as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsErrors::ZeroAddress)
                    }
                    ZeroAddress
                },
                {
                    fn InvalidPointWitness(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <InvalidPointWitness as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsErrors::InvalidPointWitness)
                    }
                    InvalidPointWitness
                },
                {
                    fn TicketIsNotAWin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <TicketIsNotAWin as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsErrors::TicketIsNotAWin)
                    }
                    TicketIsNotAWin
                },
                {
                    fn ECDSAInvalidSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsErrors::ECDSAInvalidSignature)
                    }
                    ECDSAInvalidSignature
                },
                {
                    fn InvalidNoticePeriod(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <InvalidNoticePeriod as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsErrors::InvalidNoticePeriod)
                    }
                    InvalidNoticePeriod
                },
                {
                    fn ECDSAInvalidSignatureLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<HoprChannelsErrors> {
                        <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(HoprChannelsErrors::ECDSAInvalidSignatureLength)
                    }
                    ECDSAInvalidSignatureLength
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
                Self::BalanceExceedsGlobalPerChannelAllowance(inner) => {
                    <BalanceExceedsGlobalPerChannelAllowance as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ContractNotResponsible(inner) => {
                    <ContractNotResponsible as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ECDSAInvalidSignature(inner) => {
                    <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ECDSAInvalidSignatureLength(inner) => {
                    <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ECDSAInvalidSignatureS(inner) => {
                    <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FailedCall(inner) => {
                    <FailedCall as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InsufficientChannelBalance(inner) => {
                    <InsufficientChannelBalance as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidBalance(inner) => {
                    <InvalidBalance as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidCurvePoint(inner) => {
                    <InvalidCurvePoint as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidFieldElement(inner) => {
                    <InvalidFieldElement as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidNoticePeriod(inner) => {
                    <InvalidNoticePeriod as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidPointWitness(inner) => {
                    <InvalidPointWitness as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidSafeAddress(inner) => {
                    <InvalidSafeAddress as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidTicketIndex(inner) => {
                    <InvalidTicketIndex as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidTicketSignature(inner) => {
                    <InvalidTicketSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidTokenRecipient(inner) => {
                    <InvalidTokenRecipient as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidTokensReceivedUsage(inner) => {
                    <InvalidTokensReceivedUsage as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidVRFProof(inner) => {
                    <InvalidVRFProof as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MultiSigUninitialized(inner) => {
                    <MultiSigUninitialized as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NoticePeriodNotDue(inner) => {
                    <NoticePeriodNotDue as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SourceEqualsDestination(inner) => {
                    <SourceEqualsDestination as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TicketIsNotAWin(inner) => {
                    <TicketIsNotAWin as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TokenTransferFailed(inner) => {
                    <TokenTransferFailed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::WrongChannelState(inner) => {
                    <WrongChannelState as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::BalanceExceedsGlobalPerChannelAllowance(inner) => {
                    <BalanceExceedsGlobalPerChannelAllowance as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::ECDSAInvalidSignature(inner) => {
                    <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ECDSAInvalidSignatureLength(inner) => {
                    <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ECDSAInvalidSignatureS(inner) => {
                    <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FailedCall(inner) => {
                    <FailedCall as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InsufficientChannelBalance(inner) => {
                    <InsufficientChannelBalance as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidBalance(inner) => {
                    <InvalidBalance as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidCurvePoint(inner) => {
                    <InvalidCurvePoint as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidFieldElement(inner) => {
                    <InvalidFieldElement as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidNoticePeriod(inner) => {
                    <InvalidNoticePeriod as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidPointWitness(inner) => {
                    <InvalidPointWitness as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InvalidTicketIndex(inner) => {
                    <InvalidTicketIndex as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidTicketSignature(inner) => {
                    <InvalidTicketSignature as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InvalidTokensReceivedUsage(inner) => {
                    <InvalidTokensReceivedUsage as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidVRFProof(inner) => {
                    <InvalidVRFProof as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::NoticePeriodNotDue(inner) => {
                    <NoticePeriodNotDue as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SourceEqualsDestination(inner) => {
                    <SourceEqualsDestination as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TicketIsNotAWin(inner) => {
                    <TicketIsNotAWin as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TokenTransferFailed(inner) => {
                    <TokenTransferFailed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::WrongChannelState(inner) => {
                    <WrongChannelState as alloy_sol_types::SolError>::abi_encode_raw(
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
    ///Container for all the [`HoprChannels`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum HoprChannelsEvents {
        #[allow(missing_docs)]
        ChannelBalanceDecreased(ChannelBalanceDecreased),
        #[allow(missing_docs)]
        ChannelBalanceIncreased(ChannelBalanceIncreased),
        #[allow(missing_docs)]
        ChannelClosed(ChannelClosed),
        #[allow(missing_docs)]
        ChannelOpened(ChannelOpened),
        #[allow(missing_docs)]
        DomainSeparatorUpdated(DomainSeparatorUpdated),
        #[allow(missing_docs)]
        LedgerDomainSeparatorUpdated(LedgerDomainSeparatorUpdated),
        #[allow(missing_docs)]
        OutgoingChannelClosureInitiated(OutgoingChannelClosureInitiated),
        #[allow(missing_docs)]
        TicketRedeemed(TicketRedeemed),
    }
    impl HoprChannelsEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                0u8, 76u8, 1u8, 119u8, 173u8, 21u8, 187u8, 48u8, 43u8, 68u8, 25u8, 233u8,
                172u8, 150u8, 6u8, 93u8, 111u8, 67u8, 110u8, 15u8, 97u8, 151u8, 47u8,
                151u8, 187u8, 226u8, 114u8, 131u8, 76u8, 64u8, 249u8, 52u8,
            ],
            [
                0u8, 97u8, 232u8, 3u8, 113u8, 151u8, 180u8, 217u8, 28u8, 248u8, 173u8,
                210u8, 160u8, 115u8, 102u8, 19u8, 69u8, 144u8, 129u8, 242u8, 54u8, 209u8,
                200u8, 171u8, 88u8, 84u8, 158u8, 187u8, 195u8, 48u8, 180u8, 227u8,
            ],
            [
                12u8, 70u8, 114u8, 241u8, 75u8, 99u8, 187u8, 99u8, 84u8, 250u8, 196u8,
                117u8, 238u8, 73u8, 128u8, 85u8, 162u8, 120u8, 68u8, 85u8, 103u8, 58u8,
                242u8, 36u8, 113u8, 123u8, 151u8, 112u8, 253u8, 104u8, 216u8, 209u8,
            ],
            [
                25u8, 123u8, 186u8, 104u8, 74u8, 106u8, 251u8, 123u8, 162u8, 79u8, 29u8,
                38u8, 86u8, 5u8, 65u8, 75u8, 29u8, 0u8, 81u8, 166u8, 226u8, 149u8, 214u8,
                255u8, 123u8, 110u8, 120u8, 232u8, 112u8, 215u8, 167u8, 240u8,
            ],
            [
                119u8, 31u8, 82u8, 64u8, 174u8, 95u8, 216u8, 167u8, 100u8, 13u8, 63u8,
                184u8, 47u8, 167u8, 10u8, 171u8, 47u8, 177u8, 219u8, 243u8, 95u8, 46u8,
                244u8, 100u8, 248u8, 80u8, 153u8, 70u8, 113u8, 118u8, 100u8, 197u8,
            ],
            [
                164u8, 63u8, 173u8, 131u8, 146u8, 15u8, 208u8, 148u8, 69u8, 133u8, 94u8,
                133u8, 78u8, 115u8, 201u8, 197u8, 50u8, 225u8, 116u8, 2u8, 201u8, 206u8,
                176u8, 153u8, 147u8, 162u8, 57u8, 40u8, 67u8, 165u8, 189u8, 185u8,
            ],
            [
                236u8, 33u8, 84u8, 124u8, 161u8, 226u8, 46u8, 220u8, 63u8, 155u8, 79u8,
                78u8, 13u8, 169u8, 70u8, 56u8, 181u8, 185u8, 77u8, 203u8, 24u8, 213u8,
                45u8, 204u8, 7u8, 42u8, 190u8, 88u8, 1u8, 168u8, 1u8, 60u8,
            ],
            [
                237u8, 34u8, 243u8, 77u8, 21u8, 77u8, 37u8, 58u8, 127u8, 111u8, 212u8,
                119u8, 67u8, 155u8, 229u8, 144u8, 128u8, 161u8, 85u8, 74u8, 160u8, 208u8,
                100u8, 38u8, 134u8, 214u8, 66u8, 35u8, 171u8, 84u8, 76u8, 138u8,
            ],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(ChannelOpened),
            ::core::stringify!(OutgoingChannelClosureInitiated),
            ::core::stringify!(ChannelBalanceDecreased),
            ::core::stringify!(ChannelClosed),
            ::core::stringify!(DomainSeparatorUpdated),
            ::core::stringify!(LedgerDomainSeparatorUpdated),
            ::core::stringify!(ChannelBalanceIncreased),
            ::core::stringify!(TicketRedeemed),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <ChannelOpened as alloy_sol_types::SolEvent>::SIGNATURE,
            <OutgoingChannelClosureInitiated as alloy_sol_types::SolEvent>::SIGNATURE,
            <ChannelBalanceDecreased as alloy_sol_types::SolEvent>::SIGNATURE,
            <ChannelClosed as alloy_sol_types::SolEvent>::SIGNATURE,
            <DomainSeparatorUpdated as alloy_sol_types::SolEvent>::SIGNATURE,
            <LedgerDomainSeparatorUpdated as alloy_sol_types::SolEvent>::SIGNATURE,
            <ChannelBalanceIncreased as alloy_sol_types::SolEvent>::SIGNATURE,
            <TicketRedeemed as alloy_sol_types::SolEvent>::SIGNATURE,
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
    impl alloy_sol_types::SolEventInterface for HoprChannelsEvents {
        const NAME: &'static str = "HoprChannelsEvents";
        const COUNT: usize = 8usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <ChannelBalanceDecreased as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ChannelBalanceDecreased as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ChannelBalanceDecreased)
                }
                Some(
                    <ChannelBalanceIncreased as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ChannelBalanceIncreased as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ChannelBalanceIncreased)
                }
                Some(<ChannelClosed as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ChannelClosed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ChannelClosed)
                }
                Some(<ChannelOpened as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ChannelOpened as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ChannelOpened)
                }
                Some(
                    <DomainSeparatorUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <DomainSeparatorUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::DomainSeparatorUpdated)
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
                    <OutgoingChannelClosureInitiated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OutgoingChannelClosureInitiated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OutgoingChannelClosureInitiated)
                }
                Some(<TicketRedeemed as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <TicketRedeemed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::TicketRedeemed)
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
    impl alloy_sol_types::private::IntoLogData for HoprChannelsEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ChannelBalanceDecreased(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ChannelBalanceIncreased(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ChannelClosed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ChannelOpened(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::DomainSeparatorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::LedgerDomainSeparatorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OutgoingChannelClosureInitiated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TicketRedeemed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ChannelBalanceDecreased(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ChannelBalanceIncreased(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ChannelClosed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ChannelOpened(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::DomainSeparatorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::LedgerDomainSeparatorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OutgoingChannelClosureInitiated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TicketRedeemed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`HoprChannels`](self) contract instance.

See the [wrapper's documentation](`HoprChannelsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> HoprChannelsInstance<P, N> {
        HoprChannelsInstance::<P, N>::new(address, __provider)
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
        _token: alloy::sol_types::private::Address,
        _noticePeriodChannelClosure: <HoprChannelsType::Timestamp as alloy::sol_types::SolType>::RustType,
        _safeRegistry: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<HoprChannelsInstance<P, N>>,
    > {
        HoprChannelsInstance::<
            P,
            N,
        >::deploy(__provider, _token, _noticePeriodChannelClosure, _safeRegistry)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        __provider: P,
        _token: alloy::sol_types::private::Address,
        _noticePeriodChannelClosure: <HoprChannelsType::Timestamp as alloy::sol_types::SolType>::RustType,
        _safeRegistry: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        HoprChannelsInstance::<
            P,
            N,
        >::deploy_builder(__provider, _token, _noticePeriodChannelClosure, _safeRegistry)
    }
    /**A [`HoprChannels`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`HoprChannels`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct HoprChannelsInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for HoprChannelsInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("HoprChannelsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > HoprChannelsInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`HoprChannels`](self) contract instance.

See the [wrapper's documentation](`HoprChannelsInstance`) for more details.*/
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
            _token: alloy::sol_types::private::Address,
            _noticePeriodChannelClosure: <HoprChannelsType::Timestamp as alloy::sol_types::SolType>::RustType,
            _safeRegistry: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<HoprChannelsInstance<P, N>> {
            let call_builder = Self::deploy_builder(
                __provider,
                _token,
                _noticePeriodChannelClosure,
                _safeRegistry,
            );
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            __provider: P,
            _token: alloy::sol_types::private::Address,
            _noticePeriodChannelClosure: <HoprChannelsType::Timestamp as alloy::sol_types::SolType>::RustType,
            _safeRegistry: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                __provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _token,
                            _noticePeriodChannelClosure,
                            _safeRegistry,
                        },
                    )[..],
                ]
                    .concat()
                    .into(),
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
    impl<P: ::core::clone::Clone, N> HoprChannelsInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> HoprChannelsInstance<P, N> {
            HoprChannelsInstance {
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
    > HoprChannelsInstance<P, N> {
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
        ///Creates a new call builder for the [`ERC777_HOOK_FUND_CHANNEL_MULTI_SIZE`] function.
        pub fn ERC777_HOOK_FUND_CHANNEL_MULTI_SIZE(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            ERC777_HOOK_FUND_CHANNEL_MULTI_SIZECall,
            N,
        > {
            self.call_builder(&ERC777_HOOK_FUND_CHANNEL_MULTI_SIZECall)
        }
        ///Creates a new call builder for the [`ERC777_HOOK_FUND_CHANNEL_SIZE`] function.
        pub fn ERC777_HOOK_FUND_CHANNEL_SIZE(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, ERC777_HOOK_FUND_CHANNEL_SIZECall, N> {
            self.call_builder(&ERC777_HOOK_FUND_CHANNEL_SIZECall)
        }
        ///Creates a new call builder for the [`LEDGER_VERSION`] function.
        pub fn LEDGER_VERSION(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, LEDGER_VERSIONCall, N> {
            self.call_builder(&LEDGER_VERSIONCall)
        }
        ///Creates a new call builder for the [`MAX_USED_BALANCE`] function.
        pub fn MAX_USED_BALANCE(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, MAX_USED_BALANCECall, N> {
            self.call_builder(&MAX_USED_BALANCECall)
        }
        ///Creates a new call builder for the [`MIN_USED_BALANCE`] function.
        pub fn MIN_USED_BALANCE(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, MIN_USED_BALANCECall, N> {
            self.call_builder(&MIN_USED_BALANCECall)
        }
        ///Creates a new call builder for the [`NOTICE_PERIOD_CHANNEL_CLOSURE`] function.
        pub fn NOTICE_PERIOD_CHANNEL_CLOSURE(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, NOTICE_PERIOD_CHANNEL_CLOSURECall, N> {
            self.call_builder(&NOTICE_PERIOD_CHANNEL_CLOSURECall)
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
        ///Creates a new call builder for the [`VERSION`] function.
        pub fn VERSION(&self) -> alloy_contract::SolCallBuilder<&P, VERSIONCall, N> {
            self.call_builder(&VERSIONCall)
        }
        ///Creates a new call builder for the [`_currentBlockTimestamp`] function.
        pub fn _currentBlockTimestamp(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, _currentBlockTimestampCall, N> {
            self.call_builder(&_currentBlockTimestampCall)
        }
        ///Creates a new call builder for the [`_getChannelId`] function.
        pub fn _getChannelId(
            &self,
            source: alloy::sol_types::private::Address,
            destination: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, _getChannelIdCall, N> {
            self.call_builder(
                &_getChannelIdCall {
                    source,
                    destination,
                },
            )
        }
        ///Creates a new call builder for the [`_getTicketHash`] function.
        pub fn _getTicketHash(
            &self,
            redeemable: <RedeemableTicket as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, _getTicketHashCall, N> {
            self.call_builder(&_getTicketHashCall { redeemable })
        }
        ///Creates a new call builder for the [`_isWinningTicket`] function.
        pub fn _isWinningTicket(
            &self,
            ticketHash: alloy::sol_types::private::FixedBytes<32>,
            redeemable: <RedeemableTicket as alloy::sol_types::SolType>::RustType,
            params: <HoprCrypto::VRFParameters as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, _isWinningTicketCall, N> {
            self.call_builder(
                &_isWinningTicketCall {
                    ticketHash,
                    redeemable,
                    params,
                },
            )
        }
        ///Creates a new call builder for the [`channelState`] function.
        pub fn channelState(
            &self,
            channelId: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, channelStateCall, N> {
            self.call_builder(&channelStateCall { channelId })
        }
        ///Creates a new call builder for the [`channels`] function.
        pub fn channels(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, channelsCall, N> {
            self.call_builder(&channelsCall(_0))
        }
        ///Creates a new call builder for the [`closeIncomingChannel`] function.
        pub fn closeIncomingChannel(
            &self,
            source: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, closeIncomingChannelCall, N> {
            self.call_builder(&closeIncomingChannelCall { source })
        }
        ///Creates a new call builder for the [`closeIncomingChannelSafe`] function.
        pub fn closeIncomingChannelSafe(
            &self,
            selfAddress: alloy::sol_types::private::Address,
            source: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, closeIncomingChannelSafeCall, N> {
            self.call_builder(
                &closeIncomingChannelSafeCall {
                    selfAddress,
                    source,
                },
            )
        }
        ///Creates a new call builder for the [`domainSeparator`] function.
        pub fn domainSeparator(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, domainSeparatorCall, N> {
            self.call_builder(&domainSeparatorCall)
        }
        ///Creates a new call builder for the [`finalizeOutgoingChannelClosure`] function.
        pub fn finalizeOutgoingChannelClosure(
            &self,
            destination: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, finalizeOutgoingChannelClosureCall, N> {
            self.call_builder(
                &finalizeOutgoingChannelClosureCall {
                    destination,
                },
            )
        }
        ///Creates a new call builder for the [`finalizeOutgoingChannelClosureSafe`] function.
        pub fn finalizeOutgoingChannelClosureSafe(
            &self,
            selfAddress: alloy::sol_types::private::Address,
            destination: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            finalizeOutgoingChannelClosureSafeCall,
            N,
        > {
            self.call_builder(
                &finalizeOutgoingChannelClosureSafeCall {
                    selfAddress,
                    destination,
                },
            )
        }
        ///Creates a new call builder for the [`fundChannel`] function.
        pub fn fundChannel(
            &self,
            account: alloy::sol_types::private::Address,
            amount: <HoprChannelsType::Balance as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, fundChannelCall, N> {
            self.call_builder(&fundChannelCall { account, amount })
        }
        ///Creates a new call builder for the [`fundChannelSafe`] function.
        pub fn fundChannelSafe(
            &self,
            selfAddress: alloy::sol_types::private::Address,
            account: alloy::sol_types::private::Address,
            amount: <HoprChannelsType::Balance as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, fundChannelSafeCall, N> {
            self.call_builder(
                &fundChannelSafeCall {
                    selfAddress,
                    account,
                    amount,
                },
            )
        }
        ///Creates a new call builder for the [`initiateOutgoingChannelClosure`] function.
        pub fn initiateOutgoingChannelClosure(
            &self,
            destination: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, initiateOutgoingChannelClosureCall, N> {
            self.call_builder(
                &initiateOutgoingChannelClosureCall {
                    destination,
                },
            )
        }
        ///Creates a new call builder for the [`initiateOutgoingChannelClosureSafe`] function.
        pub fn initiateOutgoingChannelClosureSafe(
            &self,
            selfAddress: alloy::sol_types::private::Address,
            destination: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            initiateOutgoingChannelClosureSafeCall,
            N,
        > {
            self.call_builder(
                &initiateOutgoingChannelClosureSafeCall {
                    selfAddress,
                    destination,
                },
            )
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
        ///Creates a new call builder for the [`multicall`] function.
        pub fn multicall(
            &self,
            data: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
        ) -> alloy_contract::SolCallBuilder<&P, multicallCall, N> {
            self.call_builder(&multicallCall { data })
        }
        ///Creates a new call builder for the [`redeemTicket`] function.
        pub fn redeemTicket(
            &self,
            redeemable: <RedeemableTicket as alloy::sol_types::SolType>::RustType,
            params: <HoprCrypto::VRFParameters as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, redeemTicketCall, N> {
            self.call_builder(
                &redeemTicketCall {
                    redeemable,
                    params,
                },
            )
        }
        ///Creates a new call builder for the [`redeemTicketSafe`] function.
        pub fn redeemTicketSafe(
            &self,
            selfAddress: alloy::sol_types::private::Address,
            redeemable: <RedeemableTicket as alloy::sol_types::SolType>::RustType,
            params: <HoprCrypto::VRFParameters as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, redeemTicketSafeCall, N> {
            self.call_builder(
                &redeemTicketSafeCall {
                    selfAddress,
                    redeemable,
                    params,
                },
            )
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
        ///Creates a new call builder for the [`updateDomainSeparator`] function.
        pub fn updateDomainSeparator(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, updateDomainSeparatorCall, N> {
            self.call_builder(&updateDomainSeparatorCall)
        }
        ///Creates a new call builder for the [`updateLedgerDomainSeparator`] function.
        pub fn updateLedgerDomainSeparator(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, updateLedgerDomainSeparatorCall, N> {
            self.call_builder(&updateLedgerDomainSeparatorCall)
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > HoprChannelsInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`ChannelBalanceDecreased`] event.
        pub fn ChannelBalanceDecreased_filter(
            &self,
        ) -> alloy_contract::Event<&P, ChannelBalanceDecreased, N> {
            self.event_filter::<ChannelBalanceDecreased>()
        }
        ///Creates a new event filter for the [`ChannelBalanceIncreased`] event.
        pub fn ChannelBalanceIncreased_filter(
            &self,
        ) -> alloy_contract::Event<&P, ChannelBalanceIncreased, N> {
            self.event_filter::<ChannelBalanceIncreased>()
        }
        ///Creates a new event filter for the [`ChannelClosed`] event.
        pub fn ChannelClosed_filter(
            &self,
        ) -> alloy_contract::Event<&P, ChannelClosed, N> {
            self.event_filter::<ChannelClosed>()
        }
        ///Creates a new event filter for the [`ChannelOpened`] event.
        pub fn ChannelOpened_filter(
            &self,
        ) -> alloy_contract::Event<&P, ChannelOpened, N> {
            self.event_filter::<ChannelOpened>()
        }
        ///Creates a new event filter for the [`DomainSeparatorUpdated`] event.
        pub fn DomainSeparatorUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, DomainSeparatorUpdated, N> {
            self.event_filter::<DomainSeparatorUpdated>()
        }
        ///Creates a new event filter for the [`LedgerDomainSeparatorUpdated`] event.
        pub fn LedgerDomainSeparatorUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, LedgerDomainSeparatorUpdated, N> {
            self.event_filter::<LedgerDomainSeparatorUpdated>()
        }
        ///Creates a new event filter for the [`OutgoingChannelClosureInitiated`] event.
        pub fn OutgoingChannelClosureInitiated_filter(
            &self,
        ) -> alloy_contract::Event<&P, OutgoingChannelClosureInitiated, N> {
            self.event_filter::<OutgoingChannelClosureInitiated>()
        }
        ///Creates a new event filter for the [`TicketRedeemed`] event.
        pub fn TicketRedeemed_filter(
            &self,
        ) -> alloy_contract::Event<&P, TicketRedeemed, N> {
            self.event_filter::<TicketRedeemed>()
        }
    }
}
