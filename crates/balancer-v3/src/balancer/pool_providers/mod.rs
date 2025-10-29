use std::ops::RangeInclusive;

use futures_util::stream::BoxStream;

pub trait PoolManagerProvider: Send + Sync {
    fn subscribe_blocks(&self) -> BoxStream<'static, Option<PoolManagerBlocks>>;
}

#[derive(Debug)]
pub enum PoolManagerBlocks {
    NewBlock(u64),
    Reorg(u64, RangeInclusive<u64>)
}

pub mod canonical_state_adapter;
