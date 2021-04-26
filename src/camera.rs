use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

/// The ray tracer camera.
///
/// All ray are sent from the camera origin.
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    /// Constructs a new `Camera`.
    ///
    /// The aspect ratio is 16:9, the viewport height is 2.0 and the focal
    /// length is 1.0. Camera center is at the origin (0, 0, 0).
    pub fn new() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::zero();
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        Camera {
            origin,
            lower_left_corner: origin
                - horizontal / 2.0
                - vertical / 2.0
                - Vec3::new(0.0, 0.0, focal_length),
            horizontal,
            vertical,
        }
    }

    /// Return a ray going from the camera origin to the X and Y coordinates
    /// represented by `u` and `v`.
    ///
    /// `u` and `v` are two coefficient making two offset vectors along the
    /// screen sides to move the ray endpoint across the screen.
    pub fn ray_to(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
