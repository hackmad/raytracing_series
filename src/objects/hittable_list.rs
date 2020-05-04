use super::Float;
use super::HitRecord;
use super::Hittable;
use super::Ray;

#[derive(Clone)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn new_from_object(object: Box<dyn Hittable>) -> HittableList {
        HittableList {
            objects: vec![object],
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let init: (Option<HitRecord>, Float) = (None, t_max);

        let result = self.objects.iter().fold(init, |acc, hittable| {
            match hittable.hit(ray, t_min, acc.1) {
                Some(HitRecord {
                    t,
                    point,
                    normal,
                    front_face,
                    material,
                }) => (
                    Some(HitRecord {
                        t,
                        point,
                        normal,
                        front_face,
                        material,
                    }),
                    t,
                ),
                None => acc,
            }
        });

        result.0
    }
}
