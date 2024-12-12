use std::{
    cell::Cell,
    cmp::{max, Ordering}
};

use alloy::primitives::U256;
use angstrom_types::{
    matching::{
        uniswap::{Direction, PoolPrice, PoolPriceVec},
        CompositeOrder, Debt, Ray, SqrtPriceX96
    },
    orders::{NetAmmOrder, OrderFillState, OrderOutcome, PoolSolution},
    sol_bindings::{grouped_orders::OrderWithStorageData, rpc_orders::TopOfBlockOrder}
};

use super::Solution;
use crate::book::{order::OrderContainer, BookOrder, OrderBook};

#[derive(Debug)]
pub enum VolumeFillMatchEndReason {
    NoMoreBids,
    NoMoreAsks,
    BothSidesAMM,
    NoLongerCross,
    ZeroQuantity,
    /// This SHOULDN'T happen but I'm using it to clean up problem spots in the
    /// code
    ErrorEncountered
}

#[derive(Clone)]
pub struct VolumeFillMatcher<'a> {
    book:             &'a OrderBook,
    bid_idx:          Cell<usize>,
    pub bid_outcomes: Vec<OrderFillState>,
    ask_idx:          Cell<usize>,
    pub ask_outcomes: Vec<OrderFillState>,
    debt:             Option<Debt>,
    amm_price:        Option<PoolPrice<'a>>,
    amm_outcome:      Option<NetAmmOrder>,
    current_partial:  Option<BookOrder>,
    results:          Solution,
    // A checkpoint should never have a checkpoint stored within itself, otherwise this gets gnarly
    checkpoint:       Option<Box<Self>>
}

impl<'a> VolumeFillMatcher<'a> {
    pub fn new(book: &'a OrderBook) -> Self {
        let bid_outcomes = vec![OrderFillState::Unfilled; book.bids().len()];
        let ask_outcomes = vec![OrderFillState::Unfilled; book.asks().len()];
        let amm_price = book.amm().map(|a| a.current_price());
        let mut new_element = Self {
            book,
            bid_idx: Cell::new(0),
            bid_outcomes,
            ask_idx: Cell::new(0),
            ask_outcomes,
            debt: None,
            amm_price,
            amm_outcome: None,
            current_partial: None,
            results: Solution::default(),
            checkpoint: None
        };
        // We can checkpoint our initial state as valid
        new_element.save_checkpoint();
        new_element
    }

    pub fn results(&self) -> &Solution {
        &self.results
    }

    /// Save our current solve state to an internal checkpoint
    fn save_checkpoint(&mut self) {
        let checkpoint = Self {
            book:            self.book,
            bid_idx:         self.bid_idx.clone(),
            bid_outcomes:    self.bid_outcomes.clone(),
            ask_idx:         self.ask_idx.clone(),
            ask_outcomes:    self.ask_outcomes.clone(),
            debt:            self.debt,
            amm_price:       self.amm_price.clone(),
            amm_outcome:     self.amm_outcome.clone(),
            current_partial: self.current_partial.clone(),
            results:         self.results.clone(),
            checkpoint:      None
        };
        self.checkpoint = Some(Box::new(checkpoint));
    }

    /// Spawn a new VolumeFillBookSolver from our checkpoint
    pub fn from_checkpoint(&self) -> Option<Self> {
        self.checkpoint.as_ref().map(|cp| *cp.clone())
    }

    /// Restore our checkpoint into this VolumeFillBookSolver - not sure if we
    /// ever want to do this but we can!
    #[allow(dead_code)]
    fn restore_checkpoint(&mut self) -> bool {
        let Some(checkpoint) = self.checkpoint.take() else {
            return false;
        };
        let Self {
            bid_idx, bid_outcomes, ask_idx, ask_outcomes, amm_price, current_partial, ..
        } = *checkpoint;
        self.bid_idx = bid_idx;
        self.bid_outcomes = bid_outcomes;
        self.ask_idx = ask_idx;
        self.ask_outcomes = ask_outcomes;
        self.amm_price = amm_price;
        self.current_partial = current_partial;
        true
    }

    fn fill_amm(
        amm: &mut PoolPrice<'a>,
        results: &mut Solution,
        amm_outcome: &mut Option<NetAmmOrder>,
        quantity: u128,
        direction: Direction
    ) -> eyre::Result<()> {
        let new_amm = amm.d_t0(quantity, direction)?;
        let final_amm_order = PoolPriceVec::from_price_range(amm.clone(), new_amm.clone())?;
        *amm = new_amm.clone();
        // Add to our solution
        results.amm_volume += quantity;
        results.amm_final_price = Some(new_amm.price().clone());
        // Update our overall AMM volume
        let is_bid = matches!(direction, Direction::BuyingT0);
        let amm_out = amm_outcome.get_or_insert_with(|| NetAmmOrder::new(is_bid));
        amm_out.add_quantity(U256::from(final_amm_order.d_t0), U256::from(final_amm_order.d_t1));
        Ok(())
    }

