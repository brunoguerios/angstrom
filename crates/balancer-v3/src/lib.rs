use std::{collections::HashSet, pin::Pin, sync::Arc};

use alloy::{
    consensus::TxReceipt,
    primitives::{Address, BlockNumber},
    sol_types::SolEvent
};
use angstrom_eth::manager::EthEvent;
use angstrom_types::{
    block_sync::BlockSyncConsumer,
    contract_bindings::balancer_controller::BalancerController::{
        BalancerPoolConfigured, BalancerPoolRemoved
    }
};
use futures::Stream;
use reth_provider::{
    CanonStateNotifications, DatabaseProviderFactory, ReceiptProvider,
    TryIntoHistoricalStateProvider
};

use crate::balancer::{
    pool_factory::V3PoolFactory, pool_providers::canonical_state_adapter::CanonicalStateAdapter
};

pub mod balancer;
pub use balancer::pool_manager::{BalancerPoolManager, SyncedBalancerPools};

/// Fetches all Balancer pools from historical event logs
/// Returns a tuple of (pool_address, token0, token1) for each configured pool
pub async fn fetch_balancer_pools<DB>(
    deploy_block: usize,
    end_block: usize,
    balancer_controller: Address,
    db: &DB
) -> Vec<(Address, Address, Address)>
where
    DB: DatabaseProviderFactory + ReceiptProvider,
    <DB as DatabaseProviderFactory>::Provider: TryIntoHistoricalStateProvider
{
    let logs = (deploy_block..=end_block)
        .flat_map(|block| {
            db.receipts_by_block((block as u64).into())
                .unwrap()
                .unwrap_or_default()
                .into_iter()
                .flat_map(|receipt| receipt.logs().to_vec())
                .filter(move |log| log.address == balancer_controller)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    logs.into_iter()
        .fold(HashSet::new(), |mut set, log| {
            if let Ok(configured) = BalancerPoolConfigured::decode_log(&log) {
                set.insert((configured.poolAddress, configured.token0, configured.token1));
                return set;
            }

            if let Ok(removed) = BalancerPoolRemoved::decode_log(&log) {
                set.retain(|(addr, ..)| *addr != removed.poolAddress);
                return set;
            }
            set
        })
        .into_iter()
        .collect::<Vec<_>>()
}

pub async fn configure_balancer_manager<P, BlockSync>(
    provider: Arc<P>,
    state_notification: CanonStateNotifications,
    current_block: BlockNumber,
    block_sync: BlockSync,
    balancer_controller: Address,
    discovered_pools: Vec<(Address, Address, Address)>, // (pool_address, token0, token1)
    update_stream: Pin<Box<dyn Stream<Item = EthEvent> + Send + Sync>>
) -> BalancerPoolManager<P, CanonicalStateAdapter<P>, BlockSync>
where
    P: alloy::providers::Provider + 'static,
    BlockSync: BlockSyncConsumer
{
    let factory = V3PoolFactory::new(provider.clone(), balancer_controller, discovered_pools);

    let notifier =
        Arc::new(CanonicalStateAdapter::new(state_notification, provider.clone(), current_block));

    BalancerPoolManager::new(factory, current_block, notifier, block_sync, update_stream).await
}
