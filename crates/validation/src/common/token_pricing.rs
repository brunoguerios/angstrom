use std::{
    collections::{HashMap, VecDeque},
    sync::Arc
};

use alloy::{
    primitives::{Address, U256, address},
    providers::Provider
};
use amms::SyncedPools;
use angstrom_types::{
    matching::SqrtPriceX96, orders::UpdatedGas, pair_with_price::PairsWithPrice, primitive::PoolId,
    sol_bindings::Ray
};
use futures::StreamExt;
use tracing::warn;
use uniswap_v4::uniswap::pool_data_loader::PoolDataLoader;

use crate::order::sim::{
    BOOK_GAS, BOOK_GAS_INTERNAL, SWITCH_WEI, TOB_GAS_INTERNAL_NORMAL, TOB_GAS_INTERNAL_SUB,
    TOB_GAS_NORMAL, TOB_GAS_SUB
};

const BLOCKS_TO_AVG_PRICE: u64 = 15;
pub const WETH_ADDRESS: Address = address!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");

// crazy that this is a thing
#[allow(clippy::too_long_first_doc_paragraph)]
/// The token price generator gives us the avg instantaneous price of the last 5
/// blocks of the underlying V4 pool. This is then used in order to convert the
/// gas used from eth to token0 of the pool the user is swapping over.
/// In the case of NON direct eth pairs. we assume that any token liquid enough
/// to trade on angstrom not with eth will always have a eth pair 1 hop away.
/// this allows for a simple lookup.
#[derive(Clone)]
pub struct TokenPriceGenerator {
    synced_pools:        SyncedPools,
    prev_prices:         HashMap<PoolId, VecDeque<PairsWithPrice>>,
    pair_to_pool:        HashMap<(Address, Address), PoolId>,
    /// the token that is the wrapped version of the gas token on the given
    /// chain
    base_gas_token:      Address,
    blocks_to_avg_price: u64,
    pub base_wei:        u128,
    current_block:       u64
}

impl TokenPriceGenerator {
    pub fn from_snapshot(
        synced_pools: SyncedPools,
        prev_prices: HashMap<PoolId, VecDeque<PairsWithPrice>>,
        base_gas_token: Address,
        base_wei: u128
    ) -> Self {
        let pair_to_pool = match &synced_pools {
            SyncedPools::Uniswap(uniswap_pools) => uniswap_pools
                .iter()
                .map(|p| {
                    let key = p.key();
                    let pool = p.value().read().unwrap();
                    ((pool.token0, pool.token1), *key)
                })
                .collect::<HashMap<(Address, Address), PoolId>>(),
            SyncedPools::Balancer(_) => HashMap::default()
        };

        let mut avg = 0;
        let remapped_prices = prev_prices
            .into_iter()
            .map(|(k, v)| {
                avg = v.len();
                let new_k = v
                    .front()
                    .and_then(|i| pair_to_pool.get(&(i.token0, i.token1)).copied())
                    .unwrap_or(k);
                (new_k, v)
            })
            .collect();

        Self {
            current_block: 0,
            synced_pools,
            prev_prices: remapped_prices,
            pair_to_pool,
            base_gas_token,
            blocks_to_avg_price: avg as u64,
            base_wei
        }
    }

    pub fn to_snapshot(&self) -> (HashMap<PoolId, VecDeque<PairsWithPrice>>, u128) {
        (self.prev_prices.clone(), self.base_wei)
    }

