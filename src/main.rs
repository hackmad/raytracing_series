//! # Recursive ray tracer

mod algebra;
mod app;
mod app_config;
mod background;
mod camera;
mod common;
mod material;
mod object;
mod renderer;
mod scene;
mod texture;
mod threadpool;
mod tiles;

use app::*;
use app_config::*;
use common::*;
use renderer::*;
use scene::*;
use tao::window::Window;
use threadpool::*;
use tiles::*;

use clap::Parser;
use std::cell::RefCell;
use std::sync::{Arc, LazyLock, Mutex};
use std::thread;
use std::time::Duration;

static CONFIG: LazyLock<AppConfig> = LazyLock::new(|| AppConfig::parse());

/// Entry point for the recursive raytracer.
fn main() -> Result<(), String> {
    // seed the random number generator.
    if let Some(seed) = CONFIG.seed {
        Random::seed(seed);
    };

    eprintln!("Running with {} threads", CONFIG.threads());

    // Allocate an image buffer for rendering.
    let image = Arc::new(Mutex::new(image::RgbaImage::new(
        CONFIG.image_width,
        CONFIG.image_height,
    )));

    // Create a thread pool for rendering tiles in parallel.
    let pool = Arc::new(Mutex::new(ThreadPool::build(CONFIG.threads()).unwrap()));

    // Track remaining tiles. It will be used to shutdown the thread pool.
    let remaining_tiles = Arc::new(Mutex::new(CONFIG.n_tiles()));

    // Create the GUI application if needed.
    let app = if CONFIG.gui {
        Some(App::build(Arc::clone(&pool), Arc::clone(&image))?)
    } else {
        None
    };

    // Start a separate thread that will queue all tiles.
    let render_thread = {
        let pool = Arc::clone(&pool);
        let image = Arc::clone(&image);
        let remaining_tiles = Arc::clone(&remaining_tiles);
        let window = app.as_ref().map(|app| Arc::clone(&app.window));
        thread::spawn(|| render(pool, image, remaining_tiles, window))
    };

    // Wait for render to complete, then save image and shutdown pool.
    let progress_thread = {
        let pool = Arc::clone(&pool);
        let image = Arc::clone(&image);
        let remaining_tiles = Arc::clone(&remaining_tiles);
        thread::spawn(|| progress(pool, image, remaining_tiles))
    };

    if CONFIG.gui {
        // Run the event loop for the GUI. This will run in the main thread.
        app.unwrap().run()
    } else {
        // Wait for remaining threads to complete.
        render_thread.join().map_err(|e| format!("{:?}", e))?;
        progress_thread.join().map_err(|e| format!("{:?}", e))
    }
}

/// Render the scene in parallel using worker threads.
///
/// * `pool`            - Thread pool.
/// * `image`           - Image buffer to render.
/// * `remaining_tiles` - Number of tiles remaining.
/// * `window`          - Optional window to redraw image to.
fn render(
    pool: Arc<Mutex<ThreadPool>>,
    image: Arc<Mutex<image::RgbaImage>>,
    remaining_tiles: Arc<Mutex<usize>>,
    window: Option<Arc<Window>>,
) {
    // Setup rendering algorithm.
    let renderer = Arc::new(RecursiveTracer {
        config: CONFIG.clone(),
        scene: Scene::new(
            CONFIG.scenery,
            CONFIG.image_width,
            CONFIG.image_height,
            CONFIG.bvh_enabled,
        ),
    });

    // Queue up the tiles to render.
    for tile_idx in 0..CONFIG.n_tiles() {
        // Clone the `Arc`s for the worker thread.
        let renderer = Arc::clone(&renderer);
        let image = Arc::clone(&image);
        let remaining_tiles = Arc::clone(&remaining_tiles);
        let window = window.clone();

        pool.lock().unwrap().execute(move || {
            thread_local! {
                // Allocate pixels for rendering a tile per thread so we don't allocate for each tile.
                pub static TILE_PIXELS: RefCell<image::RgbaImage> = {
                    eprintln!("\rAllocating tile pixels for {:?}", thread::current().id());
                    RefCell::new(image::RgbaImage::new(CONFIG.tile_size as u32, CONFIG.tile_size as u32))
                };
            }

            TILE_PIXELS.with_borrow_mut(|tile_pixels| {
                // Calculate the tile bounds.
                let tile_bounds = get_tile_bounds(tile_idx);

                // Render whole tile and then copy to destination.
                render_tile(renderer, &tile_bounds, tile_pixels);
                copy_tile(Arc::clone(&image), &tile_bounds, tile_pixels);
            });

            // Update remaining tiles.
            *remaining_tiles.lock().unwrap() -= 1;

            // Redraw window.
            window.map(|w| w.request_redraw());
        });
    }

    eprintln!("\nQueued up all tiles to render.");
}

/// Write the image to disk.
///
/// * `image` - Image to save to file.
fn write_image(image: Arc<Mutex<image::RgbaImage>>) {
    eprintln!("Saving output image to {}", CONFIG.output_path);

    if let Err(_) = image
        .lock()
        .map(|img| image::imageops::flip_vertical(&*img).save(&CONFIG.output_path))
    {
        eprintln!("Error writing output image");
    }
}

/// Displays the progress of the render. When complete it saves the image and shuts down the thread pool.
///
/// * `pool`            - Thread pool.
/// * `image`           - Image buffer to render.
/// * `remaining_tiles` - Number of tiles remaining.
fn progress(
    pool: Arc<Mutex<ThreadPool>>,
    image: Arc<Mutex<image::RgbaImage>>,
    remaining_tiles: Arc<Mutex<usize>>,
) {
    loop {
        let remaining_tiles = *remaining_tiles.lock().unwrap();

        let progress = (CONFIG.n_tiles() - remaining_tiles) as f32 / CONFIG.n_tiles() as f32;
        eprint!("\rProgress {:.2}%    ", 100_f32 * progress);

        if remaining_tiles == 0 {
            eprintln!();

            write_image(image);
            pool.lock().unwrap().shutdown();

            break;
        }

        thread::sleep(Duration::from_secs(2));
    }
}
