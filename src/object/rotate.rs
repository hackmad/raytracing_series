//! # Rotate
//!
//! A library for handling ray intersections with rotated objects.

use super::{Axis, Float, HitRecord, Hittable, Point3, Ray, RcHittable, Vec3, AABB, INFINITY};
use std::fmt;
use std::rc::Rc;

/// Models a rotated object.
#[derive(Debug, Clone)]
pub struct Rotate {
    /// Holds a `Hittable`.
    object: RcHittable,

    /// Axis of rotation.
    axis: Axis,

    /// sin(`radians`).
    sin_theta: Float,

    /// cos(`radians`).
    cos_theta: Float,

    /// Bounding box.
    bbox: AABB,
}

impl fmt::Display for Rotate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "rotate(object: {}, axis: {}, bbox: {}, sin_theta: {}, cos_theta: {})",
            self.object, self.axis, self.bbox, self.sin_theta, self.cos_theta,
        )
    }
}

impl Rotate {
    /// Create a new axis aligned box.
    ///
    /// * `object`: Holds a `Hittable`.
    /// * `axis`: Axis of rotation.
    /// * `degrees: Float` - Rotation angle.
    pub fn new(object: RcHittable, axis: Axis, degrees: Float) -> RcHittable {
        let radians = degrees.to_radians();

        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let bbox = Rc::clone(&object)
            .bounding_box(0.0, 1.0) // Motion is not supported. So (0.0, 1.0) ok.
            .expect("Missing bounding box for rotated object");

        let mut min: [Float; 3] = [INFINITY, INFINITY, INFINITY];
        let mut max: [Float; 3] = [-INFINITY, -INFINITY, -INFINITY];

        // TODO: Handle x and z axis rotations.
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = (i as Float) * bbox.max.x() + (1.0 - i as Float) * bbox.min.x();
                    let y = (j as Float) * bbox.max.y() + (1.0 - j as Float) * bbox.min.y();
                    let z = (k as Float) * bbox.max.z() + (1.0 - k as Float) * bbox.min.z();

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        Rc::new(Rotate {
            object: Rc::clone(&object),
            axis,
            sin_theta,
            cos_theta,
            bbox: AABB::new(Point3::from_array(min), Point3::from_array(max)),
        })
    }
}

impl Hittable for Rotate {
    /// Calculate the intersection of a ray with the objects.
    ///
    /// * `ray` - The incident ray.
    /// * `t_min` - The minium parameter for intersections.
    /// * `t_max` - The maximum parameter for intersections.
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let ox = self.cos_theta * ray.origin[0] - self.sin_theta * ray.origin[2];
        let oy = ray.origin[1];
        let oz = self.sin_theta * ray.origin[0] + self.cos_theta * ray.origin[2];

        let dx = self.cos_theta * ray.direction[0] - self.sin_theta * ray.direction[2];
        let dy = ray.direction[1];
        let dz = self.sin_theta * ray.direction[0] + self.cos_theta * ray.direction[2];

        let rotated_r = Ray::new(Point3::new(ox, oy, oz), Point3::new(dx, dy, dz), ray.time);

        if let Some(rec) = Rc::clone(&self.object).hit(&rotated_r, t_min, t_max) {
            let px = self.cos_theta * rec.point[0] + self.sin_theta * rec.point[2];
            let py = rec.point[1];
            let pz = -self.sin_theta * rec.point[0] + self.cos_theta * rec.point[2];

            let nx = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
            let ny = rec.normal[1];
            let nz = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

            Some(
                rec.update_point(Point3::new(px, py, pz))
                    .update_normal(&rotated_r, Vec3::new(nx, ny, nz)),
            )
        } else {
            None
        }
    }

    /// Create a bounding box across time interval `[t0, t1]`.
    ///
    /// * `time0` - Start time of motion.
    /// * `time1` - End time of motion.
    fn bounding_box(&self, _time0: Float, _time1: Float) -> Option<AABB> {
        Some(self.bbox)
    }
}
