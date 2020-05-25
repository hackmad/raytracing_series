//! # ConstantMedium
//!
//! A library for handling ray intersections within a constant medium for
//! effects like smoke and fog.

use super::{
    ArcHittable, ArcMaterial, ArcRandomizer, ArcTexture, Float, HitRecord, Hittable, Isotropic,
    Ray, Vec3, AABB, INFINITY, MIN_THICKNESS,
};
use std::fmt;
use std::sync::Arc;

/// Models a constant medium for effects like smoke and fog.
#[derive(Debug, Clone)]
pub struct ConstantMedium {
    /// Boundary
    boundary: ArcHittable,

    /// -1/ρ where ρ is the density.
    neg_inv_density: Float,

    /// Phase function (this will be an isotropic material).
    phase_function: ArcMaterial,

    /// Random number generator
    rng: ArcRandomizer,
}

impl fmt::Display for ConstantMedium {
    /// Display the constant medium parameters.
    ///
    /// * `f` - Formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "constant_medium(boundary: {}, neg_inv_density: {}, phase_function: {})",
            self.boundary, self.neg_inv_density, self.phase_function
        )
    }
}

impl ConstantMedium {
    /// Create a new constant medium.
    ///
    /// * `boundary` - Object determines surface boundary (for now only
    ///   convex objects work)
    /// * `density` - Density of medium.
    /// * `albedo` - Provides diffuse colour.
    pub fn new(
        boundary: ArcHittable,
        density: Float,
        albedo: ArcTexture,
        rng: ArcRandomizer,
    ) -> ArcHittable {
        Arc::new(ConstantMedium {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Isotropic::new(albedo, Arc::clone(&rng)),
            rng: Arc::clone(&rng),
        })
    }
}

impl Hittable for ConstantMedium {
    /// Calculate the intersection of a ray with the objects.
    ///
    /// * `ray` - The incident ray.
    /// * `t_min` - The minium parameter for intersections.
    /// * `t_max` - The maximum parameter for intersections.
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        // Print occasional samples when debugging. To enable, set enable_debug true.
        let enable_debug = false;
        let debugging = enable_debug && self.rng.float() < 0.00001;

        let rec1 = self.boundary.hit(ray, -INFINITY, INFINITY);
        if rec1.is_none() {
            return None;
        }
        let mut t0 = rec1.unwrap().t;

        let rec2 = self.boundary.hit(ray, t0 + MIN_THICKNESS, INFINITY);
        if rec2.is_none() {
            return None;
        }
        let mut t1 = rec2.unwrap().t;

        if debugging {
            eprintln!("\nt0={}, t1={}", t0, t1);
        }

        if t0 < t_min {
            t0 = t_min;
        }
        if t1 > t_max {
            t1 = t_max;
        }

        if t0 >= t1 {
            return None;
        }

        if t0 < 0.0 {
            t0 = 0.0;
        }

        let ray_length = ray.direction.length();
        let distance_inside_boundary = (t1 - t0) * ray_length;
        let hit_distance = self.neg_inv_density * self.rng.float().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = t0 + hit_distance / ray_length;

        let rec = HitRecord::new(
            ray,
            t,
            ray.at(t),
            Vec3::new(1.0, 0.0, 0.0), // arbitrary normal
            Arc::clone(&self.phase_function),
            0.0, // arbitrary
            1.0, // arbitrary
        );

        if debugging {
            eprintln!("hit_distance = {}", hit_distance);
            eprintln!("rec.t = {}", rec.t);
            eprintln!("rec.p = {}", rec.point);
        }
        Some(rec)
    }

    /// Create a bounding box across time interval `[t0, t1]`.
    ///
    /// * `time0` - Start time of motion.
    /// * `time1` - End time of motion.
    fn bounding_box(&self, time0: Float, time1: Float) -> Option<AABB> {
        self.boundary.bounding_box(time0, time1)
    }
}
