use alloy::{
    network::{Ethereum, EthereumWallet},
    node_bindings::AnvilInstance,
    providers::{ext::AnvilApi, PendingTransaction},
    pubsub::PubSubFrontend,
    signers::local::PrivateKeySigner
};
use alloy_primitives::{
    aliases::{I24, U24},
    Address, FixedBytes, U256
};
use angstrom_types::{
    contract_bindings::{
        angstrom::Angstrom::{AngstromInstance, PoolKey},
        pool_gate::PoolGate::PoolGateInstance
    },
    matching::{uniswap::LiqRange, SqrtPriceX96}
};

use super::{utils::AnvilWalletRpc, AnvilWallet};
use crate::{
    contracts::{
        deploy::tokens::mint_token_pair,
        environment::{
            angstrom::AngstromEnv,
            uniswap::{TestUniswapEnv, UniswapEnv},
            TestAnvilEnvironment
        },
        DebugTransaction
    },
    testnet_controllers::AngstromTestnetConfig,
    type_generator::amm::AMMSnapshotBuilder,
    types::initial_state::InitialTestnetState
};

const ANVIL_TESTNET_DEPLOYMENT_ENDPOINT: &'static str = "anvil_temp_deploy";

pub struct AnvilTestnetIntializer {
    provider:     AnvilWallet,
    // uniswap_env:  UniswapEnv<AnvilWallet>,
    angstrom_env: AngstromEnv<UniswapEnv<AnvilWallet>>,
    angstrom:     AngstromInstance<PubSubFrontend, AnvilWalletRpc>,
    pool_gate:    PoolGateInstance<PubSubFrontend, AnvilWalletRpc>,
    _instance:    AnvilInstance
}

impl AnvilTestnetIntializer {
    pub async fn new(config: AngstromTestnetConfig) -> eyre::Result<Self> {
        let anvil = config
            .configure_anvil(ANVIL_TESTNET_DEPLOYMENT_ENDPOINT)
            .try_spawn()
            .unwrap();

        let ipc = alloy::providers::IpcConnect::new(format!(
            "/tmp/{ANVIL_TESTNET_DEPLOYMENT_ENDPOINT}.ipc"
        ));
        let sk: PrivateKeySigner = anvil.keys()[config.anvil_key].clone().into();
        let controller_address = anvil.addresses()[config.anvil_key];

        let wallet = EthereumWallet::new(sk.clone());
        let rpc = alloy::providers::builder::<Ethereum>()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_ipc(ipc)
            .await
            .unwrap();

        let wallet_provider = AnvilWallet::new(rpc, controller_address, sk);

        let uniswap_env = UniswapEnv::new(wallet_provider.clone()).await.unwrap();
        let angstrom_env = AngstromEnv::new(uniswap_env).await.unwrap();
        let angstrom =
            AngstromInstance::new(angstrom_env.angstrom(), angstrom_env.provider().clone());
        let pool_gate =
            PoolGateInstance::new(angstrom_env.pool_gate(), angstrom_env.provider().clone());

        Ok(Self {
            provider: wallet_provider,
            //uniswap_env,
            angstrom_env,
            angstrom,
            pool_gate,
            _instance: anvil
        })
    }

    /// deploys tokens, a uniV4 pool, angstrom pool
    pub async fn deploy_pool_full(&self) -> eyre::Result<Vec<PendingTransaction>> {
        let (currency0, currency1) = mint_token_pair(self.provider.provider_ref()).await;

        let fee = U24::ZERO;
        let tick_spacing = 10;
        let pool = PoolKey {
            currency0,
            currency1,
            fee,
            tickSpacing: I24::unchecked_from(tick_spacing),
            hooks: Address::default()
        };

        let liquidity = 1_000_000_000_000_000_u128;
        let price = SqrtPriceX96::at_tick(100000)?;
        let amm = AMMSnapshotBuilder::new(price)
            .with_positions(vec![LiqRange::new(99000, 101000, liquidity)?])
            .build();

        // self.angstrom
        //     .configurePool(pool.currency0, pool.currency1, 10, U24::ZERO)
        //     .from(self.provider.controller_address)
        //     .run_safe()
        //     .await
        //     .unwrap();
        // self.angstrom
        //     .initializePool(pool.currency0, pool.currency1, U256::ZERO, *price)
        //     .run_safe()
        //     .await
        //     .unwrap();

        let init_pool = self
            .angstrom
            .initializePool(pool.currency0, pool.currency1, U256::ZERO, *price)
            .gas(50_000_000_u64)
            .send()
            .await?
            .register()
            .await?;

        let add_liq = self
            .pool_gate
            .addLiquidity(
                pool.currency0,
                pool.currency1,
                I24::unchecked_from(99000),
                I24::unchecked_from(101000),
                U256::from(liquidity),
                FixedBytes::<32>::default()
            )
            .from(self.provider.controller_address)
            .gas(50_000_000_u64)
            .send()
            .await?
            .register()
            .await?;

        Ok(vec![init_pool, add_liq])
    }

    pub async fn initialize(self) -> eyre::Result<InitialTestnetState> {
        let state_bytes = self.provider.provider_ref().anvil_dump_state().await?;
        Ok(InitialTestnetState::new(self.angstrom_env.angstrom(), state_bytes))
    }
}

#[cfg(test)]
mod tests {
    use alloy::providers::Provider;

    use super::*;

    async fn get_block(provider: &AnvilWallet) -> eyre::Result<u64> {
        Ok(provider.provider.get_block_number().await?)
    }

    #[tokio::test]
    async fn test_can_deploy() {
        let config = AngstromTestnetConfig::default();
        let initializer = AnvilTestnetIntializer::new(config).await.unwrap();

        // let current_block = get_block(&initializer.provider).await.unwrap();

        // let txs = initializer.deploy_pool_full().await.unwrap();

        // assert_eq!(current_block,
        // get_block(&initializer.provider).await.unwrap());

        // let block_mine =
        // initializer.provider.provider.evm_mine(None).await.unwrap();

        // println!("BLOCK MINED: {block_mine}");

        // assert_eq!(current_block + 1,
        // get_block(&initializer.provider).await.unwrap());

        // for tx in futures::future::join_all(txs).await {
        //     let tx = tx.unwrap();
        //     println!("MINED - {tx:?}");
        // }
    }
}
