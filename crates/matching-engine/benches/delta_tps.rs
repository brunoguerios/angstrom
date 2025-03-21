use angstrom_types::primitive::PoolId;
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use testing_tools::order_generator::PoolOrderGenerator;
use uniswap_v4::uniswap::pool_manager::SyncedUniswapPool;

pub async fn setup_synced_pool_for_order_generation() -> SyncedUniswapPool {}

pub fn tps(c: &mut Criterion) {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let pool = rt.block_on(setup_synced_pool_for_order_generation());
    let generator = PoolOrderGenerator::new_with_cfg_distro(PoolId::default(), pool, 0, 0.2);
}

criterion_group!(benches, tps);
criterion_main!(benches);
