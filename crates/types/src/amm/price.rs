//! Venue-neutral price representation
//!
//! This module provides a truly AMM-agnostic Price type that does NOT contain
//! any venue-specific conversion logic.
//!
//! Venue-specific conversions live in their respective modules:
//! - Uniswap conversions in `uni_structure/`
//! - Balancer conversions in `balancer_structure/`
//!
//! This ensures the neutral abstraction layer remains decoupled from
//! specific AMM implementations.

use std::fmt::{Debug, Display};

use serde::{Deserialize, Serialize};

/// A venue-neutral price representation
///
/// This is an opaque price value that can represent prices from any AMM.
/// The internal representation is a u128, but the interpretation of this
/// value is venue-specific.
///
/// **Important**: This type has NO knowledge of any specific AMM.
/// Conversions to/from venue-specific price types must be done in the
/// venue-specific modules (uni_structure, balancer_structure, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Price {
    /// The raw price value (opaque to consumers)
    ///
    /// For different AMMs, this might represent:
    /// - Uniswap: Lower 128 bits of SqrtPriceX96
    /// - Balancer: Spot price in fixed-point representation
    /// - Others: Venue-specific price encoding
    value: u128
}

impl Price {
    /// Create a new Price from a raw value
    ///
    /// This should typically only be called from venue-specific conversion
    /// code.
    pub fn new(value: u128) -> Self {
        Self { value }
    }

    /// Get the raw price value
    ///
    /// This should typically only be called from venue-specific conversion code
    /// that knows how to interpret the value.
    pub fn value(&self) -> u128 {
        self.value
    }
}

impl Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Price({})", self.value)
    }
}
