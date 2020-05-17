//! # Checker
//!
//! A library for the 3-dimensional checkerboard pattern provided by
//! 2 textures.

#![allow(dead_code)]
use super::{ArcTexture, Colour, Float, Point3, Texture};
use std::fmt;
use std::sync::Arc;

/// Models a 3-dimension checkerboard pattern.
#[derive(Debug, Clone)]
pub struct Checker {
    /// Provides first colour for the checkerboard pattern.
    odd: ArcTexture,

    /// Provides second colour for the checkerboard pattern.
    even: ArcTexture,

    /// Scale for the checker pattern.
    scale: Float,
}

impl Checker {
    /// Creates a new checker texture.
    ///
    /// * `t0` - Provides first colour for the checkerboard pattern.
    /// * `t1` - Provides second colour for the checkerboard pattern.
    pub fn new(t0: ArcTexture, t1: ArcTexture) -> ArcTexture {
        Arc::new(Checker {
            odd: Arc::clone(&t0),
            even: Arc::clone(&t1),
            scale: 10.0,
        })
    }
    /// Creates a new checker texture with scale.
    ///
    /// * `t0` - Provides first colour for the checkerboard pattern.
    /// * `t1` - Provides second colour for the checkerboard pattern.
    /// * `s` - Scale factor.
    pub fn scaled(t0: ArcTexture, t1: ArcTexture, s: Float) -> ArcTexture {
        Arc::new(Checker {
            odd: t0,
            even: t1,
            scale: s,
        })
    }
}

impl fmt::Display for Checker {
    /// Display the checker parameters.
    ///
    /// * `f` - Formatter.
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
