//! Settings window layout builder with lazy page loading and dynamic navigation.

use gtk4::gio;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use crate::widgets;
use babydra_core::models::settings::nav::{NavCategory, NavItem};

/// Returns whether the first-login page still needs to be shown.
fn is_first_login() -> bool {
    !babydra_core::load_babydra_config().first_login
}

/// Persists the first-login marker in `~/.babydra/babydra.conf`.
fn mark_first_login_complete() {
    let mut config = babydra_core::load_babydra_config();
    if !config.first_login {
        config.first_login = true;
        babydra_core::save_babydra_config(&config);
    }
}

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
        "system" => widgets::system_info::create_system_widget(),
        "general" | "system_update" | "recovery" => widgets::general::create_general_widget(),
        _ => gtk4::Box::new(gtk4::Orientation::Vertical, 0).upcast(),
    }
}

/// Ensures that a widget page is constructed and added to the content stack.
fn ensure_page_loaded(stack: &gtk4::Stack, name: &str) {
    if stack.child_by_name(name).is_none() {
        let page = create_widget_page(name);
        page.set_hexpand(true);
        page.set_vexpand(true);
        page.set_halign(gtk4::Align::Fill);
        page.set_valign(gtk4::Align::Fill);
        stack.add_named(&page, Some(name));
    }
}

/// Creates a new `sidebar icon for item`.
pub mod sidebar;
pub use sidebar::*;

