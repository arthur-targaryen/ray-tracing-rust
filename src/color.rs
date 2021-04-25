use std::io::Write;

use super::vec3;

/// Aliases of [`vec3::Vec3`] representing an RGB color.
pub type Color = vec3::Vec3;

impl Color {
    /// Write a single pixel's color out to a writer.
    pub fn write(&self, mut stream: impl Write) -> std::io::Result<()> {
        writeln!(
            stream,
            "{} {} {}",
            (255.999 * self.x()) as i32,
            (255.999 * self.y()) as i32,
            (255.999 * self.z()) as i32,
        )
    }
}
