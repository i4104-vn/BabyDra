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

    let pkgs = babydra_common::services::apps::pacman::get_installed_packages_list();

    let widget = render::build(&apps_data, &pkgs);

    let tab_apps_btn_copy = widget.tab_apps_btn.clone();
    let tab_packages_btn_copy = widget.tab_packages_btn.clone();
    let stack1 = widget.stack.clone();
    widget.tab_apps_btn.connect_clicked(move |_| {
        stack1.set_visible_child_name("apps");
        tab_apps_btn_copy.add_css_class("active");
        tab_packages_btn_copy.remove_css_class("active");
    });

    let tab_apps_btn_copy2 = widget.tab_apps_btn.clone();
    let tab_packages_btn_copy2 = widget.tab_packages_btn.clone();
    let stack2 = widget.stack.clone();
    widget.tab_packages_btn.connect_clicked(move |_| {
        stack2.set_visible_child_name("packages");
        tab_packages_btn_copy2.add_css_class("active");
        tab_apps_btn_copy2.remove_css_class("active");
    });

    let apps_list = widget.apps_list_box.clone();
    let pkgs_list = widget.pkgs_list_box.clone();
    widget.search_entry.connect_changed(move |entry| {
        let query = entry.text().to_lowercase();
        
        let mut app_child = apps_list.first_child();
        while let Some(c) = app_child {
            let mut visible = false;
            if query.is_empty() {
                visible = true;
            } else if let Some(row_box) = c.downcast_ref::<gtk4::Box>() {
                let mut label_child = row_box.first_child();
                while let Some(lc) = label_child {
                    if let Some(tb) = lc.downcast_ref::<gtk4::Box>() {
                        let mut sub = tb.first_child();
                        while let Some(lbl) = sub {
                            if let Some(l) = lbl.downcast_ref::<gtk4::Label>() {
                                if l.text().to_lowercase().contains(&query) {
                                    visible = true;
                                    break;
                                }
                            }
                            sub = lbl.next_sibling();
                        }
                    }
                    label_child = lc.next_sibling();
                }
            }
            c.set_visible(visible);
            app_child = c.next_sibling();
        }

        let mut pkg_child = pkgs_list.first_child();
        while let Some(c) = pkg_child {
            let mut visible = false;
            if query.is_empty() {
                visible = true;
            } else if let Some(row_box) = c.downcast_ref::<gtk4::Box>() {
                let mut label_child = row_box.first_child();
                while let Some(lc) = label_child {
                    if let Some(l) = lc.downcast_ref::<gtk4::Label>() {
                        if l.text().to_lowercase().contains(&query) {
                            visible = true;
                            break;
                        }
                    }
                    label_child = lc.next_sibling();
                }
            }
            c.set_visible(visible);
            pkg_child = c.next_sibling();
        }
    });

    widget.container.into()
}
