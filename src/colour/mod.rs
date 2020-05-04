#![allow(dead_code)]

use super::algebra::Vec3;
use super::common::clamp;
use super::common::Float;

#[derive(Copy, Clone)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Colour {
    pub fn new(r: u8, g: u8, b: u8) -> Colour {
        Colour { r, g, b }
    }

    pub fn new_from_vec3(v: Vec3, samples_per_pixel: u32) -> Colour {
        // Divide the color total by the number of samples
        let s = 1.0 / samples_per_pixel as Float;

        // Gamma-correct for a gamma value of 2.0 (sqrt)
        Colour {
            r: (256.0 * clamp((v.x() * s).sqrt(), 0.0, 0.999)) as u8,
            g: (256.0 * clamp((v.y() * s).sqrt(), 0.0, 0.999)) as u8,
            b: (256.0 * clamp((v.z() * s).sqrt(), 0.0, 0.999)) as u8,
        }
    }

    pub fn as_ppm(self) -> String {
        format!("{} {} {}", self.r, self.g, self.b)
    }
}
