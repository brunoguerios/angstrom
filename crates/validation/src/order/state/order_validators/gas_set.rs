use angstrom_types::{primitive::OrderValidationError, sol_bindings::RawPoolOrder};

use super::{OrderValidation, OrderValidationState};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct EnsureGasSet;

impl OrderValidation for EnsureGasSet {
    fn validate_order<O: RawPoolOrder>(
        &self,
        state: &mut OrderValidationState<O>
    ) -> Result<(), OrderValidationError> {
        if state.order().max_gas_token_0() == 0 {
            Err(OrderValidationError::NoGasSpecified)
        } else {
            Ok(())
        }
    }
}
