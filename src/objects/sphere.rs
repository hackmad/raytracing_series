use super::Float;
use super::HitRecord;
use super::Hittable;
use super::Material;
use super::Ray;
use super::Vec3;

#[derive(Clone)]
pub struct Sphere {
    center: Vec3,
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
                let point = ray.at(temp);
                let outward_normal = (point - self.center) / self.radius;
                return Some(HitRecord::new(
                    ray,
                    temp,
                    point,
                    outward_normal,
                    self.material.clone(),
                ));
            }

            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                let point = ray.at(temp);
                let outward_normal = (point - self.center) / self.radius;
                return Some(HitRecord::new(
                    ray,
                    temp,
                    point,
                    outward_normal,
                    self.material.clone(),
                ));
            }
        }

        None
    }
}
