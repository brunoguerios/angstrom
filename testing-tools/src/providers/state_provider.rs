use std::{fmt::Display, future::IntoFuture};

use alloy::{
    network::{Ethereum, EthereumWallet},
    node_bindings::{Anvil, AnvilInstance},
    providers::{builder, ext::AnvilApi, Provider},
    rpc::types::{anvil::MineOptions, Block},
    signers::local::PrivateKeySigner
};
use alloy_primitives::{Address, BlockNumber, Bytes, B256, U256};
use angstrom_types::contract_bindings::{
    angstrom::Angstrom::AngstromInstance, pool_gate::PoolGate::PoolGateInstance
};
use eyre::bail;
use reth_provider::{
    BlockHashReader, BlockNumReader, CanonStateNotification, CanonStateNotifications,
    CanonStateSubscriptions, ProviderError, ProviderResult
};
use reth_revm::primitives::Bytecode;
use tokio::sync::broadcast;
use validation::common::db::BlockStateProviderFactory;

use super::{
    rpc_provider::RpcStateProvider, utils::WalletProviderRpc, AnvilInitializer, WalletProvider
};
use crate::{
    contracts::environment::{
        angstrom::AngstromEnv,
        uniswap::{TestUniswapEnv, UniswapEnv},
        TestAnvilEnvironment
    },
    mocks::canon_state::AnvilConsensusCanonStateNotification,
    providers::{utils::async_to_sync, ANVIL_TESTNET_DEPLOYMENT_ENDPOINT},
    types::{initial_state::PendingDeployedPools, TestingConfig, WithWalletProvider}
};

#[derive(Debug, Clone)]
pub struct AnvilStateProvider<P> {
    provider:       P,
    canon_state:    AnvilConsensusCanonStateNotification,
    canon_state_tx: broadcast::Sender<CanonStateNotification>
}

impl<P> AnvilStateProvider<P> {
    fn update_canon_chain(&self, new_block: &Block) -> eyre::Result<()> {
        let state = self.canon_state.new_block(new_block);
        if self.canon_state_tx.receiver_count() == 0 {
            tracing::warn!("no canon state rx")
        } else {
            let _ = self
                .canon_state_tx
                .send(CanonStateNotification::Commit { new: state })?;
        }

        Ok(())
    }
}

impl WithWalletProvider for AnvilStateProvider<WalletProvider> {
    fn wallet_provider(&self) -> WalletProvider {
        self.provider.clone()
    }

    fn provider(&self) -> WalletProviderRpc {
        self.provider.provider.clone()
    }

    async fn initialize(
        node_id: u64,
        config: impl TestingConfig + Send
    ) -> eyre::Result<(Self, Option<AnvilInstance>)>
    where
        Self: Sized
    {
        let (rpc, anvil) = config.spawn_rpc(node_id).await?;

        Ok((
            Self {
                provider:       rpc,
                canon_state_tx: broadcast::channel(1000).0,
                canon_state:    AnvilConsensusCanonStateNotification::new()
            },
            anvil
        ))
    }
}

impl WithWalletProvider for AnvilStateProvider<AnvilInitializer> {
    fn provider(&self) -> WalletProviderRpc {
        self.provider.wallet_provider().provider
    }

    fn wallet_provider(&self) -> WalletProvider {
        self.provider.wallet_provider()
    }

    async fn initialize(
        node_id: u64,
        config: impl TestingConfig + Send
    ) -> eyre::Result<(Self, Option<AnvilInstance>)>
    where
        Self: Sized
    {
        let (wallet_provider, anvil) = config.spawn_rpc(node_id).await?;

        let initializer = AnvilInitializer::new(wallet_provider).await?;

        Ok((
            Self {
                provider:       initializer,
                canon_state_tx: broadcast::channel(1000).0,
                canon_state:    AnvilConsensusCanonStateNotification::new()
            },
            anvil
        ))
    }
}

impl<P: WithWalletProvider> reth_revm::DatabaseRef for AnvilStateProvider<P> {
    type Error = eyre::Error;

