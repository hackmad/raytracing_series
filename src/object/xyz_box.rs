//! # XYZbox
//!
//! A library for handling ray intersections with an axis aligned box.

use super::{
    ArcHittable, ArcMaterial, ArcRandomizer, FlipFace, Float, HitRecord, Hittable, HittableList,
    Point3, Ray, Vec3, XYrect, XZrect, YZrect, AABB,
};
use std::fmt;
use std::sync::Arc;

/// Models an axis-aligned box.
/// **NOTE:** XYZbox is used to avoid conflict with Rust's `Box`.
#[derive(Debug, Clone)]
pub struct XYZbox {
    /// Holds minimum (x0, y0, z0) coordinates.
    box_min: Point3,

    /// Holds maximum (x1, y1, z1) coordinates.
    box_max: Point3,

    /// Holds a `HittableList` containing the 6 axis aligned planes.
    sides: ArcHittable,
}

impl fmt::Display for XYZbox {
    /// Display the XYZbox parameters.
    ///
    /// * `f` - Formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "box(box_min: {}, box_max: {}, sides: {})",
            self.box_min, self.box_max, self.sides
        )
    }
}

impl XYZbox {
    /// Create a new axis aligned box.
    ///
    /// * `p0` - Holds minimum (x0, y0, z0) coordinates.
    /// * `p1` - Holds maximum (x1, y1, z1) coordinates.
    /// * `material` - Surface material.
    /// * `rng` - Random number generator.
    pub fn new(p0: Point3, p1: Point3, material: ArcMaterial, rng: ArcRandomizer) -> ArcHittable {
        let mut sides = HittableList::new(Arc::clone(&rng));

        sides.add(XYrect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
            Arc::clone(&material),
            Arc::clone(&rng),
        ));
        sides.add(FlipFace::new(XYrect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p0.z(),
            Arc::clone(&material),
            Arc::clone(&rng),
        )));

        sides.add(XZrect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p1.y(),
            Arc::clone(&material),
            Arc::clone(&rng),
        ));
        sides.add(FlipFace::new(XZrect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p0.y(),
            Arc::clone(&material),
            Arc::clone(&rng),
        )));

        sides.add(YZrect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p1.x(),
            Arc::clone(&material),
            Arc::clone(&rng),
        ));
        sides.add(FlipFace::new(YZrect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p0.x(),
            Arc::clone(&material),
            Arc::clone(&rng),
        )));

        Arc::new(XYZbox {
            box_min: p0,
            box_max: p1,
            sides: Arc::new(sides),
        })
    }
}

impl Hittable for XYZbox {
    /// Calculate the intersection of a ray with the objects.
    ///
    /// * `ray` - The incident ray.
    /// * `t_min` - The minium parameter for intersections.
    /// * `t_max` - The maximum parameter for intersections.
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }

    /// Create a bounding box across time interval `[t0, t1]`.
    ///
    /// * `_time0` - Start time of motion (ignored).
    /// * `_time1` - End time of motion (ignored).
    fn bounding_box(&self, _time0: Float, _time1: Float) -> Option<AABB> {
        Some(AABB::new(self.box_min, self.box_max))
    }

    /// Sample PDF value at hit point and given direction.
    ///
    /// * `origin` - Hit point.
    /// * `v` - Direction to sample.
    fn pdf_value(&self, origin: Point3, v: Vec3) -> Float {
        self.sides.pdf_value(origin, v)
    }

    /// Generate a random direction towards this object.
    ///
    /// * `origin` - Hit point.
    fn random(&self, origin: Point3) -> Vec3 {
        self.sides.random(origin)
    }
}
