use std::collections::HashSet;

use alloy::providers::ext::AnvilApi;
use alloy_primitives::U256;
use angstrom_types::testnet::InitialTestnetState;
use reth_chainspec::Hardforks;
use reth_provider::{BlockReader, ChainSpecProvider, HeaderProvider};
use reth_tasks::TaskExecutor;

use super::AngstromTestnet;
use crate::{
    controllers::strom::TestnetNode,
    providers::{AnvilInitializer, AnvilProvider, TestnetBlockProvider, WalletProvider},
    types::{
        config::{TestingNodeConfig, TestnetConfig},
        GlobalTestingConfig, WithWalletProvider
    }
};

impl<C> AngstromTestnet<C, TestnetConfig, WalletProvider>
where
    C: BlockReader
        + HeaderProvider
        + ChainSpecProvider
        + Unpin
        + Clone
        + ChainSpecProvider<ChainSpec: Hardforks>
        + 'static
{
    pub async fn spawn_testnet(c: C, config: TestnetConfig) -> eyre::Result<Self> {
        let block_provider = TestnetBlockProvider::new();
        let mut this = Self {
            peers: Default::default(),
            _disconnected_peers: HashSet::new(),
            _dropped_peers: HashSet::new(),
            current_max_peer_id: 0,
            config: config.clone(),
            block_provider,
            _anvil_instance: None
        };

        tracing::info!("initializing testnet with {} nodes", config.node_count());
        this.spawn_new_testnet_nodes(c).await?;
        tracing::info!("initialized testnet with {} nodes", config.node_count());

        Ok(this)
    }

    pub async fn run_to_completion(mut self, executor: TaskExecutor) {
        let all_peers = std::mem::take(&mut self.peers).into_values().map(|peer| {
            executor.spawn_critical_blocking(
                format!("testnet node {}", peer.testnet_node_id()).leak(),
                peer.testnet_future()
            )
        });

        futures::future::join_all(all_peers).await;
    }

    async fn spawn_new_testnet_nodes(&mut self, c: C) -> eyre::Result<()> {
        let mut initial_angstrom_state = None;

        let configs = (0..self.config.node_count())
            .map(|_| {
                let node_id = self.incr_peer_id();
                TestingNodeConfig::new(node_id, self.config.clone(), 100)
            })
            .collect::<Vec<_>>();

        let initial_validators = configs
            .iter()
            .map(|node_config| node_config.angstrom_validator())
            .collect::<Vec<_>>();

        for node_config in configs {
            let node_id = node_config.node_id;
            tracing::info!(node_id, "connecting to state provider");
            let provider = if self.config.is_leader(node_id) {
                tracing::info!(?node_id, "leader node init");
                let (p, initial_state) = self.leader_initialization(node_config.clone()).await?;
                initial_angstrom_state = Some(initial_state);
                p
            } else {
                tracing::info!(?node_id, "follower node init");
                AnvilProvider::new(WalletProvider::new(node_config.clone())).await?
            };

            tracing::info!(node_id, "connected to state provider");

            let mut node = TestnetNode::new(
                c.clone(),
                node_config.clone(),
                provider,
                initial_validators.clone(),
                initial_angstrom_state.clone().unwrap(),
                self.block_provider.subscribe_to_new_blocks()
            )
            .await?;

            tracing::info!(node_id, "made angstrom node");

            node.connect_to_all_peers(&mut self.peers).await;
            tracing::info!(node_id, "connected to all peers");

            self.peers.insert(node_id, node);
        }

        Ok(())
    }

    async fn leader_initialization(
        &mut self,

        config: TestingNodeConfig<TestnetConfig>
    ) -> eyre::Result<(AnvilProvider<WalletProvider>, InitialTestnetState)> {
        let mut provider = AnvilProvider::new(AnvilInitializer::new(config)).await?;
        self._anvil_instance = Some(provider._instance.take().unwrap());

        let initializer = provider.provider_mut().provider_mut();
        initializer.deploy_pool_full().await?;

        initializer
            .rpc_provider()
            .anvil_mine(Some(U256::from(5)), None)
            .await?;

        let initial_state = initializer.initialize_state_no_bytes().await?;

        Ok((provider.into_state_provider(), initial_state))
    }
}
