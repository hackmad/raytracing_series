#![allow(dead_code)]

use super::algebra::Point3;
use super::algebra::Ray;
use super::algebra::Vec3;
use super::common::random_in_unit_disk;
use super::common::Float;

#[derive(Copy, Clone)]
pub struct Camera {
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Point3,
    lens_radius: Float,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: Float, // vertical fov in degrees
        aspect_ratio: Float,
        aperture: Float,
        focus_dist: Float,
    ) -> Camera {
        let theta = vfov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        Camera {
            origin: lookfrom,
            lower_left_corner: lookfrom
                - u * (half_width * focus_dist)
                - v * (half_height * focus_dist)
                - w * focus_dist,
            horizontal: u * (2.0 * half_width * focus_dist),
            vertical: v * (2.0 * half_height * focus_dist),
            lens_radius: aperture / 2.0,
            u,
            v,
            w,
        }
    }

    pub fn get_ray(self, s: Float, t: Float) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
        )
    }
}
