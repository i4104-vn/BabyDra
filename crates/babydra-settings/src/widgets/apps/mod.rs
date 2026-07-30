pub mod handler;
pub mod render;

use gtk4::prelude::*;
use gtk4::Widget;
use babydra_common::models::app_info::{InstalledApp, InstalledPackage};

pub struct AppsData {
    pub apps_data: Vec<InstalledApp>,
    pub pkgs: Vec<InstalledPackage>,
}

pub fn create_apps_widget() -> Widget {
    // Build initial UI instantly (0ms main thread blocking!)
    let (widget, auth_dialog, _uninstall_items) = render::build(&[], &[]);

    // Wire main event handlers (tabs switching, search, console close) on the visible widget
    handler::wire_events(&widget, auth_dialog, Vec::new());

    let apps_list_box = widget.apps_list_box.clone();
    let pkgs_list_box = widget.pkgs_list_box.clone();
    let widget_c = widget.clone();

    // Offload desktop app scanning & pacman query to background thread
    let (tx, rx) = std::sync::mpsc::channel::<AppsData>();
    std::thread::spawn(move || {
        let installed_apps = babydra_common::services::apps::discovery::scan_desktop_apps_from_filesystem();
        let apps_data: Vec<InstalledApp> = installed_apps
            .into_iter()
            .map(|app| InstalledApp {
                name: app.name,
                description: app.exec,
                desktop_file: "".to_string(),
                icon: app.icon,
            })
            .collect();

        let pkgs = babydra_common::services::apps::pacman::get_installed_packages_list();

        let _ = tx.send(AppsData { apps_data, pkgs });
    });

    gtk4::glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
        if let Ok(data) = rx.try_recv() {
            // Clear placeholder lists
            while let Some(child) = apps_list_box.first_child() {
                apps_list_box.remove(&child);
            }
            while let Some(child) = pkgs_list_box.first_child() {
                pkgs_list_box.remove(&child);
            }

            // Build populated rows & wire uninstall action buttons
            let (new_w, new_auth_dlg, uninstall_items) = render::build(&data.apps_data, &data.pkgs);
            handler::wire_events(&widget_c, new_auth_dlg, uninstall_items);

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
