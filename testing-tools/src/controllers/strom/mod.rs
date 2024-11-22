mod node;
use angstrom::components::initialize_strom_handles;
use angstrom_types::testnet::InitialTestnetState;
use consensus::AngstromValidator;
pub use node::*;
mod internals;
pub use internals::*;
use reth_chainspec::Hardforks;
use reth_provider::{BlockReader, ChainSpecProvider, HeaderProvider};
use secp256k1::{PublicKey, SecretKey};
use tokio_stream::wrappers::BroadcastStream;
use tracing::instrument;

use crate::{
    anvil_state_provider::AnvilStateProviderWrapper,
    network::TestnetNodeNetwork,
    types::{TestingConfig, WithWalletProvider}
};

#[instrument(name = "node", skip(testnet_node_id, c, state_provider, pk, sk, initial_validators, inital_angstrom_state, config, block_provider), fields(id = node_id))]
pub async fn initialize_new_node<C, P>(
    c: C,
    testnet_node_id: u64,
    state_provider: AnvilStateProviderWrapper<P>,
    pk: PublicKey,
    sk: SecretKey,
    initial_validators: Vec<AngstromValidator>,
    inital_angstrom_state: InitialTestnetState,
    config: impl TestingConfig,
    block_provider: BroadcastStream<(u64, Vec<alloy_rpc_types::Transaction>)>
) -> eyre::Result<TestnetNode<C>>
where
    C: BlockReader
        + HeaderProvider
        + Unpin
        + Clone
        + ChainSpecProvider<ChainSpec: Hardforks>
        + 'static,
    P: WithWalletProvider
{
    tracing::info!("spawning node");
    let strom_handles = initialize_strom_handles();
    let (strom_network, eth_peer, strom_network_manager) = TestnetNodeNetwork::new_fully_configed(
        c,
        pk,
        sk,
        Some(strom_handles.pool_tx.clone()),
        Some(strom_handles.consensus_tx_op.clone())
    )
    .await;

    Ok(TestnetNode::new(
        testnet_node_id,
        state_provider,
        strom_network,
        strom_network_manager,
        eth_peer,
        strom_handles,
        config,
        initial_validators,
        block_provider,
        inital_angstrom_state
    )
    .await?)
}
