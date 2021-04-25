use std::io::Write;

use super::vec3;

/// Aliases of [`vec3::Vec3`] representing an RGB color.
pub type Color = vec3::Vec3;

impl Color {
    /// Write a single pixel's color out to a writer.
    ///
    /// The color is the sum of multiple samples, thus this function will
    /// scale the color and clamp it to an RGB value.
    pub fn write(&self, mut stream: impl Write, samples_per_pixel: u32) -> std::io::Result<()> {
        let mut r = self.x();
        let mut g = self.y();
        let mut b = self.z();

        // Divide the color by the number of samples.
        let scale = 1.0 / samples_per_pixel as f64;
        r *= scale;
        g *= scale;
        b *= scale;

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
