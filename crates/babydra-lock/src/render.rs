//! Render orchestrator for the lock screen windows.
//! Takes care of CSS injection for custom wallpapers and window instantiation
//! across all detected system monitors.

use crate::widgets;
use gtk4::prelude::*;
use std::cell::{Cell, RefCell};
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;
use std::process::Command;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

/// Captures one private transition image per monitor.
/// The returned paths are temporary and must be removed after the lock UI exits.
pub fn capture_transition_images() -> Vec<Option<PathBuf>> {
    let Some(display) = gtk4::gdk::Display::default() else {
        return Vec::new();
    };

    let monitors = display.monitors();
    let count = monitors.n_items();
    if count == 0 {
        return vec![capture_transition_image(None, 0)];
    }

    (0..count)
        .map(|index| {
            let monitor = monitors
                .item(index)
                .and_then(|obj| obj.downcast::<gtk4::gdk::Monitor>().ok());
            monitor
                .as_ref()
                .and_then(|monitor| capture_transition_image(monitor.connector().as_deref(), index))
        })
        .collect()
}

/// Instantiates lock screen windows for every connected display monitor.
pub fn build_lock_ui(
    app: &gtk4::Application,
    custom_wallpaper: Option<&str>,
    transition_images: &[Option<PathBuf>],
    transition_stacks: Rc<RefCell<Vec<widgets::LockTransition>>>,
    on_unlock: Rc<dyn Fn()>,
) {
    let display = gtk4::gdk::Display::default().expect("Failed to get default GDK display");
    let monitors = display.monitors();
    let num_monitors = monitors.n_items();

    if num_monitors == 0 {
        widgets::create_lock_window(
            app,
            None,
            true,
            custom_wallpaper,
            transition_images.first().and_then(|image| image.as_deref()),
            transition_stacks,
            on_unlock,
        );
    } else {
        for i in 0..num_monitors {
            if let Some(monitor) = monitors
                .item(i)
                .and_then(|obj| obj.downcast::<gtk4::gdk::Monitor>().ok())
            {
                let is_primary = i == 0;
                widgets::create_lock_window(
                    app,
                    Some(&monitor),
                    is_primary,
                    custom_wallpaper,
                    transition_images
                        .get(i as usize)
                        .and_then(|image| image.as_deref()),
                    transition_stacks.clone(),
                    on_unlock.clone(),
                );
            }
        }
    }
}

/// Reverses the blur on every monitor before the lock application exits.
pub fn reverse_lock_transition(stacks: Vec<widgets::LockTransition>, on_complete: Rc<dyn Fn()>) {
    if stacks.is_empty() {
        on_complete();
        return;
    }

    let remaining = Rc::new(Cell::new(stacks.len()));
    for stack in stacks {
        let remaining = remaining.clone();
        let on_complete = on_complete.clone();
        widgets::reverse_lock_transition(
            stack,
            Rc::new(move || {
                let count = remaining.get().saturating_sub(1);
                remaining.set(count);
                if count == 0 {
                    on_complete();
                }
            }),
        );
    }
}

fn capture_transition_image(output: Option<&str>, index: u32) -> Option<PathBuf> {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();
    let prefix = format!("babydra-lock-{}-{index}-{nonce}", std::process::id());
    let raw_path = std::env::temp_dir().join(format!("{prefix}-raw.png"));

    if let Err(error) = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&raw_path)
    {
        tracing::warn!(
            "Failed to create a private lock transition image: {}",
            error
        );
        return None;
    }

    let mut grim = Command::new("grim");
    if let Some(output) = output {
        grim.args(["-o", output]);
    }
    let captured = grim
        .args(["-t", "png", "-l", "1"])
        .arg(&raw_path)
        .status()
        .map(|status| status.success())
        .unwrap_or(false);

    if !captured {
        tracing::warn!("Failed to capture monitor for the lock transition");
        let _ = std::fs::remove_file(raw_path);
        return None;
    }

    Some(raw_path)
}
