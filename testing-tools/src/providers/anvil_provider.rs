use std::{fmt::Display, future::IntoFuture};

use alloy::{
    network::{Ethereum, EthereumWallet},
    node_bindings::{Anvil, AnvilInstance},
    providers::{builder, ext::AnvilApi, Provider},
    rpc::types::{anvil::MineOptions, Block},
    signers::local::PrivateKeySigner
};
use alloy_primitives::{Address, BlockNumber, Bytes, B256, U256};
use angstrom_types::contract_bindings::angstrom::Angstrom::AngstromInstance;
use eyre::bail;
use reth_provider::{
    BlockHashReader, BlockNumReader, CanonStateNotification, CanonStateNotifications,
    CanonStateSubscriptions, ProviderError, ProviderResult
};
use reth_revm::primitives::Bytecode;
use tokio::sync::broadcast;
use validation::common::db::BlockStateProviderFactory;

use super::{
    rpc_provider::RpcStateProvider, utils::WalletProviderRpc, AnvilInitializer, AnvilStateProvider,
    WalletProvider
};
use crate::{
    contracts::environment::uniswap::UniswapEnv,
    mocks::canon_state::AnvilConsensusCanonStateNotification,
    providers::utils::async_to_sync,
    types::{TestingConfig, WithWalletProvider}
};

#[derive(Debug)]
pub struct AnvilProvider<P> {
    provider:  AnvilStateProvider<P>,
    _instance: Option<AnvilInstance>
}
impl<P: WithWalletProvider> AnvilProvider<P> {
    pub async fn spawn_new(id: u64, config: impl TestingConfig) -> eyre::Result<Self> {
        let (provider, anvil) = AnvilStateProvider::<P>::initialize(id, config).await?;

        Ok(Self { provider, _instance: anvil })
    }

    pub async fn spawn_new_isolated() -> eyre::Result<Self> {
        let anvil = Anvil::new()
            .block_time(12)
            .chain_id(1)
            .arg("--ipc")
            .arg("--code-size-limit")
            .arg("393216")
            .arg("--disable-block-gas-limit")
            .try_spawn()?;

        let ipc = alloy::providers::IpcConnect::new("/tmp/anvil.ipc".to_string());
        let sk: PrivateKeySigner = anvil.keys()[7].clone().into();
        let controller_address = anvil.addresses()[7];

        let wallet = EthereumWallet::new(sk.clone());
        let rpc = builder::<Ethereum>()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_ipc(ipc)
            .await?;

        tracing::info!("connected to anvil");

        let (tx, _) = broadcast::channel(1000);

        Ok(Self {
            provider:  AnvilStateProvider {
                provider:       WalletProvider::new(rpc, controller_address, sk),
                canon_state_tx: tx,
                canon_state:    AnvilConsensusCanonStateNotification::new()
            },
            _instance: Some(anvil)
        })
    }

    pub fn state_provider(&self) -> AnvilStateProvider<P> {
        self.provider.clone()
    }

    pub fn wallet_provider(&self) -> WalletProvider {
        self.provider.provider.clone()
    }

    pub fn rpc_provider(&self) -> WalletProviderRpc {
        self.provider.provider().clone()
    }

    pub async fn execute_and_return_state(&self) -> eyre::Result<(Bytes, Block)> {
        let block = self.mine_block().await?;

        Ok((self.provider.provider().anvil_dump_state().await?, block))
    }

    pub async fn return_state(&self) -> eyre::Result<Bytes> {
        Ok(self.provider.provider().anvil_dump_state().await?)
    }

    pub async fn set_state(&self, state: Bytes) -> eyre::Result<()> {
        self.provider.provider().anvil_load_state(state).await?;

        Ok(())
    }

    pub async fn mine_block(&self) -> eyre::Result<Block> {
        let mined = self
            .provider
            .provider()
            .anvil_mine_detailed(Some(MineOptions::Options { timestamp: None, blocks: Some(1) }))
            .await?
            .first()
            .cloned()
            .unwrap();

        self.provider.update_canon_chain(&mined)?;

        Ok(mined)
    }
}