    /// is a bit of a pain as we need todo a look-back in-order to grab last 5
    /// blocks.
    pub async fn new<P: Provider>(
        provider: Arc<P>,
        current_block: u64,
        synced_pools: SyncedPools,
        base_gas_token: Address,
        blocks_to_avg_price_override: Option<u64>
    ) -> eyre::Result<Self> {
        let new_gas_wei = provider.get_gas_price().await.unwrap_or_default();
        let blocks_to_avg_price = blocks_to_avg_price_override.unwrap_or(BLOCKS_TO_AVG_PRICE);

        match &synced_pools {
            SyncedPools::Uniswap(uni) => {
                let mut pair_to_pool = HashMap::default();
                for id in uni.iter() {
                    let key = id.key();
                    let pool = id.value();
                    let pool = pool.read().unwrap();
                    pair_to_pool.insert((pool.token0, pool.token1), *key);
                    pair_to_pool.insert((pool.token1, pool.token0), *key);
                }

                // for each pool, we want to load the last 5 blocks and get the sqrt_price_96
                // and then convert it into the price of the underlying pool
                let pools = futures::stream::iter(uni.iter())
                    .map(|id| {
                        let pool_key = *id.key();
                        let pool = id.value().clone();
                        let provider = provider.clone();

                        async move {
                            let mut queue = VecDeque::new();
                            // scoping
                            let data_loader = {
                                let pool_read = pool.read().unwrap();
                                let data_loader = pool_read.data_loader();
                                drop(pool_read);
                                data_loader
                            };

                            for block_number in
                                current_block.saturating_sub(blocks_to_avg_price)..current_block
                            {
                                tracing::debug!(
                                    block_number,
                                    current_block,
                                    ?pool_key,
                                    "loading pool"
                                );
                                let pool_data = data_loader
                                    .load_pool_data(Some(block_number), provider.clone())
                                    .await
                                    .expect(
                                        "failed to load historical price for token price \
                                         conversion"
                                    );
                                tracing::debug!(?pool_data, "Loaded data");

                                // price as ray
                                let price = pool_data.get_raw_price();

                                queue.push_back(PairsWithPrice {
                                    token0:         pool_data.tokenA,
                                    token1:         pool_data.tokenB,
                                    price_1_over_0: price
                                });
                            }

                            (pool_key, queue)
                        }
                    })
                    .fold(HashMap::default(), |mut acc, x| async {
                        let (key, prices) = x.await;
                        acc.insert(key, prices);
                        acc
                    })
                    .await;

                Ok(Self {
                    current_block,
                    prev_prices: pools,
                    base_gas_token,
                    pair_to_pool,
                    blocks_to_avg_price,
                    synced_pools,
                    base_wei: new_gas_wei
                })
            }
            SyncedPools::Balancer(_balancer_pools) => {
                // For Balancer, we start with empty price history
                // Prices will be populated via apply_update from PairsWithPrice stream
                let prev_prices = HashMap::default();
                let pair_to_pool = HashMap::default();

                Ok(Self {
                    current_block,
                    prev_prices,
                    base_gas_token,
                    pair_to_pool,
                    blocks_to_avg_price,
                    synced_pools,
                    base_wei: new_gas_wei
                })
            }
        }
    }

    pub fn current_block(&self) -> u64 {
        self.current_block
    }

    pub fn generate_gas_updates(&self) -> Vec<UpdatedGas> {
        self.pair_to_pool
            .iter()
            .filter_map(|(&(token0, token1), pool_id)| {
                let price = self.get_conversion_rate(token0, token1)?;

                let (internal, normal) = if self.base_wei > SWITCH_WEI {
                    (TOB_GAS_INTERNAL_NORMAL, TOB_GAS_NORMAL)
                } else {
                    (TOB_GAS_INTERNAL_SUB, TOB_GAS_SUB)
                };

                let book_internal = price.inverse_quantity(BOOK_GAS_INTERNAL as u128, false);
                let book_external = price.inverse_quantity(BOOK_GAS as u128, false);
                let tob_internal = price.inverse_quantity(internal as u128, false);
                let tob_external = price.inverse_quantity(normal as u128, false);

                Some(UpdatedGas {
                    pool_id:           *pool_id,
                    gas_internal_book: book_internal,
                    gas_internal_tob:  tob_internal,
                    gas_external_tob:  tob_external,
                    gas_external_book: book_external
                })
            })
            .collect()
    }

