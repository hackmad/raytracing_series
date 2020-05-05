//! # HitRecord
//!
//! A library for handling surface intersection details.

use super::Float;
use super::Point3;
use super::Ray;
use super::RcMaterial;
use super::Vec3;
use std::rc::Rc;

/// Models information at surface intersections.
#[derive(Clone)]
pub struct HitRecord {
    /// Parameter along the incident ray.
    pub t: Float,

    /// Intersection point.
    pub point: Point3,

    /// Surface normal.
    pub normal: Vec3,

    /// Determines if incident ray is inside `false` or outside `true`.
    pub front_face: bool,

    /// The surface material.
    pub material: RcMaterial,
}

impl HitRecord {
    /// Create a new `HitRecord`.
    ///
    /// * `ray` - The incident ray.
    /// * `t` - The parameter along the ray for intersection point.
    /// * `point` - The intersection point.
    /// * `outward_normal` - Outward surface normal at point of intersection.
    /// * `material` - The surface material.
    pub fn new(
        ray: &Ray,
        t: Float,
        point: Point3,
        outward_normal: Vec3,
        material: RcMaterial,
    ) -> HitRecord {
        let front_face = ray.direction.dot(outward_normal) < 0.0;

        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        HitRecord {
            t,
            point,
            front_face,
            normal,
            material: Rc::clone(&material),
        }
    }
}
