use crate::config::model::UpdaterConfig;
use std::fs;
use std::path::{Path, PathBuf};

pub fn load_config(repo_root: &Path) -> (UpdaterConfig, Option<PathBuf>) {
    let candidate_paths = [
        repo_root.join("updater").join("updater.toml"),
        repo_root.join("updater.toml"),
        PathBuf::from("updater.toml"),
        PathBuf::from("updater/updater.toml"),
    ];

    for path in &candidate_paths {
        if path.exists() {
            if let Ok(content) = fs::read_to_string(path) {
                match toml::from_str::<UpdaterConfig>(&content) {
                    Ok(cfg) => return (cfg, Some(path.clone())),
                    Err(e) => {
                        eprintln!("Warning: Failed to parse {}: {}", path.display(), e);
                    }
                }
            }
        }
    }

    (UpdaterConfig::default(), None)
}
