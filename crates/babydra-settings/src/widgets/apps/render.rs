//! Application Manager UI layout generator matching reference design Image 5.

use gtk4::prelude::*;
use gtk4::{Box, Button, Entry, Label, ListBox, Orientation, ScrolledWindow, Stack};
use babydra_common::models::app_info::{AppsWidget, InstalledApp, InstalledPackage};

pub fn build(apps: &[InstalledApp], pkgs: &[InstalledPackage]) -> AppsWidget {
    let container = Box::new(Orientation::Vertical, 16);
    container.set_vexpand(true);
    container.set_valign(gtk4::Align::Fill);

    // Title + Search Input Header Row
    let header_box = Box::new(Orientation::Horizontal, 12);
    header_box.set_margin_bottom(4);

    let title_label = Label::new(Some(&babydra_common::i18n::t("settings.apps_title")));
    title_label.add_css_class("settings-page-title");
    title_label.set_hexpand(true);
    title_label.set_halign(gtk4::Align::Start);

    let search_entry = Entry::new();
    search_entry.set_placeholder_text(Some(&babydra_common::i18n::t("settings.apps_search_placeholder")));
    search_entry.add_css_class("sidebar-search-entry");
    search_entry.set_width_request(220);

    header_box.append(&title_label);
    header_box.append(&search_entry);
    container.append(&header_box);

    // Tabs Bar Row
    let tabs_box = Box::new(Orientation::Horizontal, 12);
    let tab_apps_btn = Button::with_label(&babydra_common::i18n::t("settings.apps_tab_apps"));
    tab_apps_btn.add_css_class("app-tab-btn");
    tab_apps_btn.add_css_class("active");
    tab_apps_btn.set_cursor_from_name(Some("pointer"));

    let tab_packages_btn = Button::with_label(&babydra_common::i18n::t("settings.apps_tab_packages"));
    tab_packages_btn.add_css_class("app-tab-btn");
    tab_packages_btn.set_cursor_from_name(Some("pointer"));

    tabs_box.append(&tab_apps_btn);
    tabs_box.append(&tab_packages_btn);
    container.append(&tabs_box);

    // Stack for Apps vs Packages view
    let stack = Stack::new();
    stack.set_transition_type(gtk4::StackTransitionType::Crossfade);
    stack.set_vexpand(true);
    stack.set_valign(gtk4::Align::Fill);

    // 1. Apps List (Glass Panel List Box)
    let apps_glass_card = Box::new(Orientation::Vertical, 0);
    apps_glass_card.add_css_class("glass-panel");
    apps_glass_card.set_vexpand(true);
    apps_glass_card.set_valign(gtk4::Align::Fill);

    let apps_scrolled = ScrolledWindow::new();
    apps_scrolled.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    apps_scrolled.set_vexpand(true);
    apps_scrolled.set_valign(gtk4::Align::Fill);

    let apps_list_box = ListBox::new();
    apps_list_box.set_selection_mode(gtk4::SelectionMode::None);

    for app in apps {
        let row_box = Box::new(Orientation::Horizontal, 14);
        row_box.add_css_class("settings-card-row");
        row_box.set_margin_top(10);
        row_box.set_margin_bottom(10);
        row_box.set_margin_start(16);
        row_box.set_margin_end(16);

        // App Icon Box Container
        let icon_box = Box::new(Orientation::Vertical, 0);
        icon_box.add_css_class("blue-icon-badge-sm");
        icon_box.set_valign(gtk4::Align::Center);
        icon_box.set_halign(gtk4::Align::Start);

        let icon_img = babydra_utils::ui::icon::get_icon("th-large", 18);
        icon_img.set_pixel_size(18);
        icon_img.set_valign(gtk4::Align::Center);
        icon_img.set_halign(gtk4::Align::Center);
        icon_img.set_vexpand(true);
        icon_box.append(&icon_img);
        row_box.append(&icon_box);

        // App Info Column (Title + Subtitle)
        let text_box = Box::new(Orientation::Vertical, 2);
        text_box.set_hexpand(true);
        text_box.set_valign(gtk4::Align::Center);

        let name_lbl = Label::new(Some(&app.name));
        name_lbl.add_css_class("settings-row-title");
        name_lbl.set_halign(gtk4::Align::Start);
        text_box.append(&name_lbl);

        let desc_lbl = Label::new(Some(&app.description));
        desc_lbl.add_css_class("settings-row-desc");
        desc_lbl.set_halign(gtk4::Align::Start);
        text_box.append(&desc_lbl);

        row_box.append(&text_box);

        // X Uninstall Action Button
        let uninstall_btn = Button::new();
        uninstall_btn.add_css_class("icon-btn");
        uninstall_btn.add_css_class("delete-btn");
        uninstall_btn.set_valign(gtk4::Align::Center);
        uninstall_btn.set_cursor_from_name(Some("pointer"));

        let x_icon = babydra_utils::ui::icon::get_icon("close", 14);
        x_icon.set_pixel_size(14);
        uninstall_btn.set_child(Some(&x_icon));

        let app_name = app.name.clone();
        let row_copy = row_box.clone();
        let apps_list_copy = apps_list_box.clone();
        uninstall_btn.connect_clicked(move |_| {
            let pkg_name = app_name.to_lowercase().replace(' ', "-");
            let row_copy_c = row_copy.clone();
            let apps_list_copy_c = apps_list_copy.clone();

            let (tx, rx) = std::sync::mpsc::channel::<String>();
            std::thread::spawn(move || {
                let _ = babydra_common::services::apps::pacman::stream_uninstall_package(&pkg_name, None, tx);
            });

            glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
                if rx.try_recv().is_ok() {
                    apps_list_copy_c.remove(&row_copy_c);
                    glib::ControlFlow::Break
                } else {
                    glib::ControlFlow::Continue
                }
            });
        });

        row_box.append(&uninstall_btn);
        apps_list_box.append(&row_box);
    }
    apps_scrolled.set_child(Some(&apps_list_box));
    apps_glass_card.append(&apps_scrolled);
    stack.add_named(&apps_glass_card, Some("apps"));

    // 2. Packages List
    let pkgs_glass_card = Box::new(Orientation::Vertical, 0);
    pkgs_glass_card.add_css_class("glass-panel");
    pkgs_glass_card.set_vexpand(true);
    pkgs_glass_card.set_valign(gtk4::Align::Fill);

    let pkgs_scrolled = ScrolledWindow::new();
    pkgs_scrolled.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    pkgs_scrolled.set_vexpand(true);
    pkgs_scrolled.set_valign(gtk4::Align::Fill);

    let pkgs_list_box = ListBox::new();
    pkgs_list_box.set_selection_mode(gtk4::SelectionMode::None);

    for pkg in pkgs.iter().take(200) {
        let row_box = Box::new(Orientation::Horizontal, 14);
        row_box.add_css_class("settings-card-row");
        row_box.set_margin_top(10);
        row_box.set_margin_bottom(10);
        row_box.set_margin_start(16);
        row_box.set_margin_end(16);

        // Package Icon Box Container
        let icon_box = Box::new(Orientation::Vertical, 0);
        icon_box.add_css_class("blue-icon-badge-sm");
        icon_box.set_valign(gtk4::Align::Center);
        icon_box.set_halign(gtk4::Align::Start);

        let icon_img = babydra_utils::ui::icon::get_icon("archive", 18);
        icon_img.set_pixel_size(18);
        icon_img.set_valign(gtk4::Align::Center);
        icon_img.set_halign(gtk4::Align::Center);
        icon_img.set_vexpand(true);
        icon_box.append(&icon_img);
        row_box.append(&icon_box);

        // Package Info Column (Name + Version)
        let text_box = Box::new(Orientation::Vertical, 2);
        text_box.set_hexpand(true);
        text_box.set_valign(gtk4::Align::Center);

        let name_lbl = Label::new(Some(&pkg.name));
        name_lbl.add_css_class("settings-row-title");
        name_lbl.set_halign(gtk4::Align::Start);
        text_box.append(&name_lbl);

        let ver_lbl = Label::new(Some(&pkg.version));
        ver_lbl.add_css_class("settings-row-desc");
        ver_lbl.set_halign(gtk4::Align::Start);
        text_box.append(&ver_lbl);

        row_box.append(&text_box);

        let uninstall_btn = Button::new();
        uninstall_btn.add_css_class("icon-btn");
        uninstall_btn.add_css_class("delete-btn");
        uninstall_btn.set_valign(gtk4::Align::Center);
        uninstall_btn.set_cursor_from_name(Some("pointer"));

        let x_icon = babydra_utils::ui::icon::get_icon("close", 14);
        x_icon.set_pixel_size(14);
        uninstall_btn.set_child(Some(&x_icon));

        let pkg_name = pkg.name.clone();
        let row_copy = row_box.clone();
        let pkgs_list_copy = pkgs_list_box.clone();
        uninstall_btn.connect_clicked(move |_| {
            let pkg_name_c = pkg_name.clone();
            let row_copy_c = row_copy.clone();
            let pkgs_list_copy_c = pkgs_list_copy.clone();

            let (tx, rx) = std::sync::mpsc::channel::<String>();
            std::thread::spawn(move || {
                let _ = babydra_common::services::apps::pacman::stream_uninstall_package(&pkg_name_c, None, tx);
            });

            glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
                if rx.try_recv().is_ok() {
                    pkgs_list_copy_c.remove(&row_copy_c);
                    glib::ControlFlow::Break
                } else {
                    glib::ControlFlow::Continue
                }
            });
        });

        row_box.append(&uninstall_btn);
        pkgs_list_box.append(&row_box);
    }
    pkgs_scrolled.set_child(Some(&pkgs_list_box));
    pkgs_glass_card.append(&pkgs_scrolled);
    stack.add_named(&pkgs_glass_card, Some("packages"));

    container.append(&stack);

    AppsWidget {
        container,
        search_entry,
        tab_apps_btn,
        tab_packages_btn,
        stack,
        apps_list_box,
        pkgs_list_box,
    }
}

