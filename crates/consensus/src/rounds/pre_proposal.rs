use std::{
    collections::HashSet,
    task::{Context, Poll, Waker}
};

use alloy::primitives::BlockNumber;
use angstrom_types::consensus::PreProposal;
use matching_engine::MatchingEngineHandle;

use super::{Consensus, ConsensusState};
use crate::{round_state::PreProposalAggregation, rounds::ConsensusTransitionMessage};

#[derive(Debug)]
pub struct PreProposalState {
    block_height:              BlockNumber,
    pre_proposals:             HashSet<PreProposal>,
    pre_proposals_aggregation: HashSet<PreProposalAggregation>,
    // used to wake the
    waker:                     Waker
}

impl PreProposalState {
    pub fn new<T, Matching>(
        block_height: BlockNumber,
        mut pre_proposals: HashSet<PreProposal>,
        pre_proposals_aggregation: HashSet<PreProposal>,
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

        received_pre_proposals.insert(my_preproposal);

        // ensure we get polled to start the checks for when we have 2f +1 pre_proposals
        // collected
        waker.wake_by_ref();

        Self { block_height, pre_proposals, pre_proposals_aggregation, waker }
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
    }

    fn poll_transition(
        &mut self,
        handles: &mut Consensus<T, Matching>,
        cx: &mut Context<'_>
    ) -> Poll<Box<dyn ConsensusState<T, Matching>>> {
    }
}
