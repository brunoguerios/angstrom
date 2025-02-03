use alloy_primitives::U256;
use angstrom_types::{
    orders::{NetAmmOrder, OrderOutcome, PoolSolution},
    sol_bindings::{
        grouped_orders::{
            FlashVariants, GroupedVanillaOrder, OrderWithStorageData, StandingVariants
        },
        rpc_orders::TopOfBlockOrder,
        RawPoolOrder, Ray
    }
};

use super::Solution;
use crate::OrderBook;

#[derive(Clone)]
pub struct BinarySearchMatcher<'a> {
    book: &'a OrderBook
}

impl<'a> BinarySearchMatcher<'a> {
    pub fn new(book: &'a OrderBook) -> Self {
        Self { book }
    }

    fn fetch_concentrated_liquidity(&self, price: Ray) -> Ray {
        let Some(book) = self.book.amm() else { return Ray::default() };
        let Some(am) = book.get_quantity_to_price(price) else { return Ray::default() };
        let is_bid = book.is_bid(price);
        let mut scaled = Ray::scale_to_ray(U256::from(am));

        // if its a bid, we need to convert to ask value
        if is_bid {
            scaled.div_ray_assign(price);
        }

        scaled
    }

    fn fetch_exact_in_ask_orders(&self, price: Ray) -> Ray {
        // grab all exact in ask orders where price >= limit price and then get y_in
        self.book
            .asks()
            .iter()
            .filter(|ask| price >= ask.price() && ask.exact_in())
            .map(|ask| Ray::scale_to_ray(U256::from(ask.amount())))
            .sum()
    }

    fn fetch_exact_out_ask_orders(&self, price: Ray) -> Ray {
        self.book
            .asks()
            .iter()
            .filter(|ask| price >= ask.price() && !ask.exact_in())
            .map(|ask| Ray::scale_to_ray(U256::from(ask.amount())).div_ray(price))
            .sum()
    }

    fn fetch_exact_in_bid_orders(&self, price: Ray) -> Ray {
        self.book
            .bids()
            .iter()
            // price is inv given price is always t0 / t1
            .filter(|bid| price <= bid.price().inv_ray_round(true) && bid.exact_in())
            .map(|bid| Ray::scale_to_ray(U256::from(bid.amount())).div_ray(price))
            .sum()
    }

    fn fetch_exact_out_bid_orders(&self, price: Ray) -> Ray {
        self.book
            .bids()
            .iter()
            // price is inv given price is always t0 / t1
            .filter(|bid| price <= bid.price().inv_ray_round(true) && !bid.exact_in())
            .map(|bid| Ray::scale_to_ray(U256::from(bid.amount())))
            .sum()
    }

    fn fetch_exact_in_partial_ask(&self, price: Ray) -> PartialLiqRange {
        let mut removal = None;

        let filled = self
            .book
            .asks()
            .iter()
            .filter(|ask| ask.is_partial())
            .filter_map(|ask| {
                // if we are on the exact, we can partial
                if price == ask.price() {
                    let (max, min) = match &ask.order {
                        GroupedVanillaOrder::Standing(StandingVariants::Partial(p)) => {
                            (p.max_amount_in, p.min_amount_in)
                        }
                        GroupedVanillaOrder::KillOrFill(FlashVariants::Partial(p)) => {
                            (p.max_amount_in, p.min_amount_in)
                        }
                        _ => panic!("not valid")
                    };
                    let (max, min) =
                        (Ray::scale_to_ray(U256::from(max)), Ray::scale_to_ray(U256::from(min)));
                    removal = Some(max - min);

                    Some(max)
                } else if price > ask.price() {
                    Some(Ray::scale_to_ray(U256::from(ask.amount())))
                } else {
                    None
                }
            })
            .sum();

        PartialLiqRange { filled_quantity: filled, optional_removal_liq: removal }
    }

    fn fetch_exact_in_partial_bid(&self, price: Ray) -> PartialLiqRange {
        let mut removal = None;

        let filled = self
            .book
            .bids()
            .iter()
            .filter(|bid| bid.is_partial())
            .filter_map(|bid| {
                // if we are on the exact, we can partial
                if price == bid.price() {
                    let (max, min) = match &bid.order {
                        GroupedVanillaOrder::Standing(StandingVariants::Partial(p)) => {
                            (p.max_amount_in, p.min_amount_in)
                        }
                        GroupedVanillaOrder::KillOrFill(FlashVariants::Partial(p)) => {
                            (p.max_amount_in, p.min_amount_in)
                        }
                        _ => panic!("not valid")
                    };

                    let (max, min) =
                        (Ray::scale_to_ray(U256::from(max)), Ray::scale_to_ray(U256::from(min)));
                    removal = Some(max.div_ray(price) - min.div_ray(price));

                    Some(max.div_ray(price))
                } else if price < bid.price().inv_ray() {
                    Some(Ray::scale_to_ray(U256::from(bid.amount())).div_ray(price))
                } else {
                    None
                }
            })
            .sum();

        PartialLiqRange { filled_quantity: filled, optional_removal_liq: removal }
    }

