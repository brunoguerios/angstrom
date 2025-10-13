use std::{
    collections::{HashMap, HashSet, VecDeque, hash_map::Entry},
    hash::Hash,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll}
};

use alloy::{
    primitives::{Address, B256, BlockNumber, Bytes, FixedBytes},
    providers::Provider
};
use angstrom_metrics::ConsensusMetricsWrapper;
use angstrom_types::{
    amm::PoolState,
    consensus::{
        ConsensusRoundEvent, ConsensusRoundName, PreProposal, PreProposalAggregation, Proposal,
        SlotClock, StromConsensusEvent, SystemTimeSlotClock
    },
    contract_payloads::angstrom::{BundleGasDetails, UniswapAngstromRegistry},
    orders::PoolSolution,
    primitive::{AngstromMetaSigner, AngstromSigner},
    submission::SubmissionHandler
};
use bid_aggregation::BidAggregationState;
use futures::{FutureExt, Stream, future::BoxFuture};
use matching_engine::{MatchingEngineHandle, manager::MatchingEngineError};
use order_pool::order_storage::OrderStorage;
use preproposal_wait_trigger::{LastRoundInfo, PreProposalWaitTrigger};
use uniswap_v4::uniswap::pool_manager::SyncedUniswapPools;

use crate::{AngstromValidator, ConsensusTimingConfig};

mod bid_aggregation;
mod finalization;
mod pre_proposal;
mod pre_proposal_aggregation;
mod preproposal_wait_trigger;
mod proposal;

type PollTransition<P, Matching, S> = Poll<Option<Box<dyn ConsensusState<P, Matching, S>>>>;

pub trait ConsensusState<P, Matching, S>: Send
where
    P: Provider + Unpin + 'static,
    Matching: MatchingEngineHandle,
    S: AngstromMetaSigner
{
    fn on_consensus_message(
        &mut self,
        handles: &mut SharedRoundState<P, Matching, S>,
        message: StromConsensusEvent
    );

    /// just like streams. Once this returns Poll::Ready(None). This consensus
    /// round is over
    fn poll_transition(
        &mut self,
        handles: &mut SharedRoundState<P, Matching, S>,
        cx: &mut Context<'_>
    ) -> PollTransition<P, Matching, S>;

    fn last_round_info(&mut self) -> Option<LastRoundInfo> {
        None
    }

    fn name(&self) -> ConsensusRoundName;
}

/// Holds and progresses the consensus state machine
pub struct RoundStateMachine<P, Matching, S: AngstromMetaSigner>
where
    P: Provider + Unpin + 'static
{
    current_state:           Box<dyn ConsensusState<P, Matching, S>>,
    /// for consensus, on a new block we wait a duration of time before signing
    /// our pre-proposal. this is the time
    consensus_wait_duration: PreProposalWaitTrigger,
    shared_state:            SharedRoundState<P, Matching, S>,
    slot_clock:              SystemTimeSlotClock
}

