//! # XZrect
//!
//! A library for handling ray intersections with axis aligned rectangle in
//! the xz-plane.

use super::{Float, HitRecord, Hittable, Point3, Ray, RcHittable, RcMaterial, Vec3, AABB};
use std::fmt;
use std::rc::Rc;

/// Models an axis-aligned rectangle in the xz-plane.
#[derive(Debug, Clone)]
pub struct XZrect {
    /// X-coordinate bound x0.
    x0: Float,

    /// X-coordinate bound x1.
    x1: Float,

    /// Y-coordinate bound z0.
    z0: Float,

    /// Y-coordinate bound z1.
    z1: Float,

    /// Y-coordinate of plane.
    y: Float,

    /// Surface material.
    material: RcMaterial,
}

impl fmt::Display for XZrect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "xz_rect(x0: {}, x1: {}, z0: {}, z1:{}, y: {}, material: {})",
            self.x0, self.x1, self.z0, self.z1, self.y, self.material
        )
    }
}

impl XZrect {
    /// Create a new axis aligned rectangle in the xz-plane.
    ///
    /// * `x0` - X-coordinate bound x0.
    /// * `x1` - X-coordinate bound x1.
    /// * `z0` - Y-coordinate bound z0.
    /// * `z1` - Y-coordinate bound z1.
    /// * `y` - Y-coordinate.
    /// * `material` - Surface material.
    pub fn new(
        x0: Float,
        x1: Float,
        z0: Float,
        z1: Float,
        y: Float,
        material: RcMaterial,
    ) -> RcHittable {
        Rc::new(XZrect {
            x0,
            x1,
            z0,
            z1,
            y,
            material: Rc::clone(&material),
        })
    }
}

impl Hittable for XZrect {
    /// Calculate the intersection of a ray with the objects.
    ///
    /// * `ray` - The incident ray.
    /// * `t_min` - The minium parameter for intersections.
    /// * `t_max` - The maximum parameter for intersections.
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let t = (self.y - ray.origin.y()) / ray.direction.y();
        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin.x() + t * ray.direction.x();
        let z = ray.origin.z() + t * ray.direction.z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        Some(HitRecord::new(
            ray,
            t,
            ray.at(t),
            Vec3::new(0.0, 1.0, 0.0),
            Rc::clone(&self.material),
            (x - self.x0) / (self.x1 - self.x0),
            (z - self.z0) / (self.z1 - self.z0),
        ))
    }

    /// Create a bounding box across time interval `[t0, t1]`.
    ///
    /// * `_time0` - Start time of motion (ignored).
    /// * `_time1` - End time of motion (ignored).
    fn bounding_box(&self, _time0: Float, _time1: Float) -> Option<AABB> {
        // The bounding box must have non-zero width in each dimension, so pad
        // the Z dimension a small amount.
        Some(AABB::new(
            Point3::new(self.x0, self.y - 0.0001, self.z0),
            Point3::new(self.x1, self.y + 0.0001, self.z1),
        ))
    }
}
