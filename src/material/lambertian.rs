use super::Material;

use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    /// Constructs a new Lambertian (diffuse) material.
    pub fn new(color: Color) -> Lambertian {
        Lambertian { albedo: color }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = record.normal + Vec3::random_normalized();

        if scatter_direction.is_near_zero() {
            scatter_direction = record.normal;
        }

        Some((
            self.albedo,
            Ray::new(record.intersection_point, scatter_direction),
        ))
    }
}
