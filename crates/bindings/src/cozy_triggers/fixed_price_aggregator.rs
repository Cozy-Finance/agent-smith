pub use fixed_price_aggregator::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod fixed_price_aggregator {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"_decimals\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_price\",\"type\":\"int256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"description\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint80\",\"name\":\"_roundId\",\"type\":\"uint80\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRoundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"latestRoundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"version\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static FIXEDPRICEAGGREGATOR_ABI: ::ethers_contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers_contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = &[
        96,
        192,
        96,
        64,
        82,
        52,
        128,
        21,
        97,
        0,
        16,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        64,
        81,
        97,
        3,
        7,
        56,
        3,
        128,
        97,
        3,
        7,
        131,
        57,
        129,
        1,
        96,
        64,
        129,
        144,
        82,
        97,
        0,
        47,
        145,
        97,
        0,
        61,
        86,
        91,
        96,
        160,
        82,
        96,
        255,
        22,
        96,
        128,
        82,
        97,
        0,
        113,
        86,
        91,
        96,
        0,
        128,
        96,
        64,
        131,
        133,
        3,
        18,
        21,
        97,
        0,
        80,
        87,
        96,
        0,
        128,
        253,
        91,
        130,
        81,
        96,
        255,
        129,
        22,
        129,
        20,
        97,
        0,
        97,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        32,
        147,
        144,
        147,
        1,
        81,
        146,
        148,
        146,
        147,
        80,
        80,
        80,
        86,
        91,
        96,
        128,
        81,
        96,
        160,
        81,
        97,
        2,
        107,
        97,
        0,
        156,
        96,
        0,
        57,
        96,
        0,
        129,
        129,
        97,
        1,
        10,
        1,
        82,
        97,
        1,
        111,
        1,
        82,
        96,
        0,
        96,
        113,
        1,
        82,
        97,
        2,
        107,
        96,
        0,
        243,
        254,
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        97,
        0,
        16,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        4,
        54,
        16,
        97,
        0,
        103,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        114,
        132,
        228,
        22,
        17,
        97,
        0,
        80,
        87,
        128,
        99,
        114,
        132,
        228,
        22,
        20,
        97,
        0,
        185,
        87,
        128,
        99,
        154,
        111,
        200,
        245,
        20,
        97,
        0,
        248,
        87,
        128,
        99,
        254,
        175,
        150,
        140,
        20,
        97,
        1,
        107,
        87,
        96,
        0,
        128,
        253,
        91,
        128,
        99,
        49,
        60,
        229,
        103,
        20,
        97,
        0,
        108,
        87,
        128,
        99,
        84,
        253,
        77,
        80,
        20,
        97,
        0,
        170,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        97,
        0,
        147,
        127,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        129,
        86,
        91,
        96,
        64,
        81,
        96,
        255,
        144,
        145,
        22,
        129,
        82,
        96,
        32,
        1,
        91,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        243,
        91,
        96,
        64,
        81,
        96,
        0,
        129,
        82,
        96,
        32,
        1,
        97,
        0,
        161,
        86,
        91,
        96,
        64,
        128,
        81,
        128,
        130,
        1,
        130,
        82,
        96,
        18,
        129,
        82,
        127,
        70,
        105,
        120,
        101,
        100,
        32,
        112,
        114,
        105,
        99,
        101,
        32,
        111,
        114,
        97,
        99,
        108,
        101,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        96,
        32,
        130,
        1,
        82,
        144,
        81,
        97,
        0,
        161,
        145,
        144,
        97,
        1,
        150,
        86,
        91,
        97,
        1,
        52,
        97,
        1,
        6,
        54,
        96,
        4,
        97,
        2,
        2,
        86,
        91,
        96,
        0,
        127,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        129,
        66,
        129,
        147,
        149,
        144,
        146,
        148,
        80,
        86,
        91,
        96,
        64,
        128,
        81,
        105,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        150,
        135,
        22,
        129,
        82,
        96,
        32,
        129,
        1,
        149,
        144,
        149,
        82,
        132,
        1,
        146,
        144,
        146,
        82,
        96,
        96,
        131,
        1,
        82,
        144,
        145,
        22,
        96,
        128,
        130,
        1,
        82,
        96,
        160,
        1,
        97,
        0,
        161,
        86,
        91,
        96,
        0,
        127,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        129,
        66,
        129,
        97,
        1,
        52,
        86,
        91,
        96,
        0,
        96,
        32,
        128,
        131,
        82,
        131,
        81,
        128,
        130,
        133,
        1,
        82,
        96,
        0,
        91,
        129,
        129,
        16,
        21,
        97,
        1,
        195,
        87,
        133,
        129,
        1,
        131,
        1,
        81,
        133,
        130,
        1,
        96,
        64,
        1,
        82,
        130,
        1,
        97,
        1,
        167,
        86,
        91,
        80,
        96,
        0,
        96,
        64,
        130,
        134,
        1,
        1,
        82,
        96,
        64,
        127,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        224,
        96,
        31,
        131,
        1,
        22,
        133,
        1,
        1,
        146,
        80,
        80,
        80,
        146,
        145,
        80,
        80,
        86,
        91,
        96,
        0,
        96,
        32,
        130,
        132,
        3,
        18,
        21,
        97,
        2,
        20,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        53,
        105,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        129,
        22,
        129,
        20,
        97,
        2,
        46,
        87,
        96,
        0,
        128,
        253,
        91,
        147,
        146,
        80,
        80,
        80,
        86,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        237,
        51,
        85,
        223,
        1,
        158,
        3,
        238,
        108,
        39,
        245,
        236,
        75,
        218,
        175,
        14,
        71,
        208,
        31,
        107,
        94,
        128,
        26,
        149,
        88,
        190,
        241,
        207,
        238,
        36,
        52,
        9,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        16,
        0,
        51,
    ];
    ///The bytecode of the contract.
    pub static FIXEDPRICEAGGREGATOR_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        97,
        0,
        16,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        4,
        54,
        16,
        97,
        0,
        103,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        114,
        132,
        228,
        22,
        17,
        97,
        0,
        80,
        87,
        128,
        99,
        114,
        132,
        228,
        22,
        20,
        97,
        0,
        185,
        87,
        128,
        99,
        154,
        111,
        200,
        245,
        20,
        97,
        0,
        248,
        87,
        128,
        99,
        254,
        175,
        150,
        140,
        20,
        97,
        1,
        107,
        87,
        96,
        0,
        128,
        253,
        91,
        128,
        99,
        49,
        60,
        229,
        103,
        20,
        97,
        0,
        108,
        87,
        128,
        99,
        84,
        253,
        77,
        80,
        20,
        97,
        0,
        170,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        97,
        0,
        147,
        127,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        129,
        86,
        91,
        96,
        64,
        81,
        96,
        255,
        144,
        145,
        22,
        129,
        82,
        96,
        32,
        1,
        91,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        243,
        91,
        96,
        64,
        81,
        96,
        0,
        129,
        82,
        96,
        32,
        1,
        97,
        0,
        161,
        86,
        91,
        96,
        64,
        128,
        81,
        128,
        130,
        1,
        130,
        82,
        96,
        18,
        129,
        82,
        127,
        70,
        105,
        120,
        101,
        100,
        32,
        112,
        114,
        105,
        99,
        101,
        32,
        111,
        114,
        97,
        99,
        108,
        101,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        96,
        32,
        130,
        1,
        82,
        144,
        81,
        97,
        0,
        161,
        145,
        144,
        97,
        1,
        150,
        86,
        91,
        97,
        1,
        52,
        97,
        1,
        6,
        54,
        96,
        4,
        97,
        2,
        2,
        86,
        91,
        96,
        0,
        127,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        129,
        66,
        129,
        147,
        149,
        144,
        146,
        148,
        80,
        86,
        91,
        96,
        64,
        128,
        81,
        105,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        150,
        135,
        22,
        129,
        82,
        96,
        32,
        129,
        1,
        149,
        144,
        149,
        82,
        132,
        1,
        146,
        144,
        146,
        82,
        96,
        96,
        131,
        1,
        82,
        144,
        145,
        22,
        96,
        128,
        130,
        1,
        82,
        96,
        160,
        1,
        97,
        0,
        161,
        86,
        91,
        96,
        0,
        127,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        129,
        66,
        129,
        97,
        1,
        52,
        86,
        91,
        96,
        0,
        96,
        32,
        128,
        131,
        82,
        131,
        81,
        128,
        130,
        133,
        1,
        82,
        96,
        0,
        91,
        129,
        129,
        16,
        21,
        97,
        1,
        195,
        87,
        133,
        129,
        1,
        131,
        1,
        81,
        133,
        130,
        1,
        96,
        64,
        1,
        82,
        130,
        1,
        97,
        1,
        167,
        86,
        91,
        80,
        96,
        0,
        96,
        64,
        130,
        134,
        1,
        1,
        82,
        96,
        64,
        127,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        224,
        96,
        31,
        131,
        1,
        22,
        133,
        1,
        1,
        146,
        80,
        80,
        80,
        146,
        145,
        80,
        80,
        86,
        91,
        96,
        0,
        96,
        32,
        130,
        132,
        3,
        18,
        21,
        97,
        2,
        20,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        53,
        105,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        129,
        22,
        129,
        20,
        97,
        2,
        46,
        87,
        96,
        0,
        128,
        253,
        91,
        147,
        146,
        80,
        80,
        80,
        86,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        237,
        51,
        85,
        223,
        1,
        158,
        3,
        238,
        108,
        39,
        245,
        236,
        75,
        218,
        175,
        14,
        71,
        208,
        31,
        107,
        94,
        128,
        26,
        149,
        88,
        190,
        241,
        207,
        238,
        36,
        52,
        9,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        16,
        0,
        51,
    ];
    ///The deployed bytecode of the contract.
    pub static FIXEDPRICEAGGREGATOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct FixedPriceAggregator<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for FixedPriceAggregator<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for FixedPriceAggregator<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for FixedPriceAggregator<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for FixedPriceAggregator<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(FixedPriceAggregator))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> FixedPriceAggregator<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers_contract::Contract::new(
                address.into(),
                FIXEDPRICEAGGREGATOR_ABI.clone(),
                client,
            ))
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers_contract::builders::ContractDeployer<M, Self>,
            ::ethers_contract::ContractError<M>,
        > {
            let factory = ::ethers_contract::ContractFactory::new(
                FIXEDPRICEAGGREGATOR_ABI.clone(),
                FIXEDPRICEAGGREGATOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers_contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers_contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `description` (0x7284e416) function
        pub fn description(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([114, 132, 228, 22], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoundData` (0x9a6fc8f5) function
        pub fn get_round_data(
            &self,
            round_id: u128,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            (
                u128,
                ::ethers::core::types::I256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                u128,
            ),
        > {
            self.0
                .method_hash([154, 111, 200, 245], round_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestRoundData` (0xfeaf968c) function
        pub fn latest_round_data(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            (
                u128,
                ::ethers::core::types::I256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                u128,
            ),
        > {
            self.0
                .method_hash([254, 175, 150, 140], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `version` (0x54fd4d50) function
        pub fn version(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([84, 253, 77, 80], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers_contract::Contract<M>>
        for FixedPriceAggregator<M>
    {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    ///Container type for all input parameters for the `description` function with signature `description()` and selector `0x7284e416`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "description", abi = "description()")]
    pub struct DescriptionCall;
    ///Container type for all input parameters for the `getRoundData` function with signature `getRoundData(uint80)` and selector `0x9a6fc8f5`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getRoundData", abi = "getRoundData(uint80)")]
    pub struct GetRoundDataCall {
        pub round_id: u128,
    }
    ///Container type for all input parameters for the `latestRoundData` function with signature `latestRoundData()` and selector `0xfeaf968c`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "latestRoundData", abi = "latestRoundData()")]
    pub struct LatestRoundDataCall;
    ///Container type for all input parameters for the `version` function with signature `version()` and selector `0x54fd4d50`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "version", abi = "version()")]
    pub struct VersionCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum FixedPriceAggregatorCalls {
        Decimals(DecimalsCall),
        Description(DescriptionCall),
        GetRoundData(GetRoundDataCall),
        LatestRoundData(LatestRoundDataCall),
        Version(VersionCall),
    }
    impl ::ethers::core::abi::AbiDecode for FixedPriceAggregatorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded) = <DescriptionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Description(decoded));
            }
            if let Ok(decoded) = <GetRoundDataCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetRoundData(decoded));
            }
            if let Ok(decoded) =
                <LatestRoundDataCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LatestRoundData(decoded));
            }
            if let Ok(decoded) = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Version(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FixedPriceAggregatorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Decimals(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Description(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetRoundData(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LatestRoundData(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for FixedPriceAggregatorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::Description(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoundData(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestRoundData(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DecimalsCall> for FixedPriceAggregatorCalls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<DescriptionCall> for FixedPriceAggregatorCalls {
        fn from(value: DescriptionCall) -> Self {
            Self::Description(value)
        }
    }
    impl ::core::convert::From<GetRoundDataCall> for FixedPriceAggregatorCalls {
        fn from(value: GetRoundDataCall) -> Self {
            Self::GetRoundData(value)
        }
    }
    impl ::core::convert::From<LatestRoundDataCall> for FixedPriceAggregatorCalls {
        fn from(value: LatestRoundDataCall) -> Self {
            Self::LatestRoundData(value)
        }
    }
    impl ::core::convert::From<VersionCall> for FixedPriceAggregatorCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    ///Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DecimalsReturn(pub u8);
    ///Container type for all return fields from the `description` function with signature `description()` and selector `0x7284e416`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DescriptionReturn(pub ::std::string::String);
    ///Container type for all return fields from the `getRoundData` function with signature `getRoundData(uint80)` and selector `0x9a6fc8f5`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetRoundDataReturn {
        pub round_id: u128,
        pub answer: ::ethers::core::types::I256,
        pub started_at: ::ethers::core::types::U256,
        pub updated_at: ::ethers::core::types::U256,
        pub answered_in_round: u128,
    }
    ///Container type for all return fields from the `latestRoundData` function with signature `latestRoundData()` and selector `0xfeaf968c`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LatestRoundDataReturn {
        pub round_id: u128,
        pub answer: ::ethers::core::types::I256,
        pub started_at: ::ethers::core::types::U256,
        pub updated_at: ::ethers::core::types::U256,
        pub answered_in_round: u128,
    }
    ///Container type for all return fields from the `version` function with signature `version()` and selector `0x54fd4d50`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct VersionReturn(pub ::ethers::core::types::U256);
}
