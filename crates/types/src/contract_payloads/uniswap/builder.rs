//! Uniswap bundle builder - converts consensus outputs to AngstromBundle

use std::collections::{HashMap, HashSet};

use alloy::primitives::{Address, FixedBytes, I256, U256};
use eyre::Result;
use itertools::Itertools;
use tracing::{Level, debug, trace, warn};

use super::AngstromBundle;
use crate::{
    amm::PoolState,
    consensus::{PreProposal, Proposal},
    contract_payloads::{
        Asset, Pair,
        angstrom::{TopOfBlockOrder, UserOrder},
        asset::builder::{AssetBuilder, AssetBuilderStage},
        rewards::{PoolUpdate, RewardsUpdate}
    },
    matching::{Ray, SqrtPriceX96, get_quantities_at_price},
    orders::{OrderFillState, OrderId, OrderOutcome, OrderSet, PoolSolution},
    primitive::PoolId,
    sol_bindings::{
        RawPoolOrder,
        grouped_orders::{AllOrders, OrderWithStorageData},
        rpc_orders::TopOfBlockOrder as RpcTopOfBlockOrder
    },
    uni_structure::{UniswapPoolState, donation::DonationCalculation}
};

const LP_DONATION_SPLIT: f64 = 0.75;

#[allow(unused)]
#[derive(Debug, Clone, Default)]
pub struct BundleGasDetails {
    /// total gas to execute the bundle on angstrom
    total_gas_cost_wei: u64
}

impl BundleGasDetails {
    pub fn new(total_gas_cost_wei: u64) -> Self {
        Self { total_gas_cost_wei }
    }
}

pub struct UniswapBundleBuilder;

impl UniswapBundleBuilder {
    /// Build AngstromBundle from consensus proposal (Uniswap pools only)
    pub fn from_proposal(
        proposal: &Proposal,
        orders: OrderSet<AllOrders, RpcTopOfBlockOrder>,
        _gas_details: BundleGasDetails,
        pools: &HashMap<PoolId, (Address, Address, UniswapPoolState, u16)>
    ) -> Result<AngstromBundle> {
        trace!("Starting from_proposal");
        let mut top_of_block_orders = Vec::new();
        let mut pool_updates = Vec::new();
        let mut pairs = Vec::new();
        let mut user_orders = Vec::new();
        let mut asset_builder = AssetBuilder::new();

        // Break out our input orders into lists of orders by pool
        let preproposals = proposal.flattened_pre_proposals();
        let orders_by_pool = Self::orders_by_pool_from_pre_proposals(&preproposals, orders);

        // fetch the accumulated amount of gas delegated to the users
        let (total_swaps, _) = Self::fetch_total_orders_and_gas_delegated_to_orders(
            &orders_by_pool,
            &proposal.solutions
        );
        // this should never underflow. if it does. means that there is underlying
        // problem with the gas delegation module

        if total_swaps == 0 {
            return Err(eyre::eyre!("have a total swaps count of 0"));
        }

        // what we need to do is go through and first add all the tokens,
        // then sort them and change the offests before we index all orders
        for solution in proposal.solutions.iter() {
            let Some((t0, t1, ..)) = pools.get(&solution.id) else {
                // This should never happen but let's handle it as gracefully as possible -
                // right now will skip the pool, not produce an error
                warn!(
                    "Skipped a solution as we couldn't find a pool for it: {:?}, {:?}",
                    pools, solution.id
                );
                continue;
            };
            asset_builder.add_or_get_asset(*t0);
            asset_builder.add_or_get_asset(*t1);
        }
        asset_builder.order_assets_properly();

        // fetch gas used
        // Walk through our solutions to add them to the structure
        for solution in proposal.solutions.iter().sorted_unstable_by_key(|k| {
            let Some((t0, t1, ..)) = pools.get(&k.id) else {
                // This should never happen but let's handle it as gracefully as possible -
                // right now will skip the pool, not produce an error
                return 0usize;
            };
            let t0_idx = asset_builder.add_or_get_asset(*t0);
            let t1_idx = asset_builder.add_or_get_asset(*t1);
            (t0_idx << 16) | t1_idx
        }) {
            // Get the information for the pool or skip this solution if we can't find a
            // pool for it
            let Some((t0, t1, snapshot, store_index)) = pools.get(&solution.id) else {
                // This should never happen but let's handle it as gracefully as possible -
                // right now will skip the pool, not produce an error
                warn!(
                    "Skipped a solution as we couldn't find a pool for it: {:?}, {:?}",
                    pools, solution.id
                );
                continue;
            };

            // Call our processing function with a fixed amount of shared gas
            // Wrap the Uniswap snapshot in the PoolState enum
            let pool_state = PoolState::Uniswap(snapshot.clone());
            Self::process_solution(
                &mut pairs,
                &mut asset_builder,
                &mut user_orders,
                &orders_by_pool,
                &mut top_of_block_orders,
                &mut pool_updates,
                solution,
                &pool_state,
                *t0,
                *t1,
                *store_index,
                Some(U256::ZERO)
            )?;
        }

        Ok(AngstromBundle::new(
            asset_builder.get_asset_array(),
            pairs,
            pool_updates,
            top_of_block_orders,
            user_orders
        ))
    }

