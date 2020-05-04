extern crate rand;

mod algebra;
mod camera;
mod common;
mod material;
mod object;
mod scene;

use algebra::*;
use common::*;
use object::*;
use scene::*;

fn main() {
    let image_width = 200;
    let image_height = 100;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let s = Scene::new_random_scene(image_width, image_height);

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("Scan lines remaining: {}          \r", j);

        for i in 0..image_width {
            let mut colour = Vec3::zero().as_colour();

            for _s in 0..samples_per_pixel {
                let u = ((i as Float) + random()) / (image_width as Float);
                let v = ((j as Float) + random()) / (image_height as Float);

                let r = s.camera.get_ray(u, v);
                colour += ray_colour(r, &s.world, max_depth);
            }

            let c = colour.to_colour_from_sample(samples_per_pixel).to_ppm();
            println!("{}", c);
        }
    }
    eprintln!("\nDone!");
}

fn ray_colour(ray: Ray, world: &HittableList, depth: u32) -> Colour {
    if depth <= 0 {
        return Vec3::zero().as_colour();
    }

    match world.hit(ray, 0.001, INFINITY) {
        Some(rec) => {
            if let Some(sr) = rec.material.clone().scatter(ray, rec) {
                ray_colour(sr.scattered, world, depth - 1) * sr.attenuation
            } else {
                background_colour(ray)
            }
        }

        _ => background_colour(ray),
    }
}

fn background_colour(ray: Ray) -> Colour {
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t).as_colour()
}
