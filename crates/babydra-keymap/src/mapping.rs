//! Combo parsing and evdev key-name mapping for `babydra-keymap`.
//!
//! Combos follow the labwc syntax stored in `keymap.toml`: modifiers are
//! single letters separated by `-` followed by the key name — `S` = Shift,
//! `C` = Ctrl, `A` = Alt, `W` = Super.  The key name is an evdev key name
//! without the `KEY_` prefix, e.g. `Tab`, `F12`, `Print`.

use babydra_core::models::shortcut::Shortcut;
use evdev::Key;

/// Modifier set required for a shortcut.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Mods {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub win: bool,
}

/// A shortcut resolved into an evdev key + required modifier set.
#[derive(Clone, Debug)]
pub struct ResolvedShortcut {
    pub mods: Mods,
    pub key: Key,
    pub command: String,
}

impl ResolvedShortcut {
    /// Converts a shared `Shortcut` model; `None` when unparseable.
    pub fn resolve(shortcut: &Shortcut) -> Option<Self> {
        let mods = parse_modifiers(&shortcut.modifiers)?;
        let key = key_name_to_evdev(&shortcut.key)?;
        Some(Self {
            mods,
            key,
            command: shortcut.command.clone(),
        })
    }
}

/// Parses a modifier string like `"S"` or `"W-S"`.
fn parse_modifiers(spec: &str) -> Option<Mods> {
    let mut mods = Mods::default();
    for token in spec.split('-') {
        match token.to_ascii_uppercase().as_str() {
            "" => continue,
            "S" => mods.shift = true,
            "C" => mods.ctrl = true,
            "A" => mods.alt = true,
            "W" => mods.win = true,
            _ => return None,
        }
    }
    Some(mods)
}

/// Maps a config key name (e.g. `"Tab"`, `"F12"`, `"Print"`) to an evdev key.
pub fn key_name_to_evdev(name: &str) -> Option<Key> {
    let up = name.to_ascii_uppercase();

    // Single characters → KEY_A..KEY_Z / KEY_0..KEY_9
    let mut chars = up.chars();
    if let (Some(c), None) = (chars.next(), chars.next()) {
        if c.is_ascii_uppercase() {
            return letter_key(c);
        }
        if c.is_ascii_digit() {
            return digit_key(c);
        }
    }

    // F-keys: F1 .. F12
    if let Some(num) = up.strip_prefix('F') {
        if let Ok(n) = num.parse::<u8>() {
            if (1..=12).contains(&n) {
                return Some(f_key(n));
            }
        }
    }

    Some(match up.as_str() {
        "TAB" => Key::KEY_TAB,
        "SPACE" => Key::KEY_SPACE,
        "ENTER" | "RETURN" => Key::KEY_ENTER,
        "ESC" | "ESCAPE" => Key::KEY_ESC,
        "BACKSPACE" | "BKSP" => Key::KEY_BACKSPACE,
        "DELETE" | "DEL" => Key::KEY_DELETE,
        "INSERT" => Key::KEY_INSERT,
        "HOME" => Key::KEY_HOME,
        "END" => Key::KEY_END,
        "PAGEUP" | "PRIOR" => Key::KEY_PAGEUP,
        "PAGEDOWN" | "NEXT" => Key::KEY_PAGEDOWN,
        "UP" => Key::KEY_UP,
        "DOWN" => Key::KEY_DOWN,
        "LEFT" => Key::KEY_LEFT,
        "RIGHT" => Key::KEY_RIGHT,
        "PRINT" | "PRINTSCREEN" | "SYSRQ" => Key::KEY_SYSRQ,
        "PAUSE" => Key::KEY_PAUSE,
        "CAPSLOCK" => Key::KEY_CAPSLOCK,
        "MINUS" => Key::KEY_MINUS,
        "EQUAL" => Key::KEY_EQUAL,
        "COMMA" => Key::KEY_COMMA,
        "DOT" | "PERIOD" => Key::KEY_DOT,
        "SLASH" => Key::KEY_SLASH,
        "SEMICOLON" => Key::KEY_SEMICOLON,
        "APOSTROPHE" => Key::KEY_APOSTROPHE,
        "GRAVE" | "BACKQUOTE" => Key::KEY_GRAVE,
        "BACKSLASH" => Key::KEY_BACKSLASH,
        "LEFTBRACKET" | "BRACKETLEFT" => Key::KEY_LEFTBRACE,
        "RIGHTBRACKET" | "BRACKETRIGHT" => Key::KEY_RIGHTBRACE,
        "MUTE" => Key::KEY_MUTE,
        "VOLUMEUP" => Key::KEY_VOLUMEUP,
        "VOLUMEDOWN" => Key::KEY_VOLUMEDOWN,
        "PLAYPAUSE" => Key::KEY_PLAYPAUSE,
        "PREVIOUSSONG" => Key::KEY_PREVIOUSSONG,
        "NEXTSONG" => Key::KEY_NEXTSONG,
        _ => return None,
    })
}

fn letter_key(c: char) -> Option<Key> {
    Some(match c {
        'A' => Key::KEY_A,
        'B' => Key::KEY_B,
        'C' => Key::KEY_C,
        'D' => Key::KEY_D,
        'E' => Key::KEY_E,
        'F' => Key::KEY_F,
        'G' => Key::KEY_G,
        'H' => Key::KEY_H,
        'I' => Key::KEY_I,
        'J' => Key::KEY_J,
        'K' => Key::KEY_K,
        'L' => Key::KEY_L,
        'M' => Key::KEY_M,
        'N' => Key::KEY_N,
        'O' => Key::KEY_O,
        'P' => Key::KEY_P,
        'Q' => Key::KEY_Q,
        'R' => Key::KEY_R,
        'S' => Key::KEY_S,
        'T' => Key::KEY_T,
        'U' => Key::KEY_U,
        'V' => Key::KEY_V,
        'W' => Key::KEY_W,
        'X' => Key::KEY_X,
        'Y' => Key::KEY_Y,
        'Z' => Key::KEY_Z,
        _ => return None,
    })
}

fn digit_key(c: char) -> Option<Key> {
    Some(match c {
        '0' => Key::KEY_0,
        '1' => Key::KEY_1,
        '2' => Key::KEY_2,
        '3' => Key::KEY_3,
        '4' => Key::KEY_4,
        '5' => Key::KEY_5,
        '6' => Key::KEY_6,
        '7' => Key::KEY_7,
        '8' => Key::KEY_8,
        '9' => Key::KEY_9,
        _ => return None,
    })
}

fn f_key(n: u8) -> Key {
    match n {
        1 => Key::KEY_F1,
        2 => Key::KEY_F2,
        3 => Key::KEY_F3,
        4 => Key::KEY_F4,
        5 => Key::KEY_F5,
        6 => Key::KEY_F6,
        7 => Key::KEY_F7,
        8 => Key::KEY_F8,
        9 => Key::KEY_F9,
        10 => Key::KEY_F10,
        11 => Key::KEY_F11,
        _ => Key::KEY_F12,
    }
}
