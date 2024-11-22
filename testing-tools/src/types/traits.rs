use std::{
    fmt::{Debug, Display},
    future::Future
};

use alloy::node_bindings::{Anvil, AnvilInstance};

use crate::providers::{utils::WalletProviderRpc, WalletProvider};

pub trait TestingConfig: Debug + Clone {
    fn configure_anvil(&self, node_id: u64) -> Anvil;

    fn spawn_rpc(
        &self,
        node_id: u64
    ) -> impl Future<Output = eyre::Result<(WalletProvider, Option<AnvilInstance>)>> + Send;

    fn anvil_endpoint(&self, id: u64) -> String;

    fn rpc_port(&self, node_id: Option<u64>) -> u64;
}

pub trait WithWalletProvider: Send + Sync {
    fn wallet_provider(&self) -> WalletProvider;

    fn provider(&self) -> WalletProviderRpc;

    fn initialize(
        node_id: u64,
        config: impl TestingConfig + Send
    ) -> impl Future<Output = eyre::Result<(Self, Option<AnvilInstance>)>> + Send
    where
        Self: Sized;
}
