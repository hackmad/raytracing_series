use super::algebra::Ray;
use super::algebra::Vec3;
use super::common::Float;

#[derive(Copy, Clone)]
pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: Float, // vertical fov in degrees
        aspect_ratio: Float,
    ) -> Camera {
        let theta = vfov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        Camera {
            origin: lookfrom,
            lower_left_corner: lookfrom - u * half_width - v * half_height - w,
            horizontal: u * (2.0 * half_width),
            vertical: v * (2.0 * half_height),
        }
    }

    pub fn get_ray(self, s: Float, t: Float) -> Ray {
        let direction =
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin;
        Ray::new(self.origin, direction)
    }
}
