use std::{
    collections::HashSet,
    task::{Context, Poll, Waker}
};

use alloy::{primitives::BlockNumber, transports::Transport};
use angstrom_network::manager::StromConsensusEvent;
use angstrom_types::consensus::{PreProposal, PreProposalAggregation};
use matching_engine::MatchingEngineHandle;

use super::{pre_proposal_aggregation, Consensus, ConsensusState};
use crate::rounds::ConsensusTransitionMessage;

/// Proposal State.
///
/// We only transition to Proposal state if we are the leader.
/// In this state we build the proposal, submit it on chain and then propagate
/// it once its landed on chain. We only submit after it has landed on chain as
/// in the case of inclusion games. the preoposal will just be dropped and there
/// is no need for others to verify.
pub struct ProposalState {}

impl ProposalState {
    pub fn new<T, Matching>(
        pre_proposal_aggregation: HashSet<PreProposalAggregation>,
        handles: &mut Consensus<T, Matching>,
        waker: Waker
    ) -> Self
    where
        T: Transport + Clone,
        Matching: MatchingEngineHandle
    {
        todo!()
    }
}

impl<T, Matching> ConsensusState<T, Matching> for ProposalState
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
    ) -> Poll<Option<Box<dyn ConsensusState<T, Matching>>>> {
        todo!()
    }
}
