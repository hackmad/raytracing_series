//! # Scene
//!
//! A library for handling scene data.

#![allow(dead_code)]

use super::algebra::*;
use super::camera::*;
use super::common::*;
use super::material::*;
use super::object::*;
use super::texture::*;
use std::rc::Rc;
use std::time::Instant;

#[derive(Debug)]
pub enum Scenery {
    LambertianDiffuse,
    Metal,
    Dielectric,
    WideAngle,
    Telephoto,
    DefocusBlur,
    RandomSpheres,
    MotionBlur,
    CheckeredFloor,
    CheckeredSpheres,
}

/// Models a scene.
pub struct Scene {
    /// The camera.
    pub camera: Camera,

    /// Objects in the scene.
    pub world: RcHittable,
}

impl Scene {
    /// Create a new scene.
    ///
    /// * `scenery` - Scene.
    /// * `image_width` - Image width.
    /// * `image_height` - Image height.
    /// * `bvh_enabled` - Use bounding volume hierarchy.
    /// * `rng` - Random number generator.
    pub fn new(
        scenery: Scenery,
        image_width: u32,
        image_height: u32,
        bvh_enabled: bool,
        rng: RcRandomizer,
    ) -> Scene {
        let (world, camera) = match scenery {
            Scenery::LambertianDiffuse => (
                diffuse_spheres(Rc::clone(&rng)),
                default_camera(image_width, image_height, Rc::clone(&rng)),
            ),
            Scenery::Metal => (
                metal_spheres(Rc::clone(&rng)),
                default_camera(image_width, image_height, Rc::clone(&rng)),
            ),
            Scenery::Dielectric => (
                dielectric_spheres(Rc::clone(&rng)),
                default_camera(image_width, image_height, Rc::clone(&rng)),
            ),
            Scenery::WideAngle => (
                dielectric_spheres(Rc::clone(&rng)),
                wide_angle_camera(image_width, image_height, Rc::clone(&rng)),
            ),
            Scenery::Telephoto => (
                dielectric_spheres(Rc::clone(&rng)),
                telephoto_camera(image_width, image_height, Rc::clone(&rng)),
            ),
            Scenery::DefocusBlur => (
                dielectric_spheres(Rc::clone(&rng)),
                large_aperture_camera(image_width, image_height, Rc::clone(&rng)),
            ),
            Scenery::RandomSpheres => (
                random_spheres(false, false, Rc::clone(&rng)),
                random_spheres_camera(image_width, image_height, Rc::clone(&rng)),
            ),
            Scenery::MotionBlur => (
                random_spheres(true, false, Rc::clone(&rng)),
                random_spheres_camera(image_width, image_height, Rc::clone(&rng)),
            ),
            Scenery::CheckeredFloor => (
                random_spheres(true, true, Rc::clone(&rng)),
                random_spheres_camera(image_width, image_height, Rc::clone(&rng)),
            ),
            Scenery::CheckeredSpheres => (
                checkered_spheres(Rc::clone(&rng)),
                checkered_spheres_camera(image_width, image_height, Rc::clone(&rng)),
            ),
        };

        let start = Instant::now();
        let world = if bvh_enabled {
            eprint!("BVH: ");
            build_bvh(&world, Rc::clone(&rng))
        } else {
            eprint!("HittableList: ");
            build_hittable_list(&world)
        };
        eprintln!("{} seconds", start.elapsed().as_secs_f32());

        Scene { camera, world }
    }
}

fn build_hittable_list(objects: &Vec<RcHittable>) -> RcHittable {
    let mut world = HittableList::new();

    for o in objects.iter() {
        world.add(Rc::clone(&o));
    }

    Rc::new(world)
}

fn build_bvh(objects: &Vec<RcHittable>, rng: RcRandomizer) -> RcHittable {
    let mut obj: Vec<RcHittable> = Vec::new();
    for o in objects {
        obj.push(Rc::clone(&o));
    }
    BVH::new(&mut obj, 0.0, 1.0, Rc::clone(&rng))
}

fn default_camera(image_width: u32, image_height: u32, rng: RcRandomizer) -> Camera {
    Camera::new(
        Point3::zero(),
        Point3::new(0.0, 0.0, -1.0),
        Point3::new(0.0, 1.0, 0.0),
        90.0,
        (image_width as Float) / (image_height as Float),
        0.001,
        100.0,
        0.0,
        1.0,
        Rc::clone(&rng),
    )
}

