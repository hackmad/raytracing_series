//! # Solid
//!
//! A library for handling solid colour as a texture.

use super::{Colour, Float, Point3, RcTexture, Texture};
use std::fmt;
use std::rc::Rc;

/// Models a solid colour as a texture.
#[derive(Debug, Clone)]
pub struct Solid {
    /// The colour value.
    colour_value: Colour,
}

impl Solid {
    /// Creates a new solid texture from a `Colour` value.
    ///
    /// * `colour` - The colour.
    pub fn new(colour: Colour) -> RcTexture {
        Rc::new(Solid {
            colour_value: colour,
        })
    }

    /// Creates a new solid texture from RGB colour values.
    ///
    /// * `colour` - The colour.
    pub fn from_rgb(r: Float, g: Float, b: Float) -> RcTexture {
        Rc::new(Solid {
            colour_value: Colour::new(r, g, b),
        })
    }
}

impl fmt::Display for Solid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "solid(colour_value: {}", self.colour_value)
    }
}

impl Texture for Solid {
    /// Return the stored colour value regardless of texture coordinates
    /// and intersection point.
    ///
    /// * `_u` - Paramteric coordinate (ignored).
    /// * `_v` - Paramteric coordinate (ignored).
    /// * `_p` - Intersection point (ignored).
    fn value(&self, _u: Float, _v: Float, _p: &Point3) -> Colour {
        self.colour_value
    }
}
