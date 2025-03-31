use std::{
    collections::{HashMap, HashSet},
    time::{SystemTime, UNIX_EPOCH}
};

use alloy::primitives::{Address, B256, U256};
use angstrom_types::{
    orders::{OrderId, OrderOrigin},
    primitive::PeerId,
    sol_bindings::{RawPoolOrder, ext::grouped_orders::AllOrders}
};
use tokio::sync::oneshot::Sender;
use validation::order::OrderValidationResults;

use crate::order_indexer::InnerCancelOrderRequest;

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

    pub fn new_order<F>(
        &mut self,

        peer_id: Option<PeerId>,
        origin: OrderOrigin,
        order: AllOrders,
        validation_res_sub: Option<Sender<OrderValidationResults>>,
        f: F
    ) where
        F: FnOnce(
            OrderOrigin,
            AllOrders,
            Option<Sender<OrderValidationResults>>,
            &HashMap<B256, InnerCancelOrderRequest>
        ) -> OrderHandlingRes
    {
        let hash = order.order_hash();
        let peer_id_c = peer_id.clone();
        let deadline = order.deadline();
        let from = order.from();

        match f(origin, order, validation_res_sub, &self.cancelled_orders) {
            OrderHandlingRes::ValidOrderRequest => {
                if let Some(peer_id) = peer_id_c {
                    self.order_hash_to_peer_id
                        .entry(hash)
                        .or_default()
                        .push(peer_id);
                }
            }
            OrderHandlingRes::CancelOrderRequest => {
                let valid_until = deadline.map_or_else(
                    || {
                        // if no deadline is provided the cancellation request is valid until block
                        // transition
                        SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs()
                    },
                    |deadline| {
                        let bytes: [u8; U256::BYTES] = deadline.to_le_bytes();
                        // should be safe
                        u64::from_le_bytes(bytes[..8].try_into().unwrap())
                    }
                );
                self.cancelled_orders
                    .insert(hash, InnerCancelOrderRequest { from, valid_until });
            }
            _ => {}
        }
    }
}

pub enum OrderHandlingRes {
    CancelOrderRequest,
    DuplicateOrderRequest,
    ValidOrderRequest,
    Filled
}
