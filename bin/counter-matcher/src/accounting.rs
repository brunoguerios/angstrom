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

    pub fn add_order(&mut self, token: Address, order_id: B256, amount: u128) {
        self.pending_adjustments
            .entry(token)
            .or_default()
            .insert(order_id, amount);
    }

    pub fn remove_order(&mut self, token: Address, order_id: &B256) {
        self.pending_adjustments
            .entry(token)
            // should never be hit
            .or_default()
            .remove(order_id);
    }

    pub async fn new_block(&mut self, block_number: u64, provider: Arc<ProviderType>) {}

    /// the order here is the counter matched order.
    /// NOTABLY, we have a max allocation of 20% of the balance of the account
    /// that can be swapped.
    pub fn can_support_amount(&self, order: &AllOrders) -> bool {
        true
    }

    fn available_funds(&self, token: &Address) -> u128 {
        self.on_chain
            .get(token)
            .cloned()
            .map(|mut amount| {
                // take all values and then saturating_sub them;
                if let Some(pending) = self.pending_adjustments.get(token) {
                    pending.values().for_each(|am| {
                        amount = amount.saturating_sub(*am);
                    });
                }

                amount
            })
            .unwrap_or_default()
    }
}
