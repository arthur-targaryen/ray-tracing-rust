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

    #[allow(dead_code)]
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    fn hit_sphere(&self, center: &Point3, radius: f64) -> bool {
        let oc = self.origin() - *center;

        let a = self.direction.dot(&self.direction);
        let b = 2.0 * oc.dot(&self.direction);
        let c = oc.dot(&oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;

        discriminant > 0.0
    }

    pub fn color(&self) -> Color {
        if self.hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5) {
            return Color::new(1.0, 0.0, 0.0);
        }

        let normalized_direction = self.direction.normalized();
        let t = 0.5 * (normalized_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(1.0, 0.7, 0.5)
    }
}
