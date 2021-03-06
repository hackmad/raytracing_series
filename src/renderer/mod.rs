//! # Renderer
//!
//! A library for renderering algorithm.

use super::algebra::{Colour, Ray};
use super::app_config::AppConfig;
use super::common::{Float, HittablePDF, MixturePDF, Random, INFINITY, PDF, RAY_EPSILON};
use super::scene::Scene;
use std::sync::Arc;

/// Implements recursive raytracer that uses importance sampling.
pub struct RecursiveTracer {
    /// The scene.
    pub scene: Scene,

    /// Application configuration.
    pub config: AppConfig,
}

impl RecursiveTracer {
    /// Trace a ray through the scene return accumulated colour. The function will
    /// generate multiple samples per pixel.
    ///
    /// * `i` - Pixel x-coordinate.
    /// * `j` - Pixel y-coordinate.
    /// * `config` - Program configuration.
    /// * `tracer` - The rendering algorithm.
    pub fn trace_ray(&self, i: u32, j: u32) -> Colour {
        let x = i as Float;
        let y = j as Float;

        let w = self.config.image_width as Float;
        let h = self.config.image_height as Float;
        let n = self.config.samples_per_pixel;

        (0..n)
            .fold(Colour::zero(), |colour, _| {
                let s = Random::samples::<Float>(2);

                let u = (x + s[0]) / w;
                let v = (y + s[1]) / h;

                let ray = self.scene.camera.get_ray(u, v);
                colour + self.ray_colour(&ray, self.config.max_depth)
            })
            .to_colour_from_sample(n)
    }

    /// Recursively traces a ray through the scene and generates the colour seen
    /// at the image plane.
    ///
    /// * `ray` - The ray.
    /// * `depth` - Maximum depth for recursion.
    fn ray_colour(&self, ray: &Ray, depth: u32) -> Colour {
        // Terminate the recursion if maximum depth is reached.
        if depth <= 0 {
            return Colour::zero();
        }

        // Note the RAY_EPSILON is used to avoid starting the ray inside the
        // surface caused due to floating point approximation errors generated
        // by the intersection routine.
        let hit = self.scene.world.hit(&ray, RAY_EPSILON, INFINITY);
        if hit.is_none() {
            return (self.scene.background)(ray);
        }

        let rec = hit.unwrap();

        // Calculate emission from material.
        let emission = rec.material.emission(ray, &rec);

        // If material did not absorb the ray and scattered it, continue tracing
        // the new ray.
        let scatter = rec.material.scatter(ray, &rec);
        if scatter.is_none() {
            return emission;
        }

        let sr = scatter.unwrap();

        if let Some(specular_ray) = sr.specular_ray {
            // Specular materials
            let colour = self.ray_colour(&specular_ray, depth - 1);
            emission + sr.attenuation * colour
        } else if let Some(scattered_ray) = sr.scattered_ray {
            // This handles isotropic material.
            let colour = self.ray_colour(&scattered_ray, depth - 1);
            emission + sr.attenuation * colour
        } else if let Some(pdf) = sr.pdf {
            // Diffuse material
            let lights = Arc::clone(&self.scene.lights);

            let light_pdf = Arc::new(HittablePDF::new(lights, rec.point));
            let diffuse_pdf = Arc::clone(&pdf);

            let p = MixturePDF::new(light_pdf, diffuse_pdf);

            let scattered = Ray::new(rec.point, p.generate(), ray.time);
            let pdf_val = p.value(scattered.direction);
            if pdf_val > 0.0 {
                let scattering_pdf = rec.material.scattering_pdf(&ray, &rec, &scattered);

                let colour = self.ray_colour(&scattered, depth - 1);
                emission + sr.attenuation * scattering_pdf * colour / pdf_val
            } else {
                emission
            }
        } else {
            emission
        }
    }
}
