//! Uniswap-specific bundle processing logic

use alloy::primitives::{Address, I256};
use tracing::trace;

use super::{LP_DONATION_SPLIT, Pair, TopOfBlockOrder};
use crate::{
    contract_payloads::{
        asset::builder::{AssetBuilder, AssetBuilderStage},
        rewards::{PoolUpdate, RewardsUpdate}
    },
    matching::{Ray, SqrtPriceX96},
    orders::PoolSolution,
    uni_structure::{UniswapPoolState, donation::DonationCalculation}
};

/// Process a Uniswap pool solution into bundle components
#[allow(clippy::too_many_arguments)]
pub fn process_uniswap_bundle(
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
) -> eyre::Result<()> {
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
        (Some(bsv), Some(tob)) => Some(&(DonationCalculation::from_vec(&tob))? + bsv.as_slice()),
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
