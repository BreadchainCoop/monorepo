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
    clippy::style
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
        pub X: alloy::sol_types::private::primitives::aliases::U256,
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
        pub X: [alloy::sol_types::private::primitives::aliases::U256; 2usize],
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
    constructor(address _paymentContract);

    function BLS_SIG_CHECKER() external view returns (address);
    function addVoter(address _voter) external;
    function blsSignatureChecker() external view returns (address);
    function currentTotalVotingPower() external view returns (uint256);
    function executeVote() external returns (bool);
    function getCurrentTotalVotingPower(uint256 _blockNumber) external view returns (uint256);
    function getCurrentVotersArray() external view returns (bytes memory);
    function lastVotePassed() external view returns (bool);
    function namespace() external view returns (bytes memory);
    function operatorExecuteVote(uint256 blockNumber) external view returns (bytes memory);
    function paymentContract() external view returns (address);
    function slashExecVote(bytes32 msgHash, BN254.G1Point memory apk, BN254.G2Point memory apkG2, BN254.G1Point memory sigma, bytes memory storageUpdates, uint256 blockNumber, address targetAddr, bytes4 targetFunction) external view returns (bytes memory);
    function voters(uint256) external view returns (address);
    function votersArrayStorage(uint256) external view returns (bytes memory);
    function writeExecuteVote(bytes32 msgHash, BN254.G1Point memory apk, BN254.G2Point memory apkG2, BN254.G1Point memory sigma, bytes memory storageUpdates, uint256 blockNumber, address targetAddr, bytes4 targetFunction) external payable returns (bytes memory);
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
        "name": "_blockNumber",
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
        "name": "blockNumber",
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
        "name": "blockNumber",
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
        "name": "blockNumber",
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
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod VotingContract {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6080604052348015600e575f5ffd5b506040516117af3803806117af833981016040819052602b91607c565b600380546001600160a01b03191673ca249215e082e17c12bb3c4881839a3f883e5c6b179055600280546001600160a01b039290921661010002610100600160a81b031990921691909117905560a7565b5f60208284031215608b575f5ffd5b81516001600160a01b038116811460a0575f5ffd5b9392505050565b6116fb806100b45f395ff3fe6080604052600436106100ef575f3560e01c8063bfa0683011610087578063ea53bac511610057578063ea53bac5146102be578063f300b862146102d7578063f4ab9adf146102ea578063fd8eac491461030b575f5ffd5b8063bfa0683014610254578063c7b3754014610278578063cf6ae6031461028b578063da58c7d91461029f575f5ffd5b80639c91dd56116100c25780639c91dd56146101bd578063a352d707146101e1578063a4f8815414610208578063aa2c58b214610235575f5ffd5b80631c178e9c146100f35780631ee07fb11461012f5780635ffd1f001461015b5780637c015a891461017a575b5f5ffd5b3480156100fe575f5ffd5b50600354610112906001600160a01b031681565b6040516001600160a01b0390911681526020015b60405180910390f35b34801561013a575f5ffd5b5061014e610149366004610f17565b610320565b6040516101269190610f2e565b348015610166575f5ffd5b5061014e610175366004610f17565b6103b7565b348015610185575f5ffd5b5061014e604051806040016040528060188152602001775f434f4d4d4f4e574152455f4147475245474154494f4e5f60401b81525081565b3480156101c8575f5ffd5b506002546101129061010090046001600160a01b031681565b3480156101ec575f5ffd5b5061011273ca249215e082e17c12bb3c4881839a3f883e5c6b81565b348015610213575f5ffd5b50610227610222366004610f17565b610447565b604051908152602001610126565b348015610240575f5ffd5b5061014e61024f3660046110d0565b61055e565b34801561025f575f5ffd5b5061026861076c565b6040519015158152602001610126565b61014e6102863660046111a8565b6107a0565b348015610296575f5ffd5b5061014e610a0f565b3480156102aa575f5ffd5b506101126102b9366004610f17565b610a36565b3480156102c9575f5ffd5b506002546102689060ff1681565b61014e6102e53660046110d0565b610a5d565b3480156102f5575f5ffd5b506103096103043660046111e7565b610e92565b005b348015610316575f5ffd5b5061022760015481565b60046020525f90815260409020805461033890611209565b80601f016020809104026020016040519081016040528092919081815260200182805461036490611209565b80156103af5780601f10610386576101008083540402835291602001916103af565b820191905f5260205f20905b81548152906001019060200180831161039257829003601f168201915b505050505081565b60605f6103c383610447565b90505f6103d1600283611241565b1590505f80600184826002866103e7575f6103ea565b60015b6040516001600160f81b031960f897881b81166020830152602182019690965260418101949094529190941b90921660618201526062810192909252608282015260a20160408051601f1981840301815291905295945050505050565b5f818152600460205260408120805482919061046290611209565b80601f016020809104026020016040519081016040528092919081815260200182805461048e90611209565b80156104d95780601f106104b0576101008083540402835291602001916104d9565b820191905f5260205f20905b8154815290600101906020018083116104bc57829003601f168201915b5050505050905080515f036104f057505f92915050565b5f818060200190518101906105059190611260565b90505f805b8251811015610555578583828151811061052657610526611314565b60200260200101516001600160a01b0316610541919061133c565b61054b9083611353565b915060010161050a565b50949350505050565b60605f604051806040016040528060188152602001775f434f4d4d4f4e574152455f4147475245474154494f4e5f60401b8152508585858a8a6040516020016105ac96959493929190611366565b60408051601f19818403018152919052805160209091012090508a8114806105f2576040805160016020820152016040516020818303038152906040529250505061075f565b5f5f60035f9054906101000a90046001600160a01b03166001600160a01b031663171f1d5b8f8f8f8f6040518563ffffffff1660e01b815260040161063a94939291906113e2565b6040805180830381865afa158015610654573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610678919061144c565b91509150811580610687575080155b156106b25760408051600160208201520160405160208183030381529060405294505050505061075f565b604051625ffd1f60e81b8152600481018990525f903090635ffd1f00906024015f60405180830381865afa1580156106ec573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052610713919081019061147d565b90505f81805190602001208c8c60405161072e929190611511565b6040805191829003822092909214156020808301919091528251808303909101815290820190915296505050505050505b9998505050505050505050565b5f5f61077743610447565b600181905590505f61078a600283611241565b6002805460ff1916911591821790559392505050565b60603467016345785d8a0000146107fa5760405162461bcd60e51b815260206004820152601960248201527809aeae6e840e6cadcc840caf0c2c6e8d8f240605c62408aa89603b1b60448201526064015b60405180910390fd5b600260019054906101000a90046001600160a01b03166001600160a01b031663d0e30db0346040518263ffffffff1660e01b81526004015f604051808303818588803b158015610848575f5ffd5b505af115801561085a573d5f5f3e3d5ffd5b505f93505050505b828110156109d15782610876826001611353565b11156108bc5760405162461bcd60e51b8152602060048201526015602482015274125b9d985b1a59081bdc18dbd919481bd9999cd95d605a1b60448201526064016107f1565b5f8484838181106108cf576108cf611314565b919091013560f81c91508290506108e581611520565b92505060ff8116156109095760405162461bcd60e51b81526004016107f190611538565b83610915836020611353565b11156109575760405162461bcd60e51b81526020600482015260116024820152704d697373696e6720736c6f74206461746160781b60448201526064016107f1565b84820135610966602084611353565b925084610974846020611353565b11156109b75760405162461bcd60e51b81526020600482015260126024820152714d697373696e672076616c7565206461746160701b60448201526064016107f1565b858301356109c6602085611353565b915591506108629050565b6001546002546040516109f6929160ff16906020019182521515602082015260400190565b6040516020818303038152906040529150505b92915050565b60605f604051602001610a22919061157c565b604051602081830303815290604052905090565b5f8181548110610a44575f80fd5b5f918252602090912001546001600160a01b0316905081565b60603467016345785d8a000014610ab25760405162461bcd60e51b815260206004820152601960248201527809aeae6e840e6cadcc840caf0c2c6e8d8f240605c62408aa89603b1b60448201526064016107f1565b600260019054906101000a90046001600160a01b03166001600160a01b031663d0e30db0346040518263ffffffff1660e01b81526004015f604051808303818588803b158015610b00575f5ffd5b505af1158015610b12573d5f5f3e3d5ffd5b50505050505f604051806040016040528060188152602001775f434f4d4d4f4e574152455f4147475245474154494f4e5f60401b8152508585858a8a604051602001610b6396959493929190611366565b6040516020818303038152906040528051906020012090508a8114610bbe5760405162461bcd60e51b8152602060048201526011602482015270496e76616c6964207369676e617475726560781b60448201526064016107f1565b5f5f60035f9054906101000a90046001600160a01b03166001600160a01b031663171f1d5b8e8e8e8e6040518563ffffffff1660e01b8152600401610c0694939291906113e2565b6040805180830381865afa158015610c20573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c44919061144c565b9150915081610c955760405162461bcd60e51b815260206004820152601860248201527f424c532070616972696e6720636865636b206661696c6564000000000000000060448201526064016107f1565b80610cda5760405162461bcd60e51b8152602060048201526015602482015274496e76616c696420424c53207369676e617475726560581b60448201526064016107f1565b5f5b88811015610e4b5788610cf0826001611353565b1115610d365760405162461bcd60e51b8152602060048201526015602482015274125b9d985b1a59081bdc18dbd919481bd9999cd95d605a1b60448201526064016107f1565b5f8a8a83818110610d4957610d49611314565b919091013560f81c9150829050610d5f81611520565b92505060ff811615610d835760405162461bcd60e51b81526004016107f190611538565b89610d8f836020611353565b1115610dd15760405162461bcd60e51b81526020600482015260116024820152704d697373696e6720736c6f74206461746160781b60448201526064016107f1565b8a820135610de0602084611353565b92508a610dee846020611353565b1115610e315760405162461bcd60e51b81526020600482015260126024820152714d697373696e672076616c7565206461746160701b60448201526064016107f1565b8b830135610e40602085611353565b91559150610cdc9050565b600154600254604051610e70929160ff16906020019182521515602082015260400190565b6040516020818303038152906040529450505050509998505050505050505050565b5f80546001810182558180527f290decd9548b62a8d60345a988386fc84ba6bc95484008f6362f93160ef3e5630180546001600160a01b0319166001600160a01b038416179055604051610eea90829060200161157c565b60408051601f19818403018152918152435f908152600460205220909150610f12828261160a565b505050565b5f60208284031215610f27575f5ffd5b5035919050565b602081525f82518060208401528060208501604085015e5f604082850101526040601f19601f83011684010191505092915050565b634e487b7160e01b5f52604160045260245ffd5b6040805190810167ffffffffffffffff81118282101715610f9a57610f9a610f63565b60405290565b604051601f8201601f1916810167ffffffffffffffff81118282101715610fc957610fc9610f63565b604052919050565b5f60408284031215610fe1575f5ffd5b610fe9610f77565b823581526020928301359281019290925250919050565b5f82601f83011261100f575f5ffd5b611017610f77565b806040840185811115611028575f5ffd5b845b8181101561104257803584526020938401930161102a565b509095945050505050565b5f5f83601f84011261105d575f5ffd5b50813567ffffffffffffffff811115611074575f5ffd5b60208301915083602082850101111561108b575f5ffd5b9250929050565b6001600160a01b03811681146110a6575f5ffd5b50565b80356110b481611092565b919050565b80356001600160e01b0319811681146110b4575f5ffd5b5f5f5f5f5f5f5f5f5f898b036101a08112156110ea575f5ffd5b8a3599506110fb8c60208d01610fd1565b98506080605f198201121561110e575f5ffd5b50611117610f77565b6111248c60608d01611000565b81526111338c60a08d01611000565b602082015296506111478b60e08c01610fd1565b95506101208a013567ffffffffffffffff811115611163575f5ffd5b61116f8c828d0161104d565b9096509450506101408a0135925061118a6101608b016110a9565b91506111996101808b016110b9565b90509295985092959850929598565b5f5f602083850312156111b9575f5ffd5b823567ffffffffffffffff8111156111cf575f5ffd5b6111db8582860161104d565b90969095509350505050565b5f602082840312156111f7575f5ffd5b813561120281611092565b9392505050565b600181811c9082168061121d57607f821691505b60208210810361123b57634e487b7160e01b5f52602260045260245ffd5b50919050565b5f8261125b57634e487b7160e01b5f52601260045260245ffd5b500690565b5f60208284031215611270575f5ffd5b815167ffffffffffffffff811115611286575f5ffd5b8201601f81018413611296575f5ffd5b805167ffffffffffffffff8111156112b0576112b0610f63565b8060051b6112c060208201610fa0565b918252602081840181019290810190878411156112db575f5ffd5b6020850194505b8385101561130957845192506112f783611092565b828252602094850194909101906112e2565b979650505050505050565b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b8082028115828204841417610a0957610a09611328565b80820180821115610a0957610a09611328565b5f87518060208a01845e8201878152606087901b6bffffffffffffffffffffffff191660208201526001600160e01b031986166034820152838560388301375f930160380192835250909695505050505050565b805f5b60028110156113dc5781518452602093840193909101906001016113bd565b50505050565b8481526101208101611401602083018680518252602090810151910152565b61140f6060830185516113ba565b602084015161142160a08401826113ba565b50825160e0830152602083015161010083015295945050505050565b805180151581146110b4575f5ffd5b5f5f6040838503121561145d575f5ffd5b6114668361143d565b91506114746020840161143d565b90509250929050565b5f6020828403121561148d575f5ffd5b815167ffffffffffffffff8111156114a3575f5ffd5b8201601f810184136114b3575f5ffd5b805167ffffffffffffffff8111156114cd576114cd610f63565b6114e0601f8201601f1916602001610fa0565b8181528560208385010111156114f4575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b818382375f9101908152919050565b5f6001820161153157611531611328565b5060010190565b60208082526024908201527f556e737570706f72746564206f7065726174696f6e20286f70206d75737420626040820152636520302960e01b606082015260800190565b602080825282548282018190525f848152918220906040840190835b818110156110425783546001600160a01b0316835260019384019360209093019201611598565b601f821115610f1257805f5260205f20601f840160051c810160208510156115e45750805b601f840160051c820191505b81811015611603575f81556001016115f0565b5050505050565b815167ffffffffffffffff81111561162457611624610f63565b611638816116328454611209565b846115bf565b6020601f82116001811461166a575f83156116535750848201515b5f19600385901b1c1916600184901b178455611603565b5f84815260208120601f198516915b828110156116995787850151825560209485019460019092019101611679565b50848210156116b657868401515f19600387901b60f8161c191681555b50505050600190811b0190555056fea26469706673582212203797890a04431c2dc22d27377559d6692e49ac83c13b3bcd8175b782cfcaa9cb64736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15`\x0EW__\xFD[P`@Qa\x17\xAF8\x03\x80a\x17\xAF\x839\x81\x01`@\x81\x90R`+\x91`|V[`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16s\xCA$\x92\x15\xE0\x82\xE1|\x12\xBB<H\x81\x83\x9A?\x88>\\k\x17\x90U`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16a\x01\0\x02a\x01\0`\x01`\xA8\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90U`\xA7V[_` \x82\x84\x03\x12\x15`\x8BW__\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14`\xA0W__\xFD[\x93\x92PPPV[a\x16\xFB\x80a\0\xB4_9_\xF3\xFE`\x80`@R`\x046\x10a\0\xEFW_5`\xE0\x1C\x80c\xBF\xA0h0\x11a\0\x87W\x80c\xEAS\xBA\xC5\x11a\0WW\x80c\xEAS\xBA\xC5\x14a\x02\xBEW\x80c\xF3\0\xB8b\x14a\x02\xD7W\x80c\xF4\xAB\x9A\xDF\x14a\x02\xEAW\x80c\xFD\x8E\xACI\x14a\x03\x0BW__\xFD[\x80c\xBF\xA0h0\x14a\x02TW\x80c\xC7\xB3u@\x14a\x02xW\x80c\xCFj\xE6\x03\x14a\x02\x8BW\x80c\xDAX\xC7\xD9\x14a\x02\x9FW__\xFD[\x80c\x9C\x91\xDDV\x11a\0\xC2W\x80c\x9C\x91\xDDV\x14a\x01\xBDW\x80c\xA3R\xD7\x07\x14a\x01\xE1W\x80c\xA4\xF8\x81T\x14a\x02\x08W\x80c\xAA,X\xB2\x14a\x025W__\xFD[\x80c\x1C\x17\x8E\x9C\x14a\0\xF3W\x80c\x1E\xE0\x7F\xB1\x14a\x01/W\x80c_\xFD\x1F\0\x14a\x01[W\x80c|\x01Z\x89\x14a\x01zW[__\xFD[4\x80\x15a\0\xFEW__\xFD[P`\x03Ta\x01\x12\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01:W__\xFD[Pa\x01Na\x01I6`\x04a\x0F\x17V[a\x03 V[`@Qa\x01&\x91\x90a\x0F.V[4\x80\x15a\x01fW__\xFD[Pa\x01Na\x01u6`\x04a\x0F\x17V[a\x03\xB7V[4\x80\x15a\x01\x85W__\xFD[Pa\x01N`@Q\x80`@\x01`@R\x80`\x18\x81R` \x01w_COMMONWARE_AGGREGATION_`@\x1B\x81RP\x81V[4\x80\x15a\x01\xC8W__\xFD[P`\x02Ta\x01\x12\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x01\xECW__\xFD[Pa\x01\x12s\xCA$\x92\x15\xE0\x82\xE1|\x12\xBB<H\x81\x83\x9A?\x88>\\k\x81V[4\x80\x15a\x02\x13W__\xFD[Pa\x02'a\x02\"6`\x04a\x0F\x17V[a\x04GV[`@Q\x90\x81R` \x01a\x01&V[4\x80\x15a\x02@W__\xFD[Pa\x01Na\x02O6`\x04a\x10\xD0V[a\x05^V[4\x80\x15a\x02_W__\xFD[Pa\x02ha\x07lV[`@Q\x90\x15\x15\x81R` \x01a\x01&V[a\x01Na\x02\x866`\x04a\x11\xA8V[a\x07\xA0V[4\x80\x15a\x02\x96W__\xFD[Pa\x01Na\n\x0FV[4\x80\x15a\x02\xAAW__\xFD[Pa\x01\x12a\x02\xB96`\x04a\x0F\x17V[a\n6V[4\x80\x15a\x02\xC9W__\xFD[P`\x02Ta\x02h\x90`\xFF\x16\x81V[a\x01Na\x02\xE56`\x04a\x10\xD0V[a\n]V[4\x80\x15a\x02\xF5W__\xFD[Pa\x03\ta\x03\x046`\x04a\x11\xE7V[a\x0E\x92V[\0[4\x80\x15a\x03\x16W__\xFD[Pa\x02'`\x01T\x81V[`\x04` R_\x90\x81R`@\x90 \x80Ta\x038\x90a\x12\tV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03d\x90a\x12\tV[\x80\x15a\x03\xAFW\x80`\x1F\x10a\x03\x86Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03\xAFV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03\x92W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[``_a\x03\xC3\x83a\x04GV[\x90P_a\x03\xD1`\x02\x83a\x12AV[\x15\x90P_\x80`\x01\x84\x82`\x02\x86a\x03\xE7W_a\x03\xEAV[`\x01[`@Q`\x01`\x01`\xF8\x1B\x03\x19`\xF8\x97\x88\x1B\x81\x16` \x83\x01R`!\x82\x01\x96\x90\x96R`A\x81\x01\x94\x90\x94R\x91\x90\x94\x1B\x90\x92\x16`a\x82\x01R`b\x81\x01\x92\x90\x92R`\x82\x82\x01R`\xA2\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x95\x94PPPPPV[_\x81\x81R`\x04` R`@\x81 \x80T\x82\x91\x90a\x04b\x90a\x12\tV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\x8E\x90a\x12\tV[\x80\x15a\x04\xD9W\x80`\x1F\x10a\x04\xB0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\xD9V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\xBCW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x80Q_\x03a\x04\xF0WP_\x92\x91PPV[_\x81\x80` \x01\x90Q\x81\x01\x90a\x05\x05\x91\x90a\x12`V[\x90P_\x80[\x82Q\x81\x10\x15a\x05UW\x85\x83\x82\x81Q\x81\x10a\x05&Wa\x05&a\x13\x14V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x05A\x91\x90a\x13<V[a\x05K\x90\x83a\x13SV[\x91P`\x01\x01a\x05\nV[P\x94\x93PPPPV[``_`@Q\x80`@\x01`@R\x80`\x18\x81R` \x01w_COMMONWARE_AGGREGATION_`@\x1B\x81RP\x85\x85\x85\x8A\x8A`@Q` \x01a\x05\xAC\x96\x95\x94\x93\x92\x91\x90a\x13fV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x90P\x8A\x81\x14\x80a\x05\xF2W`@\x80Q`\x01` \x82\x01R\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92PPPa\x07_V[__`\x03_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x17\x1F\x1D[\x8F\x8F\x8F\x8F`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06:\x94\x93\x92\x91\x90a\x13\xE2V[`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06TW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06x\x91\x90a\x14LV[\x91P\x91P\x81\x15\x80a\x06\x87WP\x80\x15[\x15a\x06\xB2W`@\x80Q`\x01` \x82\x01R\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x94PPPPPa\x07_V[`@Qb_\xFD\x1F`\xE8\x1B\x81R`\x04\x81\x01\x89\x90R_\x900\x90c_\xFD\x1F\0\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xECW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x07\x13\x91\x90\x81\x01\x90a\x14}V[\x90P_\x81\x80Q\x90` \x01 \x8C\x8C`@Qa\x07.\x92\x91\x90a\x15\x11V[`@\x80Q\x91\x82\x90\x03\x82 \x92\x90\x92\x14\x15` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R\x90\x82\x01\x90\x91R\x96PPPPPPP[\x99\x98PPPPPPPPPV[__a\x07wCa\x04GV[`\x01\x81\x90U\x90P_a\x07\x8A`\x02\x83a\x12AV[`\x02\x80T`\xFF\x19\x16\x91\x15\x91\x82\x17\x90U\x93\x92PPPV[``4g\x01cEx]\x8A\0\0\x14a\x07\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\t\xAE\xAEn\x84\x0El\xAD\xCC\x84\x0C\xAF\x0C,n\x8D\x8F$\x06\x05\xC6$\x08\xAA\x89`;\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x02`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB04`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x08HW__\xFD[PZ\xF1\x15\x80\x15a\x08ZW=__>=_\xFD[P_\x93PPPP[\x82\x81\x10\x15a\t\xD1W\x82a\x08v\x82`\x01a\x13SV[\x11\x15a\x08\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x12[\x9D\x98[\x1AY\x08\x1B\xDC\x18\xDB\xD9\x19H\x1B\xD9\x99\x9C\xD9]`Z\x1B`D\x82\x01R`d\x01a\x07\xF1V[_\x84\x84\x83\x81\x81\x10a\x08\xCFWa\x08\xCFa\x13\x14V[\x91\x90\x91\x015`\xF8\x1C\x91P\x82\x90Pa\x08\xE5\x81a\x15 V[\x92PP`\xFF\x81\x16\x15a\t\tW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xF1\x90a\x158V[\x83a\t\x15\x83` a\x13SV[\x11\x15a\tWW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RpMissing slot data`x\x1B`D\x82\x01R`d\x01a\x07\xF1V[\x84\x82\x015a\tf` \x84a\x13SV[\x92P\x84a\tt\x84` a\x13SV[\x11\x15a\t\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqMissing value data`p\x1B`D\x82\x01R`d\x01a\x07\xF1V[\x85\x83\x015a\t\xC6` \x85a\x13SV[\x91U\x91Pa\x08b\x90PV[`\x01T`\x02T`@Qa\t\xF6\x92\x91`\xFF\x16\x90` \x01\x91\x82R\x15\x15` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP[\x92\x91PPV[``_`@Q` \x01a\n\"\x91\x90a\x15|V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[_\x81\x81T\x81\x10a\nDW_\x80\xFD[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[``4g\x01cEx]\x8A\0\0\x14a\n\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\t\xAE\xAEn\x84\x0El\xAD\xCC\x84\x0C\xAF\x0C,n\x8D\x8F$\x06\x05\xC6$\x08\xAA\x89`;\x1B`D\x82\x01R`d\x01a\x07\xF1V[`\x02`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB04`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x0B\0W__\xFD[PZ\xF1\x15\x80\x15a\x0B\x12W=__>=_\xFD[PPPPP_`@Q\x80`@\x01`@R\x80`\x18\x81R` \x01w_COMMONWARE_AGGREGATION_`@\x1B\x81RP\x85\x85\x85\x8A\x8A`@Q` \x01a\x0Bc\x96\x95\x94\x93\x92\x91\x90a\x13fV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x8A\x81\x14a\x0B\xBEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RpInvalid signature`x\x1B`D\x82\x01R`d\x01a\x07\xF1V[__`\x03_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x17\x1F\x1D[\x8E\x8E\x8E\x8E`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\x06\x94\x93\x92\x91\x90a\x13\xE2V[`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0CD\x91\x90a\x14LV[\x91P\x91P\x81a\x0C\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FBLS pairing check failed\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xF1V[\x80a\x0C\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtInvalid BLS signature`X\x1B`D\x82\x01R`d\x01a\x07\xF1V[_[\x88\x81\x10\x15a\x0EKW\x88a\x0C\xF0\x82`\x01a\x13SV[\x11\x15a\r6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x12[\x9D\x98[\x1AY\x08\x1B\xDC\x18\xDB\xD9\x19H\x1B\xD9\x99\x9C\xD9]`Z\x1B`D\x82\x01R`d\x01a\x07\xF1V[_\x8A\x8A\x83\x81\x81\x10a\rIWa\rIa\x13\x14V[\x91\x90\x91\x015`\xF8\x1C\x91P\x82\x90Pa\r_\x81a\x15 V[\x92PP`\xFF\x81\x16\x15a\r\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xF1\x90a\x158V[\x89a\r\x8F\x83` a\x13SV[\x11\x15a\r\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RpMissing slot data`x\x1B`D\x82\x01R`d\x01a\x07\xF1V[\x8A\x82\x015a\r\xE0` \x84a\x13SV[\x92P\x8Aa\r\xEE\x84` a\x13SV[\x11\x15a\x0E1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqMissing value data`p\x1B`D\x82\x01R`d\x01a\x07\xF1V[\x8B\x83\x015a\x0E@` \x85a\x13SV[\x91U\x91Pa\x0C\xDC\x90PV[`\x01T`\x02T`@Qa\x0Ep\x92\x91`\xFF\x16\x90` \x01\x91\x82R\x15\x15` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x94PPPPP\x99\x98PPPPPPPPPV[_\x80T`\x01\x81\x01\x82U\x81\x80R\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90U`@Qa\x0E\xEA\x90\x82\x90` \x01a\x15|V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81RC_\x90\x81R`\x04` R \x90\x91Pa\x0F\x12\x82\x82a\x16\nV[PPPV[_` \x82\x84\x03\x12\x15a\x0F'W__\xFD[P5\x91\x90PV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0F\x9AWa\x0F\x9Aa\x0FcV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0F\xC9Wa\x0F\xC9a\x0FcV[`@R\x91\x90PV[_`@\x82\x84\x03\x12\x15a\x0F\xE1W__\xFD[a\x0F\xE9a\x0FwV[\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[_\x82`\x1F\x83\x01\x12a\x10\x0FW__\xFD[a\x10\x17a\x0FwV[\x80`@\x84\x01\x85\x81\x11\x15a\x10(W__\xFD[\x84[\x81\x81\x10\x15a\x10BW\x805\x84R` \x93\x84\x01\x93\x01a\x10*V[P\x90\x95\x94PPPPPV[__\x83`\x1F\x84\x01\x12a\x10]W__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10tW__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x10\x8BW__\xFD[\x92P\x92\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x10\xA6W__\xFD[PV[\x805a\x10\xB4\x81a\x10\x92V[\x91\x90PV[\x805`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x10\xB4W__\xFD[_________\x89\x8B\x03a\x01\xA0\x81\x12\x15a\x10\xEAW__\xFD[\x8A5\x99Pa\x10\xFB\x8C` \x8D\x01a\x0F\xD1V[\x98P`\x80`_\x19\x82\x01\x12\x15a\x11\x0EW__\xFD[Pa\x11\x17a\x0FwV[a\x11$\x8C``\x8D\x01a\x10\0V[\x81Ra\x113\x8C`\xA0\x8D\x01a\x10\0V[` \x82\x01R\x96Pa\x11G\x8B`\xE0\x8C\x01a\x0F\xD1V[\x95Pa\x01 \x8A\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11cW__\xFD[a\x11o\x8C\x82\x8D\x01a\x10MV[\x90\x96P\x94PPa\x01@\x8A\x015\x92Pa\x11\x8Aa\x01`\x8B\x01a\x10\xA9V[\x91Pa\x11\x99a\x01\x80\x8B\x01a\x10\xB9V[\x90P\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[__` \x83\x85\x03\x12\x15a\x11\xB9W__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\xCFW__\xFD[a\x11\xDB\x85\x82\x86\x01a\x10MV[\x90\x96\x90\x95P\x93PPPPV[_` \x82\x84\x03\x12\x15a\x11\xF7W__\xFD[\x815a\x12\x02\x81a\x10\x92V[\x93\x92PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x12\x1DW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x12;WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[_\x82a\x12[WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x06\x90V[_` \x82\x84\x03\x12\x15a\x12pW__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\x86W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x12\x96W__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\xB0Wa\x12\xB0a\x0FcV[\x80`\x05\x1Ba\x12\xC0` \x82\x01a\x0F\xA0V[\x91\x82R` \x81\x84\x01\x81\x01\x92\x90\x81\x01\x90\x87\x84\x11\x15a\x12\xDBW__\xFD[` \x85\x01\x94P[\x83\x85\x10\x15a\x13\tW\x84Q\x92Pa\x12\xF7\x83a\x10\x92V[\x82\x82R` \x94\x85\x01\x94\x90\x91\x01\x90a\x12\xE2V[\x97\x96PPPPPPPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\n\tWa\n\ta\x13(V[\x80\x82\x01\x80\x82\x11\x15a\n\tWa\n\ta\x13(V[_\x87Q\x80` \x8A\x01\x84^\x82\x01\x87\x81R``\x87\x90\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01R`\x01`\x01`\xE0\x1B\x03\x19\x86\x16`4\x82\x01R\x83\x85`8\x83\x017_\x93\x01`8\x01\x92\x83RP\x90\x96\x95PPPPPPV[\x80_[`\x02\x81\x10\x15a\x13\xDCW\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a\x13\xBDV[PPPPV[\x84\x81Ra\x01 \x81\x01a\x14\x01` \x83\x01\x86\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[a\x14\x0F``\x83\x01\x85Qa\x13\xBAV[` \x84\x01Qa\x14!`\xA0\x84\x01\x82a\x13\xBAV[P\x82Q`\xE0\x83\x01R` \x83\x01Qa\x01\0\x83\x01R\x95\x94PPPPPV[\x80Q\x80\x15\x15\x81\x14a\x10\xB4W__\xFD[__`@\x83\x85\x03\x12\x15a\x14]W__\xFD[a\x14f\x83a\x14=V[\x91Pa\x14t` \x84\x01a\x14=V[\x90P\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a\x14\x8DW__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xA3W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x14\xB3W__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xCDWa\x14\xCDa\x0FcV[a\x14\xE0`\x1F\x82\x01`\x1F\x19\x16` \x01a\x0F\xA0V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\x14\xF4W__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[_`\x01\x82\x01a\x151Wa\x151a\x13(V[P`\x01\x01\x90V[` \x80\x82R`$\x90\x82\x01R\x7FUnsupported operation (op must b`@\x82\x01Rce 0)`\xE0\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R\x82T\x82\x82\x01\x81\x90R_\x84\x81R\x91\x82 \x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x10BW\x83T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x93\x84\x01\x93` \x90\x93\x01\x92\x01a\x15\x98V[`\x1F\x82\x11\x15a\x0F\x12W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x15\xE4WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x16\x03W_\x81U`\x01\x01a\x15\xF0V[PPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16$Wa\x16$a\x0FcV[a\x168\x81a\x162\x84Ta\x12\tV[\x84a\x15\xBFV[` `\x1F\x82\x11`\x01\x81\x14a\x16jW_\x83\x15a\x16SWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x16\x03V[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x16\x99W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x16yV[P\x84\x82\x10\x15a\x16\xB6W\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV\xFE\xA2dipfsX\"\x12 7\x97\x89\n\x04C\x1C-\xC2-'7uY\xD6i.I\xAC\x83\xC1;;\xCD\x81u\xB7\x82\xCF\xCA\xA9\xCBdsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080604052600436106100ef575f3560e01c8063bfa0683011610087578063ea53bac511610057578063ea53bac5146102be578063f300b862146102d7578063f4ab9adf146102ea578063fd8eac491461030b575f5ffd5b8063bfa0683014610254578063c7b3754014610278578063cf6ae6031461028b578063da58c7d91461029f575f5ffd5b80639c91dd56116100c25780639c91dd56146101bd578063a352d707146101e1578063a4f8815414610208578063aa2c58b214610235575f5ffd5b80631c178e9c146100f35780631ee07fb11461012f5780635ffd1f001461015b5780637c015a891461017a575b5f5ffd5b3480156100fe575f5ffd5b50600354610112906001600160a01b031681565b6040516001600160a01b0390911681526020015b60405180910390f35b34801561013a575f5ffd5b5061014e610149366004610f17565b610320565b6040516101269190610f2e565b348015610166575f5ffd5b5061014e610175366004610f17565b6103b7565b348015610185575f5ffd5b5061014e604051806040016040528060188152602001775f434f4d4d4f4e574152455f4147475245474154494f4e5f60401b81525081565b3480156101c8575f5ffd5b506002546101129061010090046001600160a01b031681565b3480156101ec575f5ffd5b5061011273ca249215e082e17c12bb3c4881839a3f883e5c6b81565b348015610213575f5ffd5b50610227610222366004610f17565b610447565b604051908152602001610126565b348015610240575f5ffd5b5061014e61024f3660046110d0565b61055e565b34801561025f575f5ffd5b5061026861076c565b6040519015158152602001610126565b61014e6102863660046111a8565b6107a0565b348015610296575f5ffd5b5061014e610a0f565b3480156102aa575f5ffd5b506101126102b9366004610f17565b610a36565b3480156102c9575f5ffd5b506002546102689060ff1681565b61014e6102e53660046110d0565b610a5d565b3480156102f5575f5ffd5b506103096103043660046111e7565b610e92565b005b348015610316575f5ffd5b5061022760015481565b60046020525f90815260409020805461033890611209565b80601f016020809104026020016040519081016040528092919081815260200182805461036490611209565b80156103af5780601f10610386576101008083540402835291602001916103af565b820191905f5260205f20905b81548152906001019060200180831161039257829003601f168201915b505050505081565b60605f6103c383610447565b90505f6103d1600283611241565b1590505f80600184826002866103e7575f6103ea565b60015b6040516001600160f81b031960f897881b81166020830152602182019690965260418101949094529190941b90921660618201526062810192909252608282015260a20160408051601f1981840301815291905295945050505050565b5f818152600460205260408120805482919061046290611209565b80601f016020809104026020016040519081016040528092919081815260200182805461048e90611209565b80156104d95780601f106104b0576101008083540402835291602001916104d9565b820191905f5260205f20905b8154815290600101906020018083116104bc57829003601f168201915b5050505050905080515f036104f057505f92915050565b5f818060200190518101906105059190611260565b90505f805b8251811015610555578583828151811061052657610526611314565b60200260200101516001600160a01b0316610541919061133c565b61054b9083611353565b915060010161050a565b50949350505050565b60605f604051806040016040528060188152602001775f434f4d4d4f4e574152455f4147475245474154494f4e5f60401b8152508585858a8a6040516020016105ac96959493929190611366565b60408051601f19818403018152919052805160209091012090508a8114806105f2576040805160016020820152016040516020818303038152906040529250505061075f565b5f5f60035f9054906101000a90046001600160a01b03166001600160a01b031663171f1d5b8f8f8f8f6040518563ffffffff1660e01b815260040161063a94939291906113e2565b6040805180830381865afa158015610654573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610678919061144c565b91509150811580610687575080155b156106b25760408051600160208201520160405160208183030381529060405294505050505061075f565b604051625ffd1f60e81b8152600481018990525f903090635ffd1f00906024015f60405180830381865afa1580156106ec573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052610713919081019061147d565b90505f81805190602001208c8c60405161072e929190611511565b6040805191829003822092909214156020808301919091528251808303909101815290820190915296505050505050505b9998505050505050505050565b5f5f61077743610447565b600181905590505f61078a600283611241565b6002805460ff1916911591821790559392505050565b60603467016345785d8a0000146107fa5760405162461bcd60e51b815260206004820152601960248201527809aeae6e840e6cadcc840caf0c2c6e8d8f240605c62408aa89603b1b60448201526064015b60405180910390fd5b600260019054906101000a90046001600160a01b03166001600160a01b031663d0e30db0346040518263ffffffff1660e01b81526004015f604051808303818588803b158015610848575f5ffd5b505af115801561085a573d5f5f3e3d5ffd5b505f93505050505b828110156109d15782610876826001611353565b11156108bc5760405162461bcd60e51b8152602060048201526015602482015274125b9d985b1a59081bdc18dbd919481bd9999cd95d605a1b60448201526064016107f1565b5f8484838181106108cf576108cf611314565b919091013560f81c91508290506108e581611520565b92505060ff8116156109095760405162461bcd60e51b81526004016107f190611538565b83610915836020611353565b11156109575760405162461bcd60e51b81526020600482015260116024820152704d697373696e6720736c6f74206461746160781b60448201526064016107f1565b84820135610966602084611353565b925084610974846020611353565b11156109b75760405162461bcd60e51b81526020600482015260126024820152714d697373696e672076616c7565206461746160701b60448201526064016107f1565b858301356109c6602085611353565b915591506108629050565b6001546002546040516109f6929160ff16906020019182521515602082015260400190565b6040516020818303038152906040529150505b92915050565b60605f604051602001610a22919061157c565b604051602081830303815290604052905090565b5f8181548110610a44575f80fd5b5f918252602090912001546001600160a01b0316905081565b60603467016345785d8a000014610ab25760405162461bcd60e51b815260206004820152601960248201527809aeae6e840e6cadcc840caf0c2c6e8d8f240605c62408aa89603b1b60448201526064016107f1565b600260019054906101000a90046001600160a01b03166001600160a01b031663d0e30db0346040518263ffffffff1660e01b81526004015f604051808303818588803b158015610b00575f5ffd5b505af1158015610b12573d5f5f3e3d5ffd5b50505050505f604051806040016040528060188152602001775f434f4d4d4f4e574152455f4147475245474154494f4e5f60401b8152508585858a8a604051602001610b6396959493929190611366565b6040516020818303038152906040528051906020012090508a8114610bbe5760405162461bcd60e51b8152602060048201526011602482015270496e76616c6964207369676e617475726560781b60448201526064016107f1565b5f5f60035f9054906101000a90046001600160a01b03166001600160a01b031663171f1d5b8e8e8e8e6040518563ffffffff1660e01b8152600401610c0694939291906113e2565b6040805180830381865afa158015610c20573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c44919061144c565b9150915081610c955760405162461bcd60e51b815260206004820152601860248201527f424c532070616972696e6720636865636b206661696c6564000000000000000060448201526064016107f1565b80610cda5760405162461bcd60e51b8152602060048201526015602482015274496e76616c696420424c53207369676e617475726560581b60448201526064016107f1565b5f5b88811015610e4b5788610cf0826001611353565b1115610d365760405162461bcd60e51b8152602060048201526015602482015274125b9d985b1a59081bdc18dbd919481bd9999cd95d605a1b60448201526064016107f1565b5f8a8a83818110610d4957610d49611314565b919091013560f81c9150829050610d5f81611520565b92505060ff811615610d835760405162461bcd60e51b81526004016107f190611538565b89610d8f836020611353565b1115610dd15760405162461bcd60e51b81526020600482015260116024820152704d697373696e6720736c6f74206461746160781b60448201526064016107f1565b8a820135610de0602084611353565b92508a610dee846020611353565b1115610e315760405162461bcd60e51b81526020600482015260126024820152714d697373696e672076616c7565206461746160701b60448201526064016107f1565b8b830135610e40602085611353565b91559150610cdc9050565b600154600254604051610e70929160ff16906020019182521515602082015260400190565b6040516020818303038152906040529450505050509998505050505050505050565b5f80546001810182558180527f290decd9548b62a8d60345a988386fc84ba6bc95484008f6362f93160ef3e5630180546001600160a01b0319166001600160a01b038416179055604051610eea90829060200161157c565b60408051601f19818403018152918152435f908152600460205220909150610f12828261160a565b505050565b5f60208284031215610f27575f5ffd5b5035919050565b602081525f82518060208401528060208501604085015e5f604082850101526040601f19601f83011684010191505092915050565b634e487b7160e01b5f52604160045260245ffd5b6040805190810167ffffffffffffffff81118282101715610f9a57610f9a610f63565b60405290565b604051601f8201601f1916810167ffffffffffffffff81118282101715610fc957610fc9610f63565b604052919050565b5f60408284031215610fe1575f5ffd5b610fe9610f77565b823581526020928301359281019290925250919050565b5f82601f83011261100f575f5ffd5b611017610f77565b806040840185811115611028575f5ffd5b845b8181101561104257803584526020938401930161102a565b509095945050505050565b5f5f83601f84011261105d575f5ffd5b50813567ffffffffffffffff811115611074575f5ffd5b60208301915083602082850101111561108b575f5ffd5b9250929050565b6001600160a01b03811681146110a6575f5ffd5b50565b80356110b481611092565b919050565b80356001600160e01b0319811681146110b4575f5ffd5b5f5f5f5f5f5f5f5f5f898b036101a08112156110ea575f5ffd5b8a3599506110fb8c60208d01610fd1565b98506080605f198201121561110e575f5ffd5b50611117610f77565b6111248c60608d01611000565b81526111338c60a08d01611000565b602082015296506111478b60e08c01610fd1565b95506101208a013567ffffffffffffffff811115611163575f5ffd5b61116f8c828d0161104d565b9096509450506101408a0135925061118a6101608b016110a9565b91506111996101808b016110b9565b90509295985092959850929598565b5f5f602083850312156111b9575f5ffd5b823567ffffffffffffffff8111156111cf575f5ffd5b6111db8582860161104d565b90969095509350505050565b5f602082840312156111f7575f5ffd5b813561120281611092565b9392505050565b600181811c9082168061121d57607f821691505b60208210810361123b57634e487b7160e01b5f52602260045260245ffd5b50919050565b5f8261125b57634e487b7160e01b5f52601260045260245ffd5b500690565b5f60208284031215611270575f5ffd5b815167ffffffffffffffff811115611286575f5ffd5b8201601f81018413611296575f5ffd5b805167ffffffffffffffff8111156112b0576112b0610f63565b8060051b6112c060208201610fa0565b918252602081840181019290810190878411156112db575f5ffd5b6020850194505b8385101561130957845192506112f783611092565b828252602094850194909101906112e2565b979650505050505050565b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b8082028115828204841417610a0957610a09611328565b80820180821115610a0957610a09611328565b5f87518060208a01845e8201878152606087901b6bffffffffffffffffffffffff191660208201526001600160e01b031986166034820152838560388301375f930160380192835250909695505050505050565b805f5b60028110156113dc5781518452602093840193909101906001016113bd565b50505050565b8481526101208101611401602083018680518252602090810151910152565b61140f6060830185516113ba565b602084015161142160a08401826113ba565b50825160e0830152602083015161010083015295945050505050565b805180151581146110b4575f5ffd5b5f5f6040838503121561145d575f5ffd5b6114668361143d565b91506114746020840161143d565b90509250929050565b5f6020828403121561148d575f5ffd5b815167ffffffffffffffff8111156114a3575f5ffd5b8201601f810184136114b3575f5ffd5b805167ffffffffffffffff8111156114cd576114cd610f63565b6114e0601f8201601f1916602001610fa0565b8181528560208385010111156114f4575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b818382375f9101908152919050565b5f6001820161153157611531611328565b5060010190565b60208082526024908201527f556e737570706f72746564206f7065726174696f6e20286f70206d75737420626040820152636520302960e01b606082015260800190565b602080825282548282018190525f848152918220906040840190835b818110156110425783546001600160a01b0316835260019384019360209093019201611598565b601f821115610f1257805f5260205f20601f840160051c810160208510156115e45750805b601f840160051c820191505b81811015611603575f81556001016115f0565b5050505050565b815167ffffffffffffffff81111561162457611624610f63565b611638816116328454611209565b846115bf565b6020601f82116001811461166a575f83156116535750848201515b5f19600385901b1c1916600184901b178455611603565b5f84815260208120601f198516915b828110156116995787850151825560209485019460019092019101611679565b50848210156116b657868401515f19600387901b60f8161c191681555b50505050600190811b0190555056fea26469706673582212203797890a04431c2dc22d27377559d6692e49ac83c13b3bcd8175b782cfcaa9cb64736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\0\xEFW_5`\xE0\x1C\x80c\xBF\xA0h0\x11a\0\x87W\x80c\xEAS\xBA\xC5\x11a\0WW\x80c\xEAS\xBA\xC5\x14a\x02\xBEW\x80c\xF3\0\xB8b\x14a\x02\xD7W\x80c\xF4\xAB\x9A\xDF\x14a\x02\xEAW\x80c\xFD\x8E\xACI\x14a\x03\x0BW__\xFD[\x80c\xBF\xA0h0\x14a\x02TW\x80c\xC7\xB3u@\x14a\x02xW\x80c\xCFj\xE6\x03\x14a\x02\x8BW\x80c\xDAX\xC7\xD9\x14a\x02\x9FW__\xFD[\x80c\x9C\x91\xDDV\x11a\0\xC2W\x80c\x9C\x91\xDDV\x14a\x01\xBDW\x80c\xA3R\xD7\x07\x14a\x01\xE1W\x80c\xA4\xF8\x81T\x14a\x02\x08W\x80c\xAA,X\xB2\x14a\x025W__\xFD[\x80c\x1C\x17\x8E\x9C\x14a\0\xF3W\x80c\x1E\xE0\x7F\xB1\x14a\x01/W\x80c_\xFD\x1F\0\x14a\x01[W\x80c|\x01Z\x89\x14a\x01zW[__\xFD[4\x80\x15a\0\xFEW__\xFD[P`\x03Ta\x01\x12\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01:W__\xFD[Pa\x01Na\x01I6`\x04a\x0F\x17V[a\x03 V[`@Qa\x01&\x91\x90a\x0F.V[4\x80\x15a\x01fW__\xFD[Pa\x01Na\x01u6`\x04a\x0F\x17V[a\x03\xB7V[4\x80\x15a\x01\x85W__\xFD[Pa\x01N`@Q\x80`@\x01`@R\x80`\x18\x81R` \x01w_COMMONWARE_AGGREGATION_`@\x1B\x81RP\x81V[4\x80\x15a\x01\xC8W__\xFD[P`\x02Ta\x01\x12\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x01\xECW__\xFD[Pa\x01\x12s\xCA$\x92\x15\xE0\x82\xE1|\x12\xBB<H\x81\x83\x9A?\x88>\\k\x81V[4\x80\x15a\x02\x13W__\xFD[Pa\x02'a\x02\"6`\x04a\x0F\x17V[a\x04GV[`@Q\x90\x81R` \x01a\x01&V[4\x80\x15a\x02@W__\xFD[Pa\x01Na\x02O6`\x04a\x10\xD0V[a\x05^V[4\x80\x15a\x02_W__\xFD[Pa\x02ha\x07lV[`@Q\x90\x15\x15\x81R` \x01a\x01&V[a\x01Na\x02\x866`\x04a\x11\xA8V[a\x07\xA0V[4\x80\x15a\x02\x96W__\xFD[Pa\x01Na\n\x0FV[4\x80\x15a\x02\xAAW__\xFD[Pa\x01\x12a\x02\xB96`\x04a\x0F\x17V[a\n6V[4\x80\x15a\x02\xC9W__\xFD[P`\x02Ta\x02h\x90`\xFF\x16\x81V[a\x01Na\x02\xE56`\x04a\x10\xD0V[a\n]V[4\x80\x15a\x02\xF5W__\xFD[Pa\x03\ta\x03\x046`\x04a\x11\xE7V[a\x0E\x92V[\0[4\x80\x15a\x03\x16W__\xFD[Pa\x02'`\x01T\x81V[`\x04` R_\x90\x81R`@\x90 \x80Ta\x038\x90a\x12\tV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03d\x90a\x12\tV[\x80\x15a\x03\xAFW\x80`\x1F\x10a\x03\x86Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03\xAFV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03\x92W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[``_a\x03\xC3\x83a\x04GV[\x90P_a\x03\xD1`\x02\x83a\x12AV[\x15\x90P_\x80`\x01\x84\x82`\x02\x86a\x03\xE7W_a\x03\xEAV[`\x01[`@Q`\x01`\x01`\xF8\x1B\x03\x19`\xF8\x97\x88\x1B\x81\x16` \x83\x01R`!\x82\x01\x96\x90\x96R`A\x81\x01\x94\x90\x94R\x91\x90\x94\x1B\x90\x92\x16`a\x82\x01R`b\x81\x01\x92\x90\x92R`\x82\x82\x01R`\xA2\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x95\x94PPPPPV[_\x81\x81R`\x04` R`@\x81 \x80T\x82\x91\x90a\x04b\x90a\x12\tV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\x8E\x90a\x12\tV[\x80\x15a\x04\xD9W\x80`\x1F\x10a\x04\xB0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\xD9V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\xBCW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x80Q_\x03a\x04\xF0WP_\x92\x91PPV[_\x81\x80` \x01\x90Q\x81\x01\x90a\x05\x05\x91\x90a\x12`V[\x90P_\x80[\x82Q\x81\x10\x15a\x05UW\x85\x83\x82\x81Q\x81\x10a\x05&Wa\x05&a\x13\x14V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x05A\x91\x90a\x13<V[a\x05K\x90\x83a\x13SV[\x91P`\x01\x01a\x05\nV[P\x94\x93PPPPV[``_`@Q\x80`@\x01`@R\x80`\x18\x81R` \x01w_COMMONWARE_AGGREGATION_`@\x1B\x81RP\x85\x85\x85\x8A\x8A`@Q` \x01a\x05\xAC\x96\x95\x94\x93\x92\x91\x90a\x13fV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x90P\x8A\x81\x14\x80a\x05\xF2W`@\x80Q`\x01` \x82\x01R\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92PPPa\x07_V[__`\x03_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x17\x1F\x1D[\x8F\x8F\x8F\x8F`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06:\x94\x93\x92\x91\x90a\x13\xE2V[`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06TW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06x\x91\x90a\x14LV[\x91P\x91P\x81\x15\x80a\x06\x87WP\x80\x15[\x15a\x06\xB2W`@\x80Q`\x01` \x82\x01R\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x94PPPPPa\x07_V[`@Qb_\xFD\x1F`\xE8\x1B\x81R`\x04\x81\x01\x89\x90R_\x900\x90c_\xFD\x1F\0\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xECW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x07\x13\x91\x90\x81\x01\x90a\x14}V[\x90P_\x81\x80Q\x90` \x01 \x8C\x8C`@Qa\x07.\x92\x91\x90a\x15\x11V[`@\x80Q\x91\x82\x90\x03\x82 \x92\x90\x92\x14\x15` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R\x90\x82\x01\x90\x91R\x96PPPPPPP[\x99\x98PPPPPPPPPV[__a\x07wCa\x04GV[`\x01\x81\x90U\x90P_a\x07\x8A`\x02\x83a\x12AV[`\x02\x80T`\xFF\x19\x16\x91\x15\x91\x82\x17\x90U\x93\x92PPPV[``4g\x01cEx]\x8A\0\0\x14a\x07\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\t\xAE\xAEn\x84\x0El\xAD\xCC\x84\x0C\xAF\x0C,n\x8D\x8F$\x06\x05\xC6$\x08\xAA\x89`;\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x02`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB04`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x08HW__\xFD[PZ\xF1\x15\x80\x15a\x08ZW=__>=_\xFD[P_\x93PPPP[\x82\x81\x10\x15a\t\xD1W\x82a\x08v\x82`\x01a\x13SV[\x11\x15a\x08\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x12[\x9D\x98[\x1AY\x08\x1B\xDC\x18\xDB\xD9\x19H\x1B\xD9\x99\x9C\xD9]`Z\x1B`D\x82\x01R`d\x01a\x07\xF1V[_\x84\x84\x83\x81\x81\x10a\x08\xCFWa\x08\xCFa\x13\x14V[\x91\x90\x91\x015`\xF8\x1C\x91P\x82\x90Pa\x08\xE5\x81a\x15 V[\x92PP`\xFF\x81\x16\x15a\t\tW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xF1\x90a\x158V[\x83a\t\x15\x83` a\x13SV[\x11\x15a\tWW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RpMissing slot data`x\x1B`D\x82\x01R`d\x01a\x07\xF1V[\x84\x82\x015a\tf` \x84a\x13SV[\x92P\x84a\tt\x84` a\x13SV[\x11\x15a\t\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqMissing value data`p\x1B`D\x82\x01R`d\x01a\x07\xF1V[\x85\x83\x015a\t\xC6` \x85a\x13SV[\x91U\x91Pa\x08b\x90PV[`\x01T`\x02T`@Qa\t\xF6\x92\x91`\xFF\x16\x90` \x01\x91\x82R\x15\x15` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP[\x92\x91PPV[``_`@Q` \x01a\n\"\x91\x90a\x15|V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[_\x81\x81T\x81\x10a\nDW_\x80\xFD[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[``4g\x01cEx]\x8A\0\0\x14a\n\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\t\xAE\xAEn\x84\x0El\xAD\xCC\x84\x0C\xAF\x0C,n\x8D\x8F$\x06\x05\xC6$\x08\xAA\x89`;\x1B`D\x82\x01R`d\x01a\x07\xF1V[`\x02`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB04`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x0B\0W__\xFD[PZ\xF1\x15\x80\x15a\x0B\x12W=__>=_\xFD[PPPPP_`@Q\x80`@\x01`@R\x80`\x18\x81R` \x01w_COMMONWARE_AGGREGATION_`@\x1B\x81RP\x85\x85\x85\x8A\x8A`@Q` \x01a\x0Bc\x96\x95\x94\x93\x92\x91\x90a\x13fV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x8A\x81\x14a\x0B\xBEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RpInvalid signature`x\x1B`D\x82\x01R`d\x01a\x07\xF1V[__`\x03_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x17\x1F\x1D[\x8E\x8E\x8E\x8E`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\x06\x94\x93\x92\x91\x90a\x13\xE2V[`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0CD\x91\x90a\x14LV[\x91P\x91P\x81a\x0C\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FBLS pairing check failed\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xF1V[\x80a\x0C\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtInvalid BLS signature`X\x1B`D\x82\x01R`d\x01a\x07\xF1V[_[\x88\x81\x10\x15a\x0EKW\x88a\x0C\xF0\x82`\x01a\x13SV[\x11\x15a\r6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x12[\x9D\x98[\x1AY\x08\x1B\xDC\x18\xDB\xD9\x19H\x1B\xD9\x99\x9C\xD9]`Z\x1B`D\x82\x01R`d\x01a\x07\xF1V[_\x8A\x8A\x83\x81\x81\x10a\rIWa\rIa\x13\x14V[\x91\x90\x91\x015`\xF8\x1C\x91P\x82\x90Pa\r_\x81a\x15 V[\x92PP`\xFF\x81\x16\x15a\r\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xF1\x90a\x158V[\x89a\r\x8F\x83` a\x13SV[\x11\x15a\r\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RpMissing slot data`x\x1B`D\x82\x01R`d\x01a\x07\xF1V[\x8A\x82\x015a\r\xE0` \x84a\x13SV[\x92P\x8Aa\r\xEE\x84` a\x13SV[\x11\x15a\x0E1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqMissing value data`p\x1B`D\x82\x01R`d\x01a\x07\xF1V[\x8B\x83\x015a\x0E@` \x85a\x13SV[\x91U\x91Pa\x0C\xDC\x90PV[`\x01T`\x02T`@Qa\x0Ep\x92\x91`\xFF\x16\x90` \x01\x91\x82R\x15\x15` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x94PPPPP\x99\x98PPPPPPPPPV[_\x80T`\x01\x81\x01\x82U\x81\x80R\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90U`@Qa\x0E\xEA\x90\x82\x90` \x01a\x15|V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81RC_\x90\x81R`\x04` R \x90\x91Pa\x0F\x12\x82\x82a\x16\nV[PPPV[_` \x82\x84\x03\x12\x15a\x0F'W__\xFD[P5\x91\x90PV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0F\x9AWa\x0F\x9Aa\x0FcV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0F\xC9Wa\x0F\xC9a\x0FcV[`@R\x91\x90PV[_`@\x82\x84\x03\x12\x15a\x0F\xE1W__\xFD[a\x0F\xE9a\x0FwV[\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[_\x82`\x1F\x83\x01\x12a\x10\x0FW__\xFD[a\x10\x17a\x0FwV[\x80`@\x84\x01\x85\x81\x11\x15a\x10(W__\xFD[\x84[\x81\x81\x10\x15a\x10BW\x805\x84R` \x93\x84\x01\x93\x01a\x10*V[P\x90\x95\x94PPPPPV[__\x83`\x1F\x84\x01\x12a\x10]W__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10tW__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x10\x8BW__\xFD[\x92P\x92\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x10\xA6W__\xFD[PV[\x805a\x10\xB4\x81a\x10\x92V[\x91\x90PV[\x805`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x10\xB4W__\xFD[_________\x89\x8B\x03a\x01\xA0\x81\x12\x15a\x10\xEAW__\xFD[\x8A5\x99Pa\x10\xFB\x8C` \x8D\x01a\x0F\xD1V[\x98P`\x80`_\x19\x82\x01\x12\x15a\x11\x0EW__\xFD[Pa\x11\x17a\x0FwV[a\x11$\x8C``\x8D\x01a\x10\0V[\x81Ra\x113\x8C`\xA0\x8D\x01a\x10\0V[` \x82\x01R\x96Pa\x11G\x8B`\xE0\x8C\x01a\x0F\xD1V[\x95Pa\x01 \x8A\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11cW__\xFD[a\x11o\x8C\x82\x8D\x01a\x10MV[\x90\x96P\x94PPa\x01@\x8A\x015\x92Pa\x11\x8Aa\x01`\x8B\x01a\x10\xA9V[\x91Pa\x11\x99a\x01\x80\x8B\x01a\x10\xB9V[\x90P\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[__` \x83\x85\x03\x12\x15a\x11\xB9W__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\xCFW__\xFD[a\x11\xDB\x85\x82\x86\x01a\x10MV[\x90\x96\x90\x95P\x93PPPPV[_` \x82\x84\x03\x12\x15a\x11\xF7W__\xFD[\x815a\x12\x02\x81a\x10\x92V[\x93\x92PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x12\x1DW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x12;WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[_\x82a\x12[WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x06\x90V[_` \x82\x84\x03\x12\x15a\x12pW__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\x86W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x12\x96W__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\xB0Wa\x12\xB0a\x0FcV[\x80`\x05\x1Ba\x12\xC0` \x82\x01a\x0F\xA0V[\x91\x82R` \x81\x84\x01\x81\x01\x92\x90\x81\x01\x90\x87\x84\x11\x15a\x12\xDBW__\xFD[` \x85\x01\x94P[\x83\x85\x10\x15a\x13\tW\x84Q\x92Pa\x12\xF7\x83a\x10\x92V[\x82\x82R` \x94\x85\x01\x94\x90\x91\x01\x90a\x12\xE2V[\x97\x96PPPPPPPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\n\tWa\n\ta\x13(V[\x80\x82\x01\x80\x82\x11\x15a\n\tWa\n\ta\x13(V[_\x87Q\x80` \x8A\x01\x84^\x82\x01\x87\x81R``\x87\x90\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01R`\x01`\x01`\xE0\x1B\x03\x19\x86\x16`4\x82\x01R\x83\x85`8\x83\x017_\x93\x01`8\x01\x92\x83RP\x90\x96\x95PPPPPPV[\x80_[`\x02\x81\x10\x15a\x13\xDCW\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a\x13\xBDV[PPPPV[\x84\x81Ra\x01 \x81\x01a\x14\x01` \x83\x01\x86\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[a\x14\x0F``\x83\x01\x85Qa\x13\xBAV[` \x84\x01Qa\x14!`\xA0\x84\x01\x82a\x13\xBAV[P\x82Q`\xE0\x83\x01R` \x83\x01Qa\x01\0\x83\x01R\x95\x94PPPPPV[\x80Q\x80\x15\x15\x81\x14a\x10\xB4W__\xFD[__`@\x83\x85\x03\x12\x15a\x14]W__\xFD[a\x14f\x83a\x14=V[\x91Pa\x14t` \x84\x01a\x14=V[\x90P\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a\x14\x8DW__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xA3W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x14\xB3W__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xCDWa\x14\xCDa\x0FcV[a\x14\xE0`\x1F\x82\x01`\x1F\x19\x16` \x01a\x0F\xA0V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\x14\xF4W__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[_`\x01\x82\x01a\x151Wa\x151a\x13(V[P`\x01\x01\x90V[` \x80\x82R`$\x90\x82\x01R\x7FUnsupported operation (op must b`@\x82\x01Rce 0)`\xE0\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R\x82T\x82\x82\x01\x81\x90R_\x84\x81R\x91\x82 \x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x10BW\x83T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x93\x84\x01\x93` \x90\x93\x01\x92\x01a\x15\x98V[`\x1F\x82\x11\x15a\x0F\x12W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x15\xE4WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x16\x03W_\x81U`\x01\x01a\x15\xF0V[PPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16$Wa\x16$a\x0FcV[a\x168\x81a\x162\x84Ta\x12\tV[\x84a\x15\xBFV[` `\x1F\x82\x11`\x01\x81\x14a\x16jW_\x83\x15a\x16SWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x16\x03V[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x16\x99W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x16yV[P\x84\x82\x10\x15a\x16\xB6W\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV\xFE\xA2dipfsX\"\x12 7\x97\x89\n\x04C\x1C-\xC2-'7uY\xD6i.I\xAC\x83\xC1;;\xCD\x81u\xB7\x82\xCF\xCA\xA9\xCBdsolcC\0\x08\x1C\x003",
    );
    /**Constructor`.
```solidity
constructor(address _paymentContract);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
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
function getCurrentTotalVotingPower(uint256 _blockNumber) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentTotalVotingPowerCall {
        pub _blockNumber: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getCurrentTotalVotingPower(uint256)`](getCurrentTotalVotingPowerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentTotalVotingPowerReturn {
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
                    (value._blockNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentTotalVotingPowerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _blockNumber: tuple.0 }
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
                    > as alloy_sol_types::SolType>::tokenize(&self._blockNumber),
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
function operatorExecuteVote(uint256 blockNumber) external view returns (bytes memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorExecuteVoteCall {
        pub blockNumber: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`operatorExecuteVote(uint256)`](operatorExecuteVoteCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorExecuteVoteReturn {
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
                    (value.blockNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorExecuteVoteCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { blockNumber: tuple.0 }
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
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
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
function slashExecVote(bytes32 msgHash, BN254.G1Point memory apk, BN254.G2Point memory apkG2, BN254.G1Point memory sigma, bytes memory storageUpdates, uint256 blockNumber, address targetAddr, bytes4 targetFunction) external view returns (bytes memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashExecVoteCall {
        pub msgHash: alloy::sol_types::private::FixedBytes<32>,
        pub apk: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        pub apkG2: <BN254::G2Point as alloy::sol_types::SolType>::RustType,
        pub sigma: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        pub storageUpdates: alloy::sol_types::private::Bytes,
        pub blockNumber: alloy::sol_types::private::primitives::aliases::U256,
        pub targetAddr: alloy::sol_types::private::Address,
        pub targetFunction: alloy::sol_types::private::FixedBytes<4>,
    }
    ///Container type for the return parameters of the [`slashExecVote(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256),bytes,uint256,address,bytes4)`](slashExecVoteCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashExecVoteReturn {
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
                        value.blockNumber,
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
                        blockNumber: tuple.5,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
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
    /**Function with signature `voters(uint256)` and selector `0xda58c7d9`.
```solidity
function voters(uint256) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct votersCall {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`voters(uint256)`](votersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct votersReturn {
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
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`votersArrayStorage(uint256)`](votersArrayStorageCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct votersArrayStorageReturn {
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
function writeExecuteVote(bytes32 msgHash, BN254.G1Point memory apk, BN254.G2Point memory apkG2, BN254.G1Point memory sigma, bytes memory storageUpdates, uint256 blockNumber, address targetAddr, bytes4 targetFunction) external payable returns (bytes memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct writeExecuteVoteCall {
        pub msgHash: alloy::sol_types::private::FixedBytes<32>,
        pub apk: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        pub apkG2: <BN254::G2Point as alloy::sol_types::SolType>::RustType,
        pub sigma: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        pub storageUpdates: alloy::sol_types::private::Bytes,
        pub blockNumber: alloy::sol_types::private::primitives::aliases::U256,
        pub targetAddr: alloy::sol_types::private::Address,
        pub targetFunction: alloy::sol_types::private::FixedBytes<4>,
    }
    ///Container type for the return parameters of the [`writeExecuteVote(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256),bytes,uint256,address,bytes4)`](writeExecuteVoteCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct writeExecuteVoteReturn {
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
                        value.blockNumber,
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
                        blockNumber: tuple.5,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
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
        pub storageUpdates: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`writeExecuteVoteTest(bytes)`](writeExecuteVoteTestCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct writeExecuteVoteTestReturn {
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
        BLS_SIG_CHECKER(BLS_SIG_CHECKERCall),
        addVoter(addVoterCall),
        blsSignatureChecker(blsSignatureCheckerCall),
        currentTotalVotingPower(currentTotalVotingPowerCall),
        executeVote(executeVoteCall),
        getCurrentTotalVotingPower(getCurrentTotalVotingPowerCall),
        getCurrentVotersArray(getCurrentVotersArrayCall),
        lastVotePassed(lastVotePassedCall),
        namespace(namespaceCall),
        operatorExecuteVote(operatorExecuteVoteCall),
        paymentContract(paymentContractCall),
        slashExecVote(slashExecVoteCall),
        voters(votersCall),
        votersArrayStorage(votersArrayStorageCall),
        writeExecuteVote(writeExecuteVoteCall),
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
            [244u8, 171u8, 154u8, 223u8],
            [253u8, 142u8, 172u8, 73u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for VotingContractCalls {
        const NAME: &'static str = "VotingContractCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 16usize;
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
        #[allow(unsafe_code, non_snake_case)]
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
            (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate)
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
            _blockNumber: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getCurrentTotalVotingPowerCall, N> {
            self.call_builder(
                &getCurrentTotalVotingPowerCall {
                    _blockNumber,
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
            blockNumber: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, operatorExecuteVoteCall, N> {
            self.call_builder(
                &operatorExecuteVoteCall {
                    blockNumber,
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
            blockNumber: alloy::sol_types::private::primitives::aliases::U256,
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
                    blockNumber,
                    targetAddr,
                    targetFunction,
                },
            )
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
            blockNumber: alloy::sol_types::private::primitives::aliases::U256,
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
                    blockNumber,
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
