use std::{
    collections::{HashMap, HashSet},
    sync::Arc
};

use alloy_primitives::Address;
use angstrom_types::contract_payloads::angstrom::AngstromPoolConfigStore;
use reth_execution_types::Chain;
use reth_provider::CanonStateNotification;
use serde::{Deserialize, Serialize};

/// The state of our eth-updater, right before we go to the next block
#[derive(Serialize, Deserialize)]
pub struct EthUpdaterSnapshot {
    pub angstrom_address:  Address,
    pub periphery_address: Address,
    pub chain_update:      AngstromChainUpdate,
    pub angstrom_tokens:   HashMap<Address, usize>,
    pub pool_store:        Arc<AngstromPoolConfigStore>,
    /// the set of currently active nodes.
    pub node_set:          HashSet<Address>
}

#[derive(Serialize, Deserialize)]
pub enum AngstromChainUpdate {
    New(Arc<Chain>),
    Reorg { new: Arc<Chain>, old: Arc<Chain> }
}

impl From<CanonStateNotification> for AngstromChainUpdate {
    fn from(value: CanonStateNotification) -> Self {
        match value {
            CanonStateNotification::Commit { new } => Self::New(new),
            CanonStateNotification::Reorg { old, new } => Self::Reorg { new, old }
        }
    }
}
