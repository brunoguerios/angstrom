use std::collections::HashMap;

use alloy::primitives::Address;
use angstrom_types::{
    contract_payloads::angstrom::BundleGasDetails,
    orders::PoolSolution,
    primitive::PoolId,
    sol_bindings::{grouped_orders::OrderWithStorageData, rpc_orders::TopOfBlockOrder},
    uni_structure::UniswapPoolState
};
use futures::{FutureExt, future::BoxFuture};
use matching_engine::{MatchingEngineHandle, book::BookOrder, manager::MatchingEngineError};

#[derive(Clone)]
pub struct MockMatchingEngine {}

impl MatchingEngineHandle for MockMatchingEngine {
    fn solve_pools(
        &self,
        _: Vec<BookOrder>,
        _: Vec<OrderWithStorageData<TopOfBlockOrder>>,
        _: HashMap<PoolId, (Address, Address, UniswapPoolState, u16)>
    ) -> BoxFuture<Result<(Vec<PoolSolution>, BundleGasDetails), MatchingEngineError>> {
        async move { Ok((vec![], BundleGasDetails::default())) }.boxed()
    }
}