    /// builds a bundle where orders are set to max allocated gas to ensure a
    /// fully passing env. with the gas details from the response, can
    /// properly allocate order gas amounts.
    pub fn for_gas_finalization(
        limit: Vec<OrderWithStorageData<AllOrders>>,
        solutions: Vec<PoolSolution>,
        pools: &HashMap<PoolId, (Address, Address, UniswapPoolState, u16)>
    ) -> Result<AngstromBundle> {
        let mut top_of_block_orders = Vec::new();
        let mut pool_updates = Vec::new();
        let mut pairs = Vec::new();
        let mut user_orders = Vec::new();
        let mut asset_builder = AssetBuilder::new();

        let orders_by_pool: HashMap<
            alloy_primitives::FixedBytes<32>,
            HashSet<OrderWithStorageData<AllOrders>>
        > = limit.iter().fold(HashMap::new(), |mut acc, x| {
            acc.entry(x.pool_id).or_default().insert(x.clone());
            acc
        });

        // what we need to do is go through and first add all the tokens,
        // then sort them and change the offests before we index all orders
        for solution in solutions.iter() {
            let Some((t0, t1, ..)) = pools.get(&solution.id) else {
                // This should never happen but let's handle it as gracefully as possible -
                // right now will skip the pool, not produce an error
                warn!(
                    "Skipped a solution as we couldn't find a pool for it: {:?}, {:?}",
                    pools, solution.id
                );
                continue;
            };
            asset_builder.add_or_get_asset(*t0);
            asset_builder.add_or_get_asset(*t1);
        }
        asset_builder.order_assets_properly();

        // Walk through our solutions to add them to the structure
        for solution in solutions.iter().sorted_unstable_by_key(|k| {
            let Some((t0, t1, ..)) = pools.get(&k.id) else {
                // This should never happen but let's handle it as gracefully as possible -
                // right now will skip the pool, not produce an error
                return 0usize;
            };
            let t0_idx = asset_builder.add_or_get_asset(*t0);
            let t1_idx = asset_builder.add_or_get_asset(*t1);
            (t0_idx << 16) | t1_idx
        }) {
            println!("Processing solution");
            // Get the information for the pool or skip this solution if we can't find a
            // pool for it
            let Some((t0, t1, snapshot, store_index)) = pools.get(&solution.id) else {
                // This should never happen but let's handle it as gracefully as possible -
                // right now will skip the pool, not produce an error
                warn!(
                    "Skipped a solution as we couldn't find a pool for it: {:?}, {:?}",
                    pools, solution.id
                );
                continue;
            };
            // Call our processing function with a fixed amount of shared gas
            // Wrap the Uniswap snapshot in the PoolState enum
            let pool_state = PoolState::Uniswap(snapshot.clone());
            Self::process_solution(
                &mut pairs,
                &mut asset_builder,
                &mut user_orders,
                &orders_by_pool,
                &mut top_of_block_orders,
                &mut pool_updates,
                solution,
                &pool_state,
                *t0,
                *t1,
                *store_index,
                None
            )?;
        }

        Ok(AngstromBundle::new(
            asset_builder.get_asset_array(),
            pairs,
            pool_updates,
            top_of_block_orders,
            user_orders
        ))
    }

