use angstrom_types::sol_bindings::grouped_orders::{GroupedVanillaOrder, OrderWithStorageData};

use super::BookOrder;

/// There are lots of different ways we can sort the orders we get in, so let's
/// make this modular

pub enum SortStrategy {
    Unsorted,
    ByPriceByVolume
}

impl Default for SortStrategy {
    fn default() -> Self {
        Self::Unsorted
    }
}

impl SortStrategy {
    pub fn sort_bids(&self, bids: &mut [BookOrder]) {
        if let Self::ByPriceByVolume = self {
            // Sort by price and then by volume - highest price first, highest volume first
            // for same price
            bids.sort_by(|a, b| b.priority_data.cmp(&a.priority_data));
        }
    }

    pub fn sort_asks(&self, asks: &mut [BookOrder]) {
        if let Self::ByPriceByVolume = self {
            // Sort by price and then by volume - lowest price first, highest volume first
            // for same price
            asks.sort_by(|a, b| a.priority_data.cmp(&b.priority_data));
        }
    }
}
