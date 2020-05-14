//! # Common
//!
//! A library of common utility functinos.

mod random;

use super::algebra::Vec3;
use std::fmt;
use std::rc::Rc;

/// Use f64 since it provides the wider range of math operations.
pub type Float = f64;

/// The constant `π`.
pub const PI: Float = std::f64::consts::PI;

/// The constant `2π`.
pub const TWO_PI: Float = 2.0 * PI;

/// The constant `π/2`.
pub const PI_OVER_2: Float = 0.5 * PI;

/// The constant `+∞`.
pub const INFINITY: Float = std::f64::INFINITY;

/// Random number generator.
pub use self::random::{new_seeded_rng, new_thread_rng, Random};

/// Define some utility functions for generating random values.
pub trait Randomizer: fmt::Debug {
    /// Returns a random floating point values in [0, 1].
    fn float(&self) -> Float;

    /// Returns a random floating point value in [`min`, `max`].
    ///
    /// * `min` - Minimum bound
    /// * `max` - Maximum bound
    fn float_in_range(&self, min: Float, max: Float) -> Float;

    /// Returns a random usize value in [`min`, `max`].
    ///
    /// * `min` - Minimum bound
    /// * `max` - Maximum bound
    fn usize_in_range(&self, min: usize, max: usize) -> usize;

    /// Returns a random u8 value in [`min`, `max`].
    ///
    /// * `min` - Minimum bound
    /// * `max` - Maximum bound
    fn u8_in_range(&self, min: u8, max: u8) -> u8;

    /// Returns a random u16 value in [`min`, `max`].
    ///
    /// * `min` - Minimum bound
    /// * `max` - Maximum bound
    fn u16_in_range(&self, min: u16, max: u16) -> u16;

    /// Returns a random u32 value in [`min`, `max`].
    ///
    /// * `min` - Minimum bound
    /// * `max` - Maximum bound
    fn u32_in_range(&self, min: u32, max: u32) -> u32;

    /// Returns a random u64 value in [`min`, `max`].
    ///
    /// * `min` - Minimum bound
    /// * `max` - Maximum bound
    fn u64_in_range(&self, min: u64, max: u64) -> u64;

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
