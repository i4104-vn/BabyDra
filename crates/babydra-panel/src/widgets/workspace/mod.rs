mod preview;
mod render;

use babydra_core::DesktopApp;
use babydra_core::{
    filter_apps_for_workspace, focus_window, get_app_resource_usage, get_current_workspace,
    get_running_apps,
};
use babydra_ui_kit::components::popovers::{TooltipPopover, TooltipRow};
use gtk4::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct PopoverState {
    preview_popover: gtk4::Popover,
    tooltip_popover: gtk4::Popover,
    update_tooltip: Rc<dyn Fn()>,
}

/// Returns the current `active app id`.
fn get_active_app_id() -> Option<String> {
    babydra_core::services::window::get_active_window().map(|(app_id, _)| app_id)
}

/// Helper to generate a signature representing current taskbar state (apps only, not active).
fn get_apps_signature(ws_id: u32, running_apps: &[DesktopApp]) -> String {
    let mut counts = HashMap::new();
    for app in running_apps {
        let app_id = app.app_id.clone().unwrap_or_else(|| app.name.clone());
        *counts.entry(app_id).or_insert(0) += 1;
    }
    let mut sigs: Vec<String> = counts.iter().map(|(k, v)| format!("{}:{}", k, v)).collect();
    sigs.sort();
    format!("ws:{}||{}", ws_id, sigs.join("||"))
}

fn get_windows_for_app(
    apps_shared: &Arc<Mutex<Vec<DesktopApp>>>,
    app_id: &str,
) -> Vec<DesktopApp> {
    let windows_list = apps_shared.lock().map(|l| l.clone()).unwrap_or_default();
    windows_list
        .into_iter()
        .filter(|w| {
            let w_id = w.app_id.as_deref().unwrap_or(&w.name);
            w_id == app_id
        })
        .collect()
}

fn show_preview_menu(
    popover: &gtk4::Popover,
    popovers: &Rc<RefCell<Vec<PopoverState>>>,
    windows: &[DesktopApp],
    app_id: &str,
) {
    for s in popovers.borrow().iter() {
        if s.tooltip_popover.is_visible() {
            s.tooltip_popover.popdown();
        }
    }
    preview::populate_previews(popover, windows, app_id);
    popover.popup();
}

fn rebuild_taskbar(
    apps_box: &gtk4::Box,
    running_apps: Vec<DesktopApp>,
    active_app_id: Option<String>,
    popovers: &Rc<RefCell<Vec<PopoverState>>>,
    running_apps_shared: Arc<Mutex<Vec<DesktopApp>>>,
) {
    for state in popovers.borrow_mut().drain(..) {
        state.preview_popover.unparent();
        state.tooltip_popover.unparent();
    }

    while let Some(child) = apps_box.first_child() {
        apps_box.remove(&child);
    }

    if running_apps.is_empty() {
        return;
    }

    let mut groups: HashMap<String, Vec<DesktopApp>> = HashMap::new();
    for app in running_apps {
        let app_id = app.app_id.clone().unwrap_or_else(|| app.name.clone());
        groups.entry(app_id).or_default().push(app);
    }

    let mut group_keys: Vec<String> = groups.keys().cloned().collect();
    group_keys.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

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
        let btn = render::build_taskbar_btn(first_app, is_active, window_count);
        let preview_popover = render::build_popover_box(&btn);

        let tooltip_popover = TooltipPopover::new(&btn, gtk4::PositionType::Bottom);
        let popovers_sup = popovers.clone();
        tooltip_popover.set_suppress_fn(move || {
            popovers_sup
                .borrow()
                .iter()
                .any(|state| state.preview_popover.is_visible())
        });

        let app_name = first_app.name.clone();
        let app_exec = first_app.exec.clone();
        let app_id_str = app_id.clone();
        let tt_pop = tooltip_popover.clone();
        let update_fn: Rc<dyn Fn()> = Rc::new(move || {
            let usage = get_app_resource_usage(&app_id_str, &app_exec, &app_name);
            let state_str = if usage.is_running {
                babydra_core::i18n::trans("taskbar.state_running")
            } else {
                babydra_core::i18n::trans("taskbar.state_sleeping")
            };
            let rows = [
                TooltipRow::new(&babydra_core::i18n::trans("taskbar.cpu"), &usage.cpu_formatted, None),
                TooltipRow::new(&babydra_core::i18n::trans("taskbar.ram"), &usage.ram_formatted, None),
                TooltipRow::new(
                    &babydra_core::i18n::trans("taskbar.state"),
                    &state_str,
                    if usage.is_running { Some("text-success") } else { None },
                ),
            ];
            let card = TooltipPopover::build_card(&app_name, &rows);
            tt_pop.set_child(Some(&card));
        });
        tooltip_popover.attach_hover(&btn, Some(update_fn.clone()));

        let pop_left = preview_popover.clone();
        let app_id_left = app_id.clone();
        let apps_left = running_apps_shared.clone();
        let popovers_left = popovers.clone();
        btn.connect_clicked(move |_| {
            let app_windows = get_windows_for_app(&apps_left, &app_id_left);
            if app_windows.len() > 1 {
                show_preview_menu(&pop_left, &popovers_left, &app_windows, &app_id_left);
            } else if let Some(single_window) = app_windows.first() {
                let target_app_id = single_window.app_id.as_deref().unwrap_or(&app_id_left);
                let target_title = single_window.window_title.as_deref().unwrap_or("");
                focus_window(target_app_id, target_title);
            }
        });

        let right_click = gtk4::GestureClick::new();
        right_click.set_button(3);
        let pop_right = preview_popover.clone();
        let app_id_right = app_id.clone();
        let apps_right = running_apps_shared.clone();
        let popovers_right = popovers.clone();
        right_click.connect_pressed(move |_, _, _, _| {
            let app_windows = get_windows_for_app(&apps_right, &app_id_right);
            if !app_windows.is_empty() {
                show_preview_menu(&pop_right, &popovers_right, &app_windows, &app_id_right);
            }
        });
        btn.add_controller(right_click);

        popovers.borrow_mut().push(PopoverState {
            preview_popover,
            tooltip_popover: tooltip_popover.popover,
            update_tooltip: update_fn,
        });
        apps_box.append(&btn);
    }
}

