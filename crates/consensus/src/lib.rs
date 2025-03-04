mod leader_selection;
mod manager;

pub use manager::*;
pub mod rounds;

use std::pin::Pin;

use angstrom_types::consensus::{PreProposal, Proposal};
use futures::Stream;
pub use leader_selection::AngstromValidator;

#[derive(Debug, Clone)]
pub enum ConsensusMessage {
    /// Start/Cycle the consensus process as a new block has begun
    NewBlock(u64),
    /// All angstrom nodes broadcast their signed order pools to the network
    PrePropose(PreProposal),
    /// The Proposer broadcasts its signed proposal for validation.  This might
    /// be after execution-time but all nodes need to review this information
    Proposal(Proposal),
}
/// Listener for consensus data
pub trait ConsensusListener: Send + Sync + 'static {
    /// subscribes to new messages from our consensus
    fn subscribe_messages(&self) -> Pin<Box<dyn Stream<Item = ConsensusMessage>>>;
}
