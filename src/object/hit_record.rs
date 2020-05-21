//! # HitRecord
//!
//! A library for handling surface intersection details.

use super::{ArcMaterial, Float, Point3, Ray, Vec3};
use std::fmt;
use std::sync::Arc;

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
    pub material: ArcMaterial,

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
        material: ArcMaterial,
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
            material: Arc::clone(&material),
            u,
            v,
        }
    }

    /// Returns a copy with the point changed
    pub fn update_point(&self, p: Point3) -> HitRecord {
        HitRecord {
            t: self.t,
            point: p,
            front_face: self.front_face,
            normal: self.normal,
            material: Arc::clone(&self.material),
            u: self.u,
            v: self.v,
        }
    }

    /// Returns a copy with the normal changed
    pub fn update_normal(&self, ray: &Ray, outward_normal: Vec3) -> HitRecord {
        let front_face = ray.direction.dot(outward_normal) < 0.0;

        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        HitRecord {
            t: self.t,
            point: self.point,
            front_face,
            normal,
            material: Arc::clone(&self.material),
            u: self.u,
            v: self.v,
        }
    }

    /// Returns a copy with the front_Face field flipped.
    pub fn flip_front_face(&self) -> HitRecord {
        HitRecord {
            t: self.t,
            point: self.point,
            front_face: !self.front_face,
            normal: self.normal,
            material: Arc::clone(&self.material),
            u: self.u,
            v: self.v,
        }
    }
}

impl fmt::Display for HitRecord {
    /// Display the hit record.
    ///
    /// * `f` - Formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "hit_record(t: {}, point: {}, normal: {}, \
            front_face: {}, material: {}, u: {}, v: {})",
            self.t, self.point, self.normal, self.front_face, self.material, self.u, self.v,
        )
    }
}
