use std::{collections::HashSet, pin::Pin};

use alloy::providers::ext::AnvilApi;
use alloy_primitives::U256;
use angstrom_types::{block_sync::GlobalBlockSync, testnet::InitialTestnetState};
use futures::{Future, StreamExt};
use reth_chainspec::Hardforks;
use reth_provider::{BlockReader, ChainSpecProvider, HeaderProvider};
use reth_tasks::TaskExecutor;
use tokio_stream::StreamExt;

use super::AngstromTestnet;
use crate::{
    agents::AgentConfig,
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
    pub async fn spawn_testnet<F>(c: C, config: TestnetConfig, agents: Vec<F>) -> eyre::Result<Self>
    where
        F: for<'a> Fn(
            &'a InitialTestnetState,
            AgentConfig
        ) -> Pin<Box<dyn Future<Output = eyre::Result<()>> + Send + 'a>>,
        F: Clone
    {
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
        this.spawn_new_testnet_nodes(c, agents).await?;
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

        let _ = futures::future::select_all(all_peers).await;
    }

    async fn spawn_new_testnet_nodes<F>(&mut self, c: C, agents: Vec<F>) -> eyre::Result<()>
    where
        F: for<'a> Fn(
            &'a InitialTestnetState,
            AgentConfig
        ) -> Pin<Box<dyn Future<Output = eyre::Result<()>> + Send + 'a>>,
        F: Clone
    {
        let mut initial_angstrom_state = None;

        let mut configs = (0..self.config.node_count())
            .map(|_| {
                let node_id = self.incr_peer_id();
                TestingNodeConfig::new(node_id, self.config.clone(), 100)
            })
            .collect::<Vec<_>>();

        let initial_validators = configs
            .iter()
            .map(|node_config| node_config.angstrom_validator())
            .collect::<Vec<_>>();

        // initialize leader provider
        let front = configs.remove(0);
        let l_block_sync = GlobalBlockSync::new(0);
        let leader_provider = self
            .initalize_leader_provider(l_block_sync, front, &mut initial_angstrom_state)
            .await;

        let nodes = futures::stream::iter(configs.into_iter())
            .map(|node_config| async {
                let node_id = node_config.node_id;
                let block_sync = if node_id == 0 { l_block_sync } else { GlobalBlockSync::new(0) };

                tracing::info!(node_id, "connecting to state provider");
                let provider = if self.config.is_leader(node_id) {
                    leader_provider
                } else {
                    tracing::info!(?node_id, "follower node init");
                    AnvilProvider::new(
                        WalletProvider::new(node_config.clone()),
                        true,
                        block_sync.clone()
                    )
                    .await?
                };

                tracing::info!(node_id, "connected to state provider");

                let mut node = TestnetNode::new(
                    c.clone(),
                    node_config.clone(),
                    provider,
                    initial_validators.clone(),
                    initial_angstrom_state.clone().unwrap(),
                    self.block_provider.subscribe_to_new_blocks(),
                    agents.clone(),
                    block_sync
                )
                .await?;

                tracing::info!(node_id, "made angstrom node");

                eyre::Ok((node_id, node))
            })
            .buffer_unordered(100)
            .collect::<Vec<_>>()
            .await;

        for res in nodes {
            let (node_id, node) = res?;
            node.connect_to_all_peers(&mut self.peers).await;
            self.peers.insert(node_id, node);
        }

        Ok(())
    }

    async fn initalize_leader_provider(
        &self,
        block_sync: GlobalBlockSync,
        node_config: TestingNodeConfig<TestnetConfig>,
        initial_angstrom_state: &mut Option<InitialTestnetState>
    ) -> AnvilProvider<WalletProvider> {
        let (p, initial_state) = self
            .leader_initialization(node_config.clone(), block_sync.clone())
            .await?;
        *initial_angstrom_state = Some(initial_state);
        p
    }

    async fn leader_initialization(
        &mut self,
        config: TestingNodeConfig<TestnetConfig>,
        block_sync: GlobalBlockSync
    ) -> eyre::Result<(AnvilProvider<WalletProvider>, InitialTestnetState)> {
        let mut provider =
            AnvilProvider::new(AnvilInitializer::new(config.clone()), true, block_sync).await?;
        self._anvil_instance = Some(provider._instance.take().unwrap());

        let initializer = provider.provider_mut().provider_mut();
        initializer.deploy_pool_fulls(config.pool_keys()).await?;

        let initial_state = initializer.initialize_state_no_bytes().await?;
        initializer
            .rpc_provider()
            .anvil_mine(Some(U256::from(10)), None)
            .await?;

        Ok((provider.into_state_provider(), initial_state))
    }
}
