//! Venue-neutral pool swap result representation
//!
//! This module provides a generic PoolSwapResult type that can represent
//! swap results from different AMM implementations.

use std::fmt::{Debug, Display};

use serde::{Deserialize, Serialize};

use super::price::Price;

/// A venue-neutral pool swap result
///
/// This wraps venue-specific swap result types to provide a common interface
/// across different AMM implementations.
///
/// AMM-specific types:
/// - `UniswapPoolSwapResult` in `uni_structure/`
/// - `BalancerPoolSwapResult` in `balancer_structure/` (future)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PoolSwapResult {
    /// Fee in parts per million
    pub fee_ppm:      u32,
    /// Starting price of the swap
    pub start_price:  Price,
    /// Ending price of the swap
    pub end_price:    Price,
    /// Amount of token0 input
    pub amount_in_t0: u128,
    /// Amount of token1 input
    pub amount_in_t1: u128
}

impl PoolSwapResult {
    /// Create a new PoolSwapResult
    pub fn new(
        fee_ppm: u32,
        start_price: Price,
        end_price: Price,
        amount_in_t0: u128,
        amount_in_t1: u128
    ) -> Self {
        Self { fee_ppm, start_price, end_price, amount_in_t0, amount_in_t1 }
    }

    /// Check if this was an empty swap (no actual trading occurred)
    pub fn is_empty(&self) -> bool {
        self.amount_in_t0 == 0 && self.amount_in_t1 == 0
    }

    /// Get the total input amount (sum of both tokens)
    pub fn total_input(&self) -> u128 {
        self.amount_in_t0 + self.amount_in_t1
    }
}

impl Display for PoolSwapResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PoolSwapResult(fee={}ppm, t0_in={}, t1_in={}, price: {} -> {})",
            self.fee_ppm, self.amount_in_t0, self.amount_in_t1, self.start_price, self.end_price
        )
    }
}

// Conversion from UniswapPoolSwapResult to PoolSwapResult
impl From<&crate::uni_structure::pool_swap::UniswapPoolSwapResult<'_>> for PoolSwapResult {
    fn from(result: &crate::uni_structure::pool_swap::UniswapPoolSwapResult<'_>) -> Self {
        Self::new(
            result.fee,
            result.start_price.into(),
            result.end_price.into(),
            result.total_d_t0,
            result.total_d_t1
        )
    }
}

// Conversion implementations for Balancer (to be added when Balancer support is
// implemented) impl From<&BalancerPoolSwapResult> for PoolSwapResult {
//     fn from(result: &BalancerPoolSwapResult) -> Self {
//         Self::new(
//             result.fee_ppm,
//             result.start_price.into(),
//             result.end_price.into(),
//             result.amount_in_t0,
//             result.amount_in_t1,
//         )
//     }
// }
