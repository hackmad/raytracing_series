//! # Lambertian
//!
//! A library for handling Lambertian diffuse material.

use super::{ArcMaterial, ArcRandomizer, ArcTexture, HitRecord, Material, Ray, ScatterResult};
use std::fmt;
use std::sync::Arc;

/// Models a Lambertian diffuse material.
#[derive(Clone)]
pub struct Lambertian {
    /// The diffuse colour provided by a texture.
    albedo: ArcTexture,

    /// Random number generator.
    rng: ArcRandomizer,
}

impl Lambertian {
    /// Creates a new Lambertian diffuse material.
    ///
    /// * `albedo` - The diffuse colour provided by a texture.
    /// * `rng` - Random number generator.
    pub fn new(albedo: ArcTexture, rng: ArcRandomizer) -> ArcMaterial {
        Arc::new(Lambertian {
            albedo: Arc::clone(&albedo),
            rng: Arc::clone(&rng),
        })
    }
}

impl fmt::Display for Lambertian {
    /// Display the lambertian parameters.
    ///
    /// * `f` - Formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "lambertian(albedo: {})", self.albedo)
    }
}

impl fmt::Debug for Lambertian {
    /// Display the lambertian parameters.
    ///
    /// * `f` - Formatter.
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
        let scatter_direction = rec.normal + self.rng.unit_vec3();
        Some(ScatterResult {
            scattered: Ray::new(rec.point, scatter_direction, ray_in.time),
            attenuation: self.albedo.value(rec.u, rec.v, &rec.point),
        })
    }
}
