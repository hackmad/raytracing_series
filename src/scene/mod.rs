//! # Scene
//!
//! A library for handling scene data.

#![allow(dead_code)]

use super::algebra::*;
use super::background::*;
use super::camera::*;
use super::common::*;
use super::material::*;
use super::object::*;
use super::texture::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Instant;

/// Scene types.
#[derive(Debug, Copy, Clone)]
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
    PerlinSpheres,
    Earth,
    SimpleLight,
    EmptyCornellBox,
    CornellBox,
}

impl<'a> Scenery {
    /// Returns a new `HashMap<&str, Scenery>`.
    ///
    /// There is probably a better way to do this.  To get the string
    /// representations you can do this:
    /// ```
    /// let scene_map = Scenery::all();
    /// let scenes: Vec<&str> = scene_map.keys().map(|k| k.as_ref()).collect();
    /// ```
    pub fn all() -> HashMap<&'a str, Scenery> {
        let mut map = HashMap::new();

        map.insert("lambertian_diffuse", Scenery::LambertianDiffuse);
        map.insert("metal", Scenery::Metal);
        map.insert("dielectric", Scenery::Dielectric);
        map.insert("telephoto", Scenery::Telephoto);
        map.insert("wide_angle", Scenery::WideAngle);
        map.insert("defocus_blur", Scenery::DefocusBlur);
        map.insert("random_spheres", Scenery::RandomSpheres);
        map.insert("motion_blur", Scenery::MotionBlur);
        map.insert("checkered_floor", Scenery::CheckeredFloor);
        map.insert("checkered_spheres", Scenery::CheckeredSpheres);
        map.insert("perlin_spheres", Scenery::PerlinSpheres);
        map.insert("earth", Scenery::Earth);
        map.insert("simple_light", Scenery::SimpleLight);
        map.insert("empty_cornell_box", Scenery::EmptyCornellBox);
        map.insert("cornell_box", Scenery::CornellBox);

        map
    }
}

/// Models a scene.
pub struct Scene {
    /// The camera.
    pub camera: Camera,

    /// Objects in the scene.
    pub world: RcHittable,

    /// Background.
    pub background: BackgroundFn,
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
        match scenery {
            Scenery::LambertianDiffuse => Scene::new_scene(
                &diffuse_spheres(Rc::clone(&rng)),
                default_camera(image_width, image_height, Rc::clone(&rng)),
                gradient_background,
                bvh_enabled,
                rng,
            ),
            Scenery::Metal => Scene::new_scene(
                &metal_spheres(Rc::clone(&rng)),
                default_camera(image_width, image_height, Rc::clone(&rng)),
                gradient_background,
                bvh_enabled,
                rng,
            ),
            Scenery::Dielectric => Scene::new_scene(
                &dielectric_spheres(Rc::clone(&rng)),
                default_camera(image_width, image_height, Rc::clone(&rng)),
                gradient_background,
                bvh_enabled,
                rng,
            ),
            Scenery::WideAngle => Scene::new_scene(
                &dielectric_spheres(Rc::clone(&rng)),
                wide_angle_camera(image_width, image_height, Rc::clone(&rng)),
                gradient_background,
                bvh_enabled,
                rng,
            ),
            Scenery::Telephoto => Scene::new_scene(
                &dielectric_spheres(Rc::clone(&rng)),
                telephoto_camera(image_width, image_height, Rc::clone(&rng)),
                gradient_background,
                bvh_enabled,
                rng,
            ),
            Scenery::DefocusBlur => Scene::new_scene(
                &dielectric_spheres(Rc::clone(&rng)),
                large_aperture_camera(image_width, image_height, Rc::clone(&rng)),
                gradient_background,
                bvh_enabled,
                rng,
            ),
            Scenery::RandomSpheres => Scene::new_scene(
                &random_spheres(false, false, Rc::clone(&rng)),
                random_spheres_camera(image_width, image_height, Rc::clone(&rng)),
                gradient_background,
                bvh_enabled,
                rng,
            ),
            Scenery::MotionBlur => Scene::new_scene(
                &random_spheres(true, false, Rc::clone(&rng)),
                random_spheres_camera(image_width, image_height, Rc::clone(&rng)),
                gradient_background,
                bvh_enabled,
                rng,
            ),
            Scenery::CheckeredFloor => Scene::new_scene(
                &random_spheres(true, true, Rc::clone(&rng)),
                random_spheres_camera(image_width, image_height, Rc::clone(&rng)),
                gradient_background,
                bvh_enabled,
                rng,
            ),
            Scenery::CheckeredSpheres => Scene::new_scene(
                &checkered_spheres(Rc::clone(&rng)),
                checkered_spheres_camera(image_width, image_height, Rc::clone(&rng)),
                gradient_background,
                bvh_enabled,
                rng,
            ),
            Scenery::PerlinSpheres => Scene::new_scene(
                &perlin_spheres(Rc::clone(&rng)),
                checkered_spheres_camera(image_width, image_height, Rc::clone(&rng)),
                gradient_background,
                bvh_enabled,
                rng,
            ),
            Scenery::Earth => Scene::new_scene(
                &earth(Rc::clone(&rng)),
                earth_camera(image_width, image_height, Rc::clone(&rng)),
                gradient_background,
                bvh_enabled,
                rng,
            ),
            Scenery::SimpleLight => Scene::new_scene(
                &simple_light(Rc::clone(&rng)),
                simple_light_camera(image_width, image_height, Rc::clone(&rng)),
                black_background,
                bvh_enabled,
                rng,
            ),
            Scenery::EmptyCornellBox => Scene::new_scene(
                &empty_cornell_box(Rc::clone(&rng)),
                cornell_box_camera(image_width, image_height, Rc::clone(&rng)),
                black_background,
                bvh_enabled,
                rng,
            ),
            Scenery::CornellBox => Scene::new_scene(
                &cornell_box(Rc::clone(&rng)),
                cornell_box_camera(image_width, image_height, Rc::clone(&rng)),
                black_background,
                bvh_enabled,
                rng,
            ),
        }
    }

    fn new_scene(
        world: &Vec<RcHittable>,
        camera: Camera,
        background: BackgroundFn,
        bvh_enabled: bool,
        rng: RcRandomizer,
    ) -> Scene {
        Scene {
            world: build_world(world, bvh_enabled, Rc::clone(&rng)),
            camera,
            background,
        }
    }
}

