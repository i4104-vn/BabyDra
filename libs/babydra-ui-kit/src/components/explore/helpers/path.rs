pub use babydra_core::services::explore::sanitize_path;
use std::path::PathBuf;

/// Parses command line arguments to determine target directory, decoding URI if necessary.
pub fn parse_target_dir() -> (PathBuf, Option<PathBuf>) {
    let mut target_dir = glib::home_dir();
    let mut focus_item = None;

    let args: Vec<String> = std::env::args().collect();
    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];
        if arg == "-s" || arg == "--select" || arg == "--show-items" || arg == "--show-item" {
            i += 1;
            continue;
        }
        if arg.starts_with('-') {
            i += 1;
            continue;
        }

        let (dir, focus) = babydra_core::services::explore::resolve_target_from_uri(arg);
        target_dir = dir;
        focus_item = focus;
        break;
    }

    (target_dir, focus_item)
}

