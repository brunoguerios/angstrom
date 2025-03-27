use alloy::primitives::U256;
use amount_set::EnsureAmountSet;
use angstrom_types::{
    primitive::OrderValidationError,
    sol_bindings::{RawPoolOrder, Ray}
};
use max_gas_lt_min::EnsureMaxGasLessThanMinAmount;

pub mod amount_set;
pub mod max_gas_lt_min;

pub const ORDER_VALIDATORS: [OrderValidator; 2] = [
    OrderValidator::EnsureAmountSet(EnsureAmountSet),
    OrderValidator::EnsureMaxGasLessThanMinAmount(EnsureMaxGasLessThanMinAmount)
];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct OrderValidationState<'a, O: RawPoolOrder> {
    order:   &'a O,
    min_qty: Option<u128>
}

impl<'a, O: RawPoolOrder> OrderValidationState<'a, O> {
    pub const fn new(order: &'a O) -> Self {
        Self { order, min_qty: None }
    }

    pub fn min_qty_in_t0(&mut self) -> u128 {
        if let Some(min_qty) = self.min_qty {
            min_qty
        } else {
            let order = self.order;
            let min_qty = if !order.is_bid() {
                if order.exact_in() {
                    order.min_amount()
                } else {
                    Ray::from(order.limit_price()).inverse_quantity(order.min_amount(), true)
                }
            } else if order.exact_in() {
                Ray::from(order.limit_price())
                    .mul_quantity(U256::from(order.min_amount()))
                    .to::<u128>()
            } else {
                order.min_amount()
            };
            self.min_qty = Some(min_qty);
            min_qty
        }
    }
}

pub trait OrderValidation {
    fn validate_order<O: RawPoolOrder>(
        &self,
        state: &mut OrderValidationState<O>
    ) -> Result<(), OrderValidationError>;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrderValidator {
    EnsureAmountSet(EnsureAmountSet),
    EnsureMaxGasLessThanMinAmount(EnsureMaxGasLessThanMinAmount)
}

impl OrderValidation for OrderValidator {
    fn validate_order<O: RawPoolOrder>(
        &self,
        state: &mut OrderValidationState<O>
    ) -> Result<(), OrderValidationError> {
        match self {
            OrderValidator::EnsureAmountSet(validator) => validator.validate_order(state),
            OrderValidator::EnsureMaxGasLessThanMinAmount(validator) => {
                validator.validate_order(state)
            }
        }
    }
}
