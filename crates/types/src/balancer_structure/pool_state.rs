//! PoolState implementation for Balancer pools
//!
//! This module implements the PoolState trait for Balancer pool types
//! from the `balancer-maths-rust` crate.
//!
//! TODO: As we explore the balancer-maths-rust API, we'll:
//! 1. Understand what types it exports (PoolState, ReClamm, Weighted, etc.)
//! 2. Understand what functions it provides for swaps and price calculations
//! 3. Implement proper conversions to/from our neutral Price type
//! 4. Implement PoolState by delegating to balancer-maths-rust functions

use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::amm::{PoolState, PoolSwapResult, Price};

/// Wrapper around balancer-maths-rust pool state
///
/// This is a temporary placeholder until we explore the actual API
/// from balancer-maths-rust and understand what types it exports.
///
/// Expected API from balancer-maths-rust (to be verified):
/// - balancer_maths_rust::PoolState or similar
/// - balancer_maths_rust::swap_exact_in()
/// - balancer_maths_rust::swap_exact_out()
/// - balancer_maths_rust::get_spot_price()
///
/// Once we understand the API, we'll either:
/// A) Directly wrap their types
/// B) Implement PoolSim directly on their types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalancerPoolState {
    /// Pool identifier
    pub pool_id: String,

    /// Current block number
    pub block_number: u64,

    /// Swap fee in parts per million
    pub fee_ppm: u32 /* TODO: Add the actual balancer-maths-rust pool state here
                      * Expected something like:
                      * pub inner: balancer_maths_rust::PoolState, */
}

impl BalancerPoolState {
    /// Create a new Balancer pool state
    ///
    /// TODO: This constructor will need to be updated once we understand
    /// how to construct balancer-maths-rust pool states
    pub fn new(pool_id: String, block_number: u64, fee_ppm: u32) -> Self {
        Self { pool_id, block_number, fee_ppm }
    }
}

impl Display for BalancerPoolState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BalancerPool(id={}, block={}, fee={}ppm)",
            self.pool_id, self.block_number, self.fee_ppm
        )
    }
}

impl PoolState for BalancerPoolState {
    fn block_number(&self) -> u64 {
        self.block_number
    }

    fn fee_ppm(&self) -> u32 {
        self.fee_ppm
    }

    fn current_price(&self) -> Price {
        // TODO: Use balancer-maths-rust to calculate current price
        // Expected API: balancer_maths_rust::get_spot_price(&self.inner)
        Price::new(1_000_000_000_000_000_000) // Placeholder
    }

    fn swap_with_amount(
        &self,
        _amount_in: u128,
        _zero_for_one: bool
    ) -> eyre::Result<PoolSwapResult> {
        // TODO: Use balancer-maths-rust to perform swap
        // Expected API:
        // let result = balancer_maths_rust::swap_exact_in(
        //     &self.inner,
        //     amount_in,
        //     zero_for_one
        // )?;
        //
        // Then convert result to PoolSwapResult

        Err(eyre::eyre!(
            "Balancer swap with amount not yet implemented - need to integrate \
             balancer-maths-rust API"
        ))
    }

    fn swap_to_price(&self, _price_limit: Price) -> eyre::Result<PoolSwapResult> {
        // TODO: Use balancer-maths-rust to swap to target price
        // This might require:
        // 1. Binary search on amount
        // 2. Or a direct "swap_to_price" function if available

        Err(eyre::eyre!(
            "Balancer swap to price not yet implemented - need to integrate balancer-maths-rust \
             API"
        ))
    }

    fn noop(&self) -> PoolSwapResult {
        let current_price = self.current_price();
        PoolSwapResult::new(self.fee_ppm, current_price, current_price, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balancer_pool_creation() {
        let pool = BalancerPoolState::new("test-pool".to_string(), 12345, 3000);
        assert_eq!(pool.pool_id, "test-pool");
        assert_eq!(pool.block_number(), 12345);
        assert_eq!(pool.fee_ppm(), 3000);
    }

    #[test]
    fn test_noop() {
        let pool = BalancerPoolState::new("test-pool".to_string(), 12345, 3000);
        let outcome = pool.noop();
        assert!(outcome.is_empty());
        assert_eq!(outcome.fee_ppm, 3000);
    }

    #[test]
    #[ignore = "Balancer math not yet integrated"]
    fn test_simulate_swap() {
        let pool = BalancerPoolState::new("test-pool".to_string(), 12345, 3000);
        let result = pool.swap_with_amount(1000, true);
        // This should work once we integrate balancer-maths-rust
        assert!(result.is_err()); // Currently returns error
    }
}
