//! The application

use crate::{
    renderer::RecursiveTracer,
    scene::Scene,
    tiles::{copy_tile, get_tile_bounds, render_tile, TileBounds}, 
    ThreadPool,
    CONFIG,
};

use std::{
    cell::RefCell, 
    sync::{Arc, Mutex, OnceLock},
    thread,
    time::Duration,
};

use pixels::{Pixels, SurfaceTexture};

use winit::{
    application::ApplicationHandler,
    dpi::{LogicalSize, PhysicalSize},
    error::EventLoopError,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    keyboard::{Key, NamedKey},
    window::Window,
};

/// User events for the render loop.
#[derive(Debug, Clone, PartialEq)]
enum UserEvent{
    // Render the image.
    RenderTile { 
        tile_pixels: Vec<u8>,
        tile_bounds: TileBounds,
    },

    // Save the image
    SaveImage,
}

/// This proxy will be used to trigger custom events from the render loop to the winit application window.
static EVENT_LOOP_PROXY: OnceLock<EventLoopProxy<UserEvent>> = OnceLock::new();

/// The application.
pub struct App {
    /// The preview window.
    window: Option<Arc<Window>>,

    /// The preview image pixels.
    pixels: Option<Pixels<'static>>,

    /// The preview image pixel dimensions.
    pixel_size: LogicalSize<u32>,

    /// The inner dimensions of the preview window.
    window_inner_size: PhysicalSize<u32>,
}

impl App {
    /// Render the preview image.
    fn render(&self) -> Result<(), String> {
        match self.pixels.as_ref() {
            Some(pixels) => pixels.render().map_err(|err| format!("Unable to render pixel buffer to window. {}", err)),
            None => Ok(()),
        }
    }

    /// Resize the preview image.
    ///
    /// * `pixel_size`        - The dimensions of the preview image.
    /// * `window_inner_size` - The inner dimensions of the preview window.
    fn resize_pixels(
        &mut self,
        pixel_size: LogicalSize<u32>,
        window_inner_size: PhysicalSize<u32>,
    ) -> Result<(), String> {
        // Render only if the application has initialized and we have pixels and window.
        self.pixels.as_mut().map_or(Ok(()), |pixels| {
            // Resize the pixel surface texture to fit the windows inner dimensions.
            match pixels.resize_surface(window_inner_size.width, window_inner_size.height) {
                Ok(()) => {
                    // Resize the pixel image buffer.
                    match pixels.resize_buffer(pixel_size.width, pixel_size.height) {
                        Ok(()) => {
                            // Store the new sizes.
                            self.pixel_size = pixel_size;
                            self.window_inner_size = window_inner_size;

                            // Request a redraw.
                            self.window.as_ref().map(|window| window.request_redraw());
                            Ok(())
                        }
                        Err(err) => Err(format!("pixels.resize_buffer() failed.\n{}", err)),
                    }
                }
                Err(err) => Err(format!("pixels.resize_surface() failed to resize frame buffer surface.\n{}", err)),
            }
        })
    }
}

impl Default for App {
    /// Returns the "default value" for `App` initialized to the default dimensions.
    fn default() -> Self {
        Self {
            window: None,
            pixels: None,
            pixel_size: LogicalSize::new(CONFIG.image_width, CONFIG.image_height),
            window_inner_size: PhysicalSize::new(CONFIG.image_width, CONFIG.image_height),
        }
    }
}

impl ApplicationHandler<UserEvent> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create a new window.
        let window_attributes = Window::default_attributes()
            .with_title("Raytracing Series")
            .with_inner_size(self.window_inner_size)
            .with_resizable(true);

        let window = Arc::new(event_loop.create_window(window_attributes).expect("Unable to create window"));

        // Save the inner dimensions of the preview window.
        let window_inner_size = window.inner_size();

        // Create a surface texture that uses the logical inner size to render to the entire window's inner
        // dimensions.
        let surface_texture = SurfaceTexture::new(
            window_inner_size.width,
            window_inner_size.height,
            Arc::clone(&window),
        );

        // Create pixel frame buffer that matches rendered image dimensions that will be used to display it
        // in the window.
        let pixels = Pixels::new(self.pixel_size.width, self.pixel_size.height, surface_texture)
            .expect("Unable to create pixel frame buffer for window");

        self.window = Some(Arc::clone(&window));
        self.pixels = Some(pixels);
        self.window_inner_size = window_inner_size;
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                match self.render() {
                    Ok(()) => (),
                    Err(err) => {
                        eprintln!("Error redrawing pixels {}", err);
                        event_loop.exit();
                    }
                }
                self.window.as_ref().map(|window| window.request_redraw());
            }
            
            WindowEvent::Resized(new_window_inner_size) => {
                match self.resize_pixels(self.pixel_size, new_window_inner_size) {
                    Ok(()) => (),
                    Err(err) => {
                        eprintln!("Error resizing window {}", err);
                        event_loop.exit();
                   }
                }
            }

            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: key,
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => match key {
                Key::Named(NamedKey::Escape) => {
                    println!("Escape key was pressed; stopping");
                    event_loop.exit();
                }
                _ => (),
            },

            _ => (),
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: UserEvent) { 
        match event {
            UserEvent::RenderTile { tile_pixels, tile_bounds } => {
                if let Some(pixels) = self.pixels.as_mut() {
                    let frame = pixels.frame_mut();
                    copy_tile(frame, &tile_pixels, &tile_bounds);
                }
                self.window.as_ref().map(|window| window.request_redraw());
            }

            UserEvent::SaveImage => {
                if let Some(pixels) = self.pixels.as_ref() {
                    eprintln!("Saving output image to {}", CONFIG.output_path);

                    let frame = pixels.frame();

                    let rgba_image= image::RgbaImage::from_raw(
                        CONFIG.image_width,
                        CONFIG.image_height,
                        frame.to_vec(),
                    ).expect("Unable to convert pixel data to RGBA image");

                    rgba_image.save(&CONFIG.output_path)
                        .expect("Unable to save image");

                    eprintln!("Saved output image to {}", CONFIG.output_path);
                }
            }
        }
    }
}

