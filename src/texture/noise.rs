//! # Noise
//!
//! A library for the 3-dimensional noise texture

#![allow(dead_code)]
use super::{Axis, Colour, Float, Perlin, Point3, RcRandomizer, RcTexture, Texture};
use std::fmt;
use std::rc::Rc;

/// Models a 3-dimension noiseboard pattern.
#[derive(Debug, Clone)]
pub struct Noise {
    /// Perlin noise generator.
    perlin: Perlin,

    /// Frequency.
    frequency: Float,

    /// Turbulence depth.
    turbulence_depth: usize,

    /// Turbulence size.
    turbulence_size: Float,

    /// Axis along which the marble grain aligns.
    axis: Axis,
}

impl Noise {
    /// Creates a new noise texture.
    ///
    /// * `frequency` - Scale.
    /// * `turbulence_depth` - Turbulence depth.
    /// * `turbulence_size` - Turbulence size.
    /// * `size` - Grid size for Perlin noise.
    /// * `axis` - Axis along which the marble grain aligns.
    /// * `rng` - Random number generator.
    pub fn new(
        frequency: Float,
        turbulence_depth: usize,
        turbulence_size: Float,
        size: usize,
        axis: Axis,
        rng: RcRandomizer,
    ) -> RcTexture {
        Rc::new(Noise {
            perlin: Perlin::new(size, Rc::clone(&rng)),
            frequency,
            turbulence_depth,
            turbulence_size,
            axis,
        })
    }
}

impl fmt::Display for Noise {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "noise(perlin: {})", self.perlin)
    }
}

impl Texture for Noise {
    /// Return the stored colour value regardless of texture coordinates
    /// and intersection point.
    ///
    /// * `u` - Paramteric coordinate.
    /// * `v` - Paramteric coordinate.
    /// * `p` - Intersection point.
    fn value(&self, _u: Float, _v: Float, p: &Point3) -> Colour {
        let turb = self.perlin.turbulence(p, self.turbulence_depth);
        Colour::one()
            * 0.5
            * (1.0 + (self.frequency * p[self.axis] + self.turbulence_size * turb).sin())
    }
}
