use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;

/// A clear material (such as water, glass or diamonds).
/// When a ray hits it, it splits into a reflected ray and a refracted ray.
pub struct Dielectrics {
    refraction_index: f64,
}

impl Dielectrics {
    /// Constructs a new Dielectrics material.
    pub fn new(refraction_index: f64) -> Self {
        Dielectrics { refraction_index }
    }
}

impl Material for Dielectrics {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray_in.direction().normalized();

        let cos_theta = -unit_direction.dot(&record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let can_recraft = refraction_ratio * sin_theta <= 1.0;
        let direction = if can_recraft {
            unit_direction.refracted(&record.normal, refraction_ratio)
        } else {
            unit_direction.reflected(&record.normal)
        };

        Some((
            Color::new(1.0, 1.0, 1.0),
            Ray::new(record.intersection_point, direction),
        ))
    }
}
