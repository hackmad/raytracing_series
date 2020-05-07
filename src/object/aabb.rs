//! # AABB
//!
//! A library for creating axis aligned bounding boxes to accelerate
//! raytracing.

use super::Float;
use super::Point3;
use super::Ray;
use std::mem::swap;

/// Models an axis aligned bounding box.
#[derive(Clone)]
pub struct AABB {
    /// Minimum bounds for the x, y and z dimensions.
    pub min: Point3,

    /// Maximum bounds for the x, y and z dimensions.
    pub max: Point3,
}

impl AABB {
    /// Create a new axis aligned bounding box.
    ///
    /// * `min` - Minimum bounds for the x, y and z dimensions.
    /// * `max` - Maximum bounds for the x, y and z dimensions.
    pub fn new(min: Point3, max: Point3) -> AABB {
        AABB { min, max }
    }

    /// Create a box the surrounds the given 2 boxes.
    ///
    /// * `box0` - First box.
    /// * `box1` - Second box.
    pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
        let small = Point3::new(
            box0.min[0].min(box1.min[0]),
            box0.min[1].min(box1.min[1]),
            box0.min[2].min(box1.min[2]),
        );

        let big = Point3::new(
            box0.max[0].max(box1.max[0]),
            box0.max[1].max(box1.max[1]),
            box0.max[2].max(box1.max[2]),
        );

        AABB::new(small, big)
    }

    /// Returns `true` if a ray intersects the AABB; `false` otherwise.
    ///
    /// * `ray` - The incident ray.
    /// * `t_min` - The minium parameter for intersections.
    /// * `t_max` - The maximum parameter for intersections.
    pub fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> bool {
        for a in 0..3 {
            let inv_d = ray.direction[a].recip();

            let mut t0 = (self.min[a] - ray.origin[a]) * inv_d;
            let mut t1 = (self.max[a] - ray.origin[a]) * inv_d;

            if inv_d < 0.0 {
                swap(&mut t0, &mut t1);
            }

            let tmin = if t0 > t_min { t0 } else { t_min };

            let tmax = if t1 < t_max { t1 } else { t_max };

            if tmax <= tmin {
                return false;
            }
        }

        true
    }
}