    pub fn apply_update(
        &mut self,
        new_block: u64,
        new_gas_wei: u128,
        updates: Vec<PairsWithPrice>
    ) {
        // we will duplicate same price if no update for pool.
        let mut updated_pool_keys = Vec::new();
        self.base_wei = new_gas_wei;
        self.current_block = new_block;

        for mut pool_update in updates {
            let pool_key = if let Some(existing) = self
                .pair_to_pool
                .get(&(pool_update.token0, pool_update.token1))
            {
                *existing
            } else {
                let resolved = match &self.synced_pools {
                    SyncedPools::Uniswap(uniswap_pools) => uniswap_pools.iter().find_map(|val| {
                        let (pool_key, pool) = val.pair();
                        let tokens = {
                            let pool_read = pool.read().unwrap();
                            (pool_read.token0, pool_read.token1)
                        };
                        (tokens.0 == pool_update.token0 && tokens.1 == pool_update.token1)
                            .then_some(*pool_key)
                    }),
                    SyncedPools::Balancer(pools) => {
                        let _ = pools.iter();
                        None
                    }
                };

                let Some(pk) = resolved else {
                    continue;
                };

                self.pair_to_pool
                    .insert((pool_update.token0, pool_update.token1), pk);
                self.pair_to_pool
                    .insert((pool_update.token1, pool_update.token0), pk);
                pk
            };

            updated_pool_keys.push(pool_key);
            let prev_prices = self
                .prev_prices
                .entry(pool_key)
                .or_insert_with(VecDeque::new);

            pool_update.replace_price_if_empty(|| match &self.synced_pools {
                SyncedPools::Uniswap(uniswap_pools) => uniswap_pools
                    .get(&pool_key)
                    .map(|pool| Ray::from(SqrtPriceX96::from(pool.read().unwrap().sqrt_price_x96)))
                    .or_else(|| prev_prices.back().map(|price| price.price_1_over_0))
                    .unwrap_or_default(),
                SyncedPools::Balancer(pools) => {
                    let _ = pools.iter();
                    prev_prices
                        .back()
                        .map(|price| price.price_1_over_0)
                        .unwrap_or_else(|| Ray::scale_to_ray(U256::from(1)))
                }
            });

            if !pool_update.price_1_over_0.is_zero() {
                prev_prices.push_back(pool_update);
            }

            // only pop front if we extend
            if prev_prices.len() as u64 > self.blocks_to_avg_price {
                prev_prices.pop_front();
            }
        }

        self.prev_prices
            .iter_mut()
            .filter(|(k, _)| !updated_pool_keys.contains(k))
            .for_each(|(_, queue)| {
                let Some(last) = queue.back() else { return };
                let new_back = *last;

                queue.push_back(new_back);

                // only pop front if we extend
                if queue.len() as u64 > self.blocks_to_avg_price {
                    queue.pop_front();
                }
            });

        // given that we have added new pools that we got updates for,
        // we also want to make sure to add new pools that haven't been updated
        // as trading is not eligible since there's no price. Kinda a catch 22
        let remove_keys = match &self.synced_pools {
            SyncedPools::Uniswap(uniswap_pools) => self
                .prev_prices
                .keys()
                .copied()
                .filter(|f| !uniswap_pools.contains_key(f))
                .collect::<Vec<_>>(),
            SyncedPools::Balancer(pools) => {
                let _ = pools.iter();
                Vec::new()
            }
        };

        for key in remove_keys {
            self.prev_prices.remove(&key);
            self.pair_to_pool.retain(|_, v| *v != key);
        }

        if let SyncedPools::Uniswap(uniswap_pools) = &self.synced_pools {
            uniswap_pools.iter().for_each(|entry| {
                let (key, pool) = entry.pair();
                if self.prev_prices.contains_key(key) {
                    return;
                }
                let pool_r = pool.read().unwrap();
                let price: Ray = SqrtPriceX96::from(pool_r.sqrt_price_x96).into();

                let mut queue = VecDeque::new();
                queue.push_back(PairsWithPrice {
                    token0:         pool_r.token0,
                    token1:         pool_r.token1,
                    price_1_over_0: price
                });
                self.prev_prices.insert(*key, queue);
                self.pair_to_pool
                    .insert((pool_r.token0, pool_r.token1), *key);
            });
        }
    }

    pub fn new_pool_update(
        &mut self,
        pool_id: PoolId,
        token_0: Address,
        token_1: Address,
        prev_prices: VecDeque<PairsWithPrice>
    ) {
        self.pair_to_pool.insert((token_0, token_1), pool_id);
        self.pair_to_pool.insert((token_1, token_0), pool_id);
        self.prev_prices.insert(pool_id, prev_prices);
    }

