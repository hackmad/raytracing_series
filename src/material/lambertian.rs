//! # Lambertian
//!
//! A library for handling Lambertian diffuse material.

use super::random_unit_vec3;
use super::Colour;
use super::HitRecord;
use super::Material;
use super::Ray;
use super::RcMaterial;
use super::ScatterResult;
use std::rc::Rc;

/// Models a Lambertian diffuse material.
#[derive(Clone)]
pub struct Lambertian {
    /// The diffuse colour.
    albedo: Colour,
}

impl Lambertian {
    /// Creates a new Lambertian diffuse material.
    ///
    /// * `albedo` - The diffuse colour.
    pub fn new(albedo: Colour) -> RcMaterial {
        Rc::new(Lambertian { albedo })
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
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let scatter_direction = rec.normal + random_unit_vec3();
        Some(ScatterResult {
            scattered: Ray::new(rec.point, scatter_direction),
            attenuation: self.albedo,
        })
    }
}
