mod preview;
mod render;

use babydra_core::DesktopApp;
use babydra_core::{focus_window, get_running_apps};
use gtk4::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct PopoverState {
    popover: gtk4::Popover,
}

/// Returns the current `active app id`.
fn get_active_app_id() -> Option<String> {
    babydra_core::services::window::get_active_window().map(|(app_id, _)| app_id)
}

/// Helper to generate a signature representing current taskbar state (apps only, not active).
fn get_apps_signature(running_apps: &[DesktopApp]) -> String {
    let mut counts = HashMap::new();
    for app in running_apps {
        let app_id = app.app_id.clone().unwrap_or_else(|| app.name.clone());
        *counts.entry(app_id).or_insert(0) += 1;
    }
    let mut sigs: Vec<String> = counts.iter().map(|(k, v)| format!("{}:{}", k, v)).collect();
    sigs.sort();
    sigs.join("||")
}

/// Dynamic rebuild of the taskbar buttons (only called when app list changes)
fn rebuild_taskbar(
    apps_box: &gtk4::Box,
    running_apps: Vec<DesktopApp>,
    active_app_id: Option<String>,
    popovers: &Rc<RefCell<Vec<PopoverState>>>,
    running_apps_shared: Arc<Mutex<Vec<DesktopApp>>>,
) {
    // 1. Unparent all previous popovers first to prevent crashes
    for state in popovers.borrow_mut().drain(..) {
        state.popover.unparent();
    }

    // 2. Clear all existing children from the apps box container
    while let Some(child) = apps_box.first_child() {
        apps_box.remove(&child);
    }

    if running_apps.is_empty() {
        return;
    }

    // 3. Group running apps by app_id
    let mut groups: HashMap<String, Vec<DesktopApp>> = HashMap::new();
    for app in running_apps {
        let app_id = app.app_id.clone().unwrap_or_else(|| app.name.clone());
        groups.entry(app_id).or_default().push(app);
    }

    // 4. Sort groups alphabetically by app_id
    let mut group_keys: Vec<String> = groups.keys().cloned().collect();
    group_keys.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

    // 5. Build buttons for each application group
    for app_id in group_keys {
        let windows = groups.get(&app_id).unwrap();
        let first_app = &windows[0];

        let mut is_active = false;
        if let Some(ref active_id) = active_app_id {
            let active_id_lower = active_id.to_lowercase();
            is_active = app_id.to_lowercase() == active_id_lower
                || windows.iter().any(|w| {
                    w.app_id
                        .as_ref()
                        .map(|id| id.to_lowercase() == active_id_lower)
                        .unwrap_or(false)
                });
        }

        let window_count = windows.len();
        // Delegate button layout rendering
        let btn = render::build_taskbar_btn(first_app, is_active, window_count);

        let popover = render::build_popover_box(&btn);

        // Setup click action
        let pop_clone = popover.clone();
        let app_id_clone = app_id.clone();
        let apps_shared = running_apps_shared.clone();
        btn.connect_clicked(move |_| {
            let windows_list = if let Ok(lock) = apps_shared.lock() {
                lock.clone()
            } else {
                Vec::new()
            };
            let app_windows: Vec<DesktopApp> = windows_list
                .into_iter()
                .filter(|w| {
                    let w_id = w.app_id.as_deref().unwrap_or(&w.name);
                    w_id == app_id_clone
                })
                .collect();

            if app_windows.len() > 1 {
                preview::populate_previews(&pop_clone, &app_windows, &app_id_clone);
                pop_clone.popup();
            } else if let Some(single_window) = app_windows.first() {
                let target_app_id = single_window.app_id.as_deref().unwrap_or(&app_id_clone);
                let target_title = single_window.window_title.as_deref().unwrap_or("");
                focus_window(target_app_id, target_title);
            }
        });

        popovers.borrow_mut().push(PopoverState { popover });
        apps_box.append(&btn);
    }
}

