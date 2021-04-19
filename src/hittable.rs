use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Debug)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -(*outward_normal)
        }
    }
}

pub trait Hittable {
    // TODO: Use some kind of range for t_min and t_max
    fn try_hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
