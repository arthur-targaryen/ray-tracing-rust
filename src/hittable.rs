use std::ops::RangeInclusive;

use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Debug)]
pub struct HitRecord {
    pub intersection_point: Point3,
    pub t: f64,
    pub normal: Vec3,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(hitting_ray: &Ray, t: f64, outward_normal: Vec3) -> HitRecord {
        let front_face = hitting_ray.direction().dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        HitRecord {
            intersection_point: hitting_ray.at(t),
            normal,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn try_hit(&self, r: &Ray, valid_range: RangeInclusive<f64>) -> Option<HitRecord>;
}
