use std::ops::Deref;

use alloy::primitives::U256;

use super::Ray;

pub enum TokenQuantity {
    Token0(u128),
    Token1(u128)
}

impl Deref for TokenQuantity {
    type Target = u128;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Token0(q) | Self::Token1(q) => q
        }
    }
}

impl TokenQuantity {
    pub fn zero_from_uint(source: U256) -> Self {
        Self::Token0(source.to())
    }

    pub fn one_from_uint(source: U256) -> Self {
        Self::Token1(source.to())
    }

    pub fn swap_at_price(&self, price: Ray) -> Self {
        match self {
            Self::Token0(q) => Self::Token1(price.mul_quantity(U256::from(*q)).to()),
            Self::Token1(q) => Self::Token0(price.inverse_quantity(U256::from(*q)).to())
        }
    }

    pub fn quantity(&self) -> u128 {
        match self {
            Self::Token0(q) | Self::Token1(q) => *q
        }
    }

    pub fn as_u256(&self) -> U256 {
        match self {
            Self::Token0(q) | Self::Token1(q) => U256::from(*q)
        }
    }
}