    /// returns they conversion ratio in the format t1 / t0
    /// this is because rewards are in t0 and the only time
    /// we will need to convert these is if they are a bid
    pub fn conversion_rate_of_pair(&self, token_0: Address, token_1: Address) -> Option<Ray> {
        // because we know all pairs MUST have a conversion with the gas token,
        // we can source a routed swap thorugh it

        // returns GAS /t0
        let pair_1 = self.get_conversion_rate(token_0, token_1)?;
        // returns GAS / t1
        let pair_2 = self.get_conversion_rate(token_1, token_0)?;

        Some(pair_2.inv_ray().mul_ray(pair_1))
    }

    /// NOTE: assumes tokens are properly sorted.
    /// the previous prices are stored in RAY (1e27).
    /// returns price in GAS / t0
    pub fn get_eth_conversion_price(
        &self,
        token_0: Address,
        token_1: Address,
        gas_units: u64
    ) -> Option<u128> {
        let mut wei = if self.base_wei == 0 { 1e9 as u128 } else { self.base_wei };
        wei *= gas_units as u128;

        // wei in gas units

        let rate = self
            .get_conversion_rate(token_0, token_1)
            .map(|val| val.inv_ray().mul_wad(wei, 27).0.to::<u128>());
        tracing::trace!(?token_0, ?token_1, ?rate, "Getting conversion rate");
        rate
    }

    fn get_conversion_rate(&self, token_0: Address, token_1: Address) -> Option<Ray> {
        // if token zero is weth, then we mul by 1
        if token_0 == self.base_gas_token {
            return Some(Ray::scale_to_ray(U256::from(1)));
        }
        // should only be called if token_1 is weth or needs multi-hop as otherwise
        // conversion factor will be 1-1
        if token_1 == self.base_gas_token {
            // if so, just pull the price
            let pool_key = self.pair_to_pool.get(&(token_0, token_1))?;

            let prices = self.prev_prices.get(pool_key)?;
            let size = prices.len() as u64;

            if self.blocks_to_avg_price > 0 && size != self.blocks_to_avg_price {
                warn!(?size,?self.blocks_to_avg_price,"size of loaded blocks doesn't match the value we set");
            }

            // if t1 == gas, then t0am  * t1 / t0 = am t1
            return Some(prices.iter().map(|p| p.price_1_over_0).sum::<Ray>() / U256::from(size));
        }

        // need to pass through a pair.
        let (first_flip, token_0_hop1, token_1_hop1) = if token_0 < self.base_gas_token {
            (true, token_0, self.base_gas_token)
        } else {
            (false, self.base_gas_token, token_0)
        };

        let (second_flip, token_0_hop2, token_1_hop2) = if token_1 < self.base_gas_token {
            (true, token_1, self.base_gas_token)
        } else {
            (false, self.base_gas_token, token_1)
        };

        // check token_0 first for a weth pair. otherwise, check token_1.
        if let Some(key) = self.pair_to_pool.get(&(token_0_hop1, token_1_hop1)) {
            // there is a hop from token_0 to weth
            let prices = self.prev_prices.get(key)?;
            let size = prices.len() as u64;

            if self.blocks_to_avg_price > 0 && size != self.blocks_to_avg_price {
                warn!("size of loaded blocks doesn't match the value we set");
            }

            // if we have this, this means that (p0, p1) has (p0, gas) pair.
            // because of this, we can just convert directly on this.
            // if first_flip = true, means token0 < gas, were price is gas / token0.
            // thus gas_am t0 * price = gas.

            Some(
                prices
                    .iter()
                    .map(|price| {
                        // if true, means gas is token zero
                        if first_flip {
                            price.price_1_over_0
                        } else {
                            price.price_1_over_0.inv_ray()
                        }
                    })
                    .sum::<Ray>()
                    / U256::from(size)
            )
        } else if let Some(key) = self.pair_to_pool.get(&(token_0_hop2, token_1_hop2)) {
            // because we are going through token1 here and we want token zero, we need to
            // do some extra math
            let default_pool_key = self.pair_to_pool.get(&(token_0, token_1))?;

            let prices = self.prev_prices.get(default_pool_key)?;
            let size = prices.len() as u64;

            if self.blocks_to_avg_price > 0 && size != self.blocks_to_avg_price {
                warn!("size of loaded blocks doesn't match the value we set");
            }
            // price as t1 / t0
            let first_hop_price =
                prices.iter().map(|price| price.price_1_over_0).sum::<Ray>() / U256::from(size);

            // grab second hop
            let prices = self.prev_prices.get(key)?;
            let size = prices.len() as u64;

            if self.blocks_to_avg_price > 0 && size != self.blocks_to_avg_price {
                warn!("size of loaded blocks doesn't match the value we set");
            }

            // if flip = true, then  gas / token1, otherwise token1 / gas

            // token1 / WETH
            let second_hop_price = prices
                .iter()
                .map(|price| {
                    // means gas is token1
                    if second_flip { price.price_1_over_0 } else { price.price_1_over_0.inv_ray() }
                })
                .sum::<Ray>()
                / U256::from(size);

            // t1 / t0 *  gas / t1 = gas / t0
            Some(first_hop_price.mul_ray(second_hop_price))
        } else {
            tracing::error!("found a token that doesn't have a 1 hop to WETH");

            return None;
        }
    }