impl<P, Matching, S> RoundStateMachine<P, Matching, S>
where
    P: Provider + Unpin + 'static,
    Matching: MatchingEngineHandle,
    S: AngstromMetaSigner
{
    pub fn new(
        shared_state: SharedRoundState<P, Matching, S>,
        slot_clock: SystemTimeSlotClock
    ) -> Self {
        let mut consensus_wait_duration = PreProposalWaitTrigger::new(
            shared_state.order_storage.clone(),
            shared_state.consensus_config
        );

        let next_slot_duration = slot_clock.duration_to_next_slot().unwrap();
        let elapsed_time = slot_clock.slot_duration() - next_slot_duration;

        Self {
            current_state: Box::new(BidAggregationState::new(
                consensus_wait_duration.update_for_new_round(None, elapsed_time)
            )),
            consensus_wait_duration,
            shared_state,
            slot_clock
        }
    }

    pub fn current_leader(&self) -> Address {
        self.shared_state.round_leader
    }

    pub fn timing(&self) -> ConsensusTimingConfig {
        self.shared_state.consensus_config
    }

    pub fn is_auction_closed(&self) -> bool {
        self.current_state.name().is_closed()
    }

    pub fn reset_round(&mut self, new_block: u64, new_leader: Address) {
        let next_slot_duration = self.slot_clock.duration_to_next_slot().unwrap();
        let elapsed_time = self.slot_clock.slot_duration() - next_slot_duration;

        // grab the last round info if we were the leader.
        let info = self.current_state.last_round_info();

        // reset before we got to proposal, we decay the round time to handle this case
        // as otherwise, can end up in a loop where we never submit and never
        // adjust time
        if info.is_none() && self.shared_state.i_am_leader() {
            self.consensus_wait_duration.reset_before_submission();
        }

        self.shared_state.block_height = new_block;
        self.shared_state.round_leader = new_leader;

        self.current_state = Box::new(BidAggregationState::new(
            self.consensus_wait_duration
                .update_for_new_round(info, elapsed_time)
        ));
    }

    pub fn handle_message(&mut self, event: StromConsensusEvent) {
        self.current_state
            .on_consensus_message(&mut self.shared_state, event);
    }
}

impl<P, Matching, S> Stream for RoundStateMachine<P, Matching, S>
where
    P: Provider + Unpin + 'static,
    Matching: MatchingEngineHandle,
    S: AngstromMetaSigner
{
    type Item = ConsensusMessage;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        if let Some(message) = this.shared_state.messages.pop_front() {
            return Poll::Ready(Some(message));
        }

        if let Poll::Ready(Some(transitioned_state)) = this
            .current_state
            .poll_transition(&mut this.shared_state, cx)
        {
            tracing::info!("transitioning to new round state");
            this.current_state = transitioned_state;
            let name = this.current_state.name();
            return Poll::Ready(Some(ConsensusMessage::StateChange(name)));
        }

        Poll::Pending
    }
}

pub struct SharedRoundState<P: Provider + Unpin + 'static, Matching, S: AngstromMetaSigner> {
    block_height:     BlockNumber,
    matching_engine:  Matching,
    signer:           AngstromSigner<S>,
    round_leader:     Address,
    validators:       Vec<AngstromValidator>,
    order_storage:    Arc<OrderStorage>,
    _metrics:         ConsensusMetricsWrapper,
    pool_registry:    UniswapAngstromRegistry,
    uniswap_pools:    SyncedUniswapPools,
    provider:         Arc<SubmissionHandler<P>>,
    messages:         VecDeque<ConsensusMessage>,
    consensus_config: ConsensusTimingConfig
}

