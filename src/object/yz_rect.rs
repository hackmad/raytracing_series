//! # YZrect
//!
//! A library for handling ray intersections with axis aligned rectangle in
//! the yz-plane.

use super::{Float, HitRecord, Hittable, Point3, Ray, RcHittable, RcMaterial, Vec3, AABB};
use std::fmt;
use std::rc::Rc;

/// Models an axis-aligned rectangle in the yz-plane.
#[derive(Debug, Clone)]
pub struct YZrect {
    /// X-coordinate bound y0.
    y0: Float,

    /// X-coordinate bound y1.
    y1: Float,

    /// Y-coordinate bound z0.
    z0: Float,

    /// Y-coordinate bound z1.
    z1: Float,

    /// X-coordinate of plane.
    x: Float,

    /// Surface material.
    material: RcMaterial,
}

impl fmt::Display for YZrect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "yz_rect(y0: {}, y1: {}, z0: {}, z1:{}, x: {}, material: {})",
            self.y0, self.y1, self.z0, self.z1, self.x, self.material
        )
    }
}

impl YZrect {
    /// Create a new axis aligned rectangle.
    ///
    /// * `y0` - X-coordinate bound y0.
    /// * `y1` - X-coordinate bound y1.
    /// * `z0` - Y-coordinate bound z0.
    /// * `z1` - Y-coordinate bound z1.
    /// * `x` - X-coordinate.
    /// * `material` - Surface material.
    pub fn new(
        y0: Float,
        y1: Float,
        z0: Float,
        z1: Float,
        x: Float,
        material: RcMaterial,
    ) -> RcHittable {
        Rc::new(YZrect {
            y0,
            y1,
            z0,
            z1,
            x,
            material: Rc::clone(&material),
        })
    }
}

impl Hittable for YZrect {
    /// Calculate the intersection of a ray with the objects.
    ///
    /// * `ray` - The incident ray.
    /// * `t_min` - The minium parameter for intersections.
    /// * `t_max` - The maximum parameter for intersections.
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let t = (self.x - ray.origin.x()) / ray.direction.x();
        if t < t_min || t > t_max {
            return None;
        }

        let y = ray.origin.y() + t * ray.direction.y();
        let z = ray.origin.z() + t * ray.direction.z();
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        Some(HitRecord::new(
            ray,
            t,
            ray.at(t),
            Vec3::new(1.0, 0.0, 0.0),
            Rc::clone(&self.material),
            (y - self.y0) / (self.y1 - self.y0),
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
            Point3::new(self.x - 0.0001, self.y0, self.z0),
            Point3::new(self.x + 0.0001, self.y1, self.z1),
        ))
    }
}