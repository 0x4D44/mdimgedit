mod common;

use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_pad_all_sides() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(50, 50);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "pad",
            "--all",
            "10",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());
    assert!(output.exists());

    let out_img = image::open(&output).unwrap();
    assert_eq!(out_img.width(), 70); // 50 + 10 + 10
    assert_eq!(out_img.height(), 70); // 50 + 10 + 10
}

#[test]
fn test_pad_horizontal_vertical() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(50, 50);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "pad",
            "--horizontal",
            "20",
            "--vertical",
            "10",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let out_img = image::open(&output).unwrap();
    assert_eq!(out_img.width(), 90); // 50 + 20 + 20
    assert_eq!(out_img.height(), 70); // 50 + 10 + 10
}

#[test]
fn test_pad_individual_sides() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(50, 50);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "pad",
            "--top",
            "5",
            "--bottom",
            "10",
            "--left",
            "15",
            "--right",
            "20",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let out_img = image::open(&output).unwrap();
    assert_eq!(out_img.width(), 85); // 50 + 15 + 20
    assert_eq!(out_img.height(), 65); // 50 + 5 + 10
}

#[test]
fn test_pad_with_color() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    // Create a red image
    let img = image::RgbaImage::from_fn(10, 10, |_, _| image::Rgba([255, 0, 0, 255]));
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "pad",
            "--all",
            "5",
            "--color",
            "blue",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let out_img = image::open(&output).unwrap().to_rgba8();
    // Corner should be blue
    let corner = out_img.get_pixel(0, 0);
    assert_eq!(corner[0], 0); // R
    assert_eq!(corner[1], 0); // G
    assert_eq!(corner[2], 255); // B

    // Center should be red
    let center = out_img.get_pixel(10, 10);
    assert_eq!(center[0], 255); // R
    assert_eq!(center[1], 0); // G
    assert_eq!(center[2], 0); // B
}

#[test]
fn test_pad_with_hex_color() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(10, 10);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "pad",
            "--all",
            "5",
            "--color",
            "#FF00FF",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let out_img = image::open(&output).unwrap().to_rgba8();
    let corner = out_img.get_pixel(0, 0);
    assert_eq!(corner[0], 255); // R
    assert_eq!(corner[1], 0); // G
    assert_eq!(corner[2], 255); // B
}

#[test]
fn test_pad_no_padding_fails() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(10, 10);
    img.save(&input).unwrap();

    // No padding values specified
    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args(["pad", input.to_str().unwrap(), output.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(!result.status.success());
}

#[test]
fn test_pad_json_output() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(50, 50);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "pad",
            "--json",
            "--all",
            "10",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let stdout = String::from_utf8_lossy(&result.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("Should be valid JSON");

    assert_eq!(json["success"], true);
    assert_eq!(json["command"], "pad");
    assert_eq!(json["details"]["original_width"], 50);
    assert_eq!(json["details"]["original_height"], 50);
    assert_eq!(json["details"]["result_width"], 70);
    assert_eq!(json["details"]["result_height"], 70);
}

#[test]
fn test_canvas_expand() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(50, 50);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "canvas",
            "--width",
            "100",
            "--height",
            "100",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let out_img = image::open(&output).unwrap();
    assert_eq!(out_img.width(), 100);
    assert_eq!(out_img.height(), 100);
}

#[test]
fn test_canvas_shrink() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(100, 100);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "canvas",
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
fn test_canvas_with_anchor_top_left() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    // Create a red image
    let img = image::RgbaImage::from_fn(10, 10, |_, _| image::Rgba([255, 0, 0, 255]));
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "canvas",
            "--width",
            "20",
            "--height",
            "20",
            "--anchor",
            "top-left",
            "--color",
            "blue",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let out_img = image::open(&output).unwrap().to_rgba8();

    // Top-left should be original (red)
    let top_left = out_img.get_pixel(0, 0);
    assert_eq!(top_left[0], 255);
    assert_eq!(top_left[2], 0);

    // Bottom-right should be background (blue)
    let bottom_right = out_img.get_pixel(15, 15);
    assert_eq!(bottom_right[0], 0);
    assert_eq!(bottom_right[2], 255);
}

