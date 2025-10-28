//! Price conversions for Uniswap types
//!
//! This module provides conversions between the neutral Price type
//! and Uniswap-specific price representations (SqrtPriceX96).

use alloy::primitives::U160;

use crate::{
    amm::Price,
    matching::{Ray, SqrtPriceX96}
};

/// Convert SqrtPriceX96 to neutral Price
impl From<SqrtPriceX96> for Price {
    fn from(sqrt_price: SqrtPriceX96) -> Self {
        // Convert SqrtPriceX96 (U160) to u128 by taking the lower 128 bits
        let value = u128::from(sqrt_price.as_limbs()[0]);
        Price::new(value)
    }
}

/// Convert neutral Price to SqrtPriceX96
impl TryFrom<Price> for SqrtPriceX96 {
    type Error = &'static str;

    fn try_from(price: Price) -> Result<Self, Self::Error> {
        U160::from(price.value())
            .try_into()
            .map_err(|_| "Invalid SqrtPriceX96 value")
    }
}

/// Convert Ray to neutral Price (via SqrtPriceX96)
impl From<Ray> for Price {
    fn from(ray: Ray) -> Self {
        let sqrt_price: SqrtPriceX96 = ray.into();
        sqrt_price.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt_price_to_price() {
        let sqrt_price = SqrtPriceX96::from(U160::from(1000000));
        let price: Price = sqrt_price.into();
        assert_eq!(price.value(), 1000000);
    }

    #[test]
    fn test_price_to_sqrt_price() {
        let price = Price::new(1000000);
        let sqrt_price: SqrtPriceX96 = price.try_into().unwrap();
        assert_eq!(*sqrt_price, U160::from(1000000));
    }

    #[test]
    fn test_roundtrip() {
        let original = SqrtPriceX96::from(U160::from(123456789));
        let price: Price = original.into();
        let back: SqrtPriceX96 = price.try_into().unwrap();
        assert_eq!(*original, *back);
    }
}
