use super::random_in_unit_sphere;
use super::Colour;
use super::Float;
use super::HitRecord;
use super::Material;
use super::Ray;
use super::RcMaterial;
use super::ScatterResult;
use std::rc::Rc;

#[derive(Clone)]
pub struct Metal {
    albedo: Colour,
    fuzz: Float,
}

impl Metal {
    pub fn new(albedo: Colour, fuzz: Float) -> RcMaterial {
        Rc::new(Metal { albedo, fuzz })
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let reflected = ray_in.direction.unit_vector().reflect(rec.normal);
        if reflected.dot(rec.normal) > 0.0 {
            let direction = reflected + random_in_unit_sphere() * self.fuzz;
            Some(ScatterResult {
                scattered: Ray::new(rec.point, direction),
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}