    pub fn build_dummy_for_tob_gas(
        user_order: &OrderWithStorageData<RpcTopOfBlockOrder>
    ) -> Result<AngstromBundle> {
        let mut top_of_block_orders = Vec::new();
        let pool_updates = Vec::new();
        let mut pairs = Vec::new();
        let user_orders = Vec::new();
        let mut asset_builder = AssetBuilder::new();

        // Get the information for the pool or skip this solution if we can't find a
        // pool for it
        let (t0, t1) = {
            let token_in = user_order.token_in();
            let token_out = user_order.token_out();

            if token_in < token_out { (token_in, token_out) } else { (token_out, token_in) }
        };
        // Make sure the involved assets are in our assets array and we have the
        // appropriate asset index for them
        let t0_idx = asset_builder.add_or_get_asset(t0) as u16;
        let t1_idx = asset_builder.add_or_get_asset(t1) as u16;

        let pair = Pair {
            index0:       t0_idx,
            index1:       t1_idx,
            store_index:  0,
            price_1over0: U256::from(1)
        };
        pairs.push(pair);

        asset_builder.external_swap(
            AssetBuilderStage::TopOfBlock,
            user_order.token_in(),
            user_order.token_out(),
            user_order.quantity_in,
            user_order.quantity_out
        );
        // Get our list of user orders, if we have any
        top_of_block_orders.push(TopOfBlockOrder::of_max_gas(user_order, 0));

        Ok(AngstromBundle::new(
            asset_builder.get_asset_array(),
            pairs,
            pool_updates,
            top_of_block_orders,
            user_orders
        ))
    }

    pub fn build_dummy_for_user_gas(
        user_order: &OrderWithStorageData<AllOrders>
    ) -> Result<AngstromBundle> {
        // in order to properly build this. we will create a fake order with the
        // amount's flipped going the other way so we have a direct match and
        // don't have to worry about balance deltas

        let top_of_block_orders = Vec::new();
        let pool_updates = Vec::new();
        let mut pairs = Vec::new();
        let mut user_orders = Vec::new();
        let mut asset_builder = AssetBuilder::new();

        {
            // Get the information for the pool or skip this solution if we can't find a
            // pool for it
            let (t0, t1) = {
                let token_in = user_order.token_in();
                let token_out = user_order.token_out();
                if token_in < token_out { (token_in, token_out) } else { (token_out, token_in) }
            };
            // Make sure the involved assets are in our assets array and we have the
            // appropriate asset index for them
            let t0_idx = asset_builder.add_or_get_asset(t0) as u16;
            let t1_idx = asset_builder.add_or_get_asset(t1) as u16;

            // hacky but works
            if pairs.is_empty() {
                let pair = Pair {
                    index0:       t0_idx,
                    index1:       t1_idx,
                    store_index:  0,
                    price_1over0: user_order.limit_price()
                };
                pairs.push(pair);
            }

            let pair_idx = pairs.len() - 1;

            let outcome = OrderOutcome {
                id:      user_order.order_id,
                outcome: OrderFillState::CompleteFill
            };
            // Get our list of user orders, if we have any
            user_orders.push(UserOrder::from_internal_order_max_gas(
                user_order,
                &outcome,
                pair_idx as u16
            ));
        }

        Ok(AngstromBundle::new(
            asset_builder.get_asset_array(),
            pairs,
            pool_updates,
            top_of_block_orders,
            user_orders
        ))
    }

