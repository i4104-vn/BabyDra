use crate::widgets::status_bar::StatusBarWidgets;
use babydra_core::{ActivePane, SessionState};
use gtk4::prelude::*;
use std::cell::{Cell, RefCell};
use std::path::PathBuf;
use std::rc::Rc;

pub struct KeyShortcut {
    pub keyval: gtk4::gdk::Key,
    pub modifiers: gtk4::gdk::ModifierType,
    pub callback: Rc<dyn Fn()>,
}

/// Parse shortcut.
pub fn parse_shortcut(shortcut_str: &str) -> Option<(gtk4::gdk::Key, gtk4::gdk::ModifierType)> {
    let parts: Vec<&str> = shortcut_str.split('+').map(|s| s.trim()).collect();
    let mut modifiers = gtk4::gdk::ModifierType::empty();
    let mut key = None;

    for part in parts {
        let part_lower = part.to_lowercase();
        if part_lower == "ctrl" || part_lower == "control" {
            modifiers |= gtk4::gdk::ModifierType::CONTROL_MASK;
        } else if part_lower == "shift" {
            modifiers |= gtk4::gdk::ModifierType::SHIFT_MASK;
        } else if part_lower == "alt" {
            modifiers |= gtk4::gdk::ModifierType::ALT_MASK;
        } else {
            let k = match part_lower.as_str() {
                "f1" => Some(gtk4::gdk::Key::F1),
                "f2" => Some(gtk4::gdk::Key::F2),
                "f3" => Some(gtk4::gdk::Key::F3),
                "f4" => Some(gtk4::gdk::Key::F4),
                "f5" => Some(gtk4::gdk::Key::F5),
                "f6" => Some(gtk4::gdk::Key::F6),
                "f7" => Some(gtk4::gdk::Key::F7),
                "f8" => Some(gtk4::gdk::Key::F8),
                "f9" => Some(gtk4::gdk::Key::F9),
                "f10" => Some(gtk4::gdk::Key::F10),
                "f11" => Some(gtk4::gdk::Key::F11),
                "f12" => Some(gtk4::gdk::Key::F12),
                "enter" => Some(gtk4::gdk::Key::Return),
                "space" => Some(gtk4::gdk::Key::space),
                "escape" | "esc" => Some(gtk4::gdk::Key::Escape),
                "delete" | "del" => Some(gtk4::gdk::Key::Delete),
                "backspace" => Some(gtk4::gdk::Key::BackSpace),
                s if s.len() == 1 => {
                    let c = s.chars().next().unwrap();
                    gtk4::gdk::Key::from_name(&c.to_string())
                }
                s => gtk4::gdk::Key::from_name(s),
            };
            if let Some(keyval) = k {
                key = Some(keyval);
            }
        }
    }

    key.map(|k| (k, modifiers))
}

/// Registers keyboard shortcut events.
pub fn setup_key_shortcuts(
    window: &gtk4::ApplicationWindow,
    shortcuts: Vec<KeyShortcut>,
) -> gtk4::EventControllerKey {
    let key_controller = gtk4::EventControllerKey::new();
    key_controller.connect_key_pressed(move |_, keyval, _, state| {
        let clean_state = state
            & (gtk4::gdk::ModifierType::CONTROL_MASK
                | gtk4::gdk::ModifierType::SHIFT_MASK
                | gtk4::gdk::ModifierType::ALT_MASK);
        for shortcut in &shortcuts {
            if shortcut.keyval == keyval && shortcut.modifiers == clean_state {
                (shortcut.callback)();
                return glib::Propagation::Stop;
            }
        }
        glib::Propagation::Proceed
    });
    window.add_controller(key_controller.clone());
    key_controller
}

/// Automatically hides or shows the preview panel depending on window width changes.
pub fn setup_window_resize_handler(
    window: &gtk4::ApplicationWindow,
    layout_paned: gtk4::Paned,
    revealer: gtk4::Revealer,
    preview_visible: Rc<Cell<bool>>,
    user_wants_preview: Rc<Cell<bool>>,
    status_widgets_cell: Rc<RefCell<Option<StatusBarWidgets>>>,
) {
    window.connect_default_width_notify(move |win| {
        let w = win.width();
        if w < 700 && preview_visible.get() {
            revealer.set_reveal_child(false);
            preview_visible.set(false);
            let layout_paned_c = layout_paned.clone();
            let revealer_cc = revealer.clone();
            glib::timeout_add_local_once(std::time::Duration::from_millis(250), move || {
                if !revealer_cc.reveals_child() {
                    layout_paned_c.set_end_child(None::<&gtk4::Widget>);
                }
            });
            if let Some(ref sw) = *status_widgets_cell.borrow() {
                sw.btn_toggle_preview
                    .remove_css_class("status-bar-btn-active");
            }
        } else if w >= 700 && !preview_visible.get() && user_wants_preview.get() {
            layout_paned.set_end_child(Some(&revealer));
            revealer.set_reveal_child(true);
            preview_visible.set(true);
            if let Some(ref sw) = *status_widgets_cell.borrow() {
                sw.btn_toggle_preview.add_css_class("status-bar-btn-active");
            }
        }
    });
}

