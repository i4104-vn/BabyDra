//! Labwc compositor shortcut swallowing synchronization.

use crate::models::shortcut::Shortcut;
use std::fs;
use std::path::PathBuf;

/// Keeps labwc `rc.xml` in sync by swallowing all shortcut combos with a no-op action,
/// preventing keystroke characters from leaking into focused client windows while
/// letting `babydra-keymap` execute the configured commands.
pub fn sync_labwc_swallow(shortcuts: &[Shortcut]) {
    let home = std::env::var("HOME").unwrap_or_default();
    let rc_path = PathBuf::from(home)
        .join(".config")
        .join("labwc")
        .join("rc.xml");
    let Ok(content) = fs::read_to_string(&rc_path) else {
        return;
    };

    let start_marker = "<!-- BABYDRA_SWALLOW_START -->";
    let end_marker = "<!-- BABYDRA_SWALLOW_END -->";

    let Some(start_pos) = content.find(start_marker) else {
        return;
    };
    let Some(end_pos) = content.find(end_marker) else {
        return;
    };

    let mut replacement = String::from(start_marker);
    replacement.push_str("\n    <!-- Global shortcuts are handled by babydra-keymap; labwc only swallows\n         the combos here so client windows do not receive the keystrokes. -->\n");
    let mut combo_set = std::collections::BTreeSet::new();
    for sc in shortcuts {
        let combo = sc.combo();
        if !combo.is_empty() {
            combo_set.insert(combo);
        }
    }
    // Always swallow fixed non-customizable Island shortcuts like Win + F4
    combo_set.insert("W-F4".to_string());
    if let Some(sc) = crate::get_shortcut() {
        if sc.enabled && !sc.combo().is_empty() {
            combo_set.insert(sc.combo());
        }
    }

    for combo in combo_set {
        replacement.push_str(&format!(
            "    <keybind key=\"{}\">\n      <action name=\"Execute\" command=\"true\" />\n    </keybind>\n",
            combo
        ));
    }
    replacement.push_str("    ");
    replacement.push_str(end_marker);

    let new_content = format!(
        "{}{}{}",
        &content[..start_pos],
        replacement,
        &content[end_pos + end_marker.len()..]
    );

    if fs::write(&rc_path, new_content).is_ok() {
        let _ = std::process::Command::new("labwc")
            .arg("--reconfigure")
            .status();
    }
}
