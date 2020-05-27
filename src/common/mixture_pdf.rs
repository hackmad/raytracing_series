//! # MixturePDF
//!
//! A library to handle mixing probability density functions.

#![allow(dead_code)]
use super::{ArcPDF, Float, Random, Vec3, PDF};
use std::fmt;
use std::sync::Arc;

/// Models the mixture density.
#[derive(Clone)]
pub struct MixturePDF {
    /// The PDFs to mix.
    p: [ArcPDF; 2],
}

impl MixturePDF {
    /// Create a new cosine density functino given a surface normal.
    ///
    /// * `p0` - PDF related to the shape of light source.
    /// * `p1` - PDF related to the normal vector and type of surface.
    pub fn new(p0: ArcPDF, p1: ArcPDF) -> MixturePDF {
        MixturePDF {
            p: [Arc::clone(&p0), Arc::clone(&p1)],
        }
    }
}

impl fmt::Debug for MixturePDF {
    /// Display the mixture PDF parameters.
    ///
    /// * `f` - Formatter.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MixturePDF").field("p", &self.p).finish()
    }
}

impl PDF for MixturePDF {
    /// Returns the value of a PDF at a location.
    ///
    /// * `direction` - Direction of surface normal.
    fn value(&self, direction: Vec3) -> Float {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }

    /// Returns a random direction based on PDF.
    fn generate(&self) -> Vec3 {
        if Random::sample::<Float>() < 0.5 {
            self.p[0].generate()
        } else {
            self.p[1].generate()
        }
    }
}
