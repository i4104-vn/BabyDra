//! Native Arch Linux settings manager built with GTK4 + Rust.

use gtk4::prelude::*;

mod widgets;

const SHORTCUTS_HELP: &str = r#"
Bảng Phím tắt Hệ thống (Shortcuts)

Di chuyển nhanh:
  Alt + 1 : Cài đặt Wi-Fi
  Alt + 2 : Quản lý Bluetooth
  Alt + 3 : Mạng ảo VPN
  Alt + 4 : Giao diện & Hình nền
  Alt + 5 : Thông tin Hệ thống

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
        window.set_title(Some("BabyDra Settings"));
        window.set_default_size(900, 600);
        window.add_css_class("settings-window");

        let overlay = gtk4::Overlay::new();

        // Main sidebar + content layout split box
        let main_layout = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);

        // ── Left: Vertical Capsule Pill Navigation (Icon Only) ─────
        let nav_container = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
        nav_container.set_valign(gtk4::Align::Start);
        nav_container.set_margin_top(16);
        nav_container.set_margin_bottom(16);
        nav_container.set_margin_start(16);
        nav_container.add_css_class("settings-capsule-nav-vertical");
        main_layout.append(&nav_container);

        let btn_wifi = babydra_utils::components::create_icon_button("wifi", 18, &["settings-pill", "active-pill"], Some("Wi-Fi & Mạng"), || {});
        btn_wifi.set_cursor_from_name(Some("pointer"));

        let btn_bt = babydra_utils::components::create_icon_button("bluetooth", 18, &["settings-pill"], Some("Bluetooth"), || {});
        btn_bt.set_cursor_from_name(Some("pointer"));

        let btn_vpn = babydra_utils::components::create_icon_button("shield", 18, &["settings-pill"], Some("VPN & Mạng ảo"), || {});
        btn_vpn.set_cursor_from_name(Some("pointer"));

        let btn_app = babydra_utils::components::create_icon_button("display", 18, &["settings-pill"], Some("Giao diện & Hình nền"), || {});
        btn_app.set_cursor_from_name(Some("pointer"));

        let btn_sys = babydra_utils::components::create_icon_button("info", 18, &["settings-pill"], Some("Thông tin Hệ thống"), || {});
        btn_sys.set_cursor_from_name(Some("pointer"));

        nav_container.append(&btn_wifi);
        nav_container.append(&btn_bt);
        nav_container.append(&btn_vpn);
        nav_container.append(&btn_app);
        nav_container.append(&btn_sys);

        // Content Stack Panel
        let content_stack = gtk4::Stack::new();
        content_stack.set_transition_type(gtk4::StackTransitionType::Crossfade);
        content_stack.set_transition_duration(250);
        content_stack.set_hexpand(true);
        content_stack.set_vexpand(true);
        content_stack.add_css_class("settings-content");

        let wifi_widget = widgets::wifi::create_wifi_widget();
        let bt_widget = widgets::bluetooth::create_bluetooth_widget();
        let vpn_widget = widgets::vpn::create_vpn_widget();
        let app_widget = widgets::appearance::create_appearance_widget();
        let sys_widget = widgets::system_info::create_system_widget();

        content_stack.add_named(&wifi_widget, Some("wifi"));
        content_stack.add_named(&bt_widget, Some("bluetooth"));
        content_stack.add_named(&vpn_widget, Some("vpn"));
        content_stack.add_named(&app_widget, Some("appearance"));
        content_stack.add_named(&sys_widget, Some("system"));

        main_layout.append(&content_stack);
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

        // --- Capsule Navigation Handling ---
        let btn_wifi_c = btn_wifi.clone();
        let btn_bt_c = btn_bt.clone();
        let btn_vpn_c = btn_vpn.clone();
        let btn_app_c = btn_app.clone();
        let btn_sys_c = btn_sys.clone();

        let stack_c = content_stack.clone();

        let clear_active = move || {
            btn_wifi_c.remove_css_class("active-pill");
            btn_bt_c.remove_css_class("active-pill");
            btn_vpn_c.remove_css_class("active-pill");
            btn_app_c.remove_css_class("active-pill");
            btn_sys_c.remove_css_class("active-pill");
        };

        let clear1 = clear_active.clone();
        let btn_wifi_active = btn_wifi.clone();
        let stack1 = stack_c.clone();
        btn_wifi.connect_clicked(move |_| {
            clear1();
            btn_wifi_active.add_css_class("active-pill");
            stack1.set_visible_child_name("wifi");
        });

        let clear2 = clear_active.clone();
        let btn_bt_active = btn_bt.clone();
        let stack2 = stack_c.clone();
        btn_bt.connect_clicked(move |_| {
            clear2();
            btn_bt_active.add_css_class("active-pill");
            stack2.set_visible_child_name("bluetooth");
        });

        let clear3 = clear_active.clone();
        let btn_vpn_active = btn_vpn.clone();
        let stack3 = stack_c.clone();
        btn_vpn.connect_clicked(move |_| {
            clear3();
            btn_vpn_active.add_css_class("active-pill");
            stack3.set_visible_child_name("vpn");
        });

        let clear4 = clear_active.clone();
        let btn_app_active = btn_app.clone();
        let stack4 = stack_c.clone();
        btn_app.connect_clicked(move |_| {
            clear4();
            btn_app_active.add_css_class("active-pill");
            stack4.set_visible_child_name("appearance");
        });

        let clear5 = clear_active.clone();
        let btn_sys_active = btn_sys.clone();
        let stack5 = stack_c.clone();
        btn_sys.connect_clicked(move |_| {
            clear5();
            btn_sys_active.add_css_class("active-pill");
            stack5.set_visible_child_name("system");
        });

        // --- Shortcuts Keyboard Controls ---
        let key_controller = gtk4::EventControllerKey::new();
        let cheatsheet_box_key = cheatsheet_box.clone();

        let btn_wifi_k = btn_wifi.clone();
        let btn_bt_k = btn_bt.clone();
        let btn_vpn_k = btn_vpn.clone();
        let btn_app_k = btn_app.clone();
        let btn_sys_k = btn_sys.clone();

        key_controller.connect_key_pressed(move |_, keyval, _, state| {
            let is_alt = state.contains(gtk4::gdk::ModifierType::ALT_MASK);
            match keyval.name().as_deref() {
                Some("1") if is_alt => {
                    btn_wifi_k.emit_clicked();
                }
                Some("2") if is_alt => {
                    btn_bt_k.emit_clicked();
                }
                Some("3") if is_alt => {
                    btn_vpn_k.emit_clicked();
                }
                Some("4") if is_alt => {
                    btn_app_k.emit_clicked();
                }
                Some("5") if is_alt => {
                    btn_sys_k.emit_clicked();
                }
                Some("h") if is_alt => {
                    cheatsheet_box_key.set_visible(!cheatsheet_box_key.is_visible());
                }
                Some("question") => {
                    cheatsheet_box_key.set_visible(!cheatsheet_box_key.is_visible());
                }
                Some("Escape") => {
                    cheatsheet_box_key.set_visible(false);
                }
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
