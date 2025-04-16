use std::collections::HashMap;

use super::BaselinePoolState;
use crate::contract_payloads::rewards::RewardsUpdate;

pub struct DonationCalculation {
    // the amount to donate to the current tick of the pool.
    pub current_tick: u128,
    // what direction the donate is going
    pub direction:    bool,
    // pub current_tick: i32,
    // the amount to donate at the next tick increments
    pub rest:         HashMap<i32, u128>
}

impl DonationCalculation {
    pub fn into_reward_updates(
        &self,
        snapshot: &BaselinePoolState
        // _book: &Self,
    ) -> (RewardsUpdate, Option<RewardsUpdate>) {
        return (
            RewardsUpdate::CurrentOnly {
                amount:             self.current_tick,
                expected_liquidity: snapshot.liquidity.start_liquidity
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

    pub fn total_donated(&self) -> u128 {
        self.current_tick
    }
}
