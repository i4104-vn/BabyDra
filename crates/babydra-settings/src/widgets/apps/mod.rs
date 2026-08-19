pub mod handler;
pub mod render;

use babydra_core::models::app_info::InstalledApp;
use babydra_core::models::settings::AppsData;
use gtk4::prelude::*;
use gtk4::Widget;

/// Creates a new `apps widget`.
pub fn create_apps_widget() -> Widget {
    // Build the initial UI instantly (0ms main-thread blocking) so the window
    // appears before background app scanning finishes.
    let (widget, auth_dialog, _action_items) = render::build(&[], &[]);
    let auth_dialog_rc = std::rc::Rc::new(auth_dialog);
    let pending_action = std::rc::Rc::new(std::cell::RefCell::new(None::<handler::PendingAction>));

    // Wire main event handlers (tabs switching, search, console close) on the visible widget
    handler::wire_main_events(&widget, &auth_dialog_rc, pending_action.clone());

    let apps_list_box = widget.apps_list_box.clone();
    let pkgs_list_box = widget.pkgs_list_box.clone();

    // Offload desktop app scanning & pacman query to background thread
    let (tx, rx) = std::sync::mpsc::channel::<AppsData>();
    std::thread::spawn(move || {
        let installed_apps = babydra_core::services::apps::discovery::scan_desktop_apps();
        let apps_data: Vec<InstalledApp> = installed_apps
            .into_iter()
            .map(|app| InstalledApp {
                name: app.name,
                description: app.exec,
                desktop_file: "".to_string(),
                icon: app.icon,
            })
            .collect();

        let pkgs = babydra_core::services::apps::pacman::get_installed_pkgs();

        let _ = tx.send(AppsData { apps_data, pkgs });
    });

    gtk4::glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
        if let Ok(data) = rx.try_recv() {
            while let Some(child) = apps_list_box.first_child() {
                apps_list_box.remove(&child);
            }
            while let Some(child) = pkgs_list_box.first_child() {
                pkgs_list_box.remove(&child);
            }

            let (new_w, _new_auth_dlg, action_items) = render::build(&data.apps_data, &data.pkgs);
            handler::wire_uninstall_items(&auth_dialog_rc, pending_action.clone(), action_items);

            // Move populated rows into the visible apps_list_box & pkgs_list_box
            while let Some(child) = new_w.apps_list_box.first_child() {
                new_w.apps_list_box.remove(&child);
                apps_list_box.append(&child);
            }
            while let Some(child) = new_w.pkgs_list_box.first_child() {
                new_w.pkgs_list_box.remove(&child);
                pkgs_list_box.append(&child);
            }

            gtk4::glib::ControlFlow::Break
        } else {
            gtk4::glib::ControlFlow::Continue
        }
    });

    widget.root.into()
}
