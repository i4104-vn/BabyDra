//! Settings window layout builder with lazy page loading and dynamic navigation.

use gtk4::gio;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use crate::widgets;

struct NavItem {
    id: &'static str,
    icon: &'static str,
    i18n_key: &'static str,
}

struct NavCategory {
    title_key: &'static str,
    items: &'static [NavItem],
}

const NAV_CATEGORIES: &[NavCategory] = &[
    NavCategory {
        title_key: "settings.cat_network",
        items: &[
            NavItem { id: "wifi", icon: "wifi", i18n_key: "settings.nav_wifi" },
            NavItem { id: "bluetooth", icon: "bluetooth", i18n_key: "settings.nav_bluetooth" },
            NavItem { id: "vpn", icon: "lock", i18n_key: "settings.nav_vpn" },
            NavItem { id: "hosts", icon: "file-text", i18n_key: "settings.nav_hosts" },
        ],
    },
    NavCategory {
        title_key: "settings.cat_hardware",
        items: &[
            NavItem { id: "displays", icon: "desktop", i18n_key: "settings.nav_displays" },
            NavItem { id: "keybinds", icon: "cog", i18n_key: "settings.nav_keybinds" },
        ],
    },
    NavCategory {
        title_key: "settings.cat_apps",
        items: &[
            NavItem { id: "appearance", icon: "palette", i18n_key: "settings.nav_wallpaper_themes" },
            NavItem { id: "apps", icon: "th-large", i18n_key: "settings.nav_installed_apps" },
            NavItem { id: "startup", icon: "cog", i18n_key: "settings.nav_startup_apps" },
        ],
    },
    NavCategory {
        title_key: "settings.cat_system",
        items: &[
            NavItem { id: "env", icon: "sliders", i18n_key: "settings.nav_env" },
            NavItem { id: "certificates", icon: "key", i18n_key: "settings.nav_certificates" },
            NavItem { id: "system_update", icon: "history", i18n_key: "settings.nav_system_update" },
        ],
    },
];

const FOOTER_ITEM: NavItem = NavItem {
    id: "system",
    icon: "info",
    i18n_key: "settings.nav_about_system",
};

/// Instantiates a settings page widget by name on demand.
fn create_widget_page(name: &str) -> gtk4::Widget {
    match name {
        "wifi" => widgets::wifi::create_wifi_widget(),
        "vpn" => widgets::vpn::create_vpn_widget(),
        "hosts" => widgets::hosts::create_hosts_widget(),
        "env" => widgets::env::create_env_widget(),
        "bluetooth" => widgets::bluetooth::create_bluetooth_widget(),
        "displays" => widgets::displays::create_displays_widget(),
        "keybinds" => widgets::keybinds::create_keybinds_widget(),
        "appearance" => widgets::appearance::create_appearance_widget(),
        "apps" => widgets::apps::create_apps_widget(),
        "startup" => widgets::startup::create_startup_widget(),
        "certificates" => widgets::certificates::create_certificates_widget(),
        "system_update" => widgets::system_update::create_system_update_widget(),
        "system" => widgets::system_info::create_system_widget(),
        _ => gtk4::Box::new(gtk4::Orientation::Vertical, 0).upcast(),
    }
}

/// Ensures that a widget page is constructed and added to the content stack.
fn ensure_page_loaded(stack: &gtk4::Stack, name: &str) {
    if stack.child_by_name(name).is_none() {
        stack.add_named(&create_widget_page(name), Some(name));
    }
}

/// Finds and updates the Label text inside a sidebar Button (Button > Box > Label).
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

fn create_sidebar_category_header(key: &str) -> gtk4::Label {
    let lbl = gtk4::Label::new(Some(&babydra_common::i18n::t(key)));
    lbl.add_css_class("sidebar-section-label");
    lbl.set_halign(gtk4::Align::Start);
    lbl.set_margin_top(8);
    lbl.set_margin_bottom(2);
    lbl
}

