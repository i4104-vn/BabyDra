//! Native Arch Linux settings manager built with GTK4 + Rust.

use gtk4::prelude::*;

mod widgets;

const SHORTCUTS_HELP: &str = r#"
Bảng Phím tắt Hệ thống (Shortcuts)

Di chuyển nhanh (Alt + Phím):
  Alt + 1 : Wi-Fi
  Alt + 2 : VPN
  Alt + 3 : Bluetooth
  Alt + 4 : Wallpaper
  Alt + 5 : Themes
  Alt + 6 : Displays
  Alt + 7 : Installed Apps
  Alt + 8 : Startup Apps
  Alt + - : System Update
  Alt + = : About System

Trợ giúp:
  ? / Alt + H : Xem bảng phím tắt này
  Esc         : Đóng bảng phím tắt
"#;


fn main() {
    let app = gtk4::Application::new(
        Some("com.babydra.settings"),
        Default::default(),
    );

    app.connect_activate(move |app| {
        // Load custom styles
        babydra_utils::ui::theme::init_theme();

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

        // 1. App Title Header Box (Matching Image 1)
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

        let app_title_lbl = gtk4::Label::new(Some("Settings"));
        app_title_lbl.add_css_class("profile-user-name");
        app_title_lbl.set_halign(gtk4::Align::Start);

        let app_sub_lbl = gtk4::Label::new(Some("i4arch system settings"));
        app_sub_lbl.add_css_class("settings-row-desc");
        app_sub_lbl.set_halign(gtk4::Align::Start);

        title_info_box.append(&app_title_lbl);
        title_info_box.append(&app_sub_lbl);
        profile_box.append(&title_info_box);
        sidebar_box.append(&profile_box);

        // Search Input Box (Matching Image 1: Q Search settings... (Ctrl+/))
        let search_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
        search_box.set_margin_start(12);
        search_box.set_margin_end(12);
        search_box.set_margin_top(4);
        search_box.set_margin_bottom(8);

        let search_entry = gtk4::Entry::new();
        search_entry.set_placeholder_text(Some("Search settings... (Ctrl+/)"));
        search_entry.add_css_class("sidebar-search-entry");
        search_entry.set_hexpand(true);
        search_box.append(&search_entry);
        sidebar_box.append(&search_box);

        let profile_sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
        profile_sep.add_css_class("profile-separator");
        sidebar_box.append(&profile_sep);

        // 2. Navigation Scrolled List
        let sidebar_scroll = gtk4::ScrolledWindow::new();
        sidebar_scroll.set_hscrollbar_policy(gtk4::PolicyType::Never);
        sidebar_scroll.set_vexpand(true);

        let nav_container = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
        nav_container.set_margin_start(4);
        nav_container.set_margin_end(4);

        let btn_wifi = babydra_utils::components::create_sidebar_item_button("Wi-Fi", "wifi", "sidebar-item", || {});
        btn_wifi.set_cursor_from_name(Some("pointer"));

        let btn_vpn = babydra_utils::components::create_sidebar_item_button("VPN", "shield", "sidebar-item", || {});
        btn_vpn.set_cursor_from_name(Some("pointer"));

        let btn_bt = babydra_utils::components::create_sidebar_item_button("Bluetooth", "bluetooth", "sidebar-item", || {});
        btn_bt.set_cursor_from_name(Some("pointer"));

        let btn_app = babydra_utils::components::create_sidebar_item_button("Wallpaper", "palette", "sidebar-item", || {});
        btn_app.set_cursor_from_name(Some("pointer"));

        let btn_themes = babydra_utils::components::create_sidebar_item_button("Themes", "sliders", "sidebar-item", || {});
        btn_themes.set_cursor_from_name(Some("pointer"));

        let btn_displays = babydra_utils::components::create_sidebar_item_button("Displays", "desktop", "sidebar-item", || {});
        btn_displays.set_cursor_from_name(Some("pointer"));

        let btn_apps = babydra_utils::components::create_sidebar_item_button("Installed Apps", "th-large", "sidebar-item", || {});
        btn_apps.set_cursor_from_name(Some("pointer"));

        let btn_startup = babydra_utils::components::create_sidebar_item_button("Startup Apps", "cog", "sidebar-item", || {});
        btn_startup.set_cursor_from_name(Some("pointer"));

        let btn_update = babydra_utils::components::create_sidebar_item_button("System Update", "history", "sidebar-item", || {});
        btn_update.set_cursor_from_name(Some("pointer"));

        let btn_sys = babydra_utils::components::create_sidebar_item_button("About System", "info", "sidebar-item", || {});
        btn_sys.add_css_class("active-nav");
        btn_sys.set_cursor_from_name(Some("pointer"));

        nav_container.append(&btn_wifi);
        nav_container.append(&btn_vpn);
        nav_container.append(&btn_bt);
        nav_container.append(&btn_app);
        nav_container.append(&btn_themes);
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

        let wifi_widget = widgets::wifi::create_wifi_widget();
        let vpn_widget = widgets::vpn::create_vpn_widget();
        let bt_widget = widgets::bluetooth::create_bluetooth_widget();
        let app_widget = widgets::appearance::create_appearance_widget();
        let themes_widget = widgets::themes::create_themes_widget();
        let displays_widget = widgets::displays::create_displays_widget();
        let apps_widget = widgets::apps::create_apps_widget();
        let startup_widget = widgets::startup::create_startup_widget();
        let update_widget = widgets::system_update::create_system_update_widget();
        let sys_widget = widgets::system_info::create_system_widget();

        content_stack.add_named(&wifi_widget, Some("wifi"));
        content_stack.add_named(&vpn_widget, Some("vpn"));
        content_stack.add_named(&bt_widget, Some("bluetooth"));
        content_stack.add_named(&app_widget, Some("appearance"));
        content_stack.add_named(&themes_widget, Some("themes"));
        content_stack.add_named(&displays_widget, Some("displays"));
        content_stack.add_named(&apps_widget, Some("apps"));
        content_stack.add_named(&startup_widget, Some("startup"));
        content_stack.add_named(&update_widget, Some("system_update"));
        content_stack.add_named(&sys_widget, Some("system"));

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

        // --- Cheatsheet Dialog Overlay ---
        let cheatsheet_box = gtk4::Box::new(gtk4::Orientation::Vertical, 16);
        cheatsheet_box.add_css_class("cheatsheet-overlay");
        cheatsheet_box.set_halign(gtk4::Align::Center);
        cheatsheet_box.set_valign(gtk4::Align::Center);
        cheatsheet_box.set_visible(false);

        let cheatsheet_title = gtk4::Label::new(Some("Bảng phím tắt cài đặt"));
        cheatsheet_title.add_css_class("cheatsheet-title");
        cheatsheet_box.append(&cheatsheet_title);

        let cheatsheet_lbl = gtk4::Label::new(Some(SHORTCUTS_HELP));
        cheatsheet_lbl.add_css_class("cheatsheet-value");
        cheatsheet_lbl.set_justify(gtk4::Justification::Left);
        cheatsheet_box.append(&cheatsheet_lbl);

        let close_btn = gtk4::Button::with_label("Tôi đã rõ!");
        close_btn.set_halign(gtk4::Align::Center);
        close_btn.add_css_class("suggested-action");
        cheatsheet_box.append(&close_btn);

        overlay.add_overlay(&cheatsheet_box);

        // --- Navigation Active Handling ---
        let all_btns = vec![
            ("wifi", btn_wifi.clone()),
            ("vpn", btn_vpn.clone()),
            ("bluetooth", btn_bt.clone()),
            ("appearance", btn_app.clone()),
            ("themes", btn_themes.clone()),
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

        // --- Shortcuts Keyboard Controls ---
        let key_controller = gtk4::EventControllerKey::new();
        let cheatsheet_box_key = cheatsheet_box.clone();

        let btn_wifi_k = btn_wifi.clone();
        let btn_vpn_k = btn_vpn.clone();
        let btn_bt_k = btn_bt.clone();
        let btn_app_k = btn_app.clone();
        let btn_themes_k = btn_themes.clone();
        let btn_displays_k = btn_displays.clone();
        let btn_apps_k = btn_apps.clone();
        let btn_startup_k = btn_startup.clone();
        let btn_update_k = btn_update.clone();
        let btn_sys_k = btn_sys.clone();

        key_controller.connect_key_pressed(move |_, keyval, _, state| {
            let is_alt = state.contains(gtk4::gdk::ModifierType::ALT_MASK);
            match keyval.name().as_deref() {
                Some("1") if is_alt => { btn_wifi_k.emit_clicked(); }
                Some("2") if is_alt => { btn_vpn_k.emit_clicked(); }
                Some("3") if is_alt => { btn_bt_k.emit_clicked(); }
                Some("4") if is_alt => { btn_app_k.emit_clicked(); }
                Some("5") if is_alt => { btn_themes_k.emit_clicked(); }
                Some("6") if is_alt => { btn_displays_k.emit_clicked(); }
                Some("7") if is_alt => { btn_apps_k.emit_clicked(); }
                Some("8") if is_alt => { btn_startup_k.emit_clicked(); }
                Some("minus") if is_alt => { btn_update_k.emit_clicked(); }
                Some("equal") if is_alt => { btn_sys_k.emit_clicked(); }
                Some("h") if is_alt => { cheatsheet_box_key.set_visible(!cheatsheet_box_key.is_visible()); }
                Some("question") => { cheatsheet_box_key.set_visible(!cheatsheet_box_key.is_visible()); }
                Some("Escape") => { cheatsheet_box_key.set_visible(false); }
                _ => {}
            }
            gtk4::glib::Propagation::Proceed
        });


        window.add_controller(key_controller);

        let cheatsheet_close = cheatsheet_box.clone();
        close_btn.connect_clicked(move |_| {
            cheatsheet_close.set_visible(false);
        });

        window.set_child(Some(&overlay));
        window.present();
    });


    app.run_with_args(&["babydra-settings"]);
}
