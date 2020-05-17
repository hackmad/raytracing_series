//! # Moving Sphere
//!
//! A library for handling ray intersections with a sphere that moves
//! along a linear path.

use super::{
    get_sphere_uv, ArcHittable, ArcMaterial, Float, HitRecord, Hittable, Point3, Ray, Vec3, AABB,
};
use std::fmt;
use std::sync::Arc;

/// Models a sphere that moves along a linear path.
#[derive(Debug, Clone)]
pub struct MovingSphere {
    /// Center of the sphere at start time of motion.
    center0: Point3,

    /// Center of the sphere at end time of motion.
    center1: Point3,

    /// Start time of motion.
    time0: Float,

    /// End time of motion.
    time1: Float,

    /// Radius of the sphere.
    radius: Float,

    /// Surface material.
    material: ArcMaterial,
}

impl fmt::Display for MovingSphere {
    /// Display the moving sphere parameters.
    ///
    /// * `f` - Formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "moving_sphere(center0: {}, center1: {}, \
            time0: {}, time1: {} radius: {}, material: {})",
            self.center0, self.center1, self.time0, self.time1, self.radius, self.material
        )
    }
}

impl MovingSphere {
    /// Create a new moving sphere.
    ///
    /// * `center0` - Center at start time of motion.
    /// * `center1` - Center at end time of motion.
    /// * `time0` - Start time of motion.
    /// * `time1` - End time of motion.
    /// * `radius` - Radius.
    /// * `material` - Surface material.
    pub fn new(
        center0: Point3,
        center1: Point3,
        time0: Float,
        time1: Float,
        radius: Float,
        material: ArcMaterial,
    ) -> ArcHittable {
        Arc::new(MovingSphere {
            center0,
            center1,
            time0,
            time1,
            radius,
            material: Arc::clone(&material),
        })
    }

    /// Returns the center of the sphere at given time by linearly
    /// interpolating between start and end time of motion.
    ///
    /// * `time` - Time parameter to interpolate sphere position.
    pub fn center(&self, time: Float) -> Point3 {
        if self.time0 == self.time1 {
            self.center0 // avoid divide by 0 by assuming no motion.
        } else {
            let s = (time - self.time0) / (self.time1 - self.time0);
            self.center0 + (self.center1 - self.center0) * s
        }
    }

    /// Returns a `HitRecord` containing the intersection point, outward
    /// surface normal for the incident ray and given parameter.
    ///
    /// * `ray` - Incident ray.
    /// * `t` - Parameter along the ray where intersection occurred.
    fn get_hit_record(&self, ray: &Ray, t: Float) -> HitRecord {
        let point = ray.at(t);
        let outward_normal = (point - self.center(ray.time)) / self.radius;

        // Move point so it is relative to origin (0, 0, 0) to get uv-coordinates.
        let p = (point - self.center(t)) / self.radius;
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

impl Hittable for MovingSphere {
    /// Calculate the intersection of a ray with the objects.
    ///
    /// * `ray` - The incident ray.
    /// * `t_min` - The minium parameter for intersections.
    /// * `t_max` - The maximum parameter for intersections.
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let oc = ray.origin - self.center(ray.time);
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
    /// * `time0` - Start time of motion.
    /// * `time1` - End time of motion.
    fn bounding_box(&self, time0: Float, time1: Float) -> Option<AABB> {
        // Negative radii used to create hollow bubbles with dielectric materials.
        let r = self.radius.abs();
        let r = Vec3::new(r, r, r);
        let box0 = AABB::new(self.center(time0) - r, self.center(time0) + r);
        let box1 = AABB::new(self.center(time1) - r, self.center(time1) + r);
        Some(AABB::surrounding_box(box0, box1))
    }
}
