use alloy_primitives::U256;
use angstrom_types::sol_bindings::{
    grouped_orders::{FlashVariants, GroupedVanillaOrder, StandingVariants},
    RawPoolOrder, Ray
};

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

    // supply
    fn fetch_exact_in_ask_orders(&self, price: Ray) -> Ray {
        // grab all exact in ask orders where price >= limit price and then get y_in
        self.book
            .asks()
            .iter()
            .filter(|ask| price >= ask.price() && ask.exact_in())
            .map(|ask| Ray::scale_to_ray(U256::from(ask.amount())))
            .sum()
    }

    // supply
    fn fetch_exact_out_ask_orders(&self, price: Ray) -> Ray {
        self.book
            .asks()
            .iter()
            .filter(|ask| price >= ask.price() && !ask.exact_in())
            .map(|ask| Ray::scale_to_ray(U256::from(ask.amount())).div_ray(price))
            .sum()
    }

    // demand
    fn fetch_exact_in_bid_orders(&self, price: Ray) -> Ray {
        self.book
            .bids()
            .iter()
            // price is inv given price is always t0 / t1
            .filter(|bid| price <= bid.price().inv_ray_round(true) && bid.exact_in())
            .map(|bid| Ray::scale_to_ray(U256::from(bid.amount())).div_ray(price))
            .sum()
    }

    // demand
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
        let mut additional = None;

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
                    additional = Some(max - min);

                    Some(min)
                } else if price > ask.price() {
                    Some(Ray::scale_to_ray(U256::from(ask.amount())))
                } else {
                    None
                }
            })
            .sum();

        PartialLiqRange { filled_quantity: filled, optional_additional_liq: additional }
    }

    fn fetch_exact_in_partial_bid(&self, price: Ray) -> PartialLiqRange {
        let mut additional = None;

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
                    additional = Some(max.div_ray(price) - min.div_ray(price));

                    Some(min.div_ray(price))
                } else if price < bid.price().inv_ray() {
                    Some(Ray::scale_to_ray(U256::from(bid.amount())).div_ray(price))
                } else {
                    None
                }
            })
            .sum();

        PartialLiqRange { filled_quantity: filled, optional_additional_liq: additional }
    }

    fn total_supply_at_price(&self, price: Ray) -> (Ray, Option<Ray>) {
        let partial = self.fetch_exact_in_partial_ask(price);
        (
            self.fetch_concentrated_liquidity(price)
                + self.fetch_exact_in_ask_orders(price)
                + self.fetch_exact_out_ask_orders(price)
                + partial.filled_quantity,
            partial.optional_additional_liq
        )
    }

    fn total_demand_at_price(&self, price: Ray) -> (Ray, Option<Ray>) {
        let partial = self.fetch_exact_in_partial_bid(price);
        (
            self.fetch_concentrated_liquidity(price)
                + self.fetch_exact_in_bid_orders(price)
                + self.fetch_exact_out_bid_orders(price)
                + partial.filled_quantity,
            partial.optional_additional_liq
        )
    }

    /// calculates given the supply, demand, optional supply and optional demand
    /// what way the algo's price should move if we want it too
    fn calculate_solver_move(&self, p_mid: Ray) -> Option<bool> {
        let (total_supply, _) = self.total_supply_at_price(p_mid);
        let (total_demand, _) = self.total_demand_at_price(p_mid);
        return cmp_total_supply_vs_demand(total_supply, total_demand);
    }

    pub fn solve_clearing_price(&self) -> Option<Ray> {
        let ep = Ray::from(U256::from(1));
        let mut p_max = Ray::from(self.book.highest_clearing_price().saturating_add(*ep));
        let mut p_min = Ray::from(self.book.lowest_clearing_price().saturating_sub(*ep));

        println!("min: {p_min:?} max: {p_max:?}");

        let two = U256::from(2);
        while (p_max - p_min) > ep {
            // grab all supply and demand
            let p_mid = Ray::from((p_max + p_min) / two);
            println!("solving clearing price iter price = {p_mid:?}");

            let res = self.calculate_solver_move(p_mid);

            if res == Some(true) {
                p_max = p_mid;
            } else if res == Some(false) {
                p_min = p_mid
            } else {
                println!("solved based on sup, demand");
                return Some(p_mid)
            }
            println!("min: {p_min:?} max: {p_max:?}");
        }

        None
        // (p_max + p_min) / two
    }
}

fn cmp_total_supply_vs_demand(total_supply: Ray, total_demand: Ray) -> Option<bool> {
    println!("sup: {:#?} demand: {:#?}", total_supply, total_demand);

    (total_supply > total_demand)
        .then_some(true)
        .or_else(|| (total_supply < total_demand).then_some(false))
}

#[derive(Debug, Default)]
struct PartialLiqRange {
    /// the quantity that filled up to the price.
    pub filled_quantity:         Ray,
    /// the additional liquidity we can partial fill if  p = pr
    pub optional_additional_liq: Option<Ray>
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
        amm::generate_single_position_amm_at_tick, orders::UserOrderBuilder
    };

    use crate::{
        book::{order::OrderContainer, BookOrder, OrderBook},
        matcher::binary_search::BinarySearchMatcher
    };

    #[test]
    fn ask_side_double_match_works_with_amm_binary_search() {
        let pool_id = PoolId::random();
        let bid_price = Ray::from(Uint::from(1_000_000_000_u128)).inv_ray_round(true);

        let low_price = Ray::from(Uint::from(1_000_u128));

        let bid_order = UserOrderBuilder::new()
            .partial()
            .amount(100)
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
    fn ask_outweighs_bid_sets_price() {
        let pool_id = PoolId::random();
        let high_price = Ray::from(Uint::from(1_000_000_000_u128)).inv_ray_round(true);
        let low_price = Ray::from(Uint::from(1_000_u128));
        let bid_order = UserOrderBuilder::new()
            .exact()
            .amount(10)
            .min_price(high_price)
            .with_storage()
            .bid()
            .build();
        let ask_order = UserOrderBuilder::new()
            .partial()
            .amount(10)
            .min_price(low_price)
            .with_storage()
            .ask()
            .build();
        let book = OrderBook::new(pool_id, None, vec![bid_order.clone()], vec![ask_order], None);
        let matcher = BinarySearchMatcher::new(&book);
        let ucp = matcher.solve_clearing_price().unwrap();
        assert!(
            ucp == low_price,
            "Ask outweighed but the final price wasn't properly set {ucp:?} {low_price:?}"
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
        let high_price = Ray::from(Uint::from(100u128));
        // both exact out so bid is 100 y

        // demand
        let bid_order = UserOrderBuilder::new()
            .exact()
            .amount(100)
            .exact_in(false)
            .min_price(high_price)
            .with_storage()
            .bid()
            .build();

        // supply
        let ask_order = UserOrderBuilder::new()
            .exact()
            .amount(1)
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
    fn basic_solve_of_exact_orders_exact_in() {
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
}
