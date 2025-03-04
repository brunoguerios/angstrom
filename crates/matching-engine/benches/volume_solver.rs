use alloy::primitives::FixedBytes;
use matching_engine::strategy::{MatchingStrategy, SimpleCheckpointStrategy};
use rand::{Rng, thread_rng};
use testing_tools::type_generator::book::{generate_one_sided_book, generate_simple_cross_book};

const ORDER_COUNT: &[usize] = &[1, 10, 100, 1000];

static CENTER_PRICE: f64 = 100_000_000.0;

fn main() {
    divan::main();
}

#[divan::bench(consts = ORDER_COUNT)]
fn simple_cross_book<const N: usize>(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            let pool_id = FixedBytes::<32>::random();
            generate_simple_cross_book(pool_id, N, CENTER_PRICE)
        })
        .bench_refs(|book| SimpleCheckpointStrategy::run(book).map(|s| s.solution(None)));
}

#[divan::bench(consts = ORDER_COUNT)]
fn one_sided_book<const N: usize>(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            let mut rnd = thread_rng();
            let pool_id = FixedBytes::<32>::random();
            generate_one_sided_book(rnd.r#gen(), pool_id, N, CENTER_PRICE)
        })
        .bench_refs(|book| SimpleCheckpointStrategy::run(book).map(|s| s.solution(None)));
}
