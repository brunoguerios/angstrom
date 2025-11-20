//! Swap calculation wrapper for Balancer pools
//!
//! This module provides swap calculation functionality directly in `types`,
//! following the same pattern as `UniswapPoolSwap` in `uni_structure`.
//!
//! Swap calculations are implemented using `balancer-maths-rust` directly,
//! with no dependency on `balancer-v3` crate.

use alloy::primitives::{Address, U256};
use balancer_maths_rust::{
    PoolState as BmPoolState, SwapKind, Vault,
    common::{
        SwapInput,
        types::{BasePoolState, PoolStateOrBuffer},
    },
    pools::{
        ReClammImmutable, ReClammMutable, ReClammState,
        reclamm::{SwapToTargetPriceResult, calculate_reclamm_price, swap_reclamm_to_price},
    },
};
use eyre::Result;

use super::BalancerPoolState;
use crate::amm::Price;

/// Direction of swap calculation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwapDirection {
    /// Calculate output amount for given input amount
    GivenIn,
    /// Calculate input amount for given output amount
    GivenOut,
}

/// Calculator for Balancer swap operations
///
/// This implements swap calculations directly using `balancer-maths-rust`,
pub struct BalancerSwapCalculator;

impl BalancerSwapCalculator {
    /// Convert ReClamm pool fields to balancer-maths-rust ReClammState
    ///
    /// This is a pure conversion function that takes destructured ReClamm fields
    /// and converts them to the format expected by balancer-maths-rust.
    fn to_reclamm_state(
        pool_address: &Address,
        tokens: &[Address],
        scaling_factors: &[U256],
        token_rates: &[U256],
        balances_live_scaled_18: &[U256],
        swap_fee: &U256,
        aggregate_swap_fee: &U256,
        total_supply: &U256,
        last_timestamp: &U256,
        last_virtual_balances: &[U256],
        daily_price_shift_base: &U256,
        centeredness_margin: &U256,
        start_fourth_root_price_ratio: &U256,
        end_fourth_root_price_ratio: &U256,
        price_ratio_update_start_time: &u32,
        price_ratio_update_end_time: &u32,
        current_timestamp: &U256,
    ) -> ReClammState {
        ReClammState {
            base: BasePoolState {
                pool_address: format!("{:?}", pool_address),
                pool_type: "RECLAMM".to_string(),
                tokens: tokens.iter().map(|t| format!("{:?}", t)).collect(),
                scaling_factors: scaling_factors.to_vec(),
                token_rates: token_rates.to_vec(),
                balances_live_scaled_18: balances_live_scaled_18.to_vec(),
                swap_fee: *swap_fee,
                aggregate_swap_fee: *aggregate_swap_fee,
                total_supply: *total_supply,
                supports_unbalanced_liquidity: true,
                hook_type: None,
            },
            mutable: ReClammMutable {
                last_timestamp: *last_timestamp,
                last_virtual_balances: last_virtual_balances.to_vec(),
                daily_price_shift_base: *daily_price_shift_base,
                centeredness_margin: *centeredness_margin,
                start_fourth_root_price_ratio: *start_fourth_root_price_ratio,
                end_fourth_root_price_ratio: *end_fourth_root_price_ratio,
                price_ratio_update_start_time: U256::from(*price_ratio_update_start_time),
                price_ratio_update_end_time: U256::from(*price_ratio_update_end_time),
                current_timestamp: *current_timestamp,
            },
            immutable: ReClammImmutable {
                pool_address: format!("{:?}", pool_address),
                tokens: tokens.iter().map(|t| format!("{:?}", t)).collect(),
            },
        }
    }

