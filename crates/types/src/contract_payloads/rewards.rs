use alloy::primitives::aliases::I24;
use alloy_primitives::U160;
use itertools::Itertools;
use pade_macro::{PadeDecode, PadeEncode};

use super::{Asset, Pair, tob::MissingTickLiquidityError};
use crate::{contract_payloads::tob::compute_reward_checksum, matching::uniswap::PoolPriceVec};

#[derive(Debug, PadeEncode, PadeDecode)]
pub enum RewardsUpdate {
    MultiTick {
        start_tick:      I24,
        start_liquidity: u128,
        quantities:      Vec<u128>,
        reward_checksum: U160
    },
    CurrentOnly {
        amount:             u128,
        expected_liquidity: u128
    }
}

impl RewardsUpdate {
    pub fn from_vec(
        pricevec: &PoolPriceVec,
        total_donation: u128
    ) -> Result<Self, MissingTickLiquidityError> {
        let range_tick = pricevec.end_bound.tick;
        let current_tick = pricevec.start_bound.tick;
        let snapshot = pricevec.snapshot();

        let vec_donations = pricevec.t0_donation_to_end_price(total_donation);
        let from_above = range_tick > current_tick;
        let (low, high) =
            if from_above { (&current_tick, &range_tick) } else { (&range_tick, &current_tick) };
        let mut quantities = vec_donations
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
        tracing::warn!(current_tick, bound_tick, start_tick, start_liquidity, "Assembling rewards update");

        tracing::trace!(
            start_tick,
            start_liquidity,
            current_tick,
            ?quantities,
            quantities_len = quantities.len(),
            "Rewards update range dump"
        );

        match quantities.len() {
            0 | 1 => Ok(Self::CurrentOnly {
                amount:             quantities.first().copied().unwrap_or_default(),
                expected_liquidity: start_liquidity
            }),
            len => {
                let reward_checksum = compute_reward_checksum(
                    start_tick,
                    start_liquidity,
                    snapshot,
                    from_above,
                    len
                )?;
                Ok(Self::MultiTick {
                    start_tick: I24::try_from(start_tick).unwrap_or_default(),
                    start_liquidity,
                    quantities,
                    reward_checksum
                })
            }
        }
    }

    pub fn empty() -> Self {
        Self::CurrentOnly { amount: 0, expected_liquidity: 0 }
    }

    pub fn quantities(&self) -> Vec<u128> {
        match self {
            Self::MultiTick { quantities, .. } => quantities.clone(),
            Self::CurrentOnly { amount, .. } => vec![*amount]
        }
    }
}

#[derive(Debug, PadeEncode, PadeDecode)]
pub struct PoolUpdate {
    pub zero_for_one:     bool,
    pub pair_index:       u16,
    pub swap_in_quantity: u128,
    pub rewards_update:   RewardsUpdate
}

#[derive(PadeEncode, Debug)]
pub struct MockContractMessage {
    pub assets: Vec<Asset>,
    pub pairs:  Vec<Pair>,
    pub update: PoolUpdate
}
