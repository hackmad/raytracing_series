//! # Object
//!
//! A library for handling geometric objects that support ray intersections.

#![allow(dead_code)]
mod aabb;
mod bvh;
mod hit_record;
mod hittable_list;
mod moving_sphere;
mod sphere;

use super::algebra::{Axis, Point3, Ray, Vec3, AXES};
use super::common::{Float, RcRandomizer};
use super::material::RcMaterial;
use std::fmt;
use std::rc::Rc;

/// Models a collection of geometric objects that support ray intersections.
pub use self::hit_record::HitRecord;

/// Models a list of objects that can handle intersections.
pub use self::hittable_list::HittableList;

/// Models a sphere.
pub use self::sphere::Sphere;

/// Models a sphere that moves along a linear path.
pub use self::moving_sphere::MovingSphere;

/// Models an axis aligned bounding box.
pub use self::aabb::AABB;

/// Models a bounding volume hierarchy.
pub use self::bvh::BVH;

/// Models a geometric object that can handle intersections.
pub trait Hittable: fmt::Display + fmt::Debug {
    /// Calculate the intersection of a ray with the object.
    ///
    /// * `ray` - The incident ray.
    /// * `t_min` - The minium parameter for intersections.
    /// * `t_max` - The maximum parameter for intersections.
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord>;

    /// Create a bounding box across time interval `[t0, t1]`.
    /// If no bounding box exists return None. This is meant for objects
    /// like an infinite plane.
    ///
    /// * `time0` - Start time of motion.
    /// * `time1` - End time of motion.
    fn bounding_box(&self, time0: Float, time1: Float) -> Option<AABB>;
}

/// Reference counted geometric object.
pub type RcHittable = Rc<dyn Hittable>;
