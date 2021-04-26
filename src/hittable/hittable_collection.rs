use std::ops::RangeInclusive;
use std::rc::Rc;

use super::{HitRecord, Hittable};
use crate::ray::Ray;

/// Stores a list of `hittable::Hittable`.
pub struct HittableCollection {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableCollection {
    /// Constructs a new empty `HittableCollection`.
    pub fn new() -> Self {
        HittableCollection {
            objects: Vec::new(),
        }
    }

    /// Adds an element to the collection.
    pub fn add(&mut self, elem: Rc<dyn Hittable>) {
        self.objects.push(elem);
    }

    /// Clears the collection, removing all hittables.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableCollection {
    /// Tries to hit an hittable in the collection with a ray. The point of
    /// intersection must be in the `valid_range`.
    /// Returns a record of the closest hit (the distance from the ray origin's
    /// to the point of intersection), or [`None`] if no hittable can be hit.
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
