use angstrom_types::{primitive::OrderValidationError, sol_bindings::RawPoolOrder};

use super::{OrderValidation, OrderValidationState};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct EnsureAmountSet;

impl OrderValidation for EnsureAmountSet {
    fn validate_order<O: RawPoolOrder>(
        &self,
        state: &mut OrderValidationState<O>
    ) -> Result<(), OrderValidationError> {
        if state.min_qty_in_t0() == 0 {
            Err(OrderValidationError::NoAmountSpecified)
        } else {
            Ok(())
        }
    }
}
