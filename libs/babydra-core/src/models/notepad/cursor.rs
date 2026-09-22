//! Text cursor position coordinates model.

/// 1-based text cursor position (Line, Column).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CursorPosition {
    pub line: usize,
    pub column: usize,
}

impl CursorPosition {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    pub fn display_string(&self) -> String {
        format!("Ln {}, Col {}", self.line, self.column)
    }
}
