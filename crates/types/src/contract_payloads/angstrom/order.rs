use alloy::primitives::{aliases::U40, Address, Bytes, B256, U256};
use pade::PadeDecode;
use pade_macro::{PadeDecode, PadeEncode};

use crate::{
    contract_payloads::{Asset, Pair, Signature},
    orders::{OrderFillState, OrderOutcome},
    sol_bindings::{
        grouped_orders::{
            FlashVariants, GroupedVanillaOrder, OrderWithStorageData, StandingVariants
        },
        rpc_orders::{
            ExactFlashOrder, ExactStandingOrder, PartialFlashOrder, PartialStandingOrder
        },
        RawPoolOrder
    }
};

#[derive(Debug, PadeEncode, PadeDecode)]
pub enum OrderQuantities {
    Exact { quantity: u128 },
    Partial { min_quantity_in: u128, max_quantity_in: u128, filled_quantity: u128 }
}

impl OrderQuantities {
    pub fn fetch_max_amount(&self) -> u128 {
        match self {
            Self::Exact { quantity } => *quantity,
            Self::Partial { max_quantity_in, .. } => *max_quantity_in
        }
    }
}

#[derive(Debug, PadeEncode, PadeDecode)]
pub struct StandingValidation {
    nonce:    u64,
    // 40 bits wide in reality
    #[pade_width(5)]
    deadline: u64
}

impl StandingValidation {
    pub fn nonce(&self) -> u64 {
        self.nonce
    }

    pub fn deadline(&self) -> u64 {
        self.deadline
    }
}

#[derive(Debug, PadeEncode, PadeDecode)]
pub struct UserOrder {
    pub ref_id:               u32,
    pub use_internal:         bool,
    pub pair_index:           u16,
    pub min_price:            alloy::primitives::U256,
    pub recipient:            Option<Address>,
    pub hook_data:            Option<Bytes>,
    pub zero_for_one:         bool,
    pub standing_validation:  Option<StandingValidation>,
    pub order_quantities:     OrderQuantities,
    pub max_extra_fee_asset0: u128,
    pub extra_fee_asset0:     u128,
    pub exact_in:             bool,
    pub signature:            Signature
}

impl UserOrder {
    pub fn order_hash(&self, pair: &[Pair], asset: &[Asset], block: u64) -> B256 {
        let pair = &pair[self.pair_index as usize];
        match self.order_quantities {
            OrderQuantities::Exact { quantity } => {
                if let Some(validation) = &self.standing_validation {
                    // exact standing
                    ExactStandingOrder {
                        ref_id: self.ref_id,
                        exact_in: true,
                        use_internal: self.use_internal,
                        asset_in: if self.zero_for_one {
                            asset[pair.index0 as usize].addr
                        } else {
                            asset[pair.index1 as usize].addr
                        },
                        asset_out: if !self.zero_for_one {
                            asset[pair.index0 as usize].addr
                        } else {
                            asset[pair.index1 as usize].addr
                        },
                        recipient: self.recipient.unwrap_or_default(),
                        nonce: validation.nonce,
                        deadline: U40::from_limbs([validation.deadline]),
                        amount: quantity,
                        min_price: self.min_price,
                        hook_data: self.hook_data.clone().unwrap_or_default(),
                        max_extra_fee_asset0: self.max_extra_fee_asset0,
                        ..Default::default()
                    }
                    .order_hash()
                } else {
                    // exact flash
                    ExactFlashOrder {
                        ref_id: self.ref_id,
                        exact_in: true,
                        use_internal: self.use_internal,
                        asset_in: if self.zero_for_one {
                            asset[pair.index0 as usize].addr
                        } else {
                            asset[pair.index1 as usize].addr
                        },
                        asset_out: if !self.zero_for_one {
                            asset[pair.index0 as usize].addr
                        } else {
                            asset[pair.index1 as usize].addr
                        },
                        recipient: self.recipient.unwrap_or_default(),
                        valid_for_block: block,
                        amount: quantity,
                        min_price: self.min_price,
                        hook_data: self.hook_data.clone().unwrap_or_default(),
                        max_extra_fee_asset0: self.max_extra_fee_asset0,
                        ..Default::default()
                    }
                    .order_hash()
                }
            }
            OrderQuantities::Partial { min_quantity_in, max_quantity_in, .. } => {
                if let Some(validation) = &self.standing_validation {
                    PartialStandingOrder {
                        ref_id: self.ref_id,
                        use_internal: self.use_internal,
                        asset_in: if self.zero_for_one {
                            asset[pair.index0 as usize].addr
                        } else {
                            asset[pair.index1 as usize].addr
                        },
                        asset_out: if !self.zero_for_one {
                            asset[pair.index0 as usize].addr
                        } else {
                            asset[pair.index1 as usize].addr
                        },
                        recipient: self.recipient.unwrap_or_default(),
                        deadline: U40::from_limbs([validation.deadline]),
                        nonce: validation.nonce,
                        min_amount_in: min_quantity_in,
                        max_amount_in: max_quantity_in,
                        min_price: self.min_price,
                        hook_data: self.hook_data.clone().unwrap_or_default(),
                        max_extra_fee_asset0: self.max_extra_fee_asset0,
                        ..Default::default()
                    }
                    .order_hash()
                } else {
                    PartialFlashOrder {
                        ref_id: self.ref_id,
                        use_internal: self.use_internal,
                        asset_in: if self.zero_for_one {
                            asset[pair.index0 as usize].addr
                        } else {
                            asset[pair.index1 as usize].addr
                        },
                        asset_out: if !self.zero_for_one {
                            asset[pair.index0 as usize].addr
                        } else {
                            asset[pair.index1 as usize].addr
                        },
                        recipient: self.recipient.unwrap_or_default(),
                        valid_for_block: block,
                        max_amount_in: max_quantity_in,
                        min_amount_in: min_quantity_in,
                        min_price: self.min_price,
                        hook_data: self.hook_data.clone().unwrap_or_default(),
                        max_extra_fee_asset0: self.max_extra_fee_asset0,
                        ..Default::default()
                    }
                    .order_hash()
                }
            }
        }
    }

