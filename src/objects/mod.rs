#![allow(dead_code)]

mod hittable_list;
mod sphere;

use super::algebra::Ray;
use super::algebra::Vec3;
use super::common::Float;
use super::material::Material;

pub use self::hittable_list::HittableList;
pub use self::sphere::Sphere;

pub trait Hittable: ObjectClone {
    fn hit(&self, ray: Ray, t_min: Float, t_max: Float) -> Option<HitRecord>;
}

pub trait ObjectClone {
    fn clone_box(&self) -> Box<dyn Hittable>;
}

impl<T> ObjectClone for T
where
    T: 'static + Hittable + Clone,
{
    fn clone_box(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Hittable> {
    fn clone(&self) -> Box<dyn Hittable> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct HitRecord {
    pub t: Float,
    pub point: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Box<dyn Material>,
}

impl HitRecord {
    pub fn new(
        ray: Ray,
        t: Float,
        point: Vec3,
        outward_normal: Vec3,
        material: Box<dyn Material>,
    ) -> HitRecord {
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
            material,
        }
    }
}
