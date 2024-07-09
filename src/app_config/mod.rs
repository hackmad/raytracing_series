//! # AppConfig
//!
//! A library for handling application configuration

use super::scene::Scenery;
use clap::{builder::EnumValueParser, Parser};
use std::thread::available_parallelism;

/// Program configuration.
#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct AppConfig {
    /// Image height.
    #[arg(
        long = "image-width",
        short = 'w',
        value_name = "WIDTH",
        default_value_t = 200,
        help = "image width in pixels"
    )]
    pub image_width: u32,

    /// Image width.
    #[arg(
        long = "image-height",
        short = 'h',
        value_name = "HEIGHT",
        default_value_t = 100,
        help = "image height in pixels"
    )]
    pub image_height: u32,

    /// Samples per pixels for antialiasing.
    #[arg(
        long = "samples-per-pixel",
        short = 's',
        value_name = "SAMPLES",
        default_value_t = 100,
        help = "number of samples per pixel for antialiasing"
    )]
    pub samples_per_pixel: u32,

    /// Max recursion depth
    #[arg(
        long = "max-depth",
        short = 'd',
        value_name = "DEPTH",
        default_value_t = 50,
        help = "maximum depth of recursion"
    )]
    pub max_depth: u32,

    /// Scene to render.
    #[arg(
        long = "scene",
        value_name = "SCENE",
        value_parser = EnumValueParser::<Scenery>::new(),
        default_value = "random_spheres",
        help = "scene to render",
    )]
    pub scenery: Scenery,

    /// Enable bounding value hierarchy.
    #[arg(
        long = "bvh",
        value_name = "BVH",
        help = "enable bounding volume hierarchy"
    )]
    pub bvh_enabled: bool,

    /// Random number seed.
    #[arg(
        long = "seed",
        value_name = "SEED",
        help = "seed for random number generator (debug)"
    )]
    pub seed: Option<u64>,

    /// Output file path.
    #[arg(
        long = "out",
        short = 'o',
        value_name = "OUTPUT_PATH",
        required = true,
        help = "output file path. file extension determines image type."
    )]
    pub output_path: String,

    /// Number of threads.
    #[arg(
        long = "threads",
        short = 't',
        value_name = "THREADS",
        default_value_t = get_max_threads(),
        help = "number of threads to use (default = max logical cores)",
    )]
    num_threads: usize,

    /// Tile size.
    #[arg(
        long = "tile-size",
        value_name = "TILE_SIZE",
        default_value_t = 32,
        help = "tile size in pixels (default = 32)"
    )]
    pub tile_size: u8,
}

impl AppConfig {
    /// Returns the number of threads to use.
    pub fn threads(&self) -> usize {
        let max_threads = get_max_threads();
        if self.num_threads == 0 {
            panic!("Invalid num threads");
        } else if self.num_threads > max_threads {
            panic!("Num threads > max logical CPUs {}", max_threads);
        }
        self.num_threads
    }
}

/// Returns the number of threads available. If unable, then 1 is returned.
fn get_max_threads() -> usize {
    available_parallelism().map_or(1, |n| n.get())
}
