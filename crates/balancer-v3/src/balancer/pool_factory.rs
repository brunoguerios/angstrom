use std::sync::Arc;

use alloy::{primitives::Address, providers::Provider};
use angstrom_types::{balancer_structure::BalancerPoolState, primitive::PoolId};

// use crate::balancer::pool_manager::SyncedBalancerPool;

pub struct V3PoolFactory<P> {
    provider:   Arc<P>,
    controller: Address
}

impl<P> V3PoolFactory<P>
where
    P: Provider + 'static
{
    pub fn new(provider: Arc<P>, controller: Address) -> Self {
        Self { provider, controller }
    }

    pub async fn init(&self, _latest_block: u64) -> Vec<(PoolId, BalancerPoolState)> {
        // TODO: decode Balancer pool configured/removed events from controller address
        // Placeholder: return empty set to scaffold wiring
        vec![]
    }
}