    pub fn run_match(&mut self) -> VolumeFillMatchEndReason {
        // Run our match over and over until we get an end reason
        loop {
            if let Some(r) = self.single_match() {
                return r
            }
        }
    }

    pub fn single_match(&mut self) -> Option<VolumeFillMatchEndReason> {
        // Get the bid order
        let Some(bid) = Self::next_order(
            true,
            &self.bid_idx,
            &self.debt,
            self.amm_price.as_ref(),
            self.current_partial.as_ref(),
            self.book.bids(),
            &self.bid_outcomes
        ) else {
            return Some(VolumeFillMatchEndReason::NoMoreBids);
        };
        // Get the ask order
        let Some(ask) = Self::next_order(
            false,
            &self.ask_idx,
            &self.debt,
            self.amm_price.as_ref(),
            self.current_partial.as_ref(),
            self.book.asks(),
            &self.ask_outcomes
        ) else {
            return Some(VolumeFillMatchEndReason::NoMoreAsks)
        };

        // Check to see if we've hit an end state
        // If we're talking to the AMM on both sides, we're done
        if bid.is_amm() && ask.is_amm() {
            return Some(VolumeFillMatchEndReason::BothSidesAMM)
        }

        // If our prices no longer cross, we're done
        if ask.price() > bid.price() {
            return Some(VolumeFillMatchEndReason::NoLongerCross)
        }

        // Limit to price so that AMM orders will only offer the quantity they can
        // profitably sell.  (Non-AMM orders ignore the provided price)
        let ask_q = ask.quantity(bid.price());
        let bid_q = bid.quantity(ask.price());

        // Check to see if we have a 0-quantity ask and need to do an ask-side fill
        // This is only applicable if our ask order has the debt in it
        if ask_q == 0 && ask.is_debt() {
            let Some(next_ask) = Self::next_order(
                false,
                &self.ask_idx,
                // Deliberately no debt here, we want what the next available order would be
                // WITHOUT our debt
                &None,
                self.amm_price.as_ref(),
                None,
                self.book.asks(),
                &self.ask_outcomes
            ) else {
                return Some(VolumeFillMatchEndReason::NoMoreAsks);
            };

            // If we don't have a valid ask order to do an ask-side fill, we are done
            if next_ask.price() > bid.price() {
                return Some(VolumeFillMatchEndReason::NoLongerCross);
            }

            // Check to see if our next order is AMM.  If so we have to do some cool
            // bounding math where we reset the bound of our current order to be
            // the closer of the intersection point or the next order's bound.
            let normal_next_q = next_ask.quantity(bid.price());
            let next_ask_q = if next_ask.is_amm() {
                self.debt
                    .as_ref()
                    .and_then(|d| {
                        next_ask
                            .amm_intersect(*d)
                            .ok()
                            .map(|i| std::cmp::min(i, normal_next_q))
                    })
                    .unwrap_or(normal_next_q)
            } else {
                normal_next_q
            };
            // Get the quantity of the debt on the current composite bid
            let cur_ask_q = ask.negative_quantity(bid.price());

            if cur_ask_q == 0 {
                println!("No positive quantity, but no negative quantity?");
                return Some(VolumeFillMatchEndReason::ErrorEncountered);
            }

            let matched = next_ask_q.min(cur_ask_q);

            // Move the AMM if we have matched against an AMM order
            if ask.is_amm() || next_ask.is_amm() {
                if let Some(amm) = self.amm_price.as_mut() {
                    if Self::fill_amm(
                        amm,
                        &mut self.results,
                        &mut self.amm_outcome,
                        matched,
                        Direction::SellingT0
                    )
                    .is_err()
                    {
                        return Some(VolumeFillMatchEndReason::ErrorEncountered);
                    }
                }
            }

            match next_ask_q.cmp(&cur_ask_q) {
                Ordering::Equal => {
                    println!("Equal match");
                    // We annihilated
                    self.results.price = Some(next_ask.price());
                    // Mark as filled if non-AMM order
                    if !next_ask.is_amm() && !next_ask.is_composite() {
                        self.ask_outcomes[self.ask_idx.get()] = OrderFillState::CompleteFill
                    }
                    // Set the Debt's current price to the target price
                    self.debt = self.debt.map(|d| d.set_price(next_ask.price().into()));
                    // Clear the current partial because there is none
                    self.current_partial = None;
                    // Take a snapshot as a good solve state
                    self.save_checkpoint();
                }
                Ordering::Greater => {
                    println!("Greater match");
                    // Our next order is greater than our debt
                    // The end point is our next ask's price
                    self.results.price = Some(next_ask.price());
                    // Set the Debt's current price to the target price
                    self.debt = self.debt.map(|d| d.set_price(next_ask.price().into()));
                    // Create and save our partial bid
                    if !next_ask.is_amm() && !next_ask.is_composite() {
                        self.ask_outcomes[self.ask_idx.get()] =
                            self.ask_outcomes[self.ask_idx.get()].partial_fill(matched);
                        self.current_partial = Some(next_ask.fill(cur_ask_q));
                    } else {
                        self.current_partial = None;
                    }
                }
                Ordering::Less => {
                    println!("Less match");
                    // Our debt is greater than the order
                    // Find the end price of the debt and move it there
                    self.debt = self.debt.map(|d| d.partial_fill(matched));
                    // Mark as filled if non-AMM order
                    if !next_ask.is_amm() && !next_ask.is_composite() {
                        self.ask_outcomes[self.ask_idx.get()] = OrderFillState::CompleteFill
                    }
                    // This is a good solve state
                    self.save_checkpoint();
                }
            }
            // Start the matching process again
            return None;
        }

        // If either quantity is zero at this point we should break
        if ask_q == 0 || bid_q == 0 {
            return Some(VolumeFillMatchEndReason::ZeroQuantity)
        }

        let matched = ask_q.min(bid_q);
        // Store the amount we matched
        self.results.total_volume += matched;

        // Record partial fills
        if bid.is_partial() {
            self.results.partial_volume.0 += matched;
        }
        if ask.is_partial() {
            self.results.partial_volume.1 += matched;
        }

        // If bid or ask was an AMM order, we update our AMM stats
        if let Some(amm) = self.amm_price.as_mut() {
            let direction = match (bid.is_amm(), ask.is_amm()) {
                (true, false) => Some(Direction::BuyingT0),
                (false, true) => Some(Direction::SellingT0),
                (..) => None
            };
            if let Some(d) = direction {
                if Self::fill_amm(amm, &mut self.results, &mut self.amm_outcome, matched, d)
                    .is_err()
                {
                    return Some(VolumeFillMatchEndReason::ErrorEncountered);
                }
            }
        }

        // Then we see what else we need to do
        match bid_q.cmp(&ask_q) {
            Ordering::Equal => {
                // We annihilated
                self.results.price = Some((*(ask.price() + bid.price()) / U256::from(2)).into());
                // self.results.price = Some((ask.price() + bid.price()) / 2.0_f64);
                // Mark as filled if non-AMM order
                if !ask.is_amm() && !ask.is_composite() {
                    self.ask_outcomes[self.ask_idx.get()] = OrderFillState::CompleteFill
                }
                if !bid.is_amm() && !ask.is_composite() {
                    self.bid_outcomes[self.bid_idx.get()] = OrderFillState::CompleteFill
                }
                self.current_partial = None;
                // Take a snapshot as a good solve state
                self.save_checkpoint();
                // We're done here, we'll get our next bid and ask on
                // the next round
            }
            Ordering::Greater => {
                self.results.price = Some(bid.price());
                // Ask was completely filled, remainder bid
                if !ask.is_amm() && !ask.is_composite() {
                    self.ask_outcomes[self.ask_idx.get()] = OrderFillState::CompleteFill
                }
                // Create and save our partial bid
                if !bid.is_amm() && !bid.is_composite() {
                    self.bid_outcomes[self.bid_idx.get()] =
                        self.bid_outcomes[self.bid_idx.get()].partial_fill(matched);
                    self.current_partial = Some(bid.fill(ask_q));
                } else {
                    self.current_partial = None;
                }
            }
            Ordering::Less => {
                self.results.price = Some(ask.price());
                // Bid was completely filled, remainder ask
                if !bid.is_amm() && !bid.is_composite() {
                    self.bid_outcomes[self.bid_idx.get()] = OrderFillState::CompleteFill
                }
                // Create and save our parital ask
                if !ask.is_amm() && !ask.is_composite() {
                    self.ask_outcomes[self.ask_idx.get()] =
                        self.ask_outcomes[self.ask_idx.get()].partial_fill(matched);
                    self.current_partial = Some(ask.fill(bid_q));
                } else {
                    self.current_partial = None;
                }
            }
        }
        // We can checkpoint if we annihilated (No partial), if we completely filled an
        // order with an AMM order (No partial) or if we have an incomplete order but
        // it's a Partial Fill order which means this is a valid state to stop
        if let Some(ref fragment) = self.current_partial {
            if fragment.is_partial() {
                self.save_checkpoint();
            }
        }
        // Everything went well and we have no reason to stop
        None
    }

