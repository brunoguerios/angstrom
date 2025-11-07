//! Price conversions for Uniswap types
//!
//! This module provides conversions between the neutral Price type
//! and Uniswap-specific price representations (SqrtPriceX96).

use crate::{
    amm::Price,
    matching::{Ray, SqrtPriceX96}
};

/// Convert SqrtPriceX96 to neutral Price (via Ray)
impl From<SqrtPriceX96> for Price {
    fn from(sqrt_price: SqrtPriceX96) -> Self {
        // Convert SqrtPriceX96 to Ray, then to Price
        let ray: Ray = sqrt_price.into();
        Price::new(ray)
    }
}

/// Convert neutral Price to SqrtPriceX96 (via Ray)
impl TryFrom<Price> for SqrtPriceX96 {
    type Error = &'static str;

    fn try_from(price: Price) -> Result<Self, Self::Error> {
        // Convert Price's Ray to SqrtPriceX96
        Ok(price.as_ray().into())
    }
}

/// Convert Ray to neutral Price directly
impl From<Ray> for Price {
    fn from(ray: Ray) -> Self {
        Price::new(ray)
    }
}

#[cfg(test)]
mod tests {
    use alloy::primitives::{U160, U256};

    use super::*;

    #[test]
    fn test_sqrt_price_to_price_and_back() {
        // Use a realistic SqrtPriceX96 value (around 1 ETH = 3000 USDC)
        // SqrtPriceX96 for price of 3000: sqrt(3000) * 2^96
        let sqrt_price =
            SqrtPriceX96::from(U160::from_str_radix("4340327532914677254717682", 10).unwrap());
        let price: Price = sqrt_price.into();
        let back: SqrtPriceX96 = price.try_into().unwrap();

        // The roundtrip through Ray should preserve the SqrtPriceX96 value
        // within a small tolerance due to rounding
        let original_u256: U256 = sqrt_price.into();
        let back_u256: U256 = back.into();

        // Allow for small rounding differences (less than 0.01%)
        let diff = if original_u256 > back_u256 {
            original_u256 - back_u256
        } else {
            back_u256 - original_u256
        };

        // Tolerance: less than 1 in 10000
        assert!(
            diff * U256::from(10000) < original_u256,
            "Roundtrip difference too large: {} vs {}",
            original_u256,
            back_u256
        );
    }

    #[test]
    fn test_ray_to_price_to_sqrt() {
        // Start from a Ray value and ensure roundtrip works
        let ray = Ray::from(
            U256::from_str_radix(
                "3000000000000000000000000000", // 3000 * 1e27
                10
            )
            .unwrap()
        );
        let price = Price::new(ray);
        let sqrt_price: SqrtPriceX96 = price.try_into().unwrap();
        let ray_back: Ray = sqrt_price.into();

        // Roundtrip through SqrtPriceX96 should be very close
        let diff = if ray > ray_back { *ray - *ray_back } else { *ray_back - *ray };

        // Allow for rounding in the square root operations
        assert!(diff < *ray / U256::from(10000), "Ray roundtrip difference too large");
    }

    #[test]
    fn test_ray_to_price_preserves_value() {
        let ray = Ray::from(1000000000000000000000000000u128); // 1e27
        let price: Price = ray.into();
        assert_eq!(price.as_ray(), ray);
    }

    #[test]
    fn test_sqrt_price_to_ray_to_price() {
        // Test that converting SqrtPriceX96 -> Ray -> Price works
        let sqrt_price = SqrtPriceX96::from(
            U160::from_str_radix(
                "79228162514264337593543950336", // 2^96
                10
            )
            .unwrap()
        );
        let ray: Ray = sqrt_price.into();
        let price: Price = ray.into();

        // Converting back should give us the same SqrtPriceX96
        let back: SqrtPriceX96 = price.try_into().unwrap();

        // Compare as U256 values
        let original: U256 = sqrt_price.into();
        let back_u256: U256 = back.into();
        assert_eq!(original, back_u256);
    }
}
