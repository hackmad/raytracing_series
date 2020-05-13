//! # Translate
//!
//! A library for handling ray intersections with translated objects.

use super::{Float, HitRecord, Hittable, Ray, RcHittable, Vec3, AABB};
use std::fmt;
use std::rc::Rc;

/// Models a translated object.
#[derive(Debug, Clone)]
pub struct Translate {
    /// Holds a `Hittable`.
    object: RcHittable,

    /// Translation offset.
    offset: Vec3,
}

impl fmt::Display for Translate {
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
    pub fn new(object: RcHittable, displacement: Vec3) -> RcHittable {
        Rc::new(Translate {
            object: Rc::clone(&object),
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
        if let Some(rec) = Rc::clone(&self.object).hit(&moved_r, t_min, t_max) {
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
}