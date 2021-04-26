use super::material::Material;

use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;

pub struct Metal {
    pub albedo: Color,
}

impl Metal {
    /// Constructs a new Metal material.
    pub fn new(color: Color) -> Metal {
        Metal { albedo: color }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = ray_in.direction().normalized().reflected(&record.normal);

        let scattered = Ray::new(record.intersection_point, reflected);
        if scattered.direction().dot(&record.normal) > 0.0 {
            let attenuation = self.albedo;
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
