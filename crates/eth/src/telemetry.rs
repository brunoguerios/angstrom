use alloy::primitives::Address;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use angstrom_types::{
    block_sync::BlockSyncProducer, contract_payloads::angstrom::AngstromPoolConfigStore,
};
use reth_execution_types::Chain;
use reth_provider::CanonStateNotification;
use serde::{Deserialize, Serialize};

use crate::manager::EthDataCleanser;

/// The state of our eth-updater, right before we go to the next block
#[derive(Serialize, Deserialize)]
pub struct EthUpdaterSnapshot {
    pub angstrom_address: Address,
    pub periphery_address: Address,
    pub chain_update: AngstromChainUpdate,
    pub angstrom_tokens: HashMap<Address, usize>,
    pub pool_store: Arc<AngstromPoolConfigStore>,
    /// the set of currently active nodes.
    pub node_set: HashSet<Address>,
}

impl<Sync: BlockSyncProducer> From<(&EthDataCleanser<Sync>, CanonStateNotification)>
    for EthUpdaterSnapshot
{
    fn from((data, update): (&EthDataCleanser<Sync>, CanonStateNotification)) -> Self {
        Self {
            angstrom_tokens: data.angstrom_tokens.clone(),
            angstrom_address: data.angstrom_address.clone(),
            periphery_address: data.periphery_address.clone(),
            chain_update: update.into(),
            pool_store: data.pool_store,
            node_set: data.node_set,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum AngstromChainUpdate {
    New(Arc<Chain>),
    Reorg { new: Arc<Chain>, old: Arc<Chain> },
}

impl From<CanonStateNotification> for AngstromChainUpdate {
    fn from(value: CanonStateNotification) -> Self {
        match value {
            CanonStateNotification::Commit { new } => Self::New(new),
            CanonStateNotification::Reorg { old, new } => Self::Reorg { new, old },
        }
    }
}
