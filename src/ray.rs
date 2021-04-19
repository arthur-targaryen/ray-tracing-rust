use crate::color::Color;
use crate::vec3::{Point3, Vec3};

#[derive(Debug)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    #[allow(dead_code)]
    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    fn hit_sphere(&self, center: &Point3, radius: f64) -> f64 {
        let oc = self.origin() - *center;

        let a = self.direction.length_squared();
        let half_b = oc.dot(&self.direction);
        let c = oc.length_squared() - radius * radius;
        let discriminant = (half_b * half_b) - a * c;

        if discriminant < 0.0 {
            -1.0
        } else {
            (-half_b - discriminant.sqrt()) / a
        }
    }

    pub fn color(&self) -> Color {
        let t = self.hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5);
        if t > 0.0 {
            let n = (self.at(t) - Vec3::new(0.0, 0.0, -1.0)).normalized();
            return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
        }

        let normalized_direction = self.direction.normalized();
        let t = 0.5 * (normalized_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}
