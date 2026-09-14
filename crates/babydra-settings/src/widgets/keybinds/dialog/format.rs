//! Keyboard shortcut formatting and modifier conversion helpers.

use gtk4::prelude::*;
use gtk4::Button;

/// Canonical modifier letters in serialization order (labwc syntax).
pub const MOD_LETTERS: [&str; 4] = ["W", "C", "A", "S"];
/// Display names matching [`MOD_LETTERS`].
pub const MOD_NAMES: [&str; 4] = ["Super", "Ctrl", "Alt", "Shift"];

/// Formats a combo for display, e.g. `"Super + Tab"`.
pub fn pretty_combo(modifiers: &str, key: &str) -> String {
    let mut parts: Vec<String> = modifiers
        .split('-')
        .filter(|m| !m.is_empty())
        .filter_map(|m| {
            MOD_LETTERS
                .iter()
                .position(|l| *l == m)
                .map(|i| MOD_NAMES[i].to_string())
        })
        .collect();
    parts.push(pretty_key(key));
    parts.join(" + ")
}

/// Human friendly key name: single letters upper-cased, common aliases.
pub fn pretty_key(key: &str) -> String {
    match key {
        "Return" => "Enter".to_string(),
        "space" => "Space".to_string(),
        _ => {
            let mut chars = key.chars();
            if let (Some(c), None) = (chars.next(), chars.next()) {
                c.to_uppercase().to_string()
            } else {
                key.to_string()
            }
        }
    }
}

/// Joins modifier letters and a key into the labwc combo string, e.g. `"W-Tab"`.
pub fn combo_string(modifiers: &str, key: &str) -> String {
    if modifiers.is_empty() {
        key.to_string()
    } else {
        format!("{}-{}", modifiers, key)
    }
}

/// Updates the shortcut button label from a `W-C-A-S` modifier string + key.
pub fn set_combo_button(btn: &Button, modifiers: &str, key: &str) {
    if key.is_empty() {
        btn.set_label(&babydra_core::i18n::trans("settings.keybind_record"));
        btn.set_tooltip_text(None::<&str>);
    } else {
        btn.set_label(&pretty_combo(modifiers, key));
        btn.set_tooltip_text(Some(&combo_string(modifiers, key)));
    }
}

/// Reads held modifiers from the event state into canonical `W-C-A-S` order.
pub fn state_modifiers(state: &gtk4::gdk::ModifierType) -> String {
    let checks = [
        (gtk4::gdk::ModifierType::SUPER_MASK, "W"),
        (gtk4::gdk::ModifierType::CONTROL_MASK, "C"),
        (gtk4::gdk::ModifierType::ALT_MASK, "A"),
        (gtk4::gdk::ModifierType::SHIFT_MASK, "S"),
    ];
    checks
        .iter()
        .filter(|(mask, _)| state.contains(*mask))
        .map(|(_, letter)| letter.to_string())
        .collect::<Vec<_>>()
        .join("-")
}
