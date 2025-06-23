use std::collections::VecDeque;

use angstrom_types::{orders::OrderOrigin, sol_bindings::grouped_orders::AllOrders};
use order_pool::{order_storage::OrderStorage, order_tracker::OrderTracker};
use serde::{Deserialize, Serialize};

// Didn't want to have to do a massive refactor of the order pool for this.
#[derive(Serialize, Deserialize)]
pub struct OrderPoolSnapshot {
    pub order_storage: OrderStorage,
    pub order_tracker: OrderTracker,
    pub pending_transition_validation_orders: VecDeque<(OrderOrigin, AllOrders)>
}
