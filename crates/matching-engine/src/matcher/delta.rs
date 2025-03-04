use std::collections::hash_map::HashMap;

use alloy_primitives::{Address, I256, U256};
use angstrom_types::{
    contract_payloads::tob::generate_current_price_adjusted_for_donation,
    matching::{
        SqrtPriceX96,
        uniswap::{Direction, PoolPrice, PoolPriceVec}
    },
    orders::{NetAmmOrder, OrderFillState, OrderId, OrderOutcome, PoolSolution},
    sol_bindings::{
        RawPoolOrder, Ray,
        grouped_orders::{GroupedVanillaOrder, OrderWithStorageData},
        rpc_orders::TopOfBlockOrder
    }
};
use rand_distr::num_traits::ToPrimitive;

use crate::OrderBook;

#[derive(Clone)]
pub struct DeltaMatcher<'a> {
    book:            &'a OrderBook,
    /// changes if there is a tob or not
    amm_start_price: Option<PoolPrice<'a>>
}

impl<'a> DeltaMatcher<'a> {
    pub fn new(book: &'a OrderBook, tob: Option<OrderWithStorageData<TopOfBlockOrder>>) -> Self {
        let amm_start_price = if let Some(tob) = tob {
            if let Some(a) = book.amm() {
                let end = generate_current_price_adjusted_for_donation(&tob, a)
                    .expect("order structure should be valid here and never fail");

                Some(end)
            } else {
                None
            }
        } else {
            book.amm().map(|f| f.current_price())
        };

        Self { book, amm_start_price }
    }

    fn fetch_concentrated_liquidity(&self, price: Ray) -> (I256, I256) {
        let Some(start_price) = self.amm_start_price.clone() else { return Default::default() };
        let start_sqrt = start_price.as_sqrtpricex96();
        let end_sqrt = SqrtPriceX96::from(price);

        let zfo = start_sqrt >= end_sqrt;

        let direction = Direction::from_is_bid(!zfo);

        let Ok(res) = PoolPriceVec::swap_to_price(start_price.clone(), end_sqrt, direction) else {
            return Default::default();
        };

        if zfo {
            // if the amm is swapping from zero to one, it means that we need more liquidity
            // it in token 1 and less in token zero
            (I256::try_from(res.d_t0).unwrap() * I256::MINUS_ONE, I256::try_from(res.d_t1).unwrap())
        } else {
            // if we are one for zero, means we are adding liquidity in t0 and removing in
            // t1
            (I256::try_from(res.d_t0).unwrap(), I256::try_from(res.d_t1).unwrap() * I256::MINUS_ONE)
        }
    }

    fn fetch_amount_out_amount_in_non_partials(&self, price: Ray) -> (I256, I256) {
        let mut t0_delta = I256::ZERO;
        let mut t1_delta = I256::ZERO;
        self.book
            .asks()
            .iter()
            .filter(|ask| price >= ask.price() && !ask.is_partial())
            .for_each(|ask| {
                let (q_in, q_out) = Self::get_amount_in_out(ask, ask.amount(), price);
                // given its a ask, q_in is always t0
                t0_delta += I256::try_from(q_in).unwrap();
                t1_delta -= I256::try_from(q_out).unwrap();
            });

        self.book
            .bids()
            .iter()
            .filter(|bid| price <= bid.price().inv_ray_round(true) && !bid.is_partial())
            .for_each(|bid| {
                let (q_in, q_out) = Self::get_amount_in_out(bid, bid.amount(), price);
                // given its a bid, q_in is always t1
                t0_delta -= I256::try_from(q_out).unwrap();
                t1_delta += I256::try_from(q_in).unwrap();
            });

        (t0_delta, t1_delta)
    }

