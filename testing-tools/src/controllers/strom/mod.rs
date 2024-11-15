mod node;
use alloy_rpc_types::Transaction;
use angstrom::components::initialize_strom_handles;
use angstrom_types::testnet::InitialTestnetState;
use consensus::AngstromValidator;
pub use node::*;
mod strom_internals;
use reth_chainspec::Hardforks;
use reth_network::config::SecretKey;
use reth_provider::{BlockReader, ChainSpecProvider, HeaderProvider};
use secp256k1::PublicKey;
pub use strom_internals::*;
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use tracing::instrument;

use crate::{network::TestnetNodeNetwork, types::TestingConfig};

#[instrument(name = "node", skip( node_id, c, pk, sk, initial_validators, inital_angstrom_state, config, block_provider), fields(id = node_id))]
pub async fn initialize_new_node<C>(
    c: C,
    node_id: Option<u64>,
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
        + 'static
{
    tracing::info!("spawning node");
    let strom_handles = initialize_strom_handles();
    let (strom_network, mut eth_peer, mut strom_network_manager) =
        TestnetNodeNetwork::new_fully_configed(
            c,
            pk,
            sk,
            Some(strom_handles.pool_tx.clone()),
            Some(strom_handles.consensus_tx_op.clone())
        )
        .await;

    // if self.config.is_testnet() {
    //     let connections_needed = initial_validators.len();
    //     tracing::debug!(pubkey = ?strom_network.pubkey(), "attempting connections
    // to {connections_needed} peers");     let mut last_peer_count = 0;
    //     std::future::poll_fn(|cx| loop {
    //         if eth_peer.poll_unpin(cx).is_ready()
    //             || strom_network_manager.poll_unpin(cx).is_ready()
    //         {
    //             panic!("peer connection failed");
    //         }

    //         let peer_cnt = strom_network.strom_handle.peer_count();
    //         if last_peer_count != peer_cnt {
    //             tracing::trace!("connected to {peer_cnt}/{connections_needed}
    // peers");             last_peer_count = peer_cnt;
    //         }

    //         if connections_needed == peer_cnt {
    //             return std::task::Poll::Ready(())
    //         }
    //     })
    //     .await;
    // }

    Ok(TestnetNode::new(
        node_id,
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
