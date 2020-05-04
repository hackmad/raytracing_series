use super::random_unit_vec3;
use super::Colour;
use super::HitRecord;
use super::Material;
use super::Ray;
use super::ScatterResult;

#[derive(Clone)]
pub struct Lambertian {
    albedo: Colour,
}

impl Lambertian {
    pub fn new(albedo: Colour) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: Ray, rec: HitRecord) -> Option<ScatterResult> {
        let scatter_direction = rec.normal + random_unit_vec3();
        Some(ScatterResult {
            scattered: Ray::new(rec.point, scatter_direction),
            attenuation: self.albedo,
        })
    }
}
