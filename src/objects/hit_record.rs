use super::Float;
use super::Point3;
use super::Ray;
use super::RcMaterial;
use super::Vec3;
use std::rc::Rc;

#[derive(Clone)]
pub struct HitRecord {
    pub t: Float,
    pub point: Point3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: RcMaterial,
}

impl HitRecord {
    pub fn new(
        ray: Ray,
        t: Float,
        point: Point3,
        outward_normal: Vec3,
        material: RcMaterial,
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
            material: Rc::clone(&material),
        }
    }
}
