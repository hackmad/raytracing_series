//! # Vector
//!
//! A library for handling 3-dimensional vectors, points and colours.

#![allow(dead_code)]
use super::{clamp, Float};
use std::{fmt, ops};

/// Models a 3-dimensional vector.
#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    /// An array that holds the x, y and z components of a vector.
    e: [Float; 3],
}

/// Models an RGB colour value as a `Vec3`.
pub type Colour = Vec3;

/// Models a 3-dimensional point as a `Vec3`.
pub type Point3 = Vec3;

/// Type used for coordinate indexes.
pub type Axis = usize;

/// Constant representing x-coordinate index.
pub const X_AXIS: Axis = 0;

/// Constant representing y-coordinate index.
pub const Y_AXIS: Axis = 1;

/// Constant representing z-coordinate index.
pub const Z_AXIS: Axis = 2;

/// List of axes.
pub const AXES: &[Axis] = &[X_AXIS, Y_AXIS, Z_AXIS];

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.e)
    }
}

impl Vec3 {
    /// Creates a new zero vector `[0, 0, 0]`.
    pub fn zero() -> Vec3 {
        Vec3 {
            e: [0.0 as Float, 0.0 as Float, 0.0 as Float],
        }
    }

    /// Creates a new zero vector `[1, 1, 1]`.
    pub fn one() -> Vec3 {
        Vec3 {
            e: [1.0 as Float, 1.0 as Float, 1.0 as Float],
        }
    }

    /// Creates a new vector `[x, y, z]`.
    ///
    /// * `x` - The x-coordinate.
    /// * `y` - The y-coordinate.
    /// * `z` - THe z-coordinate.
    pub fn new(x: Float, y: Float, z: Float) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    /// Creates a new vector `[x, y, z]`.
    ///
    /// * `a` - Array containing x, y, z coordinates.
    pub fn from_array(a: [Float; 3]) -> Vec3 {
        Vec3 {
            e: [a[0], a[1], a[2]],
        }
    }

    /// Returns the x-component of the vector.
    pub fn x(self) -> Float {
        self.e[0]
    }

    /// Returns the y-component of the vector.
    pub fn y(self) -> Float {
        self.e[1]
    }

    /// Returns the z-component of the vector.
    pub fn z(self) -> Float {
        self.e[2]
    }

