//! Settings window layout builder.

use gtk4::prelude::*;
use gtk4::gio;

use crate::widgets;

/// Sidebar navigation i18n keys (ordered to match button creation order).
const SIDEBAR_I18N_KEYS: &[&str] = &[
    "settings.nav_wifi",
    "settings.nav_hosts",
    "settings.nav_vpn",
    "settings.nav_bluetooth",
    "settings.nav_wallpaper_themes",
    "settings.nav_displays",
    "settings.nav_installed_apps",
    "settings.nav_startup_apps",
    "settings.nav_system_update",
    "settings.nav_about_system",
];

/// Populate a content stack with all Settings widget pages.
fn populate_content_stack(stack: &gtk4::Stack) {
    // Remove existing children first
    while let Some(child) = stack.first_child() {
        stack.remove(&child);
    }

    stack.add_named(&widgets::wifi::create_wifi_widget(), Some("wifi"));
    stack.add_named(&widgets::vpn::create_vpn_widget(), Some("vpn"));
    stack.add_named(&widgets::bluetooth::create_bluetooth_widget(), Some("bluetooth"));
    stack.add_named(&widgets::appearance::create_appearance_widget(), Some("appearance"));
    stack.add_named(&widgets::displays::create_displays_widget(), Some("displays"));
    stack.add_named(&widgets::apps::create_apps_widget(), Some("apps"));
    stack.add_named(&widgets::startup::create_startup_widget(), Some("startup"));
    stack.add_named(&widgets::system_update::create_system_update_widget(), Some("system_update"));
    stack.add_named(&widgets::hosts::create_hosts_widget(), Some("hosts"));
    stack.add_named(&widgets::system_info::create_system_widget(), Some("system"));
}

/// Find and update the Label text inside a sidebar Button (Button > Box > Label).
fn update_sidebar_label(btn: &gtk4::Button, new_text: &str) {
    if let Some(child) = btn.child() {
        if let Ok(hbox) = child.downcast::<gtk4::Box>() {
            let mut widget = hbox.first_child();
            while let Some(w) = widget {
                if let Ok(label) = w.clone().downcast::<gtk4::Label>() {
                    label.set_text(new_text);
                    return;
                }
                widget = w.next_sibling();
            }
        }
    }
}



