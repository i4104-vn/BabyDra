//! Labwc compositor shortcut swallowing synchronization.

use crate::models::shortcut::Shortcut;
use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

pub const START_MARKER: &str = "<!-- BABYDRA_SWALLOW_START -->";
pub const END_MARKER: &str = "<!-- BABYDRA_SWALLOW_END -->";

/// Generates the XML content for the swallow block given a set of shortcuts.
pub fn generate_swallow_block(shortcuts: &[Shortcut]) -> String {
    let mut combo_set = BTreeSet::new();

    // Critical window switching combos must always be swallowed so labwc default OSD never fires
    combo_set.insert("A-Tab".to_string());
    combo_set.insert("A-S-Tab".to_string());

    // Fixed non-customizable Island and screenshot shortcuts
    combo_set.insert("W-F4".to_string());
    combo_set.insert("W-F12".to_string());
    combo_set.insert("Print".to_string());

    for sc in shortcuts {
        let combo = sc.combo();
        if !combo.is_empty() {
            combo_set.insert(combo);
        }
    }

    if let Some(sc) = crate::get_shortcut() {
        if sc.enabled && !sc.combo().is_empty() {
            combo_set.insert(sc.combo());
        }
    }

    let mut block = String::from(START_MARKER);
    block.push_str("\n    <!-- Global shortcuts are handled by babydra-keymap; labwc only swallows\n         the combos here so default compositor actions do not fire and client\n         windows do not receive the keystrokes. -->\n");

    for combo in combo_set {
        block.push_str(&format!(
            "    <keybind key=\"{}\">\n      <action name=\"Execute\" command=\"true\" />\n    </keybind>\n",
            combo
        ));
    }
    block.push_str("    ");
    block.push_str(END_MARKER);
    block
}

/// Updates the labwc `rc.xml` content with the generated swallow block.
pub fn update_rc_content(content: &str, shortcuts: &[Shortcut]) -> Option<String> {
    let replacement = generate_swallow_block(shortcuts);

    if let (Some(start_pos), Some(end_pos)) = (content.find(START_MARKER), content.find(END_MARKER)) {
        if start_pos <= end_pos {
            return Some(format!(
                "{}{}{}",
                &content[..start_pos],
                replacement,
                &content[end_pos + END_MARKER.len()..]
            ));
        }
    }

    // Fallback: inject after <default /> or <keyboard> if markers are missing
    if let Some(kbd_pos) = content.find("<keyboard>") {
        let after_kbd = &content[kbd_pos..];
        let insert_offset = if let Some(def_pos) = after_kbd.find("<default />") {
            def_pos + "<default />".len()
        } else {
            "<keyboard>".len()
        };
        let insert_idx = kbd_pos + insert_offset;
        return Some(format!(
            "{}\n    {}\n{}",
            &content[..insert_idx],
            replacement,
            &content[insert_idx..]
        ));
    }

    None
}

/// Keeps labwc `rc.xml` in sync by swallowing all shortcut combos with a no-op action,
/// preventing keystroke characters from leaking into focused client windows and suppressing
/// labwc's default built-in window switcher OSD (`A-Tab` / `A-S-Tab`).
pub fn sync_labwc_swallow(shortcuts: &[Shortcut]) {
    let home = std::env::var("HOME").unwrap_or_default();
    let rc_path = PathBuf::from(home)
        .join(".config")
        .join("labwc")
        .join("rc.xml");
    let Ok(content) = fs::read_to_string(&rc_path) else {
        return;
    };

    if let Some(new_content) = update_rc_content(&content, shortcuts) {
        if fs::write(&rc_path, new_content).is_ok() {
            let _ = std::process::Command::new("labwc")
                .arg("--reconfigure")
                .status();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_swallow_block_contains_alt_tab() {
        let block = generate_swallow_block(&[]);
        assert!(block.contains(START_MARKER));
        assert!(block.contains(END_MARKER));
        assert!(block.contains("<keybind key=\"A-Tab\">"));
        assert!(block.contains("<keybind key=\"A-S-Tab\">"));
        assert!(block.contains("<action name=\"Execute\" command=\"true\" />"));
    }

    #[test]
    fn test_update_rc_content_replaces_existing_block() {
        let original = r#"<labwc_config>
  <keyboard>
    <default />
    <!-- BABYDRA_SWALLOW_START -->
    <keybind key="Old-Key">
      <action name="Execute" command="true" />
    </keybind>
    <!-- BABYDRA_SWALLOW_END -->
  </keyboard>
</labwc_config>"#;

        let sc = vec![Shortcut {
            id: 1,
            modifiers: "W".to_string(),
            key: "q".to_string(),
            command: "test".to_string(),
            enabled: true,
        }];

        let updated = update_rc_content(original, &sc).unwrap();
        assert!(!updated.contains("Old-Key"));
        assert!(updated.contains("<keybind key=\"A-Tab\">"));
        assert!(updated.contains("<keybind key=\"W-q\">"));
    }

    #[test]
    fn test_update_rc_content_injects_when_missing_markers() {
        let original = r#"<labwc_config>
  <keyboard>
    <default />
  </keyboard>
</labwc_config>"#;

        let updated = update_rc_content(original, &[]).unwrap();
        assert!(updated.contains(START_MARKER));
        assert!(updated.contains("<keybind key=\"A-Tab\">"));
    }
}
