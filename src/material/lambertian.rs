//! # Lambertian
//!
//! A library for handling Lambertian diffuse material.

use super::{Colour, HitRecord, Material, Ray, RcMaterial, RcRandomizer, ScatterResult};
use std::fmt;
use std::rc::Rc;

/// Models a Lambertian diffuse material.
#[derive(Clone)]
pub struct Lambertian {
    /// The diffuse colour.
    albedo: Colour,

    /// Random number generator.
    rng: RcRandomizer,
}

impl Lambertian {
    /// Creates a new Lambertian diffuse material.
    ///
    /// * `albedo` - The diffuse colour.
    /// * `rng` - Random number generator.
    pub fn new(albedo: Colour, rng: RcRandomizer) -> RcMaterial {
        Rc::new(Lambertian {
            albedo,
            rng: Rc::clone(&rng),
        })
    }
}

impl fmt::Display for Lambertian {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "lambertian(albedo: {})", self.albedo)
    }
}

impl fmt::Debug for Lambertian {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Lambertian")
            .field("albedo", &self.albedo)
            .finish()
    }
}

impl Material for Lambertian {
    /// Scatter an incident ray and determine the attenuation.
    /// If the incident ray is absorbed, `None` is returned.
    ///
    /// We want the probability to be higher for ray scattering close to
    /// the normal, but the distribution has to be more uniform.
    ///
    /// * `ray_in` - Incident ray.
    /// * `rec` - The `HitRecord`.
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let scatter_direction = rec.normal + self.rng.clone().unit_vec3();
        Some(ScatterResult {
            scattered: Ray::new(rec.point, scatter_direction, ray_in.time),
            attenuation: self.albedo,
        })
    }
}
