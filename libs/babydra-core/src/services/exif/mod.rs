//! EXIF tag metadata extraction parser.

pub use crate::models::shell::exif::ExifData;
use exif::{In, Reader, Tag};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Attempts to open the image file and extract camera parameters from EXIF headers.
pub fn read_exif(path: &Path) -> Option<ExifData> {
    let file = File::open(path).ok()?;
    let mut bufreader = BufReader::new(file);
    let exifreader = Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader).ok()?;

    let get_tag = |tag: Tag| -> Option<String> {
        exif.get_field(tag, In::PRIMARY)
            .map(|field| field.display_value().to_string())
    };

    Some(ExifData {
        make: get_tag(Tag::Make),
        model: get_tag(Tag::Model),
        aperture: get_tag(Tag::FNumber),
        exposure_time: get_tag(Tag::ExposureTime),
        iso: get_tag(Tag::PhotographicSensitivity),
        focal_length: get_tag(Tag::FocalLength),
        lens_model: get_tag(Tag::LensModel),
        date_time: get_tag(Tag::DateTimeOriginal),
    })
}
