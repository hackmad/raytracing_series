//! # Isotropic
//!
//! A library for handling isotropic material for constant medium effects.

use super::{ArcMaterial, ArcTexture, HitRecord, Material, Random, Ray, ScatterRecord};
use std::fmt;
use std::sync::Arc;

/// Models an isotropic material for constant medium.
#[derive(Clone)]
pub struct Isotropic {
    /// The diffuse colour provided by a texture.
    albedo: ArcTexture,
}

impl Isotropic {
    /// Creates a new material for constant medium.
    ///
    /// * `albedo` - Albedo
    pub fn new(albedo: ArcTexture) -> ArcMaterial {
        Arc::new(Isotropic {
            albedo: Arc::clone(&albedo),
        })
    }
}

impl fmt::Display for Isotropic {
    /// Display the isotropic parameters.
    ///
    /// * `f` - Formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "isotropic(albedo: {})", self.albedo)
    }
}

impl fmt::Debug for Isotropic {
    /// Display the isotropic parameters.
    ///
    /// * `f` - Formatter.
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
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        // Scattering will pick a uniform random direction.
        let scatter_direction = Random::vec3_in_unit_sphere();
        let scattered_ray = Some(Ray::new(rec.point, scatter_direction, ray_in.time));
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.point);

        Some(ScatterRecord {
            scattered_ray,
            attenuation,
            specular_ray: None,
            pdf: None,
        })
    }
}
