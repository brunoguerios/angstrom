use std::collections::HashMap;

use alloy_primitives::B256;
use angstrom_types::sol_bindings::grouped_orders::AllOrders;

/// holds orders that are currently being processed.
pub struct OrderManager {
    /// order id to order
    active_orders: HashMap<B256, AllOrders>
}
