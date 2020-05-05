//! # Material
//!
//! A library for handling materials.

mod dielectric;
mod lambertian;
mod metal;

use super::algebra::Colour;
use super::algebra::Ray;
use super::algebra::Vec3;
use super::common::random;
use super::common::random_in_unit_sphere;
use super::common::random_unit_vec3;
use super::common::Float;
use super::object::HitRecord;
use std::rc::Rc;

pub use self::dielectric::Dielectric;
pub use self::lambertian::Lambertian;
pub use self::metal::Metal;

/// Models the result of scattering a ray.
#[derive(Copy, Clone)]
pub struct ScatterResult {
    /// The scattered ray.
    pub scattered: Ray,

    /// The attenuation.
    pub attenuation: Colour,
}

/// Models a material that can scatter incoming rays based on material
/// properties.
pub trait Material {
    /// Scatter an incident ray and determine the attenuation.
    /// If the incident ray is absorbed, `None` is returned.
    ///
    /// * `ray_in` - Incident ray.
    /// * `rec` - The `HitRecord`.
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterResult>;
}

/// Reference counted material.
pub type RcMaterial = Rc<dyn Material>;
