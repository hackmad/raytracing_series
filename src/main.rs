//! # Recursive ray tracer

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
mod threadpool;
mod tiles;

use app_config::*;
use common::*;
use renderer::*;
use scene::*;
use threadpool::*;
use tiles::*;

use clap::Parser;
use pixels::{Pixels, SurfaceTexture};
use std::cell::RefCell;
use std::sync::{Arc, LazyLock, Mutex};
use std::thread;
use std::time::Duration;
use tao::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::Key,
    window::{Window, WindowBuilder},
};

static CONFIG: LazyLock<AppConfig> = LazyLock::new(|| AppConfig::parse());

/// Entry point for the recursive raytracer.
fn main() -> Result<(), pixels::Error> {
    // seed the random number generator.
    if let Some(seed) = CONFIG.seed {
        Random::seed(seed);
    };

    eprintln!("Running with {} threads", CONFIG.threads());

    // Initialize logger for tao.
    env_logger::init();

    // Create a new event loop for the application.
    let event_loop = EventLoop::new();

    // Create a new window.
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Raytracing Series")
            .with_inner_size(LogicalSize::new(CONFIG.image_width, CONFIG.image_height))
            .with_resizable(false)
            .build(&event_loop)
            .unwrap(),
    );
    let inner_size = window.inner_size();

    // Create a surface texture that uses the logical inner size to render to the entire window's inner dimensions.
    // Then create pixel frame buffer that matches rendered image dimensions that will be used to display it in the
    // window.
    let pixels = {
        let surface_texture = SurfaceTexture::new(inner_size.width, inner_size.height, &window);

        Arc::new(Mutex::new(Pixels::new(
            CONFIG.image_width,
            CONFIG.image_height,
            surface_texture,
        )?))
    };

    // Allocate an image buffer for rendering.
    let image = Arc::new(Mutex::new(image::RgbaImage::new(
        CONFIG.image_width,
        CONFIG.image_height,
    )));

    // Create a thread pool for rendering tiles in parallel.
    let pool = Arc::new(Mutex::new(ThreadPool::build(CONFIG.threads()).unwrap()));

    // Track remaining tiles. It will be used to shutdown the thread pool.
    let remaining_tiles = Arc::new(Mutex::new(CONFIG.n_tiles()));

    // Start a separate thread that will queue all tiles. This way we can run the event loop in main thread.
    {
        let pool = Arc::clone(&pool);
        let image = Arc::clone(&image);
        let window = Arc::clone(&window);
        let remaining_tiles = Arc::clone(&remaining_tiles);
        thread::spawn(|| render(pool, image, window, remaining_tiles));
    }

    // Run the event loop.
    event_loop.run(move |event, _, control_flow| {
        //println!("{:?}", event);
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                // When window is closed or destroyed or Escape key is pressed, stop rendering.
                WindowEvent::CloseRequested
                | WindowEvent::Destroyed
                | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            logical_key: Key::Escape,
                            state: ElementState::Released,
                            ..
                        },
                    ..
                } => {
                    eprintln!("Exiting application.");
                    pool.lock().unwrap().shutdown();
                    *control_flow = ControlFlow::Exit;
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                // Draw the pixel frame buffer to the window. If there are errors show the error and stop rendering.
                if let Err(err) = render_to_window(Arc::clone(&image), Arc::clone(&pixels)) {
                    println!("pixels.render() failed with error.\n{}", err);
                    pool.lock().unwrap().shutdown();
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => (),
        }
    })
}

/// Render the scene in parallel using worker threads.
///
/// * `pool`            - Thread pool.
/// * `image`           - Image buffer to render.
/// * `window`          - Window.
/// * `remaining_tiles` - Number of tiles remaining.
fn render(
    pool: Arc<Mutex<ThreadPool>>,
    image: Arc<Mutex<image::RgbaImage>>,
    window: Arc<Window>,
    remaining_tiles: Arc<Mutex<usize>>,
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
        let window = Arc::clone(&window);
        let remaining_tiles = Arc::clone(&remaining_tiles);

        pool.lock().unwrap().execute(move || {
            thread_local! {
                // Allocate pixels for rendering a tile per thread so we don't allocate for each tile.
                pub static TILE_PIXELS: RefCell<image::RgbaImage> = {
                    eprintln!("Allocating tile pixels for {:?}", thread::current().id());
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

            // Request a redraw of the window.
            window.request_redraw();
        });
    }

    println!("Queued up all tiles to render.");

    // Wait for render to complete, then save image and shutdown pool.
    loop {
        if *remaining_tiles.lock().unwrap() == 0 {
            write_image(image);
            pool.lock().unwrap().shutdown();
            break;
        }

        thread::sleep(Duration::from_secs(1));
    }
}

/// Copy the rendered image to pixel frame buffer and render to window.
///
/// * `image`  - Rendered image.
/// * `pixels` - Pixel frame buffer to render to window.
fn render_to_window(
    image: Arc<Mutex<image::RgbaImage>>,
    pixels: Arc<Mutex<Pixels>>,
) -> Result<(), &'static str> {
    let image = image.lock().map_err(|_| "Unable to lock image buffer")?;

    let flipped_image = image::imageops::flip_vertical(&*image);

    let mut pixels = pixels
        .lock()
        .map_err(|_| "Unable to lock pixel frame buffer")?;

    let frame = pixels.frame_mut();
    frame.copy_from_slice(flipped_image.to_vec().as_slice());

    pixels
        .render()
        .map_err(|_| "Unable to render pixel buffer to window")?;

    Ok(())
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
