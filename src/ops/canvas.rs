use crate::cli::args::{Anchor, BlendMode};
use crate::error::{ImgEditError, Result};
use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage};

/// Add padding around an image
pub fn pad(
    img: &DynamicImage,
    top: u32,
    bottom: u32,
    left: u32,
    right: u32,
    color: Rgba<u8>,
) -> Result<DynamicImage> {
    let rgba = img.to_rgba8();
    let (orig_width, orig_height) = rgba.dimensions();

    let new_width = orig_width + left + right;
    let new_height = orig_height + top + bottom;

    if new_width == 0 || new_height == 0 {
        return Err(ImgEditError::InvalidDimensions(
            "Resulting image dimensions would be zero".to_string(),
        ));
    }

    // Create new image filled with padding color
    let mut result: RgbaImage = ImageBuffer::from_pixel(new_width, new_height, color);

    // Copy original image to the padded position
    for y in 0..orig_height {
        for x in 0..orig_width {
            let pixel = rgba.get_pixel(x, y);
            result.put_pixel(x + left, y + top, *pixel);
        }
    }

    Ok(DynamicImage::ImageRgba8(result))
}

/// Resize the canvas without scaling the image content
pub fn canvas_resize(
    img: &DynamicImage,
    new_width: u32,
    new_height: u32,
    anchor: Anchor,
    background: Rgba<u8>,
) -> Result<DynamicImage> {
    if new_width == 0 || new_height == 0 {
        return Err(ImgEditError::InvalidDimensions(
            "Canvas dimensions must be positive".to_string(),
        ));
    }

    let rgba = img.to_rgba8();
    let (orig_width, orig_height) = rgba.dimensions();

    // Calculate where to place the original image
    let (offset_x, offset_y) =
        calculate_anchor_offset(new_width, new_height, orig_width, orig_height, anchor);

    // Create new canvas
    let mut result: RgbaImage = ImageBuffer::from_pixel(new_width, new_height, background);

    // Copy pixels from original, handling clipping
    for y in 0..orig_height {
        for x in 0..orig_width {
            let dest_x = offset_x + x as i64;
            let dest_y = offset_y + y as i64;

            if dest_x >= 0 && dest_x < new_width as i64 && dest_y >= 0 && dest_y < new_height as i64
            {
                let pixel = rgba.get_pixel(x, y);
                result.put_pixel(dest_x as u32, dest_y as u32, *pixel);
            }
        }
    }

    Ok(DynamicImage::ImageRgba8(result))
}

fn calculate_anchor_offset(
    canvas_w: u32,
    canvas_h: u32,
    img_w: u32,
    img_h: u32,
    anchor: Anchor,
) -> (i64, i64) {
    let diff_w = canvas_w as i64 - img_w as i64;
    let diff_h = canvas_h as i64 - img_h as i64;

    match anchor {
        Anchor::TopLeft => (0, 0),
        Anchor::TopCenter => (diff_w / 2, 0),
        Anchor::TopRight => (diff_w, 0),
        Anchor::CenterLeft => (0, diff_h / 2),
        Anchor::Center => (diff_w / 2, diff_h / 2),
        Anchor::CenterRight => (diff_w, diff_h / 2),
        Anchor::BottomLeft => (0, diff_h),
        Anchor::BottomCenter => (diff_w / 2, diff_h),
        Anchor::BottomRight => (diff_w, diff_h),
    }
}

/// Composite (overlay) one image onto another
pub fn composite(
    base: &DynamicImage,
    overlay: &DynamicImage,
    x: i32,
    y: i32,
    anchor: Option<Anchor>,
    opacity: f32,
    blend_mode: BlendMode,
) -> Result<DynamicImage> {
    if !(0.0..=1.0).contains(&opacity) {
        return Err(ImgEditError::InvalidParameter(format!(
            "Opacity must be between 0.0 and 1.0, got {}",
            opacity
        )));
    }

    let mut base_rgba = base.to_rgba8();
    let overlay_rgba = overlay.to_rgba8();

    let (base_w, base_h) = base_rgba.dimensions();
    let (overlay_w, overlay_h) = overlay_rgba.dimensions();

    // Calculate position based on anchor or explicit coordinates
    let (pos_x, pos_y) = if let Some(anch) = anchor {
        let (ax, ay) = calculate_anchor_offset(base_w, base_h, overlay_w, overlay_h, anch);
        (ax as i32, ay as i32)
    } else {
        (x, y)
    };

    // Composite the overlay onto the base
    for oy in 0..overlay_h {
        for ox in 0..overlay_w {
            let dest_x = pos_x + ox as i32;
            let dest_y = pos_y + oy as i32;

            if dest_x >= 0 && dest_x < base_w as i32 && dest_y >= 0 && dest_y < base_h as i32 {
                let overlay_pixel = overlay_rgba.get_pixel(ox, oy);
                let base_pixel = base_rgba.get_pixel(dest_x as u32, dest_y as u32);

                let blended = blend_pixels(*base_pixel, *overlay_pixel, opacity, blend_mode);
                base_rgba.put_pixel(dest_x as u32, dest_y as u32, blended);
            }
        }
    }

    Ok(DynamicImage::ImageRgba8(base_rgba))
}

