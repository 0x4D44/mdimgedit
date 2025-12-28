use crate::error::{ImgEditError, Result};
use image::DynamicImage;

/// Flip an image horizontally (mirror left-right) and/or vertically (mirror top-bottom)
pub fn flip(img: &DynamicImage, horizontal: bool, vertical: bool) -> Result<DynamicImage> {
    if !horizontal && !vertical {
        return Err(ImgEditError::MissingOption(
            "flip requires --horizontal and/or --vertical".to_string(),
        ));
    }

    let mut result = img.clone();

    if horizontal {
        result = result.fliph();
    }

    if vertical {
        result = result.flipv();
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgba};

    fn create_test_image() -> DynamicImage {
        // Create a 4x4 test image with distinct pixel values
        let img = ImageBuffer::from_fn(4, 4, |x, y| Rgba([x as u8 * 50, y as u8 * 50, 128, 255]));
        DynamicImage::ImageRgba8(img)
    }

    #[test]
    fn test_flip_horizontal() {
        let img = create_test_image();
        let result = flip(&img, true, false).unwrap();

        // Check that pixels are mirrored horizontally
        let orig = img.to_rgba8();
        let flipped = result.to_rgba8();

        for y in 0..4 {
            for x in 0..4 {
                let orig_pixel = orig.get_pixel(x, y);
                let flipped_pixel = flipped.get_pixel(3 - x, y);
                assert_eq!(orig_pixel, flipped_pixel);
            }
        }
    }

    #[test]
    fn test_flip_vertical() {
        let img = create_test_image();
        let result = flip(&img, false, true).unwrap();

        let orig = img.to_rgba8();
        let flipped = result.to_rgba8();

        for y in 0..4 {
            for x in 0..4 {
                let orig_pixel = orig.get_pixel(x, y);
                let flipped_pixel = flipped.get_pixel(x, 3 - y);
                assert_eq!(orig_pixel, flipped_pixel);
            }
        }
    }

    #[test]
    fn test_flip_both() {
        let img = create_test_image();
        let result = flip(&img, true, true).unwrap();

        let orig = img.to_rgba8();
        let flipped = result.to_rgba8();

        for y in 0..4 {
            for x in 0..4 {
                let orig_pixel = orig.get_pixel(x, y);
                let flipped_pixel = flipped.get_pixel(3 - x, 3 - y);
                assert_eq!(orig_pixel, flipped_pixel);
            }
        }
    }

    #[test]
    fn test_flip_neither_fails() {
        let img = create_test_image();
        let result = flip(&img, false, false);
        assert!(result.is_err());
        match result {
            Err(ImgEditError::MissingOption(_)) => {}
            _ => panic!("Expected MissingOption error"),
        }
    }

    #[test]
    fn test_flip_preserves_dimensions() {
        let img = ImageBuffer::from_fn(100, 50, |_, _| Rgba([128, 128, 128, 255]));
        let img = DynamicImage::ImageRgba8(img);

        let h = flip(&img, true, false).unwrap();
        assert_eq!(h.width(), 100);
        assert_eq!(h.height(), 50);

        let v = flip(&img, false, true).unwrap();
        assert_eq!(v.width(), 100);
        assert_eq!(v.height(), 50);

        let both = flip(&img, true, true).unwrap();
        assert_eq!(both.width(), 100);
        assert_eq!(both.height(), 50);
    }
}