    /// Process a single solution (Uniswap-specific)
    #[allow(clippy::too_many_arguments)]
    pub fn process_solution(
        pairs: &mut Vec<Pair>,
        asset_builder: &mut AssetBuilder,
        user_orders: &mut Vec<UserOrder>,
        orders_by_pool: &HashMap<FixedBytes<32>, HashSet<OrderWithStorageData<AllOrders>>>,
        top_of_block_orders: &mut Vec<TopOfBlockOrder>,
        pool_updates: &mut Vec<PoolUpdate>,
        solution: &PoolSolution,
        snapshot: &PoolState,
        t0: Address,
        t1: Address,
        store_index: u16,
        shared_gas: Option<U256>
    ) -> Result<()> {
        use base64::{Engine, prelude::BASE64_STANDARD};

        tracing::info!(?solution);
        let process_solution_span =
            tracing::debug_span!("process_solution", t0 = ?t0, t1 = ?t1, pool_id = ?solution.id)
                .entered();
        // Dump the solution if solution dumps are enabled
        if tracing::event_enabled!(target: "dump::solution", Level::TRACE, pool_id = ?solution.id) {
            // Dump the solution
            let json = serde_json::to_string(&(
                solution,
                orders_by_pool,
                snapshot,
                t0,
                t1,
                store_index,
                shared_gas
            ))
            .unwrap();
            let b64_output = BASE64_STANDARD.encode(json.as_bytes());
            trace!(target: "dump::solution", data = b64_output, "Raw solution data");
        }

        debug!(t0 = ?t0, t1 = ?t1, pool_id = ?solution.id, "Processing solution for pool");

        // Make sure the involved assets are in our assets array and we have the
        // appropriate asset index for them
        let t0_idx = asset_builder.add_or_get_asset(t0) as u16;
        let t1_idx = asset_builder.add_or_get_asset(t1) as u16;
        tracing::info!(?t0, ?t1, ?t0_idx, ?t1_idx);

        // Build our Pair featuring our uniform clearing price
        // This price is in Ray format as requested.
        let pair_idx = pairs.len();

        // Log the indexes we found for the assets and pair of this solution
        trace!(t0_idx, t1_idx, pair_idx, "Found asset and pair indexes");

        // Add the ToB order to our tob order list - This is currently converting
        // between two ToB order formats
        if let Some(tob) = solution.searcher.as_ref() {
            // Account for our ToB order
            asset_builder.external_swap(
                AssetBuilderStage::TopOfBlock,
                tob.asset_in,
                tob.asset_out,
                tob.quantity_in,
                tob.quantity_out
            );
            let contract_tob = if let Some(g) = shared_gas {
                let order = TopOfBlockOrder::of(tob, g, pair_idx as u16)?;
                asset_builder.add_gas_fee(
                    AssetBuilderStage::TopOfBlock,
                    t0,
                    order.gas_used_asset_0
                );
                order
            } else {
                asset_builder.add_gas_fee(
                    AssetBuilderStage::TopOfBlock,
                    t0,
                    tob.priority_data.gas.to()
                );
                TopOfBlockOrder::of_max_gas(tob, pair_idx as u16)
            };
            let is_bid = tob.asset_in == t1;
            trace!(quantity_in = ?contract_tob.quantity_in, quantity_out = ?contract_tob.quantity_out, is_bid, "Processing searcher (ToB) order");
            top_of_block_orders.push(contract_tob);
        }

        ///////////////////////////////
        //// handling user orders ////
        /////////////////////////////

        let default = HashMap::new();
        // Get our list of user orders, if we have any
        let mut order_list: HashMap<OrderId, &OrderWithStorageData<AllOrders>> = orders_by_pool
            .get(&solution.id)
            .map(|o| o.iter().map(|order| (order.order_id, order)).collect())
            .unwrap_or_else(|| default);

        // Loop through our filled user orders, do accounting, and add them to our user
        // order list
        let mut total_user_fees: u128 = 0;
        for (outcome, order) in solution
            .limit
            .iter()
            .map(|order| (order, order_list.remove(&order.id)))
            .filter(|(outcome, o)| {
                if o.is_none() && outcome.is_filled() {
                    tracing::error!(?outcome, "lost a order");
                    return false;
                }
                outcome.is_filled()
            })
        {
            total_user_fees = total_user_fees.saturating_add(Self::apply_user_order(
                outcome,
                order,
                solution.ucp,
                solution.fee,
                shared_gas,
                pair_idx,
                asset_builder,
                user_orders
            )?);
        }
        tracing::info!(?solution.fee, total_user_fees, "fees for book");

        ///////////////////////////////
        //// handling donate merge ////
        //////////////////////////////

        // Dispatch to AMM-specific logic based on pool type
        match snapshot {
            PoolState::Uniswap(uni_snapshot) => {
                Self::process_uniswap_bundle(
                    pairs,
                    asset_builder,
                    pool_updates,
                    solution,
                    uni_snapshot,
                    total_user_fees,
                    pair_idx,
                    t0,
                    t1,
                    t0_idx,
                    t1_idx,
                    store_index
                )?;
            }
            PoolState::Balancer(_) => {
                // This should not happen in UniswapBundleBuilder
                return Err(eyre::eyre!("Balancer pool found in UniswapBundleBuilder"));
            }
        }

        // Drop our span to give it purpose in life
        drop(process_solution_span);

        // And we're done
        Ok(())
    }

