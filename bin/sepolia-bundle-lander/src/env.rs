use std::{collections::HashSet, sync::Arc};

use alloy::{
    eips::Encodable2718,
    network::TransactionBuilder,
    primitives::{Address, Bytes, FixedBytes, U256, aliases::I24},
    providers::{
        Identity, Provider, ProviderBuilder, RootProvider,
        fillers::{BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller}
    },
    sol_types::{SolCall, SolEvent}
};
use alloy_rpc_types::TransactionRequest;
use angstrom_types::{
    contract_bindings::{
        angstrom::Angstrom::PoolKey,
        controller_v_1::ControllerV1::{PoolConfigured, PoolRemoved}
    },
    primitive::{AngstromSigner, UniswapPoolRegistry}
};
use futures::{StreamExt, stream::FuturesUnordered};
use itertools::Itertools;
use uniswap_v4::uniswap::{pool::EnhancedUniswapPool, pool_data_loader::DataLoader};

use crate::{
    approveCall,
    cli::{BundleLander, JsonPKs}
};

pub type ProviderType = FillProvider<
    JoinFill<
        Identity,
        JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>
    >,
    RootProvider
>;

pub struct BundleWashTraderEnv {
    pub keys:     Vec<AngstromSigner>,
    pub provider: Arc<ProviderType>,
    pub pools:    Vec<EnhancedUniswapPool>
}

impl BundleWashTraderEnv {
    pub async fn init(cli: &BundleLander, keys: JsonPKs) -> eyre::Result<Self> {
        let provider = Arc::new(
            ProviderBuilder::<_, _, _>::default()
                .with_recommended_fillers()
                // backup
                .connect(cli.node_endpoint.as_str())
                .await
                .unwrap()
        );

        let block = provider.get_block_number().await.unwrap();

        let pools =
            fetch_angstrom_pools(7838402, block as usize, cli.angstrom_address, &provider).await;

        let uniswap_registry: UniswapPoolRegistry = pools.into();

        let mut ang_pools = Vec::new();

        for pool_priv_key in uniswap_registry.private_keys() {
            let data_loader = DataLoader::new_with_registry(
                pool_priv_key,
                uniswap_registry.clone(),
                cli.pool_manager_address
            );
            let mut pool = EnhancedUniswapPool::new(data_loader, 200);
            pool.initialize(None, provider.root().into()).await?;
            ang_pools.push(pool);
        }

        let keys = keys
            .keys
            .iter()
            .map(|key| AngstromSigner::new(key.parse().unwrap()))
            .collect::<Vec<_>>();

        let all_tokens = ang_pools
            .iter()
            .flat_map(|p| [p.token0, p.token1])
            .unique()
            .collect::<Vec<_>>();

        Self::approve_max_tokens_to_angstrom(cli.angstrom_address, all_tokens, &keys, &provider)
            .await?;

        Ok(Self { keys, provider, pools: ang_pools })
    }

    async fn approve_max_tokens_to_angstrom(
        angstrom: Address,
        tokens: Vec<Address>,
        users: &[AngstromSigner],
        provider: &ProviderType
    ) -> eyre::Result<()> {
        let mut nonce_offset = 0;
        let mut pending_orders = FuturesUnordered::new();

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

        // wait and unwrap all orders.
        while let Some(next) = pending_orders.next().await {
            next?;
        }

        Ok(())
    }
}

const CONTROLLER_ADDRESS_SLOT: FixedBytes<32> = FixedBytes::<32>::ZERO;

async fn fetch_angstrom_pools<P>(
    // the block angstrom was deployed at
    deploy_block: usize,
    end_block: usize,
    angstrom_address: Address,
    db: &P
) -> Vec<PoolKey>
where
    P: Provider
{
    let logs = futures::stream::iter(deploy_block..=end_block)
        .map(|block| async move {
            let controller_addr = Address::from_word(FixedBytes::new(
                db.get_storage_at(angstrom_address, U256::from_be_bytes(*CONTROLLER_ADDRESS_SLOT))
                    .block_id((block as u64).into())
                    .await
                    .unwrap()
                    .to_be_bytes::<32>()
            ));

            db.get_block_receipts((block as u64).into())
                .await
                .unwrap()
                .unwrap_or_default()
                .into_iter()
                .flat_map(|receipt| receipt.logs().to_vec())
                .filter(move |log| log.address() == controller_addr)
                .collect::<Vec<_>>()
        })
        .buffered(30)
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    logs.into_iter()
        .fold(HashSet::new(), |mut set, log| {
            if let Ok(pool) = PoolConfigured::decode_log(&log.clone().into_inner(), true) {
                let pool_key = PoolKey {
                    currency0:   pool.asset0,
                    currency1:   pool.asset1,
                    fee:         pool.bundleFee,
                    tickSpacing: I24::try_from_be_slice(&{
                        let bytes = pool.tickSpacing.to_be_bytes();
                        let mut a = [0u8; 3];
                        a[1..3].copy_from_slice(&bytes);
                        a
                    })
                    .unwrap(),
                    hooks:       angstrom_address
                };

                set.insert(pool_key);
                return set;
            }

            if let Ok(pool) = PoolRemoved::decode_log(&log.clone().into_inner(), true) {
                let pool_key = PoolKey {
                    currency0:   pool.asset0,
                    currency1:   pool.asset1,
                    fee:         pool.feeInE6,
                    tickSpacing: pool.tickSpacing,
                    hooks:       angstrom_address
                };

                set.remove(&pool_key);
                return set;
            }
            set
        })
        .into_iter()
        .collect::<Vec<_>>()
}
