//! In-memory clipboard history tracking service.
//!
//! Captures clipboard modifications via `wl-paste --watch`, maintaining a
//! ring buffer of recent text and image clips during the current session.

use std::collections::VecDeque;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Instant;

const SPOOL_DIR: &str = "/tmp/babydra-clipboard-spool";

/// Represents a single item stored in clipboard history.
#[derive(Clone, Debug)]
pub enum ClipboardEntry {
    Text {
        content: String,
        timestamp: Instant,
    },
    Image {
        png_bytes: Vec<u8>,
        timestamp: Instant,
    },
}

impl ClipboardEntry {
    pub fn is_text(&self) -> bool {
        matches!(self, Self::Text { .. })
    }

    pub fn is_image(&self) -> bool {
        matches!(self, Self::Image { .. })
    }

    pub fn timestamp(&self) -> Instant {
        match self {
            Self::Text { timestamp, .. } => *timestamp,
            Self::Image { timestamp, .. } => *timestamp,
        }
    }

    pub fn preview_text(&self) -> String {
        match self {
            Self::Text { content, .. } => {
                let first_non_empty = content
                    .lines()
                    .map(|l| l.trim())
                    .find(|l| !l.is_empty())
                    .unwrap_or("");
                if first_non_empty.chars().count() > 42 {
                    let mut s: String = first_non_empty.chars().take(39).collect();
                    s.push_str("...");
                    s
                } else if first_non_empty.is_empty() {
                    "(Empty)".to_string()
                } else {
                    first_non_empty.to_string()
                }
            }
            Self::Image { .. } => "[Image / Hình ảnh]".to_string(),
        }
    }
}

static CLIPBOARD_STORE: OnceLock<Arc<Mutex<VecDeque<ClipboardEntry>>>> = OnceLock::new();
static WATCHER_SPAWNED: AtomicBool = AtomicBool::new(false);

/// Returns the shared global history store.
pub fn get_clipboard_store() -> Arc<Mutex<VecDeque<ClipboardEntry>>> {
    CLIPBOARD_STORE
        .get_or_init(|| Arc::new(Mutex::new(VecDeque::new())))
        .clone()
}

/// Returns a clone of all current entries in the history ring buffer.
pub fn get_entries() -> Vec<ClipboardEntry> {
    let store = get_clipboard_store();
    let lock = store.lock().unwrap();
    lock.iter().cloned().collect()
}

/// Checks whether clipboard history feature is enabled in config.
pub fn is_clipboard_enabled() -> bool {
    crate::config::load_babydra_config().clipboard.enabled
}

/// Pushes an entry into history with deduplication and capacity bounds.
pub fn push_entry(entry: ClipboardEntry) {
    if !is_clipboard_enabled() {
        return;
    }

    let max_items = crate::config::load_babydra_config().clipboard.max_items.max(1);
    let store = get_clipboard_store();
    let mut lock = store.lock().unwrap();

    // Deduplicate against the newest entry
    if let Some(front) = lock.front() {
        match (front, &entry) {
            (
                ClipboardEntry::Text { content: c1, .. },
                ClipboardEntry::Text { content: c2, .. },
            ) => {
                if c1 == c2 {
                    return;
                }
            }
            (
                ClipboardEntry::Image { png_bytes: b1, .. },
                ClipboardEntry::Image { png_bytes: b2, .. },
            ) => {
                if b1 == b2 {
                    return;
                }
            }
            _ => {}
        }
    }

    lock.push_front(entry);
    while lock.len() > max_items {
        lock.pop_back();
    }
}

/// Clears all entries from the history buffer.
pub fn clear_history() {
    let store = get_clipboard_store();
    let mut lock = store.lock().unwrap();
    lock.clear();
}

/// Shell command invoking the D-Bus method to display Dynamic Island clipboard history.
pub const DBUS_TRIGGER_CMD: &str =
    "gdbus call --session --dest org.babydra.Island --object-path /org/babydra/Island --method org.babydra.Island.ShowClipboard >/dev/null 2>&1";

/// Resolves the dedicated clipboard shortcut from `babydra.conf` if enabled.
pub fn get_shortcut() -> Option<crate::models::shortcut::Shortcut> {
    let conf = crate::config::load_babydra_config();
    if conf.clipboard.enabled && !conf.clipboard.shortcut_key.trim().is_empty() {
        Some(crate::models::shortcut::Shortcut {
            id: 0,
            modifiers: conf.clipboard.shortcut_modifiers,
            key: conf.clipboard.shortcut_key,
            command: DBUS_TRIGGER_CMD.to_string(),
        })
    } else {
        None
    }
}

/// Triggers the Dynamic Island clipboard history display via D-Bus IPC.
pub fn trigger_clipboard() {
    let _ = Command::new("sh")
        .args(["-c", DBUS_TRIGGER_CMD])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
}

/// Copies an entry back to the Wayland system clipboard via `wl-copy`.
pub fn copy_to_system_clipboard(entry: &ClipboardEntry) -> Result<(), String> {
    match entry {
        ClipboardEntry::Text { content, .. } => {
            let mut child = Command::new("wl-copy")
                .stdin(Stdio::piped())
                .spawn()
                .map_err(|e| format!("failed to spawn wl-copy: {}", e))?;

            if let Some(mut stdin) = child.stdin.take() {
                stdin
                    .write_all(content.as_bytes())
                    .map_err(|e| format!("failed to write to wl-copy stdin: {}", e))?;
            }

            let status = child
                .wait()
                .map_err(|e| format!("failed to wait for wl-copy: {}", e))?;
            if !status.success() {
                return Err(format!("wl-copy exited with error: {:?}", status.code()));
            }
            Ok(())
        }
        ClipboardEntry::Image { png_bytes, .. } => {
            let mut child = Command::new("wl-copy")
                .arg("-t")
                .arg("image/png")
                .stdin(Stdio::piped())
                .spawn()
                .map_err(|e| format!("failed to spawn wl-copy for image: {}", e))?;

            if let Some(mut stdin) = child.stdin.take() {
                stdin
                    .write_all(png_bytes)
                    .map_err(|e| format!("failed to write image bytes to wl-copy stdin: {}", e))?;
            }

            let status = child
                .wait()
                .map_err(|e| format!("failed to wait for wl-copy: {}", e))?;
            if !status.success() {
                return Err(format!("wl-copy exited with error: {:?}", status.code()));
            }
            Ok(())
        }
    }
}

