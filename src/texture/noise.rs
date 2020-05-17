//! # Noise
//!
//! A library for the 3-dimensional noise texture

#![allow(dead_code)]
use super::{ArcRandomizer, ArcTexture, Axis, Colour, Float, Perlin, Point3, Texture};
use std::fmt;
use std::sync::{Arc, RwLock};

/// Models a 3-dimension noiseboard pattern.
#[derive(Debug, Clone)]
pub struct Noise {
    /// Perlin noise generator.
    perlin: Arc<RwLock<Perlin>>,

    /// Scale.
    scale: Float,

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
    /// * `scale` - Scale.
    /// * `turbulence_depth` - Turbulence depth.
    /// * `turbulence_size` - Turbulence size.
    /// * `grid_size` - Grid size for Perlin noise.
    /// * `axis` - Axis along which the marble grain aligns.
    /// * `rng` - Random number generator.
    pub fn new(
        scale: Float,
        turbulence_depth: usize,
        turbulence_size: Float,
        grid_size: usize,
        axis: Axis,
        rng: ArcRandomizer,
    ) -> ArcTexture {
        let perlin = Arc::new(RwLock::new(Perlin::new(grid_size, rng)));

        Arc::new(Noise {
            perlin,
            scale,
            turbulence_depth,
            turbulence_size,
            axis,
        })
    }
}

impl fmt::Display for Noise {
    /// Display the texture configuration.
    ///
    /// * `f` - Formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "noise(perlin: {})", self.perlin.read().unwrap())
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
        let perlin = self.perlin.read().unwrap();

        let turb = self.turbulence_size * perlin.turbulence(p, self.turbulence_depth);
        let scale = self.scale * p[self.axis];
        Colour::one() * (0.5 * (1.0 + (scale + turb).sin()))
    }
}
