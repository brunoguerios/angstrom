//! AMM-agnostic shared contract types

mod order;
mod registry;
mod tob;

pub use order::{OrderQuantities, StandingValidation, UserOrder};
pub use registry::{
    AngPoolConfigEntry, AngstromPoolConfigStore, AngstromPoolPartialKey, UniswapAngstromRegistry
};
pub use tob::*;

// Common types used by both Uniswap and Balancer:
// - TopOfBlockOrder (from tob module)
// - UserOrder (from order module)
// - OrderQuantities (from order module)
// - StandingValidation (from order module)
// - Pool configuration types (from registry module)

// NOTE: AngstromBundle has been moved to the uniswap module
// NOTE: Bundle building logic has been moved to the uniswap::builder module
