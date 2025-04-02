use std::sync::Arc;

use alloy::{
    primitives::{Address, U256},
    providers::{Provider, ProviderBuilder},
    sol_types::{SolCall, SolValue}
};
use angstrom_types::{
    contract_bindings::mintable_mock_erc_20::MintableMockERC20::MintableMockERC20Instance,
    contract_payloads::angstrom::UniswapAngstromRegistry,
    primitive::{AngstromSigner, ERC20Calls, PoolId, UniswapPoolRegistry}
};
use uniswap_v4::{
    fetch_angstrom_pools,
    uniswap::{
        pool::EnhancedUniswapPool,
        pool_data_loader::{DataLoader, PoolDataLoader},
        pool_manager::SyncedUniswapPools
    }
};

use crate::cli::BundleLander;

pub struct BundleWashTraderEnv {
    pub keys:     Vec<AngstromSigner>,
    pub provider: Arc<Box<dyn Provider>>,
    pub pools:    Vec<EnhancedUniswapPool>
}

impl BundleWashTraderEnv {
    pub async fn init(cli: &BundleLander) -> eyre::Result<Self> {
        let provider = Arc::new(Box::new(
            ProviderBuilder::<_, _, _>::default()
                .with_recommended_fillers()
                // backup
                .connect(&cli.node_endpoint.as_str())
                .await
                .unwrap()
        )) as Arc<Box<dyn Provider>>;

        let block = provider.get_block_number().await.unwrap();

        let pools =
            fetch_angstrom_pools(7838402, block as usize, cli.angstrom_address, todo!()).await;
        let uniswap_registry: UniswapPoolRegistry = pools.into();

        let pool_config_store = Arc::new(
            AngstromPoolConfigStore::load_from_chain(
                node_config.angstrom_address,
                BlockId::Number(BlockNumberOrTag::Latest),
                &querying_provider
            )
            .await
            .unwrap()
        );
        let uni_ang_registry =
            UniswapAngstromRegistry::new(uniswap_registry.clone(), pool_config_store.clone());
        let mut ang_pools = Vec::new();
        for pool in pools {
            let data_loader = DataLoader::new_with_registry(address, registry, pool_manager);
            let mut pool = EnhancedUniswapPool::new(data_loader, 200);
            pool.initialize(None, provider).await;
            ang_pools.push(pool);
        }

        let keys = cli
            .testing_private_keys
            .iter()
            .map(|key| AngstromSigner::new(key.parse().unwrap()))
            .collect();

        Self { keys, provider, pools: ang_pools }
    }

    async fn approve_max_tokens_to_angstrom(
        angstrom: Address,
        tokens: Vec<Address>,
        users: &[AngstromSigner],
        provider: Arc<Box<dyn Provider>>
    ) -> eyre::Result<()> {
        for token in tokens {
            let instance = MintableMockERC20Instance::new(token, provider.root());
            for user in users {
                let raw_tx = instance
                    .approve(angstrom, U256::MAX)
                    .from(user.address())
                    .build_unsigned_raw_transaction()
                    .unwrap()
                    .await
                    .unwrap();
            }
            // let tx_builder =
        }

        Ok(())
    }
}
