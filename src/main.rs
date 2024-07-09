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
mod renderer;
mod scene;
mod texture;
mod tiles;

use app_config::*;
use clap::Parser;
use common::*;
use rayon::prelude::*;
use renderer::*;
use scene::*;
use std::sync::Mutex;
use std::time::Instant;
use tiles::*;

/// Entry point for the recursive raytracer.
fn main() {
    // Load the program configuration.
    let config = AppConfig::parse();

    // Configure number of threads.
    rayon::ThreadPoolBuilder::new()
        .num_threads(config.threads())
        .build_global()
        .unwrap();

    if let Some(seed) = config.seed {
        Random::seed(seed);
    };

    // Allocate an image buffer.
    let imgbuf = Mutex::new(image::ImageBuffer::new(
        config.image_width,
        config.image_height,
    ));

    // Calculate tiles in x and y direction.
    let n_tiles_x = get_tile_count(config.tile_size, config.image_width);
    let n_tiles_y = get_tile_count(config.tile_size, config.image_height);
    let n_tiles = n_tiles_x * n_tiles_y;

    // setup rendering algorithm.
    let renderer = RecursiveTracer {
        config: config.clone(),
        scene: Scene::new(
            config.scenery,
            config.image_width,
            config.image_height,
            config.bvh_enabled,
        ),
    };

    // tracking progress.
    let percent_step = 100.0 / (n_tiles as Float);
    let current_progress = Mutex::new(0.0);
    let start = Instant::now();

    eprint!("Progress: 0.0%\r");

    (0..n_tiles).into_par_iter().for_each(|tile_idx| {
        // Process a tile in one thread.
        let tile_bounds = get_tile_bounds(
            tile_idx,
            n_tiles_x,
            config.tile_size,
            config.image_width,
            config.image_height,
        );

        for j in tile_bounds.y_min..=tile_bounds.y_max {
            for i in tile_bounds.x_min..=tile_bounds.x_max {
                let rgb = renderer.trace_ray(i, j).to_rgb();
                imgbuf
                    .lock()
                    .expect("Unable to lock image buffer")
                    .put_pixel(i, config.image_height - 1 - j, image::Rgb(rgb));
            }
        }

        // Update progress.
        let mut data = current_progress.lock().unwrap();
        *data += percent_step;
        eprint!("                 \rProgress: {:>6.2}%", *data);
    });

    // write the output file.
    eprint!("                 \rWriting output file...");
    imgbuf
        .lock()
        .expect("Unbale to lock image buffer for writing")
        .save(&config.output_path)
        .expect("Error writing output file");

    // display stats.
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
