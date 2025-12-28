use crate::cli::args::ResizeFilter;
use crate::error::{ImgEditError, Result};
use image::DynamicImage;

/// Resize an image to exact dimensions or by a scale factor
pub fn resize(
    img: &DynamicImage,
    width: Option<u32>,
    height: Option<u32>,
    scale: Option<f64>,
    filter: ResizeFilter,
) -> Result<DynamicImage> {
    let img_width = img.width();
    let img_height = img.height();

    // Determine target dimensions
    let (target_width, target_height) = if let Some(s) = scale {
        if s <= 0.0 {
            return Err(ImgEditError::InvalidParameter(
                "Scale must be positive".to_string(),
            ));
        }
        let new_w = (img_width as f64 * s).round() as u32;
        let new_h = (img_height as f64 * s).round() as u32;
        if new_w == 0 || new_h == 0 {
            return Err(ImgEditError::InvalidDimensions(
                "Scaled dimensions would be zero".to_string(),
            ));
        }
        (new_w, new_h)
    } else {
        match (width, height) {
            (Some(w), Some(h)) => {
                if w == 0 || h == 0 {
                    return Err(ImgEditError::InvalidDimensions(
                        "Width and height must be positive".to_string(),
                    ));
                }
                (w, h)
            }
            (Some(w), None) => {
                if w == 0 {
                    return Err(ImgEditError::InvalidDimensions(
                        "Width must be positive".to_string(),
                    ));
                }
                // Calculate height to preserve aspect ratio
                let ratio = w as f64 / img_width as f64;
                let h = (img_height as f64 * ratio).round() as u32;
                (w, h.max(1))
            }
            (None, Some(h)) => {
                if h == 0 {
                    return Err(ImgEditError::InvalidDimensions(
                        "Height must be positive".to_string(),
                    ));
                }
                // Calculate width to preserve aspect ratio
                let ratio = h as f64 / img_height as f64;
                let w = (img_width as f64 * ratio).round() as u32;
                (w.max(1), h)
            }
            (None, None) => {
                return Err(ImgEditError::InvalidParameter(
                    "Must specify width, height, or scale".to_string(),
                ));
            }
        }
    };

    Ok(img.resize_exact(target_width, target_height, filter.to_image_filter()))
}

