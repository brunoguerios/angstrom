// Tests around building the Angstrom bundle

mod solutionlib;

use angstrom_types::{
    contract_payloads::{angstrom::AngstromBundle, asset::builder::AssetBuilder},
    matching::uniswap::{Direction, PoolPriceVec, PoolSnapshot, Quantity},
    orders::PoolSolution
};
use base64::Engine;
use solutionlib::{FROM_ABOVE, FROM_WILL, TICK_SPACING, WEIRD_SWAP};
use tracing::Level;

pub fn with_tracing<T>(f: impl FnOnce() -> T) -> T {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_line_number(true)
        .with_file(true)
        .finish();
    tracing::subscriber::with_default(subscriber, f)
}

#[test]
fn build_bundle() {
    with_tracing(|| {
        let bytes = base64::prelude::BASE64_STANDARD.decode(WEIRD_SWAP).unwrap();
        let (solution, orders_by_pool, snapshot, t0, t1, store_index, shared_gas): (
            PoolSolution,
            _,
            PoolSnapshot,
            _,
            _,
            _,
            _
        ) = serde_json::from_slice(&bytes).unwrap();

        let mut top_of_block_orders = Vec::new();
        let mut pool_updates = Vec::new();
        let mut pairs = Vec::new();
        let mut user_orders = Vec::new();
        let mut asset_builder = AssetBuilder::new();

        AngstromBundle::process_solution(
            &mut pairs,
            &mut asset_builder,
            &mut user_orders,
            &orders_by_pool,
            &mut top_of_block_orders,
            &mut pool_updates,
            &solution,
            &snapshot,
            t0,
            t1,
            store_index,
            shared_gas
        )
        .expect("Bundle processing failed");

        let bundle = AngstromBundle::new(
            asset_builder.get_asset_array(),
            pairs,
            pool_updates,
            top_of_block_orders,
            user_orders
        );
        let (direction, quantity) = if bundle.pool_updates[0].zero_for_one {
            (Direction::SellingT0, Quantity::Token0(bundle.pool_updates[0].swap_in_quantity))
        } else {
            (Direction::BuyingT0, Quantity::Token1(bundle.pool_updates[0].swap_in_quantity))
        };
        tracing::trace!(?direction, ?quantity, "Got values from bundle");
        let inspect_vec =
            PoolPriceVec::from_swap(snapshot.current_price(), direction, quantity).unwrap();
        println!(
            "Final price: {:#?} {}",
            inspect_vec.end_bound.price(),
            inspect_vec.end_bound.tick()
        );
        println!("Bundle: {:#?}", bundle);
        tracing::trace!(start_price = ?inspect_vec.start_bound.price(), end_price = ?inspect_vec.end_bound.price(), inspect_vec.d_t0, inspect_vec.d_t1, "Vec inspect");
        println!("Number of swaps: {}", inspect_vec.steps.as_ref().unwrap().len());
        println!("Number of updates: {}", bundle.pool_updates[0].rewards_update.quantities().len());
    })
}
