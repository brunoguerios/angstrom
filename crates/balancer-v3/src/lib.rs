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
/// Returns a list of configured pool addresses
pub async fn fetch_balancer_pools<DB>(
    deploy_block: usize,
    end_block: usize,
    balancer_controller: Address,
    db: &DB
) -> Vec<Address>
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
                set.insert(configured.poolAddress);
                return set;
            }

            if let Ok(removed) = BalancerPoolRemoved::decode_log(&log) {
                set.remove(&removed.poolAddress);
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
    discovered_pools: Vec<Address>
) -> BalancerPoolManager<P>
where
    P: Provider + 'static
{
    let manager = BalancerPoolManager::new(provider);

    // Create factory and initialize pools
    if !discovered_pools.is_empty() {
        let factory = V3PoolFactory::new(discovered_pools);

        for pool in factory.create_pools() {
            manager.add_pool(pool);
        }
    }

    manager
}
