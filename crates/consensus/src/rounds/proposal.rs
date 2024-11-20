use std::{
    collections::HashSet,
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker}
};

use alloy::{
    network::TransactionBuilder,
    primitives::{Address, BlockNumber, FixedBytes},
    providers::Provider,
    rpc::types::TransactionRequest,
    transports::Transport
};
use angstrom_network::manager::StromConsensusEvent;
use angstrom_types::{
    consensus::{PreProposal, PreProposalAggregation, Proposal},
    contract_payloads::angstrom::{AngstromBundle, BundleGasDetails},
    orders::PoolSolution
};
use futures::{future::BoxFuture, FutureExt};
use matching_engine::MatchingEngineHandle;
use pade::PadeEncode;

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
    matching_engine_future:
        Option<BoxFuture<'static, eyre::Result<(Vec<PoolSolution>, BundleGasDetails)>>>,
    submission_future:      Option<()>,
    pre_proposal_aggs:      Vec<PreProposalAggregation>,
    waker:                  Waker
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
            matching_engine_future: Some(
                handles.matching_engine_output(pre_proposal_aggregation.clone())
            ),
            pre_proposal_aggs: pre_proposal_aggregation.into_iter().collect::<Vec<_>>(),
            submission_future: None,
            waker
        }
    }

    fn try_build_proposal<T, Matching>(
        &self,
        result: eyre::Result<(Vec<PoolSolution>, BundleGasDetails)>,
        handles: &mut Consensus<T, Matching>
    ) -> bool
    where
        T: Transport + Clone,
        Matching: MatchingEngineHandle
    {
        let Ok((pool_solution, gas_info)) = result else {
            tracing::error!("Failed to properly build proposal, THERE SHALL BE NO PROPOSAL THIS BLOCK :(");
            return false
        };

        let proposal = Proposal::generate_proposal(
            handles.block_height,
            handles.signer.my_id,
            self.pre_proposal_aggs.clone(),
            pool_solution,
            &handles.signer.key
        );
        let snapshot = handles.fetch_pool_snapshot();

        let Ok(bundle) = AngstromBundle::from_proposal(&proposal, gas_info, &snapshot) else {
            tracing::error!("failed to encode angstrom bundle, THERE SHALL BE NO PROPOSAL THIS BLOCK :(");
            return false
        };

        let tx = TransactionRequest::default()
            .with_to(handles.angstrom_address)
            .with_input(bundle.pade_encode());

        let submitted_tx = provider
            .send_transaction(tx)
            .await
            .map_err(|_| RoundStateMachineError::TransactionError)?;
        let _receipt = submitted_tx
            .get_receipt()
            .await
            .map_err(|_| RoundStateMachineError::TransactionError)?;

        todo!()
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
        if let Some(b_fut) = self.matching_engine_future.take() {
            match b_fut.poll_unpin(cx) {
                Poll::Ready(state) => {}
                Poll::Pending => self.building_future = Some(b_fut)
            }
        }

        // if let Some(b_fut) = self.submission_future.take() {
        //     match b_fut.poll_unpin(cx) {
        //         Poll::Ready(_) => {}
        //         Poll::Pending => self.building_future = Some(b_fut)
        //     }
        // }
    }
}
