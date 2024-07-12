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
    // load the program configuration.
    let config = AppConfig::parse();

    // configure number of threads.
    rayon::ThreadPoolBuilder::new()
        .num_threads(config.threads())
        .build_global()
        .unwrap();

    // seed the random number generator.
    if let Some(seed) = config.seed {
        Random::seed(seed);
    };

    // allocate an RGB image.
    let image_mutex = Mutex::new(image::RgbImage::new(
        config.image_width,
        config.image_height,
    ));

    // calculate tiles in x and y direction.
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

    // create pixels to be used by the rayon worker groups so we don't reallocate per tile.
    (0..n_tiles).into_par_iter().for_each_init(
        || image::RgbImage::new(config.tile_size as u32, config.tile_size as u32),
        |pixels, tile_idx| {
            // calculate the tile bounds.
            let tile_bounds = get_tile_bounds(
                tile_idx,
                n_tiles_x,
                config.tile_size,
                config.image_width,
                config.image_height,
            );

            // render whole tile and then copy to destination.
            render_tile(&renderer, &tile_bounds, pixels);
            copy_tile(&image_mutex, &tile_bounds, pixels);

            // update progress.
            let mut data = current_progress.lock().unwrap();
            *data += percent_step;
            eprint!("                 \rProgress: {:>6.2}%", *data);
        },
    );

    // write the output file.
    eprint!("                 \rWriting output file...");
    write_image(&image_mutex, &config.output_path);

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

/// Copy a tile to to the image destination.
///
/// * `image_mutex` - The mutex holding the image.
/// * `output_path` - Path to the image.
fn write_image(image_mutex: &Mutex<image::RgbImage>, output_path: &str) {
    let img = image_mutex
        .lock()
        .expect("Unbale to lock image buffer for writing");

    // flip image first because it will be upside down.
    image::imageops::flip_vertical(&*img)
        .save(output_path)
        .expect("Error writing output file");
}
