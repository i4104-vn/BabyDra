use serde::{Deserialize, Serialize};

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
