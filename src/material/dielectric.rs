//! # Dielectric
//!
//! A library for handling dielectric material.

use super::{Colour, Float, HitRecord, Material, Ray, RcMaterial, RcRandomizer, ScatterResult};
use std::fmt;
use std::rc::Rc;

/// Models a dielectric material.
#[derive(Clone)]
pub struct Dielectric {
    /// Index of refraction.
    ref_idx: Float,

    /// Reciprocal of `ref_idx`.
    one_over_ref_idx: Float,

    /// Random number generator.
    rng: RcRandomizer,
}

impl Dielectric {
    /// Creates a new dielectric material.
    ///
    /// * `ri` - Index of refraction.
    /// * `rng` - Random number generator.
    pub fn new(ri: Float, rng: RcRandomizer) -> RcMaterial {
        Rc::new(Dielectric {
            ref_idx: ri,
            one_over_ref_idx: 1.0 / ri,
            rng: Rc::clone(&rng),
        })
    }
}

impl fmt::Display for Dielectric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "dielectric(ref_idx: {}, one_over_ref_idx: {})",
            self.ref_idx, self.one_over_ref_idx
        )
    }
}

impl fmt::Debug for Dielectric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Dielectric")
            .field("ref_idx", &self.ref_idx)
            .field("one_over_ref_idx", &self.one_over_ref_idx)
            .finish()
    }
}

/// Approximate the contribution of the Fresnel factor in the specular
/// reflection of light from a non-conducting interface (surface) between
/// two media
///
/// * `cosine` - Cosine of angle between the direction from which the
///              incident light is coming and the normal.
/// * `ref_idx` - Refractive index.
fn schlick(cosine: Float, ref_idx: Float) -> Float {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

impl Material for Dielectric {
    /// Scatter an incident ray and determine the attenuation.
    /// If the incident ray is absorbed, `None` is returned.
    ///
    /// Model the refractions and total internal reflection.
    ///
    /// * `ray_in` - Incident ray.
    /// * `rec` - The `HitRecord`.
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        // No attenuation
        let attenuation = Colour::new(1.0, 1.0, 1.0);

        let etai_over_etat = if rec.front_face {
            self.one_over_ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = ray_in.direction.unit_vector();

        let cos_theta = -unit_direction.dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        if etai_over_etat * sin_theta > 1.0 {
            let reflected = unit_direction.reflect(rec.normal);
            Some(ScatterResult {
                scattered: Ray::new(rec.point, reflected, ray_in.time),
                attenuation,
            })
        } else {
            let reflect_prob = schlick(cos_theta, etai_over_etat);
            if self.rng.clone().float() < reflect_prob {
                let reflected = unit_direction.reflect(rec.normal);
                Some(ScatterResult {
                    scattered: Ray::new(rec.point, reflected, ray_in.time),
                    attenuation,
                })
            } else {
                let refracted = unit_direction.refract(rec.normal, etai_over_etat);
                Some(ScatterResult {
                    scattered: Ray::new(rec.point, refracted, ray_in.time),
                    attenuation,
                })
            }
        }
    }
}
