//! # Common
//!
//! A library of common utility functinos.

mod random;

use super::algebra::Vec3;
use std::rc::Rc;

/// Use f64 since it provides the wider range of math operations.
pub type Float = f64;

/// The constant `π`.
pub const PI: Float = std::f64::consts::PI;

/// The constant `+∞`.
pub const INFINITY: Float = std::f64::INFINITY;

/// Random number generator.
pub use self::random::{new_seeded_rng, new_thread_rng, Random};

/// Define some utility functions for generating random values.
pub trait Randomizer {
    /// Returns a random floating point values in [0, 1].
    fn float(&self) -> Float;

    /// Returns a random floating point values in [`min`, `max`].
    ///
    /// * `min` - Minimum bound
    /// * `max` - Maximum bound
    fn float_in_range(&self, min: Float, max: Float) -> Float;

    /// Returns a random vector with random components in [0, 1].
    fn vec3(&self) -> Vec3;

    /// Returns a random vector with random components in [`min`, `max`].
    ///
    /// * `min` - Minimum bound
    /// * `max` - Maximum bound
    fn vec3_in_range(&self, min: Float, max: Float) -> Vec3;

    /// Returns a random vector within the unit sphere. This vector is not
    /// normalized.
    fn in_unit_sphere(&self) -> Vec3;

    /// Returns a random unit vector by picking points on the unit sphere
    /// and then normalizing it.
    fn unit_vec3(&self) -> Vec3;

    /// Returns a random vector with uniform scatter direction for all angles
    /// away from a hit point, with no dependence on the angle from the normal.
    ///
    /// * `normal` - THe surface normal.
    fn in_hemisphere(&self, normal: Vec3) -> Vec3;

    /// Returns a random point inside unit disk
    fn in_unit_disk(&self) -> Vec3;
}

// Reference counted `Randomizer`.
pub type RcRandomizer = Rc<dyn Randomizer>;

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
