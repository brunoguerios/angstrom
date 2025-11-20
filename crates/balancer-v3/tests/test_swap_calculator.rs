//! Integration tests for Balancer swap calculator
//!
//! These tests validate swap calculations against reference data from
//! balancer-maths test fixtures.

use alloy::primitives::{Address, U256};
use angstrom_types::{
    amm::Price,
    balancer_structure::{BalancerPoolState, BalancerSwapCalculator},
    sol_bindings::Ray,
};
use balancer_v3::balancer::pool::ReClammPoolState;
use serde::Deserialize;

/// Test swap data structure from fixture
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TestSwap {
    swap_kind: u8,
    amount_raw: String,
    token_in: String,
    token_out: String,
    output_raw: String,
}

/// Test pool data structure from fixture
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TestPoolData {
    #[allow(dead_code)]
    chain_id: String,
    block_number: String,
    #[allow(dead_code)]
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
    is_pool_within_target_range: bool,
}

/// Complete test data structure
#[derive(Debug, Deserialize)]
struct TestData {
    swaps: Vec<TestSwap>,
    pool: TestPoolData,
}

/// Helper function to create ReClammPoolState from test fixture data
fn create_pool_from_fixture(test_pool: &TestPoolData) -> ReClammPoolState {
    let pool_address: Address = test_pool
        .pool_address
        .parse()
        .expect("Invalid pool address");
    let mut pool = ReClammPoolState::new(pool_address);

    // Parse tokens
    pool.tokens = test_pool
        .tokens
        .iter()
        .map(|t| t.parse().expect("Invalid token address"))
        .collect();

    // Parse numeric fields
    pool.scaling_factors = test_pool
        .scaling_factors
        .iter()
        .map(|s| s.parse().expect("Invalid scaling factor"))
        .collect();
    pool.token_rates = test_pool
        .token_rates
        .iter()
        .map(|r| r.parse().expect("Invalid token rate"))
        .collect();
    pool.balances = test_pool
        .balances_live_scaled_18
        .iter()
        .map(|b| b.parse().expect("Invalid balance"))
        .collect();
    pool.swap_fee = test_pool.swap_fee.parse().expect("Invalid swap fee");
    pool.aggregate_swap_fee = test_pool
        .aggregate_swap_fee
        .parse()
        .expect("Invalid aggregate swap fee");
    pool.total_supply = test_pool
        .total_supply
        .parse()
        .expect("Invalid total supply");
    pool.last_timestamp = test_pool
        .last_timestamp
        .parse()
        .expect("Invalid last timestamp");
    pool.last_virtual_balances = test_pool
        .last_virtual_balances
        .iter()
        .map(|v| v.parse().expect("Invalid virtual balance"))
        .collect();
    pool.current_virtual_balances = test_pool
        .current_virtual_balances
        .iter()
        .map(|v| v.parse().expect("Invalid virtual balance"))
        .collect();
    pool.daily_price_shift_base = test_pool
        .daily_price_shift_base
        .parse()
        .expect("Invalid daily price shift base");
    pool.centeredness_margin = test_pool
        .centeredness_margin
        .parse()
        .expect("Invalid centeredness margin");
    pool.start_fourth_root_price_ratio = test_pool
        .start_fourth_root_price_ratio
        .parse()
        .expect("Invalid start fourth root price ratio");
    pool.end_fourth_root_price_ratio = test_pool
        .end_fourth_root_price_ratio
        .parse()
        .expect("Invalid end fourth root price ratio");
    pool.price_ratio_update_start_time = test_pool
        .price_ratio_update_start_time
        .parse()
        .expect("Invalid price ratio update start time");
    pool.price_ratio_update_end_time = test_pool
        .price_ratio_update_end_time
        .parse()
        .expect("Invalid price ratio update end time");
    pool.current_timestamp = test_pool
        .current_timestamp
        .parse()
        .expect("Invalid current timestamp");
    pool.is_pool_within_target_range = test_pool.is_pool_within_target_range;
    pool.is_pool_initialized = true;
    pool.is_pool_paused = false;
    pool.is_pool_in_recovery_mode = false;

    // Set block number
    pool.block_number = test_pool
        .block_number
        .parse()
        .expect("Invalid block number");

    pool
}

