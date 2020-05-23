//! # HittablePDF
//!
//! A library to handle sampling directions towards a `Hittable` (e.g. lights)

#![allow(dead_code)]
use super::{ArcHittable, Float, Point3, Vec3, PDF};
use std::sync::Arc;

/// Models the probability density function for sampling directions towards
/// a `Hittable` object.
#[derive(Debug, Clone)]
pub struct HittablePDF {
    /// Object towards which to sample.
    object: ArcHittable,

    /// A point on the surface where PDF is evaluated.
    origin: Point3,
}

impl HittablePDF {
    /// Create a new cosine density functino given a surface normal.
    ///
    /// * `object` - Surface normal.
    /// * `rng` - Random number generator.
    pub fn new(object: ArcHittable, origin: &Point3) -> HittablePDF {
        HittablePDF {
            object: Arc::clone(&object),
            origin: *origin,
        }
    }
}

impl PDF for HittablePDF {
    /// Returns the value of a PDF at a location.
    ///
    /// * `direction` - Direction of surface normal.
    fn value(&self, direction: &Vec3) -> Float {
        self.object.pdf_value(&self.origin, direction)
    }

    /// Returns a random direction based on PDF.
    fn generate(&self) -> Vec3 {
        self.object.random(&self.origin)
    }
}
