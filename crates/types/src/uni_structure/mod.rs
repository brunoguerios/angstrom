use alloy::primitives::I256;
use liquidity_base::BaselineLiquidity;
use pool_swap::{PoolSwap, PoolSwapResult};

use crate::matching::{SqrtPriceX96, uniswap::Direction};

pub mod donation;
pub mod liquidity_base;
pub mod pool_swap;

pub struct BaselinePoolState {
    liquidity: BaselineLiquidity,
    fee:       u32
}

impl BaselinePoolState {
    pub fn new(liquidity: BaselineLiquidity, fee: u32) -> Self {
        Self { liquidity, fee }
    }

    pub fn fee(&self) -> u32 {
        self.fee
    }

    pub fn noop<'a>(&'a self) -> PoolSwapResult<'a> {
        PoolSwapResult {
            fee:           self.fee,
            start_price:   self.liquidity.start_sqrt_price,
            start_tick:    self.liquidity.start_tick,
            end_price:     self.liquidity.start_sqrt_price,
            end_tick:      self.liquidity.start_tick,
            total_d_t0:    0,
            total_d_t1:    0,
            steps:         vec![],
            end_liquidity: self.liquidity.current()
        }
    }

    pub fn swap_current_to<'a>(
        &'a self,
        amount: I256,
        direction: Direction,
        price_limit: Option<SqrtPriceX96>
    ) -> eyre::Result<PoolSwapResult<'a>> {
        let liq = self.liquidity.current();

        PoolSwap {
            liquidity: liq,
            target_amount: amount,
            target_price: price_limit,
            direction,
            fee: self.fee
        }
        .swap()
    }

    pub fn swap_from_price_to<'a>(
        &'a self,
        start_price: SqrtPriceX96,
        amount: I256,
        direction: Direction,
        price_limit: Option<SqrtPriceX96>
    ) -> eyre::Result<PoolSwapResult<'a>> {
        let liq = self.liquidity.at_sqrt_price(start_price)?;

        PoolSwap {
            liquidity: liq,
            target_amount: amount,
            target_price: price_limit,
            direction,
            fee: self.fee
        }
        .swap()
    }
}
