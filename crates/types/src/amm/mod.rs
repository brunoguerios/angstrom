//! AMM-agnostic pool simulation abstractions
//!
//! This module provides venue-neutral interfaces for pool simulation,
//! allowing the system to work with multiple AMM types (Uniswap, Balancer,
//! etc.) without leaking venue-specific details.
//!
//! NOTE: This module contains ONLY neutral abstractions.
//! Venue-specific implementations live in:
//! - `uni_structure/` for Uniswap
//! - `balancer_structure/` for Balancer

pub mod pool_state;
pub mod pool_swap;
pub mod price;

pub use pool_state::{PoolState, PoolStateSnapshot};
pub use pool_swap::PoolSwapResult;
pub use price::Price;
