//! # Random
//!
//! A library for generating random numbers.

#![allow(dead_code)]

use super::{Float, Vec3, TWO_PI};
use rand::distributions::uniform::SampleUniform;
use rand::distributions::{Distribution, Standard};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use std::cell::RefCell;

thread_local! {
    /// Create a new thread local seedable random number generator initialized
    /// with a random seed.
    static RNG: RefCell<ChaCha20Rng> = {
        let rng: ChaCha20Rng = SeedableRng::from_entropy();
        RefCell::new(rng)
    }
}

/// Wraps some common random sample generation routines using a thread_rng().
pub struct Random {}

impl Random {
    /// Set the seed for the random number generator.
    ///
    /// * `s` - The seed.
    pub fn seed(s: u64) {
        RNG.with(|rng| *rng.borrow_mut() = SeedableRng::seed_from_u64(s))
    }

    /// Returns a random value.
    pub fn sample<T>() -> T
    where
        Standard: Distribution<T>,
    {
        RNG.with(|rng| rng.borrow_mut().gen::<T>())
    }

    /// Returns `n` random floating point values in [0, 1].
    ///
    /// * `n` - Number of samples.
    pub fn samples<T>(n: usize) -> Vec<T>
    where
        Standard: Distribution<T>,
    {
        RNG.with(|rng| {
            let mut r = rng.borrow_mut();
            (0..n).map(|_| r.gen::<T>()).collect()
        })
    }

    /// Returns a random value in [`min`, `max`].
    ///
    /// * `min` - Minimum bound
    /// * `max` - Maximum bound
    pub fn sample_in_range<T: SampleUniform>(min: T, max: T) -> T {
        RNG.with(|rng| {
            let mut r = rng.borrow_mut();
            r.gen_range(min, max)
        })
    }

    /// Returns `n` random values in [`min`, `max`].
    ///
    /// * `n` - Number of samples.
    /// * `min` - Minimum bound
    /// * `max` - Maximum bound
    pub fn samples_in_range<T: SampleUniform>(n: usize, min: T, max: T) -> Vec<T>
    where
        T: Copy,
    {
        RNG.with(|rng| {
            let mut r = rng.borrow_mut();
            (0..n).map(|_| r.gen_range(min, max)).collect()
        })
    }

    /// Returns a random vector with random components in [0, 1].
    pub fn vec3() -> Vec3 {
        RNG.with(|rng| {
            let mut r = rng.borrow_mut();
            Vec3::new(r.gen::<Float>(), r.gen::<Float>(), r.gen::<Float>())
        })
    }

    /// Returns a random unit vector by picking points on the unit sphere
    /// and then normalizing it.
    pub fn vec3_in_range(min: Float, max: Float) -> Vec3 {
        RNG.with(|rng| {
            let mut r = rng.borrow_mut();
            Vec3::new(
                r.gen_range(min, max),
                r.gen_range(min, max),
                r.gen_range(min, max),
            )
        })
    }

    /// Returns a random vector within the unit sphere. This vector is not
    /// normalized.
    pub fn vec3_in_unit_sphere() -> Vec3 {
        RNG.with(|rng| {
            let mut r = rng.borrow_mut();
            loop {
                let p = Vec3::new(
                    r.gen_range(-1.0, 1.0),
                    r.gen_range(-1.0, 1.0),
                    r.gen_range(-1.0, 1.0),
                );
                if p.length_squared() < 1.0 {
                    break p;
                }
            }
        })
    }

    /// Returns a random unit vector by picking points on the unit sphere
    /// and then normalizing it.
    pub fn unit_vec3() -> Vec3 {
        RNG.with(|rng| {
            let mut r = rng.borrow_mut();
            let a = r.gen_range::<Float, Float, Float>(0.0, TWO_PI);
            let z = r.gen_range::<Float, Float, Float>(-1.0, 1.0);
            let r = (1.0 - z * z).sqrt();
            Vec3::new(r * a.cos(), r * a.sin(), z)
        })
    }

    /// Returns a random vector with uniform scatter direction for all angles
    /// away from a hit point, with no dependence on the angle from the normal.
    ///
    /// * `normal` - THe surface normal.
    pub fn vec3_in_hemisphere(normal: Vec3) -> Vec3 {
        let in_unit_sphere = Random::vec3_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            // In the same hemisphere as the normal
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    /// Returns a random point inside unit disk in the xy-plane.
    pub fn vec3_in_unit_disk() -> Vec3 {
        RNG.with(|rng| {
            let mut r = rng.borrow_mut();
            loop {
                let p = Vec3::new(r.gen_range(-1.0, 1.0), r.gen_range(-1.0, 1.0), 0.0);
                if p.length_squared() < 1.0 {
                    break p;
                }
            }
        })
    }

    /// Shuffle a `Vec<usize>` in place.
    ///
    /// * `v` - Vector to shuffle.
    pub fn permute(v: &mut Vec<usize>) {
        RNG.with(|rng| {
            let mut r = rng.borrow_mut();
            for i in (1..v.len()).rev() {
                let target = r.gen_range(0, i);

                let (x, y) = (v[i], v[target]);

                v[i] = y;
                v[target] = x;
            }
        })
    }

    /// Returns a random vector based on p(direction) = cos(θ) / π.
    pub fn cosine_direction() -> Vec3 {
        RNG.with(|rng| {
            let mut r = rng.borrow_mut();

            let r1 = r.gen::<Float>();
            let r2 = r.gen::<Float>();
            let z = (1.0 - r2).sqrt();

            let phi = TWO_PI * r1;

            let r2_sqrt = r2.sqrt();
            let x = phi.cos() * r2_sqrt;
            let y = phi.sin() * r2_sqrt;

            Vec3::new(x, y, z)
        })
    }

    // Return a random vector uniformly sampled from a sphere’s solid angle
    // from a point outside the sphere
    //
    // * `distance_squared` - Square of distance to a point from sphere center.
    pub fn vec3_to_sphere(radius: Float, distance_squared: Float) -> Vec3 {
        RNG.with(|rng| {
            let mut r = rng.borrow_mut();

            let r1 = r.gen::<Float>();
            let r2 = r.gen::<Float>();

            let r_squared_over_d_squared = radius * radius / distance_squared;
            let z = 1.0 + r2 * ((1.0 - r_squared_over_d_squared).sqrt() - 1.0);

            let phi = TWO_PI * r1;

            let sqrt_one_minus_z_squared = (1.0 - z * z).sqrt();
            let x = phi.cos() * sqrt_one_minus_z_squared;
            let y = phi.sin() * sqrt_one_minus_z_squared;

            Vec3::new(x, y, z)
        })
    }
}
