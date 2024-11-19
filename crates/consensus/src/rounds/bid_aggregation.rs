use std::{
    collections::HashSet,
    task::{Context, Poll, Waker},
    time::Duration
};

use alloy::{primitives::BlockNumber, transports::Transport};
use angstrom_network::manager::StromConsensusEvent;
use angstrom_types::{
    consensus::{PreProposal, PreProposalAggregation},
    primitive::PeerId
};
use futures::FutureExt;
use itertools::Itertools;
use matching_engine::MatchingEngineHandle;
use tokio::time::{sleep, Sleep};

use super::{
    pre_proposal::{self, PreProposalState},
    Consensus, ConsensusState, ConsensusTransitionMessage
};

#[derive(Debug)]
pub struct BidAggregationState {
    /// because the start is timeout based. We won't propagate our pre_proposal
    /// till the timeout occurs. However if we get one before then, we still
    /// want to hold onto it.
    received_pre_proposals:    HashSet<PreProposal>,
    /// we collect these here given that the leader could be running behind.
    pre_proposals_aggregation: HashSet<PreProposalAggregation>,
    transition_timeout:        Sleep,
    waker:                     Waker
}

impl BidAggregationState {
    pub fn new(transition_timeout: Duration, waker: Waker) -> Self {
        let sleep = sleep(transition_timeout);
        // ensures we queue the sleep timeout
        waker.wake_by_ref();

        Self {
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
        match message {
            StromConsensusEvent::PreProposal(peer_id, pre_proposal) => {
                handles.handle_pre_proposal(
                    peer_id,
                    pre_proposal,
                    &mut self.received_pre_proposals
                );
            }
            _ => todo!()
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
                handles.block_height,
                self.received_pre_proposals.drain().collect::<HashSet<_>>(),
                self.pre_proposals_aggregation
                    .drain()
                    .collect::<HashSet<_>>(),
                handles,
                cx.waker().clone()
            );

            // return the transition
            return Poll::Ready(Box::new(pre_proposal))
        }

        Poll::Pending
    }
}
