//! Uniswap AngstromBundle type and encoding

use std::collections::HashMap;

use alloy::primitives::{Address, B256};
use alloy_primitives::I256;
use pade_macro::{PadeDecode, PadeEncode};
use serde::{Deserialize, Serialize};

use crate::{
    contract_payloads::{
        Asset, Pair,
        angstrom::{TopOfBlockOrder, UserOrder},
        rewards::PoolUpdate
    },
    matching::Ray,
    testnet::TestnetStateOverrides
};

#[derive(
    Debug, PadeEncode, PadeDecode, Clone, PartialEq, Serialize, Deserialize, Eq, PartialOrd, Ord,
)]
pub struct AngstromBundle {
    pub assets:              Vec<Asset>,
    pub pairs:               Vec<Pair>,
    pub pool_updates:        Vec<PoolUpdate>,
    pub top_of_block_orders: Vec<TopOfBlockOrder>,
    pub user_orders:         Vec<UserOrder>
}

impl AngstromBundle {
    pub fn new(
        assets: Vec<Asset>,
        pairs: Vec<Pair>,
        pool_updates: Vec<PoolUpdate>,
        top_of_block_orders: Vec<TopOfBlockOrder>,
        user_orders: Vec<UserOrder>
    ) -> Self {
        Self { assets, pairs, pool_updates, top_of_block_orders, user_orders }
    }

    pub fn has_book(&self) -> bool {
        !self.user_orders.is_empty()
    }

    pub fn get_prices_per_pair(&self) -> &[Pair] {
        &self.pairs
    }

    pub fn crude_gas_estimation(&self) -> u64 {
        use itertools::Itertools;
        const BASE_GAS_FOR_POOL: usize = 350_000;
        const BASE_EST_FOR_USER: usize = 90_000;

        let pool_swap_cnt = self
            .pool_updates
            .iter()
            .filter(|update| update.swap_in_quantity != 0)
            .unique_by(|f| f.pair_index)
            .count();

        ((self.top_of_block_orders.len() + self.user_orders.len()) * BASE_EST_FOR_USER) as u64
            + (pool_swap_cnt * BASE_GAS_FOR_POOL) as u64
    }