    fn apply_user_order(
        outcome: &OrderOutcome,
        order: Option<&OrderWithStorageData<AllOrders>>,
        ucp: Ray,
        fee: u32,
        shared_gas: Option<U256>,
        pair_idx: usize,
        asset_builder: &mut AssetBuilder,
        user_orders: &mut Vec<UserOrder>
    ) -> Result<u128> {
        let order = order.unwrap();
        // Calculate our final amounts based on whether the order is in T0 or T1 context
        assert_eq!(outcome.id.hash, order.order_id.hash, "Order and outcome mismatched");

        let fill_amount = outcome.fill_amount(order.amount());

        // TODO: this needs to be properly set

        let gas = order.priority_data.gas.to::<u128>();
        let (t1, t0_net, t0_fee) = get_quantities_at_price(
            order.is_bid(),
            order.exact_in(),
            fill_amount,
            gas,
            fee as u128,
            ucp
        );

        // we don't account for the gas here in these quantites as the order
        let (quantity_in, quantity_out) = if order.is_bid() {
            // one for zero

            // If the order is a bid, we're getting all our T1 in and we're sending t0_net
            // back to the contract
            (t1, t0_net)
        } else {
            // If the order is an ask, we're getting t0_net + t0_fee + gas in and we're
            // sending t1 back to the contract
            // zero for one
            (t0_net + t0_fee + gas, t1)
        };

        tracing::info!(quantity_in = ?quantity_in, quantity_out = ?quantity_out, is_bid = order.is_bid, exact_in = order.exact_in(), "Processing user order");
        let token_in = order.token_in();
        let token_out = order.token_out();

        let user_order = if let Some(g) = shared_gas {
            UserOrder::from_internal_order(order, outcome, g, pair_idx as u16)?
        } else {
            UserOrder::from_internal_order_max_gas(order, outcome, pair_idx as u16)
        };

        // we add once we are past anything that can error.
        asset_builder.external_swap(
            AssetBuilderStage::UserOrder,
            token_in,
            token_out,
            quantity_in,
            quantity_out
        );
        user_orders.push(user_order);
        Ok(t0_fee)
    }

    fn orders_by_pool_from_pre_proposals(
        pre_proposals: &[PreProposal],
        orders: OrderSet<AllOrders, RpcTopOfBlockOrder>
    ) -> HashMap<PoolId, HashSet<OrderWithStorageData<AllOrders>>> {
        let valid_limit = pre_proposals
            .iter()
            .flat_map(|pre| pre.limit.clone())
            .collect();
        let (limit_orders, _) = orders.into_book_and_searcher(valid_limit, vec![]);

        limit_orders.into_iter().fold(HashMap::new(), |mut acc, x| {
            acc.entry(x.pool_id).or_default().insert(x);
            acc
        })
    }

