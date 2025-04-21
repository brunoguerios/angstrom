use std::{collections::HashMap, sync::Arc};

use alloy_primitives::{Address, B256};
use angstrom_types::{primitive::AngstromSigner, sol_bindings::grouped_orders::AllOrders};
use sepolia_bundle_lander::env::ProviderType;

/// used to see what wallet should create the orders
pub struct WalletAccounting {
    pub pk:              AngstromSigner,
    on_chain:            HashMap<Address, u128>,
    // token -> order_id -> amount
    pending_adjustments: HashMap<Address, HashMap<B256, u128>>
}

impl WalletAccounting {
    /// initializes the wallet with the curent
    pub async fn new(pk: AngstromSigner, provider: Arc<ProviderType>) -> Self {
        todo!()
    }

    pub async fn new_block(&mut self, block_number: u64, provider: Arc<ProviderType>) {}

    pub fn can_support_amount(&self, order: AllOrders) -> bool {
        true
    }
}
