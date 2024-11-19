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

impl<T, Matching> Consensus<T, Matching> {
    fn propagate_message(&mut self, message: ConsensusTransitionMessage) {
        self.messages.push_back(message);
    }

    fn i_am_leader(&self) -> bool {
        self.round_leader == self.signer.my_id
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
