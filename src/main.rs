use clap::Parser;
use mdimgedit::cli::output::{print_error, OutputFormat, SuccessResponse};
use mdimgedit::cli::{Cli, Command};
use mdimgedit::error::{exit_codes, ImgEditError};
use mdimgedit::ops;
use mdimgedit::parse_color;
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli = Cli::parse();
    let format = if cli.json {
        OutputFormat::Json
    } else {
        OutputFormat::Text
    };

    let result = run_command(&cli, format);

    match result {
        Ok(code) => ExitCode::from(code as u8),
        Err(e) => {
            let cmd_name = command_name(&cli.command);
            print_error(format, cmd_name, &e);
            ExitCode::from(e.exit_code() as u8)
        }
    }
}

fn command_name(cmd: &Command) -> &'static str {
    match cmd {
        Command::Info { .. } => "info",
        Command::Exif { .. } => "exif",
        Command::Crop { .. } => "crop",
        Command::Rotate { .. } => "rotate",
        Command::Flip { .. } => "flip",
        Command::Resize { .. } => "resize",
        Command::Fit { .. } => "fit",
        Command::Convert { .. } => "convert",
        Command::Grayscale { .. } => "grayscale",
        Command::Depth { .. } => "depth",
        Command::Invert { .. } => "invert",
        Command::Brightness { .. } => "brightness",
        Command::Contrast { .. } => "contrast",
        Command::Gamma { .. } => "gamma",
        Command::Blur { .. } => "blur",
        Command::Sharpen { .. } => "sharpen",
        Command::Pad { .. } => "pad",
        Command::Canvas { .. } => "canvas",
        Command::Composite { .. } => "composite",
    }
}

/// Check if output file exists and handle overwrite logic
fn check_output_overwrite(path: &Path, overwrite: bool) -> mdimgedit::Result<()> {
    if path.exists() && !overwrite {
        return Err(ImgEditError::WriteError {
            path: path.display().to_string(),
            reason: "File exists. Use --overwrite (-y) to replace.".to_string(),
        });
    }
    Ok(())
}

/// Save an image and print success response
fn save_and_respond(
    img: &image::DynamicImage,
    output: &Path,
    format: OutputFormat,
    quiet: bool,
    cmd_name: &str,
    input_path: &str,
    orig_dim: (u32, u32),
) -> mdimgedit::Result<i32> {
    img.save(output).map_err(|e| ImgEditError::WriteError {
        path: output.display().to_string(),
        reason: e.to_string(),
    })?;

    if format == OutputFormat::Json {
        let response = SuccessResponse::new(cmd_name)
            .with_input(input_path)
            .with_output(&output.display().to_string())
            .with_detail("original_width", orig_dim.0)
            .with_detail("original_height", orig_dim.1)
            .with_detail("result_width", img.width())
            .with_detail("result_height", img.height());
        println!("{}", response.to_json());
    } else if !quiet {
        println!(
            "Saved {} ({}x{} -> {}x{})",
            output.display(),
            orig_dim.0,
            orig_dim.1,
            img.width(),
            img.height()
        );
    }

    Ok(exit_codes::SUCCESS)
}

