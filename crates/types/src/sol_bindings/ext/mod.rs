//! extension functionality to sol types
use std::fmt;

use alloy::primitives::{Address, FixedBytes, TxHash, U256};
use alloy_primitives::PrimitiveSignature;
use serde::{Deserialize, Serialize};

use crate::{orders::OrderLocation, sol_bindings::Ray};

pub mod flips;
pub mod grouped_orders;

/// The capability of all default orders.
pub trait RawPoolOrder: fmt::Debug + Send + Sync + Clone + Unpin + 'static {
    fn max_gas_token_0(&self) -> u128;
    /// defines  
    /// Hash of the order
    fn order_hash(&self) -> TxHash;

    /// The order signer
    fn from(&self) -> Address;

    /// the amount specified by the user. if the order is a partial, this is the
    /// max
    // /value
    fn amount(&self) -> u128;

    /// the min qty of t0 specified by the user
    fn min_qty_t0(&self) -> Option<u128> {
        if self.amount() == 0 || self.limit_price() == U256::ZERO {
            return None;
        }

        Some(if !self.is_bid() {
            if self.exact_in() {
                self.min_amount()
            } else {
                Ray::from(self.limit_price()).inverse_quantity(self.min_amount(), true)
            }
        } else if self.exact_in() {
            Ray::from(self.limit_price())
                .mul_quantity(U256::from(self.min_amount()))
                .to::<u128>()
        } else {
            self.min_amount()
        })
    }

    /// ensure to override for variants that have tob.
    fn is_tob(&self) -> bool {
        false
    }

    /// if the order is partial. If the min amount == max amount but order is
    /// specified as partial, we don't give it priority ordering
    fn is_partial(&self) -> bool {
        self.min_amount() < self.amount()
    }

    /// the amount specified by the user. if the order is a partial, this is the
    /// min value. otherwise it is the same as amount
    fn min_amount(&self) -> u128;

    /// Limit Price
    fn limit_price(&self) -> U256;

    /// Order deadline
    fn deadline(&self) -> Option<U256>;
    /// order flash block
    fn flash_block(&self) -> Option<u64>;

    /// the way in which we avoid a respend attack
    fn respend_avoidance_strategy(&self) -> RespendAvoidanceMethod;

    /// when validating, the priority in which we sort orders to allocate
    /// the total balance and approval.
    fn validation_priority(&self) -> OrderValidationPriority {
        OrderValidationPriority {
            order_hash: self.order_hash(),
            is_tob:     self.is_tob(),
            is_partial: self.is_partial(),
            respend:    self.respend_avoidance_strategy()
        }
    }

    /// token in
    fn token_in(&self) -> Address;
    /// token out
    fn token_out(&self) -> Address;

    /// An order is a bid if it's putting in T1 to get out T0.  T1 is always the
    /// greater address, so if `token_in > token_out` then T1 is being put in to
    /// get T0 out and this order is a bid
    fn is_bid(&self) -> bool {
        self.token_in() > self.token_out()
    }

    fn is_valid_signature(&self) -> bool;

    fn order_location(&self) -> OrderLocation;

    /// whether to use angstrom balances or not
    fn use_internal(&self) -> bool;

    fn order_signature(&self) -> eyre::Result<PrimitiveSignature>;

    fn exact_in(&self) -> bool;

    fn has_hook(&self) -> bool;
}

pub trait GenerateFlippedOrder: Send + Sync + Clone + Unpin + 'static {
    fn flip(&self) -> Self
    where
        Self: Sized;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, Copy)]
pub enum RespendAvoidanceMethod {
    Nonce(u64),
    Block(u64)
}

impl Default for RespendAvoidanceMethod {
    fn default() -> Self {
        Self::Nonce(0)
    }
}

impl RespendAvoidanceMethod {
    pub fn get_ord_for_pending_orders(&self) -> u64 {
        let Self::Nonce(n) = self else { return 0 };
        *n
    }
}

#[derive(Clone, Debug, Copy)]
pub struct OrderValidationPriority {
    pub order_hash: B256,
    pub is_tob:     bool,
    pub is_partial: bool,
    pub respend:    RespendAvoidanceMethod
}
