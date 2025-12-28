use crate::error::{ImgEditError, Result};
use image::{DynamicImage, GrayImage, ImageBuffer, Luma, Rgba, RgbaImage};

/// Convert an image to grayscale
pub fn grayscale(img: &DynamicImage, preserve_alpha: bool) -> Result<DynamicImage> {
    if preserve_alpha {
        // Convert to grayscale while keeping alpha channel
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();

        let result: RgbaImage = ImageBuffer::from_fn(width, height, |x, y| {
            let pixel = rgba.get_pixel(x, y);
            // Standard luminance formula
            let gray =
                (0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32) as u8;
            Rgba([gray, gray, gray, pixel[3]])
        });

        Ok(DynamicImage::ImageRgba8(result))
    } else {
        Ok(DynamicImage::ImageLuma8(img.to_luma8()))
    }
}

/// Change the bit depth of an image
pub fn change_depth(img: &DynamicImage, bits: u8, dither: bool) -> Result<DynamicImage> {
    match bits {
        1 => convert_to_1bit(img, dither),
        8 => Ok(img.clone()), // Already 8-bit typically
        16 => convert_to_16bit(img),
        _ => Err(ImgEditError::InvalidParameter(format!(
            "Unsupported bit depth: {}. Use 1, 8, or 16.",
            bits
        ))),
    }
}

fn convert_to_1bit(img: &DynamicImage, dither: bool) -> Result<DynamicImage> {
    let gray = img.to_luma8();
    let (width, height) = gray.dimensions();

    if dither {
        // Floyd-Steinberg dithering
        let mut buffer: Vec<Vec<i32>> = gray
            .rows()
            .map(|row| row.map(|p| p[0] as i32).collect())
            .collect();

        let result: GrayImage = ImageBuffer::from_fn(width, height, |x, y| {
            let old_pixel = buffer[y as usize][x as usize].clamp(0, 255);
            let new_pixel = if old_pixel > 127 { 255 } else { 0 };
            let error = old_pixel - new_pixel;

            // Distribute error to neighbors
            if x + 1 < width {
                buffer[y as usize][(x + 1) as usize] += error * 7 / 16;
            }
            if y + 1 < height {
                if x > 0 {
                    buffer[(y + 1) as usize][(x - 1) as usize] += error * 3 / 16;
                }
                buffer[(y + 1) as usize][x as usize] += error * 5 / 16;
                if x + 1 < width {
                    buffer[(y + 1) as usize][(x + 1) as usize] += error / 16;
                }
            }

            Luma([new_pixel as u8])
        });

        Ok(DynamicImage::ImageLuma8(result))
    } else {
        // Simple threshold
        let result: GrayImage = ImageBuffer::from_fn(width, height, |x, y| {
            let pixel = gray.get_pixel(x, y)[0];
            Luma([if pixel > 127 { 255 } else { 0 }])
        });

        Ok(DynamicImage::ImageLuma8(result))
    }
}

fn convert_to_16bit(img: &DynamicImage) -> Result<DynamicImage> {
    // Convert to 16-bit RGBA using the built-in conversion
    let rgba16 = img.to_rgba16();
    Ok(DynamicImage::ImageRgba16(rgba16))
}

