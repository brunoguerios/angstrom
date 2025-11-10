use std::sync::Arc;

use alloy::{primitives::Address, providers::Provider};
use angstrom_types::{balancer_structure::BalancerPoolState, primitive::PoolId};

pub struct V3PoolFactory<P> {
    provider:         Arc<P>,
    controller:       Address,
    /// Pre-discovered pools from historical scan
    discovered_pools: Vec<(Address, Address, Address)> // (pool_address, token0, token1)
}

impl<P> V3PoolFactory<P>
where
    P: Provider + 'static
{
    pub fn new(
        provider: Arc<P>,
        controller: Address,
        discovered_pools: Vec<(Address, Address, Address)>
    ) -> Self {
        Self { provider, controller, discovered_pools }
    }

    pub async fn init(&self, latest_block: u64) -> Vec<(PoolId, BalancerPoolState)> {
        // Convert discovered pools into initialized BalancerPoolState
        self.discovered_pools
            .iter()
            .map(|(pool_address, _token0, _token1)| {
                // Convert Address (20 bytes) to PoolId (32 bytes) by zero-extending
                // PoolId is used as a key in DashMap, so we still need 32 bytes
                let mut pool_id_bytes = [0u8; 32];
                pool_id_bytes[12..32].copy_from_slice(&pool_address.into_array());
                let pool_id = PoolId::from(pool_id_bytes);

                // Create pool state with Address directly
                let pool_state = BalancerPoolState::new(
                    *pool_address,
                    latest_block,
                    0 // fee placeholder - can be read from pool contract if needed
                );

                (pool_id, pool_state)
            })
            .collect()
    }
}
