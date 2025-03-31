use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH}
};

use alloy::primitives::{Address, B256, U256};
use angstrom_types::{
    orders::{OrderId, OrderLocation, OrderOrigin},
    primitive::PeerId,
    sol_bindings::{
        RawPoolOrder, ext::grouped_orders::AllOrders, grouped_orders::OrderWithStorageData
    }
};
use tokio::sync::oneshot::Sender;
use validation::order::OrderValidationResults;

use crate::{order_indexer::InnerCancelOrderRequest, order_storage::OrderStorage};

/// This is used to remove validated orders. During validation
/// the same check wil be ran but with more accuracy
const ETH_BLOCK_TIME: Duration = Duration::from_secs(12);

/// Used as a storage of order hashes to order ids of validated and pending
/// validation orders.
#[derive(Default)]
pub struct OrderTracker {
    address_to_orders:      HashMap<Address, Vec<OrderId>>,
    /// current block_number
    /// Order hash to order id, used for order inclusion lookups
    order_hash_to_order_id: HashMap<B256, OrderId>,
    /// Used to get trigger reputation side-effects on network order submission
    order_hash_to_peer_id:  HashMap<B256, Vec<PeerId>>,
    /// Used to avoid unnecessary computation on order spam
    seen_invalid_orders:    HashSet<B256>,
    /// Used to protect against late order propagation
    cancelled_orders:       HashMap<B256, InnerCancelOrderRequest>
}

impl OrderTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_valid_cancel(&self, order: &B256, order_addr: Address) -> bool {
        let Some(req) = self.cancelled_orders.get(order) else { return false };
        req.from == order_addr
    }

    pub fn is_duplicate(&self, hash: &B256) -> bool {
        return self.order_hash_to_order_id.contains_key(hash)
            || self.seen_invalid_orders.contains(hash);
    }

    pub fn track_peer_id(&mut self, hash: B256, peer_id: Option<PeerId>) {
        if let Some(peer) = peer_id {
            self.order_hash_to_peer_id
                .entry(hash)
                .or_default()
                .push(peer);
        }
    }

    pub fn invalid_verification(&mut self, hash: B256) -> Vec<PeerId> {
        self.seen_invalid_orders.insert(hash);

        return self.order_hash_to_peer_id.remove(&hash).unwrap_or_default();
    }

    pub fn remove_expired_orders(
        &mut self,
        block_number: u64,
        storage: Arc<OrderStorage>
    ) -> Vec<B256> {
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let expiry_deadline = U256::from((time + ETH_BLOCK_TIME).as_secs()); // grab all expired hashes
        let hashes = self
            .order_hash_to_order_id
            .iter()
            .filter(|(_, v)| {
                v.deadline.map(|i| i <= expiry_deadline).unwrap_or_default()
                    || v.flash_block
                        .map(|b| b != block_number + 1)
                        .unwrap_or_default()
            })
            .map(|(k, _)| *k)
            .collect::<Vec<_>>();

        hashes
            .iter()
            // remove hash from id
            .map(|hash| self.order_hash_to_order_id.remove(hash).unwrap())
            // remove from all underlying pools
            .for_each(|id| {
                self.address_to_orders
                    .values_mut()
                    // remove from address to orders
                    .for_each(|v| v.retain(|o| o != &id));
                match id.location {
                    OrderLocation::Searcher => storage.remove_searcher_order(&id),
                    OrderLocation::Limit => storage.remove_limit_order(&id)
                };
            });

        hashes
    }

    pub fn filled_orders<'a>(
        &'a mut self,
        orders: &'a [B256],
        storage: &'a OrderStorage
    ) -> impl Iterator<Item = OrderWithStorageData<AllOrders>> + 'a {
        orders
            .iter()
            .filter_map(|hash| self.order_hash_to_order_id.remove(hash))
            .filter_map(move |order_id| match order_id.location {
                OrderLocation::Limit => storage.remove_limit_order(&order_id),
                OrderLocation::Searcher => storage.remove_searcher_order(&order_id)
            })
    }

    pub fn park_orders(&mut self, txes: &[B256], storage: &OrderStorage) {
        let order_info = txes
            .iter()
            .filter_map(|tx_hash| self.order_hash_to_order_id.get(tx_hash))
            .collect::<Vec<_>>();
        storage.park_orders(order_info);
    }

    pub fn new_valid_order(&mut self, hash: &B256, user: Address, id: OrderId) {
        self.order_hash_to_peer_id.remove(hash);
        self.order_hash_to_order_id.insert(*hash, id);
        // nonce overlap is checked during validation so its ok we
        // don't check for duplicates
        self.address_to_orders.entry(user).or_default().push(id);
    }
}

pub enum OrderHandlingRes {
    CancelOrderRequest,
    DuplicateOrderRequest,
    ValidOrderRequest,
    Filled
}
