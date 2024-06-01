use crate::vector::Vector;
use image::{ImageBuffer, Rgb, RgbImage};

pub fn write(path: &str, width: u32, height: u32, pixels: &[Vector]) {
    let mut buffer: RgbImage = ImageBuffer::new(width, height);

    for ((_, _, dst_pixel), src_pixel) in buffer.enumerate_pixels_mut().zip(pixels.iter()) {
        let p = src_pixel
            .coords
            .iter()
            .map(|c| c.max(0.0).min(1.0) * 255.0)
            .collect::<Vec<_>>();
        *dst_pixel = Rgb([p[0] as u8, p[1] as u8, p[2] as u8]);
    }

    buffer.save(path).unwrap();
}
