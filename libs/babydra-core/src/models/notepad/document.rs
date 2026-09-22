//! Document file metadata and inspection models.

use std::path::Path;

/// Metadata for an open or saved document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentInfo {
    pub file_name: String,
    pub file_size: u64,
    pub line_count: usize,
    pub encoding: String,
}

impl DocumentInfo {
    /// Constructs document metadata from an optional path and content string.
    pub fn from_path_and_text(path: Option<&Path>, text: &str, encoding: &str) -> Self {
        let file_name = path
            .and_then(|p| p.file_name())
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
        let file_size = path.map_or(text.len() as u64, |p| {
            std::fs::metadata(p).map_or(text.len() as u64, |m| m.len())
        });
        let line_count = text.lines().count().max(1);

        Self {
            file_name,
            file_size,
            line_count,
            encoding: encoding.to_string(),
        }
    }
}
