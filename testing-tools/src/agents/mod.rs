use std::net::SocketAddr;

use uniswap_v4::uniswap::pool_manager::SyncedUniswapPools;

pub mod e2e_orders;

pub struct AgentConfig {
    pub uniswap_pools: SyncedUniswapPools,
    pub rpc_address:   SocketAddr
}
