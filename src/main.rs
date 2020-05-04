extern crate rand;

mod algebra;
mod camera;
mod colour;
mod common;
mod material;
mod objects;

use algebra::*;
use camera::*;
use colour::*;
use common::*;
use material::*;
use objects::*;

fn main() {
    let image_width = 200;
    let image_height = 100;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Box::new(Lambertian::new(colour_from_vec3(Vec3::new(0.7, 0.3, 0.3)))),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(Lambertian::new(colour_from_vec3(Vec3::new(0.8, 0.8, 0.0)))),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Box::new(Metal::new(colour_from_vec3(Vec3::new(0.8, 0.6, 0.2)))),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Box::new(Metal::new(colour_from_vec3(Vec3::new(0.8, 0.8, 0.8)))),
    )));

    let cam = Camera::new();

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("Scan lines remaining: {}          \r", j);

        for i in 0..image_width {
            let mut colour = Vec3::zero();

            for _s in 0..samples_per_pixel {
                let u = ((i as f32) + random()) / (image_width as f32);
                let v = ((j as f32) + random()) / (image_height as f32);
                let r = cam.get_ray(u, v);
                colour += ray_colour(r, &world, max_depth);
            }

            let c = colour_to_ppm(colour_from_sample(colour, samples_per_pixel));
            println!("{}", c);
        }
    }
    eprintln!("\nDone!");
}

fn ray_colour(ray: Ray, world: &HittableList, depth: u32) -> Colour {
    if depth <= 0 {
        return colour_from_vec3(Vec3::zero());
    }

    match world.hit(ray, 0.001, f32::INFINITY) {
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
    let v = Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
    colour_from_vec3(v)
}
