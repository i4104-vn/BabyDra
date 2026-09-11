//! Image metadata extraction utilities for Explore.
//! Extracts dimensions, aspect ratio, total pixels, megapixels, DPI, format, color space,
//! and EXIF camera parameters.

use gdk_pixbuf::Pixbuf;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

pub use crate::models::desktop::image_meta::ImageMetadata;

/// Formats an integer with commas as thousands separators (e.g. 2073600 -> "2,073,600").
pub fn format_commas(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::with_capacity(s.len() + s.len() / 3);
    let rem = s.len() % 3;
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (i % 3 == rem || (rem == 0 && i % 3 == 0)) {
            result.push(',');
        }
        result.push(c);
    }
    result
}

/// Calculates human-readable aspect ratio from width and height.
pub fn calculate_aspect_ratio(w: u32, h: u32) -> String {
    if w == 0 || h == 0 {
        return "--".to_string();
    }

    let ratio = w as f64 / h as f64;
    const COMMON: &[(f64, &str)] = &[
        (1.0, "1:1"),
        (4.0 / 3.0, "4:3"),
        (3.0 / 4.0, "3:4"),
        (3.0 / 2.0, "3:2"),
        (2.0 / 3.0, "2:3"),
        (16.0 / 9.0, "16:9"),
        (9.0 / 16.0, "9:16"),
        (16.0 / 10.0, "16:10"),
        (10.0 / 16.0, "10:16"),
        (21.0 / 9.0, "21:9"),
        (9.0 / 21.0, "9:21"),
        (5.0 / 4.0, "5:4"),
        (4.0 / 5.0, "4:5"),
    ];

    for &(target, label) in COMMON {
        if (ratio - target).abs() < 0.015 {
            return label.to_string();
        }
    }

    fn gcd(mut a: u32, mut b: u32) -> u32 {
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        a
    }

    let g = gcd(w, h);
    let rw = w / g;
    let rh = h / g;
    if rw <= 32 && rh <= 32 {
        format!("{}:{}", rw, rh)
    } else {
        format!("{:.2}:1", ratio)
    }
}

/// Attempts to read DPI from PNG pHYs chunk.
fn read_dpi_from_png(bytes: &[u8]) -> Option<u32> {
    if bytes.len() < 8 || &bytes[0..8] != b"\x89PNG\r\n\x1a\n" {
        return None;
    }
    let mut offset = 8;
    while offset + 8 <= bytes.len() {
        let chunk_len = u32::from_be_bytes(bytes[offset..offset + 4].try_into().ok()?) as usize;
        let chunk_type = &bytes[offset + 4..offset + 8];
        let data_offset = offset + 8;
        if chunk_type == b"pHYs" && data_offset + 9 <= bytes.len() {
            let ppu_x = u32::from_be_bytes(bytes[data_offset..data_offset + 4].try_into().ok()?);
            let unit = bytes[data_offset + 8];
            if unit == 1 && ppu_x > 0 {
                let dpi = (ppu_x as f64 * 0.0254).round() as u32;
                if dpi > 0 {
                    return Some(dpi);
                }
            }
        }
        if chunk_type == b"IDAT" || chunk_type == b"IEND" {
            break;
        }
        offset = data_offset + chunk_len + 4;
    }
    None
}

/// Attempts to read DPI from JPEG JFIF (APP0) marker.
fn read_dpi_from_jpeg(bytes: &[u8]) -> Option<u32> {
    if bytes.len() < 4 || bytes[0] != 0xFF || bytes[1] != 0xD8 {
        return None;
    }
    let mut offset = 2;
    while offset + 4 < bytes.len() {
        if bytes[offset] != 0xFF {
            break;
        }
        let marker = bytes[offset + 1];
        if marker == 0xDA || marker == 0xD9 {
            break;
        }
        let seg_len = u16::from_be_bytes(bytes[offset + 2..offset + 4].try_into().ok()?) as usize;
        if marker == 0xE0 {
            let data_offset = offset + 4;
            if data_offset + 10 <= bytes.len() && &bytes[data_offset..data_offset + 5] == b"JFIF\0" {
                let units = bytes[data_offset + 7];
                let x_density =
                    u16::from_be_bytes(bytes[data_offset + 8..data_offset + 10].try_into().ok()?);
                if units == 1 && x_density > 0 {
                    return Some(x_density as u32);
                } else if units == 2 && x_density > 0 {
                    return Some((x_density as f64 * 2.54).round() as u32);
                }
            }
        }
        offset += 2 + seg_len;
    }
    None
}

/// Attempts to read DPI from EXIF tags.
fn read_dpi_from_exif(path: &Path) -> Option<u32> {
    let file = File::open(path).ok()?;
    let mut bufreader = BufReader::new(file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader).ok()?;

    let x_res = exif.get_field(exif::Tag::XResolution, exif::In::PRIMARY)?;
    let unit = exif
        .get_field(exif::Tag::ResolutionUnit, exif::In::PRIMARY)
        .and_then(|f| f.value.get_uint(0))
        .unwrap_or(2);

    let dpi_f64 = match &x_res.value {
        exif::Value::Rational(rats) => rats.first().map(|r| r.to_f64()),
        _ => None,
    }?;

    let dpi = if unit == 3 {
        (dpi_f64 * 2.54).round() as u32
    } else {
        dpi_f64.round() as u32
    };

    if (1..=100_000).contains(&dpi) {
        Some(dpi)
    } else {
        None
    }
}

