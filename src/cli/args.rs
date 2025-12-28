use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "mdimgedit",
    author = "Arthur & Claude",
    version,
    about = "AI-focused command-line image editing tool",
    long_about = "A comprehensive image manipulation utility designed for programmatic use by AI systems and automation pipelines.\n\n\
                  Supports common transformations (crop, rotate, resize), format conversion, \
                  color adjustments, and compositing operations.\n\n\
                  Use --json for machine-parseable output suitable for AI integration."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Output results as JSON (for AI consumption)
    #[arg(short, long, global = true)]
    pub json: bool,

    /// Suppress non-error output
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Overwrite output file without prompting
    #[arg(short = 'y', long, global = true)]
    pub overwrite: bool,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Display image information (dimensions, format, color depth)
    #[command(long_about = "Extract metadata from an image file.\n\n\
                      Returns dimensions, format, color type, bit depth, and file size.\n\
                      Use --json for machine-parseable output.\n\n\
                      Examples:\n  \
                        mdimgedit info image.png\n  \
                        mdimgedit info --json image.png")]
    Info {
        /// Input image file
        #[arg(value_name = "INPUT")]
        input: PathBuf,
    },

    /// Crop image to specified region
    #[command(long_about = "Extract a rectangular region from the image.\n\n\
                      Specify the region using --x, --y for the starting position and \
                      --width, --height for the size. Use --anchor to position the crop \
                      region relative to a named point.\n\n\
                      Examples:\n  \
                        mdimgedit crop --width 100 --height 100 input.png output.png\n  \
                        mdimgedit crop --x 50 --y 50 --width 200 --height 200 input.png output.png\n  \
                        mdimgedit crop --width 500 --height 500 --anchor center input.png output.png")]
    Crop {
        /// Left edge X coordinate
        #[arg(long, default_value = "0")]
        x: u32,
        /// Top edge Y coordinate
        #[arg(long, default_value = "0")]
        y: u32,
        /// Width of crop region
        #[arg(long)]
        width: u32,
        /// Height of crop region
        #[arg(long)]
        height: u32,
        /// Anchor point for positioning
        #[arg(long, value_enum, default_value = "top-left")]
        anchor: Anchor,
        /// Input image file
        #[arg(value_name = "INPUT")]
        input: PathBuf,
        /// Output image file
        #[arg(value_name = "OUTPUT")]
        output: PathBuf,
    },

    /// Rotate image by degrees
    #[command(
        long_about = "Rotate image by specified degrees counter-clockwise.\n\n\
                      For 90, 180, 270 degree rotations, uses lossless pixel remapping.\n\
                      For arbitrary angles, uses bilinear interpolation.\n\n\
                      Examples:\n  \
                        mdimgedit rotate --degrees 90 input.png output.png\n  \
                        mdimgedit rotate --degrees 45 --expand --background white input.png output.png"
    )]
    Rotate {
        /// Rotation angle in degrees (counter-clockwise)
        #[arg(long)]
        degrees: f64,
        /// Expand canvas to fit rotated image
        #[arg(long)]
        expand: bool,
        /// Background color for expanded areas
        #[arg(long, default_value = "transparent")]
        background: String,
        /// Input image file
        #[arg(value_name = "INPUT")]
        input: PathBuf,
        /// Output image file
        #[arg(value_name = "OUTPUT")]
        output: PathBuf,
    },

    /// Flip image horizontally or vertically
    #[command(
        long_about = "Mirror image horizontally (left-right) or vertically (top-bottom).\n\n\
                      At least one of --horizontal or --vertical must be specified.\n\
                      Both can be specified to flip in both directions.\n\n\
                      Examples:\n  \
                        mdimgedit flip --horizontal input.png output.png\n  \
                        mdimgedit flip --vertical input.png output.png\n  \
                        mdimgedit flip --horizontal --vertical input.png output.png"
    )]
    Flip {
        /// Flip horizontally (mirror left-right)
        #[arg(short = 'H', long)]
        horizontal: bool,
        /// Flip vertically (mirror top-bottom)
        #[arg(short = 'V', long)]
        vertical: bool,
        /// Input image file
        #[arg(value_name = "INPUT")]
        input: PathBuf,
        /// Output image file
        #[arg(value_name = "OUTPUT")]
        output: PathBuf,
    },

    /// Resize image to exact dimensions or scale factor
    #[command(
        long_about = "Resize image to specified dimensions or by a scale factor.\n\n\
                      Specify either dimensions (--width and/or --height) OR --scale, not both.\n\
                      When only one dimension is given, the other is calculated to preserve aspect ratio.\n\n\
                      Examples:\n  \
                        mdimgedit resize --width 800 --height 600 input.png output.png\n  \
                        mdimgedit resize --width 800 input.png output.png\n  \
                        mdimgedit resize --scale 0.5 input.png output.png\n  \
                        mdimgedit resize --scale 4 --filter nearest input.png output.png"
    )]
    Resize {
        /// Target width in pixels
        #[arg(long)]
        width: Option<u32>,
        /// Target height in pixels
        #[arg(long)]
        height: Option<u32>,
        /// Scale factor (e.g., 0.5 for half, 2.0 for double)
        #[arg(long)]
        scale: Option<f64>,
        /// Resampling filter
        #[arg(long, value_enum, default_value = "lanczos")]
        filter: ResizeFilter,
        /// Input image file
        #[arg(value_name = "INPUT")]
        input: PathBuf,
        /// Output image file
        #[arg(value_name = "OUTPUT")]
        output: PathBuf,
    },

    /// Resize to fit within bounds preserving aspect ratio
    #[command(
        long_about = "Resize image to fit within maximum dimensions while preserving aspect ratio.\n\n\
                      The image is scaled down to fit within the specified bounds.\n\
                      Use --upscale to allow enlarging smaller images.\n\n\
                      Examples:\n  \
                        mdimgedit fit --max-width 800 --max-height 600 input.png output.png\n  \
                        mdimgedit fit --max-width 1024 input.png output.png\n  \
                        mdimgedit fit --max-width 800 --max-height 600 --upscale input.png output.png"
    )]
    Fit {
        /// Maximum width constraint
        #[arg(long)]
        max_width: Option<u32>,
        /// Maximum height constraint
        #[arg(long)]
        max_height: Option<u32>,
        /// Allow upscaling if image is smaller than bounds
        #[arg(long)]
        upscale: bool,
        /// Resampling filter
        #[arg(long, value_enum, default_value = "lanczos")]
        filter: ResizeFilter,
        /// Input image file
        #[arg(value_name = "INPUT")]
        input: PathBuf,
        /// Output image file
        #[arg(value_name = "OUTPUT")]
        output: PathBuf,
    },

    /// Convert image format
    #[command(long_about = "Convert image between formats.\n\n\
                      Format is auto-detected from output extension if not specified.\n\
                      Use --quality for lossy formats (JPEG, WebP).\n\n\
                      Supported formats: PNG, JPEG, GIF, BMP, TIFF, WebP, ICO\n\n\
                      Examples:\n  \
                        mdimgedit convert input.png output.jpg\n  \
                        mdimgedit convert --format webp input.png output.webp\n  \
                        mdimgedit convert --quality 85 input.png output.jpg")]
    Convert {
        /// Target format (auto-detected from extension if not specified)
        #[arg(long, value_enum)]
        format: Option<ImageFormat>,
        /// Quality for lossy formats (1-100)
        #[arg(long, default_value = "90", value_parser = clap::value_parser!(u8).range(1..=100))]
        quality: u8,
        /// Input image file
        #[arg(value_name = "INPUT")]
        input: PathBuf,
        /// Output image file
        #[arg(value_name = "OUTPUT")]
        output: PathBuf,
    },

    /// Convert to grayscale
    #[command(long_about = "Convert image to grayscale.\n\n\
                      By default, preserves the alpha channel if present.\n\n\
                      Examples:\n  \
                        mdimgedit grayscale input.png output.png\n  \
                        mdimgedit grayscale --no-preserve-alpha input.png output.png")]
    Grayscale {
        /// Don't preserve alpha channel
        #[arg(long)]
        no_preserve_alpha: bool,
        /// Input image file
        #[arg(value_name = "INPUT")]
        input: PathBuf,
        /// Output image file
        #[arg(value_name = "OUTPUT")]
        output: PathBuf,
    },

    /// Change color bit depth
    #[command(long_about = "Change color bit depth of the image.\n\n\
                      Supported depths: 1 (black/white), 8 (standard), 16 (high precision).\n\
                      Use --dither when reducing depth to minimize banding.\n\n\
                      Examples:\n  \
                        mdimgedit depth --bits 1 input.png output.png\n  \
                        mdimgedit depth --bits 1 --dither input.png output.png\n  \
                        mdimgedit depth --bits 16 input.png output.png")]
    Depth {
        /// Target bit depth per channel (1, 8, or 16)
        #[arg(long, value_parser = clap::value_parser!(u8).range(1..=16))]
        bits: u8,
        /// Apply dithering when reducing depth
        #[arg(long)]
        dither: bool,
        /// Input image file
        #[arg(value_name = "INPUT")]
        input: PathBuf,
        /// Output image file
        #[arg(value_name = "OUTPUT")]
        output: PathBuf,
    },

    /// Invert image colors
    #[command(long_about = "Invert all color values in the image.\n\n\
                      By default, the alpha channel is not inverted.\n\n\
                      Examples:\n  \
                        mdimgedit invert input.png output.png\n  \
                        mdimgedit invert --invert-alpha input.png output.png")]
    Invert {
        /// Also invert the alpha channel
        #[arg(long)]
        invert_alpha: bool,
        /// Input image file
        #[arg(value_name = "INPUT")]
        input: PathBuf,
        /// Output image file
        #[arg(value_name = "OUTPUT")]
        output: PathBuf,
    },

    /// Adjust brightness
    #[command(long_about = "Adjust image brightness.\n\n\
                      Value range: -255 to 255 (0 = no change).\n\
                      Positive values brighten, negative values darken.\n\n\
                      Examples:\n  \
                        mdimgedit brightness --value 50 input.png output.png\n  \
                        mdimgedit brightness --value -30 input.png output.png")]
    Brightness {
        /// Brightness adjustment (-255 to 255)
        #[arg(long, allow_hyphen_values = true)]
        value: i32,
        /// Input image file
        #[arg(value_name = "INPUT")]
        input: PathBuf,
        /// Output image file
        #[arg(value_name = "OUTPUT")]
        output: PathBuf,
    },

    /// Adjust contrast
    #[command(long_about = "Adjust image contrast.\n\n\
                      Value is a multiplier: 1.0 = no change, <1.0 reduces, >1.0 increases.\n\
                      Range: 0.0 to 10.0.\n\n\
                      Examples:\n  \
                        mdimgedit contrast --value 1.5 input.png output.png\n  \
                        mdimgedit contrast --value 0.8 input.png output.png")]
    Contrast {
        /// Contrast multiplier (0.0 to 10.0)
        #[arg(long)]
        value: f64,
        /// Input image file
        #[arg(value_name = "INPUT")]
        input: PathBuf,
        /// Output image file
        #[arg(value_name = "OUTPUT")]
        output: PathBuf,
    },

    /// Apply gamma correction
    #[command(long_about = "Apply gamma correction to the image.\n\n\
                      Gamma < 1.0 lightens midtones, > 1.0 darkens them.\n\
                      Range: 0.1 to 10.0 (1.0 = no change).\n\n\
                      Examples:\n  \
                        mdimgedit gamma --value 0.7 input.png output.png\n  \
                        mdimgedit gamma --value 1.5 input.png output.png")]
    Gamma {
        /// Gamma value (0.1 to 10.0)
        #[arg(long)]
        value: f64,
        /// Input image file
        #[arg(value_name = "INPUT")]
        input: PathBuf,
        /// Output image file
        #[arg(value_name = "OUTPUT")]
        output: PathBuf,
    },

    /// Apply Gaussian blur
    #[command(long_about = "Apply Gaussian blur filter to the image.\n\n\
                      Radius determines blur strength (larger = more blur).\n\
                      Range: 0.1 to 100.0 pixels.\n\n\
                      Examples:\n  \
                        mdimgedit blur --radius 2.0 input.png output.png\n  \
                        mdimgedit blur --radius 10.0 input.png output.png")]
    Blur {
        /// Blur radius in pixels (0.1 to 100.0)
        #[arg(long)]
        radius: f32,
        /// Input image file
        #[arg(value_name = "INPUT")]
        input: PathBuf,
        /// Output image file
        #[arg(value_name = "OUTPUT")]
        output: PathBuf,
    },

    /// Apply sharpening filter
    #[command(long_about = "Apply sharpening filter to the image.\n\n\
                      Amount controls strength, radius controls effect spread.\n\n\
                      Examples:\n  \
                        mdimgedit sharpen input.png output.png\n  \
                        mdimgedit sharpen --amount 2.0 input.png output.png")]
    Sharpen {
        /// Sharpening strength (0.0 to 10.0)
        #[arg(long, default_value = "1.0")]
        amount: f32,
        /// Effect radius in pixels (0.1 to 10.0)
        #[arg(long, default_value = "1.0")]
        radius: f32,
        /// Input image file
        #[arg(value_name = "INPUT")]
        input: PathBuf,
        /// Output image file
        #[arg(value_name = "OUTPUT")]
        output: PathBuf,
    },

    /// Add padding/border around image
    #[command(long_about = "Add padding or border around the image.\n\n\
                      Specify padding with --all (all sides), --horizontal/--vertical, \
                      or individual --top/--bottom/--left/--right.\n\n\
                      Color formats: named (red, blue), hex (#RGB, #RRGGBB), rgb(R,G,B), rgba(R,G,B,A)\n\n\
                      Examples:\n  \
                        mdimgedit pad --all 10 input.png output.png\n  \
                        mdimgedit pad --horizontal 20 --vertical 10 input.png output.png\n  \
                        mdimgedit pad --all 5 --color red input.png output.png\n  \
                        mdimgedit pad --all 10 --color \"#FF5500\" input.png output.png")]
    Pad {
        /// Padding on all sides
        #[arg(long)]
        all: Option<u32>,
        /// Top padding
        #[arg(long)]
        top: Option<u32>,
        /// Bottom padding
        #[arg(long)]
        bottom: Option<u32>,
        /// Left padding
        #[arg(long)]
        left: Option<u32>,
        /// Right padding
        #[arg(long)]
        right: Option<u32>,
        /// Horizontal (left and right) padding
        #[arg(long)]
        horizontal: Option<u32>,
        /// Vertical (top and bottom) padding
        #[arg(long)]
        vertical: Option<u32>,
        /// Padding color
        #[arg(long, default_value = "transparent")]
        color: String,
        /// Input image file
        #[arg(value_name = "INPUT")]
        input: PathBuf,
        /// Output image file
        #[arg(value_name = "OUTPUT")]
        output: PathBuf,
    },

    /// Resize canvas without scaling content
    #[command(long_about = "Resize the canvas without scaling image content.\n\n\
                      If new canvas is larger, original image is positioned according to --anchor.\n\
                      If smaller, image is cropped from the anchor point.\n\n\
                      Examples:\n  \
                        mdimgedit canvas --width 1000 --height 1000 input.png output.png\n  \
                        mdimgedit canvas --width 1000 --height 1000 --anchor top-left input.png output.png\n  \
                        mdimgedit canvas --width 500 --height 500 --anchor center input.png output.png")]
    Canvas {
        /// New canvas width
        #[arg(long)]
        width: u32,
        /// New canvas height
        #[arg(long)]
        height: u32,
        /// Position of original image on new canvas
        #[arg(long, value_enum, default_value = "center")]
        anchor: Anchor,
        /// Background color for new canvas areas
        #[arg(long, default_value = "transparent")]
        color: String,
        /// Input image file
        #[arg(value_name = "INPUT")]
        input: PathBuf,
        /// Output image file
        #[arg(value_name = "OUTPUT")]
        output: PathBuf,
    },

    /// Overlay one image onto another
    #[command(long_about = "Composite (overlay) one image onto a base image.\n\n\
                      Position the overlay using --x/--y or --anchor.\n\
                      Control transparency with --opacity and blend mode with --blend.\n\n\
                      Examples:\n  \
                        mdimgedit composite base.png overlay.png output.png\n  \
                        mdimgedit composite --x 100 --y 50 base.png overlay.png output.png\n  \
                        mdimgedit composite --anchor center base.png overlay.png output.png\n  \
                        mdimgedit composite --opacity 0.5 base.png overlay.png output.png")]
    Composite {
        /// X position of overlay
        #[arg(long)]
        x: Option<i32>,
        /// Y position of overlay
        #[arg(long)]
        y: Option<i32>,
        /// Anchor point for positioning
        #[arg(long, value_enum)]
        anchor: Option<Anchor>,
        /// Overlay opacity (0.0 to 1.0)
        #[arg(long, default_value = "1.0")]
        opacity: f32,
        /// Blend mode
        #[arg(long, value_enum, default_value = "normal")]
        blend: BlendMode,
        /// Base image file
        #[arg(value_name = "BASE")]
        base: PathBuf,
        /// Overlay image file
        #[arg(value_name = "OVERLAY")]
        overlay: PathBuf,
        /// Output image file
        #[arg(value_name = "OUTPUT")]
        output: PathBuf,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
pub enum Anchor {
    #[value(name = "top-left")]
    TopLeft,
    #[value(name = "top-center")]
    TopCenter,
    #[value(name = "top-right")]
    TopRight,
    #[value(name = "center-left")]
    CenterLeft,
    #[value(name = "center")]
    Center,
    #[value(name = "center-right")]
    CenterRight,
    #[value(name = "bottom-left")]
    BottomLeft,
    #[value(name = "bottom-center")]
    BottomCenter,
    #[value(name = "bottom-right")]
    BottomRight,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
pub enum ResizeFilter {
    Nearest,
    Linear,
    Cubic,
    Lanczos,
}

impl ResizeFilter {
    pub fn to_image_filter(self) -> image::imageops::FilterType {
        match self {
            ResizeFilter::Nearest => image::imageops::FilterType::Nearest,
            ResizeFilter::Linear => image::imageops::FilterType::Triangle,
            ResizeFilter::Cubic => image::imageops::FilterType::CatmullRom,
            ResizeFilter::Lanczos => image::imageops::FilterType::Lanczos3,
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
pub enum ImageFormat {
    Png,
    Jpeg,
    Gif,
    Bmp,
    Tiff,
    Webp,
    Ico,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_cli_parses() {
        Cli::command().debug_assert();
    }

    #[test]
    fn test_resize_filter_conversion() {
        assert!(matches!(
            ResizeFilter::Nearest.to_image_filter(),
            image::imageops::FilterType::Nearest
        ));
        assert!(matches!(
            ResizeFilter::Lanczos.to_image_filter(),
            image::imageops::FilterType::Lanczos3
        ));
    }

    #[test]
    fn test_anchor_values() {
        let anchors = [
            Anchor::TopLeft,
            Anchor::TopCenter,
            Anchor::TopRight,
            Anchor::CenterLeft,
            Anchor::Center,
            Anchor::CenterRight,
            Anchor::BottomLeft,
            Anchor::BottomCenter,
            Anchor::BottomRight,
        ];
        assert_eq!(anchors.len(), 9);
    }
}
