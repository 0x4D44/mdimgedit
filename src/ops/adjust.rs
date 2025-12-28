use crate::error::{ImgEditError, Result};
use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage};

/// Adjust the brightness of an image
/// value: -255 to 255 (0 = no change)
pub fn brightness(img: &DynamicImage, value: i32) -> Result<DynamicImage> {
    if !(-255..=255).contains(&value) {
        return Err(ImgEditError::InvalidParameter(format!(
            "Brightness value must be between -255 and 255, got {}",
            value
        )));
    }

    let rgba = img.to_rgba8();
    let (width, height) = (rgba.width(), rgba.height());

    let result: RgbaImage = ImageBuffer::from_fn(width, height, |x, y| {
        let pixel = rgba.get_pixel(x, y);
        Rgba([
            adjust_channel(pixel[0], value),
            adjust_channel(pixel[1], value),
            adjust_channel(pixel[2], value),
            pixel[3], // Preserve alpha
        ])
    });

    Ok(DynamicImage::ImageRgba8(result))
}

fn adjust_channel(value: u8, adjustment: i32) -> u8 {
    (value as i32 + adjustment).clamp(0, 255) as u8
}

/// Adjust the contrast of an image
/// value: 0.0 to 10.0 (1.0 = no change)
pub fn contrast(img: &DynamicImage, value: f64) -> Result<DynamicImage> {
    if !(0.0..=10.0).contains(&value) {
        return Err(ImgEditError::InvalidParameter(format!(
            "Contrast value must be between 0.0 and 10.0, got {}",
            value
        )));
    }

    let rgba = img.to_rgba8();
    let (width, height) = (rgba.width(), rgba.height());

    let result: RgbaImage = ImageBuffer::from_fn(width, height, |x, y| {
        let pixel = rgba.get_pixel(x, y);
        Rgba([
            contrast_channel(pixel[0], value),
            contrast_channel(pixel[1], value),
            contrast_channel(pixel[2], value),
            pixel[3], // Preserve alpha
        ])
    });

    Ok(DynamicImage::ImageRgba8(result))
}

fn contrast_channel(value: u8, factor: f64) -> u8 {
    // Contrast adjustment around midpoint (128)
    let adjusted = ((value as f64 - 128.0) * factor + 128.0).round();
    adjusted.clamp(0.0, 255.0) as u8
}

