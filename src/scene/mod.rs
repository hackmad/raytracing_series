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
use std::sync::Arc;
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
    FinalOneWeekend,
    MotionBlur,
    CheckeredFloor,
    CheckeredSpheres,
    PerlinSpheres,
    Earth,
    SimpleLight,
    EmptyCornellBox,
    CornellBox,
    SmokeAndFog,
    FinalNextWeek,
    RotateSpheres,
    SpecularReflections,
    FinalRestOfYourLife,
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
        map.insert("final_one_weekend", Scenery::FinalOneWeekend);
        map.insert("motion_blur", Scenery::MotionBlur);
        map.insert("checkered_floor", Scenery::CheckeredFloor);
        map.insert("checkered_spheres", Scenery::CheckeredSpheres);
        map.insert("perlin_spheres", Scenery::PerlinSpheres);
        map.insert("earth", Scenery::Earth);
        map.insert("simple_light", Scenery::SimpleLight);
        map.insert("empty_cornell_box", Scenery::EmptyCornellBox);
        map.insert("cornell_box", Scenery::CornellBox);
        map.insert("smoke_and_fog", Scenery::SmokeAndFog);
        map.insert("final_next_week", Scenery::FinalNextWeek);
        map.insert("rotate_spheres", Scenery::RotateSpheres);
        map.insert("specular_reflections", Scenery::SpecularReflections);
        map.insert("final_rest_of_your_life", Scenery::FinalRestOfYourLife);

        map
    }
}

/// Models a scene.
#[derive(Clone)]
pub struct Scene {
    /// The camera.
    pub camera: Camera,

    /// Objects in the scene.
    pub world: ArcHittable,

    /// Lights in the scene.
    pub lights: ArcHittable,

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
    pub fn new(scenery: Scenery, image_width: u32, image_height: u32, bvh_enabled: bool) -> Scene {
        match scenery {
            Scenery::LambertianDiffuse => diffuse_spheres(image_width, image_height, bvh_enabled),
            Scenery::Metal => metal_spheres(image_width, image_height, bvh_enabled),
            Scenery::Dielectric => dielectric_spheres(image_width, image_height, bvh_enabled),
            Scenery::WideAngle => {
                wide_angle_dielectric_spheres(image_width, image_height, bvh_enabled)
            }
            Scenery::Telephoto => {
                telephoto_dielectric_spheres(image_width, image_height, bvh_enabled)
            }
            Scenery::DefocusBlur => {
                defocus_blue_dielectric_spheres(image_width, image_height, bvh_enabled)
            }
            Scenery::FinalOneWeekend => final_one_weekend(image_width, image_height, bvh_enabled),
            Scenery::MotionBlur => motion_blur(image_width, image_height, bvh_enabled),
            Scenery::CheckeredFloor => checkered_floor(image_width, image_height, bvh_enabled),
            Scenery::CheckeredSpheres => checkered_spheres(image_width, image_height, bvh_enabled),
            Scenery::PerlinSpheres => perlin_spheres(image_width, image_height, bvh_enabled),
            Scenery::Earth => earth(image_width, image_height, bvh_enabled),
            Scenery::SimpleLight => simple_light(image_width, image_height, bvh_enabled),
            Scenery::EmptyCornellBox => empty_cornell_box(image_width, image_height, bvh_enabled),
            Scenery::CornellBox => cornell_box(image_width, image_height, bvh_enabled),
            Scenery::SmokeAndFog => {
                cornell_box_smoke_and_fog(image_width, image_height, bvh_enabled)
            }
            Scenery::FinalNextWeek => final_next_week(image_width, image_height, bvh_enabled),
            Scenery::RotateSpheres => rotate_spheres(image_width, image_height, bvh_enabled),
            Scenery::SpecularReflections => {
                specular_reflections(image_width, image_height, bvh_enabled)
            }
            Scenery::FinalRestOfYourLife => {
                final_rest_of_your_life(image_width, image_height, bvh_enabled)
            }
        }
    }

    fn new_scene(
        world: &Vec<ArcHittable>,
        lights: &Vec<ArcHittable>,
        camera: Camera,
        background: BackgroundFn,
        bvh_enabled: bool,
    ) -> Scene {
        Scene {
            world: build_world(world, bvh_enabled),
            lights: build_hittable_list(lights),
            camera,
            background,
        }
    }
}

