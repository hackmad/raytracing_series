//! # HitRecord
//!
//! A library for handling surface intersection details.

use super::{Float, Point3, Ray, RcMaterial, Vec3};
use std::fmt;
use std::rc::Rc;

/// Models information at surface intersections.
#[derive(Debug, Clone)]
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

    /// The 2-D parametric u-coordinate of the surface.
    pub u: Float,

    /// The 2-D parametric v-coordinate of the surface.
    pub v: Float,
}

impl HitRecord {
    /// Create a new `HitRecord`.
    ///
    /// * `ray` - The incident ray.
    /// * `t` - The parameter along the ray for intersection point.
    /// * `point` - The intersection point.
    /// * `outward_normal` - Outward surface normal at point of intersection.
    /// * `material` - The surface material.
    /// * `u` - The 2-D parametric u-coordinate of the surface.
    /// * `v` - The 2-D parametric v-coordinate of the surface.
    pub fn new(
        ray: &Ray,
        t: Float,
        point: Point3,
        outward_normal: Vec3,
        material: RcMaterial,
        u: Float,
        v: Float,
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
            u,
            v,
        }
    }
}

impl fmt::Display for HitRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "hit_record(t: {}, point: {}, normal: {}, \
            front_face: {}, material: {}, u: {}, v: {})",
            self.t, self.point, self.normal, self.front_face, self.material, self.u, self.v,
        )
    }
}