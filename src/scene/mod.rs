//! # Scene
//!
//! A library for handling scene data.

#![allow(dead_code)]

use super::algebra::*;
use super::camera::*;
use super::common::*;
use super::material::*;
use super::object::*;

#[derive(Debug)]
pub enum Scenery {
    LambertianDiffuse,
    Metal,
    Dielectric,
    CameraViewpoint,
    CameraFov,
    DefocusBlur,
    RandomSpheres,
    MotionBlur,
}

/// Models a scene.
pub struct Scene {
    /// The camera.
    pub camera: Camera,

    /// Objects in the scene.
    pub world: HittableList,
}

impl Scene {
    pub fn new(scenery: Scenery, image_width: u32, image_height: u32) -> Scene {
        match scenery {
            Scenery::LambertianDiffuse => diffuse_spheres(image_width, image_height),
            Scenery::Metal => metal_spheres(image_width, image_height),
            Scenery::Dielectric => dielectric_spheres(image_width, image_height),
            Scenery::CameraViewpoint => camera_viewpoint(image_width, image_height),
            Scenery::CameraFov => camera_fov(image_width, image_height),
            Scenery::DefocusBlur => defocus_blur(image_width, image_height),
            Scenery::RandomSpheres => random_spheres(image_width, image_height),
            Scenery::MotionBlur => motion_blur(image_width, image_height),
        }
    }
}

fn default_camera(image_width: u32, image_height: u32) -> Camera {
    Camera::new(
        Vec3::zero().as_point(),
        Vec3::new(0.0, 0.0, -1.0).as_point(),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        (image_width as Float) / (image_height as Float),
        0.001,
        100.0,
        0.0,
        1.0,
    )
}

fn diffuse_spheres(image_width: u32, image_height: u32) -> Scene {
    let mut world = HittableList::new();

    world.add(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0).as_point(),
        0.5,
        Lambertian::new(Vec3::new(0.5, 0.5, 0.5).as_colour()),
    ));
    world.add(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0).as_point(),
        100.0,
        Lambertian::new(Vec3::new(0.5, 0.5, 0.5).as_colour()),
    ));

    let camera = default_camera(image_width, image_height);

    Scene { camera, world }
}

fn metal_spheres(image_width: u32, image_height: u32) -> Scene {
    let mut world = HittableList::new();

    world.add(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0).as_point(),
        0.5,
        Lambertian::new(Vec3::new(0.7, 0.3, 0.3).as_colour()),
    ));
    world.add(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0).as_point(),
        100.0,
        Lambertian::new(Vec3::new(0.8, 0.8, 0.0).as_colour()),
    ));
    world.add(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0).as_point(),
        0.5,
        Metal::new(Vec3::new(0.8, 0.6, 0.2).as_colour(), 1.0),
    ));
    world.add(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0).as_point(),
        0.5,
        Metal::new(Vec3::new(0.8, 0.8, 0.8).as_colour(), 0.3),
    ));

    let camera = default_camera(image_width, image_height);

    Scene { camera, world }
}

fn dielectric_world() -> HittableList {
    let mut world = HittableList::new();

    world.add(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0).as_point(),
        0.5,
        Lambertian::new(Vec3::new(0.1, 0.2, 0.5).as_colour()),
    ));
    world.add(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0).as_point(),
        100.0,
        Lambertian::new(Vec3::new(0.8, 0.8, 0.0).as_colour()),
    ));
    world.add(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0).as_point(),
        0.5,
        Metal::new(Vec3::new(0.8, 0.6, 0.2).as_colour(), 0.3),
    ));
    world.add(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0).as_point(),
        0.5,
        Dielectric::new(1.5),
    ));
    world.add(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0).as_point(),
        // use negative radius for hollow sphere, the geometry is unaffected,
        // but the surface normal points inward.
        -0.45,
        Dielectric::new(1.5),
    ));

    world
}
fn dielectric_spheres(image_width: u32, image_height: u32) -> Scene {
    let world = dielectric_world();
    let camera = default_camera(image_width, image_height);
    Scene { camera, world }
}

fn camera_viewpoint(image_width: u32, image_height: u32) -> Scene {
    let world = dielectric_world();
    let camera = Camera::new(
        Vec3::new(-2.0, 2.0, 1.0).as_point(),
        Vec3::new(0.0, 0.0, -1.0).as_point(),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        (image_width as Float) / (image_height as Float),
        0.001,
        100.0,
        0.0,
        1.0,
    );
    Scene { camera, world }
}

fn camera_fov(image_width: u32, image_height: u32) -> Scene {
    let world = dielectric_world();
    let camera = Camera::new(
        Vec3::new(-2.0, 2.0, 1.0).as_point(),
        Vec3::new(0.0, 0.0, -1.0).as_point(),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        (image_width as Float) / (image_height as Float),
        0.001,
        100.0,
        0.0,
        1.0,
    );
    Scene { camera, world }
}

fn defocus_blur(image_width: u32, image_height: u32) -> Scene {
    let world = dielectric_world();

    let lookfrom = Vec3::new(3.0, 3.0, 2.0).as_point();
    let lookat = Vec3::new(0.0, 0.0, -1.0).as_point();

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        (image_width as Float) / (image_height as Float),
        2.0,
        (lookfrom - lookat).length(),
        0.0,
        1.0,
    );
    Scene { camera, world }
}

/// Generate some fixed spheres and a lot of smaller random spheres.
fn random_world(motion_blur: bool) -> HittableList {
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

                    if motion_blur {
                        world.add(MovingSphere::new(
                            center,
                            center + Vec3::new(0.0, random_in_range(0.0, 0.5), 0.0),
                            0.0,
                            1.0,
                            0.2,
                            Lambertian::new(albedo),
                        ));
                    } else {
                        world.add(Sphere::new(center, 0.2, Lambertian::new(albedo)));
                    }
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

/// Create new scene with some random geometric objects and camera.
fn random_spheres(image_width: u32, image_height: u32) -> Scene {
    let world = random_world(false);
    let camera = Camera::new(
        Vec3::new(13.0, 2.0, 3.0).as_point(),
        Vec3::zero().as_point(),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        (image_width as Float) / (image_height as Float),
        0.1,
        10.0,
        0.0,
        1.0,
    );
    Scene { camera, world }
}

fn motion_blur(image_width: u32, image_height: u32) -> Scene {
    let world = random_world(true);
    let camera = Camera::new(
        Vec3::new(13.0, 2.0, 3.0).as_point(),
        Vec3::zero().as_point(),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        (image_width as Float) / (image_height as Float),
        0.1,
        10.0,
        0.0,
        1.0,
    );
    Scene { camera, world }
}