fn build_world(world: &Vec<RcHittable>, bvh_enabled: bool, rng: RcRandomizer) -> RcHittable {
    let start = Instant::now();

    let world = if bvh_enabled {
        eprint!("BVH: ");
        build_bvh(&world, Rc::clone(&rng))
    } else {
        eprint!("HittableList: ");
        build_hittable_list(&world)
    };

    eprintln!("{} seconds", start.elapsed().as_secs_f32());

    world
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

fn earth_camera(image_width: u32, image_height: u32, rng: RcRandomizer) -> Camera {
    Camera::new(
        Point3::new(0.0, 0.0, 12.0),
        Point3::zero(),
        Point3::new(0.0, 1.0, 0.0),
        20.0,
        (image_width as Float) / (image_height as Float),
        0.001,
        100.0,
        0.0,
        1.0,
        Rc::clone(&rng),
    )
}

fn simple_light_camera(image_width: u32, image_height: u32, rng: RcRandomizer) -> Camera {
    Camera::new(
        Point3::new(26.0, 3.0, 6.0),
        Point3::new(0.0, 2.0, 0.0),
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

fn cornell_box_camera(image_width: u32, image_height: u32, rng: RcRandomizer) -> Camera {
    Camera::new(
        Point3::new(278.0, 278.0, -800.0),
        Point3::new(278.0, 278.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        40.0,
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
            Lambertian::new(SolidColour::from_rgb(0.5, 0.5, 0.5), Rc::clone(&rng)),
        ),
        Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            Lambertian::new(SolidColour::from_rgb(0.5, 0.5, 0.5), Rc::clone(&rng)),
        ),
    ]
}

fn metal_spheres(rng: RcRandomizer) -> Vec<RcHittable> {
    vec![
        Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            Lambertian::new(SolidColour::from_rgb(0.7, 0.3, 0.3), Rc::clone(&rng)),
        ),
        Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            Lambertian::new(SolidColour::from_rgb(0.8, 0.8, 0.0), Rc::clone(&rng)),
        ),
        Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            Metal::new(SolidColour::from_rgb(0.8, 0.6, 0.2), 1.0, Rc::clone(&rng)),
        ),
        Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            0.5,
            Metal::new(SolidColour::from_rgb(0.8, 0.8, 0.8), 0.3, Rc::clone(&rng)),
        ),
    ]
}

