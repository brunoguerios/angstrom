use alloy::primitives::U256;
use amount_set::EnsureAmountSet;
use angstrom_types::{
    primitive::OrderValidationError,
    sol_bindings::{RawPoolOrder, Ray}
};
use gas_set::EnsureGasSet;
use max_gas_lt_min::EnsureMaxGasLessThanMinAmount;
use price_set::EnsurePriceSet;

pub mod amount_set;
pub mod gas_set;
pub mod max_gas_lt_min;
pub mod price_set;

pub const ORDER_VALIDATORS: [OrderValidator; 4] = [
    OrderValidator::EnsureAmountSet(EnsureAmountSet),
    OrderValidator::EnsureMaxGasLessThanMinAmount(EnsureMaxGasLessThanMinAmount),
    OrderValidator::EnsureGasSet(EnsureGasSet),
    OrderValidator::EnsurePriceSet(EnsurePriceSet)
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

    pub const fn order(&self) -> &'a O {
        self.order
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
    EnsureMaxGasLessThanMinAmount(EnsureMaxGasLessThanMinAmount),
    EnsurePriceSet(EnsurePriceSet),
    EnsureGasSet(EnsureGasSet)
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
            OrderValidator::EnsureGasSet(validator) => validator.validate_order(state),
            OrderValidator::EnsurePriceSet(validator) => validator.validate_order(state)
        }
    }
}

#[cfg(test)]
pub fn make_base_order() -> angstrom_types::sol_bindings::grouped_orders::GroupedVanillaOrder {
    testing_tools::type_generator::orders::UserOrderBuilder::new()
        .standing()
        .exact()
        .amount(1)
        .bid_min_price(Ray(U256::from(1)))
        .block(100)
        .deadline(U256::from(999_999))
        .nonce(0)
        .recipient(Default::default())
        .build()
}
