use crate::error::{ImgEditError, Result};
use exif::{In, Reader, Tag, Value};
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Represents a single EXIF field
#[derive(Debug, Clone, Serialize)]
pub struct ExifField {
    pub tag: String,
    pub ifd: String,
    pub value: String,
    pub description: Option<String>,
}

/// Represents all EXIF data from an image
#[derive(Debug, Clone, Default, Serialize)]
pub struct ExifData {
    pub fields: Vec<ExifField>,
    pub has_exif: bool,
    pub camera_make: Option<String>,
    pub camera_model: Option<String>,
    pub date_time: Option<String>,
    pub exposure_time: Option<String>,
    pub f_number: Option<String>,
    pub iso: Option<String>,
    pub focal_length: Option<String>,
    pub gps_latitude: Option<String>,
    pub gps_longitude: Option<String>,
    pub image_width: Option<u32>,
    pub image_height: Option<u32>,
    pub orientation: Option<u16>,
    pub software: Option<String>,
    pub artist: Option<String>,
    pub copyright: Option<String>,
}

/// Read EXIF data from an image file
pub fn read_exif<P: AsRef<Path>>(path: P) -> Result<ExifData> {
    let file = File::open(path.as_ref()).map_err(|e| {
        ImgEditError::InputNotFound(format!(
            "Cannot open file '{}': {}",
            path.as_ref().display(),
            e
        ))
    })?;
    let mut reader = BufReader::new(file);

    let exif = match Reader::new().read_from_container(&mut reader) {
        Ok(exif) => exif,
        Err(exif::Error::NotFound(_)) => {
            return Ok(ExifData::default());
        }
        Err(e) => {
            return Err(ImgEditError::UnsupportedFormat(format!(
                "Failed to read EXIF data: {}",
                e
            )));
        }
    };

    let mut data = ExifData {
        has_exif: true,
        ..Default::default()
    };

    // Collect all fields
    for field in exif.fields() {
        let tag_name = format!("{}", field.tag);
        let ifd_name = match field.ifd_num {
            In::PRIMARY => "Primary".to_string(),
            In::THUMBNAIL => "Thumbnail".to_string(),
            _ => format!("{:?}", field.ifd_num),
        };

        let value_str = field.display_value().to_string();
        let description = field.tag.description().map(|s| s.to_string());

        data.fields.push(ExifField {
            tag: tag_name.clone(),
            ifd: ifd_name,
            value: value_str,
            description,
        });

        // Extract common fields
        match field.tag {
            Tag::Make => {
                data.camera_make = Some(get_string_value(&field.value));
            }
            Tag::Model => {
                data.camera_model = Some(get_string_value(&field.value));
            }
            Tag::DateTime | Tag::DateTimeOriginal => {
                if data.date_time.is_none() {
                    data.date_time = Some(get_string_value(&field.value));
                }
            }
            Tag::ExposureTime => {
                data.exposure_time = Some(field.display_value().to_string());
            }
            Tag::FNumber => {
                data.f_number = Some(field.display_value().to_string());
            }
            Tag::PhotographicSensitivity => {
                data.iso = Some(field.display_value().to_string());
            }
            Tag::FocalLength => {
                data.focal_length = Some(field.display_value().to_string());
            }
            Tag::GPSLatitude => {
                data.gps_latitude = Some(field.display_value().to_string());
            }
            Tag::GPSLongitude => {
                data.gps_longitude = Some(field.display_value().to_string());
            }
            Tag::PixelXDimension => {
                if let Some(val) = get_uint_value(&field.value) {
                    data.image_width = Some(val);
                }
            }
            Tag::PixelYDimension => {
                if let Some(val) = get_uint_value(&field.value) {
                    data.image_height = Some(val);
                }
            }
            Tag::Orientation => {
                if let Some(val) = get_uint_value(&field.value) {
                    data.orientation = Some(val as u16);
                }
            }
            Tag::Software => {
                data.software = Some(get_string_value(&field.value));
            }
            Tag::Artist => {
                data.artist = Some(get_string_value(&field.value));
            }
            Tag::Copyright => {
                data.copyright = Some(get_string_value(&field.value));
            }
            _ => {}
        }
    }

    Ok(data)
}

/// Get specific EXIF fields by tag name
pub fn get_exif_field<P: AsRef<Path>>(path: P, tag_name: &str) -> Result<Option<ExifField>> {
    let data = read_exif(path)?;
    let tag_lower = tag_name.to_lowercase();

    Ok(data
        .fields
        .into_iter()
        .find(|f| f.tag.to_lowercase() == tag_lower))
}