    pub fn pairs_to_pools(&self) -> HashMap<(Address, Address), PoolId> {
        self.pair_to_pool.clone()
    }

    pub fn prev_prices(&self) -> HashMap<PoolId, VecDeque<PairsWithPrice>> {
        self.prev_prices.clone()
    }
}

impl std::fmt::Debug for TokenPriceGenerator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenPriceGenerator")
            .field("prev_prices", &self.prev_prices)
            .field("pair_to_pool", &self.pair_to_pool)
            .field("base_gas_token", &self.base_gas_token)
            .field("blocks_to_avg_price", &self.blocks_to_avg_price)
            .field("base_wei", &self.base_wei)
            .finish()
    }
}

#[cfg(test)]
pub mod test {
    use std::collections::{HashMap, VecDeque};

    use alloy::{
        node_bindings::WEI_IN_ETHER,
        primitives::{Address, FixedBytes, U256}
    };
    use angstrom_types::{pair_with_price::PairsWithPrice, sol_bindings::Ray};
    use revm::primitives::address;

    use super::{BLOCKS_TO_AVG_PRICE, TokenPriceGenerator, WETH_ADDRESS};
    use crate::common::SyncedPools;

    const TOKEN0: Address = address!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");
    const TOKEN1: Address = address!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc3");
    const TOKEN2: Address = address!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc1");
    const TOKEN3: Address = address!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc5");
    const TOKEN4: Address = address!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc0");
    const TOKEN5: Address = address!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc6");

    /// sets up pools with prices for all scenarios
    fn setup() -> TokenPriceGenerator {
        let mut pairs_to_key = HashMap::default();
        // setup pair lookup

        // pair 1 direct NOTE: case is where weth is token1
        pairs_to_key.insert((TOKEN2, TOKEN0), FixedBytes::<32>::with_last_byte(1));

        // pair 2 direct NOTE: case is where weth is token0
        pairs_to_key.insert((TOKEN0, TOKEN1), FixedBytes::<32>::with_last_byte(2));

        // multi-hop where token0 matches
        pairs_to_key.insert((TOKEN2, TOKEN3), FixedBytes::<32>::with_last_byte(3));

        // multi-hop where token1 matches
        pairs_to_key.insert((TOKEN4, TOKEN1), FixedBytes::<32>::with_last_byte(4));

        // setup price conversions
        let mut prices = HashMap::default();

        // assumes both 18 decimal
        let pair1_rate = U256::from(5) * WEI_IN_ETHER;
        let pair = PairsWithPrice {
            token0:         TOKEN2,
            token1:         TOKEN0,
            price_1_over_0: Ray::scale_to_ray(pair1_rate)
        };
        let queue = VecDeque::from([pair; 5]);
        prices.insert(FixedBytes::<32>::with_last_byte(1), queue);

        // assumes token1 is 6 decimals and token 0 is 18 with a conversion rate of 0.2
        // gives us 200000
        let pair2_rate = U256::from(200000);

        let pair = PairsWithPrice {
            token0:         TOKEN0,
            token1:         TOKEN1,
            price_1_over_0: Ray::scale_to_ray(pair2_rate)
        };
        let queue = VecDeque::from([pair; 5]);
        prices.insert(FixedBytes::<32>::with_last_byte(2), queue);

        // simple conversion rate of 2/1 on 18 decimals
        let pair3_rate = U256::from(2) * WEI_IN_ETHER;

        let pair = PairsWithPrice {
            token0:         TOKEN2,
            token1:         TOKEN3,
            price_1_over_0: Ray::scale_to_ray(pair3_rate)
        };
        let queue = VecDeque::from([pair; 5]);
        prices.insert(FixedBytes::<32>::with_last_byte(3), queue);

        // token 1 is 18 decimals, token 0 is 6 with a conversion rate of 1/8
        let pair4_rate = U256::from(1e18) / U256::from(8e6);

        let pair = PairsWithPrice {
            token0:         TOKEN4,
            token1:         TOKEN1,
            price_1_over_0: Ray::scale_to_ray(pair4_rate)
        };

        let queue = VecDeque::from([pair; 5]);
        prices.insert(FixedBytes::<32>::with_last_byte(4), queue);

        TokenPriceGenerator {
            current_block:       0,
            base_wei:            0,
            prev_prices:         prices,
            base_gas_token:      WETH_ADDRESS,
            pair_to_pool:        pairs_to_key,
            blocks_to_avg_price: BLOCKS_TO_AVG_PRICE,
            synced_pools:        SyncedPools::default_uniswap()
        }
    }

