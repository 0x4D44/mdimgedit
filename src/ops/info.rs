use crate::error::{ImgEditError, Result};
use image::ImageReader;
use image::{ColorType, DynamicImage};
use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct ImageInfo {
    pub file: String,
    pub format: String,
    pub width: u32,
    pub height: u32,
    pub color_type: String,
    pub bit_depth: u8,
    pub file_size_bytes: u64,
}

impl ImageInfo {
    pub fn display(&self) -> String {
        let size_display = format_file_size(self.file_size_bytes);
        format!(
            "File: {}\n\
             Format: {}\n\
             Dimensions: {}x{}\n\
             Color Type: {}\n\
             Bit Depth: {}\n\
             File Size: {}",
            self.file,
            self.format,
            self.width,
            self.height,
            self.color_type,
            self.bit_depth,
            size_display
        )
    }
}

fn format_file_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

fn color_type_to_string(color_type: ColorType) -> String {
    match color_type {
        ColorType::L8 => "Grayscale".to_string(),
        ColorType::La8 => "Grayscale+Alpha".to_string(),
        ColorType::Rgb8 => "RGB".to_string(),
        ColorType::Rgba8 => "RGBA".to_string(),
        ColorType::L16 => "Grayscale16".to_string(),
        ColorType::La16 => "Grayscale16+Alpha".to_string(),
        ColorType::Rgb16 => "RGB16".to_string(),
        ColorType::Rgba16 => "RGBA16".to_string(),
        ColorType::Rgb32F => "RGB32F".to_string(),
        ColorType::Rgba32F => "RGBA32F".to_string(),
        _ => "Unknown".to_string(),
    }
}

fn color_type_bit_depth(color_type: ColorType) -> u8 {
    match color_type {
        ColorType::L8 | ColorType::La8 | ColorType::Rgb8 | ColorType::Rgba8 => 8,
        ColorType::L16 | ColorType::La16 | ColorType::Rgb16 | ColorType::Rgba16 => 16,
        ColorType::Rgb32F | ColorType::Rgba32F => 32,
        _ => 8,
    }
}

/// Load an image from a path
pub fn load_image(path: &Path) -> Result<DynamicImage> {
    if !path.exists() {
        return Err(ImgEditError::InputNotFound(path.display().to_string()));
    }

    ImageReader::open(path)
        .map_err(|e| ImgEditError::ReadError {
            path: path.display().to_string(),
            reason: e.to_string(),
        })?
        .decode()
        .map_err(|e| ImgEditError::ReadError {
            path: path.display().to_string(),
            reason: e.to_string(),
        })
}

/// Get information about an image file
pub fn get_image_info(path: &Path) -> Result<ImageInfo> {
    // Get file metadata for size
    let metadata = fs::metadata(path).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            ImgEditError::InputNotFound(path.display().to_string())
        } else {
            ImgEditError::IoError(e)
        }
    })?;

    // Read image to get dimensions and format
    let reader = ImageReader::open(path).map_err(|e| ImgEditError::ReadError {
        path: path.display().to_string(),
        reason: e.to_string(),
    })?;

    let format = reader
        .format()
        .map(|f| format!("{:?}", f).to_uppercase())
        .unwrap_or_else(|| "UNKNOWN".to_string());

    let img = reader.decode().map_err(|e| ImgEditError::ReadError {
        path: path.display().to_string(),
        reason: e.to_string(),
    })?;

    let color_type = img.color();

    Ok(ImageInfo {
        file: path.display().to_string(),
        format,
        width: img.width(),
        height: img.height(),
        color_type: color_type_to_string(color_type),
        bit_depth: color_type_bit_depth(color_type),
        file_size_bytes: metadata.len(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_file_size_bytes() {
        assert_eq!(format_file_size(500), "500 bytes");
        assert_eq!(format_file_size(0), "0 bytes");
    }

    #[test]
    fn test_format_file_size_kb() {
        assert_eq!(format_file_size(1024), "1.00 KB");
        assert_eq!(format_file_size(2048), "2.00 KB");
        assert_eq!(format_file_size(1536), "1.50 KB");
    }

    #[test]
    fn test_format_file_size_mb() {
        assert_eq!(format_file_size(1024 * 1024), "1.00 MB");
        assert_eq!(format_file_size(2 * 1024 * 1024), "2.00 MB");
    }

    #[test]
    fn test_format_file_size_gb() {
        assert_eq!(format_file_size(1024 * 1024 * 1024), "1.00 GB");
    }

    #[test]
    fn test_color_type_to_string() {
        assert_eq!(color_type_to_string(ColorType::Rgb8), "RGB");
        assert_eq!(color_type_to_string(ColorType::Rgba8), "RGBA");
        assert_eq!(color_type_to_string(ColorType::L8), "Grayscale");
        assert_eq!(color_type_to_string(ColorType::La8), "Grayscale+Alpha");
        assert_eq!(color_type_to_string(ColorType::L16), "Grayscale16");
        assert_eq!(color_type_to_string(ColorType::La16), "Grayscale16+Alpha");
        assert_eq!(color_type_to_string(ColorType::Rgb16), "RGB16");
        assert_eq!(color_type_to_string(ColorType::Rgba16), "RGBA16");
        assert_eq!(color_type_to_string(ColorType::Rgb32F), "RGB32F");
        assert_eq!(color_type_to_string(ColorType::Rgba32F), "RGBA32F");
    }

    #[test]
    fn test_color_type_bit_depth() {
        // 8-bit types
        assert_eq!(color_type_bit_depth(ColorType::L8), 8);
        assert_eq!(color_type_bit_depth(ColorType::La8), 8);
        assert_eq!(color_type_bit_depth(ColorType::Rgb8), 8);
        assert_eq!(color_type_bit_depth(ColorType::Rgba8), 8);
        // 16-bit types
        assert_eq!(color_type_bit_depth(ColorType::L16), 16);
        assert_eq!(color_type_bit_depth(ColorType::La16), 16);
        assert_eq!(color_type_bit_depth(ColorType::Rgb16), 16);
        assert_eq!(color_type_bit_depth(ColorType::Rgba16), 16);
        // 32-bit float types
        assert_eq!(color_type_bit_depth(ColorType::Rgb32F), 32);
        assert_eq!(color_type_bit_depth(ColorType::Rgba32F), 32);
    }

    #[test]
    fn test_load_nonexistent_image() {
        let result = load_image(Path::new("nonexistent.png"));
        assert!(result.is_err());
        match result {
            Err(ImgEditError::InputNotFound(path)) => {
                assert!(path.contains("nonexistent.png"));
            }
            _ => panic!("Expected InputNotFound error"),
        }
    }

    #[test]
    fn test_get_info_nonexistent() {
        let result = get_image_info(Path::new("nonexistent.png"));
        assert!(result.is_err());
    }

    #[test]
    fn test_image_info_display() {
        let info = ImageInfo {
            file: "test.png".to_string(),
            format: "PNG".to_string(),
            width: 800,
            height: 600,
            color_type: "RGBA".to_string(),
            bit_depth: 8,
            file_size_bytes: 1536,
        };

        let display = info.display();
        assert!(display.contains("test.png"));
        assert!(display.contains("800x600"));
        assert!(display.contains("PNG"));
        assert!(display.contains("RGBA"));
        assert!(display.contains("1.50 KB"));
    }
}