fn build_world(world: &Vec<ArcHittable>, bvh_enabled: bool) -> ArcHittable {
    let start = Instant::now();

    let world = if bvh_enabled {
        eprint!("BVH: ");
        build_bvh(&world)
    } else {
        eprint!("HittableList: ");
        build_hittable_list(&world)
    };

    eprintln!("{} seconds", start.elapsed().as_secs_f32());

    world
}

fn build_hittable_list(objects: &Vec<ArcHittable>) -> ArcHittable {
    let mut world = HittableList::new();

    for o in objects.iter() {
        world.add(Arc::clone(&o));
    }

    Arc::new(world)
}

fn build_bvh(objects: &Vec<ArcHittable>) -> ArcHittable {
    let mut obj: Vec<ArcHittable> = Vec::new();
    for o in objects {
        obj.push(Arc::clone(&o));
    }
    BVH::new(&mut obj, 0.0, 1.0)
}

fn default_camera(image_width: u32, image_height: u32) -> Camera {
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
    )
}

fn random_spheres_camera(image_width: u32, image_height: u32) -> Camera {
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
    )
}

fn checkered_spheres_camera(image_width: u32, image_height: u32) -> Camera {
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
    )
}

fn cornell_box_camera(image_width: u32, image_height: u32) -> Camera {
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
    )
}

fn light_box(size: Float) -> Vec<ArcHittable> {
    let light = DiffuseLight::new(SolidColour::from_rgb(0.5, 0.7, 1.0));
    let top_light = FlipFace::new(XZrect::new(
        -size,
        size,
        -size,
        size,
        size,
        Arc::clone(&light),
    ));
    let bottom_light = XZrect::new(-size, size, -size, size, -size, Arc::clone(&light));
    let back_light = XYrect::new(-size, size, -size, size, -size, Arc::clone(&light));
    let front_light = FlipFace::new(XYrect::new(
        -size,
        size,
        -size,
        size,
        size,
        Arc::clone(&light),
    ));
    let left_light = FlipFace::new(YZrect::new(
        -size,
        size,
        -size,
        size,
        size,
        Arc::clone(&light),
    ));
    let right_light = YZrect::new(-size, size, -size, size, -size, Arc::clone(&light));

    vec![
        top_light,
        bottom_light,
        back_light,
        front_light,
        left_light,
        right_light,
    ]
}

fn diffuse_spheres(image_width: u32, image_height: u32, bvh_enabled: bool) -> Scene {
    let mut world = vec![
        Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            Lambertian::new(SolidColour::from_rgb(0.5, 0.5, 0.5)),
        ),
        Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            Lambertian::new(SolidColour::from_rgb(0.5, 0.5, 0.5)),
        ),
    ];

    let lights = light_box(1000.0);
    for light in lights.clone() {
        world.push(Arc::clone(&light));
    }

    Scene::new_scene(
        &world,
        &lights,
        default_camera(image_width, image_height),
        gradient_background,
        bvh_enabled,
    )
}

fn metal_spheres(image_width: u32, image_height: u32, bvh_enabled: bool) -> Scene {
    let mut world = vec![
        Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            Lambertian::new(SolidColour::from_rgb(0.7, 0.3, 0.3)),
        ),
        Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            Lambertian::new(SolidColour::from_rgb(0.8, 0.8, 0.0)),
        ),
        Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            Metal::new(SolidColour::from_rgb(0.8, 0.6, 0.2), 1.0),
        ),
        Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            0.5,
            Metal::new(SolidColour::from_rgb(0.8, 0.8, 0.8), 0.3),
        ),
    ];

    let lights = light_box(1000.0);
    for light in lights.clone() {
        world.push(Arc::clone(&light));
    }

    Scene::new_scene(
        &world,
        &lights,
        default_camera(image_width, image_height),
        gradient_background,
        bvh_enabled,
    )
}

fn dielectric_spheres_objects() -> Vec<ArcHittable> {
    vec![
        Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            Lambertian::new(SolidColour::from_rgb(0.1, 0.2, 0.5)),
        ),
        Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            Lambertian::new(SolidColour::from_rgb(0.8, 0.8, 0.0)),
        ),
        Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            Metal::new(SolidColour::from_rgb(0.8, 0.6, 0.2), 0.3),
        ),
        Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, Dielectric::new(1.5)),
        Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            // use negative radius for hollow sphere, the geometry is unaffected,
            // but the surface normal points inward.
            -0.45,
            Dielectric::new(1.5),
        ),
    ]
}

