//! # Material
//!
//! A library for handling materials.

mod dielectric;
mod diffuse_light;
mod lambertian;
mod metal;

use super::algebra::{Colour, Point3, Ray};
use super::common::{Float, RcRandomizer};
use super::object::HitRecord;
use super::texture::RcTexture;
use std::fmt;
use std::rc::Rc;

pub use self::dielectric::Dielectric;
pub use self::diffuse_light::DiffuseLight;
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
    /// Scatter an incident ray and determine the attenuation. If the incident
    /// ray is absorbed, `None` is returned (default for emissive light material).
    ///
    /// * `ray_in` - Incident ray.
    /// * `rec` - The `HitRecord`.
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Option<ScatterResult> {
        None
    }

    /// Return the emiited colour value. Default emission is black.
    ///
    /// * `u` - Parametric surface u-coordinate.
    /// * `v` - Parametric surface v-coordinate.
    /// * `p` - Point on the surface.
    fn emission(&self, _u: Float, _v: Float, _p: &Point3) -> Colour {
        Colour::zero()
    }
}

/// Reference counted material.
pub type RcMaterial = Rc<dyn Material>;
