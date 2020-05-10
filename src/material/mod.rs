//! # Material
//!
//! A library for handling materials.

mod dielectric;
mod lambertian;
mod metal;

use super::algebra::{Colour, Ray};
use super::common::{Float, RcRandomizer};
use super::object::HitRecord;
use std::fmt;
use std::rc::Rc;

pub use self::dielectric::Dielectric;
pub use self::lambertian::Lambertian;
pub use self::metal::Metal;

/// Models the result of scattering a ray.
#[derive(Debug, Copy, Clone)]
pub struct ScatterResult {
    /// The scattered ray.
    pub scattered: Ray,

    /// The attenuation.
    pub attenuation: Colour,
}

/// Models a material that can scatter incoming rays based on material
/// properties.
pub trait Material: fmt::Display + fmt::Debug {
    /// Scatter an incident ray and determine the attenuation.
    /// If the incident ray is absorbed, `None` is returned.
    ///
    /// * `ray_in` - Incident ray.
    /// * `rec` - The `HitRecord`.
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterResult>;
}

/// Reference counted material.
pub type RcMaterial = Rc<dyn Material>;
