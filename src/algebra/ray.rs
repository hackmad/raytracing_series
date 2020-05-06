//! # Ray
//!
//! A library for handling rays.

use super::Float;
use super::Point3;
use super::Vec3;

/// Models a ray that originates at a point and has a direction.
#[derive(Copy, Clone)]
pub struct Ray {
    /// Origin.
    pub origin: Point3,

    /// Direction.
    pub direction: Vec3,

    /// Time at which ray exists.
    pub time: Float,
}

impl Ray {
    /// Creates a new ray with the given origin and direction.
    ///
    /// * `origin` - The starting point of the ray.
    /// * `direction` - The direction vector of the ray.
    /// * `time` - The time at which the ray exists.
    pub fn new(origin: Point3, direction: Vec3, time: Float) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }

    /// Calculates a point along the ray based on parameter `t`.
    ///
    /// * `t`: The parameter.
    pub fn at(self, t: Float) -> Point3 {
        self.origin + self.direction * t
    }
}
