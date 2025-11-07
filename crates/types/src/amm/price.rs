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

use crate::sol_bindings::Ray;

/// A venue-neutral price representation
///
/// This type stores prices as Ray (1e27 fixed-point) values internally,
/// providing a standard representation across all AMM types.
///
/// **Important**: This type has NO knowledge of any specific AMM.
/// Conversions to/from venue-specific price types must be done in the
/// venue-specific modules (uni_structure, balancer_structure, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Price {
    /// The internal Ray representation (1e27 fixed-point)
    ray: Ray
}

impl Price {
    /// Create a new Price from a Ray value
    ///
    /// This should typically only be called from venue-specific conversion
    /// code.
    pub fn new(ray: Ray) -> Self {
        Self { ray }
    }

    /// Get the Ray value
    ///
    /// Returns the internal Ray representation which is a 1e27 fixed-point
    /// value suitable for high-precision calculations.
    pub fn as_ray(&self) -> Ray {
        self.ray
    }

    /// Create a Price from a raw u128 scaled by 1e27
    ///
    /// This is a convenience constructor for testing and cases where
    /// you have a pre-scaled Ray value as a u128.
    pub fn from_u128_scaled(value: u128) -> Self {
        Self { ray: Ray::from(value) }
    }

    /// Get the raw u128 value (for backwards compatibility)
    ///
    /// This returns the underlying U256 as u128. Note that this can
    /// truncate if the Ray value is too large.
    #[deprecated(note = "Use as_ray() instead for proper Ray handling")]
    pub fn value(&self) -> u128 {
        self.ray.0.to::<u128>()
    }
}

impl Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Price({})", self.ray.0)
    }
}