    fn total_supply_at_price(&self, price: Ray) -> (Ray, Option<Ray>) {
        let partial = self.fetch_exact_in_partial_ask(price);
        (
            self.fetch_concentrated_liquidity(price)
                + self.fetch_exact_in_ask_orders(price)
                + self.fetch_exact_out_ask_orders(price)
                + partial.filled_quantity,
            partial.optional_removal_liq
        )
    }

    fn total_demand_at_price(&self, price: Ray) -> (Ray, Option<Ray>) {
        let partial = self.fetch_exact_in_partial_bid(price);
        (
            self.fetch_concentrated_liquidity(price)
                + self.fetch_exact_in_bid_orders(price)
                + self.fetch_exact_out_bid_orders(price)
                + partial.filled_quantity,
            partial.optional_removal_liq
        )
    }

    /// calculates given the supply, demand, optional supply and optional demand
    /// what way the algo's price should move if we want it too
    fn calculate_solver_move(&self, p_mid: Ray) -> SupplyDemandResult {
        let (total_supply, sub_sup) = self.total_supply_at_price(p_mid);
        let (total_demand, sub_demand) = self.total_demand_at_price(p_mid);

        cmp_total_supply_vs_demand(
            total_supply,
            total_demand,
            sub_sup.unwrap_or_default(),
            sub_demand.unwrap_or_default()
        )
    }

    /// helper functions for grabbing all orders that we filled at ucp
    fn fetch_orders_at_ucp(&self, ucp: Ray) -> Vec<OrderOutcome> {
        vec![]
    }

    fn fetch_amm_movement_at_ucp(&self, ucp: Ray) -> Option<NetAmmOrder> {
        None
    }

    pub fn solution(
        &self,
        searcher: Option<OrderWithStorageData<TopOfBlockOrder>>
    ) -> PoolSolution {
        let price_and_partial_solution = self.solve_clearing_price();

        todo!()
    }

    fn solve_clearing_price(&self) -> Option<UcpSolution> {
        let ep = Ray::from(U256::from(1));
        let mut p_max = Ray::from(self.book.highest_clearing_price().saturating_add(*ep));
        let mut p_min = Ray::from(self.book.lowest_clearing_price().saturating_sub(*ep));

        println!("min: {p_min:?} max: {p_max:?}");

        let two = U256::from(1);
        while (p_max - p_min) > ep {
            // grab all supply and demand
            let p_mid = Ray::from((p_max + p_min) / two);
            println!("solving clearing price iter price = {p_mid:?}");

            let res = self.calculate_solver_move(p_mid);

            match res {
                SupplyDemandResult::MoreSupply => {
                    p_max = p_mid;
                }
                SupplyDemandResult::MoreDemand => p_min = p_mid,
                SupplyDemandResult::NaturallyEqual => {
                    println!("solved based on sup, demand no partials");

                    return Some(UcpSolution {
                        ucp:                   p_mid,
                        partial_unfill_side:   None,
                        partial_unfill_amount: None
                    })
                }
                SupplyDemandResult::PartialFillEq { side, amount_unfilled } => {
                    println!("solved based on sup, demand with partial order");
                    return Some(UcpSolution {
                        ucp:                   p_mid,
                        partial_unfill_side:   Some(side),
                        partial_unfill_amount: Some(amount_unfilled)
                    })
                }
            }

            println!("min: {p_min:?} max: {p_max:?}");
        }

        None
        // (p_max + p_min) / two
    }
}

fn cmp_total_supply_vs_demand(
    total_supply: Ray,
    total_demand: Ray,
    sub_sup: Ray,
    sub_dem: Ray
) -> SupplyDemandResult {
    println!("sup: {:#?} demand: {:#?}", total_supply, total_demand);

    // if we can subtract the extra supply or demand and flip the equality, we have
    // reached ucp
    if (total_supply >= total_demand && (total_supply - sub_sup) <= total_demand)
        || (total_supply <= total_demand && (total_demand - sub_dem) <= total_supply)
    {
        println!("solved partial fill");
        return SupplyDemandResult::PartialFillEq {
            side:            true,
            amount_unfilled: Ray::default()
        }
    }
    if total_supply > total_demand {
        SupplyDemandResult::MoreSupply
    } else if total_demand > total_supply {
        SupplyDemandResult::MoreDemand
    } else {
        SupplyDemandResult::NaturallyEqual
    }
}

#[derive(Debug)]
struct UcpSolution {
    ucp:                   Ray,
    partial_unfill_side:   Option<bool>,
    partial_unfill_amount: Option<Ray>
}

