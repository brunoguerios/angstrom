use std::collections::HashMap;

use super::{BaselinePoolState, pool_swap::PoolSwapResult};
use crate::contract_payloads::rewards::RewardsUpdate;

pub struct DonationCalculation {
    // the amount to donate to the current tick of the pool.
    // pub current_tick: u128,
    // what direction the donate is going
    // pub direction: bool,
    // pub current_tick: i32,
    // the amount to donate at the next tick increments
    pub rest:          HashMap<i32, u128>,
    pub total_donated: u128
}

impl DonationCalculation {
    pub fn into_reward_updates(
        &self,
        snapshot: &PoolSwapResult<'_>
    ) -> (RewardsUpdate, Option<RewardsUpdate>) {
        return (
            RewardsUpdate::CurrentOnly {
                amount:             self.total_donated,
                expected_liquidity: snapshot.end_liquidity.current_liquidity
            },
            None
        );

        // //
        // let pool_current_tick = snapshot.current_tick();
        //
        // // see if book donations are on both sides of the current tick.
        // // this will let us know
        //
        // // if both tob and book didn't cross any ticks,
        // // we can merge these into just a current
        // if self.rest.is_empty() && book.rest.is_empty() {
        //     RewardsUpdate {}
        // }
    }

    pub fn merge_rewards(&self, rhs: &Self) -> Self {
        let new_ticks = self.rest.iter().chain(rhs.rest.iter()).fold(
            HashMap::<i32, u128>::new(),
            |mut acc, (tick, am)| {
                acc.entry(*tick).and_modify(|a| *a += am).or_insert(*am);
                acc
            }
        );
        let total_donate = self.total_donated + rhs.total_donated;
        Self { rest: new_ticks, total_donated: total_donate }
    }

    pub fn total_donated(&self) -> u128 {
        self.total_donated
    }
}
