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

#[derive(Debug)]
pub struct PreProposalState {
    pre_proposals:             HashSet<PreProposal>,
    pre_proposals_aggregation: HashSet<PreProposalAggregation>,
    // used to wake the
    waker:                     Waker
}

impl PreProposalState {
    pub fn new<T, Matching>(
        block_height: BlockNumber,
        mut pre_proposals: HashSet<PreProposal>,
        pre_proposals_aggregation: HashSet<PreProposalAggregation>,
        handles: &mut Consensus<T, Matching>,
        waker: Waker
    ) -> Self
    where
        T: Transport + Clone,
        Matching: MatchingEngineHandle
    {
        // generate my pre_proposal
        let my_preproposal = PreProposal::new(
            block_height,
            &handles.signer.key,
            handles.signer.my_id,
            handles.order_storage.get_all_orders()
        );

        // propagate my pre_proposal
        handles.propagate_message(ConsensusTransitionMessage::PropagatePreProposal(
            my_preproposal.clone()
        ));

        pre_proposals.insert(my_preproposal);

        // ensure we get polled to start the checks for when we have 2f +1 pre_proposals
        // collected
        waker.wake_by_ref();

        Self { pre_proposals, pre_proposals_aggregation, waker }
    }
}

impl<T, Matching> ConsensusState<T, Matching> for PreProposalState
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
