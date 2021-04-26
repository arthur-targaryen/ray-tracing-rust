use crate::color::Color;
use crate::hittable::Hittable;
use crate::vec3::{Point3, Vec3};

#[derive(Debug)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    /// Constructs a new `Ray`.
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    /// Returns the origin of the vector.
    pub fn origin(&self) -> Point3 {
        self.origin
    }

    /// Returns the direction of the vector.
    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    /// Returns the 3D position along the vector ; `t` is the distance from the
    /// [`origin`].
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    /// Computes the color seen along a ray.
    ///
    /// This will try to hit anything in the `world`.
    /// If nothing can be hit, returns a blue-to-white gradient depending on ray
    /// Y coordinate.
    pub fn color(&self, world: &dyn Hittable, depth: u32) -> Color {
        if depth == 0 {
            return Color::zero();
        }

        if let Some(hit) = world.try_hit(self, 0.001..=f64::INFINITY) {
            if let Some((attenuation, scattered)) = hit.material.scatter(self, &hit) {
                return attenuation * scattered.color(world, depth - 1);
            }

            return Color::zero();
        }

        let unit_direction = self.direction().normalized();
        let t = 0.5 * (unit_direction.y() + 1.0);

        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}
