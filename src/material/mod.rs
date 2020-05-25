//! # Material
//!
//! A library for handling materials.

mod dielectric;
mod diffuse_light;
mod isotropic;
mod lambertian;
mod metal;

use super::algebra::{Colour, Ray};
use super::common::{ArcPDF, ArcRandomizer, CosinePDF, Float, PI};
use super::object::HitRecord;
use super::texture::ArcTexture;
use std::fmt;
use std::sync::Arc;

// Re-exports.
pub use self::dielectric::Dielectric;
pub use self::diffuse_light::DiffuseLight;
pub use self::isotropic::Isotropic;
pub use self::lambertian::Lambertian;
pub use self::metal::Metal;

/// Models the result of scattering a ray.
#[derive(Debug, Clone)]
pub struct ScatterRecord {
    /// Ray for specular materials.
    pub specular_ray: Option<Ray>,

    /// Scattered rays for materials like isotropic.
    pub scattered_ray: Option<Ray>,

    /// The attenuation.
    pub attenuation: Colour,

    /// The PDF value for diffuse materials.
    pub pdf: Option<ArcPDF>,
}

/// Models a material that can scatter incoming rays based on material
/// properties.
pub trait Material: fmt::Display + fmt::Debug {
    /// Scatter an incident ray and determine the attenuation. If the incident
    /// ray is absorbed, `None` is returned (default for emissive light material).
    ///
    /// * `ray_in` - Incident ray.
    /// * `rec` - The `HitRecord`.
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    /// Return the PDF value at a point on the surface. Used for impportance
    /// sampling.
    ///
    /// * `ray_in` - Incident ray.
    /// * `rec` - The `HitRecord`.
    fn scattering_pdf(&self, _ray_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> Float {
        0.0
    }

    /// Return the emiited colour value. Default emission is black.
    ///
    /// * `ray_in` - Incident ray.
    /// * `rec` - The `HitRecord`.
    fn emission(&self, _ray_in: &Ray, _rec: &HitRecord) -> Colour {
        Colour::zero()
    }
}

/// Atomic reference counted `Material`.
pub type ArcMaterial = Arc<dyn Material + Send + Sync>;
