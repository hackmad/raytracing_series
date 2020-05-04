#![allow(dead_code)]

use super::algebra::Vec3;

pub type Float = f32;
pub const PI: Float = std::f32::consts::PI;

pub fn random() -> Float {
    rand::random::<Float>()
}

pub fn random_in_range(min: Float, max: Float) -> Float {
    min + (max - min) * random()
}

pub fn random_vec3() -> Vec3 {
    Vec3::new(random(), random(), random())
}

pub fn random_vec3_in_range(min: Float, max: Float) -> Vec3 {
    Vec3::new(
        random_in_range(min, max),
        random_in_range(min, max),
        random_in_range(min, max),
    )
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_vec3_in_range(-1.0 as Float, 1.0 as Float);
        if p.length_squared() < 1.0 as Float {
            break p;
        }
    }
}

pub fn random_unit_vec3() -> Vec3 {
    let a = random_in_range(0.0 as Float, 2.0 * PI);
    let z = random_in_range(-1.0 as Float, 1.0 as Float);
    let r = (1.0 as Float - z * z).sqrt();
    Vec3::new(r * a.cos(), r * a.sin(), z)
}

pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 as Float {
        // In the same hemisphere as the normal
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn clamp(x: Float, min: Float, max: Float) -> Float {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
