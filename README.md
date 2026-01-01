# mdimgedit

**AI-focused command-line image editing tool**

`mdimgedit` is a comprehensive image manipulation utility designed for programmatic use by AI systems, automation pipelines, and developers who need deterministic image processing with machine-parseable output.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust Version](https://img.shields.io/badge/rust-1.75%2B-orange.svg)

## Key Features

*   **AI-Native Output**: Every command supports a `--json` flag to return structured data instead of human-readable text, making it trivial to parse in scripts or LLM tool-use scenarios.
*   **Deterministic**: Operations are designed to be reproducible.
*   **Comprehensive Toolset**: Includes geometric transformations (crop, resize, rotate), color adjustments (brightness, contrast, gamma), format conversion, and canvas operations.
*   **Performance**: Built on Rust's `image` and `imageproc` libraries for speed and safety.

## Installation

### From Source

Ensure you have Rust installed (1.75+ recommended).

```bash
git clone https://github.com/yourusername/mdimgedit.git
cd mdimgedit
cargo install --path .
```

## Usage

```bash
mdimgedit [OPTIONS] <COMMAND> [COMMAND-OPTIONS] <INPUT> [OUTPUT]
```

### Global Options

*   `--json`: Output results as JSON. This is the recommended mode for programmatic use.
*   `--quiet` (`-q`): Suppress non-error output.
*   `--overwrite` (`-y`): Overwrite output file without prompting.
*   `--help`: Print help information.

## Commands

### Info
Extract metadata from an image file (dimensions, format, color type).

```bash
mdimgedit info image.png
mdimgedit info --json image.png
```

### Exif
Read and display EXIF metadata.

```bash
# Show summary
mdimgedit exif photo.jpg

# Show all tags
mdimgedit exif --verbose photo.jpg

# Get specific tag
mdimgedit exif --tag "Make" photo.jpg
```

### Resize & Fit
Resize images with precise control over dimensions and sampling filters.

```bash
# Exact resize
mdimgedit resize --width 800 --height 600 input.png output.png

# Scale by factor (0.5 = 50%)
mdimgedit resize --scale 0.5 input.png output.png

# Fit within box (preserving aspect ratio)
mdimgedit fit --max-width 800 --max-height 600 input.png output.png
```

### Crop
Extract rectangular regions.

```bash
# Basic crop
mdimgedit crop --width 100 --height 100 input.png output.png

# Crop from specific coordinate
mdimgedit crop --x 50 --y 50 --width 200 --height 200 input.png output.png

# Crop using anchor point
mdimgedit crop --width 500 --height 500 --anchor center input.png output.png
```

### Rotate & Flip
Geometric transformations.

```bash
# Rotate 90 degrees
mdimgedit rotate --degrees 90 input.png output.png

# Arbitrary rotation with background fill
mdimgedit rotate --degrees 45 --expand --background white input.png output.png

# Flip/Mirror
mdimgedit flip --horizontal input.png output.png
mdimgedit flip --vertical input.png output.png
```

### Canvas & Padding
Modify the image canvas size or add borders.

```bash
# Add padding
mdimgedit pad --all 20 --color white input.png output.png

# Resize canvas (content centered)
mdimgedit canvas --width 1000 --height 1000 --anchor center input.png output.png
```

### Color Adjustments
Modify image appearance.

```bash
# Brightness (-255 to 255)
mdimgedit brightness --value 20 input.png output.png

# Contrast (multiplier, 1.0 = original)
mdimgedit contrast --value 1.2 input.png output.png

# Gamma correction
mdimgedit gamma --value 0.8 input.png output.png

# Invert colors
mdimgedit invert input.png output.png

# Grayscale
mdimgedit grayscale input.png output.png

# Change Bit Depth (1, 8, 16)
mdimgedit depth --bits 1 --dither input.png output.png
```

### Filters
Apply convolution filters.

```bash
# Blur
mdimgedit blur --radius 2.0 input.png output.png

# Sharpen
mdimgedit sharpen --amount 1.5 input.png output.png
```

### Format Conversion
Convert between image formats.

```bash
# Convert based on extension
mdimgedit convert input.png output.webp

# Explicit format and quality
mdimgedit convert --format jpeg --quality 85 input.png output.jpg
```

### Composite
Overlay one image onto another.

```bash
mdimgedit composite --x 100 --y 50 base.png overlay.png output.png
mdimgedit composite --anchor center --opacity 0.5 base.png overlay.png output.png
```

## Supported Formats

*   PNG
*   JPEG
*   GIF
*   BMP
*   TIFF
*   WebP
*   ICO

## Development

```bash
# Run tests
cargo test

# Build release binary
cargo build --release

# Format code
cargo fmt
```

## License

This project is licensed under the MIT License.
