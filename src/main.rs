//! # Recursive ray tracer

extern crate clap;
extern crate image;
extern crate rand;
extern crate rand_chacha;

mod algebra;
mod app_config;
mod background;
mod camera;
mod common;
mod material;
mod object;
mod scene;
mod texture;

use algebra::*;
use app_config::*;
use background::*;
use common::*;
use object::*;
use scene::*;
use std::sync::Arc;
use std::time::Instant;

/// Entry point for the recursive raytracer.
fn main() {
    let config = AppConfig::load();

    let rng = match config.seed {
        Some(seed) => new_seeded_rng(seed),
        None => new_thread_rng(),
    };

    let scene = Scene::new(
        config.scenery,
        config.image_width,
        config.image_height,
        config.bvh_enabled,
        Arc::clone(&rng),
    );

    let image_width = config.image_width as Float;
    let image_height = config.image_height as Float;
    let percent_step = 100.0 / image_height;

    let start = Instant::now();

    println!("P3\n{} {}\n255", config.image_width, config.image_height);

    for j in (0..config.image_height).rev() {
        let y = j as Float;

        let progress = percent_step * (image_height - y);
        eprint!("Progress: {:>6.2}%\r", progress);

        for i in 0..config.image_width {
            let x = i as Float;

            let mut colour = Colour::zero();

            for _s in 0..config.samples_per_pixel {
                let u = (x + rng.clone().float()) / image_width;
                let v = (y + rng.clone().float()) / image_height;

                let ray = scene.camera.get_ray(u, v);
                colour += ray_colour(&ray, scene.background, &scene.world, config.max_depth);
            }

            let c = colour
                .to_colour_from_sample(config.samples_per_pixel)
                .to_ppm();
            println!("{}", c);
        }
    }

    let seconds = start.elapsed().as_secs_f32();
    if seconds < 60.0 {
        eprintln!("Done: {:.2} seconds", seconds);
    } else if seconds < 3600.0 {
        eprintln!("Done: {:.2} minutes", seconds / 60.0);
    } else {
        eprintln!("Done: {:.2} hours", seconds / 3600.0);
    }
}

/// Recursively traces a ray through the scene and generates the colour seen
/// at the image plane.
///
/// * `ray` - The ray.
/// * `background` - The background.
/// * `world` - The list of geometric objects.
/// * `depth` - Maximum depth for recursion.
fn ray_colour(ray: &Ray, background: BackgroundFn, world: &ArcHittable, depth: u32) -> Colour {
    // Terminate the recursion if maximum depth is reached.
    if depth <= 0 {
        return Colour::zero();
    }

    // Note the 0.001 is used to avoid starting the ray inside the surface
    // caused due to floating point approximation errors generated by the
    // intersection routine.
    match world.hit(&ray, 0.001, INFINITY) {
        Some(rec) => {
            let emission = rec.material.emission(ray, &rec);

            // If material did not absorb the ray and scattered it, continue
            // tracing the new ray.
            if let Some(sr) = rec.material.scatter(ray, &rec) {
                emission + ray_colour(&sr.scattered, background, world, depth - 1) * sr.attenuation
            } else {
                emission
            }
        }

        _ => background(ray),
    }
}
