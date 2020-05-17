//! # XYrect
//!
//! A library for handling ray intersections with axis aligned rectangle in
//! the xy-plane.

use super::{ArcHittable, ArcMaterial, Float, HitRecord, Hittable, Point3, Ray, Vec3, AABB};
use std::fmt;
use std::sync::Arc;

/// Models an axis-aligned rectangle in the xy-plane.
#[derive(Debug, Clone)]
pub struct XYrect {
    /// X-coordinate bound x0.
    x0: Float,

    /// X-coordinate bound x1.
    x1: Float,

    /// Y-coordinate bound y0.
    y0: Float,

    /// Y-coordinate bound y1.
    y1: Float,

    /// Z-coordinate of plane.
    z: Float,

    /// Surface material.
    material: ArcMaterial,
}

impl fmt::Display for XYrect {
    /// Display the XYrect parameters.
    ///
    /// * `f` - Formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "xy_rect(x0: {}, x1: {}, y0: {}, y1:{}, z: {}, material: {})",
            self.x0, self.x1, self.y0, self.y1, self.z, self.material
        )
    }
}

impl XYrect {
    /// Create a new axis aligned rectangle in the xy-plane.
    ///
    /// * `x0` - X-coordinate bound x0.
    /// * `x1` - X-coordinate bound x1.
    /// * `y0` - Y-coordinate bound y0.
    /// * `y1` - Y-coordinate bound y1.
    /// * `z` - Z-coordinate.
    /// * `material` - Surface material.
    pub fn new(
        x0: Float,
        x1: Float,
        y0: Float,
        y1: Float,
        z: Float,
        material: ArcMaterial,
    ) -> ArcHittable {
        // Guard against mixed up min/max values.
        Arc::new(XYrect {
            x0: x0.min(x1),
            x1: x0.max(x1),
            y0: y0.min(y1),
            y1: y0.max(y1),
            z,
            material: Arc::clone(&material),
        })
    }
}

impl Hittable for XYrect {
    /// Calculate the intersection of a ray with the objects.
    ///
    /// * `ray` - The incident ray.
    /// * `t_min` - The minium parameter for intersections.
    /// * `t_max` - The maximum parameter for intersections.
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let t = (self.z - ray.origin.z()) / ray.direction.z();
        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin.x() + t * ray.direction.x();
        let y = ray.origin.y() + t * ray.direction.y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        Some(HitRecord::new(
            ray,
            t,
            ray.at(t),
            Vec3::new(0.0, 0.0, 0.1),
            Arc::clone(&self.material),
            (x - self.x0) / (self.x1 - self.x0),
            (y - self.y0) / (self.y1 - self.y0),
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
            Point3::new(self.x0, self.y0, self.z - 0.0001),
            Point3::new(self.x1, self.y1, self.z + 0.0001),
        ))
    }
}
