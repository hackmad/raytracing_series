use std::sync::Mutex;

use crate::RecursiveTracer;

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
/// * `tile_idx`  - Tile index.
/// * `n_tiles_x` - Number of tiles in x-direction.
/// * `tile_size` - Tile size in pixels.
/// * `width`     - Image width.
/// * `height`    - Image width.
pub fn get_tile_bounds(
    tile_idx: usize,
    n_tiles_x: usize,
    tile_size: u8,
    width: u32,
    height: u32,
) -> TileBounds {
    let tile_x = (tile_idx % n_tiles_x) as u32;
    let tile_y = (tile_idx / n_tiles_x) as u32;

    let ts = tile_size as u32;

    let x_min = tile_x * ts;
    let y_min = tile_y * ts;
    let x_max = (tile_x + 1) * ts - 1;
    let y_max = (tile_y + 1) * ts - 1;

    TileBounds {
        x_min,
        y_min,
        x_max: if x_max < width { x_max } else { width - 1 },
        y_max: if y_max < height { y_max } else { height - 1 },
    }
}

/// Render a single tile.
///
/// * `renderer`    - The ray tracer to use for rendering.
/// * `tile_bounds` - Tile bounds in image coordinates.
/// * `pixels`      - The tile pixels destination.
pub fn render_tile(
    renderer: &RecursiveTracer,
    tile_bounds: &TileBounds,
    pixels: &mut image::RgbImage,
) {
    for j in tile_bounds.y_min..=tile_bounds.y_max {
        for i in tile_bounds.x_min..=tile_bounds.x_max {
            let rgb = renderer.trace_ray(i, j).to_rgb();
            let x = i - tile_bounds.x_min;
            let y = j - tile_bounds.y_min;
            pixels.put_pixel(x, y, image::Rgb(rgb));
        }
    }
}

/// Copy a tile to to the image destination.
///
/// * `image_mutex` - The mutex holding the image.
/// * `tile_bounds` - Tile bounds in image coordinates.
/// * `pixels`      - The tile pixels destination.
pub fn copy_tile(
    image_mutex: &Mutex<image::RgbImage>,
    tile_bounds: &TileBounds,
    pixels: &image::RgbImage,
) {
    let mut img = image_mutex.lock().expect("Unable to lock image buffer");

    image::imageops::overlay(
        &mut *img,
        pixels,
        tile_bounds.x_min.into(),
        tile_bounds.y_min.into(),
    );
}
