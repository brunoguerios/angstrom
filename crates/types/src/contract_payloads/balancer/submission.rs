//! Balancer submission builder and types for explicit-params ABI v2
//!
//! This module implements the Balancer-specific submission path that sends
//! explicit parameters instead of encoded bundles.

use alloy::primitives::{Address, Bytes, U256};
use eyre::Result;
use serde::{Deserialize, Serialize};

use crate::{
    contract_payloads::{
        Asset, Pair, Signature,
        angstrom::{TopOfBlockOrder, UserOrder}
    },
    orders::PoolSolution,
    primitive::BalancerPoolRegistry
};

/// Complete proposal parameters for Balancer submission
///
/// Contains all orders and pool updates for a single block execution.
/// Unlike the Uniswap bundle which is encoded, these parameters are passed
/// explicitly to the contract's execute function.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalParams {
    /// Per-pair aggregate data (one per touched pair)
    pub pairs:       Vec<PairParams>,
    /// Top-of-block searcher orders
    pub tob_orders:  Vec<ToBOrderParams>,
    /// User limit orders
    pub user_orders: Vec<UserOrderParams>
}

/// Per-pair parameters including pool identity and net swap
///
/// Represents the aggregate state change for one trading pair.
/// The donation amount is calculated implicitly by the contract from ToB
/// surplus.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairParams {
    /// Balancer V3 pool address
    pub pool_address:    Address,
    /// Input token for the net swap
    pub token_in:        Address,
    /// Exact input amount (net of ToB + book swaps)
    pub exact_amount_in: u128,
    /// Output token for the net swap
    pub token_out:       Address
}

/// Top-of-block searcher order parameters
///
/// Represents a winning ToB bid with exact amounts.
/// The donation is calculated implicitly as: donation = exact_amount_in -
/// fair_swap_cost
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToBOrderParams {
    /// Token the searcher pays
    pub token_in:         Address,
    /// Token the searcher receives
    pub token_out:        Address,
    /// Exact amount searcher pays (includes donation surplus)
    pub exact_amount_in:  u128,
    /// Exact amount searcher receives
    pub exact_amount_out: u128,
    /// Maximum gas fee in asset0 (protects searcher)
    pub max_gas_asset0:   u128,
    /// Searcher's ECDSA or ERC-1271 signature
    pub signature:        Bytes
}

/// User limit order parameters
///
/// Supports both exactIn and exactOut order types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserOrderParams {
    /// Input token
    pub token_in:             Address,
    /// Output token
    pub token_out:            Address,
    /// true = exact input, false = exact output
    pub exact_in:             bool,
    /// Exact amount (input if exactIn, output if !exactIn)
    pub amount:               u128,
    /// Limit amount (min_out if exactIn, max_in if !exactIn)
    pub limit_amount:         u128,
    /// Price limit in Ray format (1e27)
    pub limit_price:          U256,
    /// Maximum gas + fee in asset0
    pub max_extra_fee_asset0: u128,
    /// User's ECDSA or ERC-1271 signature
    pub signature:            Bytes
}

/// Builder for constructing Balancer ProposalParams from consensus outputs
pub struct BalancerSubmissionBuilder;

