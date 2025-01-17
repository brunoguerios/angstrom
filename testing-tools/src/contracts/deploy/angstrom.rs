use alloy::{
    contract::RawCallBuilder,
    network::Ethereum,
    primitives::Address,
    providers::ext::DebugApi,
    rpc::types::trace::geth::{
        GethDebugBuiltInTracerType, GethDebugTracerType, GethDebugTracingOptions,
        GethDefaultTracingOptions
    },
    sol_types::SolValue
};
use alloy_sol_types::SolCall;
use angstrom_types::contract_bindings::angstrom::Angstrom;

use super::{
    mine_address, mine_create3_address, uniswap_flags::UniswapFlags, DEFAULT_CREATE2_FACTORY,
    SUB_ZERO_FACTORY
};

pub async fn deploy_angstrom<
    T: alloy::contract::private::Transport + ::core::clone::Clone,
    P: alloy::contract::private::Provider<T, N> + alloy::providers::WalletProvider<N>,
    N: alloy::contract::private::Network
>(
    provider: &P,
    pool_manager: Address,
    controller: Address,
    feemaster: Address
) -> Address {
    deploy_angstrom_with_factory(
        provider,
        pool_manager,
        DEFAULT_CREATE2_FACTORY,
        controller,
        feemaster
    )
    .await
}

pub async fn deploy_angstrom_with_factory<
    T: alloy::contract::private::Transport + ::core::clone::Clone,
    P: alloy::contract::private::Provider<T, N> + alloy::providers::WalletProvider<N>,
    N: alloy::contract::private::Network
>(
    provider: &P,
    pool_manager: Address,
    factory: Address,
    controller: Address,
    _feemaster: Address
) -> Address {
    // Setup our flags and mask
    let flags = UniswapFlags::BeforeSwap
        | UniswapFlags::BeforeInitialize
        | UniswapFlags::BeforeAddLiquidity
        | UniswapFlags::BeforeRemoveLiquidity;

    let mock_builder = Angstrom::deploy_builder(&provider, pool_manager, controller);
    let (mock_tob_address, salt) =
        mine_address(flags, UniswapFlags::mask(), mock_builder.calldata());
    let final_mock_initcode = [salt.abi_encode(), mock_builder.calldata().to_vec()].concat();
    RawCallBuilder::new_raw(&provider, final_mock_initcode.into())
        .to(factory)
        .gas(50e6 as u64)
        .send()
        .await
        .unwrap()
        .watch()
        .await
        .unwrap();
    mock_tob_address
}

pub async fn deploy_angstrom_create3<
    T: alloy::contract::private::Transport + ::core::clone::Clone,
    P: alloy::contract::private::Provider<T, Ethereum> + alloy::providers::WalletProvider<Ethereum>
>(
    provider: &P,
    pool_manager: Address,
    controller: Address
) -> Address {
    let owner = provider.default_signer_address();
    // let mock_builder = Angstrom::deploy_builder(&provider, pool_manager,
    // controller);

    let mut code = Angstrom::BYTECODE.to_vec();
    code.append(&mut (pool_manager, controller).abi_encode().to_vec());

    let (mock_tob_address, salt, nonce) = mine_create3_address(owner);

    let mint_call = _private::mintCall { to: owner, id: salt, nonce };

    let hash = RawCallBuilder::new_raw(&provider, mint_call.abi_encode().into())
        .to(SUB_ZERO_FACTORY)
        .from(owner)
        .gas(50e6 as u64)
        .send()
        .await
        .unwrap()
        .watch()
        .await
        .unwrap();

    let deploy_call = _private::deployCall { id: salt, initcode: code.into() };

    let hash = RawCallBuilder::new_raw(&provider, deploy_call.abi_encode().into())
        .from(owner)
        .gas(50e6 as u64)
        .to(SUB_ZERO_FACTORY)
        .gas(50e6 as u64)
        .send()
        .await
        .unwrap()
        .watch()
        .await
        .unwrap();

    let receipt = provider
        .get_transaction_receipt(hash)
        .await
        .unwrap()
        .unwrap();

    // let default_options = GethDebugTracingOptions::default();
    let call_options = GethDebugTracingOptions {
        config: GethDefaultTracingOptions {
            disable_storage: Some(false),
            enable_memory: Some(true),
            debug: Some(true),
            disable_stack: Some(false),
            ..Default::default()
        },
        tracer: Some(GethDebugTracerType::BuiltInTracer(GethDebugBuiltInTracerType::CallTracer)),
        ..Default::default()
    };
    let result = provider
        .debug_trace_transaction(receipt.transaction_hash, call_options)
        .await
        .unwrap();

    tracing::warn!("TRACE: {result:?}");
    // We can make this do a cool backtrace later

    mock_tob_address
}

mod _private {
    use alloy::sol;

    sol! {
        function mint(address to, uint256 id, uint8 nonce);

        function deploy(uint256 id, bytes initcode) returns (address);
    }
}
