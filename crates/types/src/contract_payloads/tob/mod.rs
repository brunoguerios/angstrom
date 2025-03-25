use std::fmt::Display;

use alloy::primitives::aliases::I24;
use alloy_primitives::{U160, U256, keccak256};
use thiserror::Error;

use crate::{
    matching::uniswap::{PoolPrice, PoolSnapshot, Quantity},
    sol_bindings::{grouped_orders::OrderWithStorageData, rpc_orders::TopOfBlockOrder}
};

/// grabs the final price vec location, adjusted for the donation amount that
/// was specified
pub fn generate_current_price_adjusted_for_donation<'a>(
    tob: &OrderWithStorageData<TopOfBlockOrder>,
    snapshot: &'a PoolSnapshot
) -> eyre::Result<PoolPrice<'a>> {
    // First let's simulate the actual ToB swap and use that to determine what our
    // leftover T0 is for rewards
    Ok(if tob.is_bid {
        // If I'm a bid, I'm buying T0.  In order to reward I will offer in more T1 than
        // needed, and I should compare the T0 I get out with the T0 I expect back in
        // order to determine the reward quantity
        let pricevec = (snapshot.current_price() + Quantity::Token1(tob.quantity_in))?;
        pricevec.end_bound
    } else {
        // If I'm an ask, I'm selling T0.  In order to reward I will offer in more T0
        // than needed and I should compare the T0 I offer to the T0 needed to produce
        // the T1 I expect to get back
        // First we find the amount of T0 in it would take to at least hit our quantity
        // out
        let cost = (snapshot.current_price() - Quantity::Token1(tob.quantity_out))?.d_t0;
        // But then we have to operate in the right direction to calculate how much T1
        // we ACTUALLY get out
        let pricevec = (snapshot.current_price() + Quantity::Token0(cost))?;
        pricevec.end_bound
    })
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Error)]
pub struct MissingTickLiquidityError;

impl Display for MissingTickLiquidityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "liquidity_at_tick was undefined for the given tick")
    }
}

/// Computes the reward checksum for a given range of ticks.
///
/// `from_above`: `true` = high to low, `false` = low to high.
/// total ticks should be the length of the reward vec
pub fn compute_reward_checksum(
    start_tick: i32,
    start_liquidity: u128,
    snapshot: &PoolSnapshot,
    from_above: bool,
    mut total_ticks: usize
) -> Result<U160, MissingTickLiquidityError> {
    let mut reward_checksum = [0u8; 32];
    let mut tick = start_tick;

    // Get initial liquidity directly from snapshot
    let mut liquidity = snapshot
        .liquidity_at_tick(start_tick)
        .unwrap_or(start_liquidity);

    loop {
        // we only checksum the ticks that w are rewarding.
        total_ticks -= 1;
        if total_ticks == 0 {
            break;
        }

        let tick_bytes: [u8; 3] = I24::try_from(tick).unwrap().to_be_bytes();
        let hash_input =
            [reward_checksum.as_slice(), &liquidity.to_be_bytes(), &tick_bytes].concat();
        reward_checksum = *keccak256(&hash_input);

        if from_above {
            // Move to the next initialized tick while enforcing tick spacing
            if let Some(next_tick) = snapshot.get_next_tick_lt(tick) {
                tick = next_tick;

                // Update liquidity for the new tick
                liquidity = snapshot
                    .liquidity_at_tick(tick)
                    .ok_or(MissingTickLiquidityError)?;
            } else {
                break;
            }
        } else {
            // Move to the next initialized tick while enforcing tick spacing
            if let Some(next_tick) = snapshot.get_next_tick_gt(tick) {
                tick = next_tick;

                // Update liquidity for the new tick
                liquidity = snapshot
                    .liquidity_at_tick(tick)
                    .ok_or(MissingTickLiquidityError)?;
            } else {
                break;
            }
        }
    }

    Ok(U160::from(U256::from_be_bytes(reward_checksum) >> 96))
}
