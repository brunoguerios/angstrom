use std::sync::Arc;

use alloy::{
    eips::Encodable2718,
    network::TransactionBuilder,
    primitives::{Address, Bytes, U256},
    providers::{Provider, ProviderBuilder},
    sol_types::{SolCall, SolValue}
};
use alloy_rpc_types::{BlockId, BlockNumberOrTag, TransactionRequest};
use angstrom_types::{
    contract_bindings::mintable_mock_erc_20::MintableMockERC20::MintableMockERC20Instance,
    contract_payloads::angstrom::{AngstromPoolConfigStore, UniswapAngstromRegistry},
    primitive::{AngstromSigner, ERC20Calls, PoolId, UniswapPoolRegistry}
};
use futures::stream::FuturesUnordered;
use itertools::Itertools;
use uniswap_v4::{
    fetch_angstrom_pools,
    uniswap::{
        pool::EnhancedUniswapPool,
        pool_data_loader::{DataLoader, PoolDataLoader},
        pool_manager::SyncedUniswapPools
    }
};

use crate::{approveCall, cli::BundleLander};

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
                cli.angstrom_address,
                BlockId::Number(BlockNumberOrTag::Latest),
                &provider.root()
            )
            .await
            .unwrap()
        );
        let uni_ang_registry =
            UniswapAngstromRegistry::new(uniswap_registry.clone(), pool_config_store.clone());
        let mut ang_pools = Vec::new();

        for pool in pools {
            let data_loader = DataLoader::new_with_registry(
                todo!("private key"),
                uniswap_registry,
                cli.pool_manager_address
            );
            let mut pool = EnhancedUniswapPool::new(data_loader, 200);
            pool.initialize(None, provider.root().into()).await;
            ang_pools.push(pool);
        }

        let keys = cli
            .testing_private_keys
            .iter()
            .map(|key| AngstromSigner::new(key.parse().unwrap()))
            .collect::<Vec<_>>();

        let all_tokens = ang_pools
            .iter()
            .flat_map(|p| [p.token0, p.token1])
            .unique()
            .collect::<Vec<_>>();

        Self::approve_max_tokens_to_angstrom(cli.angstrom_address, all_tokens, &keys, provider)
            .await?;

        Ok(Self { keys, provider, pools: ang_pools })
    }

    async fn approve_max_tokens_to_angstrom(
        angstrom: Address,
        tokens: Vec<Address>,
        users: &[AngstromSigner],
        provider: Arc<Box<dyn Provider>>
    ) -> eyre::Result<()> {
        let mut nonce_offset = 0;
        let pending_orders = FuturesUnordered::new();

        let fees = provider.estimate_eip1559_fees().await.unwrap();
        let chain_id = provider.get_chain_id().await.unwrap();

        for token in tokens {
            for user in users {
                let call: Bytes = approveCall::new((angstrom, U256::MAX)).abi_encode().into();

                let mut tx = TransactionRequest::default()
                    .with_from(user.address())
                    .with_kind(alloy_primitives::TxKind::Call(token))
                    .with_input(call);

                let next_nonce = provider
                    .get_transaction_count(user.address())
                    .await
                    .unwrap();

                tx.set_nonce(next_nonce + nonce_offset);
                tx.set_gas_limit(30_000_000);
                tx.set_max_fee_per_gas(fees.max_fee_per_gas);
                tx.set_max_priority_fee_per_gas(fees.max_priority_fee_per_gas);

                tx.set_chain_id(chain_id);
                let signed_tx = tx.build(user).await.unwrap();

                let encoded_tx = signed_tx.encoded_2718();
                pending_orders.push(
                    provider
                        .send_raw_transaction(&encoded_tx)
                        .await
                        .unwrap()
                        .watch()
                );
            }
            // with a new token, the nonce will increase
            nonce_offset += 1;
        }

        Ok(())
    }
}
