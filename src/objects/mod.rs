#![allow(dead_code)]

mod sphere;

use super::algebra::Ray;
use super::algebra::Vec3;
use super::common::Float;

pub use self::sphere::Sphere;

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: Float, t_max: Float) -> Option<HitRecord>;
}

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub t: Float,
    pub point: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(ray: Ray, t: Float, point: Vec3, outward_normal: Vec3) -> HitRecord {
        let front_face = ray.direction.dot(outward_normal) < 0.0;

        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        HitRecord {
            t,
            point,
            front_face,
            normal,
        }
    }
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn new_from_object(object: Box<dyn Hittable>) -> HittableList {
        HittableList {
            objects: vec![object],
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let init: (Option<HitRecord>, Float) = (None, t_max);

        let result = self.objects.iter().fold(init, |acc, hittable| {
            match hittable.hit(ray, t_min, acc.1) {
                Some(hit) => (Some(hit), hit.t),
                None => acc,
            }
        });

        result.0
    }
}