    pub fn fetch_needed_overrides(&self, block_number: u64) -> TestnetStateOverrides {
        let mut approvals: HashMap<Address, HashMap<Address, u128>> = HashMap::new();
        let mut balances: HashMap<Address, HashMap<Address, u128>> = HashMap::new();
        let mut angstrom_balances: HashMap<Address, HashMap<Address, u128>> = HashMap::new();

        // user orders
        self.user_orders.iter().for_each(|order| {
            let token = if order.zero_for_one {
                // token0
                self.assets[self.pairs[order.pair_index as usize].index0 as usize].addr
            } else {
                self.assets[self.pairs[order.pair_index as usize].index1 as usize].addr
            };

            // need to recover sender from signature
            let hash = order.signing_hash(&self.pairs, &self.assets, block_number);
            let address = order.signature.recover_signer(hash);

            // Grab the price because we need it most of the time
            let price = Ray::from(self.pairs[order.pair_index as usize].price_1over0);
            let qty = match (order.zero_for_one, order.exact_in) {
                // Zero for one, exact in -> quantity on the order (t0).  Extra fee is deducted from
                // this
                (true, true) => order.order_quantities.fetch_max_amount(),
                // Zero for one, exact out -> quantity needed to produce output amount (t0) + extra
                // fee (t0)
                (true, false) => {
                    price.inverse_quantity(order.order_quantities.fetch_max_amount(), true)
                        + order.extra_fee_asset0
                }
                // One for zero, exact in -> quantity on the order (t1) and fee is taken from the
                // ouptut
                (false, true) => order.order_quantities.fetch_max_amount(),
                // One for zero, exact out -> quantity needed to produce the output amount + fee
                // (t1)
                (false, false) => price.quantity(
                    order.order_quantities.fetch_max_amount() + order.extra_fee_asset0,
                    true
                )
            };

            if order.use_internal {
                // For internal balance orders, add to angstrom_balances
                angstrom_balances
                    .entry(token)
                    .or_default()
                    .entry(address)
                    .and_modify(|q| {
                        *q = q.saturating_add(qty);
                    })
                    .or_insert(qty);
            } else {
                // For external balance orders, add to approvals and balances
                approvals
                    .entry(token)
                    .or_default()
                    .entry(address)
                    .and_modify(|q| {
                        *q = q.saturating_add(qty);
                    })
                    .or_insert(qty);
                balances
                    .entry(token)
                    .or_default()
                    .entry(address)
                    .and_modify(|q| {
                        *q = q.saturating_add(qty);
                    })
                    .or_insert(qty);
            }
        });

        // tob
        self.top_of_block_orders.iter().for_each(|order| {
            let token = if order.zero_for_1 {
                // token0
                self.assets[self.pairs[order.pairs_index as usize].index0 as usize].addr
            } else {
                self.assets[self.pairs[order.pairs_index as usize].index1 as usize].addr
            };

            // need to recover sender from signature
            let hash = order.signing_hash(&self.pairs, &self.assets, block_number);
            let address = order.signature.recover_signer(hash);

            let mut qty = order.quantity_in;
            if order.zero_for_1 {
                qty += order.gas_used_asset_0;
            }

            if order.use_internal {
                // For internal balance orders, add to angstrom_balances
                angstrom_balances
                    .entry(token)
                    .or_default()
                    .entry(address)
                    .and_modify(|q| {
                        *q = q.saturating_add(qty);
                    })
                    .or_insert(qty);
            } else {
                // For external balance orders, add to approvals and balances
                approvals
                    .entry(token)
                    .or_default()
                    .entry(address)
                    .and_modify(|q| {
                        *q = q.saturating_add(qty);
                    })
                    .or_insert(qty);
                balances
                    .entry(token)
                    .or_default()
                    .entry(address)
                    .and_modify(|q| {
                        *q = q.saturating_add(qty);
                    })
                    .or_insert(qty);
            }
        });

        TestnetStateOverrides { approvals, balances, angstrom_balances }
    }

    pub fn assert_book_matches(&self) {
        use alloy::primitives::U256;

        let map = self
            .user_orders
            .iter()
            .fold(HashMap::<Address, I256>::new(), |mut acc, user| {
                let pair = &self.pairs[user.pair_index as usize];
                let asset_in = &self.assets
                    [if user.zero_for_one { pair.index0 } else { pair.index1 } as usize];
                let asset_out = &self.assets
                    [if user.zero_for_one { pair.index1 } else { pair.index0 } as usize];

                let price = Ray::from(user.min_price);
                // if we are exact in, then we can attribute amoutn
                let amount_in = if user.exact_in {
                    U256::from(user.order_quantities.fetch_max_amount())
                } else {
                    price.mul_quantity(U256::from(user.order_quantities.fetch_max_amount()))
                };

                let amount_out = if user.exact_in {
                    price.mul_quantity(U256::from(user.order_quantities.fetch_max_amount()))
                } else {
                    U256::from(user.order_quantities.fetch_max_amount())
                };

                *acc.entry(asset_in.addr).or_default() += I256::from_raw(amount_in);
                *acc.entry(asset_out.addr).or_default() -= I256::from_raw(amount_out);

                acc
            });

        for (address, delta) in map {
            if !delta.is_zero() {
                tracing::error!(?address, ?delta, "user orders don't cancel out");
            } else {
                tracing::info!(?address, "solid delta");
            }
        }
    }

