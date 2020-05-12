//! # Background
//!
//! A library for providing colour values for rays that don't intersect
//! any object in the scene.

use super::algebra::{Colour, Ray};

/// Function type accepts a ray and returns background colour.
pub type BackgroundFn = fn(ray: &Ray) -> Colour;

/// Return black. Use this if there are emissive objects in the scene.
///
/// * `_ray` - The ray (unused).
pub fn black_background(_ray: &Ray) -> Colour {
    Colour::zero()
}

/// Generate a gradient colour for the background. This is useful when
/// there are no emissive objects in the scene that act as a light source.
///
/// * `ray` - The ray.
pub fn gradient_background(ray: &Ray) -> Colour {
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Colour::new(1.0, 1.0, 1.0) * (1.0 - t) + Colour::new(0.5, 0.7, 1.0) * t
}
