use crate::error::Result;
use image::{DynamicImage, Rgba};

/// Rotate an image by the specified degrees (counter-clockwise)
pub fn rotate(
    img: &DynamicImage,
    degrees: f64,
    expand: bool,
    background: Rgba<u8>,
) -> Result<DynamicImage> {
    // Normalize degrees to 0-360 range
    let normalized = ((degrees % 360.0) + 360.0) % 360.0;

    // For exact 90-degree increments, use lossless rotation
    if (normalized - 0.0).abs() < 0.001 {
        return Ok(img.clone());
    } else if (normalized - 90.0).abs() < 0.001 {
        return Ok(img.rotate90());
    } else if (normalized - 180.0).abs() < 0.001 {
        return Ok(img.rotate180());
    } else if (normalized - 270.0).abs() < 0.001 {
        return Ok(img.rotate270());
    }

    // For arbitrary angles, use imageproc rotation
    let rgba_img = img.to_rgba8();

    if expand {
        // Calculate new dimensions to fit the rotated image
        let radians = normalized.to_radians();
        let cos = radians.cos().abs();
        let sin = radians.sin().abs();

        let old_width = rgba_img.width() as f64;
        let old_height = rgba_img.height() as f64;

        let new_width = (old_width * cos + old_height * sin).ceil() as u32;
        let new_height = (old_width * sin + old_height * cos).ceil() as u32;

        // Create a new larger canvas
        let mut canvas = image::RgbaImage::from_pixel(new_width, new_height, background);

        // Calculate offset to center the original image on the canvas
        let offset_x = ((new_width as f64 - old_width) / 2.0) as i64;
        let offset_y = ((new_height as f64 - old_height) / 2.0) as i64;

        // Copy original image to canvas center
        for (x, y, pixel) in rgba_img.enumerate_pixels() {
            let new_x = x as i64 + offset_x;
            let new_y = y as i64 + offset_y;
            if new_x >= 0 && new_x < new_width as i64 && new_y >= 0 && new_y < new_height as i64 {
                canvas.put_pixel(new_x as u32, new_y as u32, *pixel);
            }
        }

        // Rotate around new center
        let rotated = imageproc::geometric_transformations::rotate_about_center(
            &canvas,
            -radians as f32, // Negative because we want counter-clockwise
            imageproc::geometric_transformations::Interpolation::Bilinear,
            background,
        );

        Ok(DynamicImage::ImageRgba8(rotated))
    } else {
        // Rotate without expanding - clips to original size
        let radians = normalized.to_radians();

        let rotated = imageproc::geometric_transformations::rotate_about_center(
            &rgba_img,
            -radians as f32,
            imageproc::geometric_transformations::Interpolation::Bilinear,
            background,
        );

        Ok(DynamicImage::ImageRgba8(rotated))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::ImageBuffer;

    fn create_test_image(width: u32, height: u32) -> DynamicImage {
        let img = ImageBuffer::from_fn(width, height, |x, y| Rgba([x as u8, y as u8, 128, 255]));
        DynamicImage::ImageRgba8(img)
    }

    #[test]
    fn test_rotate_0_degrees() {
        let img = create_test_image(100, 100);
        let result = rotate(&img, 0.0, false, Rgba([0, 0, 0, 0])).unwrap();
        assert_eq!(result.width(), 100);
        assert_eq!(result.height(), 100);
    }

    #[test]
    fn test_rotate_90_degrees() {
        let img = create_test_image(100, 50);
        let result = rotate(&img, 90.0, false, Rgba([0, 0, 0, 0])).unwrap();
        // After 90 degree rotation, dimensions swap
        assert_eq!(result.width(), 50);
        assert_eq!(result.height(), 100);
    }

    #[test]
    fn test_rotate_180_degrees() {
        let img = create_test_image(100, 50);
        let result = rotate(&img, 180.0, false, Rgba([0, 0, 0, 0])).unwrap();
        assert_eq!(result.width(), 100);
        assert_eq!(result.height(), 50);
    }

    #[test]
    fn test_rotate_270_degrees() {
        let img = create_test_image(100, 50);
        let result = rotate(&img, 270.0, false, Rgba([0, 0, 0, 0])).unwrap();
        assert_eq!(result.width(), 50);
        assert_eq!(result.height(), 100);
    }

    #[test]
    fn test_rotate_negative_90() {
        let img = create_test_image(100, 50);
        let result = rotate(&img, -90.0, false, Rgba([0, 0, 0, 0])).unwrap();
        // -90 is same as 270
        assert_eq!(result.width(), 50);
        assert_eq!(result.height(), 100);
    }

    #[test]
    fn test_rotate_360_degrees() {
        let img = create_test_image(100, 100);
        let result = rotate(&img, 360.0, false, Rgba([0, 0, 0, 0])).unwrap();
        assert_eq!(result.width(), 100);
        assert_eq!(result.height(), 100);
    }

    #[test]
    fn test_rotate_45_degrees_no_expand() {
        let img = create_test_image(100, 100);
        let result = rotate(&img, 45.0, false, Rgba([0, 0, 0, 0])).unwrap();
        // Without expand, dimensions stay the same
        assert_eq!(result.width(), 100);
        assert_eq!(result.height(), 100);
    }

    #[test]
    fn test_rotate_45_degrees_with_expand() {
        let img = create_test_image(100, 100);
        let result = rotate(&img, 45.0, true, Rgba([0, 0, 0, 0])).unwrap();
        // With expand, dimensions should be larger (approximately sqrt(2) * 100)
        assert!(result.width() > 100);
        assert!(result.height() > 100);
    }
}
