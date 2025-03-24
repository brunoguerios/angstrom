use alloy::primitives::U256;
use angstrom_types::{
    orders::{OrderId, OrderPriorityData},
    primitive::PoolId,
    sol_bindings::{
        ext::{RawPoolOrder, grouped_orders::OrderWithStorageData},
        rpc_orders::TopOfBlockOrder
    }
};
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use matching_engine::{
    book::{BookOrder, OrderBook, sort::SortStrategy},
    strategy::BinarySearchStrategy
};
use testing_tools::order_generator::{GeneratedPoolOrders, PoolOrderGenerator};
use uniswap_v4::uniswap::pool_manager::SyncedUniswapPool;

const TPS_BUCKETS: [u64; 6] = [20, 40, 100, 200, 500, 1000];
pub async fn setup_synced_pool_for_order_generation() -> SyncedUniswapPool {
    todo!()
}

// messy
pub fn setup_inputs(
    pools: GeneratedPoolOrders,
    amm: &SyncedUniswapPool
) -> (OrderBook, OrderWithStorageData<TopOfBlockOrder>) {
    let GeneratedPoolOrders { pool_id, tob, book } = pools;
    let (_, _, amm) = amm.read().unwrap().fetch_pool_snapshot().unwrap();
    let asks = book
        .iter()
        .filter(|f| !f.is_bid())
        .map(|ask| OrderWithStorageData {
            invalidates: vec![],
            order: ask.clone(),
            priority_data: OrderPriorityData {
                price:     *ask.price(),
                volume:    ask.amount(),
                gas:       U256::ZERO,
                gas_units: 0
            },
            is_bid: false,
            is_valid: true,
            is_currently_valid: true,
            order_id: OrderId {
                flash_block: None,
                reuse_avoidance: angstrom_types::sol_bindings::RespendAvoidanceMethod::Block(0),
                hash: ask.order_hash(),
                address: Default::default(),
                deadline: None,
                pool_id,
                location: angstrom_types::orders::OrderLocation::Limit
            },
            pool_id,
            valid_block: 0,
            tob_reward: U256::ZERO
        })
        .collect::<Vec<BookOrder>>();
    let bids = book
        .iter()
        .filter(|f| f.is_bid())
        .map(|bid| OrderWithStorageData {
            invalidates: vec![],
            order: bid.clone(),
            priority_data: OrderPriorityData {
                price:     *bid.price(),
                volume:    bid.amount(),
                gas:       U256::ZERO,
                gas_units: 0
            },
            is_bid: true,
            is_valid: true,
            is_currently_valid: true,
            order_id: OrderId {
                flash_block: None,
                reuse_avoidance: angstrom_types::sol_bindings::RespendAvoidanceMethod::Block(0),
                hash: bid.order_hash(),
                address: Default::default(),
                deadline: None,
                pool_id,
                location: angstrom_types::orders::OrderLocation::Limit
            },
            pool_id,
            valid_block: 0,
            tob_reward: U256::ZERO
        })
        .collect::<Vec<BookOrder>>();

    let tob = OrderWithStorageData {
        invalidates: vec![],
        order: tob.clone(),
        priority_data: OrderPriorityData {
            price:     tob.limit_price(),
            volume:    tob.amount(),
            gas:       U256::ZERO,
            gas_units: 0
        },
        is_bid: tob.is_bid(),
        is_valid: true,
        is_currently_valid: true,
        order_id: OrderId {
            flash_block: None,
            reuse_avoidance: angstrom_types::sol_bindings::RespendAvoidanceMethod::Block(0),
            hash: tob.order_hash(),
            address: Default::default(),
            deadline: None,
            pool_id,
            location: angstrom_types::orders::OrderLocation::Limit
        },
        pool_id,
        valid_block: 0,
        tob_reward: U256::ZERO
    };

    let book = OrderBook::new(pool_id, Some(amm), bids, asks, Some(SortStrategy::ByPriceByVolume));

    (book, tob)
}

pub fn tps(c: &mut Criterion) {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let pool = rt.block_on(setup_synced_pool_for_order_generation());

    let mut group = c.benchmark_group("Matching Engine TPS");
    let mut generator =
        PoolOrderGenerator::new_with_cfg_distro(PoolId::default(), pool.clone(), 0, 0.2);
    for bucket in TPS_BUCKETS {
        group.throughput(criterion::Throughput::Elements(bucket));
        // updates the random prices
        generator.new_block(1);
        let set = generator.generate_set(bucket as usize, 0.5);
        let (book, tob) = setup_inputs(set, &pool);

        group.bench_function(BenchmarkId::from_parameter(bucket), |bench| {
            bench.iter(|| black_box(BinarySearchStrategy::run(&book, Some(tob.clone()))));
        });
    }
}

criterion_group!(benches, tps);
criterion_main!(benches);
