pub mod render;

use gtk4::prelude::*;
use gtk4::Widget;

pub fn create_apps_widget() -> Widget {
    let installed_apps = babydra_common::services::apps::discovery::scan_desktop_apps_from_filesystem();
    let apps_data: Vec<babydra_common::models::app_info::InstalledApp> = installed_apps
        .into_iter()
        .map(|app| babydra_common::models::app_info::InstalledApp {
            name: app.name,
            description: app.exec,
            desktop_file: "".to_string(),
            icon: app.icon,
        })
        .collect();

    let pkgs = vec![]; // Loaded lazily or empty by default

    let widget = render::build(&apps_data, &pkgs);

    let stack1 = widget.stack.clone();
    widget.tab_apps_btn.connect_clicked(move |_| {
        stack1.set_visible_child_name("apps");
    });

    let stack2 = widget.stack.clone();
    widget.tab_packages_btn.connect_clicked(move |_| {
        stack2.set_visible_child_name("packages");
    });

    widget.container.into()
}
