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
pub use self::random::Random;
pub use self::util::*;

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
