use std::{ops::RangeInclusive, sync::Arc};

use crate::{material::Material, ray::Ray, vec3::Point3};

use super::{HitRecord, Hittable};

pub struct Sphere {
    center: Point3,
    radius: f64,
    pub material: Arc<dyn Material + Sync + Send + 'static>,
}

impl Sphere {
    /// Constructs a new `Sphere`.
    pub fn new(
        center: Point3,
        radius: f64,
        material: Arc<dyn Material + Sync + Send + 'static>,
    ) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    /// Tries to hit a sphere with a ray. The intersection point must be in the
    /// `valid_range`.
    /// See [Line-sphere intersection on Wikipedia](https://en.wikipedia.org/wiki/Line–sphere_intersection).
    #[allow(clippy::suspicious_operation_groupings)]
    fn try_hit(&self, ray: &Ray, valid_range: RangeInclusive<f64>) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;

        let a = ray.direction().length_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = (half_b * half_b) - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if !valid_range.contains(&root) {
            root = (-half_b + sqrtd) / a;
            if !valid_range.contains(&root) {
                return None;
            }
        }

        Some(HitRecord::new(
            ray,
            root,
            (ray.at(root) - self.center) / self.radius,
            Arc::clone(&self.material),
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::color::Color;
    use crate::material::Lambertian;
    use crate::vec3::Vec3;

    use super::*;

    #[test]
    fn try_hit_sphere_with_not_hitting_ray() {
        let material = Arc::new(Lambertian::new(Color::zero()));
        let sphere = Sphere::new(
            Point3::new(1.0, 1.0, 1.0),
            0.5,
            Arc::clone(&material) as Arc<dyn Material + Send + Sync>,
        );
        let ray = Ray::new(Point3::new(-1.0, -1.0, -1.0), Vec3::new(-1.0, -1.0, -1.0));

        assert!(sphere.try_hit(&ray, 0.0..=f64::INFINITY).is_none());
    }

    #[test]
    fn try_hit_sphere_with_hitting_ray_not_in_range() {
        let material = Arc::new(Lambertian::new(Color::zero()));
        let sphere = Sphere::new(
            Point3::new(3.0, 0.0, 0.0),
            1.0,
            Arc::clone(&material) as Arc<dyn Material + Send + Sync>,
        );
        let ray = Ray::new(Point3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));

        assert!(sphere.try_hit(&ray, 0.0..=0.5).is_none());
    }

    #[test]
    fn try_hit_sphere_with_hitting_ray_in_range() {
        let material = Arc::new(Lambertian::new(Color::zero()));
        let sphere = Sphere::new(
            Point3::new(3.0, 0.0, 0.0),
            1.0,
            Arc::clone(&material) as Arc<dyn Material + Send + Sync>,
        );
        let ray = Ray::new(Point3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));

        let result = sphere
            .try_hit(&ray, 0.0..=2.0)
            .expect("ray should hit sphere");

        assert_eq!(result.intersection_point, Point3::new(2.0, 0.0, 0.0));
        assert_eq!(result.t, 1.0);
        assert_eq!(result.normal, Vec3::new(-1.0, 0.0, 0.0));
        assert!(result.front_face);
    }
}
