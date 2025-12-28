mod common;

use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_info_command_text_output() {
    let temp_dir = TempDir::new().unwrap();
    let img_path = temp_dir.path().join("test.png");

    // Create a test image
    let img = common::create_test_rgba_image(64, 48);
    img.save(&img_path).unwrap();

    // Run the info command
    let output = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args(["info", img_path.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("64x48"), "Should contain dimensions");
    assert!(stdout.contains("PNG"), "Should contain format");
    assert!(stdout.contains("RGBA"), "Should contain color type");
}

#[test]
fn test_info_command_json_output() {
    let temp_dir = TempDir::new().unwrap();
    let img_path = temp_dir.path().join("test.png");

    // Create a test image
    let img = common::create_test_rgba_image(100, 80);
    img.save(&img_path).unwrap();

    // Run the info command with --json
    let output = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args(["info", "--json", img_path.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("Should be valid JSON");

    assert_eq!(json["success"], true);
    assert_eq!(json["command"], "info");
    assert_eq!(json["details"]["width"], 100);
    assert_eq!(json["details"]["height"], 80);
    assert_eq!(json["details"]["format"], "PNG");
    assert_eq!(json["details"]["color_type"], "RGBA");
    assert_eq!(json["details"]["bit_depth"], 8);
}

#[test]
fn test_info_command_quiet_mode() {
    let temp_dir = TempDir::new().unwrap();
    let img_path = temp_dir.path().join("test.png");

    let img = common::create_test_rgba_image(32, 32);
    img.save(&img_path).unwrap();

    // Run with --quiet
    let output = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args(["info", "--quiet", img_path.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert!(
        output.stdout.is_empty(),
        "Quiet mode should produce no output"
    );
}

#[test]
fn test_info_command_nonexistent_file() {
    let output = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args(["info", "nonexistent_file.png"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(2)); // INPUT_NOT_FOUND exit code

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("not found") || stderr.contains("nonexistent"));
}

#[test]
fn test_info_command_nonexistent_file_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args(["info", "--json", "nonexistent_file.png"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    let json: serde_json::Value = serde_json::from_str(&stderr).expect("Should be valid JSON");

    assert_eq!(json["success"], false);
    assert_eq!(json["command"], "info");
    assert_eq!(json["code"], "INPUT_NOT_FOUND");
}

#[test]
fn test_help_output() {
    let output = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args(["--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("AI systems"));
    assert!(stdout.contains("info"));
    assert!(stdout.contains("crop"));
    assert!(stdout.contains("--json"));
}

#[test]
fn test_info_subcommand_help() {
    let output = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args(["info", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("dimensions"));
    assert!(stdout.contains("format"));
}

#[test]
fn test_version_output() {
    let output = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args(["--version"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("mdimgedit"));
    assert!(stdout.contains("0.1.0"));
}