    /// returns t0 delta, t1 delta, optional t0 extra fill and optional t1 extra
    /// fill.
    /// this favours partially filling
    fn fetch_amount_in_amount_out_partials(
        &self,
        price: Ray
    ) -> (I256, I256, Option<bool>, Option<u128>, Option<u128>, Option<OrderId>) {
        let mut t0_delta = I256::ZERO;
        let mut t1_delta = I256::ZERO;

        let mut extra_t0 = None;
        let mut extra_t1 = None;
        let mut side = None;
        let mut id = None;

        self.book
            .asks()
            .iter()
            .filter(|ask| price >= ask.price() && ask.is_partial())
            .for_each(|ask| {
                if price == ask.price() {
                    // min prices
                    let (q_in_min, q_out_min) =
                        Self::get_amount_in_out(ask, ask.min_amount(), price);
                    t0_delta += I256::try_from(q_in_min).unwrap();
                    t1_delta -= I256::try_from(q_out_min).unwrap();

                    // max amount
                    let (q_in_max, q_out_max) = Self::get_amount_in_out(ask, ask.amount(), price);
                    // if we are asks then
                    extra_t0 = Some(q_in_max - q_in_min);
                    extra_t1 = Some(q_out_max - q_out_min);
                    side = Some(true);
                    id = Some(ask.order_id);

                    return;
                }

                // max amount
                let (q_in, q_out) = Self::get_amount_in_out(ask, ask.amount(), price);
                // given its a ask, q_in is always t0
                t0_delta += I256::try_from(q_in).unwrap();
                t1_delta -= I256::try_from(q_out).unwrap();
            });

        self.book
            .bids()
            .iter()
            .filter(|bid| price <= bid.price().inv_ray_round(true) && bid.is_partial())
            .for_each(|bid| {
                // favour filling asks for now. will come back and fix later
                if price == bid.price().inv_ray_round(true) && extra_t0.is_none() {
                    let (q_in_min, q_out_min) =
                        Self::get_amount_in_out(bid, bid.min_amount(), price);

                    t0_delta -= I256::try_from(q_out_min).unwrap();
                    t1_delta += I256::try_from(q_in_min).unwrap();

                    let (q_in_max, q_out_max) = Self::get_amount_in_out(bid, bid.amount(), price);
                    extra_t1 = Some(q_in_max - q_in_min);
                    extra_t0 = Some(q_out_max - q_out_min);
                    side = Some(false);

                    id = Some(bid.order_id);
                    return;
                }

                let (q_in, q_out) = Self::get_amount_in_out(bid, bid.amount(), price);
                // given its a bid, q_in is always t1
                t0_delta -= I256::try_from(q_out).unwrap();
                t1_delta += I256::try_from(q_in).unwrap();
            });

        (t0_delta, t1_delta, side, extra_t0, extra_t1, id)
    }

    fn check_ucp(&self, price: Ray) -> SupplyDemandResult {
        let (book_t0, book_t1) = self.fetch_concentrated_liquidity(price);
        let (normal_t0, normal_t1) = self.fetch_amount_out_amount_in_non_partials(price);
        let (partial_t0, partial_t1, extra_is_ask, extra_t0, extra_t1, id) =
            self.fetch_amount_in_amount_out_partials(price);

        let t0_sum = book_t0 + normal_t0 + partial_t0;
        let t1_sum = book_t1 + normal_t1 + partial_t1;

        if t0_sum.is_zero() && t1_sum.is_zero() {
            return SupplyDemandResult::NaturallyEqual;
        }

        let (Some(is_ask), Some(extra_t0), Some(_extra_t1), Some(id)) =
            (extra_is_ask, extra_t0, extra_t1, id)
        else {
            tracing::info!(?t0_sum, ?t1_sum, ?price, "no extra");
            return if t0_sum < I256::ZERO {
                SupplyDemandResult::MoreDemand
            } else {
                SupplyDemandResult::MoreSupply
            };
        };

        // means we have extra demand we can add.
        if t0_sum > I256::ZERO && !is_ask {
            tracing::info!("is bid is partial and t0_sum > 0");
            // if we are a bid, then we are adding t1 and subtracing t0.
            // if we are able to flip the exquality, this means we can possibly solve here
            let delta = t0_sum - I256::try_from(extra_t0).unwrap();
            // delta neg so we flipped
            if delta <= I256::ZERO {
                return SupplyDemandResult::PartialFillEq { extra_fill_t0: t0_sum, id };
            }

        // means we have extra supply we can add
        // we are getting errors here.
        } else if t0_sum < I256::ZERO && is_ask {
            tracing::info!("is ask is partial and t0_sum < 0");
            // if we are a ask, then we are adding t0 and subtracing t1
            let delta = t0_sum + I256::try_from(extra_t0).unwrap();

            if delta >= I256::ZERO {
                return SupplyDemandResult::PartialFillEq {
                    extra_fill_t0: t0_sum.saturating_neg(),
                    id
                };
            }
        }

        tracing::info!(?t0_sum, ?t1_sum, ?price);
        if t0_sum < I256::ZERO {
            SupplyDemandResult::MoreDemand
        } else {
            SupplyDemandResult::MoreSupply
        }
    }

