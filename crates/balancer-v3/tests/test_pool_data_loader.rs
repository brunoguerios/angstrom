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

    // VaultExplorer contract on Base
    let vault_explorer: Address = "0xaD89051bEd8d96f045E8912aE1672c6C0bF8a85E"
        .parse()
        .expect("Invalid vault explorer address");

    // Set up provider for Base mainnet
    let base_rpc_url =
        std::env::var("BASE_RPC_URL").unwrap_or_else(|_| "https://base.drpc.org".to_string());

    let provider = ProviderBuilder::new()
        .connect(&base_rpc_url)
        .await
        .expect("Failed to connect to Base RPC");

    let provider = Arc::new(provider);

    // Create data loader
    let data_loader = BalancerDataLoader::new(vault_explorer, pool_address);

    // Query pool data at the specific block
    let block_number: u64 = expected.block_number.parse().expect("Invalid block number");

    let pool_data = data_loader
        .load_pool_data(Some(block_number), provider)
        .await
        .expect("Failed to query pool data");

    // Validate queried data against expected test data
    println!("✓ Successfully queried pool data from Base");
    println!("  Block: {}", block_number);
    println!("  Pool: {}", pool_address);

    // Validate token configuration
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
    println!("  ✓ Tokens match ({} tokens)", pool_data.tokens.len());

    // Validate swap fees
    assert_eq!(pool_data.swap_fee.to_string(), expected.swap_fee, "Swap fee mismatch");
    assert_eq!(
        pool_data.aggregate_swap_fee.to_string(),
        expected.aggregate_swap_fee,
        "Aggregate swap fee mismatch"
    );
    println!("  ✓ Swap fees match");

    // Validate balances
    assert_eq!(
        pool_data.balances_live_scaled18.len(),
        expected.balances_live_scaled_18.len(),
        "Balance count mismatch"
    );
    for (i, (actual, expected)) in pool_data
        .balances_live_scaled18
        .iter()
        .zip(expected.balances_live_scaled_18.iter())
        .enumerate()
    {
        assert_eq!(actual.to_string(), *expected, "Balance {} mismatch", i);
    }
    println!("  ✓ Balances match");

    // Validate pool state flags
    assert_eq!(
        pool_data.is_pool_within_target_range, expected.is_pool_within_target_range,
        "Pool within target range flag mismatch"
    );
    println!("  ✓ Pool state flags match");

    // Validate ReClamm-specific parameters
    assert_eq!(
        pool_data.last_timestamp.to_string(),
        expected.last_timestamp,
        "Last timestamp mismatch"
    );
    assert_eq!(
        pool_data.daily_price_shift_base.to_string(),
        expected.daily_price_shift_base,
        "Daily price shift base mismatch"
    );
    assert_eq!(
        pool_data.centeredness_margin.to_string(),
        expected.centeredness_margin,
        "Centeredness margin mismatch"
    );
    println!("  ✓ ReClamm parameters match");

    println!("\n✅ All validations passed! Pool data loader is working correctly.");
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
