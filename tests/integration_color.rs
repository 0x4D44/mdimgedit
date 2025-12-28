mod common;

use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_grayscale_basic() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(100, 100);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "grayscale",
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

    // Verify output is grayscale (R=G=B for all pixels)
    let out_img = image::open(&output).unwrap().to_rgba8();
    for pixel in out_img.pixels() {
        assert_eq!(pixel[0], pixel[1]);
        assert_eq!(pixel[1], pixel[2]);
    }
}

#[test]
fn test_grayscale_json_output() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(64, 48);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "grayscale",
            "--json",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let stdout = String::from_utf8_lossy(&result.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("Should be valid JSON");

    assert_eq!(json["success"], true);
    assert_eq!(json["command"], "grayscale");
}

#[test]
fn test_invert_basic() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    // Create a solid red image
    let img = image::RgbaImage::from_fn(10, 10, |_, _| image::Rgba([255, 0, 0, 255]));
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args(["invert", input.to_str().unwrap(), output.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    // Verify inverted colors (red -> cyan)
    let out_img = image::open(&output).unwrap().to_rgba8();
    let pixel = out_img.get_pixel(0, 0);
    assert_eq!(pixel[0], 0); // 255 - 255 = 0
    assert_eq!(pixel[1], 255); // 255 - 0 = 255
    assert_eq!(pixel[2], 255); // 255 - 0 = 255
    assert_eq!(pixel[3], 255); // Alpha preserved
}

#[test]
fn test_depth_1bit() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(100, 100);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "depth",
            "--bits",
            "1",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    // Verify all pixels are black or white
    let out_img = image::open(&output).unwrap().to_luma8();
    for pixel in out_img.pixels() {
        assert!(pixel[0] == 0 || pixel[0] == 255);
    }
}

#[test]
fn test_depth_1bit_with_dither() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(100, 100);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "depth",
            "--bits",
            "1",
            "--dither",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());
    assert!(output.exists());
}

#[test]
fn test_convert_png_to_jpeg() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.jpg");

    let img = common::create_test_rgba_image(100, 100);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args(["convert", input.to_str().unwrap(), output.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(
        result.status.success(),
        "{:?}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(output.exists());

    // Verify it's a valid JPEG
    let out_img = image::open(&output).unwrap();
    assert_eq!(out_img.width(), 100);
    assert_eq!(out_img.height(), 100);
}

#[test]
fn test_convert_with_quality() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.jpg");

    let img = common::create_test_rgba_image(100, 100);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "convert",
            "--quality",
            "50",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());
    assert!(output.exists());
}

#[test]
fn test_convert_png_to_bmp() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.bmp");

    let img = common::create_test_rgba_image(50, 50);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args(["convert", input.to_str().unwrap(), output.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(
        result.status.success(),
        "{:?}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(output.exists());
}

#[test]
fn test_convert_png_to_gif() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.gif");

    let img = common::create_test_rgba_image(50, 50);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args(["convert", input.to_str().unwrap(), output.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(
        result.status.success(),
        "{:?}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(output.exists());
}

#[test]
fn test_convert_with_explicit_format() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.data"); // Unusual extension

    let img = common::create_test_rgba_image(50, 50);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "convert",
            "--format",
            "png",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());
    assert!(output.exists());

    // Should be readable as PNG using explicit format hint
    let reader = image::ImageReader::open(&output)
        .unwrap()
        .with_guessed_format()
        .unwrap();
    let out_img = reader.decode().unwrap();
    assert_eq!(out_img.width(), 50);
}

#[test]
fn test_convert_png_to_tiff() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.tiff");

    let img = common::create_test_rgba_image(50, 50);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args(["convert", input.to_str().unwrap(), output.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(
        result.status.success(),
        "{:?}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(output.exists());
}

#[test]
fn test_convert_png_to_webp() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.webp");

    let img = common::create_test_rgba_image(50, 50);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args(["convert", input.to_str().unwrap(), output.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(
        result.status.success(),
        "{:?}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(output.exists());
}

#[test]
fn test_convert_png_to_ico() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.ico");

    // ICO works best with standard icon sizes
    let img = image::RgbaImage::from_fn(32, 32, |x, y| {
        if (x + y) % 2 == 0 {
            image::Rgba([255, 0, 0, 255])
        } else {
            image::Rgba([0, 0, 255, 255])
        }
    });
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args(["convert", input.to_str().unwrap(), output.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(
        result.status.success(),
        "{:?}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(output.exists());
}

#[test]
fn test_convert_with_tiff_format_flag() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.data");

    let img = common::create_test_rgba_image(30, 30);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "convert",
            "--format",
            "tiff",
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
}

#[test]
fn test_convert_with_webp_format_flag() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.data");

    let img = common::create_test_rgba_image(30, 30);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "convert",
            "--format",
            "webp",
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
}

#[test]
fn test_convert_with_ico_format_flag() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.data");

    let img = image::RgbaImage::from_fn(32, 32, |_, _| image::Rgba([128, 128, 128, 255]));
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "convert",
            "--format",
            "ico",
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
}

#[test]
fn test_grayscale_no_preserve_alpha() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    // Create image with varying alpha
    let img = image::RgbaImage::from_fn(10, 10, |x, _| {
        image::Rgba([255, 128, 64, (x * 25).min(255) as u8])
    });
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "grayscale",
            "--no-preserve-alpha",
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
}

#[test]
fn test_invert_with_alpha() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    // Create image with some transparency
    let img = image::RgbaImage::from_fn(10, 10, |_, _| image::Rgba([100, 150, 200, 128]));
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "invert",
            "--invert-alpha",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let out_img = image::open(&output).unwrap().to_rgba8();
    let pixel = out_img.get_pixel(0, 0);
    assert_eq!(pixel[0], 155); // 255 - 100
    assert_eq!(pixel[1], 105); // 255 - 150
    assert_eq!(pixel[2], 55); // 255 - 200
    assert_eq!(pixel[3], 127); // 255 - 128
}

#[test]
fn test_depth_16bit() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(20, 20);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "depth",
            "--bits",
            "16",
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
}

#[test]
fn test_convert_json_output() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.jpg");

    let img = common::create_test_rgba_image(50, 50);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "convert",
            "--json",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let stdout = String::from_utf8_lossy(&result.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("Should be valid JSON");
    assert_eq!(json["success"], true);
    assert_eq!(json["command"], "convert");
}

#[test]
fn test_invert_json_output() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(30, 30);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "invert",
            "--json",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let stdout = String::from_utf8_lossy(&result.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("Should be valid JSON");
    assert_eq!(json["success"], true);
    assert_eq!(json["command"], "invert");
}

#[test]
fn test_depth_json_output() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.png");

    let img = common::create_test_rgba_image(30, 30);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "depth",
            "--json",
            "--bits",
            "8",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());

    let stdout = String::from_utf8_lossy(&result.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("Should be valid JSON");
    assert_eq!(json["success"], true);
    assert_eq!(json["command"], "depth");
}

#[test]
fn test_convert_quiet_mode() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.png");
    let output = temp_dir.path().join("output.jpg");

    let img = common::create_test_rgba_image(30, 30);
    img.save(&input).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_mdimgedit"))
        .args([
            "convert",
            "--quiet",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(result.status.success());
    assert!(result.stdout.is_empty());
}
