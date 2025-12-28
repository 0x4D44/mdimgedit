mod common;

use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_rotate_with_expand() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(100, 50);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "rotate",
            "--degrees",
            "45",
            "--expand",
            "--background",
            "#FF0000",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(
        result.status.success(),
        "{:?}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(output.exists());

    // Expanded canvas should be larger than original
    let out_img = image::open(&output).unwrap();
    assert!(out_img.width() > 100);
    assert!(out_img.height() > 50);
}

#[test]
fn test_rotate_json_output() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(100, 100);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "rotate",
            "--json",
            "--degrees",
            "90",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let stdout = String::from_utf8_lossy(&result.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("Should be valid JSON");
    assert_eq!(json["success"], true);
    assert_eq!(json["command"], "rotate");
}

#[test]
fn test_flip_json_output() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(50, 50);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "flip",
            "--json",
            "--horizontal",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let stdout = String::from_utf8_lossy(&result.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("Should be valid JSON");
    assert_eq!(json["success"], true);
    assert_eq!(json["command"], "flip");
}

#[test]
fn test_resize_json_output() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(100, 100);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "resize",
            "--json",
            "--width",
            "50",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let stdout = String::from_utf8_lossy(&result.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("Should be valid JSON");
    assert_eq!(json["success"], true);
    assert_eq!(json["command"], "resize");
}

#[test]
fn test_fit_json_output() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(200, 100);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "fit",
            "--json",
            "--max-width",
            "100",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let stdout = String::from_utf8_lossy(&result.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("Should be valid JSON");
    assert_eq!(json["success"], true);
    assert_eq!(json["command"], "fit");
}

#[test]
fn test_quiet_mode_suppresses_output() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(50, 50);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "rotate",
            "--quiet",
            "--degrees",
            "90",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());
    // Quiet mode should produce no stdout output
    assert!(result.stdout.is_empty());
}

#[test]
fn test_resize_height_only() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(100, 200);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "resize",
            "--height",
            "100",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let out_img = image::open(&output).unwrap();
    assert_eq!(out_img.height(), 100);
    // Aspect ratio should be preserved
    assert_eq!(out_img.width(), 50);
}

#[test]
fn test_crop_basic() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    // Create 100x100 test image
    let img = common::create_test_rgba_image(100, 100);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "crop",
            "--width",
            "50",
            "--height",
            "50",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(
        result.status.success(),
        "Command failed: {:?}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(output.exists());

    // Verify output dimensions
    let out_img = image::open(&output).unwrap();
    assert_eq!(out_img.width(), 50);
    assert_eq!(out_img.height(), 50);
}

#[test]
fn test_crop_with_anchor_center() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(100, 100);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "crop",
            "--width",
            "50",
            "--height",
            "50",
            "--anchor",
            "center",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());
    let out_img = image::open(&output).unwrap();
    assert_eq!(out_img.width(), 50);
    assert_eq!(out_img.height(), 50);
}

#[test]
fn test_crop_json_output() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(100, 80);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "crop",
            "--json",
            "--width",
            "50",
            "--height",
            "40",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let stdout = String::from_utf8_lossy(&result.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("Should be valid JSON");

    assert_eq!(json["success"], true);
    assert_eq!(json["command"], "crop");
    assert_eq!(json["details"]["original_width"], 100);
    assert_eq!(json["details"]["original_height"], 80);
    assert_eq!(json["details"]["result_width"], 50);
    assert_eq!(json["details"]["result_height"], 40);
}

#[test]
fn test_rotate_90() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    // 100x50 image
    let img = common::create_test_rgba_image(100, 50);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "rotate",
            "--degrees",
            "90",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    // After 90 degree rotation, dimensions should swap
    let out_img = image::open(&output).unwrap();
    assert_eq!(out_img.width(), 50);
    assert_eq!(out_img.height(), 100);
}

#[test]
fn test_rotate_180() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(100, 50);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "rotate",
            "--degrees",
            "180",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let out_img = image::open(&output).unwrap();
    assert_eq!(out_img.width(), 100);
    assert_eq!(out_img.height(), 50);
}

#[test]
fn test_flip_horizontal() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(100, 100);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "flip",
            "--horizontal",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());
    assert!(output.exists());
}

#[test]
fn test_flip_vertical() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(100, 100);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "flip",
            "--vertical",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());
    assert!(output.exists());
}

#[test]
fn test_flip_both() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(100, 100);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "flip",
            "--horizontal",
            "--vertical",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());
    assert!(output.exists());
}

#[test]
fn test_flip_requires_direction() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(100, 100);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args(["flip", input.to_str().unwrap(), output.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(!result.status.success());
}

#[test]
fn test_resize_exact_dimensions() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(100, 100);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "resize",
            "--width",
            "50",
            "--height",
            "50",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let out_img = image::open(&output).unwrap();
    assert_eq!(out_img.width(), 50);
    assert_eq!(out_img.height(), 50);
}

#[test]
fn test_resize_width_only_preserves_aspect() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    // 100x50 aspect ratio 2:1
    let img = common::create_test_rgba_image(100, 50);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "resize",
            "--width",
            "50",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let out_img = image::open(&output).unwrap();
    assert_eq!(out_img.width(), 50);
    assert_eq!(out_img.height(), 25); // Preserved 2:1 ratio
}

#[test]
fn test_resize_scale() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(100, 100);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "resize",
            "--scale",
            "0.5",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let out_img = image::open(&output).unwrap();
    assert_eq!(out_img.width(), 50);
    assert_eq!(out_img.height(), 50);
}

#[test]
fn test_resize_with_filter() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(100, 100);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "resize",
            "--scale",
            "2",
            "--filter",
            "nearest",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let out_img = image::open(&output).unwrap();
    assert_eq!(out_img.width(), 200);
    assert_eq!(out_img.height(), 200);
}

#[test]
fn test_fit_within_bounds() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    // 200x100 image (2:1 ratio)
    let img = common::create_test_rgba_image(200, 100);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "fit",
            "--max-width",
            "100",
            "--max-height",
            "100",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let out_img = image::open(&output).unwrap();
    // Width is limiting factor, so 100x50
    assert_eq!(out_img.width(), 100);
    assert_eq!(out_img.height(), 50);
}

#[test]
fn test_fit_no_upscale() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    // Small 50x50 image
    let img = common::create_test_rgba_image(50, 50);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "fit",
            "--max-width",
            "100",
            "--max-height",
            "100",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let out_img = image::open(&output).unwrap();
    // Should not upscale
    assert_eq!(out_img.width(), 50);
    assert_eq!(out_img.height(), 50);
}

#[test]
fn test_fit_with_upscale() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(50, 50);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "fit",
            "--max-width",
            "100",
            "--max-height",
            "100",
            "--upscale",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let out_img = image::open(&output).unwrap();
    // Should upscale to 100x100
    assert_eq!(out_img.width(), 100);
    assert_eq!(out_img.height(), 100);
}

#[test]
fn test_overwrite_protection() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(100, 100);
    img.save(&input).unwrap();
    img.save(&output).unwrap(); // Create existing output

    // Without --overwrite, should fail
    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "crop",
            "--width",
            "50",
            "--height",
            "50",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(!result.status.success());

    // With --overwrite, should succeed
    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "crop",
            "--width",
            "50",
            "--height",
            "50",
            "--overwrite",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());
}