/// Update the active CSS class on existing buttons without a full rebuild.
/// Returns true if the active app changed.
fn update_active_highlight(apps_box: &gtk4::Box, active_app_id: Option<&str>) -> bool {
    let mut changed = false;
    let mut child = apps_box.first_child();
    while let Some(widget) = child {
        if let Some(btn) = widget.downcast_ref::<gtk4::Button>() {
            // Retrieve the stored app_id from the widget name
            let btn_app_id = btn.widget_name().to_string();
            if !btn_app_id.is_empty() {
                let should_be_active = active_app_id
                    .map(|a| a.to_lowercase() == btn_app_id.to_lowercase())
                    .unwrap_or(false);

                let is_active = btn.has_css_class("active");
                if should_be_active && !is_active {
                    btn.add_css_class("active");
                    changed = true;
                } else if !should_be_active && is_active {
                    btn.remove_css_class("active");
                    changed = true;
                }
            }
        }
        child = widget.next_sibling();
    }
    changed
}

/// Creates and returns a taskbar container showing running windows grouped by app class.
pub fn create_workspace_sw() -> gtk4::Box {
    let (parent_box, apps_box) = render::build_workspace_box();

    let popovers = Rc::new(RefCell::new(Vec::new()));
    let last_apps_sig = Rc::new(RefCell::new(String::new()));
    let last_active_id: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));

    // Shared: running apps list (updated slowly, 1s)
    let running_apps_shared: Arc<Mutex<Vec<DesktopApp>>> = Arc::new(Mutex::new(Vec::new()));
    // Shared: active app_id (updated fast, 100ms)
    let active_shared: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));

    // Thread 1: Poll running windows list every 1s (slow — expensive wlrctl call)
    let apps_shared_clone = running_apps_shared.clone();
    thread::spawn(move || loop {
        let apps = get_running_apps();
        if let Ok(mut lock) = apps_shared_clone.lock() {
            *lock = apps;
        }
        thread::sleep(Duration::from_millis(1000));
    });

    // Thread 2: Poll active window every 100ms (fast — lightweight wlrctl call)
    let active_shared_clone = active_shared.clone();
    thread::spawn(move || loop {
        let active = get_active_app_id();
        if let Ok(mut lock) = active_shared_clone.lock() {
            *lock = active;
        }
        thread::sleep(Duration::from_millis(100));
    });

    let apps_box_clone = apps_box.clone();
    let popovers_clone = popovers.clone();
    let sig_clone = last_apps_sig.clone();
    let last_active_clone = last_active_id.clone();
    let apps_for_timer = running_apps_shared.clone();
    let active_for_timer = active_shared.clone();
    let apps_for_rebuild = running_apps_shared.clone();

    // Initial delay load
    glib::timeout_add_local_once(Duration::from_millis(300), {
        let apps_box = apps_box_clone.clone();
        let popovers = popovers_clone.clone();
        let sig = sig_clone.clone();
        let last_active = last_active_clone.clone();
        let apps_shared = running_apps_shared.clone();
        let active_shared = active_shared.clone();
        let apps_rebuild = apps_for_rebuild.clone();
        move || {
            let apps = if let Ok(lock) = apps_shared.lock() {
                lock.clone()
            } else {
                Vec::new()
            };
            let active = if let Ok(lock) = active_shared.lock() {
                lock.clone()
            } else {
                None
            };
            *sig.borrow_mut() = get_apps_signature(&apps);
            *last_active.borrow_mut() = active.clone();
            rebuild_taskbar(&apps_box, apps, active, &popovers, apps_rebuild);
        }
    });

    // GTK timer: 100ms — check active window first (cheap CSS update), then check apps list
    glib::timeout_add_local(Duration::from_millis(100), move || {
        let active = if let Ok(lock) = active_for_timer.lock() {
            lock.clone()
        } else {
            None
        };
        let apps = if let Ok(lock) = apps_for_timer.lock() {
            lock.clone()
        } else {
            Vec::new()
        };

        let new_apps_sig = get_apps_signature(&apps);
        let active_changed = *last_active_clone.borrow() != active;

        if new_apps_sig != *sig_clone.borrow() {
            // Apps list changed → full rebuild
            *sig_clone.borrow_mut() = new_apps_sig;
            *last_active_clone.borrow_mut() = active.clone();
            rebuild_taskbar(
                &apps_box_clone,
                apps,
                active,
                &popovers_clone,
                apps_for_rebuild.clone(),
            );
        } else if active_changed {
            // Only active window changed → fast CSS-only update (no rebuild)
            *last_active_clone.borrow_mut() = active.clone();
            update_active_highlight(&apps_box_clone, active.as_deref());
        }

        glib::ControlFlow::Continue
    });

    parent_box
}
