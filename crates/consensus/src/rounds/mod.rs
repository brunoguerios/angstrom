use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll, Waker},
    time::Duration
};

use alloy::{
    network::TransactionBuilder,
    primitives::{Address, BlockNumber},
    providers::Provider,
    rpc::types::TransactionRequest,
    transports::Transport
};
use angstrom_metrics::ConsensusMetricsWrapper;
use angstrom_network::{manager::StromConsensusEvent, StromMessage};
use angstrom_types::{
    consensus::{PreProposal, PreProposalAggregation, Proposal},
    contract_payloads::angstrom::{AngstromBundle, UniswapAngstromRegistry},
    orders::OrderSet,
    primitive::PeerId,
    sol_bindings::{
        grouped_orders::{GroupedVanillaOrder, OrderWithStorageData},
        rpc_orders::TopOfBlockOrder
    }
};
use angstrom_utils::timer::async_time_fn;
use eyre::Report;
/// Represents the state of the current consensus mech.
use futures::{future::BoxFuture, Future, Stream, StreamExt};
use itertools::Itertools;
use matching_engine::MatchingEngineHandle;
use order_pool::order_storage::OrderStorage;
use pade::PadeEncode;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uniswap_v4::uniswap::pool_manager::SyncedUniswapPools;

use crate::{AngstromValidator, Signer};

mod bid_aggregation;
mod finalization;
mod pre_proposal;
mod pre_proposal_aggregation;
mod proposal;

type TransitionFuture<T, Matching> =
    Pin<Box<dyn Future<Output = Option<Box<dyn ConsensusState<T, Matching>>>> + Send + 'static>>;

pub trait ConsensusState<T, Matching>
where
    T: Transport + Clone,
    Matching: MatchingEngineHandle
{
    fn on_consensus_message(
        &mut self,
        handles: &mut Consensus<T, Matching>,
        message: StromConsensusEvent
    );

    /// just like streams. Once this returns Poll::Ready(None). This consensus
    /// round is over
    fn poll_transition(
        &mut self,
        handles: &mut Consensus<T, Matching>,
        cx: &mut Context<'_>
    ) -> Poll<Option<Box<dyn ConsensusState<T, Matching>>>>;
}

/// Holds and progresses the consensus state machine
pub struct ConsensusRoundState<T, Matching> {
    current_state:       Box<dyn ConsensusState<T, Matching>>,
    consensus_arguments: Consensus<T, Matching>
}

impl<T, Matching> Stream for ConsensusRoundState<T, Matching>
where
    T: Transport + Clone,
    Matching: MatchingEngineHandle
{
    type Item = ConsensusTransitionMessage;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        if let Poll::Ready(Some(transitioned_state)) = this
            .current_state
            .poll_transition(&mut this.consensus_arguments, cx)
        {
            this.current_state = transitioned_state;
        }
        if let Some(message) = this.consensus_arguments.messages.pop_front() {
            return Poll::Ready(Some(message))
        }

        Poll::Pending
    }
}

pub struct Consensus<T, Matching> {
    block_height:    BlockNumber,
    matching_engine: Matching,
    signer:          Signer,
    round_leader:    PeerId,
    validators:      Vec<AngstromValidator>,
    order_storage:   Arc<OrderStorage>,
    metrics:         ConsensusMetricsWrapper,
    waker:           Option<Waker>,
    pool_registry:   UniswapAngstromRegistry,
    uniswap_pools:   SyncedUniswapPools,
    provider:        Arc<Pin<Box<dyn Provider<T>>>>,
    messages:        VecDeque<ConsensusTransitionMessage>
}

// contains shared impls
impl<T, Matching> Consensus<T, Matching> {
    fn propagate_message(&mut self, message: ConsensusTransitionMessage) {
        self.messages.push_back(message);
    }

    fn i_am_leader(&self) -> bool {
        self.round_leader == self.signer.my_id
    }

    fn two_thirds_of_validation_set(&self) -> usize {
        (2 * self.validators.len() / 3) + 1
    }

    fn handle_pre_proposal_aggregation(
        &mut self,
        peer_id: PeerId,
        pre_proposal_agg: PreProposalAggregation,
        pre_proposal_agg_set: &mut HashSet<PreProposalAggregation>
    ) {
        self.handle_proposal_verification(
            peer_id,
            pre_proposal_agg,
            pre_proposal_agg_set,
            |proposal, block| proposal.is_valid(block)
        )
    }

    fn verify_proposal(&mut self, peer_id: PeerId, proposal: Proposal) -> Option<Proposal> {
        if self.round_leader != peer_id {
            return None
        }

        proposal.is_valid(&self.block_height).then(|| {
            self.messages
                .push_back(ConsensusTransitionMessage::PropagateProposal(proposal.clone()));

            proposal
        })
    }

    fn handle_pre_proposal(
        &mut self,
        peer_id: PeerId,
        pre_proposal: PreProposal,
        pre_proposal_set: &mut HashSet<PreProposal>
    ) {
        self.handle_proposal_verification(
            peer_id,
            pre_proposal,
            pre_proposal_set,
            |proposal, block| proposal.is_valid(block)
        )
    }

    fn handle_proposal_verification<P>(
        &mut self,
        peer_id: PeerId,
        proposal: P,
        proposal_set: &mut HashSet<P>,
        valid: impl FnOnce(&P, &BlockNumber) -> bool
    ) where
        P: Into<ConsensusTransitionMessage> + Eq + Hash + Clone
    {
        if !self.validators.iter().map(|v| v.peer_id).contains(&peer_id) {
            tracing::warn!(peer=?peer_id,"got a consensus message from a invalid peer");
            return
        }
        // ensure pre_proposal is valid
        if !valid(&proposal, &self.block_height) {
            tracing::info!(peer=?peer_id,"got a invalid consensus message");
            return
        }

        // if  we don't have the pre_proposal, propagate it and then store it.
        // else log a message
        if !proposal_set.contains(&proposal) {
            self.propagate_message(proposal.clone().into());
            proposal_set.insert(proposal);
        } else {
            tracing::info!(peer=?peer_id,"got a duplicate consensus message");
        }
    }
}

#[derive(Debug, Clone)]
pub enum ConsensusTransitionMessage {
    /// Either our or another nodes PreProposal. The PreProposal will only be
    /// shared here if it is new to this nodes consensus outlook
    PropagatePreProposal(PreProposal),
    PropagatePreProposalAgg(PreProposalAggregation),
    PropagateProposal(Proposal)
}

impl From<PreProposal> for ConsensusTransitionMessage {
    fn from(value: PreProposal) -> Self {
        Self::PropagatePreProposal(value)
    }
}

impl From<PreProposalAggregation> for ConsensusTransitionMessage {
    fn from(value: PreProposalAggregation) -> Self {
        Self::PropagatePreProposalAgg(value)
    }
}