    #[test]
    fn test_direct_conversion() {
        let token_conversion = setup();
        let rate = token_conversion
            .get_conversion_rate(TOKEN2, TOKEN0)
            .unwrap();

        let expected_rate = Ray::scale_to_ray(U256::from(5) * WEI_IN_ETHER);
        println!("rate: {rate:?} got: {expected_rate:?}");
        assert_eq!(rate, expected_rate)
    }

    #[test]
    fn test_multi_hop_where_token0_matches() {
        let token_conversion = setup();
        let rate = token_conversion
            .get_conversion_rate(TOKEN2, TOKEN3)
            .unwrap();

        // t3 / t2 = pair 1 = 2e45
        // weth / t2 = pair 2 =  5e45
        //
        //  given t2 < weth
        //
        //  conversion = price =  weth / t2  == 5e45

        let expected_rate = Ray::scale_to_ray(U256::from(5e18));
        println!("rate: {rate:?} got: {expected_rate:?}");
        assert_eq!(rate, expected_rate)
    }

    #[test]
    fn test_multi_hop_where_token1_matches() {
        // 625000000000000000000000000000000

        let token_conversion = setup();
        let rate = token_conversion
            .get_conversion_rate(TOKEN4, TOKEN1)
            .unwrap();

        // we have a  t1 / t4 pair and a t1 / weth pair
        //
        // t1 / t4 = 125000000000
        //
        // t1 / weth = 200000000000000000000000000000000
        //
        // what we want is weth / t4
        //
        //  if weth t0 = true, then we flip
        //  so then t1 /t4 * weth / t1 =  weth / t4
        //
        // 125000000000 * (1e54 / 200000000000000000000000000000000) =
        // 625000000000000000000000000000000

        // hop 1 rate
        // assumes token1 is 6 decimals and token 0 is 18 with a conversion rate of 0.2
        // gives us 200000 TOKEN1 / WETH
        //
        // hop 2 rate
        // token 1 is 18 decimals, token 0 is 6 with a conversion rate of 1/8
        // let pair4_rate = U256::from(1e18) / U256::from(8e6);
        //
        // gives us 0.2 * 0.8 = 0.16;
        let expected_rate = Ray::scale_to_ray(U256::from(625000));
        assert_eq!(rate, expected_rate)
    }

    #[test]
    fn test_weth_direct_cases() {
        let token_conversion = setup();

        // WETH as token0 should return 1
        let rate = token_conversion
            .get_conversion_rate(WETH_ADDRESS, TOKEN1)
            .unwrap();
        assert_eq!(rate, Ray::scale_to_ray(U256::from(1)));

        // 5 weth .inv
        let rate = token_conversion
            .get_conversion_rate(TOKEN2, WETH_ADDRESS)
            .unwrap();

        assert_eq!(rate, Ray::scale_to_ray(U256::from(5) * WEI_IN_ETHER));
    }