    /// Returns the square of the length of the vector.
    pub fn length_squared(&self) -> Float {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    /// Returns the length of the vector.
    pub fn length(&self) -> Float {
        self.length_squared().sqrt()
    }

    /// Returns the normalized unit vector.
    pub fn unit_vector(&self) -> Vec3 {
        let len = self.length().recip();
        Vec3 {
            e: [self.e[0] * len, self.e[1] * len, self.e[2] * len],
        }
    }

    /// Returns the dot product with a vector `v`,
    ///
    /// * `v` - The other vector.
    pub fn dot(&self, v: Vec3) -> Float {
        self.e[0] * v.e[0] + self.e[1] * v.e[1] + self.e[2] * v.e[2]
    }

    /// Returns the cross product with a vector `v`,
    ///
    /// * `v` - The other vector.
    pub fn cross(&self, v: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[1] * v.e[2] - self.e[2] * v.e[1],
                self.e[2] * v.e[0] - self.e[0] * v.e[2],
                self.e[0] * v.e[1] - self.e[1] * v.e[0],
            ],
        }
    }

    /// Returns the reflection along a vector `n`.
    ///
    /// * `n` - The vector along which to perform reflection.
    pub fn reflect(self, n: Vec3) -> Vec3 {
        self - n * self.dot(n) * (2.0 as Float)
    }

    /// Returns the refracted vector at a surface with normal `n` given the
    /// ratio of refractive indices of 2 materials `etai_over_etat`=`𝜂i/𝜂t`.
    /// The ratio should be calculated as refractive index of material where
    /// the ray came from `𝜂i` and the refractive index of material where the
    /// ray is transmitted `𝜂t`.
    pub fn refract(self, n: Vec3, etai_over_etat: Float) -> Vec3 {
        let cos_theta = -self.dot(n);
        let r_out_parallel = (self + n * cos_theta) * etai_over_etat;
        let r_out_perp = n * -(1.0 - r_out_parallel.length_squared()).sqrt();
        r_out_parallel + r_out_perp
    }

    /// Returns the vector as a `Point3`.
    pub fn as_point(self) -> Point3 {
        self as Point3
    }

    /// Returns the vector as a `Colour`.
    pub fn as_colour(self) -> Colour {
        self as Colour
    }

    /// Returns the gamma corrected sample colour value represented by this
    /// vector.
    ///
    /// * `samples_per_pixel` - The number of samples per pixel.
    pub fn to_colour_from_sample(self, samples_per_pixel: u32) -> Colour {
        // Divide the color total by the number of samples
        let s = 1.0 / samples_per_pixel as Float;

        // Gamma-correct for a gamma value of 2.0 (sqrt)
        Colour::new(
            256.0 * clamp((self.x() * s).sqrt(), 0.0, 0.999),
            256.0 * clamp((self.y() * s).sqrt(), 0.0, 0.999),
            256.0 * clamp((self.z() * s).sqrt(), 0.0, 0.999),
        )
    }

    /// Returns an array of `u8` to be used as rgb values.
    pub fn to_rgb(self) -> [u8; 3] {
        [self.x() as u8, self.y() as u8, self.z() as u8]
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    /// Adds the given vector `rhs` and returns the result.
    ///
    /// * `rhs` - The vector to add.
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    /// Adds the given vector `rhs` and assigns the result to `self`.
    ///
    /// * `other` - The vector to add.
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        };
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    /// Subtracts the given vector `rhs` and returns the result.
    ///
    /// * `rhs` - The vector to subtract.
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        }
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    /// Subtracts the given vector `rhs` and assigns the result to `self`.
    ///
    /// * `other` - The vector to subtract.
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        };
    }
}

impl ops::Mul<Float> for Vec3 {
    type Output = Vec3;

    /// Returns the vector scaled by factor `f`.
    ///
    /// * `g` - The scale factor.
    fn mul(self, f: Float) -> Vec3 {
        Vec3 {
            e: [self.e[0] * f, self.e[1] * f, self.e[2] * f],
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    /// Returns the result of component-wise multiplication of vector
    /// coordinates.
    ///
    /// * `other` - The vector to perform component-wise multiply.
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2],
            ],
        }
    }
}

impl ops::MulAssign<Float> for Vec3 {
    /// Scales the vector with a given factor `f` and assigns the
    /// result to `self`.
    ///
    /// * `f` - The scale factor.
    fn mul_assign(&mut self, f: Float) {
        *self = Self {
            e: [self.e[0] * f, self.e[1] * f, self.e[2] * f],
        }
    }
}

impl ops::MulAssign<Vec3> for Vec3 {
    /// Assigns the result of component-wise multiplication of vector
    /// coordinates to `self`.
    ///
    /// * `other` - The vector to perform component-wise multiply.
    fn mul_assign(&mut self, other: Vec3) {
        *self = Self {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2],
            ],
        }
    }
}

impl ops::Div<Float> for Vec3 {
    type Output = Vec3;

    /// Returns the vector scaled by factor `1/f`.
    ///
    /// * `f` - The reciprocal scale factor.
    fn div(self, f: Float) -> Vec3 {
        self * f.recip()
    }
}

impl ops::DivAssign<Float> for Vec3 {
    /// Scales the vector with a given factor `1/f` and assigns the
    /// result to `self`.
    ///
    /// * `f` - The reciprocal scale factor.
    fn div_assign(&mut self, f: Float) {
        *self *= f.recip()
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    /// Returns the vector scaled by factor `-1`.
    fn neg(self) -> Vec3 {
        self * (-1.0)
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = Float;

    /// Returns the vector components by index.
    ///
    /// * `i` - The index (0 -> x, 1 -> y, 2 -> z)
    fn index(&self, i: usize) -> &Self::Output {
        &self.e[i]
    }
}
