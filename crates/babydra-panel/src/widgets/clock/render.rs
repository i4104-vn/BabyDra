use gtk4::prelude::*;
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

/// Builds the `clock ui` UI.
pub fn build_clock_ui() -> (gtk4::Button, gtk4::Label, gtk4::Widget) {
    let clock_button = gtk4::Button::new();
    clock_button.add_css_class("panel-clock-btn");

    let clock_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    clock_box.set_valign(gtk4::Align::Center);
    clock_box.set_halign(gtk4::Align::Center);

    let overlay = gtk4::Overlay::new();
    overlay.set_valign(gtk4::Align::Center);
    overlay.set_halign(gtk4::Align::Center);

    let bell_icon = babydra_ui_kit::ui::icon::get_icon("bell", 14);
    bell_icon.add_css_class("clock-bell-icon");
    overlay.set_child(Some(&bell_icon));

    let red_dot = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    red_dot.add_css_class("clock-bell-dot");
    red_dot.set_valign(gtk4::Align::Start);
    red_dot.set_halign(gtk4::Align::End);
    overlay.add_overlay(&red_dot);

    let clock_label = gtk4::Label::new(None);
    clock_label.add_css_class("panel-clock");

    clock_box.append(&clock_label);
    clock_box.append(&overlay);

    clock_button.set_child(Some(&clock_box));

    (clock_button, clock_label, red_dot.upcast::<gtk4::Widget>())
}

/// Builds the `calendar window ui` UI.
pub fn build_calendar_window_ui(
    app: &gtk4::Application,
) -> (
    gtk4::ApplicationWindow,
    gtk4::Box,
    gtk4::Label,
    gtk4::Label,
    gtk4::Calendar,
    gtk4::Button,
    gtk4::Box,
) {
    let c_win = gtk4::ApplicationWindow::new(app);
    babydra_ui_kit::ui::theme::apply_theme_class(&c_win);
    c_win.init_layer_shell();
    c_win.set_layer(Layer::Overlay);
    c_win.set_keyboard_mode(KeyboardMode::OnDemand);

    // Anchor to all 4 edges to cover the entire screen transparently
    c_win.set_anchor(Edge::Top, true);
    c_win.set_anchor(Edge::Bottom, true);
    c_win.set_anchor(Edge::Left, true);
    c_win.set_anchor(Edge::Right, true);
    c_win.add_css_class("calendar-window");

    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 12);
    main_box.add_css_class("calendar-box");
    main_box.set_vexpand(true);
    main_box.set_valign(gtk4::Align::Fill);
    main_box.set_halign(gtk4::Align::End);
    main_box.set_width_request(360);
    main_box.set_margin_top(6);
    main_box.set_margin_bottom(10);
    main_box.set_margin_end(12);

    // 1. Header: Date on left
    let top_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    top_row.add_css_class("calendar-top-row");

    let date_label = gtk4::Label::new(None);
    date_label.add_css_class("calendar-top-date");
    date_label.set_halign(gtk4::Align::Start);
    date_label.set_hexpand(true);

    top_row.append(&date_label);
    main_box.append(&top_row);

    // Dummy time label to satisfy setup_notifications_list signature without displaying it
    let dummy_time = gtk4::Label::new(None);

    // 3. Calendar widget
    let calendar = gtk4::Calendar::new();
    calendar.add_css_class("calendar-widget");
    main_box.append(&calendar);

    // 5. Notifications Header
    let notif_header = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    notif_header.set_hexpand(true);

    let notif_title = gtk4::Label::new(Some(&babydra_core::i18n::t("panel.notifications")));
    notif_title.add_css_class("notif-panel-title");
    notif_title.set_halign(gtk4::Align::Start);
    notif_title.set_hexpand(true);

    let clear_btn = gtk4::Button::with_label(&babydra_core::i18n::t("panel.clear_all"));
    clear_btn.add_css_class("clear-all-btn");
    clear_btn.set_halign(gtk4::Align::End);

    notif_header.append(&notif_title);
    notif_header.append(&clear_btn);
    main_box.append(&notif_header);

    // 6. Notifications Scrolled Window & List Box
    let scrolled_win = gtk4::ScrolledWindow::new();
    scrolled_win.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scrolled_win.set_vexpand(true);
    scrolled_win.set_hexpand(true);
    scrolled_win.add_css_class("notif-scroll");

    let notif_stack = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    notif_stack.add_css_class("notif-stack-box");
    notif_stack.set_vexpand(true);
    notif_stack.set_valign(gtk4::Align::Start);

    scrolled_win.set_child(Some(&notif_stack));
    main_box.append(&scrolled_win);

    c_win.set_child(Some(&main_box));

    (
        c_win,
        main_box,
        date_label,
        dummy_time,
        calendar,
        clear_btn,
        notif_stack,
    )
}
