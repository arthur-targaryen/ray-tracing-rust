use std::ops::RangeInclusive;
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
    fn try_hit(&self, ray: &Ray, valid_range: RangeInclusive<f64>) -> Option<HitRecord> {
        let mut record: Option<HitRecord> = None;
        let mut closest = *valid_range.end();

        for object in &self.objects {
            if let Some(hit) = object.try_hit(ray, *valid_range.start()..=closest) {
                closest = hit.t;
                record = Some(hit);
            }
        }

        record
    }
}
