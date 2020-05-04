use super::random;
use super::Float;
use super::HitRecord;
use super::Material;
use super::Ray;
use super::RcMaterial;
use super::ScatterResult;
use super::Vec3;
use std::rc::Rc;

#[derive(Clone)]
pub struct Dielectric {
    ref_idx: Float,
    one_over_ref_idx: Float,
}

impl Dielectric {
    pub fn new(ri: Float) -> RcMaterial {
        Rc::new(Dielectric {
            ref_idx: ri,
            one_over_ref_idx: 1.0 / ri,
        })
    }
}

fn schlick(cosine: Float, ref_idx: Float) -> Float {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0).as_colour(); // no attenuation

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
                scattered: Ray::new(rec.point, reflected),
                attenuation,
            })
        } else {
            let reflect_prob = schlick(cos_theta, etai_over_etat);
            if random() < reflect_prob {
                let reflected = unit_direction.reflect(rec.normal);
                Some(ScatterResult {
                    scattered: Ray::new(rec.point, reflected),
                    attenuation,
                })
            } else {
                let refracted = unit_direction.refract(rec.normal, etai_over_etat);
                Some(ScatterResult {
                    scattered: Ray::new(rec.point, refracted),
                    attenuation,
                })
            }
        }
    }
}
