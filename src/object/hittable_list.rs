//! # HittableList
//!
//! A library for handling ray intersections with a collection of
//! geometric objects.

use super::{ArcHittable, Float, HitRecord, Hittable, Point3, Random, Ray, Vec3, AABB};
use std::fmt;
use std::sync::Arc;

/// Models a collection of geometric objects that support ray intersections.
#[derive(Debug, Clone)]
pub struct HittableList {
    /// List of objects.
    objects: Vec<ArcHittable>,
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
    pub fn add(&mut self, object: ArcHittable) {
        self.objects.push(Arc::clone(&object));
    }
}

impl fmt::Display for HittableList {
    /// Display the objects in the list.
    ///
    /// * `f` - Formatter.
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
                        material: Arc::clone(&material),
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

    /// Sample PDF value at hit point and given direction.
    ///
    /// * `origin` - Hit point.
    /// * `v` - Direction to sample.
    fn pdf_value(&self, origin: Point3, v: Vec3) -> Float {
        let weight = 1.0 / (self.objects.len() as Float);

        self.objects.iter().fold(0.0, |sum, object| {
            sum + weight * object.pdf_value(origin, v)
        })
    }

    /// Generate a random direction towards this object.
    ///
    /// * `origin` - Hit point.
    fn random(&self, origin: Point3) -> Vec3 {
        let size = self.objects.len();
        if size == 0 {
            Vec3::zero()
        } else if size == 1 {
            self.objects[0].random(origin)
        } else {
            self.objects[Random::sample_in_range(0, size - 1)].random(origin)
        }
    }
}