fn run_command(cli: &Cli, format: OutputFormat) -> mdimgedit::Result<i32> {
    match &cli.command {
        Command::Info { input } => {
            let info = ops::get_image_info(input)?;

            if format == OutputFormat::Json {
                let response = SuccessResponse::new("info")
                    .with_input(&info.file)
                    .with_detail("format", info.format.clone())
                    .with_detail("width", info.width)
                    .with_detail("height", info.height)
                    .with_detail("color_type", info.color_type.clone())
                    .with_detail("bit_depth", info.bit_depth)
                    .with_detail("file_size_bytes", info.file_size_bytes);
                println!("{}", response.to_json());
            } else if !cli.quiet {
                println!("{}", info.display());
            }

            Ok(exit_codes::SUCCESS)
        }

        Command::Exif {
            verbose,
            tag,
            input,
        } => {
            let exif_data = ops::read_exif(input)?;

            if let Some(ref tag_name) = tag {
                // Specific tag requested
                let field = exif_data
                    .fields
                    .iter()
                    .find(|f| f.tag.to_lowercase() == tag_name.to_lowercase());

                if format == OutputFormat::Json {
                    let mut response = SuccessResponse::new("exif")
                        .with_input(&input.display().to_string())
                        .with_detail("tag", tag_name.clone());

                    if let Some(f) = field {
                        response = response
                            .with_detail("value", f.value.clone())
                            .with_detail("found", true);
                    } else {
                        response = response.with_detail("found", false);
                    }
                    println!("{}", response.to_json());
                } else if !cli.quiet {
                    if let Some(f) = field {
                        println!("{}: {}", f.tag, f.value);
                    } else {
                        println!("Tag '{}' not found", tag_name);
                    }
                }
            } else if format == OutputFormat::Json {
                // Serialize ExifData directly for complete JSON output
                let fields_json =
                    serde_json::to_value(&exif_data.fields).unwrap_or(serde_json::Value::Null);
                let response = SuccessResponse::new("exif")
                    .with_input(&input.display().to_string())
                    .with_detail("has_exif", exif_data.has_exif)
                    .with_detail("field_count", exif_data.fields.len())
                    .with_detail("camera_make", exif_data.camera_make.clone())
                    .with_detail("camera_model", exif_data.camera_model.clone())
                    .with_detail("date_time", exif_data.date_time.clone())
                    .with_detail("exposure_time", exif_data.exposure_time.clone())
                    .with_detail("f_number", exif_data.f_number.clone())
                    .with_detail("iso", exif_data.iso.clone())
                    .with_detail("focal_length", exif_data.focal_length.clone())
                    .with_detail("gps_latitude", exif_data.gps_latitude.clone())
                    .with_detail("gps_longitude", exif_data.gps_longitude.clone())
                    .with_detail("orientation", exif_data.orientation)
                    .with_detail("software", exif_data.software.clone())
                    .with_detail("artist", exif_data.artist.clone())
                    .with_detail("copyright", exif_data.copyright.clone())
                    .with_detail("fields", fields_json);
                println!("{}", response.to_json());
            } else if !cli.quiet {
                if *verbose {
                    println!("{}", ops::exif::format_exif_verbose(&exif_data));
                } else {
                    println!("{}", ops::exif::format_exif_text(&exif_data));
                }
            }

            Ok(exit_codes::SUCCESS)
        }

        Command::Crop {
            x,
            y,
            width,
            height,
            anchor,
            input,
            output,
        } => {
            check_output_overwrite(output, cli.overwrite)?;
            let img = ops::load_image(input)?;
            let orig_width = img.width();
            let orig_height = img.height();

            let result = ops::crop(&img, *x, *y, *width, *height, *anchor)?;

            save_and_respond(
                &result,
                output,
                format,
                cli.quiet,
                "crop",
                &input.display().to_string(),
                (orig_width, orig_height),
            )
        }

        Command::Rotate {
            degrees,
            expand,
            background,
            input,
            output,
        } => {
            check_output_overwrite(output, cli.overwrite)?;
            let img = ops::load_image(input)?;
            let orig_width = img.width();
            let orig_height = img.height();

            let bg_color = parse_color(background)?;
            let result = ops::rotate(&img, *degrees, *expand, bg_color)?;

            save_and_respond(
                &result,
                output,
                format,
                cli.quiet,
                "rotate",
                &input.display().to_string(),
                (orig_width, orig_height),
            )
        }

        Command::Flip {
            horizontal,
            vertical,
            input,
            output,
        } => {
            check_output_overwrite(output, cli.overwrite)?;
            let img = ops::load_image(input)?;
            let orig_width = img.width();
            let orig_height = img.height();

            let result = ops::flip(&img, *horizontal, *vertical)?;

            save_and_respond(
                &result,
                output,
                format,
                cli.quiet,
                "flip",
                &input.display().to_string(),
                (orig_width, orig_height),
            )
        }

        Command::Resize {
            width,
            height,
            scale,
            filter,
            input,
            output,
        } => {
            check_output_overwrite(output, cli.overwrite)?;
            let img = ops::load_image(input)?;
            let orig_width = img.width();
            let orig_height = img.height();

            let result = ops::resize(&img, *width, *height, *scale, *filter)?;

            save_and_respond(
                &result,
                output,
                format,
                cli.quiet,
                "resize",
                &input.display().to_string(),
                (orig_width, orig_height),
            )
        }

        Command::Fit {
            max_width,
            max_height,
            upscale,
            filter,
            input,
            output,
        } => {
            check_output_overwrite(output, cli.overwrite)?;
            let img = ops::load_image(input)?;
            let orig_width = img.width();
            let orig_height = img.height();

            let result = ops::fit(&img, *max_width, *max_height, *upscale, *filter)?;

            save_and_respond(
                &result,
                output,
                format,
                cli.quiet,
                "fit",
                &input.display().to_string(),
                (orig_width, orig_height),
            )
        }

        Command::Convert {
            format: img_format,
            quality,
            input,
            output,
        } => {
            check_output_overwrite(output, cli.overwrite)?;
            let img = ops::load_image(input)?;
            let orig_width = img.width();
            let orig_height = img.height();

            let target_format = ops::determine_format(output, *img_format)?;
            ops::save_with_format(&img, output, target_format, *quality)?;

            if format == OutputFormat::Json {
                let response = SuccessResponse::new("convert")
                    .with_input(&input.display().to_string())
                    .with_output(&output.display().to_string())
                    .with_detail("original_width", orig_width)
                    .with_detail("original_height", orig_height)
                    .with_detail("result_width", img.width())
                    .with_detail("result_height", img.height())
                    .with_detail("format", format!("{:?}", target_format));
                println!("{}", response.to_json());
            } else if !cli.quiet {
                println!(
                    "Converted {} -> {} ({:?})",
                    input.display(),
                    output.display(),
                    target_format
                );
            }

            Ok(exit_codes::SUCCESS)
        }

        Command::Grayscale {
            no_preserve_alpha,
            input,
            output,
        } => {
            check_output_overwrite(output, cli.overwrite)?;
            let img = ops::load_image(input)?;
            let orig_width = img.width();
            let orig_height = img.height();

            let result = ops::grayscale(&img, !no_preserve_alpha)?;

            save_and_respond(
                &result,
                output,
                format,
                cli.quiet,
                "grayscale",
                &input.display().to_string(),
                (orig_width, orig_height),
            )
        }

        Command::Depth {
            bits,
            dither,
            input,
            output,
        } => {
            check_output_overwrite(output, cli.overwrite)?;
            let img = ops::load_image(input)?;
            let orig_width = img.width();
            let orig_height = img.height();

            let result = ops::change_depth(&img, *bits, *dither)?;

            save_and_respond(
                &result,
                output,
                format,
                cli.quiet,
                "depth",
                &input.display().to_string(),
                (orig_width, orig_height),
            )
        }

        Command::Invert {
            invert_alpha,
            input,
            output,
        } => {
            check_output_overwrite(output, cli.overwrite)?;
            let img = ops::load_image(input)?;
            let orig_width = img.width();
            let orig_height = img.height();

            let result = ops::invert(&img, *invert_alpha)?;

            save_and_respond(
                &result,
                output,
                format,
                cli.quiet,
                "invert",
                &input.display().to_string(),
                (orig_width, orig_height),
            )
        }

        Command::Brightness {
            value,
            input,
            output,
        } => {
            check_output_overwrite(output, cli.overwrite)?;
            let img = ops::load_image(input)?;
            let orig_width = img.width();
            let orig_height = img.height();

            let result = ops::brightness(&img, *value)?;

            save_and_respond(
                &result,
                output,
                format,
                cli.quiet,
                "brightness",
                &input.display().to_string(),
                (orig_width, orig_height),
            )
        }

        Command::Contrast {
            value,
            input,
            output,
        } => {
            check_output_overwrite(output, cli.overwrite)?;
            let img = ops::load_image(input)?;
            let orig_width = img.width();
            let orig_height = img.height();

            let result = ops::contrast(&img, *value)?;

            save_and_respond(
                &result,
                output,
                format,
                cli.quiet,
                "contrast",
                &input.display().to_string(),
                (orig_width, orig_height),
            )
        }

        Command::Gamma {
            value,
            input,
            output,
        } => {
            check_output_overwrite(output, cli.overwrite)?;
            let img = ops::load_image(input)?;
            let orig_width = img.width();
            let orig_height = img.height();

            let result = ops::gamma(&img, *value)?;

            save_and_respond(
                &result,
                output,
                format,
                cli.quiet,
                "gamma",
                &input.display().to_string(),
                (orig_width, orig_height),
            )
        }

        Command::Blur {
            radius,
            input,
            output,
        } => {
            check_output_overwrite(output, cli.overwrite)?;
            let img = ops::load_image(input)?;
            let orig_width = img.width();
            let orig_height = img.height();

            let result = ops::blur(&img, *radius)?;

            save_and_respond(
                &result,
                output,
                format,
                cli.quiet,
                "blur",
                &input.display().to_string(),
                (orig_width, orig_height),
            )
        }

        Command::Sharpen {
            amount,
            radius,
            input,
            output,
        } => {
            check_output_overwrite(output, cli.overwrite)?;
            let img = ops::load_image(input)?;
            let orig_width = img.width();
            let orig_height = img.height();

            let result = ops::sharpen(&img, *amount, *radius)?;

            save_and_respond(
                &result,
                output,
                format,
                cli.quiet,
                "sharpen",
                &input.display().to_string(),
                (orig_width, orig_height),
            )
        }

        Command::Pad {
            all,
            top,
            bottom,
            left,
            right,
            horizontal,
            vertical,
            color,
            input,
            output,
        } => {
            check_output_overwrite(output, cli.overwrite)?;
            let img = ops::load_image(input)?;
            let orig_width = img.width();
            let orig_height = img.height();

            // Calculate padding values with priority: specific > axis > all
            let pad_top = top.or(*vertical).or(*all).unwrap_or(0);
            let pad_bottom = bottom.or(*vertical).or(*all).unwrap_or(0);
            let pad_left = left.or(*horizontal).or(*all).unwrap_or(0);
            let pad_right = right.or(*horizontal).or(*all).unwrap_or(0);

            if pad_top == 0 && pad_bottom == 0 && pad_left == 0 && pad_right == 0 {
                return Err(ImgEditError::InvalidParameter(
                    "At least one padding value must be specified".to_string(),
                ));
            }

            let pad_color = parse_color(color)?;
            let result = ops::pad(&img, pad_top, pad_bottom, pad_left, pad_right, pad_color)?;

            save_and_respond(
                &result,
                output,
                format,
                cli.quiet,
                "pad",
                &input.display().to_string(),
                (orig_width, orig_height),
            )
        }

        Command::Canvas {
            width,
            height,
            anchor,
            color,
            input,
            output,
        } => {
            check_output_overwrite(output, cli.overwrite)?;
            let img = ops::load_image(input)?;
            let orig_width = img.width();
            let orig_height = img.height();

            let bg_color = parse_color(color)?;
            let result = ops::canvas_resize(&img, *width, *height, *anchor, bg_color)?;

            save_and_respond(
                &result,
                output,
                format,
                cli.quiet,
                "canvas",
                &input.display().to_string(),
                (orig_width, orig_height),
            )
        }

        Command::Composite {
            x,
            y,
            anchor,
            opacity,
            blend,
            base,
            overlay,
            output,
        } => {
            check_output_overwrite(output, cli.overwrite)?;
            let base_img = ops::load_image(base)?;
            let overlay_img = ops::load_image(overlay)?;
            let orig_width = base_img.width();
            let orig_height = base_img.height();

            let result = ops::composite(
                &base_img,
                &overlay_img,
                x.unwrap_or(0),
                y.unwrap_or(0),
                *anchor,
                *opacity,
                *blend,
            )?;

            save_and_respond(
                &result,
                output,
                format,
                cli.quiet,
                "composite",
                &base.display().to_string(),
                (orig_width, orig_height),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mdimgedit::cli::args::{Anchor, BlendMode, ImageFormat, ResizeFilter};
    use std::path::PathBuf;

    #[test]
    fn test_command_name() {
        let p = PathBuf::from("test.png");

        assert_eq!(command_name(&Command::Info { input: p.clone() }), "info");
        assert_eq!(
            command_name(&Command::Exif {
                verbose: false,
                tag: None,
                input: p.clone()
            }),
            "exif"
        );
        assert_eq!(
            command_name(&Command::Crop {
                x: 0,
                y: 0,
                width: 10,
                height: 10,
                anchor: Anchor::TopLeft,
                input: p.clone(),
                output: p.clone()
            }),
            "crop"
        );
        assert_eq!(
            command_name(&Command::Rotate {
                degrees: 90.0,
                expand: false,
                background: "transparent".to_string(),
                input: p.clone(),
                output: p.clone()
            }),
            "rotate"
        );
        assert_eq!(
            command_name(&Command::Flip {
                horizontal: true,
                vertical: false,
                input: p.clone(),
                output: p.clone()
            }),
            "flip"
        );
        assert_eq!(
            command_name(&Command::Resize {
                width: Some(10),
                height: None,
                scale: None,
                filter: ResizeFilter::Lanczos,
                input: p.clone(),
                output: p.clone()
            }),
            "resize"
        );
        assert_eq!(
            command_name(&Command::Fit {
                max_width: Some(10),
                max_height: None,
                upscale: false,
                filter: ResizeFilter::Lanczos,
                input: p.clone(),
                output: p.clone()
            }),
            "fit"
        );
        assert_eq!(
            command_name(&Command::Convert {
                format: Some(ImageFormat::Png),
                quality: 90,
                input: p.clone(),
                output: p.clone()
            }),
            "convert"
        );
        assert_eq!(
            command_name(&Command::Grayscale {
                no_preserve_alpha: false,
                input: p.clone(),
                output: p.clone()
            }),
            "grayscale"
        );
        assert_eq!(
            command_name(&Command::Depth {
                bits: 8,
                dither: false,
                input: p.clone(),
                output: p.clone()
            }),
            "depth"
        );
        assert_eq!(
            command_name(&Command::Invert {
                invert_alpha: false,
                input: p.clone(),
                output: p.clone()
            }),
            "invert"
        );
        assert_eq!(
            command_name(&Command::Brightness {
                value: 10,
                input: p.clone(),
                output: p.clone()
            }),
            "brightness"
        );
        assert_eq!(
            command_name(&Command::Contrast {
                value: 1.0,
                input: p.clone(),
                output: p.clone()
            }),
            "contrast"
        );
        assert_eq!(
            command_name(&Command::Gamma {
                value: 1.0,
                input: p.clone(),
                output: p.clone()
            }),
            "gamma"
        );
        assert_eq!(
            command_name(&Command::Blur {
                radius: 1.0,
                input: p.clone(),
                output: p.clone()
            }),
            "blur"
        );
        assert_eq!(
            command_name(&Command::Sharpen {
                amount: 1.0,
                radius: 1.0,
                input: p.clone(),
                output: p.clone()
            }),
            "sharpen"
        );
        assert_eq!(
            command_name(&Command::Pad {
                all: Some(10),
                top: None,
                bottom: None,
                left: None,
                right: None,
                horizontal: None,
                vertical: None,
                color: "transparent".to_string(),
                input: p.clone(),
                output: p.clone()
            }),
            "pad"
        );
        assert_eq!(
            command_name(&Command::Canvas {
                width: 100,
                height: 100,
                anchor: Anchor::Center,
                color: "transparent".to_string(),
                input: p.clone(),
                output: p.clone()
            }),
            "canvas"
        );
        assert_eq!(
            command_name(&Command::Composite {
                x: None,
                y: None,
                anchor: None,
                opacity: 1.0,
                blend: BlendMode::Normal,
                base: p.clone(),
                overlay: p.clone(),
                output: p.clone()
            }),
            "composite"
        );
    }
}
