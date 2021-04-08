use super::vec3;

pub type Color = vec3::Vec3;

pub fn write_color(&pxl_color: &Color) {
    println!(
        "{} {} {}",
        (255.999 * pxl_color.x()) as i32,
        (255.999 * pxl_color.y()) as i32,
        (255.999 * pxl_color.z()) as i32
    );
}
