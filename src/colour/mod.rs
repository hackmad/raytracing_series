#![allow(dead_code)]

use super::algebra::Vec3;
use super::common::clamp;
use super::common::Float;

pub type Colour = Vec3;

pub fn colour_from_vec3(v: Vec3) -> Colour {
    v as Colour
}

pub fn colour_from_sample(v: Vec3, samples_per_pixel: u32) -> Colour {
    // Divide the color total by the number of samples
    let s = 1.0 / samples_per_pixel as Float;

    // Gamma-correct for a gamma value of 2.0 (sqrt)
    Vec3::new(
        256.0 * clamp((v.x() * s).sqrt(), 0.0, 0.999),
        256.0 * clamp((v.y() * s).sqrt(), 0.0, 0.999),
        256.0 * clamp((v.z() * s).sqrt(), 0.0, 0.999),
    ) as Colour
}

pub fn colour_to_ppm(c: Colour) -> String {
    let r = c.x() as u8;
    let g = c.y() as u8;
    let b = c.z() as u8;
    format!("{} {} {}", r, g, b)
}
