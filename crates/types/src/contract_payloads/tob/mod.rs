use std::collections::HashMap;

use alloy::primitives::{aliases::I24, U256};
use eyre::eyre;
use itertools::Itertools;

use super::rewards::RewardsUpdate;
use crate::{
    matching::uniswap::{PoolSnapshot, Quantity, Tick},
    sol_bindings::{grouped_orders::OrderWithStorageData, rpc_orders::TopOfBlockOrder}
};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ToBOutcome {
    pub start_tick:      i32,
    pub end_tick:        i32,
    pub start_liquidity: u128,
    pub tribute:         U256,
    pub total_cost:      U256,
    pub total_reward:    U256,
    pub tick_donations:  HashMap<Tick, U256>
}

impl ToBOutcome {
    /// Sum of the donations across all ticks
    pub fn total_donations(&self) -> U256 {
        self.tick_donations
            .iter()
            .fold(U256::ZERO, |acc, (_tick, donation)| acc + donation)
    }

    /// Tick donations plus tribute to determine total value of this outcome
    pub fn total_value(&self) -> U256 {
        self.total_donations() + self.tribute
    }

    pub fn from_tob_and_snapshot(
        tob: &OrderWithStorageData<TopOfBlockOrder>,
        snapshot: &PoolSnapshot
    ) -> eyre::Result<Self> {
        // First let's simulate the actual ToB swap and use that to determine what our
        // leftover T0 is for rewards
        let (pricevec, leftover) = if tob.is_bid {
            // If I'm a bid, I'm buying T0.  In order to reward I will offer in more T1 than
            // needed, and I should compare the T0 I get out with the T0 I expect back in
            // order to determine the reward quantity
            let pricevec = (snapshot.current_price() + Quantity::Token1(tob.quantity_in))?;
            tracing::info!(?tob.quantity_out, ?tob.quantity_in, ?pricevec.d_t0, ?pricevec.d_t1);
            let leftover = pricevec
                .d_t0
                .checked_sub(tob.quantity_out)
                .ok_or_else(|| eyre!("Not enough output to cover the transaction"))?;
            (pricevec, leftover)
        } else {
            // If I'm an ask, I'm selling T0.  In order to reward I will offer in more T0
            // than needed and I should compare the T0 I offer to the T0 needed to produce
            // the T1 I expect to get back
            let pricevec = (snapshot.current_price() - Quantity::Token1(tob.quantity_out))?;
            let leftover = tob
                .quantity_in
                .checked_sub(pricevec.d_t0)
                .ok_or_else(|| eyre!("Not enough input to cover the transaction"))?;
            (pricevec, leftover)
        };
        tracing::trace!(tob.quantity_out, tob.quantity_in, "Building pricevec for quantity");
        println!("Number of swaps in pricevec: {}", pricevec.steps.as_ref().unwrap().len());
        tracing::trace!(start_price = ?pricevec.start_bound.price, end_price = ?pricevec.end_bound.price, pricevec.d_t0, pricevec.d_t1, "Pricevec inspect");
        let donation = pricevec.t0_donation_to_end_price(leftover);
        let end_tick = pricevec.end_bound.tick;

        let rewards = Self {
            start_tick: snapshot.current_price().tick(),
            end_tick,
            start_liquidity: snapshot.current_price().liquidity(),
            tribute: U256::from(donation.tribute),
            total_cost: U256::from(pricevec.input()),
            total_reward: U256::from(donation.total_donated),
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
            .filter(|t| t.0 >= low && t.0 <= high)
            // Sorts from the lowest tick to the highest tick
            .sorted_by_key(|f| f.0)
            .map(|f| f.1.saturating_to())
            .collect::<Vec<_>>();

        // If we're coming from above we have to reverse, we want highest tick to lowest
        // tick
        if from_above {
            quantities.reverse();
        } else {
            quantities.insert(0, 0);
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
            _ => RewardsUpdate::MultiTick {
                start_tick: I24::try_from(start_tick).unwrap_or_default(),
                start_liquidity,
                quantities
            }
        }
    }

    /// DEPRECATED - use Self::rewards_update_range instead
    pub fn to_rewards_update(&self) -> RewardsUpdate {
        let mut donations = self.tick_donations.iter().collect::<Vec<_>>();
        donations.sort_by(|a, b| a.0.cmp(b.0));
        if self.start_tick <= self.end_tick {
            // Will sort from lowest to highest (donations[0] will be the lowest tick
            // number)
            donations.sort_by_key(|f| f.0);
        } else {
            // Will sort from highest to lowest (donations[0] will be the highest tick
            // number)
            donations.sort_by_key(|f| std::cmp::Reverse(f.0));
        }
        let quantities = donations
            .iter()
            .map(|d| d.1.saturating_to())
            .collect::<Vec<_>>();
        tracing::trace!(donations = ?donations, len = donations.len(), "Donations dump");
        tracing::trace!(self.end_tick, "end tick");
        let start_tick = I24::try_from(self.start_tick).unwrap_or_default();

        match quantities.len() {
            0 | 1 => RewardsUpdate::CurrentOnly {
                amount: quantities.first().copied().unwrap_or_default()
            },
            _ => RewardsUpdate::MultiTick {
                start_tick,
                start_liquidity: self.start_liquidity,
                quantities
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use alloy_primitives::{aliases::I24, U256};

    use super::ToBOutcome;
    use crate::{
        contract_payloads::rewards::RewardsUpdate,
        matching::{
            uniswap::{LiqRange, PoolSnapshot},
            SqrtPriceX96
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
        let donations = HashMap::from([
            (100, U256::from(123_u128)),
            (110, U256::from(456_u128)),
            (120, U256::from(789_u128))
        ]);

        // Upwards update order checking
        let upwards_update = ToBOutcome {
            start_tick: 100,
            end_tick: 120,
            tick_donations: donations.clone(),
            ..Default::default()
        }
        .rewards_update_range(120, 100, &snapshot);
        let RewardsUpdate::MultiTick {
            start_tick: upwards_start_tick,
            quantities: upwards_quantities,
            ..
        } = upwards_update
        else {
            panic!("Upwards update was single-tick");
        };
        assert_eq!(
            upwards_start_tick,
            I24::unchecked_from(100),
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
        let RewardsUpdate::MultiTick {
            start_tick: downwards_start_tick,
            quantities: downwards_quantities,
            ..
        } = downwards_update
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
