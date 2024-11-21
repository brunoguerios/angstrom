use std::{
    collections::HashSet,
    pin::Pin,
    task::{Context, Poll, Waker},
    time::Duration
};

use alloy::transports::Transport;
use angstrom_network::manager::StromConsensusEvent;
use angstrom_types::consensus::{PreProposal, PreProposalAggregation, Proposal};
use futures::FutureExt;
use matching_engine::MatchingEngineHandle;
use tokio::time::{sleep, Sleep};

use super::{
    finalization::FinalizationState, pre_proposal::PreProposalState, ConsensusState,
    SharedRoundState
};

#[derive(Debug)]
pub struct BidAggregationState {
    /// because the start is timeout based. We won't propagate our pre_proposal
    /// till the timeout occurs. However if we get one before then, we still
    /// want to hold onto it.
    received_pre_proposals:    HashSet<PreProposal>,
    /// we collect these here given that the leader could be running behind.
    pre_proposals_aggregation: HashSet<PreProposalAggregation>,
    proposal:                  Option<Proposal>,
    transition_timeout:        Pin<Box<Sleep>>,
    waker:                     Option<Waker>
}

impl BidAggregationState {
    pub fn new(transition_timeout: Duration) -> Self {
        let sleep = sleep(transition_timeout);
        // ensures we queue the sleep timeout

        Self {
            received_pre_proposals:    HashSet::default(),
            pre_proposals_aggregation: HashSet::default(),
            transition_timeout:        Box::pin(sleep),
            proposal:                  None,
            waker:                     None
        }
    }
}

impl<T, Matching> ConsensusState<T, Matching> for BidAggregationState
where
    T: Transport + Clone,
    Matching: MatchingEngineHandle
{
    fn on_consensus_message(
        &mut self,
        handles: &mut SharedRoundState<T, Matching>,
        message: StromConsensusEvent
    ) {
        match message {
            StromConsensusEvent::PreProposal(peer_id, pre_proposal) => {
                handles.handle_pre_proposal(
                    peer_id,
                    pre_proposal,
                    &mut self.received_pre_proposals
                );
            }
            StromConsensusEvent::PreProposalAgg(peer_id, agg) => {
                handles.handle_pre_proposal_aggregation(
                    peer_id,
                    agg,
                    &mut self.pre_proposals_aggregation
                );
            }
            StromConsensusEvent::Proposal(peer_id, proposal) => {
                if let Some(proposal) = handles.verify_proposal(peer_id, proposal) {
                    // given a proposal was seen. we will skip directly to verification
                    self.proposal = Some(proposal);
                    self.waker.as_ref().inspect(|w| w.wake_by_ref());
                }
            }
        }
    }

    fn poll_transition(
        &mut self,
        handles: &mut SharedRoundState<T, Matching>,
        cx: &mut Context<'_>
    ) -> Poll<Option<Box<dyn ConsensusState<T, Matching>>>> {
        self.waker = Some(cx.waker().clone());
        if let Some(proposal) = self.proposal.take() {
            // skip to finalization
            return Poll::Ready(Some(Box::new(FinalizationState::new(
                proposal,
                handles,
                cx.waker().clone()
            ))))
        }

        if self.transition_timeout.poll_unpin(cx).is_ready() {
            // create the transition
            let pre_proposal = PreProposalState::new(
                handles.block_height,
                std::mem::take(&mut self.received_pre_proposals),
                std::mem::take(&mut self.pre_proposals_aggregation),
                handles,
                cx.waker().clone()
            );

            // return the transition
            return Poll::Ready(Some(Box::new(pre_proposal)))
        }

        Poll::Pending
    }
}
