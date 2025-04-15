use crate::matching::uniswap::TickInfo;
use alloy::primitives::U256;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Manages the liquidity ranges of the given pool snapshot
#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Liquidity {
    pub ticks: HashMap<i32, TickInfo>,
    pub tick_bitmap: HashMap<i16, U256>,
    /// they liquidity at the current tick
    pub current_liquidity: u128,
    pub(crate) current_tick: i32,
    pub(crate) tick_spacing: i32,
}
