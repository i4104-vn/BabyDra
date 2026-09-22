//! Syntax highlighting and language detection service.
//!
//! Provides globally cached syntax definitions, theme sets, language detection
//! from file paths or shebangs, and text/line highlighting.

pub mod detection;
pub mod highlight;
pub mod theme;

pub use detection::{
    detect_syntax, detect_syntax_by_extension, detect_syntax_by_name, language_name,
    list_supported_languages, syntax_set,
};
pub use highlight::{create_highlighter, highlight_line, highlight_text};
pub use theme::{
    get_theme, get_theme_by_name, list_available_themes, theme_set, DEFAULT_DARK_THEME,
    DEFAULT_LIGHT_THEME,
};

// Re-export common syntect types for consumers
pub use syntect::easy::HighlightLines;
pub use syntect::highlighting::{Color, FontStyle, Style, Theme as SyntectTheme, ThemeSet};
pub use syntect::parsing::{SyntaxReference, SyntaxSet};
pub use syntect::util::LinesWithEndings;
