use super::Colour;
use super::HitRecord;
use super::Material;
use super::Ray;
use super::ScatterResult;

#[derive(Clone)]
pub struct Metal {
    albedo: Colour,
}

impl Metal {
    pub fn new(albedo: Colour) -> Metal {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: Ray, rec: HitRecord) -> Option<ScatterResult> {
        let reflected = ray_in.direction.unit_vector().reflect(rec.normal);
        if reflected.dot(rec.normal) > 0.0 {
            Some(ScatterResult {
                scattered: Ray::new(rec.point, reflected),
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}
