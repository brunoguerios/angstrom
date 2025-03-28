use angstrom_types::{primitive::OrderValidationError, sol_bindings::RawPoolOrder};

use super::{OrderValidation, OrderValidationState};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct EnsureAmountSet;

impl OrderValidation for EnsureAmountSet {
    fn validate_order<O: RawPoolOrder>(
        &self,
        state: &mut OrderValidationState<O>
    ) -> Result<(), OrderValidationError> {
        if state.order().min_amount() == 0 {
            Err(OrderValidationError::NoAmountSpecified)
        } else {
            Ok(())
        }
    }
}

#[test]
fn test_no_amount_specified_error() {
    use angstrom_types::{
        primitive::OrderValidationError, sol_bindings::grouped_orders::StandingVariants
    };

    use crate::order::{
        GroupedVanillaOrder,
        state::order_validators::{EnsureAmountSet, OrderValidationState, make_base_order}
    };

    let mut order = make_base_order();
    if let GroupedVanillaOrder::Standing(StandingVariants::Partial(ref mut o)) = order {
        o.min_amount_in = 0; // This is the value checked by min_amount()
    }

    let validator = EnsureAmountSet;
    let mut state = OrderValidationState::new(&order);
    let result = validator.validate_order(&mut state);
    assert_eq!(result, Err(OrderValidationError::NoAmountSpecified));
}
