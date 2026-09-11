//! Image metadata data model for Explore.

use serde::{Deserialize, Serialize};

/// Comprehensive image metadata.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ImageMetadata {
    pub width: u32,
    pub height: u32,
    pub dimensions_str: String,
    pub aspect_ratio: String,
    pub total_pixels: u64,
    pub pixels_str: String,
    pub dpi: Option<u32>,
    pub dpi_str: String,
    pub format: String,
    pub color_space: Option<String>,
    pub bit_depth: Option<u32>,
    pub camera_model: Option<String>,
    pub lens_model: Option<String>,
    pub exposure: Option<String>,
    pub date_taken: Option<String>,
}
