mod popover;
mod preview;
mod render;
pub mod state;

pub use popover::build_workspace_popover;
pub use state::*;

use babydra_core::{toggle_app_window_async, DesktopApp};
use babydra_ui_kit::components::popovers::{TooltipPopover, TooltipRow};
use gtk4::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn build_resource_card(
    app_name: &str,
    usage: Option<&babydra_core::AppResourceUsage>,
) -> gtk4::Box {
    let (cpu, ram, is_running) = usage
        .map(|value| {
            (
                value.cpu_formatted.as_str(),
                value.ram_formatted.as_str(),
                value.is_running,
            )
        })
        .unwrap_or(("--%", "-- MB", true));
    let state = if is_running {
        babydra_core::i18n::trans("taskbar.state_running")
    } else {
        babydra_core::i18n::trans("taskbar.state_sleeping")
    };
    let rows = [
        TooltipRow::new(&babydra_core::i18n::trans("taskbar.cpu"), cpu, None),
        TooltipRow::new(&babydra_core::i18n::trans("taskbar.ram"), ram, None),
        TooltipRow::new(
            &babydra_core::i18n::trans("taskbar.state"),
            &state,
            is_running.then_some("text-success"),
        ),
    ];

    TooltipPopover::build_card(app_name, &rows)
}