    pub fn fill(&mut self) -> VolumeFillMatchEndReason {
        {
            loop {
                let bid = match self.current_partial {
                    Some(ref o) if o.is_bid => OrderContainer::BookOrderFragment(o),
                    _ => {
                        if let Some(o) = Self::next_order_from_book(
                            true,
                            &self.bid_idx,
                            self.book.bids(),
                            &self.bid_outcomes,
                            self.amm_price.as_ref()
                        ) {
                            o
                        } else {
                            return VolumeFillMatchEndReason::NoMoreBids
                        }
                    }
                };
                let ask = match self.current_partial {
                    Some(ref o) if !o.is_bid => OrderContainer::BookOrderFragment(o),
                    _ => {
                        if let Some(o) = Self::next_order_from_book(
                            false,
                            &self.ask_idx,
                            self.book.asks(),
                            &self.ask_outcomes,
                            self.amm_price.as_ref()
                        ) {
                            o
                        } else {
                            return VolumeFillMatchEndReason::NoMoreBids
                        }
                    }
                };

                // If we're talking to the AMM on both sides, we're done
                if bid.is_amm() && ask.is_amm() {
                    return VolumeFillMatchEndReason::BothSidesAMM
                }

                // If our prices no longer cross, we're done
                if ask.price() > bid.price() {
                    return VolumeFillMatchEndReason::NoLongerCross
                }

                // Limit to price so that AMM orders will only offer the quantity they can
                // profitably sell.  (Non-AMM orders ignore the provided price)
                let ask_q = ask.quantity(bid.price());
                let bid_q = bid.quantity(ask.price());

                // If either quantity is zero maybe we should break here? (could be a
                // replacement for price cross checking if we implement that)
                if ask_q == 0 || bid_q == 0 {
                    return VolumeFillMatchEndReason::ZeroQuantity
                }

                let matched = ask_q.min(bid_q);
                // Store the amount we matched
                self.results.total_volume += matched;

                // Record partial fills
                if bid.is_partial() {
                    self.results.partial_volume.0 += matched;
                }
                if ask.is_partial() {
                    self.results.partial_volume.1 += matched;
                }

                // If bid or ask was an AMM order, we update our AMM stats
                if let (OrderContainer::AMM(o), _) | (_, OrderContainer::AMM(o)) = (&bid, &ask) {
                    // We always update our AMM price with any quantity sold
                    let final_amm_order = o.fill(matched);
                    self.amm_price = Some(final_amm_order.end_bound.clone());
                    // Add to our solution
                    self.results.amm_volume += matched;
                    self.results.amm_final_price = Some(*final_amm_order.end_bound.price());
                    // Update our overall AMM volume
                    let amm_out = self
                        .amm_outcome
                        .get_or_insert_with(|| NetAmmOrder::new(bid.is_amm()));
                    amm_out.add_quantity(
                        U256::from(final_amm_order.d_t0),
                        U256::from(final_amm_order.d_t1)
                    );
                }

                // Then we see what else we need to do
                match bid_q.cmp(&ask_q) {
                    Ordering::Equal => {
                        // We annihilated
                        self.results.price =
                            Some((*(ask.price() + bid.price()) / U256::from(2)).into());
                        // self.results.price = Some((ask.price() + bid.price()) / 2.0_f64);
                        // Mark as filled if non-AMM order
                        if !ask.is_amm() {
                            self.ask_outcomes[self.ask_idx.get()] = OrderFillState::CompleteFill
                        }
                        if !bid.is_amm() {
                            self.bid_outcomes[self.bid_idx.get()] = OrderFillState::CompleteFill
                        }
                        self.current_partial = None;
                        // Take a snapshot as a good solve state
                        self.save_checkpoint();
                        // We're done here, we'll get our next bid and ask on
                        // the next round
                    }
                    Ordering::Greater => {
                        self.results.price = Some(bid.price());
                        // Ask was completely filled, remainder bid
                        if !ask.is_amm() {
                            self.ask_outcomes[self.ask_idx.get()] = OrderFillState::CompleteFill
                        }
                        // Create and save our partial bid
                        if !bid.is_amm() {
                            self.bid_outcomes[self.bid_idx.get()] =
                                self.bid_outcomes[self.bid_idx.get()].partial_fill(matched);
                            self.current_partial = Some(bid.fill(ask_q));
                        } else {
                            self.current_partial = None;
                        }
                    }
                    Ordering::Less => {
                        self.results.price = Some(ask.price());
                        // Bid was completely filled, remainder ask
                        if !bid.is_amm() {
                            self.bid_outcomes[self.bid_idx.get()] = OrderFillState::CompleteFill
                        }
                        // Create and save our parital ask
                        if !ask.is_amm() {
                            self.ask_outcomes[self.ask_idx.get()] =
                                self.ask_outcomes[self.ask_idx.get()].partial_fill(matched);
                            self.current_partial = Some(ask.fill(bid_q));
                        } else {
                            self.current_partial = None;
                        }
                    }
                }
                // We can checkpoint if we annihilated (No partial), if we completely filled an
                // order with an AMM order (No partial) or if we have an incomplete order but
                // it's a Partial Fill order which means this is a valid state to stop
                if let Some(ref fragment) = self.current_partial {
                    if fragment.is_partial() {
                        self.save_checkpoint();
                    }
                }
            }
        }
    }

