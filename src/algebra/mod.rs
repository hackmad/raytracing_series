//! # Algebra
//!
//! A library for linear algebra routines

mod onb;
mod ray;
mod vector;

// Import stuff for nested module usage.
use super::common::*;

// Re-exports.
pub use self::onb::ONB;
pub use self::ray::Ray;
pub use self::vector::{Axis, Colour, Point3, Vec3, AXES, X_AXIS, Y_AXIS, Z_AXIS};
