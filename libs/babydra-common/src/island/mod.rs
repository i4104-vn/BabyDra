//! Dynamic Island state persistence utilities.
//! Writes active state updates to a temporary toml file to allow other crates to react.

use std::fs;
use std::path::PathBuf;

use crate::models::IslandState;

/// Resolves the file path of the temporary Dynamic Island state file.
pub fn get_island_state_path() -> PathBuf {
    std::env::temp_dir().join("babydra-island.toml")
}

/// Overwrites the temporary state file with the updated state representation.
pub fn update_island_state(state: &IslandState) -> Result<(), std::io::Error> {
    let path = get_island_state_path();
    if let Ok(toml_str) = toml::to_string(state) {
        fs::write(path, toml_str)?;
    }
    Ok(())
}

/// Deletes the temporary state file to clear the state.
pub fn clear_island_state() -> Result<(), std::io::Error> {
    let path = get_island_state_path();
    if path.exists() {
        let _ = fs::remove_file(path);
    }
    Ok(())
}

