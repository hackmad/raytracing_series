//! # Object
//!
//! A library for handling geometric objects that support ray intersections.

#![allow(dead_code)]
mod aabb;
mod bvh;
mod constant_medium;
mod flip_face;
mod hit_record;
mod hittable_list;
mod moving_sphere;
mod rotate;
mod sphere;
mod translate;
mod xy_rect;
mod xyz_box;
mod xz_rect;
mod yz_rect;

use super::algebra::{Axis, Point3, Ray, Vec3, AXES, ONB, X_AXIS, Y_AXIS, Z_AXIS};
use super::common::{
    ArcRandomizer, Float, INFINITY, MIN_THICKNESS, PI, PI_OVER_2, RAY_EPSILON, TWO_PI,
};
use super::material::{ArcMaterial, Isotropic};
use super::texture::ArcTexture;
use std::fmt;
use std::sync::Arc;

/// Re-exports.
pub use self::aabb::AABB;
pub use self::bvh::BVH;
pub use self::constant_medium::ConstantMedium;
pub use self::flip_face::FlipFace;
pub use self::hit_record::HitRecord;
pub use self::hittable_list::HittableList;
pub use self::moving_sphere::MovingSphere;
pub use self::rotate::Rotate;
pub use self::sphere::Sphere;
pub use self::translate::Translate;
pub use self::xy_rect::XYrect;
pub use self::xyz_box::XYZbox;
pub use self::xz_rect::XZrect;
pub use self::yz_rect::YZrect;

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

    /// Sample PDF value at hit point and given direction.
    ///
    /// * `origin` - Hit point.
    /// * `v` - Direction to sample.
    fn pdf_value(&self, _origin: Point3, _v: Vec3) -> Float {
        0.0
    }

    /// Generate a random direction towards this object.
    ///
    /// * `origin` - Hit point.
    fn random(&self, _origin: Point3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0) // Arbitrary direction.
    }
}

/// Atomic reference counted `Hittable` object.
pub type ArcHittable = Arc<dyn Hittable + Send + Sync>;

/// Calculate 2-D `(u, v)` coordinates of a point on a unit sphere with center
/// `(0, 0, 0)`.
pub fn get_sphere_uv(p: &Point3) -> (Float, Float) {
    let phi = p.z().atan2(p.x());
    let theta = p.y().asin();
    (1.0 - (phi + PI) / TWO_PI, (theta + PI_OVER_2) / PI)
}
