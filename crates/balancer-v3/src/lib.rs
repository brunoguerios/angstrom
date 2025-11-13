//! Balancer V3 integration for Angstrom
//!
//! This crate provides integration with Balancer V3 pools, including:
//! - RPC query system for pool state
//! - Pool state management and synchronization
//! - Integration with balancer-maths-rust for swap calculations

use std::{collections::HashSet, sync::Arc};

use alloy::{consensus::TxReceipt, primitives::Address, providers::Provider, sol_types::SolEvent};
use angstrom_types::contract_bindings::balancer_controller::BalancerController::{
    BalancerPoolConfigured, BalancerPoolRemoved
};
use reth_provider::{DatabaseProviderFactory, ReceiptProvider, TryIntoHistoricalStateProvider};

pub mod balancer;

// Re-export common types
pub use alloy::primitives::U256;
// Re-export for future use in Step 9.8
#[cfg(feature = "event-driven")]
pub use balancer::pool_providers::canonical_state_adapter::CanonicalStateAdapter;
pub use balancer::{
    pool::ReClammPoolState,
    pool_data_loader::{BalancerDataLoader, BalancerPoolDataLoader, ReClammPoolData},
    pool_factory::V3PoolFactory,
    pool_manager::{BalancerPoolManager, SyncedBalancerPools}
};

/// Fetches all Balancer pools from historical event logs
/// Returns a tuple of (pool_address, vault_explorer_address) for each
/// configured pool
pub async fn fetch_balancer_pools<DB>(
    deploy_block: usize,
    end_block: usize,
    balancer_controller: Address,
    db: &DB
) -> Vec<(Address, Address)>
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

    // TODO: Update when contracts are ready - need vault_explorer from events
    // For now, return (pool_address, vault_explorer) where vault_explorer =
    // controller
    logs.into_iter()
        .fold(HashSet::new(), |mut set, log| {
            if let Ok(configured) = BalancerPoolConfigured::decode_log(&log) {
                // TODO: Extract vault_explorer from event when available
                set.insert((configured.poolAddress, balancer_controller));
                return set;
            }

            if let Ok(removed) = BalancerPoolRemoved::decode_log(&log) {
                set.retain(|(addr, _)| *addr != removed.poolAddress);
                return set;
            }
            set
        })
        .into_iter()
        .collect::<Vec<_>>()
}

/// Configure a simple Balancer pool manager
pub fn configure_balancer_manager<P>(
    provider: Arc<P>,
    discovered_pools: Vec<(Address, Address)>
) -> BalancerPoolManager<P>
where
    P: Provider + 'static
{
    let manager = BalancerPoolManager::new(provider);

    // Create factory and initialize pools
    if !discovered_pools.is_empty() {
        let vault_explorer = discovered_pools[0].1; // Use first vault_explorer as default
        let factory = V3PoolFactory::new(vault_explorer, discovered_pools);

        for pool in factory.create_pools() {
            manager.add_pool(pool);
        }
    }

    manager
}
