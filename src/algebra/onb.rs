//! # ONB
//!
//! A library for handling orthonormal basis vectros

#![allow(dead_code)]
use super::{Float, Vec3};
use std::{fmt, ops};

/// Models an orthonormal basis vectors.
#[derive(Debug, Clone)]
pub struct ONB {
    /// Three mutually orthogonal vectors.
    pub axis: [Vec3; 3],
}

impl ONB {
    /// Creates a new orthonormal basis using a normalized vector `n` as the
    /// w-axis and generating `u` and `v` that are mutually orthogonal with it.
    ///
    /// * `n` - Any vector (usually a normal vector on a surface).
    pub fn new(n: Vec3) -> ONB {
        let w = n.unit_vector();

        let a = if w.x().abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };

        let v = w.cross(a).unit_vector();
        let u = w.cross(v);

        ONB { axis: [u, v, w] }
    }

    /// Returns the u-axis.
    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }

    /// Returns the v-axis.
    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }

    /// Returns the w-axis.
    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }

    /// Get a vector relative to the normal (w-axis).
    pub fn local(&self, a: Float, b: Float, c: Float) -> Vec3 {
        a * self.u() + b * self.v() + c * self.w()
    }

    // Get a vector relative to the normal (w-axis).
    pub fn local_from_vec3(&self, a: &Vec3) -> Vec3 {
        a.x() * self.u() + a.y() * self.v() + a.z() * self.w()
    }
}

impl fmt::Display for ONB {
    /// Display the vector coordinates.
    ///
    /// * `f` - Formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.axis)
    }
}

impl ops::Index<usize> for ONB {
    type Output = Vec3;

    /// Returns the axis vectors by index.
    ///
    /// * `i` - The index (0 -> x, 1 -> y, 2 -> z)
    fn index(&self, i: usize) -> &Self::Output {
        &self.axis[i]
    }
}
