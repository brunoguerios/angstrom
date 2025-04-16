use std::collections::HashMap;

pub struct DonationCalculation {
    // the amount to donate to the current tick of the pool.
    pub current_tick: u128,
    // pub current_tick: i32,
    // the amount to donate at the next tick increments
    pub rest:         HashMap<i32, u128>
}

impl DonationCalculation {
    pub fn combine(&self, rhs: &Self) -> Self {
        let donations = self.rest.iter().chain(rhs.rest.iter()).fold(
            HashMap::<i32, u128>::new(),
            |mut acc, (tick, t_amount)| {
                acc.entry(*tick)
                    .and_modify(|amount| *amount += t_amount)
                    .or_insert(*t_amount);

                acc
            }
        );

        let current_tick = self.current_tick + rhs.current_tick;

        Self { current_tick, rest: donations }
    }
}
