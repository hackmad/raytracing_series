extern crate rand;

mod algebra;
mod camera;
mod colour;
mod common;
mod objects;

use algebra::*;
use camera::*;
use colour::*;
use common::*;
use objects::*;

fn main() {
    let image_width = 200;
    let image_height = 100;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

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

            let c = Colour::new_from_vec3(colour, samples_per_pixel).as_ppm();
            println!("{}", c);
        }
    }
    eprintln!("\nDone!");
}

fn ray_colour(ray: Ray, world: &HittableList, depth: u32) -> Vec3 {
    match world.hit(ray, 0.001, f32::INFINITY) {
        Some(HitRecord {
            t,
            normal,
            point,
            front_face: _,
        }) if t > 0.0 => {
            if depth <= 0 {
                Vec3::zero()
            } else {
                let target = point + random_in_hemisphere(normal);
                let new_ray = Ray::new(point, target - point);
                ray_colour(new_ray, world, depth - 1) * 0.5
            }
        }

        _ => {
            let unit_direction = ray.direction.unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    }
}
