use std::{
    collections::HashSet,
    task::{Context, Poll, Waker}
};

use alloy::{primitives::BlockNumber, transports::Transport};
use angstrom_network::manager::StromConsensusEvent;
use angstrom_types::consensus::{PreProposal, PreProposalAggregation};
use matching_engine::MatchingEngineHandle;

use super::{Consensus, ConsensusState};
use crate::rounds::ConsensusTransitionMessage;

pub struct FinalizationState {}

impl<T, Matching> ConsensusState<T, Matching> for FinalizationState
where
    T: Transport + Clone,
    Matching: MatchingEngineHandle
{
    fn on_consensus_message(
        &mut self,
        handles: &mut Consensus<T, Matching>,
        message: StromConsensusEvent
    ) {
        match message {
            StromConsensusEvent::PreProposal(peer_id, pre_proposal) => {}
            StromConsensusEvent::PreProposalAgg(peer_id, pre_proposal_agg) => {}
            StromConsensusEvent::Proposal(peer_id, proposal) => {}
        }
    }

    fn poll_transition(
        &mut self,
        handles: &mut Consensus<T, Matching>,
        cx: &mut Context<'_>
    ) -> Poll<Box<dyn ConsensusState<T, Matching>>> {
        todo!()
    }
}
