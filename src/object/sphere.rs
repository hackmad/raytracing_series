//! # Sphere
//!
//! A library for handling ray intersections with a sphere

use super::{
    get_sphere_uv, ArcHittable, ArcMaterial, Float, HitRecord, Hittable, Point3, Ray, Vec3, AABB,
};
use std::fmt;
use std::sync::Arc;

/// Models a sphere.
#[derive(Debug, Clone)]
pub struct Sphere {
    /// Center of the sphere.
    center: Point3,

    /// Radius of the sphere.
    radius: Float,

    /// Surface material.
    material: ArcMaterial,
}

impl fmt::Display for Sphere {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "sphere(center: {}, radius: {}, material: {})",
            self.center, self.radius, self.material
        )
    }
}

impl Sphere {
    /// Create a new sphere.
    ///
    /// * `center` - Center.
    /// * `radius` - Radius.
    /// * `material` - Surface material.
    pub fn new(center: Vec3, radius: Float, material: ArcMaterial) -> ArcHittable {
        Arc::new(Sphere {
            center,
            radius,
            material: Arc::clone(&material),
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

        // Move point so it is relative to origin (0, 0, 0) to get uv-coordinates.
        let p = (point - self.center) / self.radius;
        let (u, v) = get_sphere_uv(&p);

        HitRecord::new(
            ray,
            t,
            point,
            outward_normal,
            Arc::clone(&self.material),
            u,
            v,
        )
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

    /// Create a bounding box across time interval `[t0, t1]`.
    ///
    /// * `_time0` - Start time of motion (ignored).
    /// * `_time1` - End time of motion (ignored).
    fn bounding_box(&self, _time0: Float, _time1: Float) -> Option<AABB> {
        // Negative radii used to create hollow bubbles with dielectric materials.
        let r = self.radius.abs();
        let r = Vec3::new(r, r, r);
        Some(AABB::new(self.center - r, self.center + r))
    }
}
