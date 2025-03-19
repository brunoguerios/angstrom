use std::{collections::HashSet, pin::Pin, sync::Arc};

use alloy::{
    consensus::TxReceipt, primitives::aliases::I24, providers::Provider, sol_types::SolEvent,
};
use alloy_primitives::{Address, BlockNumber, FixedBytes};
use angstrom_eth::manager::EthEvent;
use angstrom_types::{
    block_sync::BlockSyncConsumer,
    contract_bindings::{
        angstrom::Angstrom::PoolKey,
        controller_v_1::ControllerV1::{PoolConfigured, PoolRemoved},
    },
    primitive::UniswapPoolRegistry,
};
use futures::Stream;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use reth_provider::{
    CanonStateNotifications, DatabaseProviderFactory, ReceiptProvider, StateProvider,
    TryIntoHistoricalStateProvider,
};
use uniswap::pool_factory::V4PoolFactory;

use crate::uniswap::{
    pool_data_loader::DataLoader, pool_manager::UniswapPoolManager,
    pool_providers::canonical_state_adapter::CanonicalStateAdapter,
};

/// This module should have information on all the Constant Function Market
/// Makers that we work with.  Right now that's only Uniswap, but if there are
/// ever any others they will be added here
pub mod uniswap;

///  name           type    slot   offset  bytes    contract
/// _controller | address| 0    | 0      | 20    | src/Angstrom.sol:Angstrom |
/// We use this so that we are able to historically go back from the current
/// block of angstrom and fetch all the new pool events. We look at the angstrom
/// contract for controller address as the control address is upgradable. This
/// means that this will be secure in the case of new deployments
const CONTROLLER_ADDRESS_SLOT: FixedBytes<32> = FixedBytes::<32>::ZERO;

/// Goes from the deploy block to the current block fetching all pools.
pub async fn fetch_angstrom_pools<DB>(
    // the block angstrom was deployed at
    deploy_block: usize,
    end_block: usize,
    angstrom_address: Address,
    db: &DB,
) -> Vec<PoolKey>
where
    DB: DatabaseProviderFactory + ReceiptProvider,
    <DB as DatabaseProviderFactory>::Provider: TryIntoHistoricalStateProvider,
{
    let logs = (deploy_block..=end_block)
        .into_iter()
        .flat_map(|block| {
            let storage_provider = db
                .database_provider_ro()
                .unwrap()
                .try_into_history_at_block(block as u64)
                .unwrap();

            let controller_addr = Address::from_word(FixedBytes::new(
                storage_provider
                    .storage(angstrom_address, CONTROLLER_ADDRESS_SLOT)
                    .unwrap()
                    .unwrap()
                    .to_be_bytes::<32>(),
            ));

            db.receipts_by_block((block as u64).into())
                .unwrap()
                .unwrap_or_default()
                .into_iter()
                .flat_map(|receipt| receipt.logs().to_vec())
                .filter(move |log| log.address == controller_addr)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    logs.into_iter()
        .fold(HashSet::new(), |mut set, log| {
            if let Ok(pool) = PoolConfigured::decode_log(&log, true) {
                let pool_key = PoolKey {
                    currency0: pool.asset0,
                    currency1: pool.asset1,
                    fee: pool.bundleFee,
                    tickSpacing: I24::try_from_be_slice(&{
                        let bytes = pool.tickSpacing.to_be_bytes();
                        let mut a = [0u8; 3];
                        a[1..3].copy_from_slice(&bytes);
                        a
                    })
                    .unwrap(),
                    hooks: angstrom_address,
                };

                set.insert(pool_key);
                return set;
            }

            if let Ok(pool) = PoolRemoved::decode_log(&log, true) {
                let pool_key = PoolKey {
                    currency0: pool.asset0,
                    currency1: pool.asset1,
                    fee: pool.feeInE6,
                    tickSpacing: pool.tickSpacing,
                    hooks: angstrom_address,
                };

                set.remove(&pool_key);
                return set;
            }
            set
        })
        .into_iter()
        .collect::<Vec<_>>()
}

pub async fn configure_uniswap_manager<BlockSync: BlockSyncConsumer>(
    provider: Arc<impl Provider + 'static>,
    state_notification: CanonStateNotifications,
    uniswap_pool_registry: UniswapPoolRegistry,
    current_block: BlockNumber,
    block_sync: BlockSync,
    pool_manager_address: Address,
    update_stream: Pin<Box<dyn Stream<Item = EthEvent> + Send + Sync>>,
) -> UniswapPoolManager<
    CanonicalStateAdapter<impl Provider + 'static>,
    impl Provider + 'static,
    BlockSync,
> {
    let factory = V4PoolFactory::new(provider.clone(), uniswap_pool_registry, pool_manager_address);

    let notifier =
        Arc::new(CanonicalStateAdapter::new(state_notification, provider.clone(), current_block));

    UniswapPoolManager::new(factory, current_block, notifier, block_sync, update_stream).await
}
