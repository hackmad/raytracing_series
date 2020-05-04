#![allow(dead_code)]

use super::clamp;
use super::Float;
use std::ops;

#[derive(Copy, Clone)]
pub struct Vec3 {
    e: [Float; 3],
}

pub type Colour = Vec3;
pub type Point3 = Vec3;

impl Vec3 {
    pub fn zero() -> Vec3 {
        Vec3 {
            e: [0.0 as Float, 0.0 as Float, 0.0 as Float],
        }
    }

    pub fn new(x: Float, y: Float, z: Float) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn x(self) -> Float {
        self.e[0]
    }

    pub fn y(self) -> Float {
        self.e[1]
    }

    pub fn z(self) -> Float {
        self.e[2]
    }

    pub fn length_squared(self) -> Float {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn length(self) -> Float {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(self) -> Vec3 {
        let len = self.length().recip();
        Vec3 {
            e: [self.e[0] * len, self.e[1] * len, self.e[2] * len],
        }
    }

    pub fn dot(self, v: Vec3) -> Float {
        self.e[0] * v.e[0] + self.e[1] * v.e[1] + self.e[2] * v.e[2]
    }

    pub fn cross(self, v: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[1] * v.e[2] - self.e[2] * v.e[1],
                self.e[2] * v.e[0] - self.e[0] * v.e[2],
                self.e[0] * v.e[1] - self.e[1] * v.e[0],
            ],
        }
    }

    pub fn reflect(self, n: Vec3) -> Vec3 {
        self - n * self.dot(n) * (2.0 as Float)
    }

    pub fn refract(self, n: Vec3, etai_over_etat: Float) -> Vec3 {
        let cos_theta = -self.dot(n);
        let r_out_parallel = (self + n * cos_theta) * etai_over_etat;
        let r_out_perp = n * -(1.0 - r_out_parallel.length_squared()).sqrt();
        r_out_parallel + r_out_perp
    }

    pub fn as_point(self) -> Point3 {
        self as Point3
    }

    pub fn as_colour(self) -> Colour {
        self as Colour
    }

    pub fn to_colour_from_sample(self, samples_per_pixel: u32) -> Colour {
        // Divide the color total by the number of samples
        let s = 1.0 / samples_per_pixel as Float;

        // Gamma-correct for a gamma value of 2.0 (sqrt)
        Vec3::new(
            256.0 * clamp((self.x() * s).sqrt(), 0.0, 0.999),
            256.0 * clamp((self.y() * s).sqrt(), 0.0, 0.999),
            256.0 * clamp((self.z() * s).sqrt(), 0.0, 0.999),
        )
        .as_colour()
    }

    pub fn to_ppm(self) -> String {
        format!("{} {} {}", self.x() as u8, self.y() as u8, self.z() as u8)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

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

    fn mul(self, f: Float) -> Vec3 {
        Vec3 {
            e: [self.e[0] * f, self.e[1] * f, self.e[2] * f],
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

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
    fn mul_assign(&mut self, f: Float) {
        *self = Self {
            e: [self.e[0] * f, self.e[1] * f, self.e[2] * f],
        }
    }
}

impl ops::MulAssign<Vec3> for Vec3 {
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

    fn div(self, f: Float) -> Vec3 {
        self * f.recip()
    }
}

impl ops::DivAssign<Float> for Vec3 {
    fn div_assign(&mut self, f: Float) {
        *self *= f.recip()
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        self * (-1.0)
    }
}
