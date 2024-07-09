//! # Recursive ray tracer

extern crate clap;
extern crate image;
extern crate rand;
extern crate rand_chacha;
extern crate rayon;

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
use clap::Parser;
use common::*;
use object::*;
use rayon::prelude::*;
use scene::*;
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// Entry point for the recursive raytracer.
fn main() {
    // Load the program configuration.
    let config = AppConfig::parse();

    // Configure number of threads.
    rayon::ThreadPoolBuilder::new()
        .num_threads(config.threads())
        .build_global()
        .unwrap();

    // Create a random number generator.
    let rng = match config.seed {
        Some(seed) => new_seeded_rng(seed),
        None => new_thread_rng(),
    };

    // Create the scene.
    let scene = Scene::new(
        config.scenery,
        config.image_width,
        config.image_height,
        config.bvh_enabled,
        Arc::clone(&rng),
    );

    // Allocate an image buffer.
    let imgbuf = Mutex::new(image::ImageBuffer::new(
        config.image_width,
        config.image_height,
    ));

    // Tracking progress.
    let percent_step = 100.0 / (config.image_height as Float);
    let current_progress = Mutex::new(0.0);
    let start = Instant::now();

    eprint!("Progress: 0.0%\r");

    (0..config.image_height).into_par_iter().for_each(|j| {
        // Process an entire scanline in one thread.
        for i in 0..config.image_width {
            let rgb = trace_ray(i, j, &config, &scene, Arc::clone(&rng)).to_rgb();
            imgbuf
                .lock()
                .expect("Unable to lock image buffer")
                .put_pixel(i, config.image_height - 1 - j, image::Rgb(rgb));
        }

        // Update progress.
        let mut data = current_progress.lock().unwrap();
        *data += percent_step;
        eprint!("                 \rProgress: {:>6.2}%", *data);
    });

    // Write the output file.
    eprint!("                 \rWriting output file...");
    imgbuf
        .lock()
        .expect("Unbale to lock image buffer for writing")
        .save(&config.output_path)
        .expect("Error writing output file");

    // Display stats.
    eprint!("                      \r");
    let seconds = start.elapsed().as_secs_f32();
    if seconds < 60.0 {
        eprintln!("Done: {:.2} seconds", seconds);
    } else if seconds < 3600.0 {
        eprintln!("Done: {:.2} minutes", seconds / 60.0);
    } else {
        eprintln!("Done: {:.2} hours", seconds / 3600.0);
    }
}

/// Trace a ray through the scene return accumulated colour. The function will
/// generate multiple samples per pixel.
///
/// * `i` - Pixel x-coordinate.
/// * `j` - Pixel y-coordinate.
/// * `config` - Program configuration.
/// * `scene` - The scene.
/// * `rng` - The random number generator.
fn trace_ray(i: u32, j: u32, config: &AppConfig, scene: &Scene, rng: ArcRandomizer) -> Colour {
    let x = i as Float;
    let y = j as Float;

    let mut colour = Colour::zero();

    for _s in 0..config.samples_per_pixel {
        let u = (x + rng.clone().float()) / (config.image_width as Float);
        let v = (y + rng.clone().float()) / (config.image_height as Float);

        let ray = scene.camera.get_ray(u, v);
        colour += ray_colour(&ray, scene.background, &scene.world, config.max_depth);
    }

    colour.to_colour_from_sample(config.samples_per_pixel)
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
                let colour = ray_colour(&sr.scattered, background, world, depth - 1);
                emission + sr.attenuation * colour
            } else {
                emission
            }
        }

        _ => background(ray),
    }
}
