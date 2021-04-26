pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::{color::Color, hittable::HitRecord, ray::Ray};

mod lambertian;
mod metal;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Color, Ray)>;
}
