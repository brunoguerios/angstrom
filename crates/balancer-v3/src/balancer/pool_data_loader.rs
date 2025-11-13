//! Balancer pool data loader
//!
//! This module provides traits and implementations for loading pool state from
//! the blockchain using multicall to aggregate multiple RPC calls into a single
//! request.

use std::{future::Future, sync::Arc};

use alloy::{
    primitives::{Address, U256},
    providers::Provider
};
use eyre::Result;

// Re-export ABIs and data structures
pub use super::loaders::{IReClamm, IVault, IVaultExplorer, ReClammPoolData};

/// Trait for loading Balancer pool data from the blockchain
pub trait BalancerPoolDataLoader: Clone + Send + Sync + 'static {
    /// Load pool data at a specific block
    fn load_pool_data<P: Provider>(
        &self,
        block_number: Option<u64>,
        provider: Arc<P>
    ) -> impl Future<Output = Result<ReClammPoolData>> + Send;

    /// Get the vault explorer address
    fn vault_explorer_address(&self) -> Address;

    /// Get the pool address
    fn pool_address(&self) -> Address;
}

/// Default implementation of BalancerPoolDataLoader
#[derive(Clone, Debug)]
pub struct BalancerDataLoader {
    vault_explorer: Address,
    pool:           Address
}

impl BalancerDataLoader {
    /// Create a new BalancerDataLoader
    pub fn new(vault_explorer: Address, pool: Address) -> Self {
        Self { vault_explorer, pool }
    }
}

impl BalancerPoolDataLoader for BalancerDataLoader {
    async fn load_pool_data<P: Provider>(
        &self,
        block_number: Option<u64>,
        provider: Arc<P>
    ) -> Result<ReClammPoolData> {
        // Create contract instances
        let vault_explorer = IVaultExplorer::new(self.vault_explorer, provider.clone());
        let pool = IReClamm::new(self.pool, provider.clone());

        // Get vault address from VaultExplorer
        let vault_address = vault_explorer.getVault().call().await?;
        let vault = IVault::new(vault_address, provider.clone());

        // Execute all calls (Alloy handles this efficiently, similar to multicall)
        // Each call will be a separate RPC request, but they can be batched by the
        // provider
        let block_id = block_number
            .map(|n| alloy::eips::BlockId::Number(alloy::eips::BlockNumberOrTag::Number(n)));

        // Call 1: Get pool config from Vault (not VaultExplorer!)
        // The TypeScript reference implementation calls getPoolConfig on the Vault
        // address
        let mut config_call = vault.getPoolConfig(self.pool);
        if let Some(block) = block_id {
            config_call = config_call.block(block);
        }
        let config = config_call.call().await?;

        // Call 2: Get pool dynamic data
        let mut dynamic_data_call = pool.getReClammPoolDynamicData();
        if let Some(block) = block_id {
            dynamic_data_call = dynamic_data_call.block(block);
        }
        let dynamic_data = dynamic_data_call.call().await?;

        // Call 3: Check if pool is within target range
        let mut range_call = pool.isPoolWithinTargetRange();
        if let Some(block) = block_id {
            range_call = range_call.block(block);
        }
        let is_within_range = range_call.call().await?;

        // Call 4: Get token info from Vault
        let mut token_info_call = vault.getPoolTokenInfo(self.pool);
        if let Some(block) = block_id {
            token_info_call = token_info_call.block(block);
        }
        let token_info = token_info_call.call().await?;
        let tokens = token_info.tokens;
        let scaling_factors = token_info.scalingFactors;

        // Call 5: Get token rates from Vault
        let mut token_rates_call = vault.getPoolTokenRates(self.pool);
        if let Some(block) = block_id {
            token_rates_call = token_rates_call.block(block);
        }
        let token_rates_result = token_rates_call.call().await?;
        let token_rates = token_rates_result.tokenRates;

        // Get current timestamp (approximate - could query block timestamp if needed)
        let current_timestamp = U256::from(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );

        // Aggregate all data into ReClammPoolData
        Ok(ReClammPoolData {
            // From Vault
            tokens,
            scaling_factors,
            token_rates,
            swap_fee: dynamic_data.staticSwapFeePercentage,
            aggregate_swap_fee: config.aggregateSwapFeePercentage,

            // From Pool - State Data
            balances_live_scaled18: dynamic_data.balancesLiveScaled18,
            total_supply: dynamic_data.totalSupply,
            last_timestamp: dynamic_data.lastTimestamp,
            last_virtual_balances: dynamic_data.lastVirtualBalances,

            // From Pool - Price Shift Parameters
            daily_price_shift_exponent: dynamic_data.dailyPriceShiftExponent,
            daily_price_shift_base: dynamic_data.dailyPriceShiftBase,
            centeredness_margin: dynamic_data.centerednessMargin,

            // From Pool - Current Price State
            current_price_ratio: dynamic_data.currentPriceRatio,
            current_fourth_root_price_ratio: dynamic_data.currentFourthRootPriceRatio,
            start_fourth_root_price_ratio: dynamic_data.startFourthRootPriceRatio,
            end_fourth_root_price_ratio: dynamic_data.endFourthRootPriceRatio,
            price_ratio_update_start_time: dynamic_data.priceRatioUpdateStartTime,
            price_ratio_update_end_time: dynamic_data.priceRatioUpdateEndTime,

            // From Pool - Status Flags
            is_pool_initialized: dynamic_data.isPoolInitialized,
            is_pool_paused: dynamic_data.isPoolPaused,
            is_pool_in_recovery_mode: dynamic_data.isPoolInRecoveryMode,
            is_pool_within_target_range: is_within_range,

            // Computed
            current_timestamp
        })
    }

    fn vault_explorer_address(&self) -> Address {
        self.vault_explorer
    }

    fn pool_address(&self) -> Address {
        self.pool
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_loader_creation() {
        let vault_explorer = Address::ZERO;
        let pool = Address::ZERO;
        let loader = BalancerDataLoader::new(vault_explorer, pool);

        assert_eq!(loader.vault_explorer_address(), vault_explorer);
        assert_eq!(loader.pool_address(), pool);
    }
}