    fn fetch_total_orders_and_gas_delegated_to_orders(
        orders_by_pool: &HashMap<FixedBytes<32>, HashSet<OrderWithStorageData<AllOrders>>>,
        solutions: &[PoolSolution]
    ) -> (u64, u64) {
        solutions
            .iter()
            .map(|s| (s, orders_by_pool.get(&s.id).cloned()))
            .filter_map(|(solution, order_list)| {
                let Some(mut order_list) = order_list.map(|i| i.into_iter().collect::<Vec<_>>())
                else {
                    return solution
                        .searcher
                        .as_ref()
                        .map(|searcher| (1, searcher.priority_data.gas_units));
                };

                // Sort the user order list so we can properly associate it with our
                // OrderOutcomes.  First bids by price then asks by price.
                order_list.sort_by(|a, b| match (a.is_bid, b.is_bid) {
                    (true, true) => b.priority_data.cmp(&a.priority_data),
                    (false, false) => a.priority_data.cmp(&b.priority_data),
                    (..) => b.is_bid.cmp(&a.is_bid)
                });
                let mut cnt = 0;
                let mut total_gas = 0;
                for (_, order) in solution
                    .limit
                    .iter()
                    .zip(order_list.iter())
                    .filter(|(outcome, _)| outcome.is_filled())
                {
                    cnt += 1;
                    total_gas += order.priority_data.gas_units;
                }

                solution.searcher.as_ref().inspect(|searcher| {
                    cnt += 1;
                    total_gas += searcher.priority_data.gas_units;
                });

                Some((cnt, total_gas))
            })
            .fold((0u64, 0u64), |(mut cnt, mut tg), x| {
                cnt += x.0;
                tg += x.1;
                (cnt, tg)
            })
    }

