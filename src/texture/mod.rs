//! # Texture
//!
//! A library for handling textures.

mod solid;

use super::algebra::{Colour, Point3};
use super::common::Float;
use std::fmt;
use std::rc::Rc;

/// Models a solid colour as a texture.
pub use self::solid::Solid;

/// Models textures.
pub trait Texture: fmt::Display + fmt::Debug {
    /// Return the texture colour at the given parametric coordinates.
    ///
    /// * `u` - Paramteric coordinate.
    /// * `v` - Paramteric coordinate.
    /// * `p` - Intersection point.
    fn value(&self, u: Float, v: Float, p: &Point3) -> Colour;
}

/// Reference counted material.
pub type RcTexture = Rc<dyn Texture>;