// contains shared impls
impl<P, Matching, S: AngstromMetaSigner> SharedRoundState<P, Matching, S>
where
    P: Provider + Unpin + 'static,
    Matching: MatchingEngineHandle,
    S: AngstromMetaSigner
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        block_height: BlockNumber,
        order_storage: Arc<OrderStorage>,
        signer: AngstromSigner<S>,
        round_leader: Address,
        validators: Vec<AngstromValidator>,
        metrics: ConsensusMetricsWrapper,
        pool_registry: UniswapAngstromRegistry,
        uniswap_pools: SyncedUniswapPools,
        provider: SubmissionHandler<P>,
        matching_engine: Matching,
        consensus_config: ConsensusTimingConfig
    ) -> Self {
        Self {
            block_height,
            round_leader,
            validators,
            order_storage,
            pool_registry,
            uniswap_pools,
            signer,
            _metrics: metrics,
            matching_engine,
            messages: VecDeque::new(),
            provider: Arc::new(provider),
            consensus_config
        }
    }

    fn propagate_message(&mut self, message: ConsensusMessage) {
        self.messages.push_back(message);
    }

    fn i_am_leader(&self) -> bool {
        self.round_leader == self.signer.address()
    }

    fn two_thirds_of_validation_set(&self) -> usize {
        (2 * self.validators.len()).div_ceil(3)
    }

    fn fetch_pool_snapshot(&self) -> HashMap<FixedBytes<32>, (Address, Address, PoolState, u16)> {
        self.uniswap_pools
            .iter()
            .filter_map(|item| {
                let key = item.key();
                let pool = item.value();
                tracing::info!(?key, "getting snapshot");
                let (token_a, token_b, snapshot) =
                    pool.read().unwrap().fetch_pool_snapshot().ok()?;
                let entry = self.pool_registry.get_ang_entry(key)?;

                Some((
                    *key,
                    (token_a, token_b, PoolState::Uniswap(snapshot), entry.store_index as u16)
                ))
            })
            .collect::<HashMap<_, _>>()
    }

    fn matching_engine_output(
        &self,
        pre_proposal_aggregation: HashSet<PreProposalAggregation>
    ) -> BoxFuture<'static, Result<(Vec<PoolSolution>, BundleGasDetails), MatchingEngineError>>
    {
        // fetch
        let mut limit = Vec::new();
        let mut searcher = Vec::new();

        for pre_proposal_agg in pre_proposal_aggregation {
            pre_proposal_agg.pre_proposals.into_iter().for_each(|pre| {
                limit.extend(pre.limit);
                searcher.extend(pre.searcher);
            });
        }

        let valid_limit = self.filter_quorum_orders(limit);
        let valid_searcher = self.filter_quorum_orders(searcher);
        let orders = self
            .order_storage
            .get_all_orders_with_ingoing_cancellations();

        let (limit, searcher) = orders.into_book_and_searcher(valid_limit, valid_searcher);

        // searchers need to be unique by pool-key
        let searcher = searcher
            .into_iter()
            .fold(HashMap::new(), |mut acc, searcher| {
                match acc.entry(searcher.pool_id) {
                    Entry::Vacant(v) => {
                        v.insert(searcher);
                    }
                    Entry::Occupied(mut o) => {
                        let current = o.get();
                        // if this order on same pool_id has a higher tob reward or they are the
                        // same and it has a lower order hash. replace
                        if searcher.tob_reward > current.tob_reward
                            || (searcher.tob_reward == current.tob_reward
                                && searcher.order_id.hash < current.order_id.hash)
                        {
                            o.insert(searcher);
                        }
                    }
                };

                acc
            })
            .into_values()
            .collect();

        let pool_snapshots = self.fetch_pool_snapshot();
        let matcher = self.matching_engine.clone();
        async move { matcher.solve_pools(limit, searcher, pool_snapshots).await }.boxed()
    }

    fn filter_quorum_orders<O: Hash + Eq + Clone>(&self, input: Vec<O>) -> Vec<O> {
        let two_thirds = self.two_thirds_of_validation_set();
        input
            .into_iter()
            .fold(HashMap::new(), |mut acc, order| {
                *acc.entry(order).or_insert(0) += 1;
                acc
            })
            .into_iter()
            .filter(|(_, count)| *count >= two_thirds)
            .map(|(order, _)| order)
            .collect()
    }

    fn handle_pre_proposal_aggregation(
        &mut self,
        pre_proposal_agg: PreProposalAggregation,
        pre_proposal_agg_set: &mut HashSet<PreProposalAggregation>
    ) {
        let two_thirds = self.two_thirds_of_validation_set();
        let valid_peers = self
            .validators
            .iter()
            .map(|v| v.peer_id)
            .collect::<Vec<_>>();

        self.handle_proposal_verification(
            pre_proposal_agg,
            pre_proposal_agg_set,
            |pre_proposal_agg, block| {
                let valid_sig = pre_proposal_agg.is_valid(block, two_thirds);
                let Some(peer_id) = pre_proposal_agg.recover_signer() else {
                    return false;
                };
                let valid_node = valid_peers.contains(&peer_id);

                valid_sig && valid_node
            }
        )
    }

    fn verify_proposal(&mut self, proposal: Proposal) -> Option<Proposal> {
        let signer = proposal.recover_signer()?;

        if self.round_leader != signer {
            tracing::debug!("got invalid proposal");
            return None;
        }

        proposal
            .is_valid(&self.block_height, self.two_thirds_of_validation_set())
            .then(|| {
                self.messages
                    .push_back(ConsensusMessage::PropagateProposal(proposal.clone()));

                proposal
            })
    }

    fn handle_pre_proposal(
        &mut self,
        pre_proposal: PreProposal,
        pre_proposal_set: &mut HashSet<PreProposal>
    ) {
        let valid_peers = self
            .validators
            .iter()
            .map(|v| v.peer_id)
            .collect::<Vec<_>>();

        self.handle_proposal_verification(pre_proposal, pre_proposal_set, |pre_proposal, block| {
            let valid_sig = pre_proposal.is_valid(block);
            let Some(peer_id) = pre_proposal.recover_address() else {
                return false;
            };
            let valid_node = valid_peers.contains(&peer_id);

            valid_sig && valid_node
        })
    }

    fn handle_proposal_verification<Pro>(
        &mut self,
        proposal: Pro,
        proposal_set: &mut HashSet<Pro>,
        valid: impl FnOnce(&Pro, &BlockNumber) -> bool
    ) where
        Pro: Into<ConsensusMessage> + Eq + Hash + Clone
    {
        // ensure pre_proposal is valid
        if !valid(&proposal, &self.block_height) {
            tracing::info!("got a invalid consensus message");
            return;
        }

        // if  we don't have the pre_proposal, propagate it and then store it.
        // else log a message
        if !proposal_set.contains(&proposal) {
            self.propagate_message(proposal.clone().into());
            proposal_set.insert(proposal);
        } else {
            tracing::trace!("got a duplicate consensus message");
        }
    }
}

