//! # Perlin
//!
//! A library for the 3-dimensional perlin noise texture

#![allow(dead_code)]
use super::{Float, Point3, RcRandomizer, Vec3};
use std::fmt;
use std::rc::Rc;

/// Perlin noise generator.
#[derive(Clone)]
pub struct Perlin {
    /// Vector containing random floating point numbers.
    random: Vec<Vec3>,

    /// Permutation for x-dimension.
    perm_x: Vec<usize>,

    /// Permutation for y-dimension.
    perm_y: Vec<usize>,

    /// Permutation for z-dimension.
    perm_z: Vec<usize>,

    /// Random number generator.
    rng: RcRandomizer,
}

impl Perlin {
    /// Creates a new perlin texture.
    ///
    /// * `size` - Grid size.
    /// * `rng` - Random number generator.
    pub fn new(size: usize, rng: RcRandomizer) -> Perlin {
        let random: Vec<Vec3> = (0..size)
            .map(|_i| Rc::clone(&rng).vec3_in_range(-1.0, 1.0).unit_vector())
            .collect();

        let perm_x = perlin_generate_perm(size, Rc::clone(&rng));
        let perm_y = perlin_generate_perm(size, Rc::clone(&rng));
        let perm_z = perlin_generate_perm(size, Rc::clone(&rng));

        Perlin {
            random,
            perm_x,
            perm_y,
            perm_z,
            rng: Rc::clone(&rng),
        }
    }

    /// Evaluates the noise function at a point.
    ///
    /// * `p` - Point to evaluate the noise function.
    pub fn noise(&self, p: &Point3) -> Float {
        let i = p.x().floor();
        let j = p.y().floor();
        let k = p.z().floor();

        let u = p.x() - i;
        let v = p.y() - j;
        let w = p.z() - k;

        let mut c = [[[Vec3::zero(); 2]; 2]; 2];

        for di in 0..2 {
            let x = self.perm_x[((i + di as Float) as usize) & 255];

            for dj in 0..2 {
                let y = self.perm_y[((j + dj as Float) as usize) & 255];

                for dk in 0..2 {
                    let z = self.perm_z[((k + dk as Float) as usize) & 255];

                    c[di][dj][dk] = self.random[x ^ y ^ z];
                }
            }
        }

        perlin_interp(&c, u, v, w)
    }

    /// Composite noise across multiple summed frequencies.
    ///
    /// * `p` - Point to evaluate the noise function.
    /// * `depth` - Number of iterations to evaluate noise function.
    pub fn turbulence(&self, p: &Point3, depth: usize) -> Float {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _i in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }
}

/// Generate a random permuation.
///
/// * `n` - Number of points.
fn perlin_generate_perm(n: usize, rng: RcRandomizer) -> Vec<usize> {
    let mut p: Vec<usize> = (0..n).map(|x| x).collect();
    permute(&mut p, rng);
    p
}

/// Shuffle a vector in place.
///
/// * `v` - Vector to shuffle.
fn permute(v: &mut Vec<usize>, rng: RcRandomizer) {
    for i in (1..v.len()).rev() {
        let target = rng.usize_in_range(0, i);

        let (x, y) = (v[i], v[target]);

        v[i] = y;
        v[target] = x;
    }
}

/// Perform interpolation.
///
/// * `c` - A 2x2x2 array of random vectors.
/// * `u` - Parametric coordinate in [0.0, 1.0].
/// * `v` - Parametric coordinate in [0.0, 1.0].
/// * `w` - Parametric coordinate in [0.0, 1.0].
fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: Float, v: Float, w: Float) -> Float {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);

    let mut accum = 0.0;

    for i in 0..2 {
        let i1 = i as Float;

        for j in 0..2 {
            let j1 = j as Float;

            for k in 0..2 {
                let k1 = k as Float;

                let weight_v = Vec3::new(u - i1, v - j1, w - k1);

                accum += (i1 * uu + (1.0 - i1) * (1.0 - uu))
                    * (j1 * vv + (1.0 - j1) * (1.0 - vv))
                    * (k1 * ww + (1.0 - k1) * (1.0 - ww))
                    * c[i][j][k].dot(weight_v);
            }
        }
    }

    accum
}

impl fmt::Display for Perlin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Debug for Perlin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Perlin")
            .field("random", &self.random)
            .field("perm_x", &self.perm_x)
            .field("perm_y", &self.perm_y)
            .field("perm_z", &self.perm_z)
            .finish()
    }
}
