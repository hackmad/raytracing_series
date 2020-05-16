//! # Texture
//!
//! A library for handling textures.

mod checker;
mod image;
mod noise;
mod perlin;
mod solid_colour;

use super::algebra::{Axis, Colour, Point3, Vec3};
use super::common::{clamp, ArcRandomizer, Float};
use std::fmt;
use std::sync::Arc;

/// Models a solid colour as a texture.
pub use self::solid_colour::SolidColour;

/// Models a 3-dimension checkerboard pattern.
pub use self::checker::Checker;

/// Models a 3-dimension checkerboard pattern.
pub use self::noise::Noise;

/// Perlin noise generator.
pub use self::perlin::Perlin;

/// Models an image texture
pub use self::image::Image;

/// Models textures.
pub trait Texture: fmt::Display + fmt::Debug {
    /// Return the texture colour at the given parametric coordinates.
    ///
    /// * `u` - Paramteric coordinate.
    /// * `v` - Paramteric coordinate.
    /// * `p` - Intersection point.
    fn value(&self, u: Float, v: Float, p: &Point3) -> Colour;
}

/// Atomic reference counted `Texture`.
pub type ArcTexture = Arc<dyn Texture>;
