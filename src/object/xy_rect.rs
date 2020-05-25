//! # XYrect
//!
//! A library for handling ray intersections with axis aligned rectangle in
//! the xy-plane.

use super::{
    ArcHittable, ArcMaterial, ArcRandomizer, Float, HitRecord, Hittable, Point3, Ray, Vec3, AABB,
    INFINITY, MIN_THICKNESS, RAY_EPSILON,
};
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

    /// Area
    area: Float,

    /// Surface material.
    material: ArcMaterial,

    /// Random number generator.
    rng: ArcRandomizer,
}

impl fmt::Display for XYrect {
    /// Display the XYrect parameters.
    ///
    /// * `f` - Formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "xy_rect(x0: {}, x1: {}, y0: {}, y1:{}, z: {}, area: {}, material: {})",
            self.x0, self.x1, self.y0, self.y1, self.z, self.area, self.material
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
    /// * `rng` - Random number generator.
    pub fn new(
        x0: Float,
        x1: Float,
        y0: Float,
        y1: Float,
        z: Float,
        material: ArcMaterial,
        rng: ArcRandomizer,
    ) -> ArcHittable {
        Arc::new(XYrect {
            x0,
            x1,
            y0,
            y1,
            z,
            area: (x1 - x0) * (y1 - y0),
            material: Arc::clone(&material),
            rng: Arc::clone(&rng),
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
            Point3::new(self.x0, self.y0, self.z - MIN_THICKNESS),
            Point3::new(self.x1, self.y1, self.z + MIN_THICKNESS),
        ))
    }

    /// Sample PDF value at hit point and given direction.
    ///
    /// * `origin` - Hit point.
    /// * `v` - Direction to sample.
    fn pdf_value(&self, origin: Point3, v: Vec3) -> Float {
        let ray = Ray::new(origin, v, 0.0);
        if let Some(rec) = self.hit(&ray, RAY_EPSILON, INFINITY) {
            // Distance to hit is rec.t * v.length().
            // So distance^2 = rec.t * rec.t * v.length_squared().
            let v_len_sq = v.length_squared();
            let v_len = v_len_sq.sqrt();
            let v_unit = v / v_len;

            let distance_squared = rec.t * rec.t * v_len_sq;
            let cosine = v_unit.dot(rec.normal.unit_vector()).abs();

            distance_squared / (cosine * self.area)
        } else {
            0.0
        }
    }

    /// Generate a random direction towards this object.
    ///
    /// * `origin` - Hit point.
    fn random(&self, origin: Point3) -> Vec3 {
        let x = self.rng.float_in_range(self.x0, self.x1);
        let y = self.rng.float_in_range(self.y0, self.y1);
        let random_point = Point3::new(x, y, self.z);
        random_point - origin
    }
}
