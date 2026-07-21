use std::rc::Rc;
use std::cell::{RefCell, Cell};
use std::path::PathBuf;
use gtk4::prelude::*;
use babydra_common::{SessionState, ActivePane};
use crate::widgets::status_bar::StatusBarWidgets;

pub struct KeyShortcut {
    pub keyval: gtk4::gdk::Key,
    pub modifiers: gtk4::gdk::ModifierType,
    pub callback: Rc<dyn Fn()>,
}

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
                s => {
                    gtk4::gdk::Key::from_name(s)
                }
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
        let clean_state = state & (gtk4::gdk::ModifierType::CONTROL_MASK 
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
                sw.btn_toggle_preview.remove_css_class("status-bar-btn-active");
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
    _session: Rc<RefCell<SessionState>>,
    navigate_pane_no_watch_ref: Rc<RefCell<Option<Rc<dyn Fn(ActivePane, PathBuf)>>>>,
    _active_pane: Rc<Cell<ActivePane>>,
    left_content_handle: Rc<babydra_common::ContentViewHandle>,
    right_content_handle: Rc<RefCell<Option<Rc<babydra_common::ContentViewHandle>>>>,
    mut watch_rx: tokio::sync::mpsc::UnboundedReceiver<()>,
) {
    let left = left_content_handle;
    let right = right_content_handle;
    let nav_no_watch = navigate_pane_no_watch_ref;
    glib::MainContext::default().spawn_local(async move {
        while let Some(_) = watch_rx.recv().await {
            if let Some(ref f) = *nav_no_watch.borrow() {
                // Refresh left pane
                let left_path = left.current_path.borrow().clone();
                f(ActivePane::Left, left_path);

                // Refresh right pane if split view is open
                if let Some(ref r_handle) = *right.borrow() {
                    let right_path = r_handle.current_path.borrow().clone();
                    f(ActivePane::Right, right_path);
                }
            }
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
        if let Err(e) = babydra_common::start_dbus_service(dbus_tx).await {
            eprintln!("Failed to start D-Bus service: {}", e);
        }
    });
}