/// Extracts detailed image metadata from an image file path.
pub fn read_image_metadata(path: &Path) -> Option<ImageMetadata> {
    // 1. Get dimensions & format from GdkPixbuf file_info
    let (format_obj, w_i32, h_i32) = Pixbuf::file_info(path)?;
    let width = w_i32.max(0) as u32;
    let height = h_i32.max(0) as u32;
    if width == 0 || height == 0 {
        return None;
    }

    let format_raw = format_obj
        .name()
        .map(|s| s.to_string())
        .unwrap_or_else(|| "image".to_string());
    let format_upper = match format_raw.to_lowercase().as_str() {
        "png" => "PNG".to_string(),
        "jpeg" => "JPEG".to_string(),
        "webp" => "WebP".to_string(),
        "gif" => "GIF".to_string(),
        "svg" => "SVG".to_string(),
        "bmp" => "BMP".to_string(),
        "ico" => "ICO".to_string(),
        "tiff" => "TIFF".to_string(),
        "avif" => "AVIF".to_string(),
        other => other.to_uppercase(),
    };

    let aspect_ratio = calculate_aspect_ratio(width, height);
    let total_pixels = (width as u64) * (height as u64);
    let megapixels = (total_pixels as f64) / 1_000_000.0;

    let pixels_str = if megapixels >= 0.1 {
        format!("{:.2} MP ({} px)", megapixels, format_commas(total_pixels))
    } else {
        format!("{} px", format_commas(total_pixels))
    };

    let dimensions_str = format!("{} × {} px ({})", width, height, aspect_ratio);

    // 2. Read DPI
    let mut header_buf = [0u8; 4096];
    let header_bytes = if let Ok(mut f) = File::open(path) {
        let n = f.read(&mut header_buf).unwrap_or(0);
        &header_buf[..n]
    } else {
        &[][..]
    };

    let dpi = read_dpi_from_exif(path)
        .or_else(|| read_dpi_from_png(header_bytes))
        .or_else(|| read_dpi_from_jpeg(header_bytes));

    let dpi_str = match dpi {
        Some(d) => format!("{} DPI", d),
        None => "--".to_string(),
    };

    // 3. Color space & bit depth estimation from header
    let (color_space, bit_depth) = if format_upper == "PNG" && header_bytes.len() >= 26 {
        // IHDR starts at offset 12: width(4), height(4), bit_depth(1), color_type(1)
        let depth = header_bytes[24] as u32;
        let ctype = header_bytes[25];
        let cs = match ctype {
            0 => format!("Grayscale ({}-bit)", depth),
            2 => format!("RGB ({}-bit)", depth * 3),
            3 => format!("Indexed ({}-bit)", depth),
            4 => format!("Grayscale + Alpha ({}-bit)", depth * 2),
            6 => format!("RGBA ({}-bit)", depth * 4),
            _ => format!("Color ({}-bit)", depth),
        };
        (Some(cs), Some(depth))
    } else if format_upper == "JPEG" {
        (Some("sRGB (24-bit)".to_string()), Some(8))
    } else if format_upper == "SVG" {
        (Some("Vector (Scalable)".to_string()), None)
    } else {
        (None, None)
    };

    // 4. EXIF photography parameters
    let exif_data = crate::services::exif::read_exif(path);
    let camera_model = exif_data.as_ref().and_then(|e| {
        match (&e.make, &e.model) {
            (Some(make), Some(model)) => {
                if model.to_lowercase().starts_with(&make.to_lowercase()) {
                    Some(model.clone())
                } else {
                    Some(format!("{} {}", make, model))
                }
            }
            (None, Some(model)) => Some(model.clone()),
            (Some(make), None) => Some(make.clone()),
            (None, None) => None,
        }
    });

    let lens_model = exif_data.as_ref().and_then(|e| e.lens_model.clone());

    let exposure = exif_data.as_ref().and_then(|e| {
        let mut parts = Vec::new();
        if let Some(ref f) = e.aperture {
            parts.push(f.clone());
        }
        if let Some(ref t) = e.exposure_time {
            parts.push(t.clone());
        }
        if let Some(ref iso) = e.iso {
            parts.push(format!("ISO {}", iso));
        }
        if let Some(ref focal) = e.focal_length {
            parts.push(focal.clone());
        }
        if parts.is_empty() {
            None
        } else {
            Some(parts.join(", "))
        }
    });

    let date_taken = exif_data.as_ref().and_then(|e| e.date_time.clone());

    Some(ImageMetadata {
        width,
        height,
        dimensions_str,
        aspect_ratio,
        total_pixels,
        pixels_str,
        dpi,
        dpi_str,
        format: format_upper,
        color_space,
        bit_depth,
        camera_model,
        lens_model,
        exposure,
        date_taken,
    })
}