fn dielectric_spheres(image_width: u32, image_height: u32, bvh_enabled: bool) -> Scene {
    let mut world = dielectric_spheres_objects();

    let lights = light_box(1000.0);
    for light in lights.clone() {
        world.push(Arc::clone(&light));
    }

    Scene::new_scene(
        &world,
        &lights,
        default_camera(image_width, image_height),
        gradient_background,
        bvh_enabled,
    )
}

fn wide_angle_dielectric_spheres(image_width: u32, image_height: u32, bvh_enabled: bool) -> Scene {
    let mut world = dielectric_spheres_objects();

    let lights = light_box(1000.0);
    for light in lights.clone() {
        world.push(Arc::clone(&light));
    }

    let camera = Camera::new(
        Point3::new(-2.0, 2.0, 1.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        (image_width as Float) / (image_height as Float),
        0.001,
        100.0,
        0.0,
        1.0,
    );

    Scene::new_scene(&world, &lights, camera, gradient_background, bvh_enabled)
}

fn telephoto_dielectric_spheres(image_width: u32, image_height: u32, bvh_enabled: bool) -> Scene {
    let mut world = dielectric_spheres_objects();

    let lights = light_box(1000.0);
    for light in lights.clone() {
        world.push(Arc::clone(&light));
    }

    let camera = Camera::new(
        Point3::new(-2.0, 2.0, 1.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        (image_width as Float) / (image_height as Float),
        0.001,
        100.0,
        0.0,
        1.0,
    );

    Scene::new_scene(&world, &lights, camera, gradient_background, bvh_enabled)
}

fn defocus_blue_dielectric_spheres(
    image_width: u32,
    image_height: u32,
    bvh_enabled: bool,
) -> Scene {
    let mut world = dielectric_spheres_objects();

    let lights = light_box(1000.0);
    for light in lights.clone() {
        world.push(Arc::clone(&light));
    }

    let lookfrom = Point3::new(3.0, 3.0, 2.0);
    let lookat = Point3::new(0.0, 0.0, -1.0);

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

    Scene::new_scene(&world, &lights, camera, gradient_background, bvh_enabled)
}

/// Generate some fixed spheres and a lot of smaller random spheres.
fn random_spheres(motion_blur: bool, checkered_floor: bool) -> Vec<ArcHittable> {
    let mut world: Vec<ArcHittable> = Vec::new();

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
        Lambertian::new(albedo),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = Random::sample::<Float>();

            let center = Point3::new(
                a as Float + 0.9 * Random::sample::<Float>(),
                0.2,
                b as Float + 0.9 * Random::sample::<Float>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = (Random::vec3() * Random::vec3()).as_colour();

                    if motion_blur {
                        let y = Random::sample_in_range(0.0, 0.5);
                        world.push(MovingSphere::new(
                            center,
                            center + Vec3::new(0.0, y, 0.0),
                            0.0,
                            1.0,
                            0.2,
                            Lambertian::new(SolidColour::new(albedo)),
                        ));
                    } else {
                        world.push(Sphere::new(
                            center,
                            0.2,
                            Lambertian::new(SolidColour::new(albedo)),
                        ));
                    }
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = SolidColour::new(Random::vec3_in_range(0.5, 1.0).as_colour());
                    let fuzz = Random::sample_in_range(0.0, 0.5);
                    world.push(Sphere::new(center, 0.2, Metal::new(albedo, fuzz)));
                } else {
                    // Glass
                    world.push(Sphere::new(center, 0.2, Dielectric::new(1.5)));
                }
            }
        }
    }

    world.push(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric::new(1.5),
    ));

    world.push(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::new(SolidColour::from_rgb(0.4, 0.2, 0.1)),
    ));

    world.push(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(SolidColour::from_rgb(0.7, 0.6, 0.5), 0.0),
    ));

    world
}

fn final_one_weekend(image_width: u32, image_height: u32, bvh_enabled: bool) -> Scene {
    let mut world = random_spheres(false, false);

    let lights = light_box(1000.0);
    for light in lights.clone() {
        world.push(Arc::clone(&light));
    }

    Scene::new_scene(
        &world,
        &lights,
        random_spheres_camera(image_width, image_height),
        gradient_background,
        bvh_enabled,
    )
}

