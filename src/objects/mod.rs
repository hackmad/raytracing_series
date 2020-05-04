#![allow(dead_code)]

mod hit_record;
mod hittable_list;
mod sphere;

use super::algebra::Point3;
use super::algebra::Ray;
use super::algebra::Vec3;
use super::common::Float;
use super::material::RcMaterial;
use std::rc::Rc;

pub use self::hit_record::HitRecord;
pub use self::hittable_list::HittableList;
pub use self::sphere::Sphere;

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: Float, t_max: Float) -> Option<HitRecord>;
}

pub type RcHittable = Rc<dyn Hittable>;
