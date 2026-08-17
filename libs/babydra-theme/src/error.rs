//! Typed errors for the theme engine.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ThemeError {
    /// Theme package folder not found.
    #[error("theme package not found: {0}")]
    NotFound(String),

    /// tokens.json / fonts.json could not be read or parsed.
    #[error("invalid theme package: {0}")]
    Invalid(String),

    /// Inheritance cycle detected while resolving a theme.
    #[error("theme inheritance cycle detected at '{0}'")]
    Cycle(String),

    /// Underlying I/O failure.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

impl From<String> for ThemeError {
    fn from(s: String) -> Self {
        ThemeError::Invalid(s)
    }
}

impl From<&str> for ThemeError {
    fn from(s: &str) -> Self {
        ThemeError::Invalid(s.to_string())
    }
}
