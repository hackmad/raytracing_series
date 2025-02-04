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
use threadpool::*;
use tiles::*;

use clap::Parser;
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

    // Create a thread pool for rendering tiles in parallel.
    let pool = Arc::new(Mutex::new(ThreadPool::build(CONFIG.threads()).unwrap()));

    // Track remaining tiles. It will be used to shutdown the thread pool.
    let remaining_tiles = Arc::new(Mutex::new(CONFIG.n_tiles()));

    // Wait for render to complete, then save image and shutdown pool.
    let progress_thread = {
        let remaining_tiles = Arc::clone(&remaining_tiles);
        thread::spawn(|| progress(remaining_tiles))
    };

    // Start a separate thread that will queue all tiles.
    let render_thread = {
        let pool = Arc::clone(&pool);
        thread::spawn(|| render(pool, remaining_tiles))
    };

    if CONFIG.gui {
        // Run the event loop for the GUI. This will run in the main thread.
        run_event_loop().map_err(|err| format!("{}", err))
    } else {
        // Wait for remaining threads to complete.
        render_thread.join().map_err(|e| format!("{:?}", e))?;
        progress_thread.join().map_err(|e| format!("{:?}", e))
    }
}

/// Displays the progress of the render. When complete it saves the image and shuts down the thread pool.
///
/// * `remaining_tiles` - Number of tiles remaining.
fn progress(remaining_tiles: Arc<Mutex<usize>>) {
    loop {
        let remaining_tiles = *remaining_tiles.lock().unwrap();

        let progress = (CONFIG.n_tiles() - remaining_tiles) as f32 / CONFIG.n_tiles() as f32;
        eprint!("\rProgress {:.2}%    ", 100_f32 * progress);

        if remaining_tiles == 0 {
            break;
        }

        thread::sleep(Duration::from_secs(2));
    }
}
