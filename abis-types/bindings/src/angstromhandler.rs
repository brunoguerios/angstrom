///Module containing a contract's types and functions.
/**

```solidity
library StdInvariant {
    struct FuzzArtifactSelector { string artifact; bytes4[] selectors; }
    struct FuzzInterface { address addr; string[] artifacts; }
    struct FuzzSelector { address addr; bytes4[] selectors; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod StdInvariant {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct FuzzArtifactSelector { string artifact; bytes4[] selectors; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FuzzArtifactSelector {
        #[allow(missing_docs)]
        pub artifact: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub selectors: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<4>,
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
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::String,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<4>>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::String,
            alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<4>>,
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
        impl ::core::convert::From<FuzzArtifactSelector> for UnderlyingRustTuple<'_> {
            fn from(value: FuzzArtifactSelector) -> Self {
                (value.artifact, value.selectors)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FuzzArtifactSelector {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    artifact: tuple.0,
                    selectors: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for FuzzArtifactSelector {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for FuzzArtifactSelector {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.artifact,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::SolType>::tokenize(&self.selectors),
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
        impl alloy_sol_types::SolType for FuzzArtifactSelector {
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
        impl alloy_sol_types::SolStruct for FuzzArtifactSelector {
            const NAME: &'static str = "FuzzArtifactSelector";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "FuzzArtifactSelector(string artifact,bytes4[] selectors)",
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.artifact,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.selectors)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for FuzzArtifactSelector {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.artifact,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.selectors,
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
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.artifact,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<4>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.selectors,
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
struct FuzzInterface { address addr; string[] artifacts; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FuzzInterface {
        #[allow(missing_docs)]
        pub addr: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub artifacts: alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
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
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
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
        impl ::core::convert::From<FuzzInterface> for UnderlyingRustTuple<'_> {
            fn from(value: FuzzInterface) -> Self {
                (value.addr, value.artifacts)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FuzzInterface {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    addr: tuple.0,
                    artifacts: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for FuzzInterface {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for FuzzInterface {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.addr,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::tokenize(&self.artifacts),
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
        impl alloy_sol_types::SolType for FuzzInterface {
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
        impl alloy_sol_types::SolStruct for FuzzInterface {
            const NAME: &'static str = "FuzzInterface";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "FuzzInterface(address addr,string[] artifacts)",
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.addr,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.artifacts)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for FuzzInterface {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.addr,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.artifacts,
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
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.addr,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::String,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.artifacts,
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
struct FuzzSelector { address addr; bytes4[] selectors; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FuzzSelector {
        #[allow(missing_docs)]
        pub addr: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub selectors: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<4>,
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
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<4>>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<4>>,
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
        impl ::core::convert::From<FuzzSelector> for UnderlyingRustTuple<'_> {
            fn from(value: FuzzSelector) -> Self {
                (value.addr, value.selectors)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FuzzSelector {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    addr: tuple.0,
                    selectors: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for FuzzSelector {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for FuzzSelector {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.addr,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::SolType>::tokenize(&self.selectors),
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
        impl alloy_sol_types::SolType for FuzzSelector {
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
        impl alloy_sol_types::SolStruct for FuzzSelector {
            const NAME: &'static str = "FuzzSelector";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "FuzzSelector(address addr,bytes4[] selectors)",
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.addr,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.selectors)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for FuzzSelector {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.addr,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.selectors,
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
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.addr,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<4>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.selectors,
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
    /**Creates a new wrapper around an on-chain [`StdInvariant`](self) contract instance.

See the [wrapper's documentation](`StdInvariantInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> StdInvariantInstance<T, P, N> {
        StdInvariantInstance::<T, P, N>::new(address, provider)
    }
    /**A [`StdInvariant`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`StdInvariant`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct StdInvariantInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for StdInvariantInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("StdInvariantInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > StdInvariantInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`StdInvariant`](self) contract instance.

See the [wrapper's documentation](`StdInvariantInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
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
    impl<T, P: ::core::clone::Clone, N> StdInvariantInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> StdInvariantInstance<T, P, N> {
            StdInvariantInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > StdInvariantInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > StdInvariantInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library StdInvariant {
    struct FuzzArtifactSelector {
        string artifact;
        bytes4[] selectors;
    }
    struct FuzzInterface {
        address addr;
        string[] artifacts;
    }
    struct FuzzSelector {
        address addr;
        bytes4[] selectors;
    }
}

interface AngstromHandler {
    type PoolId is bytes32;
    type PositionKey is bytes32;
    type PriceAB is uint256;
    struct Env {
        address owner;
        address controller;
        address feeMaster;
        address uniV4;
        address angstrom;
        address[] assets;
        address[] mirrors;
    }
    struct LiquidityAdd {
        uint256 liquidity;
        uint256 rewardStartIndex;
        uint256 rewardEndIndex;
    }
    struct LiquidityPosition {
        int24 lowerTick;
        int24 upperTick;
        address owner;
        uint256 totalRewardsX128;
        uint256 claimedRewards;
        uint256 totalLiquidity;
        uint256 activeAddsOffset;
        LiquidityAdd[] adds;
    }
    struct PRNG {
        uint256 __state;
    }
    struct Pair {
        address asset0;
        address asset1;
        PriceAB price10;
    }
    struct TickReward {
        int24 tick;
        uint128 amount;
    }

    error IndexOutOfBounds();
    error InvalidBounds();
    error OutOfBoundsVecGet();
    error Overflow();
    error PairAssetsWrong(Pair);
    error Underflow();

    event log(string);
    event log_address(address);
    event log_array(uint256[] val);
    event log_array(int256[] val);
    event log_array(address[] val);
    event log_bytes(bytes);
    event log_bytes32(bytes32);
    event log_int(int256);
    event log_named_address(string key, address val);
    event log_named_array(string key, uint256[] val);
    event log_named_array(string key, int256[] val);
    event log_named_array(string key, address[] val);
    event log_named_bytes(string key, bytes val);
    event log_named_bytes32(string key, bytes32 val);
    event log_named_decimal_int(string key, int256 val, uint256 decimals);
    event log_named_decimal_uint(string key, uint256 val, uint256 decimals);
    event log_named_int(string key, int256 val);
    event log_named_string(string key, string val);
    event log_named_uint(string key, uint256 val);
    event log_string(string);
    event log_uint(uint256);
    event logs(bytes);

    constructor(Env env);

    function IS_TEST() external view returns (bool);
    function __safeAdd(uint256 x, uint256 y) external pure returns (uint256);
    function __safeDiv(uint256 x, uint256 y) external pure returns (uint256);
    function __safeMod(uint256 x, uint256 y) external pure returns (uint256);
    function __safeMul(uint256 x, uint256 y) external pure returns (uint256);
    function __safeSub(uint256 x, uint256 y) external pure returns (uint256);
    function actors() external view returns (address[] memory);
    function addLiquidity(uint256 poolIndex, uint256 routerIndex, int24 lowerTick, int24 upperTick, uint256 liquidity) external;
    function enabledAssets() external view returns (address[] memory);
    function excludeArtifacts() external view returns (string[] memory excludedArtifacts_);
    function excludeContracts() external view returns (address[] memory excludedContracts_);
    function excludeSelectors() external view returns (StdInvariant.FuzzSelector[] memory excludedSelectors_);
    function excludeSenders() external view returns (address[] memory excludedSenders_);
    function failed() external view returns (bool);
    function getPool(uint256 poolIndex) external view returns (address asset0, address asset1, int24 tickSpacing);
    function getPosition(uint256 poolIndex, PositionKey key) external view returns (LiquidityPosition memory);
    function ghost_claimedLpRewards(address asset) external view returns (uint256 rewards);
    function ghost_netSavedDeltas(address asset) external view returns (int256 saved);
    function ghost_totalDeposits(address asset) external view returns (uint256 total);
    function ghost_totalLpRewards(address asset) external view returns (uint256 rewards);
    function ghost_unclaimableRewards(address asset) external view returns (uint256 rewards);
    function initializePool(uint256 asset0Index, uint256 asset1Index, int24 tickSpacing, uint24 bundleFee, uint24 unlockedFee, uint160 startSqrtPriceX96) external;
    function poolIndexToId(uint256 poolIndex) external view returns (PoolId);
    function positionKeys(uint256 poolIndex) external view returns (PositionKey[] memory keys);
    function removeLiquidity(uint256 poolIndex, uint256 liquidityRelativeIndex, uint256 liquidityToRemove) external;
    function rewardTicks(uint256 poolIndex, uint256 ticksToReward, PRNG memory rng) external;
    function routers() external view returns (address[] memory);
    function targetArtifactSelectors() external view returns (StdInvariant.FuzzArtifactSelector[] memory targetedArtifactSelectors_);
    function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
    function targetContracts() external view returns (address[] memory targetedContracts_);
    function targetInterfaces() external view returns (StdInvariant.FuzzInterface[] memory targetedInterfaces_);
    function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
    function targetSenders() external view returns (address[] memory targetedSenders_);
    function tickRewards(uint256 poolIndex) external view returns (TickReward[] memory);
    function totalPools() external view returns (uint256);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "env",
        "type": "tuple",
        "internalType": "struct Env",
        "components": [
          {
            "name": "owner",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "controller",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "feeMaster",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "uniV4",
            "type": "address",
            "internalType": "contract UniV4Inspector"
          },
          {
            "name": "angstrom",
            "type": "address",
            "internalType": "contract OpenAngstrom"
          },
          {
            "name": "assets",
            "type": "address[]",
            "internalType": "contract MockERC20[]"
          },
          {
            "name": "mirrors",
            "type": "address[]",
            "internalType": "contract MockERC20[]"
          }
        ]
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "IS_TEST",
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
    "name": "__safeAdd",
    "inputs": [
      {
        "name": "x",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "y",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "__safeDiv",
    "inputs": [
      {
        "name": "x",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "y",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "__safeMod",
    "inputs": [
      {
        "name": "x",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "y",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "__safeMul",
    "inputs": [
      {
        "name": "x",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "y",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "__safeSub",
    "inputs": [
      {
        "name": "x",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "y",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "actors",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "addLiquidity",
    "inputs": [
      {
        "name": "poolIndex",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "routerIndex",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "lowerTick",
        "type": "int24",
        "internalType": "int24"
      },
      {
        "name": "upperTick",
        "type": "int24",
        "internalType": "int24"
      },
      {
        "name": "liquidity",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "enabledAssets",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "excludeArtifacts",
    "inputs": [],
    "outputs": [
      {
        "name": "excludedArtifacts_",
        "type": "string[]",
        "internalType": "string[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "excludeContracts",
    "inputs": [],
    "outputs": [
      {
        "name": "excludedContracts_",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "excludeSelectors",
    "inputs": [],
    "outputs": [
      {
        "name": "excludedSelectors_",
        "type": "tuple[]",
        "internalType": "struct StdInvariant.FuzzSelector[]",
        "components": [
          {
            "name": "addr",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "selectors",
            "type": "bytes4[]",
            "internalType": "bytes4[]"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "excludeSenders",
    "inputs": [],
    "outputs": [
      {
        "name": "excludedSenders_",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "failed",
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
    "name": "getPool",
    "inputs": [
      {
        "name": "poolIndex",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "asset0",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "asset1",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "tickSpacing",
        "type": "int24",
        "internalType": "int24"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getPosition",
    "inputs": [
      {
        "name": "poolIndex",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "key",
        "type": "bytes32",
        "internalType": "PositionKey"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct LiquidityPosition",
        "components": [
          {
            "name": "lowerTick",
            "type": "int24",
            "internalType": "int24"
          },
          {
            "name": "upperTick",
            "type": "int24",
            "internalType": "int24"
          },
          {
            "name": "owner",
            "type": "address",
            "internalType": "contract RouterActor"
          },
          {
            "name": "totalRewardsX128",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "claimedRewards",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "totalLiquidity",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "activeAddsOffset",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "adds",
            "type": "tuple[]",
            "internalType": "struct LiquidityAdd[]",
            "components": [
              {
                "name": "liquidity",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "rewardStartIndex",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "rewardEndIndex",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "ghost_claimedLpRewards",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "rewards",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "ghost_netSavedDeltas",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "saved",
        "type": "int256",
        "internalType": "int256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "ghost_totalDeposits",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "total",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "ghost_totalLpRewards",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "rewards",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "ghost_unclaimableRewards",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "rewards",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initializePool",
    "inputs": [
      {
        "name": "asset0Index",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "asset1Index",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "tickSpacing",
        "type": "int24",
        "internalType": "int24"
      },
      {
        "name": "bundleFee",
        "type": "uint24",
        "internalType": "uint24"
      },
      {
        "name": "unlockedFee",
        "type": "uint24",
        "internalType": "uint24"
      },
      {
        "name": "startSqrtPriceX96",
        "type": "uint160",
        "internalType": "uint160"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "poolIndexToId",
    "inputs": [
      {
        "name": "poolIndex",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "PoolId"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "positionKeys",
    "inputs": [
      {
        "name": "poolIndex",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "keys",
        "type": "bytes32[]",
        "internalType": "PositionKey[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "removeLiquidity",
    "inputs": [
      {
        "name": "poolIndex",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "liquidityRelativeIndex",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "liquidityToRemove",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "rewardTicks",
    "inputs": [
      {
        "name": "poolIndex",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "ticksToReward",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "rng",
        "type": "tuple",
        "internalType": "struct PRNG",
        "components": [
          {
            "name": "__state",
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
    "name": "routers",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetArtifactSelectors",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedArtifactSelectors_",
        "type": "tuple[]",
        "internalType": "struct StdInvariant.FuzzArtifactSelector[]",
        "components": [
          {
            "name": "artifact",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "selectors",
            "type": "bytes4[]",
            "internalType": "bytes4[]"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetArtifacts",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedArtifacts_",
        "type": "string[]",
        "internalType": "string[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetContracts",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedContracts_",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetInterfaces",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedInterfaces_",
        "type": "tuple[]",
        "internalType": "struct StdInvariant.FuzzInterface[]",
        "components": [
          {
            "name": "addr",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "artifacts",
            "type": "string[]",
            "internalType": "string[]"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetSelectors",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedSelectors_",
        "type": "tuple[]",
        "internalType": "struct StdInvariant.FuzzSelector[]",
        "components": [
          {
            "name": "addr",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "selectors",
            "type": "bytes4[]",
            "internalType": "bytes4[]"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetSenders",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedSenders_",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "tickRewards",
    "inputs": [
      {
        "name": "poolIndex",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct TickReward[]",
        "components": [
          {
            "name": "tick",
            "type": "int24",
            "internalType": "int24"
          },
          {
            "name": "amount",
            "type": "uint128",
            "internalType": "uint128"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "totalPools",
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
    "type": "event",
    "name": "log",
    "inputs": [
      {
        "name": "",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_address",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_array",
    "inputs": [
      {
        "name": "val",
        "type": "uint256[]",
        "indexed": false,
        "internalType": "uint256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_array",
    "inputs": [
      {
        "name": "val",
        "type": "int256[]",
        "indexed": false,
        "internalType": "int256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_array",
    "inputs": [
      {
        "name": "val",
        "type": "address[]",
        "indexed": false,
        "internalType": "address[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_bytes",
    "inputs": [
      {
        "name": "",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_bytes32",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_int",
    "inputs": [
      {
        "name": "",
        "type": "int256",
        "indexed": false,
        "internalType": "int256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_address",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_array",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "uint256[]",
        "indexed": false,
        "internalType": "uint256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_array",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "int256[]",
        "indexed": false,
        "internalType": "int256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_array",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "address[]",
        "indexed": false,
        "internalType": "address[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_bytes",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_bytes32",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_decimal_int",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "int256",
        "indexed": false,
        "internalType": "int256"
      },
      {
        "name": "decimals",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_decimal_uint",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "decimals",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_int",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "int256",
        "indexed": false,
        "internalType": "int256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_string",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_uint",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_string",
    "inputs": [
      {
        "name": "",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_uint",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "logs",
    "inputs": [
      {
        "name": "",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "IndexOutOfBounds",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidBounds",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OutOfBoundsVecGet",
    "inputs": []
  },
  {
    "type": "error",
    "name": "Overflow",
    "inputs": []
  },
  {
    "type": "error",
    "name": "PairAssetsWrong",
    "inputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct Pair",
        "components": [
          {
            "name": "asset0",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "asset1",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "price10",
            "type": "uint256",
            "internalType": "PriceAB"
          }
        ]
      }
    ]
  },
  {
    "type": "error",
    "name": "Underflow",
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
pub mod AngstromHandler {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6080604052600c8054600160ff199182168117909255601f8054909116909117905534801561002c575f80fd5b5060405161ad0438038061ad0483398101604081905261004b9161028c565b8051602080546001600160a01b03199081166001600160a01b03938416178255818401516021805483169185169190911790556040840151602280548316918516919091179055606084015160238054831691851691909117905560808401516024805490921693169290921790915560a082015180518392916100d4916025918401906100fa565b5060c082015180516100f09160068401916020909101906100fa565b5090505050610375565b828054828255905f5260205f2090810192821561014d579160200282015b8281111561014d57825182546001600160a01b0319166001600160a01b03909116178255602090920191600190910190610118565b5061015992915061015d565b5090565b5b80821115610159575f815560010161015e565b634e487b7160e01b5f52604160045260245ffd5b60405160e081016001600160401b03811182821017156101a7576101a7610171565b60405290565b604051601f8201601f191681016001600160401b03811182821017156101d5576101d5610171565b604052919050565b6001600160a01b03811681146101f1575f80fd5b50565b80516101ff816101dd565b919050565b5f82601f830112610213575f80fd5b81516001600160401b0381111561022c5761022c610171565b8060051b61023c602082016101ad565b91825260208185018101929081019086841115610257575f80fd5b6020860192505b83831015610282578251610271816101dd565b82526020928301929091019061025e565b9695505050505050565b5f6020828403121561029c575f80fd5b81516001600160401b038111156102b1575f80fd5b820160e081850312156102c2575f80fd5b6102ca610185565b6102d3826101f4565b81526102e1602083016101f4565b60208201526102f2604083016101f4565b6040820152610303606083016101f4565b6060820152610314608083016101f4565b608082015260a08201516001600160401b03811115610331575f80fd5b61033d86828501610204565b60a08301525060c08201516001600160401b0381111561035b575f80fd5b61036786828501610204565b60c083015250949350505050565b61a982806103825f395ff3fe608060405234801561000f575f80fd5b5060043610610235575f3560e01c8063857620e11161013d578063b0464fdc116100b8578063ba414fa611610088578063e20c9f711161006e578063e20c9f71146104d6578063eee8e67b146104de578063fa7626d4146104f1575f80fd5b8063ba414fa6146104be578063cc7f0c2414610493575f80fd5b8063b0464fdc1461049b578063b15a7971146104a3578063b165c9e914610480578063b5508aa9146104b6575f80fd5b8063916a17c61161010d578063ab3c7e52116100f3578063ab3c7e5214610478578063aceb0e8514610480578063aeb8fbf914610493575f80fd5b8063916a17c61461044457806393db45e014610459575f80fd5b8063857620e1146103ec57806388bdca61146103ff5780638985c90b146104125780638f5d23ce14610425575f80fd5b80635ee481b6116101cd57806376e1fcc41161019d5780638068b52e116101835780638068b52e1461039957806382716e43146103b857806385226c81146103d7575f80fd5b806376e1fcc4146103665780637b2abdb614610379575f80fd5b80635ee481b6146102fd57806364239cdd1461031c57806366d9a9a0146103315780637477f51714610346575f80fd5b80632ade3880116102085780632ade3880146102d05780633e5e3c23146102e55780633f7286f4146102ed578063478ddecc146102f5575f80fd5b8063068bcd8d146102395780630d5ec4c61461027a5780631ed7831c1461029b5780632895a2b3146102b0575b5f80fd5b61024c6102473660046180cc565b6104fe565b604080516001600160a01b03948516815293909216602084015260020b908201526060015b60405180910390f35b61028d6102883660046180e3565b610595565b604051908152602001610271565b6102a36105a9565b6040516102719190618103565b6102c36102be3660046180cc565b610609565b604051610271919061814e565b6102d861062a565b604051610271919061820d565b6102a3610766565b6102a36107c4565b6102a3610822565b61028d61030b3660046182c2565b602a6020525f908152604090205481565b61032f61032a3660046182eb565b610833565b005b6103396112fc565b6040516102719190618395565b6103596103543660046180e3565b611475565b604051610271919061847d565b61028d6103743660046180e3565b6115bc565b61038c6103873660046180cc565b6115c7565b60405161027191906184f6565b61028d6103a73660046182c2565b602c6020525f908152604090205481565b61028d6103c63660046182c2565b602d6020525f908152604090205481565b6103df611656565b604051610271919061854e565b61032f6103fa366004618560565b611721565b61028d61040d3660046180cc565b611f9d565b61028d6104203660046180e3565b611ff7565b61028d6104333660046182c2565b602b6020525f908152604090205481565b61044c612002565b6040516102719190618589565b61028d6104673660046182c2565b60296020525f908152604090205481565b602e5461028d565b61028d61048e3660046180e3565b6120f8565b6102a3612103565b61044c61210f565b61032f6104b1366004618630565b612205565b6103df6127cc565b6104c6612897565b6040519015158152602001610271565b6102a3612967565b61032f6104ec3660046186f6565b6129c5565b601f546104c69060ff1681565b5f805f80602e85815481106105155761051561877a565b905f5260205f20906003020190506020600501815f01548154811061053c5761053c61877a565b5f918252602090912001546001820154602580546001600160a01b03909316965091811061056c5761056c61877a565b5f9182526020909120015460029182015494966001600160a01b03909116955093900b92915050565b5f6105a082846187d4565b90505b92915050565b606060168054806020026020016040519081016040528092919081815260200182805480156105ff57602002820191905f5260205f20905b81546001600160a01b031681526001909101906020018083116105e1575b5050505050905090565b5f8181526033602052604081206060919061062390613410565b9392505050565b6060601e805480602002602001604051908101604052809291908181526020015f905b8282101561075d575f84815260208082206040805180820182526002870290920180546001600160a01b03168352600181018054835181870281018701909452808452939591948681019491929084015b82821015610746578382905f5260205f200180546106bb906187e7565b80601f01602080910402602001604051908101604052809291908181526020018280546106e7906187e7565b80156107325780601f1061070957610100808354040283529160200191610732565b820191905f5260205f20905b81548152906001019060200180831161071557829003601f168201915b50505050508152602001906001019061069e565b50505050815250508152602001906001019061064d565b50505050905090565b606060188054806020026020016040519081016040528092919081815260200182805480156105ff57602002820191905f5260205f209081546001600160a01b031681526001909101906020018083116105e1575050505050905090565b606060178054806020026020016040519081016040528092919081815260200182805480156105ff57602002820191905f5260205f209081546001600160a01b031681526001909101906020018083116105e1575050505050905090565b606061082e60286134c9565b905090565b835f61083f602761359d565b905061084c825f836135ee565b9150808210156108a95761086160278361363b565b603580546001600160a01b03929092167fffffffffffffffffffffffff0000000000000000000000000000000000000000928316811790915560368054909216179055610a4d565b6023546040516001600160a01b03909116906108c49061808d565b6001600160a01b039091168152602001604051809103905ff0801580156108ed573d5f803e3d5ffd5b50603680547fffffffffffffffffffffffff00000000000000000000000000000000000000009081166001600160a01b039390931692831790915560358054909116821790556040517f6900a3ae000000000000000000000000000000000000000000000000000000008152600184016004820152737109709ecfa91a80626ff3989d68f67f5b1dd12d9163c657c718918390636900a3ae906024015f60405180830381865afa1580156109a3573d5f803e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526109ca9190810190618838565b6040516020016109da91906188e3565b6040516020818303038152906040526040518363ffffffff1660e01b8152600401610a06929190618914565b5f604051808303815f87803b158015610a1d575f80fd5b505af1158015610a2f573d5f803e3d5ffd5b5050603554610a4b9250602791506001600160a01b03166136ad565b505b50610a6a865f6001602e80549050610a659190618935565b6135ee565b95505f602e8781548110610a8057610a8061877a565b5f918252602082206002600390920201818101549093508291610aa491900b613898565b91509150610ae5836002015f9054906101000a900460020b610ad08960020b8560020b8560020b6138f0565b60020b905f8183071291819005919091030290565b9650610b0f836002015f9054906101000a900460020b610ad08860020b8560020b8560020b6138f0565b6040517f4c63e562000000000000000000000000000000000000000000000000000000008152600282810b908a900b14156004820152909650737109709ecfa91a80626ff3989d68f67f5b1dd12d90634c63e562906024015f6040518083038186803b158015610b7d575f80fd5b505afa158015610b8f573d5f803e3d5ffd5b505050508660020b8660020b1215610ba5579495945b50506024548154602580545f93610c26936001600160a01b0390911692918110610bd157610bd161877a565b5f918252602090912001546001850154602580546001600160a01b03909316929091908110610c0257610c0261877a565b5f918252602090912001546002808701546001600160a01b0390921691900b6139c7565b90505f610c348260a0902090565b90505f805f610c45848b8b89613a24565b6040517f4c63e5620000000000000000000000000000000000000000000000000000000081526fffffffffffffffffffffffffffffffff8416151560048201529295509093509150737109709ecfa91a80626ff3989d68f67f5b1dd12d90634c63e562906024015f6040518083038186803b158015610cc2575f80fd5b505afa158015610cd4573d5f803e3d5ffd5b50505050610cf6886001856fffffffffffffffffffffffffffffffff166135ee565b600280880154602354929a50610d1a926001600160a01b03169187918e910b613d4a565b610d35575f848152603060205260409020610d35908b613da0565b600280870154602354610d59926001600160a01b039091169187918d91900b613d4a565b610d74575f848152603060205260409020610d74908a613da0565b8554602680545f92908110610d8b57610d8b61877a565b5f9182526020822001546001890154602680546001600160a01b039093169450918110610dba57610dba61877a565b5f918252602090912001546035546040517fa9059cbb0000000000000000000000000000000000000000000000000000000081526001600160a01b03918216600482015260248101879052918116925083169063a9059cbb906044016020604051808303815f875af1158015610e32573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e569190618948565b506035546040517fa9059cbb0000000000000000000000000000000000000000000000000000000081526001600160a01b039182166004820152602481018590529082169063a9059cbb906044016020604051808303815f875af1158015610ec0573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ee49190618948565b506036546002898101546040805160a0810182525f91810182905260808101919091526001600160a01b03868116825285811660208301529190920b606083015290911690630c865879908e8e8e5f801b6040518663ffffffff1660e01b8152600401610f55959493929190618967565b60408051808303815f875af1158015610f70573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f9491906189ed565b50508754602580545f94509092508110610fb057610fb061877a565b5f9182526020822001546001890154602580546001600160a01b039093169450918110610fdf57610fdf61877a565b5f918252602090912001546035546040517fa9059cbb0000000000000000000000000000000000000000000000000000000081526001600160a01b03918216600482015260248101879052918116925083169063a9059cbb906044016020604051808303815f875af1158015611057573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061107b9190618948565b506035546040517fa9059cbb0000000000000000000000000000000000000000000000000000000081526001600160a01b039182166004820152602481018590529082169063a9059cbb906044016020604051808303815f875af11580156110e5573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111099190618948565b5060365f9054906101000a90046001600160a01b03166001600160a01b0316630c865879888e8e8e5f801b6040518663ffffffff1660e01b8152600401611154959493929190618967565b60408051808303815f875af115801561116f573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061119391906189ed565b50506036545f97506111b996508c95508b94506001600160a01b03169250613ebd915050565b90505f8881526034602052604090206111d29082613f0c565b505f88815260326020908152604080832084845282528083208b845260339092529091206112009083613f0c565b1561127f57805462ffffff8781166301000000027fffffffffffffffffffffffffffffffffffffffffffffffffffff000000000000909216908916171780825560365466010000000000006001600160a01b03909116027fffffffffffff0000000000000000000000000000000000000000ffffffffffff9091161781555b84816003015f82825461129291906187d4565b9091555050604080516060810182529586525f998a5260316020908152818b20548188019081525f199288019283526005909301805460018181018355918d5291909b20965160039091029096019586559051988501989098555050945160029091015550505050565b6060601b805480602002602001604051908101604052809291908181526020015f905b8282101561075d578382905f5260205f2090600202016040518060400160405290815f8201805461134f906187e7565b80601f016020809104026020016040519081016040528092919081815260200182805461137b906187e7565b80156113c65780601f1061139d576101008083540402835291602001916113c6565b820191905f5260205f20905b8154815290600101906020018083116113a957829003601f168201915b505050505081526020016001820180548060200260200160405190810160405280929190818152602001828054801561145d57602002820191905f5260205f20905f905b82829054906101000a900460e01b7bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19168152602001906004019060208260030104928301926001038202915080841161140a5790505b5050505050815250508152602001906001019061131f565b6114c56040518061010001604052805f60020b81526020015f60020b81526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f8152602001606081525090565b5f83815260326020908152604080832085845282528083208151610100810183528154600281810b835263010000008204810b8387015266010000000000009091046001600160a01b031682850152600183015460608301528201546080820152600382015460a0820152600482015460c08201526005820180548451818702810187019095528085529195929460e0870194939192919084015b828210156115ad578382905f5260205f2090600302016040518060600160405290815f82015481526020016001820154815260200160028201548152505081526020019060010190611560565b50505091525090949350505050565b5f6105a08284618a0f565b606060315f8381526020019081526020015f20805480602002602001604051908101604052809291908181526020015f905b8282101561164b575f8481526020908190206040805180820190915290840154600281900b8252630100000090046fffffffffffffffffffffffffffffffff16818301528252600190920191016115f9565b505050509050919050565b6060601a805480602002602001604051908101604052809291908181526020015f905b8282101561075d578382905f5260205f20018054611696906187e7565b80601f01602080910402602001604051908101604052809291908181526020018280546116c2906187e7565b801561170d5780601f106116e45761010080835404028352916020019161170d565b820191905f5260205f20905b8154815290600101906020018083116116f057829003601f168201915b505050505081526020019060010190611679565b611738835f6001602e80549050610a659190618935565b5f8181526034602052604081209194509061175290614022565b6040517f4c63e5620000000000000000000000000000000000000000000000000000000081528115156004820152909150737109709ecfa91a80626ff3989d68f67f5b1dd12d90634c63e562906024015f6040518083038186803b1580156117b8575f80fd5b505afa1580156117ca573d5f803e3d5ffd5b505050505f6117f76117e4855f600186610a659190618935565b5f87815260346020526040902090614070565b5f86815260326020908152604080832084845290915281206003810154929350916118239186916135ee565b93505f602e87815481106118395761183961877a565b5f91825260208220602454600390920201805460258054929550611872936001600160a01b0316929091908110610bd157610bd161877a565b6024549091505f906001600160a01b031663d89411446118938460a0902090565b865460405160e084901b7fffffffff000000000000000000000000000000000000000000000000000000001681526004810192909252660100000000000081046001600160a01b03166024830152600281810b60448401526301000000909104900b60648201525f608482015260a401602060405180830381865afa15801561191e573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119429190618a26565b905080602b5f6020600501865f0154815481106119615761196161877a565b5f9182526020808320909101546001600160a01b03168352820192909252604001812080549091906119949084906187d4565b9250508190555080846002015f8282546119ae91906187d4565b909155505083546001600160a01b03660100000000000082041690630c865879908490600281810b9163010000009004900b6119e98c618a3d565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e087901b168152611a2794939291905f90600401618967565b60408051808303815f875af1158015611a42573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611a6691906189ed565b5050835483546025805466010000000000009093046001600160a01b03169263baca000492908110611a9a57611a9a61877a565b5f9182526020909120015460405160e083901b7fffffffff000000000000000000000000000000000000000000000000000000001681526001600160a01b0390911660048201526024015f604051808303815f87803b158015611afb575f80fd5b505af1158015611b0d573d5f803e3d5ffd5b5050855460018601546025805466010000000000009093046001600160a01b0316945063baca00049350918110611b4657611b4661877a565b5f9182526020909120015460405160e083901b7fffffffff000000000000000000000000000000000000000000000000000000001681526001600160a01b0390911660048201526024015f604051808303815f87803b158015611ba7575f80fd5b505af1158015611bb9573d5f803e3d5ffd5b50508454602680545f94509092508110611bd557611bd561877a565b5f9182526020822001546001860154602680546001600160a01b039093169450918110611c0457611c0461877a565b5f91825260208083209091015488546002898101546040805160a08101825290810187905260808101969096526001600160a01b038881168752938416948601859052900b606085015291935066010000000000009091041690630c865879908854600281810b9163010000009004900b611c7e8e618a3d565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e087901b168152611cbc94939291905f90600401618967565b60408051808303815f875af1158015611cd7573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611cfb91906189ed565b505085546040517fbaca00040000000000000000000000000000000000000000000000000000000081526001600160a01b03848116600483015266010000000000009092049091169063baca0004906024015f604051808303815f87803b158015611d64575f80fd5b505af1158015611d76573d5f803e3d5ffd5b505087546040517fbaca00040000000000000000000000000000000000000000000000000000000081526001600160a01b0385811660048301526601000000000000909204909116925063baca000491506024015f604051808303815f87803b158015611de1575f80fd5b505af1158015611df3573d5f803e3d5ffd5b5050505050505f611e058360a0902090565b8554600286810154602354939450611e2e936001600160a01b031692859290810b91900b613d4a565b611e4e575f8181526030602052604090208554611e4e919060020b6140d2565b8454600285810154602354611e7d936001600160a01b039091169285926301000000909204810b91900b613d4a565b611ea4575f8181526030602052604090208554611ea491906301000000900460020b6140d2565b5050505083816003015f828254611ebb9190618935565b90915550505f8681526031602052604090205460048201546005830191905b8254811015611f115781838281548110611ef657611ef661877a565b5f918252602090912060026003909202010155600101611eda565b5081546004840155600383015415611f7a5760408051606081018252600380860154825260208083018581525f19948401948552600588018054600181810183555f92835293909120945193029093019182559151918101919091559051600290910155611f93565b5f888152603460205260409020611f919085614204565b505b5050505050505050565b5f80602e8381548110611fb257611fb261877a565b5f918252602090912060245460039092020180546025805492945061062393611ff0936001600160a01b0390911692908110610bd157610bd161877a565b60a0902090565b5f6105a08284618935565b6060601d805480602002602001604051908101604052809291908181526020015f905b8282101561075d575f8481526020908190206040805180820182526002860290920180546001600160a01b031683526001810180548351818702810187019094528084529394919385830193928301828280156120e057602002820191905f5260205f20905f905b82829054906101000a900460e01b7bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19168152602001906004019060208260030104928301926001038202915080841161208d5790505b50505050508152505081526020019060010190612025565b5f6105a08284618aa0565b606061082e60276134c9565b6060601c805480602002602001604051908101604052809291908181526020015f905b8282101561075d575f8481526020908190206040805180820182526002860290920180546001600160a01b031683526001810180548351818702810187019094528084529394919385830193928301828280156121ed57602002820191905f5260205f20905f905b82829054906101000a900460e01b7bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19168152602001906004019060208260030104928301926001038202915080841161219a5790505b50505050508152505081526020019060010190612132565b60255461221c9087905f90610a6590600190618935565b6025549096506122369086905f90610a6590600190618935565b94506122418361430a565b925061224c8261430a565b915061227b6001600160a01b0382166401000276a373fffd8963efd1fc6a506488495d951d5263988d266135ee565b905084860361229c57602554600187018161229857612298618a73565b0694505b848611156122a8579394935b6122ba600285900b6001617fff6138f0565b5f878152602f602090815260408083208984529091529020549094506122e29060ff16614318565b5f868152602f60209081526040808320888452825280832080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0016600117905560245481517f9d69dc4e00000000000000000000000000000000000000000000000000000000815291516123b4936001600160a01b0390921692639d69dc4e9260048281019391928290030181865afa158015612382573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906123a69190618ab3565b6001600160a01b0316614394565b90505f602060050188815481106123cd576123cd61877a565b5f918252602082200154602680546001600160a01b039092169350908a9081106123f9576123f961877a565b5f918252602082200154602580546001600160a01b039092169350908a9081106124255761242561877a565b5f918252602082200154602680546001600160a01b039092169350908b9081106124515761245161877a565b5f918252602090912001546021546040517fca669fa70000000000000000000000000000000000000000000000000000000081526001600160a01b03918216600482015291169150737109709ecfa91a80626ff3989d68f67f5b1dd12d9063ca669fa7906024015f604051808303815f87803b1580156124cf575f80fd5b505af11580156124e1573d5f803e3d5ffd5b5050602480546040517f138714650000000000000000000000000000000000000000000000000000000081526001600160a01b0389811660048301528781169382019390935261ffff8e16604482015262ffffff808e1660648301528c1660848201525f60a4820152911692506313871465915060c4015f604051808303815f87803b15801561256f575f80fd5b505af1158015612581573d5f803e3d5ffd5b505050506125998460286136ad90919063ffffffff16565b506125a56028836136ad565b50602480546040517f8587f4500000000000000000000000000000000000000000000000000000000081526001600160a01b03878116600483015285811693820193909352604481018890528883166064820152911690638587f450906084015f604051808303815f87803b15801561261c575f80fd5b505af115801561262e573d5f803e3d5ffd5b50506023546040805160a0810182525f81830181905260808201526001600160a01b038881168252868116602083015260028f900b606083015291517f6276cbbe000000000000000000000000000000000000000000000000000000008152919092169350636276cbbe92506126a991908a90600401618ace565b6020604051808303815f875af11580156126c5573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906126e99190618b40565b5050604080516060810182529a8b5260208b01998a5260029890980b978a019788525050602e80546001810182555f9190915297517f37fa166cbdbfbb1561ccd9ea985ec0218b5e68502e230525f544285b2bdf3d7e600390990298890155505093517f37fa166cbdbfbb1561ccd9ea985ec0218b5e68502e230525f544285b2bdf3d7f8601555050517f37fa166cbdbfbb1561ccd9ea985ec0218b5e68502e230525f544285b2bdf3d8090920180547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000001662ffffff9093169290921790915550565b60606019805480602002602001604051908101604052809291908181526020015f905b8282101561075d578382905f5260205f2001805461280c906187e7565b80601f0160208091040260200160405190810160405280929190818152602001828054612838906187e7565b80156128835780601f1061285a57610100808354040283529160200191612883565b820191905f5260205f20905b81548152906001019060200180831161286657829003601f168201915b5050505050815260200190600101906127ef565b6008545f9060ff16156128ae575060085460ff1690565b6040517f667f9d70000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d600482018190527f6661696c6564000000000000000000000000000000000000000000000000000060248301525f9163667f9d7090604401602060405180830381865afa15801561293c573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906129609190618a26565b1415905090565b606060158054806020026020016040519081016040528092919081815260200182805480156105ff57602002820191905f5260205f209081546001600160a01b031681526001909101906020018083116105e1575050505050905090565b6129dc835f6001602e80549050610a659190618935565b9250826038819055505f602e84815481106129f9576129f961877a565b5f91825260208220602454600390920201805460258054929550612a8b93611ff0936001600160a01b0390911692908110612a3657612a3661877a565b5f918252602090912001546001860154602580546001600160a01b03909316929091908110612a6757612a6761877a565b5f918252602090912001546002808801546001600160a01b0390921691900b6139c7565b6002808401549192505f91612aa3918491900b6143aa565b8051909150612ab3865f836135ee565b9550612abd61809a565b612ad482612acc600482618aa0565b839190614572565b5f8767ffffffffffffffff811115612aee57612aee618698565b604051908082528060200260200182016040528015612b3257816020015b604080518082019091525f8082526020820152815260200190600190039081612b0c5790505b5090505f805b89811015612ebd575f86612b4c8b8761458b565b81518110612b5c57612b5c61877a565b602002602001015190505f612bb867016345785d8a0000612b8e670de0b6b3a76400008e6145a590919063ffffffff16565b1115612bb257612bad8c60016cffffffffffffffffffffffffff61460a565b614707565b5f614707565b905060405180604001604052808360020b8152602001826fffffffffffffffffffffffffffffffff16815250858481518110612bf657612bf661877a565b6020908102919091010152612c1d6fffffffffffffffffffffffffffffffff8216856187d4565b93505f8d81526031602052604090208551869085908110612c4057612c4061877a565b60209081029190910181015182546001810184555f938452828420825191018054928401516fffffffffffffffffffffffffffffffff166301000000027fffffffffffffffffffffffffff0000000000000000000000000000000000000090931662ffffff90921691909117919091179055603782905560385482526034905260408120612ccd90614022565b90505f805b82811015612d8f576038545f908152603460205260408120612cf49083614070565b6038545f9081526032602090815260408083208484529091529020805491925090600288810b91900b13801590612d3a5750805463010000009004600290810b9088900b125b15612d85576003810154612d4e90856187d4565b603780546001810182555f919091527f42a7b7dd785cd69714a189dffb3fd7d7174edc9ece837694ce50f7078f7c31ae0183905593505b5050600101612cd2565b50805f03612dff57826fffffffffffffffffffffffffffffffff16602c5f60206005018f5f015481548110612dc657612dc661877a565b5f9182526020808320909101546001600160a01b0316835282019290925260400181208054909190612df99084906187d4565b90915550505b60375491505f5b82811015612eac575f60378281548110612e2257612e2261877a565b5f91825260208083209091015460385483526032825260408084208285529092529120600381015491925090612e879085612e806fffffffffffffffffffffffffffffffff8a16700100000000000000000000000000000000618a0f565b919061475e565b816001015f828254612e9991906187d4565b909155505060019092019150612e069050565b505060019093019250612b38915050565b506023546002808901545f92612ee39286926001600160a01b03909216918b910b6147ec565b90505f6020600501895f015481548110612eff57612eff61877a565b5f91825260208220015460018b0154602580546001600160a01b039093169450918110612f2e57612f2e61877a565b5f9182526020808320909101546001600160a01b038581168452602a90925260408320805492909116935086929091612f689084906187d4565b90915550508954602580545f92908110612f8457612f8461877a565b5f918252602090912001546040517f40c10f19000000000000000000000000000000000000000000000000000000008152306004820152602481018790526001600160a01b03909116915081906340c10f19906044015f604051808303815f87803b158015612ff1575f80fd5b505af1158015613003573d5f803e3d5ffd5b5050602480546040517f095ea7b30000000000000000000000000000000000000000000000000000000081526001600160a01b0391821660048201525f19928101929092528416925063095ea7b391506044015f604051808303815f87803b15801561306d575f80fd5b505af115801561307f573d5f803e3d5ffd5b5050602480546040517f47e7ef240000000000000000000000000000000000000000000000000000000081526001600160a01b0388811660048301529281018a9052911692506347e7ef2491506044015f604051808303815f87803b1580156130e6575f80fd5b505af11580156130f8573d5f803e3d5ffd5b5050855191505080156133f757604080516002808252606082019092525f91816020015b604080516080810182525f8082526020808301829052928201819052606082015282525f1990920191018161311c57905050905084815f815181106131635761316361877a565b60200260200101515f01906001600160a01b031690816001600160a01b031681525050838160018151811061319a5761319a61877a565b60209081029190910101516001600160a01b039190911690526040805160018082528183019092525f91816020015b604080516060810182525f80825260208083018290529282015282525f199092019101816131c957905050905085815f815181106132095761320961877a565b60200260200101515f01906001600160a01b031690816001600160a01b03168152505084815f8151811061323f5761323f61877a565b6020908102919091018101516001600160a01b03909216910152613267565b60a01c60020b90565b5f5b838110156133f3575f6040518060800160405280896001600160a01b03168152602001886001600160a01b031681526020015f6fffffffffffffffffffffffffffffffff1681526020018a84815181106132c5576132c561877a565b602090810291909101015190526024549091506001600160a01b03166397dc99656132ef86614901565b6133728760206004015f9054906101000a90046001600160a01b03166001600160a01b0316639d69dc4e6040518163ffffffff1660e01b8152600401602060405180830381865afa158015613346573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061336a9190618ab3565b889190614992565b61337c8588614a31565b60405160200161338e93929190618b5b565b6040516020818303038152906040526040518263ffffffff1660e01b81526004016133b99190618b78565b5f604051808303815f87803b1580156133d0575f80fd5b505af11580156133e2573d5f803e3d5ffd5b505060019093019250613269915050565b5050505b6133ff614ac6565b505050505050505050505050505050565b6318fb58646004525f81815260249020801954604051919068fbb67fda52d4bfb8bf90602084018161348957835480156134835780841415028152600184810154909250801561348357808414150260208201526002848101549092508015613483576003925083811415810260408301525b506134b4565b8160011c91505f5b828110156134b257848101548481141502600582901b830152600101613491565b505b8185528160051b810160405250505050919050565b63978aab926004525f818152602481206060915068fbb67fda52d4bfb8bf81548060a01b60a01c6040519450846020018260601c925083831415830281528161355757821561355257600191508185015460601c92508215613552578284141590920260208301525060028381015460601c918215613552576003915083831415830260408201525b613587565b600191821c915b82811015613585578581015460601c858114158102600583901b840152935060010161355e565b505b8186528160051b81016040525050505050919050565b63978aab926004525f8181526024812080548060a01b60a01c8060011c9350808260601c15176135e6576001935083830154156135e6576002935083830154156135e657600393505b505050919050565b5f6135fa848484614ba4565b90506106236040518060400160405280600c81526020017f426f756e6420726573756c74000000000000000000000000000000000000000081525082614d5d565b63978aab926004525f828152602481208281015460601c915068fbb67fda52d4bfb8bf8214158202915061366e8461359d565b83106136a6576040517f4e23d03500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5092915050565b63978aab926004525f828152602481206001600160a01b0392909216917fffffffffffffffffffffffffffffffffffffffffffffff04498025ad2b40474183016136fe5763f5a267f15f526004601cfd5b826137105768fbb67fda52d4bfb8bf92505b80546bffffffffffffffffffffffff811682602052806137d7578160601c80613743578560601b8455600194505061381b565b858103613750575061381b565b600184015460601c80613771578660601b600186015560019550505061381b565b86810361377f57505061381b565b600285015460601c806137a1578760601b60028701556001965050505061381b565b8781036137b05750505061381b565b5f928352604080842060019055918352818320600290558252902060039055506007908117905b845f5260405f20805461381957600191821c808301825591945081613805578560601b60031784555061381b565b8560601b828501558260020184555061381b565b505b50505092915050565b613895816040516024016138389190618b78565b60408051601f198184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f41304fac00000000000000000000000000000000000000000000000000000000179052614dd0565b50565b5f80826138c5817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff27618618b8a565b6138cf9190618bde565b9150826138df81620d89e8618b8a565b6138e99190618bde565b9050915091565b5f6138fc848484614de0565b604080518082018252600c81527f426f756e6420726573756c740000000000000000000000000000000000000000602082015290517fa322c40e0000000000000000000000000000000000000000000000000000000081526004810183905291925061062391737109709ecfa91a80626ff3989d68f67f5b1dd12d9063a322c40e906024015f60405180830381865afa15801561399b573d5f803e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526139c29190810190618838565b615041565b6040805160a0810182525f918101919091526001600160a01b038581166080830181905285821683529084166020830152600283900b606083015215613a105762800000613a12565b5f5b62ffffff166040820152949350505050565b6002818101546023545f9283928392910b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff2761881810784139082900503620d89e891909105036001016fffffffffffffffffffffffffffffffff04908290613a95906001600160a01b03168a8a6150b4565b506023549091505f90613ab2906001600160a01b03168b8a6150b4565b509050613ae3826fffffffffffffffffffffffffffffffff16826fffffffffffffffffffffffffffffffff16615119565b613aed9084618bfd565b6023549096505f9350613b0c92506001600160a01b0316905089614819565b90505f80613b1d8360a01c60020b90565b9150506001600160a01b038216613b3782828b8b8a61512e565b8854602580549398509196505f928110613b5357613b5361877a565b5f918252602090912001546040517f70a082310000000000000000000000000000000000000000000000000000000081523060048201526001600160a01b03909116906370a0823190602401602060405180830381865afa158015613bba573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613bde9190618a26565b90505f6020600501896001015481548110613bfb57613bfb61877a565b5f918252602090912001546040517f70a082310000000000000000000000000000000000000000000000000000000081523060048201526001600160a01b03909116906370a0823190602401602060405180830381865afa158015613c62573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613c869190618a26565b9050613d26612bad613d0e898510613cb0578a6fffffffffffffffffffffffffffffffff16613ccd565b613ccd6fffffffffffffffffffffffffffffffff8c16868c61475e565b898510613cec578b6fffffffffffffffffffffffffffffffff166151c5565b613d096fffffffffffffffffffffffffffffffff8d16868c61475e565b6151c5565b8a6fffffffffffffffffffffffffffffffff166151c5565b9750613d3584848d8d8c61512e565b989d909c50979a509698505050505050505050565b5f8080613d688486078213858705035b600881901d9160ff90911690565b9092509050613d9581613d856001600160a01b038a1689866151d3565b90600160ff919091161b16151590565b979650505050505050565b81545f5b81811015613df357838181548110613dbe57613dbe61877a565b905f5260205f2090600a91828204019190066003029054906101000a900460020b60020b8360020b12613df357600101613da4565b81811015613e805782848281548110613e0e57613e0e61877a565b905f5260205f2090600a91828204019190066003029054906101000a900460020b858381548110613e4157613e4161877a565b5f9182526020909120600a8083049091018054919092066003026101000a62ffffff818102199092169490911602929092179091559250600101613df3565b505081546001810183555f9283526020909220600a8084049091018054919093066003026101000a62ffffff818102199092169290911602179055565b60408051600285810b602083015284900b918101919091526001600160a01b03821660608201525f906080016040516020818303038152906040528051906020012090509392505050565b5050565b6318fb58646004525f8281526024812068fbb67fda52d4bfb8bf8303613f395763f5a267f15f526004601cfd5b82613f4b5768fbb67fda52d4bfb8bf92505b8019548160205280613fef57815480613f6b57848355600193505061401a565b848103613f78575061401a565b600183015480613f935785600185015560019450505061401a565b858103613fa157505061401a565b600284015480613fbd578660028601556001955050505061401a565b868103613fcc5750505061401a565b5f9283526040808420600190559183528183206002905582529020600390555060075b835f5260405f20805461381b57600191821c8381018690558083019182905590821b82178319559092505b505092915050565b6318fb58646004525f818152602481208019548060011c9250806140695781545f935015614069576001925082820154156140695760029250828201541561406957600392505b5050919050565b6318fb58646004525f8281526024902081015468fbb67fda52d4bfb8bf8114150261409a83614022565b82106105a3576040517f4e23d03500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81545f5b81811015614121578260020b8482815481106140f4576140f461877a565b5f9182526020909120600a8083049091015491066003026101000a900460020b14614121576001016140d6565b61412c600183618935565b8110156141c1578361413f8260016187d4565b8154811061414f5761414f61877a565b905f5260205f2090600a91828204019190066003029054906101000a900460020b8482815481106141825761418261877a565b905f5260205f2090600a91828204019190066003026101000a81548162ffffff021916908360020b62ffffff1602179055508080600101915050614121565b838054806141d1576141d1618c6a565b600190038181905f5260205f2090600a91828204019190066003026101000a81549062ffffff0219169055905550505050565b6318fb58646004525f8281526024812068fbb67fda52d4bfb8bf83036142315763f5a267f15f526004601cfd5b826142435768fbb67fda52d4bfb8bf92505b801954806142ac5760019250838254036142705760018201805483556002830180549091555f905561401a565b8360018301540361428e5760028201805460018401555f905561401a565b836002830154036142a4575f600283015561401a565b5f925061401a565b81602052835f5260405f208054806142c557505061401a565b60018360011c0392508260018203146142ef57828401548060018303860155805f52508060405f20555b5060018260011b178319555f81555060019250505092915050565b5f6105a382620f4240615201565b6040517fa59828850000000000000000000000000000000000000000000000000000000081528115156004820152737109709ecfa91a80626ff3989d68f67f5b1dd12d9063a5982885906024015f6040518083038186803b15801561437b575f80fd5b505afa15801561438d573d5f803e3d5ffd5b5050505050565b5f6105a360206001600160a01b0384163b618aa0565b6023546060905f906143cf908490610ad09061325e906001600160a01b031688614819565b90505f836143e06014610100618bde565b6143ea9190618bde565b90505f6143f78284618c97565b90505f6144048385618cd8565b5f888152603060205260408120805492935090915b80831015614468578460020b8284815481106144375761443761877a565b5f9182526020909120600a8083049091015491066003026101000a900460020b121561446857600190920191614419565b826144738183618935565b67ffffffffffffffff81111561448b5761448b618698565b6040519080825280602002602001820160405280156144b4578160200160208202803683370190505b5098505b81841015614560578460020b8385815481106144d6576144d661877a565b5f9182526020909120600a8083049091015491066003026101000a900460020b136145605782848154811061450d5761450d61877a565b905f5260205f2090600a91828204019190066003029054906101000a900460020b89828603815181106145425761454261877a565b60029290920b602092830291909101909101526001909301926144b8565b90920387525094979650505050505050565b81835261457e81615217565b8360200181905250505050565b80515f906105a09061459e9085906145a5565b8390615278565b5f816001036145b557505f6105a3565b5f826145c2815f19618aa0565b6145cc9190618a0f565b845160205290505f805b815f5260405f2090508281106145f1576001820191506145d6565b848106935050506003601f53506021601f209092525090565b5f825f0361465f5760405162461bcd60e51b815260206004820152600560248201527f4c6f77203000000000000000000000000000000000000000000000000000000060448201526064015b60405180910390fd5b8183106146ae5760405162461bcd60e51b815260206004820152601260248201527f4c6f77206e6f742062656c6f77206869676800000000000000000000000000006044820152606401614656565b5f6146b9838561536f565b90505f6146cd6146c8836153b1565b61540c565b90505f6146db878284615665565b90505f6146ef6146ea836156b7565b61589c565b90506146fb87826158d7565b98975050505050505050565b5f61475a826fffffffffffffffffffffffffffffffff80166040518060400160405280601681526020017f556e73616665206361737420746f2075696e7431323800000000000000000000815250615906565b5090565b828202818385830414851517026147e5575f198385098181108201900382848609835f0384168285116147985763ae47f7025f526004601cfd5b93849004938382119092035f839003839004600101029203041760026003830281188084028203028084028203028084028203028084028203028084028203028084029091030202610623565b0492915050565b60606148108585858561480b61325e6001600160a01b03851684614819565b61598b565b95945050505050565b5f8181526006602052604081206148396001600160a01b03851682615f3b565b949350505050565b60200260200101516fffffffffffffffffffffffffffffffff16615f6b565b8351816fffffffffffffffffffffffffffffffff1610156148d857816148a385836fffffffffffffffffffffffffffffffff16815181106148415761484161877a565b6040516020016148b4929190618d19565b604051602081830303815290604052915080806148d090618d54565b915050614860565b50806040516020016148ea9190618d90565b604051602081830303815290604052915050919050565b60605f5b825181101561495c57816149318483815181106149245761492461877a565b6020026020010151615f76565b604051602001614942929190618dc8565b60408051601f198184030181529190529150600101614905565b506149678151616063565b60e81b8160405160200161497c929190618dd6565b6040516020818303038152906040529050919050565b60605f5b84518110156149f957816149ce85858885815181106149b7576149b761877a565b60200260200101516160779092919063ffffffff16565b6040516020016149df929190618dc8565b60408051601f198184030181529190529150600101614996565b50614a048151616063565b60e81b81604051602001614a19929190618dd6565b60405160208183030381529060405290509392505050565b60605f80614a51855f01518660200151866161369092919063ffffffff16565b915091505f85606001515f0151614a68575f614a6b565b60025b82614a76575f614a79565b60015b1790508060f81b8360f01b876040015160801b614a9989606001516162b8565b604051602001614aac9493929190618e09565b604051602081830303815290604052935050505092915050565b5f614ad1602861359d565b90505f5b81811015613f08575f614ae960288361363b565b602480546040517fd57197780000000000000000000000000000000000000000000000000000000081526001600160a01b03808516600483015293945092169163d57197789101602060405180830381865afa158015614b4b573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614b6f9190618a26565b6001600160a01b0382165f908152602d602052604081208054909190614b96908490618e8c565b909155505050600101614ad5565b5f81831115614c1b5760405162461bcd60e51b815260206004820152603e60248201527f5374645574696c7320626f756e642875696e743235362c75696e743235362c7560448201527f696e74323536293a204d6178206973206c657373207468616e206d696e2e00006064820152608401614656565b828410158015614c2b5750818411155b15614c37575082610623565b5f614c428484618935565b614c4d9060016187d4565b905060038511158015614c5f57508481115b15614c7657614c6e85856187d4565b915050610623565b614c8260035f19618935565b8510158015614c9a5750614c97855f19618935565b81115b15614cb457614caa855f19618935565b614c6e9084618935565b82851115614d07575f614cc78487618935565b90505f614cd48383618eab565b9050805f03614ce857849350505050610623565b6001614cf482886187d4565b614cfe9190618935565b93505050614d55565b83851015614d55575f614d1a8686618935565b90505f614d278383618eab565b9050805f03614d3b57859350505050610623565b614d458186618935565b614d509060016187d4565b935050505b509392505050565b613f088282604051602401614d73929190618c49565b60408051601f198184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fb60e72cc000000000000000000000000000000000000000000000000000000001790526163cc565b613895816163d85b63ffffffff16565b5f81831315614e575760405162461bcd60e51b815260206004820152603b60248201527f5374645574696c7320626f756e6428696e743235362c696e743235362c696e7460448201527f323536293a204d6178206973206c657373207468616e206d696e2e00000000006064820152608401614656565b5f808512614e8e57614e897f8000000000000000000000000000000000000000000000000000000000000000866187d4565b614ec5565b6001614ebb86197f8000000000000000000000000000000000000000000000000000000000000000618935565b614ec59190618935565b90505f808512614efe57614ef97f8000000000000000000000000000000000000000000000000000000000000000866187d4565b614f35565b6001614f2b86197f8000000000000000000000000000000000000000000000000000000000000000618935565b614f359190618935565b90505f808512614f6e57614f697f8000000000000000000000000000000000000000000000000000000000000000866187d4565b614fa5565b6001614f9b86197f8000000000000000000000000000000000000000000000000000000000000000618935565b614fa59190618935565b90505f614fb3848484614ba4565b90507f8000000000000000000000000000000000000000000000000000000000000000811061500b576150067f800000000000000000000000000000000000000000000000000000000000000082618935565b6146fb565b615035817f8000000000000000000000000000000000000000000000000000000000000000618935565b6146fb901960016187d4565b613f088282604051602401615057929190618c25565b60408051601f198184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f4b5c4277000000000000000000000000000000000000000000000000000000001790526163cc565b5f806006602052835f52600460405f2001602052825f5260405f20602052631e2eaeaf5f5260205f6024601c885afa6150f45763535cf94b5f526004601cfd5b50505f516fffffffffffffffffffffffffffffffff81169460809190911d9350915050565b5f81831161512757816105a0565b5090919050565b5f808460020b8760020b12156151625761515b61514a866163f7565b615153866163f7565b8560016166d5565b91506151bb565b8360020b8760020b121561519b5761517d86615153866163f7565b915061519461518b866163f7565b8785600161679f565b90506151bb565b6151b86151a7866163f7565b6151b0866163f7565b85600161679f565b90505b9550959350505050565b5f81831061512757816105a0565b5f82815260066020908152604080832084845260050190915281206148106001600160a01b03861682615f3b565b5f6105a08362ffffff165f8462ffffff166135ee565b604080518082019091525f8152606060208201528167ffffffffffffffff81111561524457615244618698565b60405190808252806020026020018201604052801561526d578160200160208202803683370190505b506020820152919050565b81515f906152c85760405162461bcd60e51b815260206004820152600e60248201527f4e6f7468696e6720746f207573650000000000000000000000000000000000006044820152606401614656565b5f806152d4858561683f565b875190955091935091505f906152f7906152f090600190618935565b879061683f565b925050506001865f0181815161530d9190618935565b90525082156153405760208601516168f69081906153399061532f90866168fe565b848363ffffffff16565b505061381b565b60408051808201909152858152602080820183905287015181906153649082616960565b505050505092915050565b5f7812725dd1d243aba0e75fe645cc4873f9e65afe688c928e1f228310820261539f57637c5f487d5f526004601cfd5b50670de0b6b3a7640000919091020490565b5f7f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82111561475a576040517f35278d1200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6fffffffffffffffffffffffffffffffff811160071b81811c67ffffffffffffffff1060061b1781811c63ffffffff1060051b1781811c61ffff1060041b1781811c60ff1060031b175f821361546957631615e6385f526004601cfd5b7ff8f9f9faf9fdfafbf9fdfcfdfafbfcfef9fafdfafcfcfbfefafafcfbffffffff6f8421084210842108cc6318c6db6d54be83831c1c601f161a1890811b609f90811c6c465772b2bbbb5f824b15207a3081018102606090811d6d0388eaa27412d5aca026815d636e018202811d6d0df99ac502031bf953eff472fdcc018202811d6d13cdffb29d51d99322bdff5f2211018202811d6d0a0f742023def783a307a986912e018202811d6d01920d8043ca89b5239253284e42018202811d6c0b7a86d7375468fac667a0a527016c29508e458543d8aa4df2abee7883018302821d6d0139601a2efabe717e604cbb4894018302821d6d02247f7a7b6594320649aa03aba1018302821d7fffffffffffffffffffffffffffffffffffffff73c0c716a594e00d54e3c4cbc9018302821d7ffffffffffffffffffffffffffffffffffffffdc7b88c420e53a9890533129f6f01830290911d7fffffffffffffffffffffffffffffffffffffff465fda27eb4d63ded474e5f832019091027ffffffffffffffff5f6af8f7b3396644f18e157960000000000000000000000000105711340daa0d5f769dba1915cef59f0815a5506029190037d0267a36c0c95b3975ab3ee5b203a7614a3f75373f047d803ae7b6687f2b302017d57115e47018c7177eebf7cd370a3356a1b7863008a5ae8028c72b88642840160ae1d90565b5f81831261569f576040517fa883435700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b828203836156ad86836145a5565b0195945050505050565b5f7ffffffffffffffffffffffffffffffffffffffffffffffffdc0d0570925a462d782136156e457919050565b680755bf798b4a1bf1e582126157015763a37bfec95f526004601cfd5b6503782dace9d9604e83901b0591505f60606bb17217f7d1cf79abc9e3b39884821b056b80000000000000000000000001901d6bb17217f7d1cf79abc9e3b39881029093037fffffffffffffffffffffffffffffffffffffffdbf3ccf1604d263450f02a550481018102606090811d6d0277594991cfc85f6e2461837cd9018202811d7fffffffffffffffffffffffffffffffffffffe5adedaa1cb095af9e4da10e363c018202811d6db1bbb201f443cf962f1a1d3db4a5018202811d7ffffffffffffffffffffffffffffffffffffd38dc772608b0ae56cce01296c0eb018202811d6e05180bb14799ab47a8a8cb2a527d57016d02d16720577bd19bf614176fe9ea6c10fe68e7fd37d0007b713f765084018402831d9081019084017ffffffffffffffffffffffffffffffffffffffe2c69812cf03b0763fd454a8f7e010290911d6e0587f503bb6ea29d25fcb7401964500190910279d835ebba824c98fb31b83b2ca45c000000000000000000000000010574029d9dc38563c32e5c2f6dc192ee70ef65f9978af30260c3939093039290921c92915050565b5f8082121561475a576040517fcaccb6d900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f815f19048311156158f65781156158f65763bac65e5b5f526004601cfd5b50670de0b6b3a764000091020490565b6040517fd17d4b0d000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d9063d17d4b0d9061595a90869086908690600401618ebe565b5f6040518083038186803b158015615970575f80fd5b505afa158015615982573d5f803e3d5ffd5b50505050505050565b606060018360020b12156159e15760405162461bcd60e51b815260206004820152601460248201527f496e76616c6964205449434b5f53504143494e470000000000000000000000006044820152606401614656565b855115614810576159f485858886616a61565b6159fd86616b22565b8160020b8660018851615a109190618935565b81518110615a2057615a2061877a565b60200260200101515f015160020b13615ab85760408051600180825281830190925290816020015b6040805160c0810182525f80825260208083018290529282018190526060808301829052608083015260a082015282525f19909201910181615a48579050509050615a968585888587616bdb565b815f81518110615aa857615aa861877a565b6020026020010181905250614810565b855f81518110615aca57615aca61877a565b60200260200101515f015160020b8260020b13615b445760408051600180825281830190925290816020015b6040805160c0810182525f80825260208083018290529282018190526060808301829052608083015260a082015282525f19909201910181615af6579050509050615a9685858885876170e4565b815f5b615b5c6001600160a01b0388168784886175f9565b9250905080615b775781615b6f81618edc565b925050615b47565b875f81518110615b8957615b8961877a565b60200260200101515f015160020b8260020b13615c275760408051600180825281830190925290816020015b6040805160c0810182525f80825260208083018290529282018190526060808301829052608083015260a082015282525f19909201910181615bb5579050509250615c0387878a87896170e4565b835f81518110615c1557615c1561877a565b60200260200101819052505050614810565b50505f615c34878461764e565b90505f8167ffffffffffffffff811115615c5057615c50618698565b604051908082528060200260200182016040528015615c9457816020015b604080518082019091525f8082526020820152815260200190600190039081615c6e5790505b5090505f828951615ca59190618935565b67ffffffffffffffff811115615cbd57615cbd618698565b604051908082528060200260200182016040528015615d0157816020015b604080518082019091525f8082526020820152815260200190600190039081615cdb5790505b5090505f5b8951811015615d9e5783811015615d5357898181518110615d2957615d2961877a565b6020026020010151838281518110615d4357615d4361877a565b6020026020010181905250615d96565b898181518110615d6557615d6561877a565b6020026020010151828583615d7a9190618935565b81518110615d8a57615d8a61877a565b60200260200101819052505b600101615d06565b5081515f03615e2f5760408051600180825281830190925290816020015b6040805160c0810182525f80825260208083018290529282018190526060808301829052608083015260a082015282525f19909201910181615dbc579050509350615e0a888883888a6170e4565b845f81518110615e1c57615e1c61877a565b6020026020010181905250505050614810565b80515f03615e9a5760408051600180825281830190925290816020015b6040805160c0810182525f80825260208083018290529282018190526060808301829052608083015260a082015282525f19909201910181615e4c579050509350615e0a888884888a616bdb565b6040805160028082526060820190925290816020015b6040805160c0810182525f80825260208083018290529282018190526060808301829052608083015260a082015282525f19909201910181615eb0579050509350615efe888883888a6170e4565b845f81518110615f1057615f1061877a565b6020026020010181905250615f28888884888a616bdb565b84600181518110615e1c57615e1c61877a565b5f81602052631e2eaeaf5f5260205f6024601c865afa615f625763535cf94b5f526004601cfd5b50505f51919050565b60606105a3826167fd565b8051602080830151604080850151606080870151925195811b7fffffffffffffffffffffffffffffffffffffffff0000000000000000000000001694860194909452608092831b7fffffffffffffffffffffffffffffffff00000000000000000000000000000000908116603487015290831b81166044860152911b166054830152906064016040516020818303038152906040529050604481511461605e5760405162461bcd60e51b815260206004820152601860248201527f41737365747320756e6578706563746564206c656e67746800000000000000006044820152606401614656565b919050565b5f6301000000821061475a5761475a6176ec565b6060616082846176f9565b835160208501515f91829161609891879161776e565b915091505f6160af85885f0151896020015161779b565b90508260f01b8260f01b8260f01b6160c88a6040015190565b6040517fffff000000000000000000000000000000000000000000000000000000000000948516602082015292841660228401529216602482015260268101919091526046016040516020818303038152906040529350602684511461612c575f80fd5b5050509392505050565b5f80826001600160a01b0316846001600160a01b0316036161995760405162461bcd60e51b815260206004820152601360248201527f6173736574496e203d3d2061737365744f7574000000000000000000000000006044820152606401614656565b826001600160a01b0316846001600160a01b03161090505f80826161be5784866161c1565b85855b915091505f93505b86518461ffff161015616259575f878561ffff16815181106161ed576161ed61877a565b60200260200101519050616200816176f9565b826001600160a01b0316815f01516001600160a01b03161480156162395750816001600160a01b031681602001516001600160a01b0316145b15616246575050506162b0565b508361625181618f19565b9450506161c9565b86518461ffff16106162ad5760405162461bcd60e51b815260206004820152600e60248201527f50616972206e6f7420666f756e640000000000000000000000000000000000006044820152606401614656565b50505b935093915050565b80516060901561631157816020015160801b826060015160801b60405160200161497c9291907fffffffffffffffffffffffffffffffff0000000000000000000000000000000092831681529116601082015260200190565b60605f5b83608001515181101561636f5781846080015182815181106163395761633961877a565b602002602001015160801b604051602001616355929190618f30565b60408051601f198184030181529190529150600101616315565b5061637a8151616063565b60e81b8160405160200161638f929190618dd6565b6040516020818303038152906040529050826040015160e81b836060015160801b828560a0015160601b6040516020016148ea9493929190618f6c565b613895816178d6614dd8565b5f6a636f6e736f6c652e6c6f6790505f80835160208501845afa505050565b60020b5f60ff82901d80830118620d89e8811115616439576164397f8b86327a00000000000000000000000000000000000000000000000000000000846178f6565b7001fffcb933bd6fad37aa2d162d1a5940016001821602700100000000000000000000000000000000186002821615616482576ffff97272373d413259a46990580e213a0260801c5b60048216156164a1576ffff2e50f5f656932ef12357cf3c7fdcc0260801c5b60088216156164c0576fffe5caca7e10e4e61c3624eaa0941cd00260801c5b60108216156164df576fffcb9843d60f6159c9db58835c9266440260801c5b60208216156164fe576fff973b41fa98c081472e6896dfb254c00260801c5b604082161561651d576fff2ea16466c96a3843ec78b326b528610260801c5b608082161561653c576ffe5dee046a99a2a811c461f1969c30530260801c5b61010082161561655c576ffcbe86c7900a88aedcffc83b479aa3a40260801c5b61020082161561657c576ff987a7253ac413176f2b074cf7815e540260801c5b61040082161561659c576ff3392b0822b70005940c7a398e4b70f30260801c5b6108008216156165bc576fe7159475a2c29b7443b29c7fa6e889d90260801c5b6110008216156165dc576fd097f3bdfd2022b8845ad8f792aa58250260801c5b6120008216156165fc576fa9f746462d870fdf8a65dc1f90e061e50260801c5b61400082161561661c576f70d869a156d2a1b890bb3df62baf32f70260801c5b61800082161561663c576f31be135f97d08fd981231505542fcfa60260801c5b6201000082161561665d576f09aa508b5b7a84e1c677de54f3e99bc90260801c5b6202000082161561667d576e5d6af8dedb81196699c329225ee6040260801c5b6204000082161561669c576d2216e584f5fa1ea926041bedfe980260801c5b620800008216156166b9576b048a170391f7dc42444e8fa20260801c5b5f8413156166c5575f19045b63ffffffff0160201c9392505050565b5f836001600160a01b0316856001600160a01b031611156166f4579293925b6001600160a01b03851661670e5762bfc9215f526004601cfd5b7bffffffffffffffffffffffffffffffff000000000000000000000000606084901b166001600160a01b03868603168361677357866001600160a01b03166167608383896001600160a01b0316617905565b8161676d5761676d618a73565b04613d95565b613d9561678a8383896001600160a01b03166179a1565b886001600160a01b0316808204910615150190565b5f6001600160a01b038481169086160360ff81901d908101186c010000000000000000000000006fffffffffffffffffffffffffffffffff85166167e4818484617905565b9350845f83858409111684019350505050949350505050565b60606080604051019050602081016040525f8152805f19835b928101926030600a8206018453600a900480616816575050819003601f19909101908152919050565b5f805f845f015184106168945760405162461bcd60e51b815260206004820152601360248201527f496e646578206f75742d6f662d626f756e6473000000000000000000000000006044820152606401614656565b5f91506168fe805b6020870151518410156168e6575f6168bc8860200151868463ffffffff16565b905086815f0151036168da57602001516001955092506168ef915050565b5060019093019261689c565b505f9350849150505b9250925092565b602090910152565b81515f90821061693a576040517f44dd369f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b826020015182815181106169505761695061877a565b6020026020010151905092915050565b8151808361696d82618ffa565b9052506020830151518351811015616a38575f61698b826002618a0f565b90505f6001821061699c578161699f565b60015b67ffffffffffffffff8111156169b7576169b7618698565b6040519080825280602002602001820160405280156169e0578160200160208202803683370190505b5090505f5b83811015616a305786602001518181518110616a0357616a0361877a565b6020026020010151828281518110616a1d57616a1d61877a565b60209081029190910101526001016169e5565b506020860152505b8284602001518381518110616a4f57616a4f61877a565b60200260200101818152505050505050565b5f5b825181101561438d575f838281518110616a7f57616a7f61877a565b60200260200101515f015190505f80616aa1613d5a84875f8183071291050390565b90925090505f616abb6001600160a01b038a1689856151d3565b9050600160ff83161b8116616b125760405162461bcd60e51b815260206004820152601460248201527f5469636b206e6f7420696e697469616c697a65640000000000000000000000006044820152606401614656565b505060019092019150616a639050565b616b2b816179d1565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8000005f5b8251811015616bd6575f838281518110616b6a57616b6a61877a565b60200260200101515f015190508260020b8160020b13616bcc5760405162461bcd60e51b815260206004820152600e60248201527f4475706c6963617465207469636b0000000000000000000000000000000000006044820152606401614656565b9150600101616b4e565b505050565b6040805160c0810182525f808252602082018190529181018290526060808201839052608082015260a08101919091525f845111616c5b5760405162461bcd60e51b815260206004820152600a60248201527f4e6f2072657761726473000000000000000000000000000000000000000000006044820152606401614656565b5f616c7e600286516003616c6f9190618a0f565b616c799190618aa0565b615217565b90505f855f81518110616c9357616c9361877a565b602090810291909101015151905060015f5b616cba6001600160a01b038b168a8589617abe565b93509150600283810b9088900b12616d70578115616ce757616ce084600285900b616960565b505f616ca5565b80616cf181618ffa565b9150506078811115616d6b5760405162461bcd60e51b815260206004820152602d60248201527f4d41585f4c4f4f5020657863656564656420696e205f6372656174655265776160448201527f726455706461746542656c6f77000000000000000000000000000000000000006064820152608401614656565b616ca5565b505081515f039050616dfd578451600114616dcd5760405162461bcd60e51b815260206004820152601960248201527f65787065637465642072657761726473206c656e6774682031000000000000006044820152606401614656565b616df58787875f81518110616de457616de461877a565b602002602001015160200151617afb565b915050614810565b8051616e0a9060016187d4565b67ffffffffffffffff811115616e2257616e22618698565b604051908082528060200260200182016040528015616e4b578160200160208202803683370190505b5060808301525f80805b8351811015616f1d575f616e6985836168fe565b90505f616e806001600160a01b038d168c846150b4565b9150616e8e90508185619012565b93508951851015616f13575f8a8681518110616eac57616eac61877a565b602002602001015190508260020b815f015160020b1215616f1157806020015188608001518581518110616ee257616ee261877a565b6fffffffffffffffffffffffffffffffff9092166020928302919091019091015285616f0d81618ffa565b9650505b505b5050600101616e55565b508651821015616f8d57868281518110616f3957616f3961877a565b6020026020010151602001518460800151845f015181518110616f5e57616f5e61877a565b6fffffffffffffffffffffffffffffffff9092166020928302919091019091015281616f8981618ffa565b9250505b86518214616fdd5760405162461bcd60e51b815260206004820152601560248201527f4e6f7420616c6c207265776172647320757365643f00000000000000000000006044820152606401614656565b616fe7835f6168fe565b60020b60408501525f616ffc8a8a8989617b6e565b90506170088183617dad565b6fffffffffffffffffffffffffffffffff166060860181905289908b905f90815b88518110156170cb575f61703d8a836168fe565b90505f6170546001600160a01b03871688846150b4565b9150506170618482617dc7565b60408051602080820198909852608083901b7fffffffffffffffffffffffffffffffff00000000000000000000000000000000168183015260e89490941b605085015280516033818603018152605390940190528251929095019190912093925050600101617029565b505060601c60a088015250505050505095945050505050565b6040805160c0810182525f808252602082018190529181018290526060808201839052608082015260a08101919091525f8451116171645760405162461bcd60e51b815260206004820152600a60248201527f4e6f2072657761726473000000000000000000000000000000000000000000006044820152606401614656565b5f617178600286516003616c6f9190618a0f565b90505f856001875161718a9190618935565b8151811061719a5761719a61877a565b60209081029190910101515190506001815f5b8160020b8860020b12156172795782156171d6576171cf85600284900b616960565b505f61725a565b806171e081618ffa565b915050607881111561725a5760405162461bcd60e51b815260206004820152602d60248201527f4d41585f4c4f4f5020657863656564656420696e205f6372656174655265776160448201527f726455706461746541626f7665000000000000000000000000000000000000006064820152608401614656565b61726f6001600160a01b038c168b848a617de1565b90935091506171ad565b505082515f039050617317576172a660405180608001604052806044815260200161a93260449139613824565b85516001146172f75760405162461bcd60e51b815260206004820152601960248201527f4578706563746564206578616374206f6e6520726577617264000000000000006044820152606401614656565b61730e8888885f81518110616de457616de461877a565b92505050614810565b600281900b6040840152815161732e9060016187d4565b67ffffffffffffffff81111561734657617346618698565b60405190808252806020026020018201604052801561736f578160200160208202803683370190505b50608084015285515f805b8451811015617448575f61738e86836168fe565b90505f6173a56001600160a01b038e168d846150b4565b91506173b390508185619012565b9350841561743e575f8b6173c8600188618935565b815181106173d8576173d861877a565b602002602001015190508260020b815f015160020b1261743c5780602001518960800151858151811061740d5761740d61877a565b6fffffffffffffffffffffffffffffffff909216602092830291909101909101528561743881619060565b9650505b505b505060010161737a565b5081156174b557875f815181106174615761746161877a565b6020026020010151602001518560800151855f0151815181106174865761748661877a565b6fffffffffffffffffffffffffffffffff90921660209283029190910190910152816174b181619060565b9250505b81156175035760405162461bcd60e51b815260206004820152601560248201527f4e6f7420616c6c207265776172647320757365643f00000000000000000000006044820152606401614656565b5f6175108b8b8a8a617b6e565b905061751c8183617dc7565b6fffffffffffffffffffffffffffffffff16606087018190528a908c905f90815b89518110156175df575f6175518b836168fe565b90505f6175686001600160a01b03871688846150b4565b9150506175758482617dad565b60408051602080820198909852608083901b7fffffffffffffffffffffffffffffffff00000000000000000000000000000000168183015260e89490941b60508501528051603381860301815260539094019052825192909501919091209392505060010161753d565b505060601c60a08901525050505050505095945050505050565b5f80808061760e858707821386880503613d5a565b90925090506176318161762b6001600160a01b038b168a866151d3565b90617dfb565b9094509050617641828287617ec3565b9250505094509492505050565b5f8083511161769f5760405162461bcd60e51b815260206004820152600a60248201527f4e6f2072657761726473000000000000000000000000000000000000000000006044820152606401614656565b5f5b83518110156176e3578260020b8482815181106176c0576176c061877a565b60200260200101515f015160020b13156176db5790506105a3565b6001016176a1565b50509051919050565b6335278d125f526004601cfd5b805f01516001600160a01b031681602001516001600160a01b03161161389557604080517f5190344300000000000000000000000000000000000000000000000000000000815282516001600160a01b0390811660048301526020840151166024820152908201516044820152606401614656565b5f8061778261777d8686617eed565b617f86565b915061779161777d8685617eed565b9050935093915050565b5f816001600160a01b0316836001600160a01b0316106177fd5760405162461bcd60e51b815260206004820152601d60248201527f67657453746f7265496e6465783a61737365747320756e736f727465640000006044820152606401614656565b5f83815260208381526040822060281b9190617823906001600160a01b0388163b618aa0565b5f93509050855b818461ffff16101561788e575f6020856020026001015f843c505f517fffffffffffffffffffffffffffffffffffffffffffffffffffffff00000000008085169082160361787b5750505050610623565b508361788681618f19565b94505061782a565b60405162461bcd60e51b815260206004820152601060248201527f506f6f6c206e6f7420656e61626c6564000000000000000000000000000000006044820152606401614656565b80516a636f6e736f6c652e6c6f67602083015f808483855afa5050505050565b815f528060020b60045260245ffd5b5f838302815f1985870982811083820303915050808411617924575f80fd5b805f0361793657508290049050610623565b5f848688095f868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150509392505050565b5f6179ad848484617905565b905081806179bd576179bd618a73565b838509156106235760010180610623575f80fd5b5f5b8151811015613f08575f6179e88260016187d4565b90505b8251811015617ab557617a39838281518110617a0957617a0961877a565b6020026020010151848481518110617a2357617a2361877a565b6020026020010151617f9990919063ffffffff16565b15617aad57828181518110617a5057617a5061877a565b6020026020010151838381518110617a6a57617a6a61877a565b6020026020010151848481518110617a8457617a8461877a565b60200260200101858481518110617a9d57617a9d61877a565b6020026020010182905282905250505b6001016179eb565b506001016179d3565b5f808080617ad8613d5a8688078313878905036001618cd8565b909250905061763181617af56001600160a01b038b168a866151d3565b90617fa7565b6040805160c0810182525f9181018290526060808201839052608082015260a0810191909152600181526fffffffffffffffffffffffffffffffff82166020820152617b506001600160a01b03851684618069565b6fffffffffffffffffffffffffffffffff1660608201529392505050565b5f80617b8661325e6001600160a01b03881687614819565b90505f617b9c6001600160a01b03881687618069565b90508460020b8260020b1215617c9b575f82815b617bc56001600160a01b038b168a848a617abe565b9093509150600282810b9089900b12617c93578215617c0a57505f80617bf56001600160a01b038c168b856150b4565b915050617c028582617dc7565b945050617bb0565b80617c1481618ffa565b9150506078811115617c8e5760405162461bcd60e51b815260206004820152603a60248201527f4d41585f4c4f4f5020657863656564656420696e206765744c6971756964697460448201527f7941745469636b205b70726573656e74203c206675747572655d0000000000006064820152608401614656565b617bb0565b505050617da3565b8160020b8560020b1215617da3576001825f5b617cc36001600160a01b038b168a848a6175f9565b9093509150600288810b9083900b1315617d9f578215617d0957505f80617cf46001600160a01b038c168b856150b4565b915050617d018582617dad565b945050617d8d565b80617d1381618ffa565b9150506078811115617d8d5760405162461bcd60e51b815260206004820152603a60248201527f4d41585f4c4f4f5020657863656564656420696e206765744c6971756964697460448201527f7941745469636b205b667574757265203c2070726573656e745d0000000000006064820152608401614656565b81617d9781618edc565b925050617cae565b5050505b9695505050505050565b808203608081901c156105a35763c9654ed45f526004601cfd5b818101608081901c156105a35763c9654ed45f526004601cfd5b5f80808061760e613d5a60018789078413888a0503618c97565b5f805f8360ff0390505f617e9c8260ff1687901b7f0706060506020504060203020504030106050205030304010505030400000000601f6f8421084210842108cc6318c6db6d54be831560081b6fffffffffffffffffffffffffffffffff851160071b1784811c67ffffffffffffffff1060061b1784811c63ffffffff1060051b1784811c61ffff1060041b1784811c60ff1060031b1793841c1c161a1790565b9050806101001415935083617eb1575f617eb8565b8160ff1681035b925050509250929050565b5f8160ff8416617ed9600187900b610100618bde565b617ee39190618cd8565b6148399190618bde565b5f805b8351811015617f3d57838181518110617f0b57617f0b61877a565b60200260200101515f01516001600160a01b0316836001600160a01b031603617f355790506105a3565b600101617ef0565b5060405162461bcd60e51b815260206004820152600f60248201527f4173736574206e6f7420666f756e6400000000000000000000000000000000006044820152606401614656565b5f62010000821061475a5761475a6176ec565b519051600291820b910b1390565b5f805f6180428460ff1686901c7e1f0d1e100c1d070f090b19131c1706010e11080a1a141802121b150316040581196001019091166101e07f804040554300526644320000502061067405302602000010750620017611707760fc7fb6db6db6ddddddddd34d34d349249249210842108c6318c639ce739cffffffff840260f81c161b60f71c1690811c63d76453e004601f169190911a1790565b90508061010014159250826180585760ff61805f565b8360ff1681015b9150509250929050565b5f8181526006602052604081205f6148106001600160a01b03861660038401615f3b565b6118bc8061907683390190565b60405180604001604052805f81526020016180c760405180604001604052805f8152602001606081525090565b905290565b5f602082840312156180dc575f80fd5b5035919050565b5f80604083850312156180f4575f80fd5b50508035926020909101359150565b602080825282518282018190525f918401906040840190835b818110156181435783516001600160a01b031683526020938401939092019160010161811c565b509095945050505050565b602080825282518282018190525f918401906040840190835b81811015618143578351835260209384019390920191600101618167565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f82825180855260208501945060208160051b830101602085015f5b8381101561820157601f198584030188526181eb838351618185565b60209889019890935091909101906001016181cf565b50909695505050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156182a2577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc087860301845281516001600160a01b038151168652602081015190506040602087015261828c60408701826181b3565b9550506020938401939190910190600101618233565b50929695505050505050565b6001600160a01b0381168114613895575f80fd5b5f602082840312156182d2575f80fd5b8135610623816182ae565b8060020b8114613895575f80fd5b5f805f805f60a086880312156182ff575f80fd5b85359450602086013593506040860135618318816182dd565b92506060860135618328816182dd565b949793965091946080013592915050565b5f8151808452602084019350602083015f5b8281101561838b5781517fffffffff000000000000000000000000000000000000000000000000000000001686526020958601959091019060010161834b565b5093949350505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156182a2577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc087860301845281518051604087526183ff6040880182618185565b905060208201519150868103602088015261841a8183618339565b9650505060209384019391909101906001016183bb565b5f8151808452602084019350602083015f5b8281101561838b57815180518752602081015160208801526040810151604088015250606086019550602082019150600181019050618443565b60208152815160020b6020820152602082015160020b60408201525f60408301516184b360608401826001600160a01b03169052565b5060608301516080830152608083015160a083015260a083015160c083015260c083015160e083015260e083015161010080840152614839610120840182618431565b602080825282518282018190525f918401906040840190835b81811015618143578351805160020b84526020908101516fffffffffffffffffffffffffffffffff16818501529093019260409092019160010161850f565b602081525f6105a060208301846181b3565b5f805f60608486031215618572575f80fd5b505081359360208301359350604090920135919050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156182a2577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc087860301845281516001600160a01b03815116865260208101519050604060208701526186086040870182618339565b95505060209384019391909101906001016185af565b803562ffffff8116811461605e575f80fd5b5f805f805f8060c08789031215618645575f80fd5b8635955060208701359450604087013561865e816182dd565b935061866c6060880161861e565b925061867a6080880161861e565b915060a087013561868a816182ae565b809150509295509295509295565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b604051601f8201601f1916810167ffffffffffffffff811182821017156186ee576186ee618698565b604052919050565b5f805f8385036060811215618709575f80fd5b8435935060208086013593507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc082011215618742575f80fd5b506040516020810167ffffffffffffffff8111828210171561876657618766618698565b604090815294909401358452509093909250565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b808201808211156105a3576105a36187a7565b600181811c908216806187fb57607f821691505b602082108103618832577f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b50919050565b5f60208284031215618848575f80fd5b815167ffffffffffffffff81111561885e575f80fd5b8201601f8101841361886e575f80fd5b805167ffffffffffffffff81111561888857618888618698565b61889b6020601f19601f840116016186c5565b8181528560208385010111156188af575f80fd5b8160208401602083015e5f91810160200191909152949350505050565b5f81518060208401855e5f93019283525090919050565b7f6163746f725f000000000000000000000000000000000000000000000000000081525f6105a060068301846188cc565b6001600160a01b0383168152604060208201525f6148396040830184618185565b818103818111156105a3576105a36187a7565b5f60208284031215618958575f80fd5b81518015158114610623575f80fd5b61012081016189c482886001600160a01b0381511682526001600160a01b03602082015116602083015262ffffff6040820151166040830152606081015160020b60608301526001600160a01b0360808201511660808301525050565b8560020b60a08301528460020b60c08301528360e0830152826101008301529695505050505050565b5f80604083850312156189fe575f80fd5b505080516020909101519092909150565b80820281158282048414176105a3576105a36187a7565b5f60208284031215618a36575f80fd5b5051919050565b5f7f80000000000000000000000000000000000000000000000000000000000000008203618a6d57618a6d6187a7565b505f0390565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f82618aae57618aae618a73565b500490565b5f60208284031215618ac3575f80fd5b8151610623816182ae565b60c08101618b2a82856001600160a01b0381511682526001600160a01b03602082015116602083015262ffffff6040820151166040830152606081015160020b60608301526001600160a01b0360808201511660808301525050565b6001600160a01b03831660a08301529392505050565b5f60208284031215618b50575f80fd5b8151610623816182dd565b5f614810618b72618b6c84886188cc565b866188cc565b846188cc565b602081525f6105a06020830184618185565b5f8160020b8360020b80618ba057618ba0618a73565b5f1981147fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80000083141615618bd557618bd56187a7565b90059392505050565b5f8260020b8260020b028060020b91508082146136a6576136a66187a7565b6fffffffffffffffffffffffffffffffff82811682821603908111156105a3576105a36187a7565b604081525f618c376040830185618185565b82810360208401526148108185618185565b604081525f618c5b6040830185618185565b90508260208301529392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603160045260245ffd5b600282810b9082900b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8000008112627fffff821317156105a3576105a36187a7565b600281810b9083900b01627fffff81137fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff800000821217156105a3576105a36187a7565b5f618d2482856188cc565b7f2c20000000000000000000000000000000000000000000000000000000000000815261481060028201856188cc565b5f6fffffffffffffffffffffffffffffffff82166fffffffffffffffffffffffffffffffff8103618d8757618d876187a7565b60010192915050565b5f618d9b82846188cc565b7f5d0000000000000000000000000000000000000000000000000000000000000081526001019392505050565b5f614839618b7283866188cc565b7fffffff0000000000000000000000000000000000000000000000000000000000831681525f61483960038301846188cc565b7fff00000000000000000000000000000000000000000000000000000000000000851681527fffff000000000000000000000000000000000000000000000000000000000000841660018201527fffffffffffffffffffffffffffffffff00000000000000000000000000000000831660038201525f617da360138301846188cc565b8082018281125f83128015821682158216171561401a5761401a6187a7565b5f82618eb957618eb9618a73565b500690565b838152826020820152606060408201525f6148106060830184618185565b5f8160020b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8000008103618f1057618f106187a7565b5f190192915050565b5f61ffff821661ffff8103618d8757618d876187a7565b5f618f3b82856188cc565b7fffffffffffffffffffffffffffffffff000000000000000000000000000000009390931683525050601001919050565b7fffffff0000000000000000000000000000000000000000000000000000000000851681527fffffffffffffffffffffffffffffffff00000000000000000000000000000000841660038201525f618fc760138301856188cc565b7fffffffffffffffffffffffffffffffffffffffff00000000000000000000000093909316835250506014019392505050565b5f5f19820361900b5761900b6187a7565b5060010190565b600f81810b9083900b016f7fffffffffffffffffffffffffffffff81137fffffffffffffffffffffffffffffffff80000000000000000000000000000000821217156105a3576105a36187a7565b5f8161906e5761906e6187a7565b505f19019056fe6080604052348015600e575f80fd5b506040516118bc3803806118bc833981016040819052602b91604e565b5f80546001600160a01b0319166001600160a01b03929092169190911790556079565b5f60208284031215605d575f80fd5b81516001600160a01b03811681146072575f80fd5b9392505050565b611836806100865f395ff3fe608060405234801561000f575f80fd5b506004361061006f575f3560e01c806391dd73461161004d57806391dd7346146100d4578063baca0004146100f4578063beabacc814610109575f80fd5b80630495a4a2146100735780630c8658791461009957806340e2a812146100c1575b5f80fd5b610086610081366004610d6e565b61011c565b6040519081526020015b60405180910390f35b6100ac6100a7366004610dcd565b61026c565b60408051928352602083019190915201610090565b6100866100cf366004610e66565b6103b8565b6100e76100e2366004610ee5565b61050f565b6040516100909190610f70565b610107610102366004610f82565b6106bf565b005b610107610117366004610f9d565b6106e3565b5f8054819073ffffffffffffffffffffffffffffffffffffffff166348c894918260f81b8860405180606001604052808a151581526020018981526020018873ffffffffffffffffffffffffffffffffffffffff168152506040516020016101859291906110bf565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0818403018152908290526101c19291602001611106565b6040516020818303038152906040526040518263ffffffff1660e01b81526004016101ec9190610f70565b5f604051808303815f875af1158015610207573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016820160405261024c9190810190611209565b9050808060200190518101906102629190611283565b9695505050505050565b5f80548190819073ffffffffffffffffffffffffffffffffffffffff166348c89491600260f81b8a60405180608001604052808c60020b81526020018b60020b81526020018a8152602001898152506040516020016102cc92919061129a565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0818403018152908290526103089291602001611106565b6040516020818303038152906040526040518263ffffffff1660e01b81526004016103339190610f70565b5f604051808303815f875af115801561034e573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01682016040526103939190810190611209565b9050808060200190518101906103a991906112db565b92509250509550959350505050565b5f8054819073ffffffffffffffffffffffffffffffffffffffff166348c89491600160f81b8a60405180606001604052808c151581526020018b81526020018a73ffffffffffffffffffffffffffffffffffffffff16815250888860405160200161042694939291906112fd565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0818403018152908290526104629291602001611106565b6040516020818303038152906040526040518263ffffffff1660e01b815260040161048d9190610f70565b5f604051808303815f875af11580156104a8573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01682016040526104ed9190810190611209565b9050808060200190518101906105039190611283565b98975050505050505050565b5f5460609073ffffffffffffffffffffffffffffffffffffffff163314610534575f80fd5b5f6105426001828587611390565b61054b916113b7565b60f81c600281111561055f5761055f610fdb565b90505f81600281111561057457610574610fdb565b036105bc575f806105888560018189611390565b810190610595919061151b565b915091506105b2828260405180602001604052805f815250610709565b93505050506106b9565b60018160028111156105d0576105d0610fdb565b0361060e575f80806105e5866001818a611390565b8101906105f2919061154f565b925092509250610603838383610709565b9450505050506106b9565b600281600281111561062257610622610fdb565b03610651575f806106368560018189611390565b81019061064391906115e8565b915091506105b282826107d7565b6040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601360248201527f556e7265636f676e697a656420616374696f6e00000000000000000000000000604482015260640160405180910390fd5b505b92915050565b6106df73ffffffffffffffffffffffffffffffffffffffff8216336108fd565b5050565b61070473ffffffffffffffffffffffffffffffffffffffff8416838361097f565b505050565b5f80546040517ff3cd914c0000000000000000000000000000000000000000000000000000000081526060929173ffffffffffffffffffffffffffffffffffffffff169063f3cd914c9061076590889088908890600401611699565b6020604051808303815f875af1158015610781573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107a59190611283565b90506107b185826109d2565b6040805160208101839052016040516020818303038152906040529150505b9392505050565b5f80546040517f5a6bcfda00000000000000000000000000000000000000000000000000000000815260609291829173ffffffffffffffffffffffffffffffffffffffff90911690635a6bcfda906108359088908890600401611763565b60408051808303815f875af1158015610850573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061087491906112db565b9150915061088b8561088684846109f9565b6109d2565b5f8460400151136108d05784515f805490916108bf9173ffffffffffffffffffffffffffffffffffffffff16903090610a48565b90506108ce865f015182610ae4565b505b60408051602081018490529081018290526060016040516020818303038152906040529250505092915050565b5f6370a082315f5230602052602060346024601c865afa601f3d111661092a576390b8ec185f526004601cfd5b8160145260345190506fa9059cbb0000000000000000000000005f5260205f604460105f875af18060015f51141661097457803d853b151710610974576390b8ec185f526004601cfd5b505f60345292915050565b81601452806034526fa9059cbb0000000000000000000000005f5260205f604460105f875af18060015f5114166109c857803d853b1517106109c8576390b8ec185f526004601cfd5b505f603452505050565b81516109e7906109e28360801d90565b610ae4565b6106df82602001516109e283600f0b90565b5f608082811d9084901d01600f83810b9085900b01610a3f610a1a83610ce3565b610a2383610ce3565b6fffffffffffffffffffffffffffffffff1660809190911b1790565b95945050505050565b5f8281526020829052604080822090517ff135baaa00000000000000000000000000000000000000000000000000000000815260048101829052829073ffffffffffffffffffffffffffffffffffffffff87169063f135baaa90602401602060405180830381865afa158015610ac0573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102629190611283565b5f81600f0b1215610c37575f546040517fa584119400000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff84811660048301529091169063a5841194906024015f604051808303815f87803b158015610b58575f80fd5b505af1158015610b6a573d5f803e3d5ffd5b50505f8054610baa935073ffffffffffffffffffffffffffffffffffffffff868116935016908490036fffffffffffffffffffffffffffffffff1661097f565b5f8054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166311da60b46040518163ffffffff1660e01b81526004016020604051808303815f875af1158015610c13573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107049190611283565b80600f0b5f12156106df575f546040517f0b0d9c0900000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff84811660048301523060248301526fffffffffffffffffffffffffffffffff8416604483015290911690630b0d9c09906064015f604051808303815f87803b158015610cc9575f80fd5b505af1158015610cdb573d5f803e3d5ffd5b505050505050565b80600f81900b8114610d1857610d187f93dafdf100000000000000000000000000000000000000000000000000000000610d1d565b919050565b805f5260045ffd5b5f60a08284031215610d35575f80fd5b50919050565b80358015158114610d18575f80fd5b73ffffffffffffffffffffffffffffffffffffffff81168114610d6b575f80fd5b50565b5f805f806101008587031215610d82575f80fd5b610d8c8686610d25565b9350610d9a60a08601610d3b565b925060c0850135915060e0850135610db181610d4a565b939692955090935050565b8035600281900b8114610d18575f80fd5b5f805f805f6101208688031215610de2575f80fd5b610dec8787610d25565b9450610dfa60a08701610dbc565b9350610e0860c08701610dbc565b9497939650939460e08101359450610100013592915050565b5f8083601f840112610e31575f80fd5b50813567ffffffffffffffff811115610e48575f80fd5b602083019150836020828501011115610e5f575f80fd5b9250929050565b5f805f805f806101208789031215610e7c575f80fd5b610e868888610d25565b9550610e9460a08801610d3b565b945060c0870135935060e0870135610eab81610d4a565b925061010087013567ffffffffffffffff811115610ec7575f80fd5b610ed389828a01610e21565b979a9699509497509295939492505050565b5f8060208385031215610ef6575f80fd5b823567ffffffffffffffff811115610f0c575f80fd5b610f1885828601610e21565b90969095509350505050565b5f81518084528060208401602086015e5f6020828601015260207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011685010191505092915050565b602081525f6107d06020830184610f24565b5f60208284031215610f92575f80fd5b81356107d081610d4a565b5f805f60608486031215610faf575f80fd5b8335610fba81610d4a565b92506020840135610fca81610d4a565b929592945050506040919091013590565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b803562ffffff81168114610d18575f80fd5b803561102581610d4a565b73ffffffffffffffffffffffffffffffffffffffff168252602081013561104b81610d4a565b73ffffffffffffffffffffffffffffffffffffffff16602083015262ffffff61107660408301611008565b16604083015261108860608201610dbc565b60020b6060830152608081013561109e81610d4a565b73ffffffffffffffffffffffffffffffffffffffff81166080840152505050565b61010081016110ce828561101a565b8251151560a0830152602083015160c0830152604083015173ffffffffffffffffffffffffffffffffffffffff1660e08301526107d0565b7fff00000000000000000000000000000000000000000000000000000000000000831681525f82518060208501600185015e5f92016001019182525092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b604051601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016810167ffffffffffffffff811182821017156111bc576111bc611148565b604052919050565b5f67ffffffffffffffff8211156111dd576111dd611148565b50601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01660200190565b5f60208284031215611219575f80fd5b815167ffffffffffffffff81111561122f575f80fd5b8201601f8101841361123f575f80fd5b805161125261124d826111c4565b611175565b818152856020838501011115611266575f80fd5b8160208401602083015e5f91810160200191909152949350505050565b5f60208284031215611293575f80fd5b5051919050565b61012081016112a9828561101a565b8251600290810b60a08401526020840151900b60c0830152604083015160e083015260608301516101008301526107d0565b5f80604083850312156112ec575f80fd5b505080516020909101519092909150565b611307818661101a565b8351151560a0820152602084015160c0820152604084015173ffffffffffffffffffffffffffffffffffffffff1660e08201526101206101008201528161012082015281836101408301375f81830161014090810191909152601f9092017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01601019392505050565b5f808585111561139e575f80fd5b838611156113aa575f80fd5b5050820193919092039150565b80357fff0000000000000000000000000000000000000000000000000000000000000081169060018410156106b7577fff00000000000000000000000000000000000000000000000000000000000000808560010360031b1b82161691505092915050565b5f60a0828403121561142c575f80fd5b60405160a0810167ffffffffffffffff8111828210171561144f5761144f611148565b604052905080823561146081610d4a565b8152602083013561147081610d4a565b602082015261148160408401611008565b604082015261149260608401610dbc565b606082015260808301356114a581610d4a565b6080919091015292915050565b5f606082840312156114c2575f80fd5b6040516060810167ffffffffffffffff811182821017156114e5576114e5611148565b6040529050806114f483610d3b565b815260208381013590820152604083013561150e81610d4a565b6040919091015292915050565b5f80610100838503121561152d575f80fd5b611537848461141c565b91506115468460a085016114b2565b90509250929050565b5f805f6101208486031215611562575f80fd5b61156c858561141c565b925061157b8560a086016114b2565b915061010084013567ffffffffffffffff811115611597575f80fd5b8401601f810186136115a7575f80fd5b80356115b561124d826111c4565b8181528760208385010111156115c9575f80fd5b816020840160208301375f602083830101528093505050509250925092565b5f808284036101208112156115fb575f80fd5b611605858561141c565b925060807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6082011215611636575f80fd5b506040516080810167ffffffffffffffff8111828210171561165a5761165a611148565b60405261166960a08501610dbc565b815261167760c08501610dbc565b602082015260e084013560408201526101009093013560608401525092909150565b611718818573ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b8251151560a0820152602083015160c0820152604083015173ffffffffffffffffffffffffffffffffffffffff1660e08201526101206101008201525f610a3f610120830184610f24565b6117e2818473ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b8151600290810b60a08301526020830151900b60c0820152604082015160e0820152606082015161010082015261014061012082018190525f90820152610160019291505056fea164736f6c634300081a000a5741524e494e470a5741524e494e473a2041626f766520736f6d65686f772063616c6c6564207769746820646f6e61746520746f2063757272656e74206f6e6c793f3f3fa164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x0C\x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x1F\x80T\x90\x91\x16\x90\x91\x17\x90U4\x80\x15a\0,W_\x80\xFD[P`@Qa\xAD\x048\x03\x80a\xAD\x04\x839\x81\x01`@\x81\x90Ra\0K\x91a\x02\x8CV[\x80Q` \x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x82U\x81\x84\x01Q`!\x80T\x83\x16\x91\x85\x16\x91\x90\x91\x17\x90U`@\x84\x01Q`\"\x80T\x83\x16\x91\x85\x16\x91\x90\x91\x17\x90U``\x84\x01Q`#\x80T\x83\x16\x91\x85\x16\x91\x90\x91\x17\x90U`\x80\x84\x01Q`$\x80T\x90\x92\x16\x93\x16\x92\x90\x92\x17\x90\x91U`\xA0\x82\x01Q\x80Q\x83\x92\x91a\0\xD4\x91`%\x91\x84\x01\x90a\0\xFAV[P`\xC0\x82\x01Q\x80Qa\0\xF0\x91`\x06\x84\x01\x91` \x90\x91\x01\x90a\0\xFAV[P\x90PPPa\x03uV[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15a\x01MW\x91` \x02\x82\x01[\x82\x81\x11\x15a\x01MW\x82Q\x82T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90a\x01\x18V[Pa\x01Y\x92\x91Pa\x01]V[P\x90V[[\x80\x82\x11\x15a\x01YW_\x81U`\x01\x01a\x01^V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x01\xA7Wa\x01\xA7a\x01qV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x01\xD5Wa\x01\xD5a\x01qV[`@R\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xF1W_\x80\xFD[PV[\x80Qa\x01\xFF\x81a\x01\xDDV[\x91\x90PV[_\x82`\x1F\x83\x01\x12a\x02\x13W_\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02,Wa\x02,a\x01qV[\x80`\x05\x1Ba\x02<` \x82\x01a\x01\xADV[\x91\x82R` \x81\x85\x01\x81\x01\x92\x90\x81\x01\x90\x86\x84\x11\x15a\x02WW_\x80\xFD[` \x86\x01\x92P[\x83\x83\x10\x15a\x02\x82W\x82Qa\x02q\x81a\x01\xDDV[\x82R` \x92\x83\x01\x92\x90\x91\x01\x90a\x02^V[\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a\x02\x9CW_\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02\xB1W_\x80\xFD[\x82\x01`\xE0\x81\x85\x03\x12\x15a\x02\xC2W_\x80\xFD[a\x02\xCAa\x01\x85V[a\x02\xD3\x82a\x01\xF4V[\x81Ra\x02\xE1` \x83\x01a\x01\xF4V[` \x82\x01Ra\x02\xF2`@\x83\x01a\x01\xF4V[`@\x82\x01Ra\x03\x03``\x83\x01a\x01\xF4V[``\x82\x01Ra\x03\x14`\x80\x83\x01a\x01\xF4V[`\x80\x82\x01R`\xA0\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x031W_\x80\xFD[a\x03=\x86\x82\x85\x01a\x02\x04V[`\xA0\x83\x01RP`\xC0\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x03[W_\x80\xFD[a\x03g\x86\x82\x85\x01a\x02\x04V[`\xC0\x83\x01RP\x94\x93PPPPV[a\xA9\x82\x80a\x03\x82_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\x025W_5`\xE0\x1C\x80c\x85v \xE1\x11a\x01=W\x80c\xB0FO\xDC\x11a\0\xB8W\x80c\xBAAO\xA6\x11a\0\x88W\x80c\xE2\x0C\x9Fq\x11a\0nW\x80c\xE2\x0C\x9Fq\x14a\x04\xD6W\x80c\xEE\xE8\xE6{\x14a\x04\xDEW\x80c\xFAv&\xD4\x14a\x04\xF1W_\x80\xFD[\x80c\xBAAO\xA6\x14a\x04\xBEW\x80c\xCC\x7F\x0C$\x14a\x04\x93W_\x80\xFD[\x80c\xB0FO\xDC\x14a\x04\x9BW\x80c\xB1Zyq\x14a\x04\xA3W\x80c\xB1e\xC9\xE9\x14a\x04\x80W\x80c\xB5P\x8A\xA9\x14a\x04\xB6W_\x80\xFD[\x80c\x91j\x17\xC6\x11a\x01\rW\x80c\xAB<~R\x11a\0\xF3W\x80c\xAB<~R\x14a\x04xW\x80c\xAC\xEB\x0E\x85\x14a\x04\x80W\x80c\xAE\xB8\xFB\xF9\x14a\x04\x93W_\x80\xFD[\x80c\x91j\x17\xC6\x14a\x04DW\x80c\x93\xDBE\xE0\x14a\x04YW_\x80\xFD[\x80c\x85v \xE1\x14a\x03\xECW\x80c\x88\xBD\xCAa\x14a\x03\xFFW\x80c\x89\x85\xC9\x0B\x14a\x04\x12W\x80c\x8F]#\xCE\x14a\x04%W_\x80\xFD[\x80c^\xE4\x81\xB6\x11a\x01\xCDW\x80cv\xE1\xFC\xC4\x11a\x01\x9DW\x80c\x80h\xB5.\x11a\x01\x83W\x80c\x80h\xB5.\x14a\x03\x99W\x80c\x82qnC\x14a\x03\xB8W\x80c\x85\"l\x81\x14a\x03\xD7W_\x80\xFD[\x80cv\xE1\xFC\xC4\x14a\x03fW\x80c{*\xBD\xB6\x14a\x03yW_\x80\xFD[\x80c^\xE4\x81\xB6\x14a\x02\xFDW\x80cd#\x9C\xDD\x14a\x03\x1CW\x80cf\xD9\xA9\xA0\x14a\x031W\x80ctw\xF5\x17\x14a\x03FW_\x80\xFD[\x80c*\xDE8\x80\x11a\x02\x08W\x80c*\xDE8\x80\x14a\x02\xD0W\x80c>^<#\x14a\x02\xE5W\x80c?r\x86\xF4\x14a\x02\xEDW\x80cG\x8D\xDE\xCC\x14a\x02\xF5W_\x80\xFD[\x80c\x06\x8B\xCD\x8D\x14a\x029W\x80c\r^\xC4\xC6\x14a\x02zW\x80c\x1E\xD7\x83\x1C\x14a\x02\x9BW\x80c(\x95\xA2\xB3\x14a\x02\xB0W[_\x80\xFD[a\x02La\x02G6`\x04a\x80\xCCV[a\x04\xFEV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x93\x90\x92\x16` \x84\x01R`\x02\x0B\x90\x82\x01R``\x01[`@Q\x80\x91\x03\x90\xF3[a\x02\x8Da\x02\x886`\x04a\x80\xE3V[a\x05\x95V[`@Q\x90\x81R` \x01a\x02qV[a\x02\xA3a\x05\xA9V[`@Qa\x02q\x91\x90a\x81\x03V[a\x02\xC3a\x02\xBE6`\x04a\x80\xCCV[a\x06\tV[`@Qa\x02q\x91\x90a\x81NV[a\x02\xD8a\x06*V[`@Qa\x02q\x91\x90a\x82\rV[a\x02\xA3a\x07fV[a\x02\xA3a\x07\xC4V[a\x02\xA3a\x08\"V[a\x02\x8Da\x03\x0B6`\x04a\x82\xC2V[`*` R_\x90\x81R`@\x90 T\x81V[a\x03/a\x03*6`\x04a\x82\xEBV[a\x083V[\0[a\x039a\x12\xFCV[`@Qa\x02q\x91\x90a\x83\x95V[a\x03Ya\x03T6`\x04a\x80\xE3V[a\x14uV[`@Qa\x02q\x91\x90a\x84}V[a\x02\x8Da\x03t6`\x04a\x80\xE3V[a\x15\xBCV[a\x03\x8Ca\x03\x876`\x04a\x80\xCCV[a\x15\xC7V[`@Qa\x02q\x91\x90a\x84\xF6V[a\x02\x8Da\x03\xA76`\x04a\x82\xC2V[`,` R_\x90\x81R`@\x90 T\x81V[a\x02\x8Da\x03\xC66`\x04a\x82\xC2V[`-` R_\x90\x81R`@\x90 T\x81V[a\x03\xDFa\x16VV[`@Qa\x02q\x91\x90a\x85NV[a\x03/a\x03\xFA6`\x04a\x85`V[a\x17!V[a\x02\x8Da\x04\r6`\x04a\x80\xCCV[a\x1F\x9DV[a\x02\x8Da\x04 6`\x04a\x80\xE3V[a\x1F\xF7V[a\x02\x8Da\x0436`\x04a\x82\xC2V[`+` R_\x90\x81R`@\x90 T\x81V[a\x04La \x02V[`@Qa\x02q\x91\x90a\x85\x89V[a\x02\x8Da\x04g6`\x04a\x82\xC2V[`)` R_\x90\x81R`@\x90 T\x81V[`.Ta\x02\x8DV[a\x02\x8Da\x04\x8E6`\x04a\x80\xE3V[a \xF8V[a\x02\xA3a!\x03V[a\x04La!\x0FV[a\x03/a\x04\xB16`\x04a\x860V[a\"\x05V[a\x03\xDFa'\xCCV[a\x04\xC6a(\x97V[`@Q\x90\x15\x15\x81R` \x01a\x02qV[a\x02\xA3a)gV[a\x03/a\x04\xEC6`\x04a\x86\xF6V[a)\xC5V[`\x1FTa\x04\xC6\x90`\xFF\x16\x81V[_\x80_\x80`.\x85\x81T\x81\x10a\x05\x15Wa\x05\x15a\x87zV[\x90_R` _ \x90`\x03\x02\x01\x90P` `\x05\x01\x81_\x01T\x81T\x81\x10a\x05<Wa\x05<a\x87zV[_\x91\x82R` \x90\x91 \x01T`\x01\x82\x01T`%\x80T`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x96P\x91\x81\x10a\x05lWa\x05la\x87zV[_\x91\x82R` \x90\x91 \x01T`\x02\x91\x82\x01T\x94\x96`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x95P\x93\x90\x0B\x92\x91PPV[_a\x05\xA0\x82\x84a\x87\xD4V[\x90P[\x92\x91PPV[```\x16\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x05\xFFW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x05\xE1W[PPPPP\x90P\x90V[_\x81\x81R`3` R`@\x81 ``\x91\x90a\x06#\x90a4\x10V[\x93\x92PPPV[```\x1E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x07]W_\x84\x81R` \x80\x82 `@\x80Q\x80\x82\x01\x82R`\x02\x87\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x95\x91\x94\x86\x81\x01\x94\x91\x92\x90\x84\x01[\x82\x82\x10\x15a\x07FW\x83\x82\x90_R` _ \x01\x80Ta\x06\xBB\x90a\x87\xE7V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\xE7\x90a\x87\xE7V[\x80\x15a\x072W\x80`\x1F\x10a\x07\tWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x072V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07\x15W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x06\x9EV[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x06MV[PPPP\x90P\x90V[```\x18\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x05\xFFW` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x05\xE1WPPPPP\x90P\x90V[```\x17\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x05\xFFW` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x05\xE1WPPPPP\x90P\x90V[``a\x08.`(a4\xC9V[\x90P\x90V[\x83_a\x08?`'a5\x9DV[\x90Pa\x08L\x82_\x83a5\xEEV[\x91P\x80\x82\x10\x15a\x08\xA9Wa\x08a`'\x83a6;V[`5\x80T`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x83\x16\x81\x17\x90\x91U`6\x80T\x90\x92\x16\x17\x90Ua\nMV[`#T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90a\x08\xC4\x90a\x80\x8DV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x08\xEDW=_\x80>=_\xFD[P`6\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x92\x83\x17\x90\x91U`5\x80T\x90\x91\x16\x82\x17\x90U`@Q\x7Fi\0\xA3\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01\x84\x01`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\xC6W\xC7\x18\x91\x83\x90ci\0\xA3\xAE\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xA3W=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\t\xCA\x91\x90\x81\x01\x90a\x888V[`@Q` \x01a\t\xDA\x91\x90a\x88\xE3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\x06\x92\x91\x90a\x89\x14V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\n\x1DW_\x80\xFD[PZ\xF1\x15\x80\x15a\n/W=_\x80>=_\xFD[PP`5Ta\nK\x92P`'\x91P`\x01`\x01`\xA0\x1B\x03\x16a6\xADV[P[Pa\nj\x86_`\x01`.\x80T\x90Pa\ne\x91\x90a\x895V[a5\xEEV[\x95P_`.\x87\x81T\x81\x10a\n\x80Wa\n\x80a\x87zV[_\x91\x82R` \x82 `\x02`\x03\x90\x92\x02\x01\x81\x81\x01T\x90\x93P\x82\x91a\n\xA4\x91\x90\x0Ba8\x98V[\x91P\x91Pa\n\xE5\x83`\x02\x01_\x90T\x90a\x01\0\n\x90\x04`\x02\x0Ba\n\xD0\x89`\x02\x0B\x85`\x02\x0B\x85`\x02\x0Ba8\xF0V[`\x02\x0B\x90_\x81\x83\x07\x12\x91\x81\x90\x05\x91\x90\x91\x03\x02\x90V[\x96Pa\x0B\x0F\x83`\x02\x01_\x90T\x90a\x01\0\n\x90\x04`\x02\x0Ba\n\xD0\x88`\x02\x0B\x85`\x02\x0B\x85`\x02\x0Ba8\xF0V[`@Q\x7FLc\xE5b\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x02\x82\x81\x0B\x90\x8A\x90\x0B\x14\x15`\x04\x82\x01R\x90\x96Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90cLc\xE5b\x90`$\x01_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0B}W_\x80\xFD[PZ\xFA\x15\x80\x15a\x0B\x8FW=_\x80>=_\xFD[PPPP\x86`\x02\x0B\x86`\x02\x0B\x12\x15a\x0B\xA5W\x94\x95\x94[PP`$T\x81T`%\x80T_\x93a\x0C&\x93`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92\x91\x81\x10a\x0B\xD1Wa\x0B\xD1a\x87zV[_\x91\x82R` \x90\x91 \x01T`\x01\x85\x01T`%\x80T`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x90\x91\x90\x81\x10a\x0C\x02Wa\x0C\x02a\x87zV[_\x91\x82R` \x90\x91 \x01T`\x02\x80\x87\x01T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x0Ba9\xC7V[\x90P_a\x0C4\x82`\xA0\x90 \x90V[\x90P_\x80_a\x0CE\x84\x8B\x8B\x89a:$V[`@Q\x7FLc\xE5b\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x15\x15`\x04\x82\x01R\x92\x95P\x90\x93P\x91Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90cLc\xE5b\x90`$\x01_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0C\xC2W_\x80\xFD[PZ\xFA\x15\x80\x15a\x0C\xD4W=_\x80>=_\xFD[PPPPa\x0C\xF6\x88`\x01\x85o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a5\xEEV[`\x02\x80\x88\x01T`#T\x92\x9APa\r\x1A\x92`\x01`\x01`\xA0\x1B\x03\x16\x91\x87\x91\x8E\x91\x0Ba=JV[a\r5W_\x84\x81R`0` R`@\x90 a\r5\x90\x8Ba=\xA0V[`\x02\x80\x87\x01T`#Ta\rY\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91\x87\x91\x8D\x91\x90\x0Ba=JV[a\rtW_\x84\x81R`0` R`@\x90 a\rt\x90\x8Aa=\xA0V[\x85T`&\x80T_\x92\x90\x81\x10a\r\x8BWa\r\x8Ba\x87zV[_\x91\x82R` \x82 \x01T`\x01\x89\x01T`&\x80T`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x94P\x91\x81\x10a\r\xBAWa\r\xBAa\x87zV[_\x91\x82R` \x90\x91 \x01T`5T`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x87\x90R\x91\x81\x16\x92P\x83\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0E2W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0EV\x91\x90a\x89HV[P`5T`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x90\x82\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0E\xC0W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xE4\x91\x90a\x89HV[P`6T`\x02\x89\x81\x01T`@\x80Q`\xA0\x81\x01\x82R_\x91\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x81\x16` \x83\x01R\x91\x90\x92\x0B``\x83\x01R\x90\x91\x16\x90c\x0C\x86Xy\x90\x8E\x8E\x8E_\x80\x1B`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0FU\x95\x94\x93\x92\x91\x90a\x89gV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0FpW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x94\x91\x90a\x89\xEDV[PP\x87T`%\x80T_\x94P\x90\x92P\x81\x10a\x0F\xB0Wa\x0F\xB0a\x87zV[_\x91\x82R` \x82 \x01T`\x01\x89\x01T`%\x80T`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x94P\x91\x81\x10a\x0F\xDFWa\x0F\xDFa\x87zV[_\x91\x82R` \x90\x91 \x01T`5T`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x87\x90R\x91\x81\x16\x92P\x83\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x10WW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10{\x91\x90a\x89HV[P`5T`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x90\x82\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x10\xE5W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\t\x91\x90a\x89HV[P`6_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x0C\x86Xy\x88\x8E\x8E\x8E_\x80\x1B`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11T\x95\x94\x93\x92\x91\x90a\x89gV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x11oW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x93\x91\x90a\x89\xEDV[PP`6T_\x97Pa\x11\xB9\x96P\x8C\x95P\x8B\x94P`\x01`\x01`\xA0\x1B\x03\x16\x92Pa>\xBD\x91PPV[\x90P_\x88\x81R`4` R`@\x90 a\x11\xD2\x90\x82a?\x0CV[P_\x88\x81R`2` \x90\x81R`@\x80\x83 \x84\x84R\x82R\x80\x83 \x8B\x84R`3\x90\x92R\x90\x91 a\x12\0\x90\x83a?\x0CV[\x15a\x12\x7FW\x80Tb\xFF\xFF\xFF\x87\x81\x16c\x01\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\x90\x92\x16\x90\x89\x16\x17\x17\x80\x82U`6Tf\x01\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x17\x81U[\x84\x81`\x03\x01_\x82\x82Ta\x12\x92\x91\x90a\x87\xD4V[\x90\x91UPP`@\x80Q``\x81\x01\x82R\x95\x86R_\x99\x8AR`1` \x90\x81R\x81\x8B T\x81\x88\x01\x90\x81R_\x19\x92\x88\x01\x92\x83R`\x05\x90\x93\x01\x80T`\x01\x81\x81\x01\x83U\x91\x8DR\x91\x90\x9B \x96Q`\x03\x90\x91\x02\x90\x96\x01\x95\x86U\x90Q\x98\x85\x01\x98\x90\x98UPP\x94Q`\x02\x90\x91\x01UPPPPV[```\x1B\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x07]W\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01\x80Ta\x13O\x90a\x87\xE7V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13{\x90a\x87\xE7V[\x80\x15a\x13\xC6W\x80`\x1F\x10a\x13\x9DWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13\xC6V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13\xA9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x14]W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x14\nW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x13\x1FV[a\x14\xC5`@Q\x80a\x01\0\x01`@R\x80_`\x02\x0B\x81R` \x01_`\x02\x0B\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01``\x81RP\x90V[_\x83\x81R`2` \x90\x81R`@\x80\x83 \x85\x84R\x82R\x80\x83 \x81Qa\x01\0\x81\x01\x83R\x81T`\x02\x81\x81\x0B\x83Rc\x01\0\0\0\x82\x04\x81\x0B\x83\x87\x01Rf\x01\0\0\0\0\0\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x82\x85\x01R`\x01\x83\x01T``\x83\x01R\x82\x01T`\x80\x82\x01R`\x03\x82\x01T`\xA0\x82\x01R`\x04\x82\x01T`\xC0\x82\x01R`\x05\x82\x01\x80T\x84Q\x81\x87\x02\x81\x01\x87\x01\x90\x95R\x80\x85R\x91\x95\x92\x94`\xE0\x87\x01\x94\x93\x91\x92\x91\x90\x84\x01[\x82\x82\x10\x15a\x15\xADW\x83\x82\x90_R` _ \x90`\x03\x02\x01`@Q\x80``\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x15`V[PPP\x91RP\x90\x94\x93PPPPV[_a\x05\xA0\x82\x84a\x8A\x0FV[```1_\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x16KW_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R\x90\x84\x01T`\x02\x81\x90\x0B\x82Rc\x01\0\0\0\x90\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x83\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x15\xF9V[PPPP\x90P\x91\x90PV[```\x1A\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x07]W\x83\x82\x90_R` _ \x01\x80Ta\x16\x96\x90a\x87\xE7V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x16\xC2\x90a\x87\xE7V[\x80\x15a\x17\rW\x80`\x1F\x10a\x16\xE4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x17\rV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x16\xF0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x16yV[a\x178\x83_`\x01`.\x80T\x90Pa\ne\x91\x90a\x895V[_\x81\x81R`4` R`@\x81 \x91\x94P\x90a\x17R\x90a@\"V[`@Q\x7FLc\xE5b\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x15\x15`\x04\x82\x01R\x90\x91Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90cLc\xE5b\x90`$\x01_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x17\xB8W_\x80\xFD[PZ\xFA\x15\x80\x15a\x17\xCAW=_\x80>=_\xFD[PPPP_a\x17\xF7a\x17\xE4\x85_`\x01\x86a\ne\x91\x90a\x895V[_\x87\x81R`4` R`@\x90 \x90a@pV[_\x86\x81R`2` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x81 `\x03\x81\x01T\x92\x93P\x91a\x18#\x91\x86\x91a5\xEEV[\x93P_`.\x87\x81T\x81\x10a\x189Wa\x189a\x87zV[_\x91\x82R` \x82 `$T`\x03\x90\x92\x02\x01\x80T`%\x80T\x92\x95Pa\x18r\x93`\x01`\x01`\xA0\x1B\x03\x16\x92\x90\x91\x90\x81\x10a\x0B\xD1Wa\x0B\xD1a\x87zV[`$T\x90\x91P_\x90`\x01`\x01`\xA0\x1B\x03\x16c\xD8\x94\x11Da\x18\x93\x84`\xA0\x90 \x90V[\x86T`@Q`\xE0\x84\x90\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R`\x04\x81\x01\x92\x90\x92Rf\x01\0\0\0\0\0\0\x81\x04`\x01`\x01`\xA0\x1B\x03\x16`$\x83\x01R`\x02\x81\x81\x0B`D\x84\x01Rc\x01\0\0\0\x90\x91\x04\x90\x0B`d\x82\x01R_`\x84\x82\x01R`\xA4\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\x1EW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19B\x91\x90a\x8A&V[\x90P\x80`+_` `\x05\x01\x86_\x01T\x81T\x81\x10a\x19aWa\x19aa\x87zV[_\x91\x82R` \x80\x83 \x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16\x83R\x82\x01\x92\x90\x92R`@\x01\x81 \x80T\x90\x91\x90a\x19\x94\x90\x84\x90a\x87\xD4V[\x92PP\x81\x90UP\x80\x84`\x02\x01_\x82\x82Ta\x19\xAE\x91\x90a\x87\xD4V[\x90\x91UPP\x83T`\x01`\x01`\xA0\x1B\x03f\x01\0\0\0\0\0\0\x82\x04\x16\x90c\x0C\x86Xy\x90\x84\x90`\x02\x81\x81\x0B\x91c\x01\0\0\0\x90\x04\x90\x0Ba\x19\xE9\x8Ca\x8A=V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x87\x90\x1B\x16\x81Ra\x1A'\x94\x93\x92\x91\x90_\x90`\x04\x01a\x89gV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1ABW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Af\x91\x90a\x89\xEDV[PP\x83T\x83T`%\x80Tf\x01\0\0\0\0\0\0\x90\x93\x04`\x01`\x01`\xA0\x1B\x03\x16\x92c\xBA\xCA\0\x04\x92\x90\x81\x10a\x1A\x9AWa\x1A\x9Aa\x87zV[_\x91\x82R` \x90\x91 \x01T`@Q`\xE0\x83\x90\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1A\xFBW_\x80\xFD[PZ\xF1\x15\x80\x15a\x1B\rW=_\x80>=_\xFD[PP\x85T`\x01\x86\x01T`%\x80Tf\x01\0\0\0\0\0\0\x90\x93\x04`\x01`\x01`\xA0\x1B\x03\x16\x94Pc\xBA\xCA\0\x04\x93P\x91\x81\x10a\x1BFWa\x1BFa\x87zV[_\x91\x82R` \x90\x91 \x01T`@Q`\xE0\x83\x90\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1B\xA7W_\x80\xFD[PZ\xF1\x15\x80\x15a\x1B\xB9W=_\x80>=_\xFD[PP\x84T`&\x80T_\x94P\x90\x92P\x81\x10a\x1B\xD5Wa\x1B\xD5a\x87zV[_\x91\x82R` \x82 \x01T`\x01\x86\x01T`&\x80T`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x94P\x91\x81\x10a\x1C\x04Wa\x1C\x04a\x87zV[_\x91\x82R` \x80\x83 \x90\x91\x01T\x88T`\x02\x89\x81\x01T`@\x80Q`\xA0\x81\x01\x82R\x90\x81\x01\x87\x90R`\x80\x81\x01\x96\x90\x96R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16\x87R\x93\x84\x16\x94\x86\x01\x85\x90R\x90\x0B``\x85\x01R\x91\x93Pf\x01\0\0\0\0\0\0\x90\x91\x04\x16\x90c\x0C\x86Xy\x90\x88T`\x02\x81\x81\x0B\x91c\x01\0\0\0\x90\x04\x90\x0Ba\x1C~\x8Ea\x8A=V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x87\x90\x1B\x16\x81Ra\x1C\xBC\x94\x93\x92\x91\x90_\x90`\x04\x01a\x89gV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1C\xD7W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xFB\x91\x90a\x89\xEDV[PP\x85T`@Q\x7F\xBA\xCA\0\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01Rf\x01\0\0\0\0\0\0\x90\x92\x04\x90\x91\x16\x90c\xBA\xCA\0\x04\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1DdW_\x80\xFD[PZ\xF1\x15\x80\x15a\x1DvW=_\x80>=_\xFD[PP\x87T`@Q\x7F\xBA\xCA\0\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01Rf\x01\0\0\0\0\0\0\x90\x92\x04\x90\x91\x16\x92Pc\xBA\xCA\0\x04\x91P`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1D\xE1W_\x80\xFD[PZ\xF1\x15\x80\x15a\x1D\xF3W=_\x80>=_\xFD[PPPPPP_a\x1E\x05\x83`\xA0\x90 \x90V[\x85T`\x02\x86\x81\x01T`#T\x93\x94Pa\x1E.\x93`\x01`\x01`\xA0\x1B\x03\x16\x92\x85\x92\x90\x81\x0B\x91\x90\x0Ba=JV[a\x1ENW_\x81\x81R`0` R`@\x90 \x85Ta\x1EN\x91\x90`\x02\x0Ba@\xD2V[\x84T`\x02\x85\x81\x01T`#Ta\x1E}\x93`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92\x85\x92c\x01\0\0\0\x90\x92\x04\x81\x0B\x91\x90\x0Ba=JV[a\x1E\xA4W_\x81\x81R`0` R`@\x90 \x85Ta\x1E\xA4\x91\x90c\x01\0\0\0\x90\x04`\x02\x0Ba@\xD2V[PPPP\x83\x81`\x03\x01_\x82\x82Ta\x1E\xBB\x91\x90a\x895V[\x90\x91UPP_\x86\x81R`1` R`@\x90 T`\x04\x82\x01T`\x05\x83\x01\x91\x90[\x82T\x81\x10\x15a\x1F\x11W\x81\x83\x82\x81T\x81\x10a\x1E\xF6Wa\x1E\xF6a\x87zV[_\x91\x82R` \x90\x91 `\x02`\x03\x90\x92\x02\x01\x01U`\x01\x01a\x1E\xDAV[P\x81T`\x04\x84\x01U`\x03\x83\x01T\x15a\x1FzW`@\x80Q``\x81\x01\x82R`\x03\x80\x86\x01T\x82R` \x80\x83\x01\x85\x81R_\x19\x94\x84\x01\x94\x85R`\x05\x88\x01\x80T`\x01\x81\x81\x01\x83U_\x92\x83R\x93\x90\x91 \x94Q\x93\x02\x90\x93\x01\x91\x82U\x91Q\x91\x81\x01\x91\x90\x91U\x90Q`\x02\x90\x91\x01Ua\x1F\x93V[_\x88\x81R`4` R`@\x90 a\x1F\x91\x90\x85aB\x04V[P[PPPPPPPPV[_\x80`.\x83\x81T\x81\x10a\x1F\xB2Wa\x1F\xB2a\x87zV[_\x91\x82R` \x90\x91 `$T`\x03\x90\x92\x02\x01\x80T`%\x80T\x92\x94Pa\x06#\x93a\x1F\xF0\x93`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92\x90\x81\x10a\x0B\xD1Wa\x0B\xD1a\x87zV[`\xA0\x90 \x90V[_a\x05\xA0\x82\x84a\x895V[```\x1D\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x07]W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a \xE0W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a \x8DW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a %V[_a\x05\xA0\x82\x84a\x8A\xA0V[``a\x08.`'a4\xC9V[```\x1C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x07]W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a!\xEDW` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a!\x9AW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a!2V[`%Ta\"\x1C\x90\x87\x90_\x90a\ne\x90`\x01\x90a\x895V[`%T\x90\x96Pa\"6\x90\x86\x90_\x90a\ne\x90`\x01\x90a\x895V[\x94Pa\"A\x83aC\nV[\x92Pa\"L\x82aC\nV[\x91Pa\"{`\x01`\x01`\xA0\x1B\x03\x82\x16d\x01\0\x02v\xA3s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a5\xEEV[\x90P\x84\x86\x03a\"\x9CW`%T`\x01\x87\x01\x81a\"\x98Wa\"\x98a\x8AsV[\x06\x94P[\x84\x86\x11\x15a\"\xA8W\x93\x94\x93[a\"\xBA`\x02\x85\x90\x0B`\x01a\x7F\xFFa8\xF0V[_\x87\x81R`/` \x90\x81R`@\x80\x83 \x89\x84R\x90\x91R\x90 T\x90\x94Pa\"\xE2\x90`\xFF\x16aC\x18V[_\x86\x81R`/` \x90\x81R`@\x80\x83 \x88\x84R\x82R\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U`$T\x81Q\x7F\x9Di\xDCN\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x91Qa#\xB4\x93`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92c\x9Di\xDCN\x92`\x04\x82\x81\x01\x93\x91\x92\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a#\x82W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\xA6\x91\x90a\x8A\xB3V[`\x01`\x01`\xA0\x1B\x03\x16aC\x94V[\x90P_` `\x05\x01\x88\x81T\x81\x10a#\xCDWa#\xCDa\x87zV[_\x91\x82R` \x82 \x01T`&\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x93P\x90\x8A\x90\x81\x10a#\xF9Wa#\xF9a\x87zV[_\x91\x82R` \x82 \x01T`%\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x93P\x90\x8A\x90\x81\x10a$%Wa$%a\x87zV[_\x91\x82R` \x82 \x01T`&\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x93P\x90\x8B\x90\x81\x10a$QWa$Qa\x87zV[_\x91\x82R` \x90\x91 \x01T`!T`@Q\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x16\x91Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a$\xCFW_\x80\xFD[PZ\xF1\x15\x80\x15a$\xE1W=_\x80>=_\xFD[PP`$\x80T`@Q\x7F\x13\x87\x14e\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16`\x04\x83\x01R\x87\x81\x16\x93\x82\x01\x93\x90\x93Ra\xFF\xFF\x8E\x16`D\x82\x01Rb\xFF\xFF\xFF\x80\x8E\x16`d\x83\x01R\x8C\x16`\x84\x82\x01R_`\xA4\x82\x01R\x91\x16\x92Pc\x13\x87\x14e\x91P`\xC4\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a%oW_\x80\xFD[PZ\xF1\x15\x80\x15a%\x81W=_\x80>=_\xFD[PPPPa%\x99\x84`(a6\xAD\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[Pa%\xA5`(\x83a6\xADV[P`$\x80T`@Q\x7F\x85\x87\xF4P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x85\x81\x16\x93\x82\x01\x93\x90\x93R`D\x81\x01\x88\x90R\x88\x83\x16`d\x82\x01R\x91\x16\x90c\x85\x87\xF4P\x90`\x84\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a&\x1CW_\x80\xFD[PZ\xF1\x15\x80\x15a&.W=_\x80>=_\xFD[PP`#T`@\x80Q`\xA0\x81\x01\x82R_\x81\x83\x01\x81\x90R`\x80\x82\x01R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16\x82R\x86\x81\x16` \x83\x01R`\x02\x8F\x90\x0B``\x83\x01R\x91Q\x7Fbv\xCB\xBE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x91\x90\x92\x16\x93Pcbv\xCB\xBE\x92Pa&\xA9\x91\x90\x8A\x90`\x04\x01a\x8A\xCEV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a&\xC5W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\xE9\x91\x90a\x8B@V[PP`@\x80Q``\x81\x01\x82R\x9A\x8BR` \x8B\x01\x99\x8AR`\x02\x98\x90\x98\x0B\x97\x8A\x01\x97\x88RPP`.\x80T`\x01\x81\x01\x82U_\x91\x90\x91R\x97Q\x7F7\xFA\x16l\xBD\xBF\xBB\x15a\xCC\xD9\xEA\x98^\xC0!\x8B^hP.#\x05%\xF5D([+\xDF=~`\x03\x90\x99\x02\x98\x89\x01UPP\x93Q\x7F7\xFA\x16l\xBD\xBF\xBB\x15a\xCC\xD9\xEA\x98^\xC0!\x8B^hP.#\x05%\xF5D([+\xDF=\x7F\x86\x01UPPQ\x7F7\xFA\x16l\xBD\xBF\xBB\x15a\xCC\xD9\xEA\x98^\xC0!\x8B^hP.#\x05%\xF5D([+\xDF=\x80\x90\x92\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\x16b\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x17\x90\x91UPV[```\x19\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x07]W\x83\x82\x90_R` _ \x01\x80Ta(\x0C\x90a\x87\xE7V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta(8\x90a\x87\xE7V[\x80\x15a(\x83W\x80`\x1F\x10a(ZWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a(\x83V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a(fW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a'\xEFV[`\x08T_\x90`\xFF\x16\x15a(\xAEWP`\x08T`\xFF\x16\x90V[`@Q\x7Ff\x7F\x9Dp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\x04\x82\x01\x81\x90R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x83\x01R_\x91cf\x7F\x9Dp\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)<W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)`\x91\x90a\x8A&V[\x14\x15\x90P\x90V[```\x15\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x05\xFFW` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x05\xE1WPPPPP\x90P\x90V[a)\xDC\x83_`\x01`.\x80T\x90Pa\ne\x91\x90a\x895V[\x92P\x82`8\x81\x90UP_`.\x84\x81T\x81\x10a)\xF9Wa)\xF9a\x87zV[_\x91\x82R` \x82 `$T`\x03\x90\x92\x02\x01\x80T`%\x80T\x92\x95Pa*\x8B\x93a\x1F\xF0\x93`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92\x90\x81\x10a*6Wa*6a\x87zV[_\x91\x82R` \x90\x91 \x01T`\x01\x86\x01T`%\x80T`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x90\x91\x90\x81\x10a*gWa*ga\x87zV[_\x91\x82R` \x90\x91 \x01T`\x02\x80\x88\x01T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x0Ba9\xC7V[`\x02\x80\x84\x01T\x91\x92P_\x91a*\xA3\x91\x84\x91\x90\x0BaC\xAAV[\x80Q\x90\x91Pa*\xB3\x86_\x83a5\xEEV[\x95Pa*\xBDa\x80\x9AV[a*\xD4\x82a*\xCC`\x04\x82a\x8A\xA0V[\x83\x91\x90aErV[_\x87g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\xEEWa*\xEEa\x86\x98V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a+2W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a+\x0CW\x90P[P\x90P_\x80[\x89\x81\x10\x15a.\xBDW_\x86a+L\x8B\x87aE\x8BV[\x81Q\x81\x10a+\\Wa+\\a\x87zV[` \x02` \x01\x01Q\x90P_a+\xB8g\x01cEx]\x8A\0\0a+\x8Eg\r\xE0\xB6\xB3\xA7d\0\0\x8EaE\xA5\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x11\x15a+\xB2Wa+\xAD\x8C`\x01l\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFaF\nV[aG\x07V[_aG\x07V[\x90P`@Q\x80`@\x01`@R\x80\x83`\x02\x0B\x81R` \x01\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x85\x84\x81Q\x81\x10a+\xF6Wa+\xF6a\x87zV[` \x90\x81\x02\x91\x90\x91\x01\x01Ra,\x1Do\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x85a\x87\xD4V[\x93P_\x8D\x81R`1` R`@\x90 \x85Q\x86\x90\x85\x90\x81\x10a,@Wa,@a\x87zV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82T`\x01\x81\x01\x84U_\x93\x84R\x82\x84 \x82Q\x91\x01\x80T\x92\x84\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x01\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x93\x16b\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x91\x90\x91\x17\x90U`7\x82\x90U`8T\x82R`4\x90R`@\x81 a,\xCD\x90a@\"V[\x90P_\x80[\x82\x81\x10\x15a-\x8FW`8T_\x90\x81R`4` R`@\x81 a,\xF4\x90\x83a@pV[`8T_\x90\x81R`2` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 \x80T\x91\x92P\x90`\x02\x88\x81\x0B\x91\x90\x0B\x13\x80\x15\x90a-:WP\x80Tc\x01\0\0\0\x90\x04`\x02\x90\x81\x0B\x90\x88\x90\x0B\x12[\x15a-\x85W`\x03\x81\x01Ta-N\x90\x85a\x87\xD4V[`7\x80T`\x01\x81\x01\x82U_\x91\x90\x91R\x7FB\xA7\xB7\xDDx\\\xD6\x97\x14\xA1\x89\xDF\xFB?\xD7\xD7\x17N\xDC\x9E\xCE\x83v\x94\xCEP\xF7\x07\x8F|1\xAE\x01\x83\x90U\x93P[PP`\x01\x01a,\xD2V[P\x80_\x03a-\xFFW\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`,_` `\x05\x01\x8F_\x01T\x81T\x81\x10a-\xC6Wa-\xC6a\x87zV[_\x91\x82R` \x80\x83 \x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16\x83R\x82\x01\x92\x90\x92R`@\x01\x81 \x80T\x90\x91\x90a-\xF9\x90\x84\x90a\x87\xD4V[\x90\x91UPP[`7T\x91P_[\x82\x81\x10\x15a.\xACW_`7\x82\x81T\x81\x10a.\"Wa.\"a\x87zV[_\x91\x82R` \x80\x83 \x90\x91\x01T`8T\x83R`2\x82R`@\x80\x84 \x82\x85R\x90\x92R\x91 `\x03\x81\x01T\x91\x92P\x90a.\x87\x90\x85a.\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x16p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x8A\x0FV[\x91\x90aG^V[\x81`\x01\x01_\x82\x82Ta.\x99\x91\x90a\x87\xD4V[\x90\x91UPP`\x01\x90\x92\x01\x91Pa.\x06\x90PV[PP`\x01\x90\x93\x01\x92Pa+8\x91PPV[P`#T`\x02\x80\x89\x01T_\x92a.\xE3\x92\x86\x92`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x8B\x91\x0BaG\xECV[\x90P_` `\x05\x01\x89_\x01T\x81T\x81\x10a.\xFFWa.\xFFa\x87zV[_\x91\x82R` \x82 \x01T`\x01\x8B\x01T`%\x80T`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x94P\x91\x81\x10a/.Wa/.a\x87zV[_\x91\x82R` \x80\x83 \x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x84R`*\x90\x92R`@\x83 \x80T\x92\x90\x91\x16\x93P\x86\x92\x90\x91a/h\x90\x84\x90a\x87\xD4V[\x90\x91UPP\x89T`%\x80T_\x92\x90\x81\x10a/\x84Wa/\x84a\x87zV[_\x91\x82R` \x90\x91 \x01T`@Q\x7F@\xC1\x0F\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R`$\x81\x01\x87\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91P\x81\x90c@\xC1\x0F\x19\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a/\xF1W_\x80\xFD[PZ\xF1\x15\x80\x15a0\x03W=_\x80>=_\xFD[PP`$\x80T`@Q\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R_\x19\x92\x81\x01\x92\x90\x92R\x84\x16\x92Pc\t^\xA7\xB3\x91P`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a0mW_\x80\xFD[PZ\xF1\x15\x80\x15a0\x7FW=_\x80>=_\xFD[PP`$\x80T`@Q\x7FG\xE7\xEF$\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x92\x81\x01\x8A\x90R\x91\x16\x92PcG\xE7\xEF$\x91P`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a0\xE6W_\x80\xFD[PZ\xF1\x15\x80\x15a0\xF8W=_\x80>=_\xFD[PP\x85Q\x91PP\x80\x15a3\xF7W`@\x80Q`\x02\x80\x82R``\x82\x01\x90\x92R_\x91\x81` \x01[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81a1\x1CW\x90PP\x90P\x84\x81_\x81Q\x81\x10a1cWa1ca\x87zV[` \x02` \x01\x01Q_\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x83\x81`\x01\x81Q\x81\x10a1\x9AWa1\x9Aa\x87zV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90R`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91\x81` \x01[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81a1\xC9W\x90PP\x90P\x85\x81_\x81Q\x81\x10a2\tWa2\ta\x87zV[` \x02` \x01\x01Q_\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x84\x81_\x81Q\x81\x10a2?Wa2?a\x87zV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x01Ra2gV[`\xA0\x1C`\x02\x0B\x90V[_[\x83\x81\x10\x15a3\xF3W_`@Q\x80`\x80\x01`@R\x80\x89`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x88`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x8A\x84\x81Q\x81\x10a2\xC5Wa2\xC5a\x87zV[` \x90\x81\x02\x91\x90\x91\x01\x01Q\x90R`$T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16c\x97\xDC\x99ea2\xEF\x86aI\x01V[a3r\x87` `\x04\x01_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x9Di\xDCN`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3FW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3j\x91\x90a\x8A\xB3V[\x88\x91\x90aI\x92V[a3|\x85\x88aJ1V[`@Q` \x01a3\x8E\x93\x92\x91\x90a\x8B[V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a3\xB9\x91\x90a\x8BxV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a3\xD0W_\x80\xFD[PZ\xF1\x15\x80\x15a3\xE2W=_\x80>=_\xFD[PP`\x01\x90\x93\x01\x92Pa2i\x91PPV[PPP[a3\xFFaJ\xC6V[PPPPPPPPPPPPPPPV[c\x18\xFBXd`\x04R_\x81\x81R`$\x90 \x80\x19T`@Q\x91\x90h\xFB\xB6\x7F\xDAR\xD4\xBF\xB8\xBF\x90` \x84\x01\x81a4\x89W\x83T\x80\x15a4\x83W\x80\x84\x14\x15\x02\x81R`\x01\x84\x81\x01T\x90\x92P\x80\x15a4\x83W\x80\x84\x14\x15\x02` \x82\x01R`\x02\x84\x81\x01T\x90\x92P\x80\x15a4\x83W`\x03\x92P\x83\x81\x14\x15\x81\x02`@\x83\x01R[Pa4\xB4V[\x81`\x01\x1C\x91P_[\x82\x81\x10\x15a4\xB2W\x84\x81\x01T\x84\x81\x14\x15\x02`\x05\x82\x90\x1B\x83\x01R`\x01\x01a4\x91V[P[\x81\x85R\x81`\x05\x1B\x81\x01`@RPPPP\x91\x90PV[c\x97\x8A\xAB\x92`\x04R_\x81\x81R`$\x81 ``\x91Ph\xFB\xB6\x7F\xDAR\xD4\xBF\xB8\xBF\x81T\x80`\xA0\x1B`\xA0\x1C`@Q\x94P\x84` \x01\x82``\x1C\x92P\x83\x83\x14\x15\x83\x02\x81R\x81a5WW\x82\x15a5RW`\x01\x91P\x81\x85\x01T``\x1C\x92P\x82\x15a5RW\x82\x84\x14\x15\x90\x92\x02` \x83\x01RP`\x02\x83\x81\x01T``\x1C\x91\x82\x15a5RW`\x03\x91P\x83\x83\x14\x15\x83\x02`@\x82\x01R[a5\x87V[`\x01\x91\x82\x1C\x91[\x82\x81\x10\x15a5\x85W\x85\x81\x01T``\x1C\x85\x81\x14\x15\x81\x02`\x05\x83\x90\x1B\x84\x01R\x93P`\x01\x01a5^V[P[\x81\x86R\x81`\x05\x1B\x81\x01`@RPPPPP\x91\x90PV[c\x97\x8A\xAB\x92`\x04R_\x81\x81R`$\x81 \x80T\x80`\xA0\x1B`\xA0\x1C\x80`\x01\x1C\x93P\x80\x82``\x1C\x15\x17a5\xE6W`\x01\x93P\x83\x83\x01T\x15a5\xE6W`\x02\x93P\x83\x83\x01T\x15a5\xE6W`\x03\x93P[PPP\x91\x90PV[_a5\xFA\x84\x84\x84aK\xA4V[\x90Pa\x06#`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01\x7FBound result\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x82aM]V[c\x97\x8A\xAB\x92`\x04R_\x82\x81R`$\x81 \x82\x81\x01T``\x1C\x91Ph\xFB\xB6\x7F\xDAR\xD4\xBF\xB8\xBF\x82\x14\x15\x82\x02\x91Pa6n\x84a5\x9DV[\x83\x10a6\xA6W`@Q\x7FN#\xD05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x92\x91PPV[c\x97\x8A\xAB\x92`\x04R_\x82\x81R`$\x81 `\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04I\x80%\xAD+@GA\x83\x01a6\xFEWc\xF5\xA2g\xF1_R`\x04`\x1C\xFD[\x82a7\x10Wh\xFB\xB6\x7F\xDAR\xD4\xBF\xB8\xBF\x92P[\x80Tk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x82` R\x80a7\xD7W\x81``\x1C\x80a7CW\x85``\x1B\x84U`\x01\x94PPa8\x1BV[\x85\x81\x03a7PWPa8\x1BV[`\x01\x84\x01T``\x1C\x80a7qW\x86``\x1B`\x01\x86\x01U`\x01\x95PPPa8\x1BV[\x86\x81\x03a7\x7FWPPa8\x1BV[`\x02\x85\x01T``\x1C\x80a7\xA1W\x87``\x1B`\x02\x87\x01U`\x01\x96PPPPa8\x1BV[\x87\x81\x03a7\xB0WPPPa8\x1BV[_\x92\x83R`@\x80\x84 `\x01\x90U\x91\x83R\x81\x83 `\x02\x90U\x82R\x90 `\x03\x90UP`\x07\x90\x81\x17\x90[\x84_R`@_ \x80Ta8\x19W`\x01\x91\x82\x1C\x80\x83\x01\x82U\x91\x94P\x81a8\x05W\x85``\x1B`\x03\x17\x84UPa8\x1BV[\x85``\x1B\x82\x85\x01U\x82`\x02\x01\x84UPa8\x1BV[P[PPP\x92\x91PPV[a8\x95\x81`@Q`$\x01a88\x91\x90a\x8BxV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FA0O\xAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90RaM\xD0V[PV[_\x80\x82a8\xC5\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF2v\x18a\x8B\x8AV[a8\xCF\x91\x90a\x8B\xDEV[\x91P\x82a8\xDF\x81b\r\x89\xE8a\x8B\x8AV[a8\xE9\x91\x90a\x8B\xDEV[\x90P\x91P\x91V[_a8\xFC\x84\x84\x84aM\xE0V[`@\x80Q\x80\x82\x01\x82R`\x0C\x81R\x7FBound result\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90Q\x7F\xA3\"\xC4\x0E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90R\x91\x92Pa\x06#\x91sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xA3\"\xC4\x0E\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a9\x9BW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra9\xC2\x91\x90\x81\x01\x90a\x888V[aPAV[`@\x80Q`\xA0\x81\x01\x82R_\x91\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x80\x83\x01\x81\x90R\x85\x82\x16\x83R\x90\x84\x16` \x83\x01R`\x02\x83\x90\x0B``\x83\x01R\x15a:\x10Wb\x80\0\0a:\x12V[_[b\xFF\xFF\xFF\x16`@\x82\x01R\x94\x93PPPPV[`\x02\x81\x81\x01T`#T_\x92\x83\x92\x83\x92\x91\x0B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF2v\x18\x81\x81\x07\x84\x13\x90\x82\x90\x05\x03b\r\x89\xE8\x91\x90\x91\x05\x03`\x01\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x90\x82\x90a:\x95\x90`\x01`\x01`\xA0\x1B\x03\x16\x8A\x8AaP\xB4V[P`#T\x90\x91P_\x90a:\xB2\x90`\x01`\x01`\xA0\x1B\x03\x16\x8B\x8AaP\xB4V[P\x90Pa:\xE3\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aQ\x19V[a:\xED\x90\x84a\x8B\xFDV[`#T\x90\x96P_\x93Pa;\x0C\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90P\x89aH\x19V[\x90P_\x80a;\x1D\x83`\xA0\x1C`\x02\x0B\x90V[\x91PP`\x01`\x01`\xA0\x1B\x03\x82\x16a;7\x82\x82\x8B\x8B\x8AaQ.V[\x88T`%\x80T\x93\x98P\x91\x96P_\x92\x81\x10a;SWa;Sa\x87zV[_\x91\x82R` \x90\x91 \x01T`@Q\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a;\xBAW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;\xDE\x91\x90a\x8A&V[\x90P_` `\x05\x01\x89`\x01\x01T\x81T\x81\x10a;\xFBWa;\xFBa\x87zV[_\x91\x82R` \x90\x91 \x01T`@Q\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a<bW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<\x86\x91\x90a\x8A&V[\x90Pa=&a+\xADa=\x0E\x89\x85\x10a<\xB0W\x8Ao\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a<\xCDV[a<\xCDo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8C\x16\x86\x8CaG^V[\x89\x85\x10a<\xECW\x8Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aQ\xC5V[a=\to\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8D\x16\x86\x8CaG^V[aQ\xC5V[\x8Ao\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aQ\xC5V[\x97Pa=5\x84\x84\x8D\x8D\x8CaQ.V[\x98\x9D\x90\x9CP\x97\x9AP\x96\x98PPPPPPPPPV[_\x80\x80a=h\x84\x86\x07\x82\x13\x85\x87\x05\x03[`\x08\x81\x90\x1D\x91`\xFF\x90\x91\x16\x90V[\x90\x92P\x90Pa=\x95\x81a=\x85`\x01`\x01`\xA0\x1B\x03\x8A\x16\x89\x86aQ\xD3V[\x90`\x01`\xFF\x91\x90\x91\x16\x1B\x16\x15\x15\x90V[\x97\x96PPPPPPPV[\x81T_[\x81\x81\x10\x15a=\xF3W\x83\x81\x81T\x81\x10a=\xBEWa=\xBEa\x87zV[\x90_R` _ \x90`\n\x91\x82\x82\x04\x01\x91\x90\x06`\x03\x02\x90T\x90a\x01\0\n\x90\x04`\x02\x0B`\x02\x0B\x83`\x02\x0B\x12a=\xF3W`\x01\x01a=\xA4V[\x81\x81\x10\x15a>\x80W\x82\x84\x82\x81T\x81\x10a>\x0EWa>\x0Ea\x87zV[\x90_R` _ \x90`\n\x91\x82\x82\x04\x01\x91\x90\x06`\x03\x02\x90T\x90a\x01\0\n\x90\x04`\x02\x0B\x85\x83\x81T\x81\x10a>AWa>Aa\x87zV[_\x91\x82R` \x90\x91 `\n\x80\x83\x04\x90\x91\x01\x80T\x91\x90\x92\x06`\x03\x02a\x01\0\nb\xFF\xFF\xFF\x81\x81\x02\x19\x90\x92\x16\x94\x90\x91\x16\x02\x92\x90\x92\x17\x90\x91U\x92P`\x01\x01a=\xF3V[PP\x81T`\x01\x81\x01\x83U_\x92\x83R` \x90\x92 `\n\x80\x84\x04\x90\x91\x01\x80T\x91\x90\x93\x06`\x03\x02a\x01\0\nb\xFF\xFF\xFF\x81\x81\x02\x19\x90\x92\x16\x92\x90\x91\x16\x02\x17\x90UV[`@\x80Q`\x02\x85\x81\x0B` \x83\x01R\x84\x90\x0B\x91\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x82\x16``\x82\x01R_\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x93\x92PPPV[PPV[c\x18\xFBXd`\x04R_\x82\x81R`$\x81 h\xFB\xB6\x7F\xDAR\xD4\xBF\xB8\xBF\x83\x03a?9Wc\xF5\xA2g\xF1_R`\x04`\x1C\xFD[\x82a?KWh\xFB\xB6\x7F\xDAR\xD4\xBF\xB8\xBF\x92P[\x80\x19T\x81` R\x80a?\xEFW\x81T\x80a?kW\x84\x83U`\x01\x93PPa@\x1AV[\x84\x81\x03a?xWPa@\x1AV[`\x01\x83\x01T\x80a?\x93W\x85`\x01\x85\x01U`\x01\x94PPPa@\x1AV[\x85\x81\x03a?\xA1WPPa@\x1AV[`\x02\x84\x01T\x80a?\xBDW\x86`\x02\x86\x01U`\x01\x95PPPPa@\x1AV[\x86\x81\x03a?\xCCWPPPa@\x1AV[_\x92\x83R`@\x80\x84 `\x01\x90U\x91\x83R\x81\x83 `\x02\x90U\x82R\x90 `\x03\x90UP`\x07[\x83_R`@_ \x80Ta8\x1BW`\x01\x91\x82\x1C\x83\x81\x01\x86\x90U\x80\x83\x01\x91\x82\x90U\x90\x82\x1B\x82\x17\x83\x19U\x90\x92P[PP\x92\x91PPV[c\x18\xFBXd`\x04R_\x81\x81R`$\x81 \x80\x19T\x80`\x01\x1C\x92P\x80a@iW\x81T_\x93P\x15a@iW`\x01\x92P\x82\x82\x01T\x15a@iW`\x02\x92P\x82\x82\x01T\x15a@iW`\x03\x92P[PP\x91\x90PV[c\x18\xFBXd`\x04R_\x82\x81R`$\x90 \x81\x01Th\xFB\xB6\x7F\xDAR\xD4\xBF\xB8\xBF\x81\x14\x15\x02a@\x9A\x83a@\"V[\x82\x10a\x05\xA3W`@Q\x7FN#\xD05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81T_[\x81\x81\x10\x15aA!W\x82`\x02\x0B\x84\x82\x81T\x81\x10a@\xF4Wa@\xF4a\x87zV[_\x91\x82R` \x90\x91 `\n\x80\x83\x04\x90\x91\x01T\x91\x06`\x03\x02a\x01\0\n\x90\x04`\x02\x0B\x14aA!W`\x01\x01a@\xD6V[aA,`\x01\x83a\x895V[\x81\x10\x15aA\xC1W\x83aA?\x82`\x01a\x87\xD4V[\x81T\x81\x10aAOWaAOa\x87zV[\x90_R` _ \x90`\n\x91\x82\x82\x04\x01\x91\x90\x06`\x03\x02\x90T\x90a\x01\0\n\x90\x04`\x02\x0B\x84\x82\x81T\x81\x10aA\x82WaA\x82a\x87zV[\x90_R` _ \x90`\n\x91\x82\x82\x04\x01\x91\x90\x06`\x03\x02a\x01\0\n\x81T\x81b\xFF\xFF\xFF\x02\x19\x16\x90\x83`\x02\x0Bb\xFF\xFF\xFF\x16\x02\x17\x90UP\x80\x80`\x01\x01\x91PPaA!V[\x83\x80T\x80aA\xD1WaA\xD1a\x8CjV[`\x01\x90\x03\x81\x81\x90_R` _ \x90`\n\x91\x82\x82\x04\x01\x91\x90\x06`\x03\x02a\x01\0\n\x81T\x90b\xFF\xFF\xFF\x02\x19\x16\x90U\x90UPPPPV[c\x18\xFBXd`\x04R_\x82\x81R`$\x81 h\xFB\xB6\x7F\xDAR\xD4\xBF\xB8\xBF\x83\x03aB1Wc\xF5\xA2g\xF1_R`\x04`\x1C\xFD[\x82aBCWh\xFB\xB6\x7F\xDAR\xD4\xBF\xB8\xBF\x92P[\x80\x19T\x80aB\xACW`\x01\x92P\x83\x82T\x03aBpW`\x01\x82\x01\x80T\x83U`\x02\x83\x01\x80T\x90\x91U_\x90Ua@\x1AV[\x83`\x01\x83\x01T\x03aB\x8EW`\x02\x82\x01\x80T`\x01\x84\x01U_\x90Ua@\x1AV[\x83`\x02\x83\x01T\x03aB\xA4W_`\x02\x83\x01Ua@\x1AV[_\x92Pa@\x1AV[\x81` R\x83_R`@_ \x80T\x80aB\xC5WPPa@\x1AV[`\x01\x83`\x01\x1C\x03\x92P\x82`\x01\x82\x03\x14aB\xEFW\x82\x84\x01T\x80`\x01\x83\x03\x86\x01U\x80_RP\x80`@_ U[P`\x01\x82`\x01\x1B\x17\x83\x19U_\x81UP`\x01\x92PPP\x92\x91PPV[_a\x05\xA3\x82b\x0FB@aR\x01V[`@Q\x7F\xA5\x98(\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x15\x15`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xA5\x98(\x85\x90`$\x01_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15aC{W_\x80\xFD[PZ\xFA\x15\x80\x15aC\x8DW=_\x80>=_\xFD[PPPPPV[_a\x05\xA3` `\x01`\x01`\xA0\x1B\x03\x84\x16;a\x8A\xA0V[`#T``\x90_\x90aC\xCF\x90\x84\x90a\n\xD0\x90a2^\x90`\x01`\x01`\xA0\x1B\x03\x16\x88aH\x19V[\x90P_\x83aC\xE0`\x14a\x01\0a\x8B\xDEV[aC\xEA\x91\x90a\x8B\xDEV[\x90P_aC\xF7\x82\x84a\x8C\x97V[\x90P_aD\x04\x83\x85a\x8C\xD8V[_\x88\x81R`0` R`@\x81 \x80T\x92\x93P\x90\x91[\x80\x83\x10\x15aDhW\x84`\x02\x0B\x82\x84\x81T\x81\x10aD7WaD7a\x87zV[_\x91\x82R` \x90\x91 `\n\x80\x83\x04\x90\x91\x01T\x91\x06`\x03\x02a\x01\0\n\x90\x04`\x02\x0B\x12\x15aDhW`\x01\x90\x92\x01\x91aD\x19V[\x82aDs\x81\x83a\x895V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aD\x8BWaD\x8Ba\x86\x98V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15aD\xB4W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x98P[\x81\x84\x10\x15aE`W\x84`\x02\x0B\x83\x85\x81T\x81\x10aD\xD6WaD\xD6a\x87zV[_\x91\x82R` \x90\x91 `\n\x80\x83\x04\x90\x91\x01T\x91\x06`\x03\x02a\x01\0\n\x90\x04`\x02\x0B\x13aE`W\x82\x84\x81T\x81\x10aE\rWaE\ra\x87zV[\x90_R` _ \x90`\n\x91\x82\x82\x04\x01\x91\x90\x06`\x03\x02\x90T\x90a\x01\0\n\x90\x04`\x02\x0B\x89\x82\x86\x03\x81Q\x81\x10aEBWaEBa\x87zV[`\x02\x92\x90\x92\x0B` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x90\x93\x01\x92aD\xB8V[\x90\x92\x03\x87RP\x94\x97\x96PPPPPPPV[\x81\x83RaE~\x81aR\x17V[\x83` \x01\x81\x90RPPPPV[\x80Q_\x90a\x05\xA0\x90aE\x9E\x90\x85\x90aE\xA5V[\x83\x90aRxV[_\x81`\x01\x03aE\xB5WP_a\x05\xA3V[_\x82aE\xC2\x81_\x19a\x8A\xA0V[aE\xCC\x91\x90a\x8A\x0FV[\x84Q` R\x90P_\x80[\x81_R`@_ \x90P\x82\x81\x10aE\xF1W`\x01\x82\x01\x91PaE\xD6V[\x84\x81\x06\x93PPP`\x03`\x1FSP`!`\x1F \x90\x92RP\x90V[_\x82_\x03aF_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01R\x7FLow 0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x81\x83\x10aF\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7FLow not below high\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[_aF\xB9\x83\x85aSoV[\x90P_aF\xCDaF\xC8\x83aS\xB1V[aT\x0CV[\x90P_aF\xDB\x87\x82\x84aVeV[\x90P_aF\xEFaF\xEA\x83aV\xB7V[aX\x9CV[\x90PaF\xFB\x87\x82aX\xD7V[\x98\x97PPPPPPPPV[_aGZ\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16`@Q\x80`@\x01`@R\x80`\x16\x81R` \x01\x7FUnsafe cast to uint128\0\0\0\0\0\0\0\0\0\0\x81RPaY\x06V[P\x90V[\x82\x82\x02\x81\x83\x85\x83\x04\x14\x85\x15\x17\x02aG\xE5W_\x19\x83\x85\t\x81\x81\x10\x82\x01\x90\x03\x82\x84\x86\t\x83_\x03\x84\x16\x82\x85\x11aG\x98Wc\xAEG\xF7\x02_R`\x04`\x1C\xFD[\x93\x84\x90\x04\x93\x83\x82\x11\x90\x92\x03_\x83\x90\x03\x83\x90\x04`\x01\x01\x02\x92\x03\x04\x17`\x02`\x03\x83\x02\x81\x18\x80\x84\x02\x82\x03\x02\x80\x84\x02\x82\x03\x02\x80\x84\x02\x82\x03\x02\x80\x84\x02\x82\x03\x02\x80\x84\x02\x82\x03\x02\x80\x84\x02\x90\x91\x03\x02\x02a\x06#V[\x04\x92\x91PPV[``aH\x10\x85\x85\x85\x85aH\x0Ba2^`\x01`\x01`\xA0\x1B\x03\x85\x16\x84aH\x19V[aY\x8BV[\x95\x94PPPPPV[_\x81\x81R`\x06` R`@\x81 aH9`\x01`\x01`\xA0\x1B\x03\x85\x16\x82a_;V[\x94\x93PPPPV[` \x02` \x01\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a_kV[\x83Q\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15aH\xD8W\x81aH\xA3\x85\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10aHAWaHAa\x87zV[`@Q` \x01aH\xB4\x92\x91\x90a\x8D\x19V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P\x80\x80aH\xD0\x90a\x8DTV[\x91PPaH`V[P\x80`@Q` \x01aH\xEA\x91\x90a\x8D\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x91\x90PV[``_[\x82Q\x81\x10\x15aI\\W\x81aI1\x84\x83\x81Q\x81\x10aI$WaI$a\x87zV[` \x02` \x01\x01Qa_vV[`@Q` \x01aIB\x92\x91\x90a\x8D\xC8V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x91P`\x01\x01aI\x05V[PaIg\x81Qa`cV[`\xE8\x1B\x81`@Q` \x01aI|\x92\x91\x90a\x8D\xD6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[``_[\x84Q\x81\x10\x15aI\xF9W\x81aI\xCE\x85\x85\x88\x85\x81Q\x81\x10aI\xB7WaI\xB7a\x87zV[` \x02` \x01\x01Qa`w\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[`@Q` \x01aI\xDF\x92\x91\x90a\x8D\xC8V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x91P`\x01\x01aI\x96V[PaJ\x04\x81Qa`cV[`\xE8\x1B\x81`@Q` \x01aJ\x19\x92\x91\x90a\x8D\xD6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x93\x92PPPV[``_\x80aJQ\x85_\x01Q\x86` \x01Q\x86aa6\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91P\x91P_\x85``\x01Q_\x01QaJhW_aJkV[`\x02[\x82aJvW_aJyV[`\x01[\x17\x90P\x80`\xF8\x1B\x83`\xF0\x1B\x87`@\x01Q`\x80\x1BaJ\x99\x89``\x01Qab\xB8V[`@Q` \x01aJ\xAC\x94\x93\x92\x91\x90a\x8E\tV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x93PPPP\x92\x91PPV[_aJ\xD1`(a5\x9DV[\x90P_[\x81\x81\x10\x15a?\x08W_aJ\xE9`(\x83a6;V[`$\x80T`@Q\x7F\xD5q\x97x\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\x04\x83\x01R\x93\x94P\x92\x16\x91c\xD5q\x97x\x91\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aKKW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aKo\x91\x90a\x8A&V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`-` R`@\x81 \x80T\x90\x91\x90aK\x96\x90\x84\x90a\x8E\x8CV[\x90\x91UPPP`\x01\x01aJ\xD5V[_\x81\x83\x11\x15aL\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R`\x84\x01aFVV[\x82\x84\x10\x15\x80\x15aL+WP\x81\x84\x11\x15[\x15aL7WP\x82a\x06#V[_aLB\x84\x84a\x895V[aLM\x90`\x01a\x87\xD4V[\x90P`\x03\x85\x11\x15\x80\x15aL_WP\x84\x81\x11[\x15aLvWaLn\x85\x85a\x87\xD4V[\x91PPa\x06#V[aL\x82`\x03_\x19a\x895V[\x85\x10\x15\x80\x15aL\x9AWPaL\x97\x85_\x19a\x895V[\x81\x11[\x15aL\xB4WaL\xAA\x85_\x19a\x895V[aLn\x90\x84a\x895V[\x82\x85\x11\x15aM\x07W_aL\xC7\x84\x87a\x895V[\x90P_aL\xD4\x83\x83a\x8E\xABV[\x90P\x80_\x03aL\xE8W\x84\x93PPPPa\x06#V[`\x01aL\xF4\x82\x88a\x87\xD4V[aL\xFE\x91\x90a\x895V[\x93PPPaMUV[\x83\x85\x10\x15aMUW_aM\x1A\x86\x86a\x895V[\x90P_aM'\x83\x83a\x8E\xABV[\x90P\x80_\x03aM;W\x85\x93PPPPa\x06#V[aME\x81\x86a\x895V[aMP\x90`\x01a\x87\xD4V[\x93PPP[P\x93\x92PPPV[a?\x08\x82\x82`@Q`$\x01aMs\x92\x91\x90a\x8CIV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xB6\x0Er\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Rac\xCCV[a8\x95\x81ac\xD8[c\xFF\xFF\xFF\xFF\x16V[_\x81\x83\x13\x15aNWW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`;`$\x82\x01R\x7FStdUtils bound(int256,int256,int`D\x82\x01R\x7F256): Max is less than min.\0\0\0\0\0`d\x82\x01R`\x84\x01aFVV[_\x80\x85\x12aN\x8EWaN\x89\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86a\x87\xD4V[aN\xC5V[`\x01aN\xBB\x86\x19\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x895V[aN\xC5\x91\x90a\x895V[\x90P_\x80\x85\x12aN\xFEWaN\xF9\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86a\x87\xD4V[aO5V[`\x01aO+\x86\x19\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x895V[aO5\x91\x90a\x895V[\x90P_\x80\x85\x12aOnWaOi\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86a\x87\xD4V[aO\xA5V[`\x01aO\x9B\x86\x19\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x895V[aO\xA5\x91\x90a\x895V[\x90P_aO\xB3\x84\x84\x84aK\xA4V[\x90P\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x10aP\x0BWaP\x06\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82a\x895V[aF\xFBV[aP5\x81\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x895V[aF\xFB\x90\x19`\x01a\x87\xD4V[a?\x08\x82\x82`@Q`$\x01aPW\x92\x91\x90a\x8C%V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FK\\Bw\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Rac\xCCV[_\x80`\x06` R\x83_R`\x04`@_ \x01` R\x82_R`@_ ` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x88Z\xFAaP\xF4WcS\\\xF9K_R`\x04`\x1C\xFD[PP_Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x94`\x80\x91\x90\x91\x1D\x93P\x91PPV[_\x81\x83\x11aQ'W\x81a\x05\xA0V[P\x90\x91\x90PV[_\x80\x84`\x02\x0B\x87`\x02\x0B\x12\x15aQbWaQ[aQJ\x86ac\xF7V[aQS\x86ac\xF7V[\x85`\x01af\xD5V[\x91PaQ\xBBV[\x83`\x02\x0B\x87`\x02\x0B\x12\x15aQ\x9BWaQ}\x86aQS\x86ac\xF7V[\x91PaQ\x94aQ\x8B\x86ac\xF7V[\x87\x85`\x01ag\x9FV[\x90PaQ\xBBV[aQ\xB8aQ\xA7\x86ac\xF7V[aQ\xB0\x86ac\xF7V[\x85`\x01ag\x9FV[\x90P[\x95P\x95\x93PPPPV[_\x81\x83\x10aQ'W\x81a\x05\xA0V[_\x82\x81R`\x06` \x90\x81R`@\x80\x83 \x84\x84R`\x05\x01\x90\x91R\x81 aH\x10`\x01`\x01`\xA0\x1B\x03\x86\x16\x82a_;V[_a\x05\xA0\x83b\xFF\xFF\xFF\x16_\x84b\xFF\xFF\xFF\x16a5\xEEV[`@\x80Q\x80\x82\x01\x90\x91R_\x81R``` \x82\x01R\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aRDWaRDa\x86\x98V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15aRmW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P` \x82\x01R\x91\x90PV[\x81Q_\x90aR\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7FNothing to use\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[_\x80aR\xD4\x85\x85ah?V[\x87Q\x90\x95P\x91\x93P\x91P_\x90aR\xF7\x90aR\xF0\x90`\x01\x90a\x895V[\x87\x90ah?V[\x92PPP`\x01\x86_\x01\x81\x81QaS\r\x91\x90a\x895V[\x90RP\x82\x15aS@W` \x86\x01Qah\xF6\x90\x81\x90aS9\x90aS/\x90\x86ah\xFEV[\x84\x83c\xFF\xFF\xFF\xFF\x16V[PPa8\x1BV[`@\x80Q\x80\x82\x01\x90\x91R\x85\x81R` \x80\x82\x01\x83\x90R\x87\x01Q\x81\x90aSd\x90\x82ai`V[PPPPP\x92\x91PPV[_x\x12r]\xD1\xD2C\xAB\xA0\xE7_\xE6E\xCCHs\xF9\xE6Z\xFEh\x8C\x92\x8E\x1F\"\x83\x10\x82\x02aS\x9FWc|_H}_R`\x04`\x1C\xFD[Pg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90\x91\x02\x04\x90V[_\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aGZW`@Q\x7F5'\x8D\x12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17_\x82\x13aTiWc\x16\x15\xE68_R`\x04`\x1C\xFD[\x7F\xF8\xF9\xF9\xFA\xF9\xFD\xFA\xFB\xF9\xFD\xFC\xFD\xFA\xFB\xFC\xFE\xF9\xFA\xFD\xFA\xFC\xFC\xFB\xFE\xFA\xFA\xFC\xFB\xFF\xFF\xFF\xFFo\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x83\x83\x1C\x1C`\x1F\x16\x1A\x18\x90\x81\x1B`\x9F\x90\x81\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFs\xC0\xC7\x16\xA5\x94\xE0\rT\xE3\xC4\xCB\xC9\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xC7\xB8\x8CB\x0ES\xA9\x89\x053\x12\x9Fo\x01\x83\x02\x90\x91\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFF_\xDA'\xEBMc\xDE\xD4t\xE5\xF82\x01\x90\x91\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF5\xF6\xAF\x8F{3\x96dO\x18\xE1W\x96\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02\x91\x90\x03}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x90V[_\x81\x83\x12aV\x9FW`@Q\x7F\xA8\x83CW\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x82\x03\x83aV\xAD\x86\x83aE\xA5V[\x01\x95\x94PPPPPV[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xC0\xD0W\t%\xA4b\xD7\x82\x13aV\xE4W\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12aW\x01Wc\xA3{\xFE\xC9_R`\x04`\x1C\xFD[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P_``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05k\x80\0\0\0\0\0\0\0\0\0\0\0\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xDB\xF3\xCC\xF1`M&4P\xF0*U\x04\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE5\xAD\xED\xAA\x1C\xB0\x95\xAF\x9EM\xA1\x0E6<\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD8\xDCw&\x08\xB0\xAEV\xCC\xE0\x12\x96\xC0\xEB\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE,i\x81,\xF0;\x07c\xFDEJ\x8F~\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02y\xD85\xEB\xBA\x82L\x98\xFB1\xB8;,\xA4\\\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[_\x80\x82\x12\x15aGZW`@Q\x7F\xCA\xCC\xB6\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81_\x19\x04\x83\x11\x15aX\xF6W\x81\x15aX\xF6Wc\xBA\xC6^[_R`\x04`\x1C\xFD[Pg\r\xE0\xB6\xB3\xA7d\0\0\x91\x02\x04\x90V[`@Q\x7F\xD1}K\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xD1}K\r\x90aYZ\x90\x86\x90\x86\x90\x86\x90`\x04\x01a\x8E\xBEV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15aYpW_\x80\xFD[PZ\xFA\x15\x80\x15aY\x82W=_\x80>=_\xFD[PPPPPPPV[```\x01\x83`\x02\x0B\x12\x15aY\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FInvalid TICK_SPACING\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[\x85Q\x15aH\x10WaY\xF4\x85\x85\x88\x86ajaV[aY\xFD\x86ak\"V[\x81`\x02\x0B\x86`\x01\x88QaZ\x10\x91\x90a\x895V[\x81Q\x81\x10aZ WaZ a\x87zV[` \x02` \x01\x01Q_\x01Q`\x02\x0B\x13aZ\xB8W`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x90\x81` \x01[`@\x80Q`\xC0\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x80\x83\x01\x82\x90R`\x80\x83\x01R`\xA0\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81aZHW\x90PP\x90PaZ\x96\x85\x85\x88\x85\x87ak\xDBV[\x81_\x81Q\x81\x10aZ\xA8WaZ\xA8a\x87zV[` \x02` \x01\x01\x81\x90RPaH\x10V[\x85_\x81Q\x81\x10aZ\xCAWaZ\xCAa\x87zV[` \x02` \x01\x01Q_\x01Q`\x02\x0B\x82`\x02\x0B\x13a[DW`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x90\x81` \x01[`@\x80Q`\xC0\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x80\x83\x01\x82\x90R`\x80\x83\x01R`\xA0\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81aZ\xF6W\x90PP\x90PaZ\x96\x85\x85\x88\x85\x87ap\xE4V[\x81_[a[\\`\x01`\x01`\xA0\x1B\x03\x88\x16\x87\x84\x88au\xF9V[\x92P\x90P\x80a[wW\x81a[o\x81a\x8E\xDCV[\x92PPa[GV[\x87_\x81Q\x81\x10a[\x89Wa[\x89a\x87zV[` \x02` \x01\x01Q_\x01Q`\x02\x0B\x82`\x02\x0B\x13a\\'W`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x90\x81` \x01[`@\x80Q`\xC0\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x80\x83\x01\x82\x90R`\x80\x83\x01R`\xA0\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81a[\xB5W\x90PP\x92Pa\\\x03\x87\x87\x8A\x87\x89ap\xE4V[\x83_\x81Q\x81\x10a\\\x15Wa\\\x15a\x87zV[` \x02` \x01\x01\x81\x90RPPPaH\x10V[PP_a\\4\x87\x84avNV[\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\\PWa\\Pa\x86\x98V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\\\x94W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\\nW\x90P[P\x90P_\x82\x89Qa\\\xA5\x91\x90a\x895V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\\\xBDWa\\\xBDa\x86\x98V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a]\x01W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\\\xDBW\x90P[P\x90P_[\x89Q\x81\x10\x15a]\x9EW\x83\x81\x10\x15a]SW\x89\x81\x81Q\x81\x10a])Wa])a\x87zV[` \x02` \x01\x01Q\x83\x82\x81Q\x81\x10a]CWa]Ca\x87zV[` \x02` \x01\x01\x81\x90RPa]\x96V[\x89\x81\x81Q\x81\x10a]eWa]ea\x87zV[` \x02` \x01\x01Q\x82\x85\x83a]z\x91\x90a\x895V[\x81Q\x81\x10a]\x8AWa]\x8Aa\x87zV[` \x02` \x01\x01\x81\x90RP[`\x01\x01a]\x06V[P\x81Q_\x03a^/W`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x90\x81` \x01[`@\x80Q`\xC0\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x80\x83\x01\x82\x90R`\x80\x83\x01R`\xA0\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81a]\xBCW\x90PP\x93Pa^\n\x88\x88\x83\x88\x8Aap\xE4V[\x84_\x81Q\x81\x10a^\x1CWa^\x1Ca\x87zV[` \x02` \x01\x01\x81\x90RPPPPaH\x10V[\x80Q_\x03a^\x9AW`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x90\x81` \x01[`@\x80Q`\xC0\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x80\x83\x01\x82\x90R`\x80\x83\x01R`\xA0\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81a^LW\x90PP\x93Pa^\n\x88\x88\x84\x88\x8Aak\xDBV[`@\x80Q`\x02\x80\x82R``\x82\x01\x90\x92R\x90\x81` \x01[`@\x80Q`\xC0\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x80\x83\x01\x82\x90R`\x80\x83\x01R`\xA0\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81a^\xB0W\x90PP\x93Pa^\xFE\x88\x88\x83\x88\x8Aap\xE4V[\x84_\x81Q\x81\x10a_\x10Wa_\x10a\x87zV[` \x02` \x01\x01\x81\x90RPa_(\x88\x88\x84\x88\x8Aak\xDBV[\x84`\x01\x81Q\x81\x10a^\x1CWa^\x1Ca\x87zV[_\x81` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x86Z\xFAa_bWcS\\\xF9K_R`\x04`\x1C\xFD[PP_Q\x91\x90PV[``a\x05\xA3\x82ag\xFDV[\x80Q` \x80\x83\x01Q`@\x80\x85\x01Q``\x80\x87\x01Q\x92Q\x95\x81\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x16\x94\x86\x01\x94\x90\x94R`\x80\x92\x83\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16`4\x87\x01R\x90\x83\x1B\x81\x16`D\x86\x01R\x91\x1B\x16`T\x83\x01R\x90`d\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`D\x81Q\x14a`^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FAssets unexpected length\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[\x91\x90PV[_c\x01\0\0\0\x82\x10aGZWaGZav\xECV[``a`\x82\x84av\xF9V[\x83Q` \x85\x01Q_\x91\x82\x91a`\x98\x91\x87\x91awnV[\x91P\x91P_a`\xAF\x85\x88_\x01Q\x89` \x01Qaw\x9BV[\x90P\x82`\xF0\x1B\x82`\xF0\x1B\x82`\xF0\x1Ba`\xC8\x8A`@\x01Q\x90V[`@Q\x7F\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x94\x85\x16` \x82\x01R\x92\x84\x16`\"\x84\x01R\x92\x16`$\x82\x01R`&\x81\x01\x91\x90\x91R`F\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x93P`&\x84Q\x14aa,W_\x80\xFD[PPP\x93\x92PPPV[_\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x03aa\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FassetIn == assetOut\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x10\x90P_\x80\x82aa\xBEW\x84\x86aa\xC1V[\x85\x85[\x91P\x91P_\x93P[\x86Q\x84a\xFF\xFF\x16\x10\x15abYW_\x87\x85a\xFF\xFF\x16\x81Q\x81\x10aa\xEDWaa\xEDa\x87zV[` \x02` \x01\x01Q\x90Pab\0\x81av\xF9V[\x82`\x01`\x01`\xA0\x1B\x03\x16\x81_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14\x80\x15ab9WP\x81`\x01`\x01`\xA0\x1B\x03\x16\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14[\x15abFWPPPab\xB0V[P\x83abQ\x81a\x8F\x19V[\x94PPaa\xC9V[\x86Q\x84a\xFF\xFF\x16\x10ab\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7FPair not found\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[PP[\x93P\x93\x91PPV[\x80Q``\x90\x15ac\x11W\x81` \x01Q`\x80\x1B\x82``\x01Q`\x80\x1B`@Q` \x01aI|\x92\x91\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x83\x16\x81R\x91\x16`\x10\x82\x01R` \x01\x90V[``_[\x83`\x80\x01QQ\x81\x10\x15acoW\x81\x84`\x80\x01Q\x82\x81Q\x81\x10ac9Wac9a\x87zV[` \x02` \x01\x01Q`\x80\x1B`@Q` \x01acU\x92\x91\x90a\x8F0V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x91P`\x01\x01ac\x15V[Pacz\x81Qa`cV[`\xE8\x1B\x81`@Q` \x01ac\x8F\x92\x91\x90a\x8D\xD6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x82`@\x01Q`\xE8\x1B\x83``\x01Q`\x80\x1B\x82\x85`\xA0\x01Q``\x1B`@Q` \x01aH\xEA\x94\x93\x92\x91\x90a\x8FlV[a8\x95\x81ax\xD6aM\xD8V[_jconsole.log\x90P_\x80\x83Q` \x85\x01\x84Z\xFAPPPV[`\x02\x0B_`\xFF\x82\x90\x1D\x80\x83\x01\x18b\r\x89\xE8\x81\x11\x15ad9Wad9\x7F\x8B\x862z\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84ax\xF6V[p\x01\xFF\xFC\xB93\xBDo\xAD7\xAA-\x16-\x1AY@\x01`\x01\x82\x16\x02p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x18`\x02\x82\x16\x15ad\x82Wo\xFF\xF9rr7=A2Y\xA4i\x90X\x0E!:\x02`\x80\x1C[`\x04\x82\x16\x15ad\xA1Wo\xFF\xF2\xE5\x0F_ei2\xEF\x125|\xF3\xC7\xFD\xCC\x02`\x80\x1C[`\x08\x82\x16\x15ad\xC0Wo\xFF\xE5\xCA\xCA~\x10\xE4\xE6\x1C6$\xEA\xA0\x94\x1C\xD0\x02`\x80\x1C[`\x10\x82\x16\x15ad\xDFWo\xFF\xCB\x98C\xD6\x0FaY\xC9\xDBX\x83\\\x92fD\x02`\x80\x1C[` \x82\x16\x15ad\xFEWo\xFF\x97;A\xFA\x98\xC0\x81G.h\x96\xDF\xB2T\xC0\x02`\x80\x1C[`@\x82\x16\x15ae\x1DWo\xFF.\xA1df\xC9j8C\xECx\xB3&\xB5(a\x02`\x80\x1C[`\x80\x82\x16\x15ae<Wo\xFE]\xEE\x04j\x99\xA2\xA8\x11\xC4a\xF1\x96\x9C0S\x02`\x80\x1C[a\x01\0\x82\x16\x15ae\\Wo\xFC\xBE\x86\xC7\x90\n\x88\xAE\xDC\xFF\xC8;G\x9A\xA3\xA4\x02`\x80\x1C[a\x02\0\x82\x16\x15ae|Wo\xF9\x87\xA7%:\xC4\x13\x17o+\x07L\xF7\x81^T\x02`\x80\x1C[a\x04\0\x82\x16\x15ae\x9CWo\xF39+\x08\"\xB7\0\x05\x94\x0Cz9\x8EKp\xF3\x02`\x80\x1C[a\x08\0\x82\x16\x15ae\xBCWo\xE7\x15\x94u\xA2\xC2\x9BtC\xB2\x9C\x7F\xA6\xE8\x89\xD9\x02`\x80\x1C[a\x10\0\x82\x16\x15ae\xDCWo\xD0\x97\xF3\xBD\xFD \"\xB8\x84Z\xD8\xF7\x92\xAAX%\x02`\x80\x1C[a \0\x82\x16\x15ae\xFCWo\xA9\xF7FF-\x87\x0F\xDF\x8Ae\xDC\x1F\x90\xE0a\xE5\x02`\x80\x1C[a@\0\x82\x16\x15af\x1CWop\xD8i\xA1V\xD2\xA1\xB8\x90\xBB=\xF6+\xAF2\xF7\x02`\x80\x1C[a\x80\0\x82\x16\x15af<Wo1\xBE\x13_\x97\xD0\x8F\xD9\x81#\x15\x05T/\xCF\xA6\x02`\x80\x1C[b\x01\0\0\x82\x16\x15af]Wo\t\xAAP\x8B[z\x84\xE1\xC6w\xDET\xF3\xE9\x9B\xC9\x02`\x80\x1C[b\x02\0\0\x82\x16\x15af}Wn]j\xF8\xDE\xDB\x81\x19f\x99\xC3)\"^\xE6\x04\x02`\x80\x1C[b\x04\0\0\x82\x16\x15af\x9CWm\"\x16\xE5\x84\xF5\xFA\x1E\xA9&\x04\x1B\xED\xFE\x98\x02`\x80\x1C[b\x08\0\0\x82\x16\x15af\xB9Wk\x04\x8A\x17\x03\x91\xF7\xDCBDN\x8F\xA2\x02`\x80\x1C[_\x84\x13\x15af\xC5W_\x19\x04[c\xFF\xFF\xFF\xFF\x01` \x1C\x93\x92PPPV[_\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x11\x15af\xF4W\x92\x93\x92[`\x01`\x01`\xA0\x1B\x03\x85\x16ag\x0EWb\xBF\xC9!_R`\x04`\x1C\xFD[{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0``\x84\x90\x1B\x16`\x01`\x01`\xA0\x1B\x03\x86\x86\x03\x16\x83agsW\x86`\x01`\x01`\xA0\x1B\x03\x16ag`\x83\x83\x89`\x01`\x01`\xA0\x1B\x03\x16ay\x05V[\x81agmWagma\x8AsV[\x04a=\x95V[a=\x95ag\x8A\x83\x83\x89`\x01`\x01`\xA0\x1B\x03\x16ay\xA1V[\x88`\x01`\x01`\xA0\x1B\x03\x16\x80\x82\x04\x91\x06\x15\x15\x01\x90V[_`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x90\x86\x16\x03`\xFF\x81\x90\x1D\x90\x81\x01\x18l\x01\0\0\0\0\0\0\0\0\0\0\0\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16ag\xE4\x81\x84\x84ay\x05V[\x93P\x84_\x83\x85\x84\t\x11\x16\x84\x01\x93PPPP\x94\x93PPPPV[```\x80`@Q\x01\x90P` \x81\x01`@R_\x81R\x80_\x19\x83[\x92\x81\x01\x92`0`\n\x82\x06\x01\x84S`\n\x90\x04\x80ah\x16WPP\x81\x90\x03`\x1F\x19\x90\x91\x01\x90\x81R\x91\x90PV[_\x80_\x84_\x01Q\x84\x10ah\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FIndex out-of-bounds\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[_\x91Pah\xFE\x80[` \x87\x01QQ\x84\x10\x15ah\xE6W_ah\xBC\x88` \x01Q\x86\x84c\xFF\xFF\xFF\xFF\x16V[\x90P\x86\x81_\x01Q\x03ah\xDAW` \x01Q`\x01\x95P\x92Pah\xEF\x91PPV[P`\x01\x90\x93\x01\x92ah\x9CV[P_\x93P\x84\x91PP[\x92P\x92P\x92V[` \x90\x91\x01RV[\x81Q_\x90\x82\x10ai:W`@Q\x7FD\xDD6\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82` \x01Q\x82\x81Q\x81\x10aiPWaiPa\x87zV[` \x02` \x01\x01Q\x90P\x92\x91PPV[\x81Q\x80\x83aim\x82a\x8F\xFAV[\x90RP` \x83\x01QQ\x83Q\x81\x10\x15aj8W_ai\x8B\x82`\x02a\x8A\x0FV[\x90P_`\x01\x82\x10ai\x9CW\x81ai\x9FV[`\x01[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ai\xB7Wai\xB7a\x86\x98V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15ai\xE0W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83\x81\x10\x15aj0W\x86` \x01Q\x81\x81Q\x81\x10aj\x03Waj\x03a\x87zV[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10aj\x1DWaj\x1Da\x87zV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01ai\xE5V[P` \x86\x01RP[\x82\x84` \x01Q\x83\x81Q\x81\x10ajOWajOa\x87zV[` \x02` \x01\x01\x81\x81RPPPPPPV[_[\x82Q\x81\x10\x15aC\x8DW_\x83\x82\x81Q\x81\x10aj\x7FWaj\x7Fa\x87zV[` \x02` \x01\x01Q_\x01Q\x90P_\x80aj\xA1a=Z\x84\x87_\x81\x83\x07\x12\x91\x05\x03\x90V[\x90\x92P\x90P_aj\xBB`\x01`\x01`\xA0\x1B\x03\x8A\x16\x89\x85aQ\xD3V[\x90P`\x01`\xFF\x83\x16\x1B\x81\x16ak\x12W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FTick not initialized\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[PP`\x01\x90\x92\x01\x91Pajc\x90PV[ak+\x81ay\xD1V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0_[\x82Q\x81\x10\x15ak\xD6W_\x83\x82\x81Q\x81\x10akjWakja\x87zV[` \x02` \x01\x01Q_\x01Q\x90P\x82`\x02\x0B\x81`\x02\x0B\x13ak\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7FDuplicate tick\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[\x91P`\x01\x01akNV[PPPV[`@\x80Q`\xC0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x80\x82\x01\x83\x90R`\x80\x82\x01R`\xA0\x81\x01\x91\x90\x91R_\x84Q\x11al[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01R\x7FNo rewards\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[_al~`\x02\x86Q`\x03alo\x91\x90a\x8A\x0FV[aly\x91\x90a\x8A\xA0V[aR\x17V[\x90P_\x85_\x81Q\x81\x10al\x93Wal\x93a\x87zV[` \x90\x81\x02\x91\x90\x91\x01\x01QQ\x90P`\x01_[al\xBA`\x01`\x01`\xA0\x1B\x03\x8B\x16\x8A\x85\x89az\xBEV[\x93P\x91P`\x02\x83\x81\x0B\x90\x88\x90\x0B\x12ampW\x81\x15al\xE7Wal\xE0\x84`\x02\x85\x90\x0Bai`V[P_al\xA5V[\x80al\xF1\x81a\x8F\xFAV[\x91PP`x\x81\x11\x15amkW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FMAX_LOOP exceeded in _createRewa`D\x82\x01R\x7FrdUpdateBelow\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01aFVV[al\xA5V[PP\x81Q_\x03\x90Pam\xFDW\x84Q`\x01\x14am\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7Fexpected rewards length 1\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[am\xF5\x87\x87\x87_\x81Q\x81\x10am\xE4Wam\xE4a\x87zV[` \x02` \x01\x01Q` \x01Qaz\xFBV[\x91PPaH\x10V[\x80Qan\n\x90`\x01a\x87\xD4V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15an\"Wan\"a\x86\x98V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15anKW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P`\x80\x83\x01R_\x80\x80[\x83Q\x81\x10\x15ao\x1DW_ani\x85\x83ah\xFEV[\x90P_an\x80`\x01`\x01`\xA0\x1B\x03\x8D\x16\x8C\x84aP\xB4V[\x91Pan\x8E\x90P\x81\x85a\x90\x12V[\x93P\x89Q\x85\x10\x15ao\x13W_\x8A\x86\x81Q\x81\x10an\xACWan\xACa\x87zV[` \x02` \x01\x01Q\x90P\x82`\x02\x0B\x81_\x01Q`\x02\x0B\x12\x15ao\x11W\x80` \x01Q\x88`\x80\x01Q\x85\x81Q\x81\x10an\xE2Wan\xE2a\x87zV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x85ao\r\x81a\x8F\xFAV[\x96PP[P[PP`\x01\x01anUV[P\x86Q\x82\x10\x15ao\x8DW\x86\x82\x81Q\x81\x10ao9Wao9a\x87zV[` \x02` \x01\x01Q` \x01Q\x84`\x80\x01Q\x84_\x01Q\x81Q\x81\x10ao^Wao^a\x87zV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x81ao\x89\x81a\x8F\xFAV[\x92PP[\x86Q\x82\x14ao\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01R\x7FNot all rewards used?\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[ao\xE7\x83_ah\xFEV[`\x02\x0B`@\x85\x01R_ao\xFC\x8A\x8A\x89\x89a{nV[\x90Pap\x08\x81\x83a}\xADV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x86\x01\x81\x90R\x89\x90\x8B\x90_\x90\x81[\x88Q\x81\x10\x15ap\xCBW_ap=\x8A\x83ah\xFEV[\x90P_apT`\x01`\x01`\xA0\x1B\x03\x87\x16\x88\x84aP\xB4V[\x91PPapa\x84\x82a}\xC7V[`@\x80Q` \x80\x82\x01\x98\x90\x98R`\x80\x83\x90\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81\x83\x01R`\xE8\x94\x90\x94\x1B`P\x85\x01R\x80Q`3\x81\x86\x03\x01\x81R`S\x90\x94\x01\x90R\x82Q\x92\x90\x95\x01\x91\x90\x91 \x93\x92PP`\x01\x01ap)V[PP``\x1C`\xA0\x88\x01RPPPPPP\x95\x94PPPPPV[`@\x80Q`\xC0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x80\x82\x01\x83\x90R`\x80\x82\x01R`\xA0\x81\x01\x91\x90\x91R_\x84Q\x11aqdW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01R\x7FNo rewards\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[_aqx`\x02\x86Q`\x03alo\x91\x90a\x8A\x0FV[\x90P_\x85`\x01\x87Qaq\x8A\x91\x90a\x895V[\x81Q\x81\x10aq\x9AWaq\x9Aa\x87zV[` \x90\x81\x02\x91\x90\x91\x01\x01QQ\x90P`\x01\x81_[\x81`\x02\x0B\x88`\x02\x0B\x12\x15aryW\x82\x15aq\xD6Waq\xCF\x85`\x02\x84\x90\x0Bai`V[P_arZV[\x80aq\xE0\x81a\x8F\xFAV[\x91PP`x\x81\x11\x15arZW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FMAX_LOOP exceeded in _createRewa`D\x82\x01R\x7FrdUpdateAbove\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01aFVV[aro`\x01`\x01`\xA0\x1B\x03\x8C\x16\x8B\x84\x8Aa}\xE1V[\x90\x93P\x91Paq\xADV[PP\x82Q_\x03\x90Pas\x17War\xA6`@Q\x80`\x80\x01`@R\x80`D\x81R` \x01a\xA92`D\x919a8$V[\x85Q`\x01\x14ar\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FExpected exact one reward\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[as\x0E\x88\x88\x88_\x81Q\x81\x10am\xE4Wam\xE4a\x87zV[\x92PPPaH\x10V[`\x02\x81\x90\x0B`@\x84\x01R\x81Qas.\x90`\x01a\x87\xD4V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15asFWasFa\x86\x98V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15asoW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P`\x80\x84\x01R\x85Q_\x80[\x84Q\x81\x10\x15atHW_as\x8E\x86\x83ah\xFEV[\x90P_as\xA5`\x01`\x01`\xA0\x1B\x03\x8E\x16\x8D\x84aP\xB4V[\x91Pas\xB3\x90P\x81\x85a\x90\x12V[\x93P\x84\x15at>W_\x8Bas\xC8`\x01\x88a\x895V[\x81Q\x81\x10as\xD8Was\xD8a\x87zV[` \x02` \x01\x01Q\x90P\x82`\x02\x0B\x81_\x01Q`\x02\x0B\x12at<W\x80` \x01Q\x89`\x80\x01Q\x85\x81Q\x81\x10at\rWat\ra\x87zV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x85at8\x81a\x90`V[\x96PP[P[PP`\x01\x01aszV[P\x81\x15at\xB5W\x87_\x81Q\x81\x10ataWataa\x87zV[` \x02` \x01\x01Q` \x01Q\x85`\x80\x01Q\x85_\x01Q\x81Q\x81\x10at\x86Wat\x86a\x87zV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x81at\xB1\x81a\x90`V[\x92PP[\x81\x15au\x03W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01R\x7FNot all rewards used?\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[_au\x10\x8B\x8B\x8A\x8Aa{nV[\x90Pau\x1C\x81\x83a}\xC7V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x87\x01\x81\x90R\x8A\x90\x8C\x90_\x90\x81[\x89Q\x81\x10\x15au\xDFW_auQ\x8B\x83ah\xFEV[\x90P_auh`\x01`\x01`\xA0\x1B\x03\x87\x16\x88\x84aP\xB4V[\x91PPauu\x84\x82a}\xADV[`@\x80Q` \x80\x82\x01\x98\x90\x98R`\x80\x83\x90\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81\x83\x01R`\xE8\x94\x90\x94\x1B`P\x85\x01R\x80Q`3\x81\x86\x03\x01\x81R`S\x90\x94\x01\x90R\x82Q\x92\x90\x95\x01\x91\x90\x91 \x93\x92PP`\x01\x01au=V[PP``\x1C`\xA0\x89\x01RPPPPPPP\x95\x94PPPPPV[_\x80\x80\x80av\x0E\x85\x87\x07\x82\x13\x86\x88\x05\x03a=ZV[\x90\x92P\x90Pav1\x81av+`\x01`\x01`\xA0\x1B\x03\x8B\x16\x8A\x86aQ\xD3V[\x90a}\xFBV[\x90\x94P\x90PavA\x82\x82\x87a~\xC3V[\x92PPP\x94P\x94\x92PPPV[_\x80\x83Q\x11av\x9FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01R\x7FNo rewards\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[_[\x83Q\x81\x10\x15av\xE3W\x82`\x02\x0B\x84\x82\x81Q\x81\x10av\xC0Wav\xC0a\x87zV[` \x02` \x01\x01Q_\x01Q`\x02\x0B\x13\x15av\xDBW\x90Pa\x05\xA3V[`\x01\x01av\xA1V[PP\x90Q\x91\x90PV[c5'\x8D\x12_R`\x04`\x1C\xFD[\x80_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x11a8\x95W`@\x80Q\x7FQ\x904C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x04\x83\x01R` \x84\x01Q\x16`$\x82\x01R\x90\x82\x01Q`D\x82\x01R`d\x01aFVV[_\x80aw\x82aw}\x86\x86a~\xEDV[a\x7F\x86V[\x91Paw\x91aw}\x86\x85a~\xEDV[\x90P\x93P\x93\x91PPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10aw\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FgetStoreIndex:assets unsorted\0\0\0`D\x82\x01R`d\x01aFVV[_\x83\x81R` \x83\x81R`@\x82 `(\x1B\x91\x90ax#\x90`\x01`\x01`\xA0\x1B\x03\x88\x16;a\x8A\xA0V[_\x93P\x90P\x85[\x81\x84a\xFF\xFF\x16\x10\x15ax\x8EW_` \x85` \x02`\x01\x01_\x84<P_Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x80\x85\x16\x90\x82\x16\x03ax{WPPPPa\x06#V[P\x83ax\x86\x81a\x8F\x19V[\x94PPax*V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FPool not enabled\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[\x80Qjconsole.log` \x83\x01_\x80\x84\x83\x85Z\xFAPPPPPV[\x81_R\x80`\x02\x0B`\x04R`$_\xFD[_\x83\x83\x02\x81_\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80\x84\x11ay$W_\x80\xFD[\x80_\x03ay6WP\x82\x90\x04\x90Pa\x06#V[_\x84\x86\x88\t_\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP\x93\x92PPPV[_ay\xAD\x84\x84\x84ay\x05V[\x90P\x81\x80ay\xBDWay\xBDa\x8AsV[\x83\x85\t\x15a\x06#W`\x01\x01\x80a\x06#W_\x80\xFD[_[\x81Q\x81\x10\x15a?\x08W_ay\xE8\x82`\x01a\x87\xD4V[\x90P[\x82Q\x81\x10\x15az\xB5Waz9\x83\x82\x81Q\x81\x10az\tWaz\ta\x87zV[` \x02` \x01\x01Q\x84\x84\x81Q\x81\x10az#Waz#a\x87zV[` \x02` \x01\x01Qa\x7F\x99\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x15az\xADW\x82\x81\x81Q\x81\x10azPWazPa\x87zV[` \x02` \x01\x01Q\x83\x83\x81Q\x81\x10azjWazja\x87zV[` \x02` \x01\x01Q\x84\x84\x81Q\x81\x10az\x84Waz\x84a\x87zV[` \x02` \x01\x01\x85\x84\x81Q\x81\x10az\x9DWaz\x9Da\x87zV[` \x02` \x01\x01\x82\x90R\x82\x90RPP[`\x01\x01ay\xEBV[P`\x01\x01ay\xD3V[_\x80\x80\x80az\xD8a=Z\x86\x88\x07\x83\x13\x87\x89\x05\x03`\x01a\x8C\xD8V[\x90\x92P\x90Pav1\x81az\xF5`\x01`\x01`\xA0\x1B\x03\x8B\x16\x8A\x86aQ\xD3V[\x90a\x7F\xA7V[`@\x80Q`\xC0\x81\x01\x82R_\x91\x81\x01\x82\x90R``\x80\x82\x01\x83\x90R`\x80\x82\x01R`\xA0\x81\x01\x91\x90\x91R`\x01\x81Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16` \x82\x01Ra{P`\x01`\x01`\xA0\x1B\x03\x85\x16\x84a\x80iV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x82\x01R\x93\x92PPPV[_\x80a{\x86a2^`\x01`\x01`\xA0\x1B\x03\x88\x16\x87aH\x19V[\x90P_a{\x9C`\x01`\x01`\xA0\x1B\x03\x88\x16\x87a\x80iV[\x90P\x84`\x02\x0B\x82`\x02\x0B\x12\x15a|\x9BW_\x82\x81[a{\xC5`\x01`\x01`\xA0\x1B\x03\x8B\x16\x8A\x84\x8Aaz\xBEV[\x90\x93P\x91P`\x02\x82\x81\x0B\x90\x89\x90\x0B\x12a|\x93W\x82\x15a|\nWP_\x80a{\xF5`\x01`\x01`\xA0\x1B\x03\x8C\x16\x8B\x85aP\xB4V[\x91PPa|\x02\x85\x82a}\xC7V[\x94PPa{\xB0V[\x80a|\x14\x81a\x8F\xFAV[\x91PP`x\x81\x11\x15a|\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FMAX_LOOP exceeded in getLiquidit`D\x82\x01R\x7FyAtTick [present < future]\0\0\0\0\0\0`d\x82\x01R`\x84\x01aFVV[a{\xB0V[PPPa}\xA3V[\x81`\x02\x0B\x85`\x02\x0B\x12\x15a}\xA3W`\x01\x82_[a|\xC3`\x01`\x01`\xA0\x1B\x03\x8B\x16\x8A\x84\x8Aau\xF9V[\x90\x93P\x91P`\x02\x88\x81\x0B\x90\x83\x90\x0B\x13\x15a}\x9FW\x82\x15a}\tWP_\x80a|\xF4`\x01`\x01`\xA0\x1B\x03\x8C\x16\x8B\x85aP\xB4V[\x91PPa}\x01\x85\x82a}\xADV[\x94PPa}\x8DV[\x80a}\x13\x81a\x8F\xFAV[\x91PP`x\x81\x11\x15a}\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FMAX_LOOP exceeded in getLiquidit`D\x82\x01R\x7FyAtTick [future < present]\0\0\0\0\0\0`d\x82\x01R`\x84\x01aFVV[\x81a}\x97\x81a\x8E\xDCV[\x92PPa|\xAEV[PPP[\x96\x95PPPPPPV[\x80\x82\x03`\x80\x81\x90\x1C\x15a\x05\xA3Wc\xC9eN\xD4_R`\x04`\x1C\xFD[\x81\x81\x01`\x80\x81\x90\x1C\x15a\x05\xA3Wc\xC9eN\xD4_R`\x04`\x1C\xFD[_\x80\x80\x80av\x0Ea=Z`\x01\x87\x89\x07\x84\x13\x88\x8A\x05\x03a\x8C\x97V[_\x80_\x83`\xFF\x03\x90P_a~\x9C\x82`\xFF\x16\x87\x90\x1B\x7F\x07\x06\x06\x05\x06\x02\x05\x04\x06\x02\x03\x02\x05\x04\x03\x01\x06\x05\x02\x05\x03\x03\x04\x01\x05\x05\x03\x04\0\0\0\0`\x1Fo\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x83\x15`\x08\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11`\x07\x1B\x17\x84\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x84\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x84\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x84\x81\x1C`\xFF\x10`\x03\x1B\x17\x93\x84\x1C\x1C\x16\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x93P\x83a~\xB1W_a~\xB8V[\x81`\xFF\x16\x81\x03[\x92PPP\x92P\x92\x90PV[_\x81`\xFF\x84\x16a~\xD9`\x01\x87\x90\x0Ba\x01\0a\x8B\xDEV[a~\xE3\x91\x90a\x8C\xD8V[aH9\x91\x90a\x8B\xDEV[_\x80[\x83Q\x81\x10\x15a\x7F=W\x83\x81\x81Q\x81\x10a\x7F\x0BWa\x7F\x0Ba\x87zV[` \x02` \x01\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x03a\x7F5W\x90Pa\x05\xA3V[`\x01\x01a~\xF0V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01R\x7FAsset not found\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[_b\x01\0\0\x82\x10aGZWaGZav\xECV[Q\x90Q`\x02\x91\x82\x0B\x91\x0B\x13\x90V[_\x80_a\x80B\x84`\xFF\x16\x86\x90\x1C~\x1F\r\x1E\x10\x0C\x1D\x07\x0F\t\x0B\x19\x13\x1C\x17\x06\x01\x0E\x11\x08\n\x1A\x14\x18\x02\x12\x1B\x15\x03\x16\x04\x05\x81\x19`\x01\x01\x90\x91\x16a\x01\xE0\x7F\x80@@UC\0RfD2\0\0P a\x06t\x050&\x02\0\0\x10u\x06 \x01v\x11pw`\xFC\x7F\xB6\xDBm\xB6\xDD\xDD\xDD\xDD\xD3M4\xD3I$\x92I!\x08B\x10\x8Cc\x18\xC69\xCEs\x9C\xFF\xFF\xFF\xFF\x84\x02`\xF8\x1C\x16\x1B`\xF7\x1C\x16\x90\x81\x1Cc\xD7dS\xE0\x04`\x1F\x16\x91\x90\x91\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x92P\x82a\x80XW`\xFFa\x80_V[\x83`\xFF\x16\x81\x01[\x91PP\x92P\x92\x90PV[_\x81\x81R`\x06` R`@\x81 _aH\x10`\x01`\x01`\xA0\x1B\x03\x86\x16`\x03\x84\x01a_;V[a\x18\xBC\x80a\x90v\x839\x01\x90V[`@Q\x80`@\x01`@R\x80_\x81R` \x01a\x80\xC7`@Q\x80`@\x01`@R\x80_\x81R` \x01``\x81RP\x90V[\x90R\x90V[_` \x82\x84\x03\x12\x15a\x80\xDCW_\x80\xFD[P5\x91\x90PV[_\x80`@\x83\x85\x03\x12\x15a\x80\xF4W_\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x81CW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x81\x1CV[P\x90\x95\x94PPPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x81CW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x81gV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01_[\x83\x81\x10\x15a\x82\x01W`\x1F\x19\x85\x84\x03\x01\x88Ra\x81\xEB\x83\x83Qa\x81\x85V[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01a\x81\xCFV[P\x90\x96\x95PPPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\x82\xA2W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x87\x86\x03\x01\x84R\x81Q`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x86R` \x81\x01Q\x90P`@` \x87\x01Ra\x82\x8C`@\x87\x01\x82a\x81\xB3V[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x823V[P\x92\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a8\x95W_\x80\xFD[_` \x82\x84\x03\x12\x15a\x82\xD2W_\x80\xFD[\x815a\x06#\x81a\x82\xAEV[\x80`\x02\x0B\x81\x14a8\x95W_\x80\xFD[_\x80_\x80_`\xA0\x86\x88\x03\x12\x15a\x82\xFFW_\x80\xFD[\x855\x94P` \x86\x015\x93P`@\x86\x015a\x83\x18\x81a\x82\xDDV[\x92P``\x86\x015a\x83(\x81a\x82\xDDV[\x94\x97\x93\x96P\x91\x94`\x80\x015\x92\x91PPV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a\x83\x8BW\x81Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a\x83KV[P\x93\x94\x93PPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\x82\xA2W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x87\x86\x03\x01\x84R\x81Q\x80Q`@\x87Ra\x83\xFF`@\x88\x01\x82a\x81\x85V[\x90P` \x82\x01Q\x91P\x86\x81\x03` \x88\x01Ra\x84\x1A\x81\x83a\x839V[\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x83\xBBV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a\x83\x8BW\x81Q\x80Q\x87R` \x81\x01Q` \x88\x01R`@\x81\x01Q`@\x88\x01RP``\x86\x01\x95P` \x82\x01\x91P`\x01\x81\x01\x90Pa\x84CV[` \x81R\x81Q`\x02\x0B` \x82\x01R` \x82\x01Q`\x02\x0B`@\x82\x01R_`@\x83\x01Qa\x84\xB3``\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x83\x01Q`\x80\x83\x01R`\x80\x83\x01Q`\xA0\x83\x01R`\xA0\x83\x01Q`\xC0\x83\x01R`\xC0\x83\x01Q`\xE0\x83\x01R`\xE0\x83\x01Qa\x01\0\x80\x84\x01RaH9a\x01 \x84\x01\x82a\x841V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x81CW\x83Q\x80Q`\x02\x0B\x84R` \x90\x81\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x85\x01R\x90\x93\x01\x92`@\x90\x92\x01\x91`\x01\x01a\x85\x0FV[` \x81R_a\x05\xA0` \x83\x01\x84a\x81\xB3V[_\x80_``\x84\x86\x03\x12\x15a\x85rW_\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\x82\xA2W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x87\x86\x03\x01\x84R\x81Q`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x86R` \x81\x01Q\x90P`@` \x87\x01Ra\x86\x08`@\x87\x01\x82a\x839V[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x85\xAFV[\x805b\xFF\xFF\xFF\x81\x16\x81\x14a`^W_\x80\xFD[_\x80_\x80_\x80`\xC0\x87\x89\x03\x12\x15a\x86EW_\x80\xFD[\x865\x95P` \x87\x015\x94P`@\x87\x015a\x86^\x81a\x82\xDDV[\x93Pa\x86l``\x88\x01a\x86\x1EV[\x92Pa\x86z`\x80\x88\x01a\x86\x1EV[\x91P`\xA0\x87\x015a\x86\x8A\x81a\x82\xAEV[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x86\xEEWa\x86\xEEa\x86\x98V[`@R\x91\x90PV[_\x80_\x83\x85\x03``\x81\x12\x15a\x87\tW_\x80\xFD[\x845\x93P` \x80\x86\x015\x93P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x82\x01\x12\x15a\x87BW_\x80\xFD[P`@Q` \x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x87fWa\x87fa\x86\x98V[`@\x90\x81R\x94\x90\x94\x015\x84RP\x90\x93\x90\x92PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x05\xA3Wa\x05\xA3a\x87\xA7V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x87\xFBW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x882W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[P\x91\x90PV[_` \x82\x84\x03\x12\x15a\x88HW_\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x88^W_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x88nW_\x80\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x88\x88Wa\x88\x88a\x86\x98V[a\x88\x9B` `\x1F\x19`\x1F\x84\x01\x16\x01a\x86\xC5V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\x88\xAFW_\x80\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[\x7Factor_\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_a\x05\xA0`\x06\x83\x01\x84a\x88\xCCV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R_aH9`@\x83\x01\x84a\x81\x85V[\x81\x81\x03\x81\x81\x11\x15a\x05\xA3Wa\x05\xA3a\x87\xA7V[_` \x82\x84\x03\x12\x15a\x89XW_\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x06#W_\x80\xFD[a\x01 \x81\x01a\x89\xC4\x82\x88`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x82R`\x01`\x01`\xA0\x1B\x03` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01R`\x01`\x01`\xA0\x1B\x03`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[\x85`\x02\x0B`\xA0\x83\x01R\x84`\x02\x0B`\xC0\x83\x01R\x83`\xE0\x83\x01R\x82a\x01\0\x83\x01R\x96\x95PPPPPPV[_\x80`@\x83\x85\x03\x12\x15a\x89\xFEW_\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x05\xA3Wa\x05\xA3a\x87\xA7V[_` \x82\x84\x03\x12\x15a\x8A6W_\x80\xFD[PQ\x91\x90PV[_\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x03a\x8AmWa\x8Ama\x87\xA7V[P_\x03\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x82a\x8A\xAEWa\x8A\xAEa\x8AsV[P\x04\x90V[_` \x82\x84\x03\x12\x15a\x8A\xC3W_\x80\xFD[\x81Qa\x06#\x81a\x82\xAEV[`\xC0\x81\x01a\x8B*\x82\x85`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x82R`\x01`\x01`\xA0\x1B\x03` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01R`\x01`\x01`\xA0\x1B\x03`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\xA0\x83\x01R\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x8BPW_\x80\xFD[\x81Qa\x06#\x81a\x82\xDDV[_aH\x10a\x8Bra\x8Bl\x84\x88a\x88\xCCV[\x86a\x88\xCCV[\x84a\x88\xCCV[` \x81R_a\x05\xA0` \x83\x01\x84a\x81\x85V[_\x81`\x02\x0B\x83`\x02\x0B\x80a\x8B\xA0Wa\x8B\xA0a\x8AsV[_\x19\x81\x14\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x83\x14\x16\x15a\x8B\xD5Wa\x8B\xD5a\x87\xA7V[\x90\x05\x93\x92PPPV[_\x82`\x02\x0B\x82`\x02\x0B\x02\x80`\x02\x0B\x91P\x80\x82\x14a6\xA6Wa6\xA6a\x87\xA7V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x05\xA3Wa\x05\xA3a\x87\xA7V[`@\x81R_a\x8C7`@\x83\x01\x85a\x81\x85V[\x82\x81\x03` \x84\x01RaH\x10\x81\x85a\x81\x85V[`@\x81R_a\x8C[`@\x83\x01\x85a\x81\x85V[\x90P\x82` \x83\x01R\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`1`\x04R`$_\xFD[`\x02\x82\x81\x0B\x90\x82\x90\x0B\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x81\x12b\x7F\xFF\xFF\x82\x13\x17\x15a\x05\xA3Wa\x05\xA3a\x87\xA7V[`\x02\x81\x81\x0B\x90\x83\x90\x0B\x01b\x7F\xFF\xFF\x81\x13\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x82\x12\x17\x15a\x05\xA3Wa\x05\xA3a\x87\xA7V[_a\x8D$\x82\x85a\x88\xCCV[\x7F, \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RaH\x10`\x02\x82\x01\x85a\x88\xCCV[_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x03a\x8D\x87Wa\x8D\x87a\x87\xA7V[`\x01\x01\x92\x91PPV[_a\x8D\x9B\x82\x84a\x88\xCCV[\x7F]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01\x01\x93\x92PPPV[_aH9a\x8Br\x83\x86a\x88\xCCV[\x7F\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81R_aH9`\x03\x83\x01\x84a\x88\xCCV[\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x16\x81R\x7F\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x16`\x01\x82\x01R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16`\x03\x82\x01R_a}\xA3`\x13\x83\x01\x84a\x88\xCCV[\x80\x82\x01\x82\x81\x12_\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a@\x1AWa@\x1Aa\x87\xA7V[_\x82a\x8E\xB9Wa\x8E\xB9a\x8AsV[P\x06\x90V[\x83\x81R\x82` \x82\x01R```@\x82\x01R_aH\x10``\x83\x01\x84a\x81\x85V[_\x81`\x02\x0B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x81\x03a\x8F\x10Wa\x8F\x10a\x87\xA7V[_\x19\x01\x92\x91PPV[_a\xFF\xFF\x82\x16a\xFF\xFF\x81\x03a\x8D\x87Wa\x8D\x87a\x87\xA7V[_a\x8F;\x82\x85a\x88\xCCV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x93\x90\x93\x16\x83RPP`\x10\x01\x91\x90PV[\x7F\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x16\x81R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x16`\x03\x82\x01R_a\x8F\xC7`\x13\x83\x01\x85a\x88\xCCV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x93\x90\x93\x16\x83RPP`\x14\x01\x93\x92PPPV[__\x19\x82\x03a\x90\x0BWa\x90\x0Ba\x87\xA7V[P`\x01\x01\x90V[`\x0F\x81\x81\x0B\x90\x83\x90\x0B\x01o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x13\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x12\x17\x15a\x05\xA3Wa\x05\xA3a\x87\xA7V[_\x81a\x90nWa\x90na\x87\xA7V[P_\x19\x01\x90V\xFE`\x80`@R4\x80\x15`\x0EW_\x80\xFD[P`@Qa\x18\xBC8\x03\x80a\x18\xBC\x839\x81\x01`@\x81\x90R`+\x91`NV[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`yV[_` \x82\x84\x03\x12\x15`]W_\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14`rW_\x80\xFD[\x93\x92PPPV[a\x186\x80a\0\x86_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0oW_5`\xE0\x1C\x80c\x91\xDDsF\x11a\0MW\x80c\x91\xDDsF\x14a\0\xD4W\x80c\xBA\xCA\0\x04\x14a\0\xF4W\x80c\xBE\xAB\xAC\xC8\x14a\x01\tW_\x80\xFD[\x80c\x04\x95\xA4\xA2\x14a\0sW\x80c\x0C\x86Xy\x14a\0\x99W\x80c@\xE2\xA8\x12\x14a\0\xC1W[_\x80\xFD[a\0\x86a\0\x816`\x04a\rnV[a\x01\x1CV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xACa\0\xA76`\x04a\r\xCDV[a\x02lV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\0\x90V[a\0\x86a\0\xCF6`\x04a\x0EfV[a\x03\xB8V[a\0\xE7a\0\xE26`\x04a\x0E\xE5V[a\x05\x0FV[`@Qa\0\x90\x91\x90a\x0FpV[a\x01\x07a\x01\x026`\x04a\x0F\x82V[a\x06\xBFV[\0[a\x01\x07a\x01\x176`\x04a\x0F\x9DV[a\x06\xE3V[_\x80T\x81\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cH\xC8\x94\x91\x82`\xF8\x1B\x88`@Q\x80``\x01`@R\x80\x8A\x15\x15\x81R` \x01\x89\x81R` \x01\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP`@Q` \x01a\x01\x85\x92\x91\x90a\x10\xBFV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x01\xC1\x92\x91` \x01a\x11\x06V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\xEC\x91\x90a\x0FpV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x02\x07W=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x02L\x91\x90\x81\x01\x90a\x12\tV[\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x02b\x91\x90a\x12\x83V[\x96\x95PPPPPPV[_\x80T\x81\x90\x81\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cH\xC8\x94\x91`\x02`\xF8\x1B\x8A`@Q\x80`\x80\x01`@R\x80\x8C`\x02\x0B\x81R` \x01\x8B`\x02\x0B\x81R` \x01\x8A\x81R` \x01\x89\x81RP`@Q` \x01a\x02\xCC\x92\x91\x90a\x12\x9AV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x03\x08\x92\x91` \x01a\x11\x06V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x033\x91\x90a\x0FpV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x03NW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x03\x93\x91\x90\x81\x01\x90a\x12\tV[\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x03\xA9\x91\x90a\x12\xDBV[\x92P\x92PP\x95P\x95\x93PPPPV[_\x80T\x81\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cH\xC8\x94\x91`\x01`\xF8\x1B\x8A`@Q\x80``\x01`@R\x80\x8C\x15\x15\x81R` \x01\x8B\x81R` \x01\x8As\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x88\x88`@Q` \x01a\x04&\x94\x93\x92\x91\x90a\x12\xFDV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x04b\x92\x91` \x01a\x11\x06V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\x8D\x91\x90a\x0FpV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x04\xA8W=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x04\xED\x91\x90\x81\x01\x90a\x12\tV[\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x05\x03\x91\x90a\x12\x83V[\x98\x97PPPPPPPPV[_T``\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x054W_\x80\xFD[_a\x05B`\x01\x82\x85\x87a\x13\x90V[a\x05K\x91a\x13\xB7V[`\xF8\x1C`\x02\x81\x11\x15a\x05_Wa\x05_a\x0F\xDBV[\x90P_\x81`\x02\x81\x11\x15a\x05tWa\x05ta\x0F\xDBV[\x03a\x05\xBCW_\x80a\x05\x88\x85`\x01\x81\x89a\x13\x90V[\x81\x01\x90a\x05\x95\x91\x90a\x15\x1BV[\x91P\x91Pa\x05\xB2\x82\x82`@Q\x80` \x01`@R\x80_\x81RPa\x07\tV[\x93PPPPa\x06\xB9V[`\x01\x81`\x02\x81\x11\x15a\x05\xD0Wa\x05\xD0a\x0F\xDBV[\x03a\x06\x0EW_\x80\x80a\x05\xE5\x86`\x01\x81\x8Aa\x13\x90V[\x81\x01\x90a\x05\xF2\x91\x90a\x15OV[\x92P\x92P\x92Pa\x06\x03\x83\x83\x83a\x07\tV[\x94PPPPPa\x06\xB9V[`\x02\x81`\x02\x81\x11\x15a\x06\"Wa\x06\"a\x0F\xDBV[\x03a\x06QW_\x80a\x066\x85`\x01\x81\x89a\x13\x90V[\x81\x01\x90a\x06C\x91\x90a\x15\xE8V[\x91P\x91Pa\x05\xB2\x82\x82a\x07\xD7V[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FUnrecognized action\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[P[\x92\x91PPV[a\x06\xDFs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x163a\x08\xFDV[PPV[a\x07\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x83\x83a\t\x7FV[PPPV[_\x80T`@Q\x7F\xF3\xCD\x91L\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R``\x92\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\xF3\xCD\x91L\x90a\x07e\x90\x88\x90\x88\x90\x88\x90`\x04\x01a\x16\x99V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x07\x81W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xA5\x91\x90a\x12\x83V[\x90Pa\x07\xB1\x85\x82a\t\xD2V[`@\x80Q` \x81\x01\x83\x90R\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP[\x93\x92PPPV[_\x80T`@Q\x7FZk\xCF\xDA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R``\x92\x91\x82\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90cZk\xCF\xDA\x90a\x085\x90\x88\x90\x88\x90`\x04\x01a\x17cV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x08PW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08t\x91\x90a\x12\xDBV[\x91P\x91Pa\x08\x8B\x85a\x08\x86\x84\x84a\t\xF9V[a\t\xD2V[_\x84`@\x01Q\x13a\x08\xD0W\x84Q_\x80T\x90\x91a\x08\xBF\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x900\x90a\nHV[\x90Pa\x08\xCE\x86_\x01Q\x82a\n\xE4V[P[`@\x80Q` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92PPP\x92\x91PPV[_cp\xA0\x821_R0` R` `4`$`\x1C\x86Z\xFA`\x1F=\x11\x16a\t*Wc\x90\xB8\xEC\x18_R`\x04`\x1C\xFD[\x81`\x14R`4Q\x90Po\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0_R` _`D`\x10_\x87Z\xF1\x80`\x01_Q\x14\x16a\ttW\x80=\x85;\x15\x17\x10a\ttWc\x90\xB8\xEC\x18_R`\x04`\x1C\xFD[P_`4R\x92\x91PPV[\x81`\x14R\x80`4Ro\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0_R` _`D`\x10_\x87Z\xF1\x80`\x01_Q\x14\x16a\t\xC8W\x80=\x85;\x15\x17\x10a\t\xC8Wc\x90\xB8\xEC\x18_R`\x04`\x1C\xFD[P_`4RPPPV[\x81Qa\t\xE7\x90a\t\xE2\x83`\x80\x1D\x90V[a\n\xE4V[a\x06\xDF\x82` \x01Qa\t\xE2\x83`\x0F\x0B\x90V[_`\x80\x82\x81\x1D\x90\x84\x90\x1D\x01`\x0F\x83\x81\x0B\x90\x85\x90\x0B\x01a\n?a\n\x1A\x83a\x0C\xE3V[a\n#\x83a\x0C\xE3V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x91\x90\x91\x1B\x17\x90V[\x95\x94PPPPPV[_\x82\x81R` \x82\x90R`@\x80\x82 \x90Q\x7F\xF15\xBA\xAA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R\x82\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x90c\xF15\xBA\xAA\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xC0W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02b\x91\x90a\x12\x83V[_\x81`\x0F\x0B\x12\x15a\x0C7W_T`@Q\x7F\xA5\x84\x11\x94\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\xA5\x84\x11\x94\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0BXW_\x80\xFD[PZ\xF1\x15\x80\x15a\x0BjW=_\x80>=_\xFD[PP_\x80Ta\x0B\xAA\x93Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x81\x16\x93P\x16\x90\x84\x90\x03o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\t\x7FV[_\x80T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x11\xDA`\xB4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0C\x13W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x04\x91\x90a\x12\x83V[\x80`\x0F\x0B_\x12\x15a\x06\xDFW_T`@Q\x7F\x0B\r\x9C\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16`\x04\x83\x01R0`$\x83\x01Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`D\x83\x01R\x90\x91\x16\x90c\x0B\r\x9C\t\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0C\xC9W_\x80\xFD[PZ\xF1\x15\x80\x15a\x0C\xDBW=_\x80>=_\xFD[PPPPPPV[\x80`\x0F\x81\x90\x0B\x81\x14a\r\x18Wa\r\x18\x7F\x93\xDA\xFD\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\r\x1DV[\x91\x90PV[\x80_R`\x04_\xFD[_`\xA0\x82\x84\x03\x12\x15a\r5W_\x80\xFD[P\x91\x90PV[\x805\x80\x15\x15\x81\x14a\r\x18W_\x80\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\rkW_\x80\xFD[PV[_\x80_\x80a\x01\0\x85\x87\x03\x12\x15a\r\x82W_\x80\xFD[a\r\x8C\x86\x86a\r%V[\x93Pa\r\x9A`\xA0\x86\x01a\r;V[\x92P`\xC0\x85\x015\x91P`\xE0\x85\x015a\r\xB1\x81a\rJV[\x93\x96\x92\x95P\x90\x93PPV[\x805`\x02\x81\x90\x0B\x81\x14a\r\x18W_\x80\xFD[_\x80_\x80_a\x01 \x86\x88\x03\x12\x15a\r\xE2W_\x80\xFD[a\r\xEC\x87\x87a\r%V[\x94Pa\r\xFA`\xA0\x87\x01a\r\xBCV[\x93Pa\x0E\x08`\xC0\x87\x01a\r\xBCV[\x94\x97\x93\x96P\x93\x94`\xE0\x81\x015\x94Pa\x01\0\x015\x92\x91PPV[_\x80\x83`\x1F\x84\x01\x12a\x0E1W_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0EHW_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x0E_W_\x80\xFD[\x92P\x92\x90PV[_\x80_\x80_\x80a\x01 \x87\x89\x03\x12\x15a\x0E|W_\x80\xFD[a\x0E\x86\x88\x88a\r%V[\x95Pa\x0E\x94`\xA0\x88\x01a\r;V[\x94P`\xC0\x87\x015\x93P`\xE0\x87\x015a\x0E\xAB\x81a\rJV[\x92Pa\x01\0\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\xC7W_\x80\xFD[a\x0E\xD3\x89\x82\x8A\x01a\x0E!V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[_\x80` \x83\x85\x03\x12\x15a\x0E\xF6W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\x0CW_\x80\xFD[a\x0F\x18\x85\x82\x86\x01a\x0E!V[\x90\x96\x90\x95P\x93PPPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x07\xD0` \x83\x01\x84a\x0F$V[_` \x82\x84\x03\x12\x15a\x0F\x92W_\x80\xFD[\x815a\x07\xD0\x81a\rJV[_\x80_``\x84\x86\x03\x12\x15a\x0F\xAFW_\x80\xFD[\x835a\x0F\xBA\x81a\rJV[\x92P` \x84\x015a\x0F\xCA\x81a\rJV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[\x805b\xFF\xFF\xFF\x81\x16\x81\x14a\r\x18W_\x80\xFD[\x805a\x10%\x81a\rJV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x81\x015a\x10K\x81a\rJV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16` \x83\x01Rb\xFF\xFF\xFFa\x10v`@\x83\x01a\x10\x08V[\x16`@\x83\x01Ra\x10\x88``\x82\x01a\r\xBCV[`\x02\x0B``\x83\x01R`\x80\x81\x015a\x10\x9E\x81a\rJV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\x80\x84\x01RPPPV[a\x01\0\x81\x01a\x10\xCE\x82\x85a\x10\x1AV[\x82Q\x15\x15`\xA0\x83\x01R` \x83\x01Q`\xC0\x83\x01R`@\x83\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x83\x01Ra\x07\xD0V[\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81R_\x82Q\x80` \x85\x01`\x01\x85\x01^_\x92\x01`\x01\x01\x91\x82RP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x11\xBCWa\x11\xBCa\x11HV[`@R\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x11\xDDWa\x11\xDDa\x11HV[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01\x90V[_` \x82\x84\x03\x12\x15a\x12\x19W_\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12/W_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x12?W_\x80\xFD[\x80Qa\x12Ra\x12M\x82a\x11\xC4V[a\x11uV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\x12fW_\x80\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a\x12\x93W_\x80\xFD[PQ\x91\x90PV[a\x01 \x81\x01a\x12\xA9\x82\x85a\x10\x1AV[\x82Q`\x02\x90\x81\x0B`\xA0\x84\x01R` \x84\x01Q\x90\x0B`\xC0\x83\x01R`@\x83\x01Q`\xE0\x83\x01R``\x83\x01Qa\x01\0\x83\x01Ra\x07\xD0V[_\x80`@\x83\x85\x03\x12\x15a\x12\xECW_\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[a\x13\x07\x81\x86a\x10\x1AV[\x83Q\x15\x15`\xA0\x82\x01R` \x84\x01Q`\xC0\x82\x01R`@\x84\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x82\x01Ra\x01 a\x01\0\x82\x01R\x81a\x01 \x82\x01R\x81\x83a\x01@\x83\x017_\x81\x83\x01a\x01@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x01\x01\x93\x92PPPV[_\x80\x85\x85\x11\x15a\x13\x9EW_\x80\xFD[\x83\x86\x11\x15a\x13\xAAW_\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x805\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90`\x01\x84\x10\x15a\x06\xB7W\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x85`\x01\x03`\x03\x1B\x1B\x82\x16\x16\x91PP\x92\x91PPV[_`\xA0\x82\x84\x03\x12\x15a\x14,W_\x80\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x14OWa\x14Oa\x11HV[`@R\x90P\x80\x825a\x14`\x81a\rJV[\x81R` \x83\x015a\x14p\x81a\rJV[` \x82\x01Ra\x14\x81`@\x84\x01a\x10\x08V[`@\x82\x01Ra\x14\x92``\x84\x01a\r\xBCV[``\x82\x01R`\x80\x83\x015a\x14\xA5\x81a\rJV[`\x80\x91\x90\x91\x01R\x92\x91PPV[_``\x82\x84\x03\x12\x15a\x14\xC2W_\x80\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x14\xE5Wa\x14\xE5a\x11HV[`@R\x90P\x80a\x14\xF4\x83a\r;V[\x81R` \x83\x81\x015\x90\x82\x01R`@\x83\x015a\x15\x0E\x81a\rJV[`@\x91\x90\x91\x01R\x92\x91PPV[_\x80a\x01\0\x83\x85\x03\x12\x15a\x15-W_\x80\xFD[a\x157\x84\x84a\x14\x1CV[\x91Pa\x15F\x84`\xA0\x85\x01a\x14\xB2V[\x90P\x92P\x92\x90PV[_\x80_a\x01 \x84\x86\x03\x12\x15a\x15bW_\x80\xFD[a\x15l\x85\x85a\x14\x1CV[\x92Pa\x15{\x85`\xA0\x86\x01a\x14\xB2V[\x91Pa\x01\0\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\x97W_\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x15\xA7W_\x80\xFD[\x805a\x15\xB5a\x12M\x82a\x11\xC4V[\x81\x81R\x87` \x83\x85\x01\x01\x11\x15a\x15\xC9W_\x80\xFD[\x81` \x84\x01` \x83\x017_` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92P\x92V[_\x80\x82\x84\x03a\x01 \x81\x12\x15a\x15\xFBW_\x80\xFD[a\x16\x05\x85\x85a\x14\x1CV[\x92P`\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x82\x01\x12\x15a\x166W_\x80\xFD[P`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x16ZWa\x16Za\x11HV[`@Ra\x16i`\xA0\x85\x01a\r\xBCV[\x81Ra\x16w`\xC0\x85\x01a\r\xBCV[` \x82\x01R`\xE0\x84\x015`@\x82\x01Ra\x01\0\x90\x93\x015``\x84\x01RP\x92\x90\x91PV[a\x17\x18\x81\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[\x82Q\x15\x15`\xA0\x82\x01R` \x83\x01Q`\xC0\x82\x01R`@\x83\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x82\x01Ra\x01 a\x01\0\x82\x01R_a\n?a\x01 \x83\x01\x84a\x0F$V[a\x17\xE2\x81\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[\x81Q`\x02\x90\x81\x0B`\xA0\x83\x01R` \x83\x01Q\x90\x0B`\xC0\x82\x01R`@\x82\x01Q`\xE0\x82\x01R``\x82\x01Qa\x01\0\x82\x01Ra\x01@a\x01 \x82\x01\x81\x90R_\x90\x82\x01Ra\x01`\x01\x92\x91PPV\xFE\xA1dsolcC\0\x08\x1A\0\nWARNING\nWARNING: Above somehow called with donate to current only???\xA1dsolcC\0\x08\x1A\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b5060043610610235575f3560e01c8063857620e11161013d578063b0464fdc116100b8578063ba414fa611610088578063e20c9f711161006e578063e20c9f71146104d6578063eee8e67b146104de578063fa7626d4146104f1575f80fd5b8063ba414fa6146104be578063cc7f0c2414610493575f80fd5b8063b0464fdc1461049b578063b15a7971146104a3578063b165c9e914610480578063b5508aa9146104b6575f80fd5b8063916a17c61161010d578063ab3c7e52116100f3578063ab3c7e5214610478578063aceb0e8514610480578063aeb8fbf914610493575f80fd5b8063916a17c61461044457806393db45e014610459575f80fd5b8063857620e1146103ec57806388bdca61146103ff5780638985c90b146104125780638f5d23ce14610425575f80fd5b80635ee481b6116101cd57806376e1fcc41161019d5780638068b52e116101835780638068b52e1461039957806382716e43146103b857806385226c81146103d7575f80fd5b806376e1fcc4146103665780637b2abdb614610379575f80fd5b80635ee481b6146102fd57806364239cdd1461031c57806366d9a9a0146103315780637477f51714610346575f80fd5b80632ade3880116102085780632ade3880146102d05780633e5e3c23146102e55780633f7286f4146102ed578063478ddecc146102f5575f80fd5b8063068bcd8d146102395780630d5ec4c61461027a5780631ed7831c1461029b5780632895a2b3146102b0575b5f80fd5b61024c6102473660046180cc565b6104fe565b604080516001600160a01b03948516815293909216602084015260020b908201526060015b60405180910390f35b61028d6102883660046180e3565b610595565b604051908152602001610271565b6102a36105a9565b6040516102719190618103565b6102c36102be3660046180cc565b610609565b604051610271919061814e565b6102d861062a565b604051610271919061820d565b6102a3610766565b6102a36107c4565b6102a3610822565b61028d61030b3660046182c2565b602a6020525f908152604090205481565b61032f61032a3660046182eb565b610833565b005b6103396112fc565b6040516102719190618395565b6103596103543660046180e3565b611475565b604051610271919061847d565b61028d6103743660046180e3565b6115bc565b61038c6103873660046180cc565b6115c7565b60405161027191906184f6565b61028d6103a73660046182c2565b602c6020525f908152604090205481565b61028d6103c63660046182c2565b602d6020525f908152604090205481565b6103df611656565b604051610271919061854e565b61032f6103fa366004618560565b611721565b61028d61040d3660046180cc565b611f9d565b61028d6104203660046180e3565b611ff7565b61028d6104333660046182c2565b602b6020525f908152604090205481565b61044c612002565b6040516102719190618589565b61028d6104673660046182c2565b60296020525f908152604090205481565b602e5461028d565b61028d61048e3660046180e3565b6120f8565b6102a3612103565b61044c61210f565b61032f6104b1366004618630565b612205565b6103df6127cc565b6104c6612897565b6040519015158152602001610271565b6102a3612967565b61032f6104ec3660046186f6565b6129c5565b601f546104c69060ff1681565b5f805f80602e85815481106105155761051561877a565b905f5260205f20906003020190506020600501815f01548154811061053c5761053c61877a565b5f918252602090912001546001820154602580546001600160a01b03909316965091811061056c5761056c61877a565b5f9182526020909120015460029182015494966001600160a01b03909116955093900b92915050565b5f6105a082846187d4565b90505b92915050565b606060168054806020026020016040519081016040528092919081815260200182805480156105ff57602002820191905f5260205f20905b81546001600160a01b031681526001909101906020018083116105e1575b5050505050905090565b5f8181526033602052604081206060919061062390613410565b9392505050565b6060601e805480602002602001604051908101604052809291908181526020015f905b8282101561075d575f84815260208082206040805180820182526002870290920180546001600160a01b03168352600181018054835181870281018701909452808452939591948681019491929084015b82821015610746578382905f5260205f200180546106bb906187e7565b80601f01602080910402602001604051908101604052809291908181526020018280546106e7906187e7565b80156107325780601f1061070957610100808354040283529160200191610732565b820191905f5260205f20905b81548152906001019060200180831161071557829003601f168201915b50505050508152602001906001019061069e565b50505050815250508152602001906001019061064d565b50505050905090565b606060188054806020026020016040519081016040528092919081815260200182805480156105ff57602002820191905f5260205f209081546001600160a01b031681526001909101906020018083116105e1575050505050905090565b606060178054806020026020016040519081016040528092919081815260200182805480156105ff57602002820191905f5260205f209081546001600160a01b031681526001909101906020018083116105e1575050505050905090565b606061082e60286134c9565b905090565b835f61083f602761359d565b905061084c825f836135ee565b9150808210156108a95761086160278361363b565b603580546001600160a01b03929092167fffffffffffffffffffffffff0000000000000000000000000000000000000000928316811790915560368054909216179055610a4d565b6023546040516001600160a01b03909116906108c49061808d565b6001600160a01b039091168152602001604051809103905ff0801580156108ed573d5f803e3d5ffd5b50603680547fffffffffffffffffffffffff00000000000000000000000000000000000000009081166001600160a01b039390931692831790915560358054909116821790556040517f6900a3ae000000000000000000000000000000000000000000000000000000008152600184016004820152737109709ecfa91a80626ff3989d68f67f5b1dd12d9163c657c718918390636900a3ae906024015f60405180830381865afa1580156109a3573d5f803e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526109ca9190810190618838565b6040516020016109da91906188e3565b6040516020818303038152906040526040518363ffffffff1660e01b8152600401610a06929190618914565b5f604051808303815f87803b158015610a1d575f80fd5b505af1158015610a2f573d5f803e3d5ffd5b5050603554610a4b9250602791506001600160a01b03166136ad565b505b50610a6a865f6001602e80549050610a659190618935565b6135ee565b95505f602e8781548110610a8057610a8061877a565b5f918252602082206002600390920201818101549093508291610aa491900b613898565b91509150610ae5836002015f9054906101000a900460020b610ad08960020b8560020b8560020b6138f0565b60020b905f8183071291819005919091030290565b9650610b0f836002015f9054906101000a900460020b610ad08860020b8560020b8560020b6138f0565b6040517f4c63e562000000000000000000000000000000000000000000000000000000008152600282810b908a900b14156004820152909650737109709ecfa91a80626ff3989d68f67f5b1dd12d90634c63e562906024015f6040518083038186803b158015610b7d575f80fd5b505afa158015610b8f573d5f803e3d5ffd5b505050508660020b8660020b1215610ba5579495945b50506024548154602580545f93610c26936001600160a01b0390911692918110610bd157610bd161877a565b5f918252602090912001546001850154602580546001600160a01b03909316929091908110610c0257610c0261877a565b5f918252602090912001546002808701546001600160a01b0390921691900b6139c7565b90505f610c348260a0902090565b90505f805f610c45848b8b89613a24565b6040517f4c63e5620000000000000000000000000000000000000000000000000000000081526fffffffffffffffffffffffffffffffff8416151560048201529295509093509150737109709ecfa91a80626ff3989d68f67f5b1dd12d90634c63e562906024015f6040518083038186803b158015610cc2575f80fd5b505afa158015610cd4573d5f803e3d5ffd5b50505050610cf6886001856fffffffffffffffffffffffffffffffff166135ee565b600280880154602354929a50610d1a926001600160a01b03169187918e910b613d4a565b610d35575f848152603060205260409020610d35908b613da0565b600280870154602354610d59926001600160a01b039091169187918d91900b613d4a565b610d74575f848152603060205260409020610d74908a613da0565b8554602680545f92908110610d8b57610d8b61877a565b5f9182526020822001546001890154602680546001600160a01b039093169450918110610dba57610dba61877a565b5f918252602090912001546035546040517fa9059cbb0000000000000000000000000000000000000000000000000000000081526001600160a01b03918216600482015260248101879052918116925083169063a9059cbb906044016020604051808303815f875af1158015610e32573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e569190618948565b506035546040517fa9059cbb0000000000000000000000000000000000000000000000000000000081526001600160a01b039182166004820152602481018590529082169063a9059cbb906044016020604051808303815f875af1158015610ec0573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ee49190618948565b506036546002898101546040805160a0810182525f91810182905260808101919091526001600160a01b03868116825285811660208301529190920b606083015290911690630c865879908e8e8e5f801b6040518663ffffffff1660e01b8152600401610f55959493929190618967565b60408051808303815f875af1158015610f70573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f9491906189ed565b50508754602580545f94509092508110610fb057610fb061877a565b5f9182526020822001546001890154602580546001600160a01b039093169450918110610fdf57610fdf61877a565b5f918252602090912001546035546040517fa9059cbb0000000000000000000000000000000000000000000000000000000081526001600160a01b03918216600482015260248101879052918116925083169063a9059cbb906044016020604051808303815f875af1158015611057573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061107b9190618948565b506035546040517fa9059cbb0000000000000000000000000000000000000000000000000000000081526001600160a01b039182166004820152602481018590529082169063a9059cbb906044016020604051808303815f875af11580156110e5573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111099190618948565b5060365f9054906101000a90046001600160a01b03166001600160a01b0316630c865879888e8e8e5f801b6040518663ffffffff1660e01b8152600401611154959493929190618967565b60408051808303815f875af115801561116f573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061119391906189ed565b50506036545f97506111b996508c95508b94506001600160a01b03169250613ebd915050565b90505f8881526034602052604090206111d29082613f0c565b505f88815260326020908152604080832084845282528083208b845260339092529091206112009083613f0c565b1561127f57805462ffffff8781166301000000027fffffffffffffffffffffffffffffffffffffffffffffffffffff000000000000909216908916171780825560365466010000000000006001600160a01b03909116027fffffffffffff0000000000000000000000000000000000000000ffffffffffff9091161781555b84816003015f82825461129291906187d4565b9091555050604080516060810182529586525f998a5260316020908152818b20548188019081525f199288019283526005909301805460018181018355918d5291909b20965160039091029096019586559051988501989098555050945160029091015550505050565b6060601b805480602002602001604051908101604052809291908181526020015f905b8282101561075d578382905f5260205f2090600202016040518060400160405290815f8201805461134f906187e7565b80601f016020809104026020016040519081016040528092919081815260200182805461137b906187e7565b80156113c65780601f1061139d576101008083540402835291602001916113c6565b820191905f5260205f20905b8154815290600101906020018083116113a957829003601f168201915b505050505081526020016001820180548060200260200160405190810160405280929190818152602001828054801561145d57602002820191905f5260205f20905f905b82829054906101000a900460e01b7bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19168152602001906004019060208260030104928301926001038202915080841161140a5790505b5050505050815250508152602001906001019061131f565b6114c56040518061010001604052805f60020b81526020015f60020b81526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f8152602001606081525090565b5f83815260326020908152604080832085845282528083208151610100810183528154600281810b835263010000008204810b8387015266010000000000009091046001600160a01b031682850152600183015460608301528201546080820152600382015460a0820152600482015460c08201526005820180548451818702810187019095528085529195929460e0870194939192919084015b828210156115ad578382905f5260205f2090600302016040518060600160405290815f82015481526020016001820154815260200160028201548152505081526020019060010190611560565b50505091525090949350505050565b5f6105a08284618a0f565b606060315f8381526020019081526020015f20805480602002602001604051908101604052809291908181526020015f905b8282101561164b575f8481526020908190206040805180820190915290840154600281900b8252630100000090046fffffffffffffffffffffffffffffffff16818301528252600190920191016115f9565b505050509050919050565b6060601a805480602002602001604051908101604052809291908181526020015f905b8282101561075d578382905f5260205f20018054611696906187e7565b80601f01602080910402602001604051908101604052809291908181526020018280546116c2906187e7565b801561170d5780601f106116e45761010080835404028352916020019161170d565b820191905f5260205f20905b8154815290600101906020018083116116f057829003601f168201915b505050505081526020019060010190611679565b611738835f6001602e80549050610a659190618935565b5f8181526034602052604081209194509061175290614022565b6040517f4c63e5620000000000000000000000000000000000000000000000000000000081528115156004820152909150737109709ecfa91a80626ff3989d68f67f5b1dd12d90634c63e562906024015f6040518083038186803b1580156117b8575f80fd5b505afa1580156117ca573d5f803e3d5ffd5b505050505f6117f76117e4855f600186610a659190618935565b5f87815260346020526040902090614070565b5f86815260326020908152604080832084845290915281206003810154929350916118239186916135ee565b93505f602e87815481106118395761183961877a565b5f91825260208220602454600390920201805460258054929550611872936001600160a01b0316929091908110610bd157610bd161877a565b6024549091505f906001600160a01b031663d89411446118938460a0902090565b865460405160e084901b7fffffffff000000000000000000000000000000000000000000000000000000001681526004810192909252660100000000000081046001600160a01b03166024830152600281810b60448401526301000000909104900b60648201525f608482015260a401602060405180830381865afa15801561191e573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119429190618a26565b905080602b5f6020600501865f0154815481106119615761196161877a565b5f9182526020808320909101546001600160a01b03168352820192909252604001812080549091906119949084906187d4565b9250508190555080846002015f8282546119ae91906187d4565b909155505083546001600160a01b03660100000000000082041690630c865879908490600281810b9163010000009004900b6119e98c618a3d565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e087901b168152611a2794939291905f90600401618967565b60408051808303815f875af1158015611a42573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611a6691906189ed565b5050835483546025805466010000000000009093046001600160a01b03169263baca000492908110611a9a57611a9a61877a565b5f9182526020909120015460405160e083901b7fffffffff000000000000000000000000000000000000000000000000000000001681526001600160a01b0390911660048201526024015f604051808303815f87803b158015611afb575f80fd5b505af1158015611b0d573d5f803e3d5ffd5b5050855460018601546025805466010000000000009093046001600160a01b0316945063baca00049350918110611b4657611b4661877a565b5f9182526020909120015460405160e083901b7fffffffff000000000000000000000000000000000000000000000000000000001681526001600160a01b0390911660048201526024015f604051808303815f87803b158015611ba7575f80fd5b505af1158015611bb9573d5f803e3d5ffd5b50508454602680545f94509092508110611bd557611bd561877a565b5f9182526020822001546001860154602680546001600160a01b039093169450918110611c0457611c0461877a565b5f91825260208083209091015488546002898101546040805160a08101825290810187905260808101969096526001600160a01b038881168752938416948601859052900b606085015291935066010000000000009091041690630c865879908854600281810b9163010000009004900b611c7e8e618a3d565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e087901b168152611cbc94939291905f90600401618967565b60408051808303815f875af1158015611cd7573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611cfb91906189ed565b505085546040517fbaca00040000000000000000000000000000000000000000000000000000000081526001600160a01b03848116600483015266010000000000009092049091169063baca0004906024015f604051808303815f87803b158015611d64575f80fd5b505af1158015611d76573d5f803e3d5ffd5b505087546040517fbaca00040000000000000000000000000000000000000000000000000000000081526001600160a01b0385811660048301526601000000000000909204909116925063baca000491506024015f604051808303815f87803b158015611de1575f80fd5b505af1158015611df3573d5f803e3d5ffd5b5050505050505f611e058360a0902090565b8554600286810154602354939450611e2e936001600160a01b031692859290810b91900b613d4a565b611e4e575f8181526030602052604090208554611e4e919060020b6140d2565b8454600285810154602354611e7d936001600160a01b039091169285926301000000909204810b91900b613d4a565b611ea4575f8181526030602052604090208554611ea491906301000000900460020b6140d2565b5050505083816003015f828254611ebb9190618935565b90915550505f8681526031602052604090205460048201546005830191905b8254811015611f115781838281548110611ef657611ef661877a565b5f918252602090912060026003909202010155600101611eda565b5081546004840155600383015415611f7a5760408051606081018252600380860154825260208083018581525f19948401948552600588018054600181810183555f92835293909120945193029093019182559151918101919091559051600290910155611f93565b5f888152603460205260409020611f919085614204565b505b5050505050505050565b5f80602e8381548110611fb257611fb261877a565b5f918252602090912060245460039092020180546025805492945061062393611ff0936001600160a01b0390911692908110610bd157610bd161877a565b60a0902090565b5f6105a08284618935565b6060601d805480602002602001604051908101604052809291908181526020015f905b8282101561075d575f8481526020908190206040805180820182526002860290920180546001600160a01b031683526001810180548351818702810187019094528084529394919385830193928301828280156120e057602002820191905f5260205f20905f905b82829054906101000a900460e01b7bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19168152602001906004019060208260030104928301926001038202915080841161208d5790505b50505050508152505081526020019060010190612025565b5f6105a08284618aa0565b606061082e60276134c9565b6060601c805480602002602001604051908101604052809291908181526020015f905b8282101561075d575f8481526020908190206040805180820182526002860290920180546001600160a01b031683526001810180548351818702810187019094528084529394919385830193928301828280156121ed57602002820191905f5260205f20905f905b82829054906101000a900460e01b7bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19168152602001906004019060208260030104928301926001038202915080841161219a5790505b50505050508152505081526020019060010190612132565b60255461221c9087905f90610a6590600190618935565b6025549096506122369086905f90610a6590600190618935565b94506122418361430a565b925061224c8261430a565b915061227b6001600160a01b0382166401000276a373fffd8963efd1fc6a506488495d951d5263988d266135ee565b905084860361229c57602554600187018161229857612298618a73565b0694505b848611156122a8579394935b6122ba600285900b6001617fff6138f0565b5f878152602f602090815260408083208984529091529020549094506122e29060ff16614318565b5f868152602f60209081526040808320888452825280832080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0016600117905560245481517f9d69dc4e00000000000000000000000000000000000000000000000000000000815291516123b4936001600160a01b0390921692639d69dc4e9260048281019391928290030181865afa158015612382573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906123a69190618ab3565b6001600160a01b0316614394565b90505f602060050188815481106123cd576123cd61877a565b5f918252602082200154602680546001600160a01b039092169350908a9081106123f9576123f961877a565b5f918252602082200154602580546001600160a01b039092169350908a9081106124255761242561877a565b5f918252602082200154602680546001600160a01b039092169350908b9081106124515761245161877a565b5f918252602090912001546021546040517fca669fa70000000000000000000000000000000000000000000000000000000081526001600160a01b03918216600482015291169150737109709ecfa91a80626ff3989d68f67f5b1dd12d9063ca669fa7906024015f604051808303815f87803b1580156124cf575f80fd5b505af11580156124e1573d5f803e3d5ffd5b5050602480546040517f138714650000000000000000000000000000000000000000000000000000000081526001600160a01b0389811660048301528781169382019390935261ffff8e16604482015262ffffff808e1660648301528c1660848201525f60a4820152911692506313871465915060c4015f604051808303815f87803b15801561256f575f80fd5b505af1158015612581573d5f803e3d5ffd5b505050506125998460286136ad90919063ffffffff16565b506125a56028836136ad565b50602480546040517f8587f4500000000000000000000000000000000000000000000000000000000081526001600160a01b03878116600483015285811693820193909352604481018890528883166064820152911690638587f450906084015f604051808303815f87803b15801561261c575f80fd5b505af115801561262e573d5f803e3d5ffd5b50506023546040805160a0810182525f81830181905260808201526001600160a01b038881168252868116602083015260028f900b606083015291517f6276cbbe000000000000000000000000000000000000000000000000000000008152919092169350636276cbbe92506126a991908a90600401618ace565b6020604051808303815f875af11580156126c5573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906126e99190618b40565b5050604080516060810182529a8b5260208b01998a5260029890980b978a019788525050602e80546001810182555f9190915297517f37fa166cbdbfbb1561ccd9ea985ec0218b5e68502e230525f544285b2bdf3d7e600390990298890155505093517f37fa166cbdbfbb1561ccd9ea985ec0218b5e68502e230525f544285b2bdf3d7f8601555050517f37fa166cbdbfbb1561ccd9ea985ec0218b5e68502e230525f544285b2bdf3d8090920180547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000001662ffffff9093169290921790915550565b60606019805480602002602001604051908101604052809291908181526020015f905b8282101561075d578382905f5260205f2001805461280c906187e7565b80601f0160208091040260200160405190810160405280929190818152602001828054612838906187e7565b80156128835780601f1061285a57610100808354040283529160200191612883565b820191905f5260205f20905b81548152906001019060200180831161286657829003601f168201915b5050505050815260200190600101906127ef565b6008545f9060ff16156128ae575060085460ff1690565b6040517f667f9d70000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d600482018190527f6661696c6564000000000000000000000000000000000000000000000000000060248301525f9163667f9d7090604401602060405180830381865afa15801561293c573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906129609190618a26565b1415905090565b606060158054806020026020016040519081016040528092919081815260200182805480156105ff57602002820191905f5260205f209081546001600160a01b031681526001909101906020018083116105e1575050505050905090565b6129dc835f6001602e80549050610a659190618935565b9250826038819055505f602e84815481106129f9576129f961877a565b5f91825260208220602454600390920201805460258054929550612a8b93611ff0936001600160a01b0390911692908110612a3657612a3661877a565b5f918252602090912001546001860154602580546001600160a01b03909316929091908110612a6757612a6761877a565b5f918252602090912001546002808801546001600160a01b0390921691900b6139c7565b6002808401549192505f91612aa3918491900b6143aa565b8051909150612ab3865f836135ee565b9550612abd61809a565b612ad482612acc600482618aa0565b839190614572565b5f8767ffffffffffffffff811115612aee57612aee618698565b604051908082528060200260200182016040528015612b3257816020015b604080518082019091525f8082526020820152815260200190600190039081612b0c5790505b5090505f805b89811015612ebd575f86612b4c8b8761458b565b81518110612b5c57612b5c61877a565b602002602001015190505f612bb867016345785d8a0000612b8e670de0b6b3a76400008e6145a590919063ffffffff16565b1115612bb257612bad8c60016cffffffffffffffffffffffffff61460a565b614707565b5f614707565b905060405180604001604052808360020b8152602001826fffffffffffffffffffffffffffffffff16815250858481518110612bf657612bf661877a565b6020908102919091010152612c1d6fffffffffffffffffffffffffffffffff8216856187d4565b93505f8d81526031602052604090208551869085908110612c4057612c4061877a565b60209081029190910181015182546001810184555f938452828420825191018054928401516fffffffffffffffffffffffffffffffff166301000000027fffffffffffffffffffffffffff0000000000000000000000000000000000000090931662ffffff90921691909117919091179055603782905560385482526034905260408120612ccd90614022565b90505f805b82811015612d8f576038545f908152603460205260408120612cf49083614070565b6038545f9081526032602090815260408083208484529091529020805491925090600288810b91900b13801590612d3a5750805463010000009004600290810b9088900b125b15612d85576003810154612d4e90856187d4565b603780546001810182555f919091527f42a7b7dd785cd69714a189dffb3fd7d7174edc9ece837694ce50f7078f7c31ae0183905593505b5050600101612cd2565b50805f03612dff57826fffffffffffffffffffffffffffffffff16602c5f60206005018f5f015481548110612dc657612dc661877a565b5f9182526020808320909101546001600160a01b0316835282019290925260400181208054909190612df99084906187d4565b90915550505b60375491505f5b82811015612eac575f60378281548110612e2257612e2261877a565b5f91825260208083209091015460385483526032825260408084208285529092529120600381015491925090612e879085612e806fffffffffffffffffffffffffffffffff8a16700100000000000000000000000000000000618a0f565b919061475e565b816001015f828254612e9991906187d4565b909155505060019092019150612e069050565b505060019093019250612b38915050565b506023546002808901545f92612ee39286926001600160a01b03909216918b910b6147ec565b90505f6020600501895f015481548110612eff57612eff61877a565b5f91825260208220015460018b0154602580546001600160a01b039093169450918110612f2e57612f2e61877a565b5f9182526020808320909101546001600160a01b038581168452602a90925260408320805492909116935086929091612f689084906187d4565b90915550508954602580545f92908110612f8457612f8461877a565b5f918252602090912001546040517f40c10f19000000000000000000000000000000000000000000000000000000008152306004820152602481018790526001600160a01b03909116915081906340c10f19906044015f604051808303815f87803b158015612ff1575f80fd5b505af1158015613003573d5f803e3d5ffd5b5050602480546040517f095ea7b30000000000000000000000000000000000000000000000000000000081526001600160a01b0391821660048201525f19928101929092528416925063095ea7b391506044015f604051808303815f87803b15801561306d575f80fd5b505af115801561307f573d5f803e3d5ffd5b5050602480546040517f47e7ef240000000000000000000000000000000000000000000000000000000081526001600160a01b0388811660048301529281018a9052911692506347e7ef2491506044015f604051808303815f87803b1580156130e6575f80fd5b505af11580156130f8573d5f803e3d5ffd5b5050855191505080156133f757604080516002808252606082019092525f91816020015b604080516080810182525f8082526020808301829052928201819052606082015282525f1990920191018161311c57905050905084815f815181106131635761316361877a565b60200260200101515f01906001600160a01b031690816001600160a01b031681525050838160018151811061319a5761319a61877a565b60209081029190910101516001600160a01b039190911690526040805160018082528183019092525f91816020015b604080516060810182525f80825260208083018290529282015282525f199092019101816131c957905050905085815f815181106132095761320961877a565b60200260200101515f01906001600160a01b031690816001600160a01b03168152505084815f8151811061323f5761323f61877a565b6020908102919091018101516001600160a01b03909216910152613267565b60a01c60020b90565b5f5b838110156133f3575f6040518060800160405280896001600160a01b03168152602001886001600160a01b031681526020015f6fffffffffffffffffffffffffffffffff1681526020018a84815181106132c5576132c561877a565b602090810291909101015190526024549091506001600160a01b03166397dc99656132ef86614901565b6133728760206004015f9054906101000a90046001600160a01b03166001600160a01b0316639d69dc4e6040518163ffffffff1660e01b8152600401602060405180830381865afa158015613346573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061336a9190618ab3565b889190614992565b61337c8588614a31565b60405160200161338e93929190618b5b565b6040516020818303038152906040526040518263ffffffff1660e01b81526004016133b99190618b78565b5f604051808303815f87803b1580156133d0575f80fd5b505af11580156133e2573d5f803e3d5ffd5b505060019093019250613269915050565b5050505b6133ff614ac6565b505050505050505050505050505050565b6318fb58646004525f81815260249020801954604051919068fbb67fda52d4bfb8bf90602084018161348957835480156134835780841415028152600184810154909250801561348357808414150260208201526002848101549092508015613483576003925083811415810260408301525b506134b4565b8160011c91505f5b828110156134b257848101548481141502600582901b830152600101613491565b505b8185528160051b810160405250505050919050565b63978aab926004525f818152602481206060915068fbb67fda52d4bfb8bf81548060a01b60a01c6040519450846020018260601c925083831415830281528161355757821561355257600191508185015460601c92508215613552578284141590920260208301525060028381015460601c918215613552576003915083831415830260408201525b613587565b600191821c915b82811015613585578581015460601c858114158102600583901b840152935060010161355e565b505b8186528160051b81016040525050505050919050565b63978aab926004525f8181526024812080548060a01b60a01c8060011c9350808260601c15176135e6576001935083830154156135e6576002935083830154156135e657600393505b505050919050565b5f6135fa848484614ba4565b90506106236040518060400160405280600c81526020017f426f756e6420726573756c74000000000000000000000000000000000000000081525082614d5d565b63978aab926004525f828152602481208281015460601c915068fbb67fda52d4bfb8bf8214158202915061366e8461359d565b83106136a6576040517f4e23d03500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5092915050565b63978aab926004525f828152602481206001600160a01b0392909216917fffffffffffffffffffffffffffffffffffffffffffffff04498025ad2b40474183016136fe5763f5a267f15f526004601cfd5b826137105768fbb67fda52d4bfb8bf92505b80546bffffffffffffffffffffffff811682602052806137d7578160601c80613743578560601b8455600194505061381b565b858103613750575061381b565b600184015460601c80613771578660601b600186015560019550505061381b565b86810361377f57505061381b565b600285015460601c806137a1578760601b60028701556001965050505061381b565b8781036137b05750505061381b565b5f928352604080842060019055918352818320600290558252902060039055506007908117905b845f5260405f20805461381957600191821c808301825591945081613805578560601b60031784555061381b565b8560601b828501558260020184555061381b565b505b50505092915050565b613895816040516024016138389190618b78565b60408051601f198184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f41304fac00000000000000000000000000000000000000000000000000000000179052614dd0565b50565b5f80826138c5817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff27618618b8a565b6138cf9190618bde565b9150826138df81620d89e8618b8a565b6138e99190618bde565b9050915091565b5f6138fc848484614de0565b604080518082018252600c81527f426f756e6420726573756c740000000000000000000000000000000000000000602082015290517fa322c40e0000000000000000000000000000000000000000000000000000000081526004810183905291925061062391737109709ecfa91a80626ff3989d68f67f5b1dd12d9063a322c40e906024015f60405180830381865afa15801561399b573d5f803e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526139c29190810190618838565b615041565b6040805160a0810182525f918101919091526001600160a01b038581166080830181905285821683529084166020830152600283900b606083015215613a105762800000613a12565b5f5b62ffffff166040820152949350505050565b6002818101546023545f9283928392910b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff2761881810784139082900503620d89e891909105036001016fffffffffffffffffffffffffffffffff04908290613a95906001600160a01b03168a8a6150b4565b506023549091505f90613ab2906001600160a01b03168b8a6150b4565b509050613ae3826fffffffffffffffffffffffffffffffff16826fffffffffffffffffffffffffffffffff16615119565b613aed9084618bfd565b6023549096505f9350613b0c92506001600160a01b0316905089614819565b90505f80613b1d8360a01c60020b90565b9150506001600160a01b038216613b3782828b8b8a61512e565b8854602580549398509196505f928110613b5357613b5361877a565b5f918252602090912001546040517f70a082310000000000000000000000000000000000000000000000000000000081523060048201526001600160a01b03909116906370a0823190602401602060405180830381865afa158015613bba573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613bde9190618a26565b90505f6020600501896001015481548110613bfb57613bfb61877a565b5f918252602090912001546040517f70a082310000000000000000000000000000000000000000000000000000000081523060048201526001600160a01b03909116906370a0823190602401602060405180830381865afa158015613c62573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613c869190618a26565b9050613d26612bad613d0e898510613cb0578a6fffffffffffffffffffffffffffffffff16613ccd565b613ccd6fffffffffffffffffffffffffffffffff8c16868c61475e565b898510613cec578b6fffffffffffffffffffffffffffffffff166151c5565b613d096fffffffffffffffffffffffffffffffff8d16868c61475e565b6151c5565b8a6fffffffffffffffffffffffffffffffff166151c5565b9750613d3584848d8d8c61512e565b989d909c50979a509698505050505050505050565b5f8080613d688486078213858705035b600881901d9160ff90911690565b9092509050613d9581613d856001600160a01b038a1689866151d3565b90600160ff919091161b16151590565b979650505050505050565b81545f5b81811015613df357838181548110613dbe57613dbe61877a565b905f5260205f2090600a91828204019190066003029054906101000a900460020b60020b8360020b12613df357600101613da4565b81811015613e805782848281548110613e0e57613e0e61877a565b905f5260205f2090600a91828204019190066003029054906101000a900460020b858381548110613e4157613e4161877a565b5f9182526020909120600a8083049091018054919092066003026101000a62ffffff818102199092169490911602929092179091559250600101613df3565b505081546001810183555f9283526020909220600a8084049091018054919093066003026101000a62ffffff818102199092169290911602179055565b60408051600285810b602083015284900b918101919091526001600160a01b03821660608201525f906080016040516020818303038152906040528051906020012090509392505050565b5050565b6318fb58646004525f8281526024812068fbb67fda52d4bfb8bf8303613f395763f5a267f15f526004601cfd5b82613f4b5768fbb67fda52d4bfb8bf92505b8019548160205280613fef57815480613f6b57848355600193505061401a565b848103613f78575061401a565b600183015480613f935785600185015560019450505061401a565b858103613fa157505061401a565b600284015480613fbd578660028601556001955050505061401a565b868103613fcc5750505061401a565b5f9283526040808420600190559183528183206002905582529020600390555060075b835f5260405f20805461381b57600191821c8381018690558083019182905590821b82178319559092505b505092915050565b6318fb58646004525f818152602481208019548060011c9250806140695781545f935015614069576001925082820154156140695760029250828201541561406957600392505b5050919050565b6318fb58646004525f8281526024902081015468fbb67fda52d4bfb8bf8114150261409a83614022565b82106105a3576040517f4e23d03500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81545f5b81811015614121578260020b8482815481106140f4576140f461877a565b5f9182526020909120600a8083049091015491066003026101000a900460020b14614121576001016140d6565b61412c600183618935565b8110156141c1578361413f8260016187d4565b8154811061414f5761414f61877a565b905f5260205f2090600a91828204019190066003029054906101000a900460020b8482815481106141825761418261877a565b905f5260205f2090600a91828204019190066003026101000a81548162ffffff021916908360020b62ffffff1602179055508080600101915050614121565b838054806141d1576141d1618c6a565b600190038181905f5260205f2090600a91828204019190066003026101000a81549062ffffff0219169055905550505050565b6318fb58646004525f8281526024812068fbb67fda52d4bfb8bf83036142315763f5a267f15f526004601cfd5b826142435768fbb67fda52d4bfb8bf92505b801954806142ac5760019250838254036142705760018201805483556002830180549091555f905561401a565b8360018301540361428e5760028201805460018401555f905561401a565b836002830154036142a4575f600283015561401a565b5f925061401a565b81602052835f5260405f208054806142c557505061401a565b60018360011c0392508260018203146142ef57828401548060018303860155805f52508060405f20555b5060018260011b178319555f81555060019250505092915050565b5f6105a382620f4240615201565b6040517fa59828850000000000000000000000000000000000000000000000000000000081528115156004820152737109709ecfa91a80626ff3989d68f67f5b1dd12d9063a5982885906024015f6040518083038186803b15801561437b575f80fd5b505afa15801561438d573d5f803e3d5ffd5b5050505050565b5f6105a360206001600160a01b0384163b618aa0565b6023546060905f906143cf908490610ad09061325e906001600160a01b031688614819565b90505f836143e06014610100618bde565b6143ea9190618bde565b90505f6143f78284618c97565b90505f6144048385618cd8565b5f888152603060205260408120805492935090915b80831015614468578460020b8284815481106144375761443761877a565b5f9182526020909120600a8083049091015491066003026101000a900460020b121561446857600190920191614419565b826144738183618935565b67ffffffffffffffff81111561448b5761448b618698565b6040519080825280602002602001820160405280156144b4578160200160208202803683370190505b5098505b81841015614560578460020b8385815481106144d6576144d661877a565b5f9182526020909120600a8083049091015491066003026101000a900460020b136145605782848154811061450d5761450d61877a565b905f5260205f2090600a91828204019190066003029054906101000a900460020b89828603815181106145425761454261877a565b60029290920b602092830291909101909101526001909301926144b8565b90920387525094979650505050505050565b81835261457e81615217565b8360200181905250505050565b80515f906105a09061459e9085906145a5565b8390615278565b5f816001036145b557505f6105a3565b5f826145c2815f19618aa0565b6145cc9190618a0f565b845160205290505f805b815f5260405f2090508281106145f1576001820191506145d6565b848106935050506003601f53506021601f209092525090565b5f825f0361465f5760405162461bcd60e51b815260206004820152600560248201527f4c6f77203000000000000000000000000000000000000000000000000000000060448201526064015b60405180910390fd5b8183106146ae5760405162461bcd60e51b815260206004820152601260248201527f4c6f77206e6f742062656c6f77206869676800000000000000000000000000006044820152606401614656565b5f6146b9838561536f565b90505f6146cd6146c8836153b1565b61540c565b90505f6146db878284615665565b90505f6146ef6146ea836156b7565b61589c565b90506146fb87826158d7565b98975050505050505050565b5f61475a826fffffffffffffffffffffffffffffffff80166040518060400160405280601681526020017f556e73616665206361737420746f2075696e7431323800000000000000000000815250615906565b5090565b828202818385830414851517026147e5575f198385098181108201900382848609835f0384168285116147985763ae47f7025f526004601cfd5b93849004938382119092035f839003839004600101029203041760026003830281188084028203028084028203028084028203028084028203028084028203028084029091030202610623565b0492915050565b60606148108585858561480b61325e6001600160a01b03851684614819565b61598b565b95945050505050565b5f8181526006602052604081206148396001600160a01b03851682615f3b565b949350505050565b60200260200101516fffffffffffffffffffffffffffffffff16615f6b565b8351816fffffffffffffffffffffffffffffffff1610156148d857816148a385836fffffffffffffffffffffffffffffffff16815181106148415761484161877a565b6040516020016148b4929190618d19565b604051602081830303815290604052915080806148d090618d54565b915050614860565b50806040516020016148ea9190618d90565b604051602081830303815290604052915050919050565b60605f5b825181101561495c57816149318483815181106149245761492461877a565b6020026020010151615f76565b604051602001614942929190618dc8565b60408051601f198184030181529190529150600101614905565b506149678151616063565b60e81b8160405160200161497c929190618dd6565b6040516020818303038152906040529050919050565b60605f5b84518110156149f957816149ce85858885815181106149b7576149b761877a565b60200260200101516160779092919063ffffffff16565b6040516020016149df929190618dc8565b60408051601f198184030181529190529150600101614996565b50614a048151616063565b60e81b81604051602001614a19929190618dd6565b60405160208183030381529060405290509392505050565b60605f80614a51855f01518660200151866161369092919063ffffffff16565b915091505f85606001515f0151614a68575f614a6b565b60025b82614a76575f614a79565b60015b1790508060f81b8360f01b876040015160801b614a9989606001516162b8565b604051602001614aac9493929190618e09565b604051602081830303815290604052935050505092915050565b5f614ad1602861359d565b90505f5b81811015613f08575f614ae960288361363b565b602480546040517fd57197780000000000000000000000000000000000000000000000000000000081526001600160a01b03808516600483015293945092169163d57197789101602060405180830381865afa158015614b4b573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614b6f9190618a26565b6001600160a01b0382165f908152602d602052604081208054909190614b96908490618e8c565b909155505050600101614ad5565b5f81831115614c1b5760405162461bcd60e51b815260206004820152603e60248201527f5374645574696c7320626f756e642875696e743235362c75696e743235362c7560448201527f696e74323536293a204d6178206973206c657373207468616e206d696e2e00006064820152608401614656565b828410158015614c2b5750818411155b15614c37575082610623565b5f614c428484618935565b614c4d9060016187d4565b905060038511158015614c5f57508481115b15614c7657614c6e85856187d4565b915050610623565b614c8260035f19618935565b8510158015614c9a5750614c97855f19618935565b81115b15614cb457614caa855f19618935565b614c6e9084618935565b82851115614d07575f614cc78487618935565b90505f614cd48383618eab565b9050805f03614ce857849350505050610623565b6001614cf482886187d4565b614cfe9190618935565b93505050614d55565b83851015614d55575f614d1a8686618935565b90505f614d278383618eab565b9050805f03614d3b57859350505050610623565b614d458186618935565b614d509060016187d4565b935050505b509392505050565b613f088282604051602401614d73929190618c49565b60408051601f198184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fb60e72cc000000000000000000000000000000000000000000000000000000001790526163cc565b613895816163d85b63ffffffff16565b5f81831315614e575760405162461bcd60e51b815260206004820152603b60248201527f5374645574696c7320626f756e6428696e743235362c696e743235362c696e7460448201527f323536293a204d6178206973206c657373207468616e206d696e2e00000000006064820152608401614656565b5f808512614e8e57614e897f8000000000000000000000000000000000000000000000000000000000000000866187d4565b614ec5565b6001614ebb86197f8000000000000000000000000000000000000000000000000000000000000000618935565b614ec59190618935565b90505f808512614efe57614ef97f8000000000000000000000000000000000000000000000000000000000000000866187d4565b614f35565b6001614f2b86197f8000000000000000000000000000000000000000000000000000000000000000618935565b614f359190618935565b90505f808512614f6e57614f697f8000000000000000000000000000000000000000000000000000000000000000866187d4565b614fa5565b6001614f9b86197f8000000000000000000000000000000000000000000000000000000000000000618935565b614fa59190618935565b90505f614fb3848484614ba4565b90507f8000000000000000000000000000000000000000000000000000000000000000811061500b576150067f800000000000000000000000000000000000000000000000000000000000000082618935565b6146fb565b615035817f8000000000000000000000000000000000000000000000000000000000000000618935565b6146fb901960016187d4565b613f088282604051602401615057929190618c25565b60408051601f198184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f4b5c4277000000000000000000000000000000000000000000000000000000001790526163cc565b5f806006602052835f52600460405f2001602052825f5260405f20602052631e2eaeaf5f5260205f6024601c885afa6150f45763535cf94b5f526004601cfd5b50505f516fffffffffffffffffffffffffffffffff81169460809190911d9350915050565b5f81831161512757816105a0565b5090919050565b5f808460020b8760020b12156151625761515b61514a866163f7565b615153866163f7565b8560016166d5565b91506151bb565b8360020b8760020b121561519b5761517d86615153866163f7565b915061519461518b866163f7565b8785600161679f565b90506151bb565b6151b86151a7866163f7565b6151b0866163f7565b85600161679f565b90505b9550959350505050565b5f81831061512757816105a0565b5f82815260066020908152604080832084845260050190915281206148106001600160a01b03861682615f3b565b5f6105a08362ffffff165f8462ffffff166135ee565b604080518082019091525f8152606060208201528167ffffffffffffffff81111561524457615244618698565b60405190808252806020026020018201604052801561526d578160200160208202803683370190505b506020820152919050565b81515f906152c85760405162461bcd60e51b815260206004820152600e60248201527f4e6f7468696e6720746f207573650000000000000000000000000000000000006044820152606401614656565b5f806152d4858561683f565b875190955091935091505f906152f7906152f090600190618935565b879061683f565b925050506001865f0181815161530d9190618935565b90525082156153405760208601516168f69081906153399061532f90866168fe565b848363ffffffff16565b505061381b565b60408051808201909152858152602080820183905287015181906153649082616960565b505050505092915050565b5f7812725dd1d243aba0e75fe645cc4873f9e65afe688c928e1f228310820261539f57637c5f487d5f526004601cfd5b50670de0b6b3a7640000919091020490565b5f7f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82111561475a576040517f35278d1200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6fffffffffffffffffffffffffffffffff811160071b81811c67ffffffffffffffff1060061b1781811c63ffffffff1060051b1781811c61ffff1060041b1781811c60ff1060031b175f821361546957631615e6385f526004601cfd5b7ff8f9f9faf9fdfafbf9fdfcfdfafbfcfef9fafdfafcfcfbfefafafcfbffffffff6f8421084210842108cc6318c6db6d54be83831c1c601f161a1890811b609f90811c6c465772b2bbbb5f824b15207a3081018102606090811d6d0388eaa27412d5aca026815d636e018202811d6d0df99ac502031bf953eff472fdcc018202811d6d13cdffb29d51d99322bdff5f2211018202811d6d0a0f742023def783a307a986912e018202811d6d01920d8043ca89b5239253284e42018202811d6c0b7a86d7375468fac667a0a527016c29508e458543d8aa4df2abee7883018302821d6d0139601a2efabe717e604cbb4894018302821d6d02247f7a7b6594320649aa03aba1018302821d7fffffffffffffffffffffffffffffffffffffff73c0c716a594e00d54e3c4cbc9018302821d7ffffffffffffffffffffffffffffffffffffffdc7b88c420e53a9890533129f6f01830290911d7fffffffffffffffffffffffffffffffffffffff465fda27eb4d63ded474e5f832019091027ffffffffffffffff5f6af8f7b3396644f18e157960000000000000000000000000105711340daa0d5f769dba1915cef59f0815a5506029190037d0267a36c0c95b3975ab3ee5b203a7614a3f75373f047d803ae7b6687f2b302017d57115e47018c7177eebf7cd370a3356a1b7863008a5ae8028c72b88642840160ae1d90565b5f81831261569f576040517fa883435700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b828203836156ad86836145a5565b0195945050505050565b5f7ffffffffffffffffffffffffffffffffffffffffffffffffdc0d0570925a462d782136156e457919050565b680755bf798b4a1bf1e582126157015763a37bfec95f526004601cfd5b6503782dace9d9604e83901b0591505f60606bb17217f7d1cf79abc9e3b39884821b056b80000000000000000000000001901d6bb17217f7d1cf79abc9e3b39881029093037fffffffffffffffffffffffffffffffffffffffdbf3ccf1604d263450f02a550481018102606090811d6d0277594991cfc85f6e2461837cd9018202811d7fffffffffffffffffffffffffffffffffffffe5adedaa1cb095af9e4da10e363c018202811d6db1bbb201f443cf962f1a1d3db4a5018202811d7ffffffffffffffffffffffffffffffffffffd38dc772608b0ae56cce01296c0eb018202811d6e05180bb14799ab47a8a8cb2a527d57016d02d16720577bd19bf614176fe9ea6c10fe68e7fd37d0007b713f765084018402831d9081019084017ffffffffffffffffffffffffffffffffffffffe2c69812cf03b0763fd454a8f7e010290911d6e0587f503bb6ea29d25fcb7401964500190910279d835ebba824c98fb31b83b2ca45c000000000000000000000000010574029d9dc38563c32e5c2f6dc192ee70ef65f9978af30260c3939093039290921c92915050565b5f8082121561475a576040517fcaccb6d900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f815f19048311156158f65781156158f65763bac65e5b5f526004601cfd5b50670de0b6b3a764000091020490565b6040517fd17d4b0d000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d9063d17d4b0d9061595a90869086908690600401618ebe565b5f6040518083038186803b158015615970575f80fd5b505afa158015615982573d5f803e3d5ffd5b50505050505050565b606060018360020b12156159e15760405162461bcd60e51b815260206004820152601460248201527f496e76616c6964205449434b5f53504143494e470000000000000000000000006044820152606401614656565b855115614810576159f485858886616a61565b6159fd86616b22565b8160020b8660018851615a109190618935565b81518110615a2057615a2061877a565b60200260200101515f015160020b13615ab85760408051600180825281830190925290816020015b6040805160c0810182525f80825260208083018290529282018190526060808301829052608083015260a082015282525f19909201910181615a48579050509050615a968585888587616bdb565b815f81518110615aa857615aa861877a565b6020026020010181905250614810565b855f81518110615aca57615aca61877a565b60200260200101515f015160020b8260020b13615b445760408051600180825281830190925290816020015b6040805160c0810182525f80825260208083018290529282018190526060808301829052608083015260a082015282525f19909201910181615af6579050509050615a9685858885876170e4565b815f5b615b5c6001600160a01b0388168784886175f9565b9250905080615b775781615b6f81618edc565b925050615b47565b875f81518110615b8957615b8961877a565b60200260200101515f015160020b8260020b13615c275760408051600180825281830190925290816020015b6040805160c0810182525f80825260208083018290529282018190526060808301829052608083015260a082015282525f19909201910181615bb5579050509250615c0387878a87896170e4565b835f81518110615c1557615c1561877a565b60200260200101819052505050614810565b50505f615c34878461764e565b90505f8167ffffffffffffffff811115615c5057615c50618698565b604051908082528060200260200182016040528015615c9457816020015b604080518082019091525f8082526020820152815260200190600190039081615c6e5790505b5090505f828951615ca59190618935565b67ffffffffffffffff811115615cbd57615cbd618698565b604051908082528060200260200182016040528015615d0157816020015b604080518082019091525f8082526020820152815260200190600190039081615cdb5790505b5090505f5b8951811015615d9e5783811015615d5357898181518110615d2957615d2961877a565b6020026020010151838281518110615d4357615d4361877a565b6020026020010181905250615d96565b898181518110615d6557615d6561877a565b6020026020010151828583615d7a9190618935565b81518110615d8a57615d8a61877a565b60200260200101819052505b600101615d06565b5081515f03615e2f5760408051600180825281830190925290816020015b6040805160c0810182525f80825260208083018290529282018190526060808301829052608083015260a082015282525f19909201910181615dbc579050509350615e0a888883888a6170e4565b845f81518110615e1c57615e1c61877a565b6020026020010181905250505050614810565b80515f03615e9a5760408051600180825281830190925290816020015b6040805160c0810182525f80825260208083018290529282018190526060808301829052608083015260a082015282525f19909201910181615e4c579050509350615e0a888884888a616bdb565b6040805160028082526060820190925290816020015b6040805160c0810182525f80825260208083018290529282018190526060808301829052608083015260a082015282525f19909201910181615eb0579050509350615efe888883888a6170e4565b845f81518110615f1057615f1061877a565b6020026020010181905250615f28888884888a616bdb565b84600181518110615e1c57615e1c61877a565b5f81602052631e2eaeaf5f5260205f6024601c865afa615f625763535cf94b5f526004601cfd5b50505f51919050565b60606105a3826167fd565b8051602080830151604080850151606080870151925195811b7fffffffffffffffffffffffffffffffffffffffff0000000000000000000000001694860194909452608092831b7fffffffffffffffffffffffffffffffff00000000000000000000000000000000908116603487015290831b81166044860152911b166054830152906064016040516020818303038152906040529050604481511461605e5760405162461bcd60e51b815260206004820152601860248201527f41737365747320756e6578706563746564206c656e67746800000000000000006044820152606401614656565b919050565b5f6301000000821061475a5761475a6176ec565b6060616082846176f9565b835160208501515f91829161609891879161776e565b915091505f6160af85885f0151896020015161779b565b90508260f01b8260f01b8260f01b6160c88a6040015190565b6040517fffff000000000000000000000000000000000000000000000000000000000000948516602082015292841660228401529216602482015260268101919091526046016040516020818303038152906040529350602684511461612c575f80fd5b5050509392505050565b5f80826001600160a01b0316846001600160a01b0316036161995760405162461bcd60e51b815260206004820152601360248201527f6173736574496e203d3d2061737365744f7574000000000000000000000000006044820152606401614656565b826001600160a01b0316846001600160a01b03161090505f80826161be5784866161c1565b85855b915091505f93505b86518461ffff161015616259575f878561ffff16815181106161ed576161ed61877a565b60200260200101519050616200816176f9565b826001600160a01b0316815f01516001600160a01b03161480156162395750816001600160a01b031681602001516001600160a01b0316145b15616246575050506162b0565b508361625181618f19565b9450506161c9565b86518461ffff16106162ad5760405162461bcd60e51b815260206004820152600e60248201527f50616972206e6f7420666f756e640000000000000000000000000000000000006044820152606401614656565b50505b935093915050565b80516060901561631157816020015160801b826060015160801b60405160200161497c9291907fffffffffffffffffffffffffffffffff0000000000000000000000000000000092831681529116601082015260200190565b60605f5b83608001515181101561636f5781846080015182815181106163395761633961877a565b602002602001015160801b604051602001616355929190618f30565b60408051601f198184030181529190529150600101616315565b5061637a8151616063565b60e81b8160405160200161638f929190618dd6565b6040516020818303038152906040529050826040015160e81b836060015160801b828560a0015160601b6040516020016148ea9493929190618f6c565b613895816178d6614dd8565b5f6a636f6e736f6c652e6c6f6790505f80835160208501845afa505050565b60020b5f60ff82901d80830118620d89e8811115616439576164397f8b86327a00000000000000000000000000000000000000000000000000000000846178f6565b7001fffcb933bd6fad37aa2d162d1a5940016001821602700100000000000000000000000000000000186002821615616482576ffff97272373d413259a46990580e213a0260801c5b60048216156164a1576ffff2e50f5f656932ef12357cf3c7fdcc0260801c5b60088216156164c0576fffe5caca7e10e4e61c3624eaa0941cd00260801c5b60108216156164df576fffcb9843d60f6159c9db58835c9266440260801c5b60208216156164fe576fff973b41fa98c081472e6896dfb254c00260801c5b604082161561651d576fff2ea16466c96a3843ec78b326b528610260801c5b608082161561653c576ffe5dee046a99a2a811c461f1969c30530260801c5b61010082161561655c576ffcbe86c7900a88aedcffc83b479aa3a40260801c5b61020082161561657c576ff987a7253ac413176f2b074cf7815e540260801c5b61040082161561659c576ff3392b0822b70005940c7a398e4b70f30260801c5b6108008216156165bc576fe7159475a2c29b7443b29c7fa6e889d90260801c5b6110008216156165dc576fd097f3bdfd2022b8845ad8f792aa58250260801c5b6120008216156165fc576fa9f746462d870fdf8a65dc1f90e061e50260801c5b61400082161561661c576f70d869a156d2a1b890bb3df62baf32f70260801c5b61800082161561663c576f31be135f97d08fd981231505542fcfa60260801c5b6201000082161561665d576f09aa508b5b7a84e1c677de54f3e99bc90260801c5b6202000082161561667d576e5d6af8dedb81196699c329225ee6040260801c5b6204000082161561669c576d2216e584f5fa1ea926041bedfe980260801c5b620800008216156166b9576b048a170391f7dc42444e8fa20260801c5b5f8413156166c5575f19045b63ffffffff0160201c9392505050565b5f836001600160a01b0316856001600160a01b031611156166f4579293925b6001600160a01b03851661670e5762bfc9215f526004601cfd5b7bffffffffffffffffffffffffffffffff000000000000000000000000606084901b166001600160a01b03868603168361677357866001600160a01b03166167608383896001600160a01b0316617905565b8161676d5761676d618a73565b04613d95565b613d9561678a8383896001600160a01b03166179a1565b886001600160a01b0316808204910615150190565b5f6001600160a01b038481169086160360ff81901d908101186c010000000000000000000000006fffffffffffffffffffffffffffffffff85166167e4818484617905565b9350845f83858409111684019350505050949350505050565b60606080604051019050602081016040525f8152805f19835b928101926030600a8206018453600a900480616816575050819003601f19909101908152919050565b5f805f845f015184106168945760405162461bcd60e51b815260206004820152601360248201527f496e646578206f75742d6f662d626f756e6473000000000000000000000000006044820152606401614656565b5f91506168fe805b6020870151518410156168e6575f6168bc8860200151868463ffffffff16565b905086815f0151036168da57602001516001955092506168ef915050565b5060019093019261689c565b505f9350849150505b9250925092565b602090910152565b81515f90821061693a576040517f44dd369f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b826020015182815181106169505761695061877a565b6020026020010151905092915050565b8151808361696d82618ffa565b9052506020830151518351811015616a38575f61698b826002618a0f565b90505f6001821061699c578161699f565b60015b67ffffffffffffffff8111156169b7576169b7618698565b6040519080825280602002602001820160405280156169e0578160200160208202803683370190505b5090505f5b83811015616a305786602001518181518110616a0357616a0361877a565b6020026020010151828281518110616a1d57616a1d61877a565b60209081029190910101526001016169e5565b506020860152505b8284602001518381518110616a4f57616a4f61877a565b60200260200101818152505050505050565b5f5b825181101561438d575f838281518110616a7f57616a7f61877a565b60200260200101515f015190505f80616aa1613d5a84875f8183071291050390565b90925090505f616abb6001600160a01b038a1689856151d3565b9050600160ff83161b8116616b125760405162461bcd60e51b815260206004820152601460248201527f5469636b206e6f7420696e697469616c697a65640000000000000000000000006044820152606401614656565b505060019092019150616a639050565b616b2b816179d1565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8000005f5b8251811015616bd6575f838281518110616b6a57616b6a61877a565b60200260200101515f015190508260020b8160020b13616bcc5760405162461bcd60e51b815260206004820152600e60248201527f4475706c6963617465207469636b0000000000000000000000000000000000006044820152606401614656565b9150600101616b4e565b505050565b6040805160c0810182525f808252602082018190529181018290526060808201839052608082015260a08101919091525f845111616c5b5760405162461bcd60e51b815260206004820152600a60248201527f4e6f2072657761726473000000000000000000000000000000000000000000006044820152606401614656565b5f616c7e600286516003616c6f9190618a0f565b616c799190618aa0565b615217565b90505f855f81518110616c9357616c9361877a565b602090810291909101015151905060015f5b616cba6001600160a01b038b168a8589617abe565b93509150600283810b9088900b12616d70578115616ce757616ce084600285900b616960565b505f616ca5565b80616cf181618ffa565b9150506078811115616d6b5760405162461bcd60e51b815260206004820152602d60248201527f4d41585f4c4f4f5020657863656564656420696e205f6372656174655265776160448201527f726455706461746542656c6f77000000000000000000000000000000000000006064820152608401614656565b616ca5565b505081515f039050616dfd578451600114616dcd5760405162461bcd60e51b815260206004820152601960248201527f65787065637465642072657761726473206c656e6774682031000000000000006044820152606401614656565b616df58787875f81518110616de457616de461877a565b602002602001015160200151617afb565b915050614810565b8051616e0a9060016187d4565b67ffffffffffffffff811115616e2257616e22618698565b604051908082528060200260200182016040528015616e4b578160200160208202803683370190505b5060808301525f80805b8351811015616f1d575f616e6985836168fe565b90505f616e806001600160a01b038d168c846150b4565b9150616e8e90508185619012565b93508951851015616f13575f8a8681518110616eac57616eac61877a565b602002602001015190508260020b815f015160020b1215616f1157806020015188608001518581518110616ee257616ee261877a565b6fffffffffffffffffffffffffffffffff9092166020928302919091019091015285616f0d81618ffa565b9650505b505b5050600101616e55565b508651821015616f8d57868281518110616f3957616f3961877a565b6020026020010151602001518460800151845f015181518110616f5e57616f5e61877a565b6fffffffffffffffffffffffffffffffff9092166020928302919091019091015281616f8981618ffa565b9250505b86518214616fdd5760405162461bcd60e51b815260206004820152601560248201527f4e6f7420616c6c207265776172647320757365643f00000000000000000000006044820152606401614656565b616fe7835f6168fe565b60020b60408501525f616ffc8a8a8989617b6e565b90506170088183617dad565b6fffffffffffffffffffffffffffffffff166060860181905289908b905f90815b88518110156170cb575f61703d8a836168fe565b90505f6170546001600160a01b03871688846150b4565b9150506170618482617dc7565b60408051602080820198909852608083901b7fffffffffffffffffffffffffffffffff00000000000000000000000000000000168183015260e89490941b605085015280516033818603018152605390940190528251929095019190912093925050600101617029565b505060601c60a088015250505050505095945050505050565b6040805160c0810182525f808252602082018190529181018290526060808201839052608082015260a08101919091525f8451116171645760405162461bcd60e51b815260206004820152600a60248201527f4e6f2072657761726473000000000000000000000000000000000000000000006044820152606401614656565b5f617178600286516003616c6f9190618a0f565b90505f856001875161718a9190618935565b8151811061719a5761719a61877a565b60209081029190910101515190506001815f5b8160020b8860020b12156172795782156171d6576171cf85600284900b616960565b505f61725a565b806171e081618ffa565b915050607881111561725a5760405162461bcd60e51b815260206004820152602d60248201527f4d41585f4c4f4f5020657863656564656420696e205f6372656174655265776160448201527f726455706461746541626f7665000000000000000000000000000000000000006064820152608401614656565b61726f6001600160a01b038c168b848a617de1565b90935091506171ad565b505082515f039050617317576172a660405180608001604052806044815260200161a93260449139613824565b85516001146172f75760405162461bcd60e51b815260206004820152601960248201527f4578706563746564206578616374206f6e6520726577617264000000000000006044820152606401614656565b61730e8888885f81518110616de457616de461877a565b92505050614810565b600281900b6040840152815161732e9060016187d4565b67ffffffffffffffff81111561734657617346618698565b60405190808252806020026020018201604052801561736f578160200160208202803683370190505b50608084015285515f805b8451811015617448575f61738e86836168fe565b90505f6173a56001600160a01b038e168d846150b4565b91506173b390508185619012565b9350841561743e575f8b6173c8600188618935565b815181106173d8576173d861877a565b602002602001015190508260020b815f015160020b1261743c5780602001518960800151858151811061740d5761740d61877a565b6fffffffffffffffffffffffffffffffff909216602092830291909101909101528561743881619060565b9650505b505b505060010161737a565b5081156174b557875f815181106174615761746161877a565b6020026020010151602001518560800151855f0151815181106174865761748661877a565b6fffffffffffffffffffffffffffffffff90921660209283029190910190910152816174b181619060565b9250505b81156175035760405162461bcd60e51b815260206004820152601560248201527f4e6f7420616c6c207265776172647320757365643f00000000000000000000006044820152606401614656565b5f6175108b8b8a8a617b6e565b905061751c8183617dc7565b6fffffffffffffffffffffffffffffffff16606087018190528a908c905f90815b89518110156175df575f6175518b836168fe565b90505f6175686001600160a01b03871688846150b4565b9150506175758482617dad565b60408051602080820198909852608083901b7fffffffffffffffffffffffffffffffff00000000000000000000000000000000168183015260e89490941b60508501528051603381860301815260539094019052825192909501919091209392505060010161753d565b505060601c60a08901525050505050505095945050505050565b5f80808061760e858707821386880503613d5a565b90925090506176318161762b6001600160a01b038b168a866151d3565b90617dfb565b9094509050617641828287617ec3565b9250505094509492505050565b5f8083511161769f5760405162461bcd60e51b815260206004820152600a60248201527f4e6f2072657761726473000000000000000000000000000000000000000000006044820152606401614656565b5f5b83518110156176e3578260020b8482815181106176c0576176c061877a565b60200260200101515f015160020b13156176db5790506105a3565b6001016176a1565b50509051919050565b6335278d125f526004601cfd5b805f01516001600160a01b031681602001516001600160a01b03161161389557604080517f5190344300000000000000000000000000000000000000000000000000000000815282516001600160a01b0390811660048301526020840151166024820152908201516044820152606401614656565b5f8061778261777d8686617eed565b617f86565b915061779161777d8685617eed565b9050935093915050565b5f816001600160a01b0316836001600160a01b0316106177fd5760405162461bcd60e51b815260206004820152601d60248201527f67657453746f7265496e6465783a61737365747320756e736f727465640000006044820152606401614656565b5f83815260208381526040822060281b9190617823906001600160a01b0388163b618aa0565b5f93509050855b818461ffff16101561788e575f6020856020026001015f843c505f517fffffffffffffffffffffffffffffffffffffffffffffffffffffff00000000008085169082160361787b5750505050610623565b508361788681618f19565b94505061782a565b60405162461bcd60e51b815260206004820152601060248201527f506f6f6c206e6f7420656e61626c6564000000000000000000000000000000006044820152606401614656565b80516a636f6e736f6c652e6c6f67602083015f808483855afa5050505050565b815f528060020b60045260245ffd5b5f838302815f1985870982811083820303915050808411617924575f80fd5b805f0361793657508290049050610623565b5f848688095f868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150509392505050565b5f6179ad848484617905565b905081806179bd576179bd618a73565b838509156106235760010180610623575f80fd5b5f5b8151811015613f08575f6179e88260016187d4565b90505b8251811015617ab557617a39838281518110617a0957617a0961877a565b6020026020010151848481518110617a2357617a2361877a565b6020026020010151617f9990919063ffffffff16565b15617aad57828181518110617a5057617a5061877a565b6020026020010151838381518110617a6a57617a6a61877a565b6020026020010151848481518110617a8457617a8461877a565b60200260200101858481518110617a9d57617a9d61877a565b6020026020010182905282905250505b6001016179eb565b506001016179d3565b5f808080617ad8613d5a8688078313878905036001618cd8565b909250905061763181617af56001600160a01b038b168a866151d3565b90617fa7565b6040805160c0810182525f9181018290526060808201839052608082015260a0810191909152600181526fffffffffffffffffffffffffffffffff82166020820152617b506001600160a01b03851684618069565b6fffffffffffffffffffffffffffffffff1660608201529392505050565b5f80617b8661325e6001600160a01b03881687614819565b90505f617b9c6001600160a01b03881687618069565b90508460020b8260020b1215617c9b575f82815b617bc56001600160a01b038b168a848a617abe565b9093509150600282810b9089900b12617c93578215617c0a57505f80617bf56001600160a01b038c168b856150b4565b915050617c028582617dc7565b945050617bb0565b80617c1481618ffa565b9150506078811115617c8e5760405162461bcd60e51b815260206004820152603a60248201527f4d41585f4c4f4f5020657863656564656420696e206765744c6971756964697460448201527f7941745469636b205b70726573656e74203c206675747572655d0000000000006064820152608401614656565b617bb0565b505050617da3565b8160020b8560020b1215617da3576001825f5b617cc36001600160a01b038b168a848a6175f9565b9093509150600288810b9083900b1315617d9f578215617d0957505f80617cf46001600160a01b038c168b856150b4565b915050617d018582617dad565b945050617d8d565b80617d1381618ffa565b9150506078811115617d8d5760405162461bcd60e51b815260206004820152603a60248201527f4d41585f4c4f4f5020657863656564656420696e206765744c6971756964697460448201527f7941745469636b205b667574757265203c2070726573656e745d0000000000006064820152608401614656565b81617d9781618edc565b925050617cae565b5050505b9695505050505050565b808203608081901c156105a35763c9654ed45f526004601cfd5b818101608081901c156105a35763c9654ed45f526004601cfd5b5f80808061760e613d5a60018789078413888a0503618c97565b5f805f8360ff0390505f617e9c8260ff1687901b7f0706060506020504060203020504030106050205030304010505030400000000601f6f8421084210842108cc6318c6db6d54be831560081b6fffffffffffffffffffffffffffffffff851160071b1784811c67ffffffffffffffff1060061b1784811c63ffffffff1060051b1784811c61ffff1060041b1784811c60ff1060031b1793841c1c161a1790565b9050806101001415935083617eb1575f617eb8565b8160ff1681035b925050509250929050565b5f8160ff8416617ed9600187900b610100618bde565b617ee39190618cd8565b6148399190618bde565b5f805b8351811015617f3d57838181518110617f0b57617f0b61877a565b60200260200101515f01516001600160a01b0316836001600160a01b031603617f355790506105a3565b600101617ef0565b5060405162461bcd60e51b815260206004820152600f60248201527f4173736574206e6f7420666f756e6400000000000000000000000000000000006044820152606401614656565b5f62010000821061475a5761475a6176ec565b519051600291820b910b1390565b5f805f6180428460ff1686901c7e1f0d1e100c1d070f090b19131c1706010e11080a1a141802121b150316040581196001019091166101e07f804040554300526644320000502061067405302602000010750620017611707760fc7fb6db6db6ddddddddd34d34d349249249210842108c6318c639ce739cffffffff840260f81c161b60f71c1690811c63d76453e004601f169190911a1790565b90508061010014159250826180585760ff61805f565b8360ff1681015b9150509250929050565b5f8181526006602052604081205f6148106001600160a01b03861660038401615f3b565b6118bc8061907683390190565b60405180604001604052805f81526020016180c760405180604001604052805f8152602001606081525090565b905290565b5f602082840312156180dc575f80fd5b5035919050565b5f80604083850312156180f4575f80fd5b50508035926020909101359150565b602080825282518282018190525f918401906040840190835b818110156181435783516001600160a01b031683526020938401939092019160010161811c565b509095945050505050565b602080825282518282018190525f918401906040840190835b81811015618143578351835260209384019390920191600101618167565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f82825180855260208501945060208160051b830101602085015f5b8381101561820157601f198584030188526181eb838351618185565b60209889019890935091909101906001016181cf565b50909695505050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156182a2577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc087860301845281516001600160a01b038151168652602081015190506040602087015261828c60408701826181b3565b9550506020938401939190910190600101618233565b50929695505050505050565b6001600160a01b0381168114613895575f80fd5b5f602082840312156182d2575f80fd5b8135610623816182ae565b8060020b8114613895575f80fd5b5f805f805f60a086880312156182ff575f80fd5b85359450602086013593506040860135618318816182dd565b92506060860135618328816182dd565b949793965091946080013592915050565b5f8151808452602084019350602083015f5b8281101561838b5781517fffffffff000000000000000000000000000000000000000000000000000000001686526020958601959091019060010161834b565b5093949350505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156182a2577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc087860301845281518051604087526183ff6040880182618185565b905060208201519150868103602088015261841a8183618339565b9650505060209384019391909101906001016183bb565b5f8151808452602084019350602083015f5b8281101561838b57815180518752602081015160208801526040810151604088015250606086019550602082019150600181019050618443565b60208152815160020b6020820152602082015160020b60408201525f60408301516184b360608401826001600160a01b03169052565b5060608301516080830152608083015160a083015260a083015160c083015260c083015160e083015260e083015161010080840152614839610120840182618431565b602080825282518282018190525f918401906040840190835b81811015618143578351805160020b84526020908101516fffffffffffffffffffffffffffffffff16818501529093019260409092019160010161850f565b602081525f6105a060208301846181b3565b5f805f60608486031215618572575f80fd5b505081359360208301359350604090920135919050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156182a2577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc087860301845281516001600160a01b03815116865260208101519050604060208701526186086040870182618339565b95505060209384019391909101906001016185af565b803562ffffff8116811461605e575f80fd5b5f805f805f8060c08789031215618645575f80fd5b8635955060208701359450604087013561865e816182dd565b935061866c6060880161861e565b925061867a6080880161861e565b915060a087013561868a816182ae565b809150509295509295509295565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b604051601f8201601f1916810167ffffffffffffffff811182821017156186ee576186ee618698565b604052919050565b5f805f8385036060811215618709575f80fd5b8435935060208086013593507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc082011215618742575f80fd5b506040516020810167ffffffffffffffff8111828210171561876657618766618698565b604090815294909401358452509093909250565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b808201808211156105a3576105a36187a7565b600181811c908216806187fb57607f821691505b602082108103618832577f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b50919050565b5f60208284031215618848575f80fd5b815167ffffffffffffffff81111561885e575f80fd5b8201601f8101841361886e575f80fd5b805167ffffffffffffffff81111561888857618888618698565b61889b6020601f19601f840116016186c5565b8181528560208385010111156188af575f80fd5b8160208401602083015e5f91810160200191909152949350505050565b5f81518060208401855e5f93019283525090919050565b7f6163746f725f000000000000000000000000000000000000000000000000000081525f6105a060068301846188cc565b6001600160a01b0383168152604060208201525f6148396040830184618185565b818103818111156105a3576105a36187a7565b5f60208284031215618958575f80fd5b81518015158114610623575f80fd5b61012081016189c482886001600160a01b0381511682526001600160a01b03602082015116602083015262ffffff6040820151166040830152606081015160020b60608301526001600160a01b0360808201511660808301525050565b8560020b60a08301528460020b60c08301528360e0830152826101008301529695505050505050565b5f80604083850312156189fe575f80fd5b505080516020909101519092909150565b80820281158282048414176105a3576105a36187a7565b5f60208284031215618a36575f80fd5b5051919050565b5f7f80000000000000000000000000000000000000000000000000000000000000008203618a6d57618a6d6187a7565b505f0390565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f82618aae57618aae618a73565b500490565b5f60208284031215618ac3575f80fd5b8151610623816182ae565b60c08101618b2a82856001600160a01b0381511682526001600160a01b03602082015116602083015262ffffff6040820151166040830152606081015160020b60608301526001600160a01b0360808201511660808301525050565b6001600160a01b03831660a08301529392505050565b5f60208284031215618b50575f80fd5b8151610623816182dd565b5f614810618b72618b6c84886188cc565b866188cc565b846188cc565b602081525f6105a06020830184618185565b5f8160020b8360020b80618ba057618ba0618a73565b5f1981147fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80000083141615618bd557618bd56187a7565b90059392505050565b5f8260020b8260020b028060020b91508082146136a6576136a66187a7565b6fffffffffffffffffffffffffffffffff82811682821603908111156105a3576105a36187a7565b604081525f618c376040830185618185565b82810360208401526148108185618185565b604081525f618c5b6040830185618185565b90508260208301529392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603160045260245ffd5b600282810b9082900b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8000008112627fffff821317156105a3576105a36187a7565b600281810b9083900b01627fffff81137fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff800000821217156105a3576105a36187a7565b5f618d2482856188cc565b7f2c20000000000000000000000000000000000000000000000000000000000000815261481060028201856188cc565b5f6fffffffffffffffffffffffffffffffff82166fffffffffffffffffffffffffffffffff8103618d8757618d876187a7565b60010192915050565b5f618d9b82846188cc565b7f5d0000000000000000000000000000000000000000000000000000000000000081526001019392505050565b5f614839618b7283866188cc565b7fffffff0000000000000000000000000000000000000000000000000000000000831681525f61483960038301846188cc565b7fff00000000000000000000000000000000000000000000000000000000000000851681527fffff000000000000000000000000000000000000000000000000000000000000841660018201527fffffffffffffffffffffffffffffffff00000000000000000000000000000000831660038201525f617da360138301846188cc565b8082018281125f83128015821682158216171561401a5761401a6187a7565b5f82618eb957618eb9618a73565b500690565b838152826020820152606060408201525f6148106060830184618185565b5f8160020b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8000008103618f1057618f106187a7565b5f190192915050565b5f61ffff821661ffff8103618d8757618d876187a7565b5f618f3b82856188cc565b7fffffffffffffffffffffffffffffffff000000000000000000000000000000009390931683525050601001919050565b7fffffff0000000000000000000000000000000000000000000000000000000000851681527fffffffffffffffffffffffffffffffff00000000000000000000000000000000841660038201525f618fc760138301856188cc565b7fffffffffffffffffffffffffffffffffffffffff00000000000000000000000093909316835250506014019392505050565b5f5f19820361900b5761900b6187a7565b5060010190565b600f81810b9083900b016f7fffffffffffffffffffffffffffffff81137fffffffffffffffffffffffffffffffff80000000000000000000000000000000821217156105a3576105a36187a7565b5f8161906e5761906e6187a7565b505f19019056fe6080604052348015600e575f80fd5b506040516118bc3803806118bc833981016040819052602b91604e565b5f80546001600160a01b0319166001600160a01b03929092169190911790556079565b5f60208284031215605d575f80fd5b81516001600160a01b03811681146072575f80fd5b9392505050565b611836806100865f395ff3fe608060405234801561000f575f80fd5b506004361061006f575f3560e01c806391dd73461161004d57806391dd7346146100d4578063baca0004146100f4578063beabacc814610109575f80fd5b80630495a4a2146100735780630c8658791461009957806340e2a812146100c1575b5f80fd5b610086610081366004610d6e565b61011c565b6040519081526020015b60405180910390f35b6100ac6100a7366004610dcd565b61026c565b60408051928352602083019190915201610090565b6100866100cf366004610e66565b6103b8565b6100e76100e2366004610ee5565b61050f565b6040516100909190610f70565b610107610102366004610f82565b6106bf565b005b610107610117366004610f9d565b6106e3565b5f8054819073ffffffffffffffffffffffffffffffffffffffff166348c894918260f81b8860405180606001604052808a151581526020018981526020018873ffffffffffffffffffffffffffffffffffffffff168152506040516020016101859291906110bf565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0818403018152908290526101c19291602001611106565b6040516020818303038152906040526040518263ffffffff1660e01b81526004016101ec9190610f70565b5f604051808303815f875af1158015610207573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016820160405261024c9190810190611209565b9050808060200190518101906102629190611283565b9695505050505050565b5f80548190819073ffffffffffffffffffffffffffffffffffffffff166348c89491600260f81b8a60405180608001604052808c60020b81526020018b60020b81526020018a8152602001898152506040516020016102cc92919061129a565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0818403018152908290526103089291602001611106565b6040516020818303038152906040526040518263ffffffff1660e01b81526004016103339190610f70565b5f604051808303815f875af115801561034e573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01682016040526103939190810190611209565b9050808060200190518101906103a991906112db565b92509250509550959350505050565b5f8054819073ffffffffffffffffffffffffffffffffffffffff166348c89491600160f81b8a60405180606001604052808c151581526020018b81526020018a73ffffffffffffffffffffffffffffffffffffffff16815250888860405160200161042694939291906112fd565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0818403018152908290526104629291602001611106565b6040516020818303038152906040526040518263ffffffff1660e01b815260040161048d9190610f70565b5f604051808303815f875af11580156104a8573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01682016040526104ed9190810190611209565b9050808060200190518101906105039190611283565b98975050505050505050565b5f5460609073ffffffffffffffffffffffffffffffffffffffff163314610534575f80fd5b5f6105426001828587611390565b61054b916113b7565b60f81c600281111561055f5761055f610fdb565b90505f81600281111561057457610574610fdb565b036105bc575f806105888560018189611390565b810190610595919061151b565b915091506105b2828260405180602001604052805f815250610709565b93505050506106b9565b60018160028111156105d0576105d0610fdb565b0361060e575f80806105e5866001818a611390565b8101906105f2919061154f565b925092509250610603838383610709565b9450505050506106b9565b600281600281111561062257610622610fdb565b03610651575f806106368560018189611390565b81019061064391906115e8565b915091506105b282826107d7565b6040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601360248201527f556e7265636f676e697a656420616374696f6e00000000000000000000000000604482015260640160405180910390fd5b505b92915050565b6106df73ffffffffffffffffffffffffffffffffffffffff8216336108fd565b5050565b61070473ffffffffffffffffffffffffffffffffffffffff8416838361097f565b505050565b5f80546040517ff3cd914c0000000000000000000000000000000000000000000000000000000081526060929173ffffffffffffffffffffffffffffffffffffffff169063f3cd914c9061076590889088908890600401611699565b6020604051808303815f875af1158015610781573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107a59190611283565b90506107b185826109d2565b6040805160208101839052016040516020818303038152906040529150505b9392505050565b5f80546040517f5a6bcfda00000000000000000000000000000000000000000000000000000000815260609291829173ffffffffffffffffffffffffffffffffffffffff90911690635a6bcfda906108359088908890600401611763565b60408051808303815f875af1158015610850573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061087491906112db565b9150915061088b8561088684846109f9565b6109d2565b5f8460400151136108d05784515f805490916108bf9173ffffffffffffffffffffffffffffffffffffffff16903090610a48565b90506108ce865f015182610ae4565b505b60408051602081018490529081018290526060016040516020818303038152906040529250505092915050565b5f6370a082315f5230602052602060346024601c865afa601f3d111661092a576390b8ec185f526004601cfd5b8160145260345190506fa9059cbb0000000000000000000000005f5260205f604460105f875af18060015f51141661097457803d853b151710610974576390b8ec185f526004601cfd5b505f60345292915050565b81601452806034526fa9059cbb0000000000000000000000005f5260205f604460105f875af18060015f5114166109c857803d853b1517106109c8576390b8ec185f526004601cfd5b505f603452505050565b81516109e7906109e28360801d90565b610ae4565b6106df82602001516109e283600f0b90565b5f608082811d9084901d01600f83810b9085900b01610a3f610a1a83610ce3565b610a2383610ce3565b6fffffffffffffffffffffffffffffffff1660809190911b1790565b95945050505050565b5f8281526020829052604080822090517ff135baaa00000000000000000000000000000000000000000000000000000000815260048101829052829073ffffffffffffffffffffffffffffffffffffffff87169063f135baaa90602401602060405180830381865afa158015610ac0573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102629190611283565b5f81600f0b1215610c37575f546040517fa584119400000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff84811660048301529091169063a5841194906024015f604051808303815f87803b158015610b58575f80fd5b505af1158015610b6a573d5f803e3d5ffd5b50505f8054610baa935073ffffffffffffffffffffffffffffffffffffffff868116935016908490036fffffffffffffffffffffffffffffffff1661097f565b5f8054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166311da60b46040518163ffffffff1660e01b81526004016020604051808303815f875af1158015610c13573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107049190611283565b80600f0b5f12156106df575f546040517f0b0d9c0900000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff84811660048301523060248301526fffffffffffffffffffffffffffffffff8416604483015290911690630b0d9c09906064015f604051808303815f87803b158015610cc9575f80fd5b505af1158015610cdb573d5f803e3d5ffd5b505050505050565b80600f81900b8114610d1857610d187f93dafdf100000000000000000000000000000000000000000000000000000000610d1d565b919050565b805f5260045ffd5b5f60a08284031215610d35575f80fd5b50919050565b80358015158114610d18575f80fd5b73ffffffffffffffffffffffffffffffffffffffff81168114610d6b575f80fd5b50565b5f805f806101008587031215610d82575f80fd5b610d8c8686610d25565b9350610d9a60a08601610d3b565b925060c0850135915060e0850135610db181610d4a565b939692955090935050565b8035600281900b8114610d18575f80fd5b5f805f805f6101208688031215610de2575f80fd5b610dec8787610d25565b9450610dfa60a08701610dbc565b9350610e0860c08701610dbc565b9497939650939460e08101359450610100013592915050565b5f8083601f840112610e31575f80fd5b50813567ffffffffffffffff811115610e48575f80fd5b602083019150836020828501011115610e5f575f80fd5b9250929050565b5f805f805f806101208789031215610e7c575f80fd5b610e868888610d25565b9550610e9460a08801610d3b565b945060c0870135935060e0870135610eab81610d4a565b925061010087013567ffffffffffffffff811115610ec7575f80fd5b610ed389828a01610e21565b979a9699509497509295939492505050565b5f8060208385031215610ef6575f80fd5b823567ffffffffffffffff811115610f0c575f80fd5b610f1885828601610e21565b90969095509350505050565b5f81518084528060208401602086015e5f6020828601015260207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011685010191505092915050565b602081525f6107d06020830184610f24565b5f60208284031215610f92575f80fd5b81356107d081610d4a565b5f805f60608486031215610faf575f80fd5b8335610fba81610d4a565b92506020840135610fca81610d4a565b929592945050506040919091013590565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b803562ffffff81168114610d18575f80fd5b803561102581610d4a565b73ffffffffffffffffffffffffffffffffffffffff168252602081013561104b81610d4a565b73ffffffffffffffffffffffffffffffffffffffff16602083015262ffffff61107660408301611008565b16604083015261108860608201610dbc565b60020b6060830152608081013561109e81610d4a565b73ffffffffffffffffffffffffffffffffffffffff81166080840152505050565b61010081016110ce828561101a565b8251151560a0830152602083015160c0830152604083015173ffffffffffffffffffffffffffffffffffffffff1660e08301526107d0565b7fff00000000000000000000000000000000000000000000000000000000000000831681525f82518060208501600185015e5f92016001019182525092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b604051601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016810167ffffffffffffffff811182821017156111bc576111bc611148565b604052919050565b5f67ffffffffffffffff8211156111dd576111dd611148565b50601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01660200190565b5f60208284031215611219575f80fd5b815167ffffffffffffffff81111561122f575f80fd5b8201601f8101841361123f575f80fd5b805161125261124d826111c4565b611175565b818152856020838501011115611266575f80fd5b8160208401602083015e5f91810160200191909152949350505050565b5f60208284031215611293575f80fd5b5051919050565b61012081016112a9828561101a565b8251600290810b60a08401526020840151900b60c0830152604083015160e083015260608301516101008301526107d0565b5f80604083850312156112ec575f80fd5b505080516020909101519092909150565b611307818661101a565b8351151560a0820152602084015160c0820152604084015173ffffffffffffffffffffffffffffffffffffffff1660e08201526101206101008201528161012082015281836101408301375f81830161014090810191909152601f9092017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01601019392505050565b5f808585111561139e575f80fd5b838611156113aa575f80fd5b5050820193919092039150565b80357fff0000000000000000000000000000000000000000000000000000000000000081169060018410156106b7577fff00000000000000000000000000000000000000000000000000000000000000808560010360031b1b82161691505092915050565b5f60a0828403121561142c575f80fd5b60405160a0810167ffffffffffffffff8111828210171561144f5761144f611148565b604052905080823561146081610d4a565b8152602083013561147081610d4a565b602082015261148160408401611008565b604082015261149260608401610dbc565b606082015260808301356114a581610d4a565b6080919091015292915050565b5f606082840312156114c2575f80fd5b6040516060810167ffffffffffffffff811182821017156114e5576114e5611148565b6040529050806114f483610d3b565b815260208381013590820152604083013561150e81610d4a565b6040919091015292915050565b5f80610100838503121561152d575f80fd5b611537848461141c565b91506115468460a085016114b2565b90509250929050565b5f805f6101208486031215611562575f80fd5b61156c858561141c565b925061157b8560a086016114b2565b915061010084013567ffffffffffffffff811115611597575f80fd5b8401601f810186136115a7575f80fd5b80356115b561124d826111c4565b8181528760208385010111156115c9575f80fd5b816020840160208301375f602083830101528093505050509250925092565b5f808284036101208112156115fb575f80fd5b611605858561141c565b925060807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6082011215611636575f80fd5b506040516080810167ffffffffffffffff8111828210171561165a5761165a611148565b60405261166960a08501610dbc565b815261167760c08501610dbc565b602082015260e084013560408201526101009093013560608401525092909150565b611718818573ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b8251151560a0820152602083015160c0820152604083015173ffffffffffffffffffffffffffffffffffffffff1660e08201526101206101008201525f610a3f610120830184610f24565b6117e2818473ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b8151600290810b60a08301526020830151900b60c0820152604082015160e0820152606082015161010082015261014061012082018190525f90820152610160019291505056fea164736f6c634300081a000a5741524e494e470a5741524e494e473a2041626f766520736f6d65686f772063616c6c6564207769746820646f6e61746520746f2063757272656e74206f6e6c793f3f3fa164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\x025W_5`\xE0\x1C\x80c\x85v \xE1\x11a\x01=W\x80c\xB0FO\xDC\x11a\0\xB8W\x80c\xBAAO\xA6\x11a\0\x88W\x80c\xE2\x0C\x9Fq\x11a\0nW\x80c\xE2\x0C\x9Fq\x14a\x04\xD6W\x80c\xEE\xE8\xE6{\x14a\x04\xDEW\x80c\xFAv&\xD4\x14a\x04\xF1W_\x80\xFD[\x80c\xBAAO\xA6\x14a\x04\xBEW\x80c\xCC\x7F\x0C$\x14a\x04\x93W_\x80\xFD[\x80c\xB0FO\xDC\x14a\x04\x9BW\x80c\xB1Zyq\x14a\x04\xA3W\x80c\xB1e\xC9\xE9\x14a\x04\x80W\x80c\xB5P\x8A\xA9\x14a\x04\xB6W_\x80\xFD[\x80c\x91j\x17\xC6\x11a\x01\rW\x80c\xAB<~R\x11a\0\xF3W\x80c\xAB<~R\x14a\x04xW\x80c\xAC\xEB\x0E\x85\x14a\x04\x80W\x80c\xAE\xB8\xFB\xF9\x14a\x04\x93W_\x80\xFD[\x80c\x91j\x17\xC6\x14a\x04DW\x80c\x93\xDBE\xE0\x14a\x04YW_\x80\xFD[\x80c\x85v \xE1\x14a\x03\xECW\x80c\x88\xBD\xCAa\x14a\x03\xFFW\x80c\x89\x85\xC9\x0B\x14a\x04\x12W\x80c\x8F]#\xCE\x14a\x04%W_\x80\xFD[\x80c^\xE4\x81\xB6\x11a\x01\xCDW\x80cv\xE1\xFC\xC4\x11a\x01\x9DW\x80c\x80h\xB5.\x11a\x01\x83W\x80c\x80h\xB5.\x14a\x03\x99W\x80c\x82qnC\x14a\x03\xB8W\x80c\x85\"l\x81\x14a\x03\xD7W_\x80\xFD[\x80cv\xE1\xFC\xC4\x14a\x03fW\x80c{*\xBD\xB6\x14a\x03yW_\x80\xFD[\x80c^\xE4\x81\xB6\x14a\x02\xFDW\x80cd#\x9C\xDD\x14a\x03\x1CW\x80cf\xD9\xA9\xA0\x14a\x031W\x80ctw\xF5\x17\x14a\x03FW_\x80\xFD[\x80c*\xDE8\x80\x11a\x02\x08W\x80c*\xDE8\x80\x14a\x02\xD0W\x80c>^<#\x14a\x02\xE5W\x80c?r\x86\xF4\x14a\x02\xEDW\x80cG\x8D\xDE\xCC\x14a\x02\xF5W_\x80\xFD[\x80c\x06\x8B\xCD\x8D\x14a\x029W\x80c\r^\xC4\xC6\x14a\x02zW\x80c\x1E\xD7\x83\x1C\x14a\x02\x9BW\x80c(\x95\xA2\xB3\x14a\x02\xB0W[_\x80\xFD[a\x02La\x02G6`\x04a\x80\xCCV[a\x04\xFEV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x93\x90\x92\x16` \x84\x01R`\x02\x0B\x90\x82\x01R``\x01[`@Q\x80\x91\x03\x90\xF3[a\x02\x8Da\x02\x886`\x04a\x80\xE3V[a\x05\x95V[`@Q\x90\x81R` \x01a\x02qV[a\x02\xA3a\x05\xA9V[`@Qa\x02q\x91\x90a\x81\x03V[a\x02\xC3a\x02\xBE6`\x04a\x80\xCCV[a\x06\tV[`@Qa\x02q\x91\x90a\x81NV[a\x02\xD8a\x06*V[`@Qa\x02q\x91\x90a\x82\rV[a\x02\xA3a\x07fV[a\x02\xA3a\x07\xC4V[a\x02\xA3a\x08\"V[a\x02\x8Da\x03\x0B6`\x04a\x82\xC2V[`*` R_\x90\x81R`@\x90 T\x81V[a\x03/a\x03*6`\x04a\x82\xEBV[a\x083V[\0[a\x039a\x12\xFCV[`@Qa\x02q\x91\x90a\x83\x95V[a\x03Ya\x03T6`\x04a\x80\xE3V[a\x14uV[`@Qa\x02q\x91\x90a\x84}V[a\x02\x8Da\x03t6`\x04a\x80\xE3V[a\x15\xBCV[a\x03\x8Ca\x03\x876`\x04a\x80\xCCV[a\x15\xC7V[`@Qa\x02q\x91\x90a\x84\xF6V[a\x02\x8Da\x03\xA76`\x04a\x82\xC2V[`,` R_\x90\x81R`@\x90 T\x81V[a\x02\x8Da\x03\xC66`\x04a\x82\xC2V[`-` R_\x90\x81R`@\x90 T\x81V[a\x03\xDFa\x16VV[`@Qa\x02q\x91\x90a\x85NV[a\x03/a\x03\xFA6`\x04a\x85`V[a\x17!V[a\x02\x8Da\x04\r6`\x04a\x80\xCCV[a\x1F\x9DV[a\x02\x8Da\x04 6`\x04a\x80\xE3V[a\x1F\xF7V[a\x02\x8Da\x0436`\x04a\x82\xC2V[`+` R_\x90\x81R`@\x90 T\x81V[a\x04La \x02V[`@Qa\x02q\x91\x90a\x85\x89V[a\x02\x8Da\x04g6`\x04a\x82\xC2V[`)` R_\x90\x81R`@\x90 T\x81V[`.Ta\x02\x8DV[a\x02\x8Da\x04\x8E6`\x04a\x80\xE3V[a \xF8V[a\x02\xA3a!\x03V[a\x04La!\x0FV[a\x03/a\x04\xB16`\x04a\x860V[a\"\x05V[a\x03\xDFa'\xCCV[a\x04\xC6a(\x97V[`@Q\x90\x15\x15\x81R` \x01a\x02qV[a\x02\xA3a)gV[a\x03/a\x04\xEC6`\x04a\x86\xF6V[a)\xC5V[`\x1FTa\x04\xC6\x90`\xFF\x16\x81V[_\x80_\x80`.\x85\x81T\x81\x10a\x05\x15Wa\x05\x15a\x87zV[\x90_R` _ \x90`\x03\x02\x01\x90P` `\x05\x01\x81_\x01T\x81T\x81\x10a\x05<Wa\x05<a\x87zV[_\x91\x82R` \x90\x91 \x01T`\x01\x82\x01T`%\x80T`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x96P\x91\x81\x10a\x05lWa\x05la\x87zV[_\x91\x82R` \x90\x91 \x01T`\x02\x91\x82\x01T\x94\x96`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x95P\x93\x90\x0B\x92\x91PPV[_a\x05\xA0\x82\x84a\x87\xD4V[\x90P[\x92\x91PPV[```\x16\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x05\xFFW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x05\xE1W[PPPPP\x90P\x90V[_\x81\x81R`3` R`@\x81 ``\x91\x90a\x06#\x90a4\x10V[\x93\x92PPPV[```\x1E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x07]W_\x84\x81R` \x80\x82 `@\x80Q\x80\x82\x01\x82R`\x02\x87\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x95\x91\x94\x86\x81\x01\x94\x91\x92\x90\x84\x01[\x82\x82\x10\x15a\x07FW\x83\x82\x90_R` _ \x01\x80Ta\x06\xBB\x90a\x87\xE7V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\xE7\x90a\x87\xE7V[\x80\x15a\x072W\x80`\x1F\x10a\x07\tWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x072V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07\x15W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x06\x9EV[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x06MV[PPPP\x90P\x90V[```\x18\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x05\xFFW` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x05\xE1WPPPPP\x90P\x90V[```\x17\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x05\xFFW` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x05\xE1WPPPPP\x90P\x90V[``a\x08.`(a4\xC9V[\x90P\x90V[\x83_a\x08?`'a5\x9DV[\x90Pa\x08L\x82_\x83a5\xEEV[\x91P\x80\x82\x10\x15a\x08\xA9Wa\x08a`'\x83a6;V[`5\x80T`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x83\x16\x81\x17\x90\x91U`6\x80T\x90\x92\x16\x17\x90Ua\nMV[`#T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90a\x08\xC4\x90a\x80\x8DV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x08\xEDW=_\x80>=_\xFD[P`6\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x92\x83\x17\x90\x91U`5\x80T\x90\x91\x16\x82\x17\x90U`@Q\x7Fi\0\xA3\xAE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01\x84\x01`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\xC6W\xC7\x18\x91\x83\x90ci\0\xA3\xAE\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xA3W=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\t\xCA\x91\x90\x81\x01\x90a\x888V[`@Q` \x01a\t\xDA\x91\x90a\x88\xE3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\x06\x92\x91\x90a\x89\x14V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\n\x1DW_\x80\xFD[PZ\xF1\x15\x80\x15a\n/W=_\x80>=_\xFD[PP`5Ta\nK\x92P`'\x91P`\x01`\x01`\xA0\x1B\x03\x16a6\xADV[P[Pa\nj\x86_`\x01`.\x80T\x90Pa\ne\x91\x90a\x895V[a5\xEEV[\x95P_`.\x87\x81T\x81\x10a\n\x80Wa\n\x80a\x87zV[_\x91\x82R` \x82 `\x02`\x03\x90\x92\x02\x01\x81\x81\x01T\x90\x93P\x82\x91a\n\xA4\x91\x90\x0Ba8\x98V[\x91P\x91Pa\n\xE5\x83`\x02\x01_\x90T\x90a\x01\0\n\x90\x04`\x02\x0Ba\n\xD0\x89`\x02\x0B\x85`\x02\x0B\x85`\x02\x0Ba8\xF0V[`\x02\x0B\x90_\x81\x83\x07\x12\x91\x81\x90\x05\x91\x90\x91\x03\x02\x90V[\x96Pa\x0B\x0F\x83`\x02\x01_\x90T\x90a\x01\0\n\x90\x04`\x02\x0Ba\n\xD0\x88`\x02\x0B\x85`\x02\x0B\x85`\x02\x0Ba8\xF0V[`@Q\x7FLc\xE5b\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x02\x82\x81\x0B\x90\x8A\x90\x0B\x14\x15`\x04\x82\x01R\x90\x96Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90cLc\xE5b\x90`$\x01_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0B}W_\x80\xFD[PZ\xFA\x15\x80\x15a\x0B\x8FW=_\x80>=_\xFD[PPPP\x86`\x02\x0B\x86`\x02\x0B\x12\x15a\x0B\xA5W\x94\x95\x94[PP`$T\x81T`%\x80T_\x93a\x0C&\x93`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92\x91\x81\x10a\x0B\xD1Wa\x0B\xD1a\x87zV[_\x91\x82R` \x90\x91 \x01T`\x01\x85\x01T`%\x80T`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x90\x91\x90\x81\x10a\x0C\x02Wa\x0C\x02a\x87zV[_\x91\x82R` \x90\x91 \x01T`\x02\x80\x87\x01T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x0Ba9\xC7V[\x90P_a\x0C4\x82`\xA0\x90 \x90V[\x90P_\x80_a\x0CE\x84\x8B\x8B\x89a:$V[`@Q\x7FLc\xE5b\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x15\x15`\x04\x82\x01R\x92\x95P\x90\x93P\x91Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90cLc\xE5b\x90`$\x01_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0C\xC2W_\x80\xFD[PZ\xFA\x15\x80\x15a\x0C\xD4W=_\x80>=_\xFD[PPPPa\x0C\xF6\x88`\x01\x85o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a5\xEEV[`\x02\x80\x88\x01T`#T\x92\x9APa\r\x1A\x92`\x01`\x01`\xA0\x1B\x03\x16\x91\x87\x91\x8E\x91\x0Ba=JV[a\r5W_\x84\x81R`0` R`@\x90 a\r5\x90\x8Ba=\xA0V[`\x02\x80\x87\x01T`#Ta\rY\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91\x87\x91\x8D\x91\x90\x0Ba=JV[a\rtW_\x84\x81R`0` R`@\x90 a\rt\x90\x8Aa=\xA0V[\x85T`&\x80T_\x92\x90\x81\x10a\r\x8BWa\r\x8Ba\x87zV[_\x91\x82R` \x82 \x01T`\x01\x89\x01T`&\x80T`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x94P\x91\x81\x10a\r\xBAWa\r\xBAa\x87zV[_\x91\x82R` \x90\x91 \x01T`5T`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x87\x90R\x91\x81\x16\x92P\x83\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0E2W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0EV\x91\x90a\x89HV[P`5T`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x90\x82\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0E\xC0W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xE4\x91\x90a\x89HV[P`6T`\x02\x89\x81\x01T`@\x80Q`\xA0\x81\x01\x82R_\x91\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x81\x16` \x83\x01R\x91\x90\x92\x0B``\x83\x01R\x90\x91\x16\x90c\x0C\x86Xy\x90\x8E\x8E\x8E_\x80\x1B`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0FU\x95\x94\x93\x92\x91\x90a\x89gV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0FpW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x94\x91\x90a\x89\xEDV[PP\x87T`%\x80T_\x94P\x90\x92P\x81\x10a\x0F\xB0Wa\x0F\xB0a\x87zV[_\x91\x82R` \x82 \x01T`\x01\x89\x01T`%\x80T`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x94P\x91\x81\x10a\x0F\xDFWa\x0F\xDFa\x87zV[_\x91\x82R` \x90\x91 \x01T`5T`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x87\x90R\x91\x81\x16\x92P\x83\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x10WW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10{\x91\x90a\x89HV[P`5T`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x90\x82\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x10\xE5W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\t\x91\x90a\x89HV[P`6_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x0C\x86Xy\x88\x8E\x8E\x8E_\x80\x1B`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11T\x95\x94\x93\x92\x91\x90a\x89gV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x11oW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x93\x91\x90a\x89\xEDV[PP`6T_\x97Pa\x11\xB9\x96P\x8C\x95P\x8B\x94P`\x01`\x01`\xA0\x1B\x03\x16\x92Pa>\xBD\x91PPV[\x90P_\x88\x81R`4` R`@\x90 a\x11\xD2\x90\x82a?\x0CV[P_\x88\x81R`2` \x90\x81R`@\x80\x83 \x84\x84R\x82R\x80\x83 \x8B\x84R`3\x90\x92R\x90\x91 a\x12\0\x90\x83a?\x0CV[\x15a\x12\x7FW\x80Tb\xFF\xFF\xFF\x87\x81\x16c\x01\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\x90\x92\x16\x90\x89\x16\x17\x17\x80\x82U`6Tf\x01\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x17\x81U[\x84\x81`\x03\x01_\x82\x82Ta\x12\x92\x91\x90a\x87\xD4V[\x90\x91UPP`@\x80Q``\x81\x01\x82R\x95\x86R_\x99\x8AR`1` \x90\x81R\x81\x8B T\x81\x88\x01\x90\x81R_\x19\x92\x88\x01\x92\x83R`\x05\x90\x93\x01\x80T`\x01\x81\x81\x01\x83U\x91\x8DR\x91\x90\x9B \x96Q`\x03\x90\x91\x02\x90\x96\x01\x95\x86U\x90Q\x98\x85\x01\x98\x90\x98UPP\x94Q`\x02\x90\x91\x01UPPPPV[```\x1B\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x07]W\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01\x80Ta\x13O\x90a\x87\xE7V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13{\x90a\x87\xE7V[\x80\x15a\x13\xC6W\x80`\x1F\x10a\x13\x9DWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13\xC6V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13\xA9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x14]W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x14\nW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x13\x1FV[a\x14\xC5`@Q\x80a\x01\0\x01`@R\x80_`\x02\x0B\x81R` \x01_`\x02\x0B\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01``\x81RP\x90V[_\x83\x81R`2` \x90\x81R`@\x80\x83 \x85\x84R\x82R\x80\x83 \x81Qa\x01\0\x81\x01\x83R\x81T`\x02\x81\x81\x0B\x83Rc\x01\0\0\0\x82\x04\x81\x0B\x83\x87\x01Rf\x01\0\0\0\0\0\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x82\x85\x01R`\x01\x83\x01T``\x83\x01R\x82\x01T`\x80\x82\x01R`\x03\x82\x01T`\xA0\x82\x01R`\x04\x82\x01T`\xC0\x82\x01R`\x05\x82\x01\x80T\x84Q\x81\x87\x02\x81\x01\x87\x01\x90\x95R\x80\x85R\x91\x95\x92\x94`\xE0\x87\x01\x94\x93\x91\x92\x91\x90\x84\x01[\x82\x82\x10\x15a\x15\xADW\x83\x82\x90_R` _ \x90`\x03\x02\x01`@Q\x80``\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x15`V[PPP\x91RP\x90\x94\x93PPPPV[_a\x05\xA0\x82\x84a\x8A\x0FV[```1_\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x16KW_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R\x90\x84\x01T`\x02\x81\x90\x0B\x82Rc\x01\0\0\0\x90\x04o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x83\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x15\xF9V[PPPP\x90P\x91\x90PV[```\x1A\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x07]W\x83\x82\x90_R` _ \x01\x80Ta\x16\x96\x90a\x87\xE7V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x16\xC2\x90a\x87\xE7V[\x80\x15a\x17\rW\x80`\x1F\x10a\x16\xE4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x17\rV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x16\xF0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x16yV[a\x178\x83_`\x01`.\x80T\x90Pa\ne\x91\x90a\x895V[_\x81\x81R`4` R`@\x81 \x91\x94P\x90a\x17R\x90a@\"V[`@Q\x7FLc\xE5b\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x15\x15`\x04\x82\x01R\x90\x91Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90cLc\xE5b\x90`$\x01_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x17\xB8W_\x80\xFD[PZ\xFA\x15\x80\x15a\x17\xCAW=_\x80>=_\xFD[PPPP_a\x17\xF7a\x17\xE4\x85_`\x01\x86a\ne\x91\x90a\x895V[_\x87\x81R`4` R`@\x90 \x90a@pV[_\x86\x81R`2` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x81 `\x03\x81\x01T\x92\x93P\x91a\x18#\x91\x86\x91a5\xEEV[\x93P_`.\x87\x81T\x81\x10a\x189Wa\x189a\x87zV[_\x91\x82R` \x82 `$T`\x03\x90\x92\x02\x01\x80T`%\x80T\x92\x95Pa\x18r\x93`\x01`\x01`\xA0\x1B\x03\x16\x92\x90\x91\x90\x81\x10a\x0B\xD1Wa\x0B\xD1a\x87zV[`$T\x90\x91P_\x90`\x01`\x01`\xA0\x1B\x03\x16c\xD8\x94\x11Da\x18\x93\x84`\xA0\x90 \x90V[\x86T`@Q`\xE0\x84\x90\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R`\x04\x81\x01\x92\x90\x92Rf\x01\0\0\0\0\0\0\x81\x04`\x01`\x01`\xA0\x1B\x03\x16`$\x83\x01R`\x02\x81\x81\x0B`D\x84\x01Rc\x01\0\0\0\x90\x91\x04\x90\x0B`d\x82\x01R_`\x84\x82\x01R`\xA4\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\x1EW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19B\x91\x90a\x8A&V[\x90P\x80`+_` `\x05\x01\x86_\x01T\x81T\x81\x10a\x19aWa\x19aa\x87zV[_\x91\x82R` \x80\x83 \x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16\x83R\x82\x01\x92\x90\x92R`@\x01\x81 \x80T\x90\x91\x90a\x19\x94\x90\x84\x90a\x87\xD4V[\x92PP\x81\x90UP\x80\x84`\x02\x01_\x82\x82Ta\x19\xAE\x91\x90a\x87\xD4V[\x90\x91UPP\x83T`\x01`\x01`\xA0\x1B\x03f\x01\0\0\0\0\0\0\x82\x04\x16\x90c\x0C\x86Xy\x90\x84\x90`\x02\x81\x81\x0B\x91c\x01\0\0\0\x90\x04\x90\x0Ba\x19\xE9\x8Ca\x8A=V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x87\x90\x1B\x16\x81Ra\x1A'\x94\x93\x92\x91\x90_\x90`\x04\x01a\x89gV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1ABW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Af\x91\x90a\x89\xEDV[PP\x83T\x83T`%\x80Tf\x01\0\0\0\0\0\0\x90\x93\x04`\x01`\x01`\xA0\x1B\x03\x16\x92c\xBA\xCA\0\x04\x92\x90\x81\x10a\x1A\x9AWa\x1A\x9Aa\x87zV[_\x91\x82R` \x90\x91 \x01T`@Q`\xE0\x83\x90\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1A\xFBW_\x80\xFD[PZ\xF1\x15\x80\x15a\x1B\rW=_\x80>=_\xFD[PP\x85T`\x01\x86\x01T`%\x80Tf\x01\0\0\0\0\0\0\x90\x93\x04`\x01`\x01`\xA0\x1B\x03\x16\x94Pc\xBA\xCA\0\x04\x93P\x91\x81\x10a\x1BFWa\x1BFa\x87zV[_\x91\x82R` \x90\x91 \x01T`@Q`\xE0\x83\x90\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1B\xA7W_\x80\xFD[PZ\xF1\x15\x80\x15a\x1B\xB9W=_\x80>=_\xFD[PP\x84T`&\x80T_\x94P\x90\x92P\x81\x10a\x1B\xD5Wa\x1B\xD5a\x87zV[_\x91\x82R` \x82 \x01T`\x01\x86\x01T`&\x80T`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x94P\x91\x81\x10a\x1C\x04Wa\x1C\x04a\x87zV[_\x91\x82R` \x80\x83 \x90\x91\x01T\x88T`\x02\x89\x81\x01T`@\x80Q`\xA0\x81\x01\x82R\x90\x81\x01\x87\x90R`\x80\x81\x01\x96\x90\x96R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16\x87R\x93\x84\x16\x94\x86\x01\x85\x90R\x90\x0B``\x85\x01R\x91\x93Pf\x01\0\0\0\0\0\0\x90\x91\x04\x16\x90c\x0C\x86Xy\x90\x88T`\x02\x81\x81\x0B\x91c\x01\0\0\0\x90\x04\x90\x0Ba\x1C~\x8Ea\x8A=V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x87\x90\x1B\x16\x81Ra\x1C\xBC\x94\x93\x92\x91\x90_\x90`\x04\x01a\x89gV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1C\xD7W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xFB\x91\x90a\x89\xEDV[PP\x85T`@Q\x7F\xBA\xCA\0\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01Rf\x01\0\0\0\0\0\0\x90\x92\x04\x90\x91\x16\x90c\xBA\xCA\0\x04\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1DdW_\x80\xFD[PZ\xF1\x15\x80\x15a\x1DvW=_\x80>=_\xFD[PP\x87T`@Q\x7F\xBA\xCA\0\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01Rf\x01\0\0\0\0\0\0\x90\x92\x04\x90\x91\x16\x92Pc\xBA\xCA\0\x04\x91P`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1D\xE1W_\x80\xFD[PZ\xF1\x15\x80\x15a\x1D\xF3W=_\x80>=_\xFD[PPPPPP_a\x1E\x05\x83`\xA0\x90 \x90V[\x85T`\x02\x86\x81\x01T`#T\x93\x94Pa\x1E.\x93`\x01`\x01`\xA0\x1B\x03\x16\x92\x85\x92\x90\x81\x0B\x91\x90\x0Ba=JV[a\x1ENW_\x81\x81R`0` R`@\x90 \x85Ta\x1EN\x91\x90`\x02\x0Ba@\xD2V[\x84T`\x02\x85\x81\x01T`#Ta\x1E}\x93`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92\x85\x92c\x01\0\0\0\x90\x92\x04\x81\x0B\x91\x90\x0Ba=JV[a\x1E\xA4W_\x81\x81R`0` R`@\x90 \x85Ta\x1E\xA4\x91\x90c\x01\0\0\0\x90\x04`\x02\x0Ba@\xD2V[PPPP\x83\x81`\x03\x01_\x82\x82Ta\x1E\xBB\x91\x90a\x895V[\x90\x91UPP_\x86\x81R`1` R`@\x90 T`\x04\x82\x01T`\x05\x83\x01\x91\x90[\x82T\x81\x10\x15a\x1F\x11W\x81\x83\x82\x81T\x81\x10a\x1E\xF6Wa\x1E\xF6a\x87zV[_\x91\x82R` \x90\x91 `\x02`\x03\x90\x92\x02\x01\x01U`\x01\x01a\x1E\xDAV[P\x81T`\x04\x84\x01U`\x03\x83\x01T\x15a\x1FzW`@\x80Q``\x81\x01\x82R`\x03\x80\x86\x01T\x82R` \x80\x83\x01\x85\x81R_\x19\x94\x84\x01\x94\x85R`\x05\x88\x01\x80T`\x01\x81\x81\x01\x83U_\x92\x83R\x93\x90\x91 \x94Q\x93\x02\x90\x93\x01\x91\x82U\x91Q\x91\x81\x01\x91\x90\x91U\x90Q`\x02\x90\x91\x01Ua\x1F\x93V[_\x88\x81R`4` R`@\x90 a\x1F\x91\x90\x85aB\x04V[P[PPPPPPPPV[_\x80`.\x83\x81T\x81\x10a\x1F\xB2Wa\x1F\xB2a\x87zV[_\x91\x82R` \x90\x91 `$T`\x03\x90\x92\x02\x01\x80T`%\x80T\x92\x94Pa\x06#\x93a\x1F\xF0\x93`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92\x90\x81\x10a\x0B\xD1Wa\x0B\xD1a\x87zV[`\xA0\x90 \x90V[_a\x05\xA0\x82\x84a\x895V[```\x1D\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x07]W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a \xE0W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a \x8DW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a %V[_a\x05\xA0\x82\x84a\x8A\xA0V[``a\x08.`'a4\xC9V[```\x1C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x07]W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a!\xEDW` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a!\x9AW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a!2V[`%Ta\"\x1C\x90\x87\x90_\x90a\ne\x90`\x01\x90a\x895V[`%T\x90\x96Pa\"6\x90\x86\x90_\x90a\ne\x90`\x01\x90a\x895V[\x94Pa\"A\x83aC\nV[\x92Pa\"L\x82aC\nV[\x91Pa\"{`\x01`\x01`\xA0\x1B\x03\x82\x16d\x01\0\x02v\xA3s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a5\xEEV[\x90P\x84\x86\x03a\"\x9CW`%T`\x01\x87\x01\x81a\"\x98Wa\"\x98a\x8AsV[\x06\x94P[\x84\x86\x11\x15a\"\xA8W\x93\x94\x93[a\"\xBA`\x02\x85\x90\x0B`\x01a\x7F\xFFa8\xF0V[_\x87\x81R`/` \x90\x81R`@\x80\x83 \x89\x84R\x90\x91R\x90 T\x90\x94Pa\"\xE2\x90`\xFF\x16aC\x18V[_\x86\x81R`/` \x90\x81R`@\x80\x83 \x88\x84R\x82R\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U`$T\x81Q\x7F\x9Di\xDCN\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x91Qa#\xB4\x93`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92c\x9Di\xDCN\x92`\x04\x82\x81\x01\x93\x91\x92\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a#\x82W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\xA6\x91\x90a\x8A\xB3V[`\x01`\x01`\xA0\x1B\x03\x16aC\x94V[\x90P_` `\x05\x01\x88\x81T\x81\x10a#\xCDWa#\xCDa\x87zV[_\x91\x82R` \x82 \x01T`&\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x93P\x90\x8A\x90\x81\x10a#\xF9Wa#\xF9a\x87zV[_\x91\x82R` \x82 \x01T`%\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x93P\x90\x8A\x90\x81\x10a$%Wa$%a\x87zV[_\x91\x82R` \x82 \x01T`&\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x93P\x90\x8B\x90\x81\x10a$QWa$Qa\x87zV[_\x91\x82R` \x90\x91 \x01T`!T`@Q\x7F\xCAf\x9F\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x16\x91Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a$\xCFW_\x80\xFD[PZ\xF1\x15\x80\x15a$\xE1W=_\x80>=_\xFD[PP`$\x80T`@Q\x7F\x13\x87\x14e\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16`\x04\x83\x01R\x87\x81\x16\x93\x82\x01\x93\x90\x93Ra\xFF\xFF\x8E\x16`D\x82\x01Rb\xFF\xFF\xFF\x80\x8E\x16`d\x83\x01R\x8C\x16`\x84\x82\x01R_`\xA4\x82\x01R\x91\x16\x92Pc\x13\x87\x14e\x91P`\xC4\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a%oW_\x80\xFD[PZ\xF1\x15\x80\x15a%\x81W=_\x80>=_\xFD[PPPPa%\x99\x84`(a6\xAD\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[Pa%\xA5`(\x83a6\xADV[P`$\x80T`@Q\x7F\x85\x87\xF4P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x85\x81\x16\x93\x82\x01\x93\x90\x93R`D\x81\x01\x88\x90R\x88\x83\x16`d\x82\x01R\x91\x16\x90c\x85\x87\xF4P\x90`\x84\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a&\x1CW_\x80\xFD[PZ\xF1\x15\x80\x15a&.W=_\x80>=_\xFD[PP`#T`@\x80Q`\xA0\x81\x01\x82R_\x81\x83\x01\x81\x90R`\x80\x82\x01R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16\x82R\x86\x81\x16` \x83\x01R`\x02\x8F\x90\x0B``\x83\x01R\x91Q\x7Fbv\xCB\xBE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x91\x90\x92\x16\x93Pcbv\xCB\xBE\x92Pa&\xA9\x91\x90\x8A\x90`\x04\x01a\x8A\xCEV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a&\xC5W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\xE9\x91\x90a\x8B@V[PP`@\x80Q``\x81\x01\x82R\x9A\x8BR` \x8B\x01\x99\x8AR`\x02\x98\x90\x98\x0B\x97\x8A\x01\x97\x88RPP`.\x80T`\x01\x81\x01\x82U_\x91\x90\x91R\x97Q\x7F7\xFA\x16l\xBD\xBF\xBB\x15a\xCC\xD9\xEA\x98^\xC0!\x8B^hP.#\x05%\xF5D([+\xDF=~`\x03\x90\x99\x02\x98\x89\x01UPP\x93Q\x7F7\xFA\x16l\xBD\xBF\xBB\x15a\xCC\xD9\xEA\x98^\xC0!\x8B^hP.#\x05%\xF5D([+\xDF=\x7F\x86\x01UPPQ\x7F7\xFA\x16l\xBD\xBF\xBB\x15a\xCC\xD9\xEA\x98^\xC0!\x8B^hP.#\x05%\xF5D([+\xDF=\x80\x90\x92\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\x16b\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x17\x90\x91UPV[```\x19\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x07]W\x83\x82\x90_R` _ \x01\x80Ta(\x0C\x90a\x87\xE7V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta(8\x90a\x87\xE7V[\x80\x15a(\x83W\x80`\x1F\x10a(ZWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a(\x83V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a(fW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a'\xEFV[`\x08T_\x90`\xFF\x16\x15a(\xAEWP`\x08T`\xFF\x16\x90V[`@Q\x7Ff\x7F\x9Dp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\x04\x82\x01\x81\x90R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x83\x01R_\x91cf\x7F\x9Dp\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)<W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)`\x91\x90a\x8A&V[\x14\x15\x90P\x90V[```\x15\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x05\xFFW` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x05\xE1WPPPPP\x90P\x90V[a)\xDC\x83_`\x01`.\x80T\x90Pa\ne\x91\x90a\x895V[\x92P\x82`8\x81\x90UP_`.\x84\x81T\x81\x10a)\xF9Wa)\xF9a\x87zV[_\x91\x82R` \x82 `$T`\x03\x90\x92\x02\x01\x80T`%\x80T\x92\x95Pa*\x8B\x93a\x1F\xF0\x93`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92\x90\x81\x10a*6Wa*6a\x87zV[_\x91\x82R` \x90\x91 \x01T`\x01\x86\x01T`%\x80T`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x90\x91\x90\x81\x10a*gWa*ga\x87zV[_\x91\x82R` \x90\x91 \x01T`\x02\x80\x88\x01T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x0Ba9\xC7V[`\x02\x80\x84\x01T\x91\x92P_\x91a*\xA3\x91\x84\x91\x90\x0BaC\xAAV[\x80Q\x90\x91Pa*\xB3\x86_\x83a5\xEEV[\x95Pa*\xBDa\x80\x9AV[a*\xD4\x82a*\xCC`\x04\x82a\x8A\xA0V[\x83\x91\x90aErV[_\x87g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\xEEWa*\xEEa\x86\x98V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a+2W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a+\x0CW\x90P[P\x90P_\x80[\x89\x81\x10\x15a.\xBDW_\x86a+L\x8B\x87aE\x8BV[\x81Q\x81\x10a+\\Wa+\\a\x87zV[` \x02` \x01\x01Q\x90P_a+\xB8g\x01cEx]\x8A\0\0a+\x8Eg\r\xE0\xB6\xB3\xA7d\0\0\x8EaE\xA5\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x11\x15a+\xB2Wa+\xAD\x8C`\x01l\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFaF\nV[aG\x07V[_aG\x07V[\x90P`@Q\x80`@\x01`@R\x80\x83`\x02\x0B\x81R` \x01\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x85\x84\x81Q\x81\x10a+\xF6Wa+\xF6a\x87zV[` \x90\x81\x02\x91\x90\x91\x01\x01Ra,\x1Do\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x85a\x87\xD4V[\x93P_\x8D\x81R`1` R`@\x90 \x85Q\x86\x90\x85\x90\x81\x10a,@Wa,@a\x87zV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82T`\x01\x81\x01\x84U_\x93\x84R\x82\x84 \x82Q\x91\x01\x80T\x92\x84\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x01\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x93\x16b\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x91\x90\x91\x17\x90U`7\x82\x90U`8T\x82R`4\x90R`@\x81 a,\xCD\x90a@\"V[\x90P_\x80[\x82\x81\x10\x15a-\x8FW`8T_\x90\x81R`4` R`@\x81 a,\xF4\x90\x83a@pV[`8T_\x90\x81R`2` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 \x80T\x91\x92P\x90`\x02\x88\x81\x0B\x91\x90\x0B\x13\x80\x15\x90a-:WP\x80Tc\x01\0\0\0\x90\x04`\x02\x90\x81\x0B\x90\x88\x90\x0B\x12[\x15a-\x85W`\x03\x81\x01Ta-N\x90\x85a\x87\xD4V[`7\x80T`\x01\x81\x01\x82U_\x91\x90\x91R\x7FB\xA7\xB7\xDDx\\\xD6\x97\x14\xA1\x89\xDF\xFB?\xD7\xD7\x17N\xDC\x9E\xCE\x83v\x94\xCEP\xF7\x07\x8F|1\xAE\x01\x83\x90U\x93P[PP`\x01\x01a,\xD2V[P\x80_\x03a-\xFFW\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`,_` `\x05\x01\x8F_\x01T\x81T\x81\x10a-\xC6Wa-\xC6a\x87zV[_\x91\x82R` \x80\x83 \x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16\x83R\x82\x01\x92\x90\x92R`@\x01\x81 \x80T\x90\x91\x90a-\xF9\x90\x84\x90a\x87\xD4V[\x90\x91UPP[`7T\x91P_[\x82\x81\x10\x15a.\xACW_`7\x82\x81T\x81\x10a.\"Wa.\"a\x87zV[_\x91\x82R` \x80\x83 \x90\x91\x01T`8T\x83R`2\x82R`@\x80\x84 \x82\x85R\x90\x92R\x91 `\x03\x81\x01T\x91\x92P\x90a.\x87\x90\x85a.\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x16p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x8A\x0FV[\x91\x90aG^V[\x81`\x01\x01_\x82\x82Ta.\x99\x91\x90a\x87\xD4V[\x90\x91UPP`\x01\x90\x92\x01\x91Pa.\x06\x90PV[PP`\x01\x90\x93\x01\x92Pa+8\x91PPV[P`#T`\x02\x80\x89\x01T_\x92a.\xE3\x92\x86\x92`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x8B\x91\x0BaG\xECV[\x90P_` `\x05\x01\x89_\x01T\x81T\x81\x10a.\xFFWa.\xFFa\x87zV[_\x91\x82R` \x82 \x01T`\x01\x8B\x01T`%\x80T`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x94P\x91\x81\x10a/.Wa/.a\x87zV[_\x91\x82R` \x80\x83 \x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x84R`*\x90\x92R`@\x83 \x80T\x92\x90\x91\x16\x93P\x86\x92\x90\x91a/h\x90\x84\x90a\x87\xD4V[\x90\x91UPP\x89T`%\x80T_\x92\x90\x81\x10a/\x84Wa/\x84a\x87zV[_\x91\x82R` \x90\x91 \x01T`@Q\x7F@\xC1\x0F\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R`$\x81\x01\x87\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91P\x81\x90c@\xC1\x0F\x19\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a/\xF1W_\x80\xFD[PZ\xF1\x15\x80\x15a0\x03W=_\x80>=_\xFD[PP`$\x80T`@Q\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R_\x19\x92\x81\x01\x92\x90\x92R\x84\x16\x92Pc\t^\xA7\xB3\x91P`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a0mW_\x80\xFD[PZ\xF1\x15\x80\x15a0\x7FW=_\x80>=_\xFD[PP`$\x80T`@Q\x7FG\xE7\xEF$\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x92\x81\x01\x8A\x90R\x91\x16\x92PcG\xE7\xEF$\x91P`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a0\xE6W_\x80\xFD[PZ\xF1\x15\x80\x15a0\xF8W=_\x80>=_\xFD[PP\x85Q\x91PP\x80\x15a3\xF7W`@\x80Q`\x02\x80\x82R``\x82\x01\x90\x92R_\x91\x81` \x01[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81a1\x1CW\x90PP\x90P\x84\x81_\x81Q\x81\x10a1cWa1ca\x87zV[` \x02` \x01\x01Q_\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x83\x81`\x01\x81Q\x81\x10a1\x9AWa1\x9Aa\x87zV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90R`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91\x81` \x01[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81a1\xC9W\x90PP\x90P\x85\x81_\x81Q\x81\x10a2\tWa2\ta\x87zV[` \x02` \x01\x01Q_\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x84\x81_\x81Q\x81\x10a2?Wa2?a\x87zV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x01Ra2gV[`\xA0\x1C`\x02\x0B\x90V[_[\x83\x81\x10\x15a3\xF3W_`@Q\x80`\x80\x01`@R\x80\x89`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x88`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x8A\x84\x81Q\x81\x10a2\xC5Wa2\xC5a\x87zV[` \x90\x81\x02\x91\x90\x91\x01\x01Q\x90R`$T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16c\x97\xDC\x99ea2\xEF\x86aI\x01V[a3r\x87` `\x04\x01_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x9Di\xDCN`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3FW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3j\x91\x90a\x8A\xB3V[\x88\x91\x90aI\x92V[a3|\x85\x88aJ1V[`@Q` \x01a3\x8E\x93\x92\x91\x90a\x8B[V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a3\xB9\x91\x90a\x8BxV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a3\xD0W_\x80\xFD[PZ\xF1\x15\x80\x15a3\xE2W=_\x80>=_\xFD[PP`\x01\x90\x93\x01\x92Pa2i\x91PPV[PPP[a3\xFFaJ\xC6V[PPPPPPPPPPPPPPPV[c\x18\xFBXd`\x04R_\x81\x81R`$\x90 \x80\x19T`@Q\x91\x90h\xFB\xB6\x7F\xDAR\xD4\xBF\xB8\xBF\x90` \x84\x01\x81a4\x89W\x83T\x80\x15a4\x83W\x80\x84\x14\x15\x02\x81R`\x01\x84\x81\x01T\x90\x92P\x80\x15a4\x83W\x80\x84\x14\x15\x02` \x82\x01R`\x02\x84\x81\x01T\x90\x92P\x80\x15a4\x83W`\x03\x92P\x83\x81\x14\x15\x81\x02`@\x83\x01R[Pa4\xB4V[\x81`\x01\x1C\x91P_[\x82\x81\x10\x15a4\xB2W\x84\x81\x01T\x84\x81\x14\x15\x02`\x05\x82\x90\x1B\x83\x01R`\x01\x01a4\x91V[P[\x81\x85R\x81`\x05\x1B\x81\x01`@RPPPP\x91\x90PV[c\x97\x8A\xAB\x92`\x04R_\x81\x81R`$\x81 ``\x91Ph\xFB\xB6\x7F\xDAR\xD4\xBF\xB8\xBF\x81T\x80`\xA0\x1B`\xA0\x1C`@Q\x94P\x84` \x01\x82``\x1C\x92P\x83\x83\x14\x15\x83\x02\x81R\x81a5WW\x82\x15a5RW`\x01\x91P\x81\x85\x01T``\x1C\x92P\x82\x15a5RW\x82\x84\x14\x15\x90\x92\x02` \x83\x01RP`\x02\x83\x81\x01T``\x1C\x91\x82\x15a5RW`\x03\x91P\x83\x83\x14\x15\x83\x02`@\x82\x01R[a5\x87V[`\x01\x91\x82\x1C\x91[\x82\x81\x10\x15a5\x85W\x85\x81\x01T``\x1C\x85\x81\x14\x15\x81\x02`\x05\x83\x90\x1B\x84\x01R\x93P`\x01\x01a5^V[P[\x81\x86R\x81`\x05\x1B\x81\x01`@RPPPPP\x91\x90PV[c\x97\x8A\xAB\x92`\x04R_\x81\x81R`$\x81 \x80T\x80`\xA0\x1B`\xA0\x1C\x80`\x01\x1C\x93P\x80\x82``\x1C\x15\x17a5\xE6W`\x01\x93P\x83\x83\x01T\x15a5\xE6W`\x02\x93P\x83\x83\x01T\x15a5\xE6W`\x03\x93P[PPP\x91\x90PV[_a5\xFA\x84\x84\x84aK\xA4V[\x90Pa\x06#`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01\x7FBound result\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x82aM]V[c\x97\x8A\xAB\x92`\x04R_\x82\x81R`$\x81 \x82\x81\x01T``\x1C\x91Ph\xFB\xB6\x7F\xDAR\xD4\xBF\xB8\xBF\x82\x14\x15\x82\x02\x91Pa6n\x84a5\x9DV[\x83\x10a6\xA6W`@Q\x7FN#\xD05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x92\x91PPV[c\x97\x8A\xAB\x92`\x04R_\x82\x81R`$\x81 `\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04I\x80%\xAD+@GA\x83\x01a6\xFEWc\xF5\xA2g\xF1_R`\x04`\x1C\xFD[\x82a7\x10Wh\xFB\xB6\x7F\xDAR\xD4\xBF\xB8\xBF\x92P[\x80Tk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x82` R\x80a7\xD7W\x81``\x1C\x80a7CW\x85``\x1B\x84U`\x01\x94PPa8\x1BV[\x85\x81\x03a7PWPa8\x1BV[`\x01\x84\x01T``\x1C\x80a7qW\x86``\x1B`\x01\x86\x01U`\x01\x95PPPa8\x1BV[\x86\x81\x03a7\x7FWPPa8\x1BV[`\x02\x85\x01T``\x1C\x80a7\xA1W\x87``\x1B`\x02\x87\x01U`\x01\x96PPPPa8\x1BV[\x87\x81\x03a7\xB0WPPPa8\x1BV[_\x92\x83R`@\x80\x84 `\x01\x90U\x91\x83R\x81\x83 `\x02\x90U\x82R\x90 `\x03\x90UP`\x07\x90\x81\x17\x90[\x84_R`@_ \x80Ta8\x19W`\x01\x91\x82\x1C\x80\x83\x01\x82U\x91\x94P\x81a8\x05W\x85``\x1B`\x03\x17\x84UPa8\x1BV[\x85``\x1B\x82\x85\x01U\x82`\x02\x01\x84UPa8\x1BV[P[PPP\x92\x91PPV[a8\x95\x81`@Q`$\x01a88\x91\x90a\x8BxV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FA0O\xAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90RaM\xD0V[PV[_\x80\x82a8\xC5\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF2v\x18a\x8B\x8AV[a8\xCF\x91\x90a\x8B\xDEV[\x91P\x82a8\xDF\x81b\r\x89\xE8a\x8B\x8AV[a8\xE9\x91\x90a\x8B\xDEV[\x90P\x91P\x91V[_a8\xFC\x84\x84\x84aM\xE0V[`@\x80Q\x80\x82\x01\x82R`\x0C\x81R\x7FBound result\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90Q\x7F\xA3\"\xC4\x0E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90R\x91\x92Pa\x06#\x91sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xA3\"\xC4\x0E\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a9\x9BW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra9\xC2\x91\x90\x81\x01\x90a\x888V[aPAV[`@\x80Q`\xA0\x81\x01\x82R_\x91\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x80\x83\x01\x81\x90R\x85\x82\x16\x83R\x90\x84\x16` \x83\x01R`\x02\x83\x90\x0B``\x83\x01R\x15a:\x10Wb\x80\0\0a:\x12V[_[b\xFF\xFF\xFF\x16`@\x82\x01R\x94\x93PPPPV[`\x02\x81\x81\x01T`#T_\x92\x83\x92\x83\x92\x91\x0B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF2v\x18\x81\x81\x07\x84\x13\x90\x82\x90\x05\x03b\r\x89\xE8\x91\x90\x91\x05\x03`\x01\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x90\x82\x90a:\x95\x90`\x01`\x01`\xA0\x1B\x03\x16\x8A\x8AaP\xB4V[P`#T\x90\x91P_\x90a:\xB2\x90`\x01`\x01`\xA0\x1B\x03\x16\x8B\x8AaP\xB4V[P\x90Pa:\xE3\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aQ\x19V[a:\xED\x90\x84a\x8B\xFDV[`#T\x90\x96P_\x93Pa;\x0C\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90P\x89aH\x19V[\x90P_\x80a;\x1D\x83`\xA0\x1C`\x02\x0B\x90V[\x91PP`\x01`\x01`\xA0\x1B\x03\x82\x16a;7\x82\x82\x8B\x8B\x8AaQ.V[\x88T`%\x80T\x93\x98P\x91\x96P_\x92\x81\x10a;SWa;Sa\x87zV[_\x91\x82R` \x90\x91 \x01T`@Q\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a;\xBAW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;\xDE\x91\x90a\x8A&V[\x90P_` `\x05\x01\x89`\x01\x01T\x81T\x81\x10a;\xFBWa;\xFBa\x87zV[_\x91\x82R` \x90\x91 \x01T`@Q\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a<bW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<\x86\x91\x90a\x8A&V[\x90Pa=&a+\xADa=\x0E\x89\x85\x10a<\xB0W\x8Ao\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a<\xCDV[a<\xCDo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8C\x16\x86\x8CaG^V[\x89\x85\x10a<\xECW\x8Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aQ\xC5V[a=\to\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8D\x16\x86\x8CaG^V[aQ\xC5V[\x8Ao\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aQ\xC5V[\x97Pa=5\x84\x84\x8D\x8D\x8CaQ.V[\x98\x9D\x90\x9CP\x97\x9AP\x96\x98PPPPPPPPPV[_\x80\x80a=h\x84\x86\x07\x82\x13\x85\x87\x05\x03[`\x08\x81\x90\x1D\x91`\xFF\x90\x91\x16\x90V[\x90\x92P\x90Pa=\x95\x81a=\x85`\x01`\x01`\xA0\x1B\x03\x8A\x16\x89\x86aQ\xD3V[\x90`\x01`\xFF\x91\x90\x91\x16\x1B\x16\x15\x15\x90V[\x97\x96PPPPPPPV[\x81T_[\x81\x81\x10\x15a=\xF3W\x83\x81\x81T\x81\x10a=\xBEWa=\xBEa\x87zV[\x90_R` _ \x90`\n\x91\x82\x82\x04\x01\x91\x90\x06`\x03\x02\x90T\x90a\x01\0\n\x90\x04`\x02\x0B`\x02\x0B\x83`\x02\x0B\x12a=\xF3W`\x01\x01a=\xA4V[\x81\x81\x10\x15a>\x80W\x82\x84\x82\x81T\x81\x10a>\x0EWa>\x0Ea\x87zV[\x90_R` _ \x90`\n\x91\x82\x82\x04\x01\x91\x90\x06`\x03\x02\x90T\x90a\x01\0\n\x90\x04`\x02\x0B\x85\x83\x81T\x81\x10a>AWa>Aa\x87zV[_\x91\x82R` \x90\x91 `\n\x80\x83\x04\x90\x91\x01\x80T\x91\x90\x92\x06`\x03\x02a\x01\0\nb\xFF\xFF\xFF\x81\x81\x02\x19\x90\x92\x16\x94\x90\x91\x16\x02\x92\x90\x92\x17\x90\x91U\x92P`\x01\x01a=\xF3V[PP\x81T`\x01\x81\x01\x83U_\x92\x83R` \x90\x92 `\n\x80\x84\x04\x90\x91\x01\x80T\x91\x90\x93\x06`\x03\x02a\x01\0\nb\xFF\xFF\xFF\x81\x81\x02\x19\x90\x92\x16\x92\x90\x91\x16\x02\x17\x90UV[`@\x80Q`\x02\x85\x81\x0B` \x83\x01R\x84\x90\x0B\x91\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x82\x16``\x82\x01R_\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x93\x92PPPV[PPV[c\x18\xFBXd`\x04R_\x82\x81R`$\x81 h\xFB\xB6\x7F\xDAR\xD4\xBF\xB8\xBF\x83\x03a?9Wc\xF5\xA2g\xF1_R`\x04`\x1C\xFD[\x82a?KWh\xFB\xB6\x7F\xDAR\xD4\xBF\xB8\xBF\x92P[\x80\x19T\x81` R\x80a?\xEFW\x81T\x80a?kW\x84\x83U`\x01\x93PPa@\x1AV[\x84\x81\x03a?xWPa@\x1AV[`\x01\x83\x01T\x80a?\x93W\x85`\x01\x85\x01U`\x01\x94PPPa@\x1AV[\x85\x81\x03a?\xA1WPPa@\x1AV[`\x02\x84\x01T\x80a?\xBDW\x86`\x02\x86\x01U`\x01\x95PPPPa@\x1AV[\x86\x81\x03a?\xCCWPPPa@\x1AV[_\x92\x83R`@\x80\x84 `\x01\x90U\x91\x83R\x81\x83 `\x02\x90U\x82R\x90 `\x03\x90UP`\x07[\x83_R`@_ \x80Ta8\x1BW`\x01\x91\x82\x1C\x83\x81\x01\x86\x90U\x80\x83\x01\x91\x82\x90U\x90\x82\x1B\x82\x17\x83\x19U\x90\x92P[PP\x92\x91PPV[c\x18\xFBXd`\x04R_\x81\x81R`$\x81 \x80\x19T\x80`\x01\x1C\x92P\x80a@iW\x81T_\x93P\x15a@iW`\x01\x92P\x82\x82\x01T\x15a@iW`\x02\x92P\x82\x82\x01T\x15a@iW`\x03\x92P[PP\x91\x90PV[c\x18\xFBXd`\x04R_\x82\x81R`$\x90 \x81\x01Th\xFB\xB6\x7F\xDAR\xD4\xBF\xB8\xBF\x81\x14\x15\x02a@\x9A\x83a@\"V[\x82\x10a\x05\xA3W`@Q\x7FN#\xD05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81T_[\x81\x81\x10\x15aA!W\x82`\x02\x0B\x84\x82\x81T\x81\x10a@\xF4Wa@\xF4a\x87zV[_\x91\x82R` \x90\x91 `\n\x80\x83\x04\x90\x91\x01T\x91\x06`\x03\x02a\x01\0\n\x90\x04`\x02\x0B\x14aA!W`\x01\x01a@\xD6V[aA,`\x01\x83a\x895V[\x81\x10\x15aA\xC1W\x83aA?\x82`\x01a\x87\xD4V[\x81T\x81\x10aAOWaAOa\x87zV[\x90_R` _ \x90`\n\x91\x82\x82\x04\x01\x91\x90\x06`\x03\x02\x90T\x90a\x01\0\n\x90\x04`\x02\x0B\x84\x82\x81T\x81\x10aA\x82WaA\x82a\x87zV[\x90_R` _ \x90`\n\x91\x82\x82\x04\x01\x91\x90\x06`\x03\x02a\x01\0\n\x81T\x81b\xFF\xFF\xFF\x02\x19\x16\x90\x83`\x02\x0Bb\xFF\xFF\xFF\x16\x02\x17\x90UP\x80\x80`\x01\x01\x91PPaA!V[\x83\x80T\x80aA\xD1WaA\xD1a\x8CjV[`\x01\x90\x03\x81\x81\x90_R` _ \x90`\n\x91\x82\x82\x04\x01\x91\x90\x06`\x03\x02a\x01\0\n\x81T\x90b\xFF\xFF\xFF\x02\x19\x16\x90U\x90UPPPPV[c\x18\xFBXd`\x04R_\x82\x81R`$\x81 h\xFB\xB6\x7F\xDAR\xD4\xBF\xB8\xBF\x83\x03aB1Wc\xF5\xA2g\xF1_R`\x04`\x1C\xFD[\x82aBCWh\xFB\xB6\x7F\xDAR\xD4\xBF\xB8\xBF\x92P[\x80\x19T\x80aB\xACW`\x01\x92P\x83\x82T\x03aBpW`\x01\x82\x01\x80T\x83U`\x02\x83\x01\x80T\x90\x91U_\x90Ua@\x1AV[\x83`\x01\x83\x01T\x03aB\x8EW`\x02\x82\x01\x80T`\x01\x84\x01U_\x90Ua@\x1AV[\x83`\x02\x83\x01T\x03aB\xA4W_`\x02\x83\x01Ua@\x1AV[_\x92Pa@\x1AV[\x81` R\x83_R`@_ \x80T\x80aB\xC5WPPa@\x1AV[`\x01\x83`\x01\x1C\x03\x92P\x82`\x01\x82\x03\x14aB\xEFW\x82\x84\x01T\x80`\x01\x83\x03\x86\x01U\x80_RP\x80`@_ U[P`\x01\x82`\x01\x1B\x17\x83\x19U_\x81UP`\x01\x92PPP\x92\x91PPV[_a\x05\xA3\x82b\x0FB@aR\x01V[`@Q\x7F\xA5\x98(\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81\x15\x15`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xA5\x98(\x85\x90`$\x01_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15aC{W_\x80\xFD[PZ\xFA\x15\x80\x15aC\x8DW=_\x80>=_\xFD[PPPPPV[_a\x05\xA3` `\x01`\x01`\xA0\x1B\x03\x84\x16;a\x8A\xA0V[`#T``\x90_\x90aC\xCF\x90\x84\x90a\n\xD0\x90a2^\x90`\x01`\x01`\xA0\x1B\x03\x16\x88aH\x19V[\x90P_\x83aC\xE0`\x14a\x01\0a\x8B\xDEV[aC\xEA\x91\x90a\x8B\xDEV[\x90P_aC\xF7\x82\x84a\x8C\x97V[\x90P_aD\x04\x83\x85a\x8C\xD8V[_\x88\x81R`0` R`@\x81 \x80T\x92\x93P\x90\x91[\x80\x83\x10\x15aDhW\x84`\x02\x0B\x82\x84\x81T\x81\x10aD7WaD7a\x87zV[_\x91\x82R` \x90\x91 `\n\x80\x83\x04\x90\x91\x01T\x91\x06`\x03\x02a\x01\0\n\x90\x04`\x02\x0B\x12\x15aDhW`\x01\x90\x92\x01\x91aD\x19V[\x82aDs\x81\x83a\x895V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aD\x8BWaD\x8Ba\x86\x98V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15aD\xB4W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x98P[\x81\x84\x10\x15aE`W\x84`\x02\x0B\x83\x85\x81T\x81\x10aD\xD6WaD\xD6a\x87zV[_\x91\x82R` \x90\x91 `\n\x80\x83\x04\x90\x91\x01T\x91\x06`\x03\x02a\x01\0\n\x90\x04`\x02\x0B\x13aE`W\x82\x84\x81T\x81\x10aE\rWaE\ra\x87zV[\x90_R` _ \x90`\n\x91\x82\x82\x04\x01\x91\x90\x06`\x03\x02\x90T\x90a\x01\0\n\x90\x04`\x02\x0B\x89\x82\x86\x03\x81Q\x81\x10aEBWaEBa\x87zV[`\x02\x92\x90\x92\x0B` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x90\x93\x01\x92aD\xB8V[\x90\x92\x03\x87RP\x94\x97\x96PPPPPPPV[\x81\x83RaE~\x81aR\x17V[\x83` \x01\x81\x90RPPPPV[\x80Q_\x90a\x05\xA0\x90aE\x9E\x90\x85\x90aE\xA5V[\x83\x90aRxV[_\x81`\x01\x03aE\xB5WP_a\x05\xA3V[_\x82aE\xC2\x81_\x19a\x8A\xA0V[aE\xCC\x91\x90a\x8A\x0FV[\x84Q` R\x90P_\x80[\x81_R`@_ \x90P\x82\x81\x10aE\xF1W`\x01\x82\x01\x91PaE\xD6V[\x84\x81\x06\x93PPP`\x03`\x1FSP`!`\x1F \x90\x92RP\x90V[_\x82_\x03aF_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01R\x7FLow 0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x81\x83\x10aF\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7FLow not below high\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[_aF\xB9\x83\x85aSoV[\x90P_aF\xCDaF\xC8\x83aS\xB1V[aT\x0CV[\x90P_aF\xDB\x87\x82\x84aVeV[\x90P_aF\xEFaF\xEA\x83aV\xB7V[aX\x9CV[\x90PaF\xFB\x87\x82aX\xD7V[\x98\x97PPPPPPPPV[_aGZ\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16`@Q\x80`@\x01`@R\x80`\x16\x81R` \x01\x7FUnsafe cast to uint128\0\0\0\0\0\0\0\0\0\0\x81RPaY\x06V[P\x90V[\x82\x82\x02\x81\x83\x85\x83\x04\x14\x85\x15\x17\x02aG\xE5W_\x19\x83\x85\t\x81\x81\x10\x82\x01\x90\x03\x82\x84\x86\t\x83_\x03\x84\x16\x82\x85\x11aG\x98Wc\xAEG\xF7\x02_R`\x04`\x1C\xFD[\x93\x84\x90\x04\x93\x83\x82\x11\x90\x92\x03_\x83\x90\x03\x83\x90\x04`\x01\x01\x02\x92\x03\x04\x17`\x02`\x03\x83\x02\x81\x18\x80\x84\x02\x82\x03\x02\x80\x84\x02\x82\x03\x02\x80\x84\x02\x82\x03\x02\x80\x84\x02\x82\x03\x02\x80\x84\x02\x82\x03\x02\x80\x84\x02\x90\x91\x03\x02\x02a\x06#V[\x04\x92\x91PPV[``aH\x10\x85\x85\x85\x85aH\x0Ba2^`\x01`\x01`\xA0\x1B\x03\x85\x16\x84aH\x19V[aY\x8BV[\x95\x94PPPPPV[_\x81\x81R`\x06` R`@\x81 aH9`\x01`\x01`\xA0\x1B\x03\x85\x16\x82a_;V[\x94\x93PPPPV[` \x02` \x01\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a_kV[\x83Q\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15aH\xD8W\x81aH\xA3\x85\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10aHAWaHAa\x87zV[`@Q` \x01aH\xB4\x92\x91\x90a\x8D\x19V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P\x80\x80aH\xD0\x90a\x8DTV[\x91PPaH`V[P\x80`@Q` \x01aH\xEA\x91\x90a\x8D\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x91\x90PV[``_[\x82Q\x81\x10\x15aI\\W\x81aI1\x84\x83\x81Q\x81\x10aI$WaI$a\x87zV[` \x02` \x01\x01Qa_vV[`@Q` \x01aIB\x92\x91\x90a\x8D\xC8V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x91P`\x01\x01aI\x05V[PaIg\x81Qa`cV[`\xE8\x1B\x81`@Q` \x01aI|\x92\x91\x90a\x8D\xD6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[``_[\x84Q\x81\x10\x15aI\xF9W\x81aI\xCE\x85\x85\x88\x85\x81Q\x81\x10aI\xB7WaI\xB7a\x87zV[` \x02` \x01\x01Qa`w\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[`@Q` \x01aI\xDF\x92\x91\x90a\x8D\xC8V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x91P`\x01\x01aI\x96V[PaJ\x04\x81Qa`cV[`\xE8\x1B\x81`@Q` \x01aJ\x19\x92\x91\x90a\x8D\xD6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x93\x92PPPV[``_\x80aJQ\x85_\x01Q\x86` \x01Q\x86aa6\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91P\x91P_\x85``\x01Q_\x01QaJhW_aJkV[`\x02[\x82aJvW_aJyV[`\x01[\x17\x90P\x80`\xF8\x1B\x83`\xF0\x1B\x87`@\x01Q`\x80\x1BaJ\x99\x89``\x01Qab\xB8V[`@Q` \x01aJ\xAC\x94\x93\x92\x91\x90a\x8E\tV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x93PPPP\x92\x91PPV[_aJ\xD1`(a5\x9DV[\x90P_[\x81\x81\x10\x15a?\x08W_aJ\xE9`(\x83a6;V[`$\x80T`@Q\x7F\xD5q\x97x\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\x04\x83\x01R\x93\x94P\x92\x16\x91c\xD5q\x97x\x91\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aKKW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aKo\x91\x90a\x8A&V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`-` R`@\x81 \x80T\x90\x91\x90aK\x96\x90\x84\x90a\x8E\x8CV[\x90\x91UPPP`\x01\x01aJ\xD5V[_\x81\x83\x11\x15aL\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R`\x84\x01aFVV[\x82\x84\x10\x15\x80\x15aL+WP\x81\x84\x11\x15[\x15aL7WP\x82a\x06#V[_aLB\x84\x84a\x895V[aLM\x90`\x01a\x87\xD4V[\x90P`\x03\x85\x11\x15\x80\x15aL_WP\x84\x81\x11[\x15aLvWaLn\x85\x85a\x87\xD4V[\x91PPa\x06#V[aL\x82`\x03_\x19a\x895V[\x85\x10\x15\x80\x15aL\x9AWPaL\x97\x85_\x19a\x895V[\x81\x11[\x15aL\xB4WaL\xAA\x85_\x19a\x895V[aLn\x90\x84a\x895V[\x82\x85\x11\x15aM\x07W_aL\xC7\x84\x87a\x895V[\x90P_aL\xD4\x83\x83a\x8E\xABV[\x90P\x80_\x03aL\xE8W\x84\x93PPPPa\x06#V[`\x01aL\xF4\x82\x88a\x87\xD4V[aL\xFE\x91\x90a\x895V[\x93PPPaMUV[\x83\x85\x10\x15aMUW_aM\x1A\x86\x86a\x895V[\x90P_aM'\x83\x83a\x8E\xABV[\x90P\x80_\x03aM;W\x85\x93PPPPa\x06#V[aME\x81\x86a\x895V[aMP\x90`\x01a\x87\xD4V[\x93PPP[P\x93\x92PPPV[a?\x08\x82\x82`@Q`$\x01aMs\x92\x91\x90a\x8CIV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xB6\x0Er\xCC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Rac\xCCV[a8\x95\x81ac\xD8[c\xFF\xFF\xFF\xFF\x16V[_\x81\x83\x13\x15aNWW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`;`$\x82\x01R\x7FStdUtils bound(int256,int256,int`D\x82\x01R\x7F256): Max is less than min.\0\0\0\0\0`d\x82\x01R`\x84\x01aFVV[_\x80\x85\x12aN\x8EWaN\x89\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86a\x87\xD4V[aN\xC5V[`\x01aN\xBB\x86\x19\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x895V[aN\xC5\x91\x90a\x895V[\x90P_\x80\x85\x12aN\xFEWaN\xF9\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86a\x87\xD4V[aO5V[`\x01aO+\x86\x19\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x895V[aO5\x91\x90a\x895V[\x90P_\x80\x85\x12aOnWaOi\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86a\x87\xD4V[aO\xA5V[`\x01aO\x9B\x86\x19\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x895V[aO\xA5\x91\x90a\x895V[\x90P_aO\xB3\x84\x84\x84aK\xA4V[\x90P\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x10aP\x0BWaP\x06\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82a\x895V[aF\xFBV[aP5\x81\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x895V[aF\xFB\x90\x19`\x01a\x87\xD4V[a?\x08\x82\x82`@Q`$\x01aPW\x92\x91\x90a\x8C%V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FK\\Bw\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Rac\xCCV[_\x80`\x06` R\x83_R`\x04`@_ \x01` R\x82_R`@_ ` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x88Z\xFAaP\xF4WcS\\\xF9K_R`\x04`\x1C\xFD[PP_Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x94`\x80\x91\x90\x91\x1D\x93P\x91PPV[_\x81\x83\x11aQ'W\x81a\x05\xA0V[P\x90\x91\x90PV[_\x80\x84`\x02\x0B\x87`\x02\x0B\x12\x15aQbWaQ[aQJ\x86ac\xF7V[aQS\x86ac\xF7V[\x85`\x01af\xD5V[\x91PaQ\xBBV[\x83`\x02\x0B\x87`\x02\x0B\x12\x15aQ\x9BWaQ}\x86aQS\x86ac\xF7V[\x91PaQ\x94aQ\x8B\x86ac\xF7V[\x87\x85`\x01ag\x9FV[\x90PaQ\xBBV[aQ\xB8aQ\xA7\x86ac\xF7V[aQ\xB0\x86ac\xF7V[\x85`\x01ag\x9FV[\x90P[\x95P\x95\x93PPPPV[_\x81\x83\x10aQ'W\x81a\x05\xA0V[_\x82\x81R`\x06` \x90\x81R`@\x80\x83 \x84\x84R`\x05\x01\x90\x91R\x81 aH\x10`\x01`\x01`\xA0\x1B\x03\x86\x16\x82a_;V[_a\x05\xA0\x83b\xFF\xFF\xFF\x16_\x84b\xFF\xFF\xFF\x16a5\xEEV[`@\x80Q\x80\x82\x01\x90\x91R_\x81R``` \x82\x01R\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aRDWaRDa\x86\x98V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15aRmW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P` \x82\x01R\x91\x90PV[\x81Q_\x90aR\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7FNothing to use\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[_\x80aR\xD4\x85\x85ah?V[\x87Q\x90\x95P\x91\x93P\x91P_\x90aR\xF7\x90aR\xF0\x90`\x01\x90a\x895V[\x87\x90ah?V[\x92PPP`\x01\x86_\x01\x81\x81QaS\r\x91\x90a\x895V[\x90RP\x82\x15aS@W` \x86\x01Qah\xF6\x90\x81\x90aS9\x90aS/\x90\x86ah\xFEV[\x84\x83c\xFF\xFF\xFF\xFF\x16V[PPa8\x1BV[`@\x80Q\x80\x82\x01\x90\x91R\x85\x81R` \x80\x82\x01\x83\x90R\x87\x01Q\x81\x90aSd\x90\x82ai`V[PPPPP\x92\x91PPV[_x\x12r]\xD1\xD2C\xAB\xA0\xE7_\xE6E\xCCHs\xF9\xE6Z\xFEh\x8C\x92\x8E\x1F\"\x83\x10\x82\x02aS\x9FWc|_H}_R`\x04`\x1C\xFD[Pg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90\x91\x02\x04\x90V[_\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aGZW`@Q\x7F5'\x8D\x12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17_\x82\x13aTiWc\x16\x15\xE68_R`\x04`\x1C\xFD[\x7F\xF8\xF9\xF9\xFA\xF9\xFD\xFA\xFB\xF9\xFD\xFC\xFD\xFA\xFB\xFC\xFE\xF9\xFA\xFD\xFA\xFC\xFC\xFB\xFE\xFA\xFA\xFC\xFB\xFF\xFF\xFF\xFFo\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x83\x83\x1C\x1C`\x1F\x16\x1A\x18\x90\x81\x1B`\x9F\x90\x81\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFs\xC0\xC7\x16\xA5\x94\xE0\rT\xE3\xC4\xCB\xC9\x01\x83\x02\x82\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xC7\xB8\x8CB\x0ES\xA9\x89\x053\x12\x9Fo\x01\x83\x02\x90\x91\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFF_\xDA'\xEBMc\xDE\xD4t\xE5\xF82\x01\x90\x91\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF5\xF6\xAF\x8F{3\x96dO\x18\xE1W\x96\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02\x91\x90\x03}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x90V[_\x81\x83\x12aV\x9FW`@Q\x7F\xA8\x83CW\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x82\x03\x83aV\xAD\x86\x83aE\xA5V[\x01\x95\x94PPPPPV[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\xC0\xD0W\t%\xA4b\xD7\x82\x13aV\xE4W\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12aW\x01Wc\xA3{\xFE\xC9_R`\x04`\x1C\xFD[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P_``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05k\x80\0\0\0\0\0\0\0\0\0\0\0\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xDB\xF3\xCC\xF1`M&4P\xF0*U\x04\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE5\xAD\xED\xAA\x1C\xB0\x95\xAF\x9EM\xA1\x0E6<\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD8\xDCw&\x08\xB0\xAEV\xCC\xE0\x12\x96\xC0\xEB\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE,i\x81,\xF0;\x07c\xFDEJ\x8F~\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02y\xD85\xEB\xBA\x82L\x98\xFB1\xB8;,\xA4\\\0\0\0\0\0\0\0\0\0\0\0\0\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[_\x80\x82\x12\x15aGZW`@Q\x7F\xCA\xCC\xB6\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81_\x19\x04\x83\x11\x15aX\xF6W\x81\x15aX\xF6Wc\xBA\xC6^[_R`\x04`\x1C\xFD[Pg\r\xE0\xB6\xB3\xA7d\0\0\x91\x02\x04\x90V[`@Q\x7F\xD1}K\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xD1}K\r\x90aYZ\x90\x86\x90\x86\x90\x86\x90`\x04\x01a\x8E\xBEV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15aYpW_\x80\xFD[PZ\xFA\x15\x80\x15aY\x82W=_\x80>=_\xFD[PPPPPPPV[```\x01\x83`\x02\x0B\x12\x15aY\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FInvalid TICK_SPACING\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[\x85Q\x15aH\x10WaY\xF4\x85\x85\x88\x86ajaV[aY\xFD\x86ak\"V[\x81`\x02\x0B\x86`\x01\x88QaZ\x10\x91\x90a\x895V[\x81Q\x81\x10aZ WaZ a\x87zV[` \x02` \x01\x01Q_\x01Q`\x02\x0B\x13aZ\xB8W`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x90\x81` \x01[`@\x80Q`\xC0\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x80\x83\x01\x82\x90R`\x80\x83\x01R`\xA0\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81aZHW\x90PP\x90PaZ\x96\x85\x85\x88\x85\x87ak\xDBV[\x81_\x81Q\x81\x10aZ\xA8WaZ\xA8a\x87zV[` \x02` \x01\x01\x81\x90RPaH\x10V[\x85_\x81Q\x81\x10aZ\xCAWaZ\xCAa\x87zV[` \x02` \x01\x01Q_\x01Q`\x02\x0B\x82`\x02\x0B\x13a[DW`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x90\x81` \x01[`@\x80Q`\xC0\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x80\x83\x01\x82\x90R`\x80\x83\x01R`\xA0\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81aZ\xF6W\x90PP\x90PaZ\x96\x85\x85\x88\x85\x87ap\xE4V[\x81_[a[\\`\x01`\x01`\xA0\x1B\x03\x88\x16\x87\x84\x88au\xF9V[\x92P\x90P\x80a[wW\x81a[o\x81a\x8E\xDCV[\x92PPa[GV[\x87_\x81Q\x81\x10a[\x89Wa[\x89a\x87zV[` \x02` \x01\x01Q_\x01Q`\x02\x0B\x82`\x02\x0B\x13a\\'W`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x90\x81` \x01[`@\x80Q`\xC0\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x80\x83\x01\x82\x90R`\x80\x83\x01R`\xA0\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81a[\xB5W\x90PP\x92Pa\\\x03\x87\x87\x8A\x87\x89ap\xE4V[\x83_\x81Q\x81\x10a\\\x15Wa\\\x15a\x87zV[` \x02` \x01\x01\x81\x90RPPPaH\x10V[PP_a\\4\x87\x84avNV[\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\\PWa\\Pa\x86\x98V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\\\x94W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\\nW\x90P[P\x90P_\x82\x89Qa\\\xA5\x91\x90a\x895V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\\\xBDWa\\\xBDa\x86\x98V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a]\x01W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\\\xDBW\x90P[P\x90P_[\x89Q\x81\x10\x15a]\x9EW\x83\x81\x10\x15a]SW\x89\x81\x81Q\x81\x10a])Wa])a\x87zV[` \x02` \x01\x01Q\x83\x82\x81Q\x81\x10a]CWa]Ca\x87zV[` \x02` \x01\x01\x81\x90RPa]\x96V[\x89\x81\x81Q\x81\x10a]eWa]ea\x87zV[` \x02` \x01\x01Q\x82\x85\x83a]z\x91\x90a\x895V[\x81Q\x81\x10a]\x8AWa]\x8Aa\x87zV[` \x02` \x01\x01\x81\x90RP[`\x01\x01a]\x06V[P\x81Q_\x03a^/W`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x90\x81` \x01[`@\x80Q`\xC0\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x80\x83\x01\x82\x90R`\x80\x83\x01R`\xA0\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81a]\xBCW\x90PP\x93Pa^\n\x88\x88\x83\x88\x8Aap\xE4V[\x84_\x81Q\x81\x10a^\x1CWa^\x1Ca\x87zV[` \x02` \x01\x01\x81\x90RPPPPaH\x10V[\x80Q_\x03a^\x9AW`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x90\x81` \x01[`@\x80Q`\xC0\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x80\x83\x01\x82\x90R`\x80\x83\x01R`\xA0\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81a^LW\x90PP\x93Pa^\n\x88\x88\x84\x88\x8Aak\xDBV[`@\x80Q`\x02\x80\x82R``\x82\x01\x90\x92R\x90\x81` \x01[`@\x80Q`\xC0\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x80\x83\x01\x82\x90R`\x80\x83\x01R`\xA0\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81a^\xB0W\x90PP\x93Pa^\xFE\x88\x88\x83\x88\x8Aap\xE4V[\x84_\x81Q\x81\x10a_\x10Wa_\x10a\x87zV[` \x02` \x01\x01\x81\x90RPa_(\x88\x88\x84\x88\x8Aak\xDBV[\x84`\x01\x81Q\x81\x10a^\x1CWa^\x1Ca\x87zV[_\x81` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x86Z\xFAa_bWcS\\\xF9K_R`\x04`\x1C\xFD[PP_Q\x91\x90PV[``a\x05\xA3\x82ag\xFDV[\x80Q` \x80\x83\x01Q`@\x80\x85\x01Q``\x80\x87\x01Q\x92Q\x95\x81\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x16\x94\x86\x01\x94\x90\x94R`\x80\x92\x83\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16`4\x87\x01R\x90\x83\x1B\x81\x16`D\x86\x01R\x91\x1B\x16`T\x83\x01R\x90`d\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`D\x81Q\x14a`^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FAssets unexpected length\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[\x91\x90PV[_c\x01\0\0\0\x82\x10aGZWaGZav\xECV[``a`\x82\x84av\xF9V[\x83Q` \x85\x01Q_\x91\x82\x91a`\x98\x91\x87\x91awnV[\x91P\x91P_a`\xAF\x85\x88_\x01Q\x89` \x01Qaw\x9BV[\x90P\x82`\xF0\x1B\x82`\xF0\x1B\x82`\xF0\x1Ba`\xC8\x8A`@\x01Q\x90V[`@Q\x7F\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x94\x85\x16` \x82\x01R\x92\x84\x16`\"\x84\x01R\x92\x16`$\x82\x01R`&\x81\x01\x91\x90\x91R`F\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x93P`&\x84Q\x14aa,W_\x80\xFD[PPP\x93\x92PPPV[_\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x03aa\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FassetIn == assetOut\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x10\x90P_\x80\x82aa\xBEW\x84\x86aa\xC1V[\x85\x85[\x91P\x91P_\x93P[\x86Q\x84a\xFF\xFF\x16\x10\x15abYW_\x87\x85a\xFF\xFF\x16\x81Q\x81\x10aa\xEDWaa\xEDa\x87zV[` \x02` \x01\x01Q\x90Pab\0\x81av\xF9V[\x82`\x01`\x01`\xA0\x1B\x03\x16\x81_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14\x80\x15ab9WP\x81`\x01`\x01`\xA0\x1B\x03\x16\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14[\x15abFWPPPab\xB0V[P\x83abQ\x81a\x8F\x19V[\x94PPaa\xC9V[\x86Q\x84a\xFF\xFF\x16\x10ab\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7FPair not found\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[PP[\x93P\x93\x91PPV[\x80Q``\x90\x15ac\x11W\x81` \x01Q`\x80\x1B\x82``\x01Q`\x80\x1B`@Q` \x01aI|\x92\x91\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x83\x16\x81R\x91\x16`\x10\x82\x01R` \x01\x90V[``_[\x83`\x80\x01QQ\x81\x10\x15acoW\x81\x84`\x80\x01Q\x82\x81Q\x81\x10ac9Wac9a\x87zV[` \x02` \x01\x01Q`\x80\x1B`@Q` \x01acU\x92\x91\x90a\x8F0V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x91P`\x01\x01ac\x15V[Pacz\x81Qa`cV[`\xE8\x1B\x81`@Q` \x01ac\x8F\x92\x91\x90a\x8D\xD6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x82`@\x01Q`\xE8\x1B\x83``\x01Q`\x80\x1B\x82\x85`\xA0\x01Q``\x1B`@Q` \x01aH\xEA\x94\x93\x92\x91\x90a\x8FlV[a8\x95\x81ax\xD6aM\xD8V[_jconsole.log\x90P_\x80\x83Q` \x85\x01\x84Z\xFAPPPV[`\x02\x0B_`\xFF\x82\x90\x1D\x80\x83\x01\x18b\r\x89\xE8\x81\x11\x15ad9Wad9\x7F\x8B\x862z\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84ax\xF6V[p\x01\xFF\xFC\xB93\xBDo\xAD7\xAA-\x16-\x1AY@\x01`\x01\x82\x16\x02p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x18`\x02\x82\x16\x15ad\x82Wo\xFF\xF9rr7=A2Y\xA4i\x90X\x0E!:\x02`\x80\x1C[`\x04\x82\x16\x15ad\xA1Wo\xFF\xF2\xE5\x0F_ei2\xEF\x125|\xF3\xC7\xFD\xCC\x02`\x80\x1C[`\x08\x82\x16\x15ad\xC0Wo\xFF\xE5\xCA\xCA~\x10\xE4\xE6\x1C6$\xEA\xA0\x94\x1C\xD0\x02`\x80\x1C[`\x10\x82\x16\x15ad\xDFWo\xFF\xCB\x98C\xD6\x0FaY\xC9\xDBX\x83\\\x92fD\x02`\x80\x1C[` \x82\x16\x15ad\xFEWo\xFF\x97;A\xFA\x98\xC0\x81G.h\x96\xDF\xB2T\xC0\x02`\x80\x1C[`@\x82\x16\x15ae\x1DWo\xFF.\xA1df\xC9j8C\xECx\xB3&\xB5(a\x02`\x80\x1C[`\x80\x82\x16\x15ae<Wo\xFE]\xEE\x04j\x99\xA2\xA8\x11\xC4a\xF1\x96\x9C0S\x02`\x80\x1C[a\x01\0\x82\x16\x15ae\\Wo\xFC\xBE\x86\xC7\x90\n\x88\xAE\xDC\xFF\xC8;G\x9A\xA3\xA4\x02`\x80\x1C[a\x02\0\x82\x16\x15ae|Wo\xF9\x87\xA7%:\xC4\x13\x17o+\x07L\xF7\x81^T\x02`\x80\x1C[a\x04\0\x82\x16\x15ae\x9CWo\xF39+\x08\"\xB7\0\x05\x94\x0Cz9\x8EKp\xF3\x02`\x80\x1C[a\x08\0\x82\x16\x15ae\xBCWo\xE7\x15\x94u\xA2\xC2\x9BtC\xB2\x9C\x7F\xA6\xE8\x89\xD9\x02`\x80\x1C[a\x10\0\x82\x16\x15ae\xDCWo\xD0\x97\xF3\xBD\xFD \"\xB8\x84Z\xD8\xF7\x92\xAAX%\x02`\x80\x1C[a \0\x82\x16\x15ae\xFCWo\xA9\xF7FF-\x87\x0F\xDF\x8Ae\xDC\x1F\x90\xE0a\xE5\x02`\x80\x1C[a@\0\x82\x16\x15af\x1CWop\xD8i\xA1V\xD2\xA1\xB8\x90\xBB=\xF6+\xAF2\xF7\x02`\x80\x1C[a\x80\0\x82\x16\x15af<Wo1\xBE\x13_\x97\xD0\x8F\xD9\x81#\x15\x05T/\xCF\xA6\x02`\x80\x1C[b\x01\0\0\x82\x16\x15af]Wo\t\xAAP\x8B[z\x84\xE1\xC6w\xDET\xF3\xE9\x9B\xC9\x02`\x80\x1C[b\x02\0\0\x82\x16\x15af}Wn]j\xF8\xDE\xDB\x81\x19f\x99\xC3)\"^\xE6\x04\x02`\x80\x1C[b\x04\0\0\x82\x16\x15af\x9CWm\"\x16\xE5\x84\xF5\xFA\x1E\xA9&\x04\x1B\xED\xFE\x98\x02`\x80\x1C[b\x08\0\0\x82\x16\x15af\xB9Wk\x04\x8A\x17\x03\x91\xF7\xDCBDN\x8F\xA2\x02`\x80\x1C[_\x84\x13\x15af\xC5W_\x19\x04[c\xFF\xFF\xFF\xFF\x01` \x1C\x93\x92PPPV[_\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x11\x15af\xF4W\x92\x93\x92[`\x01`\x01`\xA0\x1B\x03\x85\x16ag\x0EWb\xBF\xC9!_R`\x04`\x1C\xFD[{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0``\x84\x90\x1B\x16`\x01`\x01`\xA0\x1B\x03\x86\x86\x03\x16\x83agsW\x86`\x01`\x01`\xA0\x1B\x03\x16ag`\x83\x83\x89`\x01`\x01`\xA0\x1B\x03\x16ay\x05V[\x81agmWagma\x8AsV[\x04a=\x95V[a=\x95ag\x8A\x83\x83\x89`\x01`\x01`\xA0\x1B\x03\x16ay\xA1V[\x88`\x01`\x01`\xA0\x1B\x03\x16\x80\x82\x04\x91\x06\x15\x15\x01\x90V[_`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x90\x86\x16\x03`\xFF\x81\x90\x1D\x90\x81\x01\x18l\x01\0\0\0\0\0\0\0\0\0\0\0\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16ag\xE4\x81\x84\x84ay\x05V[\x93P\x84_\x83\x85\x84\t\x11\x16\x84\x01\x93PPPP\x94\x93PPPPV[```\x80`@Q\x01\x90P` \x81\x01`@R_\x81R\x80_\x19\x83[\x92\x81\x01\x92`0`\n\x82\x06\x01\x84S`\n\x90\x04\x80ah\x16WPP\x81\x90\x03`\x1F\x19\x90\x91\x01\x90\x81R\x91\x90PV[_\x80_\x84_\x01Q\x84\x10ah\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FIndex out-of-bounds\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[_\x91Pah\xFE\x80[` \x87\x01QQ\x84\x10\x15ah\xE6W_ah\xBC\x88` \x01Q\x86\x84c\xFF\xFF\xFF\xFF\x16V[\x90P\x86\x81_\x01Q\x03ah\xDAW` \x01Q`\x01\x95P\x92Pah\xEF\x91PPV[P`\x01\x90\x93\x01\x92ah\x9CV[P_\x93P\x84\x91PP[\x92P\x92P\x92V[` \x90\x91\x01RV[\x81Q_\x90\x82\x10ai:W`@Q\x7FD\xDD6\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82` \x01Q\x82\x81Q\x81\x10aiPWaiPa\x87zV[` \x02` \x01\x01Q\x90P\x92\x91PPV[\x81Q\x80\x83aim\x82a\x8F\xFAV[\x90RP` \x83\x01QQ\x83Q\x81\x10\x15aj8W_ai\x8B\x82`\x02a\x8A\x0FV[\x90P_`\x01\x82\x10ai\x9CW\x81ai\x9FV[`\x01[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ai\xB7Wai\xB7a\x86\x98V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15ai\xE0W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83\x81\x10\x15aj0W\x86` \x01Q\x81\x81Q\x81\x10aj\x03Waj\x03a\x87zV[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10aj\x1DWaj\x1Da\x87zV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01ai\xE5V[P` \x86\x01RP[\x82\x84` \x01Q\x83\x81Q\x81\x10ajOWajOa\x87zV[` \x02` \x01\x01\x81\x81RPPPPPPV[_[\x82Q\x81\x10\x15aC\x8DW_\x83\x82\x81Q\x81\x10aj\x7FWaj\x7Fa\x87zV[` \x02` \x01\x01Q_\x01Q\x90P_\x80aj\xA1a=Z\x84\x87_\x81\x83\x07\x12\x91\x05\x03\x90V[\x90\x92P\x90P_aj\xBB`\x01`\x01`\xA0\x1B\x03\x8A\x16\x89\x85aQ\xD3V[\x90P`\x01`\xFF\x83\x16\x1B\x81\x16ak\x12W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FTick not initialized\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[PP`\x01\x90\x92\x01\x91Pajc\x90PV[ak+\x81ay\xD1V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0_[\x82Q\x81\x10\x15ak\xD6W_\x83\x82\x81Q\x81\x10akjWakja\x87zV[` \x02` \x01\x01Q_\x01Q\x90P\x82`\x02\x0B\x81`\x02\x0B\x13ak\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7FDuplicate tick\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[\x91P`\x01\x01akNV[PPPV[`@\x80Q`\xC0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x80\x82\x01\x83\x90R`\x80\x82\x01R`\xA0\x81\x01\x91\x90\x91R_\x84Q\x11al[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01R\x7FNo rewards\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[_al~`\x02\x86Q`\x03alo\x91\x90a\x8A\x0FV[aly\x91\x90a\x8A\xA0V[aR\x17V[\x90P_\x85_\x81Q\x81\x10al\x93Wal\x93a\x87zV[` \x90\x81\x02\x91\x90\x91\x01\x01QQ\x90P`\x01_[al\xBA`\x01`\x01`\xA0\x1B\x03\x8B\x16\x8A\x85\x89az\xBEV[\x93P\x91P`\x02\x83\x81\x0B\x90\x88\x90\x0B\x12ampW\x81\x15al\xE7Wal\xE0\x84`\x02\x85\x90\x0Bai`V[P_al\xA5V[\x80al\xF1\x81a\x8F\xFAV[\x91PP`x\x81\x11\x15amkW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FMAX_LOOP exceeded in _createRewa`D\x82\x01R\x7FrdUpdateBelow\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01aFVV[al\xA5V[PP\x81Q_\x03\x90Pam\xFDW\x84Q`\x01\x14am\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7Fexpected rewards length 1\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[am\xF5\x87\x87\x87_\x81Q\x81\x10am\xE4Wam\xE4a\x87zV[` \x02` \x01\x01Q` \x01Qaz\xFBV[\x91PPaH\x10V[\x80Qan\n\x90`\x01a\x87\xD4V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15an\"Wan\"a\x86\x98V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15anKW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P`\x80\x83\x01R_\x80\x80[\x83Q\x81\x10\x15ao\x1DW_ani\x85\x83ah\xFEV[\x90P_an\x80`\x01`\x01`\xA0\x1B\x03\x8D\x16\x8C\x84aP\xB4V[\x91Pan\x8E\x90P\x81\x85a\x90\x12V[\x93P\x89Q\x85\x10\x15ao\x13W_\x8A\x86\x81Q\x81\x10an\xACWan\xACa\x87zV[` \x02` \x01\x01Q\x90P\x82`\x02\x0B\x81_\x01Q`\x02\x0B\x12\x15ao\x11W\x80` \x01Q\x88`\x80\x01Q\x85\x81Q\x81\x10an\xE2Wan\xE2a\x87zV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x85ao\r\x81a\x8F\xFAV[\x96PP[P[PP`\x01\x01anUV[P\x86Q\x82\x10\x15ao\x8DW\x86\x82\x81Q\x81\x10ao9Wao9a\x87zV[` \x02` \x01\x01Q` \x01Q\x84`\x80\x01Q\x84_\x01Q\x81Q\x81\x10ao^Wao^a\x87zV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x81ao\x89\x81a\x8F\xFAV[\x92PP[\x86Q\x82\x14ao\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01R\x7FNot all rewards used?\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[ao\xE7\x83_ah\xFEV[`\x02\x0B`@\x85\x01R_ao\xFC\x8A\x8A\x89\x89a{nV[\x90Pap\x08\x81\x83a}\xADV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x86\x01\x81\x90R\x89\x90\x8B\x90_\x90\x81[\x88Q\x81\x10\x15ap\xCBW_ap=\x8A\x83ah\xFEV[\x90P_apT`\x01`\x01`\xA0\x1B\x03\x87\x16\x88\x84aP\xB4V[\x91PPapa\x84\x82a}\xC7V[`@\x80Q` \x80\x82\x01\x98\x90\x98R`\x80\x83\x90\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81\x83\x01R`\xE8\x94\x90\x94\x1B`P\x85\x01R\x80Q`3\x81\x86\x03\x01\x81R`S\x90\x94\x01\x90R\x82Q\x92\x90\x95\x01\x91\x90\x91 \x93\x92PP`\x01\x01ap)V[PP``\x1C`\xA0\x88\x01RPPPPPP\x95\x94PPPPPV[`@\x80Q`\xC0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x80\x82\x01\x83\x90R`\x80\x82\x01R`\xA0\x81\x01\x91\x90\x91R_\x84Q\x11aqdW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01R\x7FNo rewards\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[_aqx`\x02\x86Q`\x03alo\x91\x90a\x8A\x0FV[\x90P_\x85`\x01\x87Qaq\x8A\x91\x90a\x895V[\x81Q\x81\x10aq\x9AWaq\x9Aa\x87zV[` \x90\x81\x02\x91\x90\x91\x01\x01QQ\x90P`\x01\x81_[\x81`\x02\x0B\x88`\x02\x0B\x12\x15aryW\x82\x15aq\xD6Waq\xCF\x85`\x02\x84\x90\x0Bai`V[P_arZV[\x80aq\xE0\x81a\x8F\xFAV[\x91PP`x\x81\x11\x15arZW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FMAX_LOOP exceeded in _createRewa`D\x82\x01R\x7FrdUpdateAbove\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01aFVV[aro`\x01`\x01`\xA0\x1B\x03\x8C\x16\x8B\x84\x8Aa}\xE1V[\x90\x93P\x91Paq\xADV[PP\x82Q_\x03\x90Pas\x17War\xA6`@Q\x80`\x80\x01`@R\x80`D\x81R` \x01a\xA92`D\x919a8$V[\x85Q`\x01\x14ar\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FExpected exact one reward\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[as\x0E\x88\x88\x88_\x81Q\x81\x10am\xE4Wam\xE4a\x87zV[\x92PPPaH\x10V[`\x02\x81\x90\x0B`@\x84\x01R\x81Qas.\x90`\x01a\x87\xD4V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15asFWasFa\x86\x98V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15asoW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P`\x80\x84\x01R\x85Q_\x80[\x84Q\x81\x10\x15atHW_as\x8E\x86\x83ah\xFEV[\x90P_as\xA5`\x01`\x01`\xA0\x1B\x03\x8E\x16\x8D\x84aP\xB4V[\x91Pas\xB3\x90P\x81\x85a\x90\x12V[\x93P\x84\x15at>W_\x8Bas\xC8`\x01\x88a\x895V[\x81Q\x81\x10as\xD8Was\xD8a\x87zV[` \x02` \x01\x01Q\x90P\x82`\x02\x0B\x81_\x01Q`\x02\x0B\x12at<W\x80` \x01Q\x89`\x80\x01Q\x85\x81Q\x81\x10at\rWat\ra\x87zV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x85at8\x81a\x90`V[\x96PP[P[PP`\x01\x01aszV[P\x81\x15at\xB5W\x87_\x81Q\x81\x10ataWataa\x87zV[` \x02` \x01\x01Q` \x01Q\x85`\x80\x01Q\x85_\x01Q\x81Q\x81\x10at\x86Wat\x86a\x87zV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x81at\xB1\x81a\x90`V[\x92PP[\x81\x15au\x03W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01R\x7FNot all rewards used?\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[_au\x10\x8B\x8B\x8A\x8Aa{nV[\x90Pau\x1C\x81\x83a}\xC7V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x87\x01\x81\x90R\x8A\x90\x8C\x90_\x90\x81[\x89Q\x81\x10\x15au\xDFW_auQ\x8B\x83ah\xFEV[\x90P_auh`\x01`\x01`\xA0\x1B\x03\x87\x16\x88\x84aP\xB4V[\x91PPauu\x84\x82a}\xADV[`@\x80Q` \x80\x82\x01\x98\x90\x98R`\x80\x83\x90\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81\x83\x01R`\xE8\x94\x90\x94\x1B`P\x85\x01R\x80Q`3\x81\x86\x03\x01\x81R`S\x90\x94\x01\x90R\x82Q\x92\x90\x95\x01\x91\x90\x91 \x93\x92PP`\x01\x01au=V[PP``\x1C`\xA0\x89\x01RPPPPPPP\x95\x94PPPPPV[_\x80\x80\x80av\x0E\x85\x87\x07\x82\x13\x86\x88\x05\x03a=ZV[\x90\x92P\x90Pav1\x81av+`\x01`\x01`\xA0\x1B\x03\x8B\x16\x8A\x86aQ\xD3V[\x90a}\xFBV[\x90\x94P\x90PavA\x82\x82\x87a~\xC3V[\x92PPP\x94P\x94\x92PPPV[_\x80\x83Q\x11av\x9FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01R\x7FNo rewards\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[_[\x83Q\x81\x10\x15av\xE3W\x82`\x02\x0B\x84\x82\x81Q\x81\x10av\xC0Wav\xC0a\x87zV[` \x02` \x01\x01Q_\x01Q`\x02\x0B\x13\x15av\xDBW\x90Pa\x05\xA3V[`\x01\x01av\xA1V[PP\x90Q\x91\x90PV[c5'\x8D\x12_R`\x04`\x1C\xFD[\x80_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x11a8\x95W`@\x80Q\x7FQ\x904C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x04\x83\x01R` \x84\x01Q\x16`$\x82\x01R\x90\x82\x01Q`D\x82\x01R`d\x01aFVV[_\x80aw\x82aw}\x86\x86a~\xEDV[a\x7F\x86V[\x91Paw\x91aw}\x86\x85a~\xEDV[\x90P\x93P\x93\x91PPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10aw\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FgetStoreIndex:assets unsorted\0\0\0`D\x82\x01R`d\x01aFVV[_\x83\x81R` \x83\x81R`@\x82 `(\x1B\x91\x90ax#\x90`\x01`\x01`\xA0\x1B\x03\x88\x16;a\x8A\xA0V[_\x93P\x90P\x85[\x81\x84a\xFF\xFF\x16\x10\x15ax\x8EW_` \x85` \x02`\x01\x01_\x84<P_Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x80\x85\x16\x90\x82\x16\x03ax{WPPPPa\x06#V[P\x83ax\x86\x81a\x8F\x19V[\x94PPax*V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FPool not enabled\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[\x80Qjconsole.log` \x83\x01_\x80\x84\x83\x85Z\xFAPPPPPV[\x81_R\x80`\x02\x0B`\x04R`$_\xFD[_\x83\x83\x02\x81_\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80\x84\x11ay$W_\x80\xFD[\x80_\x03ay6WP\x82\x90\x04\x90Pa\x06#V[_\x84\x86\x88\t_\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP\x93\x92PPPV[_ay\xAD\x84\x84\x84ay\x05V[\x90P\x81\x80ay\xBDWay\xBDa\x8AsV[\x83\x85\t\x15a\x06#W`\x01\x01\x80a\x06#W_\x80\xFD[_[\x81Q\x81\x10\x15a?\x08W_ay\xE8\x82`\x01a\x87\xD4V[\x90P[\x82Q\x81\x10\x15az\xB5Waz9\x83\x82\x81Q\x81\x10az\tWaz\ta\x87zV[` \x02` \x01\x01Q\x84\x84\x81Q\x81\x10az#Waz#a\x87zV[` \x02` \x01\x01Qa\x7F\x99\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x15az\xADW\x82\x81\x81Q\x81\x10azPWazPa\x87zV[` \x02` \x01\x01Q\x83\x83\x81Q\x81\x10azjWazja\x87zV[` \x02` \x01\x01Q\x84\x84\x81Q\x81\x10az\x84Waz\x84a\x87zV[` \x02` \x01\x01\x85\x84\x81Q\x81\x10az\x9DWaz\x9Da\x87zV[` \x02` \x01\x01\x82\x90R\x82\x90RPP[`\x01\x01ay\xEBV[P`\x01\x01ay\xD3V[_\x80\x80\x80az\xD8a=Z\x86\x88\x07\x83\x13\x87\x89\x05\x03`\x01a\x8C\xD8V[\x90\x92P\x90Pav1\x81az\xF5`\x01`\x01`\xA0\x1B\x03\x8B\x16\x8A\x86aQ\xD3V[\x90a\x7F\xA7V[`@\x80Q`\xC0\x81\x01\x82R_\x91\x81\x01\x82\x90R``\x80\x82\x01\x83\x90R`\x80\x82\x01R`\xA0\x81\x01\x91\x90\x91R`\x01\x81Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16` \x82\x01Ra{P`\x01`\x01`\xA0\x1B\x03\x85\x16\x84a\x80iV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x82\x01R\x93\x92PPPV[_\x80a{\x86a2^`\x01`\x01`\xA0\x1B\x03\x88\x16\x87aH\x19V[\x90P_a{\x9C`\x01`\x01`\xA0\x1B\x03\x88\x16\x87a\x80iV[\x90P\x84`\x02\x0B\x82`\x02\x0B\x12\x15a|\x9BW_\x82\x81[a{\xC5`\x01`\x01`\xA0\x1B\x03\x8B\x16\x8A\x84\x8Aaz\xBEV[\x90\x93P\x91P`\x02\x82\x81\x0B\x90\x89\x90\x0B\x12a|\x93W\x82\x15a|\nWP_\x80a{\xF5`\x01`\x01`\xA0\x1B\x03\x8C\x16\x8B\x85aP\xB4V[\x91PPa|\x02\x85\x82a}\xC7V[\x94PPa{\xB0V[\x80a|\x14\x81a\x8F\xFAV[\x91PP`x\x81\x11\x15a|\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FMAX_LOOP exceeded in getLiquidit`D\x82\x01R\x7FyAtTick [present < future]\0\0\0\0\0\0`d\x82\x01R`\x84\x01aFVV[a{\xB0V[PPPa}\xA3V[\x81`\x02\x0B\x85`\x02\x0B\x12\x15a}\xA3W`\x01\x82_[a|\xC3`\x01`\x01`\xA0\x1B\x03\x8B\x16\x8A\x84\x8Aau\xF9V[\x90\x93P\x91P`\x02\x88\x81\x0B\x90\x83\x90\x0B\x13\x15a}\x9FW\x82\x15a}\tWP_\x80a|\xF4`\x01`\x01`\xA0\x1B\x03\x8C\x16\x8B\x85aP\xB4V[\x91PPa}\x01\x85\x82a}\xADV[\x94PPa}\x8DV[\x80a}\x13\x81a\x8F\xFAV[\x91PP`x\x81\x11\x15a}\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FMAX_LOOP exceeded in getLiquidit`D\x82\x01R\x7FyAtTick [future < present]\0\0\0\0\0\0`d\x82\x01R`\x84\x01aFVV[\x81a}\x97\x81a\x8E\xDCV[\x92PPa|\xAEV[PPP[\x96\x95PPPPPPV[\x80\x82\x03`\x80\x81\x90\x1C\x15a\x05\xA3Wc\xC9eN\xD4_R`\x04`\x1C\xFD[\x81\x81\x01`\x80\x81\x90\x1C\x15a\x05\xA3Wc\xC9eN\xD4_R`\x04`\x1C\xFD[_\x80\x80\x80av\x0Ea=Z`\x01\x87\x89\x07\x84\x13\x88\x8A\x05\x03a\x8C\x97V[_\x80_\x83`\xFF\x03\x90P_a~\x9C\x82`\xFF\x16\x87\x90\x1B\x7F\x07\x06\x06\x05\x06\x02\x05\x04\x06\x02\x03\x02\x05\x04\x03\x01\x06\x05\x02\x05\x03\x03\x04\x01\x05\x05\x03\x04\0\0\0\0`\x1Fo\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x83\x15`\x08\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11`\x07\x1B\x17\x84\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x84\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x84\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x84\x81\x1C`\xFF\x10`\x03\x1B\x17\x93\x84\x1C\x1C\x16\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x93P\x83a~\xB1W_a~\xB8V[\x81`\xFF\x16\x81\x03[\x92PPP\x92P\x92\x90PV[_\x81`\xFF\x84\x16a~\xD9`\x01\x87\x90\x0Ba\x01\0a\x8B\xDEV[a~\xE3\x91\x90a\x8C\xD8V[aH9\x91\x90a\x8B\xDEV[_\x80[\x83Q\x81\x10\x15a\x7F=W\x83\x81\x81Q\x81\x10a\x7F\x0BWa\x7F\x0Ba\x87zV[` \x02` \x01\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x03a\x7F5W\x90Pa\x05\xA3V[`\x01\x01a~\xF0V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01R\x7FAsset not found\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01aFVV[_b\x01\0\0\x82\x10aGZWaGZav\xECV[Q\x90Q`\x02\x91\x82\x0B\x91\x0B\x13\x90V[_\x80_a\x80B\x84`\xFF\x16\x86\x90\x1C~\x1F\r\x1E\x10\x0C\x1D\x07\x0F\t\x0B\x19\x13\x1C\x17\x06\x01\x0E\x11\x08\n\x1A\x14\x18\x02\x12\x1B\x15\x03\x16\x04\x05\x81\x19`\x01\x01\x90\x91\x16a\x01\xE0\x7F\x80@@UC\0RfD2\0\0P a\x06t\x050&\x02\0\0\x10u\x06 \x01v\x11pw`\xFC\x7F\xB6\xDBm\xB6\xDD\xDD\xDD\xDD\xD3M4\xD3I$\x92I!\x08B\x10\x8Cc\x18\xC69\xCEs\x9C\xFF\xFF\xFF\xFF\x84\x02`\xF8\x1C\x16\x1B`\xF7\x1C\x16\x90\x81\x1Cc\xD7dS\xE0\x04`\x1F\x16\x91\x90\x91\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x92P\x82a\x80XW`\xFFa\x80_V[\x83`\xFF\x16\x81\x01[\x91PP\x92P\x92\x90PV[_\x81\x81R`\x06` R`@\x81 _aH\x10`\x01`\x01`\xA0\x1B\x03\x86\x16`\x03\x84\x01a_;V[a\x18\xBC\x80a\x90v\x839\x01\x90V[`@Q\x80`@\x01`@R\x80_\x81R` \x01a\x80\xC7`@Q\x80`@\x01`@R\x80_\x81R` \x01``\x81RP\x90V[\x90R\x90V[_` \x82\x84\x03\x12\x15a\x80\xDCW_\x80\xFD[P5\x91\x90PV[_\x80`@\x83\x85\x03\x12\x15a\x80\xF4W_\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x81CW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x81\x1CV[P\x90\x95\x94PPPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x81CW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x81gV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01_[\x83\x81\x10\x15a\x82\x01W`\x1F\x19\x85\x84\x03\x01\x88Ra\x81\xEB\x83\x83Qa\x81\x85V[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01a\x81\xCFV[P\x90\x96\x95PPPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\x82\xA2W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x87\x86\x03\x01\x84R\x81Q`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x86R` \x81\x01Q\x90P`@` \x87\x01Ra\x82\x8C`@\x87\x01\x82a\x81\xB3V[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x823V[P\x92\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a8\x95W_\x80\xFD[_` \x82\x84\x03\x12\x15a\x82\xD2W_\x80\xFD[\x815a\x06#\x81a\x82\xAEV[\x80`\x02\x0B\x81\x14a8\x95W_\x80\xFD[_\x80_\x80_`\xA0\x86\x88\x03\x12\x15a\x82\xFFW_\x80\xFD[\x855\x94P` \x86\x015\x93P`@\x86\x015a\x83\x18\x81a\x82\xDDV[\x92P``\x86\x015a\x83(\x81a\x82\xDDV[\x94\x97\x93\x96P\x91\x94`\x80\x015\x92\x91PPV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a\x83\x8BW\x81Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a\x83KV[P\x93\x94\x93PPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\x82\xA2W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x87\x86\x03\x01\x84R\x81Q\x80Q`@\x87Ra\x83\xFF`@\x88\x01\x82a\x81\x85V[\x90P` \x82\x01Q\x91P\x86\x81\x03` \x88\x01Ra\x84\x1A\x81\x83a\x839V[\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x83\xBBV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a\x83\x8BW\x81Q\x80Q\x87R` \x81\x01Q` \x88\x01R`@\x81\x01Q`@\x88\x01RP``\x86\x01\x95P` \x82\x01\x91P`\x01\x81\x01\x90Pa\x84CV[` \x81R\x81Q`\x02\x0B` \x82\x01R` \x82\x01Q`\x02\x0B`@\x82\x01R_`@\x83\x01Qa\x84\xB3``\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x83\x01Q`\x80\x83\x01R`\x80\x83\x01Q`\xA0\x83\x01R`\xA0\x83\x01Q`\xC0\x83\x01R`\xC0\x83\x01Q`\xE0\x83\x01R`\xE0\x83\x01Qa\x01\0\x80\x84\x01RaH9a\x01 \x84\x01\x82a\x841V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x81CW\x83Q\x80Q`\x02\x0B\x84R` \x90\x81\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x85\x01R\x90\x93\x01\x92`@\x90\x92\x01\x91`\x01\x01a\x85\x0FV[` \x81R_a\x05\xA0` \x83\x01\x84a\x81\xB3V[_\x80_``\x84\x86\x03\x12\x15a\x85rW_\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\x82\xA2W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x87\x86\x03\x01\x84R\x81Q`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x86R` \x81\x01Q\x90P`@` \x87\x01Ra\x86\x08`@\x87\x01\x82a\x839V[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x85\xAFV[\x805b\xFF\xFF\xFF\x81\x16\x81\x14a`^W_\x80\xFD[_\x80_\x80_\x80`\xC0\x87\x89\x03\x12\x15a\x86EW_\x80\xFD[\x865\x95P` \x87\x015\x94P`@\x87\x015a\x86^\x81a\x82\xDDV[\x93Pa\x86l``\x88\x01a\x86\x1EV[\x92Pa\x86z`\x80\x88\x01a\x86\x1EV[\x91P`\xA0\x87\x015a\x86\x8A\x81a\x82\xAEV[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x86\xEEWa\x86\xEEa\x86\x98V[`@R\x91\x90PV[_\x80_\x83\x85\x03``\x81\x12\x15a\x87\tW_\x80\xFD[\x845\x93P` \x80\x86\x015\x93P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x82\x01\x12\x15a\x87BW_\x80\xFD[P`@Q` \x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x87fWa\x87fa\x86\x98V[`@\x90\x81R\x94\x90\x94\x015\x84RP\x90\x93\x90\x92PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x05\xA3Wa\x05\xA3a\x87\xA7V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x87\xFBW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x882W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[P\x91\x90PV[_` \x82\x84\x03\x12\x15a\x88HW_\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x88^W_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x88nW_\x80\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x88\x88Wa\x88\x88a\x86\x98V[a\x88\x9B` `\x1F\x19`\x1F\x84\x01\x16\x01a\x86\xC5V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\x88\xAFW_\x80\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[\x7Factor_\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_a\x05\xA0`\x06\x83\x01\x84a\x88\xCCV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R_aH9`@\x83\x01\x84a\x81\x85V[\x81\x81\x03\x81\x81\x11\x15a\x05\xA3Wa\x05\xA3a\x87\xA7V[_` \x82\x84\x03\x12\x15a\x89XW_\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x06#W_\x80\xFD[a\x01 \x81\x01a\x89\xC4\x82\x88`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x82R`\x01`\x01`\xA0\x1B\x03` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01R`\x01`\x01`\xA0\x1B\x03`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[\x85`\x02\x0B`\xA0\x83\x01R\x84`\x02\x0B`\xC0\x83\x01R\x83`\xE0\x83\x01R\x82a\x01\0\x83\x01R\x96\x95PPPPPPV[_\x80`@\x83\x85\x03\x12\x15a\x89\xFEW_\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x05\xA3Wa\x05\xA3a\x87\xA7V[_` \x82\x84\x03\x12\x15a\x8A6W_\x80\xFD[PQ\x91\x90PV[_\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x03a\x8AmWa\x8Ama\x87\xA7V[P_\x03\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x82a\x8A\xAEWa\x8A\xAEa\x8AsV[P\x04\x90V[_` \x82\x84\x03\x12\x15a\x8A\xC3W_\x80\xFD[\x81Qa\x06#\x81a\x82\xAEV[`\xC0\x81\x01a\x8B*\x82\x85`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x82R`\x01`\x01`\xA0\x1B\x03` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01R`\x01`\x01`\xA0\x1B\x03`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\xA0\x83\x01R\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x8BPW_\x80\xFD[\x81Qa\x06#\x81a\x82\xDDV[_aH\x10a\x8Bra\x8Bl\x84\x88a\x88\xCCV[\x86a\x88\xCCV[\x84a\x88\xCCV[` \x81R_a\x05\xA0` \x83\x01\x84a\x81\x85V[_\x81`\x02\x0B\x83`\x02\x0B\x80a\x8B\xA0Wa\x8B\xA0a\x8AsV[_\x19\x81\x14\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x83\x14\x16\x15a\x8B\xD5Wa\x8B\xD5a\x87\xA7V[\x90\x05\x93\x92PPPV[_\x82`\x02\x0B\x82`\x02\x0B\x02\x80`\x02\x0B\x91P\x80\x82\x14a6\xA6Wa6\xA6a\x87\xA7V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x05\xA3Wa\x05\xA3a\x87\xA7V[`@\x81R_a\x8C7`@\x83\x01\x85a\x81\x85V[\x82\x81\x03` \x84\x01RaH\x10\x81\x85a\x81\x85V[`@\x81R_a\x8C[`@\x83\x01\x85a\x81\x85V[\x90P\x82` \x83\x01R\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`1`\x04R`$_\xFD[`\x02\x82\x81\x0B\x90\x82\x90\x0B\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x81\x12b\x7F\xFF\xFF\x82\x13\x17\x15a\x05\xA3Wa\x05\xA3a\x87\xA7V[`\x02\x81\x81\x0B\x90\x83\x90\x0B\x01b\x7F\xFF\xFF\x81\x13\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x82\x12\x17\x15a\x05\xA3Wa\x05\xA3a\x87\xA7V[_a\x8D$\x82\x85a\x88\xCCV[\x7F, \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RaH\x10`\x02\x82\x01\x85a\x88\xCCV[_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x03a\x8D\x87Wa\x8D\x87a\x87\xA7V[`\x01\x01\x92\x91PPV[_a\x8D\x9B\x82\x84a\x88\xCCV[\x7F]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01\x01\x93\x92PPPV[_aH9a\x8Br\x83\x86a\x88\xCCV[\x7F\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81R_aH9`\x03\x83\x01\x84a\x88\xCCV[\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x16\x81R\x7F\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x16`\x01\x82\x01R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16`\x03\x82\x01R_a}\xA3`\x13\x83\x01\x84a\x88\xCCV[\x80\x82\x01\x82\x81\x12_\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a@\x1AWa@\x1Aa\x87\xA7V[_\x82a\x8E\xB9Wa\x8E\xB9a\x8AsV[P\x06\x90V[\x83\x81R\x82` \x82\x01R```@\x82\x01R_aH\x10``\x83\x01\x84a\x81\x85V[_\x81`\x02\x0B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x81\x03a\x8F\x10Wa\x8F\x10a\x87\xA7V[_\x19\x01\x92\x91PPV[_a\xFF\xFF\x82\x16a\xFF\xFF\x81\x03a\x8D\x87Wa\x8D\x87a\x87\xA7V[_a\x8F;\x82\x85a\x88\xCCV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x93\x90\x93\x16\x83RPP`\x10\x01\x91\x90PV[\x7F\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x16\x81R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x16`\x03\x82\x01R_a\x8F\xC7`\x13\x83\x01\x85a\x88\xCCV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x93\x90\x93\x16\x83RPP`\x14\x01\x93\x92PPPV[__\x19\x82\x03a\x90\x0BWa\x90\x0Ba\x87\xA7V[P`\x01\x01\x90V[`\x0F\x81\x81\x0B\x90\x83\x90\x0B\x01o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x13\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x12\x17\x15a\x05\xA3Wa\x05\xA3a\x87\xA7V[_\x81a\x90nWa\x90na\x87\xA7V[P_\x19\x01\x90V\xFE`\x80`@R4\x80\x15`\x0EW_\x80\xFD[P`@Qa\x18\xBC8\x03\x80a\x18\xBC\x839\x81\x01`@\x81\x90R`+\x91`NV[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`yV[_` \x82\x84\x03\x12\x15`]W_\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14`rW_\x80\xFD[\x93\x92PPPV[a\x186\x80a\0\x86_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0oW_5`\xE0\x1C\x80c\x91\xDDsF\x11a\0MW\x80c\x91\xDDsF\x14a\0\xD4W\x80c\xBA\xCA\0\x04\x14a\0\xF4W\x80c\xBE\xAB\xAC\xC8\x14a\x01\tW_\x80\xFD[\x80c\x04\x95\xA4\xA2\x14a\0sW\x80c\x0C\x86Xy\x14a\0\x99W\x80c@\xE2\xA8\x12\x14a\0\xC1W[_\x80\xFD[a\0\x86a\0\x816`\x04a\rnV[a\x01\x1CV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xACa\0\xA76`\x04a\r\xCDV[a\x02lV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\0\x90V[a\0\x86a\0\xCF6`\x04a\x0EfV[a\x03\xB8V[a\0\xE7a\0\xE26`\x04a\x0E\xE5V[a\x05\x0FV[`@Qa\0\x90\x91\x90a\x0FpV[a\x01\x07a\x01\x026`\x04a\x0F\x82V[a\x06\xBFV[\0[a\x01\x07a\x01\x176`\x04a\x0F\x9DV[a\x06\xE3V[_\x80T\x81\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cH\xC8\x94\x91\x82`\xF8\x1B\x88`@Q\x80``\x01`@R\x80\x8A\x15\x15\x81R` \x01\x89\x81R` \x01\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP`@Q` \x01a\x01\x85\x92\x91\x90a\x10\xBFV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x01\xC1\x92\x91` \x01a\x11\x06V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\xEC\x91\x90a\x0FpV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x02\x07W=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x02L\x91\x90\x81\x01\x90a\x12\tV[\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x02b\x91\x90a\x12\x83V[\x96\x95PPPPPPV[_\x80T\x81\x90\x81\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cH\xC8\x94\x91`\x02`\xF8\x1B\x8A`@Q\x80`\x80\x01`@R\x80\x8C`\x02\x0B\x81R` \x01\x8B`\x02\x0B\x81R` \x01\x8A\x81R` \x01\x89\x81RP`@Q` \x01a\x02\xCC\x92\x91\x90a\x12\x9AV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x03\x08\x92\x91` \x01a\x11\x06V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x033\x91\x90a\x0FpV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x03NW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x03\x93\x91\x90\x81\x01\x90a\x12\tV[\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x03\xA9\x91\x90a\x12\xDBV[\x92P\x92PP\x95P\x95\x93PPPPV[_\x80T\x81\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cH\xC8\x94\x91`\x01`\xF8\x1B\x8A`@Q\x80``\x01`@R\x80\x8C\x15\x15\x81R` \x01\x8B\x81R` \x01\x8As\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x88\x88`@Q` \x01a\x04&\x94\x93\x92\x91\x90a\x12\xFDV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x04b\x92\x91` \x01a\x11\x06V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\x8D\x91\x90a\x0FpV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x04\xA8W=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x04\xED\x91\x90\x81\x01\x90a\x12\tV[\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x05\x03\x91\x90a\x12\x83V[\x98\x97PPPPPPPPV[_T``\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x054W_\x80\xFD[_a\x05B`\x01\x82\x85\x87a\x13\x90V[a\x05K\x91a\x13\xB7V[`\xF8\x1C`\x02\x81\x11\x15a\x05_Wa\x05_a\x0F\xDBV[\x90P_\x81`\x02\x81\x11\x15a\x05tWa\x05ta\x0F\xDBV[\x03a\x05\xBCW_\x80a\x05\x88\x85`\x01\x81\x89a\x13\x90V[\x81\x01\x90a\x05\x95\x91\x90a\x15\x1BV[\x91P\x91Pa\x05\xB2\x82\x82`@Q\x80` \x01`@R\x80_\x81RPa\x07\tV[\x93PPPPa\x06\xB9V[`\x01\x81`\x02\x81\x11\x15a\x05\xD0Wa\x05\xD0a\x0F\xDBV[\x03a\x06\x0EW_\x80\x80a\x05\xE5\x86`\x01\x81\x8Aa\x13\x90V[\x81\x01\x90a\x05\xF2\x91\x90a\x15OV[\x92P\x92P\x92Pa\x06\x03\x83\x83\x83a\x07\tV[\x94PPPPPa\x06\xB9V[`\x02\x81`\x02\x81\x11\x15a\x06\"Wa\x06\"a\x0F\xDBV[\x03a\x06QW_\x80a\x066\x85`\x01\x81\x89a\x13\x90V[\x81\x01\x90a\x06C\x91\x90a\x15\xE8V[\x91P\x91Pa\x05\xB2\x82\x82a\x07\xD7V[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FUnrecognized action\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[P[\x92\x91PPV[a\x06\xDFs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x163a\x08\xFDV[PPV[a\x07\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x83\x83a\t\x7FV[PPPV[_\x80T`@Q\x7F\xF3\xCD\x91L\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R``\x92\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\xF3\xCD\x91L\x90a\x07e\x90\x88\x90\x88\x90\x88\x90`\x04\x01a\x16\x99V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x07\x81W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xA5\x91\x90a\x12\x83V[\x90Pa\x07\xB1\x85\x82a\t\xD2V[`@\x80Q` \x81\x01\x83\x90R\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP[\x93\x92PPPV[_\x80T`@Q\x7FZk\xCF\xDA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R``\x92\x91\x82\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90cZk\xCF\xDA\x90a\x085\x90\x88\x90\x88\x90`\x04\x01a\x17cV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x08PW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08t\x91\x90a\x12\xDBV[\x91P\x91Pa\x08\x8B\x85a\x08\x86\x84\x84a\t\xF9V[a\t\xD2V[_\x84`@\x01Q\x13a\x08\xD0W\x84Q_\x80T\x90\x91a\x08\xBF\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x900\x90a\nHV[\x90Pa\x08\xCE\x86_\x01Q\x82a\n\xE4V[P[`@\x80Q` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92PPP\x92\x91PPV[_cp\xA0\x821_R0` R` `4`$`\x1C\x86Z\xFA`\x1F=\x11\x16a\t*Wc\x90\xB8\xEC\x18_R`\x04`\x1C\xFD[\x81`\x14R`4Q\x90Po\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0_R` _`D`\x10_\x87Z\xF1\x80`\x01_Q\x14\x16a\ttW\x80=\x85;\x15\x17\x10a\ttWc\x90\xB8\xEC\x18_R`\x04`\x1C\xFD[P_`4R\x92\x91PPV[\x81`\x14R\x80`4Ro\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0_R` _`D`\x10_\x87Z\xF1\x80`\x01_Q\x14\x16a\t\xC8W\x80=\x85;\x15\x17\x10a\t\xC8Wc\x90\xB8\xEC\x18_R`\x04`\x1C\xFD[P_`4RPPPV[\x81Qa\t\xE7\x90a\t\xE2\x83`\x80\x1D\x90V[a\n\xE4V[a\x06\xDF\x82` \x01Qa\t\xE2\x83`\x0F\x0B\x90V[_`\x80\x82\x81\x1D\x90\x84\x90\x1D\x01`\x0F\x83\x81\x0B\x90\x85\x90\x0B\x01a\n?a\n\x1A\x83a\x0C\xE3V[a\n#\x83a\x0C\xE3V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x91\x90\x91\x1B\x17\x90V[\x95\x94PPPPPV[_\x82\x81R` \x82\x90R`@\x80\x82 \x90Q\x7F\xF15\xBA\xAA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R\x82\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x90c\xF15\xBA\xAA\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xC0W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02b\x91\x90a\x12\x83V[_\x81`\x0F\x0B\x12\x15a\x0C7W_T`@Q\x7F\xA5\x84\x11\x94\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\xA5\x84\x11\x94\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0BXW_\x80\xFD[PZ\xF1\x15\x80\x15a\x0BjW=_\x80>=_\xFD[PP_\x80Ta\x0B\xAA\x93Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x81\x16\x93P\x16\x90\x84\x90\x03o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\t\x7FV[_\x80T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x11\xDA`\xB4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0C\x13W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x04\x91\x90a\x12\x83V[\x80`\x0F\x0B_\x12\x15a\x06\xDFW_T`@Q\x7F\x0B\r\x9C\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16`\x04\x83\x01R0`$\x83\x01Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`D\x83\x01R\x90\x91\x16\x90c\x0B\r\x9C\t\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0C\xC9W_\x80\xFD[PZ\xF1\x15\x80\x15a\x0C\xDBW=_\x80>=_\xFD[PPPPPPV[\x80`\x0F\x81\x90\x0B\x81\x14a\r\x18Wa\r\x18\x7F\x93\xDA\xFD\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\r\x1DV[\x91\x90PV[\x80_R`\x04_\xFD[_`\xA0\x82\x84\x03\x12\x15a\r5W_\x80\xFD[P\x91\x90PV[\x805\x80\x15\x15\x81\x14a\r\x18W_\x80\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\rkW_\x80\xFD[PV[_\x80_\x80a\x01\0\x85\x87\x03\x12\x15a\r\x82W_\x80\xFD[a\r\x8C\x86\x86a\r%V[\x93Pa\r\x9A`\xA0\x86\x01a\r;V[\x92P`\xC0\x85\x015\x91P`\xE0\x85\x015a\r\xB1\x81a\rJV[\x93\x96\x92\x95P\x90\x93PPV[\x805`\x02\x81\x90\x0B\x81\x14a\r\x18W_\x80\xFD[_\x80_\x80_a\x01 \x86\x88\x03\x12\x15a\r\xE2W_\x80\xFD[a\r\xEC\x87\x87a\r%V[\x94Pa\r\xFA`\xA0\x87\x01a\r\xBCV[\x93Pa\x0E\x08`\xC0\x87\x01a\r\xBCV[\x94\x97\x93\x96P\x93\x94`\xE0\x81\x015\x94Pa\x01\0\x015\x92\x91PPV[_\x80\x83`\x1F\x84\x01\x12a\x0E1W_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0EHW_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x0E_W_\x80\xFD[\x92P\x92\x90PV[_\x80_\x80_\x80a\x01 \x87\x89\x03\x12\x15a\x0E|W_\x80\xFD[a\x0E\x86\x88\x88a\r%V[\x95Pa\x0E\x94`\xA0\x88\x01a\r;V[\x94P`\xC0\x87\x015\x93P`\xE0\x87\x015a\x0E\xAB\x81a\rJV[\x92Pa\x01\0\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\xC7W_\x80\xFD[a\x0E\xD3\x89\x82\x8A\x01a\x0E!V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[_\x80` \x83\x85\x03\x12\x15a\x0E\xF6W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\x0CW_\x80\xFD[a\x0F\x18\x85\x82\x86\x01a\x0E!V[\x90\x96\x90\x95P\x93PPPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x07\xD0` \x83\x01\x84a\x0F$V[_` \x82\x84\x03\x12\x15a\x0F\x92W_\x80\xFD[\x815a\x07\xD0\x81a\rJV[_\x80_``\x84\x86\x03\x12\x15a\x0F\xAFW_\x80\xFD[\x835a\x0F\xBA\x81a\rJV[\x92P` \x84\x015a\x0F\xCA\x81a\rJV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[\x805b\xFF\xFF\xFF\x81\x16\x81\x14a\r\x18W_\x80\xFD[\x805a\x10%\x81a\rJV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x81\x015a\x10K\x81a\rJV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16` \x83\x01Rb\xFF\xFF\xFFa\x10v`@\x83\x01a\x10\x08V[\x16`@\x83\x01Ra\x10\x88``\x82\x01a\r\xBCV[`\x02\x0B``\x83\x01R`\x80\x81\x015a\x10\x9E\x81a\rJV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\x80\x84\x01RPPPV[a\x01\0\x81\x01a\x10\xCE\x82\x85a\x10\x1AV[\x82Q\x15\x15`\xA0\x83\x01R` \x83\x01Q`\xC0\x83\x01R`@\x83\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x83\x01Ra\x07\xD0V[\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81R_\x82Q\x80` \x85\x01`\x01\x85\x01^_\x92\x01`\x01\x01\x91\x82RP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x11\xBCWa\x11\xBCa\x11HV[`@R\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x11\xDDWa\x11\xDDa\x11HV[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01\x90V[_` \x82\x84\x03\x12\x15a\x12\x19W_\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12/W_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x12?W_\x80\xFD[\x80Qa\x12Ra\x12M\x82a\x11\xC4V[a\x11uV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\x12fW_\x80\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a\x12\x93W_\x80\xFD[PQ\x91\x90PV[a\x01 \x81\x01a\x12\xA9\x82\x85a\x10\x1AV[\x82Q`\x02\x90\x81\x0B`\xA0\x84\x01R` \x84\x01Q\x90\x0B`\xC0\x83\x01R`@\x83\x01Q`\xE0\x83\x01R``\x83\x01Qa\x01\0\x83\x01Ra\x07\xD0V[_\x80`@\x83\x85\x03\x12\x15a\x12\xECW_\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[a\x13\x07\x81\x86a\x10\x1AV[\x83Q\x15\x15`\xA0\x82\x01R` \x84\x01Q`\xC0\x82\x01R`@\x84\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x82\x01Ra\x01 a\x01\0\x82\x01R\x81a\x01 \x82\x01R\x81\x83a\x01@\x83\x017_\x81\x83\x01a\x01@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x01\x01\x93\x92PPPV[_\x80\x85\x85\x11\x15a\x13\x9EW_\x80\xFD[\x83\x86\x11\x15a\x13\xAAW_\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x805\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90`\x01\x84\x10\x15a\x06\xB7W\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x85`\x01\x03`\x03\x1B\x1B\x82\x16\x16\x91PP\x92\x91PPV[_`\xA0\x82\x84\x03\x12\x15a\x14,W_\x80\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x14OWa\x14Oa\x11HV[`@R\x90P\x80\x825a\x14`\x81a\rJV[\x81R` \x83\x015a\x14p\x81a\rJV[` \x82\x01Ra\x14\x81`@\x84\x01a\x10\x08V[`@\x82\x01Ra\x14\x92``\x84\x01a\r\xBCV[``\x82\x01R`\x80\x83\x015a\x14\xA5\x81a\rJV[`\x80\x91\x90\x91\x01R\x92\x91PPV[_``\x82\x84\x03\x12\x15a\x14\xC2W_\x80\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x14\xE5Wa\x14\xE5a\x11HV[`@R\x90P\x80a\x14\xF4\x83a\r;V[\x81R` \x83\x81\x015\x90\x82\x01R`@\x83\x015a\x15\x0E\x81a\rJV[`@\x91\x90\x91\x01R\x92\x91PPV[_\x80a\x01\0\x83\x85\x03\x12\x15a\x15-W_\x80\xFD[a\x157\x84\x84a\x14\x1CV[\x91Pa\x15F\x84`\xA0\x85\x01a\x14\xB2V[\x90P\x92P\x92\x90PV[_\x80_a\x01 \x84\x86\x03\x12\x15a\x15bW_\x80\xFD[a\x15l\x85\x85a\x14\x1CV[\x92Pa\x15{\x85`\xA0\x86\x01a\x14\xB2V[\x91Pa\x01\0\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\x97W_\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x15\xA7W_\x80\xFD[\x805a\x15\xB5a\x12M\x82a\x11\xC4V[\x81\x81R\x87` \x83\x85\x01\x01\x11\x15a\x15\xC9W_\x80\xFD[\x81` \x84\x01` \x83\x017_` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92P\x92V[_\x80\x82\x84\x03a\x01 \x81\x12\x15a\x15\xFBW_\x80\xFD[a\x16\x05\x85\x85a\x14\x1CV[\x92P`\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x82\x01\x12\x15a\x166W_\x80\xFD[P`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x16ZWa\x16Za\x11HV[`@Ra\x16i`\xA0\x85\x01a\r\xBCV[\x81Ra\x16w`\xC0\x85\x01a\r\xBCV[` \x82\x01R`\xE0\x84\x015`@\x82\x01Ra\x01\0\x90\x93\x015``\x84\x01RP\x92\x90\x91PV[a\x17\x18\x81\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[\x82Q\x15\x15`\xA0\x82\x01R` \x83\x01Q`\xC0\x82\x01R`@\x83\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x82\x01Ra\x01 a\x01\0\x82\x01R_a\n?a\x01 \x83\x01\x84a\x0F$V[a\x17\xE2\x81\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[\x81Q`\x02\x90\x81\x0B`\xA0\x83\x01R` \x83\x01Q\x90\x0B`\xC0\x82\x01R`@\x82\x01Q`\xE0\x82\x01R``\x82\x01Qa\x01\0\x82\x01Ra\x01@a\x01 \x82\x01\x81\x90R_\x90\x82\x01Ra\x01`\x01\x92\x91PPV\xFE\xA1dsolcC\0\x08\x1A\0\nWARNING\nWARNING: Above somehow called with donate to current only???\xA1dsolcC\0\x08\x1A\0\n",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PoolId(alloy::sol_types::private::FixedBytes<32>);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<PoolId>
        for alloy::sol_types::private::FixedBytes<32> {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::FixedBytes<
                32,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        #[automatically_derived]
        impl PoolId {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(value: alloy::sol_types::private::FixedBytes<32>) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(self) -> alloy::sol_types::private::FixedBytes<32> {
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
        impl alloy_sol_types::SolType for PoolId {
            type RustType = alloy::sol_types::private::FixedBytes<32>;
            type Token<'a> = <alloy::sol_types::sol_data::FixedBytes<
                32,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::FixedBytes<
                32,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::FixedBytes<
                32,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for PoolId {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PositionKey(alloy::sol_types::private::FixedBytes<32>);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<PositionKey>
        for alloy::sol_types::private::FixedBytes<32> {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::FixedBytes<
                32,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        #[automatically_derived]
        impl PositionKey {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(value: alloy::sol_types::private::FixedBytes<32>) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(self) -> alloy::sol_types::private::FixedBytes<32> {
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
        impl alloy_sol_types::SolType for PositionKey {
            type RustType = alloy::sol_types::private::FixedBytes<32>;
            type Token<'a> = <alloy::sol_types::sol_data::FixedBytes<
                32,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::FixedBytes<
                32,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::FixedBytes<
                32,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for PositionKey {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PriceAB(alloy::sol_types::private::primitives::aliases::U256);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<PriceAB>
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
        #[automatically_derived]
        impl PriceAB {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(
                value: alloy::sol_types::private::primitives::aliases::U256,
            ) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(
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
        impl alloy_sol_types::SolType for PriceAB {
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
        impl alloy_sol_types::EventTopic for PriceAB {
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
    /**```solidity
struct Env { address owner; address controller; address feeMaster; address uniV4; address angstrom; address[] assets; address[] mirrors; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Env {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub controller: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub feeMaster: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub uniV4: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub angstrom: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub assets: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        #[allow(missing_docs)]
        pub mirrors: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
        impl ::core::convert::From<Env> for UnderlyingRustTuple<'_> {
            fn from(value: Env) -> Self {
                (
                    value.owner,
                    value.controller,
                    value.feeMaster,
                    value.uniV4,
                    value.angstrom,
                    value.assets,
                    value.mirrors,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Env {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    owner: tuple.0,
                    controller: tuple.1,
                    feeMaster: tuple.2,
                    uniV4: tuple.3,
                    angstrom: tuple.4,
                    assets: tuple.5,
                    mirrors: tuple.6,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Env {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Env {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.controller,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.feeMaster,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.uniV4,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.angstrom,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.assets),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.mirrors),
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
        impl alloy_sol_types::SolType for Env {
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
        impl alloy_sol_types::SolStruct for Env {
            const NAME: &'static str = "Env";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Env(address owner,address controller,address feeMaster,address uniV4,address angstrom,address[] assets,address[] mirrors)",
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.owner,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.controller,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.feeMaster,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.uniV4,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.angstrom,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.assets)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.mirrors)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Env {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.owner,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.controller,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.feeMaster,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.uniV4,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.angstrom,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.assets,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.mirrors,
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
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.owner,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.controller,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.feeMaster,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.uniV4,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.angstrom,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Address,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.assets,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Address,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.mirrors,
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
struct LiquidityAdd { uint256 liquidity; uint256 rewardStartIndex; uint256 rewardEndIndex; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LiquidityAdd {
        #[allow(missing_docs)]
        pub liquidity: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub rewardStartIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub rewardEndIndex: alloy::sol_types::private::primitives::aliases::U256,
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
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
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
        impl ::core::convert::From<LiquidityAdd> for UnderlyingRustTuple<'_> {
            fn from(value: LiquidityAdd) -> Self {
                (value.liquidity, value.rewardStartIndex, value.rewardEndIndex)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for LiquidityAdd {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    liquidity: tuple.0,
                    rewardStartIndex: tuple.1,
                    rewardEndIndex: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for LiquidityAdd {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for LiquidityAdd {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidity),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.rewardStartIndex),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.rewardEndIndex),
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
        impl alloy_sol_types::SolType for LiquidityAdd {
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
        impl alloy_sol_types::SolStruct for LiquidityAdd {
            const NAME: &'static str = "LiquidityAdd";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "LiquidityAdd(uint256 liquidity,uint256 rewardStartIndex,uint256 rewardEndIndex)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.liquidity)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.rewardStartIndex,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.rewardEndIndex,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for LiquidityAdd {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.liquidity,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.rewardStartIndex,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.rewardEndIndex,
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
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.liquidity,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.rewardStartIndex,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.rewardEndIndex,
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
struct LiquidityPosition { int24 lowerTick; int24 upperTick; address owner; uint256 totalRewardsX128; uint256 claimedRewards; uint256 totalLiquidity; uint256 activeAddsOffset; LiquidityAdd[] adds; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LiquidityPosition {
        #[allow(missing_docs)]
        pub lowerTick: alloy::sol_types::private::primitives::aliases::I24,
        #[allow(missing_docs)]
        pub upperTick: alloy::sol_types::private::primitives::aliases::I24,
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub totalRewardsX128: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub claimedRewards: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub totalLiquidity: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub activeAddsOffset: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub adds: alloy::sol_types::private::Vec<
            <LiquidityAdd as alloy::sol_types::SolType>::RustType,
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
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Int<24>,
            alloy::sol_types::sol_data::Int<24>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Array<LiquidityAdd>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::I24,
            alloy::sol_types::private::primitives::aliases::I24,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Vec<
                <LiquidityAdd as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<LiquidityPosition> for UnderlyingRustTuple<'_> {
            fn from(value: LiquidityPosition) -> Self {
                (
                    value.lowerTick,
                    value.upperTick,
                    value.owner,
                    value.totalRewardsX128,
                    value.claimedRewards,
                    value.totalLiquidity,
                    value.activeAddsOffset,
                    value.adds,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for LiquidityPosition {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    lowerTick: tuple.0,
                    upperTick: tuple.1,
                    owner: tuple.2,
                    totalRewardsX128: tuple.3,
                    claimedRewards: tuple.4,
                    totalLiquidity: tuple.5,
                    activeAddsOffset: tuple.6,
                    adds: tuple.7,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for LiquidityPosition {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for LiquidityPosition {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.lowerTick),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.upperTick),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalRewardsX128),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.claimedRewards),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalLiquidity),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.activeAddsOffset),
                    <alloy::sol_types::sol_data::Array<
                        LiquidityAdd,
                    > as alloy_sol_types::SolType>::tokenize(&self.adds),
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
        impl alloy_sol_types::SolType for LiquidityPosition {
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
        impl alloy_sol_types::SolStruct for LiquidityPosition {
            const NAME: &'static str = "LiquidityPosition";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "LiquidityPosition(int24 lowerTick,int24 upperTick,address owner,uint256 totalRewardsX128,uint256 claimedRewards,uint256 totalLiquidity,uint256 activeAddsOffset,LiquidityAdd[] adds)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <LiquidityAdd as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <LiquidityAdd as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.lowerTick)
                        .0,
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.upperTick)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.owner,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.totalRewardsX128,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.claimedRewards,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.totalLiquidity,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.activeAddsOffset,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        LiquidityAdd,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.adds)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for LiquidityPosition {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.lowerTick,
                    )
                    + <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.upperTick,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.owner,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.totalRewardsX128,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.claimedRewards,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.totalLiquidity,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.activeAddsOffset,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        LiquidityAdd,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.adds)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Int<
                    24,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.lowerTick,
                    out,
                );
                <alloy::sol_types::sol_data::Int<
                    24,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.upperTick,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.owner,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.totalRewardsX128,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.claimedRewards,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.totalLiquidity,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.activeAddsOffset,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    LiquidityAdd,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.adds,
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
struct PRNG { uint256 __state; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PRNG {
        #[allow(missing_docs)]
        pub __state: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<PRNG> for UnderlyingRustTuple<'_> {
            fn from(value: PRNG) -> Self {
                (value.__state,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PRNG {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { __state: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for PRNG {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for PRNG {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.__state),
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
        impl alloy_sol_types::SolType for PRNG {
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
        impl alloy_sol_types::SolStruct for PRNG {
            const NAME: &'static str = "PRNG";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed("PRNG(uint256 __state)")
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
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::SolType>::eip712_data_word(&self.__state)
                    .0
                    .to_vec()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for PRNG {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.__state,
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
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.__state,
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
struct Pair { address asset0; address asset1; PriceAB price10; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Pair {
        #[allow(missing_docs)]
        pub asset0: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub asset1: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub price10: <PriceAB as alloy::sol_types::SolType>::RustType,
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
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            PriceAB,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            <PriceAB as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<Pair> for UnderlyingRustTuple<'_> {
            fn from(value: Pair) -> Self {
                (value.asset0, value.asset1, value.price10)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Pair {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    asset0: tuple.0,
                    asset1: tuple.1,
                    price10: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Pair {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Pair {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset0,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset1,
                    ),
                    <PriceAB as alloy_sol_types::SolType>::tokenize(&self.price10),
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
        impl alloy_sol_types::SolType for Pair {
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
        impl alloy_sol_types::SolStruct for Pair {
            const NAME: &'static str = "Pair";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Pair(address asset0,address asset1,uint256 price10)",
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.asset0,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.asset1,
                        )
                        .0,
                    <PriceAB as alloy_sol_types::SolType>::eip712_data_word(
                            &self.price10,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Pair {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.asset0,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.asset1,
                    )
                    + <PriceAB as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.price10,
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
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.asset0,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.asset1,
                    out,
                );
                <PriceAB as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.price10,
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
struct TickReward { int24 tick; uint128 amount; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TickReward {
        #[allow(missing_docs)]
        pub tick: alloy::sol_types::private::primitives::aliases::I24,
        #[allow(missing_docs)]
        pub amount: u128,
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
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Int<24>,
            alloy::sol_types::sol_data::Uint<128>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::I24,
            u128,
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
        impl ::core::convert::From<TickReward> for UnderlyingRustTuple<'_> {
            fn from(value: TickReward) -> Self {
                (value.tick, value.amount)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TickReward {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    tick: tuple.0,
                    amount: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for TickReward {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for TickReward {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tick),
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
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
        impl alloy_sol_types::SolType for TickReward {
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
        impl alloy_sol_types::SolStruct for TickReward {
            const NAME: &'static str = "TickReward";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "TickReward(int24 tick,uint128 amount)",
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
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.tick)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amount)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for TickReward {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.tick)
                    + <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amount,
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
                <alloy::sol_types::sol_data::Int<
                    24,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tick,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amount,
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
    /**Custom error with signature `IndexOutOfBounds()` and selector `0x4e23d035`.
```solidity
error IndexOutOfBounds();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IndexOutOfBounds {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
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
        impl ::core::convert::From<IndexOutOfBounds> for UnderlyingRustTuple<'_> {
            fn from(value: IndexOutOfBounds) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for IndexOutOfBounds {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for IndexOutOfBounds {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "IndexOutOfBounds()";
            const SELECTOR: [u8; 4] = [78u8, 35u8, 208u8, 53u8];
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
    /**Custom error with signature `InvalidBounds()` and selector `0xa8834357`.
```solidity
error InvalidBounds();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidBounds {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
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
        impl ::core::convert::From<InvalidBounds> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidBounds) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidBounds {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidBounds {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidBounds()";
            const SELECTOR: [u8; 4] = [168u8, 131u8, 67u8, 87u8];
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
    /**Custom error with signature `OutOfBoundsVecGet()` and selector `0x44dd369f`.
```solidity
error OutOfBoundsVecGet();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OutOfBoundsVecGet {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
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
        impl ::core::convert::From<OutOfBoundsVecGet> for UnderlyingRustTuple<'_> {
            fn from(value: OutOfBoundsVecGet) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OutOfBoundsVecGet {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OutOfBoundsVecGet {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OutOfBoundsVecGet()";
            const SELECTOR: [u8; 4] = [68u8, 221u8, 54u8, 159u8];
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
    /**Custom error with signature `Overflow()` and selector `0x35278d12`.
```solidity
error Overflow();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Overflow {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
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
        impl ::core::convert::From<Overflow> for UnderlyingRustTuple<'_> {
            fn from(value: Overflow) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Overflow {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for Overflow {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "Overflow()";
            const SELECTOR: [u8; 4] = [53u8, 39u8, 141u8, 18u8];
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
    /**Custom error with signature `PairAssetsWrong((address,address,uint256))` and selector `0x51903443`.
```solidity
error PairAssetsWrong(Pair);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PairAssetsWrong {
        #[allow(missing_docs)]
        pub _0: <Pair as alloy::sol_types::SolType>::RustType,
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
        type UnderlyingSolTuple<'a> = (Pair,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (<Pair as alloy::sol_types::SolType>::RustType,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<PairAssetsWrong> for UnderlyingRustTuple<'_> {
            fn from(value: PairAssetsWrong) -> Self {
                (value._0,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PairAssetsWrong {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { _0: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for PairAssetsWrong {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PairAssetsWrong((address,address,uint256))";
            const SELECTOR: [u8; 4] = [81u8, 144u8, 52u8, 67u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<Pair as alloy_sol_types::SolType>::tokenize(&self._0),)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `Underflow()` and selector `0xcaccb6d9`.
```solidity
error Underflow();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Underflow {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
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
        impl ::core::convert::From<Underflow> for UnderlyingRustTuple<'_> {
            fn from(value: Underflow) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Underflow {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for Underflow {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "Underflow()";
            const SELECTOR: [u8; 4] = [202u8, 204u8, 182u8, 217u8];
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
    /**Event with signature `log(string)` and selector `0x41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f50`.
```solidity
event log(string);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log {
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log {
            type DataTuple<'a> = (alloy::sol_types::sol_data::String,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log(string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                65u8, 48u8, 79u8, 172u8, 217u8, 50u8, 61u8, 117u8, 177u8, 27u8, 205u8,
                214u8, 9u8, 203u8, 56u8, 239u8, 255u8, 253u8, 176u8, 87u8, 16u8, 247u8,
                202u8, 240u8, 233u8, 177u8, 108u8, 109u8, 157u8, 112u8, 159u8, 80u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self._0,
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
        impl alloy_sol_types::private::IntoLogData for log {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_address(address)` and selector `0x7ae74c527414ae135fd97047b12921a5ec3911b804197855d67e25c7b75ee6f3`.
```solidity
event log_address(address);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_address {
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_address {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_address(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                122u8, 231u8, 76u8, 82u8, 116u8, 20u8, 174u8, 19u8, 95u8, 217u8, 112u8,
                71u8, 177u8, 41u8, 33u8, 165u8, 236u8, 57u8, 17u8, 184u8, 4u8, 25u8,
                120u8, 85u8, 214u8, 126u8, 37u8, 199u8, 183u8, 94u8, 230u8, 243u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                        &self._0,
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
        impl alloy_sol_types::private::IntoLogData for log_address {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_address> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_address) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_array(uint256[])` and selector `0xfb102865d50addddf69da9b5aa1bced66c80cf869a5c8d0471a467e18ce9cab1`.
```solidity
event log_array(uint256[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_array_0 {
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_array_0 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_array(uint256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                251u8, 16u8, 40u8, 101u8, 213u8, 10u8, 221u8, 221u8, 246u8, 157u8, 169u8,
                181u8, 170u8, 27u8, 206u8, 214u8, 108u8, 128u8, 207u8, 134u8, 154u8,
                92u8, 141u8, 4u8, 113u8, 164u8, 103u8, 225u8, 140u8, 233u8, 202u8, 177u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { val: data.0 }
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_array_0 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_array_0> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_array_0) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_array(int256[])` and selector `0x890a82679b470f2bd82816ed9b161f97d8b967f37fa3647c21d5bf39749e2dd5`.
```solidity
event log_array(int256[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_array_1 {
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::I256,
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_array_1 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Int<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_array(int256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                137u8, 10u8, 130u8, 103u8, 155u8, 71u8, 15u8, 43u8, 216u8, 40u8, 22u8,
                237u8, 155u8, 22u8, 31u8, 151u8, 216u8, 185u8, 103u8, 243u8, 127u8,
                163u8, 100u8, 124u8, 33u8, 213u8, 191u8, 57u8, 116u8, 158u8, 45u8, 213u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { val: data.0 }
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Int<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_array_1 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_array_1> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_array_1) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_array(address[])` and selector `0x40e1840f5769073d61bd01372d9b75baa9842d5629a0c99ff103be1178a8e9e2`.
```solidity
event log_array(address[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_array_2 {
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
        impl alloy_sol_types::SolEvent for log_array_2 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_array(address[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                64u8, 225u8, 132u8, 15u8, 87u8, 105u8, 7u8, 61u8, 97u8, 189u8, 1u8, 55u8,
                45u8, 155u8, 117u8, 186u8, 169u8, 132u8, 45u8, 86u8, 41u8, 160u8, 201u8,
                159u8, 241u8, 3u8, 190u8, 17u8, 120u8, 168u8, 233u8, 226u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { val: data.0 }
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_array_2 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_array_2> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_array_2) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_bytes(bytes)` and selector `0x23b62ad0584d24a75f0bf3560391ef5659ec6db1269c56e11aa241d637f19b20`.
```solidity
event log_bytes(bytes);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_bytes {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for log_bytes {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_bytes(bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                35u8, 182u8, 42u8, 208u8, 88u8, 77u8, 36u8, 167u8, 95u8, 11u8, 243u8,
                86u8, 3u8, 145u8, 239u8, 86u8, 89u8, 236u8, 109u8, 177u8, 38u8, 156u8,
                86u8, 225u8, 26u8, 162u8, 65u8, 214u8, 55u8, 241u8, 155u8, 32u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._0,
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
        impl alloy_sol_types::private::IntoLogData for log_bytes {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_bytes> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_bytes) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_bytes32(bytes32)` and selector `0xe81699b85113eea1c73e10588b2b035e55893369632173afd43feb192fac64e3`.
```solidity
event log_bytes32(bytes32);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_bytes32 {
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_bytes32 {
            type DataTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_bytes32(bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                232u8, 22u8, 153u8, 184u8, 81u8, 19u8, 238u8, 161u8, 199u8, 62u8, 16u8,
                88u8, 139u8, 43u8, 3u8, 94u8, 85u8, 137u8, 51u8, 105u8, 99u8, 33u8,
                115u8, 175u8, 212u8, 63u8, 235u8, 25u8, 47u8, 172u8, 100u8, 227u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
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
        impl alloy_sol_types::private::IntoLogData for log_bytes32 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_bytes32> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_bytes32) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_int(int256)` and selector `0x0eb5d52624c8d28ada9fc55a8c502ed5aa3fbe2fb6e91b71b5f376882b1d2fb8`.
```solidity
event log_int(int256);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_int {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::I256,
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
        impl alloy_sol_types::SolEvent for log_int {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Int<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_int(int256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                14u8, 181u8, 213u8, 38u8, 36u8, 200u8, 210u8, 138u8, 218u8, 159u8, 197u8,
                90u8, 140u8, 80u8, 46u8, 213u8, 170u8, 63u8, 190u8, 47u8, 182u8, 233u8,
                27u8, 113u8, 181u8, 243u8, 118u8, 136u8, 43u8, 29u8, 47u8, 184u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
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
        impl alloy_sol_types::private::IntoLogData for log_int {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_int> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_int) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_named_address(string,address)` and selector `0x9c4e8541ca8f0dc1c413f9108f66d82d3cecb1bddbce437a61caa3175c4cc96f`.
```solidity
event log_named_address(string key, address val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_address {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for log_named_address {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_address(string,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                156u8, 78u8, 133u8, 65u8, 202u8, 143u8, 13u8, 193u8, 196u8, 19u8, 249u8,
                16u8, 143u8, 102u8, 216u8, 45u8, 60u8, 236u8, 177u8, 189u8, 219u8, 206u8,
                67u8, 122u8, 97u8, 202u8, 163u8, 23u8, 92u8, 76u8, 201u8, 111u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.val,
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
        impl alloy_sol_types::private::IntoLogData for log_named_address {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_address> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_address) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_named_array(string,uint256[])` and selector `0x00aaa39c9ffb5f567a4534380c737075702e1f7f14107fc95328e3b56c0325fb`.
```solidity
event log_named_array(string key, uint256[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_array_0 {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_named_array_0 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_array(string,uint256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                0u8, 170u8, 163u8, 156u8, 159u8, 251u8, 95u8, 86u8, 122u8, 69u8, 52u8,
                56u8, 12u8, 115u8, 112u8, 117u8, 112u8, 46u8, 31u8, 127u8, 20u8, 16u8,
                127u8, 201u8, 83u8, 40u8, 227u8, 181u8, 108u8, 3u8, 37u8, 251u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_array_0 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_array_0> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_array_0) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_named_array(string,int256[])` and selector `0xa73eda09662f46dde729be4611385ff34fe6c44fbbc6f7e17b042b59a3445b57`.
```solidity
event log_named_array(string key, int256[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_array_1 {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::I256,
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_named_array_1 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Int<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_array(string,int256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                167u8, 62u8, 218u8, 9u8, 102u8, 47u8, 70u8, 221u8, 231u8, 41u8, 190u8,
                70u8, 17u8, 56u8, 95u8, 243u8, 79u8, 230u8, 196u8, 79u8, 187u8, 198u8,
                247u8, 225u8, 123u8, 4u8, 43u8, 89u8, 163u8, 68u8, 91u8, 87u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Int<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_array_1 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_array_1> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_array_1) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_named_array(string,address[])` and selector `0x3bcfb2ae2e8d132dd1fce7cf278a9a19756a9fceabe470df3bdabb4bc577d1bd`.
```solidity
event log_named_array(string key, address[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_array_2 {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
        impl alloy_sol_types::SolEvent for log_named_array_2 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_array(string,address[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                59u8, 207u8, 178u8, 174u8, 46u8, 141u8, 19u8, 45u8, 209u8, 252u8, 231u8,
                207u8, 39u8, 138u8, 154u8, 25u8, 117u8, 106u8, 159u8, 206u8, 171u8,
                228u8, 112u8, 223u8, 59u8, 218u8, 187u8, 75u8, 197u8, 119u8, 209u8, 189u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_array_2 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_array_2> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_array_2) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_named_bytes(string,bytes)` and selector `0xd26e16cad4548705e4c9e2d94f98ee91c289085ee425594fd5635fa2964ccf18`.
```solidity
event log_named_bytes(string key, bytes val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_bytes {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for log_named_bytes {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_bytes(string,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                210u8, 110u8, 22u8, 202u8, 212u8, 84u8, 135u8, 5u8, 228u8, 201u8, 226u8,
                217u8, 79u8, 152u8, 238u8, 145u8, 194u8, 137u8, 8u8, 94u8, 228u8, 37u8,
                89u8, 79u8, 213u8, 99u8, 95u8, 162u8, 150u8, 76u8, 207u8, 24u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.val,
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
        impl alloy_sol_types::private::IntoLogData for log_named_bytes {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_bytes> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_bytes) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_named_bytes32(string,bytes32)` and selector `0xafb795c9c61e4fe7468c386f925d7a5429ecad9c0495ddb8d38d690614d32f99`.
```solidity
event log_named_bytes32(string key, bytes32 val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_bytes32 {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for log_named_bytes32 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_bytes32(string,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                175u8, 183u8, 149u8, 201u8, 198u8, 30u8, 79u8, 231u8, 70u8, 140u8, 56u8,
                111u8, 146u8, 93u8, 122u8, 84u8, 41u8, 236u8, 173u8, 156u8, 4u8, 149u8,
                221u8, 184u8, 211u8, 141u8, 105u8, 6u8, 20u8, 211u8, 47u8, 153u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_bytes32 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_bytes32> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_bytes32) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_named_decimal_int(string,int256,uint256)` and selector `0x5da6ce9d51151ba10c09a559ef24d520b9dac5c5b8810ae8434e4d0d86411a95`.
```solidity
event log_named_decimal_int(string key, int256 val, uint256 decimals);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_decimal_int {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::primitives::aliases::I256,
        #[allow(missing_docs)]
        pub decimals: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for log_named_decimal_int {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Int<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_decimal_int(string,int256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                93u8, 166u8, 206u8, 157u8, 81u8, 21u8, 27u8, 161u8, 12u8, 9u8, 165u8,
                89u8, 239u8, 36u8, 213u8, 32u8, 185u8, 218u8, 197u8, 197u8, 184u8, 129u8,
                10u8, 232u8, 67u8, 78u8, 77u8, 13u8, 134u8, 65u8, 26u8, 149u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    key: data.0,
                    val: data.1,
                    decimals: data.2,
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.decimals),
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
        impl alloy_sol_types::private::IntoLogData for log_named_decimal_int {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_decimal_int> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_decimal_int) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_named_decimal_uint(string,uint256,uint256)` and selector `0xeb8ba43ced7537421946bd43e828b8b2b8428927aa8f801c13d934bf11aca57b`.
```solidity
event log_named_decimal_uint(string key, uint256 val, uint256 decimals);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_decimal_uint {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub decimals: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for log_named_decimal_uint {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_decimal_uint(string,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                235u8, 139u8, 164u8, 60u8, 237u8, 117u8, 55u8, 66u8, 25u8, 70u8, 189u8,
                67u8, 232u8, 40u8, 184u8, 178u8, 184u8, 66u8, 137u8, 39u8, 170u8, 143u8,
                128u8, 28u8, 19u8, 217u8, 52u8, 191u8, 17u8, 172u8, 165u8, 123u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    key: data.0,
                    val: data.1,
                    decimals: data.2,
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.decimals),
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
        impl alloy_sol_types::private::IntoLogData for log_named_decimal_uint {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_decimal_uint> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_decimal_uint) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_named_int(string,int256)` and selector `0x2fe632779174374378442a8e978bccfbdcc1d6b2b0d81f7e8eb776ab2286f168`.
```solidity
event log_named_int(string key, int256 val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_int {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::primitives::aliases::I256,
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
        impl alloy_sol_types::SolEvent for log_named_int {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Int<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_int(string,int256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                47u8, 230u8, 50u8, 119u8, 145u8, 116u8, 55u8, 67u8, 120u8, 68u8, 42u8,
                142u8, 151u8, 139u8, 204u8, 251u8, 220u8, 193u8, 214u8, 178u8, 176u8,
                216u8, 31u8, 126u8, 142u8, 183u8, 118u8, 171u8, 34u8, 134u8, 241u8, 104u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_int {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_int> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_int) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_named_string(string,string)` and selector `0x280f4446b28a1372417dda658d30b95b2992b12ac9c7f378535f29a97acf3583`.
```solidity
event log_named_string(string key, string val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_string {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::String,
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
        impl alloy_sol_types::SolEvent for log_named_string {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_string(string,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                40u8, 15u8, 68u8, 70u8, 178u8, 138u8, 19u8, 114u8, 65u8, 125u8, 218u8,
                101u8, 141u8, 48u8, 185u8, 91u8, 41u8, 146u8, 177u8, 42u8, 201u8, 199u8,
                243u8, 120u8, 83u8, 95u8, 41u8, 169u8, 122u8, 207u8, 53u8, 131u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.val,
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
        impl alloy_sol_types::private::IntoLogData for log_named_string {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_string> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_string) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_named_uint(string,uint256)` and selector `0xb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a8`.
```solidity
event log_named_uint(string key, uint256 val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_uint {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for log_named_uint {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_uint(string,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                178u8, 222u8, 47u8, 190u8, 128u8, 26u8, 13u8, 246u8, 192u8, 203u8, 221u8,
                253u8, 68u8, 139u8, 163u8, 196u8, 29u8, 72u8, 160u8, 64u8, 202u8, 53u8,
                197u8, 108u8, 129u8, 150u8, 239u8, 15u8, 202u8, 231u8, 33u8, 168u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_uint {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_uint> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_uint) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_string(string)` and selector `0x0b2e13ff20ac7b474198655583edf70dedd2c1dc980e329c4fbb2fc0748b796b`.
```solidity
event log_string(string);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_string {
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_string {
            type DataTuple<'a> = (alloy::sol_types::sol_data::String,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_string(string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                11u8, 46u8, 19u8, 255u8, 32u8, 172u8, 123u8, 71u8, 65u8, 152u8, 101u8,
                85u8, 131u8, 237u8, 247u8, 13u8, 237u8, 210u8, 193u8, 220u8, 152u8, 14u8,
                50u8, 156u8, 79u8, 187u8, 47u8, 192u8, 116u8, 139u8, 121u8, 107u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self._0,
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
        impl alloy_sol_types::private::IntoLogData for log_string {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_string> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_string) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_uint(uint256)` and selector `0x2cab9790510fd8bdfbd2115288db33fec66691d476efc5427cfd4c0969301755`.
```solidity
event log_uint(uint256);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_uint {
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_uint {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_uint(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                44u8, 171u8, 151u8, 144u8, 81u8, 15u8, 216u8, 189u8, 251u8, 210u8, 17u8,
                82u8, 136u8, 219u8, 51u8, 254u8, 198u8, 102u8, 145u8, 212u8, 118u8,
                239u8, 197u8, 66u8, 124u8, 253u8, 76u8, 9u8, 105u8, 48u8, 23u8, 85u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
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
        impl alloy_sol_types::private::IntoLogData for log_uint {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_uint> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_uint) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `logs(bytes)` and selector `0xe7950ede0394b9f2ce4a5a1bf5a7e1852411f7e6661b4308c913c4bfd11027e4`.
```solidity
event logs(bytes);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct logs {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for logs {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "logs(bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                231u8, 149u8, 14u8, 222u8, 3u8, 148u8, 185u8, 242u8, 206u8, 74u8, 90u8,
                27u8, 245u8, 167u8, 225u8, 133u8, 36u8, 17u8, 247u8, 230u8, 102u8, 27u8,
                67u8, 8u8, 201u8, 19u8, 196u8, 191u8, 209u8, 16u8, 39u8, 228u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._0,
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
        impl alloy_sol_types::private::IntoLogData for logs {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&logs> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &logs) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(Env env);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub env: <Env as alloy::sol_types::SolType>::RustType,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (Env,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Env as alloy::sol_types::SolType>::RustType,
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
                    (value.env,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { env: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (Env,);
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
                (<Env as alloy_sol_types::SolType>::tokenize(&self.env),)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `IS_TEST()` and selector `0xfa7626d4`.
```solidity
function IS_TEST() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IS_TESTCall {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`IS_TEST()`](IS_TESTCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IS_TESTReturn {
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
            impl ::core::convert::From<IS_TESTCall> for UnderlyingRustTuple<'_> {
                fn from(value: IS_TESTCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for IS_TESTCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<IS_TESTReturn> for UnderlyingRustTuple<'_> {
                fn from(value: IS_TESTReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for IS_TESTReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for IS_TESTCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = IS_TESTReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "IS_TEST()";
            const SELECTOR: [u8; 4] = [250u8, 118u8, 38u8, 212u8];
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `__safeAdd(uint256,uint256)` and selector `0x0d5ec4c6`.
```solidity
function __safeAdd(uint256 x, uint256 y) external pure returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct __safeAddCall {
        #[allow(missing_docs)]
        pub x: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub y: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`__safeAdd(uint256,uint256)`](__safeAddCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct __safeAddReturn {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<__safeAddCall> for UnderlyingRustTuple<'_> {
                fn from(value: __safeAddCall) -> Self {
                    (value.x, value.y)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for __safeAddCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { x: tuple.0, y: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<__safeAddReturn> for UnderlyingRustTuple<'_> {
                fn from(value: __safeAddReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for __safeAddReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for __safeAddCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = __safeAddReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "__safeAdd(uint256,uint256)";
            const SELECTOR: [u8; 4] = [13u8, 94u8, 196u8, 198u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.x),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.y),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `__safeDiv(uint256,uint256)` and selector `0xaceb0e85`.
```solidity
function __safeDiv(uint256 x, uint256 y) external pure returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct __safeDivCall {
        #[allow(missing_docs)]
        pub x: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub y: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`__safeDiv(uint256,uint256)`](__safeDivCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct __safeDivReturn {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<__safeDivCall> for UnderlyingRustTuple<'_> {
                fn from(value: __safeDivCall) -> Self {
                    (value.x, value.y)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for __safeDivCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { x: tuple.0, y: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<__safeDivReturn> for UnderlyingRustTuple<'_> {
                fn from(value: __safeDivReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for __safeDivReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for __safeDivCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = __safeDivReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "__safeDiv(uint256,uint256)";
            const SELECTOR: [u8; 4] = [172u8, 235u8, 14u8, 133u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.x),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.y),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `__safeMod(uint256,uint256)` and selector `0xb165c9e9`.
```solidity
function __safeMod(uint256 x, uint256 y) external pure returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct __safeModCall {
        #[allow(missing_docs)]
        pub x: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub y: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`__safeMod(uint256,uint256)`](__safeModCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct __safeModReturn {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<__safeModCall> for UnderlyingRustTuple<'_> {
                fn from(value: __safeModCall) -> Self {
                    (value.x, value.y)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for __safeModCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { x: tuple.0, y: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<__safeModReturn> for UnderlyingRustTuple<'_> {
                fn from(value: __safeModReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for __safeModReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for __safeModCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = __safeModReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "__safeMod(uint256,uint256)";
            const SELECTOR: [u8; 4] = [177u8, 101u8, 201u8, 233u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.x),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.y),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `__safeMul(uint256,uint256)` and selector `0x76e1fcc4`.
```solidity
function __safeMul(uint256 x, uint256 y) external pure returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct __safeMulCall {
        #[allow(missing_docs)]
        pub x: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub y: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`__safeMul(uint256,uint256)`](__safeMulCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct __safeMulReturn {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<__safeMulCall> for UnderlyingRustTuple<'_> {
                fn from(value: __safeMulCall) -> Self {
                    (value.x, value.y)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for __safeMulCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { x: tuple.0, y: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<__safeMulReturn> for UnderlyingRustTuple<'_> {
                fn from(value: __safeMulReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for __safeMulReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for __safeMulCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = __safeMulReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "__safeMul(uint256,uint256)";
            const SELECTOR: [u8; 4] = [118u8, 225u8, 252u8, 196u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.x),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.y),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `__safeSub(uint256,uint256)` and selector `0x8985c90b`.
```solidity
function __safeSub(uint256 x, uint256 y) external pure returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct __safeSubCall {
        #[allow(missing_docs)]
        pub x: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub y: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`__safeSub(uint256,uint256)`](__safeSubCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct __safeSubReturn {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<__safeSubCall> for UnderlyingRustTuple<'_> {
                fn from(value: __safeSubCall) -> Self {
                    (value.x, value.y)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for __safeSubCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { x: tuple.0, y: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<__safeSubReturn> for UnderlyingRustTuple<'_> {
                fn from(value: __safeSubReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for __safeSubReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for __safeSubCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = __safeSubReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "__safeSub(uint256,uint256)";
            const SELECTOR: [u8; 4] = [137u8, 133u8, 201u8, 11u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.x),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.y),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `actors()` and selector `0xcc7f0c24`.
```solidity
function actors() external view returns (address[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct actorsCall {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`actors()`](actorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct actorsReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<actorsCall> for UnderlyingRustTuple<'_> {
                fn from(value: actorsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for actorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<actorsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: actorsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for actorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for actorsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = actorsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "actors()";
            const SELECTOR: [u8; 4] = [204u8, 127u8, 12u8, 36u8];
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `addLiquidity(uint256,uint256,int24,int24,uint256)` and selector `0x64239cdd`.
```solidity
function addLiquidity(uint256 poolIndex, uint256 routerIndex, int24 lowerTick, int24 upperTick, uint256 liquidity) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addLiquidityCall {
        #[allow(missing_docs)]
        pub poolIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub routerIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub lowerTick: alloy::sol_types::private::primitives::aliases::I24,
        #[allow(missing_docs)]
        pub upperTick: alloy::sol_types::private::primitives::aliases::I24,
        #[allow(missing_docs)]
        pub liquidity: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`addLiquidity(uint256,uint256,int24,int24,uint256)`](addLiquidityCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addLiquidityReturn {}
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::I24,
                alloy::sol_types::private::primitives::aliases::I24,
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
            impl ::core::convert::From<addLiquidityCall> for UnderlyingRustTuple<'_> {
                fn from(value: addLiquidityCall) -> Self {
                    (
                        value.poolIndex,
                        value.routerIndex,
                        value.lowerTick,
                        value.upperTick,
                        value.liquidity,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addLiquidityCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        poolIndex: tuple.0,
                        routerIndex: tuple.1,
                        lowerTick: tuple.2,
                        upperTick: tuple.3,
                        liquidity: tuple.4,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<addLiquidityReturn> for UnderlyingRustTuple<'_> {
                fn from(value: addLiquidityReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addLiquidityReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addLiquidityCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addLiquidityReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addLiquidity(uint256,uint256,int24,int24,uint256)";
            const SELECTOR: [u8; 4] = [100u8, 35u8, 156u8, 221u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.poolIndex),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.routerIndex),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.lowerTick),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.upperTick),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidity),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `enabledAssets()` and selector `0x478ddecc`.
```solidity
function enabledAssets() external view returns (address[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct enabledAssetsCall {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`enabledAssets()`](enabledAssetsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct enabledAssetsReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<enabledAssetsCall> for UnderlyingRustTuple<'_> {
                fn from(value: enabledAssetsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for enabledAssetsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<enabledAssetsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: enabledAssetsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for enabledAssetsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for enabledAssetsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = enabledAssetsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "enabledAssets()";
            const SELECTOR: [u8; 4] = [71u8, 141u8, 222u8, 204u8];
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `excludeArtifacts()` and selector `0xb5508aa9`.
```solidity
function excludeArtifacts() external view returns (string[] memory excludedArtifacts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeArtifactsCall {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`excludeArtifacts()`](excludeArtifactsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeArtifactsReturn {
        #[allow(missing_docs)]
        pub excludedArtifacts_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::String,
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
            impl ::core::convert::From<excludeArtifactsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeArtifactsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeArtifactsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
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
            impl ::core::convert::From<excludeArtifactsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeArtifactsReturn) -> Self {
                    (value.excludedArtifacts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeArtifactsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        excludedArtifacts_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for excludeArtifactsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = excludeArtifactsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "excludeArtifacts()";
            const SELECTOR: [u8; 4] = [181u8, 80u8, 138u8, 169u8];
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `excludeContracts()` and selector `0xe20c9f71`.
```solidity
function excludeContracts() external view returns (address[] memory excludedContracts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeContractsCall {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`excludeContracts()`](excludeContractsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeContractsReturn {
        #[allow(missing_docs)]
        pub excludedContracts_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
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
            impl ::core::convert::From<excludeContractsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeContractsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeContractsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<excludeContractsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeContractsReturn) -> Self {
                    (value.excludedContracts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeContractsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        excludedContracts_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for excludeContractsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = excludeContractsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "excludeContracts()";
            const SELECTOR: [u8; 4] = [226u8, 12u8, 159u8, 113u8];
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `excludeSelectors()` and selector `0xb0464fdc`.
```solidity
function excludeSelectors() external view returns (StdInvariant.FuzzSelector[] memory excludedSelectors_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeSelectorsCall {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`excludeSelectors()`](excludeSelectorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeSelectorsReturn {
        #[allow(missing_docs)]
        pub excludedSelectors_: alloy::sol_types::private::Vec<
            <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<excludeSelectorsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeSelectorsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeSelectorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<excludeSelectorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeSelectorsReturn) -> Self {
                    (value.excludedSelectors_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeSelectorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        excludedSelectors_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for excludeSelectorsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = excludeSelectorsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "excludeSelectors()";
            const SELECTOR: [u8; 4] = [176u8, 70u8, 79u8, 220u8];
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `excludeSenders()` and selector `0x1ed7831c`.
```solidity
function excludeSenders() external view returns (address[] memory excludedSenders_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeSendersCall {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`excludeSenders()`](excludeSendersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeSendersReturn {
        #[allow(missing_docs)]
        pub excludedSenders_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
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
            impl ::core::convert::From<excludeSendersCall> for UnderlyingRustTuple<'_> {
                fn from(value: excludeSendersCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for excludeSendersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<excludeSendersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeSendersReturn) -> Self {
                    (value.excludedSenders_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeSendersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { excludedSenders_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for excludeSendersCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = excludeSendersReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "excludeSenders()";
            const SELECTOR: [u8; 4] = [30u8, 215u8, 131u8, 28u8];
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `failed()` and selector `0xba414fa6`.
```solidity
function failed() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct failedCall {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`failed()`](failedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct failedReturn {
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
            impl ::core::convert::From<failedCall> for UnderlyingRustTuple<'_> {
                fn from(value: failedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for failedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<failedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: failedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for failedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for failedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = failedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "failed()";
            const SELECTOR: [u8; 4] = [186u8, 65u8, 79u8, 166u8];
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getPool(uint256)` and selector `0x068bcd8d`.
```solidity
function getPool(uint256 poolIndex) external view returns (address asset0, address asset1, int24 tickSpacing);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPoolCall {
        #[allow(missing_docs)]
        pub poolIndex: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getPool(uint256)`](getPoolCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPoolReturn {
        #[allow(missing_docs)]
        pub asset0: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub asset1: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub tickSpacing: alloy::sol_types::private::primitives::aliases::I24,
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
            impl ::core::convert::From<getPoolCall> for UnderlyingRustTuple<'_> {
                fn from(value: getPoolCall) -> Self {
                    (value.poolIndex,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPoolCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { poolIndex: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Int<24>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::I24,
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
            impl ::core::convert::From<getPoolReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getPoolReturn) -> Self {
                    (value.asset0, value.asset1, value.tickSpacing)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPoolReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset0: tuple.0,
                        asset1: tuple.1,
                        tickSpacing: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPoolCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getPoolReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Int<24>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPool(uint256)";
            const SELECTOR: [u8; 4] = [6u8, 139u8, 205u8, 141u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.poolIndex),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getPosition(uint256,bytes32)` and selector `0x7477f517`.
```solidity
function getPosition(uint256 poolIndex, PositionKey key) external view returns (LiquidityPosition memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPositionCall {
        #[allow(missing_docs)]
        pub poolIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub key: <PositionKey as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getPosition(uint256,bytes32)`](getPositionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPositionReturn {
        #[allow(missing_docs)]
        pub _0: <LiquidityPosition as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                PositionKey,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                <PositionKey as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getPositionCall> for UnderlyingRustTuple<'_> {
                fn from(value: getPositionCall) -> Self {
                    (value.poolIndex, value.key)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPositionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        poolIndex: tuple.0,
                        key: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (LiquidityPosition,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <LiquidityPosition as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getPositionReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getPositionReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPositionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPositionCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>, PositionKey);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getPositionReturn;
            type ReturnTuple<'a> = (LiquidityPosition,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPosition(uint256,bytes32)";
            const SELECTOR: [u8; 4] = [116u8, 119u8, 245u8, 23u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.poolIndex),
                    <PositionKey as alloy_sol_types::SolType>::tokenize(&self.key),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `ghost_claimedLpRewards(address)` and selector `0x8f5d23ce`.
```solidity
function ghost_claimedLpRewards(address asset) external view returns (uint256 rewards);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ghost_claimedLpRewardsCall {
        #[allow(missing_docs)]
        pub asset: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`ghost_claimedLpRewards(address)`](ghost_claimedLpRewardsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ghost_claimedLpRewardsReturn {
        #[allow(missing_docs)]
        pub rewards: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<ghost_claimedLpRewardsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: ghost_claimedLpRewardsCall) -> Self {
                    (value.asset,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ghost_claimedLpRewardsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { asset: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<ghost_claimedLpRewardsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: ghost_claimedLpRewardsReturn) -> Self {
                    (value.rewards,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ghost_claimedLpRewardsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { rewards: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ghost_claimedLpRewardsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ghost_claimedLpRewardsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ghost_claimedLpRewards(address)";
            const SELECTOR: [u8; 4] = [143u8, 93u8, 35u8, 206u8];
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
                        &self.asset,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `ghost_netSavedDeltas(address)` and selector `0x82716e43`.
```solidity
function ghost_netSavedDeltas(address asset) external view returns (int256 saved);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ghost_netSavedDeltasCall {
        #[allow(missing_docs)]
        pub asset: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`ghost_netSavedDeltas(address)`](ghost_netSavedDeltasCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ghost_netSavedDeltasReturn {
        #[allow(missing_docs)]
        pub saved: alloy::sol_types::private::primitives::aliases::I256,
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
            impl ::core::convert::From<ghost_netSavedDeltasCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: ghost_netSavedDeltasCall) -> Self {
                    (value.asset,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ghost_netSavedDeltasCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { asset: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Int<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::I256,
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
            impl ::core::convert::From<ghost_netSavedDeltasReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: ghost_netSavedDeltasReturn) -> Self {
                    (value.saved,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ghost_netSavedDeltasReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { saved: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ghost_netSavedDeltasCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ghost_netSavedDeltasReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Int<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ghost_netSavedDeltas(address)";
            const SELECTOR: [u8; 4] = [130u8, 113u8, 110u8, 67u8];
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
                        &self.asset,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `ghost_totalDeposits(address)` and selector `0x93db45e0`.
```solidity
function ghost_totalDeposits(address asset) external view returns (uint256 total);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ghost_totalDepositsCall {
        #[allow(missing_docs)]
        pub asset: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`ghost_totalDeposits(address)`](ghost_totalDepositsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ghost_totalDepositsReturn {
        #[allow(missing_docs)]
        pub total: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<ghost_totalDepositsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: ghost_totalDepositsCall) -> Self {
                    (value.asset,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ghost_totalDepositsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { asset: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<ghost_totalDepositsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: ghost_totalDepositsReturn) -> Self {
                    (value.total,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ghost_totalDepositsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { total: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ghost_totalDepositsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ghost_totalDepositsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ghost_totalDeposits(address)";
            const SELECTOR: [u8; 4] = [147u8, 219u8, 69u8, 224u8];
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
                        &self.asset,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `ghost_totalLpRewards(address)` and selector `0x5ee481b6`.
```solidity
function ghost_totalLpRewards(address asset) external view returns (uint256 rewards);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ghost_totalLpRewardsCall {
        #[allow(missing_docs)]
        pub asset: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`ghost_totalLpRewards(address)`](ghost_totalLpRewardsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ghost_totalLpRewardsReturn {
        #[allow(missing_docs)]
        pub rewards: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<ghost_totalLpRewardsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: ghost_totalLpRewardsCall) -> Self {
                    (value.asset,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ghost_totalLpRewardsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { asset: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<ghost_totalLpRewardsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: ghost_totalLpRewardsReturn) -> Self {
                    (value.rewards,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ghost_totalLpRewardsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { rewards: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ghost_totalLpRewardsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ghost_totalLpRewardsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ghost_totalLpRewards(address)";
            const SELECTOR: [u8; 4] = [94u8, 228u8, 129u8, 182u8];
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
                        &self.asset,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `ghost_unclaimableRewards(address)` and selector `0x8068b52e`.
```solidity
function ghost_unclaimableRewards(address asset) external view returns (uint256 rewards);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ghost_unclaimableRewardsCall {
        #[allow(missing_docs)]
        pub asset: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`ghost_unclaimableRewards(address)`](ghost_unclaimableRewardsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ghost_unclaimableRewardsReturn {
        #[allow(missing_docs)]
        pub rewards: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<ghost_unclaimableRewardsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: ghost_unclaimableRewardsCall) -> Self {
                    (value.asset,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ghost_unclaimableRewardsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { asset: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<ghost_unclaimableRewardsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: ghost_unclaimableRewardsReturn) -> Self {
                    (value.rewards,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ghost_unclaimableRewardsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { rewards: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ghost_unclaimableRewardsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ghost_unclaimableRewardsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ghost_unclaimableRewards(address)";
            const SELECTOR: [u8; 4] = [128u8, 104u8, 181u8, 46u8];
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
                        &self.asset,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `initializePool(uint256,uint256,int24,uint24,uint24,uint160)` and selector `0xb15a7971`.
```solidity
function initializePool(uint256 asset0Index, uint256 asset1Index, int24 tickSpacing, uint24 bundleFee, uint24 unlockedFee, uint160 startSqrtPriceX96) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializePoolCall {
        #[allow(missing_docs)]
        pub asset0Index: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub asset1Index: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub tickSpacing: alloy::sol_types::private::primitives::aliases::I24,
        #[allow(missing_docs)]
        pub bundleFee: alloy::sol_types::private::primitives::aliases::U24,
        #[allow(missing_docs)]
        pub unlockedFee: alloy::sol_types::private::primitives::aliases::U24,
        #[allow(missing_docs)]
        pub startSqrtPriceX96: alloy::sol_types::private::primitives::aliases::U160,
    }
    ///Container type for the return parameters of the [`initializePool(uint256,uint256,int24,uint24,uint24,uint160)`](initializePoolCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializePoolReturn {}
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Uint<24>,
                alloy::sol_types::sol_data::Uint<24>,
                alloy::sol_types::sol_data::Uint<160>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::I24,
                alloy::sol_types::private::primitives::aliases::U24,
                alloy::sol_types::private::primitives::aliases::U24,
                alloy::sol_types::private::primitives::aliases::U160,
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
            impl ::core::convert::From<initializePoolCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializePoolCall) -> Self {
                    (
                        value.asset0Index,
                        value.asset1Index,
                        value.tickSpacing,
                        value.bundleFee,
                        value.unlockedFee,
                        value.startSqrtPriceX96,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializePoolCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset0Index: tuple.0,
                        asset1Index: tuple.1,
                        tickSpacing: tuple.2,
                        bundleFee: tuple.3,
                        unlockedFee: tuple.4,
                        startSqrtPriceX96: tuple.5,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<initializePoolReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: initializePoolReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializePoolReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializePoolCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Uint<24>,
                alloy::sol_types::sol_data::Uint<24>,
                alloy::sol_types::sol_data::Uint<160>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializePoolReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initializePool(uint256,uint256,int24,uint24,uint24,uint160)";
            const SELECTOR: [u8; 4] = [177u8, 90u8, 121u8, 113u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.asset0Index),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.asset1Index),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tickSpacing),
                    <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.bundleFee),
                    <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.unlockedFee),
                    <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::SolType>::tokenize(&self.startSqrtPriceX96),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `poolIndexToId(uint256)` and selector `0x88bdca61`.
```solidity
function poolIndexToId(uint256 poolIndex) external view returns (PoolId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct poolIndexToIdCall {
        #[allow(missing_docs)]
        pub poolIndex: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`poolIndexToId(uint256)`](poolIndexToIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct poolIndexToIdReturn {
        #[allow(missing_docs)]
        pub _0: <PoolId as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<poolIndexToIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: poolIndexToIdCall) -> Self {
                    (value.poolIndex,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for poolIndexToIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { poolIndex: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (PoolId,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <PoolId as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<poolIndexToIdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: poolIndexToIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for poolIndexToIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for poolIndexToIdCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = poolIndexToIdReturn;
            type ReturnTuple<'a> = (PoolId,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "poolIndexToId(uint256)";
            const SELECTOR: [u8; 4] = [136u8, 189u8, 202u8, 97u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.poolIndex),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `positionKeys(uint256)` and selector `0x2895a2b3`.
```solidity
function positionKeys(uint256 poolIndex) external view returns (PositionKey[] memory keys);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct positionKeysCall {
        #[allow(missing_docs)]
        pub poolIndex: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`positionKeys(uint256)`](positionKeysCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct positionKeysReturn {
        #[allow(missing_docs)]
        pub keys: alloy::sol_types::private::Vec<
            <PositionKey as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<positionKeysCall> for UnderlyingRustTuple<'_> {
                fn from(value: positionKeysCall) -> Self {
                    (value.poolIndex,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for positionKeysCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { poolIndex: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<PositionKey>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <PositionKey as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<positionKeysReturn> for UnderlyingRustTuple<'_> {
                fn from(value: positionKeysReturn) -> Self {
                    (value.keys,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for positionKeysReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { keys: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for positionKeysCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = positionKeysReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Array<PositionKey>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "positionKeys(uint256)";
            const SELECTOR: [u8; 4] = [40u8, 149u8, 162u8, 179u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.poolIndex),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `removeLiquidity(uint256,uint256,uint256)` and selector `0x857620e1`.
```solidity
function removeLiquidity(uint256 poolIndex, uint256 liquidityRelativeIndex, uint256 liquidityToRemove) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeLiquidityCall {
        #[allow(missing_docs)]
        pub poolIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub liquidityRelativeIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub liquidityToRemove: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`removeLiquidity(uint256,uint256,uint256)`](removeLiquidityCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeLiquidityReturn {}
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<removeLiquidityCall> for UnderlyingRustTuple<'_> {
                fn from(value: removeLiquidityCall) -> Self {
                    (
                        value.poolIndex,
                        value.liquidityRelativeIndex,
                        value.liquidityToRemove,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeLiquidityCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        poolIndex: tuple.0,
                        liquidityRelativeIndex: tuple.1,
                        liquidityToRemove: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<removeLiquidityReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: removeLiquidityReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for removeLiquidityReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removeLiquidityCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeLiquidityReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removeLiquidity(uint256,uint256,uint256)";
            const SELECTOR: [u8; 4] = [133u8, 118u8, 32u8, 225u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.poolIndex),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.liquidityRelativeIndex,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidityToRemove),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `rewardTicks(uint256,uint256,(uint256))` and selector `0xeee8e67b`.
```solidity
function rewardTicks(uint256 poolIndex, uint256 ticksToReward, PRNG memory rng) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rewardTicksCall {
        #[allow(missing_docs)]
        pub poolIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub ticksToReward: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub rng: <PRNG as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`rewardTicks(uint256,uint256,(uint256))`](rewardTicksCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rewardTicksReturn {}
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                PRNG,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                <PRNG as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<rewardTicksCall> for UnderlyingRustTuple<'_> {
                fn from(value: rewardTicksCall) -> Self {
                    (value.poolIndex, value.ticksToReward, value.rng)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for rewardTicksCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        poolIndex: tuple.0,
                        ticksToReward: tuple.1,
                        rng: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<rewardTicksReturn> for UnderlyingRustTuple<'_> {
                fn from(value: rewardTicksReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for rewardTicksReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for rewardTicksCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                PRNG,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = rewardTicksReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "rewardTicks(uint256,uint256,(uint256))";
            const SELECTOR: [u8; 4] = [238u8, 232u8, 230u8, 123u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.poolIndex),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.ticksToReward),
                    <PRNG as alloy_sol_types::SolType>::tokenize(&self.rng),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `routers()` and selector `0xaeb8fbf9`.
```solidity
function routers() external view returns (address[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct routersCall {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`routers()`](routersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct routersReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<routersCall> for UnderlyingRustTuple<'_> {
                fn from(value: routersCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for routersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<routersReturn> for UnderlyingRustTuple<'_> {
                fn from(value: routersReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for routersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for routersCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = routersReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "routers()";
            const SELECTOR: [u8; 4] = [174u8, 184u8, 251u8, 249u8];
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `targetArtifactSelectors()` and selector `0x66d9a9a0`.
```solidity
function targetArtifactSelectors() external view returns (StdInvariant.FuzzArtifactSelector[] memory targetedArtifactSelectors_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactSelectorsCall {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`targetArtifactSelectors()`](targetArtifactSelectorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactSelectorsReturn {
        #[allow(missing_docs)]
        pub targetedArtifactSelectors_: alloy::sol_types::private::Vec<
            <StdInvariant::FuzzArtifactSelector as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<targetArtifactSelectorsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactSelectorsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetArtifactSelectorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzArtifactSelector>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzArtifactSelector as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<targetArtifactSelectorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactSelectorsReturn) -> Self {
                    (value.targetedArtifactSelectors_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetArtifactSelectorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedArtifactSelectors_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetArtifactSelectorsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetArtifactSelectorsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzArtifactSelector>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetArtifactSelectors()";
            const SELECTOR: [u8; 4] = [102u8, 217u8, 169u8, 160u8];
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `targetArtifacts()` and selector `0x85226c81`.
```solidity
function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactsCall {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`targetArtifacts()`](targetArtifactsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactsReturn {
        #[allow(missing_docs)]
        pub targetedArtifacts_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::String,
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
            impl ::core::convert::From<targetArtifactsCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetArtifactsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
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
            impl ::core::convert::From<targetArtifactsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactsReturn) -> Self {
                    (value.targetedArtifacts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetArtifactsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedArtifacts_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetArtifactsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetArtifactsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetArtifacts()";
            const SELECTOR: [u8; 4] = [133u8, 34u8, 108u8, 129u8];
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `targetContracts()` and selector `0x3f7286f4`.
```solidity
function targetContracts() external view returns (address[] memory targetedContracts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetContractsCall {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`targetContracts()`](targetContractsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetContractsReturn {
        #[allow(missing_docs)]
        pub targetedContracts_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
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
            impl ::core::convert::From<targetContractsCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetContractsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetContractsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<targetContractsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetContractsReturn) -> Self {
                    (value.targetedContracts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetContractsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedContracts_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetContractsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetContractsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetContracts()";
            const SELECTOR: [u8; 4] = [63u8, 114u8, 134u8, 244u8];
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `targetInterfaces()` and selector `0x2ade3880`.
```solidity
function targetInterfaces() external view returns (StdInvariant.FuzzInterface[] memory targetedInterfaces_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetInterfacesCall {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`targetInterfaces()`](targetInterfacesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetInterfacesReturn {
        #[allow(missing_docs)]
        pub targetedInterfaces_: alloy::sol_types::private::Vec<
            <StdInvariant::FuzzInterface as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<targetInterfacesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetInterfacesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetInterfacesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzInterface>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzInterface as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<targetInterfacesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetInterfacesReturn) -> Self {
                    (value.targetedInterfaces_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetInterfacesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedInterfaces_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetInterfacesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetInterfacesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzInterface>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetInterfaces()";
            const SELECTOR: [u8; 4] = [42u8, 222u8, 56u8, 128u8];
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `targetSelectors()` and selector `0x916a17c6`.
```solidity
function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSelectorsCall {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`targetSelectors()`](targetSelectorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSelectorsReturn {
        #[allow(missing_docs)]
        pub targetedSelectors_: alloy::sol_types::private::Vec<
            <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<targetSelectorsCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetSelectorsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetSelectorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<targetSelectorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetSelectorsReturn) -> Self {
                    (value.targetedSelectors_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetSelectorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedSelectors_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetSelectorsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetSelectorsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetSelectors()";
            const SELECTOR: [u8; 4] = [145u8, 106u8, 23u8, 198u8];
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `targetSenders()` and selector `0x3e5e3c23`.
```solidity
function targetSenders() external view returns (address[] memory targetedSenders_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSendersCall {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`targetSenders()`](targetSendersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSendersReturn {
        #[allow(missing_docs)]
        pub targetedSenders_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
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
            impl ::core::convert::From<targetSendersCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetSendersCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetSendersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<targetSendersReturn> for UnderlyingRustTuple<'_> {
                fn from(value: targetSendersReturn) -> Self {
                    (value.targetedSenders_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetSendersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { targetedSenders_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetSendersCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetSendersReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetSenders()";
            const SELECTOR: [u8; 4] = [62u8, 94u8, 60u8, 35u8];
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `tickRewards(uint256)` and selector `0x7b2abdb6`.
```solidity
function tickRewards(uint256 poolIndex) external view returns (TickReward[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tickRewardsCall {
        #[allow(missing_docs)]
        pub poolIndex: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`tickRewards(uint256)`](tickRewardsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tickRewardsReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
            <TickReward as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<tickRewardsCall> for UnderlyingRustTuple<'_> {
                fn from(value: tickRewardsCall) -> Self {
                    (value.poolIndex,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for tickRewardsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { poolIndex: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<TickReward>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <TickReward as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<tickRewardsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: tickRewardsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for tickRewardsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for tickRewardsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = tickRewardsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Array<TickReward>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "tickRewards(uint256)";
            const SELECTOR: [u8; 4] = [123u8, 42u8, 189u8, 182u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.poolIndex),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `totalPools()` and selector `0xab3c7e52`.
```solidity
function totalPools() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct totalPoolsCall {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`totalPools()`](totalPoolsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct totalPoolsReturn {
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
            impl ::core::convert::From<totalPoolsCall> for UnderlyingRustTuple<'_> {
                fn from(value: totalPoolsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for totalPoolsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<totalPoolsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: totalPoolsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for totalPoolsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for totalPoolsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = totalPoolsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "totalPools()";
            const SELECTOR: [u8; 4] = [171u8, 60u8, 126u8, 82u8];
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`AngstromHandler`](self) function calls.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum AngstromHandlerCalls {
        #[allow(missing_docs)]
        IS_TEST(IS_TESTCall),
        #[allow(missing_docs)]
        __safeAdd(__safeAddCall),
        #[allow(missing_docs)]
        __safeDiv(__safeDivCall),
        #[allow(missing_docs)]
        __safeMod(__safeModCall),
        #[allow(missing_docs)]
        __safeMul(__safeMulCall),
        #[allow(missing_docs)]
        __safeSub(__safeSubCall),
        #[allow(missing_docs)]
        actors(actorsCall),
        #[allow(missing_docs)]
        addLiquidity(addLiquidityCall),
        #[allow(missing_docs)]
        enabledAssets(enabledAssetsCall),
        #[allow(missing_docs)]
        excludeArtifacts(excludeArtifactsCall),
        #[allow(missing_docs)]
        excludeContracts(excludeContractsCall),
        #[allow(missing_docs)]
        excludeSelectors(excludeSelectorsCall),
        #[allow(missing_docs)]
        excludeSenders(excludeSendersCall),
        #[allow(missing_docs)]
        failed(failedCall),
        #[allow(missing_docs)]
        getPool(getPoolCall),
        #[allow(missing_docs)]
        getPosition(getPositionCall),
        #[allow(missing_docs)]
        ghost_claimedLpRewards(ghost_claimedLpRewardsCall),
        #[allow(missing_docs)]
        ghost_netSavedDeltas(ghost_netSavedDeltasCall),
        #[allow(missing_docs)]
        ghost_totalDeposits(ghost_totalDepositsCall),
        #[allow(missing_docs)]
        ghost_totalLpRewards(ghost_totalLpRewardsCall),
        #[allow(missing_docs)]
        ghost_unclaimableRewards(ghost_unclaimableRewardsCall),
        #[allow(missing_docs)]
        initializePool(initializePoolCall),
        #[allow(missing_docs)]
        poolIndexToId(poolIndexToIdCall),
        #[allow(missing_docs)]
        positionKeys(positionKeysCall),
        #[allow(missing_docs)]
        removeLiquidity(removeLiquidityCall),
        #[allow(missing_docs)]
        rewardTicks(rewardTicksCall),
        #[allow(missing_docs)]
        routers(routersCall),
        #[allow(missing_docs)]
        targetArtifactSelectors(targetArtifactSelectorsCall),
        #[allow(missing_docs)]
        targetArtifacts(targetArtifactsCall),
        #[allow(missing_docs)]
        targetContracts(targetContractsCall),
        #[allow(missing_docs)]
        targetInterfaces(targetInterfacesCall),
        #[allow(missing_docs)]
        targetSelectors(targetSelectorsCall),
        #[allow(missing_docs)]
        targetSenders(targetSendersCall),
        #[allow(missing_docs)]
        tickRewards(tickRewardsCall),
        #[allow(missing_docs)]
        totalPools(totalPoolsCall),
    }
    #[automatically_derived]
    impl AngstromHandlerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [6u8, 139u8, 205u8, 141u8],
            [13u8, 94u8, 196u8, 198u8],
            [30u8, 215u8, 131u8, 28u8],
            [40u8, 149u8, 162u8, 179u8],
            [42u8, 222u8, 56u8, 128u8],
            [62u8, 94u8, 60u8, 35u8],
            [63u8, 114u8, 134u8, 244u8],
            [71u8, 141u8, 222u8, 204u8],
            [94u8, 228u8, 129u8, 182u8],
            [100u8, 35u8, 156u8, 221u8],
            [102u8, 217u8, 169u8, 160u8],
            [116u8, 119u8, 245u8, 23u8],
            [118u8, 225u8, 252u8, 196u8],
            [123u8, 42u8, 189u8, 182u8],
            [128u8, 104u8, 181u8, 46u8],
            [130u8, 113u8, 110u8, 67u8],
            [133u8, 34u8, 108u8, 129u8],
            [133u8, 118u8, 32u8, 225u8],
            [136u8, 189u8, 202u8, 97u8],
            [137u8, 133u8, 201u8, 11u8],
            [143u8, 93u8, 35u8, 206u8],
            [145u8, 106u8, 23u8, 198u8],
            [147u8, 219u8, 69u8, 224u8],
            [171u8, 60u8, 126u8, 82u8],
            [172u8, 235u8, 14u8, 133u8],
            [174u8, 184u8, 251u8, 249u8],
            [176u8, 70u8, 79u8, 220u8],
            [177u8, 90u8, 121u8, 113u8],
            [177u8, 101u8, 201u8, 233u8],
            [181u8, 80u8, 138u8, 169u8],
            [186u8, 65u8, 79u8, 166u8],
            [204u8, 127u8, 12u8, 36u8],
            [226u8, 12u8, 159u8, 113u8],
            [238u8, 232u8, 230u8, 123u8],
            [250u8, 118u8, 38u8, 212u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for AngstromHandlerCalls {
        const NAME: &'static str = "AngstromHandlerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 35usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::IS_TEST(_) => <IS_TESTCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::__safeAdd(_) => {
                    <__safeAddCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::__safeDiv(_) => {
                    <__safeDivCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::__safeMod(_) => {
                    <__safeModCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::__safeMul(_) => {
                    <__safeMulCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::__safeSub(_) => {
                    <__safeSubCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::actors(_) => <actorsCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::addLiquidity(_) => {
                    <addLiquidityCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::enabledAssets(_) => {
                    <enabledAssetsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeArtifacts(_) => {
                    <excludeArtifactsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeContracts(_) => {
                    <excludeContractsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeSelectors(_) => {
                    <excludeSelectorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeSenders(_) => {
                    <excludeSendersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::failed(_) => <failedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getPool(_) => <getPoolCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getPosition(_) => {
                    <getPositionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::ghost_claimedLpRewards(_) => {
                    <ghost_claimedLpRewardsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::ghost_netSavedDeltas(_) => {
                    <ghost_netSavedDeltasCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::ghost_totalDeposits(_) => {
                    <ghost_totalDepositsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::ghost_totalLpRewards(_) => {
                    <ghost_totalLpRewardsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::ghost_unclaimableRewards(_) => {
                    <ghost_unclaimableRewardsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initializePool(_) => {
                    <initializePoolCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::poolIndexToId(_) => {
                    <poolIndexToIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::positionKeys(_) => {
                    <positionKeysCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removeLiquidity(_) => {
                    <removeLiquidityCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::rewardTicks(_) => {
                    <rewardTicksCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::routers(_) => <routersCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::targetArtifactSelectors(_) => {
                    <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetArtifacts(_) => {
                    <targetArtifactsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetContracts(_) => {
                    <targetContractsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetInterfaces(_) => {
                    <targetInterfacesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetSelectors(_) => {
                    <targetSelectorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetSenders(_) => {
                    <targetSendersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::tickRewards(_) => {
                    <tickRewardsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::totalPools(_) => {
                    <totalPoolsCall as alloy_sol_types::SolCall>::SELECTOR
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
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<AngstromHandlerCalls>] = &[
                {
                    fn getPool(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerCalls> {
                        <getPoolCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerCalls::getPool)
                    }
                    getPool
                },
                {
                    fn __safeAdd(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerCalls> {
                        <__safeAddCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerCalls::__safeAdd)
                    }
                    __safeAdd
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn positionKeys(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerCalls> {
                        <positionKeysCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerCalls::positionKeys)
                    }
                    positionKeys
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn targetSenders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn enabledAssets(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerCalls> {
                        <enabledAssetsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerCalls::enabledAssets)
                    }
                    enabledAssets
                },
                {
                    fn ghost_totalLpRewards(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerCalls> {
                        <ghost_totalLpRewardsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerCalls::ghost_totalLpRewards)
                    }
                    ghost_totalLpRewards
                },
                {
                    fn addLiquidity(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerCalls> {
                        <addLiquidityCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerCalls::addLiquidity)
                    }
                    addLiquidity
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn getPosition(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerCalls> {
                        <getPositionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerCalls::getPosition)
                    }
                    getPosition
                },
                {
                    fn __safeMul(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerCalls> {
                        <__safeMulCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerCalls::__safeMul)
                    }
                    __safeMul
                },
                {
                    fn tickRewards(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerCalls> {
                        <tickRewardsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerCalls::tickRewards)
                    }
                    tickRewards
                },
                {
                    fn ghost_unclaimableRewards(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerCalls> {
                        <ghost_unclaimableRewardsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerCalls::ghost_unclaimableRewards)
                    }
                    ghost_unclaimableRewards
                },
                {
                    fn ghost_netSavedDeltas(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerCalls> {
                        <ghost_netSavedDeltasCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerCalls::ghost_netSavedDeltas)
                    }
                    ghost_netSavedDeltas
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn removeLiquidity(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerCalls> {
                        <removeLiquidityCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerCalls::removeLiquidity)
                    }
                    removeLiquidity
                },
                {
                    fn poolIndexToId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerCalls> {
                        <poolIndexToIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerCalls::poolIndexToId)
                    }
                    poolIndexToId
                },
                {
                    fn __safeSub(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerCalls> {
                        <__safeSubCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerCalls::__safeSub)
                    }
                    __safeSub
                },
                {
                    fn ghost_claimedLpRewards(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerCalls> {
                        <ghost_claimedLpRewardsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerCalls::ghost_claimedLpRewards)
                    }
                    ghost_claimedLpRewards
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn ghost_totalDeposits(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerCalls> {
                        <ghost_totalDepositsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerCalls::ghost_totalDeposits)
                    }
                    ghost_totalDeposits
                },
                {
                    fn totalPools(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerCalls> {
                        <totalPoolsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerCalls::totalPools)
                    }
                    totalPools
                },
                {
                    fn __safeDiv(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerCalls> {
                        <__safeDivCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerCalls::__safeDiv)
                    }
                    __safeDiv
                },
                {
                    fn routers(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerCalls> {
                        <routersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerCalls::routers)
                    }
                    routers
                },
                {
                    fn excludeSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerCalls> {
                        <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerCalls::excludeSelectors)
                    }
                    excludeSelectors
                },
                {
                    fn initializePool(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerCalls> {
                        <initializePoolCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerCalls::initializePool)
                    }
                    initializePool
                },
                {
                    fn __safeMod(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerCalls> {
                        <__safeModCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerCalls::__safeMod)
                    }
                    __safeMod
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn failed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerCalls::failed)
                    }
                    failed
                },
                {
                    fn actors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerCalls> {
                        <actorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerCalls::actors)
                    }
                    actors
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn rewardTicks(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerCalls> {
                        <rewardTicksCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerCalls::rewardTicks)
                    }
                    rewardTicks
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerCalls::IS_TEST)
                    }
                    IS_TEST
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
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::__safeAdd(inner) => {
                    <__safeAddCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::__safeDiv(inner) => {
                    <__safeDivCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::__safeMod(inner) => {
                    <__safeModCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::__safeMul(inner) => {
                    <__safeMulCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::__safeSub(inner) => {
                    <__safeSubCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::actors(inner) => {
                    <actorsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::addLiquidity(inner) => {
                    <addLiquidityCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::enabledAssets(inner) => {
                    <enabledAssetsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::excludeArtifacts(inner) => {
                    <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::excludeContracts(inner) => {
                    <excludeContractsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::excludeSelectors(inner) => {
                    <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::excludeSenders(inner) => {
                    <excludeSendersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::failed(inner) => {
                    <failedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getPool(inner) => {
                    <getPoolCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getPosition(inner) => {
                    <getPositionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ghost_claimedLpRewards(inner) => {
                    <ghost_claimedLpRewardsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ghost_netSavedDeltas(inner) => {
                    <ghost_netSavedDeltasCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ghost_totalDeposits(inner) => {
                    <ghost_totalDepositsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ghost_totalLpRewards(inner) => {
                    <ghost_totalLpRewardsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ghost_unclaimableRewards(inner) => {
                    <ghost_unclaimableRewardsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initializePool(inner) => {
                    <initializePoolCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::poolIndexToId(inner) => {
                    <poolIndexToIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::positionKeys(inner) => {
                    <positionKeysCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::removeLiquidity(inner) => {
                    <removeLiquidityCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::rewardTicks(inner) => {
                    <rewardTicksCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::routers(inner) => {
                    <routersCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::targetArtifactSelectors(inner) => {
                    <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetArtifacts(inner) => {
                    <targetArtifactsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetContracts(inner) => {
                    <targetContractsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetInterfaces(inner) => {
                    <targetInterfacesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetSelectors(inner) => {
                    <targetSelectorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetSenders(inner) => {
                    <targetSendersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::tickRewards(inner) => {
                    <tickRewardsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::totalPools(inner) => {
                    <totalPoolsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::__safeAdd(inner) => {
                    <__safeAddCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::__safeDiv(inner) => {
                    <__safeDivCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::__safeMod(inner) => {
                    <__safeModCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::__safeMul(inner) => {
                    <__safeMulCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::__safeSub(inner) => {
                    <__safeSubCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::actors(inner) => {
                    <actorsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::addLiquidity(inner) => {
                    <addLiquidityCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::enabledAssets(inner) => {
                    <enabledAssetsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::excludeArtifacts(inner) => {
                    <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::excludeContracts(inner) => {
                    <excludeContractsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::excludeSelectors(inner) => {
                    <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::excludeSenders(inner) => {
                    <excludeSendersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::failed(inner) => {
                    <failedCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getPool(inner) => {
                    <getPoolCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getPosition(inner) => {
                    <getPositionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ghost_claimedLpRewards(inner) => {
                    <ghost_claimedLpRewardsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ghost_netSavedDeltas(inner) => {
                    <ghost_netSavedDeltasCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ghost_totalDeposits(inner) => {
                    <ghost_totalDepositsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ghost_totalLpRewards(inner) => {
                    <ghost_totalLpRewardsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ghost_unclaimableRewards(inner) => {
                    <ghost_unclaimableRewardsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initializePool(inner) => {
                    <initializePoolCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::poolIndexToId(inner) => {
                    <poolIndexToIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::positionKeys(inner) => {
                    <positionKeysCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::removeLiquidity(inner) => {
                    <removeLiquidityCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::rewardTicks(inner) => {
                    <rewardTicksCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::routers(inner) => {
                    <routersCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::targetArtifactSelectors(inner) => {
                    <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetArtifacts(inner) => {
                    <targetArtifactsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetContracts(inner) => {
                    <targetContractsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetInterfaces(inner) => {
                    <targetInterfacesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetSelectors(inner) => {
                    <targetSelectorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetSenders(inner) => {
                    <targetSendersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::tickRewards(inner) => {
                    <tickRewardsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::totalPools(inner) => {
                    <totalPoolsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`AngstromHandler`](self) custom errors.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum AngstromHandlerErrors {
        #[allow(missing_docs)]
        IndexOutOfBounds(IndexOutOfBounds),
        #[allow(missing_docs)]
        InvalidBounds(InvalidBounds),
        #[allow(missing_docs)]
        OutOfBoundsVecGet(OutOfBoundsVecGet),
        #[allow(missing_docs)]
        Overflow(Overflow),
        #[allow(missing_docs)]
        PairAssetsWrong(PairAssetsWrong),
        #[allow(missing_docs)]
        Underflow(Underflow),
    }
    #[automatically_derived]
    impl AngstromHandlerErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [53u8, 39u8, 141u8, 18u8],
            [68u8, 221u8, 54u8, 159u8],
            [78u8, 35u8, 208u8, 53u8],
            [81u8, 144u8, 52u8, 67u8],
            [168u8, 131u8, 67u8, 87u8],
            [202u8, 204u8, 182u8, 217u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for AngstromHandlerErrors {
        const NAME: &'static str = "AngstromHandlerErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 6usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::IndexOutOfBounds(_) => {
                    <IndexOutOfBounds as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidBounds(_) => {
                    <InvalidBounds as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OutOfBoundsVecGet(_) => {
                    <OutOfBoundsVecGet as alloy_sol_types::SolError>::SELECTOR
                }
                Self::Overflow(_) => <Overflow as alloy_sol_types::SolError>::SELECTOR,
                Self::PairAssetsWrong(_) => {
                    <PairAssetsWrong as alloy_sol_types::SolError>::SELECTOR
                }
                Self::Underflow(_) => <Underflow as alloy_sol_types::SolError>::SELECTOR,
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
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<AngstromHandlerErrors>] = &[
                {
                    fn Overflow(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerErrors> {
                        <Overflow as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerErrors::Overflow)
                    }
                    Overflow
                },
                {
                    fn OutOfBoundsVecGet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerErrors> {
                        <OutOfBoundsVecGet as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerErrors::OutOfBoundsVecGet)
                    }
                    OutOfBoundsVecGet
                },
                {
                    fn IndexOutOfBounds(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerErrors> {
                        <IndexOutOfBounds as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerErrors::IndexOutOfBounds)
                    }
                    IndexOutOfBounds
                },
                {
                    fn PairAssetsWrong(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerErrors> {
                        <PairAssetsWrong as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerErrors::PairAssetsWrong)
                    }
                    PairAssetsWrong
                },
                {
                    fn InvalidBounds(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerErrors> {
                        <InvalidBounds as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerErrors::InvalidBounds)
                    }
                    InvalidBounds
                },
                {
                    fn Underflow(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromHandlerErrors> {
                        <Underflow as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromHandlerErrors::Underflow)
                    }
                    Underflow
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
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::IndexOutOfBounds(inner) => {
                    <IndexOutOfBounds as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidBounds(inner) => {
                    <InvalidBounds as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OutOfBoundsVecGet(inner) => {
                    <OutOfBoundsVecGet as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::Overflow(inner) => {
                    <Overflow as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::PairAssetsWrong(inner) => {
                    <PairAssetsWrong as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::Underflow(inner) => {
                    <Underflow as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::IndexOutOfBounds(inner) => {
                    <IndexOutOfBounds as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidBounds(inner) => {
                    <InvalidBounds as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OutOfBoundsVecGet(inner) => {
                    <OutOfBoundsVecGet as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::Overflow(inner) => {
                    <Overflow as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::PairAssetsWrong(inner) => {
                    <PairAssetsWrong as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::Underflow(inner) => {
                    <Underflow as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`AngstromHandler`](self) events.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum AngstromHandlerEvents {
        #[allow(missing_docs)]
        log(log),
        #[allow(missing_docs)]
        log_address(log_address),
        #[allow(missing_docs)]
        log_array_0(log_array_0),
        #[allow(missing_docs)]
        log_array_1(log_array_1),
        #[allow(missing_docs)]
        log_array_2(log_array_2),
        #[allow(missing_docs)]
        log_bytes(log_bytes),
        #[allow(missing_docs)]
        log_bytes32(log_bytes32),
        #[allow(missing_docs)]
        log_int(log_int),
        #[allow(missing_docs)]
        log_named_address(log_named_address),
        #[allow(missing_docs)]
        log_named_array_0(log_named_array_0),
        #[allow(missing_docs)]
        log_named_array_1(log_named_array_1),
        #[allow(missing_docs)]
        log_named_array_2(log_named_array_2),
        #[allow(missing_docs)]
        log_named_bytes(log_named_bytes),
        #[allow(missing_docs)]
        log_named_bytes32(log_named_bytes32),
        #[allow(missing_docs)]
        log_named_decimal_int(log_named_decimal_int),
        #[allow(missing_docs)]
        log_named_decimal_uint(log_named_decimal_uint),
        #[allow(missing_docs)]
        log_named_int(log_named_int),
        #[allow(missing_docs)]
        log_named_string(log_named_string),
        #[allow(missing_docs)]
        log_named_uint(log_named_uint),
        #[allow(missing_docs)]
        log_string(log_string),
        #[allow(missing_docs)]
        log_uint(log_uint),
        #[allow(missing_docs)]
        logs(logs),
    }
    #[automatically_derived]
    impl AngstromHandlerEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                0u8, 170u8, 163u8, 156u8, 159u8, 251u8, 95u8, 86u8, 122u8, 69u8, 52u8,
                56u8, 12u8, 115u8, 112u8, 117u8, 112u8, 46u8, 31u8, 127u8, 20u8, 16u8,
                127u8, 201u8, 83u8, 40u8, 227u8, 181u8, 108u8, 3u8, 37u8, 251u8,
            ],
            [
                11u8, 46u8, 19u8, 255u8, 32u8, 172u8, 123u8, 71u8, 65u8, 152u8, 101u8,
                85u8, 131u8, 237u8, 247u8, 13u8, 237u8, 210u8, 193u8, 220u8, 152u8, 14u8,
                50u8, 156u8, 79u8, 187u8, 47u8, 192u8, 116u8, 139u8, 121u8, 107u8,
            ],
            [
                14u8, 181u8, 213u8, 38u8, 36u8, 200u8, 210u8, 138u8, 218u8, 159u8, 197u8,
                90u8, 140u8, 80u8, 46u8, 213u8, 170u8, 63u8, 190u8, 47u8, 182u8, 233u8,
                27u8, 113u8, 181u8, 243u8, 118u8, 136u8, 43u8, 29u8, 47u8, 184u8,
            ],
            [
                35u8, 182u8, 42u8, 208u8, 88u8, 77u8, 36u8, 167u8, 95u8, 11u8, 243u8,
                86u8, 3u8, 145u8, 239u8, 86u8, 89u8, 236u8, 109u8, 177u8, 38u8, 156u8,
                86u8, 225u8, 26u8, 162u8, 65u8, 214u8, 55u8, 241u8, 155u8, 32u8,
            ],
            [
                40u8, 15u8, 68u8, 70u8, 178u8, 138u8, 19u8, 114u8, 65u8, 125u8, 218u8,
                101u8, 141u8, 48u8, 185u8, 91u8, 41u8, 146u8, 177u8, 42u8, 201u8, 199u8,
                243u8, 120u8, 83u8, 95u8, 41u8, 169u8, 122u8, 207u8, 53u8, 131u8,
            ],
            [
                44u8, 171u8, 151u8, 144u8, 81u8, 15u8, 216u8, 189u8, 251u8, 210u8, 17u8,
                82u8, 136u8, 219u8, 51u8, 254u8, 198u8, 102u8, 145u8, 212u8, 118u8,
                239u8, 197u8, 66u8, 124u8, 253u8, 76u8, 9u8, 105u8, 48u8, 23u8, 85u8,
            ],
            [
                47u8, 230u8, 50u8, 119u8, 145u8, 116u8, 55u8, 67u8, 120u8, 68u8, 42u8,
                142u8, 151u8, 139u8, 204u8, 251u8, 220u8, 193u8, 214u8, 178u8, 176u8,
                216u8, 31u8, 126u8, 142u8, 183u8, 118u8, 171u8, 34u8, 134u8, 241u8, 104u8,
            ],
            [
                59u8, 207u8, 178u8, 174u8, 46u8, 141u8, 19u8, 45u8, 209u8, 252u8, 231u8,
                207u8, 39u8, 138u8, 154u8, 25u8, 117u8, 106u8, 159u8, 206u8, 171u8,
                228u8, 112u8, 223u8, 59u8, 218u8, 187u8, 75u8, 197u8, 119u8, 209u8, 189u8,
            ],
            [
                64u8, 225u8, 132u8, 15u8, 87u8, 105u8, 7u8, 61u8, 97u8, 189u8, 1u8, 55u8,
                45u8, 155u8, 117u8, 186u8, 169u8, 132u8, 45u8, 86u8, 41u8, 160u8, 201u8,
                159u8, 241u8, 3u8, 190u8, 17u8, 120u8, 168u8, 233u8, 226u8,
            ],
            [
                65u8, 48u8, 79u8, 172u8, 217u8, 50u8, 61u8, 117u8, 177u8, 27u8, 205u8,
                214u8, 9u8, 203u8, 56u8, 239u8, 255u8, 253u8, 176u8, 87u8, 16u8, 247u8,
                202u8, 240u8, 233u8, 177u8, 108u8, 109u8, 157u8, 112u8, 159u8, 80u8,
            ],
            [
                93u8, 166u8, 206u8, 157u8, 81u8, 21u8, 27u8, 161u8, 12u8, 9u8, 165u8,
                89u8, 239u8, 36u8, 213u8, 32u8, 185u8, 218u8, 197u8, 197u8, 184u8, 129u8,
                10u8, 232u8, 67u8, 78u8, 77u8, 13u8, 134u8, 65u8, 26u8, 149u8,
            ],
            [
                122u8, 231u8, 76u8, 82u8, 116u8, 20u8, 174u8, 19u8, 95u8, 217u8, 112u8,
                71u8, 177u8, 41u8, 33u8, 165u8, 236u8, 57u8, 17u8, 184u8, 4u8, 25u8,
                120u8, 85u8, 214u8, 126u8, 37u8, 199u8, 183u8, 94u8, 230u8, 243u8,
            ],
            [
                137u8, 10u8, 130u8, 103u8, 155u8, 71u8, 15u8, 43u8, 216u8, 40u8, 22u8,
                237u8, 155u8, 22u8, 31u8, 151u8, 216u8, 185u8, 103u8, 243u8, 127u8,
                163u8, 100u8, 124u8, 33u8, 213u8, 191u8, 57u8, 116u8, 158u8, 45u8, 213u8,
            ],
            [
                156u8, 78u8, 133u8, 65u8, 202u8, 143u8, 13u8, 193u8, 196u8, 19u8, 249u8,
                16u8, 143u8, 102u8, 216u8, 45u8, 60u8, 236u8, 177u8, 189u8, 219u8, 206u8,
                67u8, 122u8, 97u8, 202u8, 163u8, 23u8, 92u8, 76u8, 201u8, 111u8,
            ],
            [
                167u8, 62u8, 218u8, 9u8, 102u8, 47u8, 70u8, 221u8, 231u8, 41u8, 190u8,
                70u8, 17u8, 56u8, 95u8, 243u8, 79u8, 230u8, 196u8, 79u8, 187u8, 198u8,
                247u8, 225u8, 123u8, 4u8, 43u8, 89u8, 163u8, 68u8, 91u8, 87u8,
            ],
            [
                175u8, 183u8, 149u8, 201u8, 198u8, 30u8, 79u8, 231u8, 70u8, 140u8, 56u8,
                111u8, 146u8, 93u8, 122u8, 84u8, 41u8, 236u8, 173u8, 156u8, 4u8, 149u8,
                221u8, 184u8, 211u8, 141u8, 105u8, 6u8, 20u8, 211u8, 47u8, 153u8,
            ],
            [
                178u8, 222u8, 47u8, 190u8, 128u8, 26u8, 13u8, 246u8, 192u8, 203u8, 221u8,
                253u8, 68u8, 139u8, 163u8, 196u8, 29u8, 72u8, 160u8, 64u8, 202u8, 53u8,
                197u8, 108u8, 129u8, 150u8, 239u8, 15u8, 202u8, 231u8, 33u8, 168u8,
            ],
            [
                210u8, 110u8, 22u8, 202u8, 212u8, 84u8, 135u8, 5u8, 228u8, 201u8, 226u8,
                217u8, 79u8, 152u8, 238u8, 145u8, 194u8, 137u8, 8u8, 94u8, 228u8, 37u8,
                89u8, 79u8, 213u8, 99u8, 95u8, 162u8, 150u8, 76u8, 207u8, 24u8,
            ],
            [
                231u8, 149u8, 14u8, 222u8, 3u8, 148u8, 185u8, 242u8, 206u8, 74u8, 90u8,
                27u8, 245u8, 167u8, 225u8, 133u8, 36u8, 17u8, 247u8, 230u8, 102u8, 27u8,
                67u8, 8u8, 201u8, 19u8, 196u8, 191u8, 209u8, 16u8, 39u8, 228u8,
            ],
            [
                232u8, 22u8, 153u8, 184u8, 81u8, 19u8, 238u8, 161u8, 199u8, 62u8, 16u8,
                88u8, 139u8, 43u8, 3u8, 94u8, 85u8, 137u8, 51u8, 105u8, 99u8, 33u8,
                115u8, 175u8, 212u8, 63u8, 235u8, 25u8, 47u8, 172u8, 100u8, 227u8,
            ],
            [
                235u8, 139u8, 164u8, 60u8, 237u8, 117u8, 55u8, 66u8, 25u8, 70u8, 189u8,
                67u8, 232u8, 40u8, 184u8, 178u8, 184u8, 66u8, 137u8, 39u8, 170u8, 143u8,
                128u8, 28u8, 19u8, 217u8, 52u8, 191u8, 17u8, 172u8, 165u8, 123u8,
            ],
            [
                251u8, 16u8, 40u8, 101u8, 213u8, 10u8, 221u8, 221u8, 246u8, 157u8, 169u8,
                181u8, 170u8, 27u8, 206u8, 214u8, 108u8, 128u8, 207u8, 134u8, 154u8,
                92u8, 141u8, 4u8, 113u8, 164u8, 103u8, 225u8, 140u8, 233u8, 202u8, 177u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for AngstromHandlerEvents {
        const NAME: &'static str = "AngstromHandlerEvents";
        const COUNT: usize = 22usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<log as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log)
                }
                Some(<log_address as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_address as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_address)
                }
                Some(<log_array_0 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_array_0 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_array_0)
                }
                Some(<log_array_1 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_array_1 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_array_1)
                }
                Some(<log_array_2 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_array_2 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_array_2)
                }
                Some(<log_bytes as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_bytes as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_bytes)
                }
                Some(<log_bytes32 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_bytes32 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_bytes32)
                }
                Some(<log_int as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_int as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_int)
                }
                Some(
                    <log_named_address as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_address as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_address)
                }
                Some(
                    <log_named_array_0 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_array_0 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_array_0)
                }
                Some(
                    <log_named_array_1 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_array_1 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_array_1)
                }
                Some(
                    <log_named_array_2 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_array_2 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_array_2)
                }
                Some(<log_named_bytes as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_bytes as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_bytes)
                }
                Some(
                    <log_named_bytes32 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_bytes32 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_bytes32)
                }
                Some(
                    <log_named_decimal_int as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_decimal_int as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_decimal_int)
                }
                Some(
                    <log_named_decimal_uint as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_decimal_uint as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_decimal_uint)
                }
                Some(<log_named_int as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_int as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_int)
                }
                Some(<log_named_string as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_string as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_string)
                }
                Some(<log_named_uint as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_uint as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_uint)
                }
                Some(<log_string as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_string as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_string)
                }
                Some(<log_uint as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_uint as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_uint)
                }
                Some(<logs as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <logs as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::logs)
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
    impl alloy_sol_types::private::IntoLogData for AngstromHandlerEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::log(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_address(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_array_0(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_array_1(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_array_2(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_bytes(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_bytes32(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_int(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_address(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_array_0(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_array_1(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_array_2(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_bytes(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_bytes32(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_decimal_int(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_decimal_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_int(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_string(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_string(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::logs(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::log(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_address(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_array_0(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_array_1(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_array_2(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_bytes(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_bytes32(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_int(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_address(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_array_0(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_array_1(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_array_2(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_bytes(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_bytes32(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_decimal_int(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_decimal_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_int(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_string(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_string(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::logs(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`AngstromHandler`](self) contract instance.

See the [wrapper's documentation](`AngstromHandlerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> AngstromHandlerInstance<T, P, N> {
        AngstromHandlerInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        env: <Env as alloy::sol_types::SolType>::RustType,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<AngstromHandlerInstance<T, P, N>>,
    > {
        AngstromHandlerInstance::<T, P, N>::deploy(provider, env)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        env: <Env as alloy::sol_types::SolType>::RustType,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        AngstromHandlerInstance::<T, P, N>::deploy_builder(provider, env)
    }
    /**A [`AngstromHandler`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`AngstromHandler`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct AngstromHandlerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for AngstromHandlerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("AngstromHandlerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > AngstromHandlerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`AngstromHandler`](self) contract instance.

See the [wrapper's documentation](`AngstromHandlerInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
            env: <Env as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::Result<AngstromHandlerInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, env);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            provider: P,
            env: <Env as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall { env },
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
    impl<T, P: ::core::clone::Clone, N> AngstromHandlerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> AngstromHandlerInstance<T, P, N> {
            AngstromHandlerInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > AngstromHandlerInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`IS_TEST`] function.
        pub fn IS_TEST(&self) -> alloy_contract::SolCallBuilder<T, &P, IS_TESTCall, N> {
            self.call_builder(&IS_TESTCall {})
        }
        ///Creates a new call builder for the [`__safeAdd`] function.
        pub fn __safeAdd(
            &self,
            x: alloy::sol_types::private::primitives::aliases::U256,
            y: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, __safeAddCall, N> {
            self.call_builder(&__safeAddCall { x, y })
        }
        ///Creates a new call builder for the [`__safeDiv`] function.
        pub fn __safeDiv(
            &self,
            x: alloy::sol_types::private::primitives::aliases::U256,
            y: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, __safeDivCall, N> {
            self.call_builder(&__safeDivCall { x, y })
        }
        ///Creates a new call builder for the [`__safeMod`] function.
        pub fn __safeMod(
            &self,
            x: alloy::sol_types::private::primitives::aliases::U256,
            y: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, __safeModCall, N> {
            self.call_builder(&__safeModCall { x, y })
        }
        ///Creates a new call builder for the [`__safeMul`] function.
        pub fn __safeMul(
            &self,
            x: alloy::sol_types::private::primitives::aliases::U256,
            y: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, __safeMulCall, N> {
            self.call_builder(&__safeMulCall { x, y })
        }
        ///Creates a new call builder for the [`__safeSub`] function.
        pub fn __safeSub(
            &self,
            x: alloy::sol_types::private::primitives::aliases::U256,
            y: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, __safeSubCall, N> {
            self.call_builder(&__safeSubCall { x, y })
        }
        ///Creates a new call builder for the [`actors`] function.
        pub fn actors(&self) -> alloy_contract::SolCallBuilder<T, &P, actorsCall, N> {
            self.call_builder(&actorsCall {})
        }
        ///Creates a new call builder for the [`addLiquidity`] function.
        pub fn addLiquidity(
            &self,
            poolIndex: alloy::sol_types::private::primitives::aliases::U256,
            routerIndex: alloy::sol_types::private::primitives::aliases::U256,
            lowerTick: alloy::sol_types::private::primitives::aliases::I24,
            upperTick: alloy::sol_types::private::primitives::aliases::I24,
            liquidity: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, addLiquidityCall, N> {
            self.call_builder(
                &addLiquidityCall {
                    poolIndex,
                    routerIndex,
                    lowerTick,
                    upperTick,
                    liquidity,
                },
            )
        }
        ///Creates a new call builder for the [`enabledAssets`] function.
        pub fn enabledAssets(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, enabledAssetsCall, N> {
            self.call_builder(&enabledAssetsCall {})
        }
        ///Creates a new call builder for the [`excludeArtifacts`] function.
        pub fn excludeArtifacts(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, excludeArtifactsCall, N> {
            self.call_builder(&excludeArtifactsCall {})
        }
        ///Creates a new call builder for the [`excludeContracts`] function.
        pub fn excludeContracts(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, excludeContractsCall, N> {
            self.call_builder(&excludeContractsCall {})
        }
        ///Creates a new call builder for the [`excludeSelectors`] function.
        pub fn excludeSelectors(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, excludeSelectorsCall, N> {
            self.call_builder(&excludeSelectorsCall {})
        }
        ///Creates a new call builder for the [`excludeSenders`] function.
        pub fn excludeSenders(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, excludeSendersCall, N> {
            self.call_builder(&excludeSendersCall {})
        }
        ///Creates a new call builder for the [`failed`] function.
        pub fn failed(&self) -> alloy_contract::SolCallBuilder<T, &P, failedCall, N> {
            self.call_builder(&failedCall {})
        }
        ///Creates a new call builder for the [`getPool`] function.
        pub fn getPool(
            &self,
            poolIndex: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getPoolCall, N> {
            self.call_builder(&getPoolCall { poolIndex })
        }
        ///Creates a new call builder for the [`getPosition`] function.
        pub fn getPosition(
            &self,
            poolIndex: alloy::sol_types::private::primitives::aliases::U256,
            key: <PositionKey as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, getPositionCall, N> {
            self.call_builder(&getPositionCall { poolIndex, key })
        }
        ///Creates a new call builder for the [`ghost_claimedLpRewards`] function.
        pub fn ghost_claimedLpRewards(
            &self,
            asset: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, ghost_claimedLpRewardsCall, N> {
            self.call_builder(
                &ghost_claimedLpRewardsCall {
                    asset,
                },
            )
        }
        ///Creates a new call builder for the [`ghost_netSavedDeltas`] function.
        pub fn ghost_netSavedDeltas(
            &self,
            asset: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, ghost_netSavedDeltasCall, N> {
            self.call_builder(&ghost_netSavedDeltasCall { asset })
        }
        ///Creates a new call builder for the [`ghost_totalDeposits`] function.
        pub fn ghost_totalDeposits(
            &self,
            asset: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, ghost_totalDepositsCall, N> {
            self.call_builder(&ghost_totalDepositsCall { asset })
        }
        ///Creates a new call builder for the [`ghost_totalLpRewards`] function.
        pub fn ghost_totalLpRewards(
            &self,
            asset: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, ghost_totalLpRewardsCall, N> {
            self.call_builder(&ghost_totalLpRewardsCall { asset })
        }
        ///Creates a new call builder for the [`ghost_unclaimableRewards`] function.
        pub fn ghost_unclaimableRewards(
            &self,
            asset: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, ghost_unclaimableRewardsCall, N> {
            self.call_builder(
                &ghost_unclaimableRewardsCall {
                    asset,
                },
            )
        }
        ///Creates a new call builder for the [`initializePool`] function.
        pub fn initializePool(
            &self,
            asset0Index: alloy::sol_types::private::primitives::aliases::U256,
            asset1Index: alloy::sol_types::private::primitives::aliases::U256,
            tickSpacing: alloy::sol_types::private::primitives::aliases::I24,
            bundleFee: alloy::sol_types::private::primitives::aliases::U24,
            unlockedFee: alloy::sol_types::private::primitives::aliases::U24,
            startSqrtPriceX96: alloy::sol_types::private::primitives::aliases::U160,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializePoolCall, N> {
            self.call_builder(
                &initializePoolCall {
                    asset0Index,
                    asset1Index,
                    tickSpacing,
                    bundleFee,
                    unlockedFee,
                    startSqrtPriceX96,
                },
            )
        }
        ///Creates a new call builder for the [`poolIndexToId`] function.
        pub fn poolIndexToId(
            &self,
            poolIndex: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, poolIndexToIdCall, N> {
            self.call_builder(&poolIndexToIdCall { poolIndex })
        }
        ///Creates a new call builder for the [`positionKeys`] function.
        pub fn positionKeys(
            &self,
            poolIndex: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, positionKeysCall, N> {
            self.call_builder(&positionKeysCall { poolIndex })
        }
        ///Creates a new call builder for the [`removeLiquidity`] function.
        pub fn removeLiquidity(
            &self,
            poolIndex: alloy::sol_types::private::primitives::aliases::U256,
            liquidityRelativeIndex: alloy::sol_types::private::primitives::aliases::U256,
            liquidityToRemove: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, removeLiquidityCall, N> {
            self.call_builder(
                &removeLiquidityCall {
                    poolIndex,
                    liquidityRelativeIndex,
                    liquidityToRemove,
                },
            )
        }
        ///Creates a new call builder for the [`rewardTicks`] function.
        pub fn rewardTicks(
            &self,
            poolIndex: alloy::sol_types::private::primitives::aliases::U256,
            ticksToReward: alloy::sol_types::private::primitives::aliases::U256,
            rng: <PRNG as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, rewardTicksCall, N> {
            self.call_builder(
                &rewardTicksCall {
                    poolIndex,
                    ticksToReward,
                    rng,
                },
            )
        }
        ///Creates a new call builder for the [`routers`] function.
        pub fn routers(&self) -> alloy_contract::SolCallBuilder<T, &P, routersCall, N> {
            self.call_builder(&routersCall {})
        }
        ///Creates a new call builder for the [`targetArtifactSelectors`] function.
        pub fn targetArtifactSelectors(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetArtifactSelectorsCall, N> {
            self.call_builder(&targetArtifactSelectorsCall {})
        }
        ///Creates a new call builder for the [`targetArtifacts`] function.
        pub fn targetArtifacts(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetArtifactsCall, N> {
            self.call_builder(&targetArtifactsCall {})
        }
        ///Creates a new call builder for the [`targetContracts`] function.
        pub fn targetContracts(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetContractsCall, N> {
            self.call_builder(&targetContractsCall {})
        }
        ///Creates a new call builder for the [`targetInterfaces`] function.
        pub fn targetInterfaces(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetInterfacesCall, N> {
            self.call_builder(&targetInterfacesCall {})
        }
        ///Creates a new call builder for the [`targetSelectors`] function.
        pub fn targetSelectors(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetSelectorsCall, N> {
            self.call_builder(&targetSelectorsCall {})
        }
        ///Creates a new call builder for the [`targetSenders`] function.
        pub fn targetSenders(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetSendersCall, N> {
            self.call_builder(&targetSendersCall {})
        }
        ///Creates a new call builder for the [`tickRewards`] function.
        pub fn tickRewards(
            &self,
            poolIndex: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, tickRewardsCall, N> {
            self.call_builder(&tickRewardsCall { poolIndex })
        }
        ///Creates a new call builder for the [`totalPools`] function.
        pub fn totalPools(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, totalPoolsCall, N> {
            self.call_builder(&totalPoolsCall {})
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > AngstromHandlerInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`log`] event.
        pub fn log_filter(&self) -> alloy_contract::Event<T, &P, log, N> {
            self.event_filter::<log>()
        }
        ///Creates a new event filter for the [`log_address`] event.
        pub fn log_address_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_address, N> {
            self.event_filter::<log_address>()
        }
        ///Creates a new event filter for the [`log_array_0`] event.
        pub fn log_array_0_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_array_0, N> {
            self.event_filter::<log_array_0>()
        }
        ///Creates a new event filter for the [`log_array_1`] event.
        pub fn log_array_1_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_array_1, N> {
            self.event_filter::<log_array_1>()
        }
        ///Creates a new event filter for the [`log_array_2`] event.
        pub fn log_array_2_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_array_2, N> {
            self.event_filter::<log_array_2>()
        }
        ///Creates a new event filter for the [`log_bytes`] event.
        pub fn log_bytes_filter(&self) -> alloy_contract::Event<T, &P, log_bytes, N> {
            self.event_filter::<log_bytes>()
        }
        ///Creates a new event filter for the [`log_bytes32`] event.
        pub fn log_bytes32_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_bytes32, N> {
            self.event_filter::<log_bytes32>()
        }
        ///Creates a new event filter for the [`log_int`] event.
        pub fn log_int_filter(&self) -> alloy_contract::Event<T, &P, log_int, N> {
            self.event_filter::<log_int>()
        }
        ///Creates a new event filter for the [`log_named_address`] event.
        pub fn log_named_address_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_address, N> {
            self.event_filter::<log_named_address>()
        }
        ///Creates a new event filter for the [`log_named_array_0`] event.
        pub fn log_named_array_0_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_array_0, N> {
            self.event_filter::<log_named_array_0>()
        }
        ///Creates a new event filter for the [`log_named_array_1`] event.
        pub fn log_named_array_1_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_array_1, N> {
            self.event_filter::<log_named_array_1>()
        }
        ///Creates a new event filter for the [`log_named_array_2`] event.
        pub fn log_named_array_2_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_array_2, N> {
            self.event_filter::<log_named_array_2>()
        }
        ///Creates a new event filter for the [`log_named_bytes`] event.
        pub fn log_named_bytes_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_bytes, N> {
            self.event_filter::<log_named_bytes>()
        }
        ///Creates a new event filter for the [`log_named_bytes32`] event.
        pub fn log_named_bytes32_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_bytes32, N> {
            self.event_filter::<log_named_bytes32>()
        }
        ///Creates a new event filter for the [`log_named_decimal_int`] event.
        pub fn log_named_decimal_int_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_decimal_int, N> {
            self.event_filter::<log_named_decimal_int>()
        }
        ///Creates a new event filter for the [`log_named_decimal_uint`] event.
        pub fn log_named_decimal_uint_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_decimal_uint, N> {
            self.event_filter::<log_named_decimal_uint>()
        }
        ///Creates a new event filter for the [`log_named_int`] event.
        pub fn log_named_int_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_int, N> {
            self.event_filter::<log_named_int>()
        }
        ///Creates a new event filter for the [`log_named_string`] event.
        pub fn log_named_string_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_string, N> {
            self.event_filter::<log_named_string>()
        }
        ///Creates a new event filter for the [`log_named_uint`] event.
        pub fn log_named_uint_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_uint, N> {
            self.event_filter::<log_named_uint>()
        }
        ///Creates a new event filter for the [`log_string`] event.
        pub fn log_string_filter(&self) -> alloy_contract::Event<T, &P, log_string, N> {
            self.event_filter::<log_string>()
        }
        ///Creates a new event filter for the [`log_uint`] event.
        pub fn log_uint_filter(&self) -> alloy_contract::Event<T, &P, log_uint, N> {
            self.event_filter::<log_uint>()
        }
        ///Creates a new event filter for the [`logs`] event.
        pub fn logs_filter(&self) -> alloy_contract::Event<T, &P, logs, N> {
            self.event_filter::<logs>()
        }
    }
}
