use crate::error::ImgEditError;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Text,
    Json,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub success: bool,
    pub command: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub details: HashMap<String, serde_json::Value>,
}

impl SuccessResponse {
    pub fn new(command: &str) -> Self {
        Self {
            success: true,
            command: command.to_string(),
            input: None,
            output: None,
            details: HashMap::new(),
        }
    }

    pub fn with_input(mut self, input: &str) -> Self {
        self.input = Some(input.to_string());
        self
    }

    pub fn with_output(mut self, output: &str) -> Self {
        self.output = Some(output.to_string());
        self
    }

    pub fn with_detail<V: Into<serde_json::Value>>(mut self, key: &str, value: V) -> Self {
        self.details.insert(key.to_string(), value.into());
        self
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub command: String,
    pub error: String,
    pub code: String,
}

impl ErrorResponse {
    pub fn new(command: &str, err: &ImgEditError) -> Self {
        Self {
            success: false,
            command: command.to_string(),
            error: err.to_string(),
            code: err.code().to_string(),
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
    }
}

/// Print success output in the appropriate format
pub fn print_success(format: OutputFormat, response: &SuccessResponse, quiet: bool) {
    match format {
        OutputFormat::Json => {
            println!("{}", response.to_json());
        }
        OutputFormat::Text => {
            if !quiet {
                // Text output varies by command, handled by caller
            }
        }
    }
}

/// Print error output in the appropriate format
pub fn print_error(format: OutputFormat, command: &str, err: &ImgEditError) {
    match format {
        OutputFormat::Json => {
            let response = ErrorResponse::new(command, err);
            eprintln!("{}", response.to_json());
        }
        OutputFormat::Text => {
            eprintln!("Error: {}", err);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success_response_builder() {
        let response = SuccessResponse::new("info")
            .with_input("test.png")
            .with_detail("width", 1920)
            .with_detail("height", 1080);

        assert!(response.success);
        assert_eq!(response.command, "info");
        assert_eq!(response.input, Some("test.png".to_string()));
        assert_eq!(
            response.details.get("width"),
            Some(&serde_json::json!(1920))
        );
        assert_eq!(
            response.details.get("height"),
            Some(&serde_json::json!(1080))
        );
    }

    #[test]
    fn test_success_response_json() {
        let response = SuccessResponse::new("crop")
            .with_input("in.png")
            .with_output("out.png");

        let json = response.to_json();
        assert!(json.contains("\"success\": true"));
        assert!(json.contains("\"command\": \"crop\""));
    }

    #[test]
    fn test_error_response() {
        let err = ImgEditError::InputNotFound("missing.png".to_string());
        let response = ErrorResponse::new("info", &err);

        assert!(!response.success);
        assert_eq!(response.command, "info");
        assert_eq!(response.code, "INPUT_NOT_FOUND");
        assert!(response.error.contains("missing.png"));
    }

    #[test]
    fn test_error_response_json() {
        let err = ImgEditError::InvalidColor("bad".to_string());
        let response = ErrorResponse::new("pad", &err);

        let json = response.to_json();
        assert!(json.contains("\"success\": false"));
        assert!(json.contains("\"code\": \"INVALID_COLOR\""));
    }

    #[test]
    fn test_print_success_json() {
        let response = SuccessResponse::new("test")
            .with_input("in.png")
            .with_output("out.png");
        // Just verify it doesn't panic
        print_success(OutputFormat::Json, &response, false);
    }

    #[test]
    fn test_print_success_text_quiet() {
        let response = SuccessResponse::new("test");
        // Verify quiet mode doesn't panic
        print_success(OutputFormat::Text, &response, true);
    }

    #[test]
    fn test_print_success_text_not_quiet() {
        let response = SuccessResponse::new("test");
        // Verify non-quiet mode doesn't panic
        print_success(OutputFormat::Text, &response, false);
    }

    #[test]
    fn test_print_error_json() {
        let err = ImgEditError::InvalidParameter("bad param".to_string());
        // Verify JSON error output doesn't panic
        print_error(OutputFormat::Json, "test", &err);
    }

    #[test]
    fn test_print_error_text() {
        let err = ImgEditError::InvalidParameter("bad param".to_string());
        // Verify text error output doesn't panic
        print_error(OutputFormat::Text, "test", &err);
    }

    #[test]
    fn test_success_response_empty_details() {
        let response = SuccessResponse::new("test");
        let json = response.to_json();
        // Empty details should not be serialized
        assert!(!json.contains("details"));
    }

    #[test]
    fn test_success_response_no_input_output() {
        let response = SuccessResponse::new("test");
        let json = response.to_json();
        // Missing input/output should not be serialized
        assert!(!json.contains("input"));
        assert!(!json.contains("output"));
    }
}
