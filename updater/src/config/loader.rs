use crate::config::model::{UpdaterConfig, UpdaterToml};
use crate::config::workspace::WorkspaceToml;
use std::fs;
use std::path::{Path, PathBuf};

pub fn load_config(repo_root: &Path) -> (UpdaterConfig, Option<PathBuf>) {
    // 1. Load workspace.toml
    let workspace_path = repo_root.join("workspace.toml");
    let ws: WorkspaceToml = if workspace_path.exists() {
        match fs::read_to_string(&workspace_path) {
            Ok(content) => toml::from_str(&content).unwrap_or_else(|e| {
                eprintln!("Warning: Failed to parse workspace.toml: {}", e);
                WorkspaceToml::default()
            }),
            Err(e) => {
                eprintln!("Warning: Failed to read workspace.toml: {}", e);
                WorkspaceToml::default()
            }
        }
    } else {
        WorkspaceToml::default()
    };

    // 2. Load updater.toml
    let candidate_paths = [
        repo_root.join("updater").join("updater.toml"),
        repo_root.join("updater.toml"),
        PathBuf::from("updater.toml"),
        PathBuf::from("updater/updater.toml"),
    ];

    let mut updater_toml = UpdaterToml::default();
    let mut resolved_path = None;

    for path in &candidate_paths {
        if path.exists() {
            if let Ok(content) = fs::read_to_string(path) {
                match toml::from_str::<UpdaterToml>(&content) {
                    Ok(cfg) => {
                        updater_toml = cfg;
                        resolved_path = Some(path.clone());
                        break;
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to parse {}: {}", path.display(), e);
                    }
                }
            }
        }
    }

    let config = UpdaterConfig {
        meta: updater_toml.meta,
        workspace_binaries: ws.binaries,
        shell_daemons: updater_toml.binaries.shell_daemons,
        gsettings: ws.gsettings,
        mime_defaults: updater_toml.mime_defaults,
        pacman_count: ws.packages.pacman.len(),
        aur_count: ws.packages.aur.len(),
    };

    (config, resolved_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config_reads_workspace_toml() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf();
        let (config, path) = load_config(&repo_root);
        assert!(path.is_some());
        assert!(!config.workspace_binaries.is_empty());
        assert!(!config.shell_daemons.is_empty());
        assert!(!config.gsettings.is_empty());
        assert!(config.user_binaries().contains(&"babydra-panel"));
        assert!(config.system_binaries().contains(&"babydra-greeter"));
    }
}
