use alloy::primitives::U256;
use alloy_primitives::FixedBytes;
use angstrom_types::matching::{uniswap::PoolSnapshot, Ray};
use matching_engine::{book::OrderBook, matcher::VolumeFillMatcher};
use testing_tools::type_generator::{
    amm::generate_single_position_amm_at_tick, orders::UserOrderBuilder
};

struct TestOrder {
    q: u128,
    p: Ray
}

fn make_books(
    bids_raw: Vec<TestOrder>,
    asks_raw: Vec<TestOrder>,
    amm: Option<PoolSnapshot>
) -> OrderBook {
    let bids = bids_raw
        .iter()
        .map(|b| {
            UserOrderBuilder::new()
                .amount(b.q)
                .min_price(b.p)
                .with_storage()
                .bid()
                .build()
        })
        .collect();
    let asks = asks_raw
        .iter()
        .map(|a| {
            UserOrderBuilder::new()
                .amount(a.q)
                .min_price(a.p)
                .with_storage()
                .ask()
                .build()
        })
        .collect();
    OrderBook::new(
        FixedBytes::random(),
        amm,
        bids,
        asks,
        Some(matching_engine::book::sort::SortStrategy::ByPriceByVolume)
    )
}

fn raw_price(p: u128) -> Ray {
    Ray::from(U256::from(p))
}

#[test]
fn simple_book() {
    // Simple book where we expect both orders to fill and annihilate
    let book = make_books(
        vec![TestOrder { q: 100, p: raw_price(100) }],
        vec![TestOrder { q: 100, p: raw_price(10) }],
        None
    );
    let mut matcher = VolumeFillMatcher::new(&book);
    let end = matcher.run_match();
    println!("End reason: {:?}", end);
    let solution = matcher.solution(None);
    assert!(solution.limit.iter().all(|outcome| outcome.is_filled()), "All orders not filled");
}

#[test]
fn unsolveable_book() {
    // Simple book where we can't fill anything because both orders don't have the
    // right size
    let book = make_books(
        vec![TestOrder { q: 80, p: raw_price(100) }],
        vec![TestOrder { q: 100, p: raw_price(10) }],
        None
    );
    let mut matcher = VolumeFillMatcher::new(&book);
    let end = matcher.run_match();
    println!("End reason: {:?}", end);
    let solution = matcher.solution(None);
    assert!(solution.limit.iter().all(|outcome| !outcome.is_filled()), "All orders not unfilled");
}

#[test]
fn multiple_orders_fill() {
    // Multiple orders of different sizes will fill each other and result in a
    // cleared book
    let book = make_books(
        vec![
            TestOrder { q: 100, p: raw_price(100) },
            TestOrder { q: 20, p: raw_price(101) },
            TestOrder { q: 70, p: raw_price(102) },
            TestOrder { q: 210, p: raw_price(130) },
        ],
        vec![
            TestOrder { q: 25, p: raw_price(10) },
            TestOrder { q: 190, p: raw_price(11) },
            TestOrder { q: 15, p: raw_price(12) },
            TestOrder { q: 170, p: raw_price(13) },
        ],
        None
    );
    let mut matcher = VolumeFillMatcher::new(&book);
    let end = matcher.run_match();
    println!("End reason: {:?}", end);
    let solution = matcher.solution(None);
    assert!(solution.limit.iter().all(|outcome| outcome.is_filled()), "All orders not filled");
}

#[test]
fn fill_from_amm() {
    let amm = generate_single_position_amm_at_tick(100000, 100, 1_000_000_000_000_000_u128);
    let book = make_books(vec![], vec![TestOrder { q: 100, p: raw_price(100) }], Some(amm));
    let mut matcher = VolumeFillMatcher::new(&book);
    let end = matcher.run_match();
    println!("End reason: {:?}", end);
    let solution = matcher.solution(None);
    println!("Solution:\n{:?}", solution);
    assert!(solution.limit.iter().all(|outcome| outcome.is_filled()), "All orders not filled");
}
