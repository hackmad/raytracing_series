//! # FlipFace
//!
//! A library for flipping surface normals on an object.

use super::{ArcHittable, Float, HitRecord, Hittable, Ray, AABB};
use std::fmt;
use std::sync::Arc;

/// Flips the normal of a `Hittable` object.
#[derive(Debug, Clone)]
pub struct FlipFace {
    /// Object whose normals need to be flipped.
    object: ArcHittable,
}

impl fmt::Display for FlipFace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "flip_face(object: {})", self.object)
    }
}

impl FlipFace {
    /// Create a new object with flipped normals.
    ///
    /// * `object` - Object whose normals will be flipped.
    pub fn new(object: ArcHittable) -> ArcHittable {
        Arc::new(FlipFace {
            object: Arc::clone(&object),
        })
    }
}

impl Hittable for FlipFace {
    /// Calculate the intersection of a ray with the objects.
    ///
    /// * `ray` - The incident ray.
    /// * `t_min` - The minium parameter for intersections.
    /// * `t_max` - The maximum parameter for intersections.
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        if let Some(rec) = self.object.hit(ray, t_min, t_max) {
            Some(rec.flip_front_face())
        } else {
            None
        }
    }

    /// Create a bounding box across time interval `[t0, t1]`.
    ///
    /// * `time0` - Start time of motion.
    /// * `time1` - End time of motion.
    fn bounding_box(&self, time0: Float, time1: Float) -> Option<AABB> {
        self.object.bounding_box(time0, time1)
    }
}
