use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct HittableCollection {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableCollection {
    pub fn new() -> Self {
        HittableCollection {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, elem: Rc<dyn Hittable>) {
        self.objects.push(elem);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableCollection {
    fn try_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut record: Option<HitRecord> = None;
        let mut closest = t_max;

        for object in &self.objects {
            if let Some(hit) = object.try_hit(ray, t_min, closest) {
                closest = hit.t;
                record = Some(hit);
            }
        }

        record
    }
}