    fn next_order<'b>(
        bid: bool,
        book_idx: &Cell<usize>,
        debt: &Option<Debt>,
        amm: Option<&PoolPrice<'a>>,
        fragment: Option<&'b BookOrder>,
        book: &'a [BookOrder],
        fill_state: &[OrderFillState]
    ) -> Option<OrderContainer<'a, 'b>> {
        tracing::trace!("Getting next order for bid {} and debt {:?}", bid, debt);
        // If we have a fragment, that takes priority
        if let Some(f) = fragment {
            // If it's in the direction we're looking for, let's use it
            if bid == f.is_bid {
                return Some(OrderContainer::BookOrderFragment(f))
            }
        }
        // Fix what makes a price "less" or "more" advantageous depending on direction
        let (less_advantageous, more_advantageous) = if bid {
            // If it's a bid, a lower price is less advantageous and a higher price is more
            // advantageous
            (Ordering::Less, Ordering::Greater)
        } else {
            // If it's an ask, a higher price is less advantageous and a lower price is more
            // advantageous
            (Ordering::Greater, Ordering::Less)
        };
        let mut cur_idx = book_idx.get();
        while cur_idx < fill_state.len() {
            if let OrderFillState::Unfilled = fill_state[cur_idx] {
                break;
            }
            cur_idx += 1;
        }
        let book_order = book.get(cur_idx);

        // If we have some debt that is at a better price, then we're going to be making
        // a debt order
        if let Some(d) = debt {
            // Compare our debt to our book price, debt is more advantageous if there's no
            // book order
            let debt_book_cmp = book_order
                .map(|b| d.price().cmp(&b.price()))
                .unwrap_or(more_advantageous);
            // Compare our debt to our AMM, debt is more advantageous if there's no AMM
            let debt_amm_cmp = amm
                .map(|a| d.partial_cmp(a).unwrap())
                .unwrap_or(more_advantageous);

            match (debt_book_cmp, debt_amm_cmp) {
                // If the debt is less advantageous (Not sure how that could happen?) or equal to
                // the book, we should prioritize making a book order
                (dbc, _) if dbc == less_advantageous => (),
                (Ordering::Equal, _) => (),
                // Debt == AMM -> CompositeOrder(Debt, Amm) bound to the next book order
                (_, Ordering::Equal) => {
                    let bound_price = book_order.map(|b| b.price());
                    return Some(OrderContainer::Composite(CompositeOrder::new(
                        *debt,
                        amm.cloned(),
                        bound_price
                    )))
                }
                // Debt > AMM -> CompositeOrder(Debt), bound to the closer of the AMM or the next
                // book order
                (_, dac) if dac == more_advantageous => {
                    let bound_price = book_order
                        .map(|b| {
                            amm.map(|a| max(b.price(), a.as_ray()))
                                .unwrap_or_else(|| b.price())
                        })
                        .or_else(|| amm.map(|a| a.as_ray()));
                    return Some(OrderContainer::Composite(CompositeOrder::new(
                        *debt,
                        None,
                        bound_price
                    )))
                }
                _ => panic!("Debt should never be on the wrong side of the AMM")
            }
        }

        // If we have an AMM price, see if it takes precedence over our book order
        amm.and_then(|a| {
            let bound_price = book_order.map(|o| o.price());
            if let Some(bp) = bound_price {
                // If my book order is equal to or more advantageous to my AMM price, we have no
                // AMM order
                if bp.cmp(&a.as_ray()) != less_advantageous {
                    return None;
                }
            }
            // Otherwise, my AMM price is better than my book price and we should make an
            // AMM order
            Some(CompositeOrder::new(None, Some(a.clone()), bound_price))
        })
        .map(OrderContainer::Composite)
        .or_else(|| {
            book_idx.set(cur_idx);
            book_order.map(OrderContainer::BookOrder)
        })
    }

    fn next_order_from_book<'b>(
        is_bid: bool,
        index: &Cell<usize>,
        book: &'a [BookOrder],
        fill_state: &[OrderFillState],
        amm: Option<&PoolPrice<'a>>
    ) -> Option<OrderContainer<'a, 'b>> {
        let mut cur_idx = index.get();
        // Find the next unfilled order - we need to work with the index separately
        while cur_idx < fill_state.len() {
            match &fill_state[cur_idx] {
                OrderFillState::Unfilled => break,
                _ => cur_idx += 1
            }
        }
        let book_order = book.get(cur_idx);
        // See if our AMM takes precedence
        amm.and_then(|amm_price| {
            let target_price = book_order
                .map(|o| SqrtPriceX96::from(Ray::from(*OrderContainer::BookOrder(o).price())));
            // Will return None if the book order price is more beneficial than our AMM
            // price
            amm_price.order_to_target(target_price, !is_bid)
        })
        .map(OrderContainer::AMM)
        .or_else(|| {
            index.set(cur_idx);
            book_order.map(OrderContainer::BookOrder)
        })
    }

    pub fn solution(
        &self,
        searcher: Option<OrderWithStorageData<TopOfBlockOrder>>
    ) -> PoolSolution {
        let limit = self
            .bid_outcomes
            .iter()
            .enumerate()
            .map(|(idx, outcome)| (self.book.bids()[idx].order_id, outcome))
            .chain(
                self.ask_outcomes
                    .iter()
                    .enumerate()
                    .map(|(idx, outcome)| (self.book.asks()[idx].order_id, outcome))
            )
            .map(|(id, outcome)| OrderOutcome { id, outcome: outcome.clone() })
            .collect();
        let ucp: Ray = self.results.price.map(Into::into).unwrap_or_default();
        PoolSolution {
            id: self.book.id(),
            ucp,
            amm_quantity: self.amm_outcome.clone(),
            searcher,
            limit
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::Cell, cmp::max};

    use alloy::primitives::Uint;
    use alloy_primitives::FixedBytes;
    use angstrom_types::{
        matching::{uniswap::PoolSnapshot, Debt, DebtType, Ray, SqrtPriceX96},
        orders::OrderFillState,
        primitive::PoolId
    };
    use testing_tools::type_generator::{
        amm::generate_single_position_amm_at_tick, orders::UserOrderBuilder
    };

    use super::VolumeFillMatcher;
    use crate::book::{order::OrderContainer, BookOrder, OrderBook};

    #[test]
    fn runs_cleanly_on_empty_book() {
        let book = OrderBook::default();
        let matcher = VolumeFillMatcher::new(&book);
        let solution = matcher.solution(None);
        assert!(solution.ucp == Ray::ZERO, "Empty book didn't have UCP of zero");
    }

    // Let's write tests for all the basic matching outcomes to make sure they
    // work properly, then come up with some more complicated situations and
    // components to check

    #[test]
    fn bid_outweighs_ask_sets_price() {
        let pool_id = PoolId::random();
        let high_price = Ray::from(Uint::from(1_000_000_000_u128));
        let low_price = Ray::from(Uint::from(1_000_u128));
        let bid_order = UserOrderBuilder::new()
            .partial()
            .amount(100)
            .min_price(high_price)
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
        let mut matcher = VolumeFillMatcher::new(&book);
        let _fill_outcome = matcher.fill();
        let solution = matcher.from_checkpoint().unwrap().solution(None);
        assert!(
            solution.ucp == high_price,
            "Bid outweighed but the final price wasn't properly set"
        );
    }

    #[test]
    fn ask_outweighs_bid_sets_price() {
        let pool_id = PoolId::random();
        let high_price = Ray::from(Uint::from(1_000_000_000_u128));
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
            .amount(100)
            .min_price(low_price)
            .with_storage()
            .ask()
            .build();
        let book = OrderBook::new(pool_id, None, vec![bid_order.clone()], vec![ask_order], None);
        let mut matcher = VolumeFillMatcher::new(&book);
        let _fill_outcome = matcher.fill();
        let solution = matcher.from_checkpoint().unwrap().solution(None);
        assert!(
            solution.ucp == low_price,
            "Ask outweighed but the final price wasn't properly set"
        );
    }

    fn basic_order_book(
        is_bid: bool,
        count: usize,
        target_price: Ray,
        price_step: usize
    ) -> (Vec<BookOrder>, Vec<OrderFillState>) {
        let orders = (0..count)
            .map(|i| {
                // Step downwards if it's a bid to simulate bid book ordering
                let min_price = if is_bid {
                    target_price - (i * price_step)
                } else {
                    target_price + (i * price_step)
                };
                UserOrderBuilder::new()
                    .min_price(min_price)
                    .amount(100)
                    .with_storage()
                    .is_bid(is_bid)
                    .build()
            })
            .collect();
        let states = (0..count).map(|_| OrderFillState::Unfilled).collect();
        (orders, states)
    }

    #[test]
    fn gets_next_bid_order() {
        let index = Cell::new(0);
        let (book, fill_state) = basic_order_book(true, 10, Ray::from(10000_usize), 10);
        let debt = None;
        let amm = None;
        let next_order =
            VolumeFillMatcher::next_order(true, &index, &debt, amm, None, &book, &fill_state)
                .unwrap();
        if let OrderContainer::BookOrder(o) = next_order {
            assert_eq!(*o, book[0], "Next order selected was not first order in book");
        } else {
            panic!("Next order is not a BookOrder");
        }
    }

    #[test]
    fn bid_side_amm_overrides_book_order() {
        let market: PoolSnapshot =
            generate_single_position_amm_at_tick(100000, 100, 1_000_000_000_000_000_u128);
        let amm_price = market.current_price();
        let amm = Some(&amm_price);
        let debt = None;
        let index = Cell::new(0);
        let (book, fill_state) =
            basic_order_book(true, 10, Ray::from(SqrtPriceX96::at_tick(99999).unwrap()), 10);

        let next_order =
            VolumeFillMatcher::next_order(true, &index, &debt, amm, None, &book, &fill_state)
                .unwrap();

        assert!(matches!(next_order, OrderContainer::Composite(_)), "Composite order not created!");
        if let OrderContainer::Composite(c) = next_order {
            println!("Order: {:?}", c);
            assert_eq!(c.start_price(), amm_price.as_ray(), "AMM price is not starting price");
            assert!(c.quantity(book[0].price()) > 0, "Composite order has zero quantity");
        } else {
            panic!("Composite order not created but did match?");
        }
    }

    #[test]
    fn bid_side_debt_overrides_amm_and_book() {
        let market: PoolSnapshot =
            generate_single_position_amm_at_tick(100000, 100, 1_000_000_000_000_000_u128);
        let amm_price = market.current_price();
        let amm = Some(&amm_price);
        let debt = Some(Debt::new(
            DebtType::ExactIn(100000000),
            Ray::from(SqrtPriceX96::at_tick(101001).unwrap())
        ));
        let index = Cell::new(0);
        let (book, fill_state) =
            basic_order_book(true, 10, Ray::from(SqrtPriceX96::at_tick(99999).unwrap()), 10);

        let next_order =
            VolumeFillMatcher::next_order(true, &index, &debt, amm, None, &book, &fill_state)
                .unwrap();
        let order_q_target = max(book[0].price(), amm_price.as_ray());

        assert!(matches!(next_order, OrderContainer::Composite(_)), "Composite order not created!");
        if let OrderContainer::Composite(c) = next_order {
            assert!(c.debt().is_some(), "No debt in created Composite");
            assert!(c.amm().is_none(), "AMM erroneously included in created Composite");
            assert!(c.bound().is_some(), "No bound price included");
            assert!(c.quantity(order_q_target) > 0, "Composite order has zero quantity");
            assert_eq!(c.bound().unwrap(), amm_price.as_ray(), "Bound is not AMM price");
        } else {
            panic!("Composite order not created but did match?");
        }
    }

    #[test]
    fn bid_side_book_overrides_amm_and_debt() {
        let market: PoolSnapshot =
            generate_single_position_amm_at_tick(100000, 100, 1_000_000_000_000_000_u128);
        let amm_price = market.current_price();
        let amm = Some(&amm_price);
        let debt = Some(Debt::new(
            DebtType::ExactIn(100000000),
            Ray::from(SqrtPriceX96::at_tick(10001).unwrap())
        ));
        let index = Cell::new(0);
        let (book, fill_state) =
            basic_order_book(true, 10, Ray::from(SqrtPriceX96::at_tick(100100).unwrap()), 10);

        let next_order =
            VolumeFillMatcher::next_order(true, &index, &debt, amm, None, &book, &fill_state)
                .unwrap();

        assert!(matches!(next_order, OrderContainer::BookOrder(_)), "Book order not chosen");
        if let OrderContainer::BookOrder(b) = next_order {
            assert_eq!(*b, book[0], "First book order not chosen");
        } else {
            panic!("Book order not created but did match?");
        }
    }

    #[test]
    fn bid_side_debt_overrides_amm_and_book_with_book_bound() {
        let market: PoolSnapshot =
            generate_single_position_amm_at_tick(99999, 100, 1_000_000_000_000_000_u128);
        let amm_price = market.current_price();
        let amm = Some(&amm_price);
        let debt = Some(Debt::new(
            DebtType::ExactIn(100000000),
            Ray::from(SqrtPriceX96::at_tick(101001).unwrap())
        ));
        let index = Cell::new(0);
        let (book, fill_state) =
            basic_order_book(true, 10, Ray::from(SqrtPriceX96::at_tick(100000).unwrap()), 10);

        let next_order =
            VolumeFillMatcher::next_order(true, &index, &debt, amm, None, &book, &fill_state)
                .unwrap();

        let order_q_target = max(book[0].price(), amm_price.as_ray());

        assert!(matches!(next_order, OrderContainer::Composite(_)), "Composite order not created!");
        if let OrderContainer::Composite(c) = next_order {
            assert!(c.debt().is_some(), "No debt in created Composite");
            assert!(c.amm().is_none(), "AMM erroneously included in created Composite");
            assert!(c.bound().is_some(), "No bound price included");
            assert!(c.quantity(order_q_target) > 0, "Composite order has zero quantity");
            assert_eq!(c.bound().unwrap(), amm_price.as_ray(), "Bound is not AMM price");
        } else {
            panic!("Composite order not created but did match?");
        }
    }

    #[test]
    fn ask_side_debt_has_zero_quantity() {
        let debt = Some(Debt::new(
            DebtType::ExactIn(100000000),
            Ray::from(SqrtPriceX96::at_tick(100000).unwrap())
        ));
        let index = Cell::new(0);
        let (book, fill_state) =
            basic_order_book(true, 10, Ray::from(SqrtPriceX96::at_tick(101000).unwrap()), 10);

        let next_order =
            VolumeFillMatcher::next_order(false, &index, &debt, None, None, &book, &fill_state)
                .unwrap();

        assert!(matches!(next_order, OrderContainer::Composite(_)), "Composite order not created!");
        if let OrderContainer::Composite(c) = next_order {
            let q = c.quantity(book[0].price());
            assert_eq!(q, 0, "Ask-side debt doesn't have a zero quantity!");
        } else {
            panic!("Composite order not created but did match?");
        }
    }

    #[test]
    fn ask_side_double_match_works() {
        let debt_price = Ray::from(SqrtPriceX96::at_tick(90000).unwrap());
        let ask_target_price = Ray::from(SqrtPriceX96::at_tick(100000).unwrap());
        let bid_target_price = Ray::from(SqrtPriceX96::at_tick(110000).unwrap());
        let debt = Some(Debt::new(DebtType::ExactIn(100000), debt_price));
        if let Some(ref d) = debt {
            assert!(!d.valid_for_price(ask_target_price), "Debt already at ask price");
        }
        let (ask_book, _) = basic_order_book(false, 10, ask_target_price, 10);
        let (bid_book, _) = basic_order_book(true, 10, bid_target_price, 10);

        let ob = OrderBook::new(
            FixedBytes::random(),
            None,
            bid_book,
            ask_book,
            Some(crate::book::sort::SortStrategy::ByPriceByVolume)
        );
        let mut matcher = VolumeFillMatcher::new(&ob);
        matcher.debt = debt;
        let first_ask = matcher.book.asks().get(matcher.ask_idx.get()).unwrap();
        assert!(
            !debt.as_ref().unwrap().valid_for_price(first_ask.price()),
            "Debt starting at first ask price"
        );
        let end = matcher.single_match();
        println!("Fill ended: {:?}", end);
        assert!(matcher.current_partial.is_some(), "Partial order is not left over");
        let po = matcher.current_partial.as_ref().unwrap();
        assert!(!po.is_bid, "Bid left in our partials");
        assert!(po.quantity() == 92, "Wrong amount of volume taken from our order");
        assert!(matcher.debt.is_some(), "No debt left on the matcher");
        let md = matcher.debt.as_ref().unwrap();
        assert!(md.valid_for_price(po.price()), "Debt is not at the current order price");

        matcher.single_match();
        assert!(matcher.current_partial.is_some(), "Partial order is not left over");
        let po = matcher.current_partial.as_ref().unwrap();
        assert!(po.is_bid, "Ask left in our partials");
        assert!(po.quantity() == 8, "Wrong amount of volume taken from our order");
    }

    #[test]
    fn ask_side_double_match_works_with_amm() {
        let market: PoolSnapshot =
            generate_single_position_amm_at_tick(91000, 100, 1_000_000_000_000_000_u128);
        let debt_price = Ray::from(SqrtPriceX96::at_tick(90000).unwrap());
        let ask_target_price = Ray::from(SqrtPriceX96::at_tick(100000).unwrap());
        let bid_target_price = Ray::from(SqrtPriceX96::at_tick(110000).unwrap());
        let debt = Some(Debt::new(DebtType::ExactIn(100000), debt_price));
        if let Some(ref d) = debt {
            assert!(!d.valid_for_price(ask_target_price), "Debt already at ask price");
        }
        let (ask_book, _) = basic_order_book(false, 10, ask_target_price, 10);
        let (bid_book, _) = basic_order_book(true, 10, bid_target_price, 10);

        let ob = OrderBook::new(
            FixedBytes::random(),
            Some(market),
            bid_book,
            ask_book,
            Some(crate::book::sort::SortStrategy::ByPriceByVolume)
        );
        let mut matcher = VolumeFillMatcher::new(&ob);
        matcher.debt = debt;
        let first_ask = matcher.book.asks().get(matcher.ask_idx.get()).unwrap();
        assert!(
            !debt.as_ref().unwrap().valid_for_price(first_ask.price()),
            "Debt starting at first ask price"
        );
        let end = matcher.single_match();
        println!("Fill ended: {:?}", end);
        assert!(matcher.current_partial.is_some(), "Partial order is not left over");
        let po = matcher.current_partial.as_ref().unwrap();
        assert!(!po.is_bid, "Bid left in our partials");
        assert!(po.quantity() == 92, "Wrong amount of volume taken from our order");
        assert!(matcher.debt.is_some(), "No debt left on the matcher");
        let md = matcher.debt.as_ref().unwrap();
        assert!(md.valid_for_price(po.price()), "Debt is not at the current order price");

        matcher.single_match();
        assert!(matcher.current_partial.is_some(), "Partial order is not left over");
        let po = matcher.current_partial.as_ref().unwrap();
        assert!(po.is_bid, "Ask left in our partials");
        assert!(po.quantity() == 8, "Wrong amount of volume taken from our order");
    }
}
