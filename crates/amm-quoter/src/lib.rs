use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    pin::Pin,
    sync::Arc,
    task::Poll,
    time::Duration
};

use alloy::primitives::{Address, FixedBytes, U256};
use angstrom_types::{
    block_sync::BlockSyncConsumer, orders::OrderSet, primitive::PoolId,
    uni_structure::BaselinePoolState
};
use futures::{Stream, future::BoxFuture, stream::FuturesUnordered};
use matching_engine::{
    book::{BookOrder, OrderBook},
    build_book,
    matcher::delta::DeltaMatcher,
    strategy::BinarySearchStrategy
};
use order_pool::order_storage::OrderStorage;
use rayon::ThreadPool;
use serde::{Deserialize, Serialize};
use tokio::{
    sync::{mpsc, oneshot},
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

pub trait AngstromBookQuoter {
    /// will configure this stream to receieve updates of the given pool
    fn subscribe_to_updates(
        &self,
        pool_id: HashSet<PoolId>
    ) -> Pin<Box<dyn Stream<Item = Slot0Update> + Send + Sync + 'static>>;
}

pub struct QuoterManager<BlockSync: BlockSyncConsumer> {
    cur_block:  u64,
    seq_id:     u64,
    // we don't register, we just listen for new blocks and when processing
    // is good so that we can register new block and update sequence id
    block_sync: BlockSync,

    orders:              Arc<OrderStorage>,
    amms:                SyncedUniswapPools,
    threadpool:          ThreadPool,
    recv:                mpsc::Receiver<(HashSet<PoolId>, mpsc::Sender<Slot0Update>)>,
    book_snapshots:      HashMap<PoolId, BaselinePoolState>,
    pending_tasks:       FuturesUnordered<BoxFuture<'static, Slot0Update>>,
    pool_to_subscribers: HashMap<PoolId, Vec<mpsc::Sender<Slot0Update>>>,

    execution_interval: Interval
}

impl<BlockSync: BlockSyncConsumer> QuoterManager<BlockSync> {
    pub fn new(
        block_sync: BlockSync,
        orders: Arc<OrderStorage>,
        recv: mpsc::Receiver<(HashSet<PoolId>, mpsc::Sender<Slot0Update>)>,
        amms: SyncedUniswapPools,
        threadpool: ThreadPool,
        update_interval: Duration
    ) -> Self {
        let cur_block = block_sync.current_block_number();
        let book_snapshots = amms
            .iter()
            .map(|entry| {
                let pool_lock = entry.value().read().unwrap();
                let snapshot_data = pool_lock.fetch_pool_snapshot().unwrap().2;
                (*entry.key(), snapshot_data)
            })
            .collect();

        Self {
            seq_id: 0,
            block_sync,
            orders,
            amms,
            recv,
            cur_block,
            book_snapshots,
            threadpool,
            pending_tasks: FuturesUnordered::new(),
            pool_to_subscribers: HashMap::default(),
            execution_interval: interval(update_interval)
        }
    }

    fn handle_new_subscription(&mut self, pools: HashSet<PoolId>, chan: mpsc::Sender<Slot0Update>) {
        for pool in pools {
            self.pool_to_subscribers
                .entry(pool)
                .or_default()
                .push(chan.clone());
        }
    }
}

impl<BlockSync: BlockSyncConsumer> Future for QuoterManager<BlockSync> {
    type Output = ();

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>
    ) -> std::task::Poll<Self::Output> {
        while let Poll::Ready(Some((pools, subscriber))) = self.recv.poll_recv(cx) {
            self.handle_new_subscription(pools, subscriber);
        }

        while self.execution_interval.poll_tick(cx).is_ready() {
            // cycle through if we can't do any processing
            if !self.block_sync.can_operate() {
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }

            // update block number, amm snapshot and reset seq id
            if self.cur_block != self.block_sync.current_block_number() {
                self.book_snapshots = self
                    .amms
                    .iter()
                    .map(|entry| {
                        let pool_lock = entry.value().read().unwrap();
                        let snapshot_data = pool_lock.fetch_pool_snapshot().unwrap().2;
                        (*entry.key(), snapshot_data)
                    })
                    .collect();

                self.cur_block = self.block_sync.current_block_number();
                self.seq_id = 0;
            }
            // inc seq_id
            let seq_id = self.seq_id;
            self.seq_id += 1;

            let OrderSet { limit, searcher } = self.orders.get_all_orders();
            let book = build_non_proposal_books(limit, &self.book_snapshots);
            let searcher_orders: HashMap<PoolId, _> =
                searcher.into_iter().fold(HashMap::new(), |mut acc, order| {
                    acc.entry(order.pool_id).or_insert(order);
                    acc
                });

            // for (pool_id, snapshot) in &self.book_snapshots {
            //     let order_book =
            //     BinarySearchStrategy::run(book, searcher)
            //
            //     DeltaMatcher::new(book, tob, false)
            // }

            self.threadpool.spawn(op);

            // queue up new tasks on threadpool
        }

        Poll::Pending
    }
}

pub fn build_non_proposal_books(
    limit: Vec<BookOrder>,
    pool_snapshots: &HashMap<PoolId, BaselinePoolState>
) -> Vec<OrderBook> {
    let book_sources = orders_sorted_by_pool_id(limit);

    book_sources
        .into_iter()
        .map(|(id, orders)| {
            let amm = pool_snapshots.get(&id).map(|value| value.clone());
            build_book(id, amm, orders)
        })
        .collect()
}

pub fn orders_sorted_by_pool_id(limit: Vec<BookOrder>) -> HashMap<PoolId, HashSet<BookOrder>> {
    limit.into_iter().fold(HashMap::new(), |mut acc, order| {
        acc.entry(order.pool_id).or_default().insert(order);
        acc
    })
}
