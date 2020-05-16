//! # Object
//!
//! A library for handling geometric objects that support ray intersections.

#![allow(dead_code)]
mod aabb;
mod bvh;
mod constant_medium;
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

use super::algebra::{Axis, Point3, Ray, Vec3, AXES};
use super::common::{ArcRandomizer, Float, INFINITY, PI, PI_OVER_2, TWO_PI};
use super::material::{ArcMaterial, Isotropic};
use super::texture::ArcTexture;
use std::fmt;
use std::sync::Arc;

/// Models a collection of geometric objects that support ray intersections.
pub use self::hit_record::HitRecord;

/// Models a list of objects that can handle intersections.
pub use self::hittable_list::HittableList;

/// Models a sphere.
pub use self::sphere::Sphere;

/// Models a sphere that moves along a linear path.
pub use self::moving_sphere::MovingSphere;

/// Models an axis-aligned rectangle in the xy-plane.
pub use self::xy_rect::XYrect;

/// Models an axis-aligned rectangle in the xz-plane.
pub use self::xz_rect::XZrect;

/// Models an axis-aligned rectangle in the yz-plane.
pub use self::yz_rect::YZrect;

/// Models an axis-aligned box.
pub use self::xyz_box::XYZbox;

/// Models a translated object.
pub use self::translate::Translate;

/// Models a rotated object.
pub use self::rotate::Rotate;

/// Models an axis aligned bounding box.
pub use self::aabb::AABB;

/// Models a bounding volume hierarchy.
pub use self::bvh::BVH;

/// Models a constant medium for effects like smoke and fog.
pub use self::constant_medium::ConstantMedium;

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

/// Atomic reference counted `Hittable` object.
pub type ArcHittable = Arc<dyn Hittable>;

/// Calculate 2-D `(u, v)` coordinates of a point on a unit sphere with center
/// `(0, 0, 0)`.
pub fn get_sphere_uv(p: &Point3) -> (Float, Float) {
    let phi = p.z().atan2(p.x());
    let theta = p.y().asin();
    (1.0 - (phi + PI) / TWO_PI, (theta + PI_OVER_2) / PI)
}