impl BalancerSubmissionBuilder {
    /// Build ProposalParams from consensus outputs
    ///
    /// This converts the internal representation used by consensus
    /// (PoolSolution, TopOfBlockOrder, UserOrder) into the explicit
    /// parameter format expected by the Balancer Angstrom contract.
    ///
    /// # Arguments
    ///
    /// * `solutions` - Pool solutions from consensus matching
    /// * `tob_orders` - Winning top-of-block orders
    /// * `user_orders` - User limit orders to execute
    /// * `pool_registry` - Registry for looking up pool addresses
    /// * `pairs` - Pair metadata from bundle building
    /// * `assets` - Asset array containing token addresses
    ///
    /// # Returns
    ///
    /// ProposalParams ready to be submitted to the Balancer contract
    pub fn from_proposal(
        solutions: &[PoolSolution],
        tob_orders: &[TopOfBlockOrder],
        user_orders: &[UserOrder],
        pool_registry: &BalancerPoolRegistry,
        pairs: &[Pair],
        assets: &[Asset]
    ) -> Result<ProposalParams> {
        let mut pair_params = Vec::new();
        let mut tob_params = Vec::new();
        let mut user_params = Vec::new();

        // Build PairParams from solutions
        for solution in solutions {
            let pool_address = pool_registry.get_pool_address(solution.id)?;
            let (token0, token1) = pool_registry.get_pool_tokens(solution.id)?;

            // Determine swap direction and amount from the net AMM order
            if let Some(ref amm_order) = solution.amm_quantity {
                let (token_in, token_out, amount_in) = match amm_order {
                    crate::orders::NetAmmOrder::Buy(amount_in, _amount_out) => {
                        // Buy means we're selling token1 to get token0
                        (token1, token0, *amount_in)
                    }
                    crate::orders::NetAmmOrder::Sell(amount_in, _amount_out) => {
                        // Sell means we're selling token0 to get token1
                        (token0, token1, *amount_in)
                    }
                };

                pair_params.push(PairParams {
                    pool_address,
                    token_in,
                    exact_amount_in: amount_in,
                    token_out
                });
            }
        }

        // Convert ToB orders to params
        for (tob, pair) in tob_orders.iter().zip(pairs.iter()) {
            // Get token addresses from assets using pair indices
            let token0 = assets
                .get(pair.index0 as usize)
                .ok_or_else(|| eyre::eyre!("Invalid asset index0: {}", pair.index0))?
                .addr;
            let token1 = assets
                .get(pair.index1 as usize)
                .ok_or_else(|| eyre::eyre!("Invalid asset index1: {}", pair.index1))?
                .addr;

            let (token_in, token_out) =
                if tob.zero_for_1 { (token0, token1) } else { (token1, token0) };

            tob_params.push(ToBOrderParams {
                token_in,
                token_out,
                exact_amount_in: tob.quantity_in,
                exact_amount_out: tob.quantity_out,
                max_gas_asset0: tob.max_gas_asset_0,
                signature: Self::signature_to_bytes(&tob.signature)
            });
        }

        // Convert user orders to params
        for (order, pair) in user_orders.iter().zip(pairs.iter()) {
            // Get token addresses from assets using pair indices
            let token0 = assets
                .get(pair.index0 as usize)
                .ok_or_else(|| eyre::eyre!("Invalid asset index0: {}", pair.index0))?
                .addr;
            let token1 = assets
                .get(pair.index1 as usize)
                .ok_or_else(|| eyre::eyre!("Invalid asset index1: {}", pair.index1))?
                .addr;

            let (token_in, token_out) =
                if order.zero_for_one { (token0, token1) } else { (token1, token0) };

            let (amount, limit_amount) = match &order.order_quantities {
                crate::contract_payloads::angstrom::OrderQuantities::Exact { quantity } => {
                    (*quantity, 0u128)
                }
                crate::contract_payloads::angstrom::OrderQuantities::Partial {
                    min_quantity_in,
                    max_quantity_in: _,
                    filled_quantity
                } => (*filled_quantity, *min_quantity_in)
            };

            user_params.push(UserOrderParams {
                token_in,
                token_out,
                exact_in: order.exact_in,
                amount,
                limit_amount,
                limit_price: order.min_price,
                max_extra_fee_asset0: order.max_extra_fee_asset0,
                signature: Self::signature_to_bytes(&order.signature)
            });
        }

        Ok(ProposalParams {
            pairs:       pair_params,
            tob_orders:  tob_params,
            user_orders: user_params
        })
    }