/// Sets up the hot-reload receiver loop responding to directory watcher triggers.
pub fn setup_file_watcher_receiver(
    session: Rc<RefCell<SessionState>>,
    _navigate_pane_no_watch_ref: Rc<RefCell<Option<Rc<dyn Fn(ActivePane, PathBuf)>>>>,
    _active_pane: Rc<Cell<ActivePane>>,
    left_content_handle: Rc<babydra_core::ContentViewHandle>,
    right_content_handle: Rc<RefCell<Option<Rc<babydra_core::ContentViewHandle>>>>,
    mut watch_rx: tokio::sync::mpsc::UnboundedReceiver<()>,
) {
    let left = left_content_handle;
    let right = right_content_handle;
    glib::MainContext::default().spawn_local(async move {
        let pending_timer = Rc::new(RefCell::new(None::<glib::SourceId>));
        while let Some(_) = watch_rx.recv().await {
            // Drain all queued events
            while watch_rx.try_recv().is_ok() {}

            // Cancel any previously scheduled refresh timer
            if let Some(source_id) = pending_timer.borrow_mut().take() {
                source_id.remove();
            }

            let left_c = left.clone();
            let right_c = right.clone();
            let timer_ref = pending_timer.clone();
            let session_c = session.clone();

            // Schedule quiet background refresh after 350ms of quiet time
            let source_id =
                glib::timeout_add_local_once(std::time::Duration::from_millis(350), move || {
                    timer_ref.borrow_mut().take();
                    let show_hidden = session_c.borrow().active_tab().show_hidden;

                    let left_path = left_c.current_path.borrow().clone();
                    let left_handle = left_c.clone();
                    glib::spawn_future_local(async move {
                        if let Ok(entries) =
                            babydra_core::load_directory(left_path.clone(), show_hidden).await
                        {
                            if *left_handle.current_path.borrow() == left_path {
                                crate::widgets::content_view::update_content_view_silent(
                                    &left_handle,
                                    &entries,
                                    left_path,
                                );
                            }
                        }
                    });

                    if let Some(ref r_handle) = *right_c.borrow() {
                        let right_path = r_handle.current_path.borrow().clone();
                        let r_handle_c = r_handle.clone();
                        glib::spawn_future_local(async move {
                            if let Ok(entries) =
                                babydra_core::load_directory(right_path.clone(), show_hidden).await
                            {
                                if *r_handle_c.current_path.borrow() == right_path {
                                    crate::widgets::content_view::update_content_view_silent(
                                        &r_handle_c,
                                        &entries,
                                        right_path,
                                    );
                                }
                            }
                        });
                    }
                });

            *pending_timer.borrow_mut() = Some(source_id);
        }
    });
}

/// Connects the D-Bus navigation receiver loop and spawns the DBus listener service.
pub fn setup_dbus_receiver(
    navigate_pane_no_watch_ref: Rc<RefCell<Option<Rc<dyn Fn(ActivePane, PathBuf)>>>>,
    active_pane: Rc<Cell<ActivePane>>,
) {
    let (dbus_tx, mut dbus_rx) = tokio::sync::mpsc::unbounded_channel::<std::path::PathBuf>();
    glib::MainContext::default().spawn_local(async move {
        while let Some(path) = dbus_rx.recv().await {
            if let Some(ref f) = *navigate_pane_no_watch_ref.borrow() {
                f(active_pane.get(), path);
            }
        }
    });

    tokio::spawn(async move {
        if let Err(e) = babydra_core::start_dbus_service(dbus_tx).await {
            eprintln!("Failed to start D-Bus service: {}", e);
        }
    });
}

