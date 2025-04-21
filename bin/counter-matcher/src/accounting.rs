use std::collections::HashMap;

use alloy_primitives::Address;
use angstrom_types::{primitive::AngstromSigner, sol_bindings::grouped_orders::AllOrders};

/// used to see what wallet should create the orders
pub struct WalletAccounting {
    pk:               AngstromSigner,
    available_tokens: HashMap<Address, u128>
}

impl WalletAccounting {
    pub fn can_support_amount(&self, order: AllOrders) -> bool {
        true
    }
}