#[test]
fn test_canvas_with_anchor_center() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    // Create a green image
    let img = image::RgbaImage::from_fn(10, 10, |_, _| image::Rgba([0, 255, 0, 255]));
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "canvas",
            "--width",
            "30",
            "--height",
            "30",
            "--anchor",
            "center",
            "--color",
            "black",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let out_img = image::open(&output).unwrap().to_rgba8();

    // Center should be original (green)
    let center = out_img.get_pixel(15, 15);
    assert_eq!(center[1], 255); // Green

    // Corner should be background (black)
    let corner = out_img.get_pixel(0, 0);
    assert_eq!(corner[0], 0);
    assert_eq!(corner[1], 0);
    assert_eq!(corner[2], 0);
}

#[test]
fn test_canvas_json_output() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(50, 50);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "canvas",
            "--json",
            "--width",
            "100",
            "--height",
            "80",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let stdout = String::from_utf8_lossy(&result.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("Should be valid JSON");

    assert_eq!(json["success"], true);
    assert_eq!(json["command"], "canvas");
    assert_eq!(json["details"]["original_width"], 50);
    assert_eq!(json["details"]["original_height"], 50);
    assert_eq!(json["details"]["result_width"], 100);
    assert_eq!(json["details"]["result_height"], 80);
}

#[test]
fn test_composite_basic() {
    let temp_dir = TempDir::new().unwrap();
    let base = temp_dir.path().join("base.png");
    let overlay = temp_dir.path().join("overlay.png");
    let output = temp_dir.path().join("output.png");

    // Create red base
    let base_img = image::RgbaImage::from_fn(100, 100, |_, _| image::Rgba([255, 0, 0, 255]));
    base_img.save(&base).unwrap();

    // Create small green overlay
    let overlay_img = image::RgbaImage::from_fn(20, 20, |_, _| image::Rgba([0, 255, 0, 255]));
    overlay_img.save(&overlay).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "composite",
            "--x",
            "40",
            "--y",
            "40",
            base.to_str().unwrap(),
            overlay.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let out_img = image::open(&output).unwrap().to_rgba8();

    // Corner should be base (red)
    let corner = out_img.get_pixel(0, 0);
    assert_eq!(corner[0], 255);
    assert_eq!(corner[1], 0);

    // Center should be overlay (green)
    let center = out_img.get_pixel(50, 50);
    assert_eq!(center[0], 0);
    assert_eq!(center[1], 255);
}

#[test]
fn test_composite_with_anchor() {
    let temp_dir = TempDir::new().unwrap();
    let base = temp_dir.path().join("base.png");
    let overlay = temp_dir.path().join("overlay.png");
    let output = temp_dir.path().join("output.png");

    let base_img = image::RgbaImage::from_fn(100, 100, |_, _| image::Rgba([255, 0, 0, 255]));
    base_img.save(&base).unwrap();

    let overlay_img = image::RgbaImage::from_fn(20, 20, |_, _| image::Rgba([0, 0, 255, 255]));
    overlay_img.save(&overlay).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "composite",
            "--anchor",
            "center",
            base.to_str().unwrap(),
            overlay.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let out_img = image::open(&output).unwrap().to_rgba8();

    // Center should be overlay (blue)
    let center = out_img.get_pixel(50, 50);
    assert_eq!(center[2], 255);
}

#[test]
fn test_composite_with_opacity() {
    let temp_dir = TempDir::new().unwrap();
    let base = temp_dir.path().join("base.png");
    let overlay = temp_dir.path().join("overlay.png");
    let output = temp_dir.path().join("output.png");

    // White base
    let base_img = image::RgbaImage::from_fn(10, 10, |_, _| image::Rgba([255, 255, 255, 255]));
    base_img.save(&base).unwrap();

    // Black overlay
    let overlay_img = image::RgbaImage::from_fn(10, 10, |_, _| image::Rgba([0, 0, 0, 255]));
    overlay_img.save(&overlay).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "composite",
            "--opacity",
            "0.5",
            base.to_str().unwrap(),
            overlay.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let out_img = image::open(&output).unwrap().to_rgba8();
    let pixel = out_img.get_pixel(5, 5);

    // Should be a mix (gray)
    assert!(pixel[0] > 100 && pixel[0] < 200);
}

