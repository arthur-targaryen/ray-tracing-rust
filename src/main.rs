use std::{
    f64,
    io::{self, Write},
    sync::Arc,
};

use camera::Camera;
use color::Color;
use hittable::{Hittable, HittableCollection, Sphere};
use image::Image;
use material::{Dielectrics, Lambertian, Material, Metal};
use random::*;
use vec3::{Point3, Vec3};

mod camera;
mod color;
mod hittable;
mod image;
mod material;
mod perf;
mod random;
mod ray;
mod vec3;

/// Creates a random scene. Returns an [`Arc`] of [`Hittable`].
///
/// The scene contains multiple sheres of multiple materials (glass, metal and
/// diffuse).
///
/// The scene come from [chapter 13](https://raytracing.github.io/books/RayTracingInOneWeekend.html#wherenext?/afinalrender)
/// of *Ray Tracing in One Weekend*.
fn random_scene() -> Arc<dyn Hittable + Sync + Send> {
    let mut world = HittableCollection::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::clone(&ground_material) as Arc<dyn Material + Sync + Send>,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random();
            let center = Point3::new(a as f64 + 0.9 * random(), 0.2, b as f64 + 0.9 * random());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material = if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    Arc::new(Lambertian::new(albedo)) as Arc<dyn Material + Sync + Send>
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5..1.0);
                    let fuzz = random_range(0.0..0.5);
                    Arc::new(Metal::new(albedo, fuzz)) as Arc<dyn Material + Sync + Send>
                } else {
                    Arc::new(Dielectrics::new(1.5)) as Arc<dyn Material + Sync + Send>
                };

                world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material = Arc::new(Dielectrics::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material,
    )));

    let material = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material,
    )));

    let material = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material,
    )));

    Arc::new(world)
}

fn main() {
    // World
    let world = random_scene();

    // Camera
    let aspect_ratio = 3.0 / 2.0;
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::zero();
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let fov = 20.0;
    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        fov,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    // Image
    let image_width = 1200;
    let samples_per_pixel = 500;
    let max_depth = 50;
    let mut image = Image::new(
        camera,
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        world,
    );

    // Render
    image
        .render(10)
        .write(&mut io::stdout() as &mut dyn Write)
        .expect("There was an error trying to write the image to the standard output");
}
