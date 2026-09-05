use std::fs;
use std::path::Path;

use crate::models::VariantItem;

/// Reads `variants/*/variant.toml` and builds selectable variant options.
/// The `default` variant is pre-selected; others start deselected.
pub fn initial_variant_options(workspace_root: &Path) -> Vec<VariantItem> {
    let variants_dir = workspace_root.join("variants");
    let mut items = Vec::new();

    if let Ok(entries) = fs::read_dir(&variants_dir) {
        for entry in entries.flatten() {
            let dir = entry.path();
            let toml_path = dir.join("variant.toml");
            if !toml_path.is_file() {
                continue;
            }
            if let Ok(content) = fs::read_to_string(&toml_path) {
                if let Ok(table) = content.parse::<toml::Table>() {
                    let name = table
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or_default()
                        .to_string();
                    if name.is_empty() {
                        continue;
                    }
                    let theme = table
                        .get("theme")
                        .and_then(|v| v.as_str())
                        .unwrap_or("babydra-default")
                        .to_string();
                    let apps = table
                        .get("apps")
                        .and_then(|v| v.as_array())
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                .collect()
                        })
                        .unwrap_or_default();
                    items.push(VariantItem {
                        name,
                        theme,
                        apps,
                        selected: false,
                    });
                }
            }
        }
    }

    if items.is_empty() {
        items.push(VariantItem {
            name: "default".to_string(),
            theme: "babydra-default".to_string(),
            apps: Vec::new(),
            selected: true,
        });
    }

    items.sort_by(|a, b| a.name.cmp(&b.name));
    if let Some(default) = items.iter_mut().find(|v| v.name == "default") {
        default.selected = true;
    }
    items
}
