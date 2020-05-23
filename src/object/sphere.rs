//! # Sphere
//!
//! A library for handling ray intersections with a sphere

use super::{
    get_sphere_uv, ArcHittable, ArcMaterial, ArcRandomizer, Float, HitRecord, Hittable, Point3,
    Ray, Vec3, AABB, INFINITY, ONB, RAY_EPSILON, TWO_PI,
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

    /// Random number generator.
    rng: ArcRandomizer,
}

impl fmt::Display for Sphere {
    /// Display the sphere parameters.
    ///
    /// * `f` - Formatter.
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
    /// * `rng` - Random number generator.
    pub fn new(
        center: Vec3,
        radius: Float,
        material: ArcMaterial,
        rng: ArcRandomizer,
    ) -> ArcHittable {
        Arc::new(Sphere {
            center,
            radius,
            material: Arc::clone(&material),
            rng: Arc::clone(&rng),
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

    /// Sample PDF value at hit point and given direction.
    ///
    /// * `origin` - Hit point.
    /// * `v` - Direction to sample.
    fn pdf_value(&self, origin: &Point3, v: &Vec3) -> Float {
        if let Some(_) = self.hit(&Ray::new(*origin, *v, 0.0), RAY_EPSILON, INFINITY) {
            let cos_theta_max =
                (1.0 - self.radius * self.radius / (self.center - *origin).length_squared()).sqrt();
            let solid_angle = TWO_PI * (1.0 - cos_theta_max);

            1.0 / solid_angle
        } else {
            0.0
        }
    }

    /// Generate a random direction towards this object.
    ///
    /// * `origin` - Hit point.
    fn random(&self, origin: &Point3) -> Vec3 {
        let direction = self.center - *origin;
        let distance_squared = direction.length_squared();
        let uvw = ONB::new(&direction);
        uvw.local_from_vec3(&self.rng.to_sphere(self.radius, distance_squared))
    }
}
