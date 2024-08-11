use std::sync::{Arc, Mutex};

use image::{self, imageops};

use crate::{RecursiveTracer, COLOR_CHANNELS, CONFIG};

/// Tile bounds.
pub struct TileBounds {
    // Minimum x-coordinate.
    pub x_min: u32,

    // Minimum y-coordinate.
    pub y_min: u32,

    // Maximum x-coordinate.
    pub x_max: u32,

    // Maximum y-coordinate.
    pub y_max: u32,
}

/// Calculates number of tiles based on given image dimension and tile size.
///
/// * `tile_size` - Tile size in pixels.
/// * `dimension` - Image dimension (width or height).
pub fn get_tile_count(tile_size: u8, dimension: u32) -> usize {
    if dimension % tile_size as u32 == 0 {
        dimension as usize / tile_size as usize
    } else {
        dimension as usize / tile_size as usize + 1
    }
}

/// Calculates tile bounds based on a tile index. Tiles are counted from top-left to bottom-right.
///
/// * `tile_idx` - Tile index.
pub fn get_tile_bounds(tile_idx: usize) -> TileBounds {
    let tile_x = (tile_idx % CONFIG.n_tiles_x()) as u32;
    let tile_y = (tile_idx / CONFIG.n_tiles_x()) as u32;

    let y_min = tile_y * CONFIG.tile_size as u32;
    let mut y_max = y_min as u32 + CONFIG.tile_size as u32 - 1;
    if y_max > CONFIG.image_height - 1 {
        y_max = CONFIG.image_height - 1;
    }

    let x_min = tile_x * CONFIG.tile_size as u32;
    let mut x_max = x_min + CONFIG.tile_size as u32 - 1;
    if x_max > CONFIG.image_width - 1 {
        x_max = CONFIG.image_width - 1;
    }

    TileBounds {
        x_min,
        y_min,
        x_max,
        y_max,
    }
}

/// Render a single tile.
///
/// * `renderer`    - The ray tracer to use for rendering.
/// * `tile_bounds` - Tile bounds in image coordinates.
/// * `tile_pixels` - The tile pixels destination.
pub fn render_tile(
    renderer: Arc<RecursiveTracer>,
    tile_bounds: &TileBounds,
    tile_pixels: &mut [u8],
) {
    for j in tile_bounds.y_min..=tile_bounds.y_max {
        let ty = j - tile_bounds.y_min;

        for i in tile_bounds.x_min..=tile_bounds.x_max {
            let rgba = renderer.trace_ray(i, j).to_rgba();

            let tx = i - tile_bounds.x_min;
            let tile_offset = (ty * CONFIG.tile_size as u32 + tx) as usize * COLOR_CHANNELS;

            let dst = &mut tile_pixels[tile_offset..tile_offset + COLOR_CHANNELS];
            dst.copy_from_slice(&rgba);
        }
    }
}

/// Copy a tile to to the image destination.
///
/// * `imgbuf`      - The image buffer for rendered image.
/// * `tile_bounds` - Tile bounds in image coordinates.
/// * `tile_pixels` - The tile pixels source.
pub fn copy_tile(
    image: Arc<Mutex<image::RgbaImage>>,
    tile_bounds: &TileBounds,
    tile_pixels: &image::RgbaImage,
) {
    let mut img = image.lock().expect("Unable to lock image buffer");
    imageops::overlay(
        &mut *img,
        tile_pixels,
        tile_bounds.x_min as i64,
        tile_bounds.y_min as i64,
    );
}
