use alloy::primitives::I256;
use angstrom_types::{
    amm::StatefulPoolSwap,
    matching::SqrtPriceX96,
    orders::PoolSolution,
    sol_bindings::{Ray, grouped_orders::OrderWithStorageData, rpc_orders::TopOfBlockOrder}
};

use super::end_amm_state::EndAmmState;
use crate::{book::OrderBook, matcher::delta::DeltaMatcher};

pub struct BinarySearchStrategy {}

impl BinarySearchStrategy {
    pub fn run(
        book: &OrderBook,
        searcher: Option<OrderWithStorageData<TopOfBlockOrder>>
    ) -> PoolSolution {
        let mut matcher = DeltaMatcher::new(book, searcher.clone().into(), false);
        matcher.solution(searcher)
    }

    /// Get the end AMM state after matching
    pub fn give_end_amm_state(
        book: &OrderBook,
        searcher: Option<OrderWithStorageData<TopOfBlockOrder>>
    ) -> EndAmmState {
        let pool_state = book.amm().expect("Expected AMM state");

        match pool_state {
            _ if pool_state.as_uniswap().is_some() => {
                let snapshot = pool_state.as_uniswap().unwrap();

                if book.is_empty_book() {
                    return EndAmmState::Uniswap {
                        price:     *snapshot.current_price(),
                        tick:      snapshot.current_tick(),
                        liquidity: snapshot.current_liquidity()
                    };
                }

                let mut matcher = DeltaMatcher::new(book, searcher.clone().into(), false);
                let solution = matcher.solution(searcher);

                if solution.ucp.is_zero() {
                    let amm = matcher.try_get_amm_location();
                    match amm {
                        StatefulPoolSwap::Uniswap(uni) => EndAmmState::Uniswap {
                            price:     *uni.end_price,
                            tick:      uni.end_tick,
                            liquidity: uni.end_liquidity.liquidity()
                        },
                        _ => unreachable!("Uniswap pool should return Uniswap result")
                    }
                } else {
                    let post_tob_swap = matcher.try_get_amm_location();
                    let ucp: SqrtPriceX96 = solution.ucp.into();

                    // This path only works for Uniswap
                    let StatefulPoolSwap::Uniswap(post_tob) = post_tob_swap else {
                        panic!("Expected Uniswap stateful swap");
                    };

                    let book_swap_vec = post_tob.swap_to_price(ucp);

                    let net_t0 = book_swap_vec
                        .as_ref()
                        .map(|b| b.t0_signed())
                        .unwrap_or(I256::ZERO)
                        + post_tob.t0_signed();

                    let net_direction = net_t0.is_negative();
                    let amount_in = if net_t0.is_negative() {
                        net_t0.unsigned_abs()
                    } else {
                        (book_swap_vec
                            .as_ref()
                            .map(|b| b.t1_signed())
                            .unwrap_or(I256::ZERO)
                            + post_tob.t1_signed())
                        .unsigned_abs()
                    };

                    let res = snapshot
                        .swap_current_with_amount(I256::from_raw(amount_in), net_direction)
                        .unwrap()
                        .clone();

                    EndAmmState::Uniswap {
                        price:     *res.end_price,
                        tick:      res.end_tick,
                        liquidity: res.end_liquidity.liquidity()
                    }
                }
            }
            _ if pool_state.as_balancer().is_some() => {
                // TODO Step 9: Implement Balancer end state calculation
                let bal = pool_state.as_balancer().unwrap();
                EndAmmState::Balancer { price: Ray::from(bal.current_price().value()) }
            }
            _ => panic!("Unknown pool type")
        }
    }
}