#[derive(Debug)]
enum SupplyDemandResult {
    MoreSupply,
    MoreDemand,
    NaturallyEqual,
    PartialFillEq {
        // true is supply
        side:            bool,
        amount_unfilled: Ray
    }
}

#[derive(Debug, Default)]
struct PartialLiqRange {
    /// the quantity that filled up to the price.
    pub filled_quantity:      Ray,
    /// the amount of supply or demand that we can remove at the current ucp
    pub optional_removal_liq: Option<Ray>
}

#[cfg(test)]
pub mod test {

    use std::{cell::Cell, cmp::max};

    use alloy::primitives::{Uint, U256};
    use alloy_primitives::FixedBytes;
    use angstrom_types::{
        matching::{uniswap::PoolSnapshot, Debt, DebtType, Ray, SqrtPriceX96},
        orders::OrderFillState,
        primitive::PoolId
    };
    use testing_tools::type_generator::{
        amm::generate_amm_with_liquidity, orders::UserOrderBuilder
    };

    use crate::{
        book::{order::OrderContainer, BookOrder, OrderBook},
        matcher::binary_search::BinarySearchMatcher
    };

    // Helper function to create AMM with specific liquidity
    fn create_amm_at_price(price: Ray, liquidity: u128) -> PoolSnapshot {
        let sqrt_price: SqrtPriceX96 = price.into();
        let tick = sqrt_price.to_tick().unwrap();
        generate_amm_with_liquidity(tick - 10, tick + 10, liquidity, sqrt_price)
    }

    #[test]
    fn ask_side_double_match_works_with_amm_binary_search() {
        let pool_id = PoolId::random();
        let bid_price = Ray::from(Uint::from(1_000_000_000_u128)).inv_ray_round(true);
        let low_price = Ray::from(Uint::from(1_000_u128));

        let bid_order = UserOrderBuilder::new()
            .partial()
            .amount(10)
            .min_price(bid_price)
            .with_storage()
            .bid()
            .build();
        let ask_order = UserOrderBuilder::new()
            .exact()
            .amount(10)
            .min_price(low_price)
            .with_storage()
            .ask()
            .build();
        let book = OrderBook::new(pool_id, None, vec![bid_order.clone()], vec![ask_order], None);
        println!("{:#?}", book);
        let matcher = BinarySearchMatcher::new(&book);
        let ucp = matcher.solve_clearing_price().unwrap();
        let bid = bid_price.inv_ray_round(true);
        assert!(
            ucp == bid,
            "Bid outweighed but the final price wasn't properly set {ucp:?}, {bid:?}"
        );
    }

    #[test]
    fn solve_bid_ex_out_ask_ex_in() {
        let pool_id = PoolId::random();
        let high_price = Ray::from(Uint::from(1_000_000_000_u128));
        let bid_order = UserOrderBuilder::new()
            .exact()
            .amount(100)
            .exact_in(false)
            .min_price(high_price)
            .with_storage()
            .bid()
            .build();

        let ask_order = UserOrderBuilder::new()
            .exact()
            .amount(100)
            .exact_in(true)
            .min_price(high_price)
            .with_storage()
            .ask()
            .build();
        let book = OrderBook::new(pool_id, None, vec![bid_order.clone()], vec![ask_order], None);
        let matcher = BinarySearchMatcher::new(&book);
        let ucp = matcher.solve_clearing_price().unwrap();
        assert!(
            ucp == high_price,
            "Ask outweighed but the final price wasn't properly set {ucp:?} {high_price:?}"
        );
    }

    #[test]
    fn solve_bid_ex_in_ask_ex_out() {
        let pool_id = PoolId::random();

        // 0.000000000000000001
        let high_price = Ray::from(Uint::from(1_000_000_000_u128));

        let bid_order = UserOrderBuilder::new()
            .exact()
            .amount(100)
            .exact_in(true)
            .min_price(high_price)
            .with_storage()
            .bid()
            .build();

        let ask_order = UserOrderBuilder::new()
            .exact()
            .amount(100)
            .exact_in(false)
            .min_price(high_price)
            .with_storage()
            .ask()
            .build();

        let book = OrderBook::new(pool_id, None, vec![bid_order.clone()], vec![ask_order], None);
        let matcher = BinarySearchMatcher::new(&book);
        let ucp = matcher.solve_clearing_price().unwrap();
        assert!(
            ucp == high_price,
            "Ask outweighed but the final price wasn't properly set {ucp:?} {high_price:?}"
        );
    }