/// Invert the colors of an image
pub fn invert(img: &DynamicImage, invert_alpha: bool) -> Result<DynamicImage> {
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();

    let result: RgbaImage = ImageBuffer::from_fn(width, height, |x, y| {
        let pixel = rgba.get_pixel(x, y);
        if invert_alpha {
            Rgba([
                255 - pixel[0],
                255 - pixel[1],
                255 - pixel[2],
                255 - pixel[3],
            ])
        } else {
            Rgba([255 - pixel[0], 255 - pixel[1], 255 - pixel[2], pixel[3]])
        }
    });

    Ok(DynamicImage::ImageRgba8(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::ImageBuffer;

    fn create_test_image() -> DynamicImage {
        let img = ImageBuffer::from_fn(10, 10, |x, y| {
            Rgba([(x * 25) as u8, (y * 25) as u8, 128u8, 255u8])
        });
        DynamicImage::ImageRgba8(img)
    }

    fn create_gradient_image() -> DynamicImage {
        let img = ImageBuffer::from_fn(10, 10, |x, _| {
            let val = (x * 25) as u8;
            Rgba([val, val, val, 255])
        });
        DynamicImage::ImageRgba8(img)
    }

    #[test]
    fn test_grayscale_preserve_alpha() {
        let img = create_test_image();
        let result = grayscale(&img, true).unwrap();

        let rgba = result.to_rgba8();
        let pixel = rgba.get_pixel(0, 0);
        // R, G, B should be equal (grayscale)
        assert_eq!(pixel[0], pixel[1]);
        assert_eq!(pixel[1], pixel[2]);
        // Alpha should be preserved
        assert_eq!(pixel[3], 255);
    }

    #[test]
    fn test_grayscale_no_alpha() {
        let img = create_test_image();
        let result = grayscale(&img, false).unwrap();

        // Should be a luma image
        assert!(matches!(result, DynamicImage::ImageLuma8(_)));
    }

    #[test]
    fn test_depth_1bit() {
        let img = create_gradient_image();
        let result = change_depth(&img, 1, false).unwrap();

        let gray = result.to_luma8();
        // All pixels should be either 0 or 255
        for pixel in gray.pixels() {
            assert!(pixel[0] == 0 || pixel[0] == 255);
        }
    }

    #[test]
    fn test_depth_1bit_dither() {
        let img = create_gradient_image();
        let result = change_depth(&img, 1, true).unwrap();

        let gray = result.to_luma8();
        // All pixels should be either 0 or 255
        for pixel in gray.pixels() {
            assert!(pixel[0] == 0 || pixel[0] == 255);
        }
    }

    #[test]
    fn test_depth_16bit() {
        let img = create_test_image();
        let result = change_depth(&img, 16, false).unwrap();

        assert!(matches!(result, DynamicImage::ImageRgba16(_)));
    }

    #[test]
    fn test_depth_invalid() {
        let img = create_test_image();
        let result = change_depth(&img, 4, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_invert_colors() {
        let img = ImageBuffer::from_fn(1, 1, |_, _| Rgba([100, 150, 200, 255]));
        let img = DynamicImage::ImageRgba8(img);

        let result = invert(&img, false).unwrap();
        let rgba = result.to_rgba8();
        let pixel = rgba.get_pixel(0, 0);

        assert_eq!(pixel[0], 155); // 255 - 100
        assert_eq!(pixel[1], 105); // 255 - 150
        assert_eq!(pixel[2], 55); // 255 - 200
        assert_eq!(pixel[3], 255); // Alpha preserved
    }

    #[test]
    fn test_invert_with_alpha() {
        let img = ImageBuffer::from_fn(1, 1, |_, _| Rgba([100, 150, 200, 100]));
        let img = DynamicImage::ImageRgba8(img);

        let result = invert(&img, true).unwrap();
        let rgba = result.to_rgba8();
        let pixel = rgba.get_pixel(0, 0);

        assert_eq!(pixel[0], 155);
        assert_eq!(pixel[1], 105);
        assert_eq!(pixel[2], 55);
        assert_eq!(pixel[3], 155); // 255 - 100
    }

    #[test]
    fn test_invert_black_to_white() {
        let img = ImageBuffer::from_fn(1, 1, |_, _| Rgba([0, 0, 0, 255]));
        let img = DynamicImage::ImageRgba8(img);

        let result = invert(&img, false).unwrap();
        let rgba = result.to_rgba8();
        let pixel = rgba.get_pixel(0, 0);

        assert_eq!(pixel[0], 255);
        assert_eq!(pixel[1], 255);
        assert_eq!(pixel[2], 255);
    }

    #[test]
    fn test_invert_white_to_black() {
        let img = ImageBuffer::from_fn(1, 1, |_, _| Rgba([255, 255, 255, 255]));
        let img = DynamicImage::ImageRgba8(img);

        let result = invert(&img, false).unwrap();
        let rgba = result.to_rgba8();
        let pixel = rgba.get_pixel(0, 0);

        assert_eq!(pixel[0], 0);
        assert_eq!(pixel[1], 0);
        assert_eq!(pixel[2], 0);
    }
}
