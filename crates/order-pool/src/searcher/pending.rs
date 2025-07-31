use std::{
    cmp::Reverse,
    collections::{BTreeMap, HashMap, HashSet}
};

use alloy::primitives::{B256, FixedBytes};
use angstrom_types::{
    orders::OrderPriorityData,
    sol_bindings::{grouped_orders::OrderWithStorageData, rpc_orders::TopOfBlockOrder}
};
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingPool {
    /// all order hashes

    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    orders: HashMap<FixedBytes<32>, (bool, OrderWithStorageData<TopOfBlockOrder>)>,
    /// bids are sorted descending by price,
    #[serde_as(as = "Vec<(_, _)>")]
    bids:   BTreeMap<Reverse<OrderPriorityData>, FixedBytes<32>>,
    /// asks are sorted ascending by price,  
    #[serde_as(as = "Vec<(_, _)>")]
    asks:   BTreeMap<OrderPriorityData, FixedBytes<32>>
}

impl PendingPool {
    #[allow(unused)]
    pub fn new() -> Self {
        Self { orders: HashMap::new(), bids: BTreeMap::new(), asks: BTreeMap::new() }
    }

    pub fn get_order(&self, id: FixedBytes<32>) -> Option<OrderWithStorageData<TopOfBlockOrder>> {
        self.orders.get(&id).cloned().map(|order| order.1)
    }

    pub fn add_order(&mut self, order: OrderWithStorageData<TopOfBlockOrder>) {
        if order.is_bid {
            self.bids
                .insert(Reverse(order.priority_data), order.order_id.hash);
        } else {
            self.asks.insert(order.priority_data, order.order_id.hash);
        }
        self.orders.insert(order.order_id.hash, (false, order));
    }

    pub fn cancel_order(&mut self, id: FixedBytes<32>) {
        if let Some((canceled, order)) = self.orders.get_mut(&id) {
            tracing::info!(?order, "canceled tob order");
            *canceled = true;
        }
    }

    pub fn remove_all_cancelled_orders(&mut self) -> Vec<OrderWithStorageData<TopOfBlockOrder>> {
        let mut res = vec![];
        let ids = self
            .orders
            .iter()
            .filter(|(_, (is_cancelled, _))| *is_cancelled)
            .map(|(key, _)| *key)
            .collect::<Vec<_>>();
        for id in ids {
            if let Some(order) = self.remove_order(id) {
                res.push(order);
            }
        }

        res
    }

    pub fn remove_order(
        &mut self,
        id: FixedBytes<32>
    ) -> Option<OrderWithStorageData<TopOfBlockOrder>> {
        let order = self.orders.remove(&id)?.1;

        if order.is_bid {
            let _ = self.bids.remove(&Reverse(order.priority_data));
        } else {
            let _ = self.asks.remove(&order.priority_data);
        }

        // probably fine to strip extra data here
        Some(order)
    }

    pub fn get_all_orders(&self) -> Vec<OrderWithStorageData<TopOfBlockOrder>> {
        self.orders
            .values()
            .filter(|(is_cancelled, _)| !is_cancelled)
            .cloned()
            .map(|order| order.1)
            .collect()
    }

    pub fn get_all_orders_with_cancelled(&self) -> Vec<OrderWithStorageData<TopOfBlockOrder>> {
        self.orders.values().cloned().map(|order| order.1).collect()
    }

    pub fn get_all_orders_with_hashes(
        &self,
        hashes: &HashSet<B256>
    ) -> Vec<OrderWithStorageData<TopOfBlockOrder>> {
        self.orders
            .values()
            .filter_map(|(_, order)| hashes.contains(&order.order_id.hash).then_some(order))
            .cloned()
            .collect()
    }
}
