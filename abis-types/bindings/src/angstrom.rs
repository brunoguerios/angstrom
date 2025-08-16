///Module containing a contract's types and functions.
/**

```solidity
library IPoolManager {
    struct ModifyLiquidityParams { int24 tickLower; int24 tickUpper; int256 liquidityDelta; bytes32 salt; }
    struct SwapParams { bool zeroForOne; int256 amountSpecified; uint160 sqrtPriceLimitX96; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IPoolManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct ModifyLiquidityParams { int24 tickLower; int24 tickUpper; int256 liquidityDelta; bytes32 salt; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ModifyLiquidityParams {
        #[allow(missing_docs)]
        pub tickLower: alloy::sol_types::private::primitives::aliases::I24,
        #[allow(missing_docs)]
        pub tickUpper: alloy::sol_types::private::primitives::aliases::I24,
        #[allow(missing_docs)]
        pub liquidityDelta: alloy::sol_types::private::primitives::aliases::I256,
        #[allow(missing_docs)]
        pub salt: alloy::sol_types::private::FixedBytes<32>,
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
            alloy::sol_types::sol_data::Int<256>,
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::I24,
            alloy::sol_types::private::primitives::aliases::I24,
            alloy::sol_types::private::primitives::aliases::I256,
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
        impl ::core::convert::From<ModifyLiquidityParams> for UnderlyingRustTuple<'_> {
            fn from(value: ModifyLiquidityParams) -> Self {
                (value.tickLower, value.tickUpper, value.liquidityDelta, value.salt)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ModifyLiquidityParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    tickLower: tuple.0,
                    tickUpper: tuple.1,
                    liquidityDelta: tuple.2,
                    salt: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ModifyLiquidityParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ModifyLiquidityParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tickLower),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tickUpper),
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidityDelta),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
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
        impl alloy_sol_types::SolType for ModifyLiquidityParams {
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
        impl alloy_sol_types::SolStruct for ModifyLiquidityParams {
            const NAME: &'static str = "ModifyLiquidityParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ModifyLiquidityParams(int24 tickLower,int24 tickUpper,int256 liquidityDelta,bytes32 salt)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.tickLower)
                        .0,
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.tickUpper)
                        .0,
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.liquidityDelta,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.salt)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ModifyLiquidityParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.tickLower,
                    )
                    + <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.tickUpper,
                    )
                    + <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.liquidityDelta,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.salt)
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
                    &rust.tickLower,
                    out,
                );
                <alloy::sol_types::sol_data::Int<
                    24,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tickUpper,
                    out,
                );
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.liquidityDelta,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.salt,
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
struct SwapParams { bool zeroForOne; int256 amountSpecified; uint160 sqrtPriceLimitX96; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SwapParams {
        #[allow(missing_docs)]
        pub zeroForOne: bool,
        #[allow(missing_docs)]
        pub amountSpecified: alloy::sol_types::private::primitives::aliases::I256,
        #[allow(missing_docs)]
        pub sqrtPriceLimitX96: alloy::sol_types::private::primitives::aliases::U160,
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
            alloy::sol_types::sol_data::Bool,
            alloy::sol_types::sol_data::Int<256>,
            alloy::sol_types::sol_data::Uint<160>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            bool,
            alloy::sol_types::private::primitives::aliases::I256,
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
        impl ::core::convert::From<SwapParams> for UnderlyingRustTuple<'_> {
            fn from(value: SwapParams) -> Self {
                (value.zeroForOne, value.amountSpecified, value.sqrtPriceLimitX96)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SwapParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    zeroForOne: tuple.0,
                    amountSpecified: tuple.1,
                    sqrtPriceLimitX96: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for SwapParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for SwapParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.zeroForOne,
                    ),
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountSpecified),
                    <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::SolType>::tokenize(&self.sqrtPriceLimitX96),
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
        impl alloy_sol_types::SolType for SwapParams {
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
        impl alloy_sol_types::SolStruct for SwapParams {
            const NAME: &'static str = "SwapParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SwapParams(bool zeroForOne,int256 amountSpecified,uint160 sqrtPriceLimitX96)",
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
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.zeroForOne,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.amountSpecified,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.sqrtPriceLimitX96,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for SwapParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.zeroForOne,
                    )
                    + <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amountSpecified,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sqrtPriceLimitX96,
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
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.zeroForOne,
                    out,
                );
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amountSpecified,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    160,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.sqrtPriceLimitX96,
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
    /**Creates a new wrapper around an on-chain [`IPoolManager`](self) contract instance.

See the [wrapper's documentation](`IPoolManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IPoolManagerInstance<T, P, N> {
        IPoolManagerInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IPoolManager`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IPoolManager`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IPoolManagerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IPoolManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IPoolManagerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IPoolManagerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IPoolManager`](self) contract instance.

See the [wrapper's documentation](`IPoolManagerInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IPoolManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IPoolManagerInstance<T, P, N> {
            IPoolManagerInstance {
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
    > IPoolManagerInstance<T, P, N> {
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
    > IPoolManagerInstance<T, P, N> {
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
library IPoolManager {
    struct ModifyLiquidityParams {
        int24 tickLower;
        int24 tickUpper;
        int256 liquidityDelta;
        bytes32 salt;
    }
    struct SwapParams {
        bool zeroForOne;
        int256 amountSpecified;
        uint160 sqrtPriceLimitX96;
    }
}

interface Angstrom {
    type BalanceDelta is int256;
    type BeforeSwapDelta is int256;
    type Currency is address;
    type PoolConfigStore is address;
    type StoreKey is bytes27;
    struct ConfigEntryUpdate {
        uint256 index;
        StoreKey key;
        uint24 bundleFee;
        uint24 unlockedFee;
        uint24 protocolUnlockedFee;
    }
    struct PoolKey {
        Currency currency0;
        Currency currency1;
        uint24 fee;
        int24 tickSpacing;
        address hooks;
    }

    error AssetAccessOutOfBounds(uint256 index, uint256 length);
    error AssetsOutOfOrderOrNotUnique();
    error AssetsUnordered();
    error BundlDeltaUnresolved(address asset);
    error CannotSwapWhileLocked();
    error EntryKeyMismatch();
    error Expired();
    error FailedToDeployNewStore();
    error FeeAboveMax();
    error FillingTooLittle();
    error FillingTooMuch();
    error GasAboveMax();
    error IndexMayHaveChanged();
    error InsufficientCapacity();
    error InvalidHookPermissions();
    error InvalidPermitType(uint8);
    error InvalidSignature();
    error InvalidTickSpacing();
    error JustInTimeLiquidityChange();
    error LimitViolated();
    error NoEntry();
    error NonceReuse();
    error NotController();
    error NotNode();
    error NotUniswap();
    error OnlyOncePerBlock();
    error OrderAlreadyExecuted();
    error OutOfOrderOrDuplicatePairs();
    error Overflow();
    error PairAccessOutOfBounds(uint256 index, uint256 length);
    error ReaderNotAtEnd();
    error ToBGasUsedAboveMax();
    error UnlockDataTooShort();
    error UnlockFeeAboveMax();
    error UnlockedFeeNotSet(StoreKey key);
    error WrongEndLiquidity(uint128 endLiquidity, uint128 actualCurrentLiquidity);

    constructor(address uniV4, address controller);

    function afterSwap(address, PoolKey memory key, IPoolManager.SwapParams memory params, BalanceDelta swap_delta, bytes memory) external returns (bytes4, int128);
    function batchUpdatePools(PoolConfigStore expected_store, ConfigEntryUpdate[] memory updates) external;
    function beforeAddLiquidity(address sender, PoolKey memory key, IPoolManager.ModifyLiquidityParams memory params, bytes memory) external returns (bytes4);
    function beforeRemoveLiquidity(address sender, PoolKey memory key, IPoolManager.ModifyLiquidityParams memory params, bytes memory) external returns (bytes4);
    function beforeSwap(address, PoolKey memory key, IPoolManager.SwapParams memory, bytes memory optionalUnlockData) external returns (bytes4 response, BeforeSwapDelta, uint24 swapFee);
    function collect_unlock_swap_fees(address to, bytes memory packed_assets) external;
    function compose(address from, bytes memory payload) external returns (uint32);
    function configurePool(address asset0, address asset1, uint16 tickSpacing, uint24 bundleFee, uint24 unlockedFee, uint24 protocolUnlockedFee) external;
    function deposit(address asset, uint256 amount) external;
    function deposit(address asset, address to, uint256 amount) external;
    function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
    function execute(bytes memory encoded) external;
    function extsload(uint256 slot) external view returns (uint256);
    function initializePool(address assetA, address assetB, uint256 storeIndex, uint160 sqrtPriceX96) external;
    function invalidateNonce(uint64 nonce) external;
    function pullFee(address asset, uint256 amount) external;
    function removePool(StoreKey key, PoolConfigStore expected_store, uint256 store_index) external;
    function setController(address newController) external;
    function toggleNodes(address[] memory nodes) external;
    function unlockCallback(bytes memory data) external returns (bytes memory);
    function unlockWithEmptyAttestation(address node, bytes memory signature) external;
    function withdraw(address asset, address to, uint256 amount) external;
    function withdraw(address asset, uint256 amount) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "uniV4",
        "type": "address",
        "internalType": "contract IPoolManager"
      },
      {
        "name": "controller",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "afterSwap",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "key",
        "type": "tuple",
        "internalType": "struct PoolKey",
        "components": [
          {
            "name": "currency0",
            "type": "address",
            "internalType": "Currency"
          },
          {
            "name": "currency1",
            "type": "address",
            "internalType": "Currency"
          },
          {
            "name": "fee",
            "type": "uint24",
            "internalType": "uint24"
          },
          {
            "name": "tickSpacing",
            "type": "int24",
            "internalType": "int24"
          },
          {
            "name": "hooks",
            "type": "address",
            "internalType": "contract IHooks"
          }
        ]
      },
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct IPoolManager.SwapParams",
        "components": [
          {
            "name": "zeroForOne",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "amountSpecified",
            "type": "int256",
            "internalType": "int256"
          },
          {
            "name": "sqrtPriceLimitX96",
            "type": "uint160",
            "internalType": "uint160"
          }
        ]
      },
      {
        "name": "swap_delta",
        "type": "int256",
        "internalType": "BalanceDelta"
      },
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes4",
        "internalType": "bytes4"
      },
      {
        "name": "",
        "type": "int128",
        "internalType": "int128"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "batchUpdatePools",
    "inputs": [
      {
        "name": "expected_store",
        "type": "address",
        "internalType": "PoolConfigStore"
      },
      {
        "name": "updates",
        "type": "tuple[]",
        "internalType": "struct ConfigEntryUpdate[]",
        "components": [
          {
            "name": "index",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "key",
            "type": "bytes27",
            "internalType": "StoreKey"
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
            "name": "protocolUnlockedFee",
            "type": "uint24",
            "internalType": "uint24"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "beforeAddLiquidity",
    "inputs": [
      {
        "name": "sender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "key",
        "type": "tuple",
        "internalType": "struct PoolKey",
        "components": [
          {
            "name": "currency0",
            "type": "address",
            "internalType": "Currency"
          },
          {
            "name": "currency1",
            "type": "address",
            "internalType": "Currency"
          },
          {
            "name": "fee",
            "type": "uint24",
            "internalType": "uint24"
          },
          {
            "name": "tickSpacing",
            "type": "int24",
            "internalType": "int24"
          },
          {
            "name": "hooks",
            "type": "address",
            "internalType": "contract IHooks"
          }
        ]
      },
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct IPoolManager.ModifyLiquidityParams",
        "components": [
          {
            "name": "tickLower",
            "type": "int24",
            "internalType": "int24"
          },
          {
            "name": "tickUpper",
            "type": "int24",
            "internalType": "int24"
          },
          {
            "name": "liquidityDelta",
            "type": "int256",
            "internalType": "int256"
          },
          {
            "name": "salt",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      },
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes4",
        "internalType": "bytes4"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "beforeRemoveLiquidity",
    "inputs": [
      {
        "name": "sender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "key",
        "type": "tuple",
        "internalType": "struct PoolKey",
        "components": [
          {
            "name": "currency0",
            "type": "address",
            "internalType": "Currency"
          },
          {
            "name": "currency1",
            "type": "address",
            "internalType": "Currency"
          },
          {
            "name": "fee",
            "type": "uint24",
            "internalType": "uint24"
          },
          {
            "name": "tickSpacing",
            "type": "int24",
            "internalType": "int24"
          },
          {
            "name": "hooks",
            "type": "address",
            "internalType": "contract IHooks"
          }
        ]
      },
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct IPoolManager.ModifyLiquidityParams",
        "components": [
          {
            "name": "tickLower",
            "type": "int24",
            "internalType": "int24"
          },
          {
            "name": "tickUpper",
            "type": "int24",
            "internalType": "int24"
          },
          {
            "name": "liquidityDelta",
            "type": "int256",
            "internalType": "int256"
          },
          {
            "name": "salt",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      },
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes4",
        "internalType": "bytes4"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "beforeSwap",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "key",
        "type": "tuple",
        "internalType": "struct PoolKey",
        "components": [
          {
            "name": "currency0",
            "type": "address",
            "internalType": "Currency"
          },
          {
            "name": "currency1",
            "type": "address",
            "internalType": "Currency"
          },
          {
            "name": "fee",
            "type": "uint24",
            "internalType": "uint24"
          },
          {
            "name": "tickSpacing",
            "type": "int24",
            "internalType": "int24"
          },
          {
            "name": "hooks",
            "type": "address",
            "internalType": "contract IHooks"
          }
        ]
      },
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IPoolManager.SwapParams",
        "components": [
          {
            "name": "zeroForOne",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "amountSpecified",
            "type": "int256",
            "internalType": "int256"
          },
          {
            "name": "sqrtPriceLimitX96",
            "type": "uint160",
            "internalType": "uint160"
          }
        ]
      },
      {
        "name": "optionalUnlockData",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "response",
        "type": "bytes4",
        "internalType": "bytes4"
      },
      {
        "name": "",
        "type": "int256",
        "internalType": "BeforeSwapDelta"
      },
      {
        "name": "swapFee",
        "type": "uint24",
        "internalType": "uint24"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "collect_unlock_swap_fees",
    "inputs": [
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "packed_assets",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "compose",
    "inputs": [
      {
        "name": "from",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "payload",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "configurePool",
    "inputs": [
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
        "type": "uint16",
        "internalType": "uint16"
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
        "name": "protocolUnlockedFee",
        "type": "uint24",
        "internalType": "uint24"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "deposit",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "deposit",
    "inputs": [
      {
        "name": "asset",
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
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "eip712Domain",
    "inputs": [],
    "outputs": [
      {
        "name": "fields",
        "type": "bytes1",
        "internalType": "bytes1"
      },
      {
        "name": "name",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "version",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "chainId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "verifyingContract",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "salt",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "extensions",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "execute",
    "inputs": [
      {
        "name": "encoded",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "extsload",
    "inputs": [
      {
        "name": "slot",
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
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initializePool",
    "inputs": [
      {
        "name": "assetA",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "assetB",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "storeIndex",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "sqrtPriceX96",
        "type": "uint160",
        "internalType": "uint160"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "invalidateNonce",
    "inputs": [
      {
        "name": "nonce",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "pullFee",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "removePool",
    "inputs": [
      {
        "name": "key",
        "type": "bytes27",
        "internalType": "StoreKey"
      },
      {
        "name": "expected_store",
        "type": "address",
        "internalType": "PoolConfigStore"
      },
      {
        "name": "store_index",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setController",
    "inputs": [
      {
        "name": "newController",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "toggleNodes",
    "inputs": [
      {
        "name": "nodes",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "unlockCallback",
    "inputs": [
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
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
    "name": "unlockWithEmptyAttestation",
    "inputs": [
      {
        "name": "node",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "signature",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "withdraw",
    "inputs": [
      {
        "name": "asset",
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
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "withdraw",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "error",
    "name": "AssetAccessOutOfBounds",
    "inputs": [
      {
        "name": "index",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "length",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "AssetsOutOfOrderOrNotUnique",
    "inputs": []
  },
  {
    "type": "error",
    "name": "AssetsUnordered",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BundlDeltaUnresolved",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "CannotSwapWhileLocked",
    "inputs": []
  },
  {
    "type": "error",
    "name": "EntryKeyMismatch",
    "inputs": []
  },
  {
    "type": "error",
    "name": "Expired",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FailedToDeployNewStore",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FeeAboveMax",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FillingTooLittle",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FillingTooMuch",
    "inputs": []
  },
  {
    "type": "error",
    "name": "GasAboveMax",
    "inputs": []
  },
  {
    "type": "error",
    "name": "IndexMayHaveChanged",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InsufficientCapacity",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidHookPermissions",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidPermitType",
    "inputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "uint8"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidTickSpacing",
    "inputs": []
  },
  {
    "type": "error",
    "name": "JustInTimeLiquidityChange",
    "inputs": []
  },
  {
    "type": "error",
    "name": "LimitViolated",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NoEntry",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NonceReuse",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotController",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotNode",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotUniswap",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyOncePerBlock",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OrderAlreadyExecuted",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OutOfOrderOrDuplicatePairs",
    "inputs": []
  },
  {
    "type": "error",
    "name": "Overflow",
    "inputs": []
  },
  {
    "type": "error",
    "name": "PairAccessOutOfBounds",
    "inputs": [
      {
        "name": "index",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "length",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ReaderNotAtEnd",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ToBGasUsedAboveMax",
    "inputs": []
  },
  {
    "type": "error",
    "name": "UnlockDataTooShort",
    "inputs": []
  },
  {
    "type": "error",
    "name": "UnlockFeeAboveMax",
    "inputs": []
  },
  {
    "type": "error",
    "name": "UnlockedFeeNotSet",
    "inputs": [
      {
        "name": "key",
        "type": "bytes27",
        "internalType": "StoreKey"
      }
    ]
  },
  {
    "type": "error",
    "name": "WrongEndLiquidity",
    "inputs": [
      {
        "name": "endLiquidity",
        "type": "uint128",
        "internalType": "uint128"
      },
      {
        "name": "actualCurrentLiquidity",
        "type": "uint128",
        "internalType": "uint128"
      }
    ]
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
pub mod Angstrom {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x610160604052348015610010575f80fd5b50604051616a82380380616a8283398101604081905261002f9161040f565b306080524660a052808260608061007a6040805180820182526008815267416e677374726f6d60c01b60208083019190915282518084019093526002835261763160f01b9083015291565b815160209283012081519183019190912060c082905260e0819052604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f8152938401929092528282015246606083015230608083015260a0909120610100526001600160a01b039384166101208190525f80546001600160a01b0319169587169590951790945551610111925090506103eb565b6001600160a01b039091168152602001604051809103905ff08015801561013a573d5f803e3d5ffd5b506001600160a01b03166101405250610151610158565b5050610447565b61016130610180565b61017e5760405163cb13e96160e01b815260040160405180910390fd5b565b5f816101976001600160a01b0382166130006102a1565b6101a357505f92915050565b6101b86001600160a01b0382166108006102a1565b6101c457505f92915050565b6101d96001600160a01b0382166104006102a1565b156101e657505f92915050565b6101fb6001600160a01b0382166102006102a1565b61020757505f92915050565b61021c6001600160a01b0382166101006102a1565b1561022957505f92915050565b61023d6001600160a01b03821660806102a1565b61024957505f92915050565b61025d6001600160a01b03821660406102a1565b801561027857506102786001600160a01b03821660046102a1565b61028457505f92915050565b61029a6001600160a01b038216628000006102b6565b9392505050565b6001600160a01b038282161615155b92915050565b5f6102cb6001600160a01b03841660806102a1565b1580156102e757506102e76001600160a01b03841660086102a1565b156102f357505f6102b0565b6103076001600160a01b03841660406102a1565b15801561032357506103236001600160a01b03841660046102a1565b1561032f57505f6102b0565b6103446001600160a01b0384166104006102a1565b15801561036057506103606001600160a01b03841660026102a1565b1561036c57505f6102b0565b6103816001600160a01b0384166101006102a1565b15801561039d575061039d6001600160a01b03841660016102a1565b156103a957505f6102b0565b6001600160a01b038316156103da57613fff83161515806103d557506280000062ffffff83161461029a565b61029a565b5062ffffff16628000001415919050565b6108e48061619e83390190565b6001600160a01b038116811461040c575f80fd5b50565b5f8060408385031215610420575f80fd5b825161042b816103f8565b602084015190925061043c816103f8565b809150509250929050565b60805160a05160c05160e051610100516101205161014051615c1161058d5f395f8181610f940152611beb01525f818161046f01528181610955015281816109cd01528181610a2a01528181610acd01528181610b4701528181610c1c01528181610d0c01528181610e92015281816113f00152818161189201528181611bc201528181611d0301528181611d310152818161260501528181612f970152818161303901528181613060015281816134cc015281816135de0152818161361a0152818161364e01528181613692015281816136d101528181613e590152818161402701528181614a2501528181614a9401528181614b660152614bd501525f81816123dd015261413b01525f818161249701526141f501525f818161247101526141cf01525f8181612421015261417f01525f81816123fe015261415c0152615c115ff3fe608060405234801561000f575f80fd5b5060043610610184575f3560e01c80637cf98081116100dd578063b47b2fb111610088578063d9e17f9811610063578063d9e17f98146103f2578063dd4d4cf614610405578063f3fef3a314610418575f80fd5b8063b47b2fb11461037e578063d6cffd1e146103cc578063d9caed12146103df575f80fd5b80638587f450116100b85780638587f4501461033857806391dd73461461034b57806392eefe9b1461036b575f80fd5b80637cf98081146102e95780638340f5491461030a57806384b0196e1461031d575f80fd5b8063259982e51161013d57806353b41c551161011857806353b41c5514610258578063575e24b41461026b5780637407905c146102c1575f80fd5b8063259982e51461021f57806333830e481461023257806347e7ef2414610245575f80fd5b8063138714651161016d57806313871465146101b05780631828e0e7146101c357806321d0ee70146101d6575f80fd5b806309c5eabe14610188578063116a55501461019d575b5f80fd5b61019b610196366004614fe4565b61042b565b005b61019b6101ab366004615023565b61050b565b61019b6101be366004615075565b610518565b61019b6101d13660046150f1565b61076b565b6101e96101e4366004615158565b6108cc565b6040517fffffffff0000000000000000000000000000000000000000000000000000000090911681526020015b60405180910390f35b6101e961022d366004615158565b610be8565b61019b6102403660046150f1565b610f5c565b61019b6102533660046151f7565b610fff565b61019b610266366004615221565b61104f565b61027e6102793660046152b4565b611280565b604080517fffffffff000000000000000000000000000000000000000000000000000000009094168452602084019290925262ffffff1690820152606001610216565b6102d46102cf3660046150f1565b611454565b60405163ffffffff9091168152602001610216565b6102fc6102f736600461530e565b611730565b604051908152602001610216565b61019b610318366004615325565b61173a565b61032561178f565b60405161021697969594939291906153af565b61019b610346366004615461565b611837565b61035e610359366004614fe4565b6119bf565b60405161021691906154b1565b61019b6103793660046154c3565b611a53565b61039161038c3660046154de565b611a94565b604080517fffffffff000000000000000000000000000000000000000000000000000000009093168352600f9190910b602083015201610216565b61019b6103da36600461555f565b611db2565b61019b6103ed366004615325565b611e3e565b61019b6104003660046151f7565b611e8a565b61019b6104133660046155ff565b611eaa565b61019b6104263660046151f7565b611fe0565b61043361202c565b5f81900361043f575050565b6040517f48c894910000000000000000000000000000000000000000000000000000000081526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906348c89491906104a69085908590600401615661565b5f604051808303815f875af11580156104c1573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016820160405261050691908101906156a1565b505050565b61051533826120ff565b50565b61052061213a565b846001600160a01b0316866001600160a01b03161061056b576040517f32b4bc9300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f86815260208690526040812060281b6003549091505f906105a3906801000000000000000090046001600160a01b0316600161217f565b8051519091505f905b80821015610654575f835f015183815181106105ca576105ca615791565b602002602001015190506105e46105de8290565b866121f3565b15610648576106228861061c8b875f0151878151811061060657610606615791565b602002602001015161222290919063ffffffff16565b9061224c565b845180518590811061063657610636615791565b60200260200101818152505050610654565b506001909101906105ac565b80820361067057610670610669858a8a612267565b84906122b2565b6106798361232d565b600380546001600160a01b039290921668010000000000000000027fffffffff0000000000000000000000000000000000000000ffffffffffffffff90921691909117905560408051808201825262ffffff808916825287811660208084018281527fffffffffffffffffffffffffffffffffffffffffffffffffffffff00000000008a165f908152600290925294902092518354945183166301000000027fffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000009095169216919091179290921790556107519061239c565b61075f8662ffffff1661239c565b50505050505050505050565b61078060035467ffffffffffffffff16431490565b156107b7576040517fd8a6b89b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6001600160a01b0383165f9081526001602052604090205460ff16610808576040517f5cd26b6800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b7f3f25e551746414ff93f076a7dd83828ff53735b39366c74015637e004fcb02235f90815243602052604081209061083f826123db565b905061084d858286866124f1565b610883576040517f8baa579f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61088c436125dd565b600380547fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000001667ffffffffffffffff929092169190911790555050505050565b5f6108d56125fa565b5f6108df8661265c565b90505f8061093f838a6108f560208b018b6157cc565b61090560408c0160208d016157cc565b60069081526003919091525f91825260608b01356026908152603a600c209083905292825260209081526040808320848452909152902091565b90925090505f61098461097b6001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001686612670565b60a01c60020b90565b90505f6109bd8261099860208c018c6157cc565b6109a860408d0160208e016157cc565b5f898152600760205260409020929190612690565b90505f6109f46001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000168786612712565b90505f610a18865f01548403836fffffffffffffffffffffffffffffffff1661276d565b90508015610bb6576001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001663a5841194610a5c60208f018f6154c3565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b1681526001600160a01b0390911660048201526024015f604051808303815f87803b158015610ab2575f80fd5b505af1158015610ac4573d5f803e3d5ffd5b50505050610b0f7f0000000000000000000000000000000000000000000000000000000000000000828e5f016020810190610aff91906154c3565b6001600160a01b031691906127e2565b6040517f3dd45adb0000000000000000000000000000000000000000000000000000000081526001600160a01b038e811660048301527f00000000000000000000000000000000000000000000000000000000000000001690633dd45adb906024016020604051808303815f875af1158015610b8d573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610bb191906157e7565b508286555b507f21d0ee70000000000000000000000000000000000000000000000000000000009c9b505050505050505050505050565b5f610bf16125fa565b5f610bfb8661265c565b5f81815260076020526040812091925080610c4261097b6001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001686612670565b90505f83610c5360208b018b6157cc565b62ffffff1663010000008110610c6b57610c6b615791565b015490505f84610c8160408c0160208d016157cc565b62ffffff1663010000008110610c9957610c99615791565b01549050610caa60208b018b6157cc565b60020b8360020b1215610cc1578082039350610e1c565b600283900b610cd660408c0160208d016157cc565b60020b13610dc457610d3486610cef60208d018d6157cc565b8d6060016020810190610d0291906157cc565b6001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016929190612835565b610d6c57630100000085015491508185610d5160208d018d6157cc565b62ffffff1663010000008110610d6957610d69615791565b01555b610d8086610cef60408d0160208e016157cc565b610dba575063010000008401548085610d9f60408d0160208e016157cc565b62ffffff1663010000008110610db757610db7615791565b01555b8181039350610e1c565b610dd586610cef60208d018d6157cc565b610e0d57630100000085015491508185610df260208d018d6157cc565b62ffffff1663010000008110610e0a57610e0a615791565b01555b80828663010000000154030393505b505f9150819050610e7f858c610e3560208d018d6157cc565b610e4560408e0160208f016157cc565b60069081526003919091525f91825260608d01356026908152603a600c209083905292825260209081526040808320848452909152902091565b90925090505f610eb96001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000168784612712565b905060408a01355f610ecb828461582b565b9050826fffffffffffffffffffffffffffffffff165f03610eee57858555610f29565b5f610f23865f01548803856fffffffffffffffffffffffffffffffff16846fffffffffffffffffffffffffffffffff1661288b565b87038655505b507f259982e5000000000000000000000000000000000000000000000000000000009d9c50505050505050505050505050565b610f6461213a565b6040517f877415d20000000000000000000000000000000000000000000000000000000081526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063877415d290610fcd90869086908690600401615853565b5f604051808303815f87803b158015610fe4575f80fd5b505af1158015610ff6573d5f803e3d5ffd5b50505050505050565b6110146001600160a01b038316333084612938565b6001600160a01b0382165f90815260056020908152604080832033845290915281208054839290611046908490615875565b90915550505050565b61105761213a565b6003546001600160a01b0368010000000000000000909104811690841681146110ac576040517ff21fd99f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6003545f906110d0906801000000000000000090046001600160a01b03168261217f565b90505f5b8381101561124957368585838181106110ef576110ef615791565b905060a00201905061112981604001602081019061110d9190615888565b61061c61112060408501602086016158a1565b8690853561299a565b83518051833590811061113e5761113e615791565b602090810291909101015261116661115c6080830160608401615888565b62ffffff1661239c565b61117961115c60a0830160808401615888565b60408051808201909152806111946080840160608501615888565b62ffffff1681526020016111ae60a0840160808501615888565b62ffffff16905260025f6111c860408501602086016158a1565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffff00000000001681526020808201929092526040015f2082518154939092015162ffffff9081166301000000027fffffffffffffffffffffffffffffffffffffffffffffffffffff000000000000909416921691909117919091179055506001016110d4565b506112538161232d565b600360086101000a8154816001600160a01b0302191690836001600160a01b031602179055505050505050565b5f805f61128b6125fa565b6112a060035467ffffffffffffffff16431490565b611357576014841015611319575f8490036112e7576040517f1e8107a000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6040517f4926898b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f61132760148287896158ba565b611330916158e1565b60601c9050365f611344876014818b6158ba565b9150915061135383838361076b565b5050505b5f61139261136e61136b60208b018b6154c3565b90565b61138161136b60408c0160208d016154c3565b5f9182526020526040902060281b90565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffff000000000081165f9081526002602052604081205462ffffff16624000001793509091506113db8961265c565b905061142161141661097b6001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001684612670565b60089060020b612a27565b507f575e24b40000000000000000000000000000000000000000000000000000000093505f925050955095509592505050565b5f600183018335821a8061155157604080517fd505accf0000000000000000000000000000000000000000000000000000000081526001600160a01b03881660048201523360248201527fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6044820152601484013560d81c6064820181905260198501355f90811a60848401819052601a87013560a48501819052603a88013560c486018190529551605a8901983560601c969495929491939192879263d505accf9260e48084019382900301818387803b158015611531575f80fd5b505af1158015611543573d5f803e3d5ffd5b505050505050505050611714565b60018160ff160361162c576040517fd505accf0000000000000000000000000000000000000000000000000000000081526001600160a01b038716600482015233602482810191909152601484013560801c604483018190529084013560d81c6064830181905260298501355f1a60848401819052602a86013560a48501819052604a87013560c48601819052606a8801973560601c95869063d505accf9060e4015b5f604051808303815f87803b15801561160b575f80fd5b505af115801561161d573d5f803e3d5ffd5b50505050505050505050611714565b60028160ff16036116d8576040517f8fcbaf0c0000000000000000000000000000000000000000000000000000000081526001600160a01b0387166004820152336024820152601483013560e01c60448201819052601884013560d81c6064830181905260016084840152601d8501355f1a60a48401819052601e86013560c48501819052603e87013560e48601819052605e8801973560601c958690638fcbaf0c90610104016115f4565b6040517f6f1d150900000000000000000000000000000000000000000000000000000000815260ff821660048201526024015b60405180910390fd5b61171f828686612a2e565b6324a2e44b925050505b9392505050565b5f81545f5260205ff35b61174f6001600160a01b038416333084612938565b6001600160a01b038084165f90815260056020908152604080832093861683529290529081208054839290611785908490615875565b9091555050505050565b7f0f000000000000000000000000000000000000000000000000000000000000006060805f808083611825604080518082018252600881527f416e677374726f6d0000000000000000000000000000000000000000000000006020808301919091528251808401909352600283527f76310000000000000000000000000000000000000000000000000000000000009083015291565b97989097965046955030945091925090565b826001600160a01b0316846001600160a01b03161115611855579192915b5f84815260208490526040812060281b6003549091505f9061188d906801000000000000000090046001600160a01b03168386612a4b565b5090507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316636276cbbe6040518060a001604052806118d18a90565b6001600160a01b03168152602001886001600160a01b03908116825262800000602080840191909152600287810b6040808601919091523060609586015280517fffffffff0000000000000000000000000000000000000000000000000000000060e089901b168152865185166004820152928601518416602484015285015162ffffff1660448301529284015190920b60648301526080909201518216608482015290861660a482015260c4016020604051808303815f875af115801561199b573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ff69190615947565b60606119c96125fa565b825f6119d482612ab5565b60035491935091505f90611a0090849084906801000000000000000090046001600160a01b0316612b6c565b9093509050611a0e82612ce1565b611a188382612d0c565b9250611a248382612d98565b9250611a308382612e4b565b9250611a3d838787612a2e565b611a4682612eea565b60205f525f60205260405ff35b611a5b61213a565b5f80547fffffffffffffffffffffffff0000000000000000000000000000000000000000166001600160a01b0392909216919091179055565b5f80611a9e6125fa565b5f80611ac6611ab361136b60208c018c6154c3565b61138161136b60408d0160208e016154c3565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffff000000000081165f908152600260209081526040822054929350630100000090920462ffffff16918a01803582139190611b1c908c615962565b151582151503611b3557611b308a600f0b90565b611b3f565b611b3f8a60801d90565b90505f8082600f0b12611b525781611b5b565b611b5b82615981565b905082611b995780611b7085620f42406159bd565b60020b611b80620f4240846159fe565b611b8a9190615a4a565b611b949190615abd565b611bb5565b620f4240611bab600286900b836159fe565b611bb59190615a4a565b9550506001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016905063156e29f67f0000000000000000000000000000000000000000000000000000000000000000611c5a611c1a60208f018f615962565b151585151503611c3c578e6020016020810190611c3791906154c3565b611c4e565b8e5f016020810190611c4e91906154c3565b6001600160a01b031690565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e085901b1681526001600160a01b03909216600483015260248201526fffffffffffffffffffffffffffffffff871660448201526064015f604051808303815f87803b158015611ccd575f80fd5b505af1158015611cdf573d5f803e3d5ffd5b505050505050505f611cf08961265c565b90505f611d2961097b6001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001684612670565b9050611d7e827f000000000000000000000000000000000000000000000000000000000000000060085c848e6060016020810190611d6791906157cc565b5f8881526007602052604090209493929190613112565b507fb47b2fb1000000000000000000000000000000000000000000000000000000009350909150505b965096945050505050565b611dba61213a565b5f5b81811015610506575f838383818110611dd757611dd7615791565b9050602002016020810190611dec91906154c3565b6001600160a01b03165f90815260016020819052604090912080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00811660ff90911615179055919091019050611dbc565b6001600160a01b0383165f90815260056020908152604080832033845290915281208054839290611e70908490615b0b565b9091555061050690506001600160a01b03841683836127e2565b611e9261213a565b611ea66001600160a01b03831633836127e2565b5050565b611eb261213a565b6003546001600160a01b036801000000000000000090910481169083168114611f07576040517ff21fd99f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6003545f90611f2a906801000000000000000090046001600160a01b031661319b565b9050611f378186856131c2565b611f408161232d565b600380546001600160a01b039290921668010000000000000000027fffffffff0000000000000000000000000000000000000000ffffffffffffffff909216919091179055505050507fffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000165f90815260026020526040902080547fffffffffffffffffffffffffffffffffffffffffffffffffffff000000000000169055565b6001600160a01b0382165f90815260056020908152604080832033845290915281208054839290612012908490615b0b565b90915550611ea690506001600160a01b03831633836127e2565b6003544367ffffffffffffffff90911603612073576040517fd8a6b89b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b335f9081526001602052604090205460ff166120bb576040517f5cd26b6800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6120c4436125dd565b600380547fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000001667ffffffffffffffff92909216919091179055565b80600c5263daa050e9600452815f52601f600c20600160ff83161b8082541881811661213257638cb888725f526004601cfd5b909155505050565b5f546001600160a01b0316331461217d576040517f23019e6700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b60408051808201909152606081525f60208201525f6121a6846001600160a01b03166132ae565b90505f6121b38483615875565b602084810182905260408051838302810183019091525f81528086529192508381029081906001908401893c506121ea8484612301565b50505092915050565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000828116908216145b92915050565b5f61222c826132c4565b5065ffffff00000061ffff918216601884811c939093161890911b161890565b5f61225682613313565b5062ffffff80831691909118161890565b5f612271836132c4565b61227a82613313565b6122aa8261061c7fffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000871686612222565b949350505050565b815151602083015181036122f2576040517f5cef583a00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61230683612301836001615875565b905152565b81835f0151828151811061231c5761231c615791565b602002602001018181525050505050565b805180516b600b380380600b5f395ff30082525f9190600c60208202016014830184f081835292506001600160a01b038316612395576040517f5670258700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5050919050565b620f424062ffffff82161115610515576105157f140021130000000000000000000000000000000000000000000000000000000062ffffff8316613355565b7f00000000000000000000000000000000000000000000000000000000000000007f000000000000000000000000000000000000000000000000000000000000000030147f00000000000000000000000000000000000000000000000000000000000000004614166124ce5750604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f81527f000000000000000000000000000000000000000000000000000000000000000060208201527f00000000000000000000000000000000000000000000000000000000000000009181019190915246606082015230608082015260a090205b6719010000000000005f5280601a5281603a52604260182090505f603a52919050565b5f6001600160a01b038516156122aa57604051853b61259a578260408114612521576041811461256157506125d4565b60208581013560ff81901c601b0190915285356040527f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff16606052612572565b60408501355f1a6020526040856040375b50845f526020600160805f60015afa5180871860601b3d119250505f606052806040526125d4565b631626ba7e60e01b80825285600483015260248201604081528460448401528486606485013760208160648701858b5afa90519091141691505b50949350505050565b5f6801000000000000000082106125f6576125f661336a565b5090565b336001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161461217d576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6040515f9060a083823760a0902092915050565b5f8181526006602052604081206122aa6001600160a01b03851682613377565b5f808562ffffff8516630100000081106126ac576126ac615791565b015490505f8662ffffff8516630100000081106126cb576126cb615791565b015490508460020b8660020b12156126e657900390506122aa565b8560020b8460020b136126fb570390506122aa565b630100000087015491909103039050949350505050565b5f6006602052825f52600660405f2001602052815f5260405f20602052631e2eaeaf5f5260205f6024601c875afa6127515763535cf94b5f526004601cfd5b50505f516fffffffffffffffffffffffffffffffff1692915050565b81810281838204148315176127d9577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8284098181108201900370010000000000000000000000000000000081106127cc5763c56a01595f526004601cfd5b608091821c911b0161221c565b60801c92915050565b81601452806034526fa9059cbb0000000000000000000000005f5260205f604460105f875af18060015f51141661282b57803d853b15171061282b576390b8ec185f526004601cfd5b505f603452505050565b5f80806128538486078213858705035b600881901d9160ff90911690565b9092509050612880816128706001600160a01b038a1689866133a7565b90600160ff919091161b16151590565b979650505050505050565b82820281838583041485151702612931577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8385098181108201900382848609835f0384168285116128e45763ae47f7025f526004601cfd5b93849004938382119092035f839003839004600101029203041760026003830281188084028203028084028203028084028203028084028203028084028203028084029091030202611729565b0492915050565b60405181606052826040528360601b602c526f23b872dd000000000000000000000000600c5260205f6064601c5f895af18060015f51141661298c57803d873b15171061298c57637939f4245f526004601cfd5b505f60605260405250505050565b5f835f015182815181106129b0576129b0615791565b602002602001015190506129f06129c48290565b847fffffffffffffffffffffffffffffffffffffffffffffffffffffff00000000009081169116141590565b15611729576040517f23f69dc200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b80825d5050565b808201808414612a45576301842f8c5f526004601cfd5b50505050565b5f8080612a626001600160a01b03871686866133de565b905080612a9b576040517f2f659e4400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61ffff601882901c169662ffffff90911695509350505050565b6003818101915f918291803560e81c0101816044612ad38684615b0b565b612add9190615b1e565b905080602086901b1792505f805b82811015612b60575f612b09602087901c60448402015b3560601c90565b9050826001600160a01b0316816001600160a01b031611612b56576040517f80f11acf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b9150600101612aeb565b50829450505050915091565b6003838101935f91829182918291803560e81c0101816026612b8e8a84615b0b565b612b989190615b1e565b905060405193508060c0028401925082604052808460201b179450505f5b82841015612cd45760048901983560e081901c905f90612bde90612b02908c9060f01c61341c565b90505f612bf2612b028c61ffff861661341c565b90508363ffffffff168363ffffffff16111580612c215750806001600160a01b0316826001600160a01b031610155b15612c58576040517ff35f939900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b90865260208601526040852060028b019a91925060281b903560f01c5f80612c8a6001600160a01b038c168585612a4b565b60408a0191909152606089015250505060208a01993590505f760a70c3c40a64e6c51999090b65f67d924000000000000082900460808701525060a085015260c090930192612bb6565b5093505050935093915050565b63ffffffff81165f5b8181101561050657612d04602084901c604483020161347a565b600101612cea565b60408051610160810182525f602082018190529181018290526080810182905260c0810182905260e081018290526101008101829052610140810182905263f3cd914c81526280000060608201523060a082015261012080820152600384810194803560e81c0101905b818514612d8f57612d8885828661353e565b9450612d76565b50929392505050565b6003828101925f91813560e81c9091010181612db2613747565b60408051610120810182525f60208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e08101919091527f0af19d5479e90f25845cea6db89a524bb4e8da3a698213efb1b85e10a5e8be9c815267ffffffffffffffff43166101008201529091505b828614612e4157612e3a86828488613791565b9550612e27565b5093949350505050565b5f80612e55613747565b604080516101a0810182525f80825260208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e0810182905261010081018290526101208101829052610140810182905261016081018290526101808101919091526003868101969293509091803560e81c01015b808614612e4157612ee386838588613975565b9550612ed0565b6040805163ffffffff8316602481028201909252805f5b838110156130ff5760448102602086901c01803560601c6014820135608090811c906034840135901c5f612f4284612f398486615875565b60049190613b49565b90508015612f87576040517fcc67af530000000000000000000000000000000000000000000000000000000081526001600160a01b038516600482015260240161170b565b81156130e1576001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001663a5841194856040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b1681526001600160a01b0390911660048201526024015f604051808303815f87803b158015613013575f80fd5b505af1158015613025573d5f803e3d5ffd5b5061305e925050506001600160a01b0385167f0000000000000000000000000000000000000000000000000000000000000000846127e2565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166311da60b46040518163ffffffff1660e01b81526004016020604051808303815f875af11580156130bb573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906130df91906157e7565b505b6130eb8588613b7f565b505050602493909301925050600101612f01565b506024830282205f5260205fa050505050565b8260020b8260020b1315613156578260020b61313a828460020b613b8890919063ffffffff16565b60020b131561315157613151868587868686613b99565b613193565b8260020b8260020b1215613193575f600284900b828107919091129082900503810260020b8260020b121561319357613193868587868686613c18565b505050505050565b60408051808201909152606081525f602082015261221c6001600160a01b0383165f61217f565b6132146131e8845f015183815181106131dd576131dd615791565b602002602001015190565b837fffffffffffffffffffffffffffffffffffffffffffffffffffffff00000000009081169116141590565b1561324b576040517f23f69dc200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8251515f9061325c90600190615b0b565b9050808210156132a457835180518290811061327a5761327a615791565b6020026020010151845f0151838151811061329757613297615791565b6020026020010181815250505b612a458482905152565b5f61221c60206001600160a01b0384163b615b1e565b600161ffff821610806132dc5750617fff61ffff8216115b15610515576040517f270815a000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b62030d4062ffffff82161115610515576040517f76a3f95d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b815f526001600160a01b03811660045260245ffd5b6335278d125f526004601cfd5b5f81602052631e2eaeaf5f5260205f6024601c865afa61339e5763535cf94b5f526004601cfd5b50505f51919050565b5f82815260066020908152604080832084845260050190915281206133d56001600160a01b03861682613377565b95945050505050565b5f6020826020026001015f863c50505f517fffffffffffffffffffffffffffffffffffffffffffffffffffffff000000000081169190911402919050565b5f8163ffffffff84161161346b576040517fffc31e710000000000000000000000000000000000000000000000000000000081526004810183905263ffffffff8416602482015260440161170b565b602083901c6044830201611729565b602481013560801c8015611ea657604080517f0b0d9c09000000000000000000000000000000000000000000000000000000008152833560601c600482018190523060248301526044820184905291517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691630b0d9c09916064808301925f92919082900301818387803b158015613519575f80fd5b505af115801561352b573d5f803e3d5ffd5b5061050692506004915083905084613ca3565b6001838101935f919035821a9061355a90859083161515613ccf565b60028501943560f01c6135816135708583613d13565b805160208201516040909201519092565b60020b60808801526001600160a01b039081166040880152166020860190815260a090205f60108801883560801c9098506fffffffffffffffffffffffffffffffff1690505f81156136c1575f61360461097b6001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001686612670565b905061360f83613d73565b60e08a015261363e897f0000000000000000000000000000000000000000000000000000000000000000613dd4565b61367461097b6001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001686612670565b60808a01515f8681526007602052604090209193506136bb919086907f00000000000000000000000000000000000000000000000000000000000000009085908790613112565b506136fa565b6136f761097b6001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001685612670565b90505b5f6137216002871615155f86815260076020526040902060808c01518d9190889087613df1565b60208b0151919b5091506137389060049083613b49565b50989998505050505050505050565b5f61378c613753614139565b60408051604281019091527f19010000000000000000000000000000000000000000000000000000000000008152600281019190915290565b905090565b83355f90811a6001818116151560808781019190915290870135811c60208701526011870135811c60408701526021870135811c6060870181905260418801976031013590911c90811115613812576040517f2bae6c5200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6002878101973560f01c90613841908416151561382f8784613d13565b9060051b602081188201519101519091565b6001600160a01b0390811660c08a01521660a0880152506004821661386757865f613871565b60148701873560601c5b6001600160a01b031660e088015296505f61389d61389188610120902090565b60228801526042872090565b90505f600884166138b7576138b2898361422e565b6138c1565b6138c18983614298565b90995090506138d082826142dc565b60e08801518015820217600285161561390f57836fffffffffffffffffffffffffffffffff16896020018181516139079190615875565b905250613937565b836fffffffffffffffffffffffffffffffff16896040018181516139339190615b0b565b9052505b61394f828a60a001518b602001518c60800151614304565b613967818a60c001518b604001518c60800151614374565b509798975050505050505050565b5f8061398185876143d2565b60028201975091505f9081903560f01c6139aa6008851615156139a48884613d13565b906144b2565b6001600160a01b039182166101008c0152911660e08a01529250505060208701873560a08801819052909750811015613a0f576040517f8e1edfa400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60028216613a1e57865f613a28565b60148701873560601c5b6001600160a01b031661012088015296505f613a488860048516156144f4565b6101408a01529098509050613a5e8789856145d6565b97505f80613a6e898b878761461e565b919b50925090505f613a8f613a838b88614828565b60228b015260428a2090565b90505f60808716613aa957613aa48c8361422e565b613ab3565b613ab38c83614298565b909c5090506010871615613aea57613ad68b610180015164ffffffffff16614848565b613ae5818c61016001516120ff565b613af4565b613af482826142dc565b5f8b610120015190508082821502179050613b1a818d6101000151868f60c00151614374565b613b248683614882565b613b38828d60e00151878f60c00151614304565b509a9b9a5050505050505050505050565b6001600160a01b0382165f908152602084905260408120613b77613b6e825c856148ca565b92508183612a27565b509392505050565b60248282375050565b5f8183071291819005919091030290565b63010000008601545b5f613bb86001600160a01b0388168787866148e2565b95509050600285810b9085900b1215613bd15750610ff6565b8015613c12578762ffffff861663010000008110613bf157613bf1615791565b015482038862ffffff871663010000008110613c0f57613c0f615791565b01555b50613ba2565b5f613c2e6001600160a01b03871686868561493c565b94509050600283810b9085900b13613c465750613193565b8015613c90578662ffffff851663010000008110613c6657613c66615791565b0154876301000000015403875f018562ffffff1663010000008110613c8d57613c8d615791565b01555b83613c9a81615b31565b94505050613c18565b6001600160a01b0382165f908152602084905260409020612a45613cc8825c84614974565b8290612a27565b80151560c083015280613cf65773fffd8963efd1fc6a506488495d951d5263988d25613cfd565b6401000276a45b6001600160a01b03166101009092019190915250565b5f8163ffffffff841611613d62576040517ff6601b500000000000000000000000000000000000000000000000000000000081526004810183905263ffffffff8416602482015260440161170b565b5060c08102602083901c0192915050565b5f7f8000000000000000000000000000000000000000000000000000000000000000821115613dce576040517f35278d1200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505f0390565b5f80610144601c85015f855af180610506576040513d5f823e3d81fd5b5f808715613f235760208701968035608090811c9160100135901c811580613e2957506fffffffffffffffffffffffffffffffff8116155b15613e4b57508792506fffffffffffffffffffffffffffffffff169050611da7565b5f613e7f6001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000168961498c565b9050806fffffffffffffffffffffffffffffffff16826fffffffffffffffffffffffffffffffff1614613ede576040517fbecb195c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6fffffffffffffffffffffffffffffffff8116608084901b0463010000008a018054909101905550889350506fffffffffffffffffffffffffffffffff169050611da7565b5f808060038a018a3560e81d909a5090505f60108b018b3560801c909b5090505f806003808e01908e3560e81c8f0101604080516080810182528e815260028e810b60208301528d810b9282018390525f606083018190529496509294508f939290919088900b1315613fa257613f9d83888789856149b0565b613faf565b613faf8388878985614af1565b6fffffffffffffffffffffffffffffffff81167fffffffffffffffffffffffffffffffff000000000000000000000000000000008535908116919091049092019c509a5060109092019650925060801c6140098184615875565b92506140158686614c33565b81515f9061404d906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169061498c565b9050806fffffffffffffffffffffffffffffffff168a6fffffffffffffffffffffffffffffffff16146140c8576040517f6429cfd20000000000000000000000000000000000000000000000000000000081526fffffffffffffffffffffffffffffffff808c1660048301528216602482015260440161170b565b606083810151601489019835821c911c8114614110576040517fbecb195c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505050630100000090920180549098019097555090965093945050505050965096945050505050565b7f00000000000000000000000000000000000000000000000000000000000000007f000000000000000000000000000000000000000000000000000000000000000030147f000000000000000000000000000000000000000000000000000000000000000046141661136b5750604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f81527f000000000000000000000000000000000000000000000000000000000000000060208201527f00000000000000000000000000000000000000000000000000000000000000009181019190915246606082015230608082015260a0902090565b6017601483013560e81c8084018201935f92813560601c9291019061425583868484614c6c565b61428b576040517f8baa579f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b85935050505b9250929050565b5f806040518381525f6020820152604185603f8301376041850194506020600160808360015afa519150503d6142d557638baa579f5f526004601cfd5b9293915050565b80601452815f5260345f20805c156142fb57638a2ef1165f526004601cfd5b6001815d505050565b8161431160048583613ca3565b8115614358576001600160a01b038085165f9081526005602090815260408083209389168352929052908120805483929061434d908490615b0b565b9091555061436d9050565b61436d6001600160a01b038516863084612938565b5050505050565b8161438160048583613b49565b5081156143be576001600160a01b038085165f9081526005602090815260408083209389168352929052908120805483929061434d908490615875565b61436d6001600160a01b03851686836127e2565b60018101905f9035811a600483603c86013760049290920191602081161561444b5760108116614422577f6ee89dee573705c024a086e19a128ee0a5ee0547e3283adfa72fbe336a4c4b6c614444565b7f6be5f22bdcd037f6f35250c32e478fad62195ac2bbab1e2932f8c97af926b4915b845261449e565b60108116614479577f022e170cdf338f45bc718f58d29bfafbf3956c2f9ea8d19ccc9b72e42dbbb7b061449b565b7fb0617b84f694c245e54fb8032ebdc9f56eb26ea2c1b65a46c58f50dbd516e2865b84525b60018116151560c094909401939093525091565b600581901b6020811883015190830180516080909101516060850151620f4240908103906144e08284615b8d565b6144ea9190615b1e565b9150509250925092565b5f807fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470836145cc57843560e81c60038601955060405160146064038101828101604052828882378290206050828101517f7407905c0000000000000000000000000000000000000000000000000000000084526040602485018190527fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffec860160448601529985019960c09490941b77ffffffffffffffffffffffffffffffffffffffff000000009190941c1693019290921717925090505b8492509250925092565b5f6010821615614604576008836101788601376008929092019160058361019b860137600583019250614616565b67ffffffffffffffff43166101608501525b509092915050565b5f80808060208616156146d057508535608090811c604089018190526010880135821c60608a0181905260308901986020013590921c918183101561468f576040517fc4daf00300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b808311156146c9576040517f4418233100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50506146fb565b5060108601953560801c604086166146e8575f6146eb565b60015b60ff166040890152606088018190525b60208701966010810135608090811c9135901c80821115614748576040517f668fef1b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6fffffffffffffffffffffffffffffffff1660808a015260088716156147c3576fffffffffffffffffffffffffffffffff811660608816156147a25782945061479b6147948683614cb1565b8890614cbc565b93506147bd565b91925082916147ba6147b48885614cc7565b82614cd2565b94505b5061481a565b6fffffffffffffffffffffffffffffffff811660608816156147fe5791935083916147f76147f18885614cbc565b82614cb1565b9350614818565b82935061481561480e8583614cd2565b8890614cc7565b94505b505b509597919650945092505050565b5f806010831661483a5761018061483e565b6101a05b9093209392505050565b80421115610515576040517f203d82d800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8115611ea65763ffffffff82168260c01c8260048201528360201c60205f84845f855af1925050506324a2e44b5f5114601f3d111681166105065763f959fdae5f526004601cfd5b8082038281131561221c5763c9654ed45f526004601cfd5b5f8080806148fc6128458688078313878905036001615ba4565b909250905061491f816149196001600160a01b038b168a866133a7565b90614cdd565b909450905061492f828287614d9f565b9250505094509492505050565b5f808080614951858707821386880503612845565b909250905061491f8161496e6001600160a01b038b168a866133a7565b90614dc9565b8181018281121561221c5763c9654ed45f526004601cfd5b5f8181526006602052604081205f6133d56001600160a01b03861660038401613377565b5f80808060018180805b8315614a7f5760108b019a3560801c6149d38185615875565b93506fffffffffffffffffffffffffffffffff8b16608082901b0483019250828e8e62ffffff1663010000008110614a0d57614a0d615791565b015f82825401925050819055505f614a5c8b5f01518f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316614e919092919063ffffffff16565b915050614a698c82614ef6565b9b508d6013528b601052825f5260335f20925050505b885160208a0151614abc916001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016918f90614f10565b809d508195505050886040015160020b8c60020b136149ba5760609890980197909752979a9799509497509495945050505050565b5f80808060018180805b8315614bc05760108b019a3560801c614b148185615875565b93506fffffffffffffffffffffffffffffffff8b16608082901b0483019250828e8e62ffffff1663010000008110614b4e57614b4e615791565b015f82825401925050819055505f614b9d8b5f01518f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316614e919092919063ffffffff16565b915050614baa8c82614f2a565b9b508d6013528b601052825f5260335f20925050505b885160208a0151614bfd916001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016918f906148e2565b809d508195505050886040015160020b8c60020b1315614afb5760609890980197909752979a9799509497509495945050505050565b808214611ea6576040517f01842f8c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f604051631626ba7e60e01b80825285600483015260248201604081528460448401528486606485013760208160648701858b5afa9051909114169695505050505050565b5f6117298284615b0b565b5f6117298284614f44565b5f6117298284614f66565b5f6117298284615875565b5f805f614d788460ff1686901c7e1f0d1e100c1d070f090b19131c1706010e11080a1a141802121b150316040581196001019091166101e07f804040554300526644320000502061067405302602000010750620017611707760fc7fb6db6db6ddddddddd34d34d349249249210842108c6318c639ce739cffffffff840260f81c161b60f71c1690811c63d76453e004601f169190911a1790565b9050806101001415925082614d8e5760ff614d95565b8360ff1681015b9150509250929050565b5f8160ff8416614db5600187900b610100615be5565b614dbf9190615ba4565b6122aa9190615be5565b5f805f8360ff0390505f614e6a8260ff1687901b7f0706060506020504060203020504030106050205030304010505030400000000601f6f8421084210842108cc6318c6db6d54be831560081b6fffffffffffffffffffffffffffffffff851160071b1784811c67ffffffffffffffff1060061b1784811c63ffffffff1060051b1784811c61ffff1060041b1784811c60ff1060031b1793841c1c161a1790565b9050806101001415935083614e7f575f614e86565b8160ff1681035b925050509250929050565b5f806006602052835f52600460405f2001602052825f5260405f20602052631e2eaeaf5f5260205f6024601c885afa614ed15763535cf94b5f526004601cfd5b50505f516fffffffffffffffffffffffffffffffff81169460809190911d9350915050565b808203608081901c1561221c5763c9654ed45f526004601cfd5b5f80808061495161284560018789078413888a05036159bd565b818101608081901c1561221c5763c9654ed45f526004601cfd5b5f6b033b2e3c9fd0803ce8000000614f5c8385615b8d565b6117299190615b1e565b5f611729836b033b2e3c9fd0803ce80000008482820283158482048414178202614f975763ad251c275f526004601cfd5b81810615159190040192915050565b5f8083601f840112614fb6575f80fd5b50813567ffffffffffffffff811115614fcd575f80fd5b602083019150836020828501011115614291575f80fd5b5f8060208385031215614ff5575f80fd5b823567ffffffffffffffff81111561500b575f80fd5b61501785828601614fa6565b90969095509350505050565b5f60208284031215615033575f80fd5b813567ffffffffffffffff81168114611729575f80fd5b6001600160a01b0381168114610515575f80fd5b803562ffffff81168114615070575f80fd5b919050565b5f805f805f8060c0878903121561508a575f80fd5b86356150958161504a565b955060208701356150a58161504a565b9450604087013561ffff811681146150bb575f80fd5b93506150c96060880161505e565b92506150d76080880161505e565b91506150e560a0880161505e565b90509295509295509295565b5f805f60408486031215615103575f80fd5b833561510e8161504a565b9250602084013567ffffffffffffffff811115615129575f80fd5b61513586828701614fa6565b9497909650939450505050565b5f60a08284031215615152575f80fd5b50919050565b5f805f805f85870361016081121561516e575f80fd5b86356151798161504a565b95506151888860208901615142565b945060807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff40820112156151b9575f80fd5b5060c08601925061014086013567ffffffffffffffff8111156151da575f80fd5b6151e688828901614fa6565b969995985093965092949392505050565b5f8060408385031215615208575f80fd5b82356152138161504a565b946020939093013593505050565b5f805f60408486031215615233575f80fd5b833561523e8161504a565b9250602084013567ffffffffffffffff811115615259575f80fd5b8401601f81018613615269575f80fd5b803567ffffffffffffffff81111561527f575f80fd5b86602060a083028401011115615293575f80fd5b939660209190910195509293505050565b5f60608284031215615152575f80fd5b5f805f805f61014086880312156152c9575f80fd5b85356152d48161504a565b94506152e38760208801615142565b93506152f28760c088016152a4565b925061012086013567ffffffffffffffff8111156151da575f80fd5b5f6020828403121561531e575f80fd5b5035919050565b5f805f60608486031215615337575f80fd5b83356153428161504a565b925060208401356153528161504a565b929592945050506040919091013590565b5f81518084528060208401602086015e5f6020828601015260207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011685010191505092915050565b7fff000000000000000000000000000000000000000000000000000000000000008816815260e060208201525f6153e960e0830189615363565b82810360408401526153fb8189615363565b606084018890526001600160a01b038716608085015260a0840186905283810360c0850152845180825260208087019350909101905f5b81811015615450578351835260209384019390920191600101615432565b50909b9a5050505050505050505050565b5f805f8060808587031215615474575f80fd5b843561547f8161504a565b9350602085013561548f8161504a565b92506040850135915060608501356154a68161504a565b939692955090935050565b602081525f6117296020830184615363565b5f602082840312156154d3575f80fd5b81356117298161504a565b5f805f805f8061016087890312156154f4575f80fd5b86356154ff8161504a565b955061550e8860208901615142565b945061551d8860c089016152a4565b9350610120870135925061014087013567ffffffffffffffff811115615541575f80fd5b61554d89828a01614fa6565b979a9699509497509295939492505050565b5f8060208385031215615570575f80fd5b823567ffffffffffffffff811115615586575f80fd5b8301601f81018513615596575f80fd5b803567ffffffffffffffff8111156155ac575f80fd5b8560208260051b84010111156155c0575f80fd5b6020919091019590945092505050565b80357fffffffffffffffffffffffffffffffffffffffffffffffffffffff000000000081168114615070575f80fd5b5f805f60608486031215615611575f80fd5b615342846155d0565b81835281816020850137505f602082840101525f60207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f840116840101905092915050565b602081525f6122aa60208301848661561a565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b5f602082840312156156b1575f80fd5b815167ffffffffffffffff8111156156c7575f80fd5b8201601f810184136156d7575f80fd5b805167ffffffffffffffff8111156156f1576156f1615674565b6040517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0603f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f8501160116810181811067ffffffffffffffff8211171561575d5761575d615674565b604052818152828201602001861015615774575f80fd5b8160208401602083015e5f91810160200191909152949350505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b8060020b8114610515575f80fd5b5f602082840312156157dc575f80fd5b8135611729816157be565b5f602082840312156157f7575f80fd5b5051919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b6fffffffffffffffffffffffffffffffff818116838216019081111561221c5761221c6157fe565b6001600160a01b0384168152604060208201525f6133d560408301848661561a565b8082018082111561221c5761221c6157fe565b5f60208284031215615898575f80fd5b6117298261505e565b5f602082840312156158b1575f80fd5b611729826155d0565b5f80858511156158c8575f80fd5b838611156158d4575f80fd5b5050820193919092039150565b80357fffffffffffffffffffffffffffffffffffffffff0000000000000000000000008116906014841015615940577fffffffffffffffffffffffffffffffffffffffff000000000000000000000000808560140360031b1b82161691505b5092915050565b5f60208284031215615957575f80fd5b8151611729816157be565b5f60208284031215615972575f80fd5b81358015158114611729575f80fd5b5f81600f0b7fffffffffffffffffffffffffffffffff8000000000000000000000000000000081036159b5576159b56157fe565b5f0392915050565b600282810b9082900b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8000008112627fffff8213171561221c5761221c6157fe565b5f82600f0b82600f0b0280600f0b9150808214615940576159406157fe565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f81600f0b83600f0b80615a6057615a60615a1d565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81147fffffffffffffffffffffffffffffffff8000000000000000000000000000000083141615615ab457615ab46157fe565b90059392505050565b600f82810b9082900b037fffffffffffffffffffffffffffffffff8000000000000000000000000000000081126f7fffffffffffffffffffffffffffffff8213171561221c5761221c6157fe565b8181038181111561221c5761221c6157fe565b5f82615b2c57615b2c615a1d565b500490565b5f8160020b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8000008103615b6557615b656157fe565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0192915050565b808202811582820484141761221c5761221c6157fe565b600281810b9083900b01627fffff81137fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8000008212171561221c5761221c6157fe565b5f8260020b8260020b028060020b9150808214615940576159406157fe56fea164736f6c634300081a000a60c0604052348015600e575f80fd5b506040516108e43803806108e4833981016040819052602b91603f565b336080526001600160a01b031660a052606a565b5f60208284031215604e575f80fd5b81516001600160a01b03811681146063575f80fd5b9392505050565b60805160a05161083f6100a55f395f818160e7015281816101e4015281816102c201528181610394015261043f01525f608e015261083f5ff3fe608060405234801561000f575f80fd5b5060043610610034575f3560e01c8063877415d21461003857806391dd73461461004d575b5f80fd5b61004b610046366004610503565b610076565b005b61006061005b36600461056c565b6101ca565b60405161006d91906105ab565b60405180910390f35b3373ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016146100e5576040517f30cd747100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166348c894918460601b8484604051602001610139939291906105fe565b6040516020818303038152906040526040518263ffffffff1660e01b815260040161016491906105ab565b5f604051808303815f875af115801561017f573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01682016040526101c49190810190610666565b50505050565b60603373ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000161461023b576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f6102496014828587610756565b6102529161077d565b60601c90506102648360148187610756565b90945092505f6102756014856107e3565b90505f5b818110156104a6576040517efdd58e0000000000000000000000000000000000000000000000000000000081523060048201526014820287013560601c60248201819052905f907f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff169062fdd58e90604401602060405180830381865afa15801561031b573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061033f919061081b565b6040517ff5298aca00000000000000000000000000000000000000000000000000000000815230600482015273ffffffffffffffffffffffffffffffffffffffff8481166024830152604482018390529192507f00000000000000000000000000000000000000000000000000000000000000009091169063f5298aca906064015f604051808303815f87803b1580156103d7575f80fd5b505af11580156103e9573d5f803e3d5ffd5b50506040517f0b0d9c0900000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff85811660048301528881166024830152604482018590527f0000000000000000000000000000000000000000000000000000000000000000169250630b0d9c0991506064015f604051808303815f87803b158015610482575f80fd5b505af1158015610494573d5f803e3d5ffd5b50506001909401935061027992505050565b505060408051602081019091525f8152949350505050565b5f8083601f8401126104ce575f80fd5b50813567ffffffffffffffff8111156104e5575f80fd5b6020830191508360208285010111156104fc575f80fd5b9250929050565b5f805f60408486031215610515575f80fd5b833573ffffffffffffffffffffffffffffffffffffffff81168114610538575f80fd5b9250602084013567ffffffffffffffff811115610553575f80fd5b61055f868287016104be565b9497909650939450505050565b5f806020838503121561057d575f80fd5b823567ffffffffffffffff811115610593575f80fd5b61059f858286016104be565b90969095509350505050565b602081525f82518060208401528060208501604085015e5f6040828501015260407fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011684010191505092915050565b7fffffffffffffffffffffffffffffffffffffffff00000000000000000000000084168152818360148301375f910160140190815292915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b5f60208284031215610676575f80fd5b815167ffffffffffffffff81111561068c575f80fd5b8201601f8101841361069c575f80fd5b805167ffffffffffffffff8111156106b6576106b6610639565b6040517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0603f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f8501160116810181811067ffffffffffffffff8211171561072257610722610639565b604052818152828201602001861015610739575f80fd5b8160208401602083015e5f91810160200191909152949350505050565b5f8085851115610764575f80fd5b83861115610770575f80fd5b5050820193919092039150565b80357fffffffffffffffffffffffffffffffffffffffff00000000000000000000000081169060148410156107dc577fffffffffffffffffffffffffffffffffffffffff000000000000000000000000808560140360031b1b82161691505b5092915050565b5f82610816577f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b500490565b5f6020828403121561082b575f80fd5b505191905056fea164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01``@R4\x80\x15a\0\x10W_\x80\xFD[P`@Qaj\x828\x03\x80aj\x82\x839\x81\x01`@\x81\x90Ra\0/\x91a\x04\x0FV[0`\x80RF`\xA0R\x80\x82``\x80a\0z`@\x80Q\x80\x82\x01\x82R`\x08\x81RgAngstrom`\xC0\x1B` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R`\x02\x83Rav1`\xF0\x1B\x90\x83\x01R\x91V[\x81Q` \x92\x83\x01 \x81Q\x91\x83\x01\x91\x90\x91 `\xC0\x82\x90R`\xE0\x81\x90R`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81R\x93\x84\x01\x92\x90\x92R\x82\x82\x01RF``\x83\x01R0`\x80\x83\x01R`\xA0\x90\x91 a\x01\0R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16a\x01 \x81\x90R_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x95\x87\x16\x95\x90\x95\x17\x90\x94UQa\x01\x11\x92P\x90Pa\x03\xEBV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x01:W=_\x80>=_\xFD[P`\x01`\x01`\xA0\x1B\x03\x16a\x01@RPa\x01Qa\x01XV[PPa\x04GV[a\x01a0a\x01\x80V[a\x01~W`@Qc\xCB\x13\xE9a`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x81a\x01\x97`\x01`\x01`\xA0\x1B\x03\x82\x16a0\0a\x02\xA1V[a\x01\xA3WP_\x92\x91PPV[a\x01\xB8`\x01`\x01`\xA0\x1B\x03\x82\x16a\x08\0a\x02\xA1V[a\x01\xC4WP_\x92\x91PPV[a\x01\xD9`\x01`\x01`\xA0\x1B\x03\x82\x16a\x04\0a\x02\xA1V[\x15a\x01\xE6WP_\x92\x91PPV[a\x01\xFB`\x01`\x01`\xA0\x1B\x03\x82\x16a\x02\0a\x02\xA1V[a\x02\x07WP_\x92\x91PPV[a\x02\x1C`\x01`\x01`\xA0\x1B\x03\x82\x16a\x01\0a\x02\xA1V[\x15a\x02)WP_\x92\x91PPV[a\x02=`\x01`\x01`\xA0\x1B\x03\x82\x16`\x80a\x02\xA1V[a\x02IWP_\x92\x91PPV[a\x02]`\x01`\x01`\xA0\x1B\x03\x82\x16`@a\x02\xA1V[\x80\x15a\x02xWPa\x02x`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04a\x02\xA1V[a\x02\x84WP_\x92\x91PPV[a\x02\x9A`\x01`\x01`\xA0\x1B\x03\x82\x16b\x80\0\0a\x02\xB6V[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x82\x16\x16\x15\x15[\x92\x91PPV[_a\x02\xCB`\x01`\x01`\xA0\x1B\x03\x84\x16`\x80a\x02\xA1V[\x15\x80\x15a\x02\xE7WPa\x02\xE7`\x01`\x01`\xA0\x1B\x03\x84\x16`\x08a\x02\xA1V[\x15a\x02\xF3WP_a\x02\xB0V[a\x03\x07`\x01`\x01`\xA0\x1B\x03\x84\x16`@a\x02\xA1V[\x15\x80\x15a\x03#WPa\x03#`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04a\x02\xA1V[\x15a\x03/WP_a\x02\xB0V[a\x03D`\x01`\x01`\xA0\x1B\x03\x84\x16a\x04\0a\x02\xA1V[\x15\x80\x15a\x03`WPa\x03``\x01`\x01`\xA0\x1B\x03\x84\x16`\x02a\x02\xA1V[\x15a\x03lWP_a\x02\xB0V[a\x03\x81`\x01`\x01`\xA0\x1B\x03\x84\x16a\x01\0a\x02\xA1V[\x15\x80\x15a\x03\x9DWPa\x03\x9D`\x01`\x01`\xA0\x1B\x03\x84\x16`\x01a\x02\xA1V[\x15a\x03\xA9WP_a\x02\xB0V[`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a\x03\xDAWa?\xFF\x83\x16\x15\x15\x80a\x03\xD5WPb\x80\0\0b\xFF\xFF\xFF\x83\x16\x14a\x02\x9AV[a\x02\x9AV[Pb\xFF\xFF\xFF\x16b\x80\0\0\x14\x15\x91\x90PV[a\x08\xE4\x80aa\x9E\x839\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\x0CW_\x80\xFD[PV[_\x80`@\x83\x85\x03\x12\x15a\x04 W_\x80\xFD[\x82Qa\x04+\x81a\x03\xF8V[` \x84\x01Q\x90\x92Pa\x04<\x81a\x03\xF8V[\x80\x91PP\x92P\x92\x90PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa\\\x11a\x05\x8D_9_\x81\x81a\x0F\x94\x01Ra\x1B\xEB\x01R_\x81\x81a\x04o\x01R\x81\x81a\tU\x01R\x81\x81a\t\xCD\x01R\x81\x81a\n*\x01R\x81\x81a\n\xCD\x01R\x81\x81a\x0BG\x01R\x81\x81a\x0C\x1C\x01R\x81\x81a\r\x0C\x01R\x81\x81a\x0E\x92\x01R\x81\x81a\x13\xF0\x01R\x81\x81a\x18\x92\x01R\x81\x81a\x1B\xC2\x01R\x81\x81a\x1D\x03\x01R\x81\x81a\x1D1\x01R\x81\x81a&\x05\x01R\x81\x81a/\x97\x01R\x81\x81a09\x01R\x81\x81a0`\x01R\x81\x81a4\xCC\x01R\x81\x81a5\xDE\x01R\x81\x81a6\x1A\x01R\x81\x81a6N\x01R\x81\x81a6\x92\x01R\x81\x81a6\xD1\x01R\x81\x81a>Y\x01R\x81\x81a@'\x01R\x81\x81aJ%\x01R\x81\x81aJ\x94\x01R\x81\x81aKf\x01RaK\xD5\x01R_\x81\x81a#\xDD\x01RaA;\x01R_\x81\x81a$\x97\x01RaA\xF5\x01R_\x81\x81a$q\x01RaA\xCF\x01R_\x81\x81a$!\x01RaA\x7F\x01R_\x81\x81a#\xFE\x01RaA\\\x01Ra\\\x11_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\x01\x84W_5`\xE0\x1C\x80c|\xF9\x80\x81\x11a\0\xDDW\x80c\xB4{/\xB1\x11a\0\x88W\x80c\xD9\xE1\x7F\x98\x11a\0cW\x80c\xD9\xE1\x7F\x98\x14a\x03\xF2W\x80c\xDDML\xF6\x14a\x04\x05W\x80c\xF3\xFE\xF3\xA3\x14a\x04\x18W_\x80\xFD[\x80c\xB4{/\xB1\x14a\x03~W\x80c\xD6\xCF\xFD\x1E\x14a\x03\xCCW\x80c\xD9\xCA\xED\x12\x14a\x03\xDFW_\x80\xFD[\x80c\x85\x87\xF4P\x11a\0\xB8W\x80c\x85\x87\xF4P\x14a\x038W\x80c\x91\xDDsF\x14a\x03KW\x80c\x92\xEE\xFE\x9B\x14a\x03kW_\x80\xFD[\x80c|\xF9\x80\x81\x14a\x02\xE9W\x80c\x83@\xF5I\x14a\x03\nW\x80c\x84\xB0\x19n\x14a\x03\x1DW_\x80\xFD[\x80c%\x99\x82\xE5\x11a\x01=W\x80cS\xB4\x1CU\x11a\x01\x18W\x80cS\xB4\x1CU\x14a\x02XW\x80cW^$\xB4\x14a\x02kW\x80ct\x07\x90\\\x14a\x02\xC1W_\x80\xFD[\x80c%\x99\x82\xE5\x14a\x02\x1FW\x80c3\x83\x0EH\x14a\x022W\x80cG\xE7\xEF$\x14a\x02EW_\x80\xFD[\x80c\x13\x87\x14e\x11a\x01mW\x80c\x13\x87\x14e\x14a\x01\xB0W\x80c\x18(\xE0\xE7\x14a\x01\xC3W\x80c!\xD0\xEEp\x14a\x01\xD6W_\x80\xFD[\x80c\t\xC5\xEA\xBE\x14a\x01\x88W\x80c\x11jUP\x14a\x01\x9DW[_\x80\xFD[a\x01\x9Ba\x01\x966`\x04aO\xE4V[a\x04+V[\0[a\x01\x9Ba\x01\xAB6`\x04aP#V[a\x05\x0BV[a\x01\x9Ba\x01\xBE6`\x04aPuV[a\x05\x18V[a\x01\x9Ba\x01\xD16`\x04aP\xF1V[a\x07kV[a\x01\xE9a\x01\xE46`\x04aQXV[a\x08\xCCV[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xE9a\x02-6`\x04aQXV[a\x0B\xE8V[a\x01\x9Ba\x02@6`\x04aP\xF1V[a\x0F\\V[a\x01\x9Ba\x02S6`\x04aQ\xF7V[a\x0F\xFFV[a\x01\x9Ba\x02f6`\x04aR!V[a\x10OV[a\x02~a\x02y6`\x04aR\xB4V[a\x12\x80V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x94\x16\x84R` \x84\x01\x92\x90\x92Rb\xFF\xFF\xFF\x16\x90\x82\x01R``\x01a\x02\x16V[a\x02\xD4a\x02\xCF6`\x04aP\xF1V[a\x14TV[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x16V[a\x02\xFCa\x02\xF76`\x04aS\x0EV[a\x170V[`@Q\x90\x81R` \x01a\x02\x16V[a\x01\x9Ba\x03\x186`\x04aS%V[a\x17:V[a\x03%a\x17\x8FV[`@Qa\x02\x16\x97\x96\x95\x94\x93\x92\x91\x90aS\xAFV[a\x01\x9Ba\x03F6`\x04aTaV[a\x187V[a\x03^a\x03Y6`\x04aO\xE4V[a\x19\xBFV[`@Qa\x02\x16\x91\x90aT\xB1V[a\x01\x9Ba\x03y6`\x04aT\xC3V[a\x1ASV[a\x03\x91a\x03\x8C6`\x04aT\xDEV[a\x1A\x94V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x93\x16\x83R`\x0F\x91\x90\x91\x0B` \x83\x01R\x01a\x02\x16V[a\x01\x9Ba\x03\xDA6`\x04aU_V[a\x1D\xB2V[a\x01\x9Ba\x03\xED6`\x04aS%V[a\x1E>V[a\x01\x9Ba\x04\x006`\x04aQ\xF7V[a\x1E\x8AV[a\x01\x9Ba\x04\x136`\x04aU\xFFV[a\x1E\xAAV[a\x01\x9Ba\x04&6`\x04aQ\xF7V[a\x1F\xE0V[a\x043a ,V[_\x81\x90\x03a\x04?WPPV[`@Q\x7FH\xC8\x94\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cH\xC8\x94\x91\x90a\x04\xA6\x90\x85\x90\x85\x90`\x04\x01aVaV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x04\xC1W=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x05\x06\x91\x90\x81\x01\x90aV\xA1V[PPPV[a\x05\x153\x82a \xFFV[PV[a\x05 a!:V[\x84`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x10a\x05kW`@Q\x7F2\xB4\xBC\x93\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x86\x81R` \x86\x90R`@\x81 `(\x1B`\x03T\x90\x91P_\x90a\x05\xA3\x90h\x01\0\0\0\0\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01a!\x7FV[\x80QQ\x90\x91P_\x90[\x80\x82\x10\x15a\x06TW_\x83_\x01Q\x83\x81Q\x81\x10a\x05\xCAWa\x05\xCAaW\x91V[` \x02` \x01\x01Q\x90Pa\x05\xE4a\x05\xDE\x82\x90V[\x86a!\xF3V[\x15a\x06HWa\x06\"\x88a\x06\x1C\x8B\x87_\x01Q\x87\x81Q\x81\x10a\x06\x06Wa\x06\x06aW\x91V[` \x02` \x01\x01Qa\"\"\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90a\"LV[\x84Q\x80Q\x85\x90\x81\x10a\x066Wa\x066aW\x91V[` \x02` \x01\x01\x81\x81RPPPa\x06TV[P`\x01\x90\x91\x01\x90a\x05\xACV[\x80\x82\x03a\x06pWa\x06pa\x06i\x85\x8A\x8Aa\"gV[\x84\x90a\"\xB2V[a\x06y\x83a#-V[`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16h\x01\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U`@\x80Q\x80\x82\x01\x82Rb\xFF\xFF\xFF\x80\x89\x16\x82R\x87\x81\x16` \x80\x84\x01\x82\x81R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x8A\x16_\x90\x81R`\x02\x90\x92R\x94\x90 \x92Q\x83T\x94Q\x83\x16c\x01\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\x90\x95\x16\x92\x16\x91\x90\x91\x17\x92\x90\x92\x17\x90Ua\x07Q\x90a#\x9CV[a\x07_\x86b\xFF\xFF\xFF\x16a#\x9CV[PPPPPPPPPPV[a\x07\x80`\x03Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16C\x14\x90V[\x15a\x07\xB7W`@Q\x7F\xD8\xA6\xB8\x9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x01` R`@\x90 T`\xFF\x16a\x08\x08W`@Q\x7F\\\xD2kh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F?%\xE5Qtd\x14\xFF\x93\xF0v\xA7\xDD\x83\x82\x8F\xF575\xB3\x93f\xC7@\x15c~\0O\xCB\x02#_\x90\x81RC` R`@\x81 \x90a\x08?\x82a#\xDBV[\x90Pa\x08M\x85\x82\x86\x86a$\xF1V[a\x08\x83W`@Q\x7F\x8B\xAAW\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08\x8CCa%\xDDV[`\x03\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UPPPPPV[_a\x08\xD5a%\xFAV[_a\x08\xDF\x86a&\\V[\x90P_\x80a\t?\x83\x8Aa\x08\xF5` \x8B\x01\x8BaW\xCCV[a\t\x05`@\x8C\x01` \x8D\x01aW\xCCV[`\x06\x90\x81R`\x03\x91\x90\x91R_\x91\x82R``\x8B\x015`&\x90\x81R`:`\x0C \x90\x83\x90R\x92\x82R` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 \x91V[\x90\x92P\x90P_a\t\x84a\t{`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a&pV[`\xA0\x1C`\x02\x0B\x90V[\x90P_a\t\xBD\x82a\t\x98` \x8C\x01\x8CaW\xCCV[a\t\xA8`@\x8D\x01` \x8E\x01aW\xCCV[_\x89\x81R`\x07` R`@\x90 \x92\x91\x90a&\x90V[\x90P_a\t\xF4`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x87\x86a'\x12V[\x90P_a\n\x18\x86_\x01T\x84\x03\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a'mV[\x90P\x80\x15a\x0B\xB6W`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\xA5\x84\x11\x94a\n\\` \x8F\x01\x8FaT\xC3V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\n\xB2W_\x80\xFD[PZ\xF1\x15\x80\x15a\n\xC4W=_\x80>=_\xFD[PPPPa\x0B\x0F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x8E_\x01` \x81\x01\x90a\n\xFF\x91\x90aT\xC3V[`\x01`\x01`\xA0\x1B\x03\x16\x91\x90a'\xE2V[`@Q\x7F=\xD4Z\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x8E\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c=\xD4Z\xDB\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0B\x8DW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xB1\x91\x90aW\xE7V[P\x82\x86U[P\x7F!\xD0\xEEp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9C\x9BPPPPPPPPPPPPV[_a\x0B\xF1a%\xFAV[_a\x0B\xFB\x86a&\\V[_\x81\x81R`\x07` R`@\x81 \x91\x92P\x80a\x0CBa\t{`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a&pV[\x90P_\x83a\x0CS` \x8B\x01\x8BaW\xCCV[b\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a\x0CkWa\x0CkaW\x91V[\x01T\x90P_\x84a\x0C\x81`@\x8C\x01` \x8D\x01aW\xCCV[b\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a\x0C\x99Wa\x0C\x99aW\x91V[\x01T\x90Pa\x0C\xAA` \x8B\x01\x8BaW\xCCV[`\x02\x0B\x83`\x02\x0B\x12\x15a\x0C\xC1W\x80\x82\x03\x93Pa\x0E\x1CV[`\x02\x83\x90\x0Ba\x0C\xD6`@\x8C\x01` \x8D\x01aW\xCCV[`\x02\x0B\x13a\r\xC4Wa\r4\x86a\x0C\xEF` \x8D\x01\x8DaW\xCCV[\x8D``\x01` \x81\x01\x90a\r\x02\x91\x90aW\xCCV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92\x91\x90a(5V[a\rlWc\x01\0\0\0\x85\x01T\x91P\x81\x85a\rQ` \x8D\x01\x8DaW\xCCV[b\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a\riWa\riaW\x91V[\x01U[a\r\x80\x86a\x0C\xEF`@\x8D\x01` \x8E\x01aW\xCCV[a\r\xBAWPc\x01\0\0\0\x84\x01T\x80\x85a\r\x9F`@\x8D\x01` \x8E\x01aW\xCCV[b\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a\r\xB7Wa\r\xB7aW\x91V[\x01U[\x81\x81\x03\x93Pa\x0E\x1CV[a\r\xD5\x86a\x0C\xEF` \x8D\x01\x8DaW\xCCV[a\x0E\rWc\x01\0\0\0\x85\x01T\x91P\x81\x85a\r\xF2` \x8D\x01\x8DaW\xCCV[b\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a\x0E\nWa\x0E\naW\x91V[\x01U[\x80\x82\x86c\x01\0\0\0\x01T\x03\x03\x93P[P_\x91P\x81\x90Pa\x0E\x7F\x85\x8Ca\x0E5` \x8D\x01\x8DaW\xCCV[a\x0EE`@\x8E\x01` \x8F\x01aW\xCCV[`\x06\x90\x81R`\x03\x91\x90\x91R_\x91\x82R``\x8D\x015`&\x90\x81R`:`\x0C \x90\x83\x90R\x92\x82R` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 \x91V[\x90\x92P\x90P_a\x0E\xB9`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x87\x84a'\x12V[\x90P`@\x8A\x015_a\x0E\xCB\x82\x84aX+V[\x90P\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x03a\x0E\xEEW\x85\x85Ua\x0F)V[_a\x0F#\x86_\x01T\x88\x03\x85o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a(\x8BV[\x87\x03\x86UP[P\x7F%\x99\x82\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9D\x9CPPPPPPPPPPPPPV[a\x0Fda!:V[`@Q\x7F\x87t\x15\xD2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x87t\x15\xD2\x90a\x0F\xCD\x90\x86\x90\x86\x90\x86\x90`\x04\x01aXSV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0F\xE4W_\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\xF6W=_\x80>=_\xFD[PPPPPPPV[a\x10\x14`\x01`\x01`\xA0\x1B\x03\x83\x1630\x84a)8V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 \x80T\x83\x92\x90a\x10F\x90\x84\x90aXuV[\x90\x91UPPPPV[a\x10Wa!:V[`\x03T`\x01`\x01`\xA0\x1B\x03h\x01\0\0\0\0\0\0\0\0\x90\x91\x04\x81\x16\x90\x84\x16\x81\x14a\x10\xACW`@Q\x7F\xF2\x1F\xD9\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03T_\x90a\x10\xD0\x90h\x01\0\0\0\0\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x82a!\x7FV[\x90P_[\x83\x81\x10\x15a\x12IW6\x85\x85\x83\x81\x81\x10a\x10\xEFWa\x10\xEFaW\x91V[\x90P`\xA0\x02\x01\x90Pa\x11)\x81`@\x01` \x81\x01\x90a\x11\r\x91\x90aX\x88V[a\x06\x1Ca\x11 `@\x85\x01` \x86\x01aX\xA1V[\x86\x90\x855a)\x9AV[\x83Q\x80Q\x835\x90\x81\x10a\x11>Wa\x11>aW\x91V[` \x90\x81\x02\x91\x90\x91\x01\x01Ra\x11fa\x11\\`\x80\x83\x01``\x84\x01aX\x88V[b\xFF\xFF\xFF\x16a#\x9CV[a\x11ya\x11\\`\xA0\x83\x01`\x80\x84\x01aX\x88V[`@\x80Q\x80\x82\x01\x90\x91R\x80a\x11\x94`\x80\x84\x01``\x85\x01aX\x88V[b\xFF\xFF\xFF\x16\x81R` \x01a\x11\xAE`\xA0\x84\x01`\x80\x85\x01aX\x88V[b\xFF\xFF\xFF\x16\x90R`\x02_a\x11\xC8`@\x85\x01` \x86\x01aX\xA1V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01_ \x82Q\x81T\x93\x90\x92\x01Qb\xFF\xFF\xFF\x90\x81\x16c\x01\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\x90\x94\x16\x92\x16\x91\x90\x91\x17\x91\x90\x91\x17\x90UP`\x01\x01a\x10\xD4V[Pa\x12S\x81a#-V[`\x03`\x08a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPPPPPPV[_\x80_a\x12\x8Ba%\xFAV[a\x12\xA0`\x03Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16C\x14\x90V[a\x13WW`\x14\x84\x10\x15a\x13\x19W_\x84\x90\x03a\x12\xE7W`@Q\x7F\x1E\x81\x07\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x7FI&\x89\x8B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x13'`\x14\x82\x87\x89aX\xBAV[a\x130\x91aX\xE1V[``\x1C\x90P6_a\x13D\x87`\x14\x81\x8BaX\xBAV[\x91P\x91Pa\x13S\x83\x83\x83a\x07kV[PPP[_a\x13\x92a\x13na\x13k` \x8B\x01\x8BaT\xC3V[\x90V[a\x13\x81a\x13k`@\x8C\x01` \x8D\x01aT\xC3V[_\x91\x82R` R`@\x90 `(\x1B\x90V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x81\x16_\x90\x81R`\x02` R`@\x81 Tb\xFF\xFF\xFF\x16b@\0\0\x17\x93P\x90\x91Pa\x13\xDB\x89a&\\V[\x90Pa\x14!a\x14\x16a\t{`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84a&pV[`\x08\x90`\x02\x0Ba*'V[P\x7FW^$\xB4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x93P_\x92PP\x95P\x95P\x95\x92PPPV[_`\x01\x83\x01\x835\x82\x1A\x80a\x15QW`@\x80Q\x7F\xD5\x05\xAC\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x88\x16`\x04\x82\x01R3`$\x82\x01R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`D\x82\x01R`\x14\x84\x015`\xD8\x1C`d\x82\x01\x81\x90R`\x19\x85\x015_\x90\x81\x1A`\x84\x84\x01\x81\x90R`\x1A\x87\x015`\xA4\x85\x01\x81\x90R`:\x88\x015`\xC4\x86\x01\x81\x90R\x95Q`Z\x89\x01\x985``\x1C\x96\x94\x95\x92\x94\x91\x93\x91\x92\x87\x92c\xD5\x05\xAC\xCF\x92`\xE4\x80\x84\x01\x93\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a\x151W_\x80\xFD[PZ\xF1\x15\x80\x15a\x15CW=_\x80>=_\xFD[PPPPPPPPPa\x17\x14V[`\x01\x81`\xFF\x16\x03a\x16,W`@Q\x7F\xD5\x05\xAC\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x04\x82\x01R3`$\x82\x81\x01\x91\x90\x91R`\x14\x84\x015`\x80\x1C`D\x83\x01\x81\x90R\x90\x84\x015`\xD8\x1C`d\x83\x01\x81\x90R`)\x85\x015_\x1A`\x84\x84\x01\x81\x90R`*\x86\x015`\xA4\x85\x01\x81\x90R`J\x87\x015`\xC4\x86\x01\x81\x90R`j\x88\x01\x975``\x1C\x95\x86\x90c\xD5\x05\xAC\xCF\x90`\xE4\x01[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x16\x0BW_\x80\xFD[PZ\xF1\x15\x80\x15a\x16\x1DW=_\x80>=_\xFD[PPPPPPPPPPa\x17\x14V[`\x02\x81`\xFF\x16\x03a\x16\xD8W`@Q\x7F\x8F\xCB\xAF\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x04\x82\x01R3`$\x82\x01R`\x14\x83\x015`\xE0\x1C`D\x82\x01\x81\x90R`\x18\x84\x015`\xD8\x1C`d\x83\x01\x81\x90R`\x01`\x84\x84\x01R`\x1D\x85\x015_\x1A`\xA4\x84\x01\x81\x90R`\x1E\x86\x015`\xC4\x85\x01\x81\x90R`>\x87\x015`\xE4\x86\x01\x81\x90R`^\x88\x01\x975``\x1C\x95\x86\x90c\x8F\xCB\xAF\x0C\x90a\x01\x04\x01a\x15\xF4V[`@Q\x7Fo\x1D\x15\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\xFF\x82\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[a\x17\x1F\x82\x86\x86a*.V[c$\xA2\xE4K\x92PPP[\x93\x92PPPV[_\x81T_R` _\xF3[a\x17O`\x01`\x01`\xA0\x1B\x03\x84\x1630\x84a)8V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a\x17\x85\x90\x84\x90aXuV[\x90\x91UPPPPPV[\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x80_\x80\x80\x83a\x18%`@\x80Q\x80\x82\x01\x82R`\x08\x81R\x7FAngstrom\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R`\x02\x83R\x7Fv1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x83\x01R\x91V[\x97\x98\x90\x97\x96PF\x95P0\x94P\x91\x92P\x90V[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11\x15a\x18UW\x91\x92\x91[_\x84\x81R` \x84\x90R`@\x81 `(\x1B`\x03T\x90\x91P_\x90a\x18\x8D\x90h\x01\0\0\0\0\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x83\x86a*KV[P\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cbv\xCB\xBE`@Q\x80`\xA0\x01`@R\x80a\x18\xD1\x8A\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x88`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82Rb\x80\0\0` \x80\x84\x01\x91\x90\x91R`\x02\x87\x81\x0B`@\x80\x86\x01\x91\x90\x91R0``\x95\x86\x01R\x80Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x89\x90\x1B\x16\x81R\x86Q\x85\x16`\x04\x82\x01R\x92\x86\x01Q\x84\x16`$\x84\x01R\x85\x01Qb\xFF\xFF\xFF\x16`D\x83\x01R\x92\x84\x01Q\x90\x92\x0B`d\x83\x01R`\x80\x90\x92\x01Q\x82\x16`\x84\x82\x01R\x90\x86\x16`\xA4\x82\x01R`\xC4\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x19\x9BW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xF6\x91\x90aYGV[``a\x19\xC9a%\xFAV[\x82_a\x19\xD4\x82a*\xB5V[`\x03T\x91\x93P\x91P_\x90a\x1A\0\x90\x84\x90\x84\x90h\x01\0\0\0\0\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16a+lV[\x90\x93P\x90Pa\x1A\x0E\x82a,\xE1V[a\x1A\x18\x83\x82a-\x0CV[\x92Pa\x1A$\x83\x82a-\x98V[\x92Pa\x1A0\x83\x82a.KV[\x92Pa\x1A=\x83\x87\x87a*.V[a\x1AF\x82a.\xEAV[` _R_` R`@_\xF3[a\x1A[a!:V[_\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[_\x80a\x1A\x9Ea%\xFAV[_\x80a\x1A\xC6a\x1A\xB3a\x13k` \x8C\x01\x8CaT\xC3V[a\x13\x81a\x13k`@\x8D\x01` \x8E\x01aT\xC3V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x81\x16_\x90\x81R`\x02` \x90\x81R`@\x82 T\x92\x93Pc\x01\0\0\0\x90\x92\x04b\xFF\xFF\xFF\x16\x91\x8A\x01\x805\x82\x13\x91\x90a\x1B\x1C\x90\x8CaYbV[\x15\x15\x82\x15\x15\x03a\x1B5Wa\x1B0\x8A`\x0F\x0B\x90V[a\x1B?V[a\x1B?\x8A`\x80\x1D\x90V[\x90P_\x80\x82`\x0F\x0B\x12a\x1BRW\x81a\x1B[V[a\x1B[\x82aY\x81V[\x90P\x82a\x1B\x99W\x80a\x1Bp\x85b\x0FB@aY\xBDV[`\x02\x0Ba\x1B\x80b\x0FB@\x84aY\xFEV[a\x1B\x8A\x91\x90aZJV[a\x1B\x94\x91\x90aZ\xBDV[a\x1B\xB5V[b\x0FB@a\x1B\xAB`\x02\x86\x90\x0B\x83aY\xFEV[a\x1B\xB5\x91\x90aZJV[\x95PP`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90Pc\x15n)\xF6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1CZa\x1C\x1A` \x8F\x01\x8FaYbV[\x15\x15\x85\x15\x15\x03a\x1C<W\x8E` \x01` \x81\x01\x90a\x1C7\x91\x90aT\xC3V[a\x1CNV[\x8E_\x01` \x81\x01\x90a\x1CN\x91\x90aT\xC3V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`D\x82\x01R`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1C\xCDW_\x80\xFD[PZ\xF1\x15\x80\x15a\x1C\xDFW=_\x80>=_\xFD[PPPPPPP_a\x1C\xF0\x89a&\\V[\x90P_a\x1D)a\t{`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84a&pV[\x90Pa\x1D~\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x08\\\x84\x8E``\x01` \x81\x01\x90a\x1Dg\x91\x90aW\xCCV[_\x88\x81R`\x07` R`@\x90 \x94\x93\x92\x91\x90a1\x12V[P\x7F\xB4{/\xB1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x93P\x90\x91PP[\x96P\x96\x94PPPPPV[a\x1D\xBAa!:V[_[\x81\x81\x10\x15a\x05\x06W_\x83\x83\x83\x81\x81\x10a\x1D\xD7Wa\x1D\xD7aW\x91V[\x90P` \x02\x01` \x81\x01\x90a\x1D\xEC\x91\x90aT\xC3V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x01` \x81\x90R`@\x90\x91 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81\x16`\xFF\x90\x91\x16\x15\x17\x90U\x91\x90\x91\x01\x90Pa\x1D\xBCV[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 \x80T\x83\x92\x90a\x1Ep\x90\x84\x90a[\x0BV[\x90\x91UPa\x05\x06\x90P`\x01`\x01`\xA0\x1B\x03\x84\x16\x83\x83a'\xE2V[a\x1E\x92a!:V[a\x1E\xA6`\x01`\x01`\xA0\x1B\x03\x83\x163\x83a'\xE2V[PPV[a\x1E\xB2a!:V[`\x03T`\x01`\x01`\xA0\x1B\x03h\x01\0\0\0\0\0\0\0\0\x90\x91\x04\x81\x16\x90\x83\x16\x81\x14a\x1F\x07W`@Q\x7F\xF2\x1F\xD9\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03T_\x90a\x1F*\x90h\x01\0\0\0\0\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16a1\x9BV[\x90Pa\x1F7\x81\x86\x85a1\xC2V[a\x1F@\x81a#-V[`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16h\x01\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UPPPP\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x16_\x90\x81R`\x02` R`@\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\x16\x90UV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 \x80T\x83\x92\x90a \x12\x90\x84\x90a[\x0BV[\x90\x91UPa\x1E\xA6\x90P`\x01`\x01`\xA0\x1B\x03\x83\x163\x83a'\xE2V[`\x03TCg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x03a sW`@Q\x7F\xD8\xA6\xB8\x9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3_\x90\x81R`\x01` R`@\x90 T`\xFF\x16a \xBBW`@Q\x7F\\\xD2kh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a \xC4Ca%\xDDV[`\x03\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x80`\x0CRc\xDA\xA0P\xE9`\x04R\x81_R`\x1F`\x0C `\x01`\xFF\x83\x16\x1B\x80\x82T\x18\x81\x81\x16a!2Wc\x8C\xB8\x88r_R`\x04`\x1C\xFD[\x90\x91UPPPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a!}W`@Q\x7F#\x01\x9Eg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[`@\x80Q\x80\x82\x01\x90\x91R``\x81R_` \x82\x01R_a!\xA6\x84`\x01`\x01`\xA0\x1B\x03\x16a2\xAEV[\x90P_a!\xB3\x84\x83aXuV[` \x84\x81\x01\x82\x90R`@\x80Q\x83\x83\x02\x81\x01\x83\x01\x90\x91R_\x81R\x80\x86R\x91\x92P\x83\x81\x02\x90\x81\x90`\x01\x90\x84\x01\x89<Pa!\xEA\x84\x84a#\x01V[PPP\x92\x91PPV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x82\x81\x16\x90\x82\x16\x14[\x92\x91PPV[_a\",\x82a2\xC4V[Pe\xFF\xFF\xFF\0\0\0a\xFF\xFF\x91\x82\x16`\x18\x84\x81\x1C\x93\x90\x93\x16\x18\x90\x91\x1B\x16\x18\x90V[_a\"V\x82a3\x13V[Pb\xFF\xFF\xFF\x80\x83\x16\x91\x90\x91\x18\x16\x18\x90V[_a\"q\x83a2\xC4V[a\"z\x82a3\x13V[a\"\xAA\x82a\x06\x1C\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x87\x16\x86a\"\"V[\x94\x93PPPPV[\x81QQ` \x83\x01Q\x81\x03a\"\xF2W`@Q\x7F\\\xEFX:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a#\x06\x83a#\x01\x83`\x01aXuV[\x90QRV[\x81\x83_\x01Q\x82\x81Q\x81\x10a#\x1CWa#\x1CaW\x91V[` \x02` \x01\x01\x81\x81RPPPPPV[\x80Q\x80Qk`\x0B8\x03\x80`\x0B_9_\xF3\0\x82R_\x91\x90`\x0C` \x82\x02\x01`\x14\x83\x01\x84\xF0\x81\x83R\x92P`\x01`\x01`\xA0\x1B\x03\x83\x16a#\x95W`@Q\x7FVp%\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x91\x90PV[b\x0FB@b\xFF\xFF\xFF\x82\x16\x11\x15a\x05\x15Wa\x05\x15\x7F\x14\0!\x13\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0b\xFF\xFF\xFF\x83\x16a3UV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14\x16a$\xCEWP`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91RF``\x82\x01R0`\x80\x82\x01R`\xA0\x90 [g\x19\x01\0\0\0\0\0\0_R\x80`\x1AR\x81`:R`B`\x18 \x90P_`:R\x91\x90PV[_`\x01`\x01`\xA0\x1B\x03\x85\x16\x15a\"\xAAW`@Q\x85;a%\x9AW\x82`@\x81\x14a%!W`A\x81\x14a%aWPa%\xD4V[` \x85\x81\x015`\xFF\x81\x90\x1C`\x1B\x01\x90\x91R\x855`@R\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``Ra%rV[`@\x85\x015_\x1A` R`@\x85`@7[P\x84_R` `\x01`\x80_`\x01Z\xFAQ\x80\x87\x18``\x1B=\x11\x92PP_``R\x80`@Ra%\xD4V[c\x16&\xBA~`\xE0\x1B\x80\x82R\x85`\x04\x83\x01R`$\x82\x01`@\x81R\x84`D\x84\x01R\x84\x86`d\x85\x017` \x81`d\x87\x01\x85\x8BZ\xFA\x90Q\x90\x91\x14\x16\x91P[P\x94\x93PPPPV[_h\x01\0\0\0\0\0\0\0\0\x82\x10a%\xF6Wa%\xF6a3jV[P\x90V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a!}W`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q_\x90`\xA0\x83\x827`\xA0\x90 \x92\x91PPV[_\x81\x81R`\x06` R`@\x81 a\"\xAA`\x01`\x01`\xA0\x1B\x03\x85\x16\x82a3wV[_\x80\x85b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a&\xACWa&\xACaW\x91V[\x01T\x90P_\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a&\xCBWa&\xCBaW\x91V[\x01T\x90P\x84`\x02\x0B\x86`\x02\x0B\x12\x15a&\xE6W\x90\x03\x90Pa\"\xAAV[\x85`\x02\x0B\x84`\x02\x0B\x13a&\xFBW\x03\x90Pa\"\xAAV[c\x01\0\0\0\x87\x01T\x91\x90\x91\x03\x03\x90P\x94\x93PPPPV[_`\x06` R\x82_R`\x06`@_ \x01` R\x81_R`@_ ` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x87Z\xFAa'QWcS\\\xF9K_R`\x04`\x1C\xFD[PP_Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x91PPV[\x81\x81\x02\x81\x83\x82\x04\x14\x83\x15\x17a'\xD9W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x84\t\x81\x81\x10\x82\x01\x90\x03p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x10a'\xCCWc\xC5j\x01Y_R`\x04`\x1C\xFD[`\x80\x91\x82\x1C\x91\x1B\x01a\"\x1CV[`\x80\x1C\x92\x91PPV[\x81`\x14R\x80`4Ro\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0_R` _`D`\x10_\x87Z\xF1\x80`\x01_Q\x14\x16a(+W\x80=\x85;\x15\x17\x10a(+Wc\x90\xB8\xEC\x18_R`\x04`\x1C\xFD[P_`4RPPPV[_\x80\x80a(S\x84\x86\x07\x82\x13\x85\x87\x05\x03[`\x08\x81\x90\x1D\x91`\xFF\x90\x91\x16\x90V[\x90\x92P\x90Pa(\x80\x81a(p`\x01`\x01`\xA0\x1B\x03\x8A\x16\x89\x86a3\xA7V[\x90`\x01`\xFF\x91\x90\x91\x16\x1B\x16\x15\x15\x90V[\x97\x96PPPPPPPV[\x82\x82\x02\x81\x83\x85\x83\x04\x14\x85\x15\x17\x02a)1W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x85\t\x81\x81\x10\x82\x01\x90\x03\x82\x84\x86\t\x83_\x03\x84\x16\x82\x85\x11a(\xE4Wc\xAEG\xF7\x02_R`\x04`\x1C\xFD[\x93\x84\x90\x04\x93\x83\x82\x11\x90\x92\x03_\x83\x90\x03\x83\x90\x04`\x01\x01\x02\x92\x03\x04\x17`\x02`\x03\x83\x02\x81\x18\x80\x84\x02\x82\x03\x02\x80\x84\x02\x82\x03\x02\x80\x84\x02\x82\x03\x02\x80\x84\x02\x82\x03\x02\x80\x84\x02\x82\x03\x02\x80\x84\x02\x90\x91\x03\x02\x02a\x17)V[\x04\x92\x91PPV[`@Q\x81``R\x82`@R\x83``\x1B`,Ro#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0`\x0CR` _`d`\x1C_\x89Z\xF1\x80`\x01_Q\x14\x16a)\x8CW\x80=\x87;\x15\x17\x10a)\x8CWcy9\xF4$_R`\x04`\x1C\xFD[P_``R`@RPPPPV[_\x83_\x01Q\x82\x81Q\x81\x10a)\xB0Wa)\xB0aW\x91V[` \x02` \x01\x01Q\x90Pa)\xF0a)\xC4\x82\x90V[\x84\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x90\x81\x16\x91\x16\x14\x15\x90V[\x15a\x17)W`@Q\x7F#\xF6\x9D\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x82]PPV[\x80\x82\x01\x80\x84\x14a*EWc\x01\x84/\x8C_R`\x04`\x1C\xFD[PPPPV[_\x80\x80a*b`\x01`\x01`\xA0\x1B\x03\x87\x16\x86\x86a3\xDEV[\x90P\x80a*\x9BW`@Q\x7F/e\x9ED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\xFF\xFF`\x18\x82\x90\x1C\x16\x96b\xFF\xFF\xFF\x90\x91\x16\x95P\x93PPPPV[`\x03\x81\x81\x01\x91_\x91\x82\x91\x805`\xE8\x1C\x01\x01\x81`Da*\xD3\x86\x84a[\x0BV[a*\xDD\x91\x90a[\x1EV[\x90P\x80` \x86\x90\x1B\x17\x92P_\x80[\x82\x81\x10\x15a+`W_a+\t` \x87\x90\x1C`D\x84\x02\x01[5``\x1C\x90V[\x90P\x82`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x11a+VW`@Q\x7F\x80\xF1\x1A\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91P`\x01\x01a*\xEBV[P\x82\x94PPPP\x91P\x91V[`\x03\x83\x81\x01\x93_\x91\x82\x91\x82\x91\x82\x91\x805`\xE8\x1C\x01\x01\x81`&a+\x8E\x8A\x84a[\x0BV[a+\x98\x91\x90a[\x1EV[\x90P`@Q\x93P\x80`\xC0\x02\x84\x01\x92P\x82`@R\x80\x84` \x1B\x17\x94PP_[\x82\x84\x10\x15a,\xD4W`\x04\x89\x01\x985`\xE0\x81\x90\x1C\x90_\x90a+\xDE\x90a+\x02\x90\x8C\x90`\xF0\x1Ca4\x1CV[\x90P_a+\xF2a+\x02\x8Ca\xFF\xFF\x86\x16a4\x1CV[\x90P\x83c\xFF\xFF\xFF\xFF\x16\x83c\xFF\xFF\xFF\xFF\x16\x11\x15\x80a,!WP\x80`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x10\x15[\x15a,XW`@Q\x7F\xF3_\x93\x99\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x90\x86R` \x86\x01R`@\x85 `\x02\x8B\x01\x9A\x91\x92P`(\x1B\x905`\xF0\x1C_\x80a,\x8A`\x01`\x01`\xA0\x1B\x03\x8C\x16\x85\x85a*KV[`@\x8A\x01\x91\x90\x91R``\x89\x01RPPP` \x8A\x01\x995\x90P_v\np\xC3\xC4\nd\xE6\xC5\x19\x99\t\x0Be\xF6}\x92@\0\0\0\0\0\0\x82\x90\x04`\x80\x87\x01RP`\xA0\x85\x01R`\xC0\x90\x93\x01\x92a+\xB6V[P\x93PPP\x93P\x93\x91PPV[c\xFF\xFF\xFF\xFF\x81\x16_[\x81\x81\x10\x15a\x05\x06Wa-\x04` \x84\x90\x1C`D\x83\x02\x01a4zV[`\x01\x01a,\xEAV[`@\x80Qa\x01`\x81\x01\x82R_` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01@\x81\x01\x82\x90Rc\xF3\xCD\x91L\x81Rb\x80\0\0``\x82\x01R0`\xA0\x82\x01Ra\x01 \x80\x82\x01R`\x03\x84\x81\x01\x94\x805`\xE8\x1C\x01\x01\x90[\x81\x85\x14a-\x8FWa-\x88\x85\x82\x86a5>V[\x94Pa-vV[P\x92\x93\x92PPPV[`\x03\x82\x81\x01\x92_\x91\x815`\xE8\x1C\x90\x91\x01\x01\x81a-\xB2a7GV[`@\x80Qa\x01 \x81\x01\x82R_` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x91\x90\x91R\x7F\n\xF1\x9DTy\xE9\x0F%\x84\\\xEAm\xB8\x9ARK\xB4\xE8\xDA:i\x82\x13\xEF\xB1\xB8^\x10\xA5\xE8\xBE\x9C\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFC\x16a\x01\0\x82\x01R\x90\x91P[\x82\x86\x14a.AWa.:\x86\x82\x84\x88a7\x91V[\x95Pa.'V[P\x93\x94\x93PPPPV[_\x80a.Ua7GV[`@\x80Qa\x01\xA0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x82\x90Ra\x01@\x81\x01\x82\x90Ra\x01`\x81\x01\x82\x90Ra\x01\x80\x81\x01\x91\x90\x91R`\x03\x86\x81\x01\x96\x92\x93P\x90\x91\x805`\xE8\x1C\x01\x01[\x80\x86\x14a.AWa.\xE3\x86\x83\x85\x88a9uV[\x95Pa.\xD0V[`@\x80Qc\xFF\xFF\xFF\xFF\x83\x16`$\x81\x02\x82\x01\x90\x92R\x80_[\x83\x81\x10\x15a0\xFFW`D\x81\x02` \x86\x90\x1C\x01\x805``\x1C`\x14\x82\x015`\x80\x90\x81\x1C\x90`4\x84\x015\x90\x1C_a/B\x84a/9\x84\x86aXuV[`\x04\x91\x90a;IV[\x90P\x80\x15a/\x87W`@Q\x7F\xCCg\xAFS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x17\x0BV[\x81\x15a0\xE1W`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\xA5\x84\x11\x94\x85`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a0\x13W_\x80\xFD[PZ\xF1\x15\x80\x15a0%W=_\x80>=_\xFD[Pa0^\x92PPP`\x01`\x01`\xA0\x1B\x03\x85\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84a'\xE2V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x11\xDA`\xB4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a0\xBBW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0\xDF\x91\x90aW\xE7V[P[a0\xEB\x85\x88a;\x7FV[PPP`$\x93\x90\x93\x01\x92PP`\x01\x01a/\x01V[P`$\x83\x02\x82 _R` _\xA0PPPPV[\x82`\x02\x0B\x82`\x02\x0B\x13\x15a1VW\x82`\x02\x0Ba1:\x82\x84`\x02\x0Ba;\x88\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x02\x0B\x13\x15a1QWa1Q\x86\x85\x87\x86\x86\x86a;\x99V[a1\x93V[\x82`\x02\x0B\x82`\x02\x0B\x12\x15a1\x93W_`\x02\x84\x90\x0B\x82\x81\x07\x91\x90\x91\x12\x90\x82\x90\x05\x03\x81\x02`\x02\x0B\x82`\x02\x0B\x12\x15a1\x93Wa1\x93\x86\x85\x87\x86\x86\x86a<\x18V[PPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R``\x81R_` \x82\x01Ra\"\x1C`\x01`\x01`\xA0\x1B\x03\x83\x16_a!\x7FV[a2\x14a1\xE8\x84_\x01Q\x83\x81Q\x81\x10a1\xDDWa1\xDDaW\x91V[` \x02` \x01\x01Q\x90V[\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x90\x81\x16\x91\x16\x14\x15\x90V[\x15a2KW`@Q\x7F#\xF6\x9D\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82QQ_\x90a2\\\x90`\x01\x90a[\x0BV[\x90P\x80\x82\x10\x15a2\xA4W\x83Q\x80Q\x82\x90\x81\x10a2zWa2zaW\x91V[` \x02` \x01\x01Q\x84_\x01Q\x83\x81Q\x81\x10a2\x97Wa2\x97aW\x91V[` \x02` \x01\x01\x81\x81RPP[a*E\x84\x82\x90QRV[_a\"\x1C` `\x01`\x01`\xA0\x1B\x03\x84\x16;a[\x1EV[`\x01a\xFF\xFF\x82\x16\x10\x80a2\xDCWPa\x7F\xFFa\xFF\xFF\x82\x16\x11[\x15a\x05\x15W`@Q\x7F'\x08\x15\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[b\x03\r@b\xFF\xFF\xFF\x82\x16\x11\x15a\x05\x15W`@Q\x7Fv\xA3\xF9]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81_R`\x01`\x01`\xA0\x1B\x03\x81\x16`\x04R`$_\xFD[c5'\x8D\x12_R`\x04`\x1C\xFD[_\x81` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x86Z\xFAa3\x9EWcS\\\xF9K_R`\x04`\x1C\xFD[PP_Q\x91\x90PV[_\x82\x81R`\x06` \x90\x81R`@\x80\x83 \x84\x84R`\x05\x01\x90\x91R\x81 a3\xD5`\x01`\x01`\xA0\x1B\x03\x86\x16\x82a3wV[\x95\x94PPPPPV[_` \x82` \x02`\x01\x01_\x86<PP_Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x81\x16\x91\x90\x91\x14\x02\x91\x90PV[_\x81c\xFF\xFF\xFF\xFF\x84\x16\x11a4kW`@Q\x7F\xFF\xC3\x1Eq\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01a\x17\x0BV[` \x83\x90\x1C`D\x83\x02\x01a\x17)V[`$\x81\x015`\x80\x1C\x80\x15a\x1E\xA6W`@\x80Q\x7F\x0B\r\x9C\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x835``\x1C`\x04\x82\x01\x81\x90R0`$\x83\x01R`D\x82\x01\x84\x90R\x91Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91c\x0B\r\x9C\t\x91`d\x80\x83\x01\x92_\x92\x91\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a5\x19W_\x80\xFD[PZ\xF1\x15\x80\x15a5+W=_\x80>=_\xFD[Pa\x05\x06\x92P`\x04\x91P\x83\x90P\x84a<\xA3V[`\x01\x83\x81\x01\x93_\x91\x905\x82\x1A\x90a5Z\x90\x85\x90\x83\x16\x15\x15a<\xCFV[`\x02\x85\x01\x945`\xF0\x1Ca5\x81a5p\x85\x83a=\x13V[\x80Q` \x82\x01Q`@\x90\x92\x01Q\x90\x92V[`\x02\x0B`\x80\x88\x01R`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`@\x88\x01R\x16` \x86\x01\x90\x81R`\xA0\x90 _`\x10\x88\x01\x885`\x80\x1C\x90\x98Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P_\x81\x15a6\xC1W_a6\x04a\t{`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a&pV[\x90Pa6\x0F\x83a=sV[`\xE0\x8A\x01Ra6>\x89\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a=\xD4V[a6ta\t{`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a&pV[`\x80\x8A\x01Q_\x86\x81R`\x07` R`@\x90 \x91\x93Pa6\xBB\x91\x90\x86\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x85\x90\x87\x90a1\x12V[Pa6\xFAV[a6\xF7a\t{`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x85a&pV[\x90P[_a7!`\x02\x87\x16\x15\x15_\x86\x81R`\x07` R`@\x90 `\x80\x8C\x01Q\x8D\x91\x90\x88\x90\x87a=\xF1V[` \x8B\x01Q\x91\x9BP\x91Pa78\x90`\x04\x90\x83a;IV[P\x98\x99\x98PPPPPPPPPV[_a7\x8Ca7SaA9V[`@\x80Q`B\x81\x01\x90\x91R\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x02\x81\x01\x91\x90\x91R\x90V[\x90P\x90V[\x835_\x90\x81\x1A`\x01\x81\x81\x16\x15\x15`\x80\x87\x81\x01\x91\x90\x91R\x90\x87\x015\x81\x1C` \x87\x01R`\x11\x87\x015\x81\x1C`@\x87\x01R`!\x87\x015\x81\x1C``\x87\x01\x81\x90R`A\x88\x01\x97`1\x015\x90\x91\x1C\x90\x81\x11\x15a8\x12W`@Q\x7F+\xAElR\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x87\x81\x01\x975`\xF0\x1C\x90a8A\x90\x84\x16\x15\x15a8/\x87\x84a=\x13V[\x90`\x05\x1B` \x81\x18\x82\x01Q\x91\x01Q\x90\x91V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x8A\x01R\x16`\xA0\x88\x01RP`\x04\x82\x16a8gW\x86_a8qV[`\x14\x87\x01\x875``\x1C[`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x88\x01R\x96P_a8\x9Da8\x91\x88a\x01 \x90 \x90V[`\"\x88\x01R`B\x87 \x90V[\x90P_`\x08\x84\x16a8\xB7Wa8\xB2\x89\x83aB.V[a8\xC1V[a8\xC1\x89\x83aB\x98V[\x90\x99P\x90Pa8\xD0\x82\x82aB\xDCV[`\xE0\x88\x01Q\x80\x15\x82\x02\x17`\x02\x85\x16\x15a9\x0FW\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89` \x01\x81\x81Qa9\x07\x91\x90aXuV[\x90RPa97V[\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89`@\x01\x81\x81Qa93\x91\x90a[\x0BV[\x90RP[a9O\x82\x8A`\xA0\x01Q\x8B` \x01Q\x8C`\x80\x01QaC\x04V[a9g\x81\x8A`\xC0\x01Q\x8B`@\x01Q\x8C`\x80\x01QaCtV[P\x97\x98\x97PPPPPPPPV[_\x80a9\x81\x85\x87aC\xD2V[`\x02\x82\x01\x97P\x91P_\x90\x81\x905`\xF0\x1Ca9\xAA`\x08\x85\x16\x15\x15a9\xA4\x88\x84a=\x13V[\x90aD\xB2V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16a\x01\0\x8C\x01R\x91\x16`\xE0\x8A\x01R\x92PPP` \x87\x01\x875`\xA0\x88\x01\x81\x90R\x90\x97P\x81\x10\x15a:\x0FW`@Q\x7F\x8E\x1E\xDF\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82\x16a:\x1EW\x86_a:(V[`\x14\x87\x01\x875``\x1C[`\x01`\x01`\xA0\x1B\x03\x16a\x01 \x88\x01R\x96P_a:H\x88`\x04\x85\x16\x15aD\xF4V[a\x01@\x8A\x01R\x90\x98P\x90Pa:^\x87\x89\x85aE\xD6V[\x97P_\x80a:n\x89\x8B\x87\x87aF\x1EV[\x91\x9BP\x92P\x90P_a:\x8Fa:\x83\x8B\x88aH(V[`\"\x8B\x01R`B\x8A \x90V[\x90P_`\x80\x87\x16a:\xA9Wa:\xA4\x8C\x83aB.V[a:\xB3V[a:\xB3\x8C\x83aB\x98V[\x90\x9CP\x90P`\x10\x87\x16\x15a:\xEAWa:\xD6\x8Ba\x01\x80\x01Qd\xFF\xFF\xFF\xFF\xFF\x16aHHV[a:\xE5\x81\x8Ca\x01`\x01Qa \xFFV[a:\xF4V[a:\xF4\x82\x82aB\xDCV[_\x8Ba\x01 \x01Q\x90P\x80\x82\x82\x15\x02\x17\x90Pa;\x1A\x81\x8Da\x01\0\x01Q\x86\x8F`\xC0\x01QaCtV[a;$\x86\x83aH\x82V[a;8\x82\x8D`\xE0\x01Q\x87\x8F`\xC0\x01QaC\x04V[P\x9A\x9B\x9APPPPPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R` \x84\x90R`@\x81 a;wa;n\x82\\\x85aH\xCAV[\x92P\x81\x83a*'V[P\x93\x92PPPV[`$\x82\x827PPV[_\x81\x83\x07\x12\x91\x81\x90\x05\x91\x90\x91\x03\x02\x90V[c\x01\0\0\0\x86\x01T[_a;\xB8`\x01`\x01`\xA0\x1B\x03\x88\x16\x87\x87\x86aH\xE2V[\x95P\x90P`\x02\x85\x81\x0B\x90\x85\x90\x0B\x12\x15a;\xD1WPa\x0F\xF6V[\x80\x15a<\x12W\x87b\xFF\xFF\xFF\x86\x16c\x01\0\0\0\x81\x10a;\xF1Wa;\xF1aW\x91V[\x01T\x82\x03\x88b\xFF\xFF\xFF\x87\x16c\x01\0\0\0\x81\x10a<\x0FWa<\x0FaW\x91V[\x01U[Pa;\xA2V[_a<.`\x01`\x01`\xA0\x1B\x03\x87\x16\x86\x86\x85aI<V[\x94P\x90P`\x02\x83\x81\x0B\x90\x85\x90\x0B\x13a<FWPa1\x93V[\x80\x15a<\x90W\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a<fWa<faW\x91V[\x01T\x87c\x01\0\0\0\x01T\x03\x87_\x01\x85b\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a<\x8DWa<\x8DaW\x91V[\x01U[\x83a<\x9A\x81a[1V[\x94PPPa<\x18V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R` \x84\x90R`@\x90 a*Ea<\xC8\x82\\\x84aItV[\x82\x90a*'V[\x80\x15\x15`\xC0\x83\x01R\x80a<\xF6Ws\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D%a<\xFDV[d\x01\0\x02v\xA4[`\x01`\x01`\xA0\x1B\x03\x16a\x01\0\x90\x92\x01\x91\x90\x91RPV[_\x81c\xFF\xFF\xFF\xFF\x84\x16\x11a=bW`@Q\x7F\xF6`\x1BP\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01a\x17\x0BV[P`\xC0\x81\x02` \x83\x90\x1C\x01\x92\x91PPV[_\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x11\x15a=\xCEW`@Q\x7F5'\x8D\x12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P_\x03\x90V[_\x80a\x01D`\x1C\x85\x01_\x85Z\xF1\x80a\x05\x06W`@Q=_\x82>=\x81\xFD[_\x80\x87\x15a?#W` \x87\x01\x96\x805`\x80\x90\x81\x1C\x91`\x10\x015\x90\x1C\x81\x15\x80a>)WPo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15[\x15a>KWP\x87\x92Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90Pa\x1D\xA7V[_a>\x7F`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x89aI\x8CV[\x90P\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a>\xDEW`@Q\x7F\xBE\xCB\x19\\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\x80\x84\x90\x1B\x04c\x01\0\0\0\x8A\x01\x80T\x90\x91\x01\x90UP\x88\x93PPo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90Pa\x1D\xA7V[_\x80\x80`\x03\x8A\x01\x8A5`\xE8\x1D\x90\x9AP\x90P_`\x10\x8B\x01\x8B5`\x80\x1C\x90\x9BP\x90P_\x80`\x03\x80\x8E\x01\x90\x8E5`\xE8\x1C\x8F\x01\x01`@\x80Q`\x80\x81\x01\x82R\x8E\x81R`\x02\x8E\x81\x0B` \x83\x01R\x8D\x81\x0B\x92\x82\x01\x83\x90R_``\x83\x01\x81\x90R\x94\x96P\x92\x94P\x8F\x93\x92\x90\x91\x90\x88\x90\x0B\x13\x15a?\xA2Wa?\x9D\x83\x88\x87\x89\x85aI\xB0V[a?\xAFV[a?\xAF\x83\x88\x87\x89\x85aJ\xF1V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x855\x90\x81\x16\x91\x90\x91\x04\x90\x92\x01\x9CP\x9AP`\x10\x90\x92\x01\x96P\x92P`\x80\x1Ca@\t\x81\x84aXuV[\x92Pa@\x15\x86\x86aL3V[\x81Q_\x90a@M\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90aI\x8CV[\x90P\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8Ao\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a@\xC8W`@Q\x7Fd)\xCF\xD2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x8C\x16`\x04\x83\x01R\x82\x16`$\x82\x01R`D\x01a\x17\x0BV[``\x83\x81\x01Q`\x14\x89\x01\x985\x82\x1C\x91\x1C\x81\x14aA\x10W`@Q\x7F\xBE\xCB\x19\\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPc\x01\0\0\0\x90\x92\x01\x80T\x90\x98\x01\x90\x97UP\x90\x96P\x93\x94PPPPP\x96P\x96\x94PPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14\x16a\x13kWP`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91RF``\x82\x01R0`\x80\x82\x01R`\xA0\x90 \x90V[`\x17`\x14\x83\x015`\xE8\x1C\x80\x84\x01\x82\x01\x93_\x92\x815``\x1C\x92\x91\x01\x90aBU\x83\x86\x84\x84aLlV[aB\x8BW`@Q\x7F\x8B\xAAW\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85\x93PPP[\x92P\x92\x90PV[_\x80`@Q\x83\x81R_` \x82\x01R`A\x85`?\x83\x017`A\x85\x01\x94P` `\x01`\x80\x83`\x01Z\xFAQ\x91PP=aB\xD5Wc\x8B\xAAW\x9F_R`\x04`\x1C\xFD[\x92\x93\x91PPV[\x80`\x14R\x81_R`4_ \x80\\\x15aB\xFBWc\x8A.\xF1\x16_R`\x04`\x1C\xFD[`\x01\x81]PPPV[\x81aC\x11`\x04\x85\x83a<\xA3V[\x81\x15aCXW`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x89\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90aCM\x90\x84\x90a[\x0BV[\x90\x91UPaCm\x90PV[aCm`\x01`\x01`\xA0\x1B\x03\x85\x16\x860\x84a)8V[PPPPPV[\x81aC\x81`\x04\x85\x83a;IV[P\x81\x15aC\xBEW`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x89\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90aCM\x90\x84\x90aXuV[aCm`\x01`\x01`\xA0\x1B\x03\x85\x16\x86\x83a'\xE2V[`\x01\x81\x01\x90_\x905\x81\x1A`\x04\x83`<\x86\x017`\x04\x92\x90\x92\x01\x91` \x81\x16\x15aDKW`\x10\x81\x16aD\"W\x7Fn\xE8\x9D\xEEW7\x05\xC0$\xA0\x86\xE1\x9A\x12\x8E\xE0\xA5\xEE\x05G\xE3(:\xDF\xA7/\xBE3jLKlaDDV[\x7Fk\xE5\xF2+\xDC\xD07\xF6\xF3RP\xC3.G\x8F\xADb\x19Z\xC2\xBB\xAB\x1E)2\xF8\xC9z\xF9&\xB4\x91[\x84RaD\x9EV[`\x10\x81\x16aDyW\x7F\x02.\x17\x0C\xDF3\x8FE\xBCq\x8FX\xD2\x9B\xFA\xFB\xF3\x95l/\x9E\xA8\xD1\x9C\xCC\x9Br\xE4-\xBB\xB7\xB0aD\x9BV[\x7F\xB0a{\x84\xF6\x94\xC2E\xE5O\xB8\x03.\xBD\xC9\xF5n\xB2n\xA2\xC1\xB6ZF\xC5\x8FP\xDB\xD5\x16\xE2\x86[\x84R[`\x01\x81\x16\x15\x15`\xC0\x94\x90\x94\x01\x93\x90\x93RP\x91V[`\x05\x81\x90\x1B` \x81\x18\x83\x01Q\x90\x83\x01\x80Q`\x80\x90\x91\x01Q``\x85\x01Qb\x0FB@\x90\x81\x03\x90aD\xE0\x82\x84a[\x8DV[aD\xEA\x91\x90a[\x1EV[\x91PP\x92P\x92P\x92V[_\x80\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x83aE\xCCW\x845`\xE8\x1C`\x03\x86\x01\x95P`@Q`\x14`d\x03\x81\x01\x82\x81\x01`@R\x82\x88\x827\x82\x90 `P\x82\x81\x01Q\x7Ft\x07\x90\\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`@`$\x85\x01\x81\x90R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xEC\x86\x01`D\x86\x01R\x99\x85\x01\x99`\xC0\x94\x90\x94\x1Bw\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\x91\x90\x94\x1C\x16\x93\x01\x92\x90\x92\x17\x17\x92P\x90P[\x84\x92P\x92P\x92P\x92V[_`\x10\x82\x16\x15aF\x04W`\x08\x83a\x01x\x86\x017`\x08\x92\x90\x92\x01\x91`\x05\x83a\x01\x9B\x86\x017`\x05\x83\x01\x92PaF\x16V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFC\x16a\x01`\x85\x01R[P\x90\x92\x91PPV[_\x80\x80\x80` \x86\x16\x15aF\xD0WP\x855`\x80\x90\x81\x1C`@\x89\x01\x81\x90R`\x10\x88\x015\x82\x1C``\x8A\x01\x81\x90R`0\x89\x01\x98` \x015\x90\x92\x1C\x91\x81\x83\x10\x15aF\x8FW`@Q\x7F\xC4\xDA\xF0\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83\x11\x15aF\xC9W`@Q\x7FD\x18#1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPaF\xFBV[P`\x10\x86\x01\x955`\x80\x1C`@\x86\x16aF\xE8W_aF\xEBV[`\x01[`\xFF\x16`@\x89\x01R``\x88\x01\x81\x90R[` \x87\x01\x96`\x10\x81\x015`\x80\x90\x81\x1C\x915\x90\x1C\x80\x82\x11\x15aGHW`@Q\x7Ff\x8F\xEF\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x8A\x01R`\x08\x87\x16\x15aG\xC3Wo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16``\x88\x16\x15aG\xA2W\x82\x94PaG\x9BaG\x94\x86\x83aL\xB1V[\x88\x90aL\xBCV[\x93PaG\xBDV[\x91\x92P\x82\x91aG\xBAaG\xB4\x88\x85aL\xC7V[\x82aL\xD2V[\x94P[PaH\x1AV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16``\x88\x16\x15aG\xFEW\x91\x93P\x83\x91aG\xF7aG\xF1\x88\x85aL\xBCV[\x82aL\xB1V[\x93PaH\x18V[\x82\x93PaH\x15aH\x0E\x85\x83aL\xD2V[\x88\x90aL\xC7V[\x94P[P[P\x95\x97\x91\x96P\x94P\x92PPPV[_\x80`\x10\x83\x16aH:Wa\x01\x80aH>V[a\x01\xA0[\x90\x93 \x93\x92PPPV[\x80B\x11\x15a\x05\x15W`@Q\x7F =\x82\xD8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x15a\x1E\xA6Wc\xFF\xFF\xFF\xFF\x82\x16\x82`\xC0\x1C\x82`\x04\x82\x01R\x83` \x1C` _\x84\x84_\x85Z\xF1\x92PPPc$\xA2\xE4K_Q\x14`\x1F=\x11\x16\x81\x16a\x05\x06Wc\xF9Y\xFD\xAE_R`\x04`\x1C\xFD[\x80\x82\x03\x82\x81\x13\x15a\"\x1CWc\xC9eN\xD4_R`\x04`\x1C\xFD[_\x80\x80\x80aH\xFCa(E\x86\x88\x07\x83\x13\x87\x89\x05\x03`\x01a[\xA4V[\x90\x92P\x90PaI\x1F\x81aI\x19`\x01`\x01`\xA0\x1B\x03\x8B\x16\x8A\x86a3\xA7V[\x90aL\xDDV[\x90\x94P\x90PaI/\x82\x82\x87aM\x9FV[\x92PPP\x94P\x94\x92PPPV[_\x80\x80\x80aIQ\x85\x87\x07\x82\x13\x86\x88\x05\x03a(EV[\x90\x92P\x90PaI\x1F\x81aIn`\x01`\x01`\xA0\x1B\x03\x8B\x16\x8A\x86a3\xA7V[\x90aM\xC9V[\x81\x81\x01\x82\x81\x12\x15a\"\x1CWc\xC9eN\xD4_R`\x04`\x1C\xFD[_\x81\x81R`\x06` R`@\x81 _a3\xD5`\x01`\x01`\xA0\x1B\x03\x86\x16`\x03\x84\x01a3wV[_\x80\x80\x80`\x01\x81\x80\x80[\x83\x15aJ\x7FW`\x10\x8B\x01\x9A5`\x80\x1CaI\xD3\x81\x85aXuV[\x93Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16`\x80\x82\x90\x1B\x04\x83\x01\x92P\x82\x8E\x8Eb\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10aJ\rWaJ\raW\x91V[\x01_\x82\x82T\x01\x92PP\x81\x90UP_aJ\\\x8B_\x01Q\x8F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16aN\x91\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PPaJi\x8C\x82aN\xF6V[\x9BP\x8D`\x13R\x8B`\x10R\x82_R`3_ \x92PPP[\x88Q` \x8A\x01QaJ\xBC\x91`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x8F\x90aO\x10V[\x80\x9DP\x81\x95PPP\x88`@\x01Q`\x02\x0B\x8C`\x02\x0B\x13aI\xBAW``\x98\x90\x98\x01\x97\x90\x97R\x97\x9A\x97\x99P\x94\x97P\x94\x95\x94PPPPPV[_\x80\x80\x80`\x01\x81\x80\x80[\x83\x15aK\xC0W`\x10\x8B\x01\x9A5`\x80\x1CaK\x14\x81\x85aXuV[\x93Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16`\x80\x82\x90\x1B\x04\x83\x01\x92P\x82\x8E\x8Eb\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10aKNWaKNaW\x91V[\x01_\x82\x82T\x01\x92PP\x81\x90UP_aK\x9D\x8B_\x01Q\x8F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16aN\x91\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PPaK\xAA\x8C\x82aO*V[\x9BP\x8D`\x13R\x8B`\x10R\x82_R`3_ \x92PPP[\x88Q` \x8A\x01QaK\xFD\x91`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x8F\x90aH\xE2V[\x80\x9DP\x81\x95PPP\x88`@\x01Q`\x02\x0B\x8C`\x02\x0B\x13\x15aJ\xFBW``\x98\x90\x98\x01\x97\x90\x97R\x97\x9A\x97\x99P\x94\x97P\x94\x95\x94PPPPPV[\x80\x82\x14a\x1E\xA6W`@Q\x7F\x01\x84/\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`@Qc\x16&\xBA~`\xE0\x1B\x80\x82R\x85`\x04\x83\x01R`$\x82\x01`@\x81R\x84`D\x84\x01R\x84\x86`d\x85\x017` \x81`d\x87\x01\x85\x8BZ\xFA\x90Q\x90\x91\x14\x16\x96\x95PPPPPPV[_a\x17)\x82\x84a[\x0BV[_a\x17)\x82\x84aODV[_a\x17)\x82\x84aOfV[_a\x17)\x82\x84aXuV[_\x80_aMx\x84`\xFF\x16\x86\x90\x1C~\x1F\r\x1E\x10\x0C\x1D\x07\x0F\t\x0B\x19\x13\x1C\x17\x06\x01\x0E\x11\x08\n\x1A\x14\x18\x02\x12\x1B\x15\x03\x16\x04\x05\x81\x19`\x01\x01\x90\x91\x16a\x01\xE0\x7F\x80@@UC\0RfD2\0\0P a\x06t\x050&\x02\0\0\x10u\x06 \x01v\x11pw`\xFC\x7F\xB6\xDBm\xB6\xDD\xDD\xDD\xDD\xD3M4\xD3I$\x92I!\x08B\x10\x8Cc\x18\xC69\xCEs\x9C\xFF\xFF\xFF\xFF\x84\x02`\xF8\x1C\x16\x1B`\xF7\x1C\x16\x90\x81\x1Cc\xD7dS\xE0\x04`\x1F\x16\x91\x90\x91\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x92P\x82aM\x8EW`\xFFaM\x95V[\x83`\xFF\x16\x81\x01[\x91PP\x92P\x92\x90PV[_\x81`\xFF\x84\x16aM\xB5`\x01\x87\x90\x0Ba\x01\0a[\xE5V[aM\xBF\x91\x90a[\xA4V[a\"\xAA\x91\x90a[\xE5V[_\x80_\x83`\xFF\x03\x90P_aNj\x82`\xFF\x16\x87\x90\x1B\x7F\x07\x06\x06\x05\x06\x02\x05\x04\x06\x02\x03\x02\x05\x04\x03\x01\x06\x05\x02\x05\x03\x03\x04\x01\x05\x05\x03\x04\0\0\0\0`\x1Fo\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x83\x15`\x08\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11`\x07\x1B\x17\x84\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x84\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x84\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x84\x81\x1C`\xFF\x10`\x03\x1B\x17\x93\x84\x1C\x1C\x16\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x93P\x83aN\x7FW_aN\x86V[\x81`\xFF\x16\x81\x03[\x92PPP\x92P\x92\x90PV[_\x80`\x06` R\x83_R`\x04`@_ \x01` R\x82_R`@_ ` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x88Z\xFAaN\xD1WcS\\\xF9K_R`\x04`\x1C\xFD[PP_Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x94`\x80\x91\x90\x91\x1D\x93P\x91PPV[\x80\x82\x03`\x80\x81\x90\x1C\x15a\"\x1CWc\xC9eN\xD4_R`\x04`\x1C\xFD[_\x80\x80\x80aIQa(E`\x01\x87\x89\x07\x84\x13\x88\x8A\x05\x03aY\xBDV[\x81\x81\x01`\x80\x81\x90\x1C\x15a\"\x1CWc\xC9eN\xD4_R`\x04`\x1C\xFD[_k\x03;.<\x9F\xD0\x80<\xE8\0\0\0aO\\\x83\x85a[\x8DV[a\x17)\x91\x90a[\x1EV[_a\x17)\x83k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x84\x82\x82\x02\x83\x15\x84\x82\x04\x84\x14\x17\x82\x02aO\x97Wc\xAD%\x1C'_R`\x04`\x1C\xFD[\x81\x81\x06\x15\x15\x91\x90\x04\x01\x92\x91PPV[_\x80\x83`\x1F\x84\x01\x12aO\xB6W_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aO\xCDW_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aB\x91W_\x80\xFD[_\x80` \x83\x85\x03\x12\x15aO\xF5W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aP\x0BW_\x80\xFD[aP\x17\x85\x82\x86\x01aO\xA6V[\x90\x96\x90\x95P\x93PPPPV[_` \x82\x84\x03\x12\x15aP3W_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x17)W_\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\x15W_\x80\xFD[\x805b\xFF\xFF\xFF\x81\x16\x81\x14aPpW_\x80\xFD[\x91\x90PV[_\x80_\x80_\x80`\xC0\x87\x89\x03\x12\x15aP\x8AW_\x80\xFD[\x865aP\x95\x81aPJV[\x95P` \x87\x015aP\xA5\x81aPJV[\x94P`@\x87\x015a\xFF\xFF\x81\x16\x81\x14aP\xBBW_\x80\xFD[\x93PaP\xC9``\x88\x01aP^V[\x92PaP\xD7`\x80\x88\x01aP^V[\x91PaP\xE5`\xA0\x88\x01aP^V[\x90P\x92\x95P\x92\x95P\x92\x95V[_\x80_`@\x84\x86\x03\x12\x15aQ\x03W_\x80\xFD[\x835aQ\x0E\x81aPJV[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aQ)W_\x80\xFD[aQ5\x86\x82\x87\x01aO\xA6V[\x94\x97\x90\x96P\x93\x94PPPPV[_`\xA0\x82\x84\x03\x12\x15aQRW_\x80\xFD[P\x91\x90PV[_\x80_\x80_\x85\x87\x03a\x01`\x81\x12\x15aQnW_\x80\xFD[\x865aQy\x81aPJV[\x95PaQ\x88\x88` \x89\x01aQBV[\x94P`\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF@\x82\x01\x12\x15aQ\xB9W_\x80\xFD[P`\xC0\x86\x01\x92Pa\x01@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aQ\xDAW_\x80\xFD[aQ\xE6\x88\x82\x89\x01aO\xA6V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_\x80`@\x83\x85\x03\x12\x15aR\x08W_\x80\xFD[\x825aR\x13\x81aPJV[\x94` \x93\x90\x93\x015\x93PPPV[_\x80_`@\x84\x86\x03\x12\x15aR3W_\x80\xFD[\x835aR>\x81aPJV[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aRYW_\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13aRiW_\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aR\x7FW_\x80\xFD[\x86` `\xA0\x83\x02\x84\x01\x01\x11\x15aR\x93W_\x80\xFD[\x93\x96` \x91\x90\x91\x01\x95P\x92\x93PPPV[_``\x82\x84\x03\x12\x15aQRW_\x80\xFD[_\x80_\x80_a\x01@\x86\x88\x03\x12\x15aR\xC9W_\x80\xFD[\x855aR\xD4\x81aPJV[\x94PaR\xE3\x87` \x88\x01aQBV[\x93PaR\xF2\x87`\xC0\x88\x01aR\xA4V[\x92Pa\x01 \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aQ\xDAW_\x80\xFD[_` \x82\x84\x03\x12\x15aS\x1EW_\x80\xFD[P5\x91\x90PV[_\x80_``\x84\x86\x03\x12\x15aS7W_\x80\xFD[\x835aSB\x81aPJV[\x92P` \x84\x015aSR\x81aPJV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x16\x81R`\xE0` \x82\x01R_aS\xE9`\xE0\x83\x01\x89aScV[\x82\x81\x03`@\x84\x01RaS\xFB\x81\x89aScV[``\x84\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x84Q\x80\x82R` \x80\x87\x01\x93P\x90\x91\x01\x90_[\x81\x81\x10\x15aTPW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aT2V[P\x90\x9B\x9APPPPPPPPPPPV[_\x80_\x80`\x80\x85\x87\x03\x12\x15aTtW_\x80\xFD[\x845aT\x7F\x81aPJV[\x93P` \x85\x015aT\x8F\x81aPJV[\x92P`@\x85\x015\x91P``\x85\x015aT\xA6\x81aPJV[\x93\x96\x92\x95P\x90\x93PPV[` \x81R_a\x17)` \x83\x01\x84aScV[_` \x82\x84\x03\x12\x15aT\xD3W_\x80\xFD[\x815a\x17)\x81aPJV[_\x80_\x80_\x80a\x01`\x87\x89\x03\x12\x15aT\xF4W_\x80\xFD[\x865aT\xFF\x81aPJV[\x95PaU\x0E\x88` \x89\x01aQBV[\x94PaU\x1D\x88`\xC0\x89\x01aR\xA4V[\x93Pa\x01 \x87\x015\x92Pa\x01@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aUAW_\x80\xFD[aUM\x89\x82\x8A\x01aO\xA6V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[_\x80` \x83\x85\x03\x12\x15aUpW_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aU\x86W_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aU\x96W_\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aU\xACW_\x80\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15aU\xC0W_\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[\x805\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x81\x16\x81\x14aPpW_\x80\xFD[_\x80_``\x84\x86\x03\x12\x15aV\x11W_\x80\xFD[aSB\x84aU\xD0V[\x81\x83R\x81\x81` \x85\x017P_` \x82\x84\x01\x01R_` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[` \x81R_a\"\xAA` \x83\x01\x84\x86aV\x1AV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15aV\xB1W_\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aV\xC7W_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13aV\xD7W_\x80\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aV\xF1WaV\xF1aVtV[`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`?\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x85\x01\x16\x01\x16\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aW]WaW]aVtV[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15aWtW_\x80\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x80`\x02\x0B\x81\x14a\x05\x15W_\x80\xFD[_` \x82\x84\x03\x12\x15aW\xDCW_\x80\xFD[\x815a\x17)\x81aW\xBEV[_` \x82\x84\x03\x12\x15aW\xF7W_\x80\xFD[PQ\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\"\x1CWa\"\x1CaW\xFEV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`@` \x82\x01R_a3\xD5`@\x83\x01\x84\x86aV\x1AV[\x80\x82\x01\x80\x82\x11\x15a\"\x1CWa\"\x1CaW\xFEV[_` \x82\x84\x03\x12\x15aX\x98W_\x80\xFD[a\x17)\x82aP^V[_` \x82\x84\x03\x12\x15aX\xB1W_\x80\xFD[a\x17)\x82aU\xD0V[_\x80\x85\x85\x11\x15aX\xC8W_\x80\xFD[\x83\x86\x11\x15aX\xD4W_\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x805\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90`\x14\x84\x10\x15aY@W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x80\x85`\x14\x03`\x03\x1B\x1B\x82\x16\x16\x91P[P\x92\x91PPV[_` \x82\x84\x03\x12\x15aYWW_\x80\xFD[\x81Qa\x17)\x81aW\xBEV[_` \x82\x84\x03\x12\x15aYrW_\x80\xFD[\x815\x80\x15\x15\x81\x14a\x17)W_\x80\xFD[_\x81`\x0F\x0B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x03aY\xB5WaY\xB5aW\xFEV[_\x03\x92\x91PPV[`\x02\x82\x81\x0B\x90\x82\x90\x0B\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x81\x12b\x7F\xFF\xFF\x82\x13\x17\x15a\"\x1CWa\"\x1CaW\xFEV[_\x82`\x0F\x0B\x82`\x0F\x0B\x02\x80`\x0F\x0B\x91P\x80\x82\x14aY@WaY@aW\xFEV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x81`\x0F\x0B\x83`\x0F\x0B\x80aZ`WaZ`aZ\x1DV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x14\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x14\x16\x15aZ\xB4WaZ\xB4aW\xFEV[\x90\x05\x93\x92PPPV[`\x0F\x82\x81\x0B\x90\x82\x90\x0B\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x12o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x13\x17\x15a\"\x1CWa\"\x1CaW\xFEV[\x81\x81\x03\x81\x81\x11\x15a\"\x1CWa\"\x1CaW\xFEV[_\x82a[,Wa[,aZ\x1DV[P\x04\x90V[_\x81`\x02\x0B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x81\x03a[eWa[eaW\xFEV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\"\x1CWa\"\x1CaW\xFEV[`\x02\x81\x81\x0B\x90\x83\x90\x0B\x01b\x7F\xFF\xFF\x81\x13\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x82\x12\x17\x15a\"\x1CWa\"\x1CaW\xFEV[_\x82`\x02\x0B\x82`\x02\x0B\x02\x80`\x02\x0B\x91P\x80\x82\x14aY@WaY@aW\xFEV\xFE\xA1dsolcC\0\x08\x1A\0\n`\xC0`@R4\x80\x15`\x0EW_\x80\xFD[P`@Qa\x08\xE48\x03\x80a\x08\xE4\x839\x81\x01`@\x81\x90R`+\x91`?V[3`\x80R`\x01`\x01`\xA0\x1B\x03\x16`\xA0R`jV[_` \x82\x84\x03\x12\x15`NW_\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14`cW_\x80\xFD[\x93\x92PPPV[`\x80Q`\xA0Qa\x08?a\0\xA5_9_\x81\x81`\xE7\x01R\x81\x81a\x01\xE4\x01R\x81\x81a\x02\xC2\x01R\x81\x81a\x03\x94\x01Ra\x04?\x01R_`\x8E\x01Ra\x08?_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\x004W_5`\xE0\x1C\x80c\x87t\x15\xD2\x14a\08W\x80c\x91\xDDsF\x14a\0MW[_\x80\xFD[a\0Ka\0F6`\x04a\x05\x03V[a\0vV[\0[a\0`a\0[6`\x04a\x05lV[a\x01\xCAV[`@Qa\0m\x91\x90a\x05\xABV[`@Q\x80\x91\x03\x90\xF3[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\0\xE5W`@Q\x7F0\xCDtq\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cH\xC8\x94\x91\x84``\x1B\x84\x84`@Q` \x01a\x019\x93\x92\x91\x90a\x05\xFEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01d\x91\x90a\x05\xABV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x01\x7FW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x01\xC4\x91\x90\x81\x01\x90a\x06fV[PPPPV[``3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x02;W`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x02I`\x14\x82\x85\x87a\x07VV[a\x02R\x91a\x07}V[``\x1C\x90Pa\x02d\x83`\x14\x81\x87a\x07VV[\x90\x94P\x92P_a\x02u`\x14\x85a\x07\xE3V[\x90P_[\x81\x81\x10\x15a\x04\xA6W`@Q~\xFD\xD5\x8E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R`\x14\x82\x02\x87\x015``\x1C`$\x82\x01\x81\x90R\x90_\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90b\xFD\xD5\x8E\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\x1BW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03?\x91\x90a\x08\x1BV[`@Q\x7F\xF5)\x8A\xCA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16`$\x83\x01R`D\x82\x01\x83\x90R\x91\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xF5)\x8A\xCA\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x03\xD7W_\x80\xFD[PZ\xF1\x15\x80\x15a\x03\xE9W=_\x80>=_\xFD[PP`@Q\x7F\x0B\r\x9C\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x81\x16`\x04\x83\x01R\x88\x81\x16`$\x83\x01R`D\x82\x01\x85\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc\x0B\r\x9C\t\x91P`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x04\x82W_\x80\xFD[PZ\xF1\x15\x80\x15a\x04\x94W=_\x80>=_\xFD[PP`\x01\x90\x94\x01\x93Pa\x02y\x92PPPV[PP`@\x80Q` \x81\x01\x90\x91R_\x81R\x94\x93PPPPV[_\x80\x83`\x1F\x84\x01\x12a\x04\xCEW_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\xE5W_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x04\xFCW_\x80\xFD[\x92P\x92\x90PV[_\x80_`@\x84\x86\x03\x12\x15a\x05\x15W_\x80\xFD[\x835s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x058W_\x80\xFD[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05SW_\x80\xFD[a\x05_\x86\x82\x87\x01a\x04\xBEV[\x94\x97\x90\x96P\x93\x94PPPPV[_\x80` \x83\x85\x03\x12\x15a\x05}W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\x93W_\x80\xFD[a\x05\x9F\x85\x82\x86\x01a\x04\xBEV[\x90\x96\x90\x95P\x93PPPPV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x84\x16\x81R\x81\x83`\x14\x83\x017_\x91\x01`\x14\x01\x90\x81R\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a\x06vW_\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\x8CW_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x06\x9CW_\x80\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xB6Wa\x06\xB6a\x069V[`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`?\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x85\x01\x16\x01\x16\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x07\"Wa\x07\"a\x069V[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a\x079W_\x80\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_\x80\x85\x85\x11\x15a\x07dW_\x80\xFD[\x83\x86\x11\x15a\x07pW_\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x805\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90`\x14\x84\x10\x15a\x07\xDCW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x80\x85`\x14\x03`\x03\x1B\x1B\x82\x16\x16\x91P[P\x92\x91PPV[_\x82a\x08\x16W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[P\x04\x90V[_` \x82\x84\x03\x12\x15a\x08+W_\x80\xFD[PQ\x91\x90PV\xFE\xA1dsolcC\0\x08\x1A\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b5060043610610184575f3560e01c80637cf98081116100dd578063b47b2fb111610088578063d9e17f9811610063578063d9e17f98146103f2578063dd4d4cf614610405578063f3fef3a314610418575f80fd5b8063b47b2fb11461037e578063d6cffd1e146103cc578063d9caed12146103df575f80fd5b80638587f450116100b85780638587f4501461033857806391dd73461461034b57806392eefe9b1461036b575f80fd5b80637cf98081146102e95780638340f5491461030a57806384b0196e1461031d575f80fd5b8063259982e51161013d57806353b41c551161011857806353b41c5514610258578063575e24b41461026b5780637407905c146102c1575f80fd5b8063259982e51461021f57806333830e481461023257806347e7ef2414610245575f80fd5b8063138714651161016d57806313871465146101b05780631828e0e7146101c357806321d0ee70146101d6575f80fd5b806309c5eabe14610188578063116a55501461019d575b5f80fd5b61019b610196366004614fe4565b61042b565b005b61019b6101ab366004615023565b61050b565b61019b6101be366004615075565b610518565b61019b6101d13660046150f1565b61076b565b6101e96101e4366004615158565b6108cc565b6040517fffffffff0000000000000000000000000000000000000000000000000000000090911681526020015b60405180910390f35b6101e961022d366004615158565b610be8565b61019b6102403660046150f1565b610f5c565b61019b6102533660046151f7565b610fff565b61019b610266366004615221565b61104f565b61027e6102793660046152b4565b611280565b604080517fffffffff000000000000000000000000000000000000000000000000000000009094168452602084019290925262ffffff1690820152606001610216565b6102d46102cf3660046150f1565b611454565b60405163ffffffff9091168152602001610216565b6102fc6102f736600461530e565b611730565b604051908152602001610216565b61019b610318366004615325565b61173a565b61032561178f565b60405161021697969594939291906153af565b61019b610346366004615461565b611837565b61035e610359366004614fe4565b6119bf565b60405161021691906154b1565b61019b6103793660046154c3565b611a53565b61039161038c3660046154de565b611a94565b604080517fffffffff000000000000000000000000000000000000000000000000000000009093168352600f9190910b602083015201610216565b61019b6103da36600461555f565b611db2565b61019b6103ed366004615325565b611e3e565b61019b6104003660046151f7565b611e8a565b61019b6104133660046155ff565b611eaa565b61019b6104263660046151f7565b611fe0565b61043361202c565b5f81900361043f575050565b6040517f48c894910000000000000000000000000000000000000000000000000000000081526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906348c89491906104a69085908590600401615661565b5f604051808303815f875af11580156104c1573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016820160405261050691908101906156a1565b505050565b61051533826120ff565b50565b61052061213a565b846001600160a01b0316866001600160a01b03161061056b576040517f32b4bc9300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f86815260208690526040812060281b6003549091505f906105a3906801000000000000000090046001600160a01b0316600161217f565b8051519091505f905b80821015610654575f835f015183815181106105ca576105ca615791565b602002602001015190506105e46105de8290565b866121f3565b15610648576106228861061c8b875f0151878151811061060657610606615791565b602002602001015161222290919063ffffffff16565b9061224c565b845180518590811061063657610636615791565b60200260200101818152505050610654565b506001909101906105ac565b80820361067057610670610669858a8a612267565b84906122b2565b6106798361232d565b600380546001600160a01b039290921668010000000000000000027fffffffff0000000000000000000000000000000000000000ffffffffffffffff90921691909117905560408051808201825262ffffff808916825287811660208084018281527fffffffffffffffffffffffffffffffffffffffffffffffffffffff00000000008a165f908152600290925294902092518354945183166301000000027fffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000009095169216919091179290921790556107519061239c565b61075f8662ffffff1661239c565b50505050505050505050565b61078060035467ffffffffffffffff16431490565b156107b7576040517fd8a6b89b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6001600160a01b0383165f9081526001602052604090205460ff16610808576040517f5cd26b6800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b7f3f25e551746414ff93f076a7dd83828ff53735b39366c74015637e004fcb02235f90815243602052604081209061083f826123db565b905061084d858286866124f1565b610883576040517f8baa579f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61088c436125dd565b600380547fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000001667ffffffffffffffff929092169190911790555050505050565b5f6108d56125fa565b5f6108df8661265c565b90505f8061093f838a6108f560208b018b6157cc565b61090560408c0160208d016157cc565b60069081526003919091525f91825260608b01356026908152603a600c209083905292825260209081526040808320848452909152902091565b90925090505f61098461097b6001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001686612670565b60a01c60020b90565b90505f6109bd8261099860208c018c6157cc565b6109a860408d0160208e016157cc565b5f898152600760205260409020929190612690565b90505f6109f46001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000168786612712565b90505f610a18865f01548403836fffffffffffffffffffffffffffffffff1661276d565b90508015610bb6576001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001663a5841194610a5c60208f018f6154c3565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b1681526001600160a01b0390911660048201526024015f604051808303815f87803b158015610ab2575f80fd5b505af1158015610ac4573d5f803e3d5ffd5b50505050610b0f7f0000000000000000000000000000000000000000000000000000000000000000828e5f016020810190610aff91906154c3565b6001600160a01b031691906127e2565b6040517f3dd45adb0000000000000000000000000000000000000000000000000000000081526001600160a01b038e811660048301527f00000000000000000000000000000000000000000000000000000000000000001690633dd45adb906024016020604051808303815f875af1158015610b8d573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610bb191906157e7565b508286555b507f21d0ee70000000000000000000000000000000000000000000000000000000009c9b505050505050505050505050565b5f610bf16125fa565b5f610bfb8661265c565b5f81815260076020526040812091925080610c4261097b6001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001686612670565b90505f83610c5360208b018b6157cc565b62ffffff1663010000008110610c6b57610c6b615791565b015490505f84610c8160408c0160208d016157cc565b62ffffff1663010000008110610c9957610c99615791565b01549050610caa60208b018b6157cc565b60020b8360020b1215610cc1578082039350610e1c565b600283900b610cd660408c0160208d016157cc565b60020b13610dc457610d3486610cef60208d018d6157cc565b8d6060016020810190610d0291906157cc565b6001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016929190612835565b610d6c57630100000085015491508185610d5160208d018d6157cc565b62ffffff1663010000008110610d6957610d69615791565b01555b610d8086610cef60408d0160208e016157cc565b610dba575063010000008401548085610d9f60408d0160208e016157cc565b62ffffff1663010000008110610db757610db7615791565b01555b8181039350610e1c565b610dd586610cef60208d018d6157cc565b610e0d57630100000085015491508185610df260208d018d6157cc565b62ffffff1663010000008110610e0a57610e0a615791565b01555b80828663010000000154030393505b505f9150819050610e7f858c610e3560208d018d6157cc565b610e4560408e0160208f016157cc565b60069081526003919091525f91825260608d01356026908152603a600c209083905292825260209081526040808320848452909152902091565b90925090505f610eb96001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000168784612712565b905060408a01355f610ecb828461582b565b9050826fffffffffffffffffffffffffffffffff165f03610eee57858555610f29565b5f610f23865f01548803856fffffffffffffffffffffffffffffffff16846fffffffffffffffffffffffffffffffff1661288b565b87038655505b507f259982e5000000000000000000000000000000000000000000000000000000009d9c50505050505050505050505050565b610f6461213a565b6040517f877415d20000000000000000000000000000000000000000000000000000000081526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063877415d290610fcd90869086908690600401615853565b5f604051808303815f87803b158015610fe4575f80fd5b505af1158015610ff6573d5f803e3d5ffd5b50505050505050565b6110146001600160a01b038316333084612938565b6001600160a01b0382165f90815260056020908152604080832033845290915281208054839290611046908490615875565b90915550505050565b61105761213a565b6003546001600160a01b0368010000000000000000909104811690841681146110ac576040517ff21fd99f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6003545f906110d0906801000000000000000090046001600160a01b03168261217f565b90505f5b8381101561124957368585838181106110ef576110ef615791565b905060a00201905061112981604001602081019061110d9190615888565b61061c61112060408501602086016158a1565b8690853561299a565b83518051833590811061113e5761113e615791565b602090810291909101015261116661115c6080830160608401615888565b62ffffff1661239c565b61117961115c60a0830160808401615888565b60408051808201909152806111946080840160608501615888565b62ffffff1681526020016111ae60a0840160808501615888565b62ffffff16905260025f6111c860408501602086016158a1565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffff00000000001681526020808201929092526040015f2082518154939092015162ffffff9081166301000000027fffffffffffffffffffffffffffffffffffffffffffffffffffff000000000000909416921691909117919091179055506001016110d4565b506112538161232d565b600360086101000a8154816001600160a01b0302191690836001600160a01b031602179055505050505050565b5f805f61128b6125fa565b6112a060035467ffffffffffffffff16431490565b611357576014841015611319575f8490036112e7576040517f1e8107a000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6040517f4926898b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f61132760148287896158ba565b611330916158e1565b60601c9050365f611344876014818b6158ba565b9150915061135383838361076b565b5050505b5f61139261136e61136b60208b018b6154c3565b90565b61138161136b60408c0160208d016154c3565b5f9182526020526040902060281b90565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffff000000000081165f9081526002602052604081205462ffffff16624000001793509091506113db8961265c565b905061142161141661097b6001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001684612670565b60089060020b612a27565b507f575e24b40000000000000000000000000000000000000000000000000000000093505f925050955095509592505050565b5f600183018335821a8061155157604080517fd505accf0000000000000000000000000000000000000000000000000000000081526001600160a01b03881660048201523360248201527fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6044820152601484013560d81c6064820181905260198501355f90811a60848401819052601a87013560a48501819052603a88013560c486018190529551605a8901983560601c969495929491939192879263d505accf9260e48084019382900301818387803b158015611531575f80fd5b505af1158015611543573d5f803e3d5ffd5b505050505050505050611714565b60018160ff160361162c576040517fd505accf0000000000000000000000000000000000000000000000000000000081526001600160a01b038716600482015233602482810191909152601484013560801c604483018190529084013560d81c6064830181905260298501355f1a60848401819052602a86013560a48501819052604a87013560c48601819052606a8801973560601c95869063d505accf9060e4015b5f604051808303815f87803b15801561160b575f80fd5b505af115801561161d573d5f803e3d5ffd5b50505050505050505050611714565b60028160ff16036116d8576040517f8fcbaf0c0000000000000000000000000000000000000000000000000000000081526001600160a01b0387166004820152336024820152601483013560e01c60448201819052601884013560d81c6064830181905260016084840152601d8501355f1a60a48401819052601e86013560c48501819052603e87013560e48601819052605e8801973560601c958690638fcbaf0c90610104016115f4565b6040517f6f1d150900000000000000000000000000000000000000000000000000000000815260ff821660048201526024015b60405180910390fd5b61171f828686612a2e565b6324a2e44b925050505b9392505050565b5f81545f5260205ff35b61174f6001600160a01b038416333084612938565b6001600160a01b038084165f90815260056020908152604080832093861683529290529081208054839290611785908490615875565b9091555050505050565b7f0f000000000000000000000000000000000000000000000000000000000000006060805f808083611825604080518082018252600881527f416e677374726f6d0000000000000000000000000000000000000000000000006020808301919091528251808401909352600283527f76310000000000000000000000000000000000000000000000000000000000009083015291565b97989097965046955030945091925090565b826001600160a01b0316846001600160a01b03161115611855579192915b5f84815260208490526040812060281b6003549091505f9061188d906801000000000000000090046001600160a01b03168386612a4b565b5090507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316636276cbbe6040518060a001604052806118d18a90565b6001600160a01b03168152602001886001600160a01b03908116825262800000602080840191909152600287810b6040808601919091523060609586015280517fffffffff0000000000000000000000000000000000000000000000000000000060e089901b168152865185166004820152928601518416602484015285015162ffffff1660448301529284015190920b60648301526080909201518216608482015290861660a482015260c4016020604051808303815f875af115801561199b573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ff69190615947565b60606119c96125fa565b825f6119d482612ab5565b60035491935091505f90611a0090849084906801000000000000000090046001600160a01b0316612b6c565b9093509050611a0e82612ce1565b611a188382612d0c565b9250611a248382612d98565b9250611a308382612e4b565b9250611a3d838787612a2e565b611a4682612eea565b60205f525f60205260405ff35b611a5b61213a565b5f80547fffffffffffffffffffffffff0000000000000000000000000000000000000000166001600160a01b0392909216919091179055565b5f80611a9e6125fa565b5f80611ac6611ab361136b60208c018c6154c3565b61138161136b60408d0160208e016154c3565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffff000000000081165f908152600260209081526040822054929350630100000090920462ffffff16918a01803582139190611b1c908c615962565b151582151503611b3557611b308a600f0b90565b611b3f565b611b3f8a60801d90565b90505f8082600f0b12611b525781611b5b565b611b5b82615981565b905082611b995780611b7085620f42406159bd565b60020b611b80620f4240846159fe565b611b8a9190615a4a565b611b949190615abd565b611bb5565b620f4240611bab600286900b836159fe565b611bb59190615a4a565b9550506001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016905063156e29f67f0000000000000000000000000000000000000000000000000000000000000000611c5a611c1a60208f018f615962565b151585151503611c3c578e6020016020810190611c3791906154c3565b611c4e565b8e5f016020810190611c4e91906154c3565b6001600160a01b031690565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e085901b1681526001600160a01b03909216600483015260248201526fffffffffffffffffffffffffffffffff871660448201526064015f604051808303815f87803b158015611ccd575f80fd5b505af1158015611cdf573d5f803e3d5ffd5b505050505050505f611cf08961265c565b90505f611d2961097b6001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001684612670565b9050611d7e827f000000000000000000000000000000000000000000000000000000000000000060085c848e6060016020810190611d6791906157cc565b5f8881526007602052604090209493929190613112565b507fb47b2fb1000000000000000000000000000000000000000000000000000000009350909150505b965096945050505050565b611dba61213a565b5f5b81811015610506575f838383818110611dd757611dd7615791565b9050602002016020810190611dec91906154c3565b6001600160a01b03165f90815260016020819052604090912080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00811660ff90911615179055919091019050611dbc565b6001600160a01b0383165f90815260056020908152604080832033845290915281208054839290611e70908490615b0b565b9091555061050690506001600160a01b03841683836127e2565b611e9261213a565b611ea66001600160a01b03831633836127e2565b5050565b611eb261213a565b6003546001600160a01b036801000000000000000090910481169083168114611f07576040517ff21fd99f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6003545f90611f2a906801000000000000000090046001600160a01b031661319b565b9050611f378186856131c2565b611f408161232d565b600380546001600160a01b039290921668010000000000000000027fffffffff0000000000000000000000000000000000000000ffffffffffffffff909216919091179055505050507fffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000165f90815260026020526040902080547fffffffffffffffffffffffffffffffffffffffffffffffffffff000000000000169055565b6001600160a01b0382165f90815260056020908152604080832033845290915281208054839290612012908490615b0b565b90915550611ea690506001600160a01b03831633836127e2565b6003544367ffffffffffffffff90911603612073576040517fd8a6b89b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b335f9081526001602052604090205460ff166120bb576040517f5cd26b6800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6120c4436125dd565b600380547fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000001667ffffffffffffffff92909216919091179055565b80600c5263daa050e9600452815f52601f600c20600160ff83161b8082541881811661213257638cb888725f526004601cfd5b909155505050565b5f546001600160a01b0316331461217d576040517f23019e6700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b60408051808201909152606081525f60208201525f6121a6846001600160a01b03166132ae565b90505f6121b38483615875565b602084810182905260408051838302810183019091525f81528086529192508381029081906001908401893c506121ea8484612301565b50505092915050565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000828116908216145b92915050565b5f61222c826132c4565b5065ffffff00000061ffff918216601884811c939093161890911b161890565b5f61225682613313565b5062ffffff80831691909118161890565b5f612271836132c4565b61227a82613313565b6122aa8261061c7fffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000871686612222565b949350505050565b815151602083015181036122f2576040517f5cef583a00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61230683612301836001615875565b905152565b81835f0151828151811061231c5761231c615791565b602002602001018181525050505050565b805180516b600b380380600b5f395ff30082525f9190600c60208202016014830184f081835292506001600160a01b038316612395576040517f5670258700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5050919050565b620f424062ffffff82161115610515576105157f140021130000000000000000000000000000000000000000000000000000000062ffffff8316613355565b7f00000000000000000000000000000000000000000000000000000000000000007f000000000000000000000000000000000000000000000000000000000000000030147f00000000000000000000000000000000000000000000000000000000000000004614166124ce5750604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f81527f000000000000000000000000000000000000000000000000000000000000000060208201527f00000000000000000000000000000000000000000000000000000000000000009181019190915246606082015230608082015260a090205b6719010000000000005f5280601a5281603a52604260182090505f603a52919050565b5f6001600160a01b038516156122aa57604051853b61259a578260408114612521576041811461256157506125d4565b60208581013560ff81901c601b0190915285356040527f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff16606052612572565b60408501355f1a6020526040856040375b50845f526020600160805f60015afa5180871860601b3d119250505f606052806040526125d4565b631626ba7e60e01b80825285600483015260248201604081528460448401528486606485013760208160648701858b5afa90519091141691505b50949350505050565b5f6801000000000000000082106125f6576125f661336a565b5090565b336001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161461217d576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6040515f9060a083823760a0902092915050565b5f8181526006602052604081206122aa6001600160a01b03851682613377565b5f808562ffffff8516630100000081106126ac576126ac615791565b015490505f8662ffffff8516630100000081106126cb576126cb615791565b015490508460020b8660020b12156126e657900390506122aa565b8560020b8460020b136126fb570390506122aa565b630100000087015491909103039050949350505050565b5f6006602052825f52600660405f2001602052815f5260405f20602052631e2eaeaf5f5260205f6024601c875afa6127515763535cf94b5f526004601cfd5b50505f516fffffffffffffffffffffffffffffffff1692915050565b81810281838204148315176127d9577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8284098181108201900370010000000000000000000000000000000081106127cc5763c56a01595f526004601cfd5b608091821c911b0161221c565b60801c92915050565b81601452806034526fa9059cbb0000000000000000000000005f5260205f604460105f875af18060015f51141661282b57803d853b15171061282b576390b8ec185f526004601cfd5b505f603452505050565b5f80806128538486078213858705035b600881901d9160ff90911690565b9092509050612880816128706001600160a01b038a1689866133a7565b90600160ff919091161b16151590565b979650505050505050565b82820281838583041485151702612931577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8385098181108201900382848609835f0384168285116128e45763ae47f7025f526004601cfd5b93849004938382119092035f839003839004600101029203041760026003830281188084028203028084028203028084028203028084028203028084028203028084029091030202611729565b0492915050565b60405181606052826040528360601b602c526f23b872dd000000000000000000000000600c5260205f6064601c5f895af18060015f51141661298c57803d873b15171061298c57637939f4245f526004601cfd5b505f60605260405250505050565b5f835f015182815181106129b0576129b0615791565b602002602001015190506129f06129c48290565b847fffffffffffffffffffffffffffffffffffffffffffffffffffffff00000000009081169116141590565b15611729576040517f23f69dc200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b80825d5050565b808201808414612a45576301842f8c5f526004601cfd5b50505050565b5f8080612a626001600160a01b03871686866133de565b905080612a9b576040517f2f659e4400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61ffff601882901c169662ffffff90911695509350505050565b6003818101915f918291803560e81c0101816044612ad38684615b0b565b612add9190615b1e565b905080602086901b1792505f805b82811015612b60575f612b09602087901c60448402015b3560601c90565b9050826001600160a01b0316816001600160a01b031611612b56576040517f80f11acf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b9150600101612aeb565b50829450505050915091565b6003838101935f91829182918291803560e81c0101816026612b8e8a84615b0b565b612b989190615b1e565b905060405193508060c0028401925082604052808460201b179450505f5b82841015612cd45760048901983560e081901c905f90612bde90612b02908c9060f01c61341c565b90505f612bf2612b028c61ffff861661341c565b90508363ffffffff168363ffffffff16111580612c215750806001600160a01b0316826001600160a01b031610155b15612c58576040517ff35f939900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b90865260208601526040852060028b019a91925060281b903560f01c5f80612c8a6001600160a01b038c168585612a4b565b60408a0191909152606089015250505060208a01993590505f760a70c3c40a64e6c51999090b65f67d924000000000000082900460808701525060a085015260c090930192612bb6565b5093505050935093915050565b63ffffffff81165f5b8181101561050657612d04602084901c604483020161347a565b600101612cea565b60408051610160810182525f602082018190529181018290526080810182905260c0810182905260e081018290526101008101829052610140810182905263f3cd914c81526280000060608201523060a082015261012080820152600384810194803560e81c0101905b818514612d8f57612d8885828661353e565b9450612d76565b50929392505050565b6003828101925f91813560e81c9091010181612db2613747565b60408051610120810182525f60208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e08101919091527f0af19d5479e90f25845cea6db89a524bb4e8da3a698213efb1b85e10a5e8be9c815267ffffffffffffffff43166101008201529091505b828614612e4157612e3a86828488613791565b9550612e27565b5093949350505050565b5f80612e55613747565b604080516101a0810182525f80825260208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e0810182905261010081018290526101208101829052610140810182905261016081018290526101808101919091526003868101969293509091803560e81c01015b808614612e4157612ee386838588613975565b9550612ed0565b6040805163ffffffff8316602481028201909252805f5b838110156130ff5760448102602086901c01803560601c6014820135608090811c906034840135901c5f612f4284612f398486615875565b60049190613b49565b90508015612f87576040517fcc67af530000000000000000000000000000000000000000000000000000000081526001600160a01b038516600482015260240161170b565b81156130e1576001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001663a5841194856040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b1681526001600160a01b0390911660048201526024015f604051808303815f87803b158015613013575f80fd5b505af1158015613025573d5f803e3d5ffd5b5061305e925050506001600160a01b0385167f0000000000000000000000000000000000000000000000000000000000000000846127e2565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166311da60b46040518163ffffffff1660e01b81526004016020604051808303815f875af11580156130bb573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906130df91906157e7565b505b6130eb8588613b7f565b505050602493909301925050600101612f01565b506024830282205f5260205fa050505050565b8260020b8260020b1315613156578260020b61313a828460020b613b8890919063ffffffff16565b60020b131561315157613151868587868686613b99565b613193565b8260020b8260020b1215613193575f600284900b828107919091129082900503810260020b8260020b121561319357613193868587868686613c18565b505050505050565b60408051808201909152606081525f602082015261221c6001600160a01b0383165f61217f565b6132146131e8845f015183815181106131dd576131dd615791565b602002602001015190565b837fffffffffffffffffffffffffffffffffffffffffffffffffffffff00000000009081169116141590565b1561324b576040517f23f69dc200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8251515f9061325c90600190615b0b565b9050808210156132a457835180518290811061327a5761327a615791565b6020026020010151845f0151838151811061329757613297615791565b6020026020010181815250505b612a458482905152565b5f61221c60206001600160a01b0384163b615b1e565b600161ffff821610806132dc5750617fff61ffff8216115b15610515576040517f270815a000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b62030d4062ffffff82161115610515576040517f76a3f95d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b815f526001600160a01b03811660045260245ffd5b6335278d125f526004601cfd5b5f81602052631e2eaeaf5f5260205f6024601c865afa61339e5763535cf94b5f526004601cfd5b50505f51919050565b5f82815260066020908152604080832084845260050190915281206133d56001600160a01b03861682613377565b95945050505050565b5f6020826020026001015f863c50505f517fffffffffffffffffffffffffffffffffffffffffffffffffffffff000000000081169190911402919050565b5f8163ffffffff84161161346b576040517fffc31e710000000000000000000000000000000000000000000000000000000081526004810183905263ffffffff8416602482015260440161170b565b602083901c6044830201611729565b602481013560801c8015611ea657604080517f0b0d9c09000000000000000000000000000000000000000000000000000000008152833560601c600482018190523060248301526044820184905291517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691630b0d9c09916064808301925f92919082900301818387803b158015613519575f80fd5b505af115801561352b573d5f803e3d5ffd5b5061050692506004915083905084613ca3565b6001838101935f919035821a9061355a90859083161515613ccf565b60028501943560f01c6135816135708583613d13565b805160208201516040909201519092565b60020b60808801526001600160a01b039081166040880152166020860190815260a090205f60108801883560801c9098506fffffffffffffffffffffffffffffffff1690505f81156136c1575f61360461097b6001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001686612670565b905061360f83613d73565b60e08a015261363e897f0000000000000000000000000000000000000000000000000000000000000000613dd4565b61367461097b6001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001686612670565b60808a01515f8681526007602052604090209193506136bb919086907f00000000000000000000000000000000000000000000000000000000000000009085908790613112565b506136fa565b6136f761097b6001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001685612670565b90505b5f6137216002871615155f86815260076020526040902060808c01518d9190889087613df1565b60208b0151919b5091506137389060049083613b49565b50989998505050505050505050565b5f61378c613753614139565b60408051604281019091527f19010000000000000000000000000000000000000000000000000000000000008152600281019190915290565b905090565b83355f90811a6001818116151560808781019190915290870135811c60208701526011870135811c60408701526021870135811c6060870181905260418801976031013590911c90811115613812576040517f2bae6c5200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6002878101973560f01c90613841908416151561382f8784613d13565b9060051b602081188201519101519091565b6001600160a01b0390811660c08a01521660a0880152506004821661386757865f613871565b60148701873560601c5b6001600160a01b031660e088015296505f61389d61389188610120902090565b60228801526042872090565b90505f600884166138b7576138b2898361422e565b6138c1565b6138c18983614298565b90995090506138d082826142dc565b60e08801518015820217600285161561390f57836fffffffffffffffffffffffffffffffff16896020018181516139079190615875565b905250613937565b836fffffffffffffffffffffffffffffffff16896040018181516139339190615b0b565b9052505b61394f828a60a001518b602001518c60800151614304565b613967818a60c001518b604001518c60800151614374565b509798975050505050505050565b5f8061398185876143d2565b60028201975091505f9081903560f01c6139aa6008851615156139a48884613d13565b906144b2565b6001600160a01b039182166101008c0152911660e08a01529250505060208701873560a08801819052909750811015613a0f576040517f8e1edfa400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60028216613a1e57865f613a28565b60148701873560601c5b6001600160a01b031661012088015296505f613a488860048516156144f4565b6101408a01529098509050613a5e8789856145d6565b97505f80613a6e898b878761461e565b919b50925090505f613a8f613a838b88614828565b60228b015260428a2090565b90505f60808716613aa957613aa48c8361422e565b613ab3565b613ab38c83614298565b909c5090506010871615613aea57613ad68b610180015164ffffffffff16614848565b613ae5818c61016001516120ff565b613af4565b613af482826142dc565b5f8b610120015190508082821502179050613b1a818d6101000151868f60c00151614374565b613b248683614882565b613b38828d60e00151878f60c00151614304565b509a9b9a5050505050505050505050565b6001600160a01b0382165f908152602084905260408120613b77613b6e825c856148ca565b92508183612a27565b509392505050565b60248282375050565b5f8183071291819005919091030290565b63010000008601545b5f613bb86001600160a01b0388168787866148e2565b95509050600285810b9085900b1215613bd15750610ff6565b8015613c12578762ffffff861663010000008110613bf157613bf1615791565b015482038862ffffff871663010000008110613c0f57613c0f615791565b01555b50613ba2565b5f613c2e6001600160a01b03871686868561493c565b94509050600283810b9085900b13613c465750613193565b8015613c90578662ffffff851663010000008110613c6657613c66615791565b0154876301000000015403875f018562ffffff1663010000008110613c8d57613c8d615791565b01555b83613c9a81615b31565b94505050613c18565b6001600160a01b0382165f908152602084905260409020612a45613cc8825c84614974565b8290612a27565b80151560c083015280613cf65773fffd8963efd1fc6a506488495d951d5263988d25613cfd565b6401000276a45b6001600160a01b03166101009092019190915250565b5f8163ffffffff841611613d62576040517ff6601b500000000000000000000000000000000000000000000000000000000081526004810183905263ffffffff8416602482015260440161170b565b5060c08102602083901c0192915050565b5f7f8000000000000000000000000000000000000000000000000000000000000000821115613dce576040517f35278d1200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505f0390565b5f80610144601c85015f855af180610506576040513d5f823e3d81fd5b5f808715613f235760208701968035608090811c9160100135901c811580613e2957506fffffffffffffffffffffffffffffffff8116155b15613e4b57508792506fffffffffffffffffffffffffffffffff169050611da7565b5f613e7f6001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000168961498c565b9050806fffffffffffffffffffffffffffffffff16826fffffffffffffffffffffffffffffffff1614613ede576040517fbecb195c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6fffffffffffffffffffffffffffffffff8116608084901b0463010000008a018054909101905550889350506fffffffffffffffffffffffffffffffff169050611da7565b5f808060038a018a3560e81d909a5090505f60108b018b3560801c909b5090505f806003808e01908e3560e81c8f0101604080516080810182528e815260028e810b60208301528d810b9282018390525f606083018190529496509294508f939290919088900b1315613fa257613f9d83888789856149b0565b613faf565b613faf8388878985614af1565b6fffffffffffffffffffffffffffffffff81167fffffffffffffffffffffffffffffffff000000000000000000000000000000008535908116919091049092019c509a5060109092019650925060801c6140098184615875565b92506140158686614c33565b81515f9061404d906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169061498c565b9050806fffffffffffffffffffffffffffffffff168a6fffffffffffffffffffffffffffffffff16146140c8576040517f6429cfd20000000000000000000000000000000000000000000000000000000081526fffffffffffffffffffffffffffffffff808c1660048301528216602482015260440161170b565b606083810151601489019835821c911c8114614110576040517fbecb195c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505050630100000090920180549098019097555090965093945050505050965096945050505050565b7f00000000000000000000000000000000000000000000000000000000000000007f000000000000000000000000000000000000000000000000000000000000000030147f000000000000000000000000000000000000000000000000000000000000000046141661136b5750604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f81527f000000000000000000000000000000000000000000000000000000000000000060208201527f00000000000000000000000000000000000000000000000000000000000000009181019190915246606082015230608082015260a0902090565b6017601483013560e81c8084018201935f92813560601c9291019061425583868484614c6c565b61428b576040517f8baa579f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b85935050505b9250929050565b5f806040518381525f6020820152604185603f8301376041850194506020600160808360015afa519150503d6142d557638baa579f5f526004601cfd5b9293915050565b80601452815f5260345f20805c156142fb57638a2ef1165f526004601cfd5b6001815d505050565b8161431160048583613ca3565b8115614358576001600160a01b038085165f9081526005602090815260408083209389168352929052908120805483929061434d908490615b0b565b9091555061436d9050565b61436d6001600160a01b038516863084612938565b5050505050565b8161438160048583613b49565b5081156143be576001600160a01b038085165f9081526005602090815260408083209389168352929052908120805483929061434d908490615875565b61436d6001600160a01b03851686836127e2565b60018101905f9035811a600483603c86013760049290920191602081161561444b5760108116614422577f6ee89dee573705c024a086e19a128ee0a5ee0547e3283adfa72fbe336a4c4b6c614444565b7f6be5f22bdcd037f6f35250c32e478fad62195ac2bbab1e2932f8c97af926b4915b845261449e565b60108116614479577f022e170cdf338f45bc718f58d29bfafbf3956c2f9ea8d19ccc9b72e42dbbb7b061449b565b7fb0617b84f694c245e54fb8032ebdc9f56eb26ea2c1b65a46c58f50dbd516e2865b84525b60018116151560c094909401939093525091565b600581901b6020811883015190830180516080909101516060850151620f4240908103906144e08284615b8d565b6144ea9190615b1e565b9150509250925092565b5f807fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470836145cc57843560e81c60038601955060405160146064038101828101604052828882378290206050828101517f7407905c0000000000000000000000000000000000000000000000000000000084526040602485018190527fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffec860160448601529985019960c09490941b77ffffffffffffffffffffffffffffffffffffffff000000009190941c1693019290921717925090505b8492509250925092565b5f6010821615614604576008836101788601376008929092019160058361019b860137600583019250614616565b67ffffffffffffffff43166101608501525b509092915050565b5f80808060208616156146d057508535608090811c604089018190526010880135821c60608a0181905260308901986020013590921c918183101561468f576040517fc4daf00300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b808311156146c9576040517f4418233100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50506146fb565b5060108601953560801c604086166146e8575f6146eb565b60015b60ff166040890152606088018190525b60208701966010810135608090811c9135901c80821115614748576040517f668fef1b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6fffffffffffffffffffffffffffffffff1660808a015260088716156147c3576fffffffffffffffffffffffffffffffff811660608816156147a25782945061479b6147948683614cb1565b8890614cbc565b93506147bd565b91925082916147ba6147b48885614cc7565b82614cd2565b94505b5061481a565b6fffffffffffffffffffffffffffffffff811660608816156147fe5791935083916147f76147f18885614cbc565b82614cb1565b9350614818565b82935061481561480e8583614cd2565b8890614cc7565b94505b505b509597919650945092505050565b5f806010831661483a5761018061483e565b6101a05b9093209392505050565b80421115610515576040517f203d82d800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8115611ea65763ffffffff82168260c01c8260048201528360201c60205f84845f855af1925050506324a2e44b5f5114601f3d111681166105065763f959fdae5f526004601cfd5b8082038281131561221c5763c9654ed45f526004601cfd5b5f8080806148fc6128458688078313878905036001615ba4565b909250905061491f816149196001600160a01b038b168a866133a7565b90614cdd565b909450905061492f828287614d9f565b9250505094509492505050565b5f808080614951858707821386880503612845565b909250905061491f8161496e6001600160a01b038b168a866133a7565b90614dc9565b8181018281121561221c5763c9654ed45f526004601cfd5b5f8181526006602052604081205f6133d56001600160a01b03861660038401613377565b5f80808060018180805b8315614a7f5760108b019a3560801c6149d38185615875565b93506fffffffffffffffffffffffffffffffff8b16608082901b0483019250828e8e62ffffff1663010000008110614a0d57614a0d615791565b015f82825401925050819055505f614a5c8b5f01518f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316614e919092919063ffffffff16565b915050614a698c82614ef6565b9b508d6013528b601052825f5260335f20925050505b885160208a0151614abc916001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016918f90614f10565b809d508195505050886040015160020b8c60020b136149ba5760609890980197909752979a9799509497509495945050505050565b5f80808060018180805b8315614bc05760108b019a3560801c614b148185615875565b93506fffffffffffffffffffffffffffffffff8b16608082901b0483019250828e8e62ffffff1663010000008110614b4e57614b4e615791565b015f82825401925050819055505f614b9d8b5f01518f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316614e919092919063ffffffff16565b915050614baa8c82614f2a565b9b508d6013528b601052825f5260335f20925050505b885160208a0151614bfd916001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016918f906148e2565b809d508195505050886040015160020b8c60020b1315614afb5760609890980197909752979a9799509497509495945050505050565b808214611ea6576040517f01842f8c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f604051631626ba7e60e01b80825285600483015260248201604081528460448401528486606485013760208160648701858b5afa9051909114169695505050505050565b5f6117298284615b0b565b5f6117298284614f44565b5f6117298284614f66565b5f6117298284615875565b5f805f614d788460ff1686901c7e1f0d1e100c1d070f090b19131c1706010e11080a1a141802121b150316040581196001019091166101e07f804040554300526644320000502061067405302602000010750620017611707760fc7fb6db6db6ddddddddd34d34d349249249210842108c6318c639ce739cffffffff840260f81c161b60f71c1690811c63d76453e004601f169190911a1790565b9050806101001415925082614d8e5760ff614d95565b8360ff1681015b9150509250929050565b5f8160ff8416614db5600187900b610100615be5565b614dbf9190615ba4565b6122aa9190615be5565b5f805f8360ff0390505f614e6a8260ff1687901b7f0706060506020504060203020504030106050205030304010505030400000000601f6f8421084210842108cc6318c6db6d54be831560081b6fffffffffffffffffffffffffffffffff851160071b1784811c67ffffffffffffffff1060061b1784811c63ffffffff1060051b1784811c61ffff1060041b1784811c60ff1060031b1793841c1c161a1790565b9050806101001415935083614e7f575f614e86565b8160ff1681035b925050509250929050565b5f806006602052835f52600460405f2001602052825f5260405f20602052631e2eaeaf5f5260205f6024601c885afa614ed15763535cf94b5f526004601cfd5b50505f516fffffffffffffffffffffffffffffffff81169460809190911d9350915050565b808203608081901c1561221c5763c9654ed45f526004601cfd5b5f80808061495161284560018789078413888a05036159bd565b818101608081901c1561221c5763c9654ed45f526004601cfd5b5f6b033b2e3c9fd0803ce8000000614f5c8385615b8d565b6117299190615b1e565b5f611729836b033b2e3c9fd0803ce80000008482820283158482048414178202614f975763ad251c275f526004601cfd5b81810615159190040192915050565b5f8083601f840112614fb6575f80fd5b50813567ffffffffffffffff811115614fcd575f80fd5b602083019150836020828501011115614291575f80fd5b5f8060208385031215614ff5575f80fd5b823567ffffffffffffffff81111561500b575f80fd5b61501785828601614fa6565b90969095509350505050565b5f60208284031215615033575f80fd5b813567ffffffffffffffff81168114611729575f80fd5b6001600160a01b0381168114610515575f80fd5b803562ffffff81168114615070575f80fd5b919050565b5f805f805f8060c0878903121561508a575f80fd5b86356150958161504a565b955060208701356150a58161504a565b9450604087013561ffff811681146150bb575f80fd5b93506150c96060880161505e565b92506150d76080880161505e565b91506150e560a0880161505e565b90509295509295509295565b5f805f60408486031215615103575f80fd5b833561510e8161504a565b9250602084013567ffffffffffffffff811115615129575f80fd5b61513586828701614fa6565b9497909650939450505050565b5f60a08284031215615152575f80fd5b50919050565b5f805f805f85870361016081121561516e575f80fd5b86356151798161504a565b95506151888860208901615142565b945060807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff40820112156151b9575f80fd5b5060c08601925061014086013567ffffffffffffffff8111156151da575f80fd5b6151e688828901614fa6565b969995985093965092949392505050565b5f8060408385031215615208575f80fd5b82356152138161504a565b946020939093013593505050565b5f805f60408486031215615233575f80fd5b833561523e8161504a565b9250602084013567ffffffffffffffff811115615259575f80fd5b8401601f81018613615269575f80fd5b803567ffffffffffffffff81111561527f575f80fd5b86602060a083028401011115615293575f80fd5b939660209190910195509293505050565b5f60608284031215615152575f80fd5b5f805f805f61014086880312156152c9575f80fd5b85356152d48161504a565b94506152e38760208801615142565b93506152f28760c088016152a4565b925061012086013567ffffffffffffffff8111156151da575f80fd5b5f6020828403121561531e575f80fd5b5035919050565b5f805f60608486031215615337575f80fd5b83356153428161504a565b925060208401356153528161504a565b929592945050506040919091013590565b5f81518084528060208401602086015e5f6020828601015260207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011685010191505092915050565b7fff000000000000000000000000000000000000000000000000000000000000008816815260e060208201525f6153e960e0830189615363565b82810360408401526153fb8189615363565b606084018890526001600160a01b038716608085015260a0840186905283810360c0850152845180825260208087019350909101905f5b81811015615450578351835260209384019390920191600101615432565b50909b9a5050505050505050505050565b5f805f8060808587031215615474575f80fd5b843561547f8161504a565b9350602085013561548f8161504a565b92506040850135915060608501356154a68161504a565b939692955090935050565b602081525f6117296020830184615363565b5f602082840312156154d3575f80fd5b81356117298161504a565b5f805f805f8061016087890312156154f4575f80fd5b86356154ff8161504a565b955061550e8860208901615142565b945061551d8860c089016152a4565b9350610120870135925061014087013567ffffffffffffffff811115615541575f80fd5b61554d89828a01614fa6565b979a9699509497509295939492505050565b5f8060208385031215615570575f80fd5b823567ffffffffffffffff811115615586575f80fd5b8301601f81018513615596575f80fd5b803567ffffffffffffffff8111156155ac575f80fd5b8560208260051b84010111156155c0575f80fd5b6020919091019590945092505050565b80357fffffffffffffffffffffffffffffffffffffffffffffffffffffff000000000081168114615070575f80fd5b5f805f60608486031215615611575f80fd5b615342846155d0565b81835281816020850137505f602082840101525f60207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f840116840101905092915050565b602081525f6122aa60208301848661561a565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b5f602082840312156156b1575f80fd5b815167ffffffffffffffff8111156156c7575f80fd5b8201601f810184136156d7575f80fd5b805167ffffffffffffffff8111156156f1576156f1615674565b6040517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0603f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f8501160116810181811067ffffffffffffffff8211171561575d5761575d615674565b604052818152828201602001861015615774575f80fd5b8160208401602083015e5f91810160200191909152949350505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b8060020b8114610515575f80fd5b5f602082840312156157dc575f80fd5b8135611729816157be565b5f602082840312156157f7575f80fd5b5051919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b6fffffffffffffffffffffffffffffffff818116838216019081111561221c5761221c6157fe565b6001600160a01b0384168152604060208201525f6133d560408301848661561a565b8082018082111561221c5761221c6157fe565b5f60208284031215615898575f80fd5b6117298261505e565b5f602082840312156158b1575f80fd5b611729826155d0565b5f80858511156158c8575f80fd5b838611156158d4575f80fd5b5050820193919092039150565b80357fffffffffffffffffffffffffffffffffffffffff0000000000000000000000008116906014841015615940577fffffffffffffffffffffffffffffffffffffffff000000000000000000000000808560140360031b1b82161691505b5092915050565b5f60208284031215615957575f80fd5b8151611729816157be565b5f60208284031215615972575f80fd5b81358015158114611729575f80fd5b5f81600f0b7fffffffffffffffffffffffffffffffff8000000000000000000000000000000081036159b5576159b56157fe565b5f0392915050565b600282810b9082900b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8000008112627fffff8213171561221c5761221c6157fe565b5f82600f0b82600f0b0280600f0b9150808214615940576159406157fe565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f81600f0b83600f0b80615a6057615a60615a1d565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81147fffffffffffffffffffffffffffffffff8000000000000000000000000000000083141615615ab457615ab46157fe565b90059392505050565b600f82810b9082900b037fffffffffffffffffffffffffffffffff8000000000000000000000000000000081126f7fffffffffffffffffffffffffffffff8213171561221c5761221c6157fe565b8181038181111561221c5761221c6157fe565b5f82615b2c57615b2c615a1d565b500490565b5f8160020b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8000008103615b6557615b656157fe565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0192915050565b808202811582820484141761221c5761221c6157fe565b600281810b9083900b01627fffff81137fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8000008212171561221c5761221c6157fe565b5f8260020b8260020b028060020b9150808214615940576159406157fe56fea164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\x01\x84W_5`\xE0\x1C\x80c|\xF9\x80\x81\x11a\0\xDDW\x80c\xB4{/\xB1\x11a\0\x88W\x80c\xD9\xE1\x7F\x98\x11a\0cW\x80c\xD9\xE1\x7F\x98\x14a\x03\xF2W\x80c\xDDML\xF6\x14a\x04\x05W\x80c\xF3\xFE\xF3\xA3\x14a\x04\x18W_\x80\xFD[\x80c\xB4{/\xB1\x14a\x03~W\x80c\xD6\xCF\xFD\x1E\x14a\x03\xCCW\x80c\xD9\xCA\xED\x12\x14a\x03\xDFW_\x80\xFD[\x80c\x85\x87\xF4P\x11a\0\xB8W\x80c\x85\x87\xF4P\x14a\x038W\x80c\x91\xDDsF\x14a\x03KW\x80c\x92\xEE\xFE\x9B\x14a\x03kW_\x80\xFD[\x80c|\xF9\x80\x81\x14a\x02\xE9W\x80c\x83@\xF5I\x14a\x03\nW\x80c\x84\xB0\x19n\x14a\x03\x1DW_\x80\xFD[\x80c%\x99\x82\xE5\x11a\x01=W\x80cS\xB4\x1CU\x11a\x01\x18W\x80cS\xB4\x1CU\x14a\x02XW\x80cW^$\xB4\x14a\x02kW\x80ct\x07\x90\\\x14a\x02\xC1W_\x80\xFD[\x80c%\x99\x82\xE5\x14a\x02\x1FW\x80c3\x83\x0EH\x14a\x022W\x80cG\xE7\xEF$\x14a\x02EW_\x80\xFD[\x80c\x13\x87\x14e\x11a\x01mW\x80c\x13\x87\x14e\x14a\x01\xB0W\x80c\x18(\xE0\xE7\x14a\x01\xC3W\x80c!\xD0\xEEp\x14a\x01\xD6W_\x80\xFD[\x80c\t\xC5\xEA\xBE\x14a\x01\x88W\x80c\x11jUP\x14a\x01\x9DW[_\x80\xFD[a\x01\x9Ba\x01\x966`\x04aO\xE4V[a\x04+V[\0[a\x01\x9Ba\x01\xAB6`\x04aP#V[a\x05\x0BV[a\x01\x9Ba\x01\xBE6`\x04aPuV[a\x05\x18V[a\x01\x9Ba\x01\xD16`\x04aP\xF1V[a\x07kV[a\x01\xE9a\x01\xE46`\x04aQXV[a\x08\xCCV[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xE9a\x02-6`\x04aQXV[a\x0B\xE8V[a\x01\x9Ba\x02@6`\x04aP\xF1V[a\x0F\\V[a\x01\x9Ba\x02S6`\x04aQ\xF7V[a\x0F\xFFV[a\x01\x9Ba\x02f6`\x04aR!V[a\x10OV[a\x02~a\x02y6`\x04aR\xB4V[a\x12\x80V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x94\x16\x84R` \x84\x01\x92\x90\x92Rb\xFF\xFF\xFF\x16\x90\x82\x01R``\x01a\x02\x16V[a\x02\xD4a\x02\xCF6`\x04aP\xF1V[a\x14TV[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x16V[a\x02\xFCa\x02\xF76`\x04aS\x0EV[a\x170V[`@Q\x90\x81R` \x01a\x02\x16V[a\x01\x9Ba\x03\x186`\x04aS%V[a\x17:V[a\x03%a\x17\x8FV[`@Qa\x02\x16\x97\x96\x95\x94\x93\x92\x91\x90aS\xAFV[a\x01\x9Ba\x03F6`\x04aTaV[a\x187V[a\x03^a\x03Y6`\x04aO\xE4V[a\x19\xBFV[`@Qa\x02\x16\x91\x90aT\xB1V[a\x01\x9Ba\x03y6`\x04aT\xC3V[a\x1ASV[a\x03\x91a\x03\x8C6`\x04aT\xDEV[a\x1A\x94V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x93\x16\x83R`\x0F\x91\x90\x91\x0B` \x83\x01R\x01a\x02\x16V[a\x01\x9Ba\x03\xDA6`\x04aU_V[a\x1D\xB2V[a\x01\x9Ba\x03\xED6`\x04aS%V[a\x1E>V[a\x01\x9Ba\x04\x006`\x04aQ\xF7V[a\x1E\x8AV[a\x01\x9Ba\x04\x136`\x04aU\xFFV[a\x1E\xAAV[a\x01\x9Ba\x04&6`\x04aQ\xF7V[a\x1F\xE0V[a\x043a ,V[_\x81\x90\x03a\x04?WPPV[`@Q\x7FH\xC8\x94\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cH\xC8\x94\x91\x90a\x04\xA6\x90\x85\x90\x85\x90`\x04\x01aVaV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x04\xC1W=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x05\x06\x91\x90\x81\x01\x90aV\xA1V[PPPV[a\x05\x153\x82a \xFFV[PV[a\x05 a!:V[\x84`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x10a\x05kW`@Q\x7F2\xB4\xBC\x93\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x86\x81R` \x86\x90R`@\x81 `(\x1B`\x03T\x90\x91P_\x90a\x05\xA3\x90h\x01\0\0\0\0\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01a!\x7FV[\x80QQ\x90\x91P_\x90[\x80\x82\x10\x15a\x06TW_\x83_\x01Q\x83\x81Q\x81\x10a\x05\xCAWa\x05\xCAaW\x91V[` \x02` \x01\x01Q\x90Pa\x05\xE4a\x05\xDE\x82\x90V[\x86a!\xF3V[\x15a\x06HWa\x06\"\x88a\x06\x1C\x8B\x87_\x01Q\x87\x81Q\x81\x10a\x06\x06Wa\x06\x06aW\x91V[` \x02` \x01\x01Qa\"\"\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90a\"LV[\x84Q\x80Q\x85\x90\x81\x10a\x066Wa\x066aW\x91V[` \x02` \x01\x01\x81\x81RPPPa\x06TV[P`\x01\x90\x91\x01\x90a\x05\xACV[\x80\x82\x03a\x06pWa\x06pa\x06i\x85\x8A\x8Aa\"gV[\x84\x90a\"\xB2V[a\x06y\x83a#-V[`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16h\x01\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U`@\x80Q\x80\x82\x01\x82Rb\xFF\xFF\xFF\x80\x89\x16\x82R\x87\x81\x16` \x80\x84\x01\x82\x81R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x8A\x16_\x90\x81R`\x02\x90\x92R\x94\x90 \x92Q\x83T\x94Q\x83\x16c\x01\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\x90\x95\x16\x92\x16\x91\x90\x91\x17\x92\x90\x92\x17\x90Ua\x07Q\x90a#\x9CV[a\x07_\x86b\xFF\xFF\xFF\x16a#\x9CV[PPPPPPPPPPV[a\x07\x80`\x03Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16C\x14\x90V[\x15a\x07\xB7W`@Q\x7F\xD8\xA6\xB8\x9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x01` R`@\x90 T`\xFF\x16a\x08\x08W`@Q\x7F\\\xD2kh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F?%\xE5Qtd\x14\xFF\x93\xF0v\xA7\xDD\x83\x82\x8F\xF575\xB3\x93f\xC7@\x15c~\0O\xCB\x02#_\x90\x81RC` R`@\x81 \x90a\x08?\x82a#\xDBV[\x90Pa\x08M\x85\x82\x86\x86a$\xF1V[a\x08\x83W`@Q\x7F\x8B\xAAW\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08\x8CCa%\xDDV[`\x03\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UPPPPPV[_a\x08\xD5a%\xFAV[_a\x08\xDF\x86a&\\V[\x90P_\x80a\t?\x83\x8Aa\x08\xF5` \x8B\x01\x8BaW\xCCV[a\t\x05`@\x8C\x01` \x8D\x01aW\xCCV[`\x06\x90\x81R`\x03\x91\x90\x91R_\x91\x82R``\x8B\x015`&\x90\x81R`:`\x0C \x90\x83\x90R\x92\x82R` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 \x91V[\x90\x92P\x90P_a\t\x84a\t{`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a&pV[`\xA0\x1C`\x02\x0B\x90V[\x90P_a\t\xBD\x82a\t\x98` \x8C\x01\x8CaW\xCCV[a\t\xA8`@\x8D\x01` \x8E\x01aW\xCCV[_\x89\x81R`\x07` R`@\x90 \x92\x91\x90a&\x90V[\x90P_a\t\xF4`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x87\x86a'\x12V[\x90P_a\n\x18\x86_\x01T\x84\x03\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a'mV[\x90P\x80\x15a\x0B\xB6W`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\xA5\x84\x11\x94a\n\\` \x8F\x01\x8FaT\xC3V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\n\xB2W_\x80\xFD[PZ\xF1\x15\x80\x15a\n\xC4W=_\x80>=_\xFD[PPPPa\x0B\x0F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x8E_\x01` \x81\x01\x90a\n\xFF\x91\x90aT\xC3V[`\x01`\x01`\xA0\x1B\x03\x16\x91\x90a'\xE2V[`@Q\x7F=\xD4Z\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x8E\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c=\xD4Z\xDB\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0B\x8DW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xB1\x91\x90aW\xE7V[P\x82\x86U[P\x7F!\xD0\xEEp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9C\x9BPPPPPPPPPPPPV[_a\x0B\xF1a%\xFAV[_a\x0B\xFB\x86a&\\V[_\x81\x81R`\x07` R`@\x81 \x91\x92P\x80a\x0CBa\t{`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a&pV[\x90P_\x83a\x0CS` \x8B\x01\x8BaW\xCCV[b\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a\x0CkWa\x0CkaW\x91V[\x01T\x90P_\x84a\x0C\x81`@\x8C\x01` \x8D\x01aW\xCCV[b\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a\x0C\x99Wa\x0C\x99aW\x91V[\x01T\x90Pa\x0C\xAA` \x8B\x01\x8BaW\xCCV[`\x02\x0B\x83`\x02\x0B\x12\x15a\x0C\xC1W\x80\x82\x03\x93Pa\x0E\x1CV[`\x02\x83\x90\x0Ba\x0C\xD6`@\x8C\x01` \x8D\x01aW\xCCV[`\x02\x0B\x13a\r\xC4Wa\r4\x86a\x0C\xEF` \x8D\x01\x8DaW\xCCV[\x8D``\x01` \x81\x01\x90a\r\x02\x91\x90aW\xCCV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92\x91\x90a(5V[a\rlWc\x01\0\0\0\x85\x01T\x91P\x81\x85a\rQ` \x8D\x01\x8DaW\xCCV[b\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a\riWa\riaW\x91V[\x01U[a\r\x80\x86a\x0C\xEF`@\x8D\x01` \x8E\x01aW\xCCV[a\r\xBAWPc\x01\0\0\0\x84\x01T\x80\x85a\r\x9F`@\x8D\x01` \x8E\x01aW\xCCV[b\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a\r\xB7Wa\r\xB7aW\x91V[\x01U[\x81\x81\x03\x93Pa\x0E\x1CV[a\r\xD5\x86a\x0C\xEF` \x8D\x01\x8DaW\xCCV[a\x0E\rWc\x01\0\0\0\x85\x01T\x91P\x81\x85a\r\xF2` \x8D\x01\x8DaW\xCCV[b\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a\x0E\nWa\x0E\naW\x91V[\x01U[\x80\x82\x86c\x01\0\0\0\x01T\x03\x03\x93P[P_\x91P\x81\x90Pa\x0E\x7F\x85\x8Ca\x0E5` \x8D\x01\x8DaW\xCCV[a\x0EE`@\x8E\x01` \x8F\x01aW\xCCV[`\x06\x90\x81R`\x03\x91\x90\x91R_\x91\x82R``\x8D\x015`&\x90\x81R`:`\x0C \x90\x83\x90R\x92\x82R` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 \x91V[\x90\x92P\x90P_a\x0E\xB9`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x87\x84a'\x12V[\x90P`@\x8A\x015_a\x0E\xCB\x82\x84aX+V[\x90P\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x03a\x0E\xEEW\x85\x85Ua\x0F)V[_a\x0F#\x86_\x01T\x88\x03\x85o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a(\x8BV[\x87\x03\x86UP[P\x7F%\x99\x82\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9D\x9CPPPPPPPPPPPPPV[a\x0Fda!:V[`@Q\x7F\x87t\x15\xD2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x87t\x15\xD2\x90a\x0F\xCD\x90\x86\x90\x86\x90\x86\x90`\x04\x01aXSV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0F\xE4W_\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\xF6W=_\x80>=_\xFD[PPPPPPPV[a\x10\x14`\x01`\x01`\xA0\x1B\x03\x83\x1630\x84a)8V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 \x80T\x83\x92\x90a\x10F\x90\x84\x90aXuV[\x90\x91UPPPPV[a\x10Wa!:V[`\x03T`\x01`\x01`\xA0\x1B\x03h\x01\0\0\0\0\0\0\0\0\x90\x91\x04\x81\x16\x90\x84\x16\x81\x14a\x10\xACW`@Q\x7F\xF2\x1F\xD9\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03T_\x90a\x10\xD0\x90h\x01\0\0\0\0\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x82a!\x7FV[\x90P_[\x83\x81\x10\x15a\x12IW6\x85\x85\x83\x81\x81\x10a\x10\xEFWa\x10\xEFaW\x91V[\x90P`\xA0\x02\x01\x90Pa\x11)\x81`@\x01` \x81\x01\x90a\x11\r\x91\x90aX\x88V[a\x06\x1Ca\x11 `@\x85\x01` \x86\x01aX\xA1V[\x86\x90\x855a)\x9AV[\x83Q\x80Q\x835\x90\x81\x10a\x11>Wa\x11>aW\x91V[` \x90\x81\x02\x91\x90\x91\x01\x01Ra\x11fa\x11\\`\x80\x83\x01``\x84\x01aX\x88V[b\xFF\xFF\xFF\x16a#\x9CV[a\x11ya\x11\\`\xA0\x83\x01`\x80\x84\x01aX\x88V[`@\x80Q\x80\x82\x01\x90\x91R\x80a\x11\x94`\x80\x84\x01``\x85\x01aX\x88V[b\xFF\xFF\xFF\x16\x81R` \x01a\x11\xAE`\xA0\x84\x01`\x80\x85\x01aX\x88V[b\xFF\xFF\xFF\x16\x90R`\x02_a\x11\xC8`@\x85\x01` \x86\x01aX\xA1V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01_ \x82Q\x81T\x93\x90\x92\x01Qb\xFF\xFF\xFF\x90\x81\x16c\x01\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\x90\x94\x16\x92\x16\x91\x90\x91\x17\x91\x90\x91\x17\x90UP`\x01\x01a\x10\xD4V[Pa\x12S\x81a#-V[`\x03`\x08a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPPPPPPV[_\x80_a\x12\x8Ba%\xFAV[a\x12\xA0`\x03Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16C\x14\x90V[a\x13WW`\x14\x84\x10\x15a\x13\x19W_\x84\x90\x03a\x12\xE7W`@Q\x7F\x1E\x81\x07\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x7FI&\x89\x8B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x13'`\x14\x82\x87\x89aX\xBAV[a\x130\x91aX\xE1V[``\x1C\x90P6_a\x13D\x87`\x14\x81\x8BaX\xBAV[\x91P\x91Pa\x13S\x83\x83\x83a\x07kV[PPP[_a\x13\x92a\x13na\x13k` \x8B\x01\x8BaT\xC3V[\x90V[a\x13\x81a\x13k`@\x8C\x01` \x8D\x01aT\xC3V[_\x91\x82R` R`@\x90 `(\x1B\x90V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x81\x16_\x90\x81R`\x02` R`@\x81 Tb\xFF\xFF\xFF\x16b@\0\0\x17\x93P\x90\x91Pa\x13\xDB\x89a&\\V[\x90Pa\x14!a\x14\x16a\t{`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84a&pV[`\x08\x90`\x02\x0Ba*'V[P\x7FW^$\xB4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x93P_\x92PP\x95P\x95P\x95\x92PPPV[_`\x01\x83\x01\x835\x82\x1A\x80a\x15QW`@\x80Q\x7F\xD5\x05\xAC\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x88\x16`\x04\x82\x01R3`$\x82\x01R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`D\x82\x01R`\x14\x84\x015`\xD8\x1C`d\x82\x01\x81\x90R`\x19\x85\x015_\x90\x81\x1A`\x84\x84\x01\x81\x90R`\x1A\x87\x015`\xA4\x85\x01\x81\x90R`:\x88\x015`\xC4\x86\x01\x81\x90R\x95Q`Z\x89\x01\x985``\x1C\x96\x94\x95\x92\x94\x91\x93\x91\x92\x87\x92c\xD5\x05\xAC\xCF\x92`\xE4\x80\x84\x01\x93\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a\x151W_\x80\xFD[PZ\xF1\x15\x80\x15a\x15CW=_\x80>=_\xFD[PPPPPPPPPa\x17\x14V[`\x01\x81`\xFF\x16\x03a\x16,W`@Q\x7F\xD5\x05\xAC\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x04\x82\x01R3`$\x82\x81\x01\x91\x90\x91R`\x14\x84\x015`\x80\x1C`D\x83\x01\x81\x90R\x90\x84\x015`\xD8\x1C`d\x83\x01\x81\x90R`)\x85\x015_\x1A`\x84\x84\x01\x81\x90R`*\x86\x015`\xA4\x85\x01\x81\x90R`J\x87\x015`\xC4\x86\x01\x81\x90R`j\x88\x01\x975``\x1C\x95\x86\x90c\xD5\x05\xAC\xCF\x90`\xE4\x01[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x16\x0BW_\x80\xFD[PZ\xF1\x15\x80\x15a\x16\x1DW=_\x80>=_\xFD[PPPPPPPPPPa\x17\x14V[`\x02\x81`\xFF\x16\x03a\x16\xD8W`@Q\x7F\x8F\xCB\xAF\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x04\x82\x01R3`$\x82\x01R`\x14\x83\x015`\xE0\x1C`D\x82\x01\x81\x90R`\x18\x84\x015`\xD8\x1C`d\x83\x01\x81\x90R`\x01`\x84\x84\x01R`\x1D\x85\x015_\x1A`\xA4\x84\x01\x81\x90R`\x1E\x86\x015`\xC4\x85\x01\x81\x90R`>\x87\x015`\xE4\x86\x01\x81\x90R`^\x88\x01\x975``\x1C\x95\x86\x90c\x8F\xCB\xAF\x0C\x90a\x01\x04\x01a\x15\xF4V[`@Q\x7Fo\x1D\x15\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\xFF\x82\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[a\x17\x1F\x82\x86\x86a*.V[c$\xA2\xE4K\x92PPP[\x93\x92PPPV[_\x81T_R` _\xF3[a\x17O`\x01`\x01`\xA0\x1B\x03\x84\x1630\x84a)8V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a\x17\x85\x90\x84\x90aXuV[\x90\x91UPPPPPV[\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x80_\x80\x80\x83a\x18%`@\x80Q\x80\x82\x01\x82R`\x08\x81R\x7FAngstrom\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R`\x02\x83R\x7Fv1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x83\x01R\x91V[\x97\x98\x90\x97\x96PF\x95P0\x94P\x91\x92P\x90V[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11\x15a\x18UW\x91\x92\x91[_\x84\x81R` \x84\x90R`@\x81 `(\x1B`\x03T\x90\x91P_\x90a\x18\x8D\x90h\x01\0\0\0\0\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x83\x86a*KV[P\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cbv\xCB\xBE`@Q\x80`\xA0\x01`@R\x80a\x18\xD1\x8A\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x88`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82Rb\x80\0\0` \x80\x84\x01\x91\x90\x91R`\x02\x87\x81\x0B`@\x80\x86\x01\x91\x90\x91R0``\x95\x86\x01R\x80Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x89\x90\x1B\x16\x81R\x86Q\x85\x16`\x04\x82\x01R\x92\x86\x01Q\x84\x16`$\x84\x01R\x85\x01Qb\xFF\xFF\xFF\x16`D\x83\x01R\x92\x84\x01Q\x90\x92\x0B`d\x83\x01R`\x80\x90\x92\x01Q\x82\x16`\x84\x82\x01R\x90\x86\x16`\xA4\x82\x01R`\xC4\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x19\x9BW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xF6\x91\x90aYGV[``a\x19\xC9a%\xFAV[\x82_a\x19\xD4\x82a*\xB5V[`\x03T\x91\x93P\x91P_\x90a\x1A\0\x90\x84\x90\x84\x90h\x01\0\0\0\0\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16a+lV[\x90\x93P\x90Pa\x1A\x0E\x82a,\xE1V[a\x1A\x18\x83\x82a-\x0CV[\x92Pa\x1A$\x83\x82a-\x98V[\x92Pa\x1A0\x83\x82a.KV[\x92Pa\x1A=\x83\x87\x87a*.V[a\x1AF\x82a.\xEAV[` _R_` R`@_\xF3[a\x1A[a!:V[_\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[_\x80a\x1A\x9Ea%\xFAV[_\x80a\x1A\xC6a\x1A\xB3a\x13k` \x8C\x01\x8CaT\xC3V[a\x13\x81a\x13k`@\x8D\x01` \x8E\x01aT\xC3V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x81\x16_\x90\x81R`\x02` \x90\x81R`@\x82 T\x92\x93Pc\x01\0\0\0\x90\x92\x04b\xFF\xFF\xFF\x16\x91\x8A\x01\x805\x82\x13\x91\x90a\x1B\x1C\x90\x8CaYbV[\x15\x15\x82\x15\x15\x03a\x1B5Wa\x1B0\x8A`\x0F\x0B\x90V[a\x1B?V[a\x1B?\x8A`\x80\x1D\x90V[\x90P_\x80\x82`\x0F\x0B\x12a\x1BRW\x81a\x1B[V[a\x1B[\x82aY\x81V[\x90P\x82a\x1B\x99W\x80a\x1Bp\x85b\x0FB@aY\xBDV[`\x02\x0Ba\x1B\x80b\x0FB@\x84aY\xFEV[a\x1B\x8A\x91\x90aZJV[a\x1B\x94\x91\x90aZ\xBDV[a\x1B\xB5V[b\x0FB@a\x1B\xAB`\x02\x86\x90\x0B\x83aY\xFEV[a\x1B\xB5\x91\x90aZJV[\x95PP`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90Pc\x15n)\xF6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1CZa\x1C\x1A` \x8F\x01\x8FaYbV[\x15\x15\x85\x15\x15\x03a\x1C<W\x8E` \x01` \x81\x01\x90a\x1C7\x91\x90aT\xC3V[a\x1CNV[\x8E_\x01` \x81\x01\x90a\x1CN\x91\x90aT\xC3V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`D\x82\x01R`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1C\xCDW_\x80\xFD[PZ\xF1\x15\x80\x15a\x1C\xDFW=_\x80>=_\xFD[PPPPPPP_a\x1C\xF0\x89a&\\V[\x90P_a\x1D)a\t{`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84a&pV[\x90Pa\x1D~\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x08\\\x84\x8E``\x01` \x81\x01\x90a\x1Dg\x91\x90aW\xCCV[_\x88\x81R`\x07` R`@\x90 \x94\x93\x92\x91\x90a1\x12V[P\x7F\xB4{/\xB1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x93P\x90\x91PP[\x96P\x96\x94PPPPPV[a\x1D\xBAa!:V[_[\x81\x81\x10\x15a\x05\x06W_\x83\x83\x83\x81\x81\x10a\x1D\xD7Wa\x1D\xD7aW\x91V[\x90P` \x02\x01` \x81\x01\x90a\x1D\xEC\x91\x90aT\xC3V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x01` \x81\x90R`@\x90\x91 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81\x16`\xFF\x90\x91\x16\x15\x17\x90U\x91\x90\x91\x01\x90Pa\x1D\xBCV[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 \x80T\x83\x92\x90a\x1Ep\x90\x84\x90a[\x0BV[\x90\x91UPa\x05\x06\x90P`\x01`\x01`\xA0\x1B\x03\x84\x16\x83\x83a'\xE2V[a\x1E\x92a!:V[a\x1E\xA6`\x01`\x01`\xA0\x1B\x03\x83\x163\x83a'\xE2V[PPV[a\x1E\xB2a!:V[`\x03T`\x01`\x01`\xA0\x1B\x03h\x01\0\0\0\0\0\0\0\0\x90\x91\x04\x81\x16\x90\x83\x16\x81\x14a\x1F\x07W`@Q\x7F\xF2\x1F\xD9\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03T_\x90a\x1F*\x90h\x01\0\0\0\0\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16a1\x9BV[\x90Pa\x1F7\x81\x86\x85a1\xC2V[a\x1F@\x81a#-V[`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16h\x01\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UPPPP\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x16_\x90\x81R`\x02` R`@\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\x16\x90UV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 \x80T\x83\x92\x90a \x12\x90\x84\x90a[\x0BV[\x90\x91UPa\x1E\xA6\x90P`\x01`\x01`\xA0\x1B\x03\x83\x163\x83a'\xE2V[`\x03TCg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x03a sW`@Q\x7F\xD8\xA6\xB8\x9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3_\x90\x81R`\x01` R`@\x90 T`\xFF\x16a \xBBW`@Q\x7F\\\xD2kh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a \xC4Ca%\xDDV[`\x03\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x80`\x0CRc\xDA\xA0P\xE9`\x04R\x81_R`\x1F`\x0C `\x01`\xFF\x83\x16\x1B\x80\x82T\x18\x81\x81\x16a!2Wc\x8C\xB8\x88r_R`\x04`\x1C\xFD[\x90\x91UPPPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a!}W`@Q\x7F#\x01\x9Eg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[`@\x80Q\x80\x82\x01\x90\x91R``\x81R_` \x82\x01R_a!\xA6\x84`\x01`\x01`\xA0\x1B\x03\x16a2\xAEV[\x90P_a!\xB3\x84\x83aXuV[` \x84\x81\x01\x82\x90R`@\x80Q\x83\x83\x02\x81\x01\x83\x01\x90\x91R_\x81R\x80\x86R\x91\x92P\x83\x81\x02\x90\x81\x90`\x01\x90\x84\x01\x89<Pa!\xEA\x84\x84a#\x01V[PPP\x92\x91PPV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x82\x81\x16\x90\x82\x16\x14[\x92\x91PPV[_a\",\x82a2\xC4V[Pe\xFF\xFF\xFF\0\0\0a\xFF\xFF\x91\x82\x16`\x18\x84\x81\x1C\x93\x90\x93\x16\x18\x90\x91\x1B\x16\x18\x90V[_a\"V\x82a3\x13V[Pb\xFF\xFF\xFF\x80\x83\x16\x91\x90\x91\x18\x16\x18\x90V[_a\"q\x83a2\xC4V[a\"z\x82a3\x13V[a\"\xAA\x82a\x06\x1C\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x87\x16\x86a\"\"V[\x94\x93PPPPV[\x81QQ` \x83\x01Q\x81\x03a\"\xF2W`@Q\x7F\\\xEFX:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a#\x06\x83a#\x01\x83`\x01aXuV[\x90QRV[\x81\x83_\x01Q\x82\x81Q\x81\x10a#\x1CWa#\x1CaW\x91V[` \x02` \x01\x01\x81\x81RPPPPPV[\x80Q\x80Qk`\x0B8\x03\x80`\x0B_9_\xF3\0\x82R_\x91\x90`\x0C` \x82\x02\x01`\x14\x83\x01\x84\xF0\x81\x83R\x92P`\x01`\x01`\xA0\x1B\x03\x83\x16a#\x95W`@Q\x7FVp%\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x91\x90PV[b\x0FB@b\xFF\xFF\xFF\x82\x16\x11\x15a\x05\x15Wa\x05\x15\x7F\x14\0!\x13\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0b\xFF\xFF\xFF\x83\x16a3UV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14\x16a$\xCEWP`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91RF``\x82\x01R0`\x80\x82\x01R`\xA0\x90 [g\x19\x01\0\0\0\0\0\0_R\x80`\x1AR\x81`:R`B`\x18 \x90P_`:R\x91\x90PV[_`\x01`\x01`\xA0\x1B\x03\x85\x16\x15a\"\xAAW`@Q\x85;a%\x9AW\x82`@\x81\x14a%!W`A\x81\x14a%aWPa%\xD4V[` \x85\x81\x015`\xFF\x81\x90\x1C`\x1B\x01\x90\x91R\x855`@R\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``Ra%rV[`@\x85\x015_\x1A` R`@\x85`@7[P\x84_R` `\x01`\x80_`\x01Z\xFAQ\x80\x87\x18``\x1B=\x11\x92PP_``R\x80`@Ra%\xD4V[c\x16&\xBA~`\xE0\x1B\x80\x82R\x85`\x04\x83\x01R`$\x82\x01`@\x81R\x84`D\x84\x01R\x84\x86`d\x85\x017` \x81`d\x87\x01\x85\x8BZ\xFA\x90Q\x90\x91\x14\x16\x91P[P\x94\x93PPPPV[_h\x01\0\0\0\0\0\0\0\0\x82\x10a%\xF6Wa%\xF6a3jV[P\x90V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a!}W`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q_\x90`\xA0\x83\x827`\xA0\x90 \x92\x91PPV[_\x81\x81R`\x06` R`@\x81 a\"\xAA`\x01`\x01`\xA0\x1B\x03\x85\x16\x82a3wV[_\x80\x85b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a&\xACWa&\xACaW\x91V[\x01T\x90P_\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a&\xCBWa&\xCBaW\x91V[\x01T\x90P\x84`\x02\x0B\x86`\x02\x0B\x12\x15a&\xE6W\x90\x03\x90Pa\"\xAAV[\x85`\x02\x0B\x84`\x02\x0B\x13a&\xFBW\x03\x90Pa\"\xAAV[c\x01\0\0\0\x87\x01T\x91\x90\x91\x03\x03\x90P\x94\x93PPPPV[_`\x06` R\x82_R`\x06`@_ \x01` R\x81_R`@_ ` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x87Z\xFAa'QWcS\\\xF9K_R`\x04`\x1C\xFD[PP_Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x91PPV[\x81\x81\x02\x81\x83\x82\x04\x14\x83\x15\x17a'\xD9W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x84\t\x81\x81\x10\x82\x01\x90\x03p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x10a'\xCCWc\xC5j\x01Y_R`\x04`\x1C\xFD[`\x80\x91\x82\x1C\x91\x1B\x01a\"\x1CV[`\x80\x1C\x92\x91PPV[\x81`\x14R\x80`4Ro\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0_R` _`D`\x10_\x87Z\xF1\x80`\x01_Q\x14\x16a(+W\x80=\x85;\x15\x17\x10a(+Wc\x90\xB8\xEC\x18_R`\x04`\x1C\xFD[P_`4RPPPV[_\x80\x80a(S\x84\x86\x07\x82\x13\x85\x87\x05\x03[`\x08\x81\x90\x1D\x91`\xFF\x90\x91\x16\x90V[\x90\x92P\x90Pa(\x80\x81a(p`\x01`\x01`\xA0\x1B\x03\x8A\x16\x89\x86a3\xA7V[\x90`\x01`\xFF\x91\x90\x91\x16\x1B\x16\x15\x15\x90V[\x97\x96PPPPPPPV[\x82\x82\x02\x81\x83\x85\x83\x04\x14\x85\x15\x17\x02a)1W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x85\t\x81\x81\x10\x82\x01\x90\x03\x82\x84\x86\t\x83_\x03\x84\x16\x82\x85\x11a(\xE4Wc\xAEG\xF7\x02_R`\x04`\x1C\xFD[\x93\x84\x90\x04\x93\x83\x82\x11\x90\x92\x03_\x83\x90\x03\x83\x90\x04`\x01\x01\x02\x92\x03\x04\x17`\x02`\x03\x83\x02\x81\x18\x80\x84\x02\x82\x03\x02\x80\x84\x02\x82\x03\x02\x80\x84\x02\x82\x03\x02\x80\x84\x02\x82\x03\x02\x80\x84\x02\x82\x03\x02\x80\x84\x02\x90\x91\x03\x02\x02a\x17)V[\x04\x92\x91PPV[`@Q\x81``R\x82`@R\x83``\x1B`,Ro#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0`\x0CR` _`d`\x1C_\x89Z\xF1\x80`\x01_Q\x14\x16a)\x8CW\x80=\x87;\x15\x17\x10a)\x8CWcy9\xF4$_R`\x04`\x1C\xFD[P_``R`@RPPPPV[_\x83_\x01Q\x82\x81Q\x81\x10a)\xB0Wa)\xB0aW\x91V[` \x02` \x01\x01Q\x90Pa)\xF0a)\xC4\x82\x90V[\x84\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x90\x81\x16\x91\x16\x14\x15\x90V[\x15a\x17)W`@Q\x7F#\xF6\x9D\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x82]PPV[\x80\x82\x01\x80\x84\x14a*EWc\x01\x84/\x8C_R`\x04`\x1C\xFD[PPPPV[_\x80\x80a*b`\x01`\x01`\xA0\x1B\x03\x87\x16\x86\x86a3\xDEV[\x90P\x80a*\x9BW`@Q\x7F/e\x9ED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\xFF\xFF`\x18\x82\x90\x1C\x16\x96b\xFF\xFF\xFF\x90\x91\x16\x95P\x93PPPPV[`\x03\x81\x81\x01\x91_\x91\x82\x91\x805`\xE8\x1C\x01\x01\x81`Da*\xD3\x86\x84a[\x0BV[a*\xDD\x91\x90a[\x1EV[\x90P\x80` \x86\x90\x1B\x17\x92P_\x80[\x82\x81\x10\x15a+`W_a+\t` \x87\x90\x1C`D\x84\x02\x01[5``\x1C\x90V[\x90P\x82`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x11a+VW`@Q\x7F\x80\xF1\x1A\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91P`\x01\x01a*\xEBV[P\x82\x94PPPP\x91P\x91V[`\x03\x83\x81\x01\x93_\x91\x82\x91\x82\x91\x82\x91\x805`\xE8\x1C\x01\x01\x81`&a+\x8E\x8A\x84a[\x0BV[a+\x98\x91\x90a[\x1EV[\x90P`@Q\x93P\x80`\xC0\x02\x84\x01\x92P\x82`@R\x80\x84` \x1B\x17\x94PP_[\x82\x84\x10\x15a,\xD4W`\x04\x89\x01\x985`\xE0\x81\x90\x1C\x90_\x90a+\xDE\x90a+\x02\x90\x8C\x90`\xF0\x1Ca4\x1CV[\x90P_a+\xF2a+\x02\x8Ca\xFF\xFF\x86\x16a4\x1CV[\x90P\x83c\xFF\xFF\xFF\xFF\x16\x83c\xFF\xFF\xFF\xFF\x16\x11\x15\x80a,!WP\x80`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x10\x15[\x15a,XW`@Q\x7F\xF3_\x93\x99\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x90\x86R` \x86\x01R`@\x85 `\x02\x8B\x01\x9A\x91\x92P`(\x1B\x905`\xF0\x1C_\x80a,\x8A`\x01`\x01`\xA0\x1B\x03\x8C\x16\x85\x85a*KV[`@\x8A\x01\x91\x90\x91R``\x89\x01RPPP` \x8A\x01\x995\x90P_v\np\xC3\xC4\nd\xE6\xC5\x19\x99\t\x0Be\xF6}\x92@\0\0\0\0\0\0\x82\x90\x04`\x80\x87\x01RP`\xA0\x85\x01R`\xC0\x90\x93\x01\x92a+\xB6V[P\x93PPP\x93P\x93\x91PPV[c\xFF\xFF\xFF\xFF\x81\x16_[\x81\x81\x10\x15a\x05\x06Wa-\x04` \x84\x90\x1C`D\x83\x02\x01a4zV[`\x01\x01a,\xEAV[`@\x80Qa\x01`\x81\x01\x82R_` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01@\x81\x01\x82\x90Rc\xF3\xCD\x91L\x81Rb\x80\0\0``\x82\x01R0`\xA0\x82\x01Ra\x01 \x80\x82\x01R`\x03\x84\x81\x01\x94\x805`\xE8\x1C\x01\x01\x90[\x81\x85\x14a-\x8FWa-\x88\x85\x82\x86a5>V[\x94Pa-vV[P\x92\x93\x92PPPV[`\x03\x82\x81\x01\x92_\x91\x815`\xE8\x1C\x90\x91\x01\x01\x81a-\xB2a7GV[`@\x80Qa\x01 \x81\x01\x82R_` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x91\x90\x91R\x7F\n\xF1\x9DTy\xE9\x0F%\x84\\\xEAm\xB8\x9ARK\xB4\xE8\xDA:i\x82\x13\xEF\xB1\xB8^\x10\xA5\xE8\xBE\x9C\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFC\x16a\x01\0\x82\x01R\x90\x91P[\x82\x86\x14a.AWa.:\x86\x82\x84\x88a7\x91V[\x95Pa.'V[P\x93\x94\x93PPPPV[_\x80a.Ua7GV[`@\x80Qa\x01\xA0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x82\x90Ra\x01@\x81\x01\x82\x90Ra\x01`\x81\x01\x82\x90Ra\x01\x80\x81\x01\x91\x90\x91R`\x03\x86\x81\x01\x96\x92\x93P\x90\x91\x805`\xE8\x1C\x01\x01[\x80\x86\x14a.AWa.\xE3\x86\x83\x85\x88a9uV[\x95Pa.\xD0V[`@\x80Qc\xFF\xFF\xFF\xFF\x83\x16`$\x81\x02\x82\x01\x90\x92R\x80_[\x83\x81\x10\x15a0\xFFW`D\x81\x02` \x86\x90\x1C\x01\x805``\x1C`\x14\x82\x015`\x80\x90\x81\x1C\x90`4\x84\x015\x90\x1C_a/B\x84a/9\x84\x86aXuV[`\x04\x91\x90a;IV[\x90P\x80\x15a/\x87W`@Q\x7F\xCCg\xAFS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x17\x0BV[\x81\x15a0\xE1W`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\xA5\x84\x11\x94\x85`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a0\x13W_\x80\xFD[PZ\xF1\x15\x80\x15a0%W=_\x80>=_\xFD[Pa0^\x92PPP`\x01`\x01`\xA0\x1B\x03\x85\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84a'\xE2V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x11\xDA`\xB4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a0\xBBW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0\xDF\x91\x90aW\xE7V[P[a0\xEB\x85\x88a;\x7FV[PPP`$\x93\x90\x93\x01\x92PP`\x01\x01a/\x01V[P`$\x83\x02\x82 _R` _\xA0PPPPV[\x82`\x02\x0B\x82`\x02\x0B\x13\x15a1VW\x82`\x02\x0Ba1:\x82\x84`\x02\x0Ba;\x88\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x02\x0B\x13\x15a1QWa1Q\x86\x85\x87\x86\x86\x86a;\x99V[a1\x93V[\x82`\x02\x0B\x82`\x02\x0B\x12\x15a1\x93W_`\x02\x84\x90\x0B\x82\x81\x07\x91\x90\x91\x12\x90\x82\x90\x05\x03\x81\x02`\x02\x0B\x82`\x02\x0B\x12\x15a1\x93Wa1\x93\x86\x85\x87\x86\x86\x86a<\x18V[PPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R``\x81R_` \x82\x01Ra\"\x1C`\x01`\x01`\xA0\x1B\x03\x83\x16_a!\x7FV[a2\x14a1\xE8\x84_\x01Q\x83\x81Q\x81\x10a1\xDDWa1\xDDaW\x91V[` \x02` \x01\x01Q\x90V[\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x90\x81\x16\x91\x16\x14\x15\x90V[\x15a2KW`@Q\x7F#\xF6\x9D\xC2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82QQ_\x90a2\\\x90`\x01\x90a[\x0BV[\x90P\x80\x82\x10\x15a2\xA4W\x83Q\x80Q\x82\x90\x81\x10a2zWa2zaW\x91V[` \x02` \x01\x01Q\x84_\x01Q\x83\x81Q\x81\x10a2\x97Wa2\x97aW\x91V[` \x02` \x01\x01\x81\x81RPP[a*E\x84\x82\x90QRV[_a\"\x1C` `\x01`\x01`\xA0\x1B\x03\x84\x16;a[\x1EV[`\x01a\xFF\xFF\x82\x16\x10\x80a2\xDCWPa\x7F\xFFa\xFF\xFF\x82\x16\x11[\x15a\x05\x15W`@Q\x7F'\x08\x15\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[b\x03\r@b\xFF\xFF\xFF\x82\x16\x11\x15a\x05\x15W`@Q\x7Fv\xA3\xF9]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81_R`\x01`\x01`\xA0\x1B\x03\x81\x16`\x04R`$_\xFD[c5'\x8D\x12_R`\x04`\x1C\xFD[_\x81` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x86Z\xFAa3\x9EWcS\\\xF9K_R`\x04`\x1C\xFD[PP_Q\x91\x90PV[_\x82\x81R`\x06` \x90\x81R`@\x80\x83 \x84\x84R`\x05\x01\x90\x91R\x81 a3\xD5`\x01`\x01`\xA0\x1B\x03\x86\x16\x82a3wV[\x95\x94PPPPPV[_` \x82` \x02`\x01\x01_\x86<PP_Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x81\x16\x91\x90\x91\x14\x02\x91\x90PV[_\x81c\xFF\xFF\xFF\xFF\x84\x16\x11a4kW`@Q\x7F\xFF\xC3\x1Eq\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01a\x17\x0BV[` \x83\x90\x1C`D\x83\x02\x01a\x17)V[`$\x81\x015`\x80\x1C\x80\x15a\x1E\xA6W`@\x80Q\x7F\x0B\r\x9C\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x835``\x1C`\x04\x82\x01\x81\x90R0`$\x83\x01R`D\x82\x01\x84\x90R\x91Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91c\x0B\r\x9C\t\x91`d\x80\x83\x01\x92_\x92\x91\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a5\x19W_\x80\xFD[PZ\xF1\x15\x80\x15a5+W=_\x80>=_\xFD[Pa\x05\x06\x92P`\x04\x91P\x83\x90P\x84a<\xA3V[`\x01\x83\x81\x01\x93_\x91\x905\x82\x1A\x90a5Z\x90\x85\x90\x83\x16\x15\x15a<\xCFV[`\x02\x85\x01\x945`\xF0\x1Ca5\x81a5p\x85\x83a=\x13V[\x80Q` \x82\x01Q`@\x90\x92\x01Q\x90\x92V[`\x02\x0B`\x80\x88\x01R`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`@\x88\x01R\x16` \x86\x01\x90\x81R`\xA0\x90 _`\x10\x88\x01\x885`\x80\x1C\x90\x98Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P_\x81\x15a6\xC1W_a6\x04a\t{`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a&pV[\x90Pa6\x0F\x83a=sV[`\xE0\x8A\x01Ra6>\x89\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a=\xD4V[a6ta\t{`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a&pV[`\x80\x8A\x01Q_\x86\x81R`\x07` R`@\x90 \x91\x93Pa6\xBB\x91\x90\x86\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x85\x90\x87\x90a1\x12V[Pa6\xFAV[a6\xF7a\t{`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x85a&pV[\x90P[_a7!`\x02\x87\x16\x15\x15_\x86\x81R`\x07` R`@\x90 `\x80\x8C\x01Q\x8D\x91\x90\x88\x90\x87a=\xF1V[` \x8B\x01Q\x91\x9BP\x91Pa78\x90`\x04\x90\x83a;IV[P\x98\x99\x98PPPPPPPPPV[_a7\x8Ca7SaA9V[`@\x80Q`B\x81\x01\x90\x91R\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x02\x81\x01\x91\x90\x91R\x90V[\x90P\x90V[\x835_\x90\x81\x1A`\x01\x81\x81\x16\x15\x15`\x80\x87\x81\x01\x91\x90\x91R\x90\x87\x015\x81\x1C` \x87\x01R`\x11\x87\x015\x81\x1C`@\x87\x01R`!\x87\x015\x81\x1C``\x87\x01\x81\x90R`A\x88\x01\x97`1\x015\x90\x91\x1C\x90\x81\x11\x15a8\x12W`@Q\x7F+\xAElR\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x87\x81\x01\x975`\xF0\x1C\x90a8A\x90\x84\x16\x15\x15a8/\x87\x84a=\x13V[\x90`\x05\x1B` \x81\x18\x82\x01Q\x91\x01Q\x90\x91V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x8A\x01R\x16`\xA0\x88\x01RP`\x04\x82\x16a8gW\x86_a8qV[`\x14\x87\x01\x875``\x1C[`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x88\x01R\x96P_a8\x9Da8\x91\x88a\x01 \x90 \x90V[`\"\x88\x01R`B\x87 \x90V[\x90P_`\x08\x84\x16a8\xB7Wa8\xB2\x89\x83aB.V[a8\xC1V[a8\xC1\x89\x83aB\x98V[\x90\x99P\x90Pa8\xD0\x82\x82aB\xDCV[`\xE0\x88\x01Q\x80\x15\x82\x02\x17`\x02\x85\x16\x15a9\x0FW\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89` \x01\x81\x81Qa9\x07\x91\x90aXuV[\x90RPa97V[\x83o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89`@\x01\x81\x81Qa93\x91\x90a[\x0BV[\x90RP[a9O\x82\x8A`\xA0\x01Q\x8B` \x01Q\x8C`\x80\x01QaC\x04V[a9g\x81\x8A`\xC0\x01Q\x8B`@\x01Q\x8C`\x80\x01QaCtV[P\x97\x98\x97PPPPPPPPV[_\x80a9\x81\x85\x87aC\xD2V[`\x02\x82\x01\x97P\x91P_\x90\x81\x905`\xF0\x1Ca9\xAA`\x08\x85\x16\x15\x15a9\xA4\x88\x84a=\x13V[\x90aD\xB2V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16a\x01\0\x8C\x01R\x91\x16`\xE0\x8A\x01R\x92PPP` \x87\x01\x875`\xA0\x88\x01\x81\x90R\x90\x97P\x81\x10\x15a:\x0FW`@Q\x7F\x8E\x1E\xDF\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82\x16a:\x1EW\x86_a:(V[`\x14\x87\x01\x875``\x1C[`\x01`\x01`\xA0\x1B\x03\x16a\x01 \x88\x01R\x96P_a:H\x88`\x04\x85\x16\x15aD\xF4V[a\x01@\x8A\x01R\x90\x98P\x90Pa:^\x87\x89\x85aE\xD6V[\x97P_\x80a:n\x89\x8B\x87\x87aF\x1EV[\x91\x9BP\x92P\x90P_a:\x8Fa:\x83\x8B\x88aH(V[`\"\x8B\x01R`B\x8A \x90V[\x90P_`\x80\x87\x16a:\xA9Wa:\xA4\x8C\x83aB.V[a:\xB3V[a:\xB3\x8C\x83aB\x98V[\x90\x9CP\x90P`\x10\x87\x16\x15a:\xEAWa:\xD6\x8Ba\x01\x80\x01Qd\xFF\xFF\xFF\xFF\xFF\x16aHHV[a:\xE5\x81\x8Ca\x01`\x01Qa \xFFV[a:\xF4V[a:\xF4\x82\x82aB\xDCV[_\x8Ba\x01 \x01Q\x90P\x80\x82\x82\x15\x02\x17\x90Pa;\x1A\x81\x8Da\x01\0\x01Q\x86\x8F`\xC0\x01QaCtV[a;$\x86\x83aH\x82V[a;8\x82\x8D`\xE0\x01Q\x87\x8F`\xC0\x01QaC\x04V[P\x9A\x9B\x9APPPPPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R` \x84\x90R`@\x81 a;wa;n\x82\\\x85aH\xCAV[\x92P\x81\x83a*'V[P\x93\x92PPPV[`$\x82\x827PPV[_\x81\x83\x07\x12\x91\x81\x90\x05\x91\x90\x91\x03\x02\x90V[c\x01\0\0\0\x86\x01T[_a;\xB8`\x01`\x01`\xA0\x1B\x03\x88\x16\x87\x87\x86aH\xE2V[\x95P\x90P`\x02\x85\x81\x0B\x90\x85\x90\x0B\x12\x15a;\xD1WPa\x0F\xF6V[\x80\x15a<\x12W\x87b\xFF\xFF\xFF\x86\x16c\x01\0\0\0\x81\x10a;\xF1Wa;\xF1aW\x91V[\x01T\x82\x03\x88b\xFF\xFF\xFF\x87\x16c\x01\0\0\0\x81\x10a<\x0FWa<\x0FaW\x91V[\x01U[Pa;\xA2V[_a<.`\x01`\x01`\xA0\x1B\x03\x87\x16\x86\x86\x85aI<V[\x94P\x90P`\x02\x83\x81\x0B\x90\x85\x90\x0B\x13a<FWPa1\x93V[\x80\x15a<\x90W\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a<fWa<faW\x91V[\x01T\x87c\x01\0\0\0\x01T\x03\x87_\x01\x85b\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a<\x8DWa<\x8DaW\x91V[\x01U[\x83a<\x9A\x81a[1V[\x94PPPa<\x18V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R` \x84\x90R`@\x90 a*Ea<\xC8\x82\\\x84aItV[\x82\x90a*'V[\x80\x15\x15`\xC0\x83\x01R\x80a<\xF6Ws\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D%a<\xFDV[d\x01\0\x02v\xA4[`\x01`\x01`\xA0\x1B\x03\x16a\x01\0\x90\x92\x01\x91\x90\x91RPV[_\x81c\xFF\xFF\xFF\xFF\x84\x16\x11a=bW`@Q\x7F\xF6`\x1BP\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01a\x17\x0BV[P`\xC0\x81\x02` \x83\x90\x1C\x01\x92\x91PPV[_\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x11\x15a=\xCEW`@Q\x7F5'\x8D\x12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P_\x03\x90V[_\x80a\x01D`\x1C\x85\x01_\x85Z\xF1\x80a\x05\x06W`@Q=_\x82>=\x81\xFD[_\x80\x87\x15a?#W` \x87\x01\x96\x805`\x80\x90\x81\x1C\x91`\x10\x015\x90\x1C\x81\x15\x80a>)WPo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15[\x15a>KWP\x87\x92Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90Pa\x1D\xA7V[_a>\x7F`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x89aI\x8CV[\x90P\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a>\xDEW`@Q\x7F\xBE\xCB\x19\\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\x80\x84\x90\x1B\x04c\x01\0\0\0\x8A\x01\x80T\x90\x91\x01\x90UP\x88\x93PPo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90Pa\x1D\xA7V[_\x80\x80`\x03\x8A\x01\x8A5`\xE8\x1D\x90\x9AP\x90P_`\x10\x8B\x01\x8B5`\x80\x1C\x90\x9BP\x90P_\x80`\x03\x80\x8E\x01\x90\x8E5`\xE8\x1C\x8F\x01\x01`@\x80Q`\x80\x81\x01\x82R\x8E\x81R`\x02\x8E\x81\x0B` \x83\x01R\x8D\x81\x0B\x92\x82\x01\x83\x90R_``\x83\x01\x81\x90R\x94\x96P\x92\x94P\x8F\x93\x92\x90\x91\x90\x88\x90\x0B\x13\x15a?\xA2Wa?\x9D\x83\x88\x87\x89\x85aI\xB0V[a?\xAFV[a?\xAF\x83\x88\x87\x89\x85aJ\xF1V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x855\x90\x81\x16\x91\x90\x91\x04\x90\x92\x01\x9CP\x9AP`\x10\x90\x92\x01\x96P\x92P`\x80\x1Ca@\t\x81\x84aXuV[\x92Pa@\x15\x86\x86aL3V[\x81Q_\x90a@M\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90aI\x8CV[\x90P\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8Ao\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a@\xC8W`@Q\x7Fd)\xCF\xD2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x8C\x16`\x04\x83\x01R\x82\x16`$\x82\x01R`D\x01a\x17\x0BV[``\x83\x81\x01Q`\x14\x89\x01\x985\x82\x1C\x91\x1C\x81\x14aA\x10W`@Q\x7F\xBE\xCB\x19\\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPc\x01\0\0\0\x90\x92\x01\x80T\x90\x98\x01\x90\x97UP\x90\x96P\x93\x94PPPPP\x96P\x96\x94PPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14\x16a\x13kWP`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91RF``\x82\x01R0`\x80\x82\x01R`\xA0\x90 \x90V[`\x17`\x14\x83\x015`\xE8\x1C\x80\x84\x01\x82\x01\x93_\x92\x815``\x1C\x92\x91\x01\x90aBU\x83\x86\x84\x84aLlV[aB\x8BW`@Q\x7F\x8B\xAAW\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85\x93PPP[\x92P\x92\x90PV[_\x80`@Q\x83\x81R_` \x82\x01R`A\x85`?\x83\x017`A\x85\x01\x94P` `\x01`\x80\x83`\x01Z\xFAQ\x91PP=aB\xD5Wc\x8B\xAAW\x9F_R`\x04`\x1C\xFD[\x92\x93\x91PPV[\x80`\x14R\x81_R`4_ \x80\\\x15aB\xFBWc\x8A.\xF1\x16_R`\x04`\x1C\xFD[`\x01\x81]PPPV[\x81aC\x11`\x04\x85\x83a<\xA3V[\x81\x15aCXW`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x89\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90aCM\x90\x84\x90a[\x0BV[\x90\x91UPaCm\x90PV[aCm`\x01`\x01`\xA0\x1B\x03\x85\x16\x860\x84a)8V[PPPPPV[\x81aC\x81`\x04\x85\x83a;IV[P\x81\x15aC\xBEW`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x89\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90aCM\x90\x84\x90aXuV[aCm`\x01`\x01`\xA0\x1B\x03\x85\x16\x86\x83a'\xE2V[`\x01\x81\x01\x90_\x905\x81\x1A`\x04\x83`<\x86\x017`\x04\x92\x90\x92\x01\x91` \x81\x16\x15aDKW`\x10\x81\x16aD\"W\x7Fn\xE8\x9D\xEEW7\x05\xC0$\xA0\x86\xE1\x9A\x12\x8E\xE0\xA5\xEE\x05G\xE3(:\xDF\xA7/\xBE3jLKlaDDV[\x7Fk\xE5\xF2+\xDC\xD07\xF6\xF3RP\xC3.G\x8F\xADb\x19Z\xC2\xBB\xAB\x1E)2\xF8\xC9z\xF9&\xB4\x91[\x84RaD\x9EV[`\x10\x81\x16aDyW\x7F\x02.\x17\x0C\xDF3\x8FE\xBCq\x8FX\xD2\x9B\xFA\xFB\xF3\x95l/\x9E\xA8\xD1\x9C\xCC\x9Br\xE4-\xBB\xB7\xB0aD\x9BV[\x7F\xB0a{\x84\xF6\x94\xC2E\xE5O\xB8\x03.\xBD\xC9\xF5n\xB2n\xA2\xC1\xB6ZF\xC5\x8FP\xDB\xD5\x16\xE2\x86[\x84R[`\x01\x81\x16\x15\x15`\xC0\x94\x90\x94\x01\x93\x90\x93RP\x91V[`\x05\x81\x90\x1B` \x81\x18\x83\x01Q\x90\x83\x01\x80Q`\x80\x90\x91\x01Q``\x85\x01Qb\x0FB@\x90\x81\x03\x90aD\xE0\x82\x84a[\x8DV[aD\xEA\x91\x90a[\x1EV[\x91PP\x92P\x92P\x92V[_\x80\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x83aE\xCCW\x845`\xE8\x1C`\x03\x86\x01\x95P`@Q`\x14`d\x03\x81\x01\x82\x81\x01`@R\x82\x88\x827\x82\x90 `P\x82\x81\x01Q\x7Ft\x07\x90\\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`@`$\x85\x01\x81\x90R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xEC\x86\x01`D\x86\x01R\x99\x85\x01\x99`\xC0\x94\x90\x94\x1Bw\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\x91\x90\x94\x1C\x16\x93\x01\x92\x90\x92\x17\x17\x92P\x90P[\x84\x92P\x92P\x92P\x92V[_`\x10\x82\x16\x15aF\x04W`\x08\x83a\x01x\x86\x017`\x08\x92\x90\x92\x01\x91`\x05\x83a\x01\x9B\x86\x017`\x05\x83\x01\x92PaF\x16V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFC\x16a\x01`\x85\x01R[P\x90\x92\x91PPV[_\x80\x80\x80` \x86\x16\x15aF\xD0WP\x855`\x80\x90\x81\x1C`@\x89\x01\x81\x90R`\x10\x88\x015\x82\x1C``\x8A\x01\x81\x90R`0\x89\x01\x98` \x015\x90\x92\x1C\x91\x81\x83\x10\x15aF\x8FW`@Q\x7F\xC4\xDA\xF0\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83\x11\x15aF\xC9W`@Q\x7FD\x18#1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPaF\xFBV[P`\x10\x86\x01\x955`\x80\x1C`@\x86\x16aF\xE8W_aF\xEBV[`\x01[`\xFF\x16`@\x89\x01R``\x88\x01\x81\x90R[` \x87\x01\x96`\x10\x81\x015`\x80\x90\x81\x1C\x915\x90\x1C\x80\x82\x11\x15aGHW`@Q\x7Ff\x8F\xEF\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x8A\x01R`\x08\x87\x16\x15aG\xC3Wo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16``\x88\x16\x15aG\xA2W\x82\x94PaG\x9BaG\x94\x86\x83aL\xB1V[\x88\x90aL\xBCV[\x93PaG\xBDV[\x91\x92P\x82\x91aG\xBAaG\xB4\x88\x85aL\xC7V[\x82aL\xD2V[\x94P[PaH\x1AV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16``\x88\x16\x15aG\xFEW\x91\x93P\x83\x91aG\xF7aG\xF1\x88\x85aL\xBCV[\x82aL\xB1V[\x93PaH\x18V[\x82\x93PaH\x15aH\x0E\x85\x83aL\xD2V[\x88\x90aL\xC7V[\x94P[P[P\x95\x97\x91\x96P\x94P\x92PPPV[_\x80`\x10\x83\x16aH:Wa\x01\x80aH>V[a\x01\xA0[\x90\x93 \x93\x92PPPV[\x80B\x11\x15a\x05\x15W`@Q\x7F =\x82\xD8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x15a\x1E\xA6Wc\xFF\xFF\xFF\xFF\x82\x16\x82`\xC0\x1C\x82`\x04\x82\x01R\x83` \x1C` _\x84\x84_\x85Z\xF1\x92PPPc$\xA2\xE4K_Q\x14`\x1F=\x11\x16\x81\x16a\x05\x06Wc\xF9Y\xFD\xAE_R`\x04`\x1C\xFD[\x80\x82\x03\x82\x81\x13\x15a\"\x1CWc\xC9eN\xD4_R`\x04`\x1C\xFD[_\x80\x80\x80aH\xFCa(E\x86\x88\x07\x83\x13\x87\x89\x05\x03`\x01a[\xA4V[\x90\x92P\x90PaI\x1F\x81aI\x19`\x01`\x01`\xA0\x1B\x03\x8B\x16\x8A\x86a3\xA7V[\x90aL\xDDV[\x90\x94P\x90PaI/\x82\x82\x87aM\x9FV[\x92PPP\x94P\x94\x92PPPV[_\x80\x80\x80aIQ\x85\x87\x07\x82\x13\x86\x88\x05\x03a(EV[\x90\x92P\x90PaI\x1F\x81aIn`\x01`\x01`\xA0\x1B\x03\x8B\x16\x8A\x86a3\xA7V[\x90aM\xC9V[\x81\x81\x01\x82\x81\x12\x15a\"\x1CWc\xC9eN\xD4_R`\x04`\x1C\xFD[_\x81\x81R`\x06` R`@\x81 _a3\xD5`\x01`\x01`\xA0\x1B\x03\x86\x16`\x03\x84\x01a3wV[_\x80\x80\x80`\x01\x81\x80\x80[\x83\x15aJ\x7FW`\x10\x8B\x01\x9A5`\x80\x1CaI\xD3\x81\x85aXuV[\x93Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16`\x80\x82\x90\x1B\x04\x83\x01\x92P\x82\x8E\x8Eb\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10aJ\rWaJ\raW\x91V[\x01_\x82\x82T\x01\x92PP\x81\x90UP_aJ\\\x8B_\x01Q\x8F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16aN\x91\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PPaJi\x8C\x82aN\xF6V[\x9BP\x8D`\x13R\x8B`\x10R\x82_R`3_ \x92PPP[\x88Q` \x8A\x01QaJ\xBC\x91`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x8F\x90aO\x10V[\x80\x9DP\x81\x95PPP\x88`@\x01Q`\x02\x0B\x8C`\x02\x0B\x13aI\xBAW``\x98\x90\x98\x01\x97\x90\x97R\x97\x9A\x97\x99P\x94\x97P\x94\x95\x94PPPPPV[_\x80\x80\x80`\x01\x81\x80\x80[\x83\x15aK\xC0W`\x10\x8B\x01\x9A5`\x80\x1CaK\x14\x81\x85aXuV[\x93Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16`\x80\x82\x90\x1B\x04\x83\x01\x92P\x82\x8E\x8Eb\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10aKNWaKNaW\x91V[\x01_\x82\x82T\x01\x92PP\x81\x90UP_aK\x9D\x8B_\x01Q\x8F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16aN\x91\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PPaK\xAA\x8C\x82aO*V[\x9BP\x8D`\x13R\x8B`\x10R\x82_R`3_ \x92PPP[\x88Q` \x8A\x01QaK\xFD\x91`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x8F\x90aH\xE2V[\x80\x9DP\x81\x95PPP\x88`@\x01Q`\x02\x0B\x8C`\x02\x0B\x13\x15aJ\xFBW``\x98\x90\x98\x01\x97\x90\x97R\x97\x9A\x97\x99P\x94\x97P\x94\x95\x94PPPPPV[\x80\x82\x14a\x1E\xA6W`@Q\x7F\x01\x84/\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`@Qc\x16&\xBA~`\xE0\x1B\x80\x82R\x85`\x04\x83\x01R`$\x82\x01`@\x81R\x84`D\x84\x01R\x84\x86`d\x85\x017` \x81`d\x87\x01\x85\x8BZ\xFA\x90Q\x90\x91\x14\x16\x96\x95PPPPPPV[_a\x17)\x82\x84a[\x0BV[_a\x17)\x82\x84aODV[_a\x17)\x82\x84aOfV[_a\x17)\x82\x84aXuV[_\x80_aMx\x84`\xFF\x16\x86\x90\x1C~\x1F\r\x1E\x10\x0C\x1D\x07\x0F\t\x0B\x19\x13\x1C\x17\x06\x01\x0E\x11\x08\n\x1A\x14\x18\x02\x12\x1B\x15\x03\x16\x04\x05\x81\x19`\x01\x01\x90\x91\x16a\x01\xE0\x7F\x80@@UC\0RfD2\0\0P a\x06t\x050&\x02\0\0\x10u\x06 \x01v\x11pw`\xFC\x7F\xB6\xDBm\xB6\xDD\xDD\xDD\xDD\xD3M4\xD3I$\x92I!\x08B\x10\x8Cc\x18\xC69\xCEs\x9C\xFF\xFF\xFF\xFF\x84\x02`\xF8\x1C\x16\x1B`\xF7\x1C\x16\x90\x81\x1Cc\xD7dS\xE0\x04`\x1F\x16\x91\x90\x91\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x92P\x82aM\x8EW`\xFFaM\x95V[\x83`\xFF\x16\x81\x01[\x91PP\x92P\x92\x90PV[_\x81`\xFF\x84\x16aM\xB5`\x01\x87\x90\x0Ba\x01\0a[\xE5V[aM\xBF\x91\x90a[\xA4V[a\"\xAA\x91\x90a[\xE5V[_\x80_\x83`\xFF\x03\x90P_aNj\x82`\xFF\x16\x87\x90\x1B\x7F\x07\x06\x06\x05\x06\x02\x05\x04\x06\x02\x03\x02\x05\x04\x03\x01\x06\x05\x02\x05\x03\x03\x04\x01\x05\x05\x03\x04\0\0\0\0`\x1Fo\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x83\x15`\x08\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11`\x07\x1B\x17\x84\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x84\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x84\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x84\x81\x1C`\xFF\x10`\x03\x1B\x17\x93\x84\x1C\x1C\x16\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x93P\x83aN\x7FW_aN\x86V[\x81`\xFF\x16\x81\x03[\x92PPP\x92P\x92\x90PV[_\x80`\x06` R\x83_R`\x04`@_ \x01` R\x82_R`@_ ` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x88Z\xFAaN\xD1WcS\\\xF9K_R`\x04`\x1C\xFD[PP_Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x94`\x80\x91\x90\x91\x1D\x93P\x91PPV[\x80\x82\x03`\x80\x81\x90\x1C\x15a\"\x1CWc\xC9eN\xD4_R`\x04`\x1C\xFD[_\x80\x80\x80aIQa(E`\x01\x87\x89\x07\x84\x13\x88\x8A\x05\x03aY\xBDV[\x81\x81\x01`\x80\x81\x90\x1C\x15a\"\x1CWc\xC9eN\xD4_R`\x04`\x1C\xFD[_k\x03;.<\x9F\xD0\x80<\xE8\0\0\0aO\\\x83\x85a[\x8DV[a\x17)\x91\x90a[\x1EV[_a\x17)\x83k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x84\x82\x82\x02\x83\x15\x84\x82\x04\x84\x14\x17\x82\x02aO\x97Wc\xAD%\x1C'_R`\x04`\x1C\xFD[\x81\x81\x06\x15\x15\x91\x90\x04\x01\x92\x91PPV[_\x80\x83`\x1F\x84\x01\x12aO\xB6W_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aO\xCDW_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aB\x91W_\x80\xFD[_\x80` \x83\x85\x03\x12\x15aO\xF5W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aP\x0BW_\x80\xFD[aP\x17\x85\x82\x86\x01aO\xA6V[\x90\x96\x90\x95P\x93PPPPV[_` \x82\x84\x03\x12\x15aP3W_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x17)W_\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\x15W_\x80\xFD[\x805b\xFF\xFF\xFF\x81\x16\x81\x14aPpW_\x80\xFD[\x91\x90PV[_\x80_\x80_\x80`\xC0\x87\x89\x03\x12\x15aP\x8AW_\x80\xFD[\x865aP\x95\x81aPJV[\x95P` \x87\x015aP\xA5\x81aPJV[\x94P`@\x87\x015a\xFF\xFF\x81\x16\x81\x14aP\xBBW_\x80\xFD[\x93PaP\xC9``\x88\x01aP^V[\x92PaP\xD7`\x80\x88\x01aP^V[\x91PaP\xE5`\xA0\x88\x01aP^V[\x90P\x92\x95P\x92\x95P\x92\x95V[_\x80_`@\x84\x86\x03\x12\x15aQ\x03W_\x80\xFD[\x835aQ\x0E\x81aPJV[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aQ)W_\x80\xFD[aQ5\x86\x82\x87\x01aO\xA6V[\x94\x97\x90\x96P\x93\x94PPPPV[_`\xA0\x82\x84\x03\x12\x15aQRW_\x80\xFD[P\x91\x90PV[_\x80_\x80_\x85\x87\x03a\x01`\x81\x12\x15aQnW_\x80\xFD[\x865aQy\x81aPJV[\x95PaQ\x88\x88` \x89\x01aQBV[\x94P`\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF@\x82\x01\x12\x15aQ\xB9W_\x80\xFD[P`\xC0\x86\x01\x92Pa\x01@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aQ\xDAW_\x80\xFD[aQ\xE6\x88\x82\x89\x01aO\xA6V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_\x80`@\x83\x85\x03\x12\x15aR\x08W_\x80\xFD[\x825aR\x13\x81aPJV[\x94` \x93\x90\x93\x015\x93PPPV[_\x80_`@\x84\x86\x03\x12\x15aR3W_\x80\xFD[\x835aR>\x81aPJV[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aRYW_\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13aRiW_\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aR\x7FW_\x80\xFD[\x86` `\xA0\x83\x02\x84\x01\x01\x11\x15aR\x93W_\x80\xFD[\x93\x96` \x91\x90\x91\x01\x95P\x92\x93PPPV[_``\x82\x84\x03\x12\x15aQRW_\x80\xFD[_\x80_\x80_a\x01@\x86\x88\x03\x12\x15aR\xC9W_\x80\xFD[\x855aR\xD4\x81aPJV[\x94PaR\xE3\x87` \x88\x01aQBV[\x93PaR\xF2\x87`\xC0\x88\x01aR\xA4V[\x92Pa\x01 \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aQ\xDAW_\x80\xFD[_` \x82\x84\x03\x12\x15aS\x1EW_\x80\xFD[P5\x91\x90PV[_\x80_``\x84\x86\x03\x12\x15aS7W_\x80\xFD[\x835aSB\x81aPJV[\x92P` \x84\x015aSR\x81aPJV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x16\x81R`\xE0` \x82\x01R_aS\xE9`\xE0\x83\x01\x89aScV[\x82\x81\x03`@\x84\x01RaS\xFB\x81\x89aScV[``\x84\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x84Q\x80\x82R` \x80\x87\x01\x93P\x90\x91\x01\x90_[\x81\x81\x10\x15aTPW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aT2V[P\x90\x9B\x9APPPPPPPPPPPV[_\x80_\x80`\x80\x85\x87\x03\x12\x15aTtW_\x80\xFD[\x845aT\x7F\x81aPJV[\x93P` \x85\x015aT\x8F\x81aPJV[\x92P`@\x85\x015\x91P``\x85\x015aT\xA6\x81aPJV[\x93\x96\x92\x95P\x90\x93PPV[` \x81R_a\x17)` \x83\x01\x84aScV[_` \x82\x84\x03\x12\x15aT\xD3W_\x80\xFD[\x815a\x17)\x81aPJV[_\x80_\x80_\x80a\x01`\x87\x89\x03\x12\x15aT\xF4W_\x80\xFD[\x865aT\xFF\x81aPJV[\x95PaU\x0E\x88` \x89\x01aQBV[\x94PaU\x1D\x88`\xC0\x89\x01aR\xA4V[\x93Pa\x01 \x87\x015\x92Pa\x01@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aUAW_\x80\xFD[aUM\x89\x82\x8A\x01aO\xA6V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[_\x80` \x83\x85\x03\x12\x15aUpW_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aU\x86W_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aU\x96W_\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aU\xACW_\x80\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15aU\xC0W_\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[\x805\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x81\x16\x81\x14aPpW_\x80\xFD[_\x80_``\x84\x86\x03\x12\x15aV\x11W_\x80\xFD[aSB\x84aU\xD0V[\x81\x83R\x81\x81` \x85\x017P_` \x82\x84\x01\x01R_` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[` \x81R_a\"\xAA` \x83\x01\x84\x86aV\x1AV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15aV\xB1W_\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aV\xC7W_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13aV\xD7W_\x80\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aV\xF1WaV\xF1aVtV[`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`?\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x85\x01\x16\x01\x16\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aW]WaW]aVtV[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15aWtW_\x80\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x80`\x02\x0B\x81\x14a\x05\x15W_\x80\xFD[_` \x82\x84\x03\x12\x15aW\xDCW_\x80\xFD[\x815a\x17)\x81aW\xBEV[_` \x82\x84\x03\x12\x15aW\xF7W_\x80\xFD[PQ\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\"\x1CWa\"\x1CaW\xFEV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`@` \x82\x01R_a3\xD5`@\x83\x01\x84\x86aV\x1AV[\x80\x82\x01\x80\x82\x11\x15a\"\x1CWa\"\x1CaW\xFEV[_` \x82\x84\x03\x12\x15aX\x98W_\x80\xFD[a\x17)\x82aP^V[_` \x82\x84\x03\x12\x15aX\xB1W_\x80\xFD[a\x17)\x82aU\xD0V[_\x80\x85\x85\x11\x15aX\xC8W_\x80\xFD[\x83\x86\x11\x15aX\xD4W_\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x805\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90`\x14\x84\x10\x15aY@W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x80\x85`\x14\x03`\x03\x1B\x1B\x82\x16\x16\x91P[P\x92\x91PPV[_` \x82\x84\x03\x12\x15aYWW_\x80\xFD[\x81Qa\x17)\x81aW\xBEV[_` \x82\x84\x03\x12\x15aYrW_\x80\xFD[\x815\x80\x15\x15\x81\x14a\x17)W_\x80\xFD[_\x81`\x0F\x0B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x03aY\xB5WaY\xB5aW\xFEV[_\x03\x92\x91PPV[`\x02\x82\x81\x0B\x90\x82\x90\x0B\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x81\x12b\x7F\xFF\xFF\x82\x13\x17\x15a\"\x1CWa\"\x1CaW\xFEV[_\x82`\x0F\x0B\x82`\x0F\x0B\x02\x80`\x0F\x0B\x91P\x80\x82\x14aY@WaY@aW\xFEV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x81`\x0F\x0B\x83`\x0F\x0B\x80aZ`WaZ`aZ\x1DV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x14\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x14\x16\x15aZ\xB4WaZ\xB4aW\xFEV[\x90\x05\x93\x92PPPV[`\x0F\x82\x81\x0B\x90\x82\x90\x0B\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x12o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x13\x17\x15a\"\x1CWa\"\x1CaW\xFEV[\x81\x81\x03\x81\x81\x11\x15a\"\x1CWa\"\x1CaW\xFEV[_\x82a[,Wa[,aZ\x1DV[P\x04\x90V[_\x81`\x02\x0B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x81\x03a[eWa[eaW\xFEV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\"\x1CWa\"\x1CaW\xFEV[`\x02\x81\x81\x0B\x90\x83\x90\x0B\x01b\x7F\xFF\xFF\x81\x13\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x82\x12\x17\x15a\"\x1CWa\"\x1CaW\xFEV[_\x82`\x02\x0B\x82`\x02\x0B\x02\x80`\x02\x0B\x91P\x80\x82\x14aY@WaY@aW\xFEV\xFE\xA1dsolcC\0\x08\x1A\0\n",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BalanceDelta(alloy::sol_types::private::primitives::aliases::I256);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<BalanceDelta>
        for alloy::sol_types::private::primitives::aliases::I256 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Int<
                256,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Int<256>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        #[automatically_derived]
        impl BalanceDelta {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(
                value: alloy::sol_types::private::primitives::aliases::I256,
            ) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(
                self,
            ) -> alloy::sol_types::private::primitives::aliases::I256 {
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
        impl alloy_sol_types::SolType for BalanceDelta {
            type RustType = alloy::sol_types::private::primitives::aliases::I256;
            type Token<'a> = <alloy::sol_types::sol_data::Int<
                256,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Int<
                256,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Int<
                256,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for BalanceDelta {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BeforeSwapDelta(alloy::sol_types::private::primitives::aliases::I256);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<BeforeSwapDelta>
        for alloy::sol_types::private::primitives::aliases::I256 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Int<
                256,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Int<256>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        #[automatically_derived]
        impl BeforeSwapDelta {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(
                value: alloy::sol_types::private::primitives::aliases::I256,
            ) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(
                self,
            ) -> alloy::sol_types::private::primitives::aliases::I256 {
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
        impl alloy_sol_types::SolType for BeforeSwapDelta {
            type RustType = alloy::sol_types::private::primitives::aliases::I256;
            type Token<'a> = <alloy::sol_types::sol_data::Int<
                256,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Int<
                256,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Int<
                256,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for BeforeSwapDelta {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Currency(alloy::sol_types::private::Address);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Currency>
        for alloy::sol_types::private::Address {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::Token<
                '_,
            > {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Address,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        self,
                    )
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::abi_encode_packed_to(
                    self,
                    out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::abi_encoded_size(
                    self,
                )
            }
        }
        #[automatically_derived]
        impl Currency {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(value: alloy::sol_types::private::Address) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(self) -> alloy::sol_types::private::Address {
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
        impl alloy_sol_types::SolType for Currency {
            type RustType = alloy::sol_types::private::Address;
            type Token<'a> = <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::Token<
                'a,
            >;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::type_check(
                    token,
                )
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::detokenize(
                    token,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Currency {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                    rust,
                )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    out,
                )
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    rust,
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PoolConfigStore(alloy::sol_types::private::Address);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<PoolConfigStore>
        for alloy::sol_types::private::Address {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::Token<
                '_,
            > {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Address,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        self,
                    )
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::abi_encode_packed_to(
                    self,
                    out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::abi_encoded_size(
                    self,
                )
            }
        }
        #[automatically_derived]
        impl PoolConfigStore {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(value: alloy::sol_types::private::Address) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(self) -> alloy::sol_types::private::Address {
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
        impl alloy_sol_types::SolType for PoolConfigStore {
            type RustType = alloy::sol_types::private::Address;
            type Token<'a> = <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::Token<
                'a,
            >;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::type_check(
                    token,
                )
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::detokenize(
                    token,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for PoolConfigStore {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                    rust,
                )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    out,
                )
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    rust,
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StoreKey(alloy::sol_types::private::FixedBytes<27>);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<StoreKey>
        for alloy::sol_types::private::FixedBytes<27> {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::FixedBytes<
                27,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::FixedBytes<27>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::FixedBytes<
                    27,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::FixedBytes<
                    27,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::FixedBytes<
                    27,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        #[automatically_derived]
        impl StoreKey {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(value: alloy::sol_types::private::FixedBytes<27>) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(self) -> alloy::sol_types::private::FixedBytes<27> {
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
        impl alloy_sol_types::SolType for StoreKey {
            type RustType = alloy::sol_types::private::FixedBytes<27>;
            type Token<'a> = <alloy::sol_types::sol_data::FixedBytes<
                27,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::FixedBytes<
                27,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::FixedBytes<
                27,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::FixedBytes<
                    27,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::FixedBytes<
                    27,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for StoreKey {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::FixedBytes<
                    27,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::FixedBytes<
                    27,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::FixedBytes<
                    27,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct ConfigEntryUpdate { uint256 index; StoreKey key; uint24 bundleFee; uint24 unlockedFee; uint24 protocolUnlockedFee; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ConfigEntryUpdate {
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub key: <StoreKey as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub bundleFee: alloy::sol_types::private::primitives::aliases::U24,
        #[allow(missing_docs)]
        pub unlockedFee: alloy::sol_types::private::primitives::aliases::U24,
        #[allow(missing_docs)]
        pub protocolUnlockedFee: alloy::sol_types::private::primitives::aliases::U24,
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
            StoreKey,
            alloy::sol_types::sol_data::Uint<24>,
            alloy::sol_types::sol_data::Uint<24>,
            alloy::sol_types::sol_data::Uint<24>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            <StoreKey as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::primitives::aliases::U24,
            alloy::sol_types::private::primitives::aliases::U24,
            alloy::sol_types::private::primitives::aliases::U24,
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
        impl ::core::convert::From<ConfigEntryUpdate> for UnderlyingRustTuple<'_> {
            fn from(value: ConfigEntryUpdate) -> Self {
                (
                    value.index,
                    value.key,
                    value.bundleFee,
                    value.unlockedFee,
                    value.protocolUnlockedFee,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ConfigEntryUpdate {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    index: tuple.0,
                    key: tuple.1,
                    bundleFee: tuple.2,
                    unlockedFee: tuple.3,
                    protocolUnlockedFee: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ConfigEntryUpdate {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ConfigEntryUpdate {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
                    <StoreKey as alloy_sol_types::SolType>::tokenize(&self.key),
                    <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.bundleFee),
                    <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.unlockedFee),
                    <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.protocolUnlockedFee),
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
        impl alloy_sol_types::SolType for ConfigEntryUpdate {
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
        impl alloy_sol_types::SolStruct for ConfigEntryUpdate {
            const NAME: &'static str = "ConfigEntryUpdate";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ConfigEntryUpdate(uint256 index,bytes27 key,uint24 bundleFee,uint24 unlockedFee,uint24 protocolUnlockedFee)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.index)
                        .0,
                    <StoreKey as alloy_sol_types::SolType>::eip712_data_word(&self.key)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.bundleFee)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.unlockedFee)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.protocolUnlockedFee,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ConfigEntryUpdate {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.index)
                    + <StoreKey as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.key,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.bundleFee,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.unlockedFee,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.protocolUnlockedFee,
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
                    &rust.index,
                    out,
                );
                <StoreKey as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.key,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    24,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.bundleFee,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    24,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.unlockedFee,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    24,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.protocolUnlockedFee,
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
struct PoolKey { Currency currency0; Currency currency1; uint24 fee; int24 tickSpacing; address hooks; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PoolKey {
        #[allow(missing_docs)]
        pub currency0: <Currency as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub currency1: <Currency as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub fee: alloy::sol_types::private::primitives::aliases::U24,
        #[allow(missing_docs)]
        pub tickSpacing: alloy::sol_types::private::primitives::aliases::I24,
        #[allow(missing_docs)]
        pub hooks: alloy::sol_types::private::Address,
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
            Currency,
            Currency,
            alloy::sol_types::sol_data::Uint<24>,
            alloy::sol_types::sol_data::Int<24>,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <Currency as alloy::sol_types::SolType>::RustType,
            <Currency as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::primitives::aliases::U24,
            alloy::sol_types::private::primitives::aliases::I24,
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
        impl ::core::convert::From<PoolKey> for UnderlyingRustTuple<'_> {
            fn from(value: PoolKey) -> Self {
                (
                    value.currency0,
                    value.currency1,
                    value.fee,
                    value.tickSpacing,
                    value.hooks,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PoolKey {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    currency0: tuple.0,
                    currency1: tuple.1,
                    fee: tuple.2,
                    tickSpacing: tuple.3,
                    hooks: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for PoolKey {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for PoolKey {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <Currency as alloy_sol_types::SolType>::tokenize(&self.currency0),
                    <Currency as alloy_sol_types::SolType>::tokenize(&self.currency1),
                    <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.fee),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tickSpacing),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.hooks,
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
        impl alloy_sol_types::SolType for PoolKey {
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
        impl alloy_sol_types::SolStruct for PoolKey {
            const NAME: &'static str = "PoolKey";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "PoolKey(address currency0,address currency1,uint24 fee,int24 tickSpacing,address hooks)",
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
                    <Currency as alloy_sol_types::SolType>::eip712_data_word(
                            &self.currency0,
                        )
                        .0,
                    <Currency as alloy_sol_types::SolType>::eip712_data_word(
                            &self.currency1,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.fee)
                        .0,
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.tickSpacing)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.hooks,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for PoolKey {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <Currency as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.currency0,
                    )
                    + <Currency as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.currency1,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.fee)
                    + <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.tickSpacing,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.hooks,
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
                <Currency as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.currency0,
                    out,
                );
                <Currency as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.currency1,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    24,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.fee, out);
                <alloy::sol_types::sol_data::Int<
                    24,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tickSpacing,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.hooks,
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
    /**Custom error with signature `AssetAccessOutOfBounds(uint256,uint256)` and selector `0xffc31e71`.
```solidity
error AssetAccessOutOfBounds(uint256 index, uint256 length);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AssetAccessOutOfBounds {
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<AssetAccessOutOfBounds> for UnderlyingRustTuple<'_> {
            fn from(value: AssetAccessOutOfBounds) -> Self {
                (value.index, value.length)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AssetAccessOutOfBounds {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    index: tuple.0,
                    length: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AssetAccessOutOfBounds {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AssetAccessOutOfBounds(uint256,uint256)";
            const SELECTOR: [u8; 4] = [255u8, 195u8, 30u8, 113u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.length),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `AssetsOutOfOrderOrNotUnique()` and selector `0x80f11acf`.
```solidity
error AssetsOutOfOrderOrNotUnique();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AssetsOutOfOrderOrNotUnique {}
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
        impl ::core::convert::From<AssetsOutOfOrderOrNotUnique>
        for UnderlyingRustTuple<'_> {
            fn from(value: AssetsOutOfOrderOrNotUnique) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for AssetsOutOfOrderOrNotUnique {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AssetsOutOfOrderOrNotUnique {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AssetsOutOfOrderOrNotUnique()";
            const SELECTOR: [u8; 4] = [128u8, 241u8, 26u8, 207u8];
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
    /**Custom error with signature `AssetsUnordered()` and selector `0x32b4bc93`.
```solidity
error AssetsUnordered();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AssetsUnordered {}
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
        impl ::core::convert::From<AssetsUnordered> for UnderlyingRustTuple<'_> {
            fn from(value: AssetsUnordered) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AssetsUnordered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AssetsUnordered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AssetsUnordered()";
            const SELECTOR: [u8; 4] = [50u8, 180u8, 188u8, 147u8];
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
    /**Custom error with signature `BundlDeltaUnresolved(address)` and selector `0xcc67af53`.
```solidity
error BundlDeltaUnresolved(address asset);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BundlDeltaUnresolved {
        #[allow(missing_docs)]
        pub asset: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<BundlDeltaUnresolved> for UnderlyingRustTuple<'_> {
            fn from(value: BundlDeltaUnresolved) -> Self {
                (value.asset,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BundlDeltaUnresolved {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { asset: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BundlDeltaUnresolved {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BundlDeltaUnresolved(address)";
            const SELECTOR: [u8; 4] = [204u8, 103u8, 175u8, 83u8];
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
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `CannotSwapWhileLocked()` and selector `0x1e8107a0`.
```solidity
error CannotSwapWhileLocked();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CannotSwapWhileLocked {}
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
        impl ::core::convert::From<CannotSwapWhileLocked> for UnderlyingRustTuple<'_> {
            fn from(value: CannotSwapWhileLocked) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CannotSwapWhileLocked {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CannotSwapWhileLocked {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CannotSwapWhileLocked()";
            const SELECTOR: [u8; 4] = [30u8, 129u8, 7u8, 160u8];
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
    /**Custom error with signature `EntryKeyMismatch()` and selector `0x23f69dc2`.
```solidity
error EntryKeyMismatch();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EntryKeyMismatch {}
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
        impl ::core::convert::From<EntryKeyMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: EntryKeyMismatch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EntryKeyMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EntryKeyMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EntryKeyMismatch()";
            const SELECTOR: [u8; 4] = [35u8, 246u8, 157u8, 194u8];
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
    /**Custom error with signature `Expired()` and selector `0x203d82d8`.
```solidity
error Expired();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Expired {}
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
        impl ::core::convert::From<Expired> for UnderlyingRustTuple<'_> {
            fn from(value: Expired) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Expired {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for Expired {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "Expired()";
            const SELECTOR: [u8; 4] = [32u8, 61u8, 130u8, 216u8];
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
    /**Custom error with signature `FailedToDeployNewStore()` and selector `0x56702587`.
```solidity
error FailedToDeployNewStore();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FailedToDeployNewStore {}
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
        impl ::core::convert::From<FailedToDeployNewStore> for UnderlyingRustTuple<'_> {
            fn from(value: FailedToDeployNewStore) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FailedToDeployNewStore {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for FailedToDeployNewStore {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FailedToDeployNewStore()";
            const SELECTOR: [u8; 4] = [86u8, 112u8, 37u8, 135u8];
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
    /**Custom error with signature `FeeAboveMax()` and selector `0x76a3f95d`.
```solidity
error FeeAboveMax();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FeeAboveMax {}
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
        impl ::core::convert::From<FeeAboveMax> for UnderlyingRustTuple<'_> {
            fn from(value: FeeAboveMax) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FeeAboveMax {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for FeeAboveMax {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FeeAboveMax()";
            const SELECTOR: [u8; 4] = [118u8, 163u8, 249u8, 93u8];
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
    /**Custom error with signature `FillingTooLittle()` and selector `0xc4daf003`.
```solidity
error FillingTooLittle();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FillingTooLittle {}
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
        impl ::core::convert::From<FillingTooLittle> for UnderlyingRustTuple<'_> {
            fn from(value: FillingTooLittle) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FillingTooLittle {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for FillingTooLittle {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FillingTooLittle()";
            const SELECTOR: [u8; 4] = [196u8, 218u8, 240u8, 3u8];
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
    /**Custom error with signature `FillingTooMuch()` and selector `0x44182331`.
```solidity
error FillingTooMuch();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FillingTooMuch {}
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
        impl ::core::convert::From<FillingTooMuch> for UnderlyingRustTuple<'_> {
            fn from(value: FillingTooMuch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FillingTooMuch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for FillingTooMuch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FillingTooMuch()";
            const SELECTOR: [u8; 4] = [68u8, 24u8, 35u8, 49u8];
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
    /**Custom error with signature `GasAboveMax()` and selector `0x668fef1b`.
```solidity
error GasAboveMax();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GasAboveMax {}
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
        impl ::core::convert::From<GasAboveMax> for UnderlyingRustTuple<'_> {
            fn from(value: GasAboveMax) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for GasAboveMax {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for GasAboveMax {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GasAboveMax()";
            const SELECTOR: [u8; 4] = [102u8, 143u8, 239u8, 27u8];
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
    /**Custom error with signature `IndexMayHaveChanged()` and selector `0xf21fd99f`.
```solidity
error IndexMayHaveChanged();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IndexMayHaveChanged {}
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
        impl ::core::convert::From<IndexMayHaveChanged> for UnderlyingRustTuple<'_> {
            fn from(value: IndexMayHaveChanged) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for IndexMayHaveChanged {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for IndexMayHaveChanged {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "IndexMayHaveChanged()";
            const SELECTOR: [u8; 4] = [242u8, 31u8, 217u8, 159u8];
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
    /**Custom error with signature `InsufficientCapacity()` and selector `0x5cef583a`.
```solidity
error InsufficientCapacity();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientCapacity {}
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
        impl ::core::convert::From<InsufficientCapacity> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientCapacity) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientCapacity {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientCapacity {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientCapacity()";
            const SELECTOR: [u8; 4] = [92u8, 239u8, 88u8, 58u8];
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
    /**Custom error with signature `InvalidHookPermissions()` and selector `0xcb13e961`.
```solidity
error InvalidHookPermissions();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidHookPermissions {}
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
        impl ::core::convert::From<InvalidHookPermissions> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidHookPermissions) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidHookPermissions {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidHookPermissions {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidHookPermissions()";
            const SELECTOR: [u8; 4] = [203u8, 19u8, 233u8, 97u8];
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
    /**Custom error with signature `InvalidPermitType(uint8)` and selector `0x6f1d1509`.
```solidity
error InvalidPermitType(uint8);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidPermitType {
        #[allow(missing_docs)]
        pub _0: u8,
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
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u8,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidPermitType> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidPermitType) -> Self {
                (value._0,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidPermitType {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { _0: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidPermitType {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidPermitType(uint8)";
            const SELECTOR: [u8; 4] = [111u8, 29u8, 21u8, 9u8];
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
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidSignature()` and selector `0x8baa579f`.
```solidity
error InvalidSignature();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidSignature {}
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
        impl ::core::convert::From<InvalidSignature> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidSignature) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidSignature()";
            const SELECTOR: [u8; 4] = [139u8, 170u8, 87u8, 159u8];
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
    /**Custom error with signature `InvalidTickSpacing()` and selector `0x270815a0`.
```solidity
error InvalidTickSpacing();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidTickSpacing {}
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
        impl ::core::convert::From<InvalidTickSpacing> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidTickSpacing) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidTickSpacing {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidTickSpacing {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidTickSpacing()";
            const SELECTOR: [u8; 4] = [39u8, 8u8, 21u8, 160u8];
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
    /**Custom error with signature `JustInTimeLiquidityChange()` and selector `0xbecb195c`.
```solidity
error JustInTimeLiquidityChange();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct JustInTimeLiquidityChange {}
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
        impl ::core::convert::From<JustInTimeLiquidityChange>
        for UnderlyingRustTuple<'_> {
            fn from(value: JustInTimeLiquidityChange) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for JustInTimeLiquidityChange {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for JustInTimeLiquidityChange {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "JustInTimeLiquidityChange()";
            const SELECTOR: [u8; 4] = [190u8, 203u8, 25u8, 92u8];
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
    /**Custom error with signature `LimitViolated()` and selector `0x8e1edfa4`.
```solidity
error LimitViolated();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LimitViolated {}
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
        impl ::core::convert::From<LimitViolated> for UnderlyingRustTuple<'_> {
            fn from(value: LimitViolated) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for LimitViolated {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LimitViolated {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LimitViolated()";
            const SELECTOR: [u8; 4] = [142u8, 30u8, 223u8, 164u8];
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
    /**Custom error with signature `NoEntry()` and selector `0x2f659e44`.
```solidity
error NoEntry();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NoEntry {}
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
        impl ::core::convert::From<NoEntry> for UnderlyingRustTuple<'_> {
            fn from(value: NoEntry) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NoEntry {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NoEntry {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NoEntry()";
            const SELECTOR: [u8; 4] = [47u8, 101u8, 158u8, 68u8];
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
    /**Custom error with signature `NonceReuse()` and selector `0x8cb88872`.
```solidity
error NonceReuse();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NonceReuse {}
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
        impl ::core::convert::From<NonceReuse> for UnderlyingRustTuple<'_> {
            fn from(value: NonceReuse) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NonceReuse {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NonceReuse {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NonceReuse()";
            const SELECTOR: [u8; 4] = [140u8, 184u8, 136u8, 114u8];
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
    /**Custom error with signature `NotController()` and selector `0x23019e67`.
```solidity
error NotController();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotController {}
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
        impl ::core::convert::From<NotController> for UnderlyingRustTuple<'_> {
            fn from(value: NotController) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotController {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotController {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotController()";
            const SELECTOR: [u8; 4] = [35u8, 1u8, 158u8, 103u8];
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
    /**Custom error with signature `NotNode()` and selector `0x5cd26b68`.
```solidity
error NotNode();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotNode {}
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
        impl ::core::convert::From<NotNode> for UnderlyingRustTuple<'_> {
            fn from(value: NotNode) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotNode {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotNode {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotNode()";
            const SELECTOR: [u8; 4] = [92u8, 210u8, 107u8, 104u8];
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
    /**Custom error with signature `NotUniswap()` and selector `0xf8328614`.
```solidity
error NotUniswap();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotUniswap {}
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
        impl ::core::convert::From<NotUniswap> for UnderlyingRustTuple<'_> {
            fn from(value: NotUniswap) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotUniswap {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotUniswap {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotUniswap()";
            const SELECTOR: [u8; 4] = [248u8, 50u8, 134u8, 20u8];
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
    /**Custom error with signature `OnlyOncePerBlock()` and selector `0xd8a6b89b`.
```solidity
error OnlyOncePerBlock();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyOncePerBlock {}
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
        impl ::core::convert::From<OnlyOncePerBlock> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyOncePerBlock) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyOncePerBlock {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyOncePerBlock {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyOncePerBlock()";
            const SELECTOR: [u8; 4] = [216u8, 166u8, 184u8, 155u8];
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
    /**Custom error with signature `OrderAlreadyExecuted()` and selector `0x8a2ef116`.
```solidity
error OrderAlreadyExecuted();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OrderAlreadyExecuted {}
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
        impl ::core::convert::From<OrderAlreadyExecuted> for UnderlyingRustTuple<'_> {
            fn from(value: OrderAlreadyExecuted) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OrderAlreadyExecuted {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OrderAlreadyExecuted {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OrderAlreadyExecuted()";
            const SELECTOR: [u8; 4] = [138u8, 46u8, 241u8, 22u8];
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
    /**Custom error with signature `OutOfOrderOrDuplicatePairs()` and selector `0xf35f9399`.
```solidity
error OutOfOrderOrDuplicatePairs();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OutOfOrderOrDuplicatePairs {}
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
        impl ::core::convert::From<OutOfOrderOrDuplicatePairs>
        for UnderlyingRustTuple<'_> {
            fn from(value: OutOfOrderOrDuplicatePairs) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OutOfOrderOrDuplicatePairs {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OutOfOrderOrDuplicatePairs {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OutOfOrderOrDuplicatePairs()";
            const SELECTOR: [u8; 4] = [243u8, 95u8, 147u8, 153u8];
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
    /**Custom error with signature `PairAccessOutOfBounds(uint256,uint256)` and selector `0xf6601b50`.
```solidity
error PairAccessOutOfBounds(uint256 index, uint256 length);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PairAccessOutOfBounds {
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<PairAccessOutOfBounds> for UnderlyingRustTuple<'_> {
            fn from(value: PairAccessOutOfBounds) -> Self {
                (value.index, value.length)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PairAccessOutOfBounds {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    index: tuple.0,
                    length: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for PairAccessOutOfBounds {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PairAccessOutOfBounds(uint256,uint256)";
            const SELECTOR: [u8; 4] = [246u8, 96u8, 27u8, 80u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.length),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ReaderNotAtEnd()` and selector `0x01842f8c`.
```solidity
error ReaderNotAtEnd();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ReaderNotAtEnd {}
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
        impl ::core::convert::From<ReaderNotAtEnd> for UnderlyingRustTuple<'_> {
            fn from(value: ReaderNotAtEnd) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ReaderNotAtEnd {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ReaderNotAtEnd {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ReaderNotAtEnd()";
            const SELECTOR: [u8; 4] = [1u8, 132u8, 47u8, 140u8];
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
    /**Custom error with signature `ToBGasUsedAboveMax()` and selector `0x2bae6c52`.
```solidity
error ToBGasUsedAboveMax();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ToBGasUsedAboveMax {}
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
        impl ::core::convert::From<ToBGasUsedAboveMax> for UnderlyingRustTuple<'_> {
            fn from(value: ToBGasUsedAboveMax) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ToBGasUsedAboveMax {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ToBGasUsedAboveMax {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ToBGasUsedAboveMax()";
            const SELECTOR: [u8; 4] = [43u8, 174u8, 108u8, 82u8];
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
    /**Custom error with signature `UnlockDataTooShort()` and selector `0x4926898b`.
```solidity
error UnlockDataTooShort();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UnlockDataTooShort {}
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
        impl ::core::convert::From<UnlockDataTooShort> for UnderlyingRustTuple<'_> {
            fn from(value: UnlockDataTooShort) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UnlockDataTooShort {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UnlockDataTooShort {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UnlockDataTooShort()";
            const SELECTOR: [u8; 4] = [73u8, 38u8, 137u8, 139u8];
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
    /**Custom error with signature `UnlockFeeAboveMax()` and selector `0x37d7fd85`.
```solidity
error UnlockFeeAboveMax();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UnlockFeeAboveMax {}
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
        impl ::core::convert::From<UnlockFeeAboveMax> for UnderlyingRustTuple<'_> {
            fn from(value: UnlockFeeAboveMax) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UnlockFeeAboveMax {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UnlockFeeAboveMax {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UnlockFeeAboveMax()";
            const SELECTOR: [u8; 4] = [55u8, 215u8, 253u8, 133u8];
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
    /**Custom error with signature `UnlockedFeeNotSet(bytes27)` and selector `0x291a6d08`.
```solidity
error UnlockedFeeNotSet(StoreKey key);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UnlockedFeeNotSet {
        #[allow(missing_docs)]
        pub key: <StoreKey as alloy::sol_types::SolType>::RustType,
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
        type UnderlyingSolTuple<'a> = (StoreKey,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <StoreKey as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<UnlockedFeeNotSet> for UnderlyingRustTuple<'_> {
            fn from(value: UnlockedFeeNotSet) -> Self {
                (value.key,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UnlockedFeeNotSet {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { key: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UnlockedFeeNotSet {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UnlockedFeeNotSet(bytes27)";
            const SELECTOR: [u8; 4] = [41u8, 26u8, 109u8, 8u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<StoreKey as alloy_sol_types::SolType>::tokenize(&self.key),)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `WrongEndLiquidity(uint128,uint128)` and selector `0x6429cfd2`.
```solidity
error WrongEndLiquidity(uint128 endLiquidity, uint128 actualCurrentLiquidity);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WrongEndLiquidity {
        #[allow(missing_docs)]
        pub endLiquidity: u128,
        #[allow(missing_docs)]
        pub actualCurrentLiquidity: u128,
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
            alloy::sol_types::sol_data::Uint<128>,
            alloy::sol_types::sol_data::Uint<128>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u128, u128);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<WrongEndLiquidity> for UnderlyingRustTuple<'_> {
            fn from(value: WrongEndLiquidity) -> Self {
                (value.endLiquidity, value.actualCurrentLiquidity)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for WrongEndLiquidity {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    endLiquidity: tuple.0,
                    actualCurrentLiquidity: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for WrongEndLiquidity {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WrongEndLiquidity(uint128,uint128)";
            const SELECTOR: [u8; 4] = [100u8, 41u8, 207u8, 210u8];
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
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.endLiquidity),
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.actualCurrentLiquidity,
                    ),
                )
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address uniV4, address controller);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub uniV4: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub controller: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value.uniV4, value.controller)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        uniV4: tuple.0,
                        controller: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
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
                        &self.uniV4,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.controller,
                    ),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `afterSwap(address,(address,address,uint24,int24,address),(bool,int256,uint160),int256,bytes)` and selector `0xb47b2fb1`.
```solidity
function afterSwap(address, PoolKey memory key, IPoolManager.SwapParams memory params, BalanceDelta swap_delta, bytes memory) external returns (bytes4, int128);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct afterSwapCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub key: <PoolKey as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub params: <IPoolManager::SwapParams as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub swap_delta: <BalanceDelta as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _4: alloy::sol_types::private::Bytes,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`afterSwap(address,(address,address,uint24,int24,address),(bool,int256,uint160),int256,bytes)`](afterSwapCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct afterSwapReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<4>,
        #[allow(missing_docs)]
        pub _1: i128,
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
                alloy::sol_types::sol_data::Address,
                PoolKey,
                IPoolManager::SwapParams,
                BalanceDelta,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <PoolKey as alloy::sol_types::SolType>::RustType,
                <IPoolManager::SwapParams as alloy::sol_types::SolType>::RustType,
                <BalanceDelta as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<afterSwapCall> for UnderlyingRustTuple<'_> {
                fn from(value: afterSwapCall) -> Self {
                    (value._0, value.key, value.params, value.swap_delta, value._4)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for afterSwapCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        key: tuple.1,
                        params: tuple.2,
                        swap_delta: tuple.3,
                        _4: tuple.4,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<4>,
                alloy::sol_types::sol_data::Int<128>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<4>,
                i128,
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
            impl ::core::convert::From<afterSwapReturn> for UnderlyingRustTuple<'_> {
                fn from(value: afterSwapReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for afterSwapReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for afterSwapCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                PoolKey,
                IPoolManager::SwapParams,
                BalanceDelta,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = afterSwapReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<4>,
                alloy::sol_types::sol_data::Int<128>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "afterSwap(address,(address,address,uint24,int24,address),(bool,int256,uint160),int256,bytes)";
            const SELECTOR: [u8; 4] = [180u8, 123u8, 47u8, 177u8];
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
                    <PoolKey as alloy_sol_types::SolType>::tokenize(&self.key),
                    <IPoolManager::SwapParams as alloy_sol_types::SolType>::tokenize(
                        &self.params,
                    ),
                    <BalanceDelta as alloy_sol_types::SolType>::tokenize(
                        &self.swap_delta,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._4,
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
    /**Function with signature `batchUpdatePools(address,(uint256,bytes27,uint24,uint24,uint24)[])` and selector `0x53b41c55`.
```solidity
function batchUpdatePools(PoolConfigStore expected_store, ConfigEntryUpdate[] memory updates) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct batchUpdatePoolsCall {
        #[allow(missing_docs)]
        pub expected_store: <PoolConfigStore as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub updates: alloy::sol_types::private::Vec<
            <ConfigEntryUpdate as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`batchUpdatePools(address,(uint256,bytes27,uint24,uint24,uint24)[])`](batchUpdatePoolsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct batchUpdatePoolsReturn {}
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
                PoolConfigStore,
                alloy::sol_types::sol_data::Array<ConfigEntryUpdate>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <PoolConfigStore as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<
                    <ConfigEntryUpdate as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<batchUpdatePoolsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: batchUpdatePoolsCall) -> Self {
                    (value.expected_store, value.updates)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for batchUpdatePoolsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        expected_store: tuple.0,
                        updates: tuple.1,
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
            impl ::core::convert::From<batchUpdatePoolsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: batchUpdatePoolsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for batchUpdatePoolsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for batchUpdatePoolsCall {
            type Parameters<'a> = (
                PoolConfigStore,
                alloy::sol_types::sol_data::Array<ConfigEntryUpdate>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = batchUpdatePoolsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "batchUpdatePools(address,(uint256,bytes27,uint24,uint24,uint24)[])";
            const SELECTOR: [u8; 4] = [83u8, 180u8, 28u8, 85u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <PoolConfigStore as alloy_sol_types::SolType>::tokenize(
                        &self.expected_store,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        ConfigEntryUpdate,
                    > as alloy_sol_types::SolType>::tokenize(&self.updates),
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
    /**Function with signature `beforeAddLiquidity(address,(address,address,uint24,int24,address),(int24,int24,int256,bytes32),bytes)` and selector `0x259982e5`.
```solidity
function beforeAddLiquidity(address sender, PoolKey memory key, IPoolManager.ModifyLiquidityParams memory params, bytes memory) external returns (bytes4);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct beforeAddLiquidityCall {
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub key: <PoolKey as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub params: <IPoolManager::ModifyLiquidityParams as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _3: alloy::sol_types::private::Bytes,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`beforeAddLiquidity(address,(address,address,uint24,int24,address),(int24,int24,int256,bytes32),bytes)`](beforeAddLiquidityCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct beforeAddLiquidityReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<4>,
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
                alloy::sol_types::sol_data::Address,
                PoolKey,
                IPoolManager::ModifyLiquidityParams,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <PoolKey as alloy::sol_types::SolType>::RustType,
                <IPoolManager::ModifyLiquidityParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<beforeAddLiquidityCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: beforeAddLiquidityCall) -> Self {
                    (value.sender, value.key, value.params, value._3)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for beforeAddLiquidityCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        sender: tuple.0,
                        key: tuple.1,
                        params: tuple.2,
                        _3: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<4>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<beforeAddLiquidityReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: beforeAddLiquidityReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for beforeAddLiquidityReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for beforeAddLiquidityCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                PoolKey,
                IPoolManager::ModifyLiquidityParams,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = beforeAddLiquidityReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "beforeAddLiquidity(address,(address,address,uint24,int24,address),(int24,int24,int256,bytes32),bytes)";
            const SELECTOR: [u8; 4] = [37u8, 153u8, 130u8, 229u8];
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
                        &self.sender,
                    ),
                    <PoolKey as alloy_sol_types::SolType>::tokenize(&self.key),
                    <IPoolManager::ModifyLiquidityParams as alloy_sol_types::SolType>::tokenize(
                        &self.params,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._3,
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
    /**Function with signature `beforeRemoveLiquidity(address,(address,address,uint24,int24,address),(int24,int24,int256,bytes32),bytes)` and selector `0x21d0ee70`.
```solidity
function beforeRemoveLiquidity(address sender, PoolKey memory key, IPoolManager.ModifyLiquidityParams memory params, bytes memory) external returns (bytes4);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct beforeRemoveLiquidityCall {
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub key: <PoolKey as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub params: <IPoolManager::ModifyLiquidityParams as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _3: alloy::sol_types::private::Bytes,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`beforeRemoveLiquidity(address,(address,address,uint24,int24,address),(int24,int24,int256,bytes32),bytes)`](beforeRemoveLiquidityCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct beforeRemoveLiquidityReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<4>,
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
                alloy::sol_types::sol_data::Address,
                PoolKey,
                IPoolManager::ModifyLiquidityParams,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <PoolKey as alloy::sol_types::SolType>::RustType,
                <IPoolManager::ModifyLiquidityParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<beforeRemoveLiquidityCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: beforeRemoveLiquidityCall) -> Self {
                    (value.sender, value.key, value.params, value._3)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for beforeRemoveLiquidityCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        sender: tuple.0,
                        key: tuple.1,
                        params: tuple.2,
                        _3: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<4>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<beforeRemoveLiquidityReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: beforeRemoveLiquidityReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for beforeRemoveLiquidityReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for beforeRemoveLiquidityCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                PoolKey,
                IPoolManager::ModifyLiquidityParams,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = beforeRemoveLiquidityReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "beforeRemoveLiquidity(address,(address,address,uint24,int24,address),(int24,int24,int256,bytes32),bytes)";
            const SELECTOR: [u8; 4] = [33u8, 208u8, 238u8, 112u8];
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
                        &self.sender,
                    ),
                    <PoolKey as alloy_sol_types::SolType>::tokenize(&self.key),
                    <IPoolManager::ModifyLiquidityParams as alloy_sol_types::SolType>::tokenize(
                        &self.params,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._3,
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
    /**Function with signature `beforeSwap(address,(address,address,uint24,int24,address),(bool,int256,uint160),bytes)` and selector `0x575e24b4`.
```solidity
function beforeSwap(address, PoolKey memory key, IPoolManager.SwapParams memory, bytes memory optionalUnlockData) external returns (bytes4 response, BeforeSwapDelta, uint24 swapFee);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct beforeSwapCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub key: <PoolKey as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _2: <IPoolManager::SwapParams as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub optionalUnlockData: alloy::sol_types::private::Bytes,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`beforeSwap(address,(address,address,uint24,int24,address),(bool,int256,uint160),bytes)`](beforeSwapCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct beforeSwapReturn {
        #[allow(missing_docs)]
        pub response: alloy::sol_types::private::FixedBytes<4>,
        #[allow(missing_docs)]
        pub _1: <BeforeSwapDelta as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub swapFee: alloy::sol_types::private::primitives::aliases::U24,
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
                alloy::sol_types::sol_data::Address,
                PoolKey,
                IPoolManager::SwapParams,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <PoolKey as alloy::sol_types::SolType>::RustType,
                <IPoolManager::SwapParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<beforeSwapCall> for UnderlyingRustTuple<'_> {
                fn from(value: beforeSwapCall) -> Self {
                    (value._0, value.key, value._2, value.optionalUnlockData)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for beforeSwapCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        key: tuple.1,
                        _2: tuple.2,
                        optionalUnlockData: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<4>,
                BeforeSwapDelta,
                alloy::sol_types::sol_data::Uint<24>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<4>,
                <BeforeSwapDelta as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U24,
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
            impl ::core::convert::From<beforeSwapReturn> for UnderlyingRustTuple<'_> {
                fn from(value: beforeSwapReturn) -> Self {
                    (value.response, value._1, value.swapFee)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for beforeSwapReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        response: tuple.0,
                        _1: tuple.1,
                        swapFee: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for beforeSwapCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                PoolKey,
                IPoolManager::SwapParams,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = beforeSwapReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<4>,
                BeforeSwapDelta,
                alloy::sol_types::sol_data::Uint<24>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "beforeSwap(address,(address,address,uint24,int24,address),(bool,int256,uint160),bytes)";
            const SELECTOR: [u8; 4] = [87u8, 94u8, 36u8, 180u8];
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
                    <PoolKey as alloy_sol_types::SolType>::tokenize(&self.key),
                    <IPoolManager::SwapParams as alloy_sol_types::SolType>::tokenize(
                        &self._2,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.optionalUnlockData,
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
    /**Function with signature `collect_unlock_swap_fees(address,bytes)` and selector `0x33830e48`.
```solidity
function collect_unlock_swap_fees(address to, bytes memory packed_assets) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct collect_unlock_swap_feesCall {
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub packed_assets: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`collect_unlock_swap_fees(address,bytes)`](collect_unlock_swap_feesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct collect_unlock_swap_feesReturn {}
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
            impl ::core::convert::From<collect_unlock_swap_feesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: collect_unlock_swap_feesCall) -> Self {
                    (value.to, value.packed_assets)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for collect_unlock_swap_feesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        to: tuple.0,
                        packed_assets: tuple.1,
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
            impl ::core::convert::From<collect_unlock_swap_feesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: collect_unlock_swap_feesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for collect_unlock_swap_feesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for collect_unlock_swap_feesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = collect_unlock_swap_feesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "collect_unlock_swap_fees(address,bytes)";
            const SELECTOR: [u8; 4] = [51u8, 131u8, 14u8, 72u8];
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.packed_assets,
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
    /**Function with signature `compose(address,bytes)` and selector `0x7407905c`.
```solidity
function compose(address from, bytes memory payload) external returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct composeCall {
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub payload: alloy::sol_types::private::Bytes,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`compose(address,bytes)`](composeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct composeReturn {
        #[allow(missing_docs)]
        pub _0: u32,
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
            impl ::core::convert::From<composeCall> for UnderlyingRustTuple<'_> {
                fn from(value: composeCall) -> Self {
                    (value.from, value.payload)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for composeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        from: tuple.0,
                        payload: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<composeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: composeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for composeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for composeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = composeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "compose(address,bytes)";
            const SELECTOR: [u8; 4] = [116u8, 7u8, 144u8, 92u8];
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
                        &self.from,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.payload,
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
    /**Function with signature `configurePool(address,address,uint16,uint24,uint24,uint24)` and selector `0x13871465`.
```solidity
function configurePool(address asset0, address asset1, uint16 tickSpacing, uint24 bundleFee, uint24 unlockedFee, uint24 protocolUnlockedFee) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct configurePoolCall {
        #[allow(missing_docs)]
        pub asset0: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub asset1: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub tickSpacing: u16,
        #[allow(missing_docs)]
        pub bundleFee: alloy::sol_types::private::primitives::aliases::U24,
        #[allow(missing_docs)]
        pub unlockedFee: alloy::sol_types::private::primitives::aliases::U24,
        #[allow(missing_docs)]
        pub protocolUnlockedFee: alloy::sol_types::private::primitives::aliases::U24,
    }
    ///Container type for the return parameters of the [`configurePool(address,address,uint16,uint24,uint24,uint24)`](configurePoolCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct configurePoolReturn {}
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<16>,
                alloy::sol_types::sol_data::Uint<24>,
                alloy::sol_types::sol_data::Uint<24>,
                alloy::sol_types::sol_data::Uint<24>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                u16,
                alloy::sol_types::private::primitives::aliases::U24,
                alloy::sol_types::private::primitives::aliases::U24,
                alloy::sol_types::private::primitives::aliases::U24,
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
            impl ::core::convert::From<configurePoolCall> for UnderlyingRustTuple<'_> {
                fn from(value: configurePoolCall) -> Self {
                    (
                        value.asset0,
                        value.asset1,
                        value.tickSpacing,
                        value.bundleFee,
                        value.unlockedFee,
                        value.protocolUnlockedFee,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for configurePoolCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset0: tuple.0,
                        asset1: tuple.1,
                        tickSpacing: tuple.2,
                        bundleFee: tuple.3,
                        unlockedFee: tuple.4,
                        protocolUnlockedFee: tuple.5,
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
            impl ::core::convert::From<configurePoolReturn> for UnderlyingRustTuple<'_> {
                fn from(value: configurePoolReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for configurePoolReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for configurePoolCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<16>,
                alloy::sol_types::sol_data::Uint<24>,
                alloy::sol_types::sol_data::Uint<24>,
                alloy::sol_types::sol_data::Uint<24>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = configurePoolReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "configurePool(address,address,uint16,uint24,uint24,uint24)";
            const SELECTOR: [u8; 4] = [19u8, 135u8, 20u8, 101u8];
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
                        &self.asset0,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset1,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(&self.tickSpacing),
                    <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.bundleFee),
                    <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.unlockedFee),
                    <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.protocolUnlockedFee),
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
    /**Function with signature `deposit(address,uint256)` and selector `0x47e7ef24`.
```solidity
function deposit(address asset, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deposit_0Call {
        #[allow(missing_docs)]
        pub asset: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`deposit(address,uint256)`](deposit_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deposit_0Return {}
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<deposit_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: deposit_0Call) -> Self {
                    (value.asset, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deposit_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset: tuple.0,
                        amount: tuple.1,
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
            impl ::core::convert::From<deposit_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: deposit_0Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deposit_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deposit_0Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = deposit_0Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deposit(address,uint256)";
            const SELECTOR: [u8; 4] = [71u8, 231u8, 239u8, 36u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
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
    /**Function with signature `deposit(address,address,uint256)` and selector `0x8340f549`.
```solidity
function deposit(address asset, address to, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deposit_1Call {
        #[allow(missing_docs)]
        pub asset: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`deposit(address,address,uint256)`](deposit_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deposit_1Return {}
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<deposit_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: deposit_1Call) -> Self {
                    (value.asset, value.to, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deposit_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset: tuple.0,
                        to: tuple.1,
                        amount: tuple.2,
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
            impl ::core::convert::From<deposit_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: deposit_1Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deposit_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deposit_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = deposit_1Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deposit(address,address,uint256)";
            const SELECTOR: [u8; 4] = [131u8, 64u8, 245u8, 73u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
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
    /**Function with signature `eip712Domain()` and selector `0x84b0196e`.
```solidity
function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eip712DomainCall {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`eip712Domain()`](eip712DomainCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eip712DomainReturn {
        #[allow(missing_docs)]
        pub fields: alloy::sol_types::private::FixedBytes<1>,
        #[allow(missing_docs)]
        pub name: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub version: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub chainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub verifyingContract: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub salt: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub extensions: alloy::sol_types::private::Vec<
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
            impl ::core::convert::From<eip712DomainCall> for UnderlyingRustTuple<'_> {
                fn from(value: eip712DomainCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eip712DomainCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<1>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<1>,
                alloy::sol_types::private::String,
                alloy::sol_types::private::String,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<eip712DomainReturn> for UnderlyingRustTuple<'_> {
                fn from(value: eip712DomainReturn) -> Self {
                    (
                        value.fields,
                        value.name,
                        value.version,
                        value.chainId,
                        value.verifyingContract,
                        value.salt,
                        value.extensions,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eip712DomainReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        fields: tuple.0,
                        name: tuple.1,
                        version: tuple.2,
                        chainId: tuple.3,
                        verifyingContract: tuple.4,
                        salt: tuple.5,
                        extensions: tuple.6,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for eip712DomainCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = eip712DomainReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<1>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "eip712Domain()";
            const SELECTOR: [u8; 4] = [132u8, 176u8, 25u8, 110u8];
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
    /**Function with signature `execute(bytes)` and selector `0x09c5eabe`.
```solidity
function execute(bytes memory encoded) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeCall {
        #[allow(missing_docs)]
        pub encoded: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`execute(bytes)`](executeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeReturn {}
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
            impl ::core::convert::From<executeCall> for UnderlyingRustTuple<'_> {
                fn from(value: executeCall) -> Self {
                    (value.encoded,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { encoded: tuple.0 }
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
            impl ::core::convert::From<executeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: executeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for executeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = executeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "execute(bytes)";
            const SELECTOR: [u8; 4] = [9u8, 197u8, 234u8, 190u8];
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
                        &self.encoded,
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
    /**Function with signature `extsload(uint256)` and selector `0x7cf98081`.
```solidity
function extsload(uint256 slot) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct extsloadCall {
        #[allow(missing_docs)]
        pub slot: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`extsload(uint256)`](extsloadCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct extsloadReturn {
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
            impl ::core::convert::From<extsloadCall> for UnderlyingRustTuple<'_> {
                fn from(value: extsloadCall) -> Self {
                    (value.slot,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for extsloadCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { slot: tuple.0 }
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
            impl ::core::convert::From<extsloadReturn> for UnderlyingRustTuple<'_> {
                fn from(value: extsloadReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for extsloadReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for extsloadCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = extsloadReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "extsload(uint256)";
            const SELECTOR: [u8; 4] = [124u8, 249u8, 128u8, 129u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.slot),
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
    /**Function with signature `initializePool(address,address,uint256,uint160)` and selector `0x8587f450`.
```solidity
function initializePool(address assetA, address assetB, uint256 storeIndex, uint160 sqrtPriceX96) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializePoolCall {
        #[allow(missing_docs)]
        pub assetA: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub assetB: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub storeIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub sqrtPriceX96: alloy::sol_types::private::primitives::aliases::U160,
    }
    ///Container type for the return parameters of the [`initializePool(address,address,uint256,uint160)`](initializePoolCall) function.
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<160>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
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
                    (value.assetA, value.assetB, value.storeIndex, value.sqrtPriceX96)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializePoolCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        assetA: tuple.0,
                        assetB: tuple.1,
                        storeIndex: tuple.2,
                        sqrtPriceX96: tuple.3,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
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
            const SIGNATURE: &'static str = "initializePool(address,address,uint256,uint160)";
            const SELECTOR: [u8; 4] = [133u8, 135u8, 244u8, 80u8];
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
                        &self.assetA,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.assetB,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.storeIndex),
                    <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::SolType>::tokenize(&self.sqrtPriceX96),
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
    /**Function with signature `invalidateNonce(uint64)` and selector `0x116a5550`.
```solidity
function invalidateNonce(uint64 nonce) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct invalidateNonceCall {
        #[allow(missing_docs)]
        pub nonce: u64,
    }
    ///Container type for the return parameters of the [`invalidateNonce(uint64)`](invalidateNonceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct invalidateNonceReturn {}
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<invalidateNonceCall> for UnderlyingRustTuple<'_> {
                fn from(value: invalidateNonceCall) -> Self {
                    (value.nonce,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for invalidateNonceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { nonce: tuple.0 }
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
            impl ::core::convert::From<invalidateNonceReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: invalidateNonceReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for invalidateNonceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for invalidateNonceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = invalidateNonceReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "invalidateNonce(uint64)";
            const SELECTOR: [u8; 4] = [17u8, 106u8, 85u8, 80u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
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
    /**Function with signature `pullFee(address,uint256)` and selector `0xd9e17f98`.
```solidity
function pullFee(address asset, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pullFeeCall {
        #[allow(missing_docs)]
        pub asset: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`pullFee(address,uint256)`](pullFeeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pullFeeReturn {}
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<pullFeeCall> for UnderlyingRustTuple<'_> {
                fn from(value: pullFeeCall) -> Self {
                    (value.asset, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pullFeeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset: tuple.0,
                        amount: tuple.1,
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
            impl ::core::convert::From<pullFeeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pullFeeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pullFeeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pullFeeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = pullFeeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pullFee(address,uint256)";
            const SELECTOR: [u8; 4] = [217u8, 225u8, 127u8, 152u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
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
    /**Function with signature `removePool(bytes27,address,uint256)` and selector `0xdd4d4cf6`.
```solidity
function removePool(StoreKey key, PoolConfigStore expected_store, uint256 store_index) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removePoolCall {
        #[allow(missing_docs)]
        pub key: <StoreKey as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub expected_store: <PoolConfigStore as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub store_index: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`removePool(bytes27,address,uint256)`](removePoolCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removePoolReturn {}
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
                StoreKey,
                PoolConfigStore,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <StoreKey as alloy::sol_types::SolType>::RustType,
                <PoolConfigStore as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<removePoolCall> for UnderlyingRustTuple<'_> {
                fn from(value: removePoolCall) -> Self {
                    (value.key, value.expected_store, value.store_index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removePoolCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        key: tuple.0,
                        expected_store: tuple.1,
                        store_index: tuple.2,
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
            impl ::core::convert::From<removePoolReturn> for UnderlyingRustTuple<'_> {
                fn from(value: removePoolReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removePoolReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removePoolCall {
            type Parameters<'a> = (
                StoreKey,
                PoolConfigStore,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = removePoolReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removePool(bytes27,address,uint256)";
            const SELECTOR: [u8; 4] = [221u8, 77u8, 76u8, 246u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <StoreKey as alloy_sol_types::SolType>::tokenize(&self.key),
                    <PoolConfigStore as alloy_sol_types::SolType>::tokenize(
                        &self.expected_store,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.store_index),
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
    /**Function with signature `setController(address)` and selector `0x92eefe9b`.
```solidity
function setController(address newController) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setControllerCall {
        #[allow(missing_docs)]
        pub newController: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setController(address)`](setControllerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setControllerReturn {}
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
            impl ::core::convert::From<setControllerCall> for UnderlyingRustTuple<'_> {
                fn from(value: setControllerCall) -> Self {
                    (value.newController,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setControllerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newController: tuple.0 }
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
            impl ::core::convert::From<setControllerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setControllerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setControllerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setControllerCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setControllerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setController(address)";
            const SELECTOR: [u8; 4] = [146u8, 238u8, 254u8, 155u8];
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
                        &self.newController,
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
    /**Function with signature `toggleNodes(address[])` and selector `0xd6cffd1e`.
```solidity
function toggleNodes(address[] memory nodes) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct toggleNodesCall {
        #[allow(missing_docs)]
        pub nodes: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    ///Container type for the return parameters of the [`toggleNodes(address[])`](toggleNodesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct toggleNodesReturn {}
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
            impl ::core::convert::From<toggleNodesCall> for UnderlyingRustTuple<'_> {
                fn from(value: toggleNodesCall) -> Self {
                    (value.nodes,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for toggleNodesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { nodes: tuple.0 }
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
            impl ::core::convert::From<toggleNodesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: toggleNodesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for toggleNodesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for toggleNodesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = toggleNodesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "toggleNodes(address[])";
            const SELECTOR: [u8; 4] = [214u8, 207u8, 253u8, 30u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.nodes),
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
    /**Function with signature `unlockCallback(bytes)` and selector `0x91dd7346`.
```solidity
function unlockCallback(bytes memory data) external returns (bytes memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unlockCallbackCall {
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`unlockCallback(bytes)`](unlockCallbackCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unlockCallbackReturn {
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
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<unlockCallbackCall> for UnderlyingRustTuple<'_> {
                fn from(value: unlockCallbackCall) -> Self {
                    (value.data,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unlockCallbackCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { data: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
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
            impl ::core::convert::From<unlockCallbackReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: unlockCallbackReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for unlockCallbackReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for unlockCallbackCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = unlockCallbackReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "unlockCallback(bytes)";
            const SELECTOR: [u8; 4] = [145u8, 221u8, 115u8, 70u8];
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
                        &self.data,
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
    /**Function with signature `unlockWithEmptyAttestation(address,bytes)` and selector `0x1828e0e7`.
```solidity
function unlockWithEmptyAttestation(address node, bytes memory signature) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unlockWithEmptyAttestationCall {
        #[allow(missing_docs)]
        pub node: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`unlockWithEmptyAttestation(address,bytes)`](unlockWithEmptyAttestationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unlockWithEmptyAttestationReturn {}
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
            impl ::core::convert::From<unlockWithEmptyAttestationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: unlockWithEmptyAttestationCall) -> Self {
                    (value.node, value.signature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for unlockWithEmptyAttestationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        node: tuple.0,
                        signature: tuple.1,
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
            impl ::core::convert::From<unlockWithEmptyAttestationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: unlockWithEmptyAttestationReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for unlockWithEmptyAttestationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for unlockWithEmptyAttestationCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = unlockWithEmptyAttestationReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "unlockWithEmptyAttestation(address,bytes)";
            const SELECTOR: [u8; 4] = [24u8, 40u8, 224u8, 231u8];
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
                        &self.node,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
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
    /**Function with signature `withdraw(address,address,uint256)` and selector `0xd9caed12`.
```solidity
function withdraw(address asset, address to, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdraw_0Call {
        #[allow(missing_docs)]
        pub asset: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`withdraw(address,address,uint256)`](withdraw_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdraw_0Return {}
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<withdraw_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: withdraw_0Call) -> Self {
                    (value.asset, value.to, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for withdraw_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset: tuple.0,
                        to: tuple.1,
                        amount: tuple.2,
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
            impl ::core::convert::From<withdraw_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: withdraw_0Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for withdraw_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for withdraw_0Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = withdraw_0Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "withdraw(address,address,uint256)";
            const SELECTOR: [u8; 4] = [217u8, 202u8, 237u8, 18u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
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
    /**Function with signature `withdraw(address,uint256)` and selector `0xf3fef3a3`.
```solidity
function withdraw(address asset, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdraw_1Call {
        #[allow(missing_docs)]
        pub asset: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`withdraw(address,uint256)`](withdraw_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdraw_1Return {}
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<withdraw_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: withdraw_1Call) -> Self {
                    (value.asset, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for withdraw_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset: tuple.0,
                        amount: tuple.1,
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
            impl ::core::convert::From<withdraw_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: withdraw_1Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for withdraw_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for withdraw_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = withdraw_1Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "withdraw(address,uint256)";
            const SELECTOR: [u8; 4] = [243u8, 254u8, 243u8, 163u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
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
    ///Container for all the [`Angstrom`](self) function calls.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum AngstromCalls {
        #[allow(missing_docs)]
        afterSwap(afterSwapCall),
        #[allow(missing_docs)]
        batchUpdatePools(batchUpdatePoolsCall),
        #[allow(missing_docs)]
        beforeAddLiquidity(beforeAddLiquidityCall),
        #[allow(missing_docs)]
        beforeRemoveLiquidity(beforeRemoveLiquidityCall),
        #[allow(missing_docs)]
        beforeSwap(beforeSwapCall),
        #[allow(missing_docs)]
        collect_unlock_swap_fees(collect_unlock_swap_feesCall),
        #[allow(missing_docs)]
        compose(composeCall),
        #[allow(missing_docs)]
        configurePool(configurePoolCall),
        #[allow(missing_docs)]
        deposit_0(deposit_0Call),
        #[allow(missing_docs)]
        deposit_1(deposit_1Call),
        #[allow(missing_docs)]
        eip712Domain(eip712DomainCall),
        #[allow(missing_docs)]
        execute(executeCall),
        #[allow(missing_docs)]
        extsload(extsloadCall),
        #[allow(missing_docs)]
        initializePool(initializePoolCall),
        #[allow(missing_docs)]
        invalidateNonce(invalidateNonceCall),
        #[allow(missing_docs)]
        pullFee(pullFeeCall),
        #[allow(missing_docs)]
        removePool(removePoolCall),
        #[allow(missing_docs)]
        setController(setControllerCall),
        #[allow(missing_docs)]
        toggleNodes(toggleNodesCall),
        #[allow(missing_docs)]
        unlockCallback(unlockCallbackCall),
        #[allow(missing_docs)]
        unlockWithEmptyAttestation(unlockWithEmptyAttestationCall),
        #[allow(missing_docs)]
        withdraw_0(withdraw_0Call),
        #[allow(missing_docs)]
        withdraw_1(withdraw_1Call),
    }
    #[automatically_derived]
    impl AngstromCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [9u8, 197u8, 234u8, 190u8],
            [17u8, 106u8, 85u8, 80u8],
            [19u8, 135u8, 20u8, 101u8],
            [24u8, 40u8, 224u8, 231u8],
            [33u8, 208u8, 238u8, 112u8],
            [37u8, 153u8, 130u8, 229u8],
            [51u8, 131u8, 14u8, 72u8],
            [71u8, 231u8, 239u8, 36u8],
            [83u8, 180u8, 28u8, 85u8],
            [87u8, 94u8, 36u8, 180u8],
            [116u8, 7u8, 144u8, 92u8],
            [124u8, 249u8, 128u8, 129u8],
            [131u8, 64u8, 245u8, 73u8],
            [132u8, 176u8, 25u8, 110u8],
            [133u8, 135u8, 244u8, 80u8],
            [145u8, 221u8, 115u8, 70u8],
            [146u8, 238u8, 254u8, 155u8],
            [180u8, 123u8, 47u8, 177u8],
            [214u8, 207u8, 253u8, 30u8],
            [217u8, 202u8, 237u8, 18u8],
            [217u8, 225u8, 127u8, 152u8],
            [221u8, 77u8, 76u8, 246u8],
            [243u8, 254u8, 243u8, 163u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for AngstromCalls {
        const NAME: &'static str = "AngstromCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 23usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::afterSwap(_) => {
                    <afterSwapCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::batchUpdatePools(_) => {
                    <batchUpdatePoolsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::beforeAddLiquidity(_) => {
                    <beforeAddLiquidityCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::beforeRemoveLiquidity(_) => {
                    <beforeRemoveLiquidityCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::beforeSwap(_) => {
                    <beforeSwapCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::collect_unlock_swap_fees(_) => {
                    <collect_unlock_swap_feesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::compose(_) => <composeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::configurePool(_) => {
                    <configurePoolCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deposit_0(_) => {
                    <deposit_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deposit_1(_) => {
                    <deposit_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eip712Domain(_) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::execute(_) => <executeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::extsload(_) => <extsloadCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::initializePool(_) => {
                    <initializePoolCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::invalidateNonce(_) => {
                    <invalidateNonceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::pullFee(_) => <pullFeeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::removePool(_) => {
                    <removePoolCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setController(_) => {
                    <setControllerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::toggleNodes(_) => {
                    <toggleNodesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unlockCallback(_) => {
                    <unlockCallbackCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unlockWithEmptyAttestation(_) => {
                    <unlockWithEmptyAttestationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::withdraw_0(_) => {
                    <withdraw_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::withdraw_1(_) => {
                    <withdraw_1Call as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<AngstromCalls>] = &[
                {
                    fn execute(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <executeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::execute)
                    }
                    execute
                },
                {
                    fn invalidateNonce(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <invalidateNonceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::invalidateNonce)
                    }
                    invalidateNonce
                },
                {
                    fn configurePool(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <configurePoolCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::configurePool)
                    }
                    configurePool
                },
                {
                    fn unlockWithEmptyAttestation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <unlockWithEmptyAttestationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::unlockWithEmptyAttestation)
                    }
                    unlockWithEmptyAttestation
                },
                {
                    fn beforeRemoveLiquidity(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <beforeRemoveLiquidityCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::beforeRemoveLiquidity)
                    }
                    beforeRemoveLiquidity
                },
                {
                    fn beforeAddLiquidity(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <beforeAddLiquidityCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::beforeAddLiquidity)
                    }
                    beforeAddLiquidity
                },
                {
                    fn collect_unlock_swap_fees(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <collect_unlock_swap_feesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::collect_unlock_swap_fees)
                    }
                    collect_unlock_swap_fees
                },
                {
                    fn deposit_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <deposit_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::deposit_0)
                    }
                    deposit_0
                },
                {
                    fn batchUpdatePools(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <batchUpdatePoolsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::batchUpdatePools)
                    }
                    batchUpdatePools
                },
                {
                    fn beforeSwap(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <beforeSwapCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::beforeSwap)
                    }
                    beforeSwap
                },
                {
                    fn compose(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <composeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::compose)
                    }
                    compose
                },
                {
                    fn extsload(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <extsloadCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::extsload)
                    }
                    extsload
                },
                {
                    fn deposit_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <deposit_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::deposit_1)
                    }
                    deposit_1
                },
                {
                    fn eip712Domain(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <eip712DomainCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::eip712Domain)
                    }
                    eip712Domain
                },
                {
                    fn initializePool(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <initializePoolCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::initializePool)
                    }
                    initializePool
                },
                {
                    fn unlockCallback(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <unlockCallbackCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::unlockCallback)
                    }
                    unlockCallback
                },
                {
                    fn setController(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <setControllerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::setController)
                    }
                    setController
                },
                {
                    fn afterSwap(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <afterSwapCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::afterSwap)
                    }
                    afterSwap
                },
                {
                    fn toggleNodes(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <toggleNodesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::toggleNodes)
                    }
                    toggleNodes
                },
                {
                    fn withdraw_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <withdraw_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::withdraw_0)
                    }
                    withdraw_0
                },
                {
                    fn pullFee(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <pullFeeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::pullFee)
                    }
                    pullFee
                },
                {
                    fn removePool(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <removePoolCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::removePool)
                    }
                    removePool
                },
                {
                    fn withdraw_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <withdraw_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::withdraw_1)
                    }
                    withdraw_1
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
                Self::afterSwap(inner) => {
                    <afterSwapCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::batchUpdatePools(inner) => {
                    <batchUpdatePoolsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::beforeAddLiquidity(inner) => {
                    <beforeAddLiquidityCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::beforeRemoveLiquidity(inner) => {
                    <beforeRemoveLiquidityCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::beforeSwap(inner) => {
                    <beforeSwapCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::collect_unlock_swap_fees(inner) => {
                    <collect_unlock_swap_feesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::compose(inner) => {
                    <composeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::configurePool(inner) => {
                    <configurePoolCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::deposit_0(inner) => {
                    <deposit_0Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::deposit_1(inner) => {
                    <deposit_1Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::eip712Domain(inner) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::execute(inner) => {
                    <executeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::extsload(inner) => {
                    <extsloadCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::initializePool(inner) => {
                    <initializePoolCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::invalidateNonce(inner) => {
                    <invalidateNonceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::pullFee(inner) => {
                    <pullFeeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::removePool(inner) => {
                    <removePoolCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::setController(inner) => {
                    <setControllerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::toggleNodes(inner) => {
                    <toggleNodesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::unlockCallback(inner) => {
                    <unlockCallbackCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::unlockWithEmptyAttestation(inner) => {
                    <unlockWithEmptyAttestationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::withdraw_0(inner) => {
                    <withdraw_0Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::withdraw_1(inner) => {
                    <withdraw_1Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::afterSwap(inner) => {
                    <afterSwapCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::batchUpdatePools(inner) => {
                    <batchUpdatePoolsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::beforeAddLiquidity(inner) => {
                    <beforeAddLiquidityCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::beforeRemoveLiquidity(inner) => {
                    <beforeRemoveLiquidityCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::beforeSwap(inner) => {
                    <beforeSwapCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::collect_unlock_swap_fees(inner) => {
                    <collect_unlock_swap_feesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::compose(inner) => {
                    <composeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::configurePool(inner) => {
                    <configurePoolCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::deposit_0(inner) => {
                    <deposit_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::deposit_1(inner) => {
                    <deposit_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::eip712Domain(inner) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::execute(inner) => {
                    <executeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::extsload(inner) => {
                    <extsloadCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::invalidateNonce(inner) => {
                    <invalidateNonceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::pullFee(inner) => {
                    <pullFeeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::removePool(inner) => {
                    <removePoolCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setController(inner) => {
                    <setControllerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::toggleNodes(inner) => {
                    <toggleNodesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::unlockCallback(inner) => {
                    <unlockCallbackCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::unlockWithEmptyAttestation(inner) => {
                    <unlockWithEmptyAttestationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::withdraw_0(inner) => {
                    <withdraw_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::withdraw_1(inner) => {
                    <withdraw_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`Angstrom`](self) custom errors.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum AngstromErrors {
        #[allow(missing_docs)]
        AssetAccessOutOfBounds(AssetAccessOutOfBounds),
        #[allow(missing_docs)]
        AssetsOutOfOrderOrNotUnique(AssetsOutOfOrderOrNotUnique),
        #[allow(missing_docs)]
        AssetsUnordered(AssetsUnordered),
        #[allow(missing_docs)]
        BundlDeltaUnresolved(BundlDeltaUnresolved),
        #[allow(missing_docs)]
        CannotSwapWhileLocked(CannotSwapWhileLocked),
        #[allow(missing_docs)]
        EntryKeyMismatch(EntryKeyMismatch),
        #[allow(missing_docs)]
        Expired(Expired),
        #[allow(missing_docs)]
        FailedToDeployNewStore(FailedToDeployNewStore),
        #[allow(missing_docs)]
        FeeAboveMax(FeeAboveMax),
        #[allow(missing_docs)]
        FillingTooLittle(FillingTooLittle),
        #[allow(missing_docs)]
        FillingTooMuch(FillingTooMuch),
        #[allow(missing_docs)]
        GasAboveMax(GasAboveMax),
        #[allow(missing_docs)]
        IndexMayHaveChanged(IndexMayHaveChanged),
        #[allow(missing_docs)]
        InsufficientCapacity(InsufficientCapacity),
        #[allow(missing_docs)]
        InvalidHookPermissions(InvalidHookPermissions),
        #[allow(missing_docs)]
        InvalidPermitType(InvalidPermitType),
        #[allow(missing_docs)]
        InvalidSignature(InvalidSignature),
        #[allow(missing_docs)]
        InvalidTickSpacing(InvalidTickSpacing),
        #[allow(missing_docs)]
        JustInTimeLiquidityChange(JustInTimeLiquidityChange),
        #[allow(missing_docs)]
        LimitViolated(LimitViolated),
        #[allow(missing_docs)]
        NoEntry(NoEntry),
        #[allow(missing_docs)]
        NonceReuse(NonceReuse),
        #[allow(missing_docs)]
        NotController(NotController),
        #[allow(missing_docs)]
        NotNode(NotNode),
        #[allow(missing_docs)]
        NotUniswap(NotUniswap),
        #[allow(missing_docs)]
        OnlyOncePerBlock(OnlyOncePerBlock),
        #[allow(missing_docs)]
        OrderAlreadyExecuted(OrderAlreadyExecuted),
        #[allow(missing_docs)]
        OutOfOrderOrDuplicatePairs(OutOfOrderOrDuplicatePairs),
        #[allow(missing_docs)]
        Overflow(Overflow),
        #[allow(missing_docs)]
        PairAccessOutOfBounds(PairAccessOutOfBounds),
        #[allow(missing_docs)]
        ReaderNotAtEnd(ReaderNotAtEnd),
        #[allow(missing_docs)]
        ToBGasUsedAboveMax(ToBGasUsedAboveMax),
        #[allow(missing_docs)]
        UnlockDataTooShort(UnlockDataTooShort),
        #[allow(missing_docs)]
        UnlockFeeAboveMax(UnlockFeeAboveMax),
        #[allow(missing_docs)]
        UnlockedFeeNotSet(UnlockedFeeNotSet),
        #[allow(missing_docs)]
        WrongEndLiquidity(WrongEndLiquidity),
    }
    #[automatically_derived]
    impl AngstromErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [1u8, 132u8, 47u8, 140u8],
            [30u8, 129u8, 7u8, 160u8],
            [32u8, 61u8, 130u8, 216u8],
            [35u8, 1u8, 158u8, 103u8],
            [35u8, 246u8, 157u8, 194u8],
            [39u8, 8u8, 21u8, 160u8],
            [41u8, 26u8, 109u8, 8u8],
            [43u8, 174u8, 108u8, 82u8],
            [47u8, 101u8, 158u8, 68u8],
            [50u8, 180u8, 188u8, 147u8],
            [53u8, 39u8, 141u8, 18u8],
            [55u8, 215u8, 253u8, 133u8],
            [68u8, 24u8, 35u8, 49u8],
            [73u8, 38u8, 137u8, 139u8],
            [86u8, 112u8, 37u8, 135u8],
            [92u8, 210u8, 107u8, 104u8],
            [92u8, 239u8, 88u8, 58u8],
            [100u8, 41u8, 207u8, 210u8],
            [102u8, 143u8, 239u8, 27u8],
            [111u8, 29u8, 21u8, 9u8],
            [118u8, 163u8, 249u8, 93u8],
            [128u8, 241u8, 26u8, 207u8],
            [138u8, 46u8, 241u8, 22u8],
            [139u8, 170u8, 87u8, 159u8],
            [140u8, 184u8, 136u8, 114u8],
            [142u8, 30u8, 223u8, 164u8],
            [190u8, 203u8, 25u8, 92u8],
            [196u8, 218u8, 240u8, 3u8],
            [203u8, 19u8, 233u8, 97u8],
            [204u8, 103u8, 175u8, 83u8],
            [216u8, 166u8, 184u8, 155u8],
            [242u8, 31u8, 217u8, 159u8],
            [243u8, 95u8, 147u8, 153u8],
            [246u8, 96u8, 27u8, 80u8],
            [248u8, 50u8, 134u8, 20u8],
            [255u8, 195u8, 30u8, 113u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for AngstromErrors {
        const NAME: &'static str = "AngstromErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 36usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AssetAccessOutOfBounds(_) => {
                    <AssetAccessOutOfBounds as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AssetsOutOfOrderOrNotUnique(_) => {
                    <AssetsOutOfOrderOrNotUnique as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AssetsUnordered(_) => {
                    <AssetsUnordered as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BundlDeltaUnresolved(_) => {
                    <BundlDeltaUnresolved as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CannotSwapWhileLocked(_) => {
                    <CannotSwapWhileLocked as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EntryKeyMismatch(_) => {
                    <EntryKeyMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::Expired(_) => <Expired as alloy_sol_types::SolError>::SELECTOR,
                Self::FailedToDeployNewStore(_) => {
                    <FailedToDeployNewStore as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FeeAboveMax(_) => {
                    <FeeAboveMax as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FillingTooLittle(_) => {
                    <FillingTooLittle as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FillingTooMuch(_) => {
                    <FillingTooMuch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::GasAboveMax(_) => {
                    <GasAboveMax as alloy_sol_types::SolError>::SELECTOR
                }
                Self::IndexMayHaveChanged(_) => {
                    <IndexMayHaveChanged as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientCapacity(_) => {
                    <InsufficientCapacity as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidHookPermissions(_) => {
                    <InvalidHookPermissions as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidPermitType(_) => {
                    <InvalidPermitType as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSignature(_) => {
                    <InvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidTickSpacing(_) => {
                    <InvalidTickSpacing as alloy_sol_types::SolError>::SELECTOR
                }
                Self::JustInTimeLiquidityChange(_) => {
                    <JustInTimeLiquidityChange as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LimitViolated(_) => {
                    <LimitViolated as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NoEntry(_) => <NoEntry as alloy_sol_types::SolError>::SELECTOR,
                Self::NonceReuse(_) => {
                    <NonceReuse as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotController(_) => {
                    <NotController as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotNode(_) => <NotNode as alloy_sol_types::SolError>::SELECTOR,
                Self::NotUniswap(_) => {
                    <NotUniswap as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyOncePerBlock(_) => {
                    <OnlyOncePerBlock as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OrderAlreadyExecuted(_) => {
                    <OrderAlreadyExecuted as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OutOfOrderOrDuplicatePairs(_) => {
                    <OutOfOrderOrDuplicatePairs as alloy_sol_types::SolError>::SELECTOR
                }
                Self::Overflow(_) => <Overflow as alloy_sol_types::SolError>::SELECTOR,
                Self::PairAccessOutOfBounds(_) => {
                    <PairAccessOutOfBounds as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ReaderNotAtEnd(_) => {
                    <ReaderNotAtEnd as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ToBGasUsedAboveMax(_) => {
                    <ToBGasUsedAboveMax as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UnlockDataTooShort(_) => {
                    <UnlockDataTooShort as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UnlockFeeAboveMax(_) => {
                    <UnlockFeeAboveMax as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UnlockedFeeNotSet(_) => {
                    <UnlockedFeeNotSet as alloy_sol_types::SolError>::SELECTOR
                }
                Self::WrongEndLiquidity(_) => {
                    <WrongEndLiquidity as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<AngstromErrors>] = &[
                {
                    fn ReaderNotAtEnd(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <ReaderNotAtEnd as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::ReaderNotAtEnd)
                    }
                    ReaderNotAtEnd
                },
                {
                    fn CannotSwapWhileLocked(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <CannotSwapWhileLocked as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::CannotSwapWhileLocked)
                    }
                    CannotSwapWhileLocked
                },
                {
                    fn Expired(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <Expired as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::Expired)
                    }
                    Expired
                },
                {
                    fn NotController(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <NotController as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::NotController)
                    }
                    NotController
                },
                {
                    fn EntryKeyMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <EntryKeyMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::EntryKeyMismatch)
                    }
                    EntryKeyMismatch
                },
                {
                    fn InvalidTickSpacing(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <InvalidTickSpacing as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::InvalidTickSpacing)
                    }
                    InvalidTickSpacing
                },
                {
                    fn UnlockedFeeNotSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <UnlockedFeeNotSet as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::UnlockedFeeNotSet)
                    }
                    UnlockedFeeNotSet
                },
                {
                    fn ToBGasUsedAboveMax(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <ToBGasUsedAboveMax as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::ToBGasUsedAboveMax)
                    }
                    ToBGasUsedAboveMax
                },
                {
                    fn NoEntry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <NoEntry as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::NoEntry)
                    }
                    NoEntry
                },
                {
                    fn AssetsUnordered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <AssetsUnordered as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::AssetsUnordered)
                    }
                    AssetsUnordered
                },
                {
                    fn Overflow(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <Overflow as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::Overflow)
                    }
                    Overflow
                },
                {
                    fn UnlockFeeAboveMax(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <UnlockFeeAboveMax as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::UnlockFeeAboveMax)
                    }
                    UnlockFeeAboveMax
                },
                {
                    fn FillingTooMuch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <FillingTooMuch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::FillingTooMuch)
                    }
                    FillingTooMuch
                },
                {
                    fn UnlockDataTooShort(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <UnlockDataTooShort as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::UnlockDataTooShort)
                    }
                    UnlockDataTooShort
                },
                {
                    fn FailedToDeployNewStore(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <FailedToDeployNewStore as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::FailedToDeployNewStore)
                    }
                    FailedToDeployNewStore
                },
                {
                    fn NotNode(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <NotNode as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::NotNode)
                    }
                    NotNode
                },
                {
                    fn InsufficientCapacity(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <InsufficientCapacity as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::InsufficientCapacity)
                    }
                    InsufficientCapacity
                },
                {
                    fn WrongEndLiquidity(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <WrongEndLiquidity as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::WrongEndLiquidity)
                    }
                    WrongEndLiquidity
                },
                {
                    fn GasAboveMax(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <GasAboveMax as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::GasAboveMax)
                    }
                    GasAboveMax
                },
                {
                    fn InvalidPermitType(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <InvalidPermitType as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::InvalidPermitType)
                    }
                    InvalidPermitType
                },
                {
                    fn FeeAboveMax(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <FeeAboveMax as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::FeeAboveMax)
                    }
                    FeeAboveMax
                },
                {
                    fn AssetsOutOfOrderOrNotUnique(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <AssetsOutOfOrderOrNotUnique as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::AssetsOutOfOrderOrNotUnique)
                    }
                    AssetsOutOfOrderOrNotUnique
                },
                {
                    fn OrderAlreadyExecuted(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <OrderAlreadyExecuted as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::OrderAlreadyExecuted)
                    }
                    OrderAlreadyExecuted
                },
                {
                    fn InvalidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <InvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::InvalidSignature)
                    }
                    InvalidSignature
                },
                {
                    fn NonceReuse(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <NonceReuse as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::NonceReuse)
                    }
                    NonceReuse
                },
                {
                    fn LimitViolated(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <LimitViolated as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::LimitViolated)
                    }
                    LimitViolated
                },
                {
                    fn JustInTimeLiquidityChange(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <JustInTimeLiquidityChange as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::JustInTimeLiquidityChange)
                    }
                    JustInTimeLiquidityChange
                },
                {
                    fn FillingTooLittle(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <FillingTooLittle as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::FillingTooLittle)
                    }
                    FillingTooLittle
                },
                {
                    fn InvalidHookPermissions(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <InvalidHookPermissions as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::InvalidHookPermissions)
                    }
                    InvalidHookPermissions
                },
                {
                    fn BundlDeltaUnresolved(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <BundlDeltaUnresolved as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::BundlDeltaUnresolved)
                    }
                    BundlDeltaUnresolved
                },
                {
                    fn OnlyOncePerBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <OnlyOncePerBlock as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::OnlyOncePerBlock)
                    }
                    OnlyOncePerBlock
                },
                {
                    fn IndexMayHaveChanged(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <IndexMayHaveChanged as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::IndexMayHaveChanged)
                    }
                    IndexMayHaveChanged
                },
                {
                    fn OutOfOrderOrDuplicatePairs(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <OutOfOrderOrDuplicatePairs as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::OutOfOrderOrDuplicatePairs)
                    }
                    OutOfOrderOrDuplicatePairs
                },
                {
                    fn PairAccessOutOfBounds(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <PairAccessOutOfBounds as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::PairAccessOutOfBounds)
                    }
                    PairAccessOutOfBounds
                },
                {
                    fn NotUniswap(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <NotUniswap as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::NotUniswap)
                    }
                    NotUniswap
                },
                {
                    fn AssetAccessOutOfBounds(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <AssetAccessOutOfBounds as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::AssetAccessOutOfBounds)
                    }
                    AssetAccessOutOfBounds
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
                Self::AssetAccessOutOfBounds(inner) => {
                    <AssetAccessOutOfBounds as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::AssetsOutOfOrderOrNotUnique(inner) => {
                    <AssetsOutOfOrderOrNotUnique as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::AssetsUnordered(inner) => {
                    <AssetsUnordered as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BundlDeltaUnresolved(inner) => {
                    <BundlDeltaUnresolved as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CannotSwapWhileLocked(inner) => {
                    <CannotSwapWhileLocked as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EntryKeyMismatch(inner) => {
                    <EntryKeyMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::Expired(inner) => {
                    <Expired as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::FailedToDeployNewStore(inner) => {
                    <FailedToDeployNewStore as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FeeAboveMax(inner) => {
                    <FeeAboveMax as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::FillingTooLittle(inner) => {
                    <FillingTooLittle as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FillingTooMuch(inner) => {
                    <FillingTooMuch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::GasAboveMax(inner) => {
                    <GasAboveMax as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::IndexMayHaveChanged(inner) => {
                    <IndexMayHaveChanged as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InsufficientCapacity(inner) => {
                    <InsufficientCapacity as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidHookPermissions(inner) => {
                    <InvalidHookPermissions as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidPermitType(inner) => {
                    <InvalidPermitType as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidTickSpacing(inner) => {
                    <InvalidTickSpacing as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::JustInTimeLiquidityChange(inner) => {
                    <JustInTimeLiquidityChange as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LimitViolated(inner) => {
                    <LimitViolated as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::NoEntry(inner) => {
                    <NoEntry as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::NonceReuse(inner) => {
                    <NonceReuse as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::NotController(inner) => {
                    <NotController as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::NotNode(inner) => {
                    <NotNode as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::NotUniswap(inner) => {
                    <NotUniswap as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyOncePerBlock(inner) => {
                    <OnlyOncePerBlock as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OrderAlreadyExecuted(inner) => {
                    <OrderAlreadyExecuted as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OutOfOrderOrDuplicatePairs(inner) => {
                    <OutOfOrderOrDuplicatePairs as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::Overflow(inner) => {
                    <Overflow as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::PairAccessOutOfBounds(inner) => {
                    <PairAccessOutOfBounds as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ReaderNotAtEnd(inner) => {
                    <ReaderNotAtEnd as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ToBGasUsedAboveMax(inner) => {
                    <ToBGasUsedAboveMax as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UnlockDataTooShort(inner) => {
                    <UnlockDataTooShort as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UnlockFeeAboveMax(inner) => {
                    <UnlockFeeAboveMax as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UnlockedFeeNotSet(inner) => {
                    <UnlockedFeeNotSet as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::WrongEndLiquidity(inner) => {
                    <WrongEndLiquidity as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AssetAccessOutOfBounds(inner) => {
                    <AssetAccessOutOfBounds as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::AssetsOutOfOrderOrNotUnique(inner) => {
                    <AssetsOutOfOrderOrNotUnique as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::AssetsUnordered(inner) => {
                    <AssetsUnordered as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BundlDeltaUnresolved(inner) => {
                    <BundlDeltaUnresolved as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CannotSwapWhileLocked(inner) => {
                    <CannotSwapWhileLocked as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EntryKeyMismatch(inner) => {
                    <EntryKeyMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::Expired(inner) => {
                    <Expired as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::FailedToDeployNewStore(inner) => {
                    <FailedToDeployNewStore as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FeeAboveMax(inner) => {
                    <FeeAboveMax as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FillingTooLittle(inner) => {
                    <FillingTooLittle as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FillingTooMuch(inner) => {
                    <FillingTooMuch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::GasAboveMax(inner) => {
                    <GasAboveMax as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::IndexMayHaveChanged(inner) => {
                    <IndexMayHaveChanged as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InsufficientCapacity(inner) => {
                    <InsufficientCapacity as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidHookPermissions(inner) => {
                    <InvalidHookPermissions as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidPermitType(inner) => {
                    <InvalidPermitType as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidTickSpacing(inner) => {
                    <InvalidTickSpacing as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::JustInTimeLiquidityChange(inner) => {
                    <JustInTimeLiquidityChange as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LimitViolated(inner) => {
                    <LimitViolated as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NoEntry(inner) => {
                    <NoEntry as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::NonceReuse(inner) => {
                    <NonceReuse as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::NotController(inner) => {
                    <NotController as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotNode(inner) => {
                    <NotNode as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::NotUniswap(inner) => {
                    <NotUniswap as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OnlyOncePerBlock(inner) => {
                    <OnlyOncePerBlock as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OrderAlreadyExecuted(inner) => {
                    <OrderAlreadyExecuted as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OutOfOrderOrDuplicatePairs(inner) => {
                    <OutOfOrderOrDuplicatePairs as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::Overflow(inner) => {
                    <Overflow as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::PairAccessOutOfBounds(inner) => {
                    <PairAccessOutOfBounds as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ReaderNotAtEnd(inner) => {
                    <ReaderNotAtEnd as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ToBGasUsedAboveMax(inner) => {
                    <ToBGasUsedAboveMax as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UnlockDataTooShort(inner) => {
                    <UnlockDataTooShort as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UnlockFeeAboveMax(inner) => {
                    <UnlockFeeAboveMax as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UnlockedFeeNotSet(inner) => {
                    <UnlockedFeeNotSet as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::WrongEndLiquidity(inner) => {
                    <WrongEndLiquidity as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`Angstrom`](self) contract instance.

See the [wrapper's documentation](`AngstromInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> AngstromInstance<T, P, N> {
        AngstromInstance::<T, P, N>::new(address, provider)
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
        uniV4: alloy::sol_types::private::Address,
        controller: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<AngstromInstance<T, P, N>>,
    > {
        AngstromInstance::<T, P, N>::deploy(provider, uniV4, controller)
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
        uniV4: alloy::sol_types::private::Address,
        controller: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        AngstromInstance::<T, P, N>::deploy_builder(provider, uniV4, controller)
    }
    /**A [`Angstrom`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`Angstrom`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct AngstromInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for AngstromInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("AngstromInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > AngstromInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`Angstrom`](self) contract instance.

See the [wrapper's documentation](`AngstromInstance`) for more details.*/
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
            uniV4: alloy::sol_types::private::Address,
            controller: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<AngstromInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, uniV4, controller);
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
            uniV4: alloy::sol_types::private::Address,
            controller: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            uniV4,
                            controller,
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
    impl<T, P: ::core::clone::Clone, N> AngstromInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> AngstromInstance<T, P, N> {
            AngstromInstance {
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
    > AngstromInstance<T, P, N> {
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
        ///Creates a new call builder for the [`afterSwap`] function.
        pub fn afterSwap(
            &self,
            _0: alloy::sol_types::private::Address,
            key: <PoolKey as alloy::sol_types::SolType>::RustType,
            params: <IPoolManager::SwapParams as alloy::sol_types::SolType>::RustType,
            swap_delta: <BalanceDelta as alloy::sol_types::SolType>::RustType,
            _4: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, afterSwapCall, N> {
            self.call_builder(
                &afterSwapCall {
                    _0,
                    key,
                    params,
                    swap_delta,
                    _4,
                },
            )
        }
        ///Creates a new call builder for the [`batchUpdatePools`] function.
        pub fn batchUpdatePools(
            &self,
            expected_store: <PoolConfigStore as alloy::sol_types::SolType>::RustType,
            updates: alloy::sol_types::private::Vec<
                <ConfigEntryUpdate as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, batchUpdatePoolsCall, N> {
            self.call_builder(
                &batchUpdatePoolsCall {
                    expected_store,
                    updates,
                },
            )
        }
        ///Creates a new call builder for the [`beforeAddLiquidity`] function.
        pub fn beforeAddLiquidity(
            &self,
            sender: alloy::sol_types::private::Address,
            key: <PoolKey as alloy::sol_types::SolType>::RustType,
            params: <IPoolManager::ModifyLiquidityParams as alloy::sol_types::SolType>::RustType,
            _3: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, beforeAddLiquidityCall, N> {
            self.call_builder(
                &beforeAddLiquidityCall {
                    sender,
                    key,
                    params,
                    _3,
                },
            )
        }
        ///Creates a new call builder for the [`beforeRemoveLiquidity`] function.
        pub fn beforeRemoveLiquidity(
            &self,
            sender: alloy::sol_types::private::Address,
            key: <PoolKey as alloy::sol_types::SolType>::RustType,
            params: <IPoolManager::ModifyLiquidityParams as alloy::sol_types::SolType>::RustType,
            _3: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, beforeRemoveLiquidityCall, N> {
            self.call_builder(
                &beforeRemoveLiquidityCall {
                    sender,
                    key,
                    params,
                    _3,
                },
            )
        }
        ///Creates a new call builder for the [`beforeSwap`] function.
        pub fn beforeSwap(
            &self,
            _0: alloy::sol_types::private::Address,
            key: <PoolKey as alloy::sol_types::SolType>::RustType,
            _2: <IPoolManager::SwapParams as alloy::sol_types::SolType>::RustType,
            optionalUnlockData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, beforeSwapCall, N> {
            self.call_builder(
                &beforeSwapCall {
                    _0,
                    key,
                    _2,
                    optionalUnlockData,
                },
            )
        }
        ///Creates a new call builder for the [`collect_unlock_swap_fees`] function.
        pub fn collect_unlock_swap_fees(
            &self,
            to: alloy::sol_types::private::Address,
            packed_assets: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, collect_unlock_swap_feesCall, N> {
            self.call_builder(
                &collect_unlock_swap_feesCall {
                    to,
                    packed_assets,
                },
            )
        }
        ///Creates a new call builder for the [`compose`] function.
        pub fn compose(
            &self,
            from: alloy::sol_types::private::Address,
            payload: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, composeCall, N> {
            self.call_builder(&composeCall { from, payload })
        }
        ///Creates a new call builder for the [`configurePool`] function.
        pub fn configurePool(
            &self,
            asset0: alloy::sol_types::private::Address,
            asset1: alloy::sol_types::private::Address,
            tickSpacing: u16,
            bundleFee: alloy::sol_types::private::primitives::aliases::U24,
            unlockedFee: alloy::sol_types::private::primitives::aliases::U24,
            protocolUnlockedFee: alloy::sol_types::private::primitives::aliases::U24,
        ) -> alloy_contract::SolCallBuilder<T, &P, configurePoolCall, N> {
            self.call_builder(
                &configurePoolCall {
                    asset0,
                    asset1,
                    tickSpacing,
                    bundleFee,
                    unlockedFee,
                    protocolUnlockedFee,
                },
            )
        }
        ///Creates a new call builder for the [`deposit_0`] function.
        pub fn deposit_0(
            &self,
            asset: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, deposit_0Call, N> {
            self.call_builder(&deposit_0Call { asset, amount })
        }
        ///Creates a new call builder for the [`deposit_1`] function.
        pub fn deposit_1(
            &self,
            asset: alloy::sol_types::private::Address,
            to: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, deposit_1Call, N> {
            self.call_builder(&deposit_1Call { asset, to, amount })
        }
        ///Creates a new call builder for the [`eip712Domain`] function.
        pub fn eip712Domain(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, eip712DomainCall, N> {
            self.call_builder(&eip712DomainCall {})
        }
        ///Creates a new call builder for the [`execute`] function.
        pub fn execute(
            &self,
            encoded: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, executeCall, N> {
            self.call_builder(&executeCall { encoded })
        }
        ///Creates a new call builder for the [`extsload`] function.
        pub fn extsload(
            &self,
            slot: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, extsloadCall, N> {
            self.call_builder(&extsloadCall { slot })
        }
        ///Creates a new call builder for the [`initializePool`] function.
        pub fn initializePool(
            &self,
            assetA: alloy::sol_types::private::Address,
            assetB: alloy::sol_types::private::Address,
            storeIndex: alloy::sol_types::private::primitives::aliases::U256,
            sqrtPriceX96: alloy::sol_types::private::primitives::aliases::U160,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializePoolCall, N> {
            self.call_builder(
                &initializePoolCall {
                    assetA,
                    assetB,
                    storeIndex,
                    sqrtPriceX96,
                },
            )
        }
        ///Creates a new call builder for the [`invalidateNonce`] function.
        pub fn invalidateNonce(
            &self,
            nonce: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, invalidateNonceCall, N> {
            self.call_builder(&invalidateNonceCall { nonce })
        }
        ///Creates a new call builder for the [`pullFee`] function.
        pub fn pullFee(
            &self,
            asset: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, pullFeeCall, N> {
            self.call_builder(&pullFeeCall { asset, amount })
        }
        ///Creates a new call builder for the [`removePool`] function.
        pub fn removePool(
            &self,
            key: <StoreKey as alloy::sol_types::SolType>::RustType,
            expected_store: <PoolConfigStore as alloy::sol_types::SolType>::RustType,
            store_index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, removePoolCall, N> {
            self.call_builder(
                &removePoolCall {
                    key,
                    expected_store,
                    store_index,
                },
            )
        }
        ///Creates a new call builder for the [`setController`] function.
        pub fn setController(
            &self,
            newController: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setControllerCall, N> {
            self.call_builder(&setControllerCall { newController })
        }
        ///Creates a new call builder for the [`toggleNodes`] function.
        pub fn toggleNodes(
            &self,
            nodes: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, toggleNodesCall, N> {
            self.call_builder(&toggleNodesCall { nodes })
        }
        ///Creates a new call builder for the [`unlockCallback`] function.
        pub fn unlockCallback(
            &self,
            data: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, unlockCallbackCall, N> {
            self.call_builder(&unlockCallbackCall { data })
        }
        ///Creates a new call builder for the [`unlockWithEmptyAttestation`] function.
        pub fn unlockWithEmptyAttestation(
            &self,
            node: alloy::sol_types::private::Address,
            signature: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, unlockWithEmptyAttestationCall, N> {
            self.call_builder(
                &unlockWithEmptyAttestationCall {
                    node,
                    signature,
                },
            )
        }
        ///Creates a new call builder for the [`withdraw_0`] function.
        pub fn withdraw_0(
            &self,
            asset: alloy::sol_types::private::Address,
            to: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, withdraw_0Call, N> {
            self.call_builder(
                &withdraw_0Call {
                    asset,
                    to,
                    amount,
                },
            )
        }
        ///Creates a new call builder for the [`withdraw_1`] function.
        pub fn withdraw_1(
            &self,
            asset: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, withdraw_1Call, N> {
            self.call_builder(&withdraw_1Call { asset, amount })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > AngstromInstance<T, P, N> {
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
