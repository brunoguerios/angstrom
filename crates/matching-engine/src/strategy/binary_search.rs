use angstrom_types::{
    orders::PoolSolution,
    sol_bindings::{grouped_orders::OrderWithStorageData, rpc_orders::TopOfBlockOrder}
};

use crate::{book::OrderBook, matcher::binary_search::BinarySearchMatcher};

pub struct BinarySearchStrategy {}

impl BinarySearchStrategy {
    pub fn run(
        self,
        book: &OrderBook,
        searcher: Option<OrderWithStorageData<TopOfBlockOrder>>
    ) -> PoolSolution {
        let matcher = BinarySearchMatcher::new(book);
        matcher.solution(searcher)
    }
}