/// Spawns the background clipboard watcher using `wl-paste --watch` and a local spool directory.
pub fn spawn_clipboard_watcher() {
    if WATCHER_SPAWNED.swap(true, Ordering::SeqCst) {
        return; // Already running
    }

    let spool_path = PathBuf::from(SPOOL_DIR);
    let _ = fs::create_dir_all(&spool_path);

    // Clean up any stale spool files from previous sessions
    if let Ok(entries) = fs::read_dir(&spool_path) {
        for entry in entries.flatten() {
            let _ = fs::remove_file(entry.path());
        }
    }

    // Spawn watcher thread
    std::thread::Builder::new()
        .name("babydra-clipboard-watcher".to_string())
        .spawn(move || {
            let mut text_child: Option<Child> = None;
            let mut img_child: Option<Child> = None;

            loop {
                // Ensure text watcher process is alive
                let need_spawn_text = match text_child.as_mut() {
                    Some(c) => match c.try_wait() {
                        Ok(Some(_)) | Err(_) => true,
                        Ok(None) => false,
                    },
                    None => true,
                };
                if need_spawn_text {
                    let cmd = format!(
                        "DIR=\"{}\"; mkdir -p \"$DIR\"; F=\"$DIR/text_$(date +%s%N)\"; cat > \"$F.tmp\" && mv \"$F.tmp\" \"$F\"",
                        SPOOL_DIR
                    );
                    text_child = Command::new("wl-paste")
                        .args(["-t", "text", "-w", "sh", "-c", &cmd])
                        .stdin(Stdio::null())
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .spawn()
                        .ok();
                }

                // Ensure image watcher process is alive
                let need_spawn_img = match img_child.as_mut() {
                    Some(c) => match c.try_wait() {
                        Ok(Some(_)) | Err(_) => true,
                        Ok(None) => false,
                    },
                    None => true,
                };
                if need_spawn_img {
                    let cmd = format!(
                        "DIR=\"{}\"; mkdir -p \"$DIR\"; F=\"$DIR/img_$(date +%s%N).png\"; cat > \"$F.tmp\" && mv \"$F.tmp\" \"$F\"",
                        SPOOL_DIR
                    );
                    img_child = Command::new("wl-paste")
                        .args(["-t", "image/png", "-w", "sh", "-c", &cmd])
                        .stdin(Stdio::null())
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .spawn()
                        .ok();
                }

                // Poll spool directory for new files
                if let Ok(dir_entries) = fs::read_dir(SPOOL_DIR) {
                    let mut files: Vec<PathBuf> = dir_entries
                        .flatten()
                        .map(|e| e.path())
                        .filter(|p| {
                            p.is_file()
                                && !p.file_name().unwrap_or_default().to_string_lossy().ends_with(".tmp")
                        })
                        .collect();

                    // Sort chronologically by file name
                    files.sort();

                    for file in files {
                        let name = file.file_name().unwrap_or_default().to_string_lossy().to_string();
                        if name.starts_with("text_") {
                            if let Ok(content) = fs::read_to_string(&file) {
                                if !content.trim().is_empty() {
                                    push_entry(ClipboardEntry::Text {
                                        content,
                                        timestamp: Instant::now(),
                                    });
                                }
                            }
                            let _ = fs::remove_file(&file);
                        } else if name.starts_with("img_") {
                            if let Ok(png_bytes) = fs::read(&file) {
                                if !png_bytes.is_empty() {
                                    push_entry(ClipboardEntry::Image {
                                        png_bytes,
                                        timestamp: Instant::now(),
                                    });
                                }
                            }
                            let _ = fs::remove_file(&file);
                        }
                    }
                }

                std::thread::sleep(std::time::Duration::from_millis(150));
            }
        })
        .expect("failed to spawn clipboard watcher thread");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_ring_buffer_and_dedup() {
        clear_history();
        assert_eq!(get_entries().len(), 0);

        push_entry(ClipboardEntry::Text {
            content: "First clip".to_string(),
            timestamp: Instant::now(),
        });
        assert_eq!(get_entries().len(), 1);

        // Deduplication: pushing the same text again should not increase length
        push_entry(ClipboardEntry::Text {
            content: "First clip".to_string(),
            timestamp: Instant::now(),
        });
        assert_eq!(get_entries().len(), 1);

        // Pushing a different text should add it to the front
        push_entry(ClipboardEntry::Text {
            content: "Second clip".to_string(),
            timestamp: Instant::now(),
        });
        assert_eq!(get_entries().len(), 2);
        assert_eq!(get_entries()[0].preview_text(), "Second clip");

        // Pushing an image
        push_entry(ClipboardEntry::Image {
            png_bytes: vec![1, 2, 3, 4],
            timestamp: Instant::now(),
        });
        assert_eq!(get_entries().len(), 3);
        assert!(get_entries()[0].is_image());

        // Deduplication for image
        push_entry(ClipboardEntry::Image {
            png_bytes: vec![1, 2, 3, 4],
            timestamp: Instant::now(),
        });
        assert_eq!(get_entries().len(), 3);
    }
}