    #[test]
    fn test_missing_pool() {
        let token_conversion = setup();

        // Try to get price for non-existent pool
        let rate = token_conversion.get_conversion_rate(
            address!("1111111111111111111111111111111111111111"),
            address!("2222222222222222222222222222222222222222")
        );
        assert!(rate.is_none(), "Should return None for missing pool");
    }

    #[test]
    fn test_insufficient_price_data() {
        // 1000000000000000000000000000000000000000000000
        let mut token_conversion = setup();

        // Create a pool with insufficient price data
        let pool_id = FixedBytes::<32>::with_last_byte(6);
        token_conversion
            .pair_to_pool
            .insert((TOKEN5, WETH_ADDRESS), pool_id);

        let mut queue = VecDeque::new();
        queue.push_back(PairsWithPrice {
            token0:         TOKEN5,
            token1:         WETH_ADDRESS,
            price_1_over_0: Ray::scale_to_ray(U256::from(1) * WEI_IN_ETHER)
        });
        token_conversion.prev_prices.insert(pool_id, queue);

        let rate = token_conversion
            .get_conversion_rate(TOKEN5, WETH_ADDRESS)
            .unwrap();

        assert_eq!(rate, Ray::scale_to_ray(U256::from(1) * WEI_IN_ETHER));
    }

    #[test]
    fn test_empty_pool_gas_price() {
        // Create a TokenPriceGenerator with no pools
        let token_conversion = TokenPriceGenerator {
            current_block:       0,
            base_wei:            0,
            prev_prices:         HashMap::default(),
            base_gas_token:      WETH_ADDRESS,
            pair_to_pool:        HashMap::default(),
            blocks_to_avg_price: BLOCKS_TO_AVG_PRICE,
            synced_pools:        SyncedPools::default_uniswap()
        };

        // Test direct WETH case (should still work)
        let rate = token_conversion.get_conversion_rate(WETH_ADDRESS, TOKEN1);
        assert_eq!(
            rate,
            Some(Ray::scale_to_ray(U256::from(1))),
            "WETH as token0 should return 1 even with no pools"
        );

        // Test non-WETH tokens (should return None)
        let random_token1 = address!("1111111111111111111111111111111111111111");
        let random_token2 = address!("2222222222222222222222222222222222222222");

        let rate = token_conversion.get_conversion_rate(random_token1, random_token2);
        assert!(rate.is_none(), "Should return None when no pools exist");

        let rate = token_conversion.get_conversion_rate(random_token1, WETH_ADDRESS);
        assert!(rate.is_none(), "Should return None for direct WETH pair when no pools exist");
    }

    /// Custom setup with WETH as both token0 and token1 in different pools
    /// WETH/TOKEN price = 3700 (when WETH is token0)
    /// TOKEN/WETH price = 0.0002702702702702703 (when WETH is token1)
    fn setup_weth_pairs() -> TokenPriceGenerator {
        let mut pairs_to_key = HashMap::default();
        let mut prices = HashMap::default();

        // Test token that pairs with WETH
        const TEST_TOKEN: Address = address!("a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"); // USDC address as example

        // Pool 1: WETH as token0, TEST_TOKEN as token1
        // Price represents TEST_TOKEN/WETH = 3700 (meaning 1 WETH = 3700 TEST_TOKEN)
        pairs_to_key.insert((WETH_ADDRESS, TEST_TOKEN), FixedBytes::<32>::with_last_byte(1));

        // token 1 usdc, token 0 weth
        let price = 2986678470.0 / 803080000000000000.0;

        let weth_as_t0_price = Ray::from(price);
        let pair1 = PairsWithPrice {
            token0:         WETH_ADDRESS,
            token1:         TEST_TOKEN,
            price_1_over_0: weth_as_t0_price
        };
        let queue1 = VecDeque::from([pair1; 5]);
        prices.insert(FixedBytes::<32>::with_last_byte(1), queue1);

        // Pool 2: TEST_TOKEN as token0, WETH as token1
        // Price represents WETH/TEST_TOKEN = 0.0002702702702702703 (meaning 1
        // TEST_TOKEN = 0.00027... WETH)
        pairs_to_key.insert((TEST_TOKEN, WETH_ADDRESS), FixedBytes::<32>::with_last_byte(2));

        // Convert the decimal to a U256 with proper scaling
        // 0.0002702702702702703 * 10^18 = 270270270270270.3

        let price = 803080000000000000.0 / 2986678470.0;
        let weth_as_t1_price = Ray::from(price);
        let pair2 = PairsWithPrice {
            token0:         TEST_TOKEN,
            token1:         WETH_ADDRESS,
            price_1_over_0: weth_as_t1_price
        };
        let queue2 = VecDeque::from([pair2; 5]);
        prices.insert(FixedBytes::<32>::with_last_byte(2), queue2);

        TokenPriceGenerator {
            current_block:       0,
            base_wei:            0,
            prev_prices:         prices,
            base_gas_token:      WETH_ADDRESS,
            pair_to_pool:        pairs_to_key,
            blocks_to_avg_price: BLOCKS_TO_AVG_PRICE,
            synced_pools:        SyncedPools::default_uniswap()
        }
    }

