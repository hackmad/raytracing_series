//! # Random
//!
//! A library for generating random numbers.

use super::{ArcRandomizer, Float, Randomizer, Vec3, TWO_PI};
use rand::distributions::uniform::{SampleBorrow, SampleUniform};
use rand::{Rng, RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;
use std::fmt;
use std::sync::{Arc, Mutex, MutexGuard};

/// Random number generator.
pub struct Random<T: RngCore> {
    /// The random number generator.
    rng: Mutex<T>,
}

/// Create a new thread local random number generator.
pub fn new_thread_rng() -> ArcRandomizer {
    let rng: ChaCha20Rng = SeedableRng::from_entropy();
    Arc::new(Random {
        rng: Mutex::new(rng),
    })
}

/// Create a new seeded random number generator.
///
/// * `seed`: Seed.
pub fn new_seeded_rng(seed: u64) -> ArcRandomizer {
    let rng: ChaCha20Rng = SeedableRng::seed_from_u64(seed);
    Arc::new(Random {
        rng: Mutex::new(rng),
    })
}

impl<T> fmt::Debug for Random<T>
where
    T: RngCore,
{
    /// This is here to squash complaints from using ArcRandomizer in Hittable
    /// because of Debug requirements.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Random").finish()
    }
}

impl<T> Randomizer for Random<T>
where
    T: RngCore,
{
    /// Returns a random floating point value in [0, 1].
    fn float(&self) -> Float {
        let mut rng = self.rng.lock().unwrap();
        Random::float(&mut rng)
    }

    /// Returns a random floating point value in [`min`, `max`].
    ///
    /// * `min` - Minimum bound
    /// * `max` - Maximum bound
    fn float_in_range(&self, min: Float, max: Float) -> Float {
        let mut rng = self.rng.lock().unwrap();
        Random::in_range(&mut rng, min, max)
    }

    /// Returns a random usize value in [`min`, `max`].
    ///
    /// * `min` - Minimum bound
    /// * `max` - Maximum bound
    fn usize_in_range(&self, min: usize, max: usize) -> usize {
        let mut rng = self.rng.lock().unwrap();
        Random::in_range(&mut rng, min, max)
    }

    /// Returns a random u8 value in [`min`, `max`].
    ///
    /// * `min` - Minimum bound
    /// * `max` - Maximum bound
    fn u8_in_range(&self, min: u8, max: u8) -> u8 {
        let mut rng = self.rng.lock().unwrap();
        Random::in_range(&mut rng, min, max)
    }

    /// Returns a random u16 value in [`min`, `max`].
    ///
    /// * `min` - Minimum bound
    /// * `max` - Maximum bound
    fn u16_in_range(&self, min: u16, max: u16) -> u16 {
        let mut rng = self.rng.lock().unwrap();
        Random::in_range(&mut rng, min, max)
    }

    /// Returns a random u32 value in [`min`, `max`].
    ///
    /// * `min` - Minimum bound
    /// * `max` - Maximum bound
    fn u32_in_range(&self, min: u32, max: u32) -> u32 {
        let mut rng = self.rng.lock().unwrap();
        Random::in_range(&mut rng, min, max)
    }

    /// Returns a random u64 value in [`min`, `max`].
    ///
    /// * `min` - Minimum bound
    /// * `max` - Maximum bound
    fn u64_in_range(&self, min: u64, max: u64) -> u64 {
        let mut rng = self.rng.lock().unwrap();
        Random::in_range(&mut rng, min, max)
    }

    /// Returns a random vector with random components in [0, 1].
    fn vec3(&self) -> Vec3 {
        let mut rng = self.rng.lock().unwrap();
        Vec3::new(
            Random::float(&mut rng),
            Random::float(&mut rng),
            Random::float(&mut rng),
        )
    }

    /// Returns a random vector with random components in [`min`, `max`].
    ///
    /// * `min` - Minimum bound
    /// * `max` - Maximum bound
    fn vec3_in_range(&self, min: Float, max: Float) -> Vec3 {
        let mut rng = self.rng.lock().unwrap();
        Random::vec3_in_range(&mut rng, min, max)
    }

    /// Returns a random vector within the unit sphere. This vector is not
    /// normalized.
    fn in_unit_sphere(&self) -> Vec3 {
        let mut rng = self.rng.lock().unwrap();
        loop {
            let p = Random::vec3_in_range(&mut rng, -1.0, 1.0);
            if p.length_squared() < 1.0 {
                break p;
            }
        }
    }

    /// Returns a random unit vector by picking points on the unit sphere
    /// and then normalizing it.
    fn unit_vec3(&self) -> Vec3 {
        let mut rng = self.rng.lock().unwrap();
        let a = Random::in_range(&mut rng, 0.0, TWO_PI);
        let z = Random::in_range(&mut rng, -1.0, 1.0) as Float;
        let r = (1.0 - z * z).sqrt();
        Vec3::new(r * a.cos(), r * a.sin(), z)
    }

    /// Returns a random vector with uniform scatter direction for all angles
    /// away from a hit point, with no dependence on the angle from the normal.
    ///
    /// * `normal` - THe surface normal.
    fn in_hemisphere(&self, normal: Vec3) -> Vec3 {
        let in_unit_sphere = self.in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            // In the same hemisphere as the normal
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    /// Returns a random point inside unit disk
    fn in_unit_disk(&self) -> Vec3 {
        let mut rng = self.rng.lock().unwrap();
        loop {
            let p = Vec3::new(
                Random::in_range(&mut rng, -1.0, 1.0),
                Random::in_range(&mut rng, -1.0, 1.0),
                0.0,
            );
            if p.length_squared() < 1.0 {
                break p;
            }
        }
    }
}

/// This implements associated functions to help call methods on the random
/// number generator mutex without locking repeatedly to get multiple samples.
impl<T> Random<T>
where
    T: RngCore,
{
    /// Returns a random floating point value in [0, 1].
    fn float(rng: &mut MutexGuard<'_, T>) -> Float {
        rng.gen::<Float>()
    }

    /// Returns a random floating point values in [`min`, `max`].
    ///
    /// * `min` - Minimum bound
    /// * `max` - Maximum bound
    fn in_range<U: SampleUniform, B1, B2>(rng: &mut MutexGuard<'_, T>, min: B1, max: B2) -> U
    where
        B1: SampleBorrow<U> + Sized,
        B2: SampleBorrow<U> + Sized,
    {
        rng.gen_range::<U, B1, B2>(min, max)
    }

    /// Returns a random unit vector by picking points on the unit sphere
    /// and then normalizing it.
    fn vec3_in_range(rng: &mut MutexGuard<'_, T>, min: Float, max: Float) -> Vec3 {
        Vec3::new(
            Random::in_range(rng, min, max),
            Random::in_range(rng, min, max),
            Random::in_range(rng, min, max),
        )
    }
}