fn get_windows_for_app(apps_shared: &Arc<Mutex<Vec<DesktopApp>>>, app_id: &str) -> Vec<DesktopApp> {
    let windows_list = apps_shared.lock().map(|l| l.clone()).unwrap_or_default();
    let app_id_clean = app_id.strip_suffix(".desktop").unwrap_or(app_id);
    windows_list
        .into_iter()
        .filter(|w| {
            let w_id = w.app_id.as_deref().unwrap_or(&w.name);
            let w_id_clean = w_id.strip_suffix(".desktop").unwrap_or(w_id);
            w_id.eq_ignore_ascii_case(app_id) || w_id_clean.eq_ignore_ascii_case(app_id_clean)
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
    // 1. Safely popdown and unparent tracked popovers
    for state in popovers.borrow_mut().drain(..) {
        if state.preview_popover.is_visible() {
            state.preview_popover.popdown();
        }
        if state.tooltip_popover.is_visible() {
            state.tooltip_popover.popdown();
        }
        if state.preview_popover.parent().is_some() {
            state.preview_popover.unparent();
        }
        if state.tooltip_popover.parent().is_some() {
            state.tooltip_popover.unparent();
        }
    }

    // 2. Remove all child buttons and safely unparent any internal popovers
    while let Some(child) = apps_box.first_child() {
        let mut sub = child.first_child();
        while let Some(c) = sub {
            let next = c.next_sibling();
            if let Some(pop) = c.downcast_ref::<gtk4::Popover>() {
                if pop.is_visible() {
                    pop.popdown();
                }
                if pop.parent().is_some() {
                    pop.unparent();
                }
            }
            sub = next;
        }
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
    group_keys.sort_by_key(|a| a.to_lowercase());

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
        let app_name_clone = app_name.clone();
        let cached_usage: Rc<RefCell<Option<babydra_core::AppResourceUsage>>> =
            Rc::new(RefCell::new(None));

        let update_fn: Rc<dyn Fn()> = Rc::new(move || {
            let cached_opt = cached_usage.borrow().clone();
            let card = build_resource_card(&app_name_clone, cached_opt.as_ref());
            tt_pop.set_child(Some(&card));

            // Fetch fresh CPU & RAM in background thread to avoid blocking the UI.
            let tt_pop_async = tt_pop.clone();
            let app_name_async = app_name_clone.clone();
            let cache_async = cached_usage.clone();
            let (tx, rx) = std::sync::mpsc::channel::<babydra_core::AppResourceUsage>();

            glib::timeout_add_local(Duration::from_millis(40), move || {
                if let Ok(usage) = rx.try_recv() {
                    *cache_async.borrow_mut() = Some(usage.clone());
                    if tt_pop_async.parent().is_some()
                        && tt_pop_async.root().is_some()
                        && tt_pop_async.is_visible()
                    {
                        let card = build_resource_card(&app_name_async, Some(&usage));
                        tt_pop_async.set_child(Some(&card));
                    }
                    return glib::ControlFlow::Break;
                }

                // If popover has been closed or unparented, stop polling
                if tt_pop_async.parent().is_none() || !tt_pop_async.is_visible() {
                    return glib::ControlFlow::Break;
                }

                glib::ControlFlow::Continue
            });

            let id_for_thread = app_id_str.clone();
            let exec_for_thread = app_exec.clone();
            let name_for_thread = app_name_clone.clone();
            std::thread::spawn(move || {
                let usage = babydra_core::get_app_resource_usage(
                    &id_for_thread,
                    &exec_for_thread,
                    &name_for_thread,
                );
                let _ = tx.send(usage);
            });
        });
        tooltip_popover.attach_hover(&btn, Some(update_fn.clone()));

        let pop_left = preview_popover.clone();
        let app_id_left = app_id.clone();
        let apps_left = running_apps_shared.clone();
        let popovers_left = popovers.clone();
        let tt_pop_click = tooltip_popover.clone();
        btn.connect_clicked(move |_| {
            if tt_pop_click.is_visible() {
                tt_pop_click.popdown();
            }
            let app_windows = get_windows_for_app(&apps_left, &app_id_left);
            if app_windows.len() > 1 {
                if pop_left.is_visible() {
                    pop_left.popdown();
                } else {
                    show_preview_menu(&pop_left, &popovers_left, &app_windows, &app_id_left);
                }
            } else if let Some(single_window) = app_windows.first() {
                let target_app_id = single_window.app_id.as_deref().unwrap_or(&app_id_left);
                let target_title = single_window.window_title.as_deref().unwrap_or("");
                toggle_app_window_async(target_app_id, target_title);
            } else {
                toggle_app_window_async(&app_id_left, "");
            }
        });

        let right_click = gtk4::GestureClick::new();
        right_click.set_button(3);
        let pop_right = preview_popover.clone();
        let app_id_right = app_id.clone();
        let apps_right = running_apps_shared.clone();
        let popovers_right = popovers.clone();
        let tt_pop_right = tooltip_popover.clone();
        right_click.connect_pressed(move |_, _, _, _| {
            if tt_pop_right.is_visible() {
                tt_pop_right.popdown();
            }
            let app_windows = get_windows_for_app(&apps_right, &app_id_right);
            if !app_windows.is_empty() {
                if pop_right.is_visible() {
                    pop_right.popdown();
                } else {
                    show_preview_menu(&pop_right, &popovers_right, &app_windows, &app_id_right);
                }
            }
        });
        btn.add_controller(right_click);

        popovers.borrow_mut().push(PopoverState {
            preview_popover,
            tooltip_popover: tooltip_popover.popover,
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

    let running_apps_shared: Arc<Mutex<Vec<DesktopApp>>> = Arc::new(Mutex::new(Vec::new()));

    let apps_box_clone = apps_box.clone();
    let popovers_clone = popovers.clone();
    let sig_clone = last_apps_sig.clone();
    let last_active_clone = last_active_id.clone();
    let last_ws_clone = last_ws.clone();
    let apps_shared_clone = running_apps_shared.clone();

    let ws_rx = babydra_core::services::workspace::subscribe();
    ws_rx.attach(None, move |snapshot| {
        let current_ws = snapshot.current_workspace;
        let ws_apps = snapshot.apps;
        let active = snapshot.active_app_id;

        if let Ok(mut lock) = apps_shared_clone.lock() {
            *lock = ws_apps.clone();
        }

        let new_apps_sig = babydra_core::services::workspace::get_apps_signature(
            current_ws, &ws_apps,
        );
        let ws_changed = *last_ws_clone.borrow() != current_ws;
        let active_changed = *last_active_clone.borrow() != active;

        if ws_changed || new_apps_sig != *sig_clone.borrow() {
            *last_ws_clone.borrow_mut() = current_ws;
            *sig_clone.borrow_mut() = new_apps_sig;
            *last_active_clone.borrow_mut() = active.clone();
            rebuild_taskbar(
                &apps_box_clone,
                ws_apps,
                active,
                &popovers_clone,
                apps_shared_clone.clone(),
            );
        } else if active_changed {
            *last_active_clone.borrow_mut() = active.clone();
            update_active_highlight(&apps_box_clone, active.as_deref());
        }

        glib::ControlFlow::Continue
    });

    parent_box
}
