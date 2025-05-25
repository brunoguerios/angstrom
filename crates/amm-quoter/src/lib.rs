use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
    time::Duration
};

use alloy::primitives::{FixedBytes, U256};
use angstrom_types::{primitive::PoolId, uni_structure::BaselinePoolState};
use futures::{Stream, future::BoxFuture, stream::FuturesUnordered};
use order_pool::order_storage::OrderStorage;
use rayon::ThreadPool;
use serde::{Deserialize, Serialize};
use tokio::{
    sync::mpsc,
    time::{Interval, interval}
};
use uniswap_v4::uniswap::pool_manager::SyncedUniswapPools;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Slot0Update {
    /// there will be 120 updates per block or per 100ms
    pub seq_id:        u8,
    /// in case of block lag on node
    pub current_block: u64,
    /// basic identifier
    pub pool_id:       PoolId,

    pub sqrt_price_x96: U256,
    pub tick:           i32
}

pub trait AngstromBookQuoter: Stream<Item = Slot0Update> {
    /// will configure this stream to receieve updates of the given pool
    fn subscribe_to_updates(&self, pool_id: PoolId);
    /// will configure this stream to stop updates of the given pool
    fn unsubscribe_to_updates(&self, pool_id: PoolId);
}

pub struct QuoterManager {
    orders: Arc<OrderStorage>,
    amms:   SyncedUniswapPools,
    block:  u64,

    threadpool:     ThreadPool,
    book_snapshots: HashMap<PoolId, BaselinePoolState>,
    pending_tasks:  FuturesUnordered<BoxFuture<'static, Slot0Update>>,

    pool_to_subscribers: HashMap<PoolId, HashSet<mpsc::Sender<Slot0Update>>>,

    execution_interval: Interval
}

impl QuoterManager {
    pub fn new(
        orders: Arc<OrderStorage>,
        amms: SyncedUniswapPools,
        threadpool: ThreadPool,
        update_interval: Duration
    ) -> Self {
        let mut block = 0;
        let book_snapshots = amms
            .iter()
            .map(|entry| {
                let pool_lock = entry.value().read().unwrap();
                let block = pool_lock.block_number();
                let snapshot_data = pool_lock.fetch_pool_snapshot().unwrap().2;
                (*entry.key(), snapshot_data)
            })
            .collect();

        Self {
            orders,
            amms,
            block,
            book_snapshots,
            threadpool,
            pending_tasks: FuturesUnordered::new(),
            pool_to_subscribers: HashMap::default(),
            execution_interval: interval(update_interval)
        }
    }
}
