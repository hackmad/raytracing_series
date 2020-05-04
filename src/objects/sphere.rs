use super::Float;
use super::HitRecord;
use super::Hittable;
use super::Ray;
use super::Vec3;

#[derive(Copy, Clone)]
pub struct Sphere {
    center: Vec3,
    radius: Float,
}

impl Sphere {
    pub fn new(center: Vec3, radius: Float) -> Sphere {
        Sphere { center, radius }
    }

    fn get_hit_record(self, ray: Ray, t: Float) -> HitRecord {
        let point = ray.at(t);
        let outward_normal = (point - self.center) / self.radius;
        HitRecord::new(ray, t, point, outward_normal)
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
                return Some(self.get_hit_record(ray, temp));
            }

            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                return Some(self.get_hit_record(ray, temp));
            }
        }

        None
    }
}
