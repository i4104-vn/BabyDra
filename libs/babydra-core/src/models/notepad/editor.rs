//! Editor session state models.

use std::path::PathBuf;

/// Active text document session state.
#[derive(Debug, Clone)]
pub struct EditorState {
    pub file_path: Option<PathBuf>,
    pub is_modified: bool,
    pub language: String,
    pub encoding: String,
    pub line_count: usize,
    pub file_size: u64,
    pub is_picking: bool,
}

impl EditorState {
    /// Creates a new editor state.
    pub fn new(
        file_path: Option<PathBuf>,
        language: impl Into<String>,
        encoding: impl Into<String>,
        line_count: usize,
        file_size: u64,
    ) -> Self {
        Self {
            file_path,
            is_modified: false,
            language: language.into(),
            encoding: encoding.into(),
            line_count,
            file_size,
            is_picking: false,
        }
    }

    /// Returns the file name of the document, or None if untitled.
    pub fn file_name(&self) -> Option<String> {
        self.file_path
            .as_ref()
            .and_then(|p| p.file_name())
            .map(|s| s.to_string_lossy().to_string())
    }

    /// Returns true if this is an unsaved, untitled document.
    pub fn is_untitled(&self) -> bool {
        self.file_path.is_none()
    }

    /// Formats the window title according to document modified state.
    pub fn display_title(&self, fallback: &str) -> String {
        let name = self.file_name().unwrap_or_else(|| fallback.to_string());
        if self.is_modified {
            format!("● {}", name)
        } else {
            name
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_editor_state_lifecycle() {
        let state = EditorState::new(
            Some(PathBuf::from("/tmp/test.rs")),
            "Rust",
            "UTF-8",
            10,
            256,
        );
        assert_eq!(state.file_name(), Some("test.rs".to_string()));
        assert!(!state.is_untitled());
        assert_eq!(state.display_title("Untitled"), "test.rs");

        let mut modified_state = state;
        modified_state.is_modified = true;
        assert_eq!(modified_state.display_title("Untitled"), "● test.rs");
    }

    #[test]
    fn test_untitled_state() {
        let state = EditorState::new(None, "Plain Text", "UTF-8", 1, 0);
        assert!(state.is_untitled());
        assert_eq!(state.display_title("Untitled"), "Untitled");
    }
}