    fn basic_ref(
        &self,
        address: Address
    ) -> Result<Option<reth_revm::primitives::AccountInfo>, Self::Error> {
        let acc = async_to_sync(
            self.wallet_provider()
                .get_account(address)
                .latest()
                .into_future()
        )?;
        let code = async_to_sync(
            self.wallet_provider()
                .get_code_at(address)
                .latest()
                .into_future()
        )?;
        let code = Some(Bytecode::new_raw(code));

        Ok(Some(reth_revm::primitives::AccountInfo {
            code_hash: acc.code_hash,
            balance: acc.balance,
            nonce: acc.nonce,
            code
        }))
    }

    fn storage_ref(&self, address: Address, index: U256) -> Result<U256, Self::Error> {
        let acc = async_to_sync(
            self.wallet_provider()
                .get_storage_at(address, index)
                .into_future()
        )?;
        Ok(acc)
    }

    fn block_hash_ref(&self, number: u64) -> Result<B256, Self::Error> {
        let acc = async_to_sync(
            self.wallet_provider()
                .get_block_by_number(reth_primitives::BlockNumberOrTag::Number(number), false)
                .into_future()
        )?;

        let Some(block) = acc else { bail!("failed to load block") };
        Ok(block.header.hash)
    }

    fn code_by_hash_ref(&self, _: B256) -> Result<reth_revm::primitives::Bytecode, Self::Error> {
        panic!("This should not be called, as the code is already loaded");
    }
}
impl<P: WithWalletProvider> BlockNumReader for AnvilStateProvider<P> {
    fn chain_info(&self) -> ProviderResult<reth_chainspec::ChainInfo> {
        panic!("never used");
    }

    fn block_number(&self, _: alloy_primitives::B256) -> ProviderResult<Option<BlockNumber>> {
        panic!("never used");
    }

    fn convert_number(
        &self,
        _: alloy_rpc_types::BlockHashOrNumber
    ) -> ProviderResult<Option<alloy_primitives::B256>> {
        panic!("never used");
    }

    fn best_block_number(&self) -> ProviderResult<BlockNumber> {
        Ok(async_to_sync(self.wallet_provider().get_block_number().into_future()).unwrap())
    }

    fn last_block_number(&self) -> ProviderResult<BlockNumber> {
        Ok(async_to_sync(self.wallet_provider().get_block_number().into_future()).unwrap())
    }

    fn convert_hash_or_number(
        &self,
        _: alloy_rpc_types::BlockHashOrNumber
    ) -> ProviderResult<Option<BlockNumber>> {
        panic!("never used");
    }
}
impl<P: WithWalletProvider> BlockHashReader for AnvilStateProvider<P> {
    fn block_hash(&self, _: BlockNumber) -> ProviderResult<Option<alloy_primitives::B256>> {
        panic!("never used");
    }

    fn convert_block_hash(
        &self,
        _: alloy_rpc_types::BlockHashOrNumber
    ) -> ProviderResult<Option<alloy_primitives::B256>> {
        panic!("never used");
    }

    fn canonical_hashes_range(
        &self,
        _: BlockNumber,
        _: BlockNumber
    ) -> ProviderResult<Vec<alloy_primitives::B256>> {
        panic!("never used");
    }
}

impl<P: WithWalletProvider> BlockStateProviderFactory for AnvilStateProvider<P> {
    type Provider = RpcStateProvider;

    fn state_by_block(&self, block: u64) -> ProviderResult<Self::Provider> {
        Ok(RpcStateProvider::new(block, self.provider()))
    }

    fn best_block_number(&self) -> ProviderResult<BlockNumber> {
        async_to_sync(self.provider().get_block_number())
            .map_err(|_| ProviderError::BestBlockNotFound)
    }
}

impl<P: WithWalletProvider> CanonStateSubscriptions for AnvilStateProvider<P> {
    fn subscribe_to_canonical_state(&self) -> CanonStateNotifications {
        self.canon_state_tx.subscribe()
    }
}
