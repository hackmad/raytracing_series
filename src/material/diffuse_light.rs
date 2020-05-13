//! # Diffuse light
//!
//! A library for handling diffuse light emissive material.

use super::{Colour, HitRecord, Material, Ray, RcMaterial, RcRandomizer, RcTexture};
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
    /// * `ray_in` - Incident ray.
    /// * `rec` - The `HitRecord`.
    fn emission(&self, _ray_in: &Ray, rec: &HitRecord) -> Colour {
        Rc::clone(&self.emit).value(rec.u, rec.v, &rec.point)
    }
}
