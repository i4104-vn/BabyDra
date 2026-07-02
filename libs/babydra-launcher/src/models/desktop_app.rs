//! Structure representing basic desktop entries.

/// Information model of a parsed desktop entry application.
#[derive(Clone, Debug)]
pub struct DesktopApp {
    /// Friendly user-facing name of the application.
    pub name: String,
    /// Absolute or path executable execute command.
    pub exec: String,
    /// System icon theme name or filepath.
    pub icon: Option<String>,
}

