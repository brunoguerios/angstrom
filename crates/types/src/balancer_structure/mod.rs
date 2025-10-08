//! Balancer V3 pool structures and implementations
//!
//! This module provides a thin wrapper around the `balancer-maths-rust` crate,
//! implementing the PoolSim trait for Balancer pool types.
//!
//! The `balancer-maths-rust` crate handles all pool-specific math for:
//! - ReClamm pools (concentrated liquidity)
//! - Weighted pools
//! - Stable pools
//! - And other Balancer pool types
//!
//! We simply adapt these to work with Angstrom's PoolSim interface.

pub mod pool_state;

// Re-export balancer-maths-rust for direct access when needed
pub use balancer_maths_rust;
pub use pool_state::BalancerPoolState;