/// Get all EXIF fields as a HashMap for easy lookup
pub fn get_exif_map<P: AsRef<Path>>(path: P) -> Result<HashMap<String, String>> {
    let data = read_exif(path)?;
    let mut map = HashMap::new();

    for field in data.fields {
        map.insert(field.tag, field.value);
    }

    Ok(map)
}

fn get_string_value(value: &Value) -> String {
    match value {
        Value::Ascii(ref strings) => strings
            .iter()
            .map(|s| String::from_utf8_lossy(s).into_owned())
            .collect::<Vec<_>>()
            .join(", "),
        _ => format!("{:?}", value),
    }
}

fn get_uint_value(value: &Value) -> Option<u32> {
    match value {
        Value::Short(ref vals) if !vals.is_empty() => Some(vals[0] as u32),
        Value::Long(ref vals) if !vals.is_empty() => Some(vals[0]),
        _ => None,
    }
}

/// Format EXIF data for human-readable text output
pub fn format_exif_text(data: &ExifData) -> String {
    if !data.has_exif {
        return "No EXIF data found".to_string();
    }

    let mut lines = Vec::new();
    lines.push("EXIF Information:".to_string());
    lines.push("=================".to_string());

    // Show summary first
    if let Some(ref make) = data.camera_make {
        lines.push(format!("Camera Make: {}", make.trim()));
    }
    if let Some(ref model) = data.camera_model {
        lines.push(format!("Camera Model: {}", model.trim()));
    }
    if let Some(ref dt) = data.date_time {
        lines.push(format!("Date/Time: {}", dt.trim()));
    }
    if let Some(ref exp) = data.exposure_time {
        lines.push(format!("Exposure Time: {}", exp));
    }
    if let Some(ref f) = data.f_number {
        lines.push(format!("F-Number: {}", f));
    }
    if let Some(ref iso) = data.iso {
        lines.push(format!("ISO: {}", iso));
    }
    if let Some(ref fl) = data.focal_length {
        lines.push(format!("Focal Length: {}", fl));
    }
    if let Some(ref software) = data.software {
        lines.push(format!("Software: {}", software.trim()));
    }

    // GPS if available
    if data.gps_latitude.is_some() || data.gps_longitude.is_some() {
        lines.push(String::new());
        lines.push("GPS Information:".to_string());
        if let Some(ref lat) = data.gps_latitude {
            lines.push(format!("  Latitude: {}", lat));
        }
        if let Some(ref lon) = data.gps_longitude {
            lines.push(format!("  Longitude: {}", lon));
        }
    }

    lines.push(String::new());
    lines.push(format!("Total EXIF fields: {}", data.fields.len()));

    lines.join("\n")
}

/// Format EXIF data with all fields (verbose output)
pub fn format_exif_verbose(data: &ExifData) -> String {
    if !data.has_exif {
        return "No EXIF data found".to_string();
    }

    let mut lines = Vec::new();
    lines.push("EXIF Information (All Fields):".to_string());
    lines.push("==============================".to_string());

    for field in &data.fields {
        let desc_str = field
            .description
            .as_ref()
            .map(|d| format!(" ({})", d))
            .unwrap_or_default();
        lines.push(format!(
            "[{}] {}: {}{}",
            field.ifd, field.tag, field.value, desc_str
        ));
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exif_data_default() {
        let data = ExifData::default();
        assert!(!data.has_exif);
        assert!(data.fields.is_empty());
        assert!(data.camera_make.is_none());
    }

    #[test]
    fn test_format_no_exif() {
        let data = ExifData::default();
        let text = format_exif_text(&data);
        assert_eq!(text, "No EXIF data found");
    }

    #[test]
    fn test_format_with_exif() {
        let data = ExifData {
            has_exif: true,
            camera_make: Some("Canon".to_string()),
            camera_model: Some("EOS 5D".to_string()),
            fields: vec![ExifField {
                tag: "Make".to_string(),
                ifd: "Primary".to_string(),
                value: "Canon".to_string(),
                description: Some("Camera manufacturer".to_string()),
            }],
            ..Default::default()
        };

        let text = format_exif_text(&data);
        assert!(text.contains("Canon"));
        assert!(text.contains("EOS 5D"));
    }

    #[test]
    fn test_format_verbose() {
        let data = ExifData {
            has_exif: true,
            fields: vec![ExifField {
                tag: "Make".to_string(),
                ifd: "Primary".to_string(),
                value: "Nikon".to_string(),
                description: Some("Camera manufacturer".to_string()),
            }],
            ..Default::default()
        };

        let text = format_exif_verbose(&data);
        assert!(text.contains("[Primary] Make: Nikon"));
        assert!(text.contains("Camera manufacturer"));
    }

    #[test]
    fn test_read_nonexistent_file() {
        let result = read_exif("nonexistent_file.jpg");
        assert!(result.is_err());
    }
}
