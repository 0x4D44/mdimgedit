use crate::cli::args::ImageFormat;
use crate::error::{ImgEditError, Result};
use image::DynamicImage;
use std::path::Path;

/// Determine the output format from path extension or explicit format
pub fn determine_format(
    output_path: &Path,
    explicit_format: Option<ImageFormat>,
) -> Result<image::ImageFormat> {
    if let Some(fmt) = explicit_format {
        return Ok(image_format_from_cli(fmt));
    }

    // Try to determine from extension
    let ext = output_path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase());

    match ext.as_deref() {
        Some("png") => Ok(image::ImageFormat::Png),
        Some("jpg") | Some("jpeg") => Ok(image::ImageFormat::Jpeg),
        Some("gif") => Ok(image::ImageFormat::Gif),
        Some("bmp") => Ok(image::ImageFormat::Bmp),
        Some("tiff") | Some("tif") => Ok(image::ImageFormat::Tiff),
        Some("webp") => Ok(image::ImageFormat::WebP),
        Some("ico") => Ok(image::ImageFormat::Ico),
        Some(ext) => Err(ImgEditError::UnsupportedFormat(format!(
            "Unknown extension: .{}",
            ext
        ))),
        None => Err(ImgEditError::UnsupportedFormat(
            "No file extension and no --format specified".to_string(),
        )),
    }
}

fn image_format_from_cli(fmt: ImageFormat) -> image::ImageFormat {
    match fmt {
        ImageFormat::Png => image::ImageFormat::Png,
        ImageFormat::Jpeg => image::ImageFormat::Jpeg,
        ImageFormat::Gif => image::ImageFormat::Gif,
        ImageFormat::Bmp => image::ImageFormat::Bmp,
        ImageFormat::Tiff => image::ImageFormat::Tiff,
        ImageFormat::Webp => image::ImageFormat::WebP,
        ImageFormat::Ico => image::ImageFormat::Ico,
    }
}

