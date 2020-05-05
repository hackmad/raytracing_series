//! # Recursive ray tracer

extern crate clap;
extern crate rand;

mod algebra;
mod camera;
mod common;
mod material;
mod object;
mod scene;

use algebra::*;
use clap::{App, Arg};
use common::*;
use object::*;
use scene::*;

/// Program configuration
struct Config {
    /// Image height
    image_width: u32,

    /// Image width
    image_height: u32,

    /// Samples per pixels for antialiasing
    samples_per_pixel: u32,

    /// Max recursion depth
    max_depth: u32,
}

/// Entry point for the recursive raytracer.
fn main() {
    let c = app_config();
    let s = Scene::new_random_scene(c.image_width, c.image_height);

    println!("P3\n{} {}\n255", c.image_width, c.image_height);

    for j in (0..c.image_height).rev() {
        eprint!("Scan lines remaining: {}          \r", j);

        for i in 0..c.image_width {
            let mut colour = Vec3::zero().as_colour();

            for _s in 0..c.samples_per_pixel {
                let u = ((i as Float) + random()) / (c.image_width as Float);
                let v = ((j as Float) + random()) / (c.image_height as Float);

                let r = s.camera.get_ray(u, v);
                colour += ray_colour(&r, &s.world, c.max_depth);
            }

            let c = colour.to_colour_from_sample(c.samples_per_pixel).to_ppm();
            println!("{}", c);
        }
    }
    eprintln!("\nDone!");
}

/// Recursively traces a ray through the scene and generates the colour seen
/// at the image plane.
///
/// * `ray` - The ray.
/// * `world` - The list of geometric objects.
/// * `depth` - Maximum depth for recursion.
fn ray_colour(ray: &Ray, world: &HittableList, depth: u32) -> Colour {
    // Terminate the recursion if maximum depth is reached.
    if depth <= 0 {
        return Vec3::zero().as_colour();
    }

    // Note the 0.001 is used to avoid starting the ray inside the surface
    // caused due to floating point approximation errors generated by the
    // intersection routine.
    match world.hit(&ray, 0.001, INFINITY) {
        Some(rec) => {
            // If material did not absorb the ray and scattered it, continue
            // tracing the new ray.
            if let Some(sr) = rec.material.clone().scatter(ray, &rec) {
                ray_colour(&sr.scattered, world, depth - 1) * sr.attenuation
            } else {
                background_colour(ray)
            }
        }

        _ => background_colour(ray),
    }
}

/// Generate a gradient colour for the background.
///
/// * `ray` - The ray.
fn background_colour(ray: &Ray) -> Colour {
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t).as_colour()
}

fn app_config() -> Config {
    let matches = App::new("Raytracing in One Weekend")
        .version("0.0.1")
        .arg(
            Arg::with_name("image_width")
                .short('w')
                .long("image_width")
                .value_name("WIDTH")
                .takes_value(true)
                .default_value("200")
                .about("image width in pixels"),
        )
        .arg(
            Arg::with_name("image_height")
                .short('h')
                .long("image_height")
                .value_name("HEIGHT")
                .takes_value(true)
                .default_value("100")
                .about("image height in pixels"),
        )
        .arg(
            Arg::with_name("samples_per_pixel")
                .short('s')
                .long("samples_per_pixel")
                .value_name("SAMPLES")
                .takes_value(true)
                .default_value("100")
                .about("number of samples per pixel for antialiasing"),
        )
        .arg(
            Arg::with_name("max_depth")
                .short('d')
                .long("max_depth")
                .value_name("DEPTH")
                .takes_value(true)
                .default_value("50")
                .about("maximum depth of recursion"),
        )
        .get_matches();

    let image_width = match matches.value_of("image_width") {
        Some(s) => s.parse::<u32>().unwrap(),
        _ => panic!("Invalid image width"),
    };

    let image_height = match matches.value_of("image_height") {
        Some(s) => s.parse::<u32>().unwrap(),
        _ => panic!("Invalid image height"),
    };

    let samples_per_pixel = match matches.value_of("samples_per_pixel") {
        Some(s) => s.parse::<u32>().unwrap(),
        _ => panic!("Invalid samples per pixel"),
    };

    let max_depth = match matches.value_of("max_depth") {
        Some(s) => s.parse::<u32>().unwrap(),
        _ => panic!("Invalid max depth"),
    };

    Config {
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
    }
}
