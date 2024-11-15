use std::fmt::Display;

use alloy::{
    network::{Ethereum, EthereumWallet},
    node_bindings::{Anvil, AnvilInstance},
    providers::{builder, IpcConnect, WsConnect},
    signers::local::PrivateKeySigner
};
use alloy_primitives::Address;
use enr::k256::ecdsa::SigningKey;

use crate::{anvil_state_provider::AnvilWallet, types::TestingConfig};

#[derive(Debug, Clone)]
pub struct TestnetConfig {
    pub anvil_key:     usize,
    pub node_count:    u64,
    pub leader_ws_url: String,
    pub address:       Address,
    pub sk:            PrivateKeySigner,
    pub leader_config: Option<TestnetLeaderConfig>
}

impl TestnetConfig {
    pub fn new(
        anvil_key: usize,
        node_count: u64,
        leader_ws_url: String,
        address: Address,
        sk: PrivateKeySigner,
        leader_config: Option<TestnetLeaderConfig>
    ) -> Self {
        Self { anvil_key, address, sk, node_count, leader_ws_url, leader_config }
    }

    pub fn is_leader(&self) -> bool {
        self.leader_config.is_some()
    }
}

impl TestingConfig for TestnetConfig {
    fn configure_anvil(&self, id: impl Display) -> Anvil {
        let Some(config) = self.leader_config.clone() else {
            panic!("only the leader can call this!")
        };

        Anvil::new()
            .chain_id(1)
            .fork(config.eth_fork_url)
            .fork_block_number(config.fork_block)
            .arg("--ipc")
            .arg(self.anvil_endpoint(id))
            .arg("--code-size-limit")
            .arg("393216")
            .arg("--disable-block-gas-limit")
    }

    async fn spawn_rpc(
        &self,
        id: impl Display + Clone
    ) -> eyre::Result<(AnvilWallet, Option<AnvilInstance>)> {
        let sk = self.sk.clone();
        let wallet = EthereumWallet::new(sk.clone());

        if self.is_leader() {
            let anvil = self.configure_anvil(id.clone()).try_spawn()?;

            let endpoint = self.anvil_endpoint(id);
            tracing::info!(?endpoint);

            let rpc = builder::<Ethereum>()
                .with_recommended_fillers()
                .wallet(wallet)
                .on_ipc(IpcConnect::new(endpoint))
                .await?;

            tracing::info!("connected to anvil");

            Ok((AnvilWallet::new(rpc, self.address, sk), Some(anvil)))
        } else {
            let rpc = builder::<Ethereum>()
                .with_recommended_fillers()
                .wallet(wallet)
                .on_ws(WsConnect::new(self.leader_ws_url.clone()))
                .await?;

            Ok((AnvilWallet::new(rpc, self.address, sk), None))
        }
    }

    fn rpc_port(&self, _: Option<u64>) -> u64 {
        4200
    }

    fn anvil_endpoint(&self, _: impl Display) -> String {
        format!("/tmp/anvil.ipc")
    }
}

#[derive(Debug, Clone)]
pub struct TestnetLeaderConfig {
    pub fork_block:   u64,
    pub eth_fork_url: String
}

impl TestnetLeaderConfig {
    pub fn new(fork_block: u64, eth_fork_url: String) -> Self {
        Self { fork_block, eth_fork_url }
    }
}
