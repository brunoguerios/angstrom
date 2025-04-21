use std::{collections::HashMap, sync::Arc};

use alloy::{
    signers::{Signer, SignerSync},
    sol_types::SolValue
};
use alloy_primitives::B256;
use angstrom_rpc::api::OrderApiClient;
use angstrom_types::{
    orders::CancelOrderRequest,
    sol_bindings::{RawPoolOrder, grouped_orders::AllOrders}
};
use jsonrpsee::ws_client::WsClient;
use pade::PadeEncode;
use sepolia_bundle_lander::env::ProviderType;
use uniswap_v4::uniswap::{pool::EnhancedUniswapPool, pool_data_loader::DataLoader};

use crate::accounting::WalletAccounting;

/// holds orders that are currently being processed.
pub struct OrderManager {
    block_number: u64,
    wallets:      Vec<WalletAccounting>,
    pools:        Vec<EnhancedUniswapPool>,
    provider:     Arc<ProviderType>,

    /// our active orders
    active_orders: HashMap<B256, (usize, AllOrders)>,
    /// map of user order to our active counter order
    user_orders:   HashMap<B256, (B256, AllOrders)>,
    client:        WsClient
}

impl OrderManager {
    pub fn new(
        block_number: u64,
        provider: Arc<ProviderType>,
        wallets: Vec<WalletAccounting>,
        pools: Vec<EnhancedUniswapPool>,
        client: WsClient
    ) -> Self {
        Self { block_number, provider, wallets, pools, client, active_orders: Default::default() }
    }

    pub async fn new_block(&mut self, block_number: u64) {
        for pool in &mut self.pools {
            pool.update_to_block(Some(block_number), self.provider.clone())
                .await
                .unwrap();
        }
        self.block_number = block_number;
    }

    /// handles when a new order is pulled from the stream.
    pub async fn on_new_order(&mut self, order: AllOrders) {
        let hash = order.order_hash();
        if order.is_tob() || self.active_orders.contains_key(&hash) {
            // this is a order we placed
            return;
        }

        // see if we have capability to match this
    }

    pub async fn on_order_cancel(&mut self, order: AllOrders) {
        let hash = order.order_hash();
        let Some((our_hash, order)) = self.user_orders.remove(&hash) else { return };

        let Some((wallet, our_order)) = self.active_orders.remove(&our_hash) else { return };

        let order_wallet = &self.wallets[wallet];
        let addr = order_wallet.pk.address();
        let signing_bytes = (addr, our_hash).abi_encode_packed();
        let sig = order_wallet.pk.sign_message_sync(&signing_bytes).unwrap();
        let encoded_sig = sig.pade_encode();

        let cancel_request = CancelOrderRequest {
            signature:    encoded_sig.into(),
            user_address: addr,
            order_id:     our_hash
        };

        let cancel_res = self.client.cancel_order(cancel_request).await.unwrap();
        if !cancel_res {
            tracing::error!("failed to cancel order");
            panic!();
        }

        // if self.active_orders.contains_key(&hash)
    }
}
