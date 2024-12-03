use std::{net::SocketAddr, pin::Pin};

use uniswap_v4::uniswap::pool_manager::SyncedUniswapPools;

use crate::providers::{AnvilStateProvider, WalletProvider};

// pub mod e2e_orders;

#[derive(Clone)]
pub struct AgentConfig {
    pub uniswap_pools:  SyncedUniswapPools,
    pub rpc_address:    SocketAddr,
    pub current_block:  u64,
    pub state_provider: AnvilStateProvider<WalletProvider>
}