    pub fn from_internal_order(
        order: &OrderWithStorageData<GroupedVanillaOrder>,
        outcome: &OrderOutcome,
        shared_gas: U256,
        pair_index: u16
    ) -> eyre::Result<Self> {
        let order_quantities = match order.order {
            GroupedVanillaOrder::KillOrFill(_) => {
                OrderQuantities::Exact { quantity: order.max_q() }
            }
            GroupedVanillaOrder::Standing(_) => {
                let max_quantity_in: u128 = order.max_q();
                let filled_quantity = match outcome.outcome {
                    OrderFillState::CompleteFill => max_quantity_in,
                    OrderFillState::PartialFill(fill) => fill,
                    _ => 0
                };
                OrderQuantities::Partial { min_quantity_in: 0, max_quantity_in, filled_quantity }
            }
        };
        let hook_data = match order.order {
            GroupedVanillaOrder::KillOrFill(ref o) => o.hook_data().clone(),
            GroupedVanillaOrder::Standing(ref o) => o.hook_data().clone()
        };
        let gas_used: u128 = (order.priority_data.gas + shared_gas).to();
        if gas_used > order.max_gas_token_0() {
            return Err(eyre::eyre!("order used more gas than allocated"))
        }

        let sig_bytes = order.signature().clone().0.to_vec();
        let decoded_signature =
            alloy::primitives::PrimitiveSignature::pade_decode(&mut sig_bytes.as_slice(), None)
                .unwrap();
        let signature = Signature::from(decoded_signature);

        Ok(Self {
            ref_id: 0,
            use_internal: false,
            pair_index,
            min_price: *order.price(),
            recipient: None,
            hook_data: Some(hook_data),
            zero_for_one: !order.is_bid,
            standing_validation: None,
            order_quantities,
            max_extra_fee_asset0: order.max_gas_token_0(),
            extra_fee_asset0: gas_used,
            exact_in: false,
            signature
        })
    }

    pub fn from_internal_order_max_gas(
        order: &OrderWithStorageData<GroupedVanillaOrder>,
        outcome: &OrderOutcome,
        pair_index: u16
    ) -> Self {
        let order_quantities = match &order.order {
            GroupedVanillaOrder::KillOrFill(o) => match o {
                FlashVariants::Exact(_) => OrderQuantities::Exact { quantity: order.max_q() },
                FlashVariants::Partial(p_o) => OrderQuantities::Partial {
                    min_quantity_in: p_o.min_amount_in,
                    max_quantity_in: p_o.max_amount_in,
                    filled_quantity: outcome.fill_amount(p_o.max_amount_in)
                }
            },
            GroupedVanillaOrder::Standing(o) => match o {
                StandingVariants::Exact(_) => OrderQuantities::Exact { quantity: order.max_q() },
                StandingVariants::Partial(p_o) => {
                    let max_quantity_in = p_o.max_amount_in;
                    let filled_quantity = outcome.fill_amount(p_o.max_amount_in);
                    OrderQuantities::Partial {
                        min_quantity_in: p_o.min_amount_in,
                        max_quantity_in,
                        filled_quantity
                    }
                }
            }
        };
        let hook_bytes = match order.order {
            GroupedVanillaOrder::KillOrFill(ref o) => o.hook_data().clone(),
            GroupedVanillaOrder::Standing(ref o) => o.hook_data().clone()
        };
        let hook_data = if hook_bytes.is_empty() { None } else { Some(hook_bytes) };
        let sig_bytes = order.signature().to_vec();
        let decoded_signature =
            alloy::primitives::PrimitiveSignature::pade_decode(&mut sig_bytes.as_slice(), None)
                .unwrap();
        Self {
            ref_id: 0,
            use_internal: false,
            pair_index,
            min_price: *order.price(),
            recipient: None,
            hook_data,
            zero_for_one: !order.is_bid,
            standing_validation: None,
            order_quantities,
            max_extra_fee_asset0: order.max_gas_token_0(),
            extra_fee_asset0: order.max_gas_token_0(),
            exact_in: false,
            signature: Signature::from(decoded_signature)
        }
    }
}
