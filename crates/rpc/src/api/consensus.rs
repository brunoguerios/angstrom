use std::collections::{HashMap, HashSet};

use alloy_primitives::{Address, B256, U256};
use angstrom_types::{
    orders::{CancelOrderRequest, OrderLocation, OrderStatus},
    primitive::PoolId,
    sol_bindings::grouped_orders::AllOrders
};
use consensus::AngstromValidator;
use futures::StreamExt;
use jsonrpsee::{
    core::{RpcResult, Serialize},
    proc_macros::rpc
};
use serde::Deserialize;

#[cfg_attr(not(feature = "client"), rpc(server, namespace = "consensus"))]
#[cfg_attr(feature = "client", rpc(server, client, namespace = "consensus"))]
#[async_trait::async_trait]
pub trait ConsensusApi {
    #[subscription(
        name = "subscribeEmptyBlockAttestations",
        unsubscribe = "unsubscribeEmptyBlockAttestations",
        item = alloy_primitives::Bytes
    )]
    async fn subscribe_empty_block_attestations(&self) -> jsonrpsee::core::SubscriptionResult;

    #[method(name = "getCurrentLeader")]
    async fn get_current_leader(&self) -> RpcResult<Address>;

    #[method(name = "fetchConsensusState")]
    async fn fetch_consensus_state(&self) -> RpcResult<HashSet<AngstromValidator>>;
}
