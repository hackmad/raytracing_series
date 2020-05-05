//! # Scene
//!
//! A library for handling scene data.

use super::algebra::*;
use super::camera::*;
use super::common::*;
use super::material::*;
use super::object::*;

/// Models a scene.
pub struct Scene {
    /// The camera.
    pub camera: Camera,

    /// Objects in the scene.
    pub world: HittableList,
}

impl Scene {
    /// Create new scene with some random geometric objects and camera.
    pub fn new_random_scene(image_width: u32, image_height: u32) -> Scene {
        let aspect_ratio = (image_width as Float) / (image_height as Float);

        let lookfrom = Vec3::new(13.0, 2.0, 3.0).as_point();
        let lookat = Vec3::zero().as_point();
        let vup = Vec3::new(0.0, 1.0, 0.0);
        let focus_dist = 10.0;
        let aperture = 0.1;
        let vfov = 20.0;

        Scene {
            camera: Camera::new(
                lookfrom,
                lookat,
                vup,
                vfov,
                aspect_ratio,
                aperture,
                focus_dist,
            ),

            world: random_world(),
        }
    }
}

/// Generate some fixed spheres and a lot of smaller random spheres.
fn random_world() -> HittableList {
    let mut world = HittableList::new();

    world.add(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0).as_point(),
        1000.0,
        Lambertian::new(Vec3::new(0.5, 0.5, 0.5).as_colour()),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random();

            let center = Vec3::new(
                a as Float + 0.9 * random(),
                0.2,
                b as Float + 0.9 * random(),
            )
            .as_point();

            if (center - Vec3::new(4.0, 0.2, 0.0).as_point()).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = (random_vec3() * random_vec3()).as_colour();
                    world.add(Sphere::new(center, 0.2, Lambertian::new(albedo)));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = random_vec3_in_range(0.5, 1.0).as_colour();
                    let fuzz = random_in_range(0.0, 0.5);
                    world.add(Sphere::new(center, 0.2, Metal::new(albedo, fuzz)));
                } else {
                    // Glass
                    world.add(Sphere::new(center, 0.2, Dielectric::new(1.5)));
                }
            }
        }
    }

    world.add(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0).as_point(),
        1.0,
        Dielectric::new(1.5),
    ));

    world.add(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0).as_point(),
        1.0,
        Lambertian::new(Vec3::new(0.4, 0.2, 0.1).as_colour()),
    ));

    world.add(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0).as_point(),
        1.0,
        Metal::new(Vec3::new(0.7, 0.6, 0.5).as_colour(), 0.0),
    ));

    world
}