    #[test]
    fn test_weth_pair_gas_behavior() {
        // Test the specific WETH pair setup with realistic prices
        let mut token_conversion = setup_weth_pairs();
        const TEST_TOKEN: Address = address!("a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48");

        // Test with different gas prices
        let gas_prices = vec![
            100 * 1e9 as u128,   // 100 gwei
            50 * 1e9 as u128,    // 50 gwei
            10 * 1e9 as u128,    // 10 gwei
            (1e9 as u128),       // 10 gwei
            (0.3 * 1e9) as u128, // 10 gwei
        ];

        use crate::order::sim::BOOK_GAS;

        println!("\nTesting WETH as token0 (WETH/TEST_TOKEN pair):");
        let mut weth_t0_results = Vec::new();

        for gas_wei in &gas_prices {
            token_conversion.base_wei = *gas_wei;

            // Get conversion price for WETH/TEST_TOKEN pair
            let book_gas = token_conversion
                .get_eth_conversion_price(WETH_ADDRESS, TEST_TOKEN, BOOK_GAS)
                .expect("Should have conversion price");

            weth_t0_results.push((*gas_wei, book_gas));

            println!(
                "  Gas price: {} gwei -> Book gas in TEST_TOKEN: {}",
                gas_wei / 1e9 as u128,
                book_gas
            );
        }

        println!("\nTesting WETH as token1 (TEST_TOKEN/WETH pair):");
        let mut weth_t1_results = Vec::new();

        for gas_wei in &gas_prices {
            token_conversion.base_wei = *gas_wei;

            // Get conversion price for TEST_TOKEN/WETH pair
            let book_gas = token_conversion
                .get_eth_conversion_price(TEST_TOKEN, WETH_ADDRESS, BOOK_GAS)
                .expect("Should have conversion price");

            weth_t1_results.push((*gas_wei, book_gas));

            println!(
                "  Gas price: {} gwei -> Book gas in TEST_TOKEN: {}",
                gas_wei / 1e9 as u128,
                book_gas
            );
        }

        // Verify decreasing gas prices lead to decreasing token amounts for both pairs
        for i in 1..weth_t0_results.len() {
            let (prev_gas, prev_amount) = weth_t0_results[i - 1];
            let (curr_gas, curr_amount) = weth_t0_results[i];

            assert!(curr_gas < prev_gas);
            assert!(
                curr_amount < prev_amount,
                "WETH as token0: Lower gas should mean lower token amount. {} gwei -> {}, {} gwei \
                 -> {}",
                prev_gas / 1e9 as u128,
                prev_amount,
                curr_gas / 1e9 as u128,
                curr_amount
            );
        }

        for i in 1..weth_t1_results.len() {
            let (prev_gas, prev_amount) = weth_t1_results[i - 1];
            let (curr_gas, curr_amount) = weth_t1_results[i];

            assert!(curr_gas < prev_gas);
            assert!(
                curr_amount < prev_amount,
                "WETH as token1: Lower gas should mean lower token amount. {} gwei -> {}, {} gwei \
                 -> {}",
                prev_gas / 1e9 as u128,
                prev_amount,
                curr_gas / 1e9 as u128,
                curr_amount
            );
        }
    }
}
