//! Angstrom pool state enum for AMM operations
//!
//! This module defines the PoolState enum that provides a unified interface
//! for pool operations across different AMM implementations
//! (UniswapPoolState, BalancerPoolState, etc.).

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use super::{pool_swap::PoolSwapResult, price::Price};
use crate::{balancer_structure::BalancerPoolState, uni_structure::UniswapPoolState};

/// An enum for pool state that abstracts over different AMM implementations
///
/// This enum provides a zero-cost abstraction for working with different
/// AMM types while maintaining type safety and avoiding vtable overhead.
///
/// Variants:
/// - `Uniswap` - Uniswap V4 pools with tick-based concentrated liquidity
/// - `Balancer` - Balancer V3 pools (weighted, stable, concentrated)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PoolState {
    Uniswap(UniswapPoolState),
    Balancer(BalancerPoolState)
}

impl PoolState {
    /// Get the current block number
    pub fn block_number(&self) -> u64 {
        match self {
            PoolState::Uniswap(u) => u.block_number(),
            PoolState::Balancer(b) => b.block_number()
        }
    }

    /// Get the pool fee in parts per million
    pub fn fee(&self) -> u32 {
        match self {
            PoolState::Uniswap(u) => u.fee(),
            PoolState::Balancer(b) => b.fee()
        }
    }

    /// Get the current price
    pub fn current_price(&self) -> Price {
        match self {
            PoolState::Uniswap(u) => u.current_price().into(),
            PoolState::Balancer(b) => b.current_price()
        }
    }

    /// Swap with a given input amount
    ///
    /// # Arguments
    /// * `amount_in` - The amount to swap in
    /// * `zero_for_one` - Direction: true for token0->token1, false for
    ///   token1->token0
    ///
    /// # Returns
    /// The swap result or an error if the swap fails
    pub fn swap_with_amount(
        &self,
        amount_in: u128,
        zero_for_one: bool
    ) -> eyre::Result<PoolSwapResult> {
        match self {
            PoolState::Uniswap(u) => {
                let amount = alloy::primitives::I256::unchecked_from(amount_in);
                let result = u.swap_current_with_amount(amount, zero_for_one)?;
                Ok(PoolSwapResult::from(&result))
            }
            PoolState::Balancer(b) => b.swap_with_amount(amount_in, zero_for_one)
        }
    }

    /// Swap to a target price
    ///
    /// # Arguments
    /// * `price_limit` - The target price to reach
    ///
    /// # Returns
    /// The swap result or an error if the swap fails
    pub fn swap_to_price(&self, price_limit: Price) -> eyre::Result<PoolSwapResult> {
        match self {
            PoolState::Uniswap(u) => {
                let sqrt_price_limit: crate::matching::SqrtPriceX96 = price_limit
                    .try_into()
                    .map_err(|_| eyre::eyre!("Invalid price for Uniswap"))?;
                let result = u.swap_current_to_price(sqrt_price_limit)?;
                Ok(PoolSwapResult::from(&result))
            }
            PoolState::Balancer(b) => b.swap_to_price(price_limit)
        }
    }

    /// Get a no-op swap result (no trading, same start/end price)
    pub fn noop(&self) -> PoolSwapResult {
        match self {
            PoolState::Uniswap(u) => {
                let result = u.noop();
                PoolSwapResult::from(&result)
            }
            PoolState::Balancer(b) => b.noop()
        }
    }

    /// Get a reference to the underlying Uniswap pool state if this is a
    /// Uniswap pool
    pub fn as_uniswap(&self) -> Option<&UniswapPoolState> {
        match self {
            PoolState::Uniswap(u) => Some(u),
            _ => None
        }
    }

    /// Get a reference to the underlying Balancer pool state if this is a
    /// Balancer pool
    pub fn as_balancer(&self) -> Option<&BalancerPoolState> {
        match self {
            PoolState::Balancer(b) => Some(b),
            _ => None
        }
    }
}
