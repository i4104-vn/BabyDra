use std::process::Command;

/// Retrieves a value from GNOME GSettings, stripping enclosing single and double quotes.
pub fn get(schema: &str, key: &str) -> Option<String> {
    Command::new("gsettings")
        .args(["get", schema, key])
        .output()
        .ok()
        .filter(|out| out.status.success())
        .map(|out| {
            let raw = String::from_utf8_lossy(&out.stdout);
            raw.trim().trim_matches('\'').trim_matches('"').to_string()
        })
        .filter(|s| !s.is_empty())
}

/// Retrieves a boolean value from GNOME GSettings. Falls back to `default` on error.
pub fn get_bool(schema: &str, key: &str, default: bool) -> bool {
    get(schema, key)
        .map(|val| val.eq_ignore_ascii_case("true"))
        .unwrap_or(default)
}

/// Sets a raw value in GNOME GSettings.
pub fn set(schema: &str, key: &str, val: &str) -> bool {
    Command::new("gsettings")
        .args(["set", schema, key, val])
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false)
}

/// Sets a string value with single quotes in GNOME GSettings (required by GSettings schema strings).
pub fn set_string(schema: &str, key: &str, val: &str) -> bool {
    let quoted = format!("'{}'", val.replace('\'', "\\'"));
    set(schema, key, &quoted)
}

/// Sets a boolean value in GNOME GSettings.
pub fn set_bool(schema: &str, key: &str, val: bool) -> bool {
    set(schema, key, if val { "true" } else { "false" })
}
