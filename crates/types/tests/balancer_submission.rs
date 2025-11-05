//! Integration tests for Balancer submission builder
//!
//! These tests verify the end-to-end flow of building and submitting
//! Balancer proposals.

use alloy::primitives::{Address, FixedBytes, U256};
use angstrom_types::{
    contract_payloads::{
        Asset, Pair, Signature,
        angstrom::{OrderQuantities, TopOfBlockOrder, UserOrder},
        balancer::{BalancerSubmissionBuilder, ProposalParams}
    },
    orders::{NetAmmOrder, PoolSolution},
    primitive::BalancerPoolRegistry,
    submission::SubmissionPayload
};

/// Test that we can build a complete ProposalParams from realistic consensus
/// outputs
#[test]
fn test_end_to_end_proposal_building() {
    // Setup registry with test pool
    let mut registry = BalancerPoolRegistry::new();
    let pool_id = FixedBytes::from([1u8; 32]);
    let pool_addr = Address::from([0x10u8; 20]);
    let token0 = Address::from([0x20u8; 20]);
    let token1 = Address::from([0x30u8; 20]);
    registry.add_pool(pool_id, pool_addr, token0, token1);

    // Create assets
    let assets = vec![
        Asset { addr: token0, save: 1000, take: 500, settle: 500 },
        Asset { addr: token1, save: 2000, take: 1000, settle: 1000 },
    ];

    // Create pair
    let pair = Pair {
        index0:       0,
        index1:       1,
        store_index:  0,
        price_1over0: U256::from(1000000000000000000u128)
    };

    // Create pool solution with AMM swap
    let solution = PoolSolution {
        id:           pool_id,
        ucp:          angstrom_types::matching::Ray::from(1000000000000000000u128),
        searcher:     None,
        amm_quantity: Some(NetAmmOrder::Sell(1000, 950)),
        limit:        vec![],
        reward_t0:    100,
        fee:          3000
    };

    // Create ToB order
    let tob = TopOfBlockOrder {
        use_internal:     false,
        quantity_in:      1000,
        quantity_out:     900,
        max_gas_asset_0:  50,
        gas_used_asset_0: 10,
        pairs_index:      0,
        zero_for_1:       true,
        recipient:        None,
        signature:        Signature::Ecdsa { v: 27, r: [1u8; 32].into(), s: [2u8; 32].into() }
    };

    // Create user order
    let user_order = UserOrder {
        ref_id:               1,
        use_internal:         false,
        pair_index:           0,
        min_price:            U256::from(900000000000000000u128),
        recipient:            None,
        hook_data:            None,
        zero_for_one:         true,
        standing_validation:  None,
        order_quantities:     OrderQuantities::Exact { quantity: 500 },
        max_extra_fee_asset0: 25,
        extra_fee_asset0:     5,
        exact_in:             true,
        signature:            Signature::Ecdsa { v: 27, r: [3u8; 32].into(), s: [4u8; 32].into() }
    };

    // Build proposal params
    let result = BalancerSubmissionBuilder::from_proposal(
        &[solution],
        &[tob],
        &[user_order],
        &registry,
        &[pair],
        &assets
    );

    assert!(result.is_ok(), "Failed to build proposal: {:?}", result.err());
    let params = result.unwrap();

    // Verify all components are present
    assert_eq!(params.pairs.len(), 1, "Should have 1 pair");
    assert_eq!(params.tob_orders.len(), 1, "Should have 1 ToB order");
    assert_eq!(params.user_orders.len(), 1, "Should have 1 user order");

    // Verify pair params
    assert_eq!(params.pairs[0].pool_address, pool_addr);
    assert_eq!(params.pairs[0].token_in, token0);
    assert_eq!(params.pairs[0].token_out, token1);
    assert_eq!(params.pairs[0].exact_amount_in, 1000);

    // Verify ToB order params
    assert_eq!(params.tob_orders[0].token_in, token0);
    assert_eq!(params.tob_orders[0].token_out, token1);
    assert_eq!(params.tob_orders[0].exact_amount_in, 1000);
    assert_eq!(params.tob_orders[0].exact_amount_out, 900);
    assert_eq!(params.tob_orders[0].max_gas_asset0, 50);
    assert_eq!(params.tob_orders[0].signature.len(), 65);

    // Verify user order params
    assert_eq!(params.user_orders[0].token_in, token0);
    assert_eq!(params.user_orders[0].token_out, token1);
    assert_eq!(params.user_orders[0].exact_in, true);
    assert_eq!(params.user_orders[0].amount, 500);
    assert_eq!(params.user_orders[0].max_extra_fee_asset0, 25);
    assert_eq!(params.user_orders[0].signature.len(), 65);
}

