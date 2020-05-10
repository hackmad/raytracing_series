//! # Checker
//!
//! A library for the 3-dimensional checkerboard pattern provided by
//! 2 textures.

#![allow(dead_code)]
use super::{Colour, Float, Point3, RcTexture, Texture};
use std::fmt;
use std::rc::Rc;

/// Models a 3-dimension checkerboard pattern.
#[derive(Debug, Clone)]
pub struct Checker {
    /// Provides first colour for the checkerboard pattern.
    odd: RcTexture,

    /// Provides second colour for the checkerboard pattern.
    even: RcTexture,

    /// Scale for the checker pattern.
    scale: Float,
}

impl Checker {
    /// Creates a new checker texture.
    ///
    /// * `t0` - Provides first colour for the checkerboard pattern.
    /// * `t1` - Provides second colour for the checkerboard pattern.
    pub fn new(t0: RcTexture, t1: RcTexture) -> RcTexture {
        Rc::new(Checker {
            odd: Rc::clone(&t0),
            even: Rc::clone(&t1),
            scale: 10.0,
        })
    }
    /// Creates a new checker texture with scale.
    ///
    /// * `t0` - Provides first colour for the checkerboard pattern.
    /// * `t1` - Provides second colour for the checkerboard pattern.
    /// * `s` - Scale factor.
    pub fn scaled(t0: RcTexture, t1: RcTexture, s: Float) -> RcTexture {
        Rc::new(Checker {
            odd: t0,
            even: t1,
            scale: s,
        })
    }
}

impl fmt::Display for Checker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "checker(odd: {}, even: {}", self.odd, self.even)
    }
}

impl Texture for Checker {
    /// Return the stored colour value regardless of texture coordinates
    /// and intersection point.
    ///
    /// * `u` - Paramteric coordinate.
    /// * `v` - Paramteric coordinate.
    /// * `p` - Intersection point.
    fn value(&self, u: Float, v: Float, p: &Point3) -> Colour {
        let scaled = *p * self.scale;
        let sines = scaled[0].sin() * scaled[1].sin() * scaled[2].sin();

        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
