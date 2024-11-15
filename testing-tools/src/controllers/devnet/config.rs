use std::fmt::Display;

use alloy::node_bindings::Anvil;

#[derive(Debug, Clone)]
pub struct DevnetConfig {
    pub anvil_key:         usize,
    pub intial_node_count: u64,
    pub initial_rpc_port:  u16,
    pub testnet_kind:      TestnetKind,
    pub start_block:       Option<u64>,
    pub fork_url:          Option<String>
}

impl DevnetConfig {
    pub fn new(
        anvil_key: usize,
        intial_node_count: u64,
        initial_rpc_port: u16,
        testnet_kind: TestnetKind,
        start_block: Option<u64>,
        fork_url: Option<String>
    ) -> Self {
        Self { anvil_key, intial_node_count, initial_rpc_port, testnet_kind, start_block, fork_url }
    }

    pub fn rpc_port_with_node_id(&self, node_id: Option<u64>) -> u64 {
        if let Some(id) = node_id {
            self.initial_rpc_port as u64 + id
        } else {
            self.initial_rpc_port as u64
        }
    }

    pub fn is_devnet(&self) -> bool {
        matches!(self.testnet_kind, TestnetKind::Devnet)
    }

    pub fn is_testnet(&self) -> bool {
        matches!(self.testnet_kind, TestnetKind::Testnet { .. })
    }

    pub fn anvil_endpoint(&self, node_id: Option<impl Display>) -> String {
        if self.is_devnet() {
            format!("/tmp/anvil_{}.ipc", node_id.unwrap())
        } else {
            format!("/tmp/anvil.ipc")
        }
    }

    pub fn configure_anvil(&self, node_id: Option<impl Display>) -> Anvil {
        let mut anvil_builder = Anvil::new()
            .chain_id(1)
            .arg("--ipc")
            .arg(self.anvil_endpoint(node_id))
            .arg("--code-size-limit")
            .arg("393216")
            .arg("--disable-block-gas-limit");

        if self.is_devnet() {
            anvil_builder = anvil_builder.arg("--no-mining");
        }

        if let Some((start_block, fork_url)) = self.fork_config() {
            anvil_builder = anvil_builder
                .fork(fork_url)
                .arg("--fork-block-number")
                .arg(format!("{}", start_block));
        } else if self.is_testnet() {
            panic!("fork url and start block must be specified in testnet mode")
        }

        anvil_builder
    }

    pub fn fork_block_number(&self) -> Option<u64> {
        self.start_block
    }

    fn fork_config(&self) -> Option<(u64, String)> {
        self.start_block.zip(self.fork_url.clone())
    }
}

impl Default for DevnetConfig {
    fn default() -> Self {
        Self {
            anvil_key:         7,
            intial_node_count: 2,
            initial_rpc_port:  4200,
            testnet_kind:      TestnetKind::new_devnet(),
            start_block:       None,
            fork_url:          None
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum TestnetKind {
    #[default]
    Devnet,
    Testnet {
        is_leader: bool
    }
}

impl TestnetKind {
    pub fn new_devnet() -> Self {
        Self::Devnet
    }

    pub fn new_testnet(is_leader: bool) -> Self {
        Self::Testnet { is_leader }
    }
}
