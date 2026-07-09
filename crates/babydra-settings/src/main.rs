//! Native Arch Linux settings manager built with GTK4 + Rust.

use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

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
        babydra_common::init_theme();

        let window = gtk4::ApplicationWindow::new(app);
        window.set_title(Some("BabyDra Settings"));
        window.set_default_size(900, 600);
        window.add_css_class("settings-window");

        let overlay = gtk4::Overlay::new();

        // Main sidebar + content layout split box
        let main_layout = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);

        // Sidebar Panel
        let sidebar = gtk4::Box::new(gtk4::Orientation::Vertical, 12);
        sidebar.add_css_class("settings-sidebar");
        sidebar.set_width_request(240);

        let sidebar_list = gtk4::ListBox::new();
        sidebar_list.set_selection_mode(gtk4::SelectionMode::Single);
        sidebar.append(&sidebar_list);

        let add_sidebar_row = |label: &str, icon_name: &str| -> gtk4::Box {
            let row_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
            row_box.add_css_class("settings-sidebar-row");
            let icon = gtk4::Image::from_icon_name(icon_name);
            row_box.append(&icon);
            let lbl = gtk4::Label::new(Some(label));
            row_box.append(&lbl);
            row_box
        };

        let wifi_row = add_sidebar_row("Wi-Fi & Mạng", "network-wireless-symbolic");
        let bt_row = add_sidebar_row("Bluetooth", "bluetooth-active-symbolic");
        let vpn_row = add_sidebar_row("VPN & Mạng ảo", "network-vpn-symbolic");
        let app_row = add_sidebar_row("Giao diện & Hình nền", "preferences-desktop-wallpaper-symbolic");
        let sys_row = add_sidebar_row("Hệ thống", "preferences-system-symbolic");

        sidebar_list.append(&wifi_row);
        sidebar_list.append(&bt_row);
        sidebar_list.append(&vpn_row);
        sidebar_list.append(&app_row);
        sidebar_list.append(&sys_row);

        main_layout.append(&sidebar);

        // Content Stack Panel
        let content_stack = gtk4::Stack::new();
        content_stack.set_transition_type(gtk4::StackTransitionType::SlideLeftRight);
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

        // --- Sidebar Selection Handling ---
        let content_stack_clone = content_stack.clone();
        sidebar_list.connect_row_selected(move |_, row| {
            if let Some(r) = row {
                match r.index() {
                    0 => content_stack_clone.set_visible_child_name("wifi"),
                    1 => content_stack_clone.set_visible_child_name("bluetooth"),
                    2 => content_stack_clone.set_visible_child_name("vpn"),
                    3 => content_stack_clone.set_visible_child_name("appearance"),
                    4 => content_stack_clone.set_visible_child_name("system"),
                    _ => {}
                }
            }
        });

        // Select Wi-Fi by default
        sidebar_list.select_row(sidebar_list.row_at_index(0).as_ref());

        // --- Shortcuts Keyboard Controls ---
        let key_controller = gtk4::EventControllerKey::new();
        let sidebar_list_key = sidebar_list.clone();
        let cheatsheet_box_key = cheatsheet_box.clone();

        key_controller.connect_key_pressed(move |_, keyval, _, state| {
            let is_alt = state.contains(gtk4::gdk::ModifierType::ALT_MASK);
            match keyval.name().as_deref() {
                Some("1") if is_alt => {
                    sidebar_list_key.select_row(sidebar_list_key.row_at_index(0).as_ref());
                }
                Some("2") if is_alt => {
                    sidebar_list_key.select_row(sidebar_list_key.row_at_index(1).as_ref());
                }
                Some("3") if is_alt => {
                    sidebar_list_key.select_row(sidebar_list_key.row_at_index(2).as_ref());
                }
                Some("4") if is_alt => {
                    sidebar_list_key.select_row(sidebar_list_key.row_at_index(3).as_ref());
                }
                Some("5") if is_alt => {
                    sidebar_list_key.select_row(sidebar_list_key.row_at_index(4).as_ref());
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