    #[test]
    fn basic_solve_of_exact_orders_exact_out() {
        let pool_id = PoolId::random();
        // 0.0000000000001 rate
        let high_price = Ray::from(Uint::from(100_000_000_000_000u128));
        // both exact out so bid is 100 y

        // demand
        let bid_order = UserOrderBuilder::new()
            .exact()
            .amount(100_000_000_000_000)
            .exact_in(false)
            .min_price(high_price)
            .with_storage()
            .bid()
            .build();

        // supply
        let ask_order = UserOrderBuilder::new()
            .exact()
            .amount(10)
            .exact_in(false)
            .min_price(high_price)
            .with_storage()
            .ask()
            .build();
        let book = OrderBook::new(pool_id, None, vec![bid_order.clone()], vec![ask_order], None);
        let matcher = BinarySearchMatcher::new(&book);
        let ucp = matcher.solve_clearing_price().unwrap();

        assert!(
            ucp == high_price,
            "Ask outweighed but the final price wasn't properly set {ucp:?} {high_price:?}"
        );
    }

    #[test]
    fn test_amm_only_clearing() {
        let pool_id = PoolId::random();
        let price = Ray::from(Uint::from(1_000_000_000_u128)); // 1.0
        let amm = create_amm_at_price(price, 1_000_000); // Create AMM with significant liquidity

        let book = OrderBook::new(pool_id, Some(amm), vec![], vec![], None);
        let matcher = BinarySearchMatcher::new(&book);
        let ucp = matcher.solve_clearing_price().unwrap();

        println!("{ucp:?}");
        // let diff
        // assert!(
        //     (ucp - price).abs() <= Ray::from(U256::from(1)),
        //     "AMM-only clearing price should match initial price. Got {ucp:?},
        // expected {price:?}" );
    }

    #[test]
    fn test_amm_with_orders_clearing() {
        let pool_id = PoolId::random();
        let initial_price = Ray::from(Uint::from(1_000_000_000_u128)); // 1.0
        let amm = create_amm_at_price(initial_price, 1_000_000);

        // Add a large bid slightly below AMM price
        let bid_price = Ray::from(Uint::from(999_000_000_u128)); // 0.999
        let bid_order = UserOrderBuilder::new()
            .exact()
            .amount(2_000_000)
            .exact_in(true)
            .min_price(bid_price)
            .with_storage()
            .bid()
            .build();

        // Add a large ask slightly above AMM price
        let ask_price = Ray::from(Uint::from(1_001_000_000_u128)); // 1.001
        let ask_order = UserOrderBuilder::new()
            .exact()
            .amount(2_000_000)
            .exact_in(true)
            .min_price(ask_price)
            .with_storage()
            .ask()
            .build();

        let book = OrderBook::new(pool_id, Some(amm), vec![bid_order], vec![ask_order], None);
        let matcher = BinarySearchMatcher::new(&book);
        let ucp = matcher.solve_clearing_price().unwrap();

        // UCP should be between bid and ask prices due to AMM liquidity
        assert!(
            ucp >= bid_price && ucp <= ask_price,
            "Clearing price should be between bid and ask. Got {ucp:?}"
        );
    }

    #[test]
    fn test_amm_price_impact() {
        let pool_id = PoolId::random();
        let initial_price = Ray::from(Uint::from(1_000_000_000_u128)); // 1.0
        let amm = create_amm_at_price(initial_price, 100_000); // Lower liquidity for more price impact

        // Large bid order that should move price up
        let bid_price = Ray::from(Uint::from(1_100_000_000_u128)); // 1.1
        let bid_order = UserOrderBuilder::new()
            .exact()
            .amount(1_000_000)
            .exact_in(true)
            .min_price(bid_price)
            .with_storage()
            .bid()
            .build();

        let book = OrderBook::new(pool_id, Some(amm), vec![bid_order], vec![], None);
        let matcher = BinarySearchMatcher::new(&book);
        let ucp = matcher.solve_clearing_price().unwrap();

        assert!(
            ucp > initial_price,
            "Large bid should move price up from AMM initial price. Got {ucp:?}, initial was \
             {initial_price:?}"
        );
    }

    #[test]
    fn basic_solve_of_exact_orders_exact_in() {
        let pool_id = PoolId::random();
        // 0.0000000000001 rate
        let high_price = Ray::from(Uint::from(100_000_000_000_000u128));
        let bid_order = UserOrderBuilder::new()
            .exact()
            .amount(10)
            .exact_in(true)
            .min_price(high_price)
            .with_storage()
            .bid()
            .build();

        let ask_order = UserOrderBuilder::new()
            .exact()
            .amount(100_000_000_000_000)
            .exact_in(true)
            .min_price(high_price)
            .with_storage()
            .ask()
            .build();
        let book = OrderBook::new(pool_id, None, vec![bid_order.clone()], vec![ask_order], None);
        let matcher = BinarySearchMatcher::new(&book);
        let ucp = matcher.solve_clearing_price().unwrap();
        assert!(
            ucp == high_price,
            "Ask outweighed but the final price wasn't properly set {ucp:?} {high_price:?}"
        );
    }
}
