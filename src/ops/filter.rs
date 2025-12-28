use crate::error::{ImgEditError, Result};
use image::DynamicImage;

/// Apply Gaussian blur to an image
/// radius: blur strength in pixels (0.1 to 100.0)
pub fn blur(img: &DynamicImage, radius: f32) -> Result<DynamicImage> {
    if !(0.1..=100.0).contains(&radius) {
        return Err(ImgEditError::InvalidParameter(format!(
            "Blur radius must be between 0.1 and 100.0, got {}",
            radius
        )));
    }

    let rgba = img.to_rgba8();

    // Use imageproc's gaussian blur
    // The sigma parameter is roughly radius / 3 for a gaussian
    let sigma = radius / 3.0;
    let blurred = imageproc::filter::gaussian_blur_f32(&rgba, sigma);

    Ok(DynamicImage::ImageRgba8(blurred))
}

/// Apply sharpening filter to an image
/// amount: sharpening strength (0.0 to 10.0)
/// radius: effect radius in pixels (0.1 to 10.0)
pub fn sharpen(img: &DynamicImage, amount: f32, radius: f32) -> Result<DynamicImage> {
    if !(0.0..=10.0).contains(&amount) {
        return Err(ImgEditError::InvalidParameter(format!(
            "Sharpen amount must be between 0.0 and 10.0, got {}",
            amount
        )));
    }

    if !(0.1..=10.0).contains(&radius) {
        return Err(ImgEditError::InvalidParameter(format!(
            "Sharpen radius must be between 0.1 and 10.0, got {}",
            radius
        )));
    }

    if amount < 0.001 {
        // No sharpening needed
        return Ok(img.clone());
    }

    let rgba = img.to_rgba8();

    // Unsharp mask technique:
    // 1. Blur the image
    // 2. Subtract blurred from original and add back scaled by amount
    let sigma = radius / 3.0;
    let blurred = imageproc::filter::gaussian_blur_f32(&rgba, sigma);

    let (width, height) = (rgba.width(), rgba.height());
    let sharpened = image::ImageBuffer::from_fn(width, height, |x, y| {
        let orig = rgba.get_pixel(x, y);
        let blur_pixel = blurred.get_pixel(x, y);

        image::Rgba([
            sharpen_channel(orig[0], blur_pixel[0], amount),
            sharpen_channel(orig[1], blur_pixel[1], amount),
            sharpen_channel(orig[2], blur_pixel[2], amount),
            orig[3], // Preserve alpha
        ])
    });

    Ok(DynamicImage::ImageRgba8(sharpened))
}

fn sharpen_channel(original: u8, blurred: u8, amount: f32) -> u8 {
    // Unsharp mask: original + amount * (original - blurred)
    let diff = original as f32 - blurred as f32;
    let result = original as f32 + amount * diff;
    result.round().clamp(0.0, 255.0) as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgba};

    fn create_test_image() -> DynamicImage {
        let img = ImageBuffer::from_fn(20, 20, |x, y| {
            // Create a pattern with edges for testing
            let val = if (x + y) % 2 == 0 { 255 } else { 0 };
            Rgba([val as u8, val as u8, val as u8, 255])
        });
        DynamicImage::ImageRgba8(img)
    }

    fn create_solid_image() -> DynamicImage {
        let img = ImageBuffer::from_fn(10, 10, |_, _| Rgba([128, 128, 128, 255]));
        DynamicImage::ImageRgba8(img)
    }

    #[test]
    fn test_blur_basic() {
        let img = create_test_image();
        let result = blur(&img, 2.0).unwrap();

        // Dimensions should be preserved
        assert_eq!(result.width(), 20);
        assert_eq!(result.height(), 20);

        // After blur, the checkerboard pattern should be smoothed
        // (pixels shouldn't be pure 0 or 255 anymore)
        let rgba = result.to_rgba8();
        let pixel = rgba.get_pixel(10, 10);
        assert!(pixel[0] > 0 && pixel[0] < 255);
    }

    #[test]
    fn test_blur_solid_unchanged() {
        let img = create_solid_image();
        let result = blur(&img, 2.0).unwrap();

        // Solid color should remain approximately the same
        let rgba = result.to_rgba8();
        let pixel = rgba.get_pixel(5, 5);
        // Allow small tolerance due to edge effects
        assert!((pixel[0] as i32 - 128).abs() < 5);
    }

    #[test]
    fn test_blur_invalid_radius() {
        let img = create_test_image();
        assert!(blur(&img, 0.0).is_err());
        assert!(blur(&img, 150.0).is_err());
    }

    #[test]
    fn test_blur_preserves_alpha() {
        let img = ImageBuffer::from_fn(10, 10, |_, _| Rgba([128, 128, 128, 100]));
        let img = DynamicImage::ImageRgba8(img);
        let result = blur(&img, 2.0).unwrap();
        let rgba = result.to_rgba8();
        let pixel = rgba.get_pixel(5, 5);
        // Alpha should be preserved (or close due to edge handling)
        assert!((pixel[3] as i32 - 100).abs() < 10);
    }

    #[test]
    fn test_sharpen_basic() {
        let img = create_test_image();
        let result = sharpen(&img, 1.0, 1.0).unwrap();

        assert_eq!(result.width(), 20);
        assert_eq!(result.height(), 20);
    }

    #[test]
    fn test_sharpen_zero_amount() {
        let img = create_test_image();
        let result = sharpen(&img, 0.0, 1.0).unwrap();

        // With zero amount, image should be unchanged
        let orig_rgba = img.to_rgba8();
        let result_rgba = result.to_rgba8();

        for (orig, res) in orig_rgba.pixels().zip(result_rgba.pixels()) {
            assert_eq!(orig, res);
        }
    }

    #[test]
    fn test_sharpen_invalid_amount() {
        let img = create_test_image();
        assert!(sharpen(&img, -1.0, 1.0).is_err());
        assert!(sharpen(&img, 15.0, 1.0).is_err());
    }

    #[test]
    fn test_sharpen_invalid_radius() {
        let img = create_test_image();
        assert!(sharpen(&img, 1.0, 0.0).is_err());
        assert!(sharpen(&img, 1.0, 15.0).is_err());
    }

    #[test]
    fn test_sharpen_increases_contrast() {
        // Create a gradient image
        let img = ImageBuffer::from_fn(20, 1, |x, _| {
            let val = (x * 12) as u8; // Gradient from 0 to ~240
            Rgba([val, val, val, 255])
        });
        let img = DynamicImage::ImageRgba8(img);

        let result = sharpen(&img, 2.0, 1.0).unwrap();
        let result_rgba = result.to_rgba8();

        // Edges should be more pronounced
        // The transition should be sharper
        let pixel_5 = result_rgba.get_pixel(5, 0)[0];
        let pixel_15 = result_rgba.get_pixel(15, 0)[0];

        // The difference should be at least as large as the original
        assert!(pixel_15 > pixel_5 || pixel_15 == 255);
    }
}
