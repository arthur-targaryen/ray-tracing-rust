use std::{ops::RangeInclusive, sync::Arc};

pub use hittable_collection::HittableCollection;
pub use sphere::Sphere;

use crate::{
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

mod hittable_collection;
mod sphere;

/// An `HitRecord` is the result of a [`Ray`] hitting an [`Hittable`].
pub struct HitRecord<'a> {
    /// The point where the intersecting ray meets the hittable.
    pub intersection_point: Point3,
    /// The distance between the point of intersection and the intersecting ray
    /// origin.
    pub t: f64,
    /// A vector that is perpendicular to the surface at the point of
    /// intersection.
    pub normal: Vec3,
    /// Whether the intersecting ray met the hittable from the outside (i.e.
    /// [`HitRecord::front_face`] is `true`) or the inside.
    pub front_face: bool,
    /// The material of the hit face.
    pub material: Arc<dyn Material + Sync + Send + 'a>,
}

impl<'a> HitRecord<'a> {
    /// Constructs a new `HitRecord`.
    ///
    /// The record is the intersection between the `hitting_ray`, at `t` from
    /// the `hitting_ray` origin.
    /// `outward_normal` is a vector that is perpendicular to the surface at the
    /// point of intersection. In our case, it always point “outward” from the
    /// surface of the `Hittable`.
    /// `material` is the material hit by the ray.
    pub fn new(
        hitting_ray: &Ray,
        t: f64,
        outward_normal: Vec3,
        material: Arc<dyn Material + Sync + Send + 'a>,
    ) -> HitRecord<'a> {
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
            material,
        }
    }
}

/// An object that can be hit.
pub trait Hittable {
    /// Tries to hit an object with a ray. The intersection point must be in the
    /// `valid_range`.
    fn try_hit(&self, r: &Ray, valid_range: RangeInclusive<f64>) -> Option<HitRecord>;
}
