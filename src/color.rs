use crate::error::{ImgEditError, Result};
use image::Rgba;

/// Parse a color string into an RGBA value.
///
/// Supported formats:
/// - Named colors: black, white, red, green, blue, yellow, cyan, magenta, transparent
/// - Hex3: #RGB
/// - Hex4: #RGBA
/// - Hex6: #RRGGBB
/// - Hex8: #RRGGBBAA
/// - RGB: rgb(R,G,B)
/// - RGBA: rgba(R,G,B,A)
pub fn parse_color(s: &str) -> Result<Rgba<u8>> {
    let s = s.trim().to_lowercase();

    // Try named colors first
    if let Some(color) = parse_named_color(&s) {
        return Ok(color);
    }

    // Try hex format
    if let Some(hex) = s.strip_prefix('#') {
        return parse_hex_color(hex);
    }

    // Try rgb/rgba format
    if s.starts_with("rgb(") && s.ends_with(')') {
        return parse_rgb_color(&s[4..s.len() - 1]);
    }

    if s.starts_with("rgba(") && s.ends_with(')') {
        return parse_rgba_color(&s[5..s.len() - 1]);
    }

    Err(ImgEditError::InvalidColor(format!(
        "Unrecognized color format: {}",
        s
    )))
}

fn parse_named_color(s: &str) -> Option<Rgba<u8>> {
    match s {
        "black" => Some(Rgba([0, 0, 0, 255])),
        "white" => Some(Rgba([255, 255, 255, 255])),
        "red" => Some(Rgba([255, 0, 0, 255])),
        "green" => Some(Rgba([0, 255, 0, 255])),
        "blue" => Some(Rgba([0, 0, 255, 255])),
        "yellow" => Some(Rgba([255, 255, 0, 255])),
        "cyan" => Some(Rgba([0, 255, 255, 255])),
        "magenta" => Some(Rgba([255, 0, 255, 255])),
        "transparent" => Some(Rgba([0, 0, 0, 0])),
        _ => None,
    }
}

fn parse_hex_color(hex: &str) -> Result<Rgba<u8>> {
    let hex = hex.trim();

    match hex.len() {
        3 => {
            // #RGB -> #RRGGBB
            let r = parse_hex_digit(hex.chars().next().unwrap())?;
            let g = parse_hex_digit(hex.chars().nth(1).unwrap())?;
            let b = parse_hex_digit(hex.chars().nth(2).unwrap())?;
            Ok(Rgba([r * 17, g * 17, b * 17, 255]))
        }
        4 => {
            // #RGBA -> #RRGGBBAA
            let r = parse_hex_digit(hex.chars().next().unwrap())?;
            let g = parse_hex_digit(hex.chars().nth(1).unwrap())?;
            let b = parse_hex_digit(hex.chars().nth(2).unwrap())?;
            let a = parse_hex_digit(hex.chars().nth(3).unwrap())?;
            Ok(Rgba([r * 17, g * 17, b * 17, a * 17]))
        }
        6 => {
            // #RRGGBB
            let r = parse_hex_byte(&hex[0..2])?;
            let g = parse_hex_byte(&hex[2..4])?;
            let b = parse_hex_byte(&hex[4..6])?;
            Ok(Rgba([r, g, b, 255]))
        }
        8 => {
            // #RRGGBBAA
            let r = parse_hex_byte(&hex[0..2])?;
            let g = parse_hex_byte(&hex[2..4])?;
            let b = parse_hex_byte(&hex[4..6])?;
            let a = parse_hex_byte(&hex[6..8])?;
            Ok(Rgba([r, g, b, a]))
        }
        _ => Err(ImgEditError::InvalidColor(format!(
            "Invalid hex color length: {}",
            hex
        ))),
    }
}

fn parse_hex_digit(c: char) -> Result<u8> {
    c.to_digit(16)
        .map(|d| d as u8)
        .ok_or_else(|| ImgEditError::InvalidColor(format!("Invalid hex digit: {}", c)))
}

fn parse_hex_byte(s: &str) -> Result<u8> {
    u8::from_str_radix(s, 16)
        .map_err(|_| ImgEditError::InvalidColor(format!("Invalid hex byte: {}", s)))
}

fn parse_rgb_color(inner: &str) -> Result<Rgba<u8>> {
    let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
    if parts.len() != 3 {
        return Err(ImgEditError::InvalidColor(format!(
            "rgb() requires 3 values, got {}",
            parts.len()
        )));
    }

    let r = parse_color_component(parts[0])?;
    let g = parse_color_component(parts[1])?;
    let b = parse_color_component(parts[2])?;

    Ok(Rgba([r, g, b, 255]))
}

fn parse_rgba_color(inner: &str) -> Result<Rgba<u8>> {
    let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
    if parts.len() != 4 {
        return Err(ImgEditError::InvalidColor(format!(
            "rgba() requires 4 values, got {}",
            parts.len()
        )));
    }

    let r = parse_color_component(parts[0])?;
    let g = parse_color_component(parts[1])?;
    let b = parse_color_component(parts[2])?;
    let a = parse_color_component(parts[3])?;

    Ok(Rgba([r, g, b, a]))
}

