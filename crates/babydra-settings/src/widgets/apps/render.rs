//! Application Manager UI layout generator matching reference design Image 5.

use gtk4::prelude::*;
use gtk4::{Box, Button, Entry, Label, ListBox, Orientation, ScrolledWindow, Stack};
use babydra_common::models::app_info::{InstalledApp, InstalledPackage};

pub struct AppsWidget {
    pub container: Box,
    pub search_entry: Entry,
    pub tab_apps_btn: Button,
    pub tab_packages_btn: Button,
    pub stack: Stack,
    pub apps_list_box: ListBox,
    pub pkgs_list_box: ListBox,
}

pub fn build(apps: &[InstalledApp], pkgs: &[InstalledPackage]) -> AppsWidget {
    let container = Box::new(Orientation::Vertical, 16);
    container.set_vexpand(true);
    container.set_valign(gtk4::Align::Fill);

    // Title + Search Input Header Row
    let header_box = Box::new(Orientation::Horizontal, 12);
    header_box.set_margin_bottom(4);

    let title_label = Label::new(Some("Application Manager"));
    title_label.add_css_class("settings-page-title");
    title_label.set_hexpand(true);
    title_label.set_halign(gtk4::Align::Start);

    let search_entry = Entry::new();
    search_entry.set_placeholder_text(Some("Search..."));
    search_entry.add_css_class("sidebar-search-entry");
    search_entry.set_width_request(220);

    header_box.append(&title_label);
    header_box.append(&search_entry);
    container.append(&header_box);

    // Tabs Bar Row
    let tabs_box = Box::new(Orientation::Horizontal, 12);
    let tab_apps_btn = Button::with_label("Apps");
    tab_apps_btn.add_css_class("app-tab-btn");
    tab_apps_btn.add_css_class("active");
    tab_apps_btn.set_cursor_from_name(Some("pointer"));

    let tab_packages_btn = Button::with_label("Packages");
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

    for pkg in pkgs.iter().take(100) {
        let row_box = Box::new(Orientation::Horizontal, 14);
        row_box.add_css_class("settings-card-row");
        row_box.set_margin_top(8);
        row_box.set_margin_bottom(8);
        row_box.set_margin_start(16);
        row_box.set_margin_end(16);

        let name_lbl = Label::new(Some(&pkg.name));
        name_lbl.add_css_class("settings-row-title");
        name_lbl.set_hexpand(true);
        name_lbl.set_halign(gtk4::Align::Start);

        let ver_lbl = Label::new(Some(&pkg.version));
        ver_lbl.add_css_class("settings-row-desc");

        let uninstall_btn = Button::new();
        uninstall_btn.add_css_class("icon-btn");
        uninstall_btn.add_css_class("delete-btn");
        uninstall_btn.set_valign(gtk4::Align::Center);

        let x_icon = babydra_utils::ui::icon::get_icon("close", 14);
        x_icon.set_pixel_size(14);
        uninstall_btn.set_child(Some(&x_icon));

        row_box.append(&name_lbl);
        row_box.append(&ver_lbl);
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