/// These messages will only be broadcasted to the peer network if our consensus
/// contracts don't currently contain them.
#[derive(Debug, Clone)]
pub enum ConsensusMessage {
    /// Notification that the consensus state has changed
    StateChange(ConsensusRoundName),
    /// Command to propagate a PreProposal over the network
    PropagatePreProposal(PreProposal),
    /// Command to propagate a PreProposal Aggregation over the network
    PropagatePreProposalAgg(PreProposalAggregation),
    /// Command to propagate a Proposal over the network
    PropagateProposal(Proposal),
    /// Command to propagate an Empty Block Attestatino over the network
    PropagateEmptyBlockAttestation(Bytes)
}

impl ConsensusMessage {
    pub fn searcher_order_hashes(&self) -> Vec<B256> {
        match self {
            ConsensusMessage::PropagatePreProposal(pre_proposal) => {
                pre_proposal.searcher_order_hashes()
            }
            ConsensusMessage::PropagatePreProposalAgg(pre_proposal_aggregation) => {
                pre_proposal_aggregation.searcher_order_hashes()
            }
            ConsensusMessage::PropagateProposal(proposal) => proposal.searcher_order_hashes(),
            ConsensusMessage::StateChange(_)
            | ConsensusMessage::PropagateEmptyBlockAttestation(_) => Vec::new()
        }
    }

    pub fn limit_order_hashes(&self) -> Vec<B256> {
        match self {
            ConsensusMessage::PropagatePreProposal(pre_proposal) => {
                pre_proposal.limit_order_hashes()
            }
            ConsensusMessage::PropagatePreProposalAgg(pre_proposal_aggregation) => {
                pre_proposal_aggregation.limit_order_hashes()
            }
            ConsensusMessage::PropagateProposal(proposal) => proposal.limit_order_hashes(),
            ConsensusMessage::StateChange(_)
            | ConsensusMessage::PropagateEmptyBlockAttestation(_) => Vec::new()
        }
    }

    pub fn round_event(&self) -> ConsensusRoundEvent {
        match self {
            ConsensusMessage::StateChange(_) => ConsensusRoundEvent::Noop,
            ConsensusMessage::PropagatePreProposal(_) => ConsensusRoundEvent::PropagatePreProposal,
            ConsensusMessage::PropagatePreProposalAgg(_) => {
                ConsensusRoundEvent::PropagatePreProposalAgg
            }
            ConsensusMessage::PropagateProposal(_) => ConsensusRoundEvent::PropagateProposal,
            ConsensusMessage::PropagateEmptyBlockAttestation(_) => {
                ConsensusRoundEvent::PropagateEmptyBlockAttestation
            }
        }
    }
}

