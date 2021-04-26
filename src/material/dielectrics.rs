use crate::{color::Color, hittable::HitRecord, material::Material, random, ray::Ray};

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

        let cannot_recraft = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_recraft || reflectance(cos_theta, refraction_ratio) > random::random() {
                unit_direction.reflected(&record.normal)
            } else {
                unit_direction.refracted(&record.normal, refraction_ratio)
            };

        Some((
            Color::new(1.0, 1.0, 1.0),
            Ray::new(record.intersection_point, direction),
        ))
    }
}

/// See [Schlick's approximation on Wikipedia](https://en.wikipedia.org/wiki/Schlick%27s_approximation).
fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
