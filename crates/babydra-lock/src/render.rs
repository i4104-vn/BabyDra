//! Render orchestrator for the lock screen windows.
//! Takes care of CSS injection for custom wallpapers and window instantiation
//! across all detected system monitors.

use gtk4::prelude::*;
use crate::widgets;

/// Instantiates lock screen windows for every connected display monitor.
pub fn build_lock_ui(app: &gtk4::Application, custom_wallpaper: Option<&str>) {
    let display = gtk4::gdk::Display::default().expect("Failed to get default GDK display");
    let monitors = display.monitors();
    let num_monitors = monitors.n_items();

    if num_monitors == 0 {
        widgets::create_lock_window(app, None, true, custom_wallpaper);
    } else {
        for i in 0..num_monitors {
            if let Some(monitor) = monitors.item(i).and_then(|obj| obj.downcast::<gtk4::gdk::Monitor>().ok()) {
                let is_primary = i == 0;
                widgets::create_lock_window(app, Some(&monitor), is_primary, custom_wallpaper);
            }
        }
    }
}

