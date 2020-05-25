//! # Rotate
//!
//! A library for handling ray intersections with rotated objects.

use super::{
    ArcHittable, Axis, Float, HitRecord, Hittable, Point3, Ray, Vec3, AABB, INFINITY, X_AXIS,
    Y_AXIS, Z_AXIS,
};
use std::fmt;
use std::sync::Arc;

/// Models a rotated object.
#[derive(Debug, Clone)]
pub struct Rotate {
    /// Holds a `Hittable`.
    object: ArcHittable,

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
    /// Display the rotation parameters.
    ///
    /// * `f` - Formatter.
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
    pub fn new(object: ArcHittable, axis: Axis, degrees: Float) -> ArcHittable {
        let radians = degrees.to_radians();

        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        match get_rotated_bbox(Arc::clone(&object), axis, sin_theta, cos_theta) {
            Ok(bbox) => Arc::new(Rotate {
                object: Arc::clone(&object),
                axis,
                sin_theta,
                cos_theta,
                bbox,
            }),
            Err(e) => panic!(e),
        }
    }
}

impl Hittable for Rotate {
    /// Calculate the intersection of a ray with the objects.
    ///
    /// * `ray` - The incident ray.
    /// * `t_min` - The minium parameter for intersections.
    /// * `t_max` - The maximum parameter for intersections.
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        // Rotate ray into the coordinate frame of the object.
        let o = rotate_neg(&ray.origin, self.axis, self.sin_theta, self.cos_theta);
        let d = rotate_neg(&ray.direction, self.axis, self.sin_theta, self.cos_theta);
        let rotated_r = Ray::new(o, d, ray.time);

        if let Some(rec) = self.object.hit(&rotated_r, t_min, t_max) {
            // Rotate hit point and normal out of the coordinate frame of the object.
            let p = rotate(&rec.point, self.axis, self.sin_theta, self.cos_theta);
            let n = rotate(&rec.normal, self.axis, self.sin_theta, self.cos_theta);
            Some(rec.update_point(p).update_normal(&rotated_r, n))
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

    /// Sample PDF value at hit point and given direction.
    ///
    /// * `origin` - Hit point.
    /// * `v` - Direction to sample.
    fn pdf_value(&self, origin: Point3, v: Vec3) -> Float {
        self.object.pdf_value(
            rotate_neg(&origin, self.axis, self.sin_theta, self.cos_theta),
            rotate_neg(&v, self.axis, self.sin_theta, self.cos_theta),
        )
    }

    /// Generate a random direction towards this object.
    ///
    /// * `origin` - Hit point.
    fn random(&self, origin: Point3) -> Vec3 {
        self.object.random(rotate_neg(
            &origin,
            self.axis,
            self.sin_theta,
            self.cos_theta,
        ))
    }
}

/// Calculates the bounding box for an object rotated about a coordinate axis.
///
/// * `object` - Object to rotate.bbox_y_rot
/// * `axis` - Axis of rotation
/// * `sin_theta` - Sine of angle of rotation.
/// * `cos_theta` - Cosine of angle of rotation.
fn get_rotated_bbox<'a>(
    object: ArcHittable,
    axis: Axis,
    sin_theta: Float,
    cos_theta: Float,
) -> Result<AABB, &'a str> {
    // Motion is not supported. So (0.0, 1.0) ok.
    if let Some(bbox) = object.bounding_box(0.0, 1.0) {
        let mut min: [Float; 3] = [INFINITY, INFINITY, INFINITY];
        let mut max: [Float; 3] = [-INFINITY, -INFINITY, -INFINITY];

        for i in 0..2 {
            let ii = i as Float;

            for j in 0..2 {
                let jj = j as Float;

                for k in 0..2 {
                    let kk = k as Float;

                    let x = ii * bbox.max.x() + (1.0 - ii) * bbox.min.x();
                    let y = jj * bbox.max.y() + (1.0 - jj) * bbox.min.y();
                    let z = kk * bbox.max.z() + (1.0 - kk) * bbox.min.z();

                    let tester = rotate(&Vec3::new(x, y, z), axis, sin_theta, cos_theta);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        return Ok(AABB::new(Point3::from_array(min), Point3::from_array(max)));
    }

    Err("Missing bounding box for rotated object")
}

/// Rotate a point/vector around a coordinate axis by angle θ.
///
/// * `v` - Point/vector.
/// * `sin_theta` - sin(θ).
/// * `cos_theta` - cos(θ).
fn rotate<'a>(v: &Vec3, axis: Axis, sin_theta: Float, cos_theta: Float) -> Vec3 {
    let (x, y, z) = (v[0], v[1], v[2]);

    match axis {
        X_AXIS => {
            let newy = y * cos_theta - z * sin_theta;
            let newz = z * cos_theta + y * sin_theta;
            Vec3::new(x, newy, newz)
        }
        Y_AXIS => {
            let newx = x * cos_theta + z * sin_theta;
            let newz = z * cos_theta - x * sin_theta;
            Vec3::new(newx, y, newz)
        }
        Z_AXIS => {
            let newx = x * cos_theta - y * sin_theta;
            let newy = y * cos_theta + x * sin_theta;
            Vec3::new(newx, newy, z)
        }
        _ => panic!(format!("Invalid axis {}", axis)),
    }
}

/// Rotate a point/vector around a coordinate axis by -θ.
///
/// Note that the sin/cos of θ is provided and we use the identities
/// `sin(-θ) = -sin(θ)` and `cos(-θ) = cos(θ)`.
///
/// * `v` - Point/vector.
/// * `sin_theta` - sin(θ).
/// * `cos_theta` - cos(θ).
fn rotate_neg<'a>(v: &Vec3, axis: Axis, sin_theta: Float, cos_theta: Float) -> Vec3 {
    rotate(v, axis, -sin_theta, cos_theta)
}
