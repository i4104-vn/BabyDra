//! EXIF image metadata data models.

use serde::{Deserialize, Serialize};

/// Extracted EXIF photography metadata tags.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ExifData {
    pub make: Option<String>,
    pub model: Option<String>,
    pub aperture: Option<String>,
    pub exposure_time: Option<String>,
    pub iso: Option<String>,
    pub focal_length: Option<String>,
    pub lens_model: Option<String>,
    pub date_time: Option<String>,
}
