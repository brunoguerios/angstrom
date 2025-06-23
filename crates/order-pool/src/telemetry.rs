use std::collections::VecDeque;

use angstrom_types::{orders::OrderOrigin, sol_bindings::grouped_orders::AllOrders};
use serde::{Deserialize, Serialize};
use validation::order::OrderValidatorHandle;

use crate::{OrderIndexer, order_storage::OrderStorage, order_tracker::OrderTracker};

// Didn't want to have to do a massive refactor of the order pool for this.
#[derive(Clone, Serialize, Deserialize)]
pub struct OrderPoolSnapshot {
    pub order_storage: OrderStorage,
    pub order_tracker: OrderTracker,
    pub pending_transition_validation_orders: VecDeque<(OrderOrigin, AllOrders)>
}

impl<V> From<&OrderIndexer<V>> for OrderPoolSnapshot
where
    V: OrderValidatorHandle<Order = AllOrders>
{
    fn from(value: &OrderIndexer<V>) -> Self {
        let storage = value.order_storage.deep_clone();
        let tracker = value.order_tracker.clone();
        let pending = value.validator.get_waiting_orders();
        Self {
            order_tracker: tracker,
            order_storage: storage,
            pending_transition_validation_orders: pending
        }
    }
}
