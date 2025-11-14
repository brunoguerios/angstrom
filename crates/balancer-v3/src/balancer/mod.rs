//! Balancer V3 pool integration

pub mod loaders;
pub mod pool;
pub mod pool_data_loader;
pub mod pool_factory;
pub mod pool_manager;

// pool_providers available for future Step 9.8 (event-driven manager)
#[cfg(feature = "event-driven")]
pub mod pool_providers;