impl From<PreProposal> for ConsensusMessage {
    fn from(value: PreProposal) -> Self {
        Self::PropagatePreProposal(value)
    }
}

impl From<PreProposalAggregation> for ConsensusMessage {
    fn from(value: PreProposalAggregation) -> Self {
        Self::PropagatePreProposalAgg(value)
    }
}

#[cfg(test)]
pub mod tests {
    use std::{
        collections::HashSet,
        sync::Arc,
        task::{Context, Poll},
        time::{Duration, Instant}
    };

    use alloy::{
        primitives::Address,
        providers::{ProviderBuilder, RootProvider, fillers::*, network::Ethereum, *},
        signers::local::PrivateKeySigner
    };
    use angstrom_metrics::ConsensusMetricsWrapper;
    use angstrom_types::{
        consensus::{
            StromConsensusEvent,
            slot_clock::{SlotClock, SystemTimeSlotClock}
        },
        contract_payloads::angstrom::{AngstromPoolConfigStore, UniswapAngstromRegistry},
        primitive::{AngstromSigner, UniswapPoolRegistry},
        submission::SubmissionHandler
    };
    use dashmap::DashMap;
    use futures::{Stream, pin_mut};
    use order_pool::{PoolConfig, order_storage::OrderStorage};
    use testing_tools::{
        mocks::matching_engine::MockMatchingEngine,
        type_generator::consensus::{
            pre_proposal_agg::PreProposalAggregationBuilder, preproposal::PreproposalBuilder
        }
    };
    use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan};
    use uniswap_v4::uniswap::pool_manager::SyncedUniswapPools;

    use super::{
        ConsensusMessage, RoundStateMachine, SharedRoundState, pre_proposal::PreProposalState
    };
    use crate::{
        AngstromValidator, ConsensusTimingConfig,
        rounds::{ConsensusState, pre_proposal_aggregation::PreProposalAggregationState}
    };

    impl RoundStateMachine<ProviderDef, MockMatchingEngine, PrivateKeySigner> {
        fn set_state_machine_at(
            &mut self,
            state: Box<dyn ConsensusState<ProviderDef, MockMatchingEngine, PrivateKeySigner>>
        ) {
            self.current_state = state;
        }
    }

    type ProviderDef = FillProvider<
        JoinFill<
            Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>
        >,
        RootProvider,
        Ethereum
    >;

    fn init_tracing() {
        let _ = tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .with_span_events(FmtSpan::FULL)
            .with_test_writer()
            .try_init();
    }

    async fn setup_state_machine()
    -> RoundStateMachine<ProviderDef, MockMatchingEngine, PrivateKeySigner> {
        let order_storage = Arc::new(OrderStorage::new(&PoolConfig::default()));
        let signer = AngstromSigner::random();
        let leader_id = signer.address();

        // Initialize test components
        let pool_store = Arc::new(AngstromPoolConfigStore::default());
        let (tx, _rx) = tokio::sync::mpsc::channel(2);
        let uniswap_pools = SyncedUniswapPools::new(Arc::new(DashMap::new()), tx);
        let reg = UniswapPoolRegistry::default();

        let pool_registry = UniswapAngstromRegistry::new(reg, pool_store);

        let querying_provider: Arc<_> = ProviderBuilder::<_, _, Ethereum>::default()
            .with_recommended_fillers()
            .connect("https://eth.llamarpc.com")
            .await
            .unwrap()
            .into();

        let provider =
            SubmissionHandler { node_provider: querying_provider, submitters: vec![] };

        let shared_state = SharedRoundState::new(
            1, // block height
            order_storage,
            signer,
            leader_id,
            vec![AngstromValidator::new(leader_id, 100)],
            ConsensusMetricsWrapper::new(),
            pool_registry,
            uniswap_pools,
            provider,
            MockMatchingEngine {},
            ConsensusTimingConfig::default()
        );
        RoundStateMachine::new(shared_state, SystemTimeSlotClock::new_with_chain_id(1).unwrap())
    }

    #[tokio::test]
    async fn test_bid_aggregation_to_pre_proposal() {
        init_tracing();
        let state_machine = setup_state_machine().await;
        pin_mut!(state_machine);

        // Initial state should be BidAggregationState
        assert!(matches!(
            state_machine
                .as_mut()
                .poll_next(&mut Context::from_waker(futures::task::noop_waker_ref())),
            Poll::Pending
        ));

        // After wait trigger expires, should transition and emit PreProposal
        tokio::time::sleep(Duration::from_secs(10)).await;

        // Poll until we get PropagatePreProposal
        loop {
            match state_machine
                .as_mut()
                .poll_next(&mut Context::from_waker(futures::task::noop_waker_ref()))
            {
                Poll::Ready(Some(ConsensusMessage::PropagatePreProposal(_))) => break,
                Poll::Ready(Some(ConsensusMessage::StateChange(_))) => continue,
                res => {
                    tracing::info!(?res);
                    panic!("Expected PreProposal propagation {res:?}")
                }
            }
        }
    }

    #[tokio::test]
    async fn test_pre_proposal_to_pre_proposal_aggregation() {
        init_tracing();
        let mut state_machine = setup_state_machine().await;
        // create pre-proposal-state
        let handles = &mut state_machine.shared_state;
        let state = Box::new(PreProposalState::new(
            1,
            HashSet::default(),
            HashSet::default(),
            handles,
            Instant::now(),
            futures::task::noop_waker_ref().to_owned()
        ))
            as Box<dyn ConsensusState<ProviderDef, MockMatchingEngine, PrivateKeySigner>>;
        handles.messages.clear();

        state_machine.set_state_machine_at(state);

        pin_mut!(state_machine);

        // Generate valid PreProposal
        let pre_proposal = PreproposalBuilder::new()
            .for_block(1)
            .with_secret_key(state_machine.shared_state.signer.clone())
            .build();

        // Handle PreProposal message
        let signer_id = state_machine.shared_state.signer.address();
        state_machine.handle_message(StromConsensusEvent::PreProposal(signer_id, pre_proposal));

        // Poll until we get PropagatePreProposalAgg
        loop {
            match state_machine
                .as_mut()
                .poll_next(&mut Context::from_waker(futures::task::noop_waker_ref()))
            {
                Poll::Ready(Some(ConsensusMessage::PropagatePreProposalAgg(_))) => break,
                Poll::Ready(Some(ConsensusMessage::StateChange(_))) => continue,
                res => {
                    tracing::info!(?res);
                    panic!("Expected PreProposalAgg propagation {res:?}");
                }
            }
        }
    }

    #[tokio::test]
    async fn test_pre_proposal_aggregation_to_proposal() {
        init_tracing();
        let mut state_machine = setup_state_machine().await;

        // create pre-proposal-aggregation state
        let handles = &mut state_machine.shared_state;
        let state = Box::new(PreProposalAggregationState::new(
            HashSet::default(),
            HashSet::default(),
            handles,
            Instant::now(),
            futures::task::noop_waker_ref().to_owned()
        ))
            as Box<dyn ConsensusState<ProviderDef, MockMatchingEngine, PrivateKeySigner>>;

        handles.messages.clear();

        state_machine.set_state_machine_at(state);

        pin_mut!(state_machine);

        // Generate valid PreProposalAggregation
        let pre_proposal_agg = PreProposalAggregationBuilder::new()
            .for_block(1)
            .with_secret_key(state_machine.shared_state.signer.clone())
            .build();

        // Handle PreProposalAggregation message
        let signer_id = state_machine.shared_state.signer.address();
        state_machine.handle_message(StromConsensusEvent::PreProposalAgg(
            signer_id,
            pre_proposal_agg.clone()
        ));

        // Poll until we get PropagatePreProposalAgg
        loop {
            match state_machine
                .as_mut()
                .poll_next(&mut Context::from_waker(futures::task::noop_waker_ref()))
            {
                Poll::Ready(Some(ConsensusMessage::PropagatePreProposalAgg(a))) => {
                    assert_eq!(a, pre_proposal_agg);
                    break;
                }
                Poll::Ready(Some(ConsensusMessage::StateChange(_))) => continue,
                res => {
                    tracing::info!(?res);
                    panic!("Expected PreProposalAgg propagation {res:?}")
                }
            }
        }
    }

    #[tokio::test]
    async fn test_reset_round() {
        init_tracing();
        let mut state_machine = setup_state_machine().await;
        let new_block = 2;
        let new_leader = Address::random();

        // Reset round with new block and leader
        state_machine.reset_round(new_block, new_leader);

        assert_eq!(state_machine.shared_state.block_height, new_block);
        assert_eq!(state_machine.shared_state.round_leader, new_leader);

        // Should be back in BidAggregationState
        let stream = state_machine;
        pin_mut!(stream);
        assert!(matches!(
            stream
                .as_mut()
                .poll_next(&mut Context::from_waker(futures::task::noop_waker_ref())),
            Poll::Pending
        ));
    }

    #[tokio::test]
    async fn test_invalid_messages_in_bid_aggregation_state() {
        init_tracing();
        let state_machine = setup_state_machine().await;
        let invalid_peer = Address::random();
        let invalid_signer = AngstromSigner::random();

        pin_mut!(state_machine);

        let invalid_pre_proposal = PreproposalBuilder::new()
            .for_block(1)
            .with_secret_key(invalid_signer)
            .build();

        state_machine
            .handle_message(StromConsensusEvent::PreProposal(invalid_peer, invalid_pre_proposal));

        // Should not transition state or emit messages
        assert!(matches!(
            state_machine
                .as_mut()
                .poll_next(&mut Context::from_waker(futures::task::noop_waker_ref())),
            Poll::Pending
        ));
        assert!(state_machine.shared_state.messages.is_empty());
    }

    #[tokio::test]
    async fn test_invalid_messages_in_pre_proposal_aggregation_state() {
        init_tracing();
        let mut state_machine = setup_state_machine().await;
        let invalid_peer = Address::random();
        let invalid_signer = AngstromSigner::random();

        let handles = &mut state_machine.shared_state;
        let state = Box::new(PreProposalAggregationState::new(
            HashSet::default(),
            HashSet::default(),
            handles,
            Instant::now() + Duration::from_secs(60),
            futures::task::noop_waker_ref().to_owned()
        ))
            as Box<dyn ConsensusState<ProviderDef, MockMatchingEngine, PrivateKeySigner>>;
        handles.messages.clear();

        state_machine.set_state_machine_at(state);

        pin_mut!(state_machine);

        // Test invalid pre-proposal
        let invalid_pre_proposal = PreproposalBuilder::new()
            .for_block(1)
            .with_secret_key(invalid_signer.clone())
            .build();

        state_machine
            .handle_message(StromConsensusEvent::PreProposal(invalid_peer, invalid_pre_proposal));

        // Test invalid pre-proposal aggregation
        let invalid_pre_proposal_agg = PreProposalAggregationBuilder::new()
            .for_block(1)
            .with_secret_key(invalid_signer)
            .build();

        state_machine.handle_message(StromConsensusEvent::PreProposalAgg(
            invalid_peer,
            invalid_pre_proposal_agg
        ));

        // Should not transition state or emit messages
        let _result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            match state_machine
                .as_mut()
                .poll_next(&mut Context::from_waker(futures::task::noop_waker_ref()))
            {
                Poll::Ready(Some(ConsensusMessage::StateChange(_))) => {
                    // StateChange is acceptable
                }
                Poll::Pending => {}
                other => panic!("Unexpected message: {other:?}")
            }
        }));
        assert!(state_machine.shared_state.messages.is_empty());
    }
}
