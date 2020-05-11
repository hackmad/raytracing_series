//! # Noise
//!
//! A library for the 3-dimensional noise texture

#![allow(dead_code)]
use super::{Colour, Float, Perlin, Point3, RcRandomizer, RcTexture, Texture};
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
}

impl Noise {
    /// Creates a new noise texture.
    ///
    /// * `frequency` - Scale.
    /// * `turbulence_depth` - Turbulence depth.
    /// * `turbulence_size` - Turbulence size.
    /// * `size` - Grid size for Perlin noise.
    /// * `rng` - Random number generator.
    pub fn new(
        frequency: Float,
        turbulence_depth: usize,
        turbulence_size: Float,
        size: usize,
        rng: RcRandomizer,
    ) -> RcTexture {
        Rc::new(Noise {
            perlin: Perlin::new(size, Rc::clone(&rng)),
            frequency,
            turbulence_depth,
            turbulence_size,
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
        Colour::one() * 0.5 * (1.0 + (self.frequency * p.z() + self.turbulence_size * turb).sin())
    }
}
