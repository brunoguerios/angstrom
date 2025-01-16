use alloy::{contract::RawCallBuilder, primitives::Address, sol_types::SolValue};
use alloy_sol_types::SolCall;
use angstrom_types::contract_bindings::angstrom::Angstrom;

use super::{
    mine_address, mine_create3_address, uniswap_flags::UniswapFlags, DEFAULT_CREATE2_FACTORY,
    SUB_ZERO_FACTORY
};

pub async fn deploy_angstrom<
    T: alloy::contract::private::Transport + ::core::clone::Clone,
    P: alloy::contract::private::Provider<T, N>,
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
    P: alloy::contract::private::Provider<T, N>,
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
    P: alloy::contract::private::Provider<T, N>,
    N: alloy::contract::private::Network
>(
    provider: &P,
    owner: Address,
    pool_manager: Address,
    controller: Address
) -> Address {
    let mock_builder = Angstrom::deploy_builder(&provider, pool_manager, controller);
    let (mock_tob_address, salt, nonce) = mine_create3_address(owner);

    let mint_call = _private::mintCall { to: owner, id: salt, nonce };
    RawCallBuilder::new_raw(&provider, mint_call.abi_encode().into())
        .to(SUB_ZERO_FACTORY)
        .gas(50e6 as u64)
        .send()
        .await
        .unwrap()
        .watch()
        .await
        .unwrap();

    let final_mock_initcode = [salt.abi_encode(), mock_builder.calldata().to_vec()].concat();
    let deploy_call = _private::deployCall { id: salt, initcode: final_mock_initcode.into() };
    RawCallBuilder::new_raw(&provider, deploy_call.abi_encode().into())
        .to(SUB_ZERO_FACTORY)
        .gas(50e6 as u64)
        .send()
        .await
        .unwrap()
        .watch()
        .await
        .unwrap();
    mock_tob_address
}

mod _private {
    use alloy::sol;

    sol! {
        function mint(address to, uint256 id, uint8 nonce);

        function deploy(uint256 id, bytes initcode) returns (address);
    }
}
