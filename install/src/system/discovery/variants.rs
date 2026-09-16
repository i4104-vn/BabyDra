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
                    let directory_name = dir
                        .file_name()
                        .map(|name| name.to_string_lossy().to_string())
                        .unwrap_or_default();
                    let name = table
                        .get("name")
                        .and_then(|v| v.as_str())
                        .filter(|name| !name.is_empty())
                        .unwrap_or(&directory_name)
                        .to_string();
                    let theme = table
                        .get("theme")
                        .and_then(|v| v.as_str())
                        .filter(|theme| !theme.is_empty())
                        .map(str::to_owned)
                        .or_else(|| infer_theme(workspace_root, &name))
                        .unwrap_or_else(|| name.clone());
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
        let theme = infer_theme(workspace_root, "default").unwrap_or_else(|| "default".into());
        items.push(VariantItem {
            name: "default".to_string(),
            theme,
            apps: Vec::new(),
            selected: true,
        });
    }

    items.sort_by(|a, b| a.name.cmp(&b.name));
    let selected_name = items
        .iter()
        .find(|variant| variant.name == "default")
        .map(|variant| variant.name.clone())
        .or_else(|| items.first().map(|variant| variant.name.clone()));
    if let Some(selected_name) = selected_name {
        if let Some(selected) = items
            .iter_mut()
            .find(|variant| variant.name == selected_name)
        {
            selected.selected = true;
        }
    }
    items
}

fn infer_theme(workspace_root: &Path, variant_name: &str) -> Option<String> {
    let themes_dir = workspace_root.join("themes");
    let same_name = themes_dir.join(variant_name);
    if same_name.is_dir() {
        return Some(variant_name.to_owned());
    }
    let prefixed = themes_dir.join(format!("babydra-{variant_name}"));
    if prefixed.is_dir() {
        return Some(format!("babydra-{variant_name}"));
    }
    fs::read_dir(themes_dir)
        .ok()?
        .flatten()
        .map(|entry| entry.path())
        .find(|path| path.is_dir())
        .and_then(|path| {
            path.file_name()
                .map(|name| name.to_string_lossy().to_string())
        })
}