fn motion_blur(image_width: u32, image_height: u32, bvh_enabled: bool) -> Scene {
    let mut world = random_spheres(true, false);

    let lights = light_box(1000.0);
    for light in lights.clone() {
        world.push(Arc::clone(&light));
    }

    Scene::new_scene(
        &world,
        &lights,
        random_spheres_camera(image_width, image_height),
        gradient_background,
        bvh_enabled,
    )
}

fn checkered_floor(image_width: u32, image_height: u32, bvh_enabled: bool) -> Scene {
    let mut world = random_spheres(true, true);

    let lights = light_box(1000.0);
    for light in lights.clone() {
        world.push(Arc::clone(&light));
    }

    Scene::new_scene(
        &world,
        &lights,
        random_spheres_camera(image_width, image_height),
        gradient_background,
        bvh_enabled,
    )
}

fn checkered_spheres(image_width: u32, image_height: u32, bvh_enabled: bool) -> Scene {
    let mut world: Vec<ArcHittable> = Vec::new();

    let checker = Checker::new(
        SolidColour::from_rgb(0.2, 0.3, 0.1),
        SolidColour::from_rgb(0.9, 0.9, 0.9),
    );

    world.push(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Lambertian::new(Arc::clone(&checker)),
    ));

    world.push(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Lambertian::new(Arc::clone(&checker)),
    ));

    let lights = light_box(1000.0);
    for light in lights.clone() {
        world.push(Arc::clone(&light));
    }

    Scene::new_scene(
        &world,
        &lights,
        checkered_spheres_camera(image_width, image_height),
        gradient_background,
        bvh_enabled,
    )
}

fn perlin_spheres_objects() -> Vec<ArcHittable> {
    let mut world: Vec<ArcHittable> = Vec::new();

    let noise = Noise::new(4.0, 7, 10.0, 256, Z_AXIS);

    world.push(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(Arc::clone(&noise)),
    ));

    world.push(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Lambertian::new(Arc::clone(&noise)),
    ));

    world
}

fn perlin_spheres(image_width: u32, image_height: u32, bvh_enabled: bool) -> Scene {
    let mut world = perlin_spheres_objects();

    let noise = Noise::new(4.0, 7, 10.0, 256, Z_AXIS);

    world.push(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(Arc::clone(&noise)),
    ));

    world.push(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Lambertian::new(Arc::clone(&noise)),
    ));

    let lights = light_box(3000.0);
    for light in lights.clone() {
        world.push(Arc::clone(&light));
    }

    Scene::new_scene(
        &world,
        &lights,
        checkered_spheres_camera(image_width, image_height),
        gradient_background,
        bvh_enabled,
    )
}

fn earth(image_width: u32, image_height: u32, bvh_enabled: bool) -> Scene {
    let mut world: Vec<ArcHittable> = Vec::new();

    let earth_texture = Image::new("images/world.topo.bathy.200412.3x5400x2700.jpg");

    world.push(Sphere::new(
        Point3::zero(),
        2.0,
        Lambertian::new(Arc::clone(&earth_texture)),
    ));

    let lights = light_box(1000.0);
    for light in lights.clone() {
        world.push(Arc::clone(&light));
    }

    let camera = Camera::new(
        Point3::new(0.0, 0.0, 12.0),
        Point3::zero(),
        Point3::new(0.0, 1.0, 0.0),
        20.0,
        (image_width as Float) / (image_height as Float),
        0.001,
        100.0,
        0.0,
        1.0,
    );

    Scene::new_scene(&world, &lights, camera, gradient_background, bvh_enabled)
}