#[test]
fn test_composite_blend_multiply() {
    let temp_dir = TempDir::new().unwrap();
    let base = temp_dir.path().join("base.png");
    let overlay = temp_dir.path().join("overlay.png");
    let output = temp_dir.path().join("output.png");

    // White base
    let base_img = image::RgbaImage::from_fn(10, 10, |_, _| image::Rgba([255, 255, 255, 255]));
    base_img.save(&base).unwrap();

    // Gray overlay
    let overlay_img = image::RgbaImage::from_fn(10, 10, |_, _| image::Rgba([128, 128, 128, 255]));
    overlay_img.save(&overlay).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "composite",
            "--blend",
            "multiply",
            base.to_str().unwrap(),
            overlay.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let out_img = image::open(&output).unwrap().to_rgba8();
    let pixel = out_img.get_pixel(5, 5);

    // White * gray = gray
    assert!(pixel[0] > 120 && pixel[0] < 136);
}

#[test]
fn test_composite_blend_screen() {
    let temp_dir = TempDir::new().unwrap();
    let base = temp_dir.path().join("base.png");
    let overlay = temp_dir.path().join("overlay.png");
    let output = temp_dir.path().join("output.png");

    // Black base
    let base_img = image::RgbaImage::from_fn(10, 10, |_, _| image::Rgba([0, 0, 0, 255]));
    base_img.save(&base).unwrap();

    // Gray overlay
    let overlay_img = image::RgbaImage::from_fn(10, 10, |_, _| image::Rgba([128, 128, 128, 255]));
    overlay_img.save(&overlay).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "composite",
            "--blend",
            "screen",
            base.to_str().unwrap(),
            overlay.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let out_img = image::open(&output).unwrap().to_rgba8();
    let pixel = out_img.get_pixel(5, 5);

    // Black screen gray = gray
    assert!(pixel[0] > 120 && pixel[0] < 136);
}

#[test]
fn test_composite_json_output() {
    let temp_dir = TempDir::new().unwrap();
    let base = temp_dir.path().join("base.png");
    let overlay = temp_dir.path().join("overlay.png");
    let output = temp_dir.path().join("output.png");

    let base_img = common::create_test_rgba_image(100, 80);
    base_img.save(&base).unwrap();

    let overlay_img = common::create_test_rgba_image(20, 20);
    overlay_img.save(&overlay).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "composite",
            "--json",
            "--anchor",
            "center",
            base.to_str().unwrap(),
            overlay.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let stdout = String::from_utf8_lossy(&result.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("Should be valid JSON");

    assert_eq!(json["success"], true);
    assert_eq!(json["command"], "composite");
    assert_eq!(json["details"]["original_width"], 100);
    assert_eq!(json["details"]["original_height"], 80);
    assert_eq!(json["details"]["result_width"], 100);
    assert_eq!(json["details"]["result_height"], 80);
}

#[test]
fn test_composite_overlay_outside_bounds() {
    let temp_dir = TempDir::new().unwrap();
    let base = temp_dir.path().join("base.png");
    let overlay = temp_dir.path().join("overlay.png");
    let output = temp_dir.path().join("output.png");

    let base_img = common::create_test_rgba_image(50, 50);
    base_img.save(&base).unwrap();

    let overlay_img = common::create_test_rgba_image(20, 20);
    overlay_img.save(&overlay).unwrap();

    // Overlay partially outside
    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "composite",
            "--x",
            "40",
            "--y",
            "40",
            base.to_str().unwrap(),
            overlay.to_str().unwrap(),
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
fn test_composite_overlay_completely_outside() {
    let temp_dir = TempDir::new().unwrap();
    let base = temp_dir.path().join("base.png");
    let overlay = temp_dir.path().join("overlay.png");
    let output = temp_dir.path().join("output.png");

    let base_img = common::create_test_rgba_image(50, 50);
    base_img.save(&base).unwrap();

    let overlay_img = common::create_test_rgba_image(20, 20);
    overlay_img.save(&overlay).unwrap();

    // Overlay completely outside
    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "composite",
            "--x",
            "100",
            "--y",
            "100",
            base.to_str().unwrap(),
            overlay.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());
    // Should succeed but overlay won't be visible
}
