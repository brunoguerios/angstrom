//! End AMM state representation after matching
//!
//! This module defines the EndAmmState enum for venue-specific end states

use alloy::primitives::U160;
use angstrom_types::sol_bindings::Ray;

/// End AMM state after matching, venue-specific
///
/// Different AMMs have different state representations:
/// - Uniswap: SqrtPriceX96, tick, liquidity
/// - Balancer: Ray price, reserves/balances
#[derive(Debug, Clone)]
pub enum EndAmmState {
    /// Uniswap V4 end state
    Uniswap {
        price:     U160, // SqrtPriceX96
        tick:      i32,
        liquidity: u128
    },
    /// Balancer V3 end state
    // TODO Step 9: Add reserve/balance info if needed
    Balancer {
        price: Ray // Spot price in Ray
    }
}

impl EndAmmState {
    /// Get price as Ray (common across venues)
    pub fn price_ray(&self) -> Ray {
        match self {
            EndAmmState::Uniswap { price, .. } => {
                use angstrom_types::matching::SqrtPriceX96;
                Ray::from(SqrtPriceX96::from(*price))
            }
            EndAmmState::Balancer { price, .. } => *price
        }
    }
}
