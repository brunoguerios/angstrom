use std::{pin::Pin, sync::Arc};

use alloy::{
    primitives::{Address, BlockNumber},
    providers::Provider
};
use angstrom_eth::manager::EthEvent;
use angstrom_types::block_sync::BlockSyncConsumer;
use futures::Stream;
use reth_provider::CanonStateNotifications;

use crate::balancer::{
    pool_factory::V3PoolFactory, pool_providers::canonical_state_adapter::CanonicalStateAdapter
};

pub mod balancer;
pub use balancer::pool_manager::{BalancerPoolManager, SyncedBalancerPools};

pub async fn configure_balancer_manager<P, BlockSync>(
    provider: Arc<P>,
    state_notification: CanonStateNotifications,
    current_block: BlockNumber,
    block_sync: BlockSync,
    angstrom_controller: Address, /* TODO: review name/address - this should likely be the
                                   * ReClamm factory address */
    update_stream: Pin<Box<dyn Stream<Item = EthEvent> + Send + Sync>>
) -> BalancerPoolManager<P, CanonicalStateAdapter<P>, BlockSync>
where
    P: Provider + 'static,
    BlockSync: BlockSyncConsumer
{
    let factory = V3PoolFactory::new(provider.clone(), angstrom_controller);

    let notifier =
        Arc::new(CanonicalStateAdapter::new(state_notification, provider.clone(), current_block));

    BalancerPoolManager::new(factory, current_block, notifier, block_sync, update_stream).await
}
