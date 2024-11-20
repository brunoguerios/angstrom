use std::{
    collections::HashSet,
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker}
};

use alloy::{primitives::BlockNumber, transports::Transport};
use angstrom_network::manager::StromConsensusEvent;
use angstrom_types::{
    consensus::{PreProposal, PreProposalAggregation},
    contract_payloads::angstrom::BundleGasDetails,
    orders::PoolSolution
};
use futures::{future::BoxFuture, FutureExt};
use matching_engine::MatchingEngineHandle;

use super::{Consensus, ConsensusState};
use crate::rounds::ConsensusTransitionMessage;

/// Proposal State.
///
/// We only transition to Proposal state if we are the leader.
/// In this state we build the proposal, submit it on chain and then propagate
/// it once its landed on chain. We only submit after it has landed on chain as
/// in the case of inclusion games. the preoposal will just be dropped and there
/// is no need for others to verify.
pub struct ProposalState {
    building_future:
        Option<BoxFuture<'static, eyre::Result<(Vec<PoolSolution>, BundleGasDetails)>>>,
    submission_future: Option<()>,
    waker:             Waker
}

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
        // queue building future
        waker.wake_by_ref();

        Self {
            building_future: Some(handles.build_bundle(pre_proposal_aggregation)),
            submission_future: None,
            waker
        }
    }
}

impl<T, Matching> ConsensusState<T, Matching> for ProposalState
where
    T: Transport + Clone,
    Matching: MatchingEngineHandle
{
    fn on_consensus_message(&mut self, _: &mut Consensus<T, Matching>, _: StromConsensusEvent) {
        // No messages at this point can effect the consensus round and thus are
        // ignored.
    }

    fn poll_transition(
        &mut self,
        handles: &mut Consensus<T, Matching>,
        cx: &mut Context<'_>
    ) -> Poll<Option<Box<dyn ConsensusState<T, Matching>>>> {
        if let Some(b_fut) = self.building_future.take() {
            match b_fut.poll_unpin(cx) {
                Poll::Ready(state) => {}
                Poll::Pending => self.building_future = Some(b_fut)
            }
        }

        if let Some(b_fut) = self.submission_future.take() {
            match b_fut.poll_unpin(cx) {
                Poll::Ready(_) => {}
                Poll::Pending => self.building_future = Some(b_fut)
            }
        }
    }
}