pub fn build_main_window(app: &gtk4::Application) {
    let window = gtk4::ApplicationWindow::new(app);
    window.set_title(Some("Settings"));
    window.set_default_size(1000, 750);
    window.add_css_class("settings-window");

    let overlay = gtk4::Overlay::new();
    let main_layout = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);

    // ── Left: Sidebar Navigation Container ─────────────────────
    let sidebar_box = gtk4::Box::new(gtk4::Orientation::Vertical, 10);
    sidebar_box.add_css_class("sidebar");
    sidebar_box.set_width_request(240);
    sidebar_box.set_hexpand(false);
    sidebar_box.set_vexpand(true);
    sidebar_box.set_margin_top(8);
    sidebar_box.set_margin_bottom(8);
    sidebar_box.set_margin_start(8);

    // App Header Box
    let profile_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
    profile_box.set_margin_top(12);
    profile_box.set_margin_bottom(8);
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

    let profile_sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    profile_sep.add_css_class("profile-separator");
    sidebar_box.append(&profile_sep);

    // Navigation Scrolled List
    let sidebar_scroll = gtk4::ScrolledWindow::new();
    sidebar_scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    sidebar_scroll.set_vexpand(true);

    let nav_container = gtk4::Box::new(gtk4::Orientation::Vertical, 4);

    let category_labels: Rc<RefCell<Vec<(gtk4::Label, &'static str)>>> = Rc::new(RefCell::new(Vec::new()));
    let nav_buttons: Rc<RefCell<Vec<(&'static str, gtk4::Button, &'static str)>>> = Rc::new(RefCell::new(Vec::new()));

    for cat in NAV_CATEGORIES {
        let hdr = create_sidebar_category_header(cat.title_key);
        nav_container.append(&hdr);
        category_labels.borrow_mut().push((hdr, cat.title_key));

        for item in cat.items {
            let btn = babydra_utils::components::create_sidebar_item_button(
                &babydra_common::i18n::t(item.i18n_key),
                item.icon,
                "sidebar-item",
                || {},
            );
            btn.set_cursor_from_name(Some("pointer"));
            nav_container.append(&btn);
            nav_buttons.borrow_mut().push((item.id, btn, item.i18n_key));
        }
    }

    // Pinned Footer Item (About System)
    let btn_sys = babydra_utils::components::create_sidebar_item_button(
        &babydra_common::i18n::t(FOOTER_ITEM.i18n_key),
        FOOTER_ITEM.icon,
        "sidebar-item",
        || {},
    );
    btn_sys.add_css_class("active-nav");
    btn_sys.set_cursor_from_name(Some("pointer"));
    nav_buttons.borrow_mut().push((FOOTER_ITEM.id, btn_sys.clone(), FOOTER_ITEM.i18n_key));

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

    // Eagerly load initial default page only ("system")
    ensure_page_loaded(&content_stack, "system");
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

    // Wire navigation button clicks
    for (id, button, _) in nav_buttons.borrow().iter() {
        let name_str = id.to_string();
        let stack_c = content_stack.clone();
        let nav_buttons_c = nav_buttons.clone();
        let button_c = button.clone();

        button.connect_clicked(move |_| {
            for (_, b, _) in nav_buttons_c.borrow().iter() {
                b.remove_css_class("active-nav");
            }
            button_c.add_css_class("active-nav");

            ensure_page_loaded(&stack_c, &name_str);
            stack_c.set_visible_child_name(&name_str);
        });
    }

    // ── Rebuild UI Action (Language Toggle Handler) ─────────────
    let rebuild_action = gio::SimpleAction::new("rebuild-ui", None);
    {
        let content_stack_c = content_stack.clone();
        let app_title_lbl_c = app_title_lbl.clone();
        let app_sub_lbl_c = app_sub_lbl.clone();
        let category_labels_c = category_labels.clone();
        let nav_buttons_c = nav_buttons.clone();

        rebuild_action.connect_activate(move |_, _| {
            let stack = content_stack_c.clone();
            let title = app_title_lbl_c.clone();
            let sub = app_sub_lbl_c.clone();
            let cat_labels = category_labels_c.clone();
            let buttons = nav_buttons_c.clone();

            gtk4::glib::idle_add_local_once(move || {
                // 1. Update category header labels
                for (lbl, key) in cat_labels.borrow().iter() {
                    lbl.set_text(&babydra_common::i18n::t(key));
                }

                // 2. Update sidebar item labels
                for (_, btn, key) in buttons.borrow().iter() {
                    update_sidebar_label(btn, &babydra_common::i18n::t(key));
                }

                // 3. Update main header labels
                title.set_text(&babydra_common::i18n::t("settings.title"));
                sub.set_text(&babydra_common::i18n::t("settings.subtitle"));

                // 4. Clear unvisited cached pages and rebuild current active page
                if let Some(current_name) = stack.visible_child_name().map(|s| s.to_string()) {
                    while let Some(child) = stack.first_child() {
                        stack.remove(&child);
                    }
                    stack.add_named(&create_widget_page(&current_name), Some(&current_name));
                    stack.set_visible_child_name(&current_name);
                }
            });
        });
    }
    window.add_action(&rebuild_action);

    window.set_child(Some(&overlay));
    window.present();
}