fn simple_light(image_width: u32, image_height: u32, bvh_enabled: bool) -> Scene {
    let mut world = perlin_spheres_objects();

    let light = DiffuseLight::new(SolidColour::from_rgb(4.0, 4.0, 4.0));
    let sphere_light = Sphere::new(Point3::new(0.0, 7.0, 0.0), 2.0, Arc::clone(&light));
    let rect_light = XYrect::new(3.0, 5.0, 1.0, 3.0, -2.0, Arc::clone(&light));

    world.push(Arc::clone(&sphere_light));
    world.push(Arc::clone(&rect_light));

    let lights = vec![Arc::clone(&sphere_light), Arc::clone(&rect_light)];

    let camera = Camera::new(
        Point3::new(26.0, 3.0, 6.0),
        Point3::new(0.0, 2.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        (image_width as Float) / (image_height as Float),
        0.0,
        10.0,
        0.0,
        1.0,
    );

    Scene::new_scene(&world, &lights, camera, black_background, bvh_enabled)
}

fn cornell_box_base<'a>() -> (HashMap<&'a str, ArcHittable>, HashMap<&'a str, ArcMaterial>) {
    let red = Lambertian::new(SolidColour::from_rgb(0.65, 0.05, 0.05));
    let white = Lambertian::new(SolidColour::from_rgb(0.73, 0.73, 0.73));
    let green = Lambertian::new(SolidColour::from_rgb(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(SolidColour::from_rgb(15.0, 15.0, 15.0));

    let left = FlipFace::new(YZrect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&green),
    ));
    let right = YZrect::new(0.0, 555.0, 0.0, 555.0, 0.0, Arc::clone(&red));
    let top_light = FlipFace::new(XZrect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        Arc::clone(&light),
    ));
    let top = FlipFace::new(XZrect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&white),
    ));
    let bottom = XZrect::new(0.0, 555.0, 0.0, 555.0, 0.0, Arc::clone(&white));
    let back = FlipFace::new(XYrect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&white),
    ));

    let mut mat = HashMap::new();
    mat.insert("red", red);
    mat.insert("white", white);
    mat.insert("green", green);
    mat.insert("light", light);

    let mut obj = HashMap::new();
    obj.insert("left", left);
    obj.insert("right", right);
    obj.insert("top_light", top_light);
    obj.insert("top", top);
    obj.insert("bottom", bottom);
    obj.insert("back", back);

    (obj, mat)
}

fn empty_cornell_box(image_width: u32, image_height: u32, bvh_enabled: bool) -> Scene {
    let (objects, _) = cornell_box_base();

    let mut world: Vec<ArcHittable> = Vec::new();
    let mut lights: Vec<ArcHittable> = Vec::new();

    for (key, object) in objects {
        world.push(Arc::clone(&object));

        if key == "top_light" {
            lights.push(Arc::clone(&object));
        }
    }

    Scene::new_scene(
        &world,
        &lights,
        cornell_box_camera(image_width, image_height),
        black_background,
        bvh_enabled,
    )
}

fn cornell_box(image_width: u32, image_height: u32, bvh_enabled: bool) -> Scene {
    let (objects, materials) = cornell_box_base();

    let white = materials
        .get("white")
        .expect("White material not found for cornell box.");

    let mut world: Vec<ArcHittable> = Vec::new();
    let mut lights: Vec<ArcHittable> = Vec::new();

    for (key, object) in objects {
        world.push(Arc::clone(&object));

        if key == "top_light" {
            lights.push(Arc::clone(&object));
        }
    }

    world.push(Translate::new(
        Rotate::new(
            XYZbox::new(
                Point3::zero(),
                Point3::new(165.0, 330.0, 165.0),
                Arc::clone(&white),
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
                Arc::clone(&white),
            ),
            Y_AXIS,
            -18.0,
        ),
        Vec3::new(130.0, 0.0, 65.0),
    ));

    Scene::new_scene(
        &world,
        &lights,
        cornell_box_camera(image_width, image_height),
        black_background,
        bvh_enabled,
    )
}

fn cornell_box_smoke_and_fog(image_width: u32, image_height: u32, bvh_enabled: bool) -> Scene {
    let (objects, materials) = cornell_box_base();

    let mut world: Vec<ArcHittable> = Vec::new();

    for (name, object) in objects {
        if name != "top_light" {
            world.push(Arc::clone(&object));
        }
    }

    let diffuse_light = DiffuseLight::new(SolidColour::from_rgb(7.0, 7.0, 7.0));
    let light = FlipFace::new(XZrect::new(
        113.0,
        443.0,
        127.0,
        432.0,
        554.0,
        Arc::clone(&diffuse_light),
    ));
    world.push(Arc::clone(&light));

    let lights = vec![Arc::clone(&light)];

    let white = materials
        .get("white")
        .expect("White material not found for cornell box.");

    let box1 = Translate::new(
        Rotate::new(
            XYZbox::new(
                Point3::zero(),
                Point3::new(165.0, 330.0, 165.0),
                Arc::clone(&white),
            ),
            Y_AXIS,
            15.0,
        ),
        Vec3::new(265.0, 0.0, 295.0),
    );

    let box2 = Translate::new(
        Rotate::new(
            XYZbox::new(
                Point3::zero(),
                Point3::new(165.0, 165.0, 165.0),
                Arc::clone(&white),
            ),
            Y_AXIS,
            -18.0,
        ),
        Vec3::new(130.0, 0.0, 65.0),
    );

    world.push(ConstantMedium::new(
        Arc::clone(&box1),
        0.01,
        SolidColour::from_rgb(0.0, 0.0, 0.0),
    ));

    world.push(ConstantMedium::new(
        Arc::clone(&box2),
        0.01,
        SolidColour::from_rgb(1.0, 1.0, 1.0),
    ));

    Scene::new_scene(
        &world,
        &lights,
        cornell_box_camera(image_width, image_height),
        black_background,
        bvh_enabled,
    )
}

