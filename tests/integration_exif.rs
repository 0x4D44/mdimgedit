mod common;

use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_exif_command_no_exif_data() {
    let temp_dir = TempDir::new().unwrap();
    let img_path = temp_dir.path().join("test.png");

    // Create a test PNG image (PNGs typically don't have EXIF)
    let img = common::create_test_rgba_image(64, 48);
    img.save(&img_path).unwrap();

    // Run the exif command
    let output = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args(["exif", img_path.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("No EXIF data found"),
        "Should indicate no EXIF data"
    );
}

#[test]
fn test_exif_command_json_no_exif() {
    let temp_dir = TempDir::new().unwrap();
    let img_path = temp_dir.path().join("test.png");

    let img = common::create_test_rgba_image(100, 80);
    img.save(&img_path).unwrap();

    // Run the exif command with --json
    let output = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args(["exif", "--json", img_path.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("Should be valid JSON");

    assert_eq!(json["success"], true);
    assert_eq!(json["command"], "exif");
    assert_eq!(json["details"]["has_exif"], false);
    assert_eq!(json["details"]["field_count"], 0);
}

#[test]
fn test_exif_command_quiet_mode() {
    let temp_dir = TempDir::new().unwrap();
    let img_path = temp_dir.path().join("test.png");

    let img = common::create_test_rgba_image(32, 32);
    img.save(&img_path).unwrap();

    // Run with --quiet
    let output = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args(["exif", "--quiet", img_path.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert!(
        output.stdout.is_empty(),
        "Quiet mode should produce no output"
    );
}

#[test]
fn test_exif_command_verbose_no_exif() {
    let temp_dir = TempDir::new().unwrap();
    let img_path = temp_dir.path().join("test.png");

    let img = common::create_test_rgba_image(32, 32);
    img.save(&img_path).unwrap();

    // Run with --verbose
    let output = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args(["exif", "--verbose", img_path.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("No EXIF data found"),
        "Verbose mode should still indicate no EXIF"
    );
}

#[test]
fn test_exif_command_tag_not_found() {
    let temp_dir = TempDir::new().unwrap();
    let img_path = temp_dir.path().join("test.png");

    let img = common::create_test_rgba_image(32, 32);
    img.save(&img_path).unwrap();

    // Run with --tag (which won't exist)
    let output = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args(["exif", "--tag", "Make", img_path.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("not found"),
        "Should indicate tag not found"
    );
}

#[test]
fn test_exif_command_tag_json_not_found() {
    let temp_dir = TempDir::new().unwrap();
    let img_path = temp_dir.path().join("test.png");

    let img = common::create_test_rgba_image(32, 32);
    img.save(&img_path).unwrap();

    // Run with --tag and --json
    let output = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "exif",
            "--json",
            "--tag",
            "Model",
            img_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("Should be valid JSON");

    assert_eq!(json["success"], true);
    assert_eq!(json["details"]["tag"], "Model");
    assert_eq!(json["details"]["found"], false);
}

#[test]
fn test_exif_command_nonexistent_file() {
    let output = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args(["exif", "nonexistent_file.jpg"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(2)); // INPUT_NOT_FOUND exit code
}

#[test]
fn test_exif_command_nonexistent_file_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args(["exif", "--json", "nonexistent_file.jpg"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    let json: serde_json::Value = serde_json::from_str(&stderr).expect("Should be valid JSON");

    assert_eq!(json["success"], false);
    assert_eq!(json["command"], "exif");
    assert_eq!(json["code"], "INPUT_NOT_FOUND");
}

#[test]
fn test_exif_subcommand_help() {
    let output = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args(["exif", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("EXIF"));
    assert!(stdout.contains("--verbose"));
    assert!(stdout.contains("--tag"));
}
