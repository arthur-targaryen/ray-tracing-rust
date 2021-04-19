use std::ops::RangeInclusive;

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Point3;

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn try_hit(&self, r: &Ray, valid_range: RangeInclusive<f64>) -> Option<HitRecord> {
        let oc = r.origin() - self.center;

        let a = r.direction().length_squared();
        let half_b = oc.dot(&r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = (half_b * half_b) - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if !valid_range.contains(&root) {
            root = (-half_b + sqrtd) / a;
            if !valid_range.contains(&root) {
                return None;
            }
        }

        Some(HitRecord::new(
            root,
            (r.at(root) - self.center) / self.radius,
            r,
        ))
    }
}