    /// Convert internal Signature to Bytes for contract submission
    fn signature_to_bytes(sig: &Signature) -> Bytes {
        match sig {
            Signature::Contract { from: _, signature } => signature.clone(),
            Signature::Ecdsa { v, r, s } => {
                // Encode as [r || s || v] format (65 bytes)
                let mut bytes = Vec::with_capacity(65);
                bytes.extend_from_slice(r.as_slice());
                bytes.extend_from_slice(s.as_slice());
                bytes.push(*v);
                Bytes::from(bytes)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use alloy::primitives::FixedBytes;

    use super::*;
    use crate::{
        contract_payloads::angstrom::OrderQuantities, orders::NetAmmOrder,
        primitive::BalancerPoolRegistry
    };

    fn create_test_registry() -> BalancerPoolRegistry {
        let mut registry = BalancerPoolRegistry::new();
        let pool_id = FixedBytes::from([1u8; 32]);
        let pool_addr = Address::from([0x10u8; 20]);
        let token0 = Address::from([0x20u8; 20]);
        let token1 = Address::from([0x30u8; 20]);
        registry.add_pool(pool_id, pool_addr, token0, token1);
        registry
    }

    fn create_test_assets() -> Vec<Asset> {
        vec![
            Asset { addr: Address::from([0x20u8; 20]), save: 0, take: 0, settle: 0 },
            Asset { addr: Address::from([0x30u8; 20]), save: 0, take: 0, settle: 0 },
        ]
    }

    fn create_test_pair() -> Pair {
        Pair {
            index0:       0,
            index1:       1,
            store_index:  0,
            price_1over0: U256::from(1000000000000000000u128)
        }
    }

    #[test]
    fn test_signature_to_bytes_ecdsa() {
        let sig = Signature::Ecdsa { v: 27, r: [1u8; 32].into(), s: [2u8; 32].into() };

        let bytes = BalancerSubmissionBuilder::signature_to_bytes(&sig);
        assert_eq!(bytes.len(), 65);
        assert_eq!(bytes[64], 27); // v is last byte
    }

    #[test]
    fn test_signature_to_bytes_contract() {
        let sig_bytes = Bytes::from(vec![1, 2, 3, 4]);
        let sig = Signature::Contract { from: Address::ZERO, signature: sig_bytes.clone() };

        let bytes = BalancerSubmissionBuilder::signature_to_bytes(&sig);
        assert_eq!(bytes, sig_bytes);
    }

    #[test]
    fn test_from_proposal_empty() {
        let registry = create_test_registry();
        let assets = create_test_assets();

        let result =
            BalancerSubmissionBuilder::from_proposal(&[], &[], &[], &registry, &[], &assets);

        assert!(result.is_ok());
        let params = result.unwrap();
        assert_eq!(params.pairs.len(), 0);
        assert_eq!(params.tob_orders.len(), 0);
        assert_eq!(params.user_orders.len(), 0);
    }

    #[test]
    fn test_from_proposal_with_pool_solution() {
        let registry = create_test_registry();
        let assets = create_test_assets();
        let pool_id = FixedBytes::from([1u8; 32]);

        let solution = PoolSolution {
            id:           pool_id,
            ucp:          crate::matching::Ray::default(),
            searcher:     None,
            amm_quantity: Some(NetAmmOrder::Sell(1000, 900)),
            limit:        vec![],
            reward_t0:    100,
            fee:          3000
        };

        let result = BalancerSubmissionBuilder::from_proposal(
            &[solution],
            &[],
            &[],
            &registry,
            &[],
            &assets
        );

        assert!(result.is_ok());
        let params = result.unwrap();
        assert_eq!(params.pairs.len(), 1);
        assert_eq!(params.pairs[0].exact_amount_in, 1000);
        assert_eq!(params.pairs[0].pool_address, Address::from([0x10u8; 20]));
        assert_eq!(params.pairs[0].token_in, Address::from([0x20u8; 20]));
        assert_eq!(params.pairs[0].token_out, Address::from([0x30u8; 20]));
    }

    #[test]
    fn test_from_proposal_with_tob_order() {
        let registry = create_test_registry();
        let assets = create_test_assets();
        let pair = create_test_pair();

        let tob = TopOfBlockOrder {
            use_internal:     false,
            quantity_in:      1000,
            quantity_out:     900,
            max_gas_asset_0:  50,
            gas_used_asset_0: 0,
            pairs_index:      0,
            zero_for_1:       true,
            recipient:        None,
            signature:        Signature::Ecdsa { v: 27, r: [1u8; 32].into(), s: [2u8; 32].into() }
        };

        let result =
            BalancerSubmissionBuilder::from_proposal(&[], &[tob], &[], &registry, &[pair], &assets);

        assert!(result.is_ok());
        let params = result.unwrap();
        assert_eq!(params.tob_orders.len(), 1);
        assert_eq!(params.tob_orders[0].exact_amount_in, 1000);
        assert_eq!(params.tob_orders[0].exact_amount_out, 900);
        assert_eq!(params.tob_orders[0].max_gas_asset0, 50);
        assert_eq!(params.tob_orders[0].token_in, Address::from([0x20u8; 20]));
        assert_eq!(params.tob_orders[0].token_out, Address::from([0x30u8; 20]));
        assert_eq!(params.tob_orders[0].signature.len(), 65);
    }

    #[test]
    fn test_from_proposal_with_user_order_exact() {
        let registry = create_test_registry();
        let assets = create_test_assets();
        let pair = create_test_pair();

        let user_order = UserOrder {
            ref_id:               1,
            use_internal:         false,
            pair_index:           0,
            min_price:            U256::from(1000000000000000000u128),
            recipient:            None,
            hook_data:            None,
            zero_for_one:         true,
            standing_validation:  None,
            order_quantities:     OrderQuantities::Exact { quantity: 500 },
            max_extra_fee_asset0: 25,
            extra_fee_asset0:     0,
            exact_in:             true,
            signature:            Signature::Ecdsa {
                v: 27,
                r: [3u8; 32].into(),
                s: [4u8; 32].into()
            }
        };

        let result = BalancerSubmissionBuilder::from_proposal(
            &[],
            &[],
            &[user_order],
            &registry,
            &[pair],
            &assets
        );

        assert!(result.is_ok());
        let params = result.unwrap();
        assert_eq!(params.user_orders.len(), 1);
        assert_eq!(params.user_orders[0].amount, 500);
        assert_eq!(params.user_orders[0].limit_amount, 0);
        assert_eq!(params.user_orders[0].max_extra_fee_asset0, 25);
        assert_eq!(params.user_orders[0].exact_in, true);
        assert_eq!(params.user_orders[0].token_in, Address::from([0x20u8; 20]));
        assert_eq!(params.user_orders[0].token_out, Address::from([0x30u8; 20]));
    }

    #[test]
    fn test_from_proposal_with_user_order_partial() {
        let registry = create_test_registry();
        let assets = create_test_assets();
        let pair = create_test_pair();

        let user_order = UserOrder {
            ref_id:               2,
            use_internal:         false,
            pair_index:           0,
            min_price:            U256::from(1000000000000000000u128),
            recipient:            None,
            hook_data:            None,
            zero_for_one:         false,
            standing_validation:  None,
            order_quantities:     OrderQuantities::Partial {
                min_quantity_in: 100,
                max_quantity_in: 1000,
                filled_quantity: 750
            },
            max_extra_fee_asset0: 30,
            extra_fee_asset0:     0,
            exact_in:             false,
            signature:            Signature::Ecdsa {
                v: 28,
                r: [5u8; 32].into(),
                s: [6u8; 32].into()
            }
        };

        let result = BalancerSubmissionBuilder::from_proposal(
            &[],
            &[],
            &[user_order],
            &registry,
            &[pair],
            &assets
        );

        assert!(result.is_ok());
        let params = result.unwrap();
        assert_eq!(params.user_orders.len(), 1);
        assert_eq!(params.user_orders[0].amount, 750);
        assert_eq!(params.user_orders[0].limit_amount, 100);
        assert_eq!(params.user_orders[0].exact_in, false);
        assert_eq!(params.user_orders[0].token_in, Address::from([0x30u8; 20]));
        assert_eq!(params.user_orders[0].token_out, Address::from([0x20u8; 20]));
    }

    #[test]
    fn test_from_proposal_invalid_pool_id() {
        let registry = create_test_registry();
        let assets = create_test_assets();
        let invalid_pool_id = FixedBytes::from([99u8; 32]);

        let solution = PoolSolution {
            id:           invalid_pool_id,
            ucp:          crate::matching::Ray::default(),
            searcher:     None,
            amm_quantity: Some(NetAmmOrder::Sell(1000, 900)),
            limit:        vec![],
            reward_t0:    100,
            fee:          3000
        };

        let result = BalancerSubmissionBuilder::from_proposal(
            &[solution],
            &[],
            &[],
            &registry,
            &[],
            &assets
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_from_proposal_invalid_asset_index() {
        let registry = create_test_registry();
        let assets = create_test_assets();

        let pair = Pair {
            index0:       0,
            index1:       10, // Invalid index
            store_index:  0,
            price_1over0: U256::from(1000000000000000000u128)
        };

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

        let result =
            BalancerSubmissionBuilder::from_proposal(&[], &[tob], &[], &registry, &[pair], &assets);

        assert!(result.is_err());
    }
}
