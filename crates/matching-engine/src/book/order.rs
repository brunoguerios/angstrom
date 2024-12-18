use angstrom_types::{
    matching::{
        max_t1_for_t0,
        uniswap::{Direction, PoolPriceVec},
        CompositeOrder, Debt, DebtType
    },
    orders::{OrderFillState, OrderId, OrderPrice, OrderVolume},
    sol_bindings::{
        grouped_orders::{
            FlashVariants, GroupedVanillaOrder, OrderWithStorageData, StandingVariants
        },
        RawPoolOrder
    }
};

use super::BookOrder;

/// Definition of the various types of order that we can serve, as well as the
/// outcomes we're able to have for them
#[derive(Clone, Debug)]
pub enum OrderContainer<'a> {
    /// A complete order from our book
    BookOrder(&'a BookOrder),
    /// A fragment of an order from our book yet to be filled
    BookOrderFragment { order: &'a BookOrder, state: OrderFillState },
    /// An order constructed from the current state of our AMM
    AMM(PoolPriceVec<'a>),
    /// A CompositeOrder built of Debt or AMM or Both
    Composite(CompositeOrder<'a>)
}

impl<'a> OrderContainer<'a> {
    pub fn id(&self) -> Option<OrderId> {
        match self {
            Self::BookOrder(o) => Some(o.order_id),
            Self::BookOrderFragment { order, .. } => Some(order.order_id),
            _ => None
        }
    }

    pub fn is_composite(&self) -> bool {
        matches!(self, Self::Composite(_))
    }

    /// Is `true` when the order in the container includes the AMM, either as a
    /// distinct AMM order or as a Composite order that includes the AMM
    pub fn is_amm(&self) -> bool {
        if let Self::Composite(o) = self {
            o.has_amm()
        } else {
            matches!(self, Self::AMM(_))
        }
    }

    /// Is `true` when the order in the container includes debt, this can only
    /// be true of a Composite order
    pub fn is_debt(&self) -> bool {
        if let Self::Composite(o) = self {
            o.has_debt()
        } else {
            false
        }
    }

    /// Returns a Debt item covering the partially matched order at its current
    /// price
    pub fn partial_debt(&self, matched_t0: u128) -> Option<Debt> {
        if self.inverse_order() {
            if let Self::BookOrder(o) | Self::BookOrderFragment { order: o, .. } = self {
                let magnitude = if o.is_bid {
                    DebtType::exact_in(max_t1_for_t0(matched_t0, Direction::BuyingT0, o.price()))
                } else {
                    DebtType::exact_out(max_t1_for_t0(matched_t0, Direction::SellingT0, o.price()))
                };
                return Some(Debt::new(magnitude, o.price()));
            }
        }
        None
    }

    /// Represents an applicable order as a debt at its current price
    pub fn as_debt(&self) -> Option<Debt> {
        if self.inverse_order() {
            if let Self::BookOrder(o) | Self::BookOrderFragment { order: o, .. } = self {
                let magnitude = if o.is_bid {
                    DebtType::exact_in(o.max_q())
                } else {
                    DebtType::exact_out(o.max_q())
                };
                return Some(Debt::new(magnitude, o.price()))
            }
        }
        None
    }

    pub fn amm_intersect(&self, debt: Debt) -> eyre::Result<u128> {
        match self {
            Self::AMM(a) => a.start_bound.intersect_with_debt(debt),
            _ => Ok(0)
        }
    }

    /// Is the underlying order a Partial Fill compatible order
    pub fn is_partial(&self) -> bool {
        match self {
            Self::BookOrder(o) => {
                matches!(
                    o.order,
                    GroupedVanillaOrder::Standing(StandingVariants::Partial(_))
                        | GroupedVanillaOrder::KillOrFill(FlashVariants::Partial(_))
                )
            }
            Self::BookOrderFragment { order, .. } => {
                matches!(
                    order.order,
                    GroupedVanillaOrder::Standing(StandingVariants::Partial(_))
                        | GroupedVanillaOrder::KillOrFill(FlashVariants::Partial(_))
                )
            }
            Self::AMM(_) => false,
            Self::Composite(_) => false
        }
    }

    /// If `true`, this is an inverse order that operates with T1 as a base
    /// quantity instead of T0.  That means this order will cause or react to
    /// debt
    pub fn inverse_order(&self) -> bool {
        let order = match self {
            Self::BookOrder(o) => o,
            Self::BookOrderFragment { order, .. } => order,
            _ => return false
        };
        let is_bid = order.is_bid();
        let exact_in = order.exact_in();
        is_bid == exact_in
    }

    fn book_order_q_t0(
        order: &OrderWithStorageData<GroupedVanillaOrder>,
        debt: Option<&Debt>
    ) -> u128 {
        // Get the raw max quantity of the order
        let raw_q = order.max_q();
        // Bid exact_in orders and ask exact_out orders are in T1 Context
        let t1_context = order.is_bid() == order.exact_in();

        if t1_context {
            // Exact In bid or Exact Out ask both interact with debt so we calculate them at
            // our target_price which is the price of the Debt if one already exists,
            // otherwise the limit price of this order When calculating T1, we
            // round bids down (you get the minimum out) and we round asks up (you put more
            // in than needed)
            // First we find our target price - target price is always properly flipped to
            // be T1/T0
            let (target_price, net_q) = if let Some(d) = debt {
                let q = if order.is_bid() == d.bid_side() {
                    // Same side - we add our slack to the order to properly determine how much T0
                    // should be allocated
                    raw_q + d.slack()
                } else {
                    // Opposite side
                    raw_q.saturating_sub(d.slack())
                };
                (d.price(), q)
            } else {
                (order.price_for_book_side(order.is_bid()), raw_q)
            };
            let round_up = !order.is_bid();
            target_price.inverse_quantity(net_q, round_up)
        } else {
            // Exact Out bid (normal bid) and Exact In ask (normal ask)
            // These don't cause or interact with debt so they just offer what they offer
            raw_q
        }
    }

    /// Raw quantity of a book order
    pub fn raw_book_quantity(&self) -> u128 {
        if let Self::BookOrder(o) | Self::BookOrderFragment { order: o, .. } = self {
            o.max_q()
        } else {
            0
        }
    }

    /// Retrieve the quantity available within the bounds of a given order
    pub fn quantity(&self, target_price: OrderPrice, debt: Option<&Debt>) -> OrderVolume {
        match self {
            Self::BookOrder(o) => Self::book_order_q_t0(o, debt),
            Self::BookOrderFragment { order, state: OrderFillState::PartialFill(partial_q) } => {
                Self::book_order_q_t0(order, debt).saturating_sub(*partial_q)
            }
            Self::BookOrderFragment { order, .. } => Self::book_order_q_t0(order, debt),
            Self::AMM(ammo) => ammo.quantity(target_price).0,
            Self::Composite(c) => c.quantity(target_price.into())
        }
    }

    /// Gets the amount a composite order needs to self-fill in order to move to
    /// a new target price
    pub fn negative_quantity(&self, target_price: OrderPrice) -> OrderVolume {
        match self {
            Self::Composite(c) => c.negative_quantity(target_price.into()),
            _ => 0
        }
    }

    /// Retrieve the starting price bound for a given order.  This price is
    /// always t0/t1 and is flipped if necessary
    pub fn price(&self) -> OrderPrice {
        match self {
            Self::BookOrder(o) => o.price_for_book_side(o.is_bid).into(),
            Self::BookOrderFragment { order, .. } => order.price_for_book_side(order.is_bid).into(),
            Self::AMM(o) => (*o.start_bound.price()).into(),
            Self::Composite(o) => o.start_price().into()
        }
    }
}

// Make some tests for book_order_quantity
