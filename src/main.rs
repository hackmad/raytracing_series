//! # Recursive ray tracer

extern crate clap;
extern crate image;
extern crate num_cpus;
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
mod renderer;
mod scene;
mod texture;

use app_config::*;
use common::*;
use rayon::prelude::*;
use renderer::*;
use scene::*;
use std::sync::Mutex;
use std::time::Instant;

/// Entry point for the recursive raytracer.
fn main() {
    // Load the program configuration.
    let config = AppConfig::load();

    // Configure number of threads.
    rayon::ThreadPoolBuilder::new()
        .num_threads(config.num_threads)
        .build_global()
        .unwrap();

    if let Some(seed) = config.seed {
        Random::seed(seed);
    };

    // Setup rendering algorithm
    let renderer = RecursiveTracer {
        config: config.clone(),
        scene: Scene::new(
            config.scenery,
            config.image_width,
            config.image_height,
            config.bvh_enabled,
        ),
    };

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
            let rgb = renderer.trace_ray(i, j).to_rgb();
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
    eprint!("                                         \r");
    let seconds = start.elapsed().as_secs_f32();
    if seconds < 60.0 {
        eprintln!("Done: {:.2} seconds", seconds);
    } else if seconds < 3600.0 {
        eprintln!("Done: {:.2} minutes", seconds / 60.0);
    } else {
        eprintln!("Done: {:.2} hours", seconds / 3600.0);
    }
}
