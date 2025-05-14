mod leader_selection;
mod manager;

pub use manager::*;
pub mod rounds;

use std::{collections::HashSet, pin::Pin};

use alloy::primitives::{Address, Bytes};
use angstrom_types::consensus::{PreProposal, Proposal};
use futures::Stream;
pub use leader_selection::AngstromValidator;
use tokio::sync::oneshot;

pub struct ConsensusDataWithBlock<T> {
    data:  T,
    block: u64
}

pub trait ConsensusHandle: Send + Sync + Clone + Unpin + 'static {
    fn subscribe_empty_block_attestations(&self) -> Pin<Box<dyn Stream<Item = Bytes>>>;

    fn get_current_leader(&self) -> Pin<Box<dyn Future<Output = Address> + Send>>;

    fn fetch_consensus_state(
        &self
    ) -> Pin<Box<dyn Future<Output = HashSet<AngstromValidator>> + Send>>;
}

pub struct ConsensusHandler {
    tx: tokio::sync::mpsc::Sender<ConsensusRequest>
}

pub enum ConsensusRequest {
    CurrentLeader(oneshot::Sender<Address>),
    CurrentConsensusState(oneshot::Sender<Address>)
}
