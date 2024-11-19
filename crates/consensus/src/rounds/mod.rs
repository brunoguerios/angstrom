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
    consensus::{PreProposal, Proposal},
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
use futures::{future::BoxFuture, Future, Stream};
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

    fn poll_transition(
        &mut self,
        handles: &mut Consensus<T, Matching>,
        cx: &mut Context<'_>
    ) -> Poll<Box<dyn ConsensusState<T, Matching>>>;
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
        if let Poll::Ready(transitioned_state) = self
            .current_state
            .poll_transition(&mut self.consensus_arguments, cx)
        {
            *self.current_state = transitioned_state;
        }

        if let Some(message) = self.messages.pop_front() {
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

    fn handle_pre_proposal(
        &mut self,
        peer_id: PeerId,
        pre_proposal: PreProposal,
        pre_proposal_set: &mut HashSet<PreProposal>
    ) {
        if !self.validators.iter().map(|v| v.peer_id).contains(&peer_id) {
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
        if !pre_proposal_set.contains(&pre_proposal) {
            self.propagate_message(ConsensusTransitionMessage::PropagatePreProposal(
                pre_proposal.clone()
            ));
            pre_proposal_set.insert(pre_proposal);
        } else {
            tracing::info!(peer=?peer_id,"got a duplicate pre_proposal");
        }
    }
}

impl<T, Matching> Stream for Consensus<T, Matching>
where
    T: Transport + Clone,
    Matching: MatchingEngineHandle
{
    type Item = ConsensusTransitionMessage;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let Some(message) = self.messages.pop_front() {
            return Poll::Ready(Some(message))
        }
        Poll::Pending
    }
}

#[derive(Debug, Clone)]
pub enum ConsensusTransitionMessage {
    /// Either our or another nodes PreProposal. The PreProposal will only be
    /// shared here if it is new to this nodes consensus outlook
    PropagatePreProposal(PreProposal)
}
