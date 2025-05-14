mod leader_selection;
mod manager;

pub use manager::*;
pub mod rounds;

use std::{collections::HashSet, pin::Pin};

use alloy::primitives::{Address, Bytes};
use futures::Stream;
pub use leader_selection::AngstromValidator;
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConsensusDataWithBlock<T> {
    pub data:  T,
    pub block: u64
}

pub trait ConsensusHandle: Send + Sync + Clone + Unpin + 'static {
    fn subscribe_empty_block_attestations(
        &self
    ) -> Pin<Box<dyn Stream<Item = ConsensusDataWithBlock<Bytes>> + Send>>;

    fn get_current_leader(
        &self
    ) -> Pin<Box<dyn Future<Output = ConsensusDataWithBlock<Address>> + Send>>;

    fn fetch_consensus_state(
        &self
    ) -> Pin<Box<dyn Future<Output = ConsensusDataWithBlock<HashSet<AngstromValidator>>> + Send>>;
}

pub struct ConsensusHandler {
    tx: tokio::sync::mpsc::Sender<ConsensusRequest>
}

pub enum ConsensusRequest {
    CurrentLeader(oneshot::Sender<Address>),
    CurrentConsensusState(oneshot::Sender<Address>)
}
