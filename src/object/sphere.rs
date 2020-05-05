//! # Sphere
//!
//! A library for handling ray intersections with a sphere

use super::Float;
use super::HitRecord;
use super::Hittable;
use super::Point3;
use super::Ray;
use super::RcHittable;
use super::RcMaterial;
use super::Vec3;
use std::rc::Rc;

/// Models a sphere.
#[derive(Clone)]
pub struct Sphere {
    /// Center of the sphere.
    center: Point3,

    /// Radius of the sphere.
    radius: Float,

    /// Surface material.
    material: RcMaterial,
}

impl Sphere {
    /// Create a new sphere.
    ///
    /// * `center` - Center.
    /// * `radius` - Radius.
    /// * `material` - Surface material.
    pub fn new(center: Vec3, radius: Float, material: RcMaterial) -> RcHittable {
        Rc::new(Sphere {
            center,
            radius,
            material: Rc::clone(&material),
        })
    }

    /// Returns a `HitRecord` containing the intersection point, outward
    /// surface normal for the incident ray and given parameter.
    ///
    /// * `ray` - Incident ray.
    /// * `t` - Parameter along the ray where intersection occurred.
    fn get_hit_record(&self, ray: &Ray, t: Float) -> HitRecord {
        let point = ray.at(t);
        let outward_normal = (point - self.center) / self.radius;
        HitRecord::new(ray, t, point, outward_normal, Rc::clone(&self.material))
    }
}

impl Hittable for Sphere {
    /// Calculate the intersection of a ray with the objects.
    ///
    /// * `ray` - The incident ray.
    /// * `t_min` - The minium parameter for intersections.
    /// * `t_max` - The maximum parameter for intersections.
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let t = (-half_b - root) / a;
            if t < t_max && t > t_min {
                return Some(self.get_hit_record(ray, t));
            }

            let t = (-half_b + root) / a;
            if t < t_max && t > t_min {
                return Some(self.get_hit_record(ray, t));
            }
        }

        None
    }
}
