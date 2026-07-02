//! Switcher most-recently-used (MRU) window history tracker.
//! Stores and reads simple window ordering caches from a temporary file.

use std::io::Write;

/// Retrieves the switcher window focus history list.
pub fn get_history() -> Vec<String> {
    let history_path = "/tmp/babydra-switcher-history.txt";
    if let Ok(content) = std::fs::read_to_string(history_path) {
        content.lines().map(|s| s.to_string()).collect()
    } else {
        Vec::new()
    }
}

/// Prepends the recently activated window to the MRU history list and saves it back to the temporary file.
pub fn save_history(active_name: &str) {
    let history_path = "/tmp/babydra-switcher-history.txt";
    let mut history = get_history();
    
    history.retain(|x| x != active_name);
    history.insert(0, active_name.to_string());
    history.truncate(20);
    
    if let Ok(mut file) = std::fs::File::create(history_path) {
        for name in history {
            let _ = writeln!(file, "{}", name);
        }
    }
}

