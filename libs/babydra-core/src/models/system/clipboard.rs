//! Clipboard item models.

use std::time::Instant;

/// Represents a single item stored in clipboard history.
#[derive(Clone, Debug)]
pub enum ClipboardEntry {
    Text {
        content: String,
        timestamp: Instant,
    },
    Image {
        png_bytes: Vec<u8>,
        timestamp: Instant,
    },
}

impl ClipboardEntry {
    pub fn is_text(&self) -> bool {
        matches!(self, Self::Text { .. })
    }

    pub fn is_image(&self) -> bool {
        matches!(self, Self::Image { .. })
    }

    pub fn timestamp(&self) -> Instant {
        match self {
            Self::Text { timestamp, .. } => *timestamp,
            Self::Image { timestamp, .. } => *timestamp,
        }
    }

    /// Formats up to 3 lines of preview text and returns `(text, excess_lines_count)`.
    ///
    /// If text contains multiple lines, line breaks are preserved up to 3 lines.
    /// If total lines exceed 3, returns the first 3 lines and `excess_lines_count = total - 3`.
    pub fn preview_lines(&self) -> (String, usize) {
        match self {
            Self::Text { content, .. } => {
                let trimmed = content
                    .trim_start_matches(|c| c == '\r' || c == '\n')
                    .trim_end();
                if trimmed.trim().is_empty() {
                    return (String::new(), 0);
                }
                let raw_lines: Vec<&str> = trimmed.lines().collect();
                let total = raw_lines.len();
                if total <= 3 {
                    let cleaned: Vec<String> = raw_lines.into_iter().map(clean_line).collect();
                    (cleaned.join("\n"), 0)
                } else {
                    let cleaned: Vec<String> = raw_lines[..3]
                        .iter()
                        .copied()
                        .map(clean_line)
                        .collect();
                    let excess = total - 3;
                    (cleaned.join("\n"), excess)
                }
            }
            Self::Image { .. } => (String::new(), 0),
        }
    }

    pub fn preview_text(&self) -> String {
        let (lines, _) = self.preview_lines();
        lines
    }
}

fn clean_line(line: &str) -> String {
    let expanded = line.replace('\t', "    ");
    if expanded.chars().count() > 80 {
        let mut s: String = expanded.chars().take(77).collect();
        s.push_str("...");
        s
    } else {
        expanded
    }
}
