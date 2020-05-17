//! # Perlin
//!
//! A library for the 3-dimensional perlin noise texture

#![allow(dead_code)]
use super::{ArcRandomizer, Float, Point3, Vec3};
use std::fmt;

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
}

impl Perlin {
    /// Creates a new perlin texture.
    ///
    /// * `size` - Grid size.
    /// * `rng` - Random number generator.
    pub fn new(size: usize, rng: ArcRandomizer) -> Perlin {
        let random: Vec<Vec3> = (0..size)
            .map(|_i| rng.vec3_in_range(-1.0, 1.0).unit_vector())
            .collect();

        let perm_x = perlin_generate_perm(size, &rng);
        let perm_y = perlin_generate_perm(size, &rng);
        let perm_z = perlin_generate_perm(size, &rng);

        Perlin {
            random,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    /// Evaluates the noise function at a point.
    ///
    /// * `p` - Point to evaluate the noise function.
    pub fn noise(&self, p: &Point3) -> Float {
        let fx = p.x().floor();
        let fy = p.y().floor();
        let fz = p.z().floor();

        let u = p.x() - fx;
        let v = p.y() - fy;
        let w = p.z() - fz;

        let i = fx as usize;
        let j = fy as usize;
        let k = fz as usize;

        let mut c = [[[Vec3::zero(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let x = self.perm_x[(i + di) & 255];
                    let y = self.perm_y[(j + dj) & 255];
                    let z = self.perm_z[(k + dk) & 255];

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
fn perlin_generate_perm(n: usize, rng: &ArcRandomizer) -> Vec<usize> {
    let mut p: Vec<usize> = (0..n).map(|x| x).collect();
    rng.permute(&mut p);
    p
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
        let ii = i as Float;

        for j in 0..2 {
            let jj = j as Float;

            for k in 0..2 {
                let kk = k as Float;

                let weight_v = Vec3::new(u - ii, v - jj, w - kk);

                accum += (ii * uu + (1.0 - ii) * (1.0 - uu))
                    * (jj * vv + (1.0 - jj) * (1.0 - vv))
                    * (kk * ww + (1.0 - kk) * (1.0 - ww))
                    * c[i][j][k].dot(weight_v);
            }
        }
    }

    accum
}

impl fmt::Display for Perlin {
    /// Display the perlin noise parameters.
    ///
    /// * `f` - Formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Debug for Perlin {
    /// Display the perlin noise parameters.
    ///
    /// * `f` - Formatter.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Perlin")
            .field("random", &self.random)
            .field("perm_x", &self.perm_x)
            .field("perm_y", &self.perm_y)
            .field("perm_z", &self.perm_z)
            .finish()
    }
}
