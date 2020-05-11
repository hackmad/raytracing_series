//! # Random
//!
//! A library for generating random numbers.

use super::{Float, Randomizer, RcRandomizer, Vec3, PI};
use rand::{thread_rng, Rng, RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;
use std::cell::RefCell;
use std::rc::Rc;

/// Random number generator.
#[derive(Clone)]
pub struct Random<T: RngCore> {
    /// The random number generator.
    rng: RefCell<T>,
}

/// Create a new thread local random number generator.
pub fn new_thread_rng() -> RcRandomizer {
    Rc::new(Random {
        rng: RefCell::new(thread_rng()),
    })
}

/// Create a new seeded random number generator.
///
/// * `seed`: Seed.
pub fn new_seeded_rng(seed: u64) -> RcRandomizer {
    let rng: ChaCha20Rng = SeedableRng::seed_from_u64(seed);
    Rc::new(Random {
        rng: RefCell::new(rng),
    })
}

impl<T> Randomizer for Random<T>
where
    T: RngCore,
{
    /// Returns a random floating point value in [0, 1].
    fn float(&self) -> Float {
        self.rng.borrow_mut().gen::<Float>()
    }

    /// Returns a random floating point values in [`min`, `max`].
    ///
    /// * `min` - Minimum bound
    /// * `max` - Maximum bound
    fn float_in_range(&self, min: Float, max: Float) -> Float {
        self.rng.borrow_mut().gen_range(min, max)
    }

    /// Returns a random usize value in [`min`, `max`].
    ///
    /// * `min` - Minimum bound
    /// * `max` - Maximum bound
    fn usize_in_range(&self, min: usize, max: usize) -> usize {
        self.rng.borrow_mut().gen_range(min, max)
    }

    /// Returns a random vector with random components in [0, 1].
    fn vec3(&self) -> Vec3 {
        Vec3::new(self.float(), self.float(), self.float())
    }

    /// Returns a random vector with random components in [`min`, `max`].
    ///
    /// * `min` - Minimum bound
    /// * `max` - Maximum bound
    fn vec3_in_range(&self, min: Float, max: Float) -> Vec3 {
        Vec3::new(
            self.float_in_range(min, max),
            self.float_in_range(min, max),
            self.float_in_range(min, max),
        )
    }

    /// Returns a random vector within the unit sphere. This vector is not
    /// normalized.
    fn in_unit_sphere(&self) -> Vec3 {
        loop {
            let p = self.vec3_in_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                break p;
            }
        }
    }

    /// Returns a random unit vector by picking points on the unit sphere
    /// and then normalizing it.
    fn unit_vec3(&self) -> Vec3 {
        let a = self.float_in_range(0.0, 2.0 * PI);
        let z = self.float_in_range(-1.0, 1.0);
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
        loop {
            let p = Vec3::new(
                self.float_in_range(-1.0, 1.0),
                self.float_in_range(-1.0, 1.0),
                0.0,
            );
            if p.length_squared() < 1.0 {
                break p;
            }
        }
    }
}
