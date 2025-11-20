//! Unified pool state enum for Balancer pools
//!
//! This module provides a unified enum that wraps different Balancer pool types,
//! following the same pattern as `UniswapPoolState` in `uni_structure`.
//!
//! This is a pure data structure with no dependencies on `balancer-v3` crate,
//! allowing it to be used throughout the `types` crate without circular dependencies.

use alloy::primitives::{Address, U256};
use serde::{Deserialize, Serialize};

use crate::amm::{PoolSwapResult, Price};
use crate::balancer_structure::swap::BalancerSwapCalculator;

/// Unified enum for different Balancer pool types
///
/// This enum wraps different Balancer pool implementations as pure data structures.
/// Similar to `UniswapPoolState`, this contains only the pool state data without
/// any data loaders or initialization logic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BalancerPoolState {
    /// ReClamm pool (concentrated liquidity)
    ReClamm {
        pool_address: Address,
        block_number: u64,

        // From Vault
        tokens: Vec<Address>,
        scaling_factors: Vec<U256>,
        token_rates: Vec<U256>,
        balances_live_scaled_18: Vec<U256>,
        swap_fee: U256,
        aggregate_swap_fee: U256,
        total_supply: U256,

        // ReClamm-specific state
        last_timestamp: U256,
        last_virtual_balances: Vec<U256>,
        current_virtual_balances: Vec<U256>,

        // Price shift parameters
        daily_price_shift_exponent: U256,
        daily_price_shift_base: U256,
        centeredness_margin: U256,

        // Current price state
        current_price_ratio: U256,
        current_fourth_root_price_ratio: U256,
        start_fourth_root_price_ratio: U256,
        end_fourth_root_price_ratio: U256,
        price_ratio_update_start_time: u32,
        price_ratio_update_end_time: u32,

        // Status flags
        is_pool_initialized: bool,
        is_pool_paused: bool,
        is_pool_in_recovery_mode: bool,
        is_pool_within_target_range: bool,

        // Computed
        current_timestamp: U256,
    },
    // Future: Weighted(WeightedPoolState),
    // Future: Stable(StablePoolState),
}

impl BalancerPoolState {
    /// Get the pool address
    pub fn pool_address(&self) -> Address {
        match self {
            Self::ReClamm { pool_address, .. } => *pool_address,
        }
    }

    /// Get the tokens in the pool
    pub fn tokens(&self) -> &[Address] {
        match self {
            Self::ReClamm { tokens, .. } => tokens,
        }
    }

    /// Get the balances of tokens in the pool
    pub fn balances(&self) -> &[U256] {
        match self {
            Self::ReClamm { balances_live_scaled_18: balances, .. } => balances,
        }
    }

    /// Get the swap fee
    pub fn swap_fee(&self) -> U256 {
        match self {
            Self::ReClamm { swap_fee, .. } => *swap_fee,
        }
    }

    /// Get the block number
    pub fn block_number(&self) -> u64 {
        match self {
            Self::ReClamm { block_number, .. } => *block_number,
        }
    }

    /// Get the swap fee in parts per million
    pub fn fee(&self) -> u32 {
        match self {
            Self::ReClamm { .. } => {
                // Convert U256 to u32 (swap_fee is typically in 18 decimal format)
                // For now, return a reasonable default - this should be calculated properly
                // based on the actual fee format
                3000 // Default 0.3% fee
            }
        }
    }

    /// Get the current price
    ///
    /// Calculates the actual price from pool state using balancer-maths-rust
    pub fn current_price(&self) -> Price {
        use crate::sol_bindings::Ray;

        // Calculate price using balancer-maths-rust
        let price_scaled_18 = BalancerSwapCalculator::calculate_reclamm_price(self)
            .map_err(|e| eyre::eyre!("Calculate reclamm price failed: {}", e))
            .unwrap();

        // Convert from 1e18 format to Ray (1e27) by multiplying by 1e9
        let price_ray = price_scaled_18 * U256::from(1_000_000_000u64);

        Price::new(Ray::from(price_ray.to::<u128>()))
    }

    /// Swap with a given input amount
    ///
    /// # Arguments
    /// * `amount_in` - The amount to swap in
    /// * `zero_for_one` - Direction: true for token0->token1, false for token1->token0
    ///
    /// # Returns
    /// The swap result or an error if the swap fails
    pub fn swap_with_amount(
        &self,
        amount_in: u128,
        zero_for_one: bool,
    ) -> eyre::Result<PoolSwapResult> {
        let tokens = self.tokens();
        let token_in = if zero_for_one { tokens[0] } else { tokens[1] };
        let token_out = if zero_for_one { tokens[1] } else { tokens[0] };

        // Calculate amount out in raw scale
        let amount_out_scaled_raw = BalancerSwapCalculator::get_amount_out(
            self,
            U256::from(amount_in),
            token_in,
            token_out,
        )
        .map_err(|e| eyre::eyre!("Get amount out failed: {}", e))?;

        let current_price_scaled_27 = self.current_price();

        // Calculate end price after swap
        let token_in_index = if zero_for_one { 0 } else { 1 };
        let token_out_index = if zero_for_one { 1 } else { 0 };

        let end_price_scaled_27 = BalancerSwapCalculator::price_after_swap(
            self,
            token_in_index,
            token_out_index,
            U256::from(amount_in), // TODO Confirm scaling is correct
            amount_out_scaled_raw, // TODO Check scaling is correct
        )
        .map_err(|e| eyre::eyre!("Failed to calculate price after swap: {}", e))?;

        // Split amounts based on direction
        // PoolSwapResult stores input amounts for both tokens
        let (amount_in_t0, amount_in_t1) =
            if zero_for_one { (amount_in, 0) } else { (0, amount_in) };

        Ok(PoolSwapResult::new(
            self.fee(),
            current_price_scaled_27,
            end_price_scaled_27,
            amount_in_t0,
            amount_in_t1,
        ))
    }

    /// Swap to a target price
    ///
    /// # Arguments
    /// * `price_limit` - The target price to reach
    ///
    /// # Returns
    /// The swap result or an error if the swap fails
    pub fn swap_to_price(&self, price_limit: Price) -> eyre::Result<PoolSwapResult> {
        // Call the calculator to get the swap result
        let result = BalancerSwapCalculator::swap_to_price(self, price_limit)
            .map_err(|e| eyre::eyre!("Swap to price failed: {}", e))?;

        // Get current price for start_price
        let start_price_scaled_27 = self.current_price();

        // Convert SwapToTargetPriceResult to PoolSwapResult
        let (amount_in_t0, amount_in_t1) = if result.token_in_index == 0 {
            (result.amount_in_raw.to::<u128>(), 0)
        } else if result.token_in_index == 1 {
            (0, result.amount_in_raw.to::<u128>())
        } else {
            return Err(eyre::eyre!(
                "Invalid token_in_index: {}, expected 0 or 1",
                result.token_in_index
            ));
        };

        // TODO - The amount will be scaled raw, confirm this is correct.
        Ok(PoolSwapResult::new(
            self.fee(),
            start_price_scaled_27,
            price_limit, // end_price is the target price we're swapping to
            amount_in_t0,
            amount_in_t1,
        ))
    }

    /// Get a no-op swap result (no trading, same start/end price)
    pub fn noop(&self) -> PoolSwapResult {
        let current_price = self.current_price();
        PoolSwapResult::new(self.fee(), current_price, current_price, 0, 0)
    }
}