    pub fn get_accounts(&self, block_number: u64) -> impl Iterator<Item = Address> + '_ {
        self.top_of_block_orders
            .iter()
            .map(move |order| order.user_address(&self.pairs, &self.assets, block_number))
            .chain(
                self.user_orders.iter().map(move |order| {
                    order.recover_signer(&self.pairs, &self.assets, block_number)
                })
            )
    }

    /// the block number is the block that this bundle was executed at.
    pub fn get_order_hashes(&self, block_number: u64) -> impl Iterator<Item = B256> + '_ {
        self.top_of_block_orders
            .iter()
            .map(move |order| order.order_hash(&self.pairs, &self.assets, block_number))
            .chain(
                self.user_orders
                    .iter()
                    .map(move |order| order.order_hash(&self.pairs, &self.assets, block_number))
            )
    }
}

#[cfg(test)]
mod test {
    use super::AngstromBundle;

    #[test]
    fn can_be_constructed() {
        let _result = AngstromBundle::new(vec![], vec![], vec![], vec![], vec![]);
    }

    #[test]
    fn decode_tob_angstrom_bundle() {
        let bundle: [u8; 376] = [
            0, 0, 136, 122, 185, 133, 215, 244, 70, 250, 54, 98, 245, 212, 171, 94, 242, 10, 107,
            160, 94, 237, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 192, 42,
            170, 57, 178, 35, 254, 141, 10, 14, 92, 79, 39, 234, 217, 8, 60, 117, 108, 194, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 237, 67, 85, 63, 95, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 237, 67, 85, 63, 95, 0, 0, 38, 0, 0, 0, 1, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 1, 0, 0, 35, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 237, 67, 85, 63, 95, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 152, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 3, 183, 17, 221, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 237, 67, 85, 63, 95, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 12, 193, 120, 139, 238, 5, 82, 51, 29, 109, 124, 113, 245, 142, 31, 6, 216,
            47, 227, 99, 27, 110, 150, 112, 234, 129, 56, 107, 225, 163, 117, 76, 121, 246, 253,
            249, 39, 68, 131, 150, 103, 127, 217, 176, 52, 185, 222, 70, 255, 251, 186, 8, 243,
            112, 12, 12, 247, 87, 89, 190, 161, 56, 9, 90, 204, 75, 252, 28, 228, 93, 15, 115, 133,
            106, 184, 0, 241, 21, 160, 212, 52, 123, 21, 16, 129, 0, 0, 0
        ];

        let slice = &mut bundle.as_slice();

        let mut bundle: AngstromBundle = pade::PadeDecode::pade_decode(slice, None).unwrap();
        println!("{bundle:?}");
        let tob = bundle.top_of_block_orders.remove(0);
        println!("{tob:?}");
    }

    #[test]
    fn decode_user_angstrom_bundle() {
        let bundle: [u8; 373] = [
            0, 0, 136, 57, 251, 60, 242, 199, 91, 76, 34, 70, 86, 22, 254, 22, 128, 255, 34, 164,
            166, 244, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 204, 100, 109, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 204, 100, 109,
            192, 42, 170, 57, 178, 35, 254, 141, 10, 14, 92, 79, 39, 234, 217, 8, 60, 117, 108,
            194, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 64, 15, 29, 48, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 64, 15, 29, 48, 25, 0, 0, 38, 0, 0,
            0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 16, 67, 96, 206,
            21, 193, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 184, 168, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 16, 67, 96, 206, 21, 193, 48, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 3, 204, 100, 109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 204, 100, 109, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 204, 100, 109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            3, 204, 100, 109, 27, 173, 77, 129, 8, 3, 181, 255, 66, 55, 66, 206, 216, 73, 59, 189,
            66, 160, 50, 207, 190, 202, 63, 115, 71, 92, 14, 98, 123, 109, 168, 226, 241, 91, 144,
            45, 255, 160, 52, 65, 145, 173, 31, 90, 90, 206, 232, 240, 156, 123, 216, 158, 62, 155,
            36, 55, 255, 111, 67, 204, 109, 84, 52, 115, 11
        ];
        let slice = &mut bundle.as_slice();

        let mut bundle: AngstromBundle = pade::PadeDecode::pade_decode(slice, None).unwrap();
        println!("{bundle:?}");
        let user = bundle.user_orders.remove(0);
        println!("{user:?}");
    }
}