    /// calculates given the supply, demand, optional supply and optional demand
    /// what way the algo's price should move if we want it too
    fn get_amount_in_out(
        order: &OrderWithStorageData<GroupedVanillaOrder>,
        fill_amount: u128,
        ray_ucp: Ray
    ) -> (u128, u128) {
        match (order.is_bid(), order.exact_in()) {
            // fill_amount is the exact amount of T1 being input to get a T0 output
            (true, true) => (
                // am in
                fill_amount,
                // am out - round down because we'll always try to give you less
                ray_ucp.inverse_quantity(fill_amount, false) - order.priority_data.gas.to::<u128>()
            ),
            // fill amount is the exact amount of T0 being output for a T1 input
            (true, false) => {
                // Round up because we'll always ask you to pay more
                (ray_ucp.quantity(fill_amount, true), fill_amount)
            }
            // fill amount is the exact amount of T0 being input for a T1 output
            (false, true) => {
                // Round down because we'll always try to give you less
                (
                    fill_amount,
                    ray_ucp.quantity(fill_amount - order.priority_data.gas.to::<u128>(), false)
                )
            }
            // fill amount is the exact amount of T1 expected out for a given T0 input
            (false, false) => (
                // Round up because we'll always ask you to pay more
                ray_ucp.inverse_quantity(fill_amount, true),
                fill_amount
            )
        }
    }

    /// helper functions for grabbing all orders that we filled at ucp
    fn fetch_orders_at_ucp(&self, fetch: &UcpSolution) -> Vec<OrderOutcome> {
        // we can only have partial fills when ucp is exactly on.
        let (order_id, amount) = fetch.get_partial_unfill().unzip();
        let mut map: HashMap<Address, i128> = HashMap::new();
        let mut map2: HashMap<Address, i128> = HashMap::new();

        let res = self
            .book
            .bids()
            .iter()
            .map(|bid| OrderOutcome {
                id:      bid.order_id,
                outcome: if order_id == Some(bid.order_id) {
                    // partials are always exact in, so we need to convert this out
                    println!(
                        "{} - {} is bid: true",
                        bid.amount(),
                        (U256::try_from(amount.unwrap()).unwrap()).to::<u128>()
                    );
                    // the amount here is always in Y. however for bids that are exact in, we want
                    // X
                    let partial_am = fetch
                        .ucp
                        .mul_quantity(U256::try_from(amount.unwrap()).unwrap())
                        .to::<u128>();

                    let amount_in_partial = bid.min_amount() + partial_am;
                    let (amount_in, amount_out) =
                        Self::get_amount_in_out(bid, amount_in_partial, fetch.ucp);
                    *map.entry(bid.token_in()).or_default() += amount_in.to_i128().unwrap();
                    *map.entry(bid.token_out()).or_default() -= amount_out.to_i128().unwrap();

                    OrderFillState::PartialFill(amount_in_partial)
                } else if fetch.ucp <= bid.price().inv_ray_round(true) {
                    let (amount_in, amount_out) =
                        Self::get_amount_in_out(bid, bid.amount(), fetch.ucp);

                    *map.entry(bid.token_in()).or_default() += amount_in.to_i128().unwrap();
                    *map.entry(bid.token_out()).or_default() -= amount_out.to_i128().unwrap();
                    OrderFillState::CompleteFill
                } else {
                    OrderFillState::Unfilled
                }
            })
            .chain(self.book.asks().iter().map(|ask| OrderOutcome {
                id:      ask.order_id,
                outcome: if order_id == Some(ask.order_id) {
                    let amount_parital =
                        ask.min_amount() + U256::try_from(amount.unwrap()).unwrap().to::<u128>();

                    let (amount_in, amount_out) =
                        Self::get_amount_in_out(ask, amount_parital, fetch.ucp);

                    *map2.entry(ask.token_in()).or_default() += amount_in.to_i128().unwrap();
                    *map2.entry(ask.token_out()).or_default() -= amount_out.to_i128().unwrap();
                    OrderFillState::PartialFill(amount_parital)
                } else if fetch.ucp >= ask.price() {
                    let (amount_in, amount_out) =
                        Self::get_amount_in_out(ask, ask.amount(), fetch.ucp);
                    *map2.entry(ask.token_in()).or_default() += amount_in.to_i128().unwrap();
                    *map2.entry(ask.token_out()).or_default() -= amount_out.to_i128().unwrap();
                    OrderFillState::CompleteFill
                } else {
                    OrderFillState::Unfilled
                }
            }))
            .collect::<Vec<_>>();

        let (zero, one) = self
            .book
            .asks()
            .first()
            .map(|a| (a.token_in(), a.token_out()))
            .unwrap();

        if let Some(amm) = self.fetch_amm_movement_at_ucp(fetch.ucp) {
            match amm {
                // ask
                NetAmmOrder::Buy(t0, t1) => {
                    *map.entry(zero).or_default() -= t0.to_i128().unwrap();
                    *map.entry(one).or_default() += t1.to_i128().unwrap();
                }
                // bid
                NetAmmOrder::Sell(t0, t1) => {
                    *map.entry(zero).or_default() += t0.to_i128().unwrap();
                    *map.entry(one).or_default() -= t1.to_i128().unwrap();
                }
            }
        }

        for (k, v) in map2 {
            *map.entry(k).or_default() += v;
        }
        tracing::info!("order outcome\n\n\n {:#?}", map);

        res
    }