pub fn build_main_window(app: &gtk4::Application) {
    let window = gtk4::ApplicationWindow::new(app);
    window.set_title(Some("Settings"));
    window.set_default_size(960, 640);
    window.add_css_class("settings-window");

    let overlay = gtk4::Overlay::new();

    // Main layout split box
    let main_layout = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);

    // ── Left: Windows 11 / Acrylic Sidebar Navigation ───────────
    let sidebar_box = gtk4::Box::new(gtk4::Orientation::Vertical, 10);
    sidebar_box.add_css_class("sidebar");
    sidebar_box.set_width_request(240);
    sidebar_box.set_hexpand(false);
    sidebar_box.set_vexpand(true);
    sidebar_box.set_margin_top(8);
    sidebar_box.set_margin_bottom(8);
    sidebar_box.set_margin_start(8);

    // 1. App Title Header Box
    let profile_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
    profile_box.set_margin_top(12);
    profile_box.set_margin_bottom(4);
    profile_box.set_margin_start(12);
    profile_box.set_margin_end(12);

    let logo_img = babydra_utils::ui::icon::get_icon("logo", 28);
    logo_img.set_pixel_size(28);
    logo_img.set_valign(gtk4::Align::Center);
    profile_box.append(&logo_img);

    let title_info_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    title_info_box.set_valign(gtk4::Align::Center);

    let app_title_lbl = gtk4::Label::new(Some(&babydra_common::i18n::t("settings.title")));
    app_title_lbl.add_css_class("profile-user-name");
    app_title_lbl.set_halign(gtk4::Align::Start);

    let app_sub_lbl = gtk4::Label::new(Some(&babydra_common::i18n::t("settings.subtitle")));
    app_sub_lbl.add_css_class("settings-row-desc");
    app_sub_lbl.set_halign(gtk4::Align::Start);

    title_info_box.append(&app_title_lbl);
    title_info_box.append(&app_sub_lbl);
    profile_box.append(&title_info_box);
    sidebar_box.append(&profile_box);

    // Search Input Box
    let search_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    search_box.set_margin_start(12);
    search_box.set_margin_end(12);
    search_box.set_margin_top(4);
    search_box.set_margin_bottom(8);

    let search_entry = gtk4::Entry::new();
    search_entry.set_placeholder_text(Some(&babydra_common::i18n::t("settings.search_placeholder")));
    search_entry.add_css_class("sidebar-search-entry");
    search_entry.set_hexpand(true);
    search_box.append(&search_entry);
    sidebar_box.append(&search_box);

    let profile_sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    profile_sep.add_css_class("profile-separator");
    sidebar_box.append(&profile_sep);

    // 2. Navigation Scrolled List
    let sidebar_scroll = gtk4::ScrolledWindow::new();
    sidebar_scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    sidebar_scroll.set_vexpand(true);

    let nav_container = gtk4::Box::new(gtk4::Orientation::Vertical, 4);

    let btn_wifi = babydra_utils::components::create_sidebar_item_button(&babydra_common::i18n::t("settings.nav_wifi"), "wifi", "sidebar-item", || {});
    btn_wifi.set_cursor_from_name(Some("pointer"));

    let btn_update = babydra_utils::components::create_sidebar_item_button(&babydra_common::i18n::t("settings.nav_system_update"), "history", "sidebar-item", || {});
    btn_update.set_cursor_from_name(Some("pointer"));

    let btn_vpn = babydra_utils::components::create_sidebar_item_button(&babydra_common::i18n::t("settings.nav_vpn"), "lock", "sidebar-item", || {});
    btn_vpn.set_cursor_from_name(Some("pointer"));

    let btn_bt = babydra_utils::components::create_sidebar_item_button(&babydra_common::i18n::t("settings.nav_bluetooth"), "bluetooth", "sidebar-item", || {});
    btn_bt.set_cursor_from_name(Some("pointer"));

    let btn_app = babydra_utils::components::create_sidebar_item_button(&babydra_common::i18n::t("settings.nav_wallpaper_themes"), "palette", "sidebar-item", || {});
    btn_app.set_cursor_from_name(Some("pointer"));

    let btn_displays = babydra_utils::components::create_sidebar_item_button(&babydra_common::i18n::t("settings.nav_displays"), "desktop", "sidebar-item", || {});
    btn_displays.set_cursor_from_name(Some("pointer"));

    let btn_apps = babydra_utils::components::create_sidebar_item_button(&babydra_common::i18n::t("settings.nav_installed_apps"), "th-large", "sidebar-item", || {});
    btn_apps.set_cursor_from_name(Some("pointer"));

    let btn_startup = babydra_utils::components::create_sidebar_item_button(&babydra_common::i18n::t("settings.nav_startup_apps"), "cog", "sidebar-item", || {});
    btn_startup.set_cursor_from_name(Some("pointer"));

    let btn_hosts = babydra_utils::components::create_sidebar_item_button(&babydra_common::i18n::t("settings.nav_hosts"), "file-text", "sidebar-item", || {});
    btn_hosts.set_cursor_from_name(Some("pointer"));

    let btn_sys = babydra_utils::components::create_sidebar_item_button(&babydra_common::i18n::t("settings.nav_about_system"), "info", "sidebar-item", || {});
    btn_sys.add_css_class("active-nav");
    btn_sys.set_cursor_from_name(Some("pointer"));

    nav_container.append(&btn_wifi);
    nav_container.append(&btn_hosts);
    nav_container.append(&btn_vpn);
    nav_container.append(&btn_bt);
    nav_container.append(&btn_app);
    nav_container.append(&btn_displays);
    nav_container.append(&btn_apps);
    nav_container.append(&btn_startup);
    nav_container.append(&btn_update);

    // Divider before pinned footer About System item
    let footer_sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    footer_sep.add_css_class("profile-separator");

    let footer_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    footer_box.set_margin_start(4);
    footer_box.set_margin_end(4);
    footer_box.set_margin_bottom(8);
    footer_box.append(&footer_sep);
    footer_box.append(&btn_sys);

    sidebar_scroll.set_child(Some(&nav_container));
    sidebar_box.append(&sidebar_scroll);
    sidebar_box.append(&footer_box);
    main_layout.append(&sidebar_box);

    // ── Right: Content Stack Panel ──────────────────────────────
    let content_stack = gtk4::Stack::new();
    content_stack.set_transition_type(gtk4::StackTransitionType::Crossfade);
    content_stack.set_transition_duration(200);
    content_stack.set_hexpand(true);
    content_stack.set_vexpand(true);
    content_stack.add_css_class("settings-content");

    populate_content_stack(&content_stack);
    content_stack.set_visible_child_name("system");

    let right_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    right_box.set_hexpand(true);
    right_box.set_vexpand(true);
    right_box.set_margin_top(8);
    right_box.set_margin_bottom(8);
    right_box.set_margin_start(8);
    right_box.set_margin_end(8);
    right_box.append(&content_stack);

    main_layout.append(&right_box);
    overlay.set_child(Some(&main_layout));

    // --- Navigation Active Handling ---
    let all_btns = vec![
        ("wifi", btn_wifi.clone()),
        ("hosts", btn_hosts.clone()),
        ("vpn", btn_vpn.clone()),
        ("bluetooth", btn_bt.clone()),
        ("appearance", btn_app.clone()),
        ("displays", btn_displays.clone()),
        ("apps", btn_apps.clone()),
        ("startup", btn_startup.clone()),
        ("system_update", btn_update.clone()),
        ("system", btn_sys.clone()),
    ];

    for (name, button) in all_btns.iter() {
        let name_str = name.to_string();
        let stack_c = content_stack.clone();
        let all_btns_c = all_btns.clone();
        let button_c = button.clone();
        button.connect_clicked(move |_| {
            for (_, b) in all_btns_c.iter() {
                b.remove_css_class("active-nav");
            }
            button_c.add_css_class("active-nav");
            stack_c.set_visible_child_name(&name_str);
        });
    }

    // --- Rebuild UI Action (triggered by language toggle) ---
    let rebuild_action = gio::SimpleAction::new("rebuild-ui", None);
    {
        let content_stack_c = content_stack.clone();
        let app_title_lbl_c = app_title_lbl.clone();
        let app_sub_lbl_c = app_sub_lbl.clone();
        let search_entry_c = search_entry.clone();
        let all_btns_c = all_btns.clone();

        rebuild_action.connect_activate(move |_, _| {
            let stack = content_stack_c.clone();
            let title = app_title_lbl_c.clone();
            let sub = app_sub_lbl_c.clone();
            let search = search_entry_c.clone();
            let btns = all_btns_c.clone();

            // Use idle_add_local_once to avoid blocking the click handler
            gtk4::glib::idle_add_local_once(move || {
                // 1. Update sidebar labels
                for (i, (_, btn)) in btns.iter().enumerate() {
                    if let Some(key) = SIDEBAR_I18N_KEYS.get(i) {
                        update_sidebar_label(btn, &babydra_common::i18n::t(key));
                    }
                }

                // 2. Update header labels
                title.set_text(&babydra_common::i18n::t("settings.title"));
                sub.set_text(&babydra_common::i18n::t("settings.subtitle"));
                search.set_placeholder_text(Some(&babydra_common::i18n::t("settings.search_placeholder")));

                // 3. Rebuild content stack pages (preserving current page)
                let current_page = stack.visible_child_name().map(|s| s.to_string());
                populate_content_stack(&stack);
                if let Some(page) = current_page {
                    stack.set_visible_child_name(&page);
                }
            });
        });
    }
    window.add_action(&rebuild_action);

    window.set_child(Some(&overlay));
    window.present();
}
