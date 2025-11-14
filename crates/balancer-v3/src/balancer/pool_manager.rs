//! Balancer pool manager
//!
//! Manages a collection of Balancer pools and handles block updates/reorgs

use std::{ops::Range, sync::Arc};

use alloy::{primitives::Address, providers::Provider};
use dashmap::DashMap;
use parking_lot::RwLock;

use super::pool::ReClammPoolState;

pub type SyncedBalancerPool = Arc<RwLock<ReClammPoolState>>;
type PoolMap = Arc<DashMap<Address, SyncedBalancerPool>>;

#[derive(Clone)]
pub struct SyncedBalancerPools {
    pools: PoolMap
}

impl SyncedBalancerPools {
    pub fn new(pools: PoolMap) -> Self {
        Self { pools }
    }

    pub fn iter(&self) -> dashmap::iter::Iter<'_, Address, SyncedBalancerPool> {
        self.pools.iter()
    }

    pub fn get(&self, address: &Address) -> Option<SyncedBalancerPool> {
        self.pools.get(address).map(|p| p.clone())
    }

    pub fn insert(&self, address: Address, pool: SyncedBalancerPool) {
        self.pools.insert(address, pool);
    }

    pub fn len(&self) -> usize {
        self.pools.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pools.is_empty()
    }
}

/// Manages a collection of Balancer pools
pub struct BalancerPoolManager<P: Provider> {
    pools:               Arc<DashMap<Address, Arc<RwLock<ReClammPoolState>>>>,
    provider:            Arc<P>,
    latest_synced_block: u64
}

impl<P: Provider> BalancerPoolManager<P> {
    /// Create a new BalancerPoolManager
    pub fn new(provider: Arc<P>) -> Self {
        Self { pools: Arc::new(DashMap::new()), provider, latest_synced_block: 0 }
    }

    /// Add a pool to the manager
    pub fn add_pool(&self, pool: ReClammPoolState) {
        let pool_address = pool.pool_address;
        self.pools.insert(pool_address, Arc::new(RwLock::new(pool)));
    }

    /// Get a pool by address
    pub fn get_pool(&self, address: &Address) -> Option<Arc<RwLock<ReClammPoolState>>> {
        self.pools.get(address).map(|p| p.clone())
    }

    /// Get all pools as SyncedBalancerPools
    pub fn pools(&self) -> SyncedBalancerPools {
        SyncedBalancerPools::new(self.pools.clone())
    }

    /// Handle a new block
    pub async fn handle_new_block(&mut self, block_number: u64) {
        for pool in self.pools.iter() {
            let pool = pool.value();
            let mut l = pool.write();

            // Re-query pool state at new block
            let _ = l.update_to_block(block_number, self.provider.clone()).await;
        }

        self.latest_synced_block = block_number;
    }

    /// Handle a reorg
    pub fn handle_reorg(&mut self, tip: u64, _range: Range<u64>) {
        // On reorg, rollback to the new tip
        // The next handle_new_block will re-sync all pools
        self.latest_synced_block = tip;
    }

    /// Get the latest synced block
    pub fn latest_synced_block(&self) -> u64 {
        self.latest_synced_block
    }

    /// Get the number of pools
    pub fn pool_count(&self) -> usize {
        self.pools.len()
    }
}

#[cfg(test)]
mod tests {
    use alloy::providers::ProviderBuilder;

    use super::*;

    #[test]
    fn test_synced_pools() {
        let pools = SyncedBalancerPools::new(Arc::new(DashMap::new()));
        assert!(pools.is_empty());
        assert_eq!(pools.len(), 0);

        let pool = Arc::new(RwLock::new(ReClammPoolState::new(Address::ZERO)));
        pools.insert(Address::ZERO, pool.clone());

        assert!(!pools.is_empty());
        assert_eq!(pools.len(), 1);
        assert!(pools.get(&Address::ZERO).is_some());
    }

    #[tokio::test]
    async fn test_pool_manager_creation() {
        let provider = ProviderBuilder::new()
            .connect("http://localhost:8545")
            .await
            .unwrap();
        let manager = BalancerPoolManager::new(Arc::new(provider));

        assert_eq!(manager.pool_count(), 0);
        assert_eq!(manager.latest_synced_block(), 0);
    }

    #[tokio::test]
    async fn test_add_pool() {
        let provider = ProviderBuilder::new()
            .connect("http://localhost:8545")
            .await
            .unwrap();
        let manager = BalancerPoolManager::new(Arc::new(provider));

        let pool = ReClammPoolState::new(Address::ZERO);
        manager.add_pool(pool);

        assert_eq!(manager.pool_count(), 1);
        assert!(manager.get_pool(&Address::ZERO).is_some());
    }
}
