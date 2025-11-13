//! Balancer pool state management

use std::sync::Arc;

use alloy::{
    primitives::{Address, U256},
    providers::Provider
};
use eyre::Result;

use super::pool_data_loader::{BalancerDataLoader, BalancerPoolDataLoader, ReClammPoolData};

/// ReClamm pool state
#[derive(Clone, Debug)]
pub struct ReClammPoolState {
    pub data_loader:    BalancerDataLoader,
    pub block_number:   u64,
    pub pool_address:   Address,
    pub vault_explorer: Address,

    // Cached from Vault
    pub tokens:             Vec<Address>,
    pub scaling_factors:    Vec<U256>,
    pub token_rates:        Vec<U256>,
    pub balances:           Vec<U256>,
    pub swap_fee:           U256,
    pub aggregate_swap_fee: U256,
    pub total_supply:       U256,

    // ReClamm-specific state
    pub last_timestamp:        U256,
    pub last_virtual_balances: Vec<U256>,

    // Price shift parameters
    pub daily_price_shift_exponent: U256,
    pub daily_price_shift_base:     U256,
    pub centeredness_margin:        U256,

    // Current price state
    pub current_price_ratio:             U256,
    pub current_fourth_root_price_ratio: U256,
    pub start_fourth_root_price_ratio:   U256,
    pub end_fourth_root_price_ratio:     U256,
    pub price_ratio_update_start_time:   u32,
    pub price_ratio_update_end_time:     u32,

    // Status flags
    pub is_pool_initialized:         bool,
    pub is_pool_paused:              bool,
    pub is_pool_in_recovery_mode:    bool,
    pub is_pool_within_target_range: bool,

    // Computed
    pub current_timestamp: U256
}

impl ReClammPoolState {
    /// Create a new ReClammPoolState
    pub fn new(vault_explorer: Address, pool_address: Address) -> Self {
        let data_loader = BalancerDataLoader::new(vault_explorer, pool_address);

        Self {
            data_loader,
            block_number: 0,
            pool_address,
            vault_explorer,
            tokens: Vec::new(),
            scaling_factors: Vec::new(),
            token_rates: Vec::new(),
            balances: Vec::new(),
            swap_fee: U256::ZERO,
            aggregate_swap_fee: U256::ZERO,
            total_supply: U256::ZERO,
            last_timestamp: U256::ZERO,
            last_virtual_balances: Vec::new(),
            daily_price_shift_exponent: U256::ZERO,
            daily_price_shift_base: U256::ZERO,
            centeredness_margin: U256::ZERO,
            current_price_ratio: U256::ZERO,
            current_fourth_root_price_ratio: U256::ZERO,
            start_fourth_root_price_ratio: U256::ZERO,
            end_fourth_root_price_ratio: U256::ZERO,
            price_ratio_update_start_time: 0,
            price_ratio_update_end_time: 0,
            is_pool_initialized: false,
            is_pool_paused: false,
            is_pool_in_recovery_mode: false,
            is_pool_within_target_range: false,
            current_timestamp: U256::ZERO
        }
    }

    /// Initialize pool state from blockchain
    pub async fn initialize<P: Provider>(
        &mut self,
        block_number: Option<u64>,
        provider: Arc<P>
    ) -> Result<()> {
        let pool_data = self
            .data_loader
            .load_pool_data(block_number, provider)
            .await?;

        self.update_from_pool_data(pool_data);
        self.block_number = block_number.unwrap_or(self.block_number);

        Ok(())
    }

    /// Update to a specific block
    pub async fn update_to_block<P: Provider>(
        &mut self,
        block_number: u64,
        provider: Arc<P>
    ) -> Result<()> {
        self.initialize(Some(block_number), provider).await
    }

    /// Update internal state from pool data
    fn update_from_pool_data(&mut self, pool_data: ReClammPoolData) {
        // From Vault
        self.tokens = pool_data.tokens;
        self.scaling_factors = pool_data.scaling_factors;
        self.token_rates = pool_data.token_rates;
        self.swap_fee = pool_data.swap_fee;
        self.aggregate_swap_fee = pool_data.aggregate_swap_fee;

        // State data
        self.balances = pool_data.balances_live_scaled18;
        self.total_supply = pool_data.total_supply;
        self.last_timestamp = pool_data.last_timestamp;
        self.last_virtual_balances = pool_data.last_virtual_balances;

        // Price shift parameters
        self.daily_price_shift_exponent = pool_data.daily_price_shift_exponent;
        self.daily_price_shift_base = pool_data.daily_price_shift_base;
        self.centeredness_margin = pool_data.centeredness_margin;

        // Current price state
        self.current_price_ratio = pool_data.current_price_ratio;
        self.current_fourth_root_price_ratio = pool_data.current_fourth_root_price_ratio;
        self.start_fourth_root_price_ratio = pool_data.start_fourth_root_price_ratio;
        self.end_fourth_root_price_ratio = pool_data.end_fourth_root_price_ratio;
        self.price_ratio_update_start_time = pool_data.price_ratio_update_start_time;
        self.price_ratio_update_end_time = pool_data.price_ratio_update_end_time;

        // Status flags
        self.is_pool_initialized = pool_data.is_pool_initialized;
        self.is_pool_paused = pool_data.is_pool_paused;
        self.is_pool_in_recovery_mode = pool_data.is_pool_in_recovery_mode;
        self.is_pool_within_target_range = pool_data.is_pool_within_target_range;

        // Computed
        self.current_timestamp = pool_data.current_timestamp;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_state_creation() {
        let vault_explorer = Address::ZERO;
        let pool = Address::ZERO;
        let state = ReClammPoolState::new(vault_explorer, pool);

        assert_eq!(state.pool_address, pool);
        assert_eq!(state.vault_explorer, vault_explorer);
        assert_eq!(state.block_number, 0);
    }
}
