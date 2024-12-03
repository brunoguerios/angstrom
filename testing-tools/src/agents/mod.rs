use std::{net::SocketAddr, pin::Pin};

use futures::Stream;
use uniswap_v4::uniswap::pool_manager::SyncedUniswapPools;

// pub mod e2e_orders;

pub struct AgentConfig {
    pub uniswap_pools: SyncedUniswapPools,
    pub rpc_address:   SocketAddr,
    pub current_block: u64,
    pub block_stream:  Pin<Box<dyn Stream<Item = u64> + Send + Sync>>
}
