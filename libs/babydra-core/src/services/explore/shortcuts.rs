//! Keyboard shortcut parsing and key matching helpers shared between Explore and Desktop.

use gdk4::{Key, ModifierType};

/// Cleans modifier state to only retain Ctrl, Shift, and Alt masks.
pub fn clean_modifiers(modifiers: ModifierType) -> ModifierType {
    modifiers & (ModifierType::CONTROL_MASK | ModifierType::SHIFT_MASK | ModifierType::ALT_MASK)
}

/// Parses a shortcut string like "Ctrl+Shift+T", "Delete", or "Return" into a `(Key, ModifierType)` pair.
pub fn parse_shortcut(shortcut_str: &str) -> Option<(Key, ModifierType)> {
    let parts: Vec<&str> = shortcut_str.split('+').map(|s| s.trim()).collect();
    let mut modifiers = ModifierType::empty();
    let mut key = None;

    for part in parts {
        let part_lower = part.to_lowercase();
        if part_lower == "ctrl" || part_lower == "control" {
            modifiers |= ModifierType::CONTROL_MASK;
        } else if part_lower == "shift" {
            modifiers |= ModifierType::SHIFT_MASK;
        } else if part_lower == "alt" {
            modifiers |= ModifierType::ALT_MASK;
        } else {
            let k = match part_lower.as_str() {
                "f1" => Some(Key::F1),
                "f2" => Some(Key::F2),
                "f3" => Some(Key::F3),
                "f4" => Some(Key::F4),
                "f5" => Some(Key::F5),
                "f6" => Some(Key::F6),
                "f7" => Some(Key::F7),
                "f8" => Some(Key::F8),
                "f9" => Some(Key::F9),
                "f10" => Some(Key::F10),
                "f11" => Some(Key::F11),
                "f12" => Some(Key::F12),
                "enter" | "return" => Some(Key::Return),
                "space" => Some(Key::space),
                "escape" | "esc" => Some(Key::Escape),
                "delete" | "del" => Some(Key::Delete),
                "backspace" => Some(Key::BackSpace),
                s if s.len() == 1 => {
                    let c = s.chars().next().unwrap();
                    Key::from_name(c.to_string())
                }
                s => Key::from_name(s),
            };
            if let Some(keyval) = k {
                key = Some(keyval);
            }
        }
    }

    key.map(|k| (k, modifiers))
}

/// Checks if an event's `keyval` and cleaned `modifiers` match the given target shortcut.
pub fn matches_key(
    keyval: Key,
    clean_mod: ModifierType,
    target: (Key, ModifierType),
) -> bool {
    let (target_key, target_mod) = target;
    if clean_mod != target_mod {
        return false;
    }
    keyval == target_key
        || keyval.to_lower() == target_key.to_lower()
        || (target_key == Key::Delete && keyval == Key::KP_Delete)
        || (target_key == Key::Return && keyval == Key::KP_Enter)
}

/// Checks if an event's `keyval` and raw modifier `state` match the given target shortcut.
pub fn matches_shortcut(
    keyval: Key,
    state: ModifierType,
    target: (Key, ModifierType),
) -> bool {
    let clean_mod = clean_modifiers(state);
    matches_key(keyval, clean_mod, target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_shortcut_single_keys() {
        let (key, m) = parse_shortcut("Delete").unwrap();
        assert_eq!(key, Key::Delete);
        assert_eq!(m, ModifierType::empty());

        let (key, m) = parse_shortcut("Enter").unwrap();
        assert_eq!(key, Key::Return);
        assert_eq!(m, ModifierType::empty());

        let (key, m) = parse_shortcut("F5").unwrap();
        assert_eq!(key, Key::F5);
        assert_eq!(m, ModifierType::empty());
    }

    #[test]
    fn test_parse_shortcut_with_modifiers() {
        let (key, m) = parse_shortcut("Ctrl+Shift+T").unwrap();
        assert_eq!(key.to_lower(), Key::t);
        assert_eq!(m, ModifierType::CONTROL_MASK | ModifierType::SHIFT_MASK);

        let (key, m) = parse_shortcut("Ctrl+X").unwrap();
        assert_eq!(key.to_lower(), Key::x);
        assert_eq!(m, ModifierType::CONTROL_MASK);

        let (key, m) = parse_shortcut("Shift+Delete").unwrap();
        assert_eq!(key, Key::Delete);
        assert_eq!(m, ModifierType::SHIFT_MASK);
    }

    #[test]
    fn test_matches_key_and_keypad_aliases() {
        let del_target = (Key::Delete, ModifierType::empty());
        assert!(matches_key(Key::Delete, ModifierType::empty(), del_target));
        assert!(matches_key(Key::KP_Delete, ModifierType::empty(), del_target));
        assert!(!matches_key(Key::Delete, ModifierType::SHIFT_MASK, del_target));

        let enter_target = (Key::Return, ModifierType::empty());
        assert!(matches_key(Key::Return, ModifierType::empty(), enter_target));
        assert!(matches_key(Key::KP_Enter, ModifierType::empty(), enter_target));
    }

    #[test]
    fn test_matches_shortcut_cleans_extra_modifiers() {
        let target = (Key::x, ModifierType::CONTROL_MASK);
        let raw_state = ModifierType::CONTROL_MASK | ModifierType::LOCK_MASK;
        assert!(matches_shortcut(Key::x, raw_state, target));
        assert!(matches_shortcut(Key::X, raw_state, target));
    }
}
