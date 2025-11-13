use std::{ops::RangeInclusive, sync::Arc};

use alloy::{providers::Provider, rpc::types::eth::Filter};
use alloy_primitives::Log;
use futures_util::stream::BoxStream;

use crate::balancer::pool_manager::PoolManagerError;

pub trait PoolManagerProvider: Send + Sync + Clone + Unpin {
    fn subscribe_blocks(self) -> BoxStream<'static, Option<PoolManagerBlocks>>;

    fn get_logs(&self, filter: &Filter) -> Result<Vec<Log>, PoolManagerError>;
    fn provider(&self) -> Arc<impl Provider>;
}

#[derive(Debug, Clone)]
pub enum PoolManagerBlocks {
    NewBlock(u64),
    Reorg(u64, RangeInclusive<u64>)
}

pub mod canonical_state_adapter;
