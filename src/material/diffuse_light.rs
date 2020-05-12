//! # Diffuse light
//!
//! A library for handling diffuse light emissive material.

use super::{Colour, Float, Material, Point3, RcMaterial, RcRandomizer, RcTexture};
use std::fmt;
use std::rc::Rc;

/// Models a diffuse light material.
#[derive(Clone)]
pub struct DiffuseLight {
    /// The emission provided by a texture.
    emit: RcTexture,

    /// Random number generator.
    rng: RcRandomizer,
}

impl DiffuseLight {
    /// Creates a new diffuse light material.
    ///
    /// * `emeit` - Emission provided by a texture.
    /// * `rng` - Random number generator.
    pub fn new(emit: RcTexture, rng: RcRandomizer) -> RcMaterial {
        Rc::new(DiffuseLight {
            emit: Rc::clone(&emit),
            rng: Rc::clone(&rng),
        })
    }
}

impl fmt::Display for DiffuseLight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "diffuse_light(emit: {})", self.emit)
    }
}

impl fmt::Debug for DiffuseLight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DiffuseLight")
            .field("emit", &self.emit)
            .finish()
    }
}

impl Material for DiffuseLight {
    /// Return the emiited colour value. Default emission is black.
    ///
    /// * `u` - Parametric surface u-coordinate.
    /// * `v` - Parametric surface v-coordinate.
    /// * `u` - Point on the surface.
    fn emission(&self, u: Float, v: Float, p: &Point3) -> Colour {
        Rc::clone(&self.emit).value(u, v, p)
    }
}
