use std::io::Write;

use crate::vec3;

/// Aliases of [`vec3::Vec3`] representing an RGB color.
pub type Color = vec3::Vec3;

impl Color {
    /// Write a single pixel's color out to a writer.
    ///
    /// The color is the sum of multiple samples, thus this function will
    /// scale the color and clamp it to an RGB value.
    pub fn write(&self, stream: &mut dyn Write, samples_per_pixel: usize) -> std::io::Result<()> {
        let r = self.x();
        let g = self.y();
        let b = self.z();

        // Divide the color by the number of samples and gamma-correct for
        // gamma = 2.0.
        let scale = 1.0 / samples_per_pixel as f64;
        let r = (scale * r).sqrt();
        let g = (scale * g).sqrt();
        let b = (scale * b).sqrt();

        // Write the translated [0; 255] value of each color component.
        writeln!(
            stream,
            "{} {} {}",
            (256.0 * r.clamp(0.0, 0.999)) as i32,
            (256.0 * g.clamp(0.0, 0.999)) as i32,
            (256.0 * b.clamp(0.0, 0.999)) as i32,
        )
    }
}
