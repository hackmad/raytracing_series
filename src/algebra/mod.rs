//! # Algebra
//!
//! A library for linear algebra routines

mod ray;
mod vector;

// Import stuff for nested module usage.
use super::common::*;

// Re-exports
pub use self::ray::Ray;
pub use self::vector::Colour;
pub use self::vector::Point3;
pub use self::vector::Vec3;