/// Resize an image to fit within maximum bounds while preserving aspect ratio
pub fn fit(
    img: &DynamicImage,
    max_width: Option<u32>,
    max_height: Option<u32>,
    upscale: bool,
    filter: ResizeFilter,
) -> Result<DynamicImage> {
    if max_width.is_none() && max_height.is_none() {
        return Err(ImgEditError::InvalidParameter(
            "Must specify at least one of max-width or max-height".to_string(),
        ));
    }

    let img_width = img.width();
    let img_height = img.height();

    // Calculate scale factors for each constraint
    let width_scale = max_width.map(|w| w as f64 / img_width as f64);
    let height_scale = max_height.map(|h| h as f64 / img_height as f64);

    // Use the smaller scale to ensure the image fits within both constraints
    let scale = match (width_scale, height_scale) {
        (Some(ws), Some(hs)) => ws.min(hs),
        (Some(ws), None) => ws,
        (None, Some(hs)) => hs,
        (None, None) => unreachable!(),
    };

    // Don't upscale unless requested
    let final_scale = if !upscale && scale > 1.0 { 1.0 } else { scale };

    if (final_scale - 1.0).abs() < 0.0001 {
        // No change needed
        return Ok(img.clone());
    }

    let target_width = (img_width as f64 * final_scale).round() as u32;
    let target_height = (img_height as f64 * final_scale).round() as u32;

    if target_width == 0 || target_height == 0 {
        return Err(ImgEditError::InvalidDimensions(
            "Resulting dimensions would be zero".to_string(),
        ));
    }

    Ok(img.resize_exact(target_width, target_height, filter.to_image_filter()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgba};

    fn create_test_image(width: u32, height: u32) -> DynamicImage {
        let img = ImageBuffer::from_fn(width, height, |_, _| Rgba([128, 128, 128, 255]));
        DynamicImage::ImageRgba8(img)
    }

    #[test]
    fn test_resize_exact_dimensions() {
        let img = create_test_image(100, 100);
        let result = resize(&img, Some(50), Some(50), None, ResizeFilter::Lanczos).unwrap();
        assert_eq!(result.width(), 50);
        assert_eq!(result.height(), 50);
    }

    #[test]
    fn test_resize_width_only() {
        let img = create_test_image(100, 50);
        let result = resize(&img, Some(50), None, None, ResizeFilter::Lanczos).unwrap();
        assert_eq!(result.width(), 50);
        assert_eq!(result.height(), 25); // Preserves 2:1 aspect ratio
    }

    #[test]
    fn test_resize_height_only() {
        let img = create_test_image(100, 50);
        let result = resize(&img, None, Some(100), None, ResizeFilter::Lanczos).unwrap();
        assert_eq!(result.width(), 200); // Preserves 2:1 aspect ratio
        assert_eq!(result.height(), 100);
    }

    #[test]
    fn test_resize_scale_up() {
        let img = create_test_image(100, 100);
        let result = resize(&img, None, None, Some(2.0), ResizeFilter::Lanczos).unwrap();
        assert_eq!(result.width(), 200);
        assert_eq!(result.height(), 200);
    }

    #[test]
    fn test_resize_scale_down() {
        let img = create_test_image(100, 100);
        let result = resize(&img, None, None, Some(0.5), ResizeFilter::Lanczos).unwrap();
        assert_eq!(result.width(), 50);
        assert_eq!(result.height(), 50);
    }

    #[test]
    fn test_resize_invalid_zero_scale() {
        let img = create_test_image(100, 100);
        let result = resize(&img, None, None, Some(0.0), ResizeFilter::Lanczos);
        assert!(result.is_err());
    }

    #[test]
    fn test_resize_negative_scale() {
        let img = create_test_image(100, 100);
        let result = resize(&img, None, None, Some(-1.0), ResizeFilter::Lanczos);
        assert!(result.is_err());
    }

    #[test]
    fn test_resize_no_params() {
        let img = create_test_image(100, 100);
        let result = resize(&img, None, None, None, ResizeFilter::Lanczos);
        assert!(result.is_err());
    }

    #[test]
    fn test_resize_zero_dimension() {
        let img = create_test_image(100, 100);
        let result = resize(&img, Some(0), Some(50), None, ResizeFilter::Lanczos);
        assert!(result.is_err());
    }

    #[test]
    fn test_fit_within_width() {
        let img = create_test_image(200, 100);
        let result = fit(&img, Some(100), None, false, ResizeFilter::Lanczos).unwrap();
        assert_eq!(result.width(), 100);
        assert_eq!(result.height(), 50);
    }

    #[test]
    fn test_fit_within_height() {
        let img = create_test_image(200, 100);
        let result = fit(&img, None, Some(50), false, ResizeFilter::Lanczos).unwrap();
        assert_eq!(result.width(), 100);
        assert_eq!(result.height(), 50);
    }

    #[test]
    fn test_fit_within_both_width_limited() {
        let img = create_test_image(200, 100);
        let result = fit(&img, Some(100), Some(100), false, ResizeFilter::Lanczos).unwrap();
        // Width is the limiting factor
        assert_eq!(result.width(), 100);
        assert_eq!(result.height(), 50);
    }

    #[test]
    fn test_fit_within_both_height_limited() {
        let img = create_test_image(100, 200);
        let result = fit(&img, Some(100), Some(100), false, ResizeFilter::Lanczos).unwrap();
        // Height is the limiting factor
        assert_eq!(result.width(), 50);
        assert_eq!(result.height(), 100);
    }

    #[test]
    fn test_fit_no_upscale() {
        let img = create_test_image(50, 50);
        let result = fit(&img, Some(100), Some(100), false, ResizeFilter::Lanczos).unwrap();
        // Should not upscale
        assert_eq!(result.width(), 50);
        assert_eq!(result.height(), 50);
    }

    #[test]
    fn test_fit_with_upscale() {
        let img = create_test_image(50, 50);
        let result = fit(&img, Some(100), Some(100), true, ResizeFilter::Lanczos).unwrap();
        // Should upscale
        assert_eq!(result.width(), 100);
        assert_eq!(result.height(), 100);
    }

    #[test]
    fn test_fit_no_params() {
        let img = create_test_image(100, 100);
        let result = fit(&img, None, None, false, ResizeFilter::Lanczos);
        assert!(result.is_err());
    }

    #[test]
    fn test_resize_all_filters() {
        let img = create_test_image(100, 100);

        let filters = [
            ResizeFilter::Nearest,
            ResizeFilter::Linear,
            ResizeFilter::Cubic,
            ResizeFilter::Lanczos,
        ];

        for filter in filters {
            let result = resize(&img, Some(50), Some(50), None, filter).unwrap();
            assert_eq!(result.width(), 50);
            assert_eq!(result.height(), 50);
        }
    }
}