fn blend_pixels(base: Rgba<u8>, overlay: Rgba<u8>, opacity: f32, mode: BlendMode) -> Rgba<u8> {
    // Apply opacity to overlay alpha
    let overlay_alpha = (overlay[3] as f32 / 255.0) * opacity;

    if overlay_alpha < 0.001 {
        return base;
    }

    let base_alpha = base[3] as f32 / 255.0;

    // Blend each channel based on blend mode
    let (br, bg, bb) = (base[0] as f32, base[1] as f32, base[2] as f32);
    let (or, og, ob) = (overlay[0] as f32, overlay[1] as f32, overlay[2] as f32);

    let (blended_r, blended_g, blended_b) = match mode {
        BlendMode::Normal => (or, og, ob),
        BlendMode::Multiply => (br * or / 255.0, bg * og / 255.0, bb * ob / 255.0),
        BlendMode::Screen => (
            255.0 - (255.0 - br) * (255.0 - or) / 255.0,
            255.0 - (255.0 - bg) * (255.0 - og) / 255.0,
            255.0 - (255.0 - bb) * (255.0 - ob) / 255.0,
        ),
        BlendMode::Overlay => (
            overlay_channel(br, or),
            overlay_channel(bg, og),
            overlay_channel(bb, ob),
        ),
    };

    // Alpha compositing
    let out_alpha = overlay_alpha + base_alpha * (1.0 - overlay_alpha);

    if out_alpha < 0.001 {
        return Rgba([0, 0, 0, 0]);
    }

    let blend_factor = overlay_alpha / out_alpha;

    Rgba([
        lerp(br, blended_r, blend_factor) as u8,
        lerp(bg, blended_g, blend_factor) as u8,
        lerp(bb, blended_b, blend_factor) as u8,
        (out_alpha * 255.0) as u8,
    ])
}

