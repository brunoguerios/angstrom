use std::{
    collections::HashSet,
    pin::Pin,
    task::{Context, Poll, Waker}
};

use alloy::{primitives::BlockNumber, transports::Transport};
use angstrom_network::manager::StromConsensusEvent;
use angstrom_types::consensus::{PreProposal, PreProposalAggregation, Proposal};
use futures::Future;
use matching_engine::MatchingEngineHandle;

use super::{Consensus, ConsensusState};
use crate::rounds::ConsensusTransitionMessage;

/// The finalization state.
///
/// At this point we verify the proposal that was sent. Once slashing is added,
/// we will have a fork here (higher level module will shove this state machine
/// off) where we will wait for proposals to be propagated (consensus states you
/// have a day max). in which they will be verified and the round will
/// officially close.
pub struct FinalizationState {
    verification_future: Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>>,
    proposal:            Proposal
}

impl FinalizationState {
    pub fn new<T, Matching>(
        proposal: Proposal,
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

impl<T, Matching> ConsensusState<T, Matching> for FinalizationState
where
    T: Transport + Clone,
    Matching: MatchingEngineHandle
{
    fn on_consensus_message(&mut self, _: &mut Consensus<T, Matching>, _: StromConsensusEvent) {
        // no messages consensus related matter at this point. is just waiting
        // to be reset.
    }

    fn poll_transition(
        &mut self,
        handles: &mut Consensus<T, Matching>,
        cx: &mut Context<'_>
    ) -> Poll<Option<Box<dyn ConsensusState<T, Matching>>>> {
        todo!()
    }
}
