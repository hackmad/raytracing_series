extern crate rand;

mod algebra;
mod camera;
mod common;
mod material;
mod objects;

use algebra::*;
use camera::*;
use common::*;
use material::*;
use objects::*;

fn main() {
    let image_width = 200;
    let image_height = 100;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let aspect_ratio = (image_width as Float) / (image_height as Float);

    let world = random_scene();

    let lookfrom = Vec3::new(13.0, 2.0, 3.0).as_point();
    let lookat = Vec3::zero().as_point();
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.1;
    let vfov = 20.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        focus_dist,
    );

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("Scan lines remaining: {}          \r", j);

        for i in 0..image_width {
            let mut colour = Vec3::zero().as_colour();

            for _s in 0..samples_per_pixel {
                let u = ((i as Float) + random()) / (image_width as Float);
                let v = ((j as Float) + random()) / (image_height as Float);
                let r = cam.get_ray(u, v);
                colour += ray_colour(r, &world, max_depth);
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

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5).as_colour())),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random();
            let center = Vec3::new(
                a as Float + 0.9 * random(),
                0.2,
                b as Float + 0.9 * random(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = (random_vec3() * random_vec3()).as_colour();
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(Lambertian::new(albedo)),
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random_vec3_in_range(0.5, 1.0).as_colour();
                    let fuzz = random_in_range(0.0, 0.5);

                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(Metal::new(albedo, fuzz)),
                    )));
                } else {
                    // glass
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Box::new(Dielectric::new(1.5)),
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1).as_colour())),
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5).as_colour(), 0.0)),
    )));

    world
}
