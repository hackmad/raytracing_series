//! # CosinePDF
//!
//! A library to handle cosine probability density function.

#![allow(dead_code)]
use super::{Float, Random, Vec3, ONB, PDF, PI};

/// Models the cosine probability density function.
#[derive(Debug, Clone)]
pub struct CosinePDF {
    /// The orthonormal basis vectors for a point on a surface based on
    /// surface normal.
    uvw: ONB,
}

impl CosinePDF {
    /// Create a new cosine density functino given a surface normal.
    ///
    /// * `n` - Surface normal.
    pub fn new(n: Vec3) -> CosinePDF {
        CosinePDF { uvw: ONB::new(n) }
    }
}

impl PDF for CosinePDF {
    /// Returns the value of a PDF at a location.
    ///
    /// * `direction` - Direction of surface normal.
    fn value(&self, direction: Vec3) -> Float {
        let cosine = direction.unit_vector().dot(self.uvw.w());
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }

    /// Returns a random direction based on PDF.
    fn generate(&self) -> Vec3 {
        self.uvw.local_from_vec3(&Random::cosine_direction())
    }
}
