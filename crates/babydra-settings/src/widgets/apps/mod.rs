pub mod handler;
pub mod render;

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

    let pkgs = babydra_common::services::apps::pacman::get_installed_packages_list();

    let widget = render::build(&apps_data, &pkgs);
    handler::wire_events(&widget);

    widget.container.into()
}
