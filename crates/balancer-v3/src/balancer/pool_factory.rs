use alloy::primitives::Address;

use super::pool::ReClammPoolState;

pub struct V3PoolFactory {
    /// Pre-discovered pool addresses from historical scan
    discovered_pools: Vec<Address>
}

impl V3PoolFactory {
    pub fn new(discovered_pools: Vec<Address>) -> Self {
        Self { discovered_pools }
    }

    pub fn create_pools(&self) -> Vec<ReClammPoolState> {
        // Convert discovered pools into uninitialized ReClammPoolState
        // The pool manager will initialize them by querying RPC
        self.discovered_pools
            .iter()
            .map(|pool_address| ReClammPoolState::new(*pool_address))
            .collect()
    }
}
