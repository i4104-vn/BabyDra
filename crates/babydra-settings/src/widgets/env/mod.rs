pub mod render;

use babydra_core::models::env_var::EnvVar;
use gtk4::prelude::*;
use gtk4::{Box, Entry, Widget};
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

/// Returns the current `labwc env path`.
fn get_labwc_env_path() -> PathBuf {
    glib::home_dir()
        .join(".config")
        .join("labwc")
        .join("environment")
}

/// Load labwc env vars.
fn load_labwc_env_vars() -> Vec<EnvVar> {
    let path = get_labwc_env_path();
    let mut result = Vec::new();
    if let Ok(file) = fs::File::open(&path) {
        let reader = BufReader::new(file);
        let mut id = 1;
        for line in reader.lines().flatten() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            let clean_line = trimmed.strip_prefix("export ").unwrap_or(trimmed).trim();
            if let Some((k, v)) = clean_line.split_once('=') {
                let key = k.trim().to_string();
                let value = v.trim().to_string();
                if !key.is_empty() {
                    result.push(EnvVar { id, key, value });
                    id += 1;
                }
            }
        }
    }
    result
}

/// Save labwc env vars.
fn save_labwc_env_vars(vars: &[EnvVar]) -> std::io::Result<()> {
    let path = get_labwc_env_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = fs::File::create(&path)?;
    writeln!(
        file,
        "# Labwc Environment Variables managed by babydra-settings"
    )?;
    for v in vars {
        let k = v.key.trim();
        if !k.is_empty() {
            writeln!(file, "{}={}", k, v.value.trim())?;
        }
    }
    Ok(())
}

/// Creates a new `env widget`.
pub fn create_env_widget() -> Widget {
    // Read environment variables directly from ~/.config/labwc/environment
    let vars = load_labwc_env_vars();

    let widget = render::build(&vars);

    let parent_card = widget.list_box.clone();
    widget.add_btn.connect_clicked(move |_| {
        let empty_var = EnvVar {
            id: 0,
            key: "".to_string(),
            value: "".to_string(),
        };
        let row = render::create_env_row(&empty_var, parent_card.clone());
        parent_card.append(&row);
    });

    let parent_card_save = widget.list_box.clone();
    widget.save_btn.connect_clicked(move |_| {
        let mut save_list = Vec::new();
        let mut id = 1;
        let mut child = parent_card_save.first_child();
        while let Some(c) = child {
            if let Some(row_box) = c.downcast_ref::<Box>() {
                let mut key = String::new();
                let mut val = String::new();
                let mut count = 0;
                let mut sub_child = row_box.first_child();
                while let Some(sc) = sub_child {
                    if let Some(entry) = sc.downcast_ref::<Entry>() {
                        if count == 0 {
                            key = entry.text().to_string();
                            count += 1;
                        } else {
                            val = entry.text().to_string();
                        }
                    }
                    sub_child = sc.next_sibling();
                }
                if !key.trim().is_empty() {
                    save_list.push(EnvVar {
                        id,
                        key: key.clone(),
                        value: val.clone(),
                    });
                    id += 1;
                }
            }
            child = c.next_sibling();
        }
        let _ = save_labwc_env_vars(&save_list);
    });

    widget.container.into()
}
