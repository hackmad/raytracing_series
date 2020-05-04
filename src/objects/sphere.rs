use super::Float;
use super::HitRecord;
use super::Hittable;
use super::Material;
use super::Point3;
use super::Ray;
use super::Vec3;

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: Float,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: Float, material: Box<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

fn get_hit_record(
    ray: Ray,
    t: Float,
    center: Point3,
    radius: Float,
    mat: Box<dyn Material>,
) -> HitRecord {
    let point = ray.at(t);
    let outward_normal = (point - center) / radius;
    HitRecord::new(ray, t, point, outward_normal, mat)
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                return Some(get_hit_record(
                    ray,
                    temp,
                    self.center,
                    self.radius,
                    self.material.clone(),
                ));
            }

            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                return Some(get_hit_record(
                    ray,
                    temp,
                    self.center,
                    self.radius,
                    self.material.clone(),
                ));
            }
        }

        None
    }
}
