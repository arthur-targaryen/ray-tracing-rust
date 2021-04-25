use std::io;
use std::rc::Rc;

use camera::Camera;
use color::Color;
use random::*;
use vec3::Point3;

use crate::hittable_collection::HittableCollection;
use crate::sphere::Sphere;

mod camera;
mod color;
mod hittable;
mod hittable_collection;
mod random;
mod ray;
mod sphere;
mod vec3;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel: u32 = 100;
    let max_depth = 50;

    // World
    let mut world = HittableCollection::new();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::new();

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let mut pixel_color = Color::zero();

            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random()) / (image_width - 1) as f64;
                let v = (j as f64 + random()) / (image_height - 1) as f64;
                let r = camera.ray_to(u, v);
                pixel_color += r.color(&world, max_depth);
            }

            pixel_color
                .write(io::stdout(), samples_per_pixel)
                .expect("There was an error trying to write the image to the standard output");
        }
    }

    eprintln!("\nDone!");
}
