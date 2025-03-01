///Module containing a contract's types and functions.
/**

```solidity
library BN254 {
    struct G1Point { uint256 X; uint256 Y; }
    struct G2Point { uint256[2] X; uint256[2] Y; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod BN254 {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct G1Point { uint256 X; uint256 Y; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct G1Point {
        #[allow(missing_docs)]
        pub X: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub Y: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<G1Point> for UnderlyingRustTuple<'_> {
            fn from(value: G1Point) -> Self {
                (value.X, value.Y)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for G1Point {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { X: tuple.0, Y: tuple.1 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for G1Point {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for G1Point {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.X),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.Y),
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
        impl alloy_sol_types::SolType for G1Point {
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
        impl alloy_sol_types::SolStruct for G1Point {
            const NAME: &'static str = "G1Point";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed("G1Point(uint256 X,uint256 Y)")
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.X)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.Y)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for G1Point {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.X)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.Y)
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
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.X, out);
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.Y, out);
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
    /**```solidity
struct G2Point { uint256[2] X; uint256[2] Y; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct G2Point {
        #[allow(missing_docs)]
        pub X: [alloy::sol_types::private::primitives::aliases::U256; 2usize],
        #[allow(missing_docs)]
        pub Y: [alloy::sol_types::private::primitives::aliases::U256; 2usize],
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
            alloy::sol_types::sol_data::FixedArray<
                alloy::sol_types::sol_data::Uint<256>,
                2usize,
            >,
            alloy::sol_types::sol_data::FixedArray<
                alloy::sol_types::sol_data::Uint<256>,
                2usize,
            >,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            [alloy::sol_types::private::primitives::aliases::U256; 2usize],
            [alloy::sol_types::private::primitives::aliases::U256; 2usize],
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
        impl ::core::convert::From<G2Point> for UnderlyingRustTuple<'_> {
            fn from(value: G2Point) -> Self {
                (value.X, value.Y)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for G2Point {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { X: tuple.0, Y: tuple.1 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for G2Point {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for G2Point {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::SolType>::tokenize(&self.X),
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::SolType>::tokenize(&self.Y),
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
        impl alloy_sol_types::SolType for G2Point {
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
        impl alloy_sol_types::SolStruct for G2Point {
            const NAME: &'static str = "G2Point";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "G2Point(uint256[2] X,uint256[2] Y)",
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
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.X)
                        .0,
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.Y)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for G2Point {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.X)
                    + <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.Y)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::Uint<256>,
                    2usize,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.X, out);
                <alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::Uint<256>,
                    2usize,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.Y, out);
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
    /**Creates a new wrapper around an on-chain [`BN254`](self) contract instance.

See the [wrapper's documentation](`BN254Instance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> BN254Instance<T, P, N> {
        BN254Instance::<T, P, N>::new(address, provider)
    }
    /**A [`BN254`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`BN254`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct BN254Instance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for BN254Instance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("BN254Instance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > BN254Instance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`BN254`](self) contract instance.

See the [wrapper's documentation](`BN254Instance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> BN254Instance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> BN254Instance<T, P, N> {
            BN254Instance {
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
    > BN254Instance<T, P, N> {
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
    > BN254Instance<T, P, N> {
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
library BN254 {
    struct G1Point {
        uint256 X;
        uint256 Y;
    }
    struct G2Point {
        uint256[2] X;
        uint256[2] Y;
    }
}

interface VotingContract {
    error InvalidTransitionIndex();

    constructor(address _paymentContract);

    function BLS_SIG_CHECKER() external view returns (address);
    function addVoter(address _voter) external;
    function blsSignatureChecker() external view returns (address);
    function currentTotalVotingPower() external view returns (uint256);
    function executeVote() external returns (bool);
    function getCurrentTotalVotingPower(uint256 transitionIndex) external view returns (uint256);
    function getCurrentVotersArray() external view returns (bytes memory);
    function lastVotePassed() external view returns (bool);
    function namespace() external view returns (bytes memory);
    function operatorExecuteVote(uint256 transitionIndex) external view returns (bytes memory);
    function paymentContract() external view returns (address);
    function slashExecVote(bytes32 msgHash, BN254.G1Point memory apk, BN254.G2Point memory apkG2, BN254.G1Point memory sigma, bytes memory storageUpdates, uint256 transitionIndex, address targetAddr, bytes4 targetFunction) external returns (bytes memory);
    function stateTransitionCount() external view returns (uint256 count);
    function voters(uint256) external view returns (address);
    function votersArrayStorage(uint256) external view returns (bytes memory);
    function writeExecuteVote(bytes32 msgHash, BN254.G1Point memory apk, BN254.G2Point memory apkG2, BN254.G1Point memory sigma, bytes memory storageUpdates, uint256 transitionIndex, address targetAddr, bytes4 targetFunction) external payable returns (bytes memory);
    function writeExecuteVoteTest(bytes memory storageUpdates) external payable returns (bytes memory);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_paymentContract",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "BLS_SIG_CHECKER",
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
    "name": "addVoter",
    "inputs": [
      {
        "name": "_voter",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "blsSignatureChecker",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract BLSSignatureChecker"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "currentTotalVotingPower",
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
    "name": "executeVote",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getCurrentTotalVotingPower",
    "inputs": [
      {
        "name": "transitionIndex",
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
    "name": "getCurrentVotersArray",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "lastVotePassed",
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
    "name": "namespace",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "operatorExecuteVote",
    "inputs": [
      {
        "name": "transitionIndex",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "paymentContract",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract PaymentContract"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "slashExecVote",
    "inputs": [
      {
        "name": "msgHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "apk",
        "type": "tuple",
        "internalType": "struct BN254.G1Point",
        "components": [
          {
            "name": "X",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "Y",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "apkG2",
        "type": "tuple",
        "internalType": "struct BN254.G2Point",
        "components": [
          {
            "name": "X",
            "type": "uint256[2]",
            "internalType": "uint256[2]"
          },
          {
            "name": "Y",
            "type": "uint256[2]",
            "internalType": "uint256[2]"
          }
        ]
      },
      {
        "name": "sigma",
        "type": "tuple",
        "internalType": "struct BN254.G1Point",
        "components": [
          {
            "name": "X",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "Y",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "storageUpdates",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "transitionIndex",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "targetAddr",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "targetFunction",
        "type": "bytes4",
        "internalType": "bytes4"
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
    "name": "stateTransitionCount",
    "inputs": [],
    "outputs": [
      {
        "name": "count",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "voters",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
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
    "name": "votersArrayStorage",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "writeExecuteVote",
    "inputs": [
      {
        "name": "msgHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "apk",
        "type": "tuple",
        "internalType": "struct BN254.G1Point",
        "components": [
          {
            "name": "X",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "Y",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "apkG2",
        "type": "tuple",
        "internalType": "struct BN254.G2Point",
        "components": [
          {
            "name": "X",
            "type": "uint256[2]",
            "internalType": "uint256[2]"
          },
          {
            "name": "Y",
            "type": "uint256[2]",
            "internalType": "uint256[2]"
          }
        ]
      },
      {
        "name": "sigma",
        "type": "tuple",
        "internalType": "struct BN254.G1Point",
        "components": [
          {
            "name": "X",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "Y",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "storageUpdates",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "transitionIndex",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "targetAddr",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "targetFunction",
        "type": "bytes4",
        "internalType": "bytes4"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "writeExecuteVoteTest",
    "inputs": [
      {
        "name": "storageUpdates",
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
    "stateMutability": "payable"
  },
  {
    "type": "error",
    "name": "InvalidTransitionIndex",
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
pub mod VotingContract {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60806040523461002f576100196100146100f4565b610649565b610021610034565b612c146107468239612c1490f35b61003a565b60405190565b5f80fd5b601f801991011690565b634e487b7160e01b5f52604160045260245ffd5b906100669061003e565b810190811060018060401b0382111761007e57604052565b610048565b9061009661008f610034565b928361005c565b565b5f80fd5b60018060a01b031690565b6100b09061009c565b90565b6100bc816100a7565b036100c357565b5f80fd5b905051906100d4826100b3565b565b906020828203126100ef576100ec915f016100c7565b90565b610098565b61011261335a8038038061010781610083565b9283398101906100d6565b90565b73ca249215e082e17c12bb3c4881839a3f883e5c6b90565b90565b61014461013f6101499261009c565b61012d565b61009c565b90565b61015590610130565b90565b6101619061014c565b90565b5f1b90565b9061017a60018060a01b0391610164565b9181191691161790565b61018d9061014c565b90565b90565b906101a86101a36101af92610184565b610190565b8254610169565b9055565b6101bc90610130565b90565b6101c8906101b3565b90565b60081b90565b906101e4610100600160a81b03916101cb565b9181191691161790565b6101f7906101b3565b90565b90565b9061021261020d610219926101ee565b6101fa565b82546101d1565b9055565b90565b5f5260205f2090565b634e487b7160e01b5f52603260045260245ffd5b5490565b61024a8161023d565b8210156102645761025c600191610220565b910201905f90565b610229565b1b90565b9190600861028d91029161028760018060a01b0384610269565b92610269565b9181191691161790565b6102a090610130565b90565b6102ac90610297565b90565b90565b91906102c86102c36102d0936102a3565b6102af565b90835461026d565b9055565b908154916801000000000000000083101561030457826102fc91600161030295018155610241565b906102b2565b565b610048565b5490565b60209181520190565b5f5260205f2090565b610328906100a7565b9052565b906103398160209361031f565b0190565b5f1c90565b60018060a01b031690565b61035961035e9161033d565b610342565b90565b61036b905461034d565b90565b60010190565b9061039161038b61038484610309565b809361030d565b92610316565b905f5b8181106103a15750505090565b9091926103c16103bb6001926103b687610361565b61032c565b9461036e565b9101919091610394565b6103e09160208201915f818403910152610374565b90565b90565b6103fa6103f56103ff926103e3565b61012d565b6103e3565b90565b9061040c906103e6565b5f5260205260405f2090565b5190565b634e487b7160e01b5f52602260045260245ffd5b9060016002830492168015610450575b602083101461044b57565b61041c565b91607f1691610440565b5f5260205f2090565b601f602091010490565b919060086104889102916104825f1984610269565b92610269565b9181191691161790565b90565b91906104ab6104a66104b3936103e6565b610492565b90835461046d565b9055565b5f90565b6104cd916104c76104b7565b91610495565b565b5b8181106104db575050565b806104e85f6001936104bb565b016104d0565b9190601f81116104fe575b505050565b61050a61052f9361045a565b90602061051684610463565b83019310610537575b61052890610463565b01906104cf565b5f80806104f9565b91506105288192905061051f565b1c90565b90610559905f1990600802610545565b191690565b8161056891610549565b906002021790565b9061057a81610418565b9060018060401b0382116106385761059c826105968554610430565b856104ee565b602090601f83116001146105d0579180916105bf935f926105c4575b505061055e565b90555b565b90915001515f806105b8565b601f198316916105df8561045a565b925f5b81811061062057509160029391856001969410610606575b505050020190556105c2565b610616910151601f841690610549565b90555f80806105fa565b919360206001819287870151815501950192016105e2565b610048565b9061064791610570565b565b61066f6106769161066a61066361065e610115565b610158565b6003610193565b6101bf565b60026101fd565b6106896106825f61021d565b33906102d4565b7fdebfdfd5a50ad117c10898d68b5ccf0893c6b40d4f443f902e2e7646601bdeaf546001017fdebfdfd5a50ad117c10898d68b5ccf0893c6b40d4f443f902e2e7646601bdeaf556107156106ee6106fd5f6106e2610034565b928391602083016103cb565b6020820181038252038261005c565b610710600461070a610717565b90610402565b61063d565b565b61071f6104b7565b507fdebfdfd5a50ad117c10898d68b5ccf0893c6b40d4f443f902e2e7646601bdeaf549056fe60806040526004361015610013575b610d87565b61001d5f3561012c565b80631c178e9c146101275780631ee07fb1146101225780635ffd1f001461011d5780637c015a89146101185780639c91dd5614610113578063a352d7071461010e578063a4f8815414610109578063aa2c58b214610104578063bfa06830146100ff578063c7b37540146100fa578063cf6ae603146100f5578063da58c7d9146100f0578063ea53bac5146100eb578063f300b862146100e6578063f4833e20146100e1578063f4ab9adf146100dc5763fd8eac490361000e57610d52565b610ce7565b610c8f565b610c5a565b610c25565b610bb5565b610adb565b610aaf565b610a44565b6109de565b610738565b6106e1565b61065b565b6105b7565b6104d2565b61049d565b6101f7565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f91031261014a57565b61013c565b1c90565b60018060a01b031690565b61016e906008610173930261014f565b610153565b90565b90610181915461015e565b90565b61019060035f90610176565b90565b60018060a01b031690565b90565b6101b56101b06101ba92610193565b61019e565b610193565b90565b6101c6906101a1565b90565b6101d2906101bd565b90565b6101de906101c9565b9052565b91906101f5905f602085019401906101d5565b565b3461022757610207366004610140565b610223610212610184565b61021a610132565b918291826101e2565b0390f35b610138565b5f80fd5b90565b61023c81610230565b0361024357565b5f80fd5b9050359061025482610233565b565b9060208282031261026f5761026c915f01610247565b90565b61013c565b61028861028361028d92610230565b61019e565b610230565b90565b9061029a90610274565b5f5260205260405f2090565b634e487b7160e01b5f525f60045260245ffd5b634e487b7160e01b5f52602260045260245ffd5b90600160028304921680156102ed575b60208310146102e857565b6102b9565b91607f16916102dd565b60209181520190565b5f5260205f2090565b905f929180549061032361031c836102cd565b80946102f7565b916001811690815f1461037a575060011461033e575b505050565b61034b9192939450610300565b915f925b81841061036257505001905f8080610339565b6001816020929593955484860152019101929061034f565b92949550505060ff19168252151560200201905f8080610339565b9061039f91610309565b90565b601f801991011690565b634e487b7160e01b5f52604160045260245ffd5b906103ca906103a2565b810190811067ffffffffffffffff8211176103e457604052565b6103ac565b90610409610402926103f9610132565b93848092610395565b03836103c0565b565b905f1061041e5761041b906103e9565b90565b6102a6565b610439906104346004915f92610290565b61040b565b90565b5190565b60209181520190565b90825f9392825e0152565b61047361047c6020936104819361046a8161043c565b93848093610440565b95869101610449565b6103a2565b0190565b61049a9160208201915f818403910152610454565b90565b346104cd576104c96104b86104b3366004610256565b610423565b6104c0610132565b91829182610485565b0390f35b610138565b34610502576104fe6104ed6104e8366004610256565b610ed8565b6104f5610132565b91829182610485565b0390f35b610138565b9061051a610513610132565b92836103c0565b565b67ffffffffffffffff811161053a576105366020916103a2565b0190565b6103ac565b9061055161054c8361051c565b610507565b918252565b5f7f5f434f4d4d4f4e574152455f4147475245474154494f4e5f0000000000000000910152565b610587601861053f565b9061059460208301610556565b565b61059e61057d565b90565b6105a9610596565b90565b6105b46105a1565b90565b346105e7576105c7366004610140565b6105e36105d26105ac565b6105da610132565b91829182610485565b0390f35b610138565b60018060a01b031690565b61060790600861060c930261014f565b6105ec565b90565b9061061a91546105f7565b90565b61062a600260019061060f565b90565b610636906101bd565b90565b6106429061062d565b9052565b9190610659905f60208501940190610639565b565b3461068b5761066b366004610140565b61068761067661061d565b61067e610132565b91829182610646565b0390f35b610138565b73ca249215e082e17c12bb3c4881839a3f883e5c6b90565b6106b0610690565b90565b6106bc90610193565b90565b6106c8906106b3565b9052565b91906106df905f602085019401906106bf565b565b34610711576106f1366004610140565b61070d6106fc6106a8565b610704610132565b918291826106cc565b0390f35b610138565b61071f90610230565b9052565b9190610736905f60208501940190610716565b565b346107685761076461075361074e366004610256565b611165565b61075b610132565b91829182610723565b0390f35b610138565b90565b6107798161076d565b0361078057565b5f80fd5b9050359061079182610770565b565b5f80fd5b91906040838203126107d1576107ca906107b16040610507565b936107be825f8301610247565b5f860152602001610247565b6020830152565b610793565b5f80fd5b67ffffffffffffffff81116107ef5760200290565b6103ac565b5f80fd5b9092919261080d610808826107da565b610507565b93602085920283019281841161084557915b83831061082c5750505050565b6020809161083a8486610247565b81520192019161081f565b6107f4565b9080601f8301121561086557610862916002906107f8565b90565b6107d6565b91906080838203126108a45761089d906108846040610507565b93610891825f830161084a565b5f86015260400161084a565b6020830152565b610793565b5f80fd5b909182601f830112156108e75781359167ffffffffffffffff83116108e25760200192600183028401116108dd57565b6107f4565b6108a9565b6107d6565b6108f5816106b3565b036108fc57565b5f80fd5b9050359061090d826108ec565b565b63ffffffff60e01b1690565b6109248161090f565b0361092b57565b5f80fd5b9050359061093c8261091b565b565b9190916101a0818403126109d957610958835f8301610784565b926109668160208401610797565b92610974826060850161086a565b926109828360e08301610797565b9261012082013567ffffffffffffffff81116109d457816109a49184016108ad565b9290936109d16109b8846101408501610247565b936109c7816101608601610900565b936101800161092f565b90565b61022c565b61013c565b34610a1857610a14610a036109f436600461093e565b9796909695919594929461194b565b610a0b610132565b91829182610485565b0390f35b610138565b151590565b610a2b90610a1d565b9052565b9190610a42905f60208501940190610a22565b565b34610a7457610a54366004610140565b610a70610a5f611a8c565b610a67610132565b91829182610a2f565b0390f35b610138565b90602082820312610aaa575f82013567ffffffffffffffff8111610aa557610aa192016108ad565b9091565b61022c565b61013c565b610ad7610ac6610ac0366004610a79565b9061215c565b610ace610132565b91829182610485565b0390f35b34610b0b57610aeb366004610140565b610b07610af661222e565b610afe610132565b91829182610485565b0390f35b610138565b634e487b7160e01b5f52603260045260245ffd5b5490565b5f5260205f2090565b610b3a81610b24565b821015610b5457610b4c600191610b28565b910201905f90565b610b10565b60018060a01b031690565b610b74906008610b79930261014f565b610b59565b90565b90610b879154610b64565b90565b5f610b9481610b24565b821015610bb157610bae91610ba891610b31565b90610b7c565b90565b5f80fd5b34610be557610be1610bd0610bcb366004610256565b610b8a565b610bd8610132565b918291826106cc565b0390f35b610138565b60ff1690565b610c00906008610c05930261014f565b610bea565b90565b90610c139154610bf0565b90565b610c2260025f90610c08565b90565b34610c5557610c35366004610140565b610c51610c40610c16565b610c48610132565b91829182610a2f565b0390f35b610138565b610c8b610c7a610c6b36600461093e565b97969096959195949294612835565b610c82610132565b91829182610485565b0390f35b34610cbf57610c9f366004610140565b610cbb610caa612851565b610cb2610132565b91829182610723565b0390f35b610138565b90602082820312610cdd57610cda915f01610900565b90565b61013c565b5f0190565b34610d1557610cff610cfa366004610cc4565b612bd3565b610d07610132565b80610d1181610ce2565b0390f35b610138565b90565b610d2d906008610d32930261014f565b610d1a565b90565b90610d409154610d1d565b90565b610d4f60015f90610d35565b90565b34610d8257610d62366004610140565b610d7e610d6d610d43565b610d75610132565b91829182610723565b0390f35b610138565b5f80fd5b606090565b90565b610da7610da2610dac92610d90565b61019e565b610230565b90565b634e487b7160e01b5f52601260045260245ffd5b610dcf610dd591610230565b91610230565b908115610de0570690565b610daf565b90565b610dfc610df7610e0192610de5565b61019e565b610230565b90565b60ff1690565b610e1e610e19610e2392610de5565b61019e565b610e04565b90565b90565b610e3d610e38610e4292610e26565b61019e565b610230565b90565b60f81b90565b610e5490610e45565b90565b610e63610e6891610e04565b610e4b565b9052565b90565b610e7b610e8091610230565b610e6c565b9052565b610ed494610ec460208099989596610ebc828099610eb4600189610eac829b610ecc9d610e57565b018092610e6f565b018092610e6f565b018092610e57565b018092610e6f565b018092610e6f565b0190565b610eea90610ee4610d8b565b50611165565b610f76610f0182610efb6002610d93565b90610dc3565b610f13610f0d5f610de8565b91610230565b14610f67610f205f610e0a565b91610f2b6001610e29565b9490610f365f610e0a565b610f406002610d93565b915f14610f7957610f516001610e29565b925b610f5b610132565b97889660208801610e84565b602082018103825203826103c0565b90565b610f825f610de8565b92610f53565b5f90565b610f95906103e9565b90565b634e487b7160e01b5f52601160045260245ffd5b610fb590610230565b5f8114610fc3576001900390565b610f98565b67ffffffffffffffff8111610fe05760208091020190565b6103ac565b90505190610ff2826108ec565b565b9092919261100961100482610fc8565b610507565b938185526020808601920283019281841161104657915b83831061102d5750505050565b6020809161103b8486610fe5565b815201920191611020565b6107f4565b9080601f830112156110695781602061106693519101610ff4565b90565b6107d6565b9060208282031261109e575f82015167ffffffffffffffff811161109957611096920161104b565b90565b61022c565b61013c565b60016110af9101610230565b90565b5190565b906110c0826110b2565b8110156110d1576020809102010190565b610b10565b6110e090516106b3565b90565b6110ec906101a1565b90565b6111036110fe61110892610193565b61019e565b610230565b90565b61111a61112091939293610230565b92610230565b9161112c838202610230565b92818404149015171561113b57565b610f98565b61114f61115591939293610230565b92610230565b820180921161116057565b610f98565b9061116e610f88565b5061118361117e60048490610290565b610f8c565b82905b61118f8161043c565b6111a161119b5f610de8565b91610230565b148061129d575b156111fb5750806111c16111bb5f610de8565b91610230565b146111ec576111cf90610fac565b916111e46111df60048590610290565b610f8c565b929092611186565b5090506111f85f610de8565b90565b6112199150929192602061120e8261043c565b81830101910161106e565b906112235f610de8565b9161122d5f610de8565b925b8361124a61124461123f856110b2565b610230565b91610230565b10156112965761128a6112909161128461127561127061126b878a906110b6565b6110d6565b6110e3565b61127f87916110ef565b61110b565b90611140565b936110a3565b9261122f565b9250505090565b50816112b16112ab5f610de8565b91610230565b116111a8565b906113109998979695949392917fdebfdfd5a50ad117c10898d68b5ccf0893c6b40d4f443f902e2e7646601bdeaf546001017fdebfdfd5a50ad117c10898d68b5ccf0893c6b40d4f443f902e2e7646601bdeaf556116b1565b90565b905090565b61133d6113349260209261132b8161043c565b94858093611313565b93849101610449565b0190565b60601b90565b61135090611341565b90565b61135c90611347565b90565b61136b611370916106b3565b611353565b9052565b90565b6113836113889161090f565b611374565b9052565b90825f939282370152565b9091826113a7816113ae93611313565b809361138c565b0190565b6004936113e160206113f09997956113d96113d26014966113e998611318565b8092610e6f565b01809261135f565b018092611377565b0191611397565b90565b5f1b90565b61140191611318565b90565b61140c610132565b3d5f823e3d90fd5b5f1c90565b61142561142a91611414565b610153565b90565b6114379054611419565b90565b5f80fd5b60e01b90565b61144d81610a1d565b0361145457565b5f80fd5b9050519061146582611444565b565b919060408382031261148f578061148361148c925f8601611458565b93602001611458565b90565b61013c565b61149d9061076d565b9052565b6114aa90610230565b9052565b906020806114d0936114c65f8201515f8601906114a1565b01519101906114a1565b565b50600290565b905090565b90565b906114ed816020936114a1565b0190565b60200190565b61151361150d611506836114d2565b80946114d8565b916114dd565b5f915b8383106115235750505050565b61153961153360019284516114e0565b926114f1565b92019190611516565b90604060206115659361155b5f8201515f8601906114f7565b01519101906114f7565b565b61159d6115a49461159360e09498979561158961012086019a5f870190611494565b60208501906114ae565b6060830190611542565b01906114ae565b565b6115af906101bd565b90565b5f80fd5b67ffffffffffffffff81116115d4576115d06020916103a2565b0190565b6103ac565b909291926115ee6115e9826115b6565b610507565b9381855260208501908284011161160a5761160892610449565b565b6115b2565b9080601f8301121561162d5781602061162a935191016115d9565b90565b6107d6565b90602082820312611662575f82015167ffffffffffffffff811161165d5761165a920161160f565b90565b61022c565b61013c565b9092919261167c611677826115b6565b610507565b93818552602085019082840111611698576116969261138c565b565b6115b2565b6116a8913691611667565b90565b60200190565b85986116f9896116eb8a9d60209799989b5f979b61170a97506116d26105a1565b9496929091926116e0610132565b9788968c88016113b2565b8682018103825203826103c0565b611701610132565b918291826113f8565b039060025afa156119465761173b6117225f516113f3565b61173461172e8661076d565b9161076d565b1415610a1d565b6119115761177760409361178261175a611755600361142d565b6101c9565b9363171f1d5b92959761176b610132565b9889978896879661143e565b865260048601611567565b03915afa801561190c575f809290916118d9575b506117a19015610a1d565b9081156118c8575b50611897575f6117e3916117bc306115a6565b6117d8635ffd1f006117cc610132565b9586948593849361143e565b835260048301610723565b03915afa9182156118925761184361183d61181161185e9661186d9661184a965f91611870575b509461169d565b61182361181d8261043c565b916116ab565b20926118376118318261043c565b916116ab565b2061076d565b9161076d565b1415610a1d565b611852610132565b92839160208301610a2f565b602082018103825203826103c0565b90565b61188c91503d805f833e61188481836103c0565b810190611632565b5f61180a565b611404565b5050506118b66118c560016118aa610132565b92839160208301610a2f565b602082018103825203826103c0565b90565b6118d3915015610a1d565b5f6117a9565b6117a192506118ff915060403d8111611905575b6118f781836103c0565b810190611467565b91611796565b503d6118ed565b611404565b505050505050506119346119436001611928610132565b92839160208301610a2f565b602082018103825203826103c0565b90565b611404565b90611964989796959493929161195f610d8b565b6112b7565b90565b5f90565b6119bb907fdebfdfd5a50ad117c10898d68b5ccf0893c6b40d4f443f902e2e7646601bdeaf546001017fdebfdfd5a50ad117c10898d68b5ccf0893c6b40d4f443f902e2e7646601bdeaf55611a3c565b90565b906119ca5f19916113f3565b9181191691161790565b90565b906119ec6119e76119f392610274565b6119d4565b82546119be565b9055565b90611a0360ff916113f3565b9181191691161790565b611a1690610a1d565b90565b90565b90611a31611a2c611a3892611a0d565b611a19565b82546119f7565b9055565b50611a6b611a50611a4b612851565b611165565b611a5b8160016119d7565b611a656002610d93565b90610dc3565b611a7d611a775f610de8565b91610230565b14611a89816002611a1c565b90565b611a9c611a97611967565b61196b565b90565b90611af192917fdebfdfd5a50ad117c10898d68b5ccf0893c6b40d4f443f902e2e7646601bdeaf546001017fdebfdfd5a50ad117c10898d68b5ccf0893c6b40d4f443f902e2e7646601bdeaf55611efa565b90565b90565b611b0b611b06611b1092611af4565b61019e565b610230565b90565b60209181520190565b5f7f4d7573742073656e642065786163746c7920302e312045544800000000000000910152565b611b506019602092611b13565b611b5981611b1c565b0190565b611b729060208101905f818303910152611b43565b90565b15611b7c57565b611b84610132565b62461bcd60e51b815280611b9a60048201611b5d565b0390fd5b60081c90565b611bb0611bb591611b9e565b6105ec565b90565b611bc29054611ba4565b90565b5f910312611bcf57565b61013c565b5090565b5f7f496e76616c6964206f70636f6465206f66667365740000000000000000000000910152565b611c0c6015602092611b13565b611c1581611bd8565b0190565b611c2e9060208101905f818303910152611bff565b90565b15611c3857565b611c40610132565b62461bcd60e51b815280611c5660048201611c19565b0390fd5b9190811015611c6a576001020190565b610b10565b60ff60f81b1690565b60f81c90565b611c92611c8d611c9792610e04565b61019e565b610e04565b90565b611ca6611cab91611c78565b611c7e565b90565b611cb790610230565b5f198114611cc55760010190565b610f98565b60207f6520302900000000000000000000000000000000000000000000000000000000917f556e737570706f72746564206f7065726174696f6e20286f70206d75737420625f8201520152565b611d246024604092611b13565b611d2d81611cca565b0190565b611d469060208101905f818303910152611d17565b90565b15611d5057565b611d58610132565b62461bcd60e51b815280611d6e60048201611d31565b0390fd5b90565b611d89611d84611d8e92611d72565b61019e565b610230565b90565b5f7f4d697373696e6720736c6f742064617461000000000000000000000000000000910152565b611dc56011602092611b13565b611dce81611d91565b0190565b611de79060208101905f818303910152611db8565b90565b15611df157565b611df9610132565b62461bcd60e51b815280611e0f60048201611dd2565b0390fd5b5f7f4d697373696e672076616c756520646174610000000000000000000000000000910152565b611e476012602092611b13565b611e5081611e13565b0190565b611e699060208101905f818303910152611e3a565b90565b15611e7357565b611e7b610132565b62461bcd60e51b815280611e9160048201611e54565b0390fd5b611ea1611ea691611414565b610d1a565b90565b611eb39054611e95565b90565b611ec2611ec791611414565b610bea565b90565b611ed49054611eb6565b90565b916020611ef8929493611ef160408201965f830190610716565b0190610a22565b565b92919250611f2234611f1c611f1667016345785d8a0000611af7565b91610230565b14611b75565b611f34611f2f6002611bb8565b61062d565b63d0e30db0349190813b15612157575f91611f5b91611f51610132565b948593849261143e565b825281611f6a60048201610ce2565b03925af1801561215257612126575b50611f835f610de8565b915b82611fa2611f9c611f97858890611bd4565b610230565b91610230565b10156120e157611fe2611fbf84611fb96001610e29565b90611140565b611fdb611fd5611fd0868990611bd4565b610230565b91610230565b1115611c31565b61202461200b612005612000611ffa86898991611c5a565b35611c6f565b611c9a565b94611cae565b9361201e6120185f610e0a565b91610e04565b14611d49565b61205e61203b846120356020611d75565b90611140565b61205761205161204c868990611bd4565b610230565b91610230565b1115611dea565b612066610f88565b5061207f83830135936120796020611d75565b90611140565b6120b9612096826120906020611d75565b90611140565b6120b26120ac6120a7878a90611bd4565b610230565b91610230565b1115611e6c565b6120c1610f88565b506120da81840135916120d46020611d75565b90611140565b9355611f85565b925050506120ef6001611ea9565b6121236120fc6002611eca565b91612114612108610132565b93849260208401611ed7565b602082018103825203826103c0565b90565b612145905f3d811161214b575b61213d81836103c0565b810190611bc5565b5f611f79565b503d612133565b611404565b61143a565b9061216e91612169610d8b565b611a9f565b90565b60209181520190565b612183906106b3565b9052565b906121948160209361217a565b0190565b6121a46121a991611414565b610b59565b90565b6121b69054612198565b90565b60010190565b906121dc6121d66121cf84610b24565b8093612171565b92610b28565b905f5b8181106121ec5750505090565b90919261220c612206600192612201876121ac565b612187565b946121b9565b91019190916121df565b61222b9160208201915f8184039101526121bf565b90565b612236610d8b565b506122526122615f612246610132565b92839160208301612216565b602082018103825203826103c0565b90565b906122bd9998979695949392917fdebfdfd5a50ad117c10898d68b5ccf0893c6b40d4f443f902e2e7646601bdeaf546001017fdebfdfd5a50ad117c10898d68b5ccf0893c6b40d4f443f902e2e7646601bdeaf55612469565b90565b156122c757565b5f6339bb705160e11b8152806122df60048201610ce2565b0390fd5b5f7f496e76616c6964207369676e6174757265000000000000000000000000000000910152565b6123176011602092611b13565b612320816122e3565b0190565b6123399060208101905f81830391015261230a565b90565b1561234357565b61234b610132565b62461bcd60e51b81528061236160048201612324565b0390fd5b5f7f424c532070616972696e6720636865636b206661696c65640000000000000000910152565b6123996018602092611b13565b6123a281612365565b0190565b6123bb9060208101905f81830391015261238c565b90565b156123c557565b6123cd610132565b62461bcd60e51b8152806123e3600482016123a6565b0390fd5b5f7f496e76616c696420424c53207369676e61747572650000000000000000000000910152565b61241b6015602092611b13565b612424816123e7565b0190565b61243d9060208101905f81830391015261240e565b90565b1561244757565b61244f610132565b62461bcd60e51b81528061246560048201612428565b0390fd5b999890959997919497969296506124ac61248d846124876001610e29565b90611140565b6124a66124a061249b612851565b610230565b91610230565b146122c0565b6124d0346124ca6124c467016345785d8a0000611af7565b91610230565b14611b75565b6124e26124dd6002611bb8565b61062d565b9163d0e30db0349390813b15612830575f9161250a91612500610132565b968793849261143e565b82528161251960048201610ce2565b03925af1801561282b5760209461255c61257b948e5f9761256a956127ff575b508d6125436105a1565b949692909192612551610132565b9788968c88016113b2565b8682018103825203826103c0565b612572610132565b918291826113f8565b039060025afa156127fa576125e36040936125b16125995f516113f3565b6125ab6125a58461076d565b9161076d565b1461233c565b6125ee6125c66125c1600361142d565b6101c9565b9363171f1d5b9295976125d7610132565b9889978896879661143e565b865260048601611567565b03915afa9081156127f557612616915f809290916127c2575b50612611906123be565b612440565b61261f5f610de8565b915b8261263e612638612633858890611bd4565b610230565b91610230565b101561277d5761267e61265b846126556001610e29565b90611140565b61267761267161266c868990611bd4565b610230565b91610230565b1115611c31565b6126c06126a76126a161269c61269686898991611c5a565b35611c6f565b611c9a565b94611cae565b936126ba6126b45f610e0a565b91610e04565b14611d49565b6126fa6126d7846126d16020611d75565b90611140565b6126f36126ed6126e8868990611bd4565b610230565b91610230565b1115611dea565b612702610f88565b5061271b83830135936127156020611d75565b90611140565b6127556127328261272c6020611d75565b90611140565b61274e612748612743878a90611bd4565b610230565b91610230565b1115611e6c565b61275d610f88565b5061277681840135916127706020611d75565b90611140565b9355612621565b9250505061278b6001611ea9565b6127bf6127986002611eca565b916127b06127a4610132565b93849260208401611ed7565b602082018103825203826103c0565b90565b61261192506127e8915060403d81116127ee575b6127e081836103c0565b810190611467565b91612607565b503d6127d6565b611404565b611404565b61281e90893d8111612824575b61281681836103c0565b810190611bc5565b5f612539565b503d61280c565b611404565b61143a565b9061284e9897969594939291612849610d8b565b612264565b90565b612859610f88565b507fdebfdfd5a50ad117c10898d68b5ccf0893c6b40d4f443f902e2e7646601bdeaf5490565b6128cf907fdebfdfd5a50ad117c10898d68b5ccf0893c6b40d4f443f902e2e7646601bdeaf546001017fdebfdfd5a50ad117c10898d68b5ccf0893c6b40d4f443f902e2e7646601bdeaf55612b7a565b565b90565b5f5260205f2090565b5490565b6128ea816128dd565b821015612904576128fc6001916128d4565b910201905f90565b610b10565b1b90565b9190600861292d91029161292760018060a01b0384612909565b92612909565b9181191691161790565b612940906101bd565b90565b90565b919061295c61295761296493612937565b612943565b90835461290d565b9055565b90815491680100000000000000008310156129985782612990916001612996950181556128e1565b90612946565b565b6103ac565b601f602091010490565b919060086129c29102916129bc5f1984612909565b92612909565b9181191691161790565b91906129e26129dd6129ea93610274565b6119d4565b9083546129a7565b9055565b612a00916129fa610f88565b916129cc565b565b5b818110612a0e575050565b80612a1b5f6001936129ee565b01612a03565b9190601f8111612a31575b505050565b612a3d612a6293610300565b906020612a498461299d565b83019310612a6a575b612a5b9061299d565b0190612a02565b5f8080612a2c565b9150612a5b81929050612a52565b90612a88905f199060080261014f565b191690565b81612a9791612a78565b906002021790565b90612aa98161043c565b9067ffffffffffffffff8211612b6957612acd82612ac785546102cd565b85612a21565b602090601f8311600114612b0157918091612af0935f92612af5575b5050612a8d565b90555b565b90915001515f80612ae9565b601f19831691612b1085610300565b925f5b818110612b5157509160029391856001969410612b37575b50505002019055612af3565b612b47910151601f841690612a78565b90555f8080612b2b565b91936020600181928787015181550195019201612b13565b6103ac565b90612b7891612a9f565b565b612b8c90612b875f6128d1565b612968565b612bd1612baa612bb95f612b9e610132565b92839160208301612216565b602082018103825203826103c0565b612bcc6004612bc6612851565b90610290565b612b6e565b565b612bdc9061287f565b56fea2646970667358221220baa6f9f1286e16243c6a957ad1c7d318a431b9dcef6d5382dda64d2558fffbba64736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4a\0/Wa\0\x19a\0\x14a\0\xF4V[a\x06IV[a\0!a\x004V[a,\x14a\x07F\x829a,\x14\x90\xF3[a\0:V[`@Q\x90V[_\x80\xFD[`\x1F\x80\x19\x91\x01\x16\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\0f\x90a\0>V[\x81\x01\x90\x81\x10`\x01\x80`@\x1B\x03\x82\x11\x17a\0~W`@RV[a\0HV[\x90a\0\x96a\0\x8Fa\x004V[\x92\x83a\0\\V[V[_\x80\xFD[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\0\xB0\x90a\0\x9CV[\x90V[a\0\xBC\x81a\0\xA7V[\x03a\0\xC3WV[_\x80\xFD[\x90PQ\x90a\0\xD4\x82a\0\xB3V[V[\x90` \x82\x82\x03\x12a\0\xEFWa\0\xEC\x91_\x01a\0\xC7V[\x90V[a\0\x98V[a\x01\x12a3Z\x808\x03\x80a\x01\x07\x81a\0\x83V[\x92\x839\x81\x01\x90a\0\xD6V[\x90V[s\xCA$\x92\x15\xE0\x82\xE1|\x12\xBB<H\x81\x83\x9A?\x88>\\k\x90V[\x90V[a\x01Da\x01?a\x01I\x92a\0\x9CV[a\x01-V[a\0\x9CV[\x90V[a\x01U\x90a\x010V[\x90V[a\x01a\x90a\x01LV[\x90V[_\x1B\x90V[\x90a\x01z`\x01\x80`\xA0\x1B\x03\x91a\x01dV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x01\x8D\x90a\x01LV[\x90V[\x90V[\x90a\x01\xA8a\x01\xA3a\x01\xAF\x92a\x01\x84V[a\x01\x90V[\x82Ta\x01iV[\x90UV[a\x01\xBC\x90a\x010V[\x90V[a\x01\xC8\x90a\x01\xB3V[\x90V[`\x08\x1B\x90V[\x90a\x01\xE4a\x01\0`\x01`\xA8\x1B\x03\x91a\x01\xCBV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x01\xF7\x90a\x01\xB3V[\x90V[\x90V[\x90a\x02\x12a\x02\ra\x02\x19\x92a\x01\xEEV[a\x01\xFAV[\x82Ta\x01\xD1V[\x90UV[\x90V[_R` _ \x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[T\x90V[a\x02J\x81a\x02=V[\x82\x10\x15a\x02dWa\x02\\`\x01\x91a\x02 V[\x91\x02\x01\x90_\x90V[a\x02)V[\x1B\x90V[\x91\x90`\x08a\x02\x8D\x91\x02\x91a\x02\x87`\x01\x80`\xA0\x1B\x03\x84a\x02iV[\x92a\x02iV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x02\xA0\x90a\x010V[\x90V[a\x02\xAC\x90a\x02\x97V[\x90V[\x90V[\x91\x90a\x02\xC8a\x02\xC3a\x02\xD0\x93a\x02\xA3V[a\x02\xAFV[\x90\x83Ta\x02mV[\x90UV[\x90\x81T\x91h\x01\0\0\0\0\0\0\0\0\x83\x10\x15a\x03\x04W\x82a\x02\xFC\x91`\x01a\x03\x02\x95\x01\x81Ua\x02AV[\x90a\x02\xB2V[V[a\0HV[T\x90V[` \x91\x81R\x01\x90V[_R` _ \x90V[a\x03(\x90a\0\xA7V[\x90RV[\x90a\x039\x81` \x93a\x03\x1FV[\x01\x90V[_\x1C\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x03Ya\x03^\x91a\x03=V[a\x03BV[\x90V[a\x03k\x90Ta\x03MV[\x90V[`\x01\x01\x90V[\x90a\x03\x91a\x03\x8Ba\x03\x84\x84a\x03\tV[\x80\x93a\x03\rV[\x92a\x03\x16V[\x90_[\x81\x81\x10a\x03\xA1WPPP\x90V[\x90\x91\x92a\x03\xC1a\x03\xBB`\x01\x92a\x03\xB6\x87a\x03aV[a\x03,V[\x94a\x03nV[\x91\x01\x91\x90\x91a\x03\x94V[a\x03\xE0\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x03tV[\x90V[\x90V[a\x03\xFAa\x03\xF5a\x03\xFF\x92a\x03\xE3V[a\x01-V[a\x03\xE3V[\x90V[\x90a\x04\x0C\x90a\x03\xE6V[_R` R`@_ \x90V[Q\x90V[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x90`\x01`\x02\x83\x04\x92\x16\x80\x15a\x04PW[` \x83\x10\x14a\x04KWV[a\x04\x1CV[\x91`\x7F\x16\x91a\x04@V[_R` _ \x90V[`\x1F` \x91\x01\x04\x90V[\x91\x90`\x08a\x04\x88\x91\x02\x91a\x04\x82_\x19\x84a\x02iV[\x92a\x02iV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[\x91\x90a\x04\xABa\x04\xA6a\x04\xB3\x93a\x03\xE6V[a\x04\x92V[\x90\x83Ta\x04mV[\x90UV[_\x90V[a\x04\xCD\x91a\x04\xC7a\x04\xB7V[\x91a\x04\x95V[V[[\x81\x81\x10a\x04\xDBWPPV[\x80a\x04\xE8_`\x01\x93a\x04\xBBV[\x01a\x04\xD0V[\x91\x90`\x1F\x81\x11a\x04\xFEW[PPPV[a\x05\na\x05/\x93a\x04ZV[\x90` a\x05\x16\x84a\x04cV[\x83\x01\x93\x10a\x057W[a\x05(\x90a\x04cV[\x01\x90a\x04\xCFV[_\x80\x80a\x04\xF9V[\x91Pa\x05(\x81\x92\x90Pa\x05\x1FV[\x1C\x90V[\x90a\x05Y\x90_\x19\x90`\x08\x02a\x05EV[\x19\x16\x90V[\x81a\x05h\x91a\x05IV[\x90`\x02\x02\x17\x90V[\x90a\x05z\x81a\x04\x18V[\x90`\x01\x80`@\x1B\x03\x82\x11a\x068Wa\x05\x9C\x82a\x05\x96\x85Ta\x040V[\x85a\x04\xEEV[` \x90`\x1F\x83\x11`\x01\x14a\x05\xD0W\x91\x80\x91a\x05\xBF\x93_\x92a\x05\xC4W[PPa\x05^V[\x90U[V[\x90\x91P\x01Q_\x80a\x05\xB8V[`\x1F\x19\x83\x16\x91a\x05\xDF\x85a\x04ZV[\x92_[\x81\x81\x10a\x06 WP\x91`\x02\x93\x91\x85`\x01\x96\x94\x10a\x06\x06W[PPP\x02\x01\x90Ua\x05\xC2V[a\x06\x16\x91\x01Q`\x1F\x84\x16\x90a\x05IV[\x90U_\x80\x80a\x05\xFAV[\x91\x93` `\x01\x81\x92\x87\x87\x01Q\x81U\x01\x95\x01\x92\x01a\x05\xE2V[a\0HV[\x90a\x06G\x91a\x05pV[V[a\x06oa\x06v\x91a\x06ja\x06ca\x06^a\x01\x15V[a\x01XV[`\x03a\x01\x93V[a\x01\xBFV[`\x02a\x01\xFDV[a\x06\x89a\x06\x82_a\x02\x1DV[3\x90a\x02\xD4V[\x7F\xDE\xBF\xDF\xD5\xA5\n\xD1\x17\xC1\x08\x98\xD6\x8B\\\xCF\x08\x93\xC6\xB4\rOD?\x90..vF`\x1B\xDE\xAFT`\x01\x01\x7F\xDE\xBF\xDF\xD5\xA5\n\xD1\x17\xC1\x08\x98\xD6\x8B\\\xCF\x08\x93\xC6\xB4\rOD?\x90..vF`\x1B\xDE\xAFUa\x07\x15a\x06\xEEa\x06\xFD_a\x06\xE2a\x004V[\x92\x83\x91` \x83\x01a\x03\xCBV[` \x82\x01\x81\x03\x82R\x03\x82a\0\\V[a\x07\x10`\x04a\x07\na\x07\x17V[\x90a\x04\x02V[a\x06=V[V[a\x07\x1Fa\x04\xB7V[P\x7F\xDE\xBF\xDF\xD5\xA5\n\xD1\x17\xC1\x08\x98\xD6\x8B\\\xCF\x08\x93\xC6\xB4\rOD?\x90..vF`\x1B\xDE\xAFT\x90V\xFE`\x80`@R`\x046\x10\x15a\0\x13W[a\r\x87V[a\0\x1D_5a\x01,V[\x80c\x1C\x17\x8E\x9C\x14a\x01'W\x80c\x1E\xE0\x7F\xB1\x14a\x01\"W\x80c_\xFD\x1F\0\x14a\x01\x1DW\x80c|\x01Z\x89\x14a\x01\x18W\x80c\x9C\x91\xDDV\x14a\x01\x13W\x80c\xA3R\xD7\x07\x14a\x01\x0EW\x80c\xA4\xF8\x81T\x14a\x01\tW\x80c\xAA,X\xB2\x14a\x01\x04W\x80c\xBF\xA0h0\x14a\0\xFFW\x80c\xC7\xB3u@\x14a\0\xFAW\x80c\xCFj\xE6\x03\x14a\0\xF5W\x80c\xDAX\xC7\xD9\x14a\0\xF0W\x80c\xEAS\xBA\xC5\x14a\0\xEBW\x80c\xF3\0\xB8b\x14a\0\xE6W\x80c\xF4\x83> \x14a\0\xE1W\x80c\xF4\xAB\x9A\xDF\x14a\0\xDCWc\xFD\x8E\xACI\x03a\0\x0EWa\rRV[a\x0C\xE7V[a\x0C\x8FV[a\x0CZV[a\x0C%V[a\x0B\xB5V[a\n\xDBV[a\n\xAFV[a\nDV[a\t\xDEV[a\x078V[a\x06\xE1V[a\x06[V[a\x05\xB7V[a\x04\xD2V[a\x04\x9DV[a\x01\xF7V[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x91\x03\x12a\x01JWV[a\x01<V[\x1C\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x01n\x90`\x08a\x01s\x93\x02a\x01OV[a\x01SV[\x90V[\x90a\x01\x81\x91Ta\x01^V[\x90V[a\x01\x90`\x03_\x90a\x01vV[\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[\x90V[a\x01\xB5a\x01\xB0a\x01\xBA\x92a\x01\x93V[a\x01\x9EV[a\x01\x93V[\x90V[a\x01\xC6\x90a\x01\xA1V[\x90V[a\x01\xD2\x90a\x01\xBDV[\x90V[a\x01\xDE\x90a\x01\xC9V[\x90RV[\x91\x90a\x01\xF5\x90_` \x85\x01\x94\x01\x90a\x01\xD5V[V[4a\x02'Wa\x02\x076`\x04a\x01@V[a\x02#a\x02\x12a\x01\x84V[a\x02\x1Aa\x012V[\x91\x82\x91\x82a\x01\xE2V[\x03\x90\xF3[a\x018V[_\x80\xFD[\x90V[a\x02<\x81a\x020V[\x03a\x02CWV[_\x80\xFD[\x90P5\x90a\x02T\x82a\x023V[V[\x90` \x82\x82\x03\x12a\x02oWa\x02l\x91_\x01a\x02GV[\x90V[a\x01<V[a\x02\x88a\x02\x83a\x02\x8D\x92a\x020V[a\x01\x9EV[a\x020V[\x90V[\x90a\x02\x9A\x90a\x02tV[_R` R`@_ \x90V[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x90`\x01`\x02\x83\x04\x92\x16\x80\x15a\x02\xEDW[` \x83\x10\x14a\x02\xE8WV[a\x02\xB9V[\x91`\x7F\x16\x91a\x02\xDDV[` \x91\x81R\x01\x90V[_R` _ \x90V[\x90_\x92\x91\x80T\x90a\x03#a\x03\x1C\x83a\x02\xCDV[\x80\x94a\x02\xF7V[\x91`\x01\x81\x16\x90\x81_\x14a\x03zWP`\x01\x14a\x03>W[PPPV[a\x03K\x91\x92\x93\x94Pa\x03\0V[\x91_\x92[\x81\x84\x10a\x03bWPP\x01\x90_\x80\x80a\x039V[`\x01\x81` \x92\x95\x93\x95T\x84\x86\x01R\x01\x91\x01\x92\x90a\x03OV[\x92\x94\x95PPP`\xFF\x19\x16\x82R\x15\x15` \x02\x01\x90_\x80\x80a\x039V[\x90a\x03\x9F\x91a\x03\tV[\x90V[`\x1F\x80\x19\x91\x01\x16\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\x03\xCA\x90a\x03\xA2V[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\xE4W`@RV[a\x03\xACV[\x90a\x04\ta\x04\x02\x92a\x03\xF9a\x012V[\x93\x84\x80\x92a\x03\x95V[\x03\x83a\x03\xC0V[V[\x90_\x10a\x04\x1EWa\x04\x1B\x90a\x03\xE9V[\x90V[a\x02\xA6V[a\x049\x90a\x044`\x04\x91_\x92a\x02\x90V[a\x04\x0BV[\x90V[Q\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[a\x04sa\x04|` \x93a\x04\x81\x93a\x04j\x81a\x04<V[\x93\x84\x80\x93a\x04@V[\x95\x86\x91\x01a\x04IV[a\x03\xA2V[\x01\x90V[a\x04\x9A\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x04TV[\x90V[4a\x04\xCDWa\x04\xC9a\x04\xB8a\x04\xB36`\x04a\x02VV[a\x04#V[a\x04\xC0a\x012V[\x91\x82\x91\x82a\x04\x85V[\x03\x90\xF3[a\x018V[4a\x05\x02Wa\x04\xFEa\x04\xEDa\x04\xE86`\x04a\x02VV[a\x0E\xD8V[a\x04\xF5a\x012V[\x91\x82\x91\x82a\x04\x85V[\x03\x90\xF3[a\x018V[\x90a\x05\x1Aa\x05\x13a\x012V[\x92\x83a\x03\xC0V[V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x05:Wa\x056` \x91a\x03\xA2V[\x01\x90V[a\x03\xACV[\x90a\x05Qa\x05L\x83a\x05\x1CV[a\x05\x07V[\x91\x82RV[_\x7F_COMMONWARE_AGGREGATION_\0\0\0\0\0\0\0\0\x91\x01RV[a\x05\x87`\x18a\x05?V[\x90a\x05\x94` \x83\x01a\x05VV[V[a\x05\x9Ea\x05}V[\x90V[a\x05\xA9a\x05\x96V[\x90V[a\x05\xB4a\x05\xA1V[\x90V[4a\x05\xE7Wa\x05\xC76`\x04a\x01@V[a\x05\xE3a\x05\xD2a\x05\xACV[a\x05\xDAa\x012V[\x91\x82\x91\x82a\x04\x85V[\x03\x90\xF3[a\x018V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x06\x07\x90`\x08a\x06\x0C\x93\x02a\x01OV[a\x05\xECV[\x90V[\x90a\x06\x1A\x91Ta\x05\xF7V[\x90V[a\x06*`\x02`\x01\x90a\x06\x0FV[\x90V[a\x066\x90a\x01\xBDV[\x90V[a\x06B\x90a\x06-V[\x90RV[\x91\x90a\x06Y\x90_` \x85\x01\x94\x01\x90a\x069V[V[4a\x06\x8BWa\x06k6`\x04a\x01@V[a\x06\x87a\x06va\x06\x1DV[a\x06~a\x012V[\x91\x82\x91\x82a\x06FV[\x03\x90\xF3[a\x018V[s\xCA$\x92\x15\xE0\x82\xE1|\x12\xBB<H\x81\x83\x9A?\x88>\\k\x90V[a\x06\xB0a\x06\x90V[\x90V[a\x06\xBC\x90a\x01\x93V[\x90V[a\x06\xC8\x90a\x06\xB3V[\x90RV[\x91\x90a\x06\xDF\x90_` \x85\x01\x94\x01\x90a\x06\xBFV[V[4a\x07\x11Wa\x06\xF16`\x04a\x01@V[a\x07\ra\x06\xFCa\x06\xA8V[a\x07\x04a\x012V[\x91\x82\x91\x82a\x06\xCCV[\x03\x90\xF3[a\x018V[a\x07\x1F\x90a\x020V[\x90RV[\x91\x90a\x076\x90_` \x85\x01\x94\x01\x90a\x07\x16V[V[4a\x07hWa\x07da\x07Sa\x07N6`\x04a\x02VV[a\x11eV[a\x07[a\x012V[\x91\x82\x91\x82a\x07#V[\x03\x90\xF3[a\x018V[\x90V[a\x07y\x81a\x07mV[\x03a\x07\x80WV[_\x80\xFD[\x90P5\x90a\x07\x91\x82a\x07pV[V[_\x80\xFD[\x91\x90`@\x83\x82\x03\x12a\x07\xD1Wa\x07\xCA\x90a\x07\xB1`@a\x05\x07V[\x93a\x07\xBE\x82_\x83\x01a\x02GV[_\x86\x01R` \x01a\x02GV[` \x83\x01RV[a\x07\x93V[_\x80\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07\xEFW` \x02\x90V[a\x03\xACV[_\x80\xFD[\x90\x92\x91\x92a\x08\ra\x08\x08\x82a\x07\xDAV[a\x05\x07V[\x93` \x85\x92\x02\x83\x01\x92\x81\x84\x11a\x08EW\x91[\x83\x83\x10a\x08,WPPPPV[` \x80\x91a\x08:\x84\x86a\x02GV[\x81R\x01\x92\x01\x91a\x08\x1FV[a\x07\xF4V[\x90\x80`\x1F\x83\x01\x12\x15a\x08eWa\x08b\x91`\x02\x90a\x07\xF8V[\x90V[a\x07\xD6V[\x91\x90`\x80\x83\x82\x03\x12a\x08\xA4Wa\x08\x9D\x90a\x08\x84`@a\x05\x07V[\x93a\x08\x91\x82_\x83\x01a\x08JV[_\x86\x01R`@\x01a\x08JV[` \x83\x01RV[a\x07\x93V[_\x80\xFD[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x08\xE7W\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x08\xE2W` \x01\x92`\x01\x83\x02\x84\x01\x11a\x08\xDDWV[a\x07\xF4V[a\x08\xA9V[a\x07\xD6V[a\x08\xF5\x81a\x06\xB3V[\x03a\x08\xFCWV[_\x80\xFD[\x90P5\x90a\t\r\x82a\x08\xECV[V[c\xFF\xFF\xFF\xFF`\xE0\x1B\x16\x90V[a\t$\x81a\t\x0FV[\x03a\t+WV[_\x80\xFD[\x90P5\x90a\t<\x82a\t\x1BV[V[\x91\x90\x91a\x01\xA0\x81\x84\x03\x12a\t\xD9Wa\tX\x83_\x83\x01a\x07\x84V[\x92a\tf\x81` \x84\x01a\x07\x97V[\x92a\tt\x82``\x85\x01a\x08jV[\x92a\t\x82\x83`\xE0\x83\x01a\x07\x97V[\x92a\x01 \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\t\xD4W\x81a\t\xA4\x91\x84\x01a\x08\xADV[\x92\x90\x93a\t\xD1a\t\xB8\x84a\x01@\x85\x01a\x02GV[\x93a\t\xC7\x81a\x01`\x86\x01a\t\0V[\x93a\x01\x80\x01a\t/V[\x90V[a\x02,V[a\x01<V[4a\n\x18Wa\n\x14a\n\x03a\t\xF46`\x04a\t>V[\x97\x96\x90\x96\x95\x91\x95\x94\x92\x94a\x19KV[a\n\x0Ba\x012V[\x91\x82\x91\x82a\x04\x85V[\x03\x90\xF3[a\x018V[\x15\x15\x90V[a\n+\x90a\n\x1DV[\x90RV[\x91\x90a\nB\x90_` \x85\x01\x94\x01\x90a\n\"V[V[4a\ntWa\nT6`\x04a\x01@V[a\npa\n_a\x1A\x8CV[a\nga\x012V[\x91\x82\x91\x82a\n/V[\x03\x90\xF3[a\x018V[\x90` \x82\x82\x03\x12a\n\xAAW_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\n\xA5Wa\n\xA1\x92\x01a\x08\xADV[\x90\x91V[a\x02,V[a\x01<V[a\n\xD7a\n\xC6a\n\xC06`\x04a\nyV[\x90a!\\V[a\n\xCEa\x012V[\x91\x82\x91\x82a\x04\x85V[\x03\x90\xF3[4a\x0B\x0BWa\n\xEB6`\x04a\x01@V[a\x0B\x07a\n\xF6a\".V[a\n\xFEa\x012V[\x91\x82\x91\x82a\x04\x85V[\x03\x90\xF3[a\x018V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[T\x90V[_R` _ \x90V[a\x0B:\x81a\x0B$V[\x82\x10\x15a\x0BTWa\x0BL`\x01\x91a\x0B(V[\x91\x02\x01\x90_\x90V[a\x0B\x10V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x0Bt\x90`\x08a\x0By\x93\x02a\x01OV[a\x0BYV[\x90V[\x90a\x0B\x87\x91Ta\x0BdV[\x90V[_a\x0B\x94\x81a\x0B$V[\x82\x10\x15a\x0B\xB1Wa\x0B\xAE\x91a\x0B\xA8\x91a\x0B1V[\x90a\x0B|V[\x90V[_\x80\xFD[4a\x0B\xE5Wa\x0B\xE1a\x0B\xD0a\x0B\xCB6`\x04a\x02VV[a\x0B\x8AV[a\x0B\xD8a\x012V[\x91\x82\x91\x82a\x06\xCCV[\x03\x90\xF3[a\x018V[`\xFF\x16\x90V[a\x0C\0\x90`\x08a\x0C\x05\x93\x02a\x01OV[a\x0B\xEAV[\x90V[\x90a\x0C\x13\x91Ta\x0B\xF0V[\x90V[a\x0C\"`\x02_\x90a\x0C\x08V[\x90V[4a\x0CUWa\x0C56`\x04a\x01@V[a\x0CQa\x0C@a\x0C\x16V[a\x0CHa\x012V[\x91\x82\x91\x82a\n/V[\x03\x90\xF3[a\x018V[a\x0C\x8Ba\x0Cza\x0Ck6`\x04a\t>V[\x97\x96\x90\x96\x95\x91\x95\x94\x92\x94a(5V[a\x0C\x82a\x012V[\x91\x82\x91\x82a\x04\x85V[\x03\x90\xF3[4a\x0C\xBFWa\x0C\x9F6`\x04a\x01@V[a\x0C\xBBa\x0C\xAAa(QV[a\x0C\xB2a\x012V[\x91\x82\x91\x82a\x07#V[\x03\x90\xF3[a\x018V[\x90` \x82\x82\x03\x12a\x0C\xDDWa\x0C\xDA\x91_\x01a\t\0V[\x90V[a\x01<V[_\x01\x90V[4a\r\x15Wa\x0C\xFFa\x0C\xFA6`\x04a\x0C\xC4V[a+\xD3V[a\r\x07a\x012V[\x80a\r\x11\x81a\x0C\xE2V[\x03\x90\xF3[a\x018V[\x90V[a\r-\x90`\x08a\r2\x93\x02a\x01OV[a\r\x1AV[\x90V[\x90a\r@\x91Ta\r\x1DV[\x90V[a\rO`\x01_\x90a\r5V[\x90V[4a\r\x82Wa\rb6`\x04a\x01@V[a\r~a\rma\rCV[a\rua\x012V[\x91\x82\x91\x82a\x07#V[\x03\x90\xF3[a\x018V[_\x80\xFD[``\x90V[\x90V[a\r\xA7a\r\xA2a\r\xAC\x92a\r\x90V[a\x01\x9EV[a\x020V[\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a\r\xCFa\r\xD5\x91a\x020V[\x91a\x020V[\x90\x81\x15a\r\xE0W\x06\x90V[a\r\xAFV[\x90V[a\r\xFCa\r\xF7a\x0E\x01\x92a\r\xE5V[a\x01\x9EV[a\x020V[\x90V[`\xFF\x16\x90V[a\x0E\x1Ea\x0E\x19a\x0E#\x92a\r\xE5V[a\x01\x9EV[a\x0E\x04V[\x90V[\x90V[a\x0E=a\x0E8a\x0EB\x92a\x0E&V[a\x01\x9EV[a\x020V[\x90V[`\xF8\x1B\x90V[a\x0ET\x90a\x0EEV[\x90V[a\x0Eca\x0Eh\x91a\x0E\x04V[a\x0EKV[\x90RV[\x90V[a\x0E{a\x0E\x80\x91a\x020V[a\x0ElV[\x90RV[a\x0E\xD4\x94a\x0E\xC4` \x80\x99\x98\x95\x96a\x0E\xBC\x82\x80\x99a\x0E\xB4`\x01\x89a\x0E\xAC\x82\x9Ba\x0E\xCC\x9Da\x0EWV[\x01\x80\x92a\x0EoV[\x01\x80\x92a\x0EoV[\x01\x80\x92a\x0EWV[\x01\x80\x92a\x0EoV[\x01\x80\x92a\x0EoV[\x01\x90V[a\x0E\xEA\x90a\x0E\xE4a\r\x8BV[Pa\x11eV[a\x0Fva\x0F\x01\x82a\x0E\xFB`\x02a\r\x93V[\x90a\r\xC3V[a\x0F\x13a\x0F\r_a\r\xE8V[\x91a\x020V[\x14a\x0Fga\x0F _a\x0E\nV[\x91a\x0F+`\x01a\x0E)V[\x94\x90a\x0F6_a\x0E\nV[a\x0F@`\x02a\r\x93V[\x91_\x14a\x0FyWa\x0FQ`\x01a\x0E)V[\x92[a\x0F[a\x012V[\x97\x88\x96` \x88\x01a\x0E\x84V[` \x82\x01\x81\x03\x82R\x03\x82a\x03\xC0V[\x90V[a\x0F\x82_a\r\xE8V[\x92a\x0FSV[_\x90V[a\x0F\x95\x90a\x03\xE9V[\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x0F\xB5\x90a\x020V[_\x81\x14a\x0F\xC3W`\x01\x90\x03\x90V[a\x0F\x98V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0F\xE0W` \x80\x91\x02\x01\x90V[a\x03\xACV[\x90PQ\x90a\x0F\xF2\x82a\x08\xECV[V[\x90\x92\x91\x92a\x10\ta\x10\x04\x82a\x0F\xC8V[a\x05\x07V[\x93\x81\x85R` \x80\x86\x01\x92\x02\x83\x01\x92\x81\x84\x11a\x10FW\x91[\x83\x83\x10a\x10-WPPPPV[` \x80\x91a\x10;\x84\x86a\x0F\xE5V[\x81R\x01\x92\x01\x91a\x10 V[a\x07\xF4V[\x90\x80`\x1F\x83\x01\x12\x15a\x10iW\x81` a\x10f\x93Q\x91\x01a\x0F\xF4V[\x90V[a\x07\xD6V[\x90` \x82\x82\x03\x12a\x10\x9EW_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x10\x99Wa\x10\x96\x92\x01a\x10KV[\x90V[a\x02,V[a\x01<V[`\x01a\x10\xAF\x91\x01a\x020V[\x90V[Q\x90V[\x90a\x10\xC0\x82a\x10\xB2V[\x81\x10\x15a\x10\xD1W` \x80\x91\x02\x01\x01\x90V[a\x0B\x10V[a\x10\xE0\x90Qa\x06\xB3V[\x90V[a\x10\xEC\x90a\x01\xA1V[\x90V[a\x11\x03a\x10\xFEa\x11\x08\x92a\x01\x93V[a\x01\x9EV[a\x020V[\x90V[a\x11\x1Aa\x11 \x91\x93\x92\x93a\x020V[\x92a\x020V[\x91a\x11,\x83\x82\x02a\x020V[\x92\x81\x84\x04\x14\x90\x15\x17\x15a\x11;WV[a\x0F\x98V[a\x11Oa\x11U\x91\x93\x92\x93a\x020V[\x92a\x020V[\x82\x01\x80\x92\x11a\x11`WV[a\x0F\x98V[\x90a\x11na\x0F\x88V[Pa\x11\x83a\x11~`\x04\x84\x90a\x02\x90V[a\x0F\x8CV[\x82\x90[a\x11\x8F\x81a\x04<V[a\x11\xA1a\x11\x9B_a\r\xE8V[\x91a\x020V[\x14\x80a\x12\x9DW[\x15a\x11\xFBWP\x80a\x11\xC1a\x11\xBB_a\r\xE8V[\x91a\x020V[\x14a\x11\xECWa\x11\xCF\x90a\x0F\xACV[\x91a\x11\xE4a\x11\xDF`\x04\x85\x90a\x02\x90V[a\x0F\x8CV[\x92\x90\x92a\x11\x86V[P\x90Pa\x11\xF8_a\r\xE8V[\x90V[a\x12\x19\x91P\x92\x91\x92` a\x12\x0E\x82a\x04<V[\x81\x83\x01\x01\x91\x01a\x10nV[\x90a\x12#_a\r\xE8V[\x91a\x12-_a\r\xE8V[\x92[\x83a\x12Ja\x12Da\x12?\x85a\x10\xB2V[a\x020V[\x91a\x020V[\x10\x15a\x12\x96Wa\x12\x8Aa\x12\x90\x91a\x12\x84a\x12ua\x12pa\x12k\x87\x8A\x90a\x10\xB6V[a\x10\xD6V[a\x10\xE3V[a\x12\x7F\x87\x91a\x10\xEFV[a\x11\x0BV[\x90a\x11@V[\x93a\x10\xA3V[\x92a\x12/V[\x92PPP\x90V[P\x81a\x12\xB1a\x12\xAB_a\r\xE8V[\x91a\x020V[\x11a\x11\xA8V[\x90a\x13\x10\x99\x98\x97\x96\x95\x94\x93\x92\x91\x7F\xDE\xBF\xDF\xD5\xA5\n\xD1\x17\xC1\x08\x98\xD6\x8B\\\xCF\x08\x93\xC6\xB4\rOD?\x90..vF`\x1B\xDE\xAFT`\x01\x01\x7F\xDE\xBF\xDF\xD5\xA5\n\xD1\x17\xC1\x08\x98\xD6\x8B\\\xCF\x08\x93\xC6\xB4\rOD?\x90..vF`\x1B\xDE\xAFUa\x16\xB1V[\x90V[\x90P\x90V[a\x13=a\x134\x92` \x92a\x13+\x81a\x04<V[\x94\x85\x80\x93a\x13\x13V[\x93\x84\x91\x01a\x04IV[\x01\x90V[``\x1B\x90V[a\x13P\x90a\x13AV[\x90V[a\x13\\\x90a\x13GV[\x90V[a\x13ka\x13p\x91a\x06\xB3V[a\x13SV[\x90RV[\x90V[a\x13\x83a\x13\x88\x91a\t\x0FV[a\x13tV[\x90RV[\x90\x82_\x93\x92\x827\x01RV[\x90\x91\x82a\x13\xA7\x81a\x13\xAE\x93a\x13\x13V[\x80\x93a\x13\x8CV[\x01\x90V[`\x04\x93a\x13\xE1` a\x13\xF0\x99\x97\x95a\x13\xD9a\x13\xD2`\x14\x96a\x13\xE9\x98a\x13\x18V[\x80\x92a\x0EoV[\x01\x80\x92a\x13_V[\x01\x80\x92a\x13wV[\x01\x91a\x13\x97V[\x90V[_\x1B\x90V[a\x14\x01\x91a\x13\x18V[\x90V[a\x14\x0Ca\x012V[=_\x82>=\x90\xFD[_\x1C\x90V[a\x14%a\x14*\x91a\x14\x14V[a\x01SV[\x90V[a\x147\x90Ta\x14\x19V[\x90V[_\x80\xFD[`\xE0\x1B\x90V[a\x14M\x81a\n\x1DV[\x03a\x14TWV[_\x80\xFD[\x90PQ\x90a\x14e\x82a\x14DV[V[\x91\x90`@\x83\x82\x03\x12a\x14\x8FW\x80a\x14\x83a\x14\x8C\x92_\x86\x01a\x14XV[\x93` \x01a\x14XV[\x90V[a\x01<V[a\x14\x9D\x90a\x07mV[\x90RV[a\x14\xAA\x90a\x020V[\x90RV[\x90` \x80a\x14\xD0\x93a\x14\xC6_\x82\x01Q_\x86\x01\x90a\x14\xA1V[\x01Q\x91\x01\x90a\x14\xA1V[V[P`\x02\x90V[\x90P\x90V[\x90V[\x90a\x14\xED\x81` \x93a\x14\xA1V[\x01\x90V[` \x01\x90V[a\x15\x13a\x15\ra\x15\x06\x83a\x14\xD2V[\x80\x94a\x14\xD8V[\x91a\x14\xDDV[_\x91[\x83\x83\x10a\x15#WPPPPV[a\x159a\x153`\x01\x92\x84Qa\x14\xE0V[\x92a\x14\xF1V[\x92\x01\x91\x90a\x15\x16V[\x90`@` a\x15e\x93a\x15[_\x82\x01Q_\x86\x01\x90a\x14\xF7V[\x01Q\x91\x01\x90a\x14\xF7V[V[a\x15\x9Da\x15\xA4\x94a\x15\x93`\xE0\x94\x98\x97\x95a\x15\x89a\x01 \x86\x01\x9A_\x87\x01\x90a\x14\x94V[` \x85\x01\x90a\x14\xAEV[``\x83\x01\x90a\x15BV[\x01\x90a\x14\xAEV[V[a\x15\xAF\x90a\x01\xBDV[\x90V[_\x80\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x15\xD4Wa\x15\xD0` \x91a\x03\xA2V[\x01\x90V[a\x03\xACV[\x90\x92\x91\x92a\x15\xEEa\x15\xE9\x82a\x15\xB6V[a\x05\x07V[\x93\x81\x85R` \x85\x01\x90\x82\x84\x01\x11a\x16\nWa\x16\x08\x92a\x04IV[V[a\x15\xB2V[\x90\x80`\x1F\x83\x01\x12\x15a\x16-W\x81` a\x16*\x93Q\x91\x01a\x15\xD9V[\x90V[a\x07\xD6V[\x90` \x82\x82\x03\x12a\x16bW_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x16]Wa\x16Z\x92\x01a\x16\x0FV[\x90V[a\x02,V[a\x01<V[\x90\x92\x91\x92a\x16|a\x16w\x82a\x15\xB6V[a\x05\x07V[\x93\x81\x85R` \x85\x01\x90\x82\x84\x01\x11a\x16\x98Wa\x16\x96\x92a\x13\x8CV[V[a\x15\xB2V[a\x16\xA8\x916\x91a\x16gV[\x90V[` \x01\x90V[\x85\x98a\x16\xF9\x89a\x16\xEB\x8A\x9D` \x97\x99\x98\x9B_\x97\x9Ba\x17\n\x97Pa\x16\xD2a\x05\xA1V[\x94\x96\x92\x90\x91\x92a\x16\xE0a\x012V[\x97\x88\x96\x8C\x88\x01a\x13\xB2V[\x86\x82\x01\x81\x03\x82R\x03\x82a\x03\xC0V[a\x17\x01a\x012V[\x91\x82\x91\x82a\x13\xF8V[\x03\x90`\x02Z\xFA\x15a\x19FWa\x17;a\x17\"_Qa\x13\xF3V[a\x174a\x17.\x86a\x07mV[\x91a\x07mV[\x14\x15a\n\x1DV[a\x19\x11Wa\x17w`@\x93a\x17\x82a\x17Za\x17U`\x03a\x14-V[a\x01\xC9V[\x93c\x17\x1F\x1D[\x92\x95\x97a\x17ka\x012V[\x98\x89\x97\x88\x96\x87\x96a\x14>V[\x86R`\x04\x86\x01a\x15gV[\x03\x91Z\xFA\x80\x15a\x19\x0CW_\x80\x92\x90\x91a\x18\xD9W[Pa\x17\xA1\x90\x15a\n\x1DV[\x90\x81\x15a\x18\xC8W[Pa\x18\x97W_a\x17\xE3\x91a\x17\xBC0a\x15\xA6V[a\x17\xD8c_\xFD\x1F\0a\x17\xCCa\x012V[\x95\x86\x94\x85\x93\x84\x93a\x14>V[\x83R`\x04\x83\x01a\x07#V[\x03\x91Z\xFA\x91\x82\x15a\x18\x92Wa\x18Ca\x18=a\x18\x11a\x18^\x96a\x18m\x96a\x18J\x96_\x91a\x18pW[P\x94a\x16\x9DV[a\x18#a\x18\x1D\x82a\x04<V[\x91a\x16\xABV[ \x92a\x187a\x181\x82a\x04<V[\x91a\x16\xABV[ a\x07mV[\x91a\x07mV[\x14\x15a\n\x1DV[a\x18Ra\x012V[\x92\x83\x91` \x83\x01a\n/V[` \x82\x01\x81\x03\x82R\x03\x82a\x03\xC0V[\x90V[a\x18\x8C\x91P=\x80_\x83>a\x18\x84\x81\x83a\x03\xC0V[\x81\x01\x90a\x162V[_a\x18\nV[a\x14\x04V[PPPa\x18\xB6a\x18\xC5`\x01a\x18\xAAa\x012V[\x92\x83\x91` \x83\x01a\n/V[` \x82\x01\x81\x03\x82R\x03\x82a\x03\xC0V[\x90V[a\x18\xD3\x91P\x15a\n\x1DV[_a\x17\xA9V[a\x17\xA1\x92Pa\x18\xFF\x91P`@=\x81\x11a\x19\x05W[a\x18\xF7\x81\x83a\x03\xC0V[\x81\x01\x90a\x14gV[\x91a\x17\x96V[P=a\x18\xEDV[a\x14\x04V[PPPPPPPa\x194a\x19C`\x01a\x19(a\x012V[\x92\x83\x91` \x83\x01a\n/V[` \x82\x01\x81\x03\x82R\x03\x82a\x03\xC0V[\x90V[a\x14\x04V[\x90a\x19d\x98\x97\x96\x95\x94\x93\x92\x91a\x19_a\r\x8BV[a\x12\xB7V[\x90V[_\x90V[a\x19\xBB\x90\x7F\xDE\xBF\xDF\xD5\xA5\n\xD1\x17\xC1\x08\x98\xD6\x8B\\\xCF\x08\x93\xC6\xB4\rOD?\x90..vF`\x1B\xDE\xAFT`\x01\x01\x7F\xDE\xBF\xDF\xD5\xA5\n\xD1\x17\xC1\x08\x98\xD6\x8B\\\xCF\x08\x93\xC6\xB4\rOD?\x90..vF`\x1B\xDE\xAFUa\x1A<V[\x90V[\x90a\x19\xCA_\x19\x91a\x13\xF3V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[\x90a\x19\xECa\x19\xE7a\x19\xF3\x92a\x02tV[a\x19\xD4V[\x82Ta\x19\xBEV[\x90UV[\x90a\x1A\x03`\xFF\x91a\x13\xF3V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x1A\x16\x90a\n\x1DV[\x90V[\x90V[\x90a\x1A1a\x1A,a\x1A8\x92a\x1A\rV[a\x1A\x19V[\x82Ta\x19\xF7V[\x90UV[Pa\x1Aka\x1APa\x1AKa(QV[a\x11eV[a\x1A[\x81`\x01a\x19\xD7V[a\x1Ae`\x02a\r\x93V[\x90a\r\xC3V[a\x1A}a\x1Aw_a\r\xE8V[\x91a\x020V[\x14a\x1A\x89\x81`\x02a\x1A\x1CV[\x90V[a\x1A\x9Ca\x1A\x97a\x19gV[a\x19kV[\x90V[\x90a\x1A\xF1\x92\x91\x7F\xDE\xBF\xDF\xD5\xA5\n\xD1\x17\xC1\x08\x98\xD6\x8B\\\xCF\x08\x93\xC6\xB4\rOD?\x90..vF`\x1B\xDE\xAFT`\x01\x01\x7F\xDE\xBF\xDF\xD5\xA5\n\xD1\x17\xC1\x08\x98\xD6\x8B\\\xCF\x08\x93\xC6\xB4\rOD?\x90..vF`\x1B\xDE\xAFUa\x1E\xFAV[\x90V[\x90V[a\x1B\x0Ba\x1B\x06a\x1B\x10\x92a\x1A\xF4V[a\x01\x9EV[a\x020V[\x90V[` \x91\x81R\x01\x90V[_\x7FMust send exactly 0.1 ETH\0\0\0\0\0\0\0\x91\x01RV[a\x1BP`\x19` \x92a\x1B\x13V[a\x1BY\x81a\x1B\x1CV[\x01\x90V[a\x1Br\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x1BCV[\x90V[\x15a\x1B|WV[a\x1B\x84a\x012V[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x1B\x9A`\x04\x82\x01a\x1B]V[\x03\x90\xFD[`\x08\x1C\x90V[a\x1B\xB0a\x1B\xB5\x91a\x1B\x9EV[a\x05\xECV[\x90V[a\x1B\xC2\x90Ta\x1B\xA4V[\x90V[_\x91\x03\x12a\x1B\xCFWV[a\x01<V[P\x90V[_\x7FInvalid opcode offset\0\0\0\0\0\0\0\0\0\0\0\x91\x01RV[a\x1C\x0C`\x15` \x92a\x1B\x13V[a\x1C\x15\x81a\x1B\xD8V[\x01\x90V[a\x1C.\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x1B\xFFV[\x90V[\x15a\x1C8WV[a\x1C@a\x012V[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x1CV`\x04\x82\x01a\x1C\x19V[\x03\x90\xFD[\x91\x90\x81\x10\x15a\x1CjW`\x01\x02\x01\x90V[a\x0B\x10V[`\xFF`\xF8\x1B\x16\x90V[`\xF8\x1C\x90V[a\x1C\x92a\x1C\x8Da\x1C\x97\x92a\x0E\x04V[a\x01\x9EV[a\x0E\x04V[\x90V[a\x1C\xA6a\x1C\xAB\x91a\x1CxV[a\x1C~V[\x90V[a\x1C\xB7\x90a\x020V[_\x19\x81\x14a\x1C\xC5W`\x01\x01\x90V[a\x0F\x98V[` \x7Fe 0)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x7FUnsupported operation (op must b_\x82\x01R\x01RV[a\x1D$`$`@\x92a\x1B\x13V[a\x1D-\x81a\x1C\xCAV[\x01\x90V[a\x1DF\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x1D\x17V[\x90V[\x15a\x1DPWV[a\x1DXa\x012V[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x1Dn`\x04\x82\x01a\x1D1V[\x03\x90\xFD[\x90V[a\x1D\x89a\x1D\x84a\x1D\x8E\x92a\x1DrV[a\x01\x9EV[a\x020V[\x90V[_\x7FMissing slot data\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x01RV[a\x1D\xC5`\x11` \x92a\x1B\x13V[a\x1D\xCE\x81a\x1D\x91V[\x01\x90V[a\x1D\xE7\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x1D\xB8V[\x90V[\x15a\x1D\xF1WV[a\x1D\xF9a\x012V[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x1E\x0F`\x04\x82\x01a\x1D\xD2V[\x03\x90\xFD[_\x7FMissing value data\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x01RV[a\x1EG`\x12` \x92a\x1B\x13V[a\x1EP\x81a\x1E\x13V[\x01\x90V[a\x1Ei\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x1E:V[\x90V[\x15a\x1EsWV[a\x1E{a\x012V[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x1E\x91`\x04\x82\x01a\x1ETV[\x03\x90\xFD[a\x1E\xA1a\x1E\xA6\x91a\x14\x14V[a\r\x1AV[\x90V[a\x1E\xB3\x90Ta\x1E\x95V[\x90V[a\x1E\xC2a\x1E\xC7\x91a\x14\x14V[a\x0B\xEAV[\x90V[a\x1E\xD4\x90Ta\x1E\xB6V[\x90V[\x91` a\x1E\xF8\x92\x94\x93a\x1E\xF1`@\x82\x01\x96_\x83\x01\x90a\x07\x16V[\x01\x90a\n\"V[V[\x92\x91\x92Pa\x1F\"4a\x1F\x1Ca\x1F\x16g\x01cEx]\x8A\0\0a\x1A\xF7V[\x91a\x020V[\x14a\x1BuV[a\x1F4a\x1F/`\x02a\x1B\xB8V[a\x06-V[c\xD0\xE3\r\xB04\x91\x90\x81;\x15a!WW_\x91a\x1F[\x91a\x1FQa\x012V[\x94\x85\x93\x84\x92a\x14>V[\x82R\x81a\x1Fj`\x04\x82\x01a\x0C\xE2V[\x03\x92Z\xF1\x80\x15a!RWa!&W[Pa\x1F\x83_a\r\xE8V[\x91[\x82a\x1F\xA2a\x1F\x9Ca\x1F\x97\x85\x88\x90a\x1B\xD4V[a\x020V[\x91a\x020V[\x10\x15a \xE1Wa\x1F\xE2a\x1F\xBF\x84a\x1F\xB9`\x01a\x0E)V[\x90a\x11@V[a\x1F\xDBa\x1F\xD5a\x1F\xD0\x86\x89\x90a\x1B\xD4V[a\x020V[\x91a\x020V[\x11\x15a\x1C1V[a $a \x0Ba \x05a \0a\x1F\xFA\x86\x89\x89\x91a\x1CZV[5a\x1CoV[a\x1C\x9AV[\x94a\x1C\xAEV[\x93a \x1Ea \x18_a\x0E\nV[\x91a\x0E\x04V[\x14a\x1DIV[a ^a ;\x84a 5` a\x1DuV[\x90a\x11@V[a Wa Qa L\x86\x89\x90a\x1B\xD4V[a\x020V[\x91a\x020V[\x11\x15a\x1D\xEAV[a fa\x0F\x88V[Pa \x7F\x83\x83\x015\x93a y` a\x1DuV[\x90a\x11@V[a \xB9a \x96\x82a \x90` a\x1DuV[\x90a\x11@V[a \xB2a \xACa \xA7\x87\x8A\x90a\x1B\xD4V[a\x020V[\x91a\x020V[\x11\x15a\x1ElV[a \xC1a\x0F\x88V[Pa \xDA\x81\x84\x015\x91a \xD4` a\x1DuV[\x90a\x11@V[\x93Ua\x1F\x85V[\x92PPPa \xEF`\x01a\x1E\xA9V[a!#a \xFC`\x02a\x1E\xCAV[\x91a!\x14a!\x08a\x012V[\x93\x84\x92` \x84\x01a\x1E\xD7V[` \x82\x01\x81\x03\x82R\x03\x82a\x03\xC0V[\x90V[a!E\x90_=\x81\x11a!KW[a!=\x81\x83a\x03\xC0V[\x81\x01\x90a\x1B\xC5V[_a\x1FyV[P=a!3V[a\x14\x04V[a\x14:V[\x90a!n\x91a!ia\r\x8BV[a\x1A\x9FV[\x90V[` \x91\x81R\x01\x90V[a!\x83\x90a\x06\xB3V[\x90RV[\x90a!\x94\x81` \x93a!zV[\x01\x90V[a!\xA4a!\xA9\x91a\x14\x14V[a\x0BYV[\x90V[a!\xB6\x90Ta!\x98V[\x90V[`\x01\x01\x90V[\x90a!\xDCa!\xD6a!\xCF\x84a\x0B$V[\x80\x93a!qV[\x92a\x0B(V[\x90_[\x81\x81\x10a!\xECWPPP\x90V[\x90\x91\x92a\"\x0Ca\"\x06`\x01\x92a\"\x01\x87a!\xACV[a!\x87V[\x94a!\xB9V[\x91\x01\x91\x90\x91a!\xDFV[a\"+\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra!\xBFV[\x90V[a\"6a\r\x8BV[Pa\"Ra\"a_a\"Fa\x012V[\x92\x83\x91` \x83\x01a\"\x16V[` \x82\x01\x81\x03\x82R\x03\x82a\x03\xC0V[\x90V[\x90a\"\xBD\x99\x98\x97\x96\x95\x94\x93\x92\x91\x7F\xDE\xBF\xDF\xD5\xA5\n\xD1\x17\xC1\x08\x98\xD6\x8B\\\xCF\x08\x93\xC6\xB4\rOD?\x90..vF`\x1B\xDE\xAFT`\x01\x01\x7F\xDE\xBF\xDF\xD5\xA5\n\xD1\x17\xC1\x08\x98\xD6\x8B\\\xCF\x08\x93\xC6\xB4\rOD?\x90..vF`\x1B\xDE\xAFUa$iV[\x90V[\x15a\"\xC7WV[_c9\xBBpQ`\xE1\x1B\x81R\x80a\"\xDF`\x04\x82\x01a\x0C\xE2V[\x03\x90\xFD[_\x7FInvalid signature\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x01RV[a#\x17`\x11` \x92a\x1B\x13V[a# \x81a\"\xE3V[\x01\x90V[a#9\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra#\nV[\x90V[\x15a#CWV[a#Ka\x012V[bF\x1B\xCD`\xE5\x1B\x81R\x80a#a`\x04\x82\x01a#$V[\x03\x90\xFD[_\x7FBLS pairing check failed\0\0\0\0\0\0\0\0\x91\x01RV[a#\x99`\x18` \x92a\x1B\x13V[a#\xA2\x81a#eV[\x01\x90V[a#\xBB\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra#\x8CV[\x90V[\x15a#\xC5WV[a#\xCDa\x012V[bF\x1B\xCD`\xE5\x1B\x81R\x80a#\xE3`\x04\x82\x01a#\xA6V[\x03\x90\xFD[_\x7FInvalid BLS signature\0\0\0\0\0\0\0\0\0\0\0\x91\x01RV[a$\x1B`\x15` \x92a\x1B\x13V[a$$\x81a#\xE7V[\x01\x90V[a$=\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra$\x0EV[\x90V[\x15a$GWV[a$Oa\x012V[bF\x1B\xCD`\xE5\x1B\x81R\x80a$e`\x04\x82\x01a$(V[\x03\x90\xFD[\x99\x98\x90\x95\x99\x97\x91\x94\x97\x96\x92\x96Pa$\xACa$\x8D\x84a$\x87`\x01a\x0E)V[\x90a\x11@V[a$\xA6a$\xA0a$\x9Ba(QV[a\x020V[\x91a\x020V[\x14a\"\xC0V[a$\xD04a$\xCAa$\xC4g\x01cEx]\x8A\0\0a\x1A\xF7V[\x91a\x020V[\x14a\x1BuV[a$\xE2a$\xDD`\x02a\x1B\xB8V[a\x06-V[\x91c\xD0\xE3\r\xB04\x93\x90\x81;\x15a(0W_\x91a%\n\x91a%\0a\x012V[\x96\x87\x93\x84\x92a\x14>V[\x82R\x81a%\x19`\x04\x82\x01a\x0C\xE2V[\x03\x92Z\xF1\x80\x15a(+W` \x94a%\\a%{\x94\x8E_\x97a%j\x95a'\xFFW[P\x8Da%Ca\x05\xA1V[\x94\x96\x92\x90\x91\x92a%Qa\x012V[\x97\x88\x96\x8C\x88\x01a\x13\xB2V[\x86\x82\x01\x81\x03\x82R\x03\x82a\x03\xC0V[a%ra\x012V[\x91\x82\x91\x82a\x13\xF8V[\x03\x90`\x02Z\xFA\x15a'\xFAWa%\xE3`@\x93a%\xB1a%\x99_Qa\x13\xF3V[a%\xABa%\xA5\x84a\x07mV[\x91a\x07mV[\x14a#<V[a%\xEEa%\xC6a%\xC1`\x03a\x14-V[a\x01\xC9V[\x93c\x17\x1F\x1D[\x92\x95\x97a%\xD7a\x012V[\x98\x89\x97\x88\x96\x87\x96a\x14>V[\x86R`\x04\x86\x01a\x15gV[\x03\x91Z\xFA\x90\x81\x15a'\xF5Wa&\x16\x91_\x80\x92\x90\x91a'\xC2W[Pa&\x11\x90a#\xBEV[a$@V[a&\x1F_a\r\xE8V[\x91[\x82a&>a&8a&3\x85\x88\x90a\x1B\xD4V[a\x020V[\x91a\x020V[\x10\x15a'}Wa&~a&[\x84a&U`\x01a\x0E)V[\x90a\x11@V[a&wa&qa&l\x86\x89\x90a\x1B\xD4V[a\x020V[\x91a\x020V[\x11\x15a\x1C1V[a&\xC0a&\xA7a&\xA1a&\x9Ca&\x96\x86\x89\x89\x91a\x1CZV[5a\x1CoV[a\x1C\x9AV[\x94a\x1C\xAEV[\x93a&\xBAa&\xB4_a\x0E\nV[\x91a\x0E\x04V[\x14a\x1DIV[a&\xFAa&\xD7\x84a&\xD1` a\x1DuV[\x90a\x11@V[a&\xF3a&\xEDa&\xE8\x86\x89\x90a\x1B\xD4V[a\x020V[\x91a\x020V[\x11\x15a\x1D\xEAV[a'\x02a\x0F\x88V[Pa'\x1B\x83\x83\x015\x93a'\x15` a\x1DuV[\x90a\x11@V[a'Ua'2\x82a',` a\x1DuV[\x90a\x11@V[a'Na'Ha'C\x87\x8A\x90a\x1B\xD4V[a\x020V[\x91a\x020V[\x11\x15a\x1ElV[a']a\x0F\x88V[Pa'v\x81\x84\x015\x91a'p` a\x1DuV[\x90a\x11@V[\x93Ua&!V[\x92PPPa'\x8B`\x01a\x1E\xA9V[a'\xBFa'\x98`\x02a\x1E\xCAV[\x91a'\xB0a'\xA4a\x012V[\x93\x84\x92` \x84\x01a\x1E\xD7V[` \x82\x01\x81\x03\x82R\x03\x82a\x03\xC0V[\x90V[a&\x11\x92Pa'\xE8\x91P`@=\x81\x11a'\xEEW[a'\xE0\x81\x83a\x03\xC0V[\x81\x01\x90a\x14gV[\x91a&\x07V[P=a'\xD6V[a\x14\x04V[a\x14\x04V[a(\x1E\x90\x89=\x81\x11a($W[a(\x16\x81\x83a\x03\xC0V[\x81\x01\x90a\x1B\xC5V[_a%9V[P=a(\x0CV[a\x14\x04V[a\x14:V[\x90a(N\x98\x97\x96\x95\x94\x93\x92\x91a(Ia\r\x8BV[a\"dV[\x90V[a(Ya\x0F\x88V[P\x7F\xDE\xBF\xDF\xD5\xA5\n\xD1\x17\xC1\x08\x98\xD6\x8B\\\xCF\x08\x93\xC6\xB4\rOD?\x90..vF`\x1B\xDE\xAFT\x90V[a(\xCF\x90\x7F\xDE\xBF\xDF\xD5\xA5\n\xD1\x17\xC1\x08\x98\xD6\x8B\\\xCF\x08\x93\xC6\xB4\rOD?\x90..vF`\x1B\xDE\xAFT`\x01\x01\x7F\xDE\xBF\xDF\xD5\xA5\n\xD1\x17\xC1\x08\x98\xD6\x8B\\\xCF\x08\x93\xC6\xB4\rOD?\x90..vF`\x1B\xDE\xAFUa+zV[V[\x90V[_R` _ \x90V[T\x90V[a(\xEA\x81a(\xDDV[\x82\x10\x15a)\x04Wa(\xFC`\x01\x91a(\xD4V[\x91\x02\x01\x90_\x90V[a\x0B\x10V[\x1B\x90V[\x91\x90`\x08a)-\x91\x02\x91a)'`\x01\x80`\xA0\x1B\x03\x84a)\tV[\x92a)\tV[\x91\x81\x19\x16\x91\x16\x17\x90V[a)@\x90a\x01\xBDV[\x90V[\x90V[\x91\x90a)\\a)Wa)d\x93a)7V[a)CV[\x90\x83Ta)\rV[\x90UV[\x90\x81T\x91h\x01\0\0\0\0\0\0\0\0\x83\x10\x15a)\x98W\x82a)\x90\x91`\x01a)\x96\x95\x01\x81Ua(\xE1V[\x90a)FV[V[a\x03\xACV[`\x1F` \x91\x01\x04\x90V[\x91\x90`\x08a)\xC2\x91\x02\x91a)\xBC_\x19\x84a)\tV[\x92a)\tV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x91\x90a)\xE2a)\xDDa)\xEA\x93a\x02tV[a\x19\xD4V[\x90\x83Ta)\xA7V[\x90UV[a*\0\x91a)\xFAa\x0F\x88V[\x91a)\xCCV[V[[\x81\x81\x10a*\x0EWPPV[\x80a*\x1B_`\x01\x93a)\xEEV[\x01a*\x03V[\x91\x90`\x1F\x81\x11a*1W[PPPV[a*=a*b\x93a\x03\0V[\x90` a*I\x84a)\x9DV[\x83\x01\x93\x10a*jW[a*[\x90a)\x9DV[\x01\x90a*\x02V[_\x80\x80a*,V[\x91Pa*[\x81\x92\x90Pa*RV[\x90a*\x88\x90_\x19\x90`\x08\x02a\x01OV[\x19\x16\x90V[\x81a*\x97\x91a*xV[\x90`\x02\x02\x17\x90V[\x90a*\xA9\x81a\x04<V[\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a+iWa*\xCD\x82a*\xC7\x85Ta\x02\xCDV[\x85a*!V[` \x90`\x1F\x83\x11`\x01\x14a+\x01W\x91\x80\x91a*\xF0\x93_\x92a*\xF5W[PPa*\x8DV[\x90U[V[\x90\x91P\x01Q_\x80a*\xE9V[`\x1F\x19\x83\x16\x91a+\x10\x85a\x03\0V[\x92_[\x81\x81\x10a+QWP\x91`\x02\x93\x91\x85`\x01\x96\x94\x10a+7W[PPP\x02\x01\x90Ua*\xF3V[a+G\x91\x01Q`\x1F\x84\x16\x90a*xV[\x90U_\x80\x80a++V[\x91\x93` `\x01\x81\x92\x87\x87\x01Q\x81U\x01\x95\x01\x92\x01a+\x13V[a\x03\xACV[\x90a+x\x91a*\x9FV[V[a+\x8C\x90a+\x87_a(\xD1V[a)hV[a+\xD1a+\xAAa+\xB9_a+\x9Ea\x012V[\x92\x83\x91` \x83\x01a\"\x16V[` \x82\x01\x81\x03\x82R\x03\x82a\x03\xC0V[a+\xCC`\x04a+\xC6a(QV[\x90a\x02\x90V[a+nV[V[a+\xDC\x90a(\x7FV[V\xFE\xA2dipfsX\"\x12 \xBA\xA6\xF9\xF1(n\x16$<j\x95z\xD1\xC7\xD3\x18\xA41\xB9\xDC\xEFmS\x82\xDD\xA6M%X\xFF\xFB\xBAdsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610013575b610d87565b61001d5f3561012c565b80631c178e9c146101275780631ee07fb1146101225780635ffd1f001461011d5780637c015a89146101185780639c91dd5614610113578063a352d7071461010e578063a4f8815414610109578063aa2c58b214610104578063bfa06830146100ff578063c7b37540146100fa578063cf6ae603146100f5578063da58c7d9146100f0578063ea53bac5146100eb578063f300b862146100e6578063f4833e20146100e1578063f4ab9adf146100dc5763fd8eac490361000e57610d52565b610ce7565b610c8f565b610c5a565b610c25565b610bb5565b610adb565b610aaf565b610a44565b6109de565b610738565b6106e1565b61065b565b6105b7565b6104d2565b61049d565b6101f7565b60e01c90565b60405190565b5f80fd5b5f80fd5b5f91031261014a57565b61013c565b1c90565b60018060a01b031690565b61016e906008610173930261014f565b610153565b90565b90610181915461015e565b90565b61019060035f90610176565b90565b60018060a01b031690565b90565b6101b56101b06101ba92610193565b61019e565b610193565b90565b6101c6906101a1565b90565b6101d2906101bd565b90565b6101de906101c9565b9052565b91906101f5905f602085019401906101d5565b565b3461022757610207366004610140565b610223610212610184565b61021a610132565b918291826101e2565b0390f35b610138565b5f80fd5b90565b61023c81610230565b0361024357565b5f80fd5b9050359061025482610233565b565b9060208282031261026f5761026c915f01610247565b90565b61013c565b61028861028361028d92610230565b61019e565b610230565b90565b9061029a90610274565b5f5260205260405f2090565b634e487b7160e01b5f525f60045260245ffd5b634e487b7160e01b5f52602260045260245ffd5b90600160028304921680156102ed575b60208310146102e857565b6102b9565b91607f16916102dd565b60209181520190565b5f5260205f2090565b905f929180549061032361031c836102cd565b80946102f7565b916001811690815f1461037a575060011461033e575b505050565b61034b9192939450610300565b915f925b81841061036257505001905f8080610339565b6001816020929593955484860152019101929061034f565b92949550505060ff19168252151560200201905f8080610339565b9061039f91610309565b90565b601f801991011690565b634e487b7160e01b5f52604160045260245ffd5b906103ca906103a2565b810190811067ffffffffffffffff8211176103e457604052565b6103ac565b90610409610402926103f9610132565b93848092610395565b03836103c0565b565b905f1061041e5761041b906103e9565b90565b6102a6565b610439906104346004915f92610290565b61040b565b90565b5190565b60209181520190565b90825f9392825e0152565b61047361047c6020936104819361046a8161043c565b93848093610440565b95869101610449565b6103a2565b0190565b61049a9160208201915f818403910152610454565b90565b346104cd576104c96104b86104b3366004610256565b610423565b6104c0610132565b91829182610485565b0390f35b610138565b34610502576104fe6104ed6104e8366004610256565b610ed8565b6104f5610132565b91829182610485565b0390f35b610138565b9061051a610513610132565b92836103c0565b565b67ffffffffffffffff811161053a576105366020916103a2565b0190565b6103ac565b9061055161054c8361051c565b610507565b918252565b5f7f5f434f4d4d4f4e574152455f4147475245474154494f4e5f0000000000000000910152565b610587601861053f565b9061059460208301610556565b565b61059e61057d565b90565b6105a9610596565b90565b6105b46105a1565b90565b346105e7576105c7366004610140565b6105e36105d26105ac565b6105da610132565b91829182610485565b0390f35b610138565b60018060a01b031690565b61060790600861060c930261014f565b6105ec565b90565b9061061a91546105f7565b90565b61062a600260019061060f565b90565b610636906101bd565b90565b6106429061062d565b9052565b9190610659905f60208501940190610639565b565b3461068b5761066b366004610140565b61068761067661061d565b61067e610132565b91829182610646565b0390f35b610138565b73ca249215e082e17c12bb3c4881839a3f883e5c6b90565b6106b0610690565b90565b6106bc90610193565b90565b6106c8906106b3565b9052565b91906106df905f602085019401906106bf565b565b34610711576106f1366004610140565b61070d6106fc6106a8565b610704610132565b918291826106cc565b0390f35b610138565b61071f90610230565b9052565b9190610736905f60208501940190610716565b565b346107685761076461075361074e366004610256565b611165565b61075b610132565b91829182610723565b0390f35b610138565b90565b6107798161076d565b0361078057565b5f80fd5b9050359061079182610770565b565b5f80fd5b91906040838203126107d1576107ca906107b16040610507565b936107be825f8301610247565b5f860152602001610247565b6020830152565b610793565b5f80fd5b67ffffffffffffffff81116107ef5760200290565b6103ac565b5f80fd5b9092919261080d610808826107da565b610507565b93602085920283019281841161084557915b83831061082c5750505050565b6020809161083a8486610247565b81520192019161081f565b6107f4565b9080601f8301121561086557610862916002906107f8565b90565b6107d6565b91906080838203126108a45761089d906108846040610507565b93610891825f830161084a565b5f86015260400161084a565b6020830152565b610793565b5f80fd5b909182601f830112156108e75781359167ffffffffffffffff83116108e25760200192600183028401116108dd57565b6107f4565b6108a9565b6107d6565b6108f5816106b3565b036108fc57565b5f80fd5b9050359061090d826108ec565b565b63ffffffff60e01b1690565b6109248161090f565b0361092b57565b5f80fd5b9050359061093c8261091b565b565b9190916101a0818403126109d957610958835f8301610784565b926109668160208401610797565b92610974826060850161086a565b926109828360e08301610797565b9261012082013567ffffffffffffffff81116109d457816109a49184016108ad565b9290936109d16109b8846101408501610247565b936109c7816101608601610900565b936101800161092f565b90565b61022c565b61013c565b34610a1857610a14610a036109f436600461093e565b9796909695919594929461194b565b610a0b610132565b91829182610485565b0390f35b610138565b151590565b610a2b90610a1d565b9052565b9190610a42905f60208501940190610a22565b565b34610a7457610a54366004610140565b610a70610a5f611a8c565b610a67610132565b91829182610a2f565b0390f35b610138565b90602082820312610aaa575f82013567ffffffffffffffff8111610aa557610aa192016108ad565b9091565b61022c565b61013c565b610ad7610ac6610ac0366004610a79565b9061215c565b610ace610132565b91829182610485565b0390f35b34610b0b57610aeb366004610140565b610b07610af661222e565b610afe610132565b91829182610485565b0390f35b610138565b634e487b7160e01b5f52603260045260245ffd5b5490565b5f5260205f2090565b610b3a81610b24565b821015610b5457610b4c600191610b28565b910201905f90565b610b10565b60018060a01b031690565b610b74906008610b79930261014f565b610b59565b90565b90610b879154610b64565b90565b5f610b9481610b24565b821015610bb157610bae91610ba891610b31565b90610b7c565b90565b5f80fd5b34610be557610be1610bd0610bcb366004610256565b610b8a565b610bd8610132565b918291826106cc565b0390f35b610138565b60ff1690565b610c00906008610c05930261014f565b610bea565b90565b90610c139154610bf0565b90565b610c2260025f90610c08565b90565b34610c5557610c35366004610140565b610c51610c40610c16565b610c48610132565b91829182610a2f565b0390f35b610138565b610c8b610c7a610c6b36600461093e565b97969096959195949294612835565b610c82610132565b91829182610485565b0390f35b34610cbf57610c9f366004610140565b610cbb610caa612851565b610cb2610132565b91829182610723565b0390f35b610138565b90602082820312610cdd57610cda915f01610900565b90565b61013c565b5f0190565b34610d1557610cff610cfa366004610cc4565b612bd3565b610d07610132565b80610d1181610ce2565b0390f35b610138565b90565b610d2d906008610d32930261014f565b610d1a565b90565b90610d409154610d1d565b90565b610d4f60015f90610d35565b90565b34610d8257610d62366004610140565b610d7e610d6d610d43565b610d75610132565b91829182610723565b0390f35b610138565b5f80fd5b606090565b90565b610da7610da2610dac92610d90565b61019e565b610230565b90565b634e487b7160e01b5f52601260045260245ffd5b610dcf610dd591610230565b91610230565b908115610de0570690565b610daf565b90565b610dfc610df7610e0192610de5565b61019e565b610230565b90565b60ff1690565b610e1e610e19610e2392610de5565b61019e565b610e04565b90565b90565b610e3d610e38610e4292610e26565b61019e565b610230565b90565b60f81b90565b610e5490610e45565b90565b610e63610e6891610e04565b610e4b565b9052565b90565b610e7b610e8091610230565b610e6c565b9052565b610ed494610ec460208099989596610ebc828099610eb4600189610eac829b610ecc9d610e57565b018092610e6f565b018092610e6f565b018092610e57565b018092610e6f565b018092610e6f565b0190565b610eea90610ee4610d8b565b50611165565b610f76610f0182610efb6002610d93565b90610dc3565b610f13610f0d5f610de8565b91610230565b14610f67610f205f610e0a565b91610f2b6001610e29565b9490610f365f610e0a565b610f406002610d93565b915f14610f7957610f516001610e29565b925b610f5b610132565b97889660208801610e84565b602082018103825203826103c0565b90565b610f825f610de8565b92610f53565b5f90565b610f95906103e9565b90565b634e487b7160e01b5f52601160045260245ffd5b610fb590610230565b5f8114610fc3576001900390565b610f98565b67ffffffffffffffff8111610fe05760208091020190565b6103ac565b90505190610ff2826108ec565b565b9092919261100961100482610fc8565b610507565b938185526020808601920283019281841161104657915b83831061102d5750505050565b6020809161103b8486610fe5565b815201920191611020565b6107f4565b9080601f830112156110695781602061106693519101610ff4565b90565b6107d6565b9060208282031261109e575f82015167ffffffffffffffff811161109957611096920161104b565b90565b61022c565b61013c565b60016110af9101610230565b90565b5190565b906110c0826110b2565b8110156110d1576020809102010190565b610b10565b6110e090516106b3565b90565b6110ec906101a1565b90565b6111036110fe61110892610193565b61019e565b610230565b90565b61111a61112091939293610230565b92610230565b9161112c838202610230565b92818404149015171561113b57565b610f98565b61114f61115591939293610230565b92610230565b820180921161116057565b610f98565b9061116e610f88565b5061118361117e60048490610290565b610f8c565b82905b61118f8161043c565b6111a161119b5f610de8565b91610230565b148061129d575b156111fb5750806111c16111bb5f610de8565b91610230565b146111ec576111cf90610fac565b916111e46111df60048590610290565b610f8c565b929092611186565b5090506111f85f610de8565b90565b6112199150929192602061120e8261043c565b81830101910161106e565b906112235f610de8565b9161122d5f610de8565b925b8361124a61124461123f856110b2565b610230565b91610230565b10156112965761128a6112909161128461127561127061126b878a906110b6565b6110d6565b6110e3565b61127f87916110ef565b61110b565b90611140565b936110a3565b9261122f565b9250505090565b50816112b16112ab5f610de8565b91610230565b116111a8565b906113109998979695949392917fdebfdfd5a50ad117c10898d68b5ccf0893c6b40d4f443f902e2e7646601bdeaf546001017fdebfdfd5a50ad117c10898d68b5ccf0893c6b40d4f443f902e2e7646601bdeaf556116b1565b90565b905090565b61133d6113349260209261132b8161043c565b94858093611313565b93849101610449565b0190565b60601b90565b61135090611341565b90565b61135c90611347565b90565b61136b611370916106b3565b611353565b9052565b90565b6113836113889161090f565b611374565b9052565b90825f939282370152565b9091826113a7816113ae93611313565b809361138c565b0190565b6004936113e160206113f09997956113d96113d26014966113e998611318565b8092610e6f565b01809261135f565b018092611377565b0191611397565b90565b5f1b90565b61140191611318565b90565b61140c610132565b3d5f823e3d90fd5b5f1c90565b61142561142a91611414565b610153565b90565b6114379054611419565b90565b5f80fd5b60e01b90565b61144d81610a1d565b0361145457565b5f80fd5b9050519061146582611444565b565b919060408382031261148f578061148361148c925f8601611458565b93602001611458565b90565b61013c565b61149d9061076d565b9052565b6114aa90610230565b9052565b906020806114d0936114c65f8201515f8601906114a1565b01519101906114a1565b565b50600290565b905090565b90565b906114ed816020936114a1565b0190565b60200190565b61151361150d611506836114d2565b80946114d8565b916114dd565b5f915b8383106115235750505050565b61153961153360019284516114e0565b926114f1565b92019190611516565b90604060206115659361155b5f8201515f8601906114f7565b01519101906114f7565b565b61159d6115a49461159360e09498979561158961012086019a5f870190611494565b60208501906114ae565b6060830190611542565b01906114ae565b565b6115af906101bd565b90565b5f80fd5b67ffffffffffffffff81116115d4576115d06020916103a2565b0190565b6103ac565b909291926115ee6115e9826115b6565b610507565b9381855260208501908284011161160a5761160892610449565b565b6115b2565b9080601f8301121561162d5781602061162a935191016115d9565b90565b6107d6565b90602082820312611662575f82015167ffffffffffffffff811161165d5761165a920161160f565b90565b61022c565b61013c565b9092919261167c611677826115b6565b610507565b93818552602085019082840111611698576116969261138c565b565b6115b2565b6116a8913691611667565b90565b60200190565b85986116f9896116eb8a9d60209799989b5f979b61170a97506116d26105a1565b9496929091926116e0610132565b9788968c88016113b2565b8682018103825203826103c0565b611701610132565b918291826113f8565b039060025afa156119465761173b6117225f516113f3565b61173461172e8661076d565b9161076d565b1415610a1d565b6119115761177760409361178261175a611755600361142d565b6101c9565b9363171f1d5b92959761176b610132565b9889978896879661143e565b865260048601611567565b03915afa801561190c575f809290916118d9575b506117a19015610a1d565b9081156118c8575b50611897575f6117e3916117bc306115a6565b6117d8635ffd1f006117cc610132565b9586948593849361143e565b835260048301610723565b03915afa9182156118925761184361183d61181161185e9661186d9661184a965f91611870575b509461169d565b61182361181d8261043c565b916116ab565b20926118376118318261043c565b916116ab565b2061076d565b9161076d565b1415610a1d565b611852610132565b92839160208301610a2f565b602082018103825203826103c0565b90565b61188c91503d805f833e61188481836103c0565b810190611632565b5f61180a565b611404565b5050506118b66118c560016118aa610132565b92839160208301610a2f565b602082018103825203826103c0565b90565b6118d3915015610a1d565b5f6117a9565b6117a192506118ff915060403d8111611905575b6118f781836103c0565b810190611467565b91611796565b503d6118ed565b611404565b505050505050506119346119436001611928610132565b92839160208301610a2f565b602082018103825203826103c0565b90565b611404565b90611964989796959493929161195f610d8b565b6112b7565b90565b5f90565b6119bb907fdebfdfd5a50ad117c10898d68b5ccf0893c6b40d4f443f902e2e7646601bdeaf546001017fdebfdfd5a50ad117c10898d68b5ccf0893c6b40d4f443f902e2e7646601bdeaf55611a3c565b90565b906119ca5f19916113f3565b9181191691161790565b90565b906119ec6119e76119f392610274565b6119d4565b82546119be565b9055565b90611a0360ff916113f3565b9181191691161790565b611a1690610a1d565b90565b90565b90611a31611a2c611a3892611a0d565b611a19565b82546119f7565b9055565b50611a6b611a50611a4b612851565b611165565b611a5b8160016119d7565b611a656002610d93565b90610dc3565b611a7d611a775f610de8565b91610230565b14611a89816002611a1c565b90565b611a9c611a97611967565b61196b565b90565b90611af192917fdebfdfd5a50ad117c10898d68b5ccf0893c6b40d4f443f902e2e7646601bdeaf546001017fdebfdfd5a50ad117c10898d68b5ccf0893c6b40d4f443f902e2e7646601bdeaf55611efa565b90565b90565b611b0b611b06611b1092611af4565b61019e565b610230565b90565b60209181520190565b5f7f4d7573742073656e642065786163746c7920302e312045544800000000000000910152565b611b506019602092611b13565b611b5981611b1c565b0190565b611b729060208101905f818303910152611b43565b90565b15611b7c57565b611b84610132565b62461bcd60e51b815280611b9a60048201611b5d565b0390fd5b60081c90565b611bb0611bb591611b9e565b6105ec565b90565b611bc29054611ba4565b90565b5f910312611bcf57565b61013c565b5090565b5f7f496e76616c6964206f70636f6465206f66667365740000000000000000000000910152565b611c0c6015602092611b13565b611c1581611bd8565b0190565b611c2e9060208101905f818303910152611bff565b90565b15611c3857565b611c40610132565b62461bcd60e51b815280611c5660048201611c19565b0390fd5b9190811015611c6a576001020190565b610b10565b60ff60f81b1690565b60f81c90565b611c92611c8d611c9792610e04565b61019e565b610e04565b90565b611ca6611cab91611c78565b611c7e565b90565b611cb790610230565b5f198114611cc55760010190565b610f98565b60207f6520302900000000000000000000000000000000000000000000000000000000917f556e737570706f72746564206f7065726174696f6e20286f70206d75737420625f8201520152565b611d246024604092611b13565b611d2d81611cca565b0190565b611d469060208101905f818303910152611d17565b90565b15611d5057565b611d58610132565b62461bcd60e51b815280611d6e60048201611d31565b0390fd5b90565b611d89611d84611d8e92611d72565b61019e565b610230565b90565b5f7f4d697373696e6720736c6f742064617461000000000000000000000000000000910152565b611dc56011602092611b13565b611dce81611d91565b0190565b611de79060208101905f818303910152611db8565b90565b15611df157565b611df9610132565b62461bcd60e51b815280611e0f60048201611dd2565b0390fd5b5f7f4d697373696e672076616c756520646174610000000000000000000000000000910152565b611e476012602092611b13565b611e5081611e13565b0190565b611e699060208101905f818303910152611e3a565b90565b15611e7357565b611e7b610132565b62461bcd60e51b815280611e9160048201611e54565b0390fd5b611ea1611ea691611414565b610d1a565b90565b611eb39054611e95565b90565b611ec2611ec791611414565b610bea565b90565b611ed49054611eb6565b90565b916020611ef8929493611ef160408201965f830190610716565b0190610a22565b565b92919250611f2234611f1c611f1667016345785d8a0000611af7565b91610230565b14611b75565b611f34611f2f6002611bb8565b61062d565b63d0e30db0349190813b15612157575f91611f5b91611f51610132565b948593849261143e565b825281611f6a60048201610ce2565b03925af1801561215257612126575b50611f835f610de8565b915b82611fa2611f9c611f97858890611bd4565b610230565b91610230565b10156120e157611fe2611fbf84611fb96001610e29565b90611140565b611fdb611fd5611fd0868990611bd4565b610230565b91610230565b1115611c31565b61202461200b612005612000611ffa86898991611c5a565b35611c6f565b611c9a565b94611cae565b9361201e6120185f610e0a565b91610e04565b14611d49565b61205e61203b846120356020611d75565b90611140565b61205761205161204c868990611bd4565b610230565b91610230565b1115611dea565b612066610f88565b5061207f83830135936120796020611d75565b90611140565b6120b9612096826120906020611d75565b90611140565b6120b26120ac6120a7878a90611bd4565b610230565b91610230565b1115611e6c565b6120c1610f88565b506120da81840135916120d46020611d75565b90611140565b9355611f85565b925050506120ef6001611ea9565b6121236120fc6002611eca565b91612114612108610132565b93849260208401611ed7565b602082018103825203826103c0565b90565b612145905f3d811161214b575b61213d81836103c0565b810190611bc5565b5f611f79565b503d612133565b611404565b61143a565b9061216e91612169610d8b565b611a9f565b90565b60209181520190565b612183906106b3565b9052565b906121948160209361217a565b0190565b6121a46121a991611414565b610b59565b90565b6121b69054612198565b90565b60010190565b906121dc6121d66121cf84610b24565b8093612171565b92610b28565b905f5b8181106121ec5750505090565b90919261220c612206600192612201876121ac565b612187565b946121b9565b91019190916121df565b61222b9160208201915f8184039101526121bf565b90565b612236610d8b565b506122526122615f612246610132565b92839160208301612216565b602082018103825203826103c0565b90565b906122bd9998979695949392917fdebfdfd5a50ad117c10898d68b5ccf0893c6b40d4f443f902e2e7646601bdeaf546001017fdebfdfd5a50ad117c10898d68b5ccf0893c6b40d4f443f902e2e7646601bdeaf55612469565b90565b156122c757565b5f6339bb705160e11b8152806122df60048201610ce2565b0390fd5b5f7f496e76616c6964207369676e6174757265000000000000000000000000000000910152565b6123176011602092611b13565b612320816122e3565b0190565b6123399060208101905f81830391015261230a565b90565b1561234357565b61234b610132565b62461bcd60e51b81528061236160048201612324565b0390fd5b5f7f424c532070616972696e6720636865636b206661696c65640000000000000000910152565b6123996018602092611b13565b6123a281612365565b0190565b6123bb9060208101905f81830391015261238c565b90565b156123c557565b6123cd610132565b62461bcd60e51b8152806123e3600482016123a6565b0390fd5b5f7f496e76616c696420424c53207369676e61747572650000000000000000000000910152565b61241b6015602092611b13565b612424816123e7565b0190565b61243d9060208101905f81830391015261240e565b90565b1561244757565b61244f610132565b62461bcd60e51b81528061246560048201612428565b0390fd5b999890959997919497969296506124ac61248d846124876001610e29565b90611140565b6124a66124a061249b612851565b610230565b91610230565b146122c0565b6124d0346124ca6124c467016345785d8a0000611af7565b91610230565b14611b75565b6124e26124dd6002611bb8565b61062d565b9163d0e30db0349390813b15612830575f9161250a91612500610132565b968793849261143e565b82528161251960048201610ce2565b03925af1801561282b5760209461255c61257b948e5f9761256a956127ff575b508d6125436105a1565b949692909192612551610132565b9788968c88016113b2565b8682018103825203826103c0565b612572610132565b918291826113f8565b039060025afa156127fa576125e36040936125b16125995f516113f3565b6125ab6125a58461076d565b9161076d565b1461233c565b6125ee6125c66125c1600361142d565b6101c9565b9363171f1d5b9295976125d7610132565b9889978896879661143e565b865260048601611567565b03915afa9081156127f557612616915f809290916127c2575b50612611906123be565b612440565b61261f5f610de8565b915b8261263e612638612633858890611bd4565b610230565b91610230565b101561277d5761267e61265b846126556001610e29565b90611140565b61267761267161266c868990611bd4565b610230565b91610230565b1115611c31565b6126c06126a76126a161269c61269686898991611c5a565b35611c6f565b611c9a565b94611cae565b936126ba6126b45f610e0a565b91610e04565b14611d49565b6126fa6126d7846126d16020611d75565b90611140565b6126f36126ed6126e8868990611bd4565b610230565b91610230565b1115611dea565b612702610f88565b5061271b83830135936127156020611d75565b90611140565b6127556127328261272c6020611d75565b90611140565b61274e612748612743878a90611bd4565b610230565b91610230565b1115611e6c565b61275d610f88565b5061277681840135916127706020611d75565b90611140565b9355612621565b9250505061278b6001611ea9565b6127bf6127986002611eca565b916127b06127a4610132565b93849260208401611ed7565b602082018103825203826103c0565b90565b61261192506127e8915060403d81116127ee575b6127e081836103c0565b810190611467565b91612607565b503d6127d6565b611404565b611404565b61281e90893d8111612824575b61281681836103c0565b810190611bc5565b5f612539565b503d61280c565b611404565b61143a565b9061284e9897969594939291612849610d8b565b612264565b90565b612859610f88565b507fdebfdfd5a50ad117c10898d68b5ccf0893c6b40d4f443f902e2e7646601bdeaf5490565b6128cf907fdebfdfd5a50ad117c10898d68b5ccf0893c6b40d4f443f902e2e7646601bdeaf546001017fdebfdfd5a50ad117c10898d68b5ccf0893c6b40d4f443f902e2e7646601bdeaf55612b7a565b565b90565b5f5260205f2090565b5490565b6128ea816128dd565b821015612904576128fc6001916128d4565b910201905f90565b610b10565b1b90565b9190600861292d91029161292760018060a01b0384612909565b92612909565b9181191691161790565b612940906101bd565b90565b90565b919061295c61295761296493612937565b612943565b90835461290d565b9055565b90815491680100000000000000008310156129985782612990916001612996950181556128e1565b90612946565b565b6103ac565b601f602091010490565b919060086129c29102916129bc5f1984612909565b92612909565b9181191691161790565b91906129e26129dd6129ea93610274565b6119d4565b9083546129a7565b9055565b612a00916129fa610f88565b916129cc565b565b5b818110612a0e575050565b80612a1b5f6001936129ee565b01612a03565b9190601f8111612a31575b505050565b612a3d612a6293610300565b906020612a498461299d565b83019310612a6a575b612a5b9061299d565b0190612a02565b5f8080612a2c565b9150612a5b81929050612a52565b90612a88905f199060080261014f565b191690565b81612a9791612a78565b906002021790565b90612aa98161043c565b9067ffffffffffffffff8211612b6957612acd82612ac785546102cd565b85612a21565b602090601f8311600114612b0157918091612af0935f92612af5575b5050612a8d565b90555b565b90915001515f80612ae9565b601f19831691612b1085610300565b925f5b818110612b5157509160029391856001969410612b37575b50505002019055612af3565b612b47910151601f841690612a78565b90555f8080612b2b565b91936020600181928787015181550195019201612b13565b6103ac565b90612b7891612a9f565b565b612b8c90612b875f6128d1565b612968565b612bd1612baa612bb95f612b9e610132565b92839160208301612216565b602082018103825203826103c0565b612bcc6004612bc6612851565b90610290565b612b6e565b565b612bdc9061287f565b56fea2646970667358221220baa6f9f1286e16243c6a957ad1c7d318a431b9dcef6d5382dda64d2558fffbba64736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x13W[a\r\x87V[a\0\x1D_5a\x01,V[\x80c\x1C\x17\x8E\x9C\x14a\x01'W\x80c\x1E\xE0\x7F\xB1\x14a\x01\"W\x80c_\xFD\x1F\0\x14a\x01\x1DW\x80c|\x01Z\x89\x14a\x01\x18W\x80c\x9C\x91\xDDV\x14a\x01\x13W\x80c\xA3R\xD7\x07\x14a\x01\x0EW\x80c\xA4\xF8\x81T\x14a\x01\tW\x80c\xAA,X\xB2\x14a\x01\x04W\x80c\xBF\xA0h0\x14a\0\xFFW\x80c\xC7\xB3u@\x14a\0\xFAW\x80c\xCFj\xE6\x03\x14a\0\xF5W\x80c\xDAX\xC7\xD9\x14a\0\xF0W\x80c\xEAS\xBA\xC5\x14a\0\xEBW\x80c\xF3\0\xB8b\x14a\0\xE6W\x80c\xF4\x83> \x14a\0\xE1W\x80c\xF4\xAB\x9A\xDF\x14a\0\xDCWc\xFD\x8E\xACI\x03a\0\x0EWa\rRV[a\x0C\xE7V[a\x0C\x8FV[a\x0CZV[a\x0C%V[a\x0B\xB5V[a\n\xDBV[a\n\xAFV[a\nDV[a\t\xDEV[a\x078V[a\x06\xE1V[a\x06[V[a\x05\xB7V[a\x04\xD2V[a\x04\x9DV[a\x01\xF7V[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[_\x91\x03\x12a\x01JWV[a\x01<V[\x1C\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x01n\x90`\x08a\x01s\x93\x02a\x01OV[a\x01SV[\x90V[\x90a\x01\x81\x91Ta\x01^V[\x90V[a\x01\x90`\x03_\x90a\x01vV[\x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[\x90V[a\x01\xB5a\x01\xB0a\x01\xBA\x92a\x01\x93V[a\x01\x9EV[a\x01\x93V[\x90V[a\x01\xC6\x90a\x01\xA1V[\x90V[a\x01\xD2\x90a\x01\xBDV[\x90V[a\x01\xDE\x90a\x01\xC9V[\x90RV[\x91\x90a\x01\xF5\x90_` \x85\x01\x94\x01\x90a\x01\xD5V[V[4a\x02'Wa\x02\x076`\x04a\x01@V[a\x02#a\x02\x12a\x01\x84V[a\x02\x1Aa\x012V[\x91\x82\x91\x82a\x01\xE2V[\x03\x90\xF3[a\x018V[_\x80\xFD[\x90V[a\x02<\x81a\x020V[\x03a\x02CWV[_\x80\xFD[\x90P5\x90a\x02T\x82a\x023V[V[\x90` \x82\x82\x03\x12a\x02oWa\x02l\x91_\x01a\x02GV[\x90V[a\x01<V[a\x02\x88a\x02\x83a\x02\x8D\x92a\x020V[a\x01\x9EV[a\x020V[\x90V[\x90a\x02\x9A\x90a\x02tV[_R` R`@_ \x90V[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x90`\x01`\x02\x83\x04\x92\x16\x80\x15a\x02\xEDW[` \x83\x10\x14a\x02\xE8WV[a\x02\xB9V[\x91`\x7F\x16\x91a\x02\xDDV[` \x91\x81R\x01\x90V[_R` _ \x90V[\x90_\x92\x91\x80T\x90a\x03#a\x03\x1C\x83a\x02\xCDV[\x80\x94a\x02\xF7V[\x91`\x01\x81\x16\x90\x81_\x14a\x03zWP`\x01\x14a\x03>W[PPPV[a\x03K\x91\x92\x93\x94Pa\x03\0V[\x91_\x92[\x81\x84\x10a\x03bWPP\x01\x90_\x80\x80a\x039V[`\x01\x81` \x92\x95\x93\x95T\x84\x86\x01R\x01\x91\x01\x92\x90a\x03OV[\x92\x94\x95PPP`\xFF\x19\x16\x82R\x15\x15` \x02\x01\x90_\x80\x80a\x039V[\x90a\x03\x9F\x91a\x03\tV[\x90V[`\x1F\x80\x19\x91\x01\x16\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\x03\xCA\x90a\x03\xA2V[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\xE4W`@RV[a\x03\xACV[\x90a\x04\ta\x04\x02\x92a\x03\xF9a\x012V[\x93\x84\x80\x92a\x03\x95V[\x03\x83a\x03\xC0V[V[\x90_\x10a\x04\x1EWa\x04\x1B\x90a\x03\xE9V[\x90V[a\x02\xA6V[a\x049\x90a\x044`\x04\x91_\x92a\x02\x90V[a\x04\x0BV[\x90V[Q\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[a\x04sa\x04|` \x93a\x04\x81\x93a\x04j\x81a\x04<V[\x93\x84\x80\x93a\x04@V[\x95\x86\x91\x01a\x04IV[a\x03\xA2V[\x01\x90V[a\x04\x9A\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x04TV[\x90V[4a\x04\xCDWa\x04\xC9a\x04\xB8a\x04\xB36`\x04a\x02VV[a\x04#V[a\x04\xC0a\x012V[\x91\x82\x91\x82a\x04\x85V[\x03\x90\xF3[a\x018V[4a\x05\x02Wa\x04\xFEa\x04\xEDa\x04\xE86`\x04a\x02VV[a\x0E\xD8V[a\x04\xF5a\x012V[\x91\x82\x91\x82a\x04\x85V[\x03\x90\xF3[a\x018V[\x90a\x05\x1Aa\x05\x13a\x012V[\x92\x83a\x03\xC0V[V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x05:Wa\x056` \x91a\x03\xA2V[\x01\x90V[a\x03\xACV[\x90a\x05Qa\x05L\x83a\x05\x1CV[a\x05\x07V[\x91\x82RV[_\x7F_COMMONWARE_AGGREGATION_\0\0\0\0\0\0\0\0\x91\x01RV[a\x05\x87`\x18a\x05?V[\x90a\x05\x94` \x83\x01a\x05VV[V[a\x05\x9Ea\x05}V[\x90V[a\x05\xA9a\x05\x96V[\x90V[a\x05\xB4a\x05\xA1V[\x90V[4a\x05\xE7Wa\x05\xC76`\x04a\x01@V[a\x05\xE3a\x05\xD2a\x05\xACV[a\x05\xDAa\x012V[\x91\x82\x91\x82a\x04\x85V[\x03\x90\xF3[a\x018V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x06\x07\x90`\x08a\x06\x0C\x93\x02a\x01OV[a\x05\xECV[\x90V[\x90a\x06\x1A\x91Ta\x05\xF7V[\x90V[a\x06*`\x02`\x01\x90a\x06\x0FV[\x90V[a\x066\x90a\x01\xBDV[\x90V[a\x06B\x90a\x06-V[\x90RV[\x91\x90a\x06Y\x90_` \x85\x01\x94\x01\x90a\x069V[V[4a\x06\x8BWa\x06k6`\x04a\x01@V[a\x06\x87a\x06va\x06\x1DV[a\x06~a\x012V[\x91\x82\x91\x82a\x06FV[\x03\x90\xF3[a\x018V[s\xCA$\x92\x15\xE0\x82\xE1|\x12\xBB<H\x81\x83\x9A?\x88>\\k\x90V[a\x06\xB0a\x06\x90V[\x90V[a\x06\xBC\x90a\x01\x93V[\x90V[a\x06\xC8\x90a\x06\xB3V[\x90RV[\x91\x90a\x06\xDF\x90_` \x85\x01\x94\x01\x90a\x06\xBFV[V[4a\x07\x11Wa\x06\xF16`\x04a\x01@V[a\x07\ra\x06\xFCa\x06\xA8V[a\x07\x04a\x012V[\x91\x82\x91\x82a\x06\xCCV[\x03\x90\xF3[a\x018V[a\x07\x1F\x90a\x020V[\x90RV[\x91\x90a\x076\x90_` \x85\x01\x94\x01\x90a\x07\x16V[V[4a\x07hWa\x07da\x07Sa\x07N6`\x04a\x02VV[a\x11eV[a\x07[a\x012V[\x91\x82\x91\x82a\x07#V[\x03\x90\xF3[a\x018V[\x90V[a\x07y\x81a\x07mV[\x03a\x07\x80WV[_\x80\xFD[\x90P5\x90a\x07\x91\x82a\x07pV[V[_\x80\xFD[\x91\x90`@\x83\x82\x03\x12a\x07\xD1Wa\x07\xCA\x90a\x07\xB1`@a\x05\x07V[\x93a\x07\xBE\x82_\x83\x01a\x02GV[_\x86\x01R` \x01a\x02GV[` \x83\x01RV[a\x07\x93V[_\x80\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07\xEFW` \x02\x90V[a\x03\xACV[_\x80\xFD[\x90\x92\x91\x92a\x08\ra\x08\x08\x82a\x07\xDAV[a\x05\x07V[\x93` \x85\x92\x02\x83\x01\x92\x81\x84\x11a\x08EW\x91[\x83\x83\x10a\x08,WPPPPV[` \x80\x91a\x08:\x84\x86a\x02GV[\x81R\x01\x92\x01\x91a\x08\x1FV[a\x07\xF4V[\x90\x80`\x1F\x83\x01\x12\x15a\x08eWa\x08b\x91`\x02\x90a\x07\xF8V[\x90V[a\x07\xD6V[\x91\x90`\x80\x83\x82\x03\x12a\x08\xA4Wa\x08\x9D\x90a\x08\x84`@a\x05\x07V[\x93a\x08\x91\x82_\x83\x01a\x08JV[_\x86\x01R`@\x01a\x08JV[` \x83\x01RV[a\x07\x93V[_\x80\xFD[\x90\x91\x82`\x1F\x83\x01\x12\x15a\x08\xE7W\x815\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x08\xE2W` \x01\x92`\x01\x83\x02\x84\x01\x11a\x08\xDDWV[a\x07\xF4V[a\x08\xA9V[a\x07\xD6V[a\x08\xF5\x81a\x06\xB3V[\x03a\x08\xFCWV[_\x80\xFD[\x90P5\x90a\t\r\x82a\x08\xECV[V[c\xFF\xFF\xFF\xFF`\xE0\x1B\x16\x90V[a\t$\x81a\t\x0FV[\x03a\t+WV[_\x80\xFD[\x90P5\x90a\t<\x82a\t\x1BV[V[\x91\x90\x91a\x01\xA0\x81\x84\x03\x12a\t\xD9Wa\tX\x83_\x83\x01a\x07\x84V[\x92a\tf\x81` \x84\x01a\x07\x97V[\x92a\tt\x82``\x85\x01a\x08jV[\x92a\t\x82\x83`\xE0\x83\x01a\x07\x97V[\x92a\x01 \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\t\xD4W\x81a\t\xA4\x91\x84\x01a\x08\xADV[\x92\x90\x93a\t\xD1a\t\xB8\x84a\x01@\x85\x01a\x02GV[\x93a\t\xC7\x81a\x01`\x86\x01a\t\0V[\x93a\x01\x80\x01a\t/V[\x90V[a\x02,V[a\x01<V[4a\n\x18Wa\n\x14a\n\x03a\t\xF46`\x04a\t>V[\x97\x96\x90\x96\x95\x91\x95\x94\x92\x94a\x19KV[a\n\x0Ba\x012V[\x91\x82\x91\x82a\x04\x85V[\x03\x90\xF3[a\x018V[\x15\x15\x90V[a\n+\x90a\n\x1DV[\x90RV[\x91\x90a\nB\x90_` \x85\x01\x94\x01\x90a\n\"V[V[4a\ntWa\nT6`\x04a\x01@V[a\npa\n_a\x1A\x8CV[a\nga\x012V[\x91\x82\x91\x82a\n/V[\x03\x90\xF3[a\x018V[\x90` \x82\x82\x03\x12a\n\xAAW_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\n\xA5Wa\n\xA1\x92\x01a\x08\xADV[\x90\x91V[a\x02,V[a\x01<V[a\n\xD7a\n\xC6a\n\xC06`\x04a\nyV[\x90a!\\V[a\n\xCEa\x012V[\x91\x82\x91\x82a\x04\x85V[\x03\x90\xF3[4a\x0B\x0BWa\n\xEB6`\x04a\x01@V[a\x0B\x07a\n\xF6a\".V[a\n\xFEa\x012V[\x91\x82\x91\x82a\x04\x85V[\x03\x90\xF3[a\x018V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[T\x90V[_R` _ \x90V[a\x0B:\x81a\x0B$V[\x82\x10\x15a\x0BTWa\x0BL`\x01\x91a\x0B(V[\x91\x02\x01\x90_\x90V[a\x0B\x10V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x0Bt\x90`\x08a\x0By\x93\x02a\x01OV[a\x0BYV[\x90V[\x90a\x0B\x87\x91Ta\x0BdV[\x90V[_a\x0B\x94\x81a\x0B$V[\x82\x10\x15a\x0B\xB1Wa\x0B\xAE\x91a\x0B\xA8\x91a\x0B1V[\x90a\x0B|V[\x90V[_\x80\xFD[4a\x0B\xE5Wa\x0B\xE1a\x0B\xD0a\x0B\xCB6`\x04a\x02VV[a\x0B\x8AV[a\x0B\xD8a\x012V[\x91\x82\x91\x82a\x06\xCCV[\x03\x90\xF3[a\x018V[`\xFF\x16\x90V[a\x0C\0\x90`\x08a\x0C\x05\x93\x02a\x01OV[a\x0B\xEAV[\x90V[\x90a\x0C\x13\x91Ta\x0B\xF0V[\x90V[a\x0C\"`\x02_\x90a\x0C\x08V[\x90V[4a\x0CUWa\x0C56`\x04a\x01@V[a\x0CQa\x0C@a\x0C\x16V[a\x0CHa\x012V[\x91\x82\x91\x82a\n/V[\x03\x90\xF3[a\x018V[a\x0C\x8Ba\x0Cza\x0Ck6`\x04a\t>V[\x97\x96\x90\x96\x95\x91\x95\x94\x92\x94a(5V[a\x0C\x82a\x012V[\x91\x82\x91\x82a\x04\x85V[\x03\x90\xF3[4a\x0C\xBFWa\x0C\x9F6`\x04a\x01@V[a\x0C\xBBa\x0C\xAAa(QV[a\x0C\xB2a\x012V[\x91\x82\x91\x82a\x07#V[\x03\x90\xF3[a\x018V[\x90` \x82\x82\x03\x12a\x0C\xDDWa\x0C\xDA\x91_\x01a\t\0V[\x90V[a\x01<V[_\x01\x90V[4a\r\x15Wa\x0C\xFFa\x0C\xFA6`\x04a\x0C\xC4V[a+\xD3V[a\r\x07a\x012V[\x80a\r\x11\x81a\x0C\xE2V[\x03\x90\xF3[a\x018V[\x90V[a\r-\x90`\x08a\r2\x93\x02a\x01OV[a\r\x1AV[\x90V[\x90a\r@\x91Ta\r\x1DV[\x90V[a\rO`\x01_\x90a\r5V[\x90V[4a\r\x82Wa\rb6`\x04a\x01@V[a\r~a\rma\rCV[a\rua\x012V[\x91\x82\x91\x82a\x07#V[\x03\x90\xF3[a\x018V[_\x80\xFD[``\x90V[\x90V[a\r\xA7a\r\xA2a\r\xAC\x92a\r\x90V[a\x01\x9EV[a\x020V[\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a\r\xCFa\r\xD5\x91a\x020V[\x91a\x020V[\x90\x81\x15a\r\xE0W\x06\x90V[a\r\xAFV[\x90V[a\r\xFCa\r\xF7a\x0E\x01\x92a\r\xE5V[a\x01\x9EV[a\x020V[\x90V[`\xFF\x16\x90V[a\x0E\x1Ea\x0E\x19a\x0E#\x92a\r\xE5V[a\x01\x9EV[a\x0E\x04V[\x90V[\x90V[a\x0E=a\x0E8a\x0EB\x92a\x0E&V[a\x01\x9EV[a\x020V[\x90V[`\xF8\x1B\x90V[a\x0ET\x90a\x0EEV[\x90V[a\x0Eca\x0Eh\x91a\x0E\x04V[a\x0EKV[\x90RV[\x90V[a\x0E{a\x0E\x80\x91a\x020V[a\x0ElV[\x90RV[a\x0E\xD4\x94a\x0E\xC4` \x80\x99\x98\x95\x96a\x0E\xBC\x82\x80\x99a\x0E\xB4`\x01\x89a\x0E\xAC\x82\x9Ba\x0E\xCC\x9Da\x0EWV[\x01\x80\x92a\x0EoV[\x01\x80\x92a\x0EoV[\x01\x80\x92a\x0EWV[\x01\x80\x92a\x0EoV[\x01\x80\x92a\x0EoV[\x01\x90V[a\x0E\xEA\x90a\x0E\xE4a\r\x8BV[Pa\x11eV[a\x0Fva\x0F\x01\x82a\x0E\xFB`\x02a\r\x93V[\x90a\r\xC3V[a\x0F\x13a\x0F\r_a\r\xE8V[\x91a\x020V[\x14a\x0Fga\x0F _a\x0E\nV[\x91a\x0F+`\x01a\x0E)V[\x94\x90a\x0F6_a\x0E\nV[a\x0F@`\x02a\r\x93V[\x91_\x14a\x0FyWa\x0FQ`\x01a\x0E)V[\x92[a\x0F[a\x012V[\x97\x88\x96` \x88\x01a\x0E\x84V[` \x82\x01\x81\x03\x82R\x03\x82a\x03\xC0V[\x90V[a\x0F\x82_a\r\xE8V[\x92a\x0FSV[_\x90V[a\x0F\x95\x90a\x03\xE9V[\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x0F\xB5\x90a\x020V[_\x81\x14a\x0F\xC3W`\x01\x90\x03\x90V[a\x0F\x98V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0F\xE0W` \x80\x91\x02\x01\x90V[a\x03\xACV[\x90PQ\x90a\x0F\xF2\x82a\x08\xECV[V[\x90\x92\x91\x92a\x10\ta\x10\x04\x82a\x0F\xC8V[a\x05\x07V[\x93\x81\x85R` \x80\x86\x01\x92\x02\x83\x01\x92\x81\x84\x11a\x10FW\x91[\x83\x83\x10a\x10-WPPPPV[` \x80\x91a\x10;\x84\x86a\x0F\xE5V[\x81R\x01\x92\x01\x91a\x10 V[a\x07\xF4V[\x90\x80`\x1F\x83\x01\x12\x15a\x10iW\x81` a\x10f\x93Q\x91\x01a\x0F\xF4V[\x90V[a\x07\xD6V[\x90` \x82\x82\x03\x12a\x10\x9EW_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x10\x99Wa\x10\x96\x92\x01a\x10KV[\x90V[a\x02,V[a\x01<V[`\x01a\x10\xAF\x91\x01a\x020V[\x90V[Q\x90V[\x90a\x10\xC0\x82a\x10\xB2V[\x81\x10\x15a\x10\xD1W` \x80\x91\x02\x01\x01\x90V[a\x0B\x10V[a\x10\xE0\x90Qa\x06\xB3V[\x90V[a\x10\xEC\x90a\x01\xA1V[\x90V[a\x11\x03a\x10\xFEa\x11\x08\x92a\x01\x93V[a\x01\x9EV[a\x020V[\x90V[a\x11\x1Aa\x11 \x91\x93\x92\x93a\x020V[\x92a\x020V[\x91a\x11,\x83\x82\x02a\x020V[\x92\x81\x84\x04\x14\x90\x15\x17\x15a\x11;WV[a\x0F\x98V[a\x11Oa\x11U\x91\x93\x92\x93a\x020V[\x92a\x020V[\x82\x01\x80\x92\x11a\x11`WV[a\x0F\x98V[\x90a\x11na\x0F\x88V[Pa\x11\x83a\x11~`\x04\x84\x90a\x02\x90V[a\x0F\x8CV[\x82\x90[a\x11\x8F\x81a\x04<V[a\x11\xA1a\x11\x9B_a\r\xE8V[\x91a\x020V[\x14\x80a\x12\x9DW[\x15a\x11\xFBWP\x80a\x11\xC1a\x11\xBB_a\r\xE8V[\x91a\x020V[\x14a\x11\xECWa\x11\xCF\x90a\x0F\xACV[\x91a\x11\xE4a\x11\xDF`\x04\x85\x90a\x02\x90V[a\x0F\x8CV[\x92\x90\x92a\x11\x86V[P\x90Pa\x11\xF8_a\r\xE8V[\x90V[a\x12\x19\x91P\x92\x91\x92` a\x12\x0E\x82a\x04<V[\x81\x83\x01\x01\x91\x01a\x10nV[\x90a\x12#_a\r\xE8V[\x91a\x12-_a\r\xE8V[\x92[\x83a\x12Ja\x12Da\x12?\x85a\x10\xB2V[a\x020V[\x91a\x020V[\x10\x15a\x12\x96Wa\x12\x8Aa\x12\x90\x91a\x12\x84a\x12ua\x12pa\x12k\x87\x8A\x90a\x10\xB6V[a\x10\xD6V[a\x10\xE3V[a\x12\x7F\x87\x91a\x10\xEFV[a\x11\x0BV[\x90a\x11@V[\x93a\x10\xA3V[\x92a\x12/V[\x92PPP\x90V[P\x81a\x12\xB1a\x12\xAB_a\r\xE8V[\x91a\x020V[\x11a\x11\xA8V[\x90a\x13\x10\x99\x98\x97\x96\x95\x94\x93\x92\x91\x7F\xDE\xBF\xDF\xD5\xA5\n\xD1\x17\xC1\x08\x98\xD6\x8B\\\xCF\x08\x93\xC6\xB4\rOD?\x90..vF`\x1B\xDE\xAFT`\x01\x01\x7F\xDE\xBF\xDF\xD5\xA5\n\xD1\x17\xC1\x08\x98\xD6\x8B\\\xCF\x08\x93\xC6\xB4\rOD?\x90..vF`\x1B\xDE\xAFUa\x16\xB1V[\x90V[\x90P\x90V[a\x13=a\x134\x92` \x92a\x13+\x81a\x04<V[\x94\x85\x80\x93a\x13\x13V[\x93\x84\x91\x01a\x04IV[\x01\x90V[``\x1B\x90V[a\x13P\x90a\x13AV[\x90V[a\x13\\\x90a\x13GV[\x90V[a\x13ka\x13p\x91a\x06\xB3V[a\x13SV[\x90RV[\x90V[a\x13\x83a\x13\x88\x91a\t\x0FV[a\x13tV[\x90RV[\x90\x82_\x93\x92\x827\x01RV[\x90\x91\x82a\x13\xA7\x81a\x13\xAE\x93a\x13\x13V[\x80\x93a\x13\x8CV[\x01\x90V[`\x04\x93a\x13\xE1` a\x13\xF0\x99\x97\x95a\x13\xD9a\x13\xD2`\x14\x96a\x13\xE9\x98a\x13\x18V[\x80\x92a\x0EoV[\x01\x80\x92a\x13_V[\x01\x80\x92a\x13wV[\x01\x91a\x13\x97V[\x90V[_\x1B\x90V[a\x14\x01\x91a\x13\x18V[\x90V[a\x14\x0Ca\x012V[=_\x82>=\x90\xFD[_\x1C\x90V[a\x14%a\x14*\x91a\x14\x14V[a\x01SV[\x90V[a\x147\x90Ta\x14\x19V[\x90V[_\x80\xFD[`\xE0\x1B\x90V[a\x14M\x81a\n\x1DV[\x03a\x14TWV[_\x80\xFD[\x90PQ\x90a\x14e\x82a\x14DV[V[\x91\x90`@\x83\x82\x03\x12a\x14\x8FW\x80a\x14\x83a\x14\x8C\x92_\x86\x01a\x14XV[\x93` \x01a\x14XV[\x90V[a\x01<V[a\x14\x9D\x90a\x07mV[\x90RV[a\x14\xAA\x90a\x020V[\x90RV[\x90` \x80a\x14\xD0\x93a\x14\xC6_\x82\x01Q_\x86\x01\x90a\x14\xA1V[\x01Q\x91\x01\x90a\x14\xA1V[V[P`\x02\x90V[\x90P\x90V[\x90V[\x90a\x14\xED\x81` \x93a\x14\xA1V[\x01\x90V[` \x01\x90V[a\x15\x13a\x15\ra\x15\x06\x83a\x14\xD2V[\x80\x94a\x14\xD8V[\x91a\x14\xDDV[_\x91[\x83\x83\x10a\x15#WPPPPV[a\x159a\x153`\x01\x92\x84Qa\x14\xE0V[\x92a\x14\xF1V[\x92\x01\x91\x90a\x15\x16V[\x90`@` a\x15e\x93a\x15[_\x82\x01Q_\x86\x01\x90a\x14\xF7V[\x01Q\x91\x01\x90a\x14\xF7V[V[a\x15\x9Da\x15\xA4\x94a\x15\x93`\xE0\x94\x98\x97\x95a\x15\x89a\x01 \x86\x01\x9A_\x87\x01\x90a\x14\x94V[` \x85\x01\x90a\x14\xAEV[``\x83\x01\x90a\x15BV[\x01\x90a\x14\xAEV[V[a\x15\xAF\x90a\x01\xBDV[\x90V[_\x80\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x15\xD4Wa\x15\xD0` \x91a\x03\xA2V[\x01\x90V[a\x03\xACV[\x90\x92\x91\x92a\x15\xEEa\x15\xE9\x82a\x15\xB6V[a\x05\x07V[\x93\x81\x85R` \x85\x01\x90\x82\x84\x01\x11a\x16\nWa\x16\x08\x92a\x04IV[V[a\x15\xB2V[\x90\x80`\x1F\x83\x01\x12\x15a\x16-W\x81` a\x16*\x93Q\x91\x01a\x15\xD9V[\x90V[a\x07\xD6V[\x90` \x82\x82\x03\x12a\x16bW_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x16]Wa\x16Z\x92\x01a\x16\x0FV[\x90V[a\x02,V[a\x01<V[\x90\x92\x91\x92a\x16|a\x16w\x82a\x15\xB6V[a\x05\x07V[\x93\x81\x85R` \x85\x01\x90\x82\x84\x01\x11a\x16\x98Wa\x16\x96\x92a\x13\x8CV[V[a\x15\xB2V[a\x16\xA8\x916\x91a\x16gV[\x90V[` \x01\x90V[\x85\x98a\x16\xF9\x89a\x16\xEB\x8A\x9D` \x97\x99\x98\x9B_\x97\x9Ba\x17\n\x97Pa\x16\xD2a\x05\xA1V[\x94\x96\x92\x90\x91\x92a\x16\xE0a\x012V[\x97\x88\x96\x8C\x88\x01a\x13\xB2V[\x86\x82\x01\x81\x03\x82R\x03\x82a\x03\xC0V[a\x17\x01a\x012V[\x91\x82\x91\x82a\x13\xF8V[\x03\x90`\x02Z\xFA\x15a\x19FWa\x17;a\x17\"_Qa\x13\xF3V[a\x174a\x17.\x86a\x07mV[\x91a\x07mV[\x14\x15a\n\x1DV[a\x19\x11Wa\x17w`@\x93a\x17\x82a\x17Za\x17U`\x03a\x14-V[a\x01\xC9V[\x93c\x17\x1F\x1D[\x92\x95\x97a\x17ka\x012V[\x98\x89\x97\x88\x96\x87\x96a\x14>V[\x86R`\x04\x86\x01a\x15gV[\x03\x91Z\xFA\x80\x15a\x19\x0CW_\x80\x92\x90\x91a\x18\xD9W[Pa\x17\xA1\x90\x15a\n\x1DV[\x90\x81\x15a\x18\xC8W[Pa\x18\x97W_a\x17\xE3\x91a\x17\xBC0a\x15\xA6V[a\x17\xD8c_\xFD\x1F\0a\x17\xCCa\x012V[\x95\x86\x94\x85\x93\x84\x93a\x14>V[\x83R`\x04\x83\x01a\x07#V[\x03\x91Z\xFA\x91\x82\x15a\x18\x92Wa\x18Ca\x18=a\x18\x11a\x18^\x96a\x18m\x96a\x18J\x96_\x91a\x18pW[P\x94a\x16\x9DV[a\x18#a\x18\x1D\x82a\x04<V[\x91a\x16\xABV[ \x92a\x187a\x181\x82a\x04<V[\x91a\x16\xABV[ a\x07mV[\x91a\x07mV[\x14\x15a\n\x1DV[a\x18Ra\x012V[\x92\x83\x91` \x83\x01a\n/V[` \x82\x01\x81\x03\x82R\x03\x82a\x03\xC0V[\x90V[a\x18\x8C\x91P=\x80_\x83>a\x18\x84\x81\x83a\x03\xC0V[\x81\x01\x90a\x162V[_a\x18\nV[a\x14\x04V[PPPa\x18\xB6a\x18\xC5`\x01a\x18\xAAa\x012V[\x92\x83\x91` \x83\x01a\n/V[` \x82\x01\x81\x03\x82R\x03\x82a\x03\xC0V[\x90V[a\x18\xD3\x91P\x15a\n\x1DV[_a\x17\xA9V[a\x17\xA1\x92Pa\x18\xFF\x91P`@=\x81\x11a\x19\x05W[a\x18\xF7\x81\x83a\x03\xC0V[\x81\x01\x90a\x14gV[\x91a\x17\x96V[P=a\x18\xEDV[a\x14\x04V[PPPPPPPa\x194a\x19C`\x01a\x19(a\x012V[\x92\x83\x91` \x83\x01a\n/V[` \x82\x01\x81\x03\x82R\x03\x82a\x03\xC0V[\x90V[a\x14\x04V[\x90a\x19d\x98\x97\x96\x95\x94\x93\x92\x91a\x19_a\r\x8BV[a\x12\xB7V[\x90V[_\x90V[a\x19\xBB\x90\x7F\xDE\xBF\xDF\xD5\xA5\n\xD1\x17\xC1\x08\x98\xD6\x8B\\\xCF\x08\x93\xC6\xB4\rOD?\x90..vF`\x1B\xDE\xAFT`\x01\x01\x7F\xDE\xBF\xDF\xD5\xA5\n\xD1\x17\xC1\x08\x98\xD6\x8B\\\xCF\x08\x93\xC6\xB4\rOD?\x90..vF`\x1B\xDE\xAFUa\x1A<V[\x90V[\x90a\x19\xCA_\x19\x91a\x13\xF3V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[\x90a\x19\xECa\x19\xE7a\x19\xF3\x92a\x02tV[a\x19\xD4V[\x82Ta\x19\xBEV[\x90UV[\x90a\x1A\x03`\xFF\x91a\x13\xF3V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x1A\x16\x90a\n\x1DV[\x90V[\x90V[\x90a\x1A1a\x1A,a\x1A8\x92a\x1A\rV[a\x1A\x19V[\x82Ta\x19\xF7V[\x90UV[Pa\x1Aka\x1APa\x1AKa(QV[a\x11eV[a\x1A[\x81`\x01a\x19\xD7V[a\x1Ae`\x02a\r\x93V[\x90a\r\xC3V[a\x1A}a\x1Aw_a\r\xE8V[\x91a\x020V[\x14a\x1A\x89\x81`\x02a\x1A\x1CV[\x90V[a\x1A\x9Ca\x1A\x97a\x19gV[a\x19kV[\x90V[\x90a\x1A\xF1\x92\x91\x7F\xDE\xBF\xDF\xD5\xA5\n\xD1\x17\xC1\x08\x98\xD6\x8B\\\xCF\x08\x93\xC6\xB4\rOD?\x90..vF`\x1B\xDE\xAFT`\x01\x01\x7F\xDE\xBF\xDF\xD5\xA5\n\xD1\x17\xC1\x08\x98\xD6\x8B\\\xCF\x08\x93\xC6\xB4\rOD?\x90..vF`\x1B\xDE\xAFUa\x1E\xFAV[\x90V[\x90V[a\x1B\x0Ba\x1B\x06a\x1B\x10\x92a\x1A\xF4V[a\x01\x9EV[a\x020V[\x90V[` \x91\x81R\x01\x90V[_\x7FMust send exactly 0.1 ETH\0\0\0\0\0\0\0\x91\x01RV[a\x1BP`\x19` \x92a\x1B\x13V[a\x1BY\x81a\x1B\x1CV[\x01\x90V[a\x1Br\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x1BCV[\x90V[\x15a\x1B|WV[a\x1B\x84a\x012V[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x1B\x9A`\x04\x82\x01a\x1B]V[\x03\x90\xFD[`\x08\x1C\x90V[a\x1B\xB0a\x1B\xB5\x91a\x1B\x9EV[a\x05\xECV[\x90V[a\x1B\xC2\x90Ta\x1B\xA4V[\x90V[_\x91\x03\x12a\x1B\xCFWV[a\x01<V[P\x90V[_\x7FInvalid opcode offset\0\0\0\0\0\0\0\0\0\0\0\x91\x01RV[a\x1C\x0C`\x15` \x92a\x1B\x13V[a\x1C\x15\x81a\x1B\xD8V[\x01\x90V[a\x1C.\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x1B\xFFV[\x90V[\x15a\x1C8WV[a\x1C@a\x012V[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x1CV`\x04\x82\x01a\x1C\x19V[\x03\x90\xFD[\x91\x90\x81\x10\x15a\x1CjW`\x01\x02\x01\x90V[a\x0B\x10V[`\xFF`\xF8\x1B\x16\x90V[`\xF8\x1C\x90V[a\x1C\x92a\x1C\x8Da\x1C\x97\x92a\x0E\x04V[a\x01\x9EV[a\x0E\x04V[\x90V[a\x1C\xA6a\x1C\xAB\x91a\x1CxV[a\x1C~V[\x90V[a\x1C\xB7\x90a\x020V[_\x19\x81\x14a\x1C\xC5W`\x01\x01\x90V[a\x0F\x98V[` \x7Fe 0)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x7FUnsupported operation (op must b_\x82\x01R\x01RV[a\x1D$`$`@\x92a\x1B\x13V[a\x1D-\x81a\x1C\xCAV[\x01\x90V[a\x1DF\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x1D\x17V[\x90V[\x15a\x1DPWV[a\x1DXa\x012V[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x1Dn`\x04\x82\x01a\x1D1V[\x03\x90\xFD[\x90V[a\x1D\x89a\x1D\x84a\x1D\x8E\x92a\x1DrV[a\x01\x9EV[a\x020V[\x90V[_\x7FMissing slot data\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x01RV[a\x1D\xC5`\x11` \x92a\x1B\x13V[a\x1D\xCE\x81a\x1D\x91V[\x01\x90V[a\x1D\xE7\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x1D\xB8V[\x90V[\x15a\x1D\xF1WV[a\x1D\xF9a\x012V[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x1E\x0F`\x04\x82\x01a\x1D\xD2V[\x03\x90\xFD[_\x7FMissing value data\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x01RV[a\x1EG`\x12` \x92a\x1B\x13V[a\x1EP\x81a\x1E\x13V[\x01\x90V[a\x1Ei\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra\x1E:V[\x90V[\x15a\x1EsWV[a\x1E{a\x012V[bF\x1B\xCD`\xE5\x1B\x81R\x80a\x1E\x91`\x04\x82\x01a\x1ETV[\x03\x90\xFD[a\x1E\xA1a\x1E\xA6\x91a\x14\x14V[a\r\x1AV[\x90V[a\x1E\xB3\x90Ta\x1E\x95V[\x90V[a\x1E\xC2a\x1E\xC7\x91a\x14\x14V[a\x0B\xEAV[\x90V[a\x1E\xD4\x90Ta\x1E\xB6V[\x90V[\x91` a\x1E\xF8\x92\x94\x93a\x1E\xF1`@\x82\x01\x96_\x83\x01\x90a\x07\x16V[\x01\x90a\n\"V[V[\x92\x91\x92Pa\x1F\"4a\x1F\x1Ca\x1F\x16g\x01cEx]\x8A\0\0a\x1A\xF7V[\x91a\x020V[\x14a\x1BuV[a\x1F4a\x1F/`\x02a\x1B\xB8V[a\x06-V[c\xD0\xE3\r\xB04\x91\x90\x81;\x15a!WW_\x91a\x1F[\x91a\x1FQa\x012V[\x94\x85\x93\x84\x92a\x14>V[\x82R\x81a\x1Fj`\x04\x82\x01a\x0C\xE2V[\x03\x92Z\xF1\x80\x15a!RWa!&W[Pa\x1F\x83_a\r\xE8V[\x91[\x82a\x1F\xA2a\x1F\x9Ca\x1F\x97\x85\x88\x90a\x1B\xD4V[a\x020V[\x91a\x020V[\x10\x15a \xE1Wa\x1F\xE2a\x1F\xBF\x84a\x1F\xB9`\x01a\x0E)V[\x90a\x11@V[a\x1F\xDBa\x1F\xD5a\x1F\xD0\x86\x89\x90a\x1B\xD4V[a\x020V[\x91a\x020V[\x11\x15a\x1C1V[a $a \x0Ba \x05a \0a\x1F\xFA\x86\x89\x89\x91a\x1CZV[5a\x1CoV[a\x1C\x9AV[\x94a\x1C\xAEV[\x93a \x1Ea \x18_a\x0E\nV[\x91a\x0E\x04V[\x14a\x1DIV[a ^a ;\x84a 5` a\x1DuV[\x90a\x11@V[a Wa Qa L\x86\x89\x90a\x1B\xD4V[a\x020V[\x91a\x020V[\x11\x15a\x1D\xEAV[a fa\x0F\x88V[Pa \x7F\x83\x83\x015\x93a y` a\x1DuV[\x90a\x11@V[a \xB9a \x96\x82a \x90` a\x1DuV[\x90a\x11@V[a \xB2a \xACa \xA7\x87\x8A\x90a\x1B\xD4V[a\x020V[\x91a\x020V[\x11\x15a\x1ElV[a \xC1a\x0F\x88V[Pa \xDA\x81\x84\x015\x91a \xD4` a\x1DuV[\x90a\x11@V[\x93Ua\x1F\x85V[\x92PPPa \xEF`\x01a\x1E\xA9V[a!#a \xFC`\x02a\x1E\xCAV[\x91a!\x14a!\x08a\x012V[\x93\x84\x92` \x84\x01a\x1E\xD7V[` \x82\x01\x81\x03\x82R\x03\x82a\x03\xC0V[\x90V[a!E\x90_=\x81\x11a!KW[a!=\x81\x83a\x03\xC0V[\x81\x01\x90a\x1B\xC5V[_a\x1FyV[P=a!3V[a\x14\x04V[a\x14:V[\x90a!n\x91a!ia\r\x8BV[a\x1A\x9FV[\x90V[` \x91\x81R\x01\x90V[a!\x83\x90a\x06\xB3V[\x90RV[\x90a!\x94\x81` \x93a!zV[\x01\x90V[a!\xA4a!\xA9\x91a\x14\x14V[a\x0BYV[\x90V[a!\xB6\x90Ta!\x98V[\x90V[`\x01\x01\x90V[\x90a!\xDCa!\xD6a!\xCF\x84a\x0B$V[\x80\x93a!qV[\x92a\x0B(V[\x90_[\x81\x81\x10a!\xECWPPP\x90V[\x90\x91\x92a\"\x0Ca\"\x06`\x01\x92a\"\x01\x87a!\xACV[a!\x87V[\x94a!\xB9V[\x91\x01\x91\x90\x91a!\xDFV[a\"+\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra!\xBFV[\x90V[a\"6a\r\x8BV[Pa\"Ra\"a_a\"Fa\x012V[\x92\x83\x91` \x83\x01a\"\x16V[` \x82\x01\x81\x03\x82R\x03\x82a\x03\xC0V[\x90V[\x90a\"\xBD\x99\x98\x97\x96\x95\x94\x93\x92\x91\x7F\xDE\xBF\xDF\xD5\xA5\n\xD1\x17\xC1\x08\x98\xD6\x8B\\\xCF\x08\x93\xC6\xB4\rOD?\x90..vF`\x1B\xDE\xAFT`\x01\x01\x7F\xDE\xBF\xDF\xD5\xA5\n\xD1\x17\xC1\x08\x98\xD6\x8B\\\xCF\x08\x93\xC6\xB4\rOD?\x90..vF`\x1B\xDE\xAFUa$iV[\x90V[\x15a\"\xC7WV[_c9\xBBpQ`\xE1\x1B\x81R\x80a\"\xDF`\x04\x82\x01a\x0C\xE2V[\x03\x90\xFD[_\x7FInvalid signature\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x01RV[a#\x17`\x11` \x92a\x1B\x13V[a# \x81a\"\xE3V[\x01\x90V[a#9\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra#\nV[\x90V[\x15a#CWV[a#Ka\x012V[bF\x1B\xCD`\xE5\x1B\x81R\x80a#a`\x04\x82\x01a#$V[\x03\x90\xFD[_\x7FBLS pairing check failed\0\0\0\0\0\0\0\0\x91\x01RV[a#\x99`\x18` \x92a\x1B\x13V[a#\xA2\x81a#eV[\x01\x90V[a#\xBB\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra#\x8CV[\x90V[\x15a#\xC5WV[a#\xCDa\x012V[bF\x1B\xCD`\xE5\x1B\x81R\x80a#\xE3`\x04\x82\x01a#\xA6V[\x03\x90\xFD[_\x7FInvalid BLS signature\0\0\0\0\0\0\0\0\0\0\0\x91\x01RV[a$\x1B`\x15` \x92a\x1B\x13V[a$$\x81a#\xE7V[\x01\x90V[a$=\x90` \x81\x01\x90_\x81\x83\x03\x91\x01Ra$\x0EV[\x90V[\x15a$GWV[a$Oa\x012V[bF\x1B\xCD`\xE5\x1B\x81R\x80a$e`\x04\x82\x01a$(V[\x03\x90\xFD[\x99\x98\x90\x95\x99\x97\x91\x94\x97\x96\x92\x96Pa$\xACa$\x8D\x84a$\x87`\x01a\x0E)V[\x90a\x11@V[a$\xA6a$\xA0a$\x9Ba(QV[a\x020V[\x91a\x020V[\x14a\"\xC0V[a$\xD04a$\xCAa$\xC4g\x01cEx]\x8A\0\0a\x1A\xF7V[\x91a\x020V[\x14a\x1BuV[a$\xE2a$\xDD`\x02a\x1B\xB8V[a\x06-V[\x91c\xD0\xE3\r\xB04\x93\x90\x81;\x15a(0W_\x91a%\n\x91a%\0a\x012V[\x96\x87\x93\x84\x92a\x14>V[\x82R\x81a%\x19`\x04\x82\x01a\x0C\xE2V[\x03\x92Z\xF1\x80\x15a(+W` \x94a%\\a%{\x94\x8E_\x97a%j\x95a'\xFFW[P\x8Da%Ca\x05\xA1V[\x94\x96\x92\x90\x91\x92a%Qa\x012V[\x97\x88\x96\x8C\x88\x01a\x13\xB2V[\x86\x82\x01\x81\x03\x82R\x03\x82a\x03\xC0V[a%ra\x012V[\x91\x82\x91\x82a\x13\xF8V[\x03\x90`\x02Z\xFA\x15a'\xFAWa%\xE3`@\x93a%\xB1a%\x99_Qa\x13\xF3V[a%\xABa%\xA5\x84a\x07mV[\x91a\x07mV[\x14a#<V[a%\xEEa%\xC6a%\xC1`\x03a\x14-V[a\x01\xC9V[\x93c\x17\x1F\x1D[\x92\x95\x97a%\xD7a\x012V[\x98\x89\x97\x88\x96\x87\x96a\x14>V[\x86R`\x04\x86\x01a\x15gV[\x03\x91Z\xFA\x90\x81\x15a'\xF5Wa&\x16\x91_\x80\x92\x90\x91a'\xC2W[Pa&\x11\x90a#\xBEV[a$@V[a&\x1F_a\r\xE8V[\x91[\x82a&>a&8a&3\x85\x88\x90a\x1B\xD4V[a\x020V[\x91a\x020V[\x10\x15a'}Wa&~a&[\x84a&U`\x01a\x0E)V[\x90a\x11@V[a&wa&qa&l\x86\x89\x90a\x1B\xD4V[a\x020V[\x91a\x020V[\x11\x15a\x1C1V[a&\xC0a&\xA7a&\xA1a&\x9Ca&\x96\x86\x89\x89\x91a\x1CZV[5a\x1CoV[a\x1C\x9AV[\x94a\x1C\xAEV[\x93a&\xBAa&\xB4_a\x0E\nV[\x91a\x0E\x04V[\x14a\x1DIV[a&\xFAa&\xD7\x84a&\xD1` a\x1DuV[\x90a\x11@V[a&\xF3a&\xEDa&\xE8\x86\x89\x90a\x1B\xD4V[a\x020V[\x91a\x020V[\x11\x15a\x1D\xEAV[a'\x02a\x0F\x88V[Pa'\x1B\x83\x83\x015\x93a'\x15` a\x1DuV[\x90a\x11@V[a'Ua'2\x82a',` a\x1DuV[\x90a\x11@V[a'Na'Ha'C\x87\x8A\x90a\x1B\xD4V[a\x020V[\x91a\x020V[\x11\x15a\x1ElV[a']a\x0F\x88V[Pa'v\x81\x84\x015\x91a'p` a\x1DuV[\x90a\x11@V[\x93Ua&!V[\x92PPPa'\x8B`\x01a\x1E\xA9V[a'\xBFa'\x98`\x02a\x1E\xCAV[\x91a'\xB0a'\xA4a\x012V[\x93\x84\x92` \x84\x01a\x1E\xD7V[` \x82\x01\x81\x03\x82R\x03\x82a\x03\xC0V[\x90V[a&\x11\x92Pa'\xE8\x91P`@=\x81\x11a'\xEEW[a'\xE0\x81\x83a\x03\xC0V[\x81\x01\x90a\x14gV[\x91a&\x07V[P=a'\xD6V[a\x14\x04V[a\x14\x04V[a(\x1E\x90\x89=\x81\x11a($W[a(\x16\x81\x83a\x03\xC0V[\x81\x01\x90a\x1B\xC5V[_a%9V[P=a(\x0CV[a\x14\x04V[a\x14:V[\x90a(N\x98\x97\x96\x95\x94\x93\x92\x91a(Ia\r\x8BV[a\"dV[\x90V[a(Ya\x0F\x88V[P\x7F\xDE\xBF\xDF\xD5\xA5\n\xD1\x17\xC1\x08\x98\xD6\x8B\\\xCF\x08\x93\xC6\xB4\rOD?\x90..vF`\x1B\xDE\xAFT\x90V[a(\xCF\x90\x7F\xDE\xBF\xDF\xD5\xA5\n\xD1\x17\xC1\x08\x98\xD6\x8B\\\xCF\x08\x93\xC6\xB4\rOD?\x90..vF`\x1B\xDE\xAFT`\x01\x01\x7F\xDE\xBF\xDF\xD5\xA5\n\xD1\x17\xC1\x08\x98\xD6\x8B\\\xCF\x08\x93\xC6\xB4\rOD?\x90..vF`\x1B\xDE\xAFUa+zV[V[\x90V[_R` _ \x90V[T\x90V[a(\xEA\x81a(\xDDV[\x82\x10\x15a)\x04Wa(\xFC`\x01\x91a(\xD4V[\x91\x02\x01\x90_\x90V[a\x0B\x10V[\x1B\x90V[\x91\x90`\x08a)-\x91\x02\x91a)'`\x01\x80`\xA0\x1B\x03\x84a)\tV[\x92a)\tV[\x91\x81\x19\x16\x91\x16\x17\x90V[a)@\x90a\x01\xBDV[\x90V[\x90V[\x91\x90a)\\a)Wa)d\x93a)7V[a)CV[\x90\x83Ta)\rV[\x90UV[\x90\x81T\x91h\x01\0\0\0\0\0\0\0\0\x83\x10\x15a)\x98W\x82a)\x90\x91`\x01a)\x96\x95\x01\x81Ua(\xE1V[\x90a)FV[V[a\x03\xACV[`\x1F` \x91\x01\x04\x90V[\x91\x90`\x08a)\xC2\x91\x02\x91a)\xBC_\x19\x84a)\tV[\x92a)\tV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x91\x90a)\xE2a)\xDDa)\xEA\x93a\x02tV[a\x19\xD4V[\x90\x83Ta)\xA7V[\x90UV[a*\0\x91a)\xFAa\x0F\x88V[\x91a)\xCCV[V[[\x81\x81\x10a*\x0EWPPV[\x80a*\x1B_`\x01\x93a)\xEEV[\x01a*\x03V[\x91\x90`\x1F\x81\x11a*1W[PPPV[a*=a*b\x93a\x03\0V[\x90` a*I\x84a)\x9DV[\x83\x01\x93\x10a*jW[a*[\x90a)\x9DV[\x01\x90a*\x02V[_\x80\x80a*,V[\x91Pa*[\x81\x92\x90Pa*RV[\x90a*\x88\x90_\x19\x90`\x08\x02a\x01OV[\x19\x16\x90V[\x81a*\x97\x91a*xV[\x90`\x02\x02\x17\x90V[\x90a*\xA9\x81a\x04<V[\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a+iWa*\xCD\x82a*\xC7\x85Ta\x02\xCDV[\x85a*!V[` \x90`\x1F\x83\x11`\x01\x14a+\x01W\x91\x80\x91a*\xF0\x93_\x92a*\xF5W[PPa*\x8DV[\x90U[V[\x90\x91P\x01Q_\x80a*\xE9V[`\x1F\x19\x83\x16\x91a+\x10\x85a\x03\0V[\x92_[\x81\x81\x10a+QWP\x91`\x02\x93\x91\x85`\x01\x96\x94\x10a+7W[PPP\x02\x01\x90Ua*\xF3V[a+G\x91\x01Q`\x1F\x84\x16\x90a*xV[\x90U_\x80\x80a++V[\x91\x93` `\x01\x81\x92\x87\x87\x01Q\x81U\x01\x95\x01\x92\x01a+\x13V[a\x03\xACV[\x90a+x\x91a*\x9FV[V[a+\x8C\x90a+\x87_a(\xD1V[a)hV[a+\xD1a+\xAAa+\xB9_a+\x9Ea\x012V[\x92\x83\x91` \x83\x01a\"\x16V[` \x82\x01\x81\x03\x82R\x03\x82a\x03\xC0V[a+\xCC`\x04a+\xC6a(QV[\x90a\x02\x90V[a+nV[V[a+\xDC\x90a(\x7FV[V\xFE\xA2dipfsX\"\x12 \xBA\xA6\xF9\xF1(n\x16$<j\x95z\xD1\xC7\xD3\x18\xA41\xB9\xDC\xEFmS\x82\xDD\xA6M%X\xFF\xFB\xBAdsolcC\0\x08\x1C\x003",
    );
    /**Custom error with signature `InvalidTransitionIndex()` and selector `0x7376e0a2`.
```solidity
error InvalidTransitionIndex();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidTransitionIndex {}
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
        impl ::core::convert::From<InvalidTransitionIndex> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidTransitionIndex) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidTransitionIndex {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidTransitionIndex {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidTransitionIndex()";
            const SELECTOR: [u8; 4] = [115u8, 118u8, 224u8, 162u8];
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
    /**Constructor`.
```solidity
constructor(address _paymentContract);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _paymentContract: alloy::sol_types::private::Address,
    }
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value._paymentContract,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _paymentContract: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
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
                        &self._paymentContract,
                    ),
                )
            }
        }
    };
    /**Function with signature `BLS_SIG_CHECKER()` and selector `0xa352d707`.
```solidity
function BLS_SIG_CHECKER() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BLS_SIG_CHECKERCall {}
    ///Container type for the return parameters of the [`BLS_SIG_CHECKER()`](BLS_SIG_CHECKERCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BLS_SIG_CHECKERReturn {
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
            impl ::core::convert::From<BLS_SIG_CHECKERCall> for UnderlyingRustTuple<'_> {
                fn from(value: BLS_SIG_CHECKERCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for BLS_SIG_CHECKERCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
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
            impl ::core::convert::From<BLS_SIG_CHECKERReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: BLS_SIG_CHECKERReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for BLS_SIG_CHECKERReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for BLS_SIG_CHECKERCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = BLS_SIG_CHECKERReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BLS_SIG_CHECKER()";
            const SELECTOR: [u8; 4] = [163u8, 82u8, 215u8, 7u8];
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
    /**Function with signature `addVoter(address)` and selector `0xf4ab9adf`.
```solidity
function addVoter(address _voter) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addVoterCall {
        #[allow(missing_docs)]
        pub _voter: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`addVoter(address)`](addVoterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addVoterReturn {}
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
            impl ::core::convert::From<addVoterCall> for UnderlyingRustTuple<'_> {
                fn from(value: addVoterCall) -> Self {
                    (value._voter,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addVoterCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _voter: tuple.0 }
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
            impl ::core::convert::From<addVoterReturn> for UnderlyingRustTuple<'_> {
                fn from(value: addVoterReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addVoterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addVoterCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addVoterReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addVoter(address)";
            const SELECTOR: [u8; 4] = [244u8, 171u8, 154u8, 223u8];
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
                        &self._voter,
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
    /**Function with signature `blsSignatureChecker()` and selector `0x1c178e9c`.
```solidity
function blsSignatureChecker() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct blsSignatureCheckerCall {}
    ///Container type for the return parameters of the [`blsSignatureChecker()`](blsSignatureCheckerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct blsSignatureCheckerReturn {
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
            impl ::core::convert::From<blsSignatureCheckerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: blsSignatureCheckerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for blsSignatureCheckerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
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
            impl ::core::convert::From<blsSignatureCheckerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: blsSignatureCheckerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for blsSignatureCheckerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for blsSignatureCheckerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = blsSignatureCheckerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "blsSignatureChecker()";
            const SELECTOR: [u8; 4] = [28u8, 23u8, 142u8, 156u8];
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
    /**Function with signature `currentTotalVotingPower()` and selector `0xfd8eac49`.
```solidity
function currentTotalVotingPower() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct currentTotalVotingPowerCall {}
    ///Container type for the return parameters of the [`currentTotalVotingPower()`](currentTotalVotingPowerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct currentTotalVotingPowerReturn {
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
            impl ::core::convert::From<currentTotalVotingPowerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: currentTotalVotingPowerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for currentTotalVotingPowerCall {
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
            impl ::core::convert::From<currentTotalVotingPowerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: currentTotalVotingPowerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for currentTotalVotingPowerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for currentTotalVotingPowerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = currentTotalVotingPowerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "currentTotalVotingPower()";
            const SELECTOR: [u8; 4] = [253u8, 142u8, 172u8, 73u8];
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
    /**Function with signature `executeVote()` and selector `0xbfa06830`.
```solidity
function executeVote() external returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeVoteCall {}
    ///Container type for the return parameters of the [`executeVote()`](executeVoteCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeVoteReturn {
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
            impl ::core::convert::From<executeVoteCall> for UnderlyingRustTuple<'_> {
                fn from(value: executeVoteCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeVoteCall {
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
            impl ::core::convert::From<executeVoteReturn> for UnderlyingRustTuple<'_> {
                fn from(value: executeVoteReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeVoteReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for executeVoteCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = executeVoteReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "executeVote()";
            const SELECTOR: [u8; 4] = [191u8, 160u8, 104u8, 48u8];
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
    /**Function with signature `getCurrentTotalVotingPower(uint256)` and selector `0xa4f88154`.
```solidity
function getCurrentTotalVotingPower(uint256 transitionIndex) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentTotalVotingPowerCall {
        #[allow(missing_docs)]
        pub transitionIndex: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getCurrentTotalVotingPower(uint256)`](getCurrentTotalVotingPowerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentTotalVotingPowerReturn {
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
            impl ::core::convert::From<getCurrentTotalVotingPowerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentTotalVotingPowerCall) -> Self {
                    (value.transitionIndex,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentTotalVotingPowerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { transitionIndex: tuple.0 }
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
            impl ::core::convert::From<getCurrentTotalVotingPowerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentTotalVotingPowerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentTotalVotingPowerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCurrentTotalVotingPowerCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCurrentTotalVotingPowerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCurrentTotalVotingPower(uint256)";
            const SELECTOR: [u8; 4] = [164u8, 248u8, 129u8, 84u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.transitionIndex),
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
    /**Function with signature `getCurrentVotersArray()` and selector `0xcf6ae603`.
```solidity
function getCurrentVotersArray() external view returns (bytes memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentVotersArrayCall {}
    ///Container type for the return parameters of the [`getCurrentVotersArray()`](getCurrentVotersArrayCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentVotersArrayReturn {
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
            impl ::core::convert::From<getCurrentVotersArrayCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentVotersArrayCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentVotersArrayCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<getCurrentVotersArrayReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentVotersArrayReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentVotersArrayReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCurrentVotersArrayCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCurrentVotersArrayReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCurrentVotersArray()";
            const SELECTOR: [u8; 4] = [207u8, 106u8, 230u8, 3u8];
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
    /**Function with signature `lastVotePassed()` and selector `0xea53bac5`.
```solidity
function lastVotePassed() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastVotePassedCall {}
    ///Container type for the return parameters of the [`lastVotePassed()`](lastVotePassedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastVotePassedReturn {
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
            impl ::core::convert::From<lastVotePassedCall> for UnderlyingRustTuple<'_> {
                fn from(value: lastVotePassedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lastVotePassedCall {
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
            impl ::core::convert::From<lastVotePassedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: lastVotePassedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for lastVotePassedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lastVotePassedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = lastVotePassedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "lastVotePassed()";
            const SELECTOR: [u8; 4] = [234u8, 83u8, 186u8, 197u8];
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
    /**Function with signature `namespace()` and selector `0x7c015a89`.
```solidity
function namespace() external view returns (bytes memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct namespaceCall {}
    ///Container type for the return parameters of the [`namespace()`](namespaceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct namespaceReturn {
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
            impl ::core::convert::From<namespaceCall> for UnderlyingRustTuple<'_> {
                fn from(value: namespaceCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for namespaceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<namespaceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: namespaceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for namespaceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for namespaceCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = namespaceReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "namespace()";
            const SELECTOR: [u8; 4] = [124u8, 1u8, 90u8, 137u8];
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
    /**Function with signature `operatorExecuteVote(uint256)` and selector `0x5ffd1f00`.
```solidity
function operatorExecuteVote(uint256 transitionIndex) external view returns (bytes memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorExecuteVoteCall {
        #[allow(missing_docs)]
        pub transitionIndex: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`operatorExecuteVote(uint256)`](operatorExecuteVoteCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorExecuteVoteReturn {
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
            impl ::core::convert::From<operatorExecuteVoteCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: operatorExecuteVoteCall) -> Self {
                    (value.transitionIndex,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorExecuteVoteCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { transitionIndex: tuple.0 }
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
            impl ::core::convert::From<operatorExecuteVoteReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: operatorExecuteVoteReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorExecuteVoteReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for operatorExecuteVoteCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = operatorExecuteVoteReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "operatorExecuteVote(uint256)";
            const SELECTOR: [u8; 4] = [95u8, 253u8, 31u8, 0u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.transitionIndex),
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
    /**Function with signature `paymentContract()` and selector `0x9c91dd56`.
```solidity
function paymentContract() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paymentContractCall {}
    ///Container type for the return parameters of the [`paymentContract()`](paymentContractCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paymentContractReturn {
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
            impl ::core::convert::From<paymentContractCall> for UnderlyingRustTuple<'_> {
                fn from(value: paymentContractCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for paymentContractCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
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
            impl ::core::convert::From<paymentContractReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: paymentContractReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for paymentContractReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for paymentContractCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = paymentContractReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "paymentContract()";
            const SELECTOR: [u8; 4] = [156u8, 145u8, 221u8, 86u8];
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
    /**Function with signature `slashExecVote(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256),bytes,uint256,address,bytes4)` and selector `0xaa2c58b2`.
```solidity
function slashExecVote(bytes32 msgHash, BN254.G1Point memory apk, BN254.G2Point memory apkG2, BN254.G1Point memory sigma, bytes memory storageUpdates, uint256 transitionIndex, address targetAddr, bytes4 targetFunction) external returns (bytes memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashExecVoteCall {
        #[allow(missing_docs)]
        pub msgHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub apk: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub apkG2: <BN254::G2Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub sigma: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub storageUpdates: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub transitionIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub targetAddr: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub targetFunction: alloy::sol_types::private::FixedBytes<4>,
    }
    ///Container type for the return parameters of the [`slashExecVote(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256),bytes,uint256,address,bytes4)`](slashExecVoteCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashExecVoteReturn {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                BN254::G1Point,
                BN254::G2Point,
                BN254::G1Point,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<4>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                <BN254::G1Point as alloy::sol_types::SolType>::RustType,
                <BN254::G2Point as alloy::sol_types::SolType>::RustType,
                <BN254::G1Point as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<4>,
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
            impl ::core::convert::From<slashExecVoteCall> for UnderlyingRustTuple<'_> {
                fn from(value: slashExecVoteCall) -> Self {
                    (
                        value.msgHash,
                        value.apk,
                        value.apkG2,
                        value.sigma,
                        value.storageUpdates,
                        value.transitionIndex,
                        value.targetAddr,
                        value.targetFunction,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for slashExecVoteCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        msgHash: tuple.0,
                        apk: tuple.1,
                        apkG2: tuple.2,
                        sigma: tuple.3,
                        storageUpdates: tuple.4,
                        transitionIndex: tuple.5,
                        targetAddr: tuple.6,
                        targetFunction: tuple.7,
                    }
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
            impl ::core::convert::From<slashExecVoteReturn> for UnderlyingRustTuple<'_> {
                fn from(value: slashExecVoteReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for slashExecVoteReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for slashExecVoteCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                BN254::G1Point,
                BN254::G2Point,
                BN254::G1Point,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<4>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = slashExecVoteReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "slashExecVote(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256),bytes,uint256,address,bytes4)";
            const SELECTOR: [u8; 4] = [170u8, 44u8, 88u8, 178u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.msgHash),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.apk),
                    <BN254::G2Point as alloy_sol_types::SolType>::tokenize(&self.apkG2),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.sigma),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.storageUpdates,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.transitionIndex),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.targetAddr,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::SolType>::tokenize(&self.targetFunction),
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
    /**Function with signature `stateTransitionCount()` and selector `0xf4833e20`.
```solidity
function stateTransitionCount() external view returns (uint256 count);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stateTransitionCountCall {}
    ///Container type for the return parameters of the [`stateTransitionCount()`](stateTransitionCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stateTransitionCountReturn {
        #[allow(missing_docs)]
        pub count: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<stateTransitionCountCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: stateTransitionCountCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for stateTransitionCountCall {
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
            impl ::core::convert::From<stateTransitionCountReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: stateTransitionCountReturn) -> Self {
                    (value.count,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for stateTransitionCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { count: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for stateTransitionCountCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = stateTransitionCountReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "stateTransitionCount()";
            const SELECTOR: [u8; 4] = [244u8, 131u8, 62u8, 32u8];
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
    /**Function with signature `voters(uint256)` and selector `0xda58c7d9`.
```solidity
function voters(uint256) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct votersCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`voters(uint256)`](votersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct votersReturn {
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
            impl ::core::convert::From<votersCall> for UnderlyingRustTuple<'_> {
                fn from(value: votersCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for votersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
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
            impl ::core::convert::From<votersReturn> for UnderlyingRustTuple<'_> {
                fn from(value: votersReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for votersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for votersCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = votersReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "voters(uint256)";
            const SELECTOR: [u8; 4] = [218u8, 88u8, 199u8, 217u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
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
    /**Function with signature `votersArrayStorage(uint256)` and selector `0x1ee07fb1`.
```solidity
function votersArrayStorage(uint256) external view returns (bytes memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct votersArrayStorageCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`votersArrayStorage(uint256)`](votersArrayStorageCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct votersArrayStorageReturn {
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
            impl ::core::convert::From<votersArrayStorageCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: votersArrayStorageCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for votersArrayStorageCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
            impl ::core::convert::From<votersArrayStorageReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: votersArrayStorageReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for votersArrayStorageReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for votersArrayStorageCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = votersArrayStorageReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "votersArrayStorage(uint256)";
            const SELECTOR: [u8; 4] = [30u8, 224u8, 127u8, 177u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
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
    /**Function with signature `writeExecuteVote(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256),bytes,uint256,address,bytes4)` and selector `0xf300b862`.
```solidity
function writeExecuteVote(bytes32 msgHash, BN254.G1Point memory apk, BN254.G2Point memory apkG2, BN254.G1Point memory sigma, bytes memory storageUpdates, uint256 transitionIndex, address targetAddr, bytes4 targetFunction) external payable returns (bytes memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct writeExecuteVoteCall {
        #[allow(missing_docs)]
        pub msgHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub apk: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub apkG2: <BN254::G2Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub sigma: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub storageUpdates: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub transitionIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub targetAddr: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub targetFunction: alloy::sol_types::private::FixedBytes<4>,
    }
    ///Container type for the return parameters of the [`writeExecuteVote(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256),bytes,uint256,address,bytes4)`](writeExecuteVoteCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct writeExecuteVoteReturn {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                BN254::G1Point,
                BN254::G2Point,
                BN254::G1Point,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<4>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                <BN254::G1Point as alloy::sol_types::SolType>::RustType,
                <BN254::G2Point as alloy::sol_types::SolType>::RustType,
                <BN254::G1Point as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<4>,
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
            impl ::core::convert::From<writeExecuteVoteCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: writeExecuteVoteCall) -> Self {
                    (
                        value.msgHash,
                        value.apk,
                        value.apkG2,
                        value.sigma,
                        value.storageUpdates,
                        value.transitionIndex,
                        value.targetAddr,
                        value.targetFunction,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for writeExecuteVoteCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        msgHash: tuple.0,
                        apk: tuple.1,
                        apkG2: tuple.2,
                        sigma: tuple.3,
                        storageUpdates: tuple.4,
                        transitionIndex: tuple.5,
                        targetAddr: tuple.6,
                        targetFunction: tuple.7,
                    }
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
            impl ::core::convert::From<writeExecuteVoteReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: writeExecuteVoteReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for writeExecuteVoteReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for writeExecuteVoteCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                BN254::G1Point,
                BN254::G2Point,
                BN254::G1Point,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<4>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = writeExecuteVoteReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "writeExecuteVote(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256),bytes,uint256,address,bytes4)";
            const SELECTOR: [u8; 4] = [243u8, 0u8, 184u8, 98u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.msgHash),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.apk),
                    <BN254::G2Point as alloy_sol_types::SolType>::tokenize(&self.apkG2),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.sigma),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.storageUpdates,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.transitionIndex),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.targetAddr,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::SolType>::tokenize(&self.targetFunction),
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
    /**Function with signature `writeExecuteVoteTest(bytes)` and selector `0xc7b37540`.
```solidity
function writeExecuteVoteTest(bytes memory storageUpdates) external payable returns (bytes memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct writeExecuteVoteTestCall {
        #[allow(missing_docs)]
        pub storageUpdates: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`writeExecuteVoteTest(bytes)`](writeExecuteVoteTestCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct writeExecuteVoteTestReturn {
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
            impl ::core::convert::From<writeExecuteVoteTestCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: writeExecuteVoteTestCall) -> Self {
                    (value.storageUpdates,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for writeExecuteVoteTestCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { storageUpdates: tuple.0 }
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
            impl ::core::convert::From<writeExecuteVoteTestReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: writeExecuteVoteTestReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for writeExecuteVoteTestReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for writeExecuteVoteTestCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = writeExecuteVoteTestReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "writeExecuteVoteTest(bytes)";
            const SELECTOR: [u8; 4] = [199u8, 179u8, 117u8, 64u8];
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
                        &self.storageUpdates,
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
    ///Container for all the [`VotingContract`](self) function calls.
    pub enum VotingContractCalls {
        #[allow(missing_docs)]
        BLS_SIG_CHECKER(BLS_SIG_CHECKERCall),
        #[allow(missing_docs)]
        addVoter(addVoterCall),
        #[allow(missing_docs)]
        blsSignatureChecker(blsSignatureCheckerCall),
        #[allow(missing_docs)]
        currentTotalVotingPower(currentTotalVotingPowerCall),
        #[allow(missing_docs)]
        executeVote(executeVoteCall),
        #[allow(missing_docs)]
        getCurrentTotalVotingPower(getCurrentTotalVotingPowerCall),
        #[allow(missing_docs)]
        getCurrentVotersArray(getCurrentVotersArrayCall),
        #[allow(missing_docs)]
        lastVotePassed(lastVotePassedCall),
        #[allow(missing_docs)]
        namespace(namespaceCall),
        #[allow(missing_docs)]
        operatorExecuteVote(operatorExecuteVoteCall),
        #[allow(missing_docs)]
        paymentContract(paymentContractCall),
        #[allow(missing_docs)]
        slashExecVote(slashExecVoteCall),
        #[allow(missing_docs)]
        stateTransitionCount(stateTransitionCountCall),
        #[allow(missing_docs)]
        voters(votersCall),
        #[allow(missing_docs)]
        votersArrayStorage(votersArrayStorageCall),
        #[allow(missing_docs)]
        writeExecuteVote(writeExecuteVoteCall),
        #[allow(missing_docs)]
        writeExecuteVoteTest(writeExecuteVoteTestCall),
    }
    #[automatically_derived]
    impl VotingContractCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [28u8, 23u8, 142u8, 156u8],
            [30u8, 224u8, 127u8, 177u8],
            [95u8, 253u8, 31u8, 0u8],
            [124u8, 1u8, 90u8, 137u8],
            [156u8, 145u8, 221u8, 86u8],
            [163u8, 82u8, 215u8, 7u8],
            [164u8, 248u8, 129u8, 84u8],
            [170u8, 44u8, 88u8, 178u8],
            [191u8, 160u8, 104u8, 48u8],
            [199u8, 179u8, 117u8, 64u8],
            [207u8, 106u8, 230u8, 3u8],
            [218u8, 88u8, 199u8, 217u8],
            [234u8, 83u8, 186u8, 197u8],
            [243u8, 0u8, 184u8, 98u8],
            [244u8, 131u8, 62u8, 32u8],
            [244u8, 171u8, 154u8, 223u8],
            [253u8, 142u8, 172u8, 73u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for VotingContractCalls {
        const NAME: &'static str = "VotingContractCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 17usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::BLS_SIG_CHECKER(_) => {
                    <BLS_SIG_CHECKERCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addVoter(_) => <addVoterCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::blsSignatureChecker(_) => {
                    <blsSignatureCheckerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::currentTotalVotingPower(_) => {
                    <currentTotalVotingPowerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::executeVote(_) => {
                    <executeVoteCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCurrentTotalVotingPower(_) => {
                    <getCurrentTotalVotingPowerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCurrentVotersArray(_) => {
                    <getCurrentVotersArrayCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::lastVotePassed(_) => {
                    <lastVotePassedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::namespace(_) => {
                    <namespaceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::operatorExecuteVote(_) => {
                    <operatorExecuteVoteCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::paymentContract(_) => {
                    <paymentContractCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::slashExecVote(_) => {
                    <slashExecVoteCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::stateTransitionCount(_) => {
                    <stateTransitionCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::voters(_) => <votersCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::votersArrayStorage(_) => {
                    <votersArrayStorageCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::writeExecuteVote(_) => {
                    <writeExecuteVoteCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::writeExecuteVoteTest(_) => {
                    <writeExecuteVoteTestCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<VotingContractCalls>] = &[
                {
                    fn blsSignatureChecker(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<VotingContractCalls> {
                        <blsSignatureCheckerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(VotingContractCalls::blsSignatureChecker)
                    }
                    blsSignatureChecker
                },
                {
                    fn votersArrayStorage(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<VotingContractCalls> {
                        <votersArrayStorageCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(VotingContractCalls::votersArrayStorage)
                    }
                    votersArrayStorage
                },
                {
                    fn operatorExecuteVote(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<VotingContractCalls> {
                        <operatorExecuteVoteCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(VotingContractCalls::operatorExecuteVote)
                    }
                    operatorExecuteVote
                },
                {
                    fn namespace(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<VotingContractCalls> {
                        <namespaceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(VotingContractCalls::namespace)
                    }
                    namespace
                },
                {
                    fn paymentContract(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<VotingContractCalls> {
                        <paymentContractCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(VotingContractCalls::paymentContract)
                    }
                    paymentContract
                },
                {
                    fn BLS_SIG_CHECKER(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<VotingContractCalls> {
                        <BLS_SIG_CHECKERCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(VotingContractCalls::BLS_SIG_CHECKER)
                    }
                    BLS_SIG_CHECKER
                },
                {
                    fn getCurrentTotalVotingPower(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<VotingContractCalls> {
                        <getCurrentTotalVotingPowerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(VotingContractCalls::getCurrentTotalVotingPower)
                    }
                    getCurrentTotalVotingPower
                },
                {
                    fn slashExecVote(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<VotingContractCalls> {
                        <slashExecVoteCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(VotingContractCalls::slashExecVote)
                    }
                    slashExecVote
                },
                {
                    fn executeVote(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<VotingContractCalls> {
                        <executeVoteCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(VotingContractCalls::executeVote)
                    }
                    executeVote
                },
                {
                    fn writeExecuteVoteTest(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<VotingContractCalls> {
                        <writeExecuteVoteTestCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(VotingContractCalls::writeExecuteVoteTest)
                    }
                    writeExecuteVoteTest
                },
                {
                    fn getCurrentVotersArray(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<VotingContractCalls> {
                        <getCurrentVotersArrayCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(VotingContractCalls::getCurrentVotersArray)
                    }
                    getCurrentVotersArray
                },
                {
                    fn voters(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<VotingContractCalls> {
                        <votersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(VotingContractCalls::voters)
                    }
                    voters
                },
                {
                    fn lastVotePassed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<VotingContractCalls> {
                        <lastVotePassedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(VotingContractCalls::lastVotePassed)
                    }
                    lastVotePassed
                },
                {
                    fn writeExecuteVote(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<VotingContractCalls> {
                        <writeExecuteVoteCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(VotingContractCalls::writeExecuteVote)
                    }
                    writeExecuteVote
                },
                {
                    fn stateTransitionCount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<VotingContractCalls> {
                        <stateTransitionCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(VotingContractCalls::stateTransitionCount)
                    }
                    stateTransitionCount
                },
                {
                    fn addVoter(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<VotingContractCalls> {
                        <addVoterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(VotingContractCalls::addVoter)
                    }
                    addVoter
                },
                {
                    fn currentTotalVotingPower(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<VotingContractCalls> {
                        <currentTotalVotingPowerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(VotingContractCalls::currentTotalVotingPower)
                    }
                    currentTotalVotingPower
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
                Self::BLS_SIG_CHECKER(inner) => {
                    <BLS_SIG_CHECKERCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::addVoter(inner) => {
                    <addVoterCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::blsSignatureChecker(inner) => {
                    <blsSignatureCheckerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::currentTotalVotingPower(inner) => {
                    <currentTotalVotingPowerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::executeVote(inner) => {
                    <executeVoteCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCurrentTotalVotingPower(inner) => {
                    <getCurrentTotalVotingPowerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCurrentVotersArray(inner) => {
                    <getCurrentVotersArrayCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::lastVotePassed(inner) => {
                    <lastVotePassedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::namespace(inner) => {
                    <namespaceCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::operatorExecuteVote(inner) => {
                    <operatorExecuteVoteCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::paymentContract(inner) => {
                    <paymentContractCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::slashExecVote(inner) => {
                    <slashExecVoteCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::stateTransitionCount(inner) => {
                    <stateTransitionCountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::voters(inner) => {
                    <votersCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::votersArrayStorage(inner) => {
                    <votersArrayStorageCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::writeExecuteVote(inner) => {
                    <writeExecuteVoteCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::writeExecuteVoteTest(inner) => {
                    <writeExecuteVoteTestCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::BLS_SIG_CHECKER(inner) => {
                    <BLS_SIG_CHECKERCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::addVoter(inner) => {
                    <addVoterCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::blsSignatureChecker(inner) => {
                    <blsSignatureCheckerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::currentTotalVotingPower(inner) => {
                    <currentTotalVotingPowerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::executeVote(inner) => {
                    <executeVoteCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCurrentTotalVotingPower(inner) => {
                    <getCurrentTotalVotingPowerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCurrentVotersArray(inner) => {
                    <getCurrentVotersArrayCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::lastVotePassed(inner) => {
                    <lastVotePassedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::namespace(inner) => {
                    <namespaceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::operatorExecuteVote(inner) => {
                    <operatorExecuteVoteCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::paymentContract(inner) => {
                    <paymentContractCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::slashExecVote(inner) => {
                    <slashExecVoteCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::stateTransitionCount(inner) => {
                    <stateTransitionCountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::voters(inner) => {
                    <votersCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::votersArrayStorage(inner) => {
                    <votersArrayStorageCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::writeExecuteVote(inner) => {
                    <writeExecuteVoteCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::writeExecuteVoteTest(inner) => {
                    <writeExecuteVoteTestCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`VotingContract`](self) custom errors.
    pub enum VotingContractErrors {
        #[allow(missing_docs)]
        InvalidTransitionIndex(InvalidTransitionIndex),
    }
    #[automatically_derived]
    impl VotingContractErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[[115u8, 118u8, 224u8, 162u8]];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for VotingContractErrors {
        const NAME: &'static str = "VotingContractErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 1usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::InvalidTransitionIndex(_) => {
                    <InvalidTransitionIndex as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<VotingContractErrors>] = &[
                {
                    fn InvalidTransitionIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<VotingContractErrors> {
                        <InvalidTransitionIndex as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(VotingContractErrors::InvalidTransitionIndex)
                    }
                    InvalidTransitionIndex
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
                Self::InvalidTransitionIndex(inner) => {
                    <InvalidTransitionIndex as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::InvalidTransitionIndex(inner) => {
                    <InvalidTransitionIndex as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`VotingContract`](self) contract instance.

See the [wrapper's documentation](`VotingContractInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> VotingContractInstance<T, P, N> {
        VotingContractInstance::<T, P, N>::new(address, provider)
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
        _paymentContract: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<VotingContractInstance<T, P, N>>,
    > {
        VotingContractInstance::<T, P, N>::deploy(provider, _paymentContract)
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
        _paymentContract: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        VotingContractInstance::<T, P, N>::deploy_builder(provider, _paymentContract)
    }
    /**A [`VotingContract`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`VotingContract`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct VotingContractInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for VotingContractInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("VotingContractInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > VotingContractInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`VotingContract`](self) contract instance.

See the [wrapper's documentation](`VotingContractInstance`) for more details.*/
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
            _paymentContract: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<VotingContractInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, _paymentContract);
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
            _paymentContract: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _paymentContract,
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
    impl<T, P: ::core::clone::Clone, N> VotingContractInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> VotingContractInstance<T, P, N> {
            VotingContractInstance {
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
    > VotingContractInstance<T, P, N> {
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
        ///Creates a new call builder for the [`BLS_SIG_CHECKER`] function.
        pub fn BLS_SIG_CHECKER(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, BLS_SIG_CHECKERCall, N> {
            self.call_builder(&BLS_SIG_CHECKERCall {})
        }
        ///Creates a new call builder for the [`addVoter`] function.
        pub fn addVoter(
            &self,
            _voter: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, addVoterCall, N> {
            self.call_builder(&addVoterCall { _voter })
        }
        ///Creates a new call builder for the [`blsSignatureChecker`] function.
        pub fn blsSignatureChecker(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, blsSignatureCheckerCall, N> {
            self.call_builder(&blsSignatureCheckerCall {})
        }
        ///Creates a new call builder for the [`currentTotalVotingPower`] function.
        pub fn currentTotalVotingPower(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, currentTotalVotingPowerCall, N> {
            self.call_builder(&currentTotalVotingPowerCall {})
        }
        ///Creates a new call builder for the [`executeVote`] function.
        pub fn executeVote(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, executeVoteCall, N> {
            self.call_builder(&executeVoteCall {})
        }
        ///Creates a new call builder for the [`getCurrentTotalVotingPower`] function.
        pub fn getCurrentTotalVotingPower(
            &self,
            transitionIndex: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getCurrentTotalVotingPowerCall, N> {
            self.call_builder(
                &getCurrentTotalVotingPowerCall {
                    transitionIndex,
                },
            )
        }
        ///Creates a new call builder for the [`getCurrentVotersArray`] function.
        pub fn getCurrentVotersArray(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getCurrentVotersArrayCall, N> {
            self.call_builder(&getCurrentVotersArrayCall {})
        }
        ///Creates a new call builder for the [`lastVotePassed`] function.
        pub fn lastVotePassed(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, lastVotePassedCall, N> {
            self.call_builder(&lastVotePassedCall {})
        }
        ///Creates a new call builder for the [`namespace`] function.
        pub fn namespace(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, namespaceCall, N> {
            self.call_builder(&namespaceCall {})
        }
        ///Creates a new call builder for the [`operatorExecuteVote`] function.
        pub fn operatorExecuteVote(
            &self,
            transitionIndex: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, operatorExecuteVoteCall, N> {
            self.call_builder(
                &operatorExecuteVoteCall {
                    transitionIndex,
                },
            )
        }
        ///Creates a new call builder for the [`paymentContract`] function.
        pub fn paymentContract(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, paymentContractCall, N> {
            self.call_builder(&paymentContractCall {})
        }
        ///Creates a new call builder for the [`slashExecVote`] function.
        pub fn slashExecVote(
            &self,
            msgHash: alloy::sol_types::private::FixedBytes<32>,
            apk: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            apkG2: <BN254::G2Point as alloy::sol_types::SolType>::RustType,
            sigma: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            storageUpdates: alloy::sol_types::private::Bytes,
            transitionIndex: alloy::sol_types::private::primitives::aliases::U256,
            targetAddr: alloy::sol_types::private::Address,
            targetFunction: alloy::sol_types::private::FixedBytes<4>,
        ) -> alloy_contract::SolCallBuilder<T, &P, slashExecVoteCall, N> {
            self.call_builder(
                &slashExecVoteCall {
                    msgHash,
                    apk,
                    apkG2,
                    sigma,
                    storageUpdates,
                    transitionIndex,
                    targetAddr,
                    targetFunction,
                },
            )
        }
        ///Creates a new call builder for the [`stateTransitionCount`] function.
        pub fn stateTransitionCount(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, stateTransitionCountCall, N> {
            self.call_builder(&stateTransitionCountCall {})
        }
        ///Creates a new call builder for the [`voters`] function.
        pub fn voters(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, votersCall, N> {
            self.call_builder(&votersCall { _0 })
        }
        ///Creates a new call builder for the [`votersArrayStorage`] function.
        pub fn votersArrayStorage(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, votersArrayStorageCall, N> {
            self.call_builder(&votersArrayStorageCall { _0 })
        }
        ///Creates a new call builder for the [`writeExecuteVote`] function.
        pub fn writeExecuteVote(
            &self,
            msgHash: alloy::sol_types::private::FixedBytes<32>,
            apk: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            apkG2: <BN254::G2Point as alloy::sol_types::SolType>::RustType,
            sigma: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            storageUpdates: alloy::sol_types::private::Bytes,
            transitionIndex: alloy::sol_types::private::primitives::aliases::U256,
            targetAddr: alloy::sol_types::private::Address,
            targetFunction: alloy::sol_types::private::FixedBytes<4>,
        ) -> alloy_contract::SolCallBuilder<T, &P, writeExecuteVoteCall, N> {
            self.call_builder(
                &writeExecuteVoteCall {
                    msgHash,
                    apk,
                    apkG2,
                    sigma,
                    storageUpdates,
                    transitionIndex,
                    targetAddr,
                    targetFunction,
                },
            )
        }
        ///Creates a new call builder for the [`writeExecuteVoteTest`] function.
        pub fn writeExecuteVoteTest(
            &self,
            storageUpdates: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, writeExecuteVoteTestCall, N> {
            self.call_builder(
                &writeExecuteVoteTestCall {
                    storageUpdates,
                },
            )
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > VotingContractInstance<T, P, N> {
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