fn update_active_highlight(apps_box: &gtk4::Box, active_app_id: Option<&str>) -> bool {
    let mut changed = false;
    let mut child = apps_box.first_child();
    while let Some(widget) = child {
        if let Some(btn) = widget.downcast_ref::<gtk4::Button>() {
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

pub fn create_workspace_sw() -> gtk4::Box {
    let (parent_box, apps_box) = render::build_workspace_box();

    let popovers = Rc::new(RefCell::new(Vec::new()));
    let last_apps_sig = Rc::new(RefCell::new(String::new()));
    let last_active_id: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
    let last_ws = Rc::new(RefCell::new(0u32));

    let initial_apps = get_running_apps();
    let running_apps_shared: Arc<Mutex<Vec<DesktopApp>>> =
        Arc::new(Mutex::new(initial_apps));
    let active_shared: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));

    let apps_shared_clone = running_apps_shared.clone();
    thread::spawn(move || loop {
        let apps = get_running_apps();
        if let Ok(mut lock) = apps_shared_clone.lock() {
            *lock = apps;
        }
        thread::sleep(Duration::from_millis(300));
    });

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

    let last_ws_init = last_ws.clone();
    glib::timeout_add_local_once(Duration::from_millis(300), {
        let apps_box = apps_box_clone.clone();
        let popovers = popovers_clone.clone();
        let sig = sig_clone.clone();
        let last_active = last_active_clone.clone();
        let apps_shared = running_apps_shared.clone();
        let active_shared = active_shared.clone();
        let apps_rebuild = apps_for_rebuild.clone();
        move || {
            let all_apps = if let Ok(lock) = apps_shared.lock() {
                lock.clone()
            } else {
                Vec::new()
            };
            let active = if let Ok(lock) = active_shared.lock() {
                lock.clone()
            } else {
                None
            };
            let current_ws = get_current_workspace();
            *last_ws_init.borrow_mut() = current_ws;
            let ws_apps = filter_apps_for_workspace(current_ws, &all_apps, current_ws);
            *sig.borrow_mut() = get_apps_signature(current_ws, &ws_apps);
            *last_active.borrow_mut() = active.clone();
            rebuild_taskbar(&apps_box, ws_apps, active, &popovers, apps_rebuild);
        }
    });

    let last_ws_timer = last_ws.clone();
    glib::timeout_add_local(Duration::from_millis(100), move || {
        let active = if let Ok(lock) = active_for_timer.lock() {
            lock.clone()
        } else {
            None
        };
        let all_apps = if let Ok(lock) = apps_for_timer.lock() {
            lock.clone()
        } else {
            Vec::new()
        };

        let current_ws = get_current_workspace();
        let ws_changed = *last_ws_timer.borrow() != current_ws;
        let ws_apps = filter_apps_for_workspace(current_ws, &all_apps, current_ws);

        let new_apps_sig = get_apps_signature(current_ws, &ws_apps);
        let active_changed = *last_active_clone.borrow() != active;

        if ws_changed || new_apps_sig != *sig_clone.borrow() {
            *last_ws_timer.borrow_mut() = current_ws;
            *sig_clone.borrow_mut() = new_apps_sig;
            *last_active_clone.borrow_mut() = active.clone();
            rebuild_taskbar(
                &apps_box_clone,
                ws_apps,
                active,
                &popovers_clone,
                apps_for_rebuild.clone(),
            );
        } else if active_changed {
            *last_active_clone.borrow_mut() = active.clone();
            update_active_highlight(&apps_box_clone, active.as_deref());
        }

        glib::ControlFlow::Continue
    });

    let popovers_timer = popovers.clone();
    glib::timeout_add_local(Duration::from_secs(5), move || {
        for state in popovers_timer.borrow().iter() {
            if state.tooltip_popover.is_visible() {
                (state.update_tooltip)();
            }
        }
        glib::ControlFlow::Continue
    });

    parent_box
}
