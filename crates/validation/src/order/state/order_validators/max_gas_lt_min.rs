use angstrom_types::{primitive::OrderValidationError, sol_bindings::RawPoolOrder};

use super::{OrderValidation, OrderValidationState};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct EnsureMaxGasLessThanMinAmount;

impl OrderValidation for EnsureMaxGasLessThanMinAmount {
    fn validate_order<O: RawPoolOrder>(
        &self,
        state: &mut OrderValidationState<O>
    ) -> Result<(), OrderValidationError> {
        if state.min_qty_in_t0() < state.order.max_gas_token_0() {
            Err(OrderValidationError::MaxGasGreaterThanMinAmount)
        } else {
            Ok(())
        }
    }
}
