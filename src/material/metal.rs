use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

use super::Material;

/// A material that reflects an incoming ray.
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    /// Constructs a new Metal material.
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: fuzz.max(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = ray_in.direction().normalized().reflected(&record.normal);

        let scattered = Ray::new(
            record.intersection_point,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );
        if scattered.direction().dot(&record.normal) > 0.0 {
            let attenuation = self.albedo;
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