    fn fetch_amm_movement_at_ucp(&self, ucp: Ray) -> Option<NetAmmOrder> {
        let start_price = self.amm_start_price.clone()?;

        let start_sqrt = start_price.as_sqrtpricex96();
        let end_sqrt = SqrtPriceX96::from(ucp);
        let zfo = start_sqrt >= end_sqrt;
        let direction = Direction::from_is_bid(!zfo);

        let Ok(res) = PoolPriceVec::swap_to_price(start_price.clone(), end_sqrt, direction) else {
            return None;
        };

        let mut tob_amm = NetAmmOrder::new(direction);
        tob_amm.add_quantity(res.d_t0, res.d_t1);

        Some(tob_amm)
    }

    // short on asks.
    pub fn solution(
        &mut self,
        searcher: Option<OrderWithStorageData<TopOfBlockOrder>>
    ) -> PoolSolution {
        let Some(price_and_partial_solution) = self.solve_clearing_price() else {
            tracing::info!("no solve");
            return PoolSolution {
                id: self.book.id(),
                searcher,
                limit: self
                    .book
                    .bids()
                    .iter()
                    .map(|o| OrderOutcome {
                        id:      o.order_id,
                        outcome: OrderFillState::Unfilled
                    })
                    .chain(self.book.asks().iter().map(|o| OrderOutcome {
                        id:      o.order_id,
                        outcome: OrderFillState::Unfilled
                    }))
                    .collect(),
                ..Default::default()
            };
        };

        let limit = self.fetch_orders_at_ucp(&price_and_partial_solution);

        let amm = self.fetch_amm_movement_at_ucp(price_and_partial_solution.ucp);

        PoolSolution {
            id: self.book.id(),
            ucp: price_and_partial_solution.ucp,
            amm_quantity: amm,
            limit,
            searcher
        }
    }

    fn solve_clearing_price(&self) -> Option<UcpSolution> {
        let ep = Ray::from(U256::from(1));
        let mut p_max = Ray::from(self.book.highest_clearing_price().saturating_add(*ep));
        let mut p_min = Ray::from(self.book.lowest_clearing_price().saturating_sub(*ep));
        println!("min: {p_min:?} max: {p_max:?}");

        let two = U256::from(2);
        while (p_max - p_min) > ep {
            // grab all supply and demand
            let p_mid = (p_max + p_min) / two;

            // the delta of t0
            let res = self.check_ucp(p_mid);

            match res {
                SupplyDemandResult::MoreSupply => {
                    p_max = p_mid;
                }
                SupplyDemandResult::MoreDemand => p_min = p_mid,
                SupplyDemandResult::NaturallyEqual => {
                    println!("solved based on sup, demand no partials");

                    return Some(UcpSolution {
                        ucp:           p_mid,
                        extra_t0_fill: None,
                        partial_id:    None
                    });
                }
                SupplyDemandResult::PartialFillEq { extra_fill_t0, id } => {
                    println!("solved based on sup, demand with partial order");
                    return Some(UcpSolution {
                        ucp:           p_mid,
                        extra_t0_fill: Some(extra_fill_t0),
                        partial_id:    Some(id)
                    });
                }
            }
        }

        None
    }
}

#[derive(Debug)]
struct UcpSolution {
    ucp:           Ray,
    // true means supply
    extra_t0_fill: Option<I256>,
    partial_id:    Option<OrderId>
}

impl UcpSolution {
    pub fn get_partial_unfill(&self) -> Option<(OrderId, I256)> {
        Some((self.partial_id?, self.extra_t0_fill?))
    }
}

#[derive(Debug)]
pub enum SupplyDemandResult {
    MoreSupply,
    MoreDemand,
    NaturallyEqual,
    PartialFillEq { extra_fill_t0: I256, id: OrderId }
}
