use std::{
    collections::HashMap,
    future::Future,
    pin::Pin,
    sync::{Arc, RwLock},
    task::Poll
};

use alloy::{primitives::BlockNumber, providers::Provider as AlloyProvider};
use angstrom_eth::manager::EthEvent;
use angstrom_types::{
    balancer_structure::BalancerPoolState, block_sync::BlockSyncConsumer, primitive::PoolId
};
use dashmap::DashMap;
use futures_util::{Stream, StreamExt, stream::BoxStream};

// intentionally no DB-bound imports; Balancer scaffold uses Alloy provider in factory
use super::{
    pool_factory::V3PoolFactory,
    pool_providers::{PoolManagerBlocks, PoolManagerProvider}
};

pub type SyncedBalancerPool = Arc<RwLock<BalancerPoolState>>;
type PoolMap = Arc<DashMap<PoolId, SyncedBalancerPool>>;

#[derive(Clone)]
pub struct SyncedBalancerPools {
    pools: PoolMap
}

impl SyncedBalancerPools {
    pub fn new(pools: PoolMap) -> Self {
        Self { pools }
    }

    pub fn iter(&self) -> dashmap::iter::Iter<'_, PoolId, SyncedBalancerPool> {
        self.pools.iter()
    }
}

pub struct BalancerPoolManager<P, Provider, BlockSync> {
    factory:             V3PoolFactory<P>,
    pools:               SyncedBalancerPools,
    latest_synced_block: u64,
    provider:            Arc<Provider>,
    block_sync:          BlockSync,
    block_stream:        BoxStream<'static, Option<PoolManagerBlocks>>,
    update_stream:       Pin<Box<dyn Stream<Item = EthEvent> + Send + Sync>>
}

impl<P, Provider, BlockSync> BalancerPoolManager<P, Provider, BlockSync>
where
    P: AlloyProvider + 'static,
    Provider: PoolManagerProvider + Send + Sync + 'static,
    BlockSync: BlockSyncConsumer
{
    pub async fn new(
        factory: V3PoolFactory<P>,
        latest_synced_block: BlockNumber,
        provider: Arc<Provider>,
        block_sync: BlockSync,
        update_stream: Pin<Box<dyn Stream<Item = EthEvent> + Send + Sync>>
    ) -> Self {
        block_sync.register("BalancerV3");

        let rwlock_pools: HashMap<PoolId, SyncedBalancerPool> = factory
            .init(latest_synced_block as u64)
            .await
            .into_iter()
            .map(|(pool_id, pool_state)| (pool_id, Arc::new(RwLock::new(pool_state))))
            .collect();

        let block_stream = provider.subscribe_blocks();

        Self {
            factory,
            pools: SyncedBalancerPools::new(Arc::new(DashMap::from_iter(rwlock_pools.into_iter()))),
            latest_synced_block,
            provider,
            block_sync,
            update_stream,
            block_stream
        }
    }
}

impl<P, Provider, BlockSync> BalancerPoolManager<P, Provider, BlockSync>
where
    BlockSync: BlockSyncConsumer
{
    pub fn pool_addresses(&self) -> impl Iterator<Item = PoolId> + '_ {
        self.pools.iter().map(|k| *k.key())
    }

    pub fn pools(&self) -> SyncedBalancerPools {
        self.pools.clone()
    }

    fn handle_new_block_info(&mut self, block_info: PoolManagerBlocks) {
        match block_info {
            PoolManagerBlocks::NewBlock(block) => {
                self.latest_synced_block = block;
            }
            PoolManagerBlocks::Reorg(tip, _rng) => {
                self.latest_synced_block = tip;
                self.block_sync.sign_off_reorg("BalancerV3", _rng, None);
            }
        }

        for pool in self.pools.pools.iter() {
            let mut pool = pool.value().write().expect("lock");
            pool.block_number = self.latest_synced_block;
        }

        self.block_sync
            .sign_off_on_block("BalancerV3", self.latest_synced_block, None);
    }
}

impl<P, Provider, BlockSync> Future for BalancerPoolManager<P, Provider, BlockSync>
where
    P: AlloyProvider + 'static,
    Provider: PoolManagerProvider + Send + Sync + 'static,
    BlockSync: BlockSyncConsumer
{
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        while let Poll::Ready(Some(Some(block_info))) = self.block_stream.poll_next_unpin(cx) {
            self.handle_new_block_info(block_info);
        }
        while let Poll::Ready(Some(event)) = self.update_stream.poll_next_unpin(cx) {
            match event {
                EthEvent::BalancerNewPool { pool_address, token0, token1 } => {
                    tracing::info!(?pool_address, ?token0, ?token1, "Adding new Balancer pool");

                    // Convert Address (20 bytes) to PoolId (32 bytes) by zero-extending
                    // PoolId is used as a key in DashMap, so we still need 32 bytes
                    let mut pool_id_bytes = [0u8; 32];
                    pool_id_bytes[12..32].copy_from_slice(&pool_address.into_array());
                    let pool_id = PoolId::from(pool_id_bytes);

                    // Create new pool state with Address directly
                    let pool_state = BalancerPoolState::new(
                        pool_address,
                        self.latest_synced_block,
                        0 // fee placeholder; read from pool if needed
                    );

                    self.pools
                        .pools
                        .insert(pool_id, Arc::new(RwLock::new(pool_state)));
                }

                EthEvent::BalancerRemovedPool { pool_address } => {
                    tracing::info!(?pool_address, "Removing Balancer pool");
                    // Convert Address (20 bytes) to PoolId (32 bytes) by zero-extending
                    let mut pool_id_bytes = [0u8; 32];
                    pool_id_bytes[12..32].copy_from_slice(&pool_address.into_array());
                    let pool_id = PoolId::from(pool_id_bytes);
                    self.pools.pools.remove(&pool_id);
                }

                // Ignore Uniswap events (won't occur in Balancer-only nodes)
                EthEvent::NewPool { .. } | EthEvent::RemovedPool { .. } => {}

                _ => {}
            }
        }
        Poll::Pending
    }
}
