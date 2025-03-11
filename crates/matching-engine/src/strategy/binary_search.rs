use angstrom_types::{
    orders::PoolSolution,
    sol_bindings::{grouped_orders::OrderWithStorageData, rpc_orders::TopOfBlockOrder}
};

use crate::{book::OrderBook, matcher::delta::DeltaMatcher};

pub struct BinarySearchStrategy {}

impl BinarySearchStrategy {
    pub fn run(
        book: &OrderBook,
        searcher: Option<OrderWithStorageData<TopOfBlockOrder>>,
        fee: u128
    ) -> PoolSolution {
        let mut matcher = DeltaMatcher::new(book, searcher.clone(), fee, false);
        matcher.solution(searcher)
    }
}
