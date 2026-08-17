//! Typed errors for `babydra-core`.
//!
//! Replaces the legacy `Result<_, String>` signatures so callers can match on
//! error kinds instead of string-matching. `From<String>` keeps migration
//! cheap: existing `Err(format!(...))` sites compile via `.into()`.

use thiserror::Error;

/// Common error type for all core services.
#[derive(Debug, Error)]
pub enum CoreError {
    /// Underlying I/O failure (file read/write, command spawn, …).
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// A spawned system command failed (non-zero exit or missing binary).
    #[error("command failed: {0}")]
    Command(String),

    /// User-provided input is invalid (path, name, value…).
    #[error("invalid input: {0}")]
    Invalid(String),

    /// The requested resource does not exist.
    #[error("not found: {0}")]
    NotFound(String),

    /// Generic / unmapped error message.
    #[error("{0}")]
    Message(String),
}

impl From<String> for CoreError {
    fn from(s: String) -> Self {
        CoreError::Message(s)
    }
}

impl From<&str> for CoreError {
    fn from(s: &str) -> Self {
        CoreError::Message(s.to_string())
    }
}

/// Convenience constructor mirroring `format!`.
impl CoreError {
    pub fn msg(msg: impl Into<String>) -> Self {
        CoreError::Message(msg.into())
    }
}

/// Common alias used across services.
pub type CoreResult<T> = Result<T, CoreError>;
