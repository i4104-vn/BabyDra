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

        if let Some(val) = arg.strip_prefix("-p=").or_else(|| arg.strip_prefix("--path=")) {
            let (dir, focus) = babydra_core::services::explore::resolve_target_from_uri(val);
            target_dir = dir;
            focus_item = focus;
            break;
        }

        if let Some(val) = arg.strip_prefix("-s=").or_else(|| arg.strip_prefix("--select=")) {
            let (dir, focus) = babydra_core::services::explore::resolve_target_from_uri(val);
            target_dir = dir;
            focus_item = focus;
            break;
        }

        if arg == "-p" || arg == "--path" || arg == "-s" || arg == "--select" || arg == "--show-items" || arg == "--show-item" {
            if i + 1 < args.len() {
                let next_arg = &args[i + 1];
                let (dir, focus) = babydra_core::services::explore::resolve_target_from_uri(next_arg);
                target_dir = dir;
                focus_item = focus;
            }
            break;
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
