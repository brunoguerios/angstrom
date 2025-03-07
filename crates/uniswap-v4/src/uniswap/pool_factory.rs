use crate::DataLoader;
use alloy::primitives::Address;
use alloy::providers::Provider;
use angstrom_types::contract_bindings::angstrom::Angstrom::PoolKey;
use angstrom_types::primitive::UniswapPoolRegistry;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::Arc;

use super::pool::EnhancedUniswapPool;
use super::pool_data_loader::PoolDataLoader;

pub struct V4PoolFactory<P> {
    provider: Arc<P>,
    registry: UniswapPoolRegistry,
    pool_manager: Address,
}
impl<P: Provider + 'static> V4PoolFactory<P> {
    pub fn new(provider: Arc<P>, registry: UniswapPoolRegistry, pool_manager: Address) -> Self {
        Self { provider, registry, pool_manager }
    }

    pub fn create_new_angstrom_pool<A>(
        &mut self,
        pool_id: A,
        pool_key: PoolKey,
    ) -> EnhancedUniswapPool<DataLoader<A>, A>
    where
        A: Eq + Hash + Debug + Default + Copy + Sync + Send + Unpin + 'static,
        DataLoader<A>: PoolDataLoader<A>,
    {
        todo!()
    }
}
