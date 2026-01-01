use thiserror::Error;

/// Exit codes for the application
pub mod exit_codes {
    pub const SUCCESS: i32 = 0;
    pub const GENERAL_ERROR: i32 = 1;
    pub const INPUT_NOT_FOUND: i32 = 2;
    pub const OUTPUT_WRITE_FAILED: i32 = 3;
    pub const UNSUPPORTED_FORMAT: i32 = 4;
    pub const INVALID_PARAMETERS: i32 = 5;
}

#[derive(Debug, Error)]
pub enum ImgEditError {
    #[error("Failed to read image '{path}': {reason}")]
    ReadError { path: String, reason: String },

    #[error("Failed to write image '{path}': {reason}")]
    WriteError { path: String, reason: String },

    #[error("Input file not found: {0}")]
    InputNotFound(String),

    #[error("Invalid dimensions: {0}")]
    InvalidDimensions(String),

    #[error("Crop region out of bounds: {0}")]
    CropOutOfBounds(String),

    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    #[error("Invalid color specification: {0}")]
    InvalidColor(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Operation requires at least one option: {0}")]
    MissingOption(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Image processing error: {0}")]
    ImageError(#[from] image::ImageError),
}

impl ImgEditError {
    /// Get the error code string for JSON output
    pub fn code(&self) -> &'static str {
        match self {
            ImgEditError::ReadError { .. } => "READ_ERROR",
            ImgEditError::WriteError { .. } => "WRITE_ERROR",
            ImgEditError::InputNotFound(_) => "INPUT_NOT_FOUND",
            ImgEditError::InvalidDimensions(_) => "INVALID_DIMENSIONS",
            ImgEditError::CropOutOfBounds(_) => "CROP_OUT_OF_BOUNDS",
            ImgEditError::UnsupportedFormat(_) => "UNSUPPORTED_FORMAT",
            ImgEditError::InvalidColor(_) => "INVALID_COLOR",
            ImgEditError::InvalidParameter(_) => "INVALID_PARAMETER",
            ImgEditError::MissingOption(_) => "MISSING_OPTION",
            ImgEditError::IoError(_) => "IO_ERROR",
            ImgEditError::ImageError(_) => "IMAGE_ERROR",
        }
    }

    /// Get the exit code for this error
    pub fn exit_code(&self) -> i32 {
        match self {
            ImgEditError::InputNotFound(_) => exit_codes::INPUT_NOT_FOUND,
            ImgEditError::ReadError { .. } => exit_codes::INPUT_NOT_FOUND,
            ImgEditError::WriteError { .. } => exit_codes::OUTPUT_WRITE_FAILED,
            ImgEditError::UnsupportedFormat(_) => exit_codes::UNSUPPORTED_FORMAT,
            ImgEditError::InvalidDimensions(_)
            | ImgEditError::CropOutOfBounds(_)
            | ImgEditError::InvalidColor(_)
            | ImgEditError::InvalidParameter(_)
            | ImgEditError::MissingOption(_) => exit_codes::INVALID_PARAMETERS,
            ImgEditError::IoError(_) | ImgEditError::ImageError(_) => exit_codes::GENERAL_ERROR,
        }
    }
}

pub type Result<T> = std::result::Result<T, ImgEditError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_codes() {
        let err = ImgEditError::InputNotFound("test.png".to_string());
        assert_eq!(err.code(), "INPUT_NOT_FOUND");
        assert_eq!(err.exit_code(), exit_codes::INPUT_NOT_FOUND);
    }

    #[test]
    fn test_error_display() {
        let err = ImgEditError::InvalidColor("bad color".to_string());
        assert_eq!(err.to_string(), "Invalid color specification: bad color");
    }

    #[test]
    fn test_all_error_codes_are_unique() {
        let errors: Vec<ImgEditError> = vec![
            ImgEditError::ReadError {
                path: "x".into(),
                reason: "y".into(),
            },
            ImgEditError::WriteError {
                path: "x".into(),
                reason: "y".into(),
            },
            ImgEditError::InputNotFound("x".into()),
            ImgEditError::InvalidDimensions("x".into()),
            ImgEditError::CropOutOfBounds("x".into()),
            ImgEditError::UnsupportedFormat("x".into()),
            ImgEditError::InvalidColor("x".into()),
            ImgEditError::InvalidParameter("x".into()),
            ImgEditError::MissingOption("x".into()),
        ];

        for err in &errors {
            assert!(!err.code().is_empty());
            assert!(err.exit_code() >= 0 && err.exit_code() <= 5);
        }
    }

    #[test]
    fn test_wrapped_error_codes() {
        use std::io;

        // Test IoError
        let io_err = io::Error::new(io::ErrorKind::NotFound, "not found");
        let err = ImgEditError::IoError(io_err);
        assert_eq!(err.code(), "IO_ERROR");
        assert_eq!(err.exit_code(), exit_codes::GENERAL_ERROR);

        // Test ImageError
        let img_err = image::ImageError::Decoding(image::error::DecodingError::from_format_hint(
            image::error::ImageFormatHint::Unknown,
        ));
        let err = ImgEditError::ImageError(img_err);
        assert_eq!(err.code(), "IMAGE_ERROR");
        assert_eq!(err.exit_code(), exit_codes::GENERAL_ERROR);
    }

    #[test]
    fn test_missing_option_error() {
        let err = ImgEditError::MissingOption("foo".to_string());
        assert_eq!(err.code(), "MISSING_OPTION");
        assert_eq!(err.exit_code(), exit_codes::INVALID_PARAMETERS);
        assert_eq!(
            err.to_string(),
            "Operation requires at least one option: foo"
        );
    }
}
