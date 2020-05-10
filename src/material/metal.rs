//! # Metal
//!
//! A library for handling reflective material.

use super::{Colour, Float, HitRecord, Material, Ray, RcMaterial, RcRandomizer, ScatterResult};
use std::fmt;
use std::rc::Rc;

/// Models a metal
#[derive(Clone)]
pub struct Metal {
    /// The diffuse colour.
    albedo: Colour,

    /// Fuzziness factor used for blurred reflections.
    fuzz: Float,

    /// Random number generator.
    rng: RcRandomizer,
}

impl Metal {
    /// Creates a new metal material.
    ///
    /// * `albedo` - The diffuse colour.
    /// * `fuzz` - The fuzziness factor for blurred reflections.
    /// * `rng` - Random number generator.
    pub fn new(albedo: Colour, fuzz: Float, rng: RcRandomizer) -> RcMaterial {
        Rc::new(Metal {
            albedo,
            fuzz,
            rng: Rc::clone(&rng),
        })
    }
}

impl fmt::Display for Metal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "metal(albedo: {}, fuzz: {})", self.albedo, self.fuzz)
    }
}

impl fmt::Debug for Metal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Metal")
            .field("albedo", &self.albedo)
            .field("fuzz", &self.fuzz)
            .finish()
    }
}

impl Material for Metal {
    /// Scatter an incident ray and determine the attenuation.
    /// If the incident ray is absorbed, `None` is returned.
    ///
    /// Model the reflections. For grazing angles, the ray is absorbed.
    /// Use a small sphere based on `fuzz` to rngize the reflected
    /// direction for blurry reflection.
    ///
    /// * `ray_in` - Incident ray.
    /// * `rec` - The `HitRecord`.
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let reflected = ray_in.direction.unit_vector().reflect(rec.normal);
        if reflected.dot(rec.normal) > 0.0 {
            let direction = reflected + self.rng.clone().in_unit_sphere() * self.fuzz;
            Some(ScatterResult {
                scattered: Ray::new(rec.point, direction, ray_in.time),
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}