    /// Process a Uniswap pool solution into bundle components
    #[allow(clippy::too_many_arguments)]
    fn process_uniswap_bundle(
        pairs: &mut Vec<Pair>,
        asset_builder: &mut AssetBuilder,
        pool_updates: &mut Vec<PoolUpdate>,
        solution: &PoolSolution,
        snapshot: &UniswapPoolState,
        total_user_fees: u128,
        pair_idx: usize,
        t0: Address,
        _t1: Address,
        t0_idx: u16,
        t1_idx: u16,
        store_index: u16
    ) -> Result<()> {
        let mut ucp = solution.ucp;

        // Process ToB swap if present
        let tob_swap_info = if let Some(ref tob) = solution.searcher {
            match TopOfBlockOrder::calc_vec_and_reward(tob, snapshot) {
                Ok((v, reward_q)) => {
                    // if we have a tob swap, and ucp == 0, we are going to want to update it
                    if ucp.is_zero() {
                        ucp = Ray::from(v.end_price);
                    }

                    trace!(
                        net_t0 = v.total_d_t0,
                        net_t1 = v.total_d_t1,
                        end_price = ?v.end_price,
                        is_bid = !v.zero_for_one(),
                        reward_q,
                        "Processing ToB swap vs AMM"
                    );
                    Some((v, reward_q))
                }
                Err(error) => {
                    tracing::error!(?error, "Error in ToB swap vs AMM");
                    None
                }
            }
        } else {
            trace!("No ToB Swap vs AMM");
            None
        };

        pairs.push(Pair { index0: t0_idx, index1: t1_idx, store_index, price_1over0: *ucp });

        // If we have a ToB swap, our post-tob-price is the price at the end of that
        // swap, otherwise we're starting from the snapshot's current price
        let post_tob_price = tob_swap_info
            .clone()
            .map(|(v, _)| v)
            .unwrap_or_else(|| snapshot.noop());

        // Process book swap if UCP is non-zero
        let book_swap_vec = if solution.ucp.is_zero() {
            trace!("No book swap, UCP was zero");
            None
        } else {
            let ucp_sqrt: SqrtPriceX96 = solution.ucp.into();
            let book_swap_vec = post_tob_price.swap_to_price(ucp_sqrt)?;
            trace!(
                net_t0 = book_swap_vec.total_d_t0,
                net_t1 = book_swap_vec.total_d_t1,
                end_price = ?book_swap_vec.end_price,
                is_bid = !book_swap_vec.zero_for_one(),
                reward_q = solution.reward_t0,
                "Processing Book swap vs AMM"
            );
            Some(book_swap_vec)
        };

        // Calculate donation splits
        let total_lp_user_donate = (total_user_fees as f64 * LP_DONATION_SPLIT) as u128;
        let save_amount = total_user_fees - total_lp_user_donate;

        // Calculate donations from book and ToB swaps
        let book_donation_vec = book_swap_vec
            .as_ref()
            .map(|bsv| bsv.t0_donation_vec(solution.reward_t0 + total_lp_user_donate));

        let tob_donation_vec = tob_swap_info
            .as_ref()
            .map(|(tob_vec, tob_d)| tob_vec.t0_donation_vec(*tob_d));

        let donation = match (book_donation_vec, tob_donation_vec) {
            (Some(bsv), Some(tob)) => {
                Some(&(DonationCalculation::from_vec(&tob))? + bsv.as_slice())
            }
            (Some(bsv), None) => Some(DonationCalculation::from_vec(&bsv)?),
            (None, Some(tob)) => Some(DonationCalculation::from_vec(&tob)?),
            (None, None) => None
        };
        let total_donation = donation
            .as_ref()
            .map(|d| d.total_donated)
            .unwrap_or(solution.reward_t0 + total_lp_user_donate);

        // Calculate net pool swap vector
        let net_pool_vec = if let Some((tob_vec, _)) = tob_swap_info {
            let net_t0 = book_swap_vec
                .as_ref()
                .map(|b| b.t0_signed())
                .unwrap_or(I256::ZERO)
                + tob_vec.t0_signed();

            let net_direction = net_t0.is_negative();

            let amount_in = if net_t0.is_negative() {
                net_t0.unsigned_abs()
            } else {
                (book_swap_vec
                    .as_ref()
                    .map(|b| b.t1_signed())
                    .unwrap_or(I256::ZERO)
                    + tob_vec.t1_signed())
                .unsigned_abs()
            };

            snapshot
                .swap_current_with_amount(I256::from_raw(amount_in), net_direction)?
                .clone()
        } else {
            book_swap_vec.unwrap_or_else(|| snapshot.noop())
        };

        trace!(
            net_t0 = net_pool_vec.total_d_t0,
            net_t1 = net_pool_vec.total_d_t1,
            end_price = ?net_pool_vec.end_price,
            is_bid = !net_pool_vec.zero_for_one(),
            reward_q = total_donation,
            "Built net swap vec"
        );

        // Account for net pool swap
        let (asset_in_index, asset_out_index) =
            if net_pool_vec.zero_for_one() { (t0_idx, t1_idx) } else { (t1_idx, t0_idx) };
        let (quantity_in, quantity_out) = (net_pool_vec.input(), net_pool_vec.output());

        asset_builder.uniswap_swap(
            AssetBuilderStage::Swap,
            asset_in_index as usize,
            asset_out_index as usize,
            quantity_in,
            quantity_out
        );

        // Account for rewards and fees
        asset_builder.allocate(AssetBuilderStage::Reward, t0, total_donation);
        asset_builder.allocate(AssetBuilderStage::Reward, t0, save_amount);
        asset_builder.add_gas_fee(AssetBuilderStage::Reward, t0, save_amount);

        // Build rewards update
        let (rewards_update, optional_reward) = donation
            .map(|d| d.into_reward_updates())
            .unwrap_or_else(|| {
                (
                    RewardsUpdate::CurrentOnly {
                        amount:             total_donation,
                        expected_liquidity: snapshot.current_liquidity()
                    },
                    None
                )
            });

        // Add pool updates
        pool_updates.push(PoolUpdate {
            zero_for_one: net_pool_vec.zero_for_one(),
            pair_index: pair_idx as u16,
            swap_in_quantity: net_pool_vec.input(),
            rewards_update
        });

        if let Some(optional_reward) = optional_reward {
            pool_updates.push(PoolUpdate {
                zero_for_one:     false,
                pair_index:       pair_idx as u16,
                swap_in_quantity: 0,
                rewards_update:   optional_reward
            });
        }

        Ok(())
    }
}