fn attach_sidebar_popover(
    btn: &gtk4::Button,
    title_key: &'static str,
    popover_labels: &Rc<RefCell<Vec<(gtk4::Label, &'static str)>>>,
    active_popover: &Rc<RefCell<Option<gtk4::Popover>>>,
) -> gtk4::Popover {
    let popover = gtk4::Popover::new();
    popover.set_parent(btn);
    popover.set_position(gtk4::PositionType::Right);
    popover.set_has_arrow(true);
    popover.set_autohide(false);
    popover.set_can_target(false);
    popover.add_css_class("sidebar-item-popover");

    let lbl = gtk4::Label::new(Some(&babydra_core::i18n::trans(title_key)));
    lbl.add_css_class("sidebar-item-popover-label");
    lbl.set_halign(gtk4::Align::Center);
    lbl.set_valign(gtk4::Align::Center);
    popover.set_child(Some(&lbl));
    popover_labels.borrow_mut().push((lbl, title_key));

    let motion = gtk4::EventControllerMotion::new();
    {
        let p_enter = popover.clone();
        let active_pop_c = active_popover.clone();
        motion.connect_enter(move |_, _, _| {
            if let Some(prev) = active_pop_c.borrow_mut().take() {
                prev.popdown();
            }
            *active_pop_c.borrow_mut() = Some(p_enter.clone());
            p_enter.popup();
        });
    }
    {
        let p_leave = popover.clone();
        let active_pop_c = active_popover.clone();
        motion.connect_leave(move |_| {
            p_leave.popdown();
            let mut cur = active_pop_c.borrow_mut();
            if let Some(ref c) = *cur {
                if c == &p_leave {
                    *cur = None;
                }
            }
        });
    }
    btn.add_controller(motion);

    popover
}

fn create_sidebar_item_btn(
    id: &'static str,
    icon_widget: &gtk4::Widget,
    i18n_key: &'static str,
    target_page_id: &str,
    popover_labels: &Rc<RefCell<Vec<(gtk4::Label, &'static str)>>>,
    active_popover: &Rc<RefCell<Option<gtk4::Popover>>>,
) -> gtk4::Button {
    let btn = gtk4::Button::new();
    btn.add_css_class("sidebar-item");
    btn.add_css_class("sidebar-icon-btn");
    btn.add_css_class("flat");
    btn.set_cursor_from_name(Some("pointer"));
    btn.set_halign(gtk4::Align::Center);
    btn.set_valign(gtk4::Align::Center);
    btn.set_size_request(44, 44);

    let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    hbox.set_halign(gtk4::Align::Center);
    hbox.set_valign(gtk4::Align::Center);
    icon_widget.set_valign(gtk4::Align::Center);
    icon_widget.set_halign(gtk4::Align::Center);
    hbox.append(icon_widget);

    let hidden_lbl = gtk4::Label::new(Some(&babydra_core::i18n::trans(i18n_key)));
    hidden_lbl.set_visible(false);
    hbox.append(&hidden_lbl);

    btn.set_child(Some(&hbox));

    if id == target_page_id {
        btn.add_css_class("active-nav");
    }

    attach_sidebar_popover(&btn, i18n_key, popover_labels, active_popover);

    btn
}

pub fn build_main_window(app: &gtk4::Application, initial_page: Option<&str>) {
    let window = gtk4::ApplicationWindow::new(app);
    window.set_title(Some("Settings"));
    window.set_icon_name(Some("babydra-settings"));
    window.set_default_size(1000, 750);
    window.add_css_class("settings-window");

    // The first login after installation opens the About page once.
    let is_first = is_first_login();
    let target_page_id = if is_first {
        "system".to_string() // About page
    } else {
        let initial = initial_page.unwrap_or("system");
        if initial == "system_update" || initial == "recovery" {
            "general".to_string()
        } else {
            initial.to_string()
        }
    };

    let overlay = gtk4::Overlay::new();
    let main_layout = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    // `main_layout` is an overlay child.  Overlay children otherwise keep
    // their natural size on some GTK backends, which can collapse Settings
    // to an empty/partially visible surface when the window is resized.
    main_layout.set_hexpand(true);
    main_layout.set_vexpand(true);
    main_layout.set_halign(gtk4::Align::Fill);
    main_layout.set_valign(gtk4::Align::Fill);

    let backdrop = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    backdrop.set_hexpand(true);
    backdrop.set_vexpand(true);
    backdrop.set_can_target(false);
    backdrop.add_css_class("settings-surface-tint");

    // ── Left: Slim Sidebar Navigation Dock ─────────────────────
    let sidebar_box = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
    sidebar_box.add_css_class("sidebar");
    sidebar_box.set_width_request(68);
    sidebar_box.set_hexpand(false);
    sidebar_box.set_vexpand(true);
    sidebar_box.set_margin_top(12);
    sidebar_box.set_margin_bottom(12);
    sidebar_box.set_margin_start(10);
    sidebar_box.set_margin_end(2);
    sidebar_box.set_halign(gtk4::Align::Center);

    let active_popover: Rc<RefCell<Option<gtk4::Popover>>> = Rc::new(RefCell::new(None));
    let popover_labels: Rc<RefCell<Vec<(gtk4::Label, &'static str)>>> =
        Rc::new(RefCell::new(Vec::new()));

    // App Header Button (navigates to Generic Settings)
    let profile_btn = gtk4::Button::new();
    profile_btn.add_css_class("sidebar-header-btn");
    profile_btn.add_css_class("flat");
    profile_btn.set_cursor_from_name(Some("pointer"));
    profile_btn.set_halign(gtk4::Align::Center);
    profile_btn.set_valign(gtk4::Align::Center);
    profile_btn.set_size_request(44, 44);
    if target_page_id == "general" {
        profile_btn.add_css_class("active-nav");
    }

    let logo_img = babydra_ui_kit::ui::icon::get_icon("logo", 26);
    logo_img.set_pixel_size(26);
    logo_img.set_valign(gtk4::Align::Center);
    logo_img.set_halign(gtk4::Align::Center);
    profile_btn.set_child(Some(&logo_img));

    attach_sidebar_popover(
        &profile_btn,
        "settings.title",
        &popover_labels,
        &active_popover,
    );
    sidebar_box.append(&profile_btn);

    let profile_sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    profile_sep.add_css_class("profile-separator");
    sidebar_box.append(&profile_sep);

    // Navigation Scrolled List
    let sidebar_scroll = gtk4::ScrolledWindow::new();
    sidebar_scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    sidebar_scroll.set_vexpand(true);
    sidebar_scroll.set_halign(gtk4::Align::Center);

    let nav_container = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
    nav_container.set_halign(gtk4::Align::Center);

    type NavButtonItem = (&'static str, gtk4::Button, &'static str, &'static str);
    let nav_buttons: Rc<RefCell<Vec<NavButtonItem>>> = Rc::new(RefCell::new(Vec::new()));
    let sidebar_status = Rc::new(RefCell::new(sidebar::SidebarStatus::default()));

    for (cat_idx, cat) in NAV_CATEGORIES.iter().enumerate() {
        if cat_idx > 0 {
            let sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
            sep.add_css_class("sidebar-nav-separator");
            nav_container.append(&sep);
        }

        for item in cat.items {
            let icon_w = sidebar::create_sidebar_icon(item.id, item.icon);
            let btn = create_sidebar_item_btn(
                item.id,
                &icon_w,
                item.i18n_key,
                &target_page_id,
                &popover_labels,
                &active_popover,
            );
            nav_container.append(&btn);
            nav_buttons
                .borrow_mut()
                .push((item.id, btn, item.i18n_key, item.icon));
        }
    }

    // Pinned Footer Item (About System)
    let sys_icon_w = sidebar::create_sidebar_icon(FOOTER_ITEM.id, FOOTER_ITEM.icon);
    let btn_sys = create_sidebar_item_btn(
        FOOTER_ITEM.id,
        &sys_icon_w,
        FOOTER_ITEM.i18n_key,
        &target_page_id,
        &popover_labels,
        &active_popover,
    );
    nav_buttons.borrow_mut().push((
        FOOTER_ITEM.id,
        btn_sys.clone(),
        FOOTER_ITEM.i18n_key,
        FOOTER_ITEM.icon,
    ));

    let footer_sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    footer_sep.add_css_class("sidebar-nav-separator");

    let footer_box = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
    footer_box.set_halign(gtk4::Align::Center);
    footer_box.set_margin_bottom(6);
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
    content_stack.set_halign(gtk4::Align::Fill);
    content_stack.set_valign(gtk4::Align::Fill);
    content_stack.add_css_class("settings-content");

    // Eagerly load target initial page
    ensure_page_loaded(&content_stack, &target_page_id);
    content_stack.set_visible_child_name(&target_page_id);

    let right_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    right_box.add_css_class("settings-content-card");
    right_box.set_hexpand(true);
    right_box.set_vexpand(true);
    right_box.set_halign(gtk4::Align::Fill);
    right_box.set_valign(gtk4::Align::Fill);
    right_box.set_margin_top(12);
    right_box.set_margin_bottom(12);
    right_box.set_margin_start(6);
    right_box.set_margin_end(12);
    // Every settings page owns its scrolling viewport.  Adding a second
    // non-scrollable ScrolledWindow here makes the Stack measure at zero on
    // GTK4 versions that use the viewport's minimum size during allocation.
    right_box.append(&content_stack);

    main_layout.append(&right_box);
    overlay.set_child(Some(&backdrop));
    overlay.add_overlay(&main_layout);

    // ── Global Loading Overlay ───────────────────────────────
    let loading_card = babydra_ui_kit::components::create_loading_card(72);
    loading_card.set_valign(gtk4::Align::Center);
    loading_card.set_halign(gtk4::Align::Center);
    loading_card.set_size_request(240, 240);

    let overlay_blocker = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    overlay_blocker.set_vexpand(true);
    overlay_blocker.set_hexpand(true);
    overlay_blocker.add_css_class("global-loading-blocker");
    overlay_blocker.append(&loading_card);
    overlay_blocker.set_visible(false); // Hidden by default

    // Consume all clicks so nothing underneath can be clicked
    let click_interceptor = gtk4::GestureClick::new();
    click_interceptor.connect_pressed(|_, _, _, _| {
        // Do nothing, just intercept
    });
    overlay_blocker.add_controller(click_interceptor);

    overlay.add_overlay(&overlay_blocker);

    // Wire navigation button clicks
    {
        let stack_c = content_stack.clone();
        let nav_buttons_c = nav_buttons.clone();
        let profile_btn_c = profile_btn.clone();
        let active_pop_c = active_popover.clone();
        profile_btn.connect_clicked(move |_| {
            if let Some(p) = active_pop_c.borrow_mut().take() {
                p.popdown();
            }
            for (_, b, _, _) in nav_buttons_c.borrow().iter() {
                b.remove_css_class("active-nav");
            }
            profile_btn_c.add_css_class("active-nav");

            ensure_page_loaded(&stack_c, "general");
            stack_c.set_visible_child_name("general");
        });
    }

    for (id, button, _, _) in nav_buttons.borrow().iter() {
        let name_str = id.to_string();
        let stack_c = content_stack.clone();
        let nav_buttons_c = nav_buttons.clone();
        let button_c = button.clone();
        let profile_btn_c = profile_btn.clone();
        let active_pop_c = active_popover.clone();

        button.connect_clicked(move |_| {
            if let Some(p) = active_pop_c.borrow_mut().take() {
                p.popdown();
            }
            profile_btn_c.remove_css_class("active-nav");
            for (_, b, _, _) in nav_buttons_c.borrow().iter() {
                b.remove_css_class("active-nav");
            }
            button_c.add_css_class("active-nav");

            ensure_page_loaded(&stack_c, &name_str);
            stack_c.set_visible_child_name(&name_str);
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
        let nav_buttons_c = nav_buttons.clone();
        let popover_labels_c = popover_labels.clone();

        rebuild_action.connect_activate(move |_, _| {
            let stack = content_stack_c.clone();
            let buttons = nav_buttons_c.clone();
            let pop_labels = popover_labels_c.clone();

            gtk4::glib::idle_add_local_once(move || {
                // 1. Update popover tooltip labels
                for (lbl, key) in pop_labels.borrow().iter() {
                    lbl.set_text(&babydra_core::i18n::trans(key));
                }

                // 2. Update sidebar item icons and labels
                refresh_sidebar(&buttons);

                // 3. Clear unvisited cached pages and rebuild current active page
                if let Some(current_name) = stack.visible_child_name().map(|s| s.to_string()) {
                    while let Some(child) = stack.first_child() {
                        stack.remove(&child);
                    }
                    ensure_page_loaded(&stack, &current_name);
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
    show_loading_action.connect_activate(move |action, param| {
        if let Some(val) = param.and_then(|v| v.get::<bool>()) {
            blocker_c.set_visible(val);
            action.set_state(&val.to_variant());
        }
    });
    window.add_action(&show_loading_action);

    window.set_child(Some(&overlay));
    sidebar::start_status_updates(nav_buttons.clone(), sidebar_status);
    window.present();

    if is_first {
        mark_first_login_complete();
    }
}
