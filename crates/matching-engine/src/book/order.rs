use angstrom_types::{
    matching::uniswap::PoolPriceVec,
    orders::{OrderID, OrderId, OrderPrice, OrderVolume},
    sol_bindings::grouped_orders::{
        FlashVariants, GroupedVanillaOrder, OrderWithStorageData, StandingVariants
    }
};

use super::BookOrder;

/// Definition of the various types of order that we can serve, as well as the
/// outcomes we're able to have for them
#[derive(Clone, Debug)]
pub enum OrderContainer<'a, 'b> {
    /// A complete order from our book
    BookOrder(&'a BookOrder),
    /// A fragment of an order from our book yet to be filled
    BookOrderFragment(&'b BookOrder),
    /// An order constructed from the current state of our AMM
    AMM(PoolPriceVec<'a>)
}

impl<'a, 'b> OrderContainer<'a, 'b> {
    pub fn id(&self) -> Option<OrderId> {
        match self {
            Self::BookOrder(o) => Some(o.order_id),
            Self::BookOrderFragment(o) => Some(o.order_id),
            _ => None
        }
    }

    pub fn is_amm(&self) -> bool {
        matches!(self, Self::AMM(_))
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
            Self::BookOrderFragment(o) => {
                matches!(
                    o.order,
                    GroupedVanillaOrder::Standing(StandingVariants::Partial(_))
                        | GroupedVanillaOrder::KillOrFill(FlashVariants::Partial(_))
                )
            }
            Self::AMM(_) => false
        }
    }

    /// Retrieve the quantity available within the bounds of a given order
    pub fn quantity(&self, limit_price: OrderPrice) -> OrderVolume {
        match self {
            Self::BookOrder(o) => o.quantity(),
            Self::BookOrderFragment(o) => o.quantity(),
            Self::AMM(ammo) => ammo.quantity(limit_price).0
        }
    }

    /// Retrieve the price for a given order
    pub fn price(&self) -> OrderPrice {
        match self {
            Self::BookOrder(o) => o.price().into(),
            Self::BookOrderFragment(o) => o.price().into(),
            Self::AMM(o) => (*o.start_bound.price()).into()
        }
    }

    /// Produce a new order representing the remainder of the current order
    /// after the fill operation has been performed
    pub fn fill(&self, filled_quantity: OrderVolume) -> BookOrder {
        match self {
            Self::AMM(_) => panic!("This should never happen"),
            Self::BookOrder(o) => {
                let newo = (**o).clone();
                newo.try_map_inner(|f| Ok(f.fill(filled_quantity))).unwrap()
            }
            Self::BookOrderFragment(o) => {
                let newo = (**o).clone();
                newo.try_map_inner(|f| Ok(f.fill(filled_quantity))).unwrap()
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum Order<'a> {
    Flash(FlashVariants),
    Standing(StandingVariants),
    AMM(PoolPriceVec<'a>)
}

impl<'a> Order<'a> {
    /// Determine if this is an AMM order
    pub fn is_amm(&self) -> bool {
        matches!(self, Self::AMM(_))
    }

    pub fn id(&self) -> Option<OrderID> {
        match self {
            Self::Flash(_) => Some(0),
            Self::Standing(_) => Some(0),
            _ => None
        }
    }

    /// Retrieve the quantity available within the bounds of a given order
    pub fn quantity(&self, limit_price: OrderPrice) -> OrderVolume {
        match self {
            Self::Flash(lo) => match lo {
                FlashVariants::Exact(e) => e.amount,
                FlashVariants::Partial(p) => p.max_amount_in
            },
            Self::Standing(lo) => match lo {
                StandingVariants::Exact(e) => e.amount,
                StandingVariants::Partial(p) => p.max_amount_in
            },
            Self::AMM(ammo) => ammo.quantity(limit_price).0
        }
    }

    // /// Retrieve the price for a given order
    // pub fn price(&self) -> OrderPrice {
    //     match self {
    //         Self::KillOrFill(lo) => lo.min_price,
    //         Self::PartialFill(lo) => lo.min_price,
    //         Self::AMM(ammo) => ammo.start_bound.as_u256()
    //     }
    // }

    // /// Produce a new order representing the remainder of the current order
    // /// after the fill operation has been performed
    // pub fn fill(&self, filled_quantity: OrderVolume) -> Self {
    //     match self {
    //         Self::Flash(lo) => Self::Flash(FlashOrder {
    //             max_amount_in_or_out: lo.max_amount_in_or_out - filled_quantity,
    //             ..lo.clone()
    //         }),
    //         Self::Standing(lo) => Self::PartialFill(StandingOrder {
    //             max_amount_in_or_out: lo.max_amount_in_or_out - filled_quantity,
    //             ..lo.clone()
    //         }),
    //         Self::AMM(r) => {
    //             r.fill(filled_quantity);
    //             // Return a bogus order that we never use
    //             Self::PartialFill(StandingOrder::default())
    //         }
    //     }
    // }
}
