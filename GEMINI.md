# mdimgedit - AI-Focused Image Editing CLI

## Project Overview

`mdimgedit` is a specialized command-line image editing utility built in Rust. It is designed primarily for programmatic use by AI systems and automation pipelines, offering deterministic image manipulation with machine-parseable JSON output.

**Key Features:**
*   **Deterministic Operations:** Consistent behavior for cropping, resizing, rotating, and filtering images.
*   **AI-Friendly Output:** All commands support a `--json` flag to return structured data suitable for parsing by other tools or agents.
*   **Comprehensive Toolset:** Supports transformations, format conversion, color adjustments, and canvas operations.

## Building and Running

This project uses standard Rust tooling (`cargo`).

*   **Build (Debug):** `cargo build`
*   **Build (Release):** `cargo build --release`
*   **Run:** `cargo run -- <COMMAND> [ARGS]`
    *   Example: `cargo run -- info input.png`
*   **Test:** `cargo test`
    *   Run specific test: `cargo test <test_name>`
*   **Lint:** `cargo clippy`
*   **Format:** `cargo fmt`

## Development Conventions

### Code Style & Standards
*   **Formatting:** Strictly adhere to `rustfmt`.
*   **Linting:** Ensure code passes `clippy` checks.
*   **Error Handling:**
    *   Use `Result<T, ImgEditError>` for all fallible operations.
    *   `ImgEditError` is defined in `src/error.rs`.
    *   Errors should be descriptive and mapped to specific exit codes (defined in `src/error.rs`).

### Architecture
The codebase is organized into clear modules:
*   `src/main.rs`: Entry point and command dispatch logic.
*   `src/lib.rs`: Library root and public API exports.
*   `src/cli/`: Handles command-line interface concerns.
    *   `args.rs`: Defines CLI arguments and subcommands using `clap`.
    *   `output.rs`: Manages text and JSON output formatting.
*   `src/ops/`: Contains the implementation of image operations.
    *   Each file (e.g., `crop.rs`, `resize.rs`) implements a specific category of operations.
    *   Operations typically take a `&DynamicImage` and return a new `DynamicImage` (immutable transformation pattern).

### Adding New Commands
1.  **Define:** Add a new variant to the `Command` enum in `src/cli/args.rs`.
2.  **Implement:** Create the operation logic in a corresponding file within `src/ops/` (or add to an existing one).
3.  **Export:** Ensure the new module is accessible via `src/ops/mod.rs`.
4.  **Dispatch:** Update the matching logic in `run_command()` within `src/main.rs` to handle the new command.
5.  **Test:** Add unit tests in the operation module and integration tests in `tests/`.

### Testing
*   **Unit Tests:** Located within source files (e.g., `src/ops/crop.rs`) to test logic in isolation.
*   **Integration Tests:** Located in `tests/`. These test the full CLI workflow, often using fixtures from `tests/fixtures/`.
*   **Coverage:** Aim for high code coverage (>95%), especially for operation logic.

## Key Operations

*   **Info:** Extract metadata (dimensions, format, color type).
*   **Transforms:** Crop, Rotate, Flip, Resize, Fit.
*   **Color/Format:** Convert formats, Grayscale, Bit Depth, Invert.
*   **Adjustments:** Brightness, Contrast, Gamma.
*   **Filters:** Blur, Sharpen.
*   **Canvas:** Padding, Canvas Resizing, Compositing.

## Documentation
*   **Design Docs:** See `wrk_docs/` for high-level design and specifications.
*   **Context:** `CLAUDE.md` contains legacy context that may still be relevant for understanding design decisions.
