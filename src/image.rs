use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::Hittable;
use crate::perf::ThreadPool;
use crate::random;
use std::io::Write;

use std::sync::{Arc, Mutex};

use progressing::{mapping::Bar as MappingBar, Baring};

pub struct Image {
    camera: Arc<Camera>,
    image_width: usize,
    image_height: usize,
    samples_per_pixel: usize,
    max_depth: usize,
    world: Arc<dyn Hittable + Send + Sync>,
    pixels: Vec<Color>,
}

impl Image {
    pub fn new(
        camera: Camera,
        aspect_ratio: f64,
        image_width: usize,
        samples_per_pixel: usize,
        max_depth: usize,
        world: Arc<dyn Hittable + Send + Sync>,
    ) -> Image {
        let image_height = (image_width as f64 / aspect_ratio as f64) as usize;
        Image {
            camera: Arc::new(camera),
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
            world,
            pixels: vec![Color::zero(); image_width * image_height],
        }
    }

    pub fn render(&mut self, threads: usize) -> &Self {
        let mut pool = ThreadPool::new(threads);

        let pixels = Arc::new(Mutex::new(self.pixels.clone()));

        let mut progress_bar = MappingBar::with_range(0, self.image_height).timed();
        progress_bar.set_len(20);
        eprintln!("{}", progress_bar);
        let progress_bar = Arc::new(Mutex::new(progress_bar));

        for j in (0..self.image_height).rev() {
            let progress_bar = Arc::clone(&progress_bar);
            let world = Arc::clone(&self.world);
            let pixels = Arc::clone(&pixels);
            let camera = Arc::clone(&self.camera);

            let image_width = self.image_width;
            let image_height = self.image_height;
            let max_depth = self.max_depth;
            let samples_per_pixel = self.samples_per_pixel;

            pool.execute(move || {
                let mut chunk = Vec::with_capacity(image_width);
                for i in 0..image_width {
                    let mut pixel_color = Color::zero();

                    for _ in 0..samples_per_pixel {
                        let u = (i as f64 + random()) / (image_width - 1) as f64;
                        let v = (j as f64 + random()) / (image_height - 1) as f64;
                        let r = camera.ray_to(u, v);
                        pixel_color += r.color(&world, max_depth);
                    }

                    chunk.push(pixel_color);
                }

                let mut image = pixels.lock().unwrap();
                chunk.into_iter().enumerate().for_each(|(index, pixel)| {
                    image[((image_height - j - 1) * image_width + index)] = pixel;
                });

                let mut progress_bar = progress_bar.lock().unwrap();
                progress_bar.add(true);
                if progress_bar.has_progressed_significantly() {
                    progress_bar.remember_significant_progress();
                    eprintln!("{}", progress_bar);
                }
            });
        }

        pool.wait_all_jobs();

        self.pixels = pixels.lock().unwrap().clone();

        self
    }

    pub fn write(&self, stream: &mut dyn Write) -> std::io::Result<()> {
        eprintln!("\nWritting image...");

        writeln!(
            stream,
            "P3\n{} {}\n255",
            self.image_width, self.image_height
        )?;

        for pixel in &self.pixels {
            pixel.write(stream, self.samples_per_pixel)?;
        }

        eprintln!("Done!");

        std::io::Result::Ok(())
    }
}