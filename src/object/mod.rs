//! # Object
//!
//! A library for handling geometric objects that support ray intersections.

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

/// Models a collection of geometric objects that support ray intersections.
pub use self::hit_record::HitRecord;

/// Models a list of objects that can handle intersections.
pub use self::hittable_list::HittableList;

/// Models a sphere.
pub use self::sphere::Sphere;

/// Models a geometric object that can handle intersections.
pub trait Hittable {
    /// Calculate the intersection of a ray with the object.
    ///
    /// * `ray` - The incident ray.
    /// * `t_min` - The minium parameter for intersections.
    /// * `t_max` - The maximum parameter for intersections.
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord>;
}

/// Reference counted geometric object.
pub type RcHittable = Rc<dyn Hittable>;
