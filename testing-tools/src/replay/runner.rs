use std::sync::Arc;

use alloy::{
    network::{Ethereum, EthereumWallet},
    node_bindings::{Anvil, AnvilInstance},
    signers::local::PrivateKeySigner
};
use angstrom::components::initialize_strom_handles;
use angstrom_types::primitive::{AngstromSigner, init_with_chain_id, try_init_with_chain_id};
use reth_chainspec::Hardforks;
use reth_network::NetworkHandle;
use reth_provider::{
    BlockReader, ChainSpecProvider, HeaderProvider, ReceiptProvider, noop::NoopProvider
};
use telemetry::blocklog::BlockLog;

use crate::{
    controllers::strom::initialize_strom_components_at_block,
    providers::{
        AnvilProvider, AnvilStateProvider, TestnetBlockProvider, WalletProvider,
        utils::async_to_sync
    },
    types::{GlobalTestingConfig, WithWalletProvider}
};

/// Replay Runner
///
/// The Replay Runner allows us to take any snapshot of a block and replay it
/// in order to allow us to debug quickly.
pub struct ReplayRunner
//<C, W>
// where
//     C: BlockReader<Block = reth_primitives::Block>
//         + ReceiptProvider<Receipt = reth_primitives::Receipt>
//         + HeaderProvider<Header = reth_primitives::Header>
//         + ChainSpecProvider<ChainSpec: Hardforks>
//         + Unpin
//         + Clone
//         + 'static,
//     W: WithWalletProvider, {
{
    anvil:          AnvilInstance,
    anvil_provider: AnvilProvider<WalletProvider>
}

impl ReplayRunner {
    pub async fn new(block_log: BlockLog, fork_url: String) -> Self {
        let chain_id = block_log.constants.as_ref().unwrap().chain_id;
        try_init_with_chain_id(chain_id).unwrap();

        let endpoint = "/tmp/anvil-replay.ipc";

        let anvil = Anvil::new()
            .fork_block_number(block_log.blocknum())
            .chain_id(chain_id)
            .fork(fork_url)
            .arg("--ipc")
            .arg(endpoint)
            // We not needed for the given block.
            .arg("--no-mining")
            .spawn();

        let node_addr = block_log.constants.as_ref().unwrap().node_address();
        let sk = AngstromSigner::for_address(node_addr).into_signer();

        let wallet = EthereumWallet::new(sk.clone());

        tracing::info!(?endpoint);

        let rpc = alloy::providers::builder::<Ethereum>()
            .with_recommended_fillers()
            .wallet(wallet)
            .connect(&endpoint)
            .await
            .unwrap();

        tracing::info!("connected to anvil");

        let wallet_provider = WalletProvider::new_with_provider(rpc, sk);
        let state_provider = AnvilStateProvider::new(wallet_provider);
        let anvil_provider = AnvilProvider::new(state_provider, None, None);
        let reth_provider = NoopProvider::mainnet();

        let strom_handles = initialize_strom_handles();
        let query_provider = Arc::new(anvil_provider.rpc_provider());

        Self { anvil, anvil_provider }
    }
}
