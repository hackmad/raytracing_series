//! # Isotropic
//!
//! A library for handling isotropic material for constant medium effects.

use super::{HitRecord, Material, Ray, RcMaterial, RcRandomizer, RcTexture, ScatterResult};
use std::fmt;
use std::rc::Rc;

/// Models an isotropic material for constant medium.
#[derive(Clone)]
pub struct Isotropic {
    /// The diffuse colour provided by a texture.
    albedo: RcTexture,

    /// Random number generator.
    rng: RcRandomizer,
}

impl Isotropic {
    /// Creates a new material for constant medium.
    ///
    /// * `albedo` - Albedo
    /// * `rng` - Random number generator.
    pub fn new(albedo: RcTexture, rng: RcRandomizer) -> RcMaterial {
        Rc::new(Isotropic {
            albedo: Rc::clone(&albedo),
            rng: Rc::clone(&rng),
        })
    }
}

impl fmt::Display for Isotropic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "isotropic(albedo: {})", self.albedo)
    }
}

impl fmt::Debug for Isotropic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Isotropic")
            .field("albedo", &self.albedo)
            .finish()
    }
}

impl Material for Isotropic {
    /// Scatter an incident ray and determine the attenuation.
    /// If the incident ray is absorbed, `None` is returned.
    ///
    /// We want the probability to be higher for ray scattering close to
    /// the normal, but the distribution has to be more uniform.
    ///
    /// * `ray_in` - Incident ray.
    /// * `rec` - The `HitRecord`.
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        // Scattering will pick a uniform random direction.
        let scatter_direction = Rc::clone(&self.rng).in_unit_sphere();
        Some(ScatterResult {
            scattered: Ray::new(rec.point, scatter_direction, ray_in.time),
            attenuation: Rc::clone(&self.albedo).value(rec.u, rec.v, &rec.point),
        })
    }
}
