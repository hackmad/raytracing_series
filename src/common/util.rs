//! # Util
//!
//! A library of common utility functinos.

use super::Float;

/// Clamp the given value.clamp
///
/// * `x` - The value to clamp.
/// * `min` - Minimum bound.
/// * `max` - Maximum bound.
pub fn clamp(x: Float, min: Float, max: Float) -> Float {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