/// Test swap calculations against reference data from fixture
#[test]
fn test_swap_calculations_against_fixture() {
    // Load test data
    let test_data_json = include_str!("fixtures/8453-31094200-ReClamm-WETH-USDC-In-Range.json");
    let test_data: TestData =
        serde_json::from_str(test_data_json).expect("Failed to parse test data JSON");

    // Create pool state from fixture
    let reclamm_pool = create_pool_from_fixture(&test_data.pool);
    // Construct BalancerPoolState for swap calculations
    let pool = BalancerPoolState::ReClamm {
        pool_address: reclamm_pool.pool_address,
        block_number: reclamm_pool.block_number,
        tokens: reclamm_pool.tokens.clone(),
        scaling_factors: reclamm_pool.scaling_factors.clone(),
        token_rates: reclamm_pool.token_rates.clone(),
        balances_live_scaled_18: reclamm_pool.balances.clone(),
        swap_fee: reclamm_pool.swap_fee,
        aggregate_swap_fee: reclamm_pool.aggregate_swap_fee,
        total_supply: reclamm_pool.total_supply,
        last_timestamp: reclamm_pool.last_timestamp,
        last_virtual_balances: reclamm_pool.last_virtual_balances.clone(),
        current_virtual_balances: reclamm_pool.current_virtual_balances.clone(),
        daily_price_shift_exponent: reclamm_pool.daily_price_shift_exponent,
        daily_price_shift_base: reclamm_pool.daily_price_shift_base,
        centeredness_margin: reclamm_pool.centeredness_margin,
        current_price_ratio: reclamm_pool.current_price_ratio,
        current_fourth_root_price_ratio: reclamm_pool.current_fourth_root_price_ratio,
        start_fourth_root_price_ratio: reclamm_pool.start_fourth_root_price_ratio,
        end_fourth_root_price_ratio: reclamm_pool.end_fourth_root_price_ratio,
        price_ratio_update_start_time: reclamm_pool.price_ratio_update_start_time,
        price_ratio_update_end_time: reclamm_pool.price_ratio_update_end_time,
        is_pool_initialized: reclamm_pool.is_pool_initialized,
        is_pool_paused: reclamm_pool.is_pool_paused,
        is_pool_in_recovery_mode: reclamm_pool.is_pool_in_recovery_mode,
        is_pool_within_target_range: reclamm_pool.is_pool_within_target_range,
        current_timestamp: reclamm_pool.current_timestamp,
    };

    println!("✓ Created pool state from fixture");
    println!("  Pool: {}", test_data.pool.pool_address);
    println!("  Block: {}", test_data.pool.block_number);
    println!("  Testing {} swaps", test_data.swaps.len());

    // Test each swap
    for (i, swap) in test_data.swaps.iter().enumerate() {
        let token_in: Address = swap.token_in.parse().expect("Invalid token_in");
        let token_out: Address = swap.token_out.parse().expect("Invalid token_out");
        let amount_raw: U256 = swap.amount_raw.parse().expect("Invalid amount_raw");
        let expected_output: U256 = swap.output_raw.parse().expect("Invalid output_raw");

        let result = match swap.swap_kind {
            0 => {
                // GivenIn: calculate amount out
                BalancerSwapCalculator::get_amount_out(&pool, amount_raw, token_in, token_out)
            }
            1 => {
                // GivenOut: calculate amount in
                BalancerSwapCalculator::get_amount_in(&pool, amount_raw, token_in, token_out)
            }
            _ => {
                panic!("Unknown swap kind: {}", swap.swap_kind);
            }
        };

        match result {
            Ok(actual_output) => {
                assert_eq!(
                    actual_output, expected_output,
                    "Swap {} mismatch: expected {}, got {}",
                    i, expected_output, actual_output
                );
                println!(
                    "  ✓ Swap {}: {} -> {} (kind: {})",
                    i, amount_raw, actual_output, swap.swap_kind
                );
            }
            Err(e) => {
                panic!("Swap {} failed: {:?}", i, e);
            }
        }
    }

    println!("✅ All {} swaps validated successfully", test_data.swaps.len());
}

