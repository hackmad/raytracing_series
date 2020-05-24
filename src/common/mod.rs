//! # Common
//!
//! A library of common utility functinos.

mod cosine_pdf;
mod hittable_pdf;
mod mixture_pdf;
mod random;
mod util;

use super::algebra::{Point3, Vec3, ONB};
use super::object::ArcHittable;
use std::fmt;
use std::sync::Arc;

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

/// Used to offset ray positions and AAB boundaries to avoid intersection issues.
pub const RAY_EPSILON: Float = 0.001;

/// Minimum thickness of AABB used for 2-d shapes like planes.
pub const MIN_THICKNESS: Float = 0.0001;

/// Re-exports.
pub use self::cosine_pdf::CosinePDF;
pub use self::hittable_pdf::HittablePDF;
pub use self::mixture_pdf::MixturePDF;
pub use self::random::{new_seeded_rng, new_thread_rng, Random};
pub use self::util::*;

/// Define some utility functions for generating random values.
///
/// **NOTE**: The `*_in_range` functions could be done using generics but
/// then they have to be moved into another trait. If we do it in here,
/// we won't be able to use `Randomizer` as trait objects in `ArcRandomizer`.
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

    /// Shuffle a `Vec<usize>` in place.
    ///
    /// * `v` - Vector to shuffle.
    fn permute(&self, v: &mut Vec<usize>);

    /// Returns a random vector based on p(direction) = cos(θ) / π.
    fn cosine_direction(&self) -> Vec3;

    // Return a random vector uniformly sampled from a sphere’s solid angle
    // from a point outside the sphere
    //
    // * `distance_squared` - Square of distance to a point from center.
    fn to_sphere(&self, radius: Float, distance_squared: Float) -> Vec3;
}

// Atomic reference counted `Randomizer`.
pub type ArcRandomizer = Arc<dyn Randomizer + Send + Sync>;

/// Probability density functions.
pub trait PDF: fmt::Debug {
    /// Returns the value of a PDF at a location.
    ///
    /// * `direction` - Direction of surface normal.
    fn value(&self, direction: Vec3) -> Float;

    /// Returns a random direction based on PDF.
    fn generate(&self) -> Vec3;
}

// Atomic reference counted `PDF`.
pub type ArcPDF = Arc<dyn PDF + Send + Sync>;
