use std::collections::HashMap;

use alloy_primitives::B256;
use angstrom_types::sol_bindings::grouped_orders::AllOrders;
use jsonrpsee::ws_client::WsClient;

use crate::accounting::WalletAccounting;

/// holds orders that are currently being processed.
pub struct OrderManager {
    wallets: Vec<WalletAccounting>,

    /// order id to index into wallets of the order + order itself.
    /// we need to track wallet incase we need to cancel it
    active_orders: HashMap<B256, (usize, AllOrders)>,
    client:        WsClient
}

impl OrderManager {}
