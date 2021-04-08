use std::io;

use ray_tracing::color::Color;

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.25;

            let pxl_color = Color::new(r, g, b);
            pxl_color
                .write(io::stdout())
                .expect("There was an error trying to write the image to the standard output");
        }
    }

    eprintln!("\nDone!");
}
