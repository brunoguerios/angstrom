use std::{
    collections::HashSet,
    task::{Context, Poll, Waker},
    time::Duration
};

use alloy::primitives::BlockNumber;
use angstrom_network::manager::StromConsensusEvent;
use angstrom_types::consensus::PreProposal;
use itertools::Itertools;
use matching_engine::MatchingEngineHandle;
use reth_primitives::revm_primitives::HashSet;
use tokio::time::{sleep, Sleep};

use super::{
    pre_proposal::{self, PreProposalState},
    Consensus, ConsensusState, ConsensusTransitionMessage
};

#[derive(Debug)]
pub struct BidAggregationState {
    block_height:              BlockNumber,
    /// because the start is timeout based. We won't propagate our pre_proposal
    /// till the timeout occurs. However if we get one before then, we still
    /// want to hold onto it.
    received_pre_proposals:    HashSet<PreProposal>,
    /// we collect these here given that this node could be running pretty far
    /// behind and we don't want to drop any valid sets.
    pre_proposals_aggregation: HashSet<PreProposalAggregation>,
    transition_timeout:        Sleep,
    waker:                     Waker
}

impl BidAggregationState {
    pub fn new(block_height: BlockNumber, transition_timeout: Duration, waker: Waker) -> Self {
        let sleep = sleep(transition_timeout);
        // ensures we queue the sleep timeout
        waker.wake_by_ref();

        Self {
            block_height,
            received_pre_proposals: HashSet::default(),
            pre_proposals_aggregation: HashSet::default(),
            transition_timeout: sleep,
            waker
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
        handles: &mut Consensus<T, Matching>,
        message: StromConsensusEvent
    ) {
        let StromConsensusEvent::PreProposal(peer_id, pre_proposal) = message  else { return };

        // ensure message is from a valid peer
        if !handles
            .validators
            .iter()
            .map(|v| v.peer_id)
            .contains(&peer_id)
        {
            tracing::warn!(peer=?peer_id,"got a pre_proposal from a invalid peer");
            return
        }

        // ensure pre_proposal is valid
        if !pre_proposal.is_valid(&self.block_height) {
            tracing::info!(peer=?peer_id,"got a invalid pre_proposal");
            return
        }

        // if  we don't have the pre_proposal, propagate it and then store it.
        // else log a message
        if !self.received_pre_proposals.contains(&pre_proposal) {
            handles.propagate_message(ConsensusTransitionMessage::PropagatePreProposal(
                pre_proposal.clone()
            ));
            self.received_pre_proposals.insert(pre_proposal);
        } else {
            tracing::info!(peer=?peer_id,"got a duplicate pre_proposal");
        }
    }

    fn poll_transition(
        &mut self,
        handles: &mut Consensus<T, Matching>,
        cx: &mut Context<'_>
    ) -> Poll<Box<dyn ConsensusState<T, Matching>>> {
        if self.transition_timeout.poll_unpin(cx).is_ready() {
            // create the transition
            let pre_proposal = PreProposalState::new(
                self.block_height,
                self.received_pre_proposals.drain().collect::<HashSet<_>>(),
                handles,
                cx.waker().clone()
            );

            // return the transition
            return Poll::Ready(Box::new(pre_proposal))
        }

        Poll::Pending
    }
}
