use std::rc::Rc;
use std::cell::{RefCell, Cell};
use std::path::PathBuf;
use gtk4::prelude::*;
use babydra_common::{SessionState, ActivePane, ContentViewHandle};
use crate::widgets::status_bar::StatusBarWidgets;

/// Registers keyboard shortcut events (F3 for split view, F4 for preview pane, Ctrl+H for hidden files).
pub fn setup_key_shortcuts(
    window: &gtk4::ApplicationWindow,
    toggle_split: Rc<dyn Fn()>,
    toggle_preview: Rc<dyn Fn()>,
    toggle_hidden: Rc<dyn Fn()>,
) {
    let key_controller = gtk4::EventControllerKey::new();
    key_controller.connect_key_pressed(move |_, keyval, _, state| {
        let has_ctrl = state.contains(gtk4::gdk::ModifierType::CONTROL_MASK);
        if keyval == gtk4::gdk::Key::F3 {
            toggle_split();
            glib::Propagation::Stop
        } else if keyval == gtk4::gdk::Key::F4 {
            toggle_preview();
            glib::Propagation::Stop
        } else if has_ctrl && (keyval == gtk4::gdk::Key::h || keyval == gtk4::gdk::Key::H) {
            toggle_hidden();
            glib::Propagation::Stop
        } else {
            glib::Propagation::Proceed
        }
    });
    window.add_controller(key_controller);
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
    session: Rc<RefCell<SessionState>>,
    navigate_pane_no_watch_ref: Rc<RefCell<Option<Rc<dyn Fn(ActivePane, PathBuf)>>>>,
    active_pane: Rc<Cell<ActivePane>>,
    mut watch_rx: tokio::sync::mpsc::UnboundedReceiver<()>,
) {
    glib::MainContext::default().spawn_local(async move {
        while let Some(_) = watch_rx.recv().await {
            let path = session.borrow().active_tab().current_path.clone();
            if let Some(ref f) = *navigate_pane_no_watch_ref.borrow() {
                f(active_pane.get(), path);
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

/// Registers the system dark/light mode preference change notifier.
pub fn setup_theme_listener(
    left_content_handle: Rc<ContentViewHandle>,
    right_content_handle: Rc<RefCell<Option<Rc<ContentViewHandle>>>>,
) {
    if let Some(settings) = gtk4::Settings::default() {
        let left_handle = left_content_handle.clone();
        let right_handle = right_content_handle.clone();
        settings.connect_gtk_application_prefer_dark_theme_notify(move |_| {
            crate::widgets::content_view::update_content_view_ui(&left_handle);
            if let Some(ref rh) = *right_handle.borrow() {
                crate::widgets::content_view::update_content_view_ui(rh);
            }
        });
    }
}
