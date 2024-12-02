use std::collections::HashSet;

use alloy::providers::ext::AnvilApi;
use alloy_primitives::U256;
use reth_chainspec::Hardforks;
use reth_provider::{BlockReader, ChainSpecProvider, HeaderProvider};

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
        tracing::info!("initialization testnet with {} nodes", config.node_count());

        Ok(this)
    }

    pub async fn run_to_completion(mut self) {
        let all_peers = std::mem::take(&mut self.peers)
            .into_values()
            .map(|peer| tokio::spawn(peer.testnet_future()));

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
            let mut provider = if self.config.is_leader(node_id) {
                tracing::info!(?node_id, "is leader init");
                let mut initializer =
                    AnvilProvider::new(AnvilInitializer::new(node_config.clone())).await?;

                let provider = initializer.provider_mut().provider_mut();
                provider.deploy_pool_full().await?;

                provider
                    .rpc_provider()
                    .anvil_mine(Some(U256::from(5)), None)
                    .await?;

                let initial_state = provider.initialize_state().await?;
                println!("{initial_state:?}");

                initial_angstrom_state = Some(initial_state);
                initializer.into_state_provider()
            } else {
                tracing::info!(?node_id, "default init");
                let state_bytes = initial_angstrom_state.clone().unwrap().state.unwrap();
                let provider = AnvilProvider::new(WalletProvider::new(node_config.clone())).await?;
                provider.set_state(state_bytes).await?;

                provider
            };

            let instance = provider._instance.take();
            if instance.is_some() {
                self._anvil_instance = instance;
            }

            tracing::info!(node_id, "connected to state provider");

            let mut node = TestnetNode::new(
                c.clone(),
                node_config,
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
}
