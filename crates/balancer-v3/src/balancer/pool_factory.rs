use alloy::primitives::Address;

use super::pool::ReClammPoolState;

pub struct V3PoolFactory {
    vault_explorer:   Address,
    /// Pre-discovered pools from historical scan
    /// Format: (pool_address, vault_explorer_address)
    discovered_pools: Vec<(Address, Address)>
}

impl V3PoolFactory {
    pub fn new(vault_explorer: Address, discovered_pools: Vec<(Address, Address)>) -> Self {
        Self { vault_explorer, discovered_pools }
    }

    pub fn create_pools(&self) -> Vec<ReClammPoolState> {
        // Convert discovered pools into uninitialized ReClammPoolState
        // The pool manager will initialize them by querying RPC
        self.discovered_pools
            .iter()
            .map(|(pool_address, vault_explorer)| {
                ReClammPoolState::new(*vault_explorer, *pool_address)
            })
            .collect()
    }
}
