//! Balancer pool registry for managing pool addresses and metadata
//!
//! This module provides a registry for looking up Balancer V3 pool information
//! needed for submission building.

use std::collections::HashMap;

use alloy::primitives::Address;
use eyre::Result;

use crate::primitive::PoolId;

/// Balancer pool information
#[derive(Debug, Clone)]
pub struct BalancerPoolInfo {
    /// Pool contract address (Balancer V3)
    pub address: Address,
    /// Token 0 address (lower address)
    pub token0:  Address,
    /// Token 1 address (higher address)
    pub token1:  Address
}

/// Registry for Balancer pool lookups
///
/// Maps from internal PoolId to Balancer pool addresses and token pairs.
/// This is used by the submission builder to convert consensus outputs
/// into the explicit parameter format expected by the Balancer contract.
#[derive(Debug, Default, Clone)]
pub struct BalancerPoolRegistry {
    pools: HashMap<PoolId, BalancerPoolInfo>
}

impl BalancerPoolRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self { pools: HashMap::new() }
    }

    /// Add a pool to the registry
    pub fn add_pool(
        &mut self,
        pool_id: PoolId,
        address: Address,
        token0: Address,
        token1: Address
    ) {
        self.pools
            .insert(pool_id, BalancerPoolInfo { address, token0, token1 });
    }

    /// Get pool address for a given pool ID
    pub fn get_pool_address(&self, pool_id: PoolId) -> Result<Address> {
        self.pools
            .get(&pool_id)
            .map(|info| info.address)
            .ok_or_else(|| eyre::eyre!("Pool not found: {:?}", pool_id))
    }

    /// Get token pair for a given pool ID
    pub fn get_pool_tokens(&self, pool_id: PoolId) -> Result<(Address, Address)> {
        self.pools
            .get(&pool_id)
            .map(|info| (info.token0, info.token1))
            .ok_or_else(|| eyre::eyre!("Pool not found: {:?}", pool_id))
    }

    /// Get complete pool information
    pub fn get_pool_info(&self, pool_id: PoolId) -> Result<&BalancerPoolInfo> {
        self.pools
            .get(&pool_id)
            .ok_or_else(|| eyre::eyre!("Pool not found: {:?}", pool_id))
    }

    /// Check if a pool exists in the registry
    pub fn contains_pool(&self, pool_id: &PoolId) -> bool {
        self.pools.contains_key(pool_id)
    }

    /// Get all pool IDs
    pub fn pool_ids(&self) -> Vec<PoolId> {
        self.pools.keys().copied().collect()
    }

    /// Number of pools in the registry
    pub fn len(&self) -> usize {
        self.pools.len()
    }

    /// Check if the registry is empty
    pub fn is_empty(&self) -> bool {
        self.pools.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use alloy::primitives::FixedBytes;

    use super::*;

    #[test]
    fn test_add_and_get_pool() {
        let mut registry = BalancerPoolRegistry::new();
        let pool_id = FixedBytes::from([1u8; 32]);
        let pool_addr = Address::from([2u8; 20]);
        let token0 = Address::from([3u8; 20]);
        let token1 = Address::from([4u8; 20]);

        registry.add_pool(pool_id, pool_addr, token0, token1);

        let addr = registry.get_pool_address(pool_id).unwrap();
        assert_eq!(addr, pool_addr);

        let (t0, t1) = registry.get_pool_tokens(pool_id).unwrap();
        assert_eq!(t0, token0);
        assert_eq!(t1, token1);
    }

    #[test]
    fn test_get_nonexistent_pool() {
        let registry = BalancerPoolRegistry::new();
        let pool_id = FixedBytes::from([1u8; 32]);

        let result = registry.get_pool_address(pool_id);
        assert!(result.is_err());
    }

    #[test]
    fn test_contains_pool() {
        let mut registry = BalancerPoolRegistry::new();
        let pool_id = FixedBytes::from([1u8; 32]);
        let pool_addr = Address::from([2u8; 20]);
        let token0 = Address::from([3u8; 20]);
        let token1 = Address::from([4u8; 20]);

        assert!(!registry.contains_pool(&pool_id));

        registry.add_pool(pool_id, pool_addr, token0, token1);
        assert!(registry.contains_pool(&pool_id));
    }

    #[test]
    fn test_pool_ids() {
        let mut registry = BalancerPoolRegistry::new();
        let pool_id1 = FixedBytes::from([1u8; 32]);
        let pool_id2 = FixedBytes::from([2u8; 32]);
        let pool_addr = Address::from([2u8; 20]);
        let token0 = Address::from([3u8; 20]);
        let token1 = Address::from([4u8; 20]);

        registry.add_pool(pool_id1, pool_addr, token0, token1);
        registry.add_pool(pool_id2, pool_addr, token0, token1);

        let ids = registry.pool_ids();
        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&pool_id1));
        assert!(ids.contains(&pool_id2));
    }
}
