# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

`mdimgedit` is an AI-focused command-line image editing utility designed for programmatic use by AI systems and automation pipelines. It provides deterministic image manipulation operations with machine-parseable JSON output.

## Build Commands

```bash
cargo build                    # Debug build
cargo build --release          # Release build
cargo run -- <command>         # Run with arguments
cargo test                     # Run all tests
cargo test <test_name>         # Run specific test
cargo clippy                   # Lint
cargo fmt                      # Format code
```

## Architecture

### Module Structure

```
src/
├── main.rs           # Entry point, command dispatch
├── lib.rs            # Library root, public API re-exports
├── cli/
│   ├── args.rs       # Clap argument definitions (Command enum, Anchor, ResizeFilter, etc.)
│   └── output.rs     # Text/JSON output formatting
├── ops/              # Image operations (each file = one operation category)
│   ├── info.rs       # Image metadata extraction, load_image()
│   ├── crop.rs       # Crop operation
│   ├── rotate.rs     # Rotation (90/180/270 lossless, arbitrary with interpolation)
│   ├── flip.rs       # Horizontal/vertical flip
│   ├── resize.rs     # Resize and fit operations
│   ├── convert.rs    # Format conversion, save_with_format()
│   ├── color.rs      # Grayscale, bit depth, invert
│   ├── adjust.rs     # Brightness, contrast, gamma
│   ├── filter.rs     # Blur, sharpen
│   └── canvas.rs     # Padding, canvas resize, composite
├── color.rs          # Color parsing (hex, rgb, rgba, named colors)
└── error.rs          # ImgEditError enum, exit codes
```

### Key Patterns

- All operations return `Result<T, ImgEditError>`
- Operations in `ops/` take `&DynamicImage` and return new `DynamicImage` (immutable transforms)
- CLI args defined in `src/cli/args.rs` using clap derive macros
- Exit codes defined in `src/error.rs::exit_codes` (0-5 range)
- JSON output via `--json` flag, structured as `SuccessResponse` or error with code

### Adding New Commands

1. Add variant to `Command` enum in `src/cli/args.rs`
2. Implement operation function in appropriate `src/ops/*.rs` file
3. Export from `src/ops/mod.rs`
4. Add match arm in `run_command()` in `src/main.rs`
5. Add to `command_name()` match

## Supported Operations

- **Transforms**: crop, rotate, flip, resize, fit
- **Format**: convert (PNG, JPEG, GIF, BMP, TIFF, WebP, ICO)
- **Color**: grayscale, depth, invert
- **Adjustments**: brightness, contrast, gamma
- **Filters**: blur, sharpen
- **Canvas**: pad, canvas (resize without scaling), composite

## Color Specification

Colors are parsed in `src/color.rs`. Supported formats:
- Named: `black`, `white`, `red`, `green`, `blue`, `yellow`, `cyan`, `magenta`, `transparent`
- Hex: `#RGB`, `#RGBA`, `#RRGGBB`, `#RRGGBBAA`
- RGB: `rgb(R,G,B)`
- RGBA: `rgba(R,G,B,A)`

## Design Document

See `wrk_docs/2025.12.28 - HLD - Image Editing CLI Tool.md` for full specification including:
- Complete command syntax and options
- JSON output schemas
- Error handling strategy
- Test fixture requirements