    /// Calculate swap amount (input or output depending on direction)
    pub fn calculate_swap(
        pool: &BalancerPoolState,
        amount: U256,
        token_in: Address,
        token_out: Address,
        direction: SwapDirection,
    ) -> Result<U256> {
        let vault = Vault::new();

        // Route to appropriate converter based on pool type
        let pool_state = match pool {
            BalancerPoolState::ReClamm {
                pool_address,
                tokens,
                scaling_factors,
                token_rates,
                balances_live_scaled_18,
                swap_fee,
                aggregate_swap_fee,
                total_supply,
                last_timestamp,
                last_virtual_balances,
                daily_price_shift_base,
                centeredness_margin,
                start_fourth_root_price_ratio,
                end_fourth_root_price_ratio,
                price_ratio_update_start_time,
                price_ratio_update_end_time,
                current_timestamp,
                ..
            } => {
                let reclamm_state = Self::to_reclamm_state(
                    pool_address,
                    tokens,
                    scaling_factors,
                    token_rates,
                    balances_live_scaled_18,
                    swap_fee,
                    aggregate_swap_fee,
                    total_supply,
                    last_timestamp,
                    last_virtual_balances,
                    daily_price_shift_base,
                    centeredness_margin,
                    start_fourth_root_price_ratio,
                    end_fourth_root_price_ratio,
                    price_ratio_update_start_time,
                    price_ratio_update_end_time,
                    current_timestamp,
                );
                PoolStateOrBuffer::Pool(Box::new(BmPoolState::ReClamm(reclamm_state)))
            } // Future: Weighted { .. } => {
              //     let weighted_state = Self::to_weighted_state(...);
              //     PoolStateOrBuffer::Pool(Box::new(BmPoolState::Weighted(weighted_state)))
              // }
              // Future: Stable { .. } => {
              //     let stable_state = Self::to_stable_state(...);
              //     PoolStateOrBuffer::Pool(Box::new(BmPoolState::Stable(stable_state)))
              // }
        };

        let swap_kind = match direction {
            SwapDirection::GivenIn => SwapKind::GivenIn,
            SwapDirection::GivenOut => SwapKind::GivenOut,
        };

        let swap_input = SwapInput {
            amount_raw: amount,
            swap_kind,
            token_in: format!("{:?}", token_in),
            token_out: format!("{:?}", token_out),
        };

        let result = vault.swap(
            &swap_input,
            &pool_state,
            None, // No hooks for now
        )?;

        Ok(result)
    }

    /// Calculate amount out for given amount in
    pub fn get_amount_out(
        pool: &BalancerPoolState,
        amount_in: U256,
        token_in: Address,
        token_out: Address,
    ) -> Result<U256> {
        Self::calculate_swap(pool, amount_in, token_in, token_out, SwapDirection::GivenIn)
    }

    /// Calculate amount in for given amount out
    pub fn get_amount_in(
        pool: &BalancerPoolState,
        amount_out: U256,
        token_in: Address,
        token_out: Address,
    ) -> Result<U256> {
        Self::calculate_swap(pool, amount_out, token_in, token_out, SwapDirection::GivenOut)
    }

    /// Swap to a target price
    ///
    /// # Arguments
    /// * `pool` - The pool state
    /// * `target_price` - The target price to reach (in Ray format, 1e27)
    ///
    /// # Returns
    /// The swap result containing token indices and amounts, or an error string
    pub fn swap_to_price(
        pool: &BalancerPoolState,
        target_price: Price,
    ) -> Result<SwapToTargetPriceResult, String> {
        match pool {
            BalancerPoolState::ReClamm {
                token_rates,
                balances_live_scaled_18,
                current_virtual_balances,
                swap_fee,
                aggregate_swap_fee,
                scaling_factors,
                ..
            } => {
                // Convert target_price from Ray (1e27) to 1e18 format
                // Ray value is scaled by 1e27, we need 1e18, so divide by 1e9
                let target_price_scaled_18 = target_price.as_ray().0 / U256::from(1_000_000_000u64);

                // Call swap_reclamm_to_price
                swap_reclamm_to_price(
                    token_rates,
                    scaling_factors,
                    balances_live_scaled_18,
                    current_virtual_balances,
                    swap_fee,
                    aggregate_swap_fee,
                    &U256::ZERO, // TODO pool_creator_fee_percentage (not stored in pool state)
                    &target_price_scaled_18,
                )
            } // Future: Weighted { .. } => {
              //     // Implement for weighted pools when needed
              //     Err("Weighted pool swap_to_price not yet implemented".to_string())
              // }
              // Future: Stable { .. } => {
              //     // Implement for stable pools when needed
              //     Err("Stable pool swap_to_price not yet implemented".to_string())
              // }
        }
    }

