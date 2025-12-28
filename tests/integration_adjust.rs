mod common;

use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_brightness_increase() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    // Create a gray image
    let img = image::RgbaImage::from_fn(10, 10, |_, _| image::Rgba([100, 100, 100, 255]));
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "brightness",
            "--value",
            "50",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let out_img = image::open(&output).unwrap().to_rgba8();
    let pixel = out_img.get_pixel(5, 5);
    assert_eq!(pixel[0], 150); // 100 + 50
}

#[test]
fn test_brightness_decrease() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = image::RgbaImage::from_fn(10, 10, |_, _| image::Rgba([100, 100, 100, 255]));
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "brightness",
            "--value",
            "-50",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let out_img = image::open(&output).unwrap().to_rgba8();
    let pixel = out_img.get_pixel(5, 5);
    assert_eq!(pixel[0], 50); // 100 - 50
}

#[test]
fn test_contrast_increase() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(50, 50);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "contrast",
            "--value",
            "1.5",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());
    assert!(output.exists());
}

#[test]
fn test_contrast_decrease() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(50, 50);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "contrast",
            "--value",
            "0.5",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());
    assert!(output.exists());
}

#[test]
fn test_gamma_lighten() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    // Create a mid-gray image
    let img = image::RgbaImage::from_fn(10, 10, |_, _| image::Rgba([128, 128, 128, 255]));
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "gamma",
            "--value",
            "0.5",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let out_img = image::open(&output).unwrap().to_rgba8();
    let pixel = out_img.get_pixel(5, 5);
    assert!(pixel[0] > 128); // Should be lighter
}

#[test]
fn test_gamma_darken() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = image::RgbaImage::from_fn(10, 10, |_, _| image::Rgba([128, 128, 128, 255]));
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "gamma",
            "--value",
            "2.0",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let out_img = image::open(&output).unwrap().to_rgba8();
    let pixel = out_img.get_pixel(5, 5);
    assert!(pixel[0] < 128); // Should be darker
}

#[test]
fn test_blur_basic() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(50, 50);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "blur",
            "--radius",
            "2.0",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());
    assert!(output.exists());

    let out_img = image::open(&output).unwrap();
    assert_eq!(out_img.width(), 50);
    assert_eq!(out_img.height(), 50);
}

#[test]
fn test_sharpen_basic() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(50, 50);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args(["sharpen", input.to_str().unwrap(), output.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());
    assert!(output.exists());
}

#[test]
fn test_sharpen_with_params() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(50, 50);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "sharpen",
            "--amount",
            "2.0",
            "--radius",
            "1.5",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());
    assert!(output.exists());
}

#[test]
fn test_brightness_json_output() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(64, 48);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "brightness",
            "--json",
            "--value",
            "25",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let stdout = String::from_utf8_lossy(&result.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("Should be valid JSON");

    assert_eq!(json["success"], true);
    assert_eq!(json["command"], "brightness");
    assert_eq!(json["details"]["original_width"], 64);
    assert_eq!(json["details"]["original_height"], 48);
}
