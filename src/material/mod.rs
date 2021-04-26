mod lambertian;
mod metal;

pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Color, Ray)>;
}
