//! # Diffuse light
//!
//! A library for handling diffuse light emissive material.

use super::{ArcMaterial, ArcRandomizer, ArcTexture, Colour, HitRecord, Material, Ray};
use std::fmt;
use std::sync::Arc;

/// Models a diffuse light material.
#[derive(Clone)]
pub struct DiffuseLight {
    /// The emission provided by a texture.
    emit: ArcTexture,

    /// Random number generator.
    rng: ArcRandomizer,
}

impl DiffuseLight {
    /// Creates a new diffuse light material.
    ///
    /// * `emeit` - Emission provided by a texture.
    /// * `rng` - Random number generator.
    pub fn new(emit: ArcTexture, rng: ArcRandomizer) -> ArcMaterial {
        Arc::new(DiffuseLight {
            emit: Arc::clone(&emit),
            rng: Arc::clone(&rng),
        })
    }
}

impl fmt::Display for DiffuseLight {
    /// Display the diffuse light parameters configuration.
    ///
    /// * `f` - Formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "diffuse_light(emit: {})", self.emit)
    }
}

impl fmt::Debug for DiffuseLight {
    /// Display the diffuse light parameters configuration.
    ///
    /// * `f` - Formatter.
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
        if rec.front_face {
            self.emit.value(rec.u, rec.v, &rec.point)
        } else {
            Colour::zero()
        }
    }
}