fn final_next_week(image_width: u32, image_height: u32, bvh_enabled: bool) -> Scene {
    let mut world: Vec<ArcHittable> = Vec::new();

    let ground = Lambertian::new(SolidColour::from_rgb(0.48, 0.83, 0.53));

    let mut boxes1: Vec<ArcHittable> = Vec::new();

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + (i as Float) * w;
            let z0 = -1000.0 + (j as Float) * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = Random::sample_in_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.push(XYZbox::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                Arc::clone(&ground),
            ));
        }
    }

    world.push(BVH::new(&mut boxes1, 0.0, 1.0));

    let light = DiffuseLight::new(SolidColour::from_rgb(7.0, 7.0, 7.0));
    let rect_light = FlipFace::new(XZrect::new(
        123.0,
        423.0,
        147.0,
        412.0,
        554.0,
        Arc::clone(&light),
    ));
    world.push(Arc::clone(&rect_light));

    let lights = vec![Arc::clone(&rect_light)];

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Lambertian::new(SolidColour::from_rgb(0.7, 0.3, 0.1));
    world.push(MovingSphere::new(
        center1,
        center2,
        0.0,
        1.0,
        50.0,
        moving_sphere_material,
    ));

    world.push(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Dielectric::new(1.5),
    ));

    world.push(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Metal::new(SolidColour::from_rgb(0.8, 0.8, 0.9), 1.0),
    ));

    let boundary = Sphere::new(Point3::new(360.0, 150.0, 145.0), 70.0, Dielectric::new(1.5));
    world.push(Arc::clone(&boundary));

    world.push(ConstantMedium::new(
        Arc::clone(&boundary),
        0.2,
        SolidColour::from_rgb(0.2, 0.4, 0.9),
    ));

    let boundary = Sphere::new(Point3::new(0.0, 0.0, 0.0), 5000.0, Dielectric::new(1.5));
    world.push(ConstantMedium::new(
        Arc::clone(&boundary),
        0.0001,
        SolidColour::from_rgb(1.0, 1.0, 1.0),
    ));

    let earth_texture = Image::new("images/world.topo.bathy.200412.3x5400x2700.jpg");
    let emat = Lambertian::new(earth_texture);
    world.push(Sphere::new(Point3::new(400.0, 200.0, 400.0), 100.0, emat));

    let pertext = Noise::new(0.1, 7, 10.0, 256, X_AXIS);
    world.push(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Lambertian::new(pertext),
    ));

    let mut boxes2: Vec<ArcHittable> = Vec::new();
    let white = Lambertian::new(SolidColour::from_rgb(0.73, 0.73, 0.73));
    let ns = 1000;
    for _j in 0..ns {
        boxes2.push(Sphere::new(
            Random::vec3_in_range(0.0, 165.0),
            10.0,
            Arc::clone(&white),
        ));
    }

    world.push(Translate::new(
        Rotate::new(BVH::new(&mut boxes2, 0.0, 1.0), Y_AXIS, 15.0),
        Vec3::new(-100.0, 270.0, 395.0),
    ));

    let camera = Camera::new(
        Point3::new(478.0, 278.0, -600.0),
        Point3::new(278.0, 278.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        40.0,
        (image_width as Float) / (image_height as Float),
        0.0,
        10.0,
        0.0,
        1.0,
    );

    Scene::new_scene(&world, &lights, camera, black_background, bvh_enabled)
}

