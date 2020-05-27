//! # Camera
//!
//! A library for handling cameras that use a thin lens approximation to
//! handle defocus blur.

#![allow(dead_code)]
use super::algebra::{Point3, Ray, Vec3};
use super::common::{Float, Random};
use std::fmt;

#[derive(Clone)]
pub struct Camera {
    /// The lower left corner of the image plane.
    lower_left_corner: Point3,

    /// The horizontal vector along image plane.
    horizontal: Vec3,

    /// The vertical vector along image plane.
    vertical: Vec3,

    // The origin point where camera is located.
    origin: Point3,

    // Radius of camera lens.
    lens_radius: Float,

    /// Orthonomal basis vector u describing the orientation.
    u: Vec3,

    /// Orthonomal basis vector v describing the orientation.
    v: Vec3,

    /// Orthonomal basis vector w describing the orientation.
    w: Vec3,

    /// Keeps track of start time for motion blur.
    time0: Float,

    /// Keeps track of end time for motion blur.
    time1: Float,
}

impl fmt::Display for Camera {
    /// Display the camera configuration.
    ///
    /// * `f` - Formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "camera(lower_left_corner: {}, horizontal: {}, vertical: {}, \
                origin: {}, lens_radius: {}, u: {}, v: {}, w: {}, \
                time0: {}, time1: {})",
            self.lower_left_corner,
            self.horizontal,
            self.vertical,
            self.origin,
            self.lens_radius,
            self.u,
            self.v,
            self.w,
            self.time0,
            self.time1
        )
    }
}

impl fmt::Debug for Camera {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Point")
            .field("lower_left_corner", &self.lower_left_corner)
            .field("horizontal", &self.horizontal)
            .field("vertical", &self.vertical)
            .field("origin", &self.origin)
            .field("lens_radius", &self.lens_radius)
            .field("u", &self.u)
            .field("v", &self.v)
            .field("w", &self.w)
            .field("time0", &self.time0)
            .field("time1", &self.time1)
            .finish()
    }
}

impl Camera {
    /// Create a new camera.
    ///
    /// * `lookfrom` - Location of camera.
    /// * `lookat` - Point towards which camera is looking.
    /// * `vup` - The vector representing the up direction.
    /// * `vfov` - Vertical field of view in degrees.
    /// * `aspect_ratio` - The aspect ratio of image.
    /// * `aperture` - The camere aperture.
    /// * `focus_dist` - The distance to focal plane.
    /// * `time0` - Start time for motion blur.
    /// * `time1` - End time for motion blur.
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: Float,
        aspect_ratio: Float,
        aperture: Float,
        focus_dist: Float,
        time0: Float,
        time1: Float,
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
            time0,
            time1,
        }
    }

    /// Returns a ray for the given parametric coordinates along the image
    /// image plane. The ray's time paramter is set at random value between
    /// `time0` and `time1` for motion blur effect.
    ///
    /// * `s`: Horizontal parameter.
    /// * `t`: Vertical parameter.
    pub fn get_ray(&self, s: Float, t: Float) -> Ray {
        let rd = Random::vec3_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();
        let time = Random::sample_in_range(self.time0, self.time1);

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
            time,
        )
    }
}