/// Apply gamma correction to an image
/// value: 0.1 to 10.0 (1.0 = no change)
pub fn gamma(img: &DynamicImage, value: f64) -> Result<DynamicImage> {
    if !(0.1..=10.0).contains(&value) {
        return Err(ImgEditError::InvalidParameter(format!(
            "Gamma value must be between 0.1 and 10.0, got {}",
            value
        )));
    }

    // Build a lookup table for efficiency
    // gamma < 1 lightens (raises dark values), gamma > 1 darkens (lowers mid values)
    let lut: Vec<u8> = (0..=255)
        .map(|i| {
            let normalized = i as f64 / 255.0;
            let corrected = normalized.powf(value);
            (corrected * 255.0).round().clamp(0.0, 255.0) as u8
        })
        .collect();

    let rgba = img.to_rgba8();
    let (width, height) = (rgba.width(), rgba.height());

    let result: RgbaImage = ImageBuffer::from_fn(width, height, |x, y| {
        let pixel = rgba.get_pixel(x, y);
        Rgba([
            lut[pixel[0] as usize],
            lut[pixel[1] as usize],
            lut[pixel[2] as usize],
            pixel[3], // Preserve alpha
        ])
    });

    Ok(DynamicImage::ImageRgba8(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_image() -> DynamicImage {
        let img = ImageBuffer::from_fn(10, 10, |_, _| Rgba([128, 128, 128, 255]));
        DynamicImage::ImageRgba8(img)
    }

    fn create_gray_image(value: u8) -> DynamicImage {
        let img = ImageBuffer::from_fn(1, 1, |_, _| Rgba([value, value, value, 255]));
        DynamicImage::ImageRgba8(img)
    }

    #[test]
    fn test_brightness_increase() {
        let img = create_gray_image(100);
        let result = brightness(&img, 50).unwrap();
        let rgba = result.to_rgba8();
        let pixel = rgba.get_pixel(0, 0);
        assert_eq!(pixel[0], 150);
    }

    #[test]
    fn test_brightness_decrease() {
        let img = create_gray_image(100);
        let result = brightness(&img, -50).unwrap();
        let rgba = result.to_rgba8();
        let pixel = rgba.get_pixel(0, 0);
        assert_eq!(pixel[0], 50);
    }

    #[test]
    fn test_brightness_clamp_high() {
        let img = create_gray_image(200);
        let result = brightness(&img, 100).unwrap();
        let rgba = result.to_rgba8();
        let pixel = rgba.get_pixel(0, 0);
        assert_eq!(pixel[0], 255); // Clamped
    }

    #[test]
    fn test_brightness_clamp_low() {
        let img = create_gray_image(50);
        let result = brightness(&img, -100).unwrap();
        let rgba = result.to_rgba8();
        let pixel = rgba.get_pixel(0, 0);
        assert_eq!(pixel[0], 0); // Clamped
    }

    #[test]
    fn test_brightness_preserves_alpha() {
        let img = ImageBuffer::from_fn(1, 1, |_, _| Rgba([128, 128, 128, 100]));
        let img = DynamicImage::ImageRgba8(img);
        let result = brightness(&img, 50).unwrap();
        let rgba = result.to_rgba8();
        let pixel = rgba.get_pixel(0, 0);
        assert_eq!(pixel[3], 100);
    }

    #[test]
    fn test_brightness_invalid_value() {
        let img = create_test_image();
        assert!(brightness(&img, 300).is_err());
        assert!(brightness(&img, -300).is_err());
    }

    #[test]
    fn test_contrast_increase() {
        let img = create_gray_image(200);
        let result = contrast(&img, 2.0).unwrap();
        let rgba = result.to_rgba8();
        let pixel = rgba.get_pixel(0, 0);
        // (200 - 128) * 2 + 128 = 272 -> clamped to 255
        assert_eq!(pixel[0], 255);
    }

    #[test]
    fn test_contrast_decrease() {
        let img = create_gray_image(200);
        let result = contrast(&img, 0.5).unwrap();
        let rgba = result.to_rgba8();
        let pixel = rgba.get_pixel(0, 0);
        // (200 - 128) * 0.5 + 128 = 164
        assert_eq!(pixel[0], 164);
    }

    #[test]
    fn test_contrast_no_change_at_midpoint() {
        let img = create_gray_image(128);
        let result = contrast(&img, 2.0).unwrap();
        let rgba = result.to_rgba8();
        let pixel = rgba.get_pixel(0, 0);
        assert_eq!(pixel[0], 128); // Midpoint unchanged
    }

    #[test]
    fn test_contrast_invalid_value() {
        let img = create_test_image();
        assert!(contrast(&img, -0.5).is_err());
        assert!(contrast(&img, 15.0).is_err());
    }

    #[test]
    fn test_gamma_lighten() {
        let img = create_gray_image(128);
        let result = gamma(&img, 0.5).unwrap();
        let rgba = result.to_rgba8();
        let pixel = rgba.get_pixel(0, 0);
        // gamma < 1 lightens midtones
        assert!(pixel[0] > 128);
    }

    #[test]
    fn test_gamma_darken() {
        let img = create_gray_image(128);
        let result = gamma(&img, 2.0).unwrap();
        let rgba = result.to_rgba8();
        let pixel = rgba.get_pixel(0, 0);
        // gamma > 1 darkens midtones
        assert!(pixel[0] < 128);
    }

    #[test]
    fn test_gamma_no_change() {
        let img = create_gray_image(128);
        let result = gamma(&img, 1.0).unwrap();
        let rgba = result.to_rgba8();
        let pixel = rgba.get_pixel(0, 0);
        assert_eq!(pixel[0], 128); // No change at gamma 1.0
    }

    #[test]
    fn test_gamma_preserves_extremes() {
        // Black stays black
        let img = create_gray_image(0);
        let result = gamma(&img, 0.5).unwrap();
        let rgba = result.to_rgba8();
        assert_eq!(rgba.get_pixel(0, 0)[0], 0);

        // White stays white
        let img = create_gray_image(255);
        let result = gamma(&img, 0.5).unwrap();
        let rgba = result.to_rgba8();
        assert_eq!(rgba.get_pixel(0, 0)[0], 255);
    }

    #[test]
    fn test_gamma_invalid_value() {
        let img = create_test_image();
        assert!(gamma(&img, 0.0).is_err());
        assert!(gamma(&img, 15.0).is_err());
    }
}
