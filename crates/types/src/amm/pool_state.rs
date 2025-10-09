//! Angstrom pool state trait for AMM-agnostic operations
//!
//! This module defines the PoolState trait that provides a common
//! interface for pool operations across different AMM implementations
//! (UniswapPoolState, BalancerPoolState, etc.).

use std::{any::Any, fmt::Debug};

use serde::{Deserialize, Serialize};

use super::{pool_swap::PoolSwapResult, price::Price};

/// A trait for pool state that abstracts over different AMM implementations
///
/// This trait provides the minimal interface needed by downstream consumers
/// (matching engine, quoter, consensus, telemetry) without exposing
/// venue-specific details like ticks, bitmaps, or weights.
///
/// Implementations:
/// - `UniswapPoolState` in `uni_structure/`
/// - `BalancerPoolState` in `balancer_structure/`
pub trait PoolState: Send + Sync + Debug {
    /// Downcast helper for accessing concrete pool types
    ///
    /// This allows downcasting to specific pool types when needed
    /// (e.g., for venue-specific operations like ToB orders)
    fn as_any(&self) -> &dyn Any;

    /// Clone this pool state into a new Box
    ///
    /// This is needed because Box<dyn PoolState> doesn't automatically
    /// implement Clone
    fn clone_box(&self) -> Box<dyn PoolState>;

    /// Get the current block number
    fn block_number(&self) -> u64;

    /// Get the pool fee in parts per million
    fn fee(&self) -> u32;

    /// Get the current price
    fn current_price(&self) -> Price;

    /// Swap with a given input amount
    ///
    /// # Arguments
    /// * `amount_in` - The amount to swap in
    /// * `zero_for_one` - Direction: true for token0->token1, false for
    ///   token1->token0
    ///
    /// # Returns
    /// The swap result or an error if the swap fails
    fn swap_with_amount(&self, amount_in: u128, zero_for_one: bool)
    -> eyre::Result<PoolSwapResult>;

    /// Swap to a target price
    ///
    /// # Arguments
    /// * `price_limit` - The target price to reach
    ///
    /// # Returns
    /// The swap result or an error if the swap fails
    fn swap_to_price(&self, price_limit: Price) -> eyre::Result<PoolSwapResult>;

    /// Get a no-op swap result (no trading, same start/end price)
    fn noop(&self) -> PoolSwapResult;
}

/// Implement Clone for Box<dyn PoolState> using the clone_box method
impl Clone for Box<dyn PoolState> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// A wrapper that provides ergonomic arithmetic operations for
/// PoolState
///
/// This allows existing code that uses `Add<Quantity>` and `Sub<Quantity>`
/// to continue working with the new trait-based approach.
pub struct PoolStateRef<'a>(pub &'a dyn PoolState);

impl<'a> PoolStateRef<'a> {
    /// Create a new PoolStateRef wrapper
    pub fn new(pool_state: &'a dyn PoolState) -> Self {
        Self(pool_state)
    }
}

/// Serializable, venue-neutral snapshot of a PoolState based on a noop swap
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolStateSnapshot {
    pub fee:          u32,
    pub start_price:  u128,
    pub end_price:    u128,
    pub amount_in_t0: u128,
    pub amount_in_t1: u128
}

impl From<&dyn PoolState> for PoolStateSnapshot {
    fn from(ps: &dyn PoolState) -> Self {
        let psr = ps.noop();
        Self {
            fee:          ps.fee(),
            start_price:  psr.start_price.value(),
            end_price:    psr.end_price.value(),
            amount_in_t0: psr.amount_in_t0,
            amount_in_t1: psr.amount_in_t1
        }
    }
}

// Implement arithmetic operations for ergonomic usage
// Note: These would need to be implemented based on the specific Quantity type
// and how the existing arithmetic operations work. This is a placeholder
// for the actual implementation.

// impl<'a> core::ops::Add<crate::matching::uniswap::Quantity> for
// PoolStateRef<'a> {     type Output = eyre::Result<SwapOutcome>;
//
//     fn add(self, quantity: crate::matching::uniswap::Quantity) ->
// Self::Output {         // Delegate to swap_with_amount based on quantity
// direction and amount         self.0.swap_with_amount(quantity.amount(),
// quantity.zero_for_one())     }
// }

// impl<'a> core::ops::Sub<crate::matching::uniswap::Quantity> for
// PoolStateRef<'a> {     type Output = eyre::Result<SwapOutcome>;
//
//     fn sub(self, quantity: crate::matching::uniswap::Quantity) ->
// Self::Output {         // Delegate to swap_with_amount with opposite
// direction         self.0.swap_with_amount(quantity.amount(),
// !quantity.zero_for_one())     }
// }
