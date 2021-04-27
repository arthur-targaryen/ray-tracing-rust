use std::{f64, io, rc::Rc};

use camera::Camera;
use color::Color;
use hittable::{HittableCollection, Sphere};
use material::{Dielectrics, Lambertian, Material, Metal};
use random::*;
use vec3::{Point3, Vec3};

mod camera;
mod color;
mod hittable;
mod material;
mod random;
mod ray;
mod vec3;

fn random_scene() -> HittableCollection {
    let mut world = HittableCollection::new();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::clone(&ground_material) as Rc<dyn Material>,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random();
            let center = Point3::new(a as f64 + 0.9 * random(), 0.2, b as f64 + 0.9 * random());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material = if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    Rc::new(Lambertian::new(albedo)) as Rc<dyn Material>
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5..1.0);
                    let fuzz = random_range(0.0..0.5);
                    Rc::new(Metal::new(albedo, fuzz)) as Rc<dyn Material>
                } else {
                    Rc::new(Dielectrics::new(1.5)) as Rc<dyn Material>
                };

                world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material = Rc::new(Dielectrics::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material,
    )));

    let material = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material,
    )));

    let material = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material,
    )));

    world
}

fn main() {
    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel: u32 = 500;
    let max_depth = 50;

    // World
    let world = random_scene();

    // Camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::zero();
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

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