    /// Calculate the current price of a ReClamm pool
    ///
    /// # Arguments
    /// * `pool` - The pool state
    ///
    /// # Returns
    /// The current price in 1e18 format, or an error string
    pub fn calculate_reclamm_price(pool: &BalancerPoolState) -> Result<U256, String> {
        match pool {
            BalancerPoolState::ReClamm {
                balances_live_scaled_18,
                current_virtual_balances,
                ..
            } => {
                // Call the external calculate_reclamm_price function
                Ok(calculate_reclamm_price(balances_live_scaled_18, current_virtual_balances))
            } // Future: Weighted { .. } => {
              //     // Implement for weighted pools when needed
              //     Err("Weighted pool calculate_reclamm_price not yet implemented".to_string())
              // }
              // Future: Stable { .. } => {
              //     // Implement for stable pools when needed
              //     Err("Stable pool calculate_reclamm_price not yet implemented".to_string())
              // }
        }
    }

    /// Helper to apply swap to balances
    ///
    /// Takes immutable balances and returns a new Vec with updated balances after the swap.
    /// Converts raw amounts to scaled18 format before applying the swap.
    fn update_balances(
        balances_scaled_18: &[U256],
        scaling_factors: &[U256],
        token_in_index: usize,
        token_out_index: usize,
        amount_in_raw: U256,
        amount_out_raw: U256,
    ) -> Vec<U256> {
        // Clone balances to avoid mutating the original
        let mut new_balances = balances_scaled_18.to_vec();

        let amount_in_scaled_18 = amount_in_raw * scaling_factors[token_in_index];
        let amount_out_scaled_18 = amount_out_raw * scaling_factors[token_out_index];

        new_balances[token_in_index] += amount_in_scaled_18;
        new_balances[token_out_index] -= amount_out_scaled_18;

        new_balances
    }

    /// Calculate the price after applying a swap to pool balances
    ///
    /// # Arguments
    /// * `pool` - The pool state
    /// * `token_in_index` - Index of the token being swapped in
    /// * `token_out_index` - Index of the token being swapped out
    /// * `amount_in_raw` - Raw amount of token in (before scaling)
    /// * `amount_out_raw` - Raw amount of token out (before scaling)
    ///
    /// # Returns
    /// The price after the swap in Ray format (1e27), or an error string
    pub fn price_after_swap(
        pool: &BalancerPoolState,
        token_in_index: usize,
        token_out_index: usize,
        amount_in_raw: U256,
        amount_out_raw: U256,
    ) -> Result<Price, String> {
        match pool {
            BalancerPoolState::ReClamm {
                balances_live_scaled_18,
                current_virtual_balances,
                scaling_factors,
                ..
            } => {
                // Update balances with the swap
                let updated_balances_scaled_18 = Self::update_balances(
                    balances_live_scaled_18,
                    scaling_factors,
                    token_in_index,
                    token_out_index,
                    amount_in_raw,
                    amount_out_raw,
                );

                // Calculate price with updated balances and existing current_virtual_balances
                let price_scaled_18 =
                    calculate_reclamm_price(&updated_balances_scaled_18, current_virtual_balances);

                // Convert from 1e18 format to Ray (1e27) by multiplying by 1e9
                let price_scaled_27 = price_scaled_18 * U256::from(1_000_000_000u64);

                // Convert to Price type
                use crate::sol_bindings::Ray;
                Ok(Price::new(Ray::from(price_scaled_27.to::<u128>())))
            } // Future: Weighted { .. } => {
              //     // Implement for weighted pools when needed
              //     Err("Weighted pool price_after_swap not yet implemented".to_string())
              // }
              // Future: Stable { .. } => {
              //     // Implement for stable pools when needed
              //     Err("Stable pool price_after_swap not yet implemented".to_string())
              // }
        }
    }
}