/// Test that ProposalParams can be wrapped in SubmissionPayload
#[test]
fn test_submission_payload_balancer() {
    let params = ProposalParams { pairs: vec![], tob_orders: vec![], user_orders: vec![] };

    let payload = SubmissionPayload::BalancerBundle(params);

    match payload {
        SubmissionPayload::BalancerBundle(p) => {
            assert_eq!(p.pairs.len(), 0);
            assert_eq!(p.tob_orders.len(), 0);
            assert_eq!(p.user_orders.len(), 0);
        }
        _ => panic!("Expected BalancerParams variant")
    }
}

/// Test multi-pool scenario
#[test]
fn test_multiple_pools() {
    let mut registry = BalancerPoolRegistry::new();

    // Add two pools
    let pool_id1 = FixedBytes::from([1u8; 32]);
    let pool_id2 = FixedBytes::from([2u8; 32]);

    registry.add_pool(
        pool_id1,
        Address::from([0x10u8; 20]),
        Address::from([0x20u8; 20]),
        Address::from([0x30u8; 20])
    );

    registry.add_pool(
        pool_id2,
        Address::from([0x11u8; 20]),
        Address::from([0x21u8; 20]),
        Address::from([0x31u8; 20])
    );

    let assets = vec![
        Asset { addr: Address::from([0x20u8; 20]), save: 0, take: 0, settle: 0 },
        Asset { addr: Address::from([0x30u8; 20]), save: 0, take: 0, settle: 0 },
        Asset { addr: Address::from([0x21u8; 20]), save: 0, take: 0, settle: 0 },
        Asset { addr: Address::from([0x31u8; 20]), save: 0, take: 0, settle: 0 },
    ];

    let solutions = vec![
        PoolSolution {
            id:           pool_id1,
            ucp:          angstrom_types::matching::Ray::default(),
            searcher:     None,
            amm_quantity: Some(NetAmmOrder::Sell(1000, 950)),
            limit:        vec![],
            reward_t0:    50,
            fee:          3000
        },
        PoolSolution {
            id:           pool_id2,
            ucp:          angstrom_types::matching::Ray::default(),
            searcher:     None,
            amm_quantity: Some(NetAmmOrder::Buy(800, 850)),
            limit:        vec![],
            reward_t0:    40,
            fee:          3000
        },
    ];

    let result =
        BalancerSubmissionBuilder::from_proposal(&solutions, &[], &[], &registry, &[], &assets);

    assert!(result.is_ok());
    let params = result.unwrap();
    assert_eq!(params.pairs.len(), 2, "Should have 2 pairs");
}

/// Test that signature encoding is consistent
#[test]
fn test_signature_consistency() {
    let mut registry = BalancerPoolRegistry::new();
    let pool_id = FixedBytes::from([1u8; 32]);
    registry.add_pool(
        pool_id,
        Address::from([0x10u8; 20]),
        Address::from([0x20u8; 20]),
        Address::from([0x30u8; 20])
    );

    let assets = vec![
        Asset { addr: Address::from([0x20u8; 20]), save: 0, take: 0, settle: 0 },
        Asset { addr: Address::from([0x30u8; 20]), save: 0, take: 0, settle: 0 },
    ];

    let pair = Pair {
        index0:       0,
        index1:       1,
        store_index:  0,
        price_1over0: U256::from(1000000000000000000u128)
    };

    // Test with ECDSA signature
    let tob_ecdsa = TopOfBlockOrder {
        use_internal:     false,
        quantity_in:      1000,
        quantity_out:     900,
        max_gas_asset_0:  50,
        gas_used_asset_0: 0,
        pairs_index:      0,
        zero_for_1:       true,
        recipient:        None,
        signature:        Signature::Ecdsa {
            v: 27,
            r: [0xAAu8; 32].into(),
            s: [0xBBu8; 32].into()
        }
    };

    let result = BalancerSubmissionBuilder::from_proposal(
        &[],
        &[tob_ecdsa],
        &[],
        &registry,
        &[pair.clone()],
        &assets
    );

    assert!(result.is_ok());
    let params = result.unwrap();
    assert_eq!(params.tob_orders[0].signature.len(), 65);
    assert_eq!(params.tob_orders[0].signature[0], 0xAA);
    assert_eq!(params.tob_orders[0].signature[32], 0xBB);
    assert_eq!(params.tob_orders[0].signature[64], 27);
}

/// Test error handling for mismatched order and pair counts
#[test]
fn test_mismatched_counts() {
    let registry = BalancerPoolRegistry::new();
    let assets = vec![];

    let tob = TopOfBlockOrder {
        use_internal:     false,
        quantity_in:      1000,
        quantity_out:     900,
        max_gas_asset_0:  50,
        gas_used_asset_0: 0,
        pairs_index:      0,
        zero_for_1:       true,
        recipient:        None,
        signature:        Signature::default()
    };

    // ToB order without corresponding pair should not panic
    let result = BalancerSubmissionBuilder::from_proposal(
        &[],
        &[tob],
        &[],
        &registry,
        &[], // No pairs
        &assets
    );

    // Should succeed with empty results (zip stops at shorter iterator)
    assert!(result.is_ok());
    let params = result.unwrap();
    assert_eq!(params.tob_orders.len(), 0);
}
