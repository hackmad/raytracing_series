//! # AppConfig
//!
//! A library for handling application configuration

use super::scene::Scenery;
use clap::{App, Arg};

/// Program configuration.
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Image height.
    pub image_width: u32,

    /// Image width.
    pub image_height: u32,

    /// Samples per pixels for antialiasing.
    pub samples_per_pixel: u32,

    /// Max recursion depth
    pub max_depth: u32,

    /// Scene to render.
    pub scenery: Scenery,

    /// Enable bounding value hierarchy.
    pub bvh_enabled: bool,

    /// Random number seed.
    pub seed: Option<u64>,

    /// Output file path.
    pub output_path: String,

    /// Number of threads.
    pub num_threads: usize,
}

impl AppConfig {
    pub fn load() -> AppConfig {
        // Get list of all scenes & get a list of their string representations.
        let scene_map = Scenery::all();
        let scenes: Vec<&str> = scene_map.keys().map(|k| k.as_ref()).collect();

        let matches = App::new("Raytracing in One Weekend")
            .version("0.0.1")
            .arg(
                Arg::with_name("image_width")
                    .short('w')
                    .long("image_width")
                    .value_name("WIDTH")
                    .takes_value(true)
                    .default_value("200")
                    .about("image width in pixels"),
            )
            .arg(
                Arg::with_name("image_height")
                    .short('h')
                    .long("image_height")
                    .value_name("HEIGHT")
                    .takes_value(true)
                    .default_value("100")
                    .about("image height in pixels"),
            )
            .arg(
                Arg::with_name("samples_per_pixel")
                    .short('s')
                    .long("samples_per_pixel")
                    .value_name("SAMPLES")
                    .takes_value(true)
                    .default_value("100")
                    .about("number of samples per pixel for antialiasing"),
            )
            .arg(
                Arg::with_name("max_depth")
                    .short('d')
                    .long("max_depth")
                    .value_name("DEPTH")
                    .takes_value(true)
                    .default_value("50")
                    .about("maximum depth of recursion"),
            )
            .arg(
                Arg::with_name("scene")
                    .long("scene")
                    .value_name("SCENE")
                    .takes_value(true)
                    .possible_values(&scenes)
                    .default_value("random_spheres")
                    .about("scene to render"),
            )
            .arg(
                Arg::with_name("bvh")
                    .long("bvh")
                    .value_name("BVH")
                    .takes_value(false)
                    .about("enable bounding volume hierarchy"),
            )
            .arg(
                Arg::with_name("seed")
                    .long("seed")
                    .value_name("SEED")
                    .takes_value(true)
                    .about("seed for rng number generator (debug)"),
            )
            .arg(
                Arg::with_name("out")
                    .short('o')
                    .long("out")
                    .value_name("OUTPUT_PATH")
                    .required(true)
                    .takes_value(true)
                    .about("output file path. file extension determines image type."),
            )
            .arg(
                Arg::with_name("threads")
                    .short('t')
                    .long("threads")
                    .value_name("THREADS")
                    .takes_value(true)
                    .about("number of threads to use (default = max logical cores)"),
            )
            .get_matches();

        let image_width = match matches.value_of("image_width") {
            Some(s) => s.parse::<u32>().unwrap(),
            _ => panic!("Invalid image width"),
        };

        let image_height = match matches.value_of("image_height") {
            Some(s) => s.parse::<u32>().unwrap(),
            _ => panic!("Invalid image height"),
        };

        let samples_per_pixel = match matches.value_of("samples_per_pixel") {
            Some(s) => s.parse::<u32>().unwrap(),
            _ => panic!("Invalid samples per pixel"),
        };

        let max_depth = match matches.value_of("max_depth") {
            Some(s) => s.parse::<u32>().unwrap(),
            _ => panic!("Invalid max depth"),
        };

        let scenery = match matches.value_of("scene") {
            Some(s) => scene_map
                .get(s)
                .expect(format!("Unknown scene {}", s).as_ref()),
            _ => panic!("Invalid scene name"),
        };

        let bvh_enabled = matches.is_present("bvh");

        let seed = match matches.value_of("seed") {
            Some(s) => Some(s.parse::<u64>().unwrap()),
            _ => None,
        };

        let output_path = match matches.value_of("out") {
            Some(s) => s.to_string(),
            _ => panic!("Missing output path"),
        };

        let max_threads = num_cpus::get();
        let num_threads = match matches.value_of("threads") {
            Some(s) => {
                let n = s.parse::<usize>().expect("Invalid num threads");

                if n == 0 {
                    panic!("Invalid num threads");
                } else if n > max_threads {
                    panic!(format!("Num threads > max logical CPUs {}", max_threads));
                }

                n
            }

            _ => max_threads,
        };

        AppConfig {
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
            scenery: *scenery,
            bvh_enabled,
            seed,
            output_path,
            num_threads,
        }
    }
}
