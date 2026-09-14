use serde::{Deserialize, Serialize};

fn default_true() -> bool {
    true
}

/// A predefined system shortcut for a BabyDra crate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemShortcut {
    pub id: String,
    pub crate_name: String,
    pub name_key: String,
    pub description_key: String,
    pub command: String,
    pub default_modifiers: String,
    pub default_key: String,
    pub modifiers: String,
    pub key: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
}

impl SystemShortcut {
    /// Builds the full combo spec (e.g. `"W-q"`).
    pub fn combo(&self) -> String {
        if self.modifiers.is_empty() {
            self.key.clone()
        } else {
            format!("{}-{}", self.modifiers, self.key)
        }
    }
}

/// A global keyboard shortcut handled by the `babydra-keymap` daemon.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shortcut {
    pub id: usize,
    /// Modifiers as labwc-style letters joined by `-`: `S` (Shift), `C`
    /// (Ctrl), `A` (Alt), `W` (Super).  Empty string for no modifier.
    pub modifiers: String,
    /// Key name without the `KEY_` prefix (e.g. `Tab`, `F12`, `Print`).
    pub key: String,
    /// Shell command executed when the shortcut fires.
    pub command: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
}

impl Shortcut {
    /// Builds the full combo spec (e.g. `"A-Tab"`).
    pub fn combo(&self) -> String {
        if self.modifiers.is_empty() {
            self.key.clone()
        } else {
            format!("{}-{}", self.modifiers, self.key)
        }
    }
}