fn parse_color_component(s: &str) -> Result<u8> {
    s.parse::<u8>()
        .map_err(|_| ImgEditError::InvalidColor(format!("Invalid color component: {}", s)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_named_colors() {
        assert_eq!(parse_color("black").unwrap(), Rgba([0, 0, 0, 255]));
        assert_eq!(parse_color("white").unwrap(), Rgba([255, 255, 255, 255]));
        assert_eq!(parse_color("red").unwrap(), Rgba([255, 0, 0, 255]));
        assert_eq!(parse_color("green").unwrap(), Rgba([0, 255, 0, 255]));
        assert_eq!(parse_color("blue").unwrap(), Rgba([0, 0, 255, 255]));
        assert_eq!(parse_color("yellow").unwrap(), Rgba([255, 255, 0, 255]));
        assert_eq!(parse_color("cyan").unwrap(), Rgba([0, 255, 255, 255]));
        assert_eq!(parse_color("magenta").unwrap(), Rgba([255, 0, 255, 255]));
        assert_eq!(parse_color("transparent").unwrap(), Rgba([0, 0, 0, 0]));
    }

    #[test]
    fn test_named_colors_case_insensitive() {
        assert_eq!(parse_color("BLACK").unwrap(), Rgba([0, 0, 0, 255]));
        assert_eq!(parse_color("White").unwrap(), Rgba([255, 255, 255, 255]));
        assert_eq!(parse_color("RED").unwrap(), Rgba([255, 0, 0, 255]));
    }

    #[test]
    fn test_hex3() {
        assert_eq!(parse_color("#fff").unwrap(), Rgba([255, 255, 255, 255]));
        assert_eq!(parse_color("#000").unwrap(), Rgba([0, 0, 0, 255]));
        assert_eq!(parse_color("#f00").unwrap(), Rgba([255, 0, 0, 255]));
        assert_eq!(parse_color("#0f0").unwrap(), Rgba([0, 255, 0, 255]));
        assert_eq!(parse_color("#00f").unwrap(), Rgba([0, 0, 255, 255]));
        assert_eq!(parse_color("#abc").unwrap(), Rgba([170, 187, 204, 255]));
    }

    #[test]
    fn test_hex4() {
        assert_eq!(parse_color("#ffff").unwrap(), Rgba([255, 255, 255, 255]));
        assert_eq!(parse_color("#fff0").unwrap(), Rgba([255, 255, 255, 0]));
        assert_eq!(parse_color("#0000").unwrap(), Rgba([0, 0, 0, 0]));
        assert_eq!(parse_color("#f008").unwrap(), Rgba([255, 0, 0, 136]));
    }

    #[test]
    fn test_hex6() {
        assert_eq!(parse_color("#ffffff").unwrap(), Rgba([255, 255, 255, 255]));
        assert_eq!(parse_color("#000000").unwrap(), Rgba([0, 0, 0, 255]));
        assert_eq!(parse_color("#ff5500").unwrap(), Rgba([255, 85, 0, 255]));
        assert_eq!(parse_color("#123456").unwrap(), Rgba([18, 52, 86, 255]));
    }

    #[test]
    fn test_hex8() {
        assert_eq!(
            parse_color("#ffffffff").unwrap(),
            Rgba([255, 255, 255, 255])
        );
        assert_eq!(parse_color("#00000000").unwrap(), Rgba([0, 0, 0, 0]));
        assert_eq!(parse_color("#ff550080").unwrap(), Rgba([255, 85, 0, 128]));
    }

    #[test]
    fn test_rgb() {
        assert_eq!(
            parse_color("rgb(255,255,255)").unwrap(),
            Rgba([255, 255, 255, 255])
        );
        assert_eq!(parse_color("rgb(0,0,0)").unwrap(), Rgba([0, 0, 0, 255]));
        assert_eq!(
            parse_color("rgb(255, 128, 0)").unwrap(),
            Rgba([255, 128, 0, 255])
        );
    }

    #[test]
    fn test_rgba() {
        assert_eq!(
            parse_color("rgba(255,255,255,255)").unwrap(),
            Rgba([255, 255, 255, 255])
        );
        assert_eq!(parse_color("rgba(0,0,0,0)").unwrap(), Rgba([0, 0, 0, 0]));
        assert_eq!(
            parse_color("rgba(255, 128, 0, 128)").unwrap(),
            Rgba([255, 128, 0, 128])
        );
    }

    #[test]
    fn test_whitespace_handling() {
        assert_eq!(
            parse_color("  white  ").unwrap(),
            Rgba([255, 255, 255, 255])
        );
        assert_eq!(
            parse_color("rgb( 100 , 150 , 200 )").unwrap(),
            Rgba([100, 150, 200, 255])
        );
    }

    #[test]
    fn test_invalid_colors() {
        assert!(parse_color("notacolor").is_err());
        assert!(parse_color("#gg0000").is_err());
        assert!(parse_color("#12345").is_err()); // Invalid length
        assert!(parse_color("rgb(256,0,0)").is_err()); // Out of range
        assert!(parse_color("rgb(0,0)").is_err()); // Too few components
        assert!(parse_color("rgba(0,0,0)").is_err()); // Too few for rgba
    }
}
