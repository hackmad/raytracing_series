//! # Common
//!
//! A library of common utility functinos.

#![allow(dead_code)]
use super::algebra::Vec3;

/// Use f64 since it provides the wider range of math operations.
pub type Float = f64;

/// The constant `π`.
pub const PI: Float = std::f64::consts::PI;

/// The constant `+∞`.
pub const INFINITY: Float = std::f64::INFINITY;

/// Returns a random floating point values in [0, 1].
pub fn random() -> Float {
    rand::random::<Float>()
}

/// Returns a random floating point values in [`min`, `max`].
///
/// * `min` - Minimum bound
/// * `max` - Maximum bound
pub fn random_in_range(min: Float, max: Float) -> Float {
    min + (max - min) * random()
}

/// Returns a random vector with random components in [0, 1].
pub fn random_vec3() -> Vec3 {
    Vec3::new(random(), random(), random())
}

/// Returns a random vector with random components in [`min`, `max`].
///
/// * `min` - Minimum bound
/// * `max` - Maximum bound
pub fn random_vec3_in_range(min: Float, max: Float) -> Vec3 {
    Vec3::new(
        random_in_range(min, max),
        random_in_range(min, max),
        random_in_range(min, max),
    )
}

/// Returns a random vector within the unit sphere. This vector is not
/// normalized.
pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_vec3_in_range(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            break p;
        }
    }
}

/// Returns a random unit vector by picking points on the unit sphere
/// and then normalizing it.
pub fn random_unit_vec3() -> Vec3 {
    let a = random_in_range(0.0, 2.0 * PI);
    let z = random_in_range(-1.0, 1.0);
    let r = (1.0 - z * z).sqrt();
    Vec3::new(r * a.cos(), r * a.sin(), z)
}

/// Returns a random vector with uniform scatter direction for all angles
/// away from a hit point, with no dependence on the angle from the normal.
///
/// * `normal` - THe surface normal.
pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
        // In the same hemisphere as the normal
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

/// Returns a random point inside unit disk
pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(random_in_range(-1.0, 1.0), random_in_range(-1.0, 1.0), 0.0);
        if p.length_squared() < 1.0 {
            break p;
        }
    }
}

/// Clamp the given value.clamp
///
/// * `x` - The value to clamp.
/// * `min` - Minimum bound.
/// * `max` - Maximum bound.
pub fn clamp(x: Float, min: Float, max: Float) -> Float {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
