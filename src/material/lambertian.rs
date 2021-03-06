//! # Lambertian
//!
//! A library for handling Lambertian diffuse material.

use super::{
    ArcMaterial, ArcTexture, CosinePDF, Float, HitRecord, Material, Ray, ScatterRecord, PI,
};
use std::fmt;
use std::sync::Arc;

/// Models a Lambertian diffuse material.
#[derive(Clone)]
pub struct Lambertian {
    /// The diffuse colour provided by a texture.
    albedo: ArcTexture,
}

impl Lambertian {
    /// Creates a new Lambertian diffuse material.
    ///
    /// * `albedo` - The diffuse colour provided by a texture.
    pub fn new(albedo: ArcTexture) -> ArcMaterial {
        Arc::new(Lambertian {
            albedo: Arc::clone(&albedo),
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
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let pdf = CosinePDF::new(rec.normal);

        Some(ScatterRecord {
            attenuation: self.albedo.value(rec.u, rec.v, &rec.point),
            pdf: Some(Arc::new(pdf)),
            scattered_ray: None, // pdf handles it
            specular_ray: None,
        })
    }

    /// Return the PDF value at a point on the surface. Used for impportance
    /// sampling.
    ///
    /// * `ray_in` - Incident ray.
    /// * `rec` - The `HitRecord`.
    /// * `scattered` - The scattered ray.
    fn scattering_pdf(&self, _ray_in: &Ray, rec: &HitRecord, scattered: &Ray) -> Float {
        let n = rec.normal.unit_vector();
        let v = scattered.direction.unit_vector();
        let cosine = n.dot(v);
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }
}
