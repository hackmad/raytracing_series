//! # HittableList
//!
//! A library for handling ray intersections with a collection of
//! geometric objects.

use super::Float;
use super::HitRecord;
use super::Hittable;
use super::Ray;
use super::RcHittable;
use std::rc::Rc;

/// Models a collection of geometric objects that support ray intersections.
#[derive(Clone)]
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
                }) => (
                    Some(HitRecord {
                        t,
                        point,
                        normal,
                        front_face,
                        material: Rc::clone(&material),
                    }),
                    t,
                ),
                None => acc,
            }
        });

        result.0
    }
}