/// Connects status bar button signals (view modes, sort dropdown, toggle preview, settings dialog).
pub fn setup_status_bar_wiring(
    status_bar_widgets_cell: Rc<RefCell<Option<StatusBarWidgets>>>,
    toggle_preview_rc: Rc<dyn Fn()>,
    view_mode_callback_rc: Rc<dyn Fn(String)>,
    sort_callback_rc: Rc<dyn Fn(String)>,
    parent_win: gtk4::Window,
    rebuild_shortcuts_cell: Rc<RefCell<Option<Rc<dyn Fn()>>>>,
    session: Rc<RefCell<SessionState>>,
    nav_c: Rc<RefCell<Option<Rc<dyn Fn(ActivePane, PathBuf)>>>>,
    act_c: Rc<Cell<ActivePane>>,
    preview_vc: Rc<Cell<bool>>,
) {
    if let Some(ref sw) = *status_bar_widgets_cell.borrow() {
        let toggle_p = toggle_preview_rc.clone();
        sw.btn_toggle_preview.connect_clicked(move |_| {
            toggle_p();
        });

        // View modes (Grid / List)
        let cb1 = view_mode_callback_rc.clone();
        sw.btn_view_icons.connect_clicked(move |_| {
            cb1("icons".to_string());
        });
        let cb2 = view_mode_callback_rc.clone();
        sw.btn_view_list.connect_clicked(move |_| {
            cb2("list".to_string());
        });

        // Sort Dropdown
        let sort_cb = sort_callback_rc.clone();
        sw.dropdown_sort.connect_selected_notify(move |dd| {
            let selected = dd.selected();
            let mode = match selected {
                0 => "auto".to_string(),
                1 => "date".to_string(),
                2 => "group".to_string(),
                _ => "auto".to_string(),
            };
            sort_cb(mode);
        });

        // Settings button
        let parent_win_c = parent_win.clone();
        let rebuild_c = rebuild_shortcuts_cell.clone();
        let session_inner = session.clone();
        let nav_inner = nav_c.clone();
        let act_inner = act_c.clone();
        let preview_inner = preview_vc.clone();
        let toggle_p_inner = toggle_preview_rc.clone();

        sw.btn_settings.connect_clicked(move |_| {
            let rebuild_inner_cb = rebuild_c.clone();
            let session_inner_cb = session_inner.clone();
            let nav_inner_cb = nav_inner.clone();
            let act_inner_cb = act_inner.clone();
            let preview_inner_cb = preview_inner.clone();
            let toggle_p_inner_cb = toggle_p_inner.clone();

            crate::widgets::settings_dialog::show_settings_dialog(&parent_win_c, move || {
                let settings = babydra_core::load_explore_settings();

                {
                    let mut s = session_inner_cb.borrow_mut();
                    let tab = s.active_tab_mut();
                    tab.show_hidden = settings.show_hidden;
                }

                let preview_changed = preview_inner_cb.get() != settings.preview_visible;
                if preview_changed {
                    toggle_p_inner_cb();
                }

                let path = session_inner_cb.borrow().active_tab().current_path.clone();
                if let Some(ref f) = *nav_inner_cb.borrow() {
                    f(act_inner_cb.get(), path);
                }

                if let Some(ref rebuild) = *rebuild_inner_cb.borrow() {
                    rebuild();
                }
            });
        });

        let settings = babydra_core::load_explore_settings();
        if settings.view_mode == "list" {
            sw.btn_view_list.add_css_class("status-bar-btn-active");
            sw.btn_view_icons.remove_css_class("status-bar-btn-active");
        } else {
            sw.btn_view_icons.add_css_class("status-bar-btn-active");
            sw.btn_view_list.remove_css_class("status-bar-btn-active");
        }
    }
}

/// Connects global application key shortcuts and rebuild callback.
pub fn setup_window_shortcuts(
    window: &gtk4::ApplicationWindow,
    toggle_split_view_rc: Rc<dyn Fn()>,
    toggle_preview_rc: Rc<dyn Fn()>,
    toggle_hidden_rc: Rc<dyn Fn()>,
    cut_cb_rc: Rc<dyn Fn()>,
    copy_cb_rc: Rc<dyn Fn()>,
    paste_cb_rc: Rc<dyn Fn()>,
    undo_cb_rc: Rc<dyn Fn()>,
    rebuild_shortcuts_cell: Rc<RefCell<Option<Rc<dyn Fn()>>>>,
) -> Rc<dyn Fn()> {
    let window = window.clone();
    let current_key_controller = Rc::new(RefCell::new(None::<gtk4::EventControllerKey>));
    let current_controller = current_key_controller.clone();

    let rebuild_shortcuts = move || {
        if let Some(ref old_controller) = *current_controller.borrow() {
            window.remove_controller(old_controller);
        }

        let settings = babydra_core::load_explore_settings();
        let mut shortcuts = Vec::new();

        let mut add_shortcut = |action: &str, cb: Rc<dyn Fn()>| {
            let shortcut_str = settings.get_keybind(action);
            if let Some((keyval, modifiers)) = parse_shortcut(&shortcut_str) {
                shortcuts.push(KeyShortcut {
                    keyval: keyval.clone(),
                    modifiers,
                    callback: cb.clone(),
                });

                if let Some(name) = keyval.name() {
                    if name.len() == 1 {
                        let upper_name = name.to_uppercase();
                        if let Some(upper_key) = gtk4::gdk::Key::from_name(&upper_name) {
                            shortcuts.push(KeyShortcut {
                                keyval: upper_key,
                                modifiers,
                                callback: cb.clone(),
                            });
                        }
                    }
                }
            }
        };

        add_shortcut("toggle_split", toggle_split_view_rc.clone());
        add_shortcut("toggle_preview", toggle_preview_rc.clone());
        add_shortcut("toggle_hidden", toggle_hidden_rc.clone());
        add_shortcut("cut", cut_cb_rc.clone());
        add_shortcut("copy", copy_cb_rc.clone());
        add_shortcut("paste", paste_cb_rc.clone());
        add_shortcut("undo", undo_cb_rc.clone());

        let new_controller = setup_key_shortcuts(&window, shortcuts);
        current_controller.replace(Some(new_controller));
    };

    let rebuild_shortcuts_rc = Rc::new(rebuild_shortcuts) as Rc<dyn Fn()>;
    rebuild_shortcuts_cell.replace(Some(rebuild_shortcuts_rc.clone()));
    rebuild_shortcuts_rc();
    rebuild_shortcuts_rc
}