/// Test swap_given_in specifically
#[test]
fn test_swap_given_in() {
    let test_data_json = include_str!("fixtures/8453-31094200-ReClamm-WETH-USDC-In-Range.json");
    let test_data: TestData =
        serde_json::from_str(test_data_json).expect("Failed to parse test data JSON");

    let reclamm_pool = create_pool_from_fixture(&test_data.pool);
    let pool = BalancerPoolState::ReClamm {
        pool_address: reclamm_pool.pool_address,
        block_number: reclamm_pool.block_number,
        tokens: reclamm_pool.tokens.clone(),
        scaling_factors: reclamm_pool.scaling_factors.clone(),
        token_rates: reclamm_pool.token_rates.clone(),
        balances_live_scaled_18: reclamm_pool.balances.clone(),
        swap_fee: reclamm_pool.swap_fee,
        aggregate_swap_fee: reclamm_pool.aggregate_swap_fee,
        total_supply: reclamm_pool.total_supply,
        last_timestamp: reclamm_pool.last_timestamp,
        last_virtual_balances: reclamm_pool.last_virtual_balances.clone(),
        current_virtual_balances: reclamm_pool.current_virtual_balances.clone(),
        daily_price_shift_exponent: reclamm_pool.daily_price_shift_exponent,
        daily_price_shift_base: reclamm_pool.daily_price_shift_base,
        centeredness_margin: reclamm_pool.centeredness_margin,
        current_price_ratio: reclamm_pool.current_price_ratio,
        current_fourth_root_price_ratio: reclamm_pool.current_fourth_root_price_ratio,
        start_fourth_root_price_ratio: reclamm_pool.start_fourth_root_price_ratio,
        end_fourth_root_price_ratio: reclamm_pool.end_fourth_root_price_ratio,
        price_ratio_update_start_time: reclamm_pool.price_ratio_update_start_time,
        price_ratio_update_end_time: reclamm_pool.price_ratio_update_end_time,
        is_pool_initialized: reclamm_pool.is_pool_initialized,
        is_pool_paused: reclamm_pool.is_pool_paused,
        is_pool_in_recovery_mode: reclamm_pool.is_pool_in_recovery_mode,
        is_pool_within_target_range: reclamm_pool.is_pool_within_target_range,
        current_timestamp: reclamm_pool.current_timestamp,
    };

    // Find a GivenIn swap (swapKind: 0)
    let given_in_swap = test_data
        .swaps
        .iter()
        .find(|s| s.swap_kind == 0)
        .expect("No GivenIn swap found in fixture");

    let token_in: Address = given_in_swap.token_in.parse().expect("Invalid token_in");
    let token_out: Address = given_in_swap.token_out.parse().expect("Invalid token_out");
    let amount_in: U256 = given_in_swap
        .amount_raw
        .parse()
        .expect("Invalid amount_raw");
    let expected_out: U256 = given_in_swap
        .output_raw
        .parse()
        .expect("Invalid output_raw");

    let amount_out = BalancerSwapCalculator::get_amount_out(&pool, amount_in, token_in, token_out)
        .expect("Swap calculation failed");

    assert_eq!(amount_out, expected_out, "GivenIn swap output mismatch");
    assert!(amount_out > U256::ZERO, "Output should be positive");
}

/// Test swap_given_out specifically
#[test]
fn test_swap_given_out() {
    let test_data_json = include_str!("fixtures/8453-31094200-ReClamm-WETH-USDC-In-Range.json");
    let test_data: TestData =
        serde_json::from_str(test_data_json).expect("Failed to parse test data JSON");

    let reclamm_pool = create_pool_from_fixture(&test_data.pool);
    let pool = BalancerPoolState::ReClamm {
        pool_address: reclamm_pool.pool_address,
        block_number: reclamm_pool.block_number,
        tokens: reclamm_pool.tokens.clone(),
        scaling_factors: reclamm_pool.scaling_factors.clone(),
        token_rates: reclamm_pool.token_rates.clone(),
        balances_live_scaled_18: reclamm_pool.balances.clone(),
        swap_fee: reclamm_pool.swap_fee,
        aggregate_swap_fee: reclamm_pool.aggregate_swap_fee,
        total_supply: reclamm_pool.total_supply,
        last_timestamp: reclamm_pool.last_timestamp,
        last_virtual_balances: reclamm_pool.last_virtual_balances.clone(),
        current_virtual_balances: reclamm_pool.current_virtual_balances.clone(),
        daily_price_shift_exponent: reclamm_pool.daily_price_shift_exponent,
        daily_price_shift_base: reclamm_pool.daily_price_shift_base,
        centeredness_margin: reclamm_pool.centeredness_margin,
        current_price_ratio: reclamm_pool.current_price_ratio,
        current_fourth_root_price_ratio: reclamm_pool.current_fourth_root_price_ratio,
        start_fourth_root_price_ratio: reclamm_pool.start_fourth_root_price_ratio,
        end_fourth_root_price_ratio: reclamm_pool.end_fourth_root_price_ratio,
        price_ratio_update_start_time: reclamm_pool.price_ratio_update_start_time,
        price_ratio_update_end_time: reclamm_pool.price_ratio_update_end_time,
        is_pool_initialized: reclamm_pool.is_pool_initialized,
        is_pool_paused: reclamm_pool.is_pool_paused,
        is_pool_in_recovery_mode: reclamm_pool.is_pool_in_recovery_mode,
        is_pool_within_target_range: reclamm_pool.is_pool_within_target_range,
        current_timestamp: reclamm_pool.current_timestamp,
    };

    // Find a GivenOut swap (swapKind: 1)
    let given_out_swap = test_data
        .swaps
        .iter()
        .find(|s| s.swap_kind == 1)
        .expect("No GivenOut swap found in fixture");

    let token_in: Address = given_out_swap.token_in.parse().expect("Invalid token_in");
    let token_out: Address = given_out_swap.token_out.parse().expect("Invalid token_out");
    let amount_out: U256 = given_out_swap
        .amount_raw
        .parse()
        .expect("Invalid amount_raw");
    let expected_in: U256 = given_out_swap
        .output_raw
        .parse()
        .expect("Invalid output_raw");

    let amount_in = BalancerSwapCalculator::get_amount_in(&pool, amount_out, token_in, token_out)
        .expect("Swap calculation failed");

    assert_eq!(amount_in, expected_in, "GivenOut swap input mismatch");
    assert!(amount_in > U256::ZERO, "Input should be positive");
}
