//! Unified AMM pool types for Angstrom
//!
//! This crate provides a common `SyncedPools` enum that can be used across
//! validation, consensus, and quoter modules without creating circular
//! dependencies.

use balancer_v3::balancer::pool_manager::SyncedBalancerPools;
use uniswap_v4::uniswap::pool_manager::SyncedUniswapPools;

/// Unified enum for different AMM pool types
#[derive(Clone)]
pub enum SyncedPools {
    Uniswap(SyncedUniswapPools),
    Balancer(SyncedBalancerPools)
}

impl SyncedPools {
    /// Creates a default Uniswap pools instance (empty, with dummy channel)
    pub fn default_uniswap() -> Self {
        SyncedPools::Uniswap(SyncedUniswapPools::new(
            Default::default(),
            tokio::sync::mpsc::channel(1).0
        ))
    }

    /// Creates a default Balancer pools instance (empty)
    pub fn default_balancer() -> Self {
        SyncedPools::Balancer(SyncedBalancerPools::new(Default::default()))
    }

    /// Returns a reference to the Uniswap pools if this is a Uniswap variant
    pub fn as_uniswap(&self) -> Option<&SyncedUniswapPools> {
        match self {
            SyncedPools::Uniswap(pools) => Some(pools),
            SyncedPools::Balancer(_) => None
        }
    }

    /// Returns a reference to the Balancer pools if this is a Balancer variant
    pub fn as_balancer(&self) -> Option<&SyncedBalancerPools> {
        match self {
            SyncedPools::Balancer(pools) => Some(pools),
            SyncedPools::Uniswap(_) => None
        }
    }
}

impl From<SyncedUniswapPools> for SyncedPools {
    fn from(value: SyncedUniswapPools) -> Self {
        SyncedPools::Uniswap(value)
    }
}

impl From<SyncedBalancerPools> for SyncedPools {
    fn from(value: SyncedBalancerPools) -> Self {
        SyncedPools::Balancer(value)
    }
}

impl Default for SyncedPools {
    fn default() -> Self {
        Self::default_uniswap()
    }
}
