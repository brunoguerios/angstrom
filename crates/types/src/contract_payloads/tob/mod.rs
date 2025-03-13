use std::collections::HashMap;

use alloy::primitives::aliases::I24;
use alloy_primitives::{U160, U256, keccak256};
use eyre::eyre;
use itertools::Itertools;

use super::rewards::RewardsUpdate;
use crate::{
    matching::uniswap::{PoolPrice, PoolSnapshot, Quantity, Tick},
    sol_bindings::{grouped_orders::OrderWithStorageData, rpc_orders::TopOfBlockOrder}
};

#[derive(Debug, Default)]
pub struct ToBOutcome {
    pub start_tick:             i32,
    pub end_tick:               i32,
    pub start_liquidity:        u128,
    pub tribute:                u128,
    pub total_cost:             u128,
    pub total_allocated_output: u128,
    pub total_swap_output:      u128,
    pub total_reward:           u128,
    pub tick_donations:         HashMap<(Tick, Tick), u128>
}

impl ToBOutcome {
    /// Sum of the donations across all ticks
    pub fn total_donations(&self) -> u128 {
        self.tick_donations
            .iter()
            .fold(0, |acc, (_tick, donation)| acc + donation)
    }

    /// Tick donations plus tribute to determine total value of this outcome
    pub fn total_value(&self) -> u128 {
        self.total_donations() + self.tribute
    }

    pub fn from_tob_and_snapshot(
        tob: &OrderWithStorageData<TopOfBlockOrder>,
        snapshot: &PoolSnapshot
    ) -> eyre::Result<Self> {
        // First let's simulate the actual ToB swap and use that to determine what our
        // leftover T0 is for rewards
        let (pricevec, leftover_t0) = if tob.is_bid {
            // If I'm a bid, I'm buying T0.  In order to reward I will offer in more T1 than
            // needed, and I should compare the T0 I get out with the T0 I expect back in
            // order to determine the reward quantity
            let pricevec = (snapshot.current_price() + Quantity::Token1(tob.quantity_in))?;

            let leftover = pricevec
                .d_t0
                .checked_sub(tob.quantity_out)
                .ok_or_else(|| eyre!("Not enough output to cover the transaction"))?;
            (pricevec, leftover)
        } else {
            // If I'm an ask, I'm selling T0.  In order to reward I will offer in more T0
            // than needed and I should compare the T0 I offer to the T0 needed to produce
            // the T1 I expect to get back
            // First we find the amount of T0 in it would take to at least hit our quantity
            // out
            let cost = (snapshot.current_price() - Quantity::Token1(tob.quantity_out))?.d_t0;

            let leftover = tob
                .quantity_in
                .checked_sub(cost)
                .ok_or_else(|| eyre!("Not enough input to cover the transaction"))?;

            tracing::info!(?cost,?tob.quantity_out,?tob.quantity_in);
            // But then we have to operate in the right direction to calculate how much T1
            // we ACTUALLY get out
            let pricevec = (snapshot.current_price() + Quantity::Token0(cost))?;
            (pricevec, leftover)
        };
        tracing::trace!(leftover_t0, "Found leftover");
        tracing::trace!(tob.quantity_out, tob.quantity_in, "Building pricevec for quantity");
        tracing::trace!(start_price = ?pricevec.start_bound.price, end_price = ?pricevec.end_bound.price, pricevec.d_t0, pricevec.d_t1, "Pricevec inspect");
        let donation = pricevec.t0_donation_to_end_price(leftover_t0);
        tracing::trace!(donation.tribute, donation.total_donated, ?donation.tick_donations, "Donation inspection");
        let end_tick = pricevec.end_bound.tick;

        let rewards = Self {
            start_tick: snapshot.current_price().tick(),
            end_tick,
            start_liquidity: snapshot.current_price().liquidity(),
            tribute: donation.tribute,
            total_cost: pricevec.input(),
            total_allocated_output: tob.quantity_out,
            total_swap_output: pricevec.output(),
            total_reward: donation.total_donated,
            tick_donations: donation.tick_donations
        };

        Ok(rewards)
    }

