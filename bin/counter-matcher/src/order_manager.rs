use std::{collections::HashMap, sync::Arc};

use alloy::{signers::SignerSync, sol_types::SolValue};
use alloy_primitives::B256;
use angstrom_rpc::api::OrderApiClient;
use angstrom_types::{
    matching::Ray,
    orders::CancelOrderRequest,
    sol_bindings::{RawPoolOrder, grouped_orders::AllOrders}
};
use jsonrpsee::ws_client::WsClient;
use pade::PadeEncode;
use sepolia_bundle_lander::env::ProviderType;
use testing_tools::type_generator::orders::UserOrderBuilder;

use crate::accounting::WalletAccounting;

/// holds orders that are currently being processed.
pub struct OrderManager {
    block_number: u64,
    wallets:      Vec<WalletAccounting>,
    provider:     Arc<ProviderType>,

    /// our active orders to the wallet that signed them.
    /// due to how we do accounting, we don't have to store the order
    active_orders: HashMap<B256, usize>,
    /// map of user order to our active counter order
    user_orders:   HashMap<B256, B256>,
    client:        WsClient
}

impl OrderManager {
    pub fn new(
        block_number: u64,
        provider: Arc<ProviderType>,
        wallets: Vec<WalletAccounting>,
        client: WsClient
    ) -> Self {
        Self {
            block_number,
            provider,
            wallets,
            client,
            active_orders: Default::default(),
            user_orders: Default::default()
        }
    }

    pub async fn new_block(&mut self, block_number: u64) {
        for wallet in &mut self.wallets {
            wallet
                .update_balances_for_block(block_number, self.provider.clone())
                .await;
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

        self.try_create_counter_order(&order).await;
    }

    pub async fn on_order_cancel(&mut self, order: AllOrders) {
        let hash = order.order_hash();

        let Some(our_hash) = self.user_orders.remove(&hash) else { return };
        let Some(wallet) = self.active_orders.remove(&our_hash) else { return };

        let order_wallet = &mut self.wallets[wallet];
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

        // remove the token in that we allocated to this order
        order_wallet.remove_order(order.token_in(), &our_hash);
    }

    // this order will be partial
    async fn try_create_counter_order(&mut self, placed_user_order: &AllOrders) {
        // calculate the flipped order token and amounts
        let price_1_0 = if placed_user_order.is_bid() {
            Ray::from(placed_user_order.limit_price()).inv_ray_round(true)
        } else {
            Ray::from(placed_user_order.limit_price())
        };

        let (token_in, token_out, amount_needed, price) = if !placed_user_order.exact_in() {
            (
                placed_user_order.token_out(),
                placed_user_order.token_in(),
                placed_user_order.amount(),
                Ray::from(placed_user_order.limit_price()).inv_ray_round(true)
            )
        } else {
            // is exact in. need to get amount out

            let amount = if placed_user_order.is_bid() {
                // one for zero
                price_1_0.inverse_quantity(placed_user_order.amount(), true)
            } else {
                // zero for 1
                price_1_0.quantity(placed_user_order.amount(), false)
            };

            (
                placed_user_order.token_out(),
                placed_user_order.token_in(),
                amount,
                // always other side.
                Ray::from(placed_user_order.limit_price()).inv_ray_round(true)
            )
        };

        // see if there is any wallet that can supply these amounts

        let Some((wallet_index, wallet)) = self
            .wallets
            .iter_mut()
            .enumerate()
            .find(|(_, wallet)| wallet.can_support_amount(&token_in, amount_needed))
        else {
            tracing::info!(?placed_user_order, "no wallet has enough to support user order");
            return;
        };

        // order with deadline and nonce
        let order: AllOrders = if let Some(deadline) = placed_user_order.deadline() {
            let nonce = self.client.valid_nonce(wallet.pk.address()).await.unwrap();
            UserOrderBuilder::default()
                .partial()
                .standing()
                .nonce(nonce)
                .deadline(deadline)
                .exact_in(true)
                .amount(amount_needed)
                .asset_in(token_in)
                .asset_out(token_out)
                .min_price(price)
                .signing_key(Some(wallet.pk.clone()))
                .build()
                .into()
        } else {
            UserOrderBuilder::default()
                .partial()
                .kill_or_fill()
                .exact_in(true)
                .amount(amount_needed)
                .asset_in(token_in)
                .asset_out(token_out)
                .min_price(price)
                .signing_key(Some(wallet.pk.clone()))
                .build()
                .into()
        };

        let our_order_hash = order.order_hash();

        self.user_orders
            .insert(placed_user_order.order_hash(), our_order_hash);
        self.active_orders.insert(our_order_hash, wallet_index);

        let res = self.client.send_order(order).await.unwrap();

        if let Err(e) = res {
            tracing::error!(%e, "failed to place counter order");

            self.active_orders.remove(&our_order_hash);
            self.user_orders.remove(&placed_user_order.order_hash());
            return;
        }

        // add order to wallet accounting
        wallet.add_order(token_in, our_order_hash, amount_needed);
    }
}
