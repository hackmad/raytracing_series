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

#[derive(Copy, Clone)]
pub struct ScatterResult {
    pub scattered: Ray,
    pub attenuation: Colour,
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterResult>;
}

pub type RcMaterial = Rc<dyn Material>;
