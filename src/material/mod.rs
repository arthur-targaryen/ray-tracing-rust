pub use dielectrics::Dielectrics;
pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::{color::Color, hittable::HitRecord, ray::Ray};

mod dielectrics;
mod lambertian;
mod metal;

/// Object material.
pub trait Material {
    /// Returns the attenuation and scattered ray.
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Color, Ray)>;
}
