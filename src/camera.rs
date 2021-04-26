use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

/// The ray tracer camera.
///
/// All ray are sent from the camera origin.
#[allow(dead_code)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
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
        aperture: f64,
        focus_distance: f64,
    ) -> Camera {
        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normalized();
        let u = vup.cross(&w).normalized();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;
        Camera {
            origin,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - focus_distance * w,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
        }
    }

    /// Return a ray going from the camera origin to the X and Y coordinates
    /// represented by `s` and `t`.
    ///
    /// `s` and `t` are two coefficient making two offset vectors along the
    /// screen sides to move the ray endpoint across the screen.
    pub fn ray_to(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
