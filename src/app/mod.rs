//! The application

use std::sync::{Arc, Mutex};

use pixels::{Pixels, SurfaceTexture};
use tao::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::Key,
    window::{Window, WindowBuilder},
};

use crate::{ThreadPool, CONFIG};

/// The application.
pub struct App {
    /// Event loop.
    event_loop: EventLoop<()>,

    /// Window.
    pub window: Arc<Window>,

    /// Pixel buffer for displaying image in the window.
    pixels: Arc<Mutex<Pixels>>,

    /// Thread pool used for rendering the image in parallel.
    pool: Arc<Mutex<ThreadPool>>,

    /// The image buffer used for rendering.
    image: Arc<Mutex<image::RgbaImage>>,
}

impl App {
    /// Build a new `App`.
    ///
    /// * `pool`  - Thread pool used for rendering the image in parallel.
    /// * `image` - The image buffer used for rendering.
    pub fn build(
        pool: Arc<Mutex<ThreadPool>>,
        image: Arc<Mutex<image::RgbaImage>>,
    ) -> Result<Self, String> {
        // Initialize logger for tao.
        env_logger::init();

        // Create a new event loop for the application.
        let event_loop = EventLoop::new();

        // Create a new window.
        let window = WindowBuilder::new()
            .with_title("Raytracing Series")
            .with_inner_size(LogicalSize::new(CONFIG.image_width, CONFIG.image_height))
            .with_resizable(false)
            .build(&event_loop)
            .map_err(|e| e.to_string())?;

        let inner_size = window.inner_size();

        // Create a surface texture that uses the logical inner size to render to the entire window's inner dimensions.
        let surface_texture = SurfaceTexture::new(inner_size.width, inner_size.height, &window);

        // Create pixel frame buffer that matches rendered image dimensions that will be used to display it in the
        // window.
        let pixels = Pixels::new(CONFIG.image_width, CONFIG.image_height, surface_texture)
            .map_err(|e| e.to_string())?;

        Ok(Self {
            event_loop,
            window: Arc::new(window),
            pixels: Arc::new(Mutex::new(pixels)),
            pool,
            image,
        })
    }

    /// Run the event loop displaying the GUI window until it is closed or some error occurs.
    ///
    /// NOTE: This consumes `self` to avoid life time issues with references and also it runs indefinitely so it can
    /// only ever be called once.
    pub fn run(self) -> ! {
        let Self {
            pool,
            image,
            pixels,
            event_loop,
            window: _,
        } = self;

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