/// Run the event loop displaying a window until it is closed or some error occurs.
pub fn run_event_loop() -> Result<(), EventLoopError> {
    eprintln!("Creating event loop");
    let event_loop = EventLoop::<UserEvent>::with_user_event().build().expect("Unable to create event loop");

    eprintln!("Creating event loop proxy");
    EVENT_LOOP_PROXY.get_or_init(|| event_loop.create_proxy());

    eprintln!("Running winit app");
    let mut app = App::default();
    event_loop.run_app(&mut app)
}

/// Send a user event to the event loop.
fn send_user_event(event: UserEvent) {
    // The rendering is done a different thread. We could end up here before the event loop is created. So just 
    // check and wait until event loop is ready. This loop will execute only once when the first scene starts 
    // processing.
    while EVENT_LOOP_PROXY.get().is_none() {
        thread::sleep(Duration::from_millis(100));
    }
    EVENT_LOOP_PROXY.get().map(|proxy| proxy.send_event(event));
}

/// Use a threadpool to queue up all the tiles for rendering.
pub fn render(pool: Arc<Mutex<ThreadPool>>, remaining_tiles: Arc<Mutex<usize>>) {
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

    let n_tiles = CONFIG.n_tiles();
    
    // Allocate an image buffer if not rendering to GUI.
    let image = if !CONFIG.gui {
        Some(Arc::new(Mutex::new(vec![0_u8; CONFIG.image_pixel_bytes()])))
    } else {
        None
    };

    // Queue up the tiles to render.
    for tile_idx in 0..n_tiles {
        // Clone the `Arc`s for the worker thread.
        let renderer = Arc::clone(&renderer);
        let remaining_tiles = Arc::clone(&remaining_tiles);
        let image = image.clone();

        pool.lock().unwrap().execute(move || {
            thread_local! {
                // Allocate pixels for rendering a tile per thread so we don't allocate for each tile.
                pub static TILE_PIXELS: RefCell<Vec<u8>> = {
                    eprintln!("\rAllocating tile pixels for {:?}", thread::current().id());
                    RefCell::new(vec![0; CONFIG.tiles_pixel_bytes()])
                };
            }

            TILE_PIXELS.with_borrow_mut(|tile_pixels| {
                // Calculate the tile bounds.
                let tile_bounds = get_tile_bounds(tile_idx);

                // Render whole tile and then copy to destination.
                render_tile(renderer, &tile_bounds, tile_pixels);

                if CONFIG.gui {
                    // Render the tile to the window.
                    send_user_event(UserEvent::RenderTile { tile_pixels: tile_pixels.clone(), tile_bounds });
                } else if let Some(image) = image.as_ref() {
                    // Render tile to image buffer.
                    let mut image_mutex = image.lock().unwrap();
                    copy_tile(&mut image_mutex, tile_pixels, &tile_bounds);
                }
            });

            *remaining_tiles.lock().unwrap() -= 1;
        });
    }

    loop {
        let remaining_tiles = *remaining_tiles.lock().unwrap();
        if remaining_tiles == 0 {
            thread::sleep(Duration::from_secs(2));
            eprintln!();

            if CONFIG.gui {
                // Save the window pixels.
                send_user_event(UserEvent::SaveImage);
            } else if let Some(image) = image.as_ref() {
                // Save the image buffer.
                eprintln!("Saving output image to {}", CONFIG.output_path);
                let image_mutex = image.lock().unwrap();

                if let Some(rgba_image) = image::RgbaImage::from_vec(
                    CONFIG.image_width,
                    CONFIG.image_height,
                    image_mutex.to_vec(),
                ) {
                    rgba_image.save(&CONFIG.output_path).expect("Unable to save image");
                    eprintln!("Saved output image to {}", CONFIG.output_path);
                } else {
                    eprintln!("Unable to convert pixel data to RGBA image");
                }
            }

            pool.lock().unwrap().shutdown();
            break;
        }
        thread::sleep(Duration::from_secs(1));
    }
}