fn rotate_spheres(image_width: u32, image_height: u32, bvh_enabled: bool) -> Scene {
    let mut world: Vec<ArcHittable> = Vec::new();

    let red = Lambertian::new(SolidColour::from_rgb(0.8, 0.2, 0.2));
    let green = Lambertian::new(SolidColour::from_rgb(0.2, 0.8, 0.2));
    let blue = Lambertian::new(SolidColour::from_rgb(0.2, 0.2, 0.8));
    let metal = Metal::new(SolidColour::from_rgb(0.8, 0.8, 0.8), 0.25);

    let lights = light_box(1000.0);
    for light in lights.clone() {
        world.push(Arc::clone(&light));
    }

    let max_angle = 90.0;
    let max_radius = 0.1;
    let n = 8;

    for angle in (0..n).map(|i| i as Float * max_angle / (n as Float)) {
        let f = 1.0 - angle / max_angle;

        world.push(Rotate::new(
            Sphere::new(Point3::new(f, 0.0, 0.0), max_radius * f, Arc::clone(&red)),
            Z_AXIS,
            angle,
        ));

        world.push(Rotate::new(
            Sphere::new(Point3::new(0.0, 0.0, f), max_radius * f, Arc::clone(&green)),
            Y_AXIS,
            angle,
        ));

        world.push(Rotate::new(
            Sphere::new(Point3::new(0.0, f, 0.0), max_radius * f, Arc::clone(&blue)),
            X_AXIS,
            angle,
        ));
    }

    world.push(Sphere::new(
        Point3::new(2.0, 2.0, 2.0),
        2.0,
        Arc::clone(&metal),
    ));

    let camera = Camera::new(
        Point3::new(-1.4, -1.4, -1.4),
        Point3::zero(),
        Vec3::new(0.0, 1.0, 0.0),
        40.0,
        (image_width as Float) / (image_height as Float),
        0.0,
        10.0,
        0.0,
        1.0,
    );

    Scene::new_scene(&world, &lights, camera, black_background, bvh_enabled)
}

fn specular_reflections(image_width: u32, image_height: u32, bvh_enabled: bool) -> Scene {
    let (objects, materials) = cornell_box_base();

    let white = materials
        .get("white")
        .expect("White material not found for cornell box.");

    let aluminum = Metal::new(SolidColour::from_rgb(0.8, 0.85, 0.88), 0.0);

    let mut world: Vec<ArcHittable> = Vec::new();
    let mut lights: Vec<ArcHittable> = Vec::new();

    for (key, object) in objects {
        world.push(Arc::clone(&object));

        if key == "top_light" {
            lights.push(Arc::clone(&object));
        }
    }

    world.push(Translate::new(
        Rotate::new(
            XYZbox::new(
                Point3::zero(),
                Point3::new(165.0, 330.0, 165.0),
                Arc::clone(&aluminum),
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
                Arc::clone(&white),
            ),
            Y_AXIS,
            -18.0,
        ),
        Vec3::new(130.0, 0.0, 65.0),
    ));

    Scene::new_scene(
        &world,
        &lights,
        cornell_box_camera(image_width, image_height),
        black_background,
        bvh_enabled,
    )
}

fn final_rest_of_your_life(image_width: u32, image_height: u32, bvh_enabled: bool) -> Scene {
    let (objects, materials) = cornell_box_base();

    let white = materials
        .get("white")
        .expect("White material not found for cornell box.");

    let mut world: Vec<ArcHittable> = Vec::new();
    let mut lights: Vec<ArcHittable> = Vec::new();

    for (key, object) in objects {
        world.push(Arc::clone(&object));

        if key == "top_light" {
            lights.push(Arc::clone(&object));
        }
    }

    world.push(Translate::new(
        Rotate::new(
            XYZbox::new(
                Point3::zero(),
                Point3::new(165.0, 330.0, 165.0),
                Arc::clone(&white),
            ),
            Y_AXIS,
            15.0,
        ),
        Vec3::new(265.0, 0.0, 295.0),
    ));

    let glass_mat = Dielectric::new(1.5);
    let glass = Sphere::new(Point3::new(190.0, 90.0, 190.0), 90.0, glass_mat);

    world.push(Arc::clone(&glass));
    lights.push(Arc::clone(&glass));

    Scene::new_scene(
        &world,
        &lights,
        cornell_box_camera(image_width, image_height),
        black_background,
        bvh_enabled,
    )
}
