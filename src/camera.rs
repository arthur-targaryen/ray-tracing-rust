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
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        vertical_fov: f64,
        aspect_ratio: f64,
    ) -> Camera {
        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normalized();
        let u = vup.cross(&w).normalized();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        Camera {
            origin,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - w,
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
