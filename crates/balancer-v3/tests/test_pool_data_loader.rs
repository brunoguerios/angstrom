//! Integration tests for Balancer pool data loader
//!
//! These tests query real deployed contracts and validate against known test
//! data

use std::sync::Arc;

use alloy::{primitives::Address, providers::ProviderBuilder};
use balancer_v3::balancer::pool_data_loader::{BalancerDataLoader, BalancerPoolDataLoader};
use serde::{Deserialize, Serialize};

/// Test data structure matching balancer-maths testData format
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct TestPoolData {
    chain_id: String,
    block_number: String,
    pool_type: String,
    pool_address: String,
    tokens: Vec<String>,
    scaling_factors: Vec<String>,
    aggregate_swap_fee: String,
    swap_fee: String,
    total_supply: String,
    balances_live_scaled_18: Vec<String>,
    token_rates: Vec<String>,
    last_timestamp: String,
    last_virtual_balances: Vec<String>,
    current_virtual_balances: Vec<String>,
    daily_price_shift_base: String,
    centeredness_margin: String,
    start_fourth_root_price_ratio: String,
    end_fourth_root_price_ratio: String,
    price_ratio_update_start_time: String,
    price_ratio_update_end_time: String,
    current_timestamp: String,
    is_pool_within_target_range: bool
}

#[derive(Debug, Deserialize)]
struct TestData {
    pool: TestPoolData
}

/// Test ReClamm pool data loader against Base mainnet pool
///
/// Pool: WETH-USDC on Base
/// Block: 31094200
/// Test data from:
/// balancer-maths/testData/8453-31094200-ReClamm-WETH-USDC-In-Range.json
#[tokio::test]
#[ignore = "requires Base RPC endpoint - run with: cargo test --test test_pool_data_loader -- \
            --ignored"]
async fn test_reclamm_pool_data_loader_base() {
    // Load test data
    let test_data_json = include_str!("fixtures/8453-31094200-ReClamm-WETH-USDC-In-Range.json");
    let test_data: TestData =
        serde_json::from_str(test_data_json).expect("Failed to parse test data JSON");

    let expected = test_data.pool;

    // Parse addresses
    let pool_address: Address = expected.pool_address.parse().expect("Invalid pool address");

    // Set up provider for Base mainnet
    let base_rpc_url =
        std::env::var("BASE_RPC_URL").unwrap_or_else(|_| "https://base.drpc.org".to_string());

    let provider = ProviderBuilder::new()
        .connect(&base_rpc_url)
        .await
        .expect("Failed to connect to Base RPC");

    let provider = Arc::new(provider);

    // Create data loader
    let data_loader = BalancerDataLoader::new(pool_address);

    // Query pool data at the specific block
    let block_number: u64 = expected.block_number.parse().expect("Invalid block number");

    let pool_data = data_loader
        .load_pool_data(Some(block_number), provider)
        .await
        .expect("Failed to query pool data");

    // Helper macro to validate arrays match
    macro_rules! validate_array {
        ($actual:expr, $expected:expr, $name:expr) => {
            assert_eq!($actual.len(), $expected.len(), "{} count mismatch", $name);
            for (i, (actual, expected)) in $actual.iter().zip($expected.iter()).enumerate() {
                assert_eq!(actual.to_string(), *expected, "{} {} mismatch", $name, i);
            }
        };
    }

    // Helper macro to validate single values match
    macro_rules! validate_value {
        ($actual:expr, $expected:expr, $name:expr) => {
            assert_eq!($actual.to_string(), $expected, "{} mismatch", $name);
        };
    }

    println!("✓ Queried pool data from Base (block: {}, pool: {})", block_number, pool_address);

    // Validate tokens
    assert_eq!(pool_data.tokens.len(), expected.tokens.len(), "Token count mismatch");
    for (i, (actual, expected)) in pool_data
        .tokens
        .iter()
        .zip(expected.tokens.iter())
        .enumerate()
    {
        let expected_addr: Address = expected.parse().expect("Invalid expected token address");
        assert_eq!(actual, &expected_addr, "Token {} address mismatch", i);
    }

    // Validate all numeric fields
    validate_value!(pool_data.swap_fee, expected.swap_fee, "Swap fee");
    validate_value!(
        pool_data.aggregate_swap_fee,
        expected.aggregate_swap_fee,
        "Aggregate swap fee"
    );
    validate_array!(pool_data.balances_live_scaled18, expected.balances_live_scaled_18, "Balance");
    validate_array!(
        pool_data.last_virtual_balances,
        expected.last_virtual_balances,
        "Last virtual balance"
    );
    validate_array!(
        pool_data.current_virtual_balances,
        expected.current_virtual_balances,
        "Current virtual balance"
    );
    validate_value!(pool_data.total_supply, expected.total_supply, "Total supply");
    validate_array!(pool_data.token_rates, expected.token_rates, "Token rate");
    validate_array!(pool_data.scaling_factors, expected.scaling_factors, "Scaling factor");

    assert_eq!(
        pool_data.is_pool_within_target_range, expected.is_pool_within_target_range,
        "Pool within target range flag mismatch"
    );

    validate_value!(pool_data.last_timestamp, expected.last_timestamp, "Last timestamp");
    validate_value!(
        pool_data.daily_price_shift_base,
        expected.daily_price_shift_base,
        "Daily price shift base"
    );
    validate_value!(
        pool_data.centeredness_margin,
        expected.centeredness_margin,
        "Centeredness margin"
    );
    validate_value!(
        pool_data.start_fourth_root_price_ratio,
        expected.start_fourth_root_price_ratio,
        "Start fourth root price ratio"
    );
    validate_value!(
        pool_data.end_fourth_root_price_ratio,
        expected.end_fourth_root_price_ratio,
        "End fourth root price ratio"
    );
    validate_value!(
        pool_data.price_ratio_update_start_time,
        expected.price_ratio_update_start_time,
        "Price ratio update start time"
    );
    validate_value!(
        pool_data.price_ratio_update_end_time,
        expected.price_ratio_update_end_time,
        "Price ratio update end time"
    );

    println!("✅ All {} fields validated successfully", 17);
}

/// Helper test to verify test data can be parsed correctly
#[test]
fn test_parse_test_data() {
    let test_data_json = include_str!("fixtures/8453-31094200-ReClamm-WETH-USDC-In-Range.json");
    let test_data: TestData =
        serde_json::from_str(test_data_json).expect("Failed to parse test data JSON");

    let pool = test_data.pool;

    // Verify basic structure
    assert_eq!(pool.chain_id, "8453");
    assert_eq!(pool.block_number, "31094200");
    assert_eq!(pool.pool_type, "RECLAMM");
    assert_eq!(pool.pool_address, "0xBa615a0A9237b64BFb3051f8160483C10Dde0012");
    assert_eq!(pool.tokens.len(), 2);
    assert_eq!(pool.balances_live_scaled_18.len(), 2);
    assert!(pool.is_pool_within_target_range);

    println!("✓ Test data parsed successfully");
    println!("  Pool: {} on chain {}", pool.pool_address, pool.chain_id);
    println!("  Block: {}", pool.block_number);
    println!("  Tokens: {} tokens", pool.tokens.len());
}
