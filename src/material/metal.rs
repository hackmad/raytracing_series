//! # Metal
//!
//! A library for handling reflective material.

use super::{ArcMaterial, ArcTexture, Float, HitRecord, Material, Random, Ray, ScatterRecord};
use std::fmt;
use std::sync::Arc;

/// Models a metal
#[derive(Clone)]
pub struct Metal {
    /// The diffuse colour provided by a texture.
    albedo: ArcTexture,

    /// Fuzziness factor used for blurred reflections.
    fuzz: Float,
}

impl Metal {
    /// Creates a new metal material.
    ///
    /// * `albedo` - The diffuse colour provided by a texture.
    /// * `fuzz` - The fuzziness factor for blurred reflections.
    pub fn new(albedo: ArcTexture, fuzz: Float) -> ArcMaterial {
        Arc::new(Metal {
            albedo: Arc::clone(&albedo),
            fuzz,
        })
    }
}

impl fmt::Display for Metal {
    /// Display the metal parameters.
    ///
    /// * `f` - Formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "metal(albedo: {}, fuzz: {})", self.albedo, self.fuzz)
    }
}

impl fmt::Debug for Metal {
    /// Display the metal parameters.
    ///
    /// * `f` - Formatter.
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
    /// Use a small sphere based on `fuzz` to randomize the reflected
    /// direction for blurry reflection.
    ///
    /// * `ray_in` - Incident ray.
    /// * `rec` - The `HitRecord`.
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let unit_normal = rec.normal.unit_vector();
        let reflected = ray_in.direction.unit_vector().reflect(unit_normal);

        let scatter_direction = reflected + self.fuzz * Random::vec3_in_unit_sphere();

        if scatter_direction.dot(unit_normal) > 0.0 {
            Some(ScatterRecord {
                specular_ray: Some(Ray::new(rec.point, scatter_direction, ray_in.time)),
                attenuation: self.albedo.value(rec.u, rec.v, &rec.point),
                scattered_ray: None,
                pdf: None,
            })
        } else {
            None
        }
    }
}
