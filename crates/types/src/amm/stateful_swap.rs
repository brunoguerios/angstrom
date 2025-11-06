//! Stateful swap result enum for AMM operations
//!
//! This module defines the StatefulPoolSwap enum that wraps venue-specific
//! swap results with stateful methods for chained swap simulation.

use alloy::primitives::I256;

use crate::{matching::SqrtPriceX96, sol_bindings::Ray, uni_structure::pool_swap::UniswapPoolSwapResult};

/// Stateful swap result that can continue simulating swaps
///
/// This enum wraps venue-specific swap results that maintain internal
/// state (liquidity positions, tick ranges, etc.) needed for chained
/// swap simulation during matching.
#[derive(Clone)]
pub enum StatefulPoolSwap<'a> {
    /// Uniswap V4 concentrated liquidity swap result
    Uniswap(UniswapPoolSwapResult<'a>),
    /// Balancer V3 swap result (to be implemented in Step 9)
    Balancer(BalancerPoolSwapResult<'a>)
}

impl<'a> StatefulPoolSwap<'a> {
    /// Continue swap to a target price from current state
    pub fn swap_to_price(&'a self, price: Ray) -> eyre::Result<Self> {
        match self {
            StatefulPoolSwap::Uniswap(uni) => {
                let sqrt_price = SqrtPriceX96::from(price);
                Ok(StatefulPoolSwap::Uniswap(uni.swap_to_price(sqrt_price)?))
            }
            StatefulPoolSwap::Balancer(bal) => {
                // TODO Step 9: Implement Balancer swap continuation
                Ok(StatefulPoolSwap::Balancer(bal.swap_to_price(price)?))
            }
        }
    }

    /// Continue swap with a given amount from current state
    pub fn swap_to_amount(&'a self, amount: I256, direction: bool) -> eyre::Result<Self> {
        match self {
            StatefulPoolSwap::Uniswap(uni) => {
                Ok(StatefulPoolSwap::Uniswap(uni.swap_to_amount(amount, direction)?))
            }
            StatefulPoolSwap::Balancer(bal) => {
                // TODO Step 9: Implement Balancer swap continuation
                Ok(StatefulPoolSwap::Balancer(bal.swap_to_amount(amount, direction)?))
            }
        }
    }

    /// Get token0 delta with sign (negative = input, positive = output)
    pub fn t0_signed(&self) -> I256 {
        match self {
            StatefulPoolSwap::Uniswap(uni) => uni.t0_signed(),
            StatefulPoolSwap::Balancer(bal) => bal.t0_signed()
        }
    }

    /// Get token1 delta with sign (negative = input, positive = output)
    pub fn t1_signed(&self) -> I256 {
        match self {
            StatefulPoolSwap::Uniswap(uni) => uni.t1_signed(),
            StatefulPoolSwap::Balancer(bal) => bal.t1_signed()
        }
    }

    /// Get total token0 moved in this swap
    pub fn total_d_t0(&self) -> u128 {
        match self {
            StatefulPoolSwap::Uniswap(uni) => uni.total_d_t0,
            StatefulPoolSwap::Balancer(bal) => bal.total_d_t0
        }
    }

    /// Get total token1 moved in this swap
    pub fn total_d_t1(&self) -> u128 {
        match self {
            StatefulPoolSwap::Uniswap(uni) => uni.total_d_t1,
            StatefulPoolSwap::Balancer(bal) => bal.total_d_t1
        }
    }

    /// Get the ending price after this swap
    pub fn end_price_ray(&self) -> Ray {
        match self {
            StatefulPoolSwap::Uniswap(uni) => Ray::from(uni.end_price),
            StatefulPoolSwap::Balancer(bal) => bal.end_price
        }
    }

    /// Check if this was an empty swap (no tokens moved)
    pub fn was_empty_swap(&self) -> bool {
        match self {
            StatefulPoolSwap::Uniswap(uni) => uni.was_empty_swap(),
            StatefulPoolSwap::Balancer(bal) => bal.was_empty_swap()
        }
    }
}

/// Placeholder for Balancer stateful swap result (Step 9)
#[derive(Clone)]
pub struct BalancerPoolSwapResult<'a> {
    pub total_d_t0: u128,
    pub total_d_t1: u128,
    pub end_price:  Ray,
    // TODO Step 9: Add actual balancer-maths-rust state here
    pub _marker:    std::marker::PhantomData<&'a ()>
}

impl<'a> BalancerPoolSwapResult<'a> {
    pub fn t0_signed(&self) -> I256 {
        // TODO Step 9: Implement proper sign calculation
        I256::ZERO
    }

    pub fn t1_signed(&self) -> I256 {
        // TODO Step 9: Implement proper sign calculation
        I256::ZERO
    }

    pub fn was_empty_swap(&self) -> bool {
        self.total_d_t0 == 0 || self.total_d_t1 == 0
    }

    pub fn swap_to_price(&self, _price: Ray) -> eyre::Result<Self> {
        Err(eyre::eyre!("Balancer swap_to_price not yet implemented (Step 9)"))
    }

    pub fn swap_to_amount(&self, _amount: I256, _direction: bool) -> eyre::Result<Self> {
        Err(eyre::eyre!("Balancer swap_to_amount not yet implemented (Step 9)"))
    }
}

