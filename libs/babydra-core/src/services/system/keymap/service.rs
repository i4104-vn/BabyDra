//! Keymap query and mutation services.

use super::catalog::SYSTEM_CRATE_CATALOG;
use super::config::{get_config_path, ConfigFile};
use crate::error::CoreResult;
use crate::models::shortcut::{Shortcut, SystemShortcut};
use std::collections::HashMap;
use std::fs;

/// Retrieves the list of all BabyDra crate system shortcuts.
pub fn get_system_shortcuts() -> Vec<SystemShortcut> {
    let path = get_config_path();
    let file = if path.exists() {
        fs::read_to_string(&path)
            .ok()
            .and_then(|raw| toml::from_str::<ConfigFile>(&raw).ok())
            .unwrap_or_default()
    } else {
        ConfigFile::default()
    };

    let mut result = Vec::new();

    for def in SYSTEM_CRATE_CATALOG {
        if let Some(entry) = file.system_shortcuts.get(def.id) {
            result.push(SystemShortcut {
                id: def.id.to_string(),
                crate_name: def.crate_name.to_string(),
                name_key: def.name_key.to_string(),
                description_key: def.description_key.to_string(),
                command: def.command.to_string(),
                default_modifiers: def.default_modifiers.to_string(),
                default_key: def.default_key.to_string(),
                modifiers: entry.modifiers.clone(),
                key: entry.key.clone(),
                enabled: entry.enabled,
            });
            continue;
        }

        // Check if existing [shortcuts] map has a matching command
        let mut matched = None;
        for (combo, cmd) in file.shortcuts.iter().chain(file.custom_shortcuts.iter()) {
            if cmd == def.command || (!cmd.is_empty() && def.command.ends_with(cmd)) {
                let mut parts = combo.split('-');
                let key = parts.next_back().unwrap_or_default().to_string();
                let modifiers = parts.collect::<Vec<_>>().join("-");
                matched = Some((modifiers, key));
                break;
            }
        }

        if let Some((mods, key)) = matched {
            result.push(SystemShortcut {
                id: def.id.to_string(),
                crate_name: def.crate_name.to_string(),
                name_key: def.name_key.to_string(),
                description_key: def.description_key.to_string(),
                command: def.command.to_string(),
                default_modifiers: def.default_modifiers.to_string(),
                default_key: def.default_key.to_string(),
                modifiers: mods,
                key,
                enabled: true,
            });
        } else {
            result.push(SystemShortcut {
                id: def.id.to_string(),
                crate_name: def.crate_name.to_string(),
                name_key: def.name_key.to_string(),
                description_key: def.description_key.to_string(),
                command: def.command.to_string(),
                default_modifiers: def.default_modifiers.to_string(),
                default_key: def.default_key.to_string(),
                modifiers: def.default_modifiers.to_string(),
                key: def.default_key.to_string(),
                enabled: true,
            });
        }
    }

    result
}

/// Retrieves user-defined custom shortcuts (non-crate commands).
pub fn get_custom_shortcuts() -> Vec<Shortcut> {
    let path = get_config_path();
    let file = if path.exists() {
        fs::read_to_string(&path)
            .ok()
            .and_then(|raw| toml::from_str::<ConfigFile>(&raw).ok())
            .unwrap_or_default()
    } else {
        return Vec::new();
    };

    let system_cmds: std::collections::HashSet<&str> =
        SYSTEM_CRATE_CATALOG.iter().map(|d| d.command).collect();

    let mut custom_map: HashMap<String, String> = HashMap::new();

    for (combo, cmd) in file.custom_shortcuts {
        custom_map.insert(combo, cmd);
    }

    for (combo, cmd) in file.shortcuts {
        let is_system = system_cmds
            .iter()
            .any(|&sc| sc == cmd || (!cmd.is_empty() && sc.ends_with(&cmd)));
        if !is_system && !custom_map.contains_key(&combo) {
            custom_map.insert(combo, cmd);
        }
    }

    let mut shortcuts: Vec<Shortcut> = custom_map
        .into_iter()
        .enumerate()
        .map(|(i, (combo, command))| {
            let mut parts = combo.split('-');
            let key = parts.next_back().unwrap_or_default().to_string();
            let modifiers = parts.collect::<Vec<_>>().join("-");
            Shortcut {
                id: i + 1,
                modifiers,
                key,
                command,
                enabled: true,
            }
        })
        .collect();

    shortcuts.sort_by_key(|a| a.combo());
    shortcuts
}

/// Retrieves the global active shortcut list for the daemon.
pub fn get_shortcuts() -> Vec<Shortcut> {
    let system = get_system_shortcuts();
    let custom = get_custom_shortcuts();

    let mut all = Vec::new();
    let mut id = 1;

    for s in system {
        if s.enabled && !s.key.is_empty() && !s.command.trim().is_empty() {
            all.push(Shortcut {
                id,
                modifiers: s.modifiers,
                key: s.key,
                command: s.command,
                enabled: true,
            });
            id += 1;
        }
    }

    for c in custom {
        if c.enabled && !c.key.is_empty() && !c.command.trim().is_empty() {
            all.push(Shortcut {
                id,
                modifiers: c.modifiers,
                key: c.key,
                command: c.command,
                enabled: true,
            });
            id += 1;
        }
    }

    all
}

/// Saves both system crate shortcuts and custom shortcuts to `keymap.toml`.
pub fn save_keymap_configuration(
    system_shortcuts: &[SystemShortcut],
    custom_shortcuts: &[Shortcut],
) -> CoreResult<()> {
    let path = get_config_path();

    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let mut content = String::from(
        "# babydra-keymap — global shortcut configuration\n\
         # Combo format: modifiers then key, separated by '-'.\n\
         #   Modifiers: S = Shift, C = Ctrl, A = Alt, W = Super\n\
         # Changes are hot-reloaded by babydra-keymap.\n\n",
    );

    content.push_str("# BabyDra Crate System Keymaps\n");
    for s in system_shortcuts {
        content.push_str(&format!("[system_shortcuts.{}]\n", s.id));
        content.push_str(&format!("crate = \"{}\"\n", s.crate_name));
        content.push_str(&format!("command = \"{}\"\n", s.command));
        content.push_str(&format!("modifiers = \"{}\"\n", s.modifiers));
        content.push_str(&format!("key = \"{}\"\n", s.key));
        content.push_str(&format!("enabled = {}\n\n", s.enabled));
    }

    content.push_str("# Custom User Shortcuts\n[custom_shortcuts]\n");
    for c in custom_shortcuts {
        if !c.combo().is_empty() && !c.command.trim().is_empty() {
            content.push_str(&format!("\"{}\" = \"{}\"\n", c.combo(), c.command));
        }
    }

    content.push_str("\n# Active shortcuts map for daemon\n[shortcuts]\n");
    for s in system_shortcuts {
        if s.enabled && !s.key.is_empty() && !s.command.trim().is_empty() {
            content.push_str(&format!("\"{}\" = \"{}\"\n", s.combo(), s.command));
        }
    }
    for c in custom_shortcuts {
        if c.enabled && !c.key.is_empty() && !c.command.trim().is_empty() {
            content.push_str(&format!("\"{}\" = \"{}\"\n", c.combo(), c.command));
        }
    }

    fs::write(&path, content)?;
    Ok(())
}

/// Saves the shortcut list to `keymap.toml` maintaining backward compatibility.
pub fn save_shortcuts(shortcuts: &[Shortcut]) -> CoreResult<()> {
    let system = get_system_shortcuts();
    save_keymap_configuration(&system, shortcuts)
}