fn wide_angle_camera(image_width: u32, image_height: u32, rng: RcRandomizer) -> Camera {
    Camera::new(
        Point3::new(-2.0, 2.0, 1.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        (image_width as Float) / (image_height as Float),
        0.001,
        100.0,
        0.0,
        1.0,
        Rc::clone(&rng),
    )
}

fn telephoto_camera(image_width: u32, image_height: u32, rng: RcRandomizer) -> Camera {
    Camera::new(
        Point3::new(-2.0, 2.0, 1.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        (image_width as Float) / (image_height as Float),
        0.001,
        100.0,
        0.0,
        1.0,
        Rc::clone(&rng),
    )
}

fn large_aperture_camera(image_width: u32, image_height: u32, rng: RcRandomizer) -> Camera {
    let lookfrom = Point3::new(3.0, 3.0, 2.0);
    let lookat = Point3::new(0.0, 0.0, -1.0);

    Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        (image_width as Float) / (image_height as Float),
        2.0,
        (lookfrom - lookat).length(),
        0.0,
        1.0,
        Rc::clone(&rng),
    )
}

fn random_spheres_camera(image_width: u32, image_height: u32, rng: RcRandomizer) -> Camera {
    Camera::new(
        Point3::new(13.0, 2.0, 3.0),
        Point3::zero(),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        (image_width as Float) / (image_height as Float),
        0.1,
        10.0,
        0.0,
        1.0,
        Rc::clone(&rng),
    )
}

fn checkered_spheres_camera(image_width: u32, image_height: u32, rng: RcRandomizer) -> Camera {
    Camera::new(
        Point3::new(13.0, 2.0, 3.0),
        Point3::zero(),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        (image_width as Float) / (image_height as Float),
        0.0,
        10.0,
        0.0,
        1.0,
        Rc::clone(&rng),
    )
}

fn diffuse_spheres(rng: RcRandomizer) -> Vec<RcHittable> {
    vec![
        Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            Lambertian::new(Solid::from_rgb(0.5, 0.5, 0.5), Rc::clone(&rng)),
        ),
        Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            Lambertian::new(Solid::from_rgb(0.5, 0.5, 0.5), Rc::clone(&rng)),
        ),
    ]
}

fn metal_spheres(rng: RcRandomizer) -> Vec<RcHittable> {
    vec![
        Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            Lambertian::new(Solid::from_rgb(0.7, 0.3, 0.3), Rc::clone(&rng)),
        ),
        Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            Lambertian::new(Solid::from_rgb(0.8, 0.8, 0.0), Rc::clone(&rng)),
        ),
        Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            Metal::new(Solid::from_rgb(0.8, 0.6, 0.2), 1.0, Rc::clone(&rng)),
        ),
        Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            0.5,
            Metal::new(Solid::from_rgb(0.8, 0.8, 0.8), 0.3, Rc::clone(&rng)),
        ),
    ]
}

fn dielectric_spheres(rng: RcRandomizer) -> Vec<RcHittable> {
    vec![
        Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            Lambertian::new(Solid::from_rgb(0.1, 0.2, 0.5), Rc::clone(&rng)),
        ),
        Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            Lambertian::new(Solid::from_rgb(0.8, 0.8, 0.0), Rc::clone(&rng)),
        ),
        Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            Metal::new(Solid::from_rgb(0.8, 0.6, 0.2), 0.3, Rc::clone(&rng)),
        ),
        Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            0.5,
            Dielectric::new(1.5, Rc::clone(&rng)),
        ),
        Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            // use negative radius for hollow sphere, the geometry is unaffected,
            // but the surface normal points inward.
            -0.45,
            Dielectric::new(1.5, Rc::clone(&rng)),
        ),
    ]
}

/// Generate some fixed spheres and a lot of smaller rng spheres.
fn random_spheres(motion_blur: bool, checkered_floor: bool, rng: RcRandomizer) -> Vec<RcHittable> {
    let mut world: Vec<RcHittable> = Vec::new();

    let albedo = if checkered_floor {
        Checker::new(
            Solid::from_rgb(0.2, 0.3, 0.1),
            Solid::from_rgb(0.9, 0.9, 0.9),
        )
    } else {
        Solid::from_rgb(0.5, 0.5, 0.5)
    };
    world.push(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(albedo, Rc::clone(&rng)),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.float();

            let center = Point3::new(
                a as Float + 0.9 * rng.float(),
                0.2,
                b as Float + 0.9 * rng.float(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = (rng.vec3() * rng.vec3()).as_colour();

                    if motion_blur {
                        let y = rng.clone().float_in_range(0.0, 0.5);
                        world.push(MovingSphere::new(
                            center,
                            center + Vec3::new(0.0, y, 0.0),
                            0.0,
                            1.0,
                            0.2,
                            Lambertian::new(Solid::new(albedo), Rc::clone(&rng)),
                        ));
                    } else {
                        world.push(Sphere::new(
                            center,
                            0.2,
                            Lambertian::new(Solid::new(albedo), Rc::clone(&rng)),
                        ));
                    }
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Solid::new(rng.vec3_in_range(0.5, 1.0).as_colour());
                    let fuzz = rng.float_in_range(0.0, 0.5);
                    world.push(Sphere::new(
                        center,
                        0.2,
                        Metal::new(albedo, fuzz, Rc::clone(&rng)),
                    ));
                } else {
                    // Glass
                    world.push(Sphere::new(
                        center,
                        0.2,
                        Dielectric::new(1.5, Rc::clone(&rng)),
                    ));
                }
            }
        }
    }

    world.push(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric::new(1.5, Rc::clone(&rng)),
    ));

    world.push(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::new(Solid::from_rgb(0.4, 0.2, 0.1), Rc::clone(&rng)),
    ));

    world.push(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(Solid::from_rgb(0.7, 0.6, 0.5), 0.0, Rc::clone(&rng)),
    ));

    world
}

fn checkered_spheres(rng: RcRandomizer) -> Vec<RcHittable> {
    let mut world: Vec<RcHittable> = Vec::new();

    let checker = Checker::new(
        Solid::from_rgb(0.2, 0.3, 0.1),
        Solid::from_rgb(0.9, 0.9, 0.9),
    );

    world.push(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Lambertian::new(Rc::clone(&checker), Rc::clone(&rng)),
    ));

    world.push(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Lambertian::new(Rc::clone(&checker), Rc::clone(&rng)),
    ));

    world
}
