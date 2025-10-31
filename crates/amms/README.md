# AMMs Crate

This crate provides unified types for different Automated Market Maker (AMM) pool managers used throughout the Angstrom system.

## Purpose

The `amms` crate was created to solve circular dependency issues that arose when trying to share AMM pool types across multiple crates (`validation`, `consensus`, and `amm-quoter`). By placing the unified `SyncedPools` enum in this foundational crate, all other crates can depend on it without creating circular dependencies.

## Types

### `SyncedPools`

A unified enum that abstracts over different AMM pool manager implementations:

```rust
pub enum SyncedPools {
    Uniswap(SyncedUniswapPools),
    Balancer(SyncedBalancerPools)
}
```

### Helper Methods

- `as_uniswap()` - Returns a reference to Uniswap pools if this is a Uniswap variant
- `as_balancer()` - Returns a reference to Balancer pools if this is a Balancer variant
- `default_uniswap()` - Creates a default Uniswap pools instance (for testing)
- `default_balancer()` - Creates a default Balancer pools instance (for testing)

### From Implementations

The enum provides `From` implementations for easy conversion from concrete pool types:

```rust
let uniswap_pools = SyncedUniswapPools::new(...);
let synced_pools: SyncedPools = uniswap_pools.into();
```

## Extensibility

This design makes it easy to add support for additional AMMs in the future. Simply:

1. Add a new variant to the `SyncedPools` enum
2. Implement the corresponding helper method (e.g., `as_curve()`)
3. Add a `From` implementation for the new pool type

## Dependencies

This crate depends on:
- `uniswap-v4` - For `SyncedUniswapPools` type
- `balancer-v3` - For `SyncedBalancerPools` type
- `tokio` - For async channel types used in pool initialization

