//! # Metal
//!
//! A library for handling reflective material.

use super::random_in_unit_sphere;
use super::Colour;
use super::Float;
use super::HitRecord;
use super::Material;
use super::Ray;
use super::RcMaterial;
use super::ScatterResult;
use std::rc::Rc;

/// Models a metal
#[derive(Clone)]
pub struct Metal {
    /// The diffuse colour.
    albedo: Colour,

    /// Fuzziness factor used for blurred reflections.
    fuzz: Float,
}

impl Metal {
    /// Creates a new metal material.
    ///
    /// * `albedo` - The diffuse colour.
    /// * `fuzz` - The fuzziness factor for blurred reflections.
    pub fn new(albedo: Colour, fuzz: Float) -> RcMaterial {
        Rc::new(Metal { albedo, fuzz })
    }
}

impl Material for Metal {
    /// Scatter an incident ray and determine the attenuation.
    /// If the incident ray is absorbed, `None` is returned.
    ///
    /// Model the reflections. For grazing angles, the ray is absorbed.
    /// Use a small sphere based on `fuzz` to randomize the reflected
    /// direction for blurry reflection.
    ///
    /// * `ray_in` - Incident ray.
    /// * `rec` - The `HitRecord`.
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let reflected = ray_in.direction.unit_vector().reflect(rec.normal);
        if reflected.dot(rec.normal) > 0.0 {
            let direction = reflected + random_in_unit_sphere() * self.fuzz;
            Some(ScatterResult {
                scattered: Ray::new(rec.point, direction, ray_in.time),
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}