/// Save an image in the specified format with quality settings
pub fn save_with_format(
    img: &DynamicImage,
    output_path: &Path,
    format: image::ImageFormat,
    quality: u8,
) -> Result<()> {
    use std::fs::File;
    use std::io::BufWriter;

    let file = File::create(output_path).map_err(|e| ImgEditError::WriteError {
        path: output_path.display().to_string(),
        reason: e.to_string(),
    })?;
    let writer = BufWriter::new(file);

    match format {
        image::ImageFormat::Jpeg => {
            let rgb = img.to_rgb8();
            let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(writer, quality);
            encoder
                .encode_image(&rgb)
                .map_err(|e| ImgEditError::WriteError {
                    path: output_path.display().to_string(),
                    reason: e.to_string(),
                })?;
        }
        image::ImageFormat::Png => {
            let encoder = image::codecs::png::PngEncoder::new(writer);
            img.write_with_encoder(encoder)
                .map_err(|e| ImgEditError::WriteError {
                    path: output_path.display().to_string(),
                    reason: e.to_string(),
                })?;
        }
        image::ImageFormat::Gif => {
            let encoder = image::codecs::gif::GifEncoder::new(writer);
            img.write_with_encoder(encoder)
                .map_err(|e| ImgEditError::WriteError {
                    path: output_path.display().to_string(),
                    reason: e.to_string(),
                })?;
        }
        image::ImageFormat::Bmp => {
            let mut writer = writer;
            let encoder = image::codecs::bmp::BmpEncoder::new(&mut writer);
            img.write_with_encoder(encoder)
                .map_err(|e| ImgEditError::WriteError {
                    path: output_path.display().to_string(),
                    reason: e.to_string(),
                })?;
        }
        image::ImageFormat::Tiff => {
            let encoder = image::codecs::tiff::TiffEncoder::new(writer);
            img.write_with_encoder(encoder)
                .map_err(|e| ImgEditError::WriteError {
                    path: output_path.display().to_string(),
                    reason: e.to_string(),
                })?;
        }
        image::ImageFormat::WebP => {
            // WebP encoder - use lossy encoding with quality
            let encoder = image::codecs::webp::WebPEncoder::new_lossless(writer);
            img.write_with_encoder(encoder)
                .map_err(|e| ImgEditError::WriteError {
                    path: output_path.display().to_string(),
                    reason: e.to_string(),
                })?;
        }
        image::ImageFormat::Ico => {
            let encoder = image::codecs::ico::IcoEncoder::new(writer);
            img.write_with_encoder(encoder)
                .map_err(|e| ImgEditError::WriteError {
                    path: output_path.display().to_string(),
                    reason: e.to_string(),
                })?;
        }
        _ => {
            return Err(ImgEditError::UnsupportedFormat(format!(
                "Format {:?} not supported for writing",
                format
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_format_from_extension() {
        assert!(matches!(
            determine_format(Path::new("test.png"), None),
            Ok(image::ImageFormat::Png)
        ));
        assert!(matches!(
            determine_format(Path::new("test.jpg"), None),
            Ok(image::ImageFormat::Jpeg)
        ));
        assert!(matches!(
            determine_format(Path::new("test.jpeg"), None),
            Ok(image::ImageFormat::Jpeg)
        ));
        assert!(matches!(
            determine_format(Path::new("test.gif"), None),
            Ok(image::ImageFormat::Gif)
        ));
        assert!(matches!(
            determine_format(Path::new("test.bmp"), None),
            Ok(image::ImageFormat::Bmp)
        ));
        assert!(matches!(
            determine_format(Path::new("test.tiff"), None),
            Ok(image::ImageFormat::Tiff)
        ));
        assert!(matches!(
            determine_format(Path::new("test.webp"), None),
            Ok(image::ImageFormat::WebP)
        ));
        assert!(matches!(
            determine_format(Path::new("test.ico"), None),
            Ok(image::ImageFormat::Ico)
        ));
    }

    #[test]
    fn test_determine_format_explicit_overrides() {
        // Explicit format should override extension
        assert!(matches!(
            determine_format(Path::new("test.png"), Some(ImageFormat::Jpeg)),
            Ok(image::ImageFormat::Jpeg)
        ));
    }

    #[test]
    fn test_determine_format_unknown_extension() {
        let result = determine_format(Path::new("test.xyz"), None);
        assert!(result.is_err());
    }

    #[test]
    fn test_determine_format_no_extension() {
        let result = determine_format(Path::new("test"), None);
        assert!(result.is_err());
    }

    #[test]
    fn test_determine_format_case_insensitive() {
        assert!(matches!(
            determine_format(Path::new("test.PNG"), None),
            Ok(image::ImageFormat::Png)
        ));
        assert!(matches!(
            determine_format(Path::new("test.JPG"), None),
            Ok(image::ImageFormat::Jpeg)
        ));
    }

    #[test]
    fn test_determine_format_tif_extension() {
        assert!(matches!(
            determine_format(Path::new("test.tif"), None),
            Ok(image::ImageFormat::Tiff)
        ));
    }

    #[test]
    fn test_image_format_from_cli_all_variants() {
        use crate::cli::args::ImageFormat;

        assert!(matches!(
            image_format_from_cli(ImageFormat::Png),
            image::ImageFormat::Png
        ));
        assert!(matches!(
            image_format_from_cli(ImageFormat::Jpeg),
            image::ImageFormat::Jpeg
        ));
        assert!(matches!(
            image_format_from_cli(ImageFormat::Gif),
            image::ImageFormat::Gif
        ));
        assert!(matches!(
            image_format_from_cli(ImageFormat::Bmp),
            image::ImageFormat::Bmp
        ));
        assert!(matches!(
            image_format_from_cli(ImageFormat::Tiff),
            image::ImageFormat::Tiff
        ));
        assert!(matches!(
            image_format_from_cli(ImageFormat::Webp),
            image::ImageFormat::WebP
        ));
        assert!(matches!(
            image_format_from_cli(ImageFormat::Ico),
            image::ImageFormat::Ico
        ));
    }

    #[test]
    fn test_save_with_format_png() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let output = temp_dir.path().join("output.png");
        let img = DynamicImage::new_rgba8(10, 10);

        let result = save_with_format(&img, &output, image::ImageFormat::Png, 90);
        assert!(result.is_ok());
        assert!(output.exists());
    }

    #[test]
    fn test_save_with_format_jpeg() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let output = temp_dir.path().join("output.jpg");
        let img = DynamicImage::new_rgba8(10, 10);

        let result = save_with_format(&img, &output, image::ImageFormat::Jpeg, 85);
        assert!(result.is_ok());
        assert!(output.exists());
    }

    #[test]
    fn test_save_with_format_bmp() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let output = temp_dir.path().join("output.bmp");
        let img = DynamicImage::new_rgba8(10, 10);

        let result = save_with_format(&img, &output, image::ImageFormat::Bmp, 90);
        assert!(result.is_ok());
        assert!(output.exists());
    }

    #[test]
    fn test_save_with_format_gif() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let output = temp_dir.path().join("output.gif");
        let img = DynamicImage::new_rgba8(10, 10);

        let result = save_with_format(&img, &output, image::ImageFormat::Gif, 90);
        assert!(result.is_ok());
        assert!(output.exists());
    }

    #[test]
    fn test_save_with_format_tiff() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let output = temp_dir.path().join("output.tiff");
        let img = DynamicImage::new_rgba8(10, 10);

        let result = save_with_format(&img, &output, image::ImageFormat::Tiff, 90);
        assert!(result.is_ok());
        assert!(output.exists());
    }

    #[test]
    fn test_save_with_format_webp() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let output = temp_dir.path().join("output.webp");
        let img = DynamicImage::new_rgba8(10, 10);

        let result = save_with_format(&img, &output, image::ImageFormat::WebP, 90);
        assert!(result.is_ok());
        assert!(output.exists());
    }

    #[test]
    fn test_save_with_format_ico() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let output = temp_dir.path().join("output.ico");
        // ICO requires specific dimensions, use 32x32
        let img = DynamicImage::new_rgba8(32, 32);

        let result = save_with_format(&img, &output, image::ImageFormat::Ico, 90);
        assert!(result.is_ok());
        assert!(output.exists());
    }
}
