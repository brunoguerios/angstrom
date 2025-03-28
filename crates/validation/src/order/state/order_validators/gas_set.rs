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

#[test]
fn test_no_gas_specified() {
    use angstrom_types::{
        primitive::OrderValidationError, sol_bindings::grouped_orders::StandingVariants
    };

    use crate::order::{
        GroupedVanillaOrder,
        state::order_validators::{EnsureGasSet, OrderValidationState, make_base_order}
    };

    let mut order = make_base_order();
    if let GroupedVanillaOrder::Standing(StandingVariants::Exact(ref mut o)) = order {
        o.max_extra_fee_asset0 = 0;
    }

    let validator = EnsureGasSet;
    let mut state = OrderValidationState::new(&order);
    let result = validator.validate_order(&mut state);
    assert_eq!(result, Err(OrderValidationError::NoGasSpecified));
}
