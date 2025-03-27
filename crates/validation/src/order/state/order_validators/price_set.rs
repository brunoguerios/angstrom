use angstrom_types::{primitive::OrderValidationError, sol_bindings::RawPoolOrder};

use super::{OrderValidation, OrderValidationState};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct EnsurePriceSet;

impl OrderValidation for EnsurePriceSet {
    fn validate_order<O: RawPoolOrder>(
        &self,
        state: &mut OrderValidationState<O>
    ) -> Result<(), OrderValidationError> {
        if state.order().limit_price().is_zero() {
            Err(OrderValidationError::NoPriceSpecified)
        } else {
            Ok(())
        }
    }
}
