//! # HittableList
//!
//! A library for handling ray intersections with a collection of
//! geometric objects.

use super::{Float, HitRecord, Hittable, Ray, RcHittable, AABB};
use std::fmt;
use std::rc::Rc;

/// Models a collection of geometric objects that support ray intersections.
#[derive(Debug, Clone)]
pub struct HittableList {
    objects: Vec<RcHittable>,
}

impl HittableList {
    /// Create a new collection of geometric objects.
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    /// Clear the list of objects.
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    /// Add a new object to the list.
    pub fn add(&mut self, object: RcHittable) {
        self.objects.push(Rc::clone(&object));
    }
}

impl fmt::Display for HittableList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?}]", self.objects)
    }
}

impl Hittable for HittableList {
    /// Calculate the intersection of a ray with the objects.
    ///
    /// * `ray` - The incident ray.
    /// * `t_min` - The minium parameter for intersections.
    /// * `t_max` - The maximum parameter for intersections.
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let init: (Option<HitRecord>, Float) = (None, t_max);

        let result = self.objects.iter().fold(init, |acc, hittable| {
            match hittable.hit(ray, t_min, acc.1) {
                Some(HitRecord {
                    t,
                    point,
                    normal,
                    front_face,
                    material,
                    u,
                    v,
                }) => (
                    Some(HitRecord {
                        t,
                        point,
                        normal,
                        front_face,
                        material: Rc::clone(&material),
                        u,
                        v,
                    }),
                    t,
                ),
                None => acc,
            }
        });

        result.0
    }

    /// Create a bounding box across time interval `[t0, t1]`.
    ///
    /// If we encounter an object whose bounding box cannot be calculated,
    /// None is returned.
    ///
    /// * `time0` - Start time of motion.
    /// * `time1` - End time of motion.
    fn bounding_box(&self, time0: Float, time1: Float) -> Option<AABB> {
        match self
            .objects
            .iter()
            .try_fold(None, |acc: Option<AABB>, object| {
                match object.bounding_box(time0, time1) {
                    Some(bbox0) => Ok(Some(match acc {
                        Some(bbox1) => AABB::surrounding_box(bbox0, bbox1),
                        None => bbox0,
                    })),
                    None => Err(()),
                }
            }) {
            Ok(bbox) => bbox,
            _ => None,
        }
    }
}