fn overlay_channel(base: f32, overlay: f32) -> f32 {
    if base < 128.0 {
        2.0 * base * overlay / 255.0
    } else {
        255.0 - 2.0 * (255.0 - base) * (255.0 - overlay) / 255.0
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_image(width: u32, height: u32, color: Rgba<u8>) -> DynamicImage {
        let img = ImageBuffer::from_pixel(width, height, color);
        DynamicImage::ImageRgba8(img)
    }

    #[test]
    fn test_pad_all_sides() {
        let img = create_test_image(10, 10, Rgba([255, 0, 0, 255]));
        let result = pad(&img, 5, 5, 5, 5, Rgba([0, 0, 0, 255])).unwrap();

        assert_eq!(result.width(), 20);
        assert_eq!(result.height(), 20);

        let rgba = result.to_rgba8();
        // Corner should be padding color (black)
        assert_eq!(rgba.get_pixel(0, 0), &Rgba([0, 0, 0, 255]));
        // Center should be original color (red)
        assert_eq!(rgba.get_pixel(10, 10), &Rgba([255, 0, 0, 255]));
    }

    #[test]
    fn test_pad_asymmetric() {
        let img = create_test_image(10, 10, Rgba([255, 0, 0, 255]));
        let result = pad(&img, 2, 3, 4, 5, Rgba([0, 255, 0, 255])).unwrap();

        assert_eq!(result.width(), 10 + 4 + 5);
        assert_eq!(result.height(), 10 + 2 + 3);
    }

    #[test]
    fn test_pad_transparent() {
        let img = create_test_image(10, 10, Rgba([255, 0, 0, 255]));
        let result = pad(&img, 5, 5, 5, 5, Rgba([0, 0, 0, 0])).unwrap();

        let rgba = result.to_rgba8();
        // Corner should be transparent
        assert_eq!(rgba.get_pixel(0, 0)[3], 0);
    }

    #[test]
    fn test_canvas_expand() {
        let img = create_test_image(10, 10, Rgba([255, 0, 0, 255]));
        let result = canvas_resize(&img, 20, 20, Anchor::Center, Rgba([0, 0, 0, 255])).unwrap();

        assert_eq!(result.width(), 20);
        assert_eq!(result.height(), 20);

        let rgba = result.to_rgba8();
        // Center should be original
        assert_eq!(rgba.get_pixel(10, 10), &Rgba([255, 0, 0, 255]));
        // Corner should be background
        assert_eq!(rgba.get_pixel(0, 0), &Rgba([0, 0, 0, 255]));
    }

    #[test]
    fn test_canvas_shrink() {
        let img = create_test_image(20, 20, Rgba([255, 0, 0, 255]));
        let result = canvas_resize(&img, 10, 10, Anchor::Center, Rgba([0, 0, 0, 255])).unwrap();

        assert_eq!(result.width(), 10);
        assert_eq!(result.height(), 10);

        // All visible should be original color
        let rgba = result.to_rgba8();
        assert_eq!(rgba.get_pixel(5, 5), &Rgba([255, 0, 0, 255]));
    }

    #[test]
    fn test_canvas_anchor_top_left() {
        let img = create_test_image(10, 10, Rgba([255, 0, 0, 255]));
        let result = canvas_resize(&img, 20, 20, Anchor::TopLeft, Rgba([0, 0, 0, 255])).unwrap();

        let rgba = result.to_rgba8();
        // Top-left should be original
        assert_eq!(rgba.get_pixel(0, 0), &Rgba([255, 0, 0, 255]));
        // Bottom-right should be background
        assert_eq!(rgba.get_pixel(15, 15), &Rgba([0, 0, 0, 255]));
    }

    #[test]
    fn test_canvas_anchor_bottom_right() {
        let img = create_test_image(10, 10, Rgba([255, 0, 0, 255]));
        let result =
            canvas_resize(&img, 20, 20, Anchor::BottomRight, Rgba([0, 0, 0, 255])).unwrap();

        let rgba = result.to_rgba8();
        // Top-left should be background
        assert_eq!(rgba.get_pixel(0, 0), &Rgba([0, 0, 0, 255]));
        // Bottom-right should be original
        assert_eq!(rgba.get_pixel(15, 15), &Rgba([255, 0, 0, 255]));
    }

    #[test]
    fn test_canvas_anchor_remaining_variants() {
        let img = create_test_image(10, 10, Rgba([255, 0, 0, 255]));
        // Just verify they run without error and produce correct dimensions
        let anchors = [
            Anchor::TopCenter,
            Anchor::TopRight,
            Anchor::CenterLeft,
            Anchor::CenterRight,
            Anchor::BottomLeft,
            Anchor::BottomCenter,
        ];

        for anchor in anchors {
            let result = canvas_resize(&img, 20, 20, anchor, Rgba([0, 0, 0, 255])).unwrap();
            assert_eq!(result.width(), 20);
            assert_eq!(result.height(), 20);
        }
    }

    #[test]
    fn test_blend_mode_overlay() {
        // Overlay logic:
        // if base < 0.5: 2 * base * overlay
        // if base >= 0.5: 1 - 2 * (1 - base) * (1 - overlay)

        // Case 1: Base < 0.5
        let base = create_test_image(1, 1, Rgba([64, 64, 64, 255])); // ~0.25
        let overlay = create_test_image(1, 1, Rgba([128, 128, 128, 255])); // ~0.5

        let result = composite(&base, &overlay, 0, 0, None, 1.0, BlendMode::Overlay).unwrap();
        let pixel = result.to_rgba8().get_pixel(0, 0)[0];
        // 2 * 0.25 * 0.5 = 0.25 -> 64
        assert!((pixel as i32 - 64).abs() < 2);

        // Case 2: Base >= 0.5
        let base = create_test_image(1, 1, Rgba([192, 192, 192, 255])); // ~0.75
        let overlay = create_test_image(1, 1, Rgba([128, 128, 128, 255])); // ~0.5

        let result = composite(&base, &overlay, 0, 0, None, 1.0, BlendMode::Overlay).unwrap();
        let pixel = result.to_rgba8().get_pixel(0, 0)[0];
        // 1 - 2 * (0.25) * (0.5) = 1 - 0.25 = 0.75 -> 192
        assert!((pixel as i32 - 192).abs() < 2);
    }

    #[test]
    fn test_canvas_zero_dimension() {
        let img = create_test_image(10, 10, Rgba([255, 0, 0, 255]));
        let result = canvas_resize(&img, 0, 10, Anchor::Center, Rgba([0, 0, 0, 255]));
        assert!(result.is_err());
    }

    #[test]
    fn test_composite_basic() {
        let base = create_test_image(20, 20, Rgba([255, 0, 0, 255]));
        let overlay = create_test_image(10, 10, Rgba([0, 255, 0, 255]));

        let result = composite(&base, &overlay, 5, 5, None, 1.0, BlendMode::Normal).unwrap();

        assert_eq!(result.width(), 20);
        assert_eq!(result.height(), 20);

        let rgba = result.to_rgba8();
        // Top-left corner should be base color
        assert_eq!(rgba.get_pixel(0, 0), &Rgba([255, 0, 0, 255]));
        // Center should be overlay color
        assert_eq!(rgba.get_pixel(10, 10), &Rgba([0, 255, 0, 255]));
    }

    #[test]
    fn test_composite_with_anchor() {
        let base = create_test_image(20, 20, Rgba([255, 0, 0, 255]));
        let overlay = create_test_image(10, 10, Rgba([0, 255, 0, 255]));

        let result = composite(
            &base,
            &overlay,
            0,
            0,
            Some(Anchor::Center),
            1.0,
            BlendMode::Normal,
        )
        .unwrap();

        let rgba = result.to_rgba8();
        // Center should be overlay
        assert_eq!(rgba.get_pixel(10, 10), &Rgba([0, 255, 0, 255]));
    }

    #[test]
    fn test_composite_with_opacity() {
        let base = create_test_image(10, 10, Rgba([255, 0, 0, 255]));
        let overlay = create_test_image(10, 10, Rgba([0, 255, 0, 255]));

        let result = composite(&base, &overlay, 0, 0, None, 0.5, BlendMode::Normal).unwrap();

        let rgba = result.to_rgba8();
        let pixel = rgba.get_pixel(5, 5);
        // Should be a blend of red and green
        assert!(pixel[0] > 100 && pixel[0] < 200);
        assert!(pixel[1] > 100 && pixel[1] < 200);
    }

    #[test]
    fn test_composite_zero_opacity() {
        let base = create_test_image(10, 10, Rgba([255, 0, 0, 255]));
        let overlay = create_test_image(10, 10, Rgba([0, 255, 0, 255]));

        let result = composite(&base, &overlay, 0, 0, None, 0.0, BlendMode::Normal).unwrap();

        let rgba = result.to_rgba8();
        // Should be unchanged base color
        assert_eq!(rgba.get_pixel(5, 5), &Rgba([255, 0, 0, 255]));
    }

    #[test]
    fn test_composite_invalid_opacity() {
        let base = create_test_image(10, 10, Rgba([255, 0, 0, 255]));
        let overlay = create_test_image(10, 10, Rgba([0, 255, 0, 255]));

        assert!(composite(&base, &overlay, 0, 0, None, 1.5, BlendMode::Normal).is_err());
        assert!(composite(&base, &overlay, 0, 0, None, -0.5, BlendMode::Normal).is_err());
    }

    #[test]
    fn test_composite_overlay_clipped() {
        let base = create_test_image(10, 10, Rgba([255, 0, 0, 255]));
        let overlay = create_test_image(10, 10, Rgba([0, 255, 0, 255]));

        // Overlay placed partially outside
        let result = composite(&base, &overlay, 5, 5, None, 1.0, BlendMode::Normal).unwrap();

        let rgba = result.to_rgba8();
        // Top-left should still be base
        assert_eq!(rgba.get_pixel(0, 0), &Rgba([255, 0, 0, 255]));
        // Where overlay is visible, should be green
        assert_eq!(rgba.get_pixel(8, 8), &Rgba([0, 255, 0, 255]));
    }

    #[test]
    fn test_blend_mode_multiply() {
        let base = create_test_image(1, 1, Rgba([255, 255, 255, 255]));
        let overlay = create_test_image(1, 1, Rgba([128, 128, 128, 255]));

        let result = composite(&base, &overlay, 0, 0, None, 1.0, BlendMode::Multiply).unwrap();
        let rgba = result.to_rgba8();
        let pixel = rgba.get_pixel(0, 0);
        // White * gray = gray
        assert!(pixel[0] > 120 && pixel[0] < 136);
    }

    #[test]
    fn test_blend_mode_screen() {
        let base = create_test_image(1, 1, Rgba([0, 0, 0, 255]));
        let overlay = create_test_image(1, 1, Rgba([128, 128, 128, 255]));

        let result = composite(&base, &overlay, 0, 0, None, 1.0, BlendMode::Screen).unwrap();
        let rgba = result.to_rgba8();
        let pixel = rgba.get_pixel(0, 0);
        // Black screen gray = gray
        assert!(pixel[0] > 120 && pixel[0] < 136);
    }
}
