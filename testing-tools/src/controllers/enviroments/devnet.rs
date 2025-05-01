use std::{collections::HashSet, pin::Pin};

use alloy::providers::ext::AnvilApi;
use angstrom_types::{block_sync::GlobalBlockSync, testnet::InitialTestnetState};
use futures::{Future, FutureExt};
use itertools::Itertools;
use reth_chainspec::Hardforks;
use reth_provider::{BlockReader, ChainSpecProvider, HeaderProvider, ReceiptProvider};
use reth_tasks::TaskExecutor;

use super::AngstromTestnet;
use crate::{
    agents::AgentConfig,
    controllers::{enviroments::DevnetStateMachine, strom::TestnetNode},
    providers::{AnvilInitializer, AnvilProvider, TestnetBlockProvider, WalletProvider},
    types::{
        GlobalTestingConfig,
        config::{DevnetConfig, TestingNodeConfig},
        initial_state::PartialConfigPoolKey
    }
};

impl<C> AngstromTestnet<C, DevnetConfig, WalletProvider>
where
    C: BlockReader<Block = reth_primitives::Block>
        + ReceiptProvider<Receipt = reth_primitives::Receipt>
        + HeaderProvider<Header = reth_primitives::Header>
        + Unpin
        + Clone
        + ChainSpecProvider<ChainSpec: Hardforks>
        + 'static
{
    pub async fn spawn_devnet(c: C, config: DevnetConfig, ex: TaskExecutor) -> eyre::Result<Self> {
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

        tracing::info!("initializing devnet with {} nodes", config.node_count());
        this.spawn_new_devnet_nodes(c, ex).await?;
        tracing::info!("initialization devnet with {} nodes", config.node_count());

        Ok(this)
    }

    pub fn as_state_machine<'a>(self) -> DevnetStateMachine<'a, C> {
        DevnetStateMachine::new(self)
    }

    async fn spawn_new_devnet_nodes(&mut self, c: C, ex: TaskExecutor) -> eyre::Result<()> {
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
        let node_addresses = configs
            .iter()
            .map(|c| c.angstrom_signer().address())
            .collect::<Vec<_>>();

        for node_config in configs {
            let node_id = node_config.node_id;
            let block_sync = GlobalBlockSync::new(0);
            tracing::info!(node_id, "connecting to state provider");
            let provider = if self.config.is_leader(node_id) {
                let mut initializer = AnvilProvider::new(
                    AnvilInitializer::new(node_config.clone(), node_addresses.clone())
                        .then(async |v| v.map(|i| (i.0, i.1, Some(i.2)))),
                    false,
                    block_sync.clone()
                )
                .await?;
                let provider = initializer.provider_mut().provider_mut();
                let initial_state = provider.initialize_state(ex.clone()).await?;
                initial_angstrom_state = Some(initial_state);

                initializer.rpc_provider().anvil_mine(Some(5), None).await?;
                initializer.into_state_provider()
            } else {
                tracing::info!(?node_id, "default init");
                let state_bytes = initial_angstrom_state.clone().unwrap().state.unwrap();
                let provider = AnvilProvider::new(
                    WalletProvider::new(node_config.clone())
                        .then(async |v| v.map(|i| (i.0, i.1, None))),
                    false,
                    block_sync.clone()
                )
                .await?;
                provider.set_state(state_bytes).await?;
                provider.rpc_provider().anvil_mine(Some(5), None).await?;
                provider
            };
            tracing::info!(node_id, "connected to state provider");

            let mut node = TestnetNode::new(
                c.clone(),
                node_config,
                provider,
                initial_validators.clone(),
                initial_angstrom_state.clone().unwrap(),
                self.block_provider.subscribe_to_new_blocks(),
                vec![a],
                block_sync,
                ex.clone()
            )
            .await?;
            tracing::info!(node_id, "made angstrom node");

            node.connect_to_all_peers(&mut self.peers).await;
            tracing::info!(node_id, "connected to all peers");

            self.peers.insert(node_id, node);

            if node_id != 0 {
                self.single_peer_update_state(0, node_id).await?;
            }
        }

        Ok(())
    }

    /// deploys a new pool
    pub async fn deploy_new_pool(&mut self, pool_key: PartialConfigPoolKey) -> eyre::Result<()> {
        tracing::debug!("deploying new pool on state machine");
        let node = self.get_peer_with(|n| n.state_provider().deployed_addresses().is_some());
        node.start_network_and_consensus_and_validation();
        let provider = node.state_provider();
        let config = node.testnet_node_config();

        let mut initializer = AnvilInitializer::new_existing(provider, config);
        initializer.deploy_pool_fulls(vec![pool_key]).await?;

        node.stop_network_and_consensus_and_validation();

        Ok(())
    }
}

fn a<'a>(
    _: &'a InitialTestnetState,
    _: AgentConfig
) -> Pin<Box<dyn Future<Output = eyre::Result<()>> + Send + 'a>> {
    Box::pin(async { eyre::Ok(()) })
}
