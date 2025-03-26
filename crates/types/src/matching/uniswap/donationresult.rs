use std::collections::HashMap;

use eyre::eyre;

use super::Tick;
use crate::matching::SqrtPriceX96;

#[derive(Debug)]
pub struct DonationResult {
    /// HashMap from liquidity range bounds `(lower_tick, upper_tick)` to
    /// donation quantity in T0
    pub tick_donations: HashMap<(Tick, Tick), u128>,
    pub final_price:    SqrtPriceX96,
    /// Total amount of donation in T0
    pub total_donated:  u128,
    /// Total amount of "tribute" taken by Angstrom in T0
    pub tribute:        u128
}

impl DonationResult {
    /// Combine this donation result with another donation result.  Donations to
    /// the same liquidity segment will be summed.  This method will also check
    /// to ensure that our output is a set of donations to a contiguous range.
    pub fn combine(&self, rhs: &Self) -> eyre::Result<Self> {
        // Merge our two HashMaps
        let mut tick_donations = HashMap::new();
        for (k, v) in self.tick_donations.iter().chain(rhs.tick_donations.iter()) {
            tick_donations
                .entry(*k)
                .and_modify(|e| *e += *v)
                .or_insert(*v);
        }

        // Validate that we're still covering a contiguous range
        let mut all_ranges = tick_donations.iter().map(|(k, _)| k).collect::<Vec<_>>();
        all_ranges.sort_by_key(|(l, _)| *l);
        let contiguous = all_ranges.windows(2).all(|k| k[0].1 == k[1].0);
        if !contiguous {
            return Err(eyre!("Does not still cover a contiguous range"));
        }

        // Do we still need this?
        let final_price = self.final_price;
        let total_donated = self.total_donated + rhs.total_donated;
        let tribute = self.tribute + rhs.tribute;

        Ok(Self { tick_donations, final_price, total_donated, tribute })
    }
}