    pub fn rewards_update_range(
        &self,
        current_tick: Tick,
        range_tick: Tick,
        snapshot: &PoolSnapshot
    ) -> RewardsUpdate {
        let from_above = range_tick > current_tick;
        let (low, high) =
            if from_above { (&current_tick, &range_tick) } else { (&range_tick, &current_tick) };
        let mut quantities = self
            .tick_donations
            .iter()
            .filter(|((low_tick, high_tick), _)| *high_tick > *low && *low_tick <= *high)
            // Sorts from the lowest tick to the highest tick
            .sorted_by_key(|f| f.0)
            .map(|f| *f.1)
            .collect::<Vec<_>>();

        // If we're coming from above we have to reverse, we want highest tick to lowest
        // tick
        if from_above {
            quantities.reverse();
        }

        let (start_tick, start_liquidity) = snapshot
            .get_range_for_tick(range_tick)
            .map(|r| (if from_above { r.lower_tick() } else { r.upper_tick() }, r.liquidity()))
            .unwrap_or_default();

        tracing::trace!(
            start_tick,
            start_liquidity,
            current_tick,
            ?quantities,
            quantities_len = quantities.len(),
            "Rewards update range dump"
        );

        match quantities.len() {
            0 | 1 => RewardsUpdate::CurrentOnly {
                amount: quantities.first().copied().unwrap_or_default()
            },
            _ => {
                let reward_checksum =
                    compute_reward_checksum(start_tick, start_liquidity, &snapshot, from_above);
                RewardsUpdate::MultiTick {
                    start_tick: I24::try_from(start_tick).unwrap_or_default(),
                    start_liquidity,
                    quantities,
                    reward_checksum
                }
            }
        }
    }
}

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

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use alloy_primitives::aliases::I24;

    use super::ToBOutcome;
    use crate::{
        contract_payloads::rewards::RewardsUpdate,
        matching::{
            SqrtPriceX96,
            uniswap::{LiqRange, PoolSnapshot}
        }
    };

    #[test]
    fn sorts_correctly() {
        let snapshot = PoolSnapshot::new(
            vec![
                LiqRange::new(100, 110, 123).unwrap(),
                LiqRange::new(110, 120, 456).unwrap(),
                LiqRange::new(120, 130, 789).unwrap(),
            ],
            SqrtPriceX96::at_tick(100).unwrap()
        )
        .unwrap();
        let donations =
            HashMap::from([((100, 110), 123_u128), ((110, 120), 456_u128), ((120, 130), 789_u128)]);

        // Upwards update order checking
        let upwards_update = ToBOutcome {
            start_tick: 100,
            end_tick: 120,
            tick_donations: donations.clone(),
            ..Default::default()
        }
        .rewards_update_range(120, 100, &snapshot);
        let RewardsUpdate::MultiTick { start_tick: upwards_start_tick, .. } = upwards_update else {
            panic!("Upwards update was single-tick");
        };
        assert_eq!(
            upwards_start_tick,
            I24::unchecked_from(110),
            "Upwards update did not start at lowest tick"
        );

        // Downwards update order checking
        let downwards_update = ToBOutcome {
            start_tick: 120,
            end_tick: 100,
            tick_donations: donations.clone(),
            ..Default::default()
        }
        .rewards_update_range(100, 120, &snapshot);
        let RewardsUpdate::MultiTick { start_tick: downwards_start_tick, .. } = downwards_update
        else {
            panic!("Downwards update was single-tick");
        };
        assert_eq!(
            downwards_start_tick,
            I24::unchecked_from(120),
            "Downwards update did not start at highest tick"
        );
    }
}

/// Computes the reward checksum for a given range of ticks.
///
/// `from_above`: `true` = high to low, `false` = low to high.
fn compute_reward_checksum(
    start_tick: i32,
    start_liquidity: u128,
    snapshot: &PoolSnapshot,
    from_above: bool
) -> U160 {
    let mut reward_checksum = [0u8; 32];
    let mut tick = start_tick;
    let tick_spacing = snapshot.tick_spacing();
    let current_tick = snapshot.current_price().tick();

    tracing::info!(
        start_tick,
        start_liquidity,
        tick_spacing,
        from_above,
        current_tick,
        "Starting checksum computation"
    );

    // Get initial liquidity directly from snapshot
    let mut liquidity = snapshot
        .liquidity_at_tick(start_tick)
        .unwrap_or(start_liquidity);

    loop {
        tracing::info!(tick, liquidity, "Processing tick before update");

        let tick_bytes = &tick.to_be_bytes()[1..];
        let hash_input =
            [reward_checksum.as_slice(), &liquidity.to_be_bytes(), tick_bytes].concat();
        reward_checksum = *keccak256(&hash_input);

        tracing::info!(tick, liquidity, "Updated liquidity in checksum");

        // Move to the next initialized tick while enforcing tick spacing
        if let Some(next_tick) = snapshot.get_next_tick_gt(tick, tick_spacing) {
            tracing::info!(tick, next_tick, "Moving to next initialized tick");
            tick = next_tick;

            // Update liquidity for the new tick
            liquidity = snapshot.liquidity_at_tick(tick).unwrap_or(0);
        } else {
            tracing::info!(tick, "No more initialized ticks found, stopping");
            break;
        }
    }

    let final_checksum = U160::from(U256::from_be_bytes(reward_checksum) >> 96);

    tracing::info!(checksum = format!("{:?}", final_checksum), "Final computed checksum");

    final_checksum
}
