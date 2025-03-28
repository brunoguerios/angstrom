use angstrom_types::{primitive::OrderValidationError, sol_bindings::RawPoolOrder};

use super::{OrderValidation, OrderValidationState};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct EnsureMaxGasLessThanMinAmount;

impl OrderValidation for EnsureMaxGasLessThanMinAmount {
    fn validate_order<O: RawPoolOrder>(
        &self,
        state: &mut OrderValidationState<O>
    ) -> Result<(), OrderValidationError> {
        if state.min_qty_in_t0() < state.order().max_gas_token_0() {
            Err(OrderValidationError::MaxGasGreaterThanMinAmount)
        } else {
            Ok(())
        }
    }
}

#[test]
fn test_max_gas_greater_than_min_error() {
    use angstrom_types::{
        primitive::OrderValidationError, sol_bindings::grouped_orders::StandingVariants
    };

    use crate::order::{
        GroupedVanillaOrder,
        state::order_validators::{
            EnsureMaxGasLessThanMinAmount, OrderValidationState, make_base_order
        }
    };

    let mut order = make_base_order();
    if let GroupedVanillaOrder::Standing(StandingVariants::Exact(ref mut o)) = order {
        o.max_extra_fee_asset0 = o.amount + 1;
    }

    let validator = EnsureMaxGasLessThanMinAmount;
    let mut state = OrderValidationState::new(&order);
    let result = validator.validate_order(&mut state);
    assert_eq!(result, Err(OrderValidationError::MaxGasGreaterThanMinAmount));
}
