use rand::{Rng, distributions::Standard, prelude::Distribution};

use crate::sol_bindings::{
    grouped_orders::{AllOrders, FlashVariants, StandingVariants},
    rpc_orders::{
        ExactFlashOrder, ExactStandingOrder, OrderMeta, PartialFlashOrder, PartialStandingOrder,
        TopOfBlockOrder
    },
    testnet::random::RandomizerSized
};

impl Distribution<AllOrders> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> AllOrders {
        let rand_variant = rng.gen_range(0..3);

        match rand_variant {
            0 => AllOrders::Flash(rng.r#gen()),
            1 => AllOrders::Standing(rng.r#gen()),
            2 => AllOrders::TOB(rng.r#gen()),
            _ => unreachable!()
        }
    }
}

impl Distribution<FlashVariants> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> FlashVariants {
        let rand_variant = rng.gen_range(0..2);

        match rand_variant {
            0 => FlashVariants::Exact(rng.r#gen()),
            1 => FlashVariants::Partial(rng.r#gen()),
            _ => unreachable!()
        }
    }
}

impl Distribution<ExactFlashOrder> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ExactFlashOrder {
        ExactFlashOrder { hook_data: rng.gen_sized::<150>(), ..rng.r#gen() }
    }
}

impl Distribution<PartialFlashOrder> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PartialFlashOrder {
        PartialFlashOrder { hook_data: rng.gen_sized::<150>(), ..rng.r#gen() }
    }
}

impl Distribution<StandingVariants> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> StandingVariants {
        let rand_variant = rng.gen_range(0..2);

        match rand_variant {
            0 => StandingVariants::Exact(rng.r#gen()),
            1 => StandingVariants::Partial(rng.r#gen()),
            _ => unreachable!()
        }
    }
}

impl Distribution<ExactStandingOrder> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ExactStandingOrder {
        ExactStandingOrder { hook_data: rng.gen_sized::<150>(), ..rng.r#gen() }
    }
}

impl Distribution<PartialStandingOrder> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PartialStandingOrder {
        PartialStandingOrder { hook_data: rng.gen_sized::<150>(), ..rng.r#gen() }
    }
}

impl Distribution<TopOfBlockOrder> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TopOfBlockOrder {
        TopOfBlockOrder { ..rng.r#gen() }
    }
}

impl Distribution<OrderMeta> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> OrderMeta {
        OrderMeta {
            isEcdsa:   rng.r#gen(),
            from:      rng.r#gen(),
            signature: rng.gen_sized::<64>()
        }
    }
}

// impl Distribution<ContractBundle> for Standard {
//     fn sample<R: Rng + ?Sized>(&self, mut rng: &mut R) -> ContractBundle {
//         let vec_sizes = rng.gen_range(0..=10);
//         ContractBundle {
//             assets:               rng.gen_many(vec_sizes),
//             initial_prices:       rng.gen_many(vec_sizes),
//             pre_transformations:  rng.gen_many_sized::<150>(vec_sizes),
//             top_of_block_orders:  rng.gen_many(vec_sizes),
//             swaps:                rng.gen_many(vec_sizes),
//             orders:               rng.gen_many(vec_sizes),
//             post_transformations: rng.gen_many_sized::<150>(vec_sizes),
//             donates:              rng.gen_many(vec_sizes)
//         }
//     }
// }

// impl Distribution<SolPrice> for Standard {
//     fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SolPrice {
// }

// impl Distribution<crate::sol_bindings::sol::TopOfBlockOrder> for Standard {
//     fn sample<R: Rng + ?Sized>(&self, rng: &mut R) ->
// crate::sol_bindings::sol::TopOfBlockOrder {
//         crate::sol_bindings::sol::TopOfBlockOrder {
//             recipient:     rng.r#gen(),
//             hook:          rng.r#gen(),
//             hookPayload:   rng.gen_sized::<150>(),
//             amountIn:      rng.r#gen(),
//             amountOut:     rng.r#gen(),
//             assetInIndex:  rng.r#gen(),
//             assetInForm:   rng.r#gen(),
//             assetOutIndex: rng.r#gen(),
//             assetOutForm:  rng.r#gen(),
//             from:          rng.r#gen(),
//             signature:     rng.gen_sized::<64>()
//         }
//     }
// }
//
// impl Distribution<crate::sol_bindings::sol::SolSwap> for Standard {
//     fn sample<R: Rng + ?Sized>(&self, rng: &mut R) ->
// crate::sol_bindings::sol::SolSwap {         crate::sol_bindings::sol::SolSwap
// {             amountIn:    rng.r#gen(),
//             asset0Index: rng.r#gen(),
//             asset1Index: rng.r#gen(),
//             zeroForOne:  rng.r#gen()
//         }
//     }
// }
//
// impl Distribution<crate::sol_bindings::sol::SolGenericOrder> for Standard {
//     fn sample<R: Rng + ?Sized>(&self, rng: &mut R) ->
// crate::sol_bindings::sol::SolGenericOrder {
//         crate::sol_bindings::sol::SolGenericOrder {
//             otype:           rng.r#gen(),
//             mode:            rng.r#gen(),
//             amountSpecified: rng.r#gen(),
//             minPrice:        rng.r#gen(),
//             assetInIndex:    rng.r#gen(),
//             assetInForm:     rng.r#gen(),
//             assetOutIndex:   rng.r#gen(),
//             assetOutForm:    rng.r#gen(),
//             nonce:           rng.r#gen(),
//             deadline:        rng.r#gen(),
//             recipient:       rng.r#gen(),
//             hook:            rng.r#gen(),
//             hookPayload:     rng.gen_sized::<150>(),
//             amountFilled:    rng.r#gen(),
//             from:            rng.r#gen(),
//             signature:       rng.gen_sized::<64>()
//         }
//     }
// }
//
// impl Distribution<crate::sol_bindings::sol::SolOrderType> for Standard {
//     fn sample<R: Rng + ?Sized>(&self, rng: &mut R) ->
// crate::sol_bindings::sol::SolOrderType {         let variants = 3;
//         let n: u8 = rng.gen_range(0..variants);
//
//         match n {
//             0 => crate::sol_bindings::sol::SolOrderType::Flash,
//             1 => crate::sol_bindings::sol::SolOrderType::Standing,
//             2 => crate::sol_bindings::sol::SolOrderType::__Invalid,
//             _ => unreachable!(
//                 "crate::sol_bindings::sol::SolOrderType on has {variants}
// variant types"             )
//         }
//     }
// }
//
// impl Distribution<crate::sol_bindings::sol::SolOrderMode> for Standard {
//     fn sample<R: Rng + ?Sized>(&self, rng: &mut R) ->
// crate::sol_bindings::sol::SolOrderMode {         let variants = 4;
//         let n: u8 = rng.gen_range(0..variants);
//
//         match n {
//             0 => crate::sol_bindings::sol::SolOrderMode::ExactIn,
//             1 => crate::sol_bindings::sol::SolOrderMode::ExactOut,
//             2 => crate::sol_bindings::sol::SolOrderMode::Partial,
//             3 => crate::sol_bindings::sol::SolOrderMode::__Invalid,
//             _ => unreachable!(
//                 "crate::sol_bindings::sol::SolOrderMode on has {variants}
// variant types"             )
//         }
//     }
// }
//
// impl Distribution<crate::sol_bindings::sol::SolAssetForm> for Standard {
//     fn sample<R: Rng + ?Sized>(&self, rng: &mut R) ->
// crate::sol_bindings::sol::SolAssetForm {         let variants = 4;
//         let n: u8 = rng.gen_range(0..variants);
//
//         match n {
//             0 => crate::sol_bindings::sol::SolAssetForm::AngstromClaim,
//             1 => crate::sol_bindings::sol::SolAssetForm::Liquid,
//             2 => crate::sol_bindings::sol::SolAssetForm::UniV4Claim,
//             3 => crate::sol_bindings::sol::SolAssetForm::__Invalid,
//             _ => unreachable!(
//                 "crate::sol_bindings::sol::SolAssetForm on has {variants}
// variant types"             )
//         }
//     }
// }
//
// impl Distribution<crate::sol_bindings::sol::SolDonate> for Standard {
//     fn sample<R: Rng + ?Sized>(&self, mut rng: &mut R) ->
// crate::sol_bindings::sol::SolDonate {
//         crate::sol_bindings::sol::SolDonate {
//             asset0Index:    rng.r#gen(),
//             asset1Index:    rng.r#gen(),
//             startTick:
// Signed::from_be_bytes(rng.gen_range(-8388608..=8388607).to_be_bytes()),
//             totalTicks:     rng.r#gen(),
//             startLiquidity: rng.r#gen(),
//             amounts0:       rng.gen_many(1)
//         }
//     }
// }

// #[cfg(test)]
// mod tests {

//     use crate::sol_bindings::grouped_orders::{random_AllOrders, AllOrders};

//     #[test]
//     fn test_all_orders() {
//         let all_orders = random_AllOrders!(Standing);
//         println!("{:?}", all_orders);
//         assert!(matches!(all_orders, AllOrders::Standing(_)));

//         let all_orders = random_AllOrders!();
//         assert!(
//             matches!(all_orders, AllOrders::TOB(_))
//                 || matches!(all_orders, AllOrders::Flash(_))
//                 || matches!(all_orders, AllOrders::Standing(_))
//         );

//         let all_orders = random_AllOrders!(TOB);
//         assert!(matches!(all_orders, AllOrders::TOB(_)));
//     }
// }