fn dielectric_spheres(rng: RcRandomizer) -> Vec<RcHittable> {
    vec![
        Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            Lambertian::new(SolidColour::from_rgb(0.1, 0.2, 0.5), Rc::clone(&rng)),
        ),
        Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            Lambertian::new(SolidColour::from_rgb(0.8, 0.8, 0.0), Rc::clone(&rng)),
        ),
        Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            Metal::new(SolidColour::from_rgb(0.8, 0.6, 0.2), 0.3, Rc::clone(&rng)),
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
            SolidColour::from_rgb(0.2, 0.3, 0.1),
            SolidColour::from_rgb(0.9, 0.9, 0.9),
        )
    } else {
        SolidColour::from_rgb(0.5, 0.5, 0.5)
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
                            Lambertian::new(SolidColour::new(albedo), Rc::clone(&rng)),
                        ));
                    } else {
                        world.push(Sphere::new(
                            center,
                            0.2,
                            Lambertian::new(SolidColour::new(albedo), Rc::clone(&rng)),
                        ));
                    }
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = SolidColour::new(rng.vec3_in_range(0.5, 1.0).as_colour());
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
        Lambertian::new(SolidColour::from_rgb(0.4, 0.2, 0.1), Rc::clone(&rng)),
    ));

    world.push(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(SolidColour::from_rgb(0.7, 0.6, 0.5), 0.0, Rc::clone(&rng)),
    ));

    world
}

fn checkered_spheres(rng: RcRandomizer) -> Vec<RcHittable> {
    let mut world: Vec<RcHittable> = Vec::new();

    let checker = Checker::new(
        SolidColour::from_rgb(0.2, 0.3, 0.1),
        SolidColour::from_rgb(0.9, 0.9, 0.9),
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

fn perlin_spheres(rng: RcRandomizer) -> Vec<RcHittable> {
    let mut world: Vec<RcHittable> = Vec::new();

    let noise = Noise::new(4.0, 7, 10.0, 256, Rc::clone(&rng));

    world.push(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(Rc::clone(&noise), Rc::clone(&rng)),
    ));

    world.push(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Lambertian::new(Rc::clone(&noise), Rc::clone(&rng)),
    ));

    world
}

fn earth(rng: RcRandomizer) -> Vec<RcHittable> {
    let mut world: Vec<RcHittable> = Vec::new();

    let earth_texture = Image::new("images/world.topo.bathy.200412.3x5400x2700.jpg");

    world.push(Sphere::new(
        Point3::zero(),
        2.0,
        Lambertian::new(Rc::clone(&earth_texture), Rc::clone(&rng)),
    ));

    world
}

fn simple_light(rng: RcRandomizer) -> Vec<RcHittable> {
    let mut world: Vec<RcHittable> = Vec::new();

    for s in perlin_spheres(Rc::clone(&rng)) {
        world.push(s);
    }

    let light = DiffuseLight::new(SolidColour::from_rgb(4.0, 4.0, 4.0), Rc::clone(&rng));

    world.push(Sphere::new(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        Rc::clone(&light),
    ));

    world.push(XYrect::new(3.0, 5.0, 1.0, 3.0, -2.0, Rc::clone(&light)));

    world
}

fn empty_cornell_box(rng: RcRandomizer) -> Vec<RcHittable> {
    let mut world: Vec<RcHittable> = Vec::new();

    let red = Lambertian::new(SolidColour::from_rgb(0.65, 0.05, 0.05), Rc::clone(&rng));
    let white = Lambertian::new(SolidColour::from_rgb(0.73, 0.73, 0.73), Rc::clone(&rng));
    let green = Lambertian::new(SolidColour::from_rgb(0.12, 0.45, 0.15), Rc::clone(&rng));
    let light = DiffuseLight::new(SolidColour::from_rgb(15.0, 15.0, 15.0), Rc::clone(&rng));

    world.push(FlipFace::new(YZrect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Rc::clone(&green),
    )));

    world.push(YZrect::new(0.0, 555.0, 0.0, 555.0, 0.0, Rc::clone(&red)));

    world.push(XZrect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        Rc::clone(&light),
    ));

    world.push(FlipFace::new(XZrect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Rc::clone(&white),
    )));

    world.push(XZrect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Rc::clone(&white),
    ));

    world.push(FlipFace::new(XYrect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Rc::clone(&white),
    )));

    world
}

fn cornell_box(rng: RcRandomizer) -> Vec<RcHittable> {
    let mut world: Vec<RcHittable> = Vec::new();

    let objects = empty_cornell_box(Rc::clone(&rng));
    for object in objects.iter() {
        world.push(Rc::clone(&object));
    }

    let white = Lambertian::new(SolidColour::from_rgb(0.73, 0.73, 0.73), Rc::clone(&rng));

    world.push(Translate::new(
        Rotate::new(
            XYZbox::new(
                Point3::zero(),
                Point3::new(165.0, 330.0, 165.0),
                Rc::clone(&white),
            ),
            Y_AXIS,
            15.0,
        ),
        Vec3::new(265.0, 0.0, 295.0),
    ));

    world.push(Translate::new(
        Rotate::new(
            XYZbox::new(
                Point3::zero(),
                Point3::new(165.0, 165.0, 165.0),
                Rc::clone(&white),
            ),
            Y_AXIS,
            -18.0,
        ),
        Vec3::new(130.0, 0.0, 65.0),
    ));

    world
}
