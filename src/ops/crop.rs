use crate::cli::args::Anchor;
use crate::error::{ImgEditError, Result};
use image::DynamicImage;

/// Calculate crop coordinates based on anchor position
pub fn calculate_crop_position(
    img_width: u32,
    img_height: u32,
    crop_width: u32,
    crop_height: u32,
    x_offset: u32,
    y_offset: u32,
    anchor: Anchor,
) -> (u32, u32) {
    let (anchor_x, anchor_y) = match anchor {
        Anchor::TopLeft => (0, 0),
        Anchor::TopCenter => ((img_width.saturating_sub(crop_width)) / 2, 0),
        Anchor::TopRight => (img_width.saturating_sub(crop_width), 0),
        Anchor::CenterLeft => (0, (img_height.saturating_sub(crop_height)) / 2),
        Anchor::Center => (
            (img_width.saturating_sub(crop_width)) / 2,
            (img_height.saturating_sub(crop_height)) / 2,
        ),
        Anchor::CenterRight => (
            img_width.saturating_sub(crop_width),
            (img_height.saturating_sub(crop_height)) / 2,
        ),
        Anchor::BottomLeft => (0, img_height.saturating_sub(crop_height)),
        Anchor::BottomCenter => (
            (img_width.saturating_sub(crop_width)) / 2,
            img_height.saturating_sub(crop_height),
        ),
        Anchor::BottomRight => (
            img_width.saturating_sub(crop_width),
            img_height.saturating_sub(crop_height),
        ),
    };

    (anchor_x + x_offset, anchor_y + y_offset)
}

/// Crop an image to the specified region
pub fn crop(
    img: &DynamicImage,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    anchor: Anchor,
) -> Result<DynamicImage> {
    let img_width = img.width();
    let img_height = img.height();

    // Validate dimensions
    if width == 0 || height == 0 {
        return Err(ImgEditError::InvalidDimensions(
            "Crop width and height must be greater than 0".to_string(),
        ));
    }

    // Calculate actual position based on anchor
    let (actual_x, actual_y) =
        calculate_crop_position(img_width, img_height, width, height, x, y, anchor);

    // Check bounds
    if actual_x + width > img_width || actual_y + height > img_height {
        return Err(ImgEditError::CropOutOfBounds(format!(
            "Crop region ({}, {}) + {}x{} exceeds image bounds {}x{}",
            actual_x, actual_y, width, height, img_width, img_height
        )));
    }

    Ok(img.crop_imm(actual_x, actual_y, width, height))
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgba};

    fn create_test_image(width: u32, height: u32) -> DynamicImage {
        let img = ImageBuffer::from_fn(width, height, |x, y| Rgba([x as u8, y as u8, 128, 255]));
        DynamicImage::ImageRgba8(img)
    }

    #[test]
    fn test_crop_basic() {
        let img = create_test_image(100, 100);
        let result = crop(&img, 10, 10, 50, 50, Anchor::TopLeft).unwrap();
        assert_eq!(result.width(), 50);
        assert_eq!(result.height(), 50);
    }

    #[test]
    fn test_crop_full_image() {
        let img = create_test_image(100, 100);
        let result = crop(&img, 0, 0, 100, 100, Anchor::TopLeft).unwrap();
        assert_eq!(result.width(), 100);
        assert_eq!(result.height(), 100);
    }

    #[test]
    fn test_crop_center_anchor() {
        let img = create_test_image(100, 100);
        let result = crop(&img, 0, 0, 50, 50, Anchor::Center).unwrap();
        assert_eq!(result.width(), 50);
        assert_eq!(result.height(), 50);
    }

    #[test]
    fn test_crop_out_of_bounds() {
        let img = create_test_image(100, 100);
        let result = crop(&img, 60, 60, 50, 50, Anchor::TopLeft);
        assert!(result.is_err());
        match result {
            Err(ImgEditError::CropOutOfBounds(_)) => {}
            _ => panic!("Expected CropOutOfBounds error"),
        }
    }

    #[test]
    fn test_crop_zero_dimensions() {
        let img = create_test_image(100, 100);
        let result = crop(&img, 0, 0, 0, 50, Anchor::TopLeft);
        assert!(result.is_err());
        match result {
            Err(ImgEditError::InvalidDimensions(_)) => {}
            _ => panic!("Expected InvalidDimensions error"),
        }
    }

    #[test]
    fn test_calculate_crop_position_all_anchors() {
        let (x, y) = calculate_crop_position(100, 100, 50, 50, 0, 0, Anchor::TopLeft);
        assert_eq!((x, y), (0, 0));

        let (x, y) = calculate_crop_position(100, 100, 50, 50, 0, 0, Anchor::TopCenter);
        assert_eq!((x, y), (25, 0));

        let (x, y) = calculate_crop_position(100, 100, 50, 50, 0, 0, Anchor::TopRight);
        assert_eq!((x, y), (50, 0));

        let (x, y) = calculate_crop_position(100, 100, 50, 50, 0, 0, Anchor::CenterLeft);
        assert_eq!((x, y), (0, 25));

        let (x, y) = calculate_crop_position(100, 100, 50, 50, 0, 0, Anchor::Center);
        assert_eq!((x, y), (25, 25));

        let (x, y) = calculate_crop_position(100, 100, 50, 50, 0, 0, Anchor::CenterRight);
        assert_eq!((x, y), (50, 25));

        let (x, y) = calculate_crop_position(100, 100, 50, 50, 0, 0, Anchor::BottomLeft);
        assert_eq!((x, y), (0, 50));

        let (x, y) = calculate_crop_position(100, 100, 50, 50, 0, 0, Anchor::BottomCenter);
        assert_eq!((x, y), (25, 50));

        let (x, y) = calculate_crop_position(100, 100, 50, 50, 0, 0, Anchor::BottomRight);
        assert_eq!((x, y), (50, 50));
    }

    #[test]
    fn test_calculate_crop_position_with_offset() {
        let (x, y) = calculate_crop_position(100, 100, 50, 50, 5, 10, Anchor::TopLeft);
        assert_eq!((x, y), (5, 10));
    }
}
