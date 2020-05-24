//! # Translate
//!
//! A library for handling ray intersections with translated objects.

use super::{ArcHittable, Float, HitRecord, Hittable, Point3, Ray, Vec3, AABB};
use std::fmt;
use std::sync::Arc;

/// Models a translated object.
#[derive(Debug, Clone)]
pub struct Translate {
    /// Holds a `Hittable`.
    object: ArcHittable,

    /// Translation offset.
    offset: Vec3,
}

impl fmt::Display for Translate {
    /// Display the translation parameters.
    ///
    /// * `f` - Formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "translate(object: {}, offset: {})",
            self.object, self.offset
        )
    }
}

impl Translate {
    /// Create a new axis aligned box.
    ///
    /// * `object`: Holds a `Hittable`.
    /// * `displacement: Vec3` - Translation offset.
    pub fn new(object: ArcHittable, displacement: Vec3) -> ArcHittable {
        Arc::new(Translate {
            object: Arc::clone(&object),
            offset: displacement,
        })
    }
}

impl Hittable for Translate {
    /// Calculate the intersection of a ray with the objects.
    ///
    /// * `ray` - The incident ray.
    /// * `t_min` - The minium parameter for intersections.
    /// * `t_max` - The maximum parameter for intersections.
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let moved_r = Ray::new(ray.origin - self.offset, ray.direction, ray.time);
        if let Some(rec) = self.object.hit(&moved_r, t_min, t_max) {
            Some(
                rec.update_point(rec.point + self.offset)
                    .update_normal(&moved_r, rec.normal),
            )
        } else {
            None
        }
    }

    /// Create a bounding box across time interval `[t0, t1]`.
    ///
    /// * `time0` - Start time of motion.
    /// * `time1` - End time of motion.
    fn bounding_box(&self, time0: Float, time1: Float) -> Option<AABB> {
        if let Some(bbox) = self.object.bounding_box(time0, time1) {
            Some(AABB::new(bbox.min + self.offset, bbox.max + self.offset))
        } else {
            None
        }
    }

    /// Sample PDF value at hit point and given direction.
    ///
    /// * `origin` - Hit point.
    /// * `v` - Direction to sample.
    fn pdf_value(&self, origin: Point3, v: Vec3) -> Float {
        self.object.pdf_value(origin - self.offset, v)
    }

    /// Generate a random direction towards this object.
    ///
    /// * `origin` - Hit point.
    fn random(&self, origin: Point3) -> Vec3 {
        self.object.random(origin - self.offset)
    }
}
