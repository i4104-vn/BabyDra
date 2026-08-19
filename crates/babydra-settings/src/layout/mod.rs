//! Settings window layout builder with lazy page loading and dynamic navigation.

use gtk4::gio;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use crate::widgets;
use babydra_core::models::settings::nav::{NavCategory, NavItem};

const NAV_CATEGORIES: &[NavCategory] = &[
    NavCategory {
        title_key: "settings.cat_network",
        items: &[
            NavItem {
                id: "wifi",
                icon: "wifi",
                i18n_key: "settings.nav_wifi",
            },
            NavItem {
                id: "bluetooth",
                icon: "bluetooth",
                i18n_key: "settings.nav_bluetooth",
            },
            NavItem {
                id: "vpn",
                icon: "shield",
                i18n_key: "settings.nav_vpn",
            },
            NavItem {
                id: "certificates",
                icon: "key",
                i18n_key: "settings.nav_certificates",
            },
            NavItem {
                id: "hosts",
                icon: "file-text",
                i18n_key: "settings.nav_hosts",
            },
        ],
    },
    NavCategory {
        title_key: "settings.cat_apps",
        items: &[
            NavItem {
                id: "displays",
                icon: "desktop",
                i18n_key: "settings.nav_displays",
            },
            NavItem {
                id: "appearance",
                icon: "palette",
                i18n_key: "settings.nav_wallpaper_themes",
            },
            NavItem {
                id: "power",
                icon: "battery",
                i18n_key: "settings.nav_power",
            },
        ],
    },
    NavCategory {
        title_key: "settings.cat_system",
        items: &[
            NavItem {
                id: "startup",
                icon: "cog",
                i18n_key: "settings.nav_startup_apps",
            },
            NavItem {
                id: "apps",
                icon: "th-large",
                i18n_key: "settings.nav_installed_apps",
            },
            NavItem {
                id: "env",
                icon: "sliders",
                i18n_key: "settings.nav_env",
            },
            NavItem {
                id: "keybinds",
                icon: "cog",
                i18n_key: "settings.nav_keybinds",
            },
            NavItem {
                id: "system_update",
                icon: "history",
                i18n_key: "settings.nav_system_update",
            },
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
        "bluetooth" => widgets::bluetooth::create_bt_widget(),
        "displays" => widgets::displays::create_displays(),
        "power" => widgets::power::create_power_widget(),
        "keybinds" => widgets::keybinds::create_keybinds(),
        "appearance" => widgets::appearance::create_appearance(),
        "apps" => widgets::apps::create_apps_widget(),
        "startup" => widgets::startup::create_startup(),
        "certificates" => widgets::certificates::create_cert_widget(),
        "system_update" => widgets::system_update::create_update_widget(),
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

/// Creates a new `sidebar icon for item`.
pub mod sidebar;
pub use sidebar::*;

pub fn build_main_window(app: &gtk4::Application, initial_page: Option<&str>) {
    let window = gtk4::ApplicationWindow::new(app);
    window.set_title(Some("Settings"));
    window.set_icon_name(Some("babydra-settings"));
    window.set_default_size(1000, 750);
    window.add_css_class("settings-window");

    let target_page_id = initial_page.unwrap_or("system");

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

    let logo_img = babydra_ui_kit::ui::icon::get_icon("logo", 28);
    logo_img.set_pixel_size(28);
    logo_img.set_valign(gtk4::Align::Center);
    profile_box.append(&logo_img);

    let title_info_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    title_info_box.set_valign(gtk4::Align::Center);

    let app_title_lbl = gtk4::Label::new(Some(&babydra_core::i18n::trans("settings.title")));
    app_title_lbl.add_css_class("profile-user-name");
    app_title_lbl.set_halign(gtk4::Align::Start);

    let app_sub_lbl = gtk4::Label::new(Some(&babydra_core::i18n::trans("settings.subtitle")));
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

    let category_labels: Rc<RefCell<Vec<(gtk4::Label, &'static str)>>> =
        Rc::new(RefCell::new(Vec::new()));
    let nav_buttons: Rc<RefCell<Vec<(&'static str, gtk4::Button, &'static str, &'static str)>>> =
        Rc::new(RefCell::new(Vec::new()));

    for cat in NAV_CATEGORIES {
        let hdr = sidebar::create_sidebar_cat(cat.title_key);
        nav_container.append(&hdr);
        category_labels.borrow_mut().push((hdr, cat.title_key));

        for item in cat.items {
            let icon_w = sidebar::create_sidebar_icon(item.id, item.icon);
            let btn = babydra_ui_kit::components::create_sidebar_wbtn(
                &babydra_core::i18n::trans(item.i18n_key),
                &icon_w,
                "sidebar-item",
                || {},
            );
            btn.set_cursor_from_name(Some("pointer"));
            if item.id == target_page_id {
                btn.add_css_class("active-nav");
            }
            nav_container.append(&btn);
            nav_buttons
                .borrow_mut()
                .push((item.id, btn, item.i18n_key, item.icon));
        }
    }

    // Pinned Footer Item (About System)
    let sys_icon_w = sidebar::create_sidebar_icon(FOOTER_ITEM.id, FOOTER_ITEM.icon);
    let btn_sys = babydra_ui_kit::components::create_sidebar_wbtn(
        &babydra_core::i18n::trans(FOOTER_ITEM.i18n_key),
        &sys_icon_w,
        "sidebar-item",
        || {},
    );
    if FOOTER_ITEM.id == target_page_id {
        btn_sys.add_css_class("active-nav");
    }
    btn_sys.set_cursor_from_name(Some("pointer"));
    nav_buttons.borrow_mut().push((
        FOOTER_ITEM.id,
        btn_sys.clone(),
        FOOTER_ITEM.i18n_key,
        FOOTER_ITEM.icon,
    ));

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

    // Eagerly load target initial page
    ensure_page_loaded(&content_stack, target_page_id);
    content_stack.set_visible_child_name(target_page_id);

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

    // ── Global Loading Overlay ───────────────────────────────
    let loading_box = gtk4::Box::new(gtk4::Orientation::Vertical, 10);
    loading_box.set_valign(gtk4::Align::Center);
    loading_box.set_halign(gtk4::Align::Center);
    loading_box.set_vexpand(true);
    loading_box.set_hexpand(true);

    let spinner = gtk4::Spinner::new();
    spinner.set_size_request(48, 48);

    let loading_lbl = gtk4::Label::new(Some(&babydra_core::i18n::trans("settings.loading")));
    loading_lbl.add_css_class("settings-row-title");

    loading_box.append(&spinner);
    loading_box.append(&loading_lbl);

    let overlay_blocker = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    overlay_blocker.set_vexpand(true);
    overlay_blocker.set_hexpand(true);
    overlay_blocker.add_css_class("global-loading-blocker");
    overlay_blocker.append(&loading_box);
    overlay_blocker.set_visible(false); // Hidden by default

    // Consume all clicks so nothing underneath can be clicked
    let click_interceptor = gtk4::GestureClick::new();
    click_interceptor.connect_pressed(|_, _, _, _| {
        // Do nothing, just intercept
    });
    overlay_blocker.add_controller(click_interceptor);

    overlay.add_overlay(&overlay_blocker);

    // Wire navigation button clicks
    for (id, button, _, _) in nav_buttons.borrow().iter() {
        let name_str = id.to_string();
        let stack_c = content_stack.clone();
        let nav_buttons_c = nav_buttons.clone();
        let button_c = button.clone();

        button.connect_clicked(move |_| {
            for (_, b, _, _) in nav_buttons_c.borrow().iter() {
                b.remove_css_class("active-nav");
            }
            button_c.add_css_class("active-nav");

            ensure_page_loaded(&stack_c, &name_str);
            stack_c.set_visible_child_name(&name_str);

            // Refresh dynamic icons and state when switching navigation tabs
            refresh_sidebar(&nav_buttons_c);
        });
    }

    // ── Refresh Sidebar Action ──────────────────────────────────
    let refresh_sidebar_action = gio::SimpleAction::new("refresh-sidebar", None);
    {
        let nav_buttons_c = nav_buttons.clone();
        refresh_sidebar_action.connect_activate(move |_, _| {
            let buttons = nav_buttons_c.clone();
            gtk4::glib::idle_add_local_once(move || {
                refresh_sidebar(&buttons);
            });
        });
    }
    window.add_action(&refresh_sidebar_action);

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
                    lbl.set_text(&babydra_core::i18n::trans(key));
                }

                // 2. Update sidebar item icons and labels
                refresh_sidebar(&buttons);

                // 3. Update main header labels
                title.set_text(&babydra_core::i18n::trans("settings.title"));
                sub.set_text(&babydra_core::i18n::trans("settings.subtitle"));

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

    // ── Loading Action ─────────────
    let show_loading_action = gio::SimpleAction::new_stateful(
        "show-loading",
        Some(gtk4::glib::VariantTy::BOOLEAN),
        &false.to_variant(),
    );
    let blocker_c = overlay_blocker.clone();
    let spinner_c = spinner.clone();
    show_loading_action.connect_activate(move |action, param| {
        if let Some(val) = param.and_then(|v| v.get::<bool>()) {
            blocker_c.set_visible(val);
            if val {
                spinner_c.start();
            } else {
                spinner_c.stop();
            }
            action.set_state(&val.to_variant());
        }
    });
    window.add_action(&show_loading_action);

    window.set_child(Some(&overlay));
    window.present();
}
