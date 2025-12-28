use image::{ImageBuffer, Rgba, RgbaImage};
use std::path::Path;

/// Create a simple test RGBA image with a gradient pattern
pub fn create_test_rgba_image(width: u32, height: u32) -> RgbaImage {
    ImageBuffer::from_fn(width, height, |x, y| {
        let r = (x * 255 / width.max(1)) as u8;
        let g = (y * 255 / height.max(1)) as u8;
        let b = 128u8;
        let a = 255u8;
        Rgba([r, g, b, a])
    })
}

/// Create a test RGB image (no alpha)
pub fn create_test_rgb_image(width: u32, height: u32) -> image::RgbImage {
    image::ImageBuffer::from_fn(width, height, |x, y| {
        let r = (x * 255 / width.max(1)) as u8;
        let g = (y * 255 / height.max(1)) as u8;
        let b = 128u8;
        image::Rgb([r, g, b])
    })
}

/// Create a test grayscale image
pub fn create_test_gray_image(width: u32, height: u32) -> image::GrayImage {
    image::ImageBuffer::from_fn(width, height, |x, y| {
        let val = ((x + y) * 255 / (width + height).max(1)) as u8;
        image::Luma([val])
    })
}

/// Save a test image and return the path
pub fn save_test_image<P: AsRef<Path>>(img: &RgbaImage, path: P) -> Result<(), image::ImageError> {
    img.save(path)
}
