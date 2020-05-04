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
use super::objects::HitRecord;

pub use self::dielectric::Dielectric;
pub use self::lambertian::Lambertian;
pub use self::metal::Metal;

#[derive(Copy, Clone)]
pub struct ScatterResult {
    pub scattered: Ray,
    pub attenuation: Colour,
}

pub trait Material: MaterialClone {
    fn scatter(&self, ray_in: Ray, rec: HitRecord) -> Option<ScatterResult>;
}

pub trait MaterialClone {
    fn clone_box(&self) -> Box<dyn Material>;
}

impl<T> MaterialClone for T
where
    T: 'static + Material + Clone,
{
    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Box<dyn Material> {
        self.clone_box()
    }
}
