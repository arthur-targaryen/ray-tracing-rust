use std::io;
use std::rc::Rc;

use camera::Camera;
use color::Color;
use hittable::{HittableCollection, Sphere};
use material::{Dielectrics, Lambertian, Material, Metal};
use random::*;
use vec3::Point3;

mod camera;
mod color;
mod hittable;
mod material;
mod random;
mod ray;
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

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Dielectrics::new(1.5));
    let material_left = Rc::new(Dielectrics::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::clone(&material_ground) as Rc<dyn Material>,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        Rc::clone(&material_center) as Rc<dyn Material>,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Rc::clone(&material_left) as Rc<dyn Material>,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Rc::clone(&material_right) as Rc<dyn Material>,
    )));

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
