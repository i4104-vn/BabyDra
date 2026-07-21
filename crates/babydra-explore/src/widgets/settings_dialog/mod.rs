use gtk4::prelude::*;
use gtk4::{Box, Orientation, Button, Window, Align, Stack, Separator};
use babydra_common::i18n::t;

mod general;
mod context_menu;
mod keybinds;

/// Displays the main settings dialog with tabs for general settings, keyboard shortcuts, and custom context menus.
pub fn show_settings_dialog(parent: &gtk4::Window, on_change_callback: impl Fn() + 'static) {
    let window = Window::builder()
        .title(&t("explore.settings"))
        .transient_for(parent)
        .modal(true)
        .resizable(true)
        .default_width(750)
        .default_height(550)
        .css_classes(vec!["explore-dialog".to_string()])
        .build();

    let main_vbox = Box::new(Orientation::Vertical, 0);
    window.set_child(Some(&main_vbox));

    let content_hbox = Box::new(Orientation::Horizontal, 0);
    content_hbox.set_vexpand(true);
    main_vbox.append(&content_hbox);

    // ── Left: Sidebar ──────────────────────────────────────────
    let sidebar_box = Box::new(Orientation::Vertical, 0);
    sidebar_box.set_size_request(140, -1);
    sidebar_box.set_margin_top(0);
    sidebar_box.set_margin_bottom(0);
    sidebar_box.set_margin_start(0);
    sidebar_box.set_margin_end(0);
    sidebar_box.add_css_class("settings-sidebar");
    content_hbox.append(&sidebar_box);

    let stack = Stack::new();
    stack.set_transition_type(gtk4::StackTransitionType::SlideLeftRight);
    stack.set_hexpand(true);
    stack.set_vexpand(true);

    // Sidebar items list
    let btn_general = Button::builder()
        .label(&t("explore.settings_general"))
        .hexpand(true)
        .halign(Align::Fill)
        .css_classes(vec!["sidebar-item".to_string(), "active-nav".to_string()])
        .build();
    btn_general.set_cursor_from_name(Some("pointer"));

    let btn_keybinds = Button::builder()
        .label(&t("explore.settings_keybinds"))
        .hexpand(true)
        .halign(Align::Fill)
        .css_classes(vec!["sidebar-item".to_string()])
        .build();
    btn_keybinds.set_cursor_from_name(Some("pointer"));

    let btn_context = Button::builder()
        .label(&t("explore.settings_context_menu"))
        .hexpand(true)
        .halign(Align::Fill)
        .css_classes(vec!["sidebar-item".to_string()])
        .build();
    btn_context.set_cursor_from_name(Some("pointer"));

    sidebar_box.append(&btn_general);
    sidebar_box.append(&btn_keybinds);
    sidebar_box.append(&btn_context);

    // Separator between sidebar and content stack
    let sep_sidebar = Separator::new(Orientation::Vertical);
    content_hbox.append(&sep_sidebar);

    // Right content area wrapper box
    let right_vbox = Box::new(Orientation::Vertical, 10);
    right_vbox.set_margin_top(16);
    right_vbox.set_margin_bottom(16);
    right_vbox.set_margin_start(20);
    right_vbox.set_margin_end(20);
    right_vbox.set_hexpand(true);
    right_vbox.set_vexpand(true);
    content_hbox.append(&right_vbox);

    right_vbox.append(&stack);

    // ── Build pages ──────────────────────────────────────────
    let tab_general = general::build_general_page();
    let tab_keybinds = keybinds::build_keybinds_page();
    let tab_context = context_menu::build_context_menu_page();

    stack.add_named(&tab_general, Some("general"));
    stack.add_named(&tab_keybinds, Some("keybinds"));
    stack.add_named(&tab_context, Some("context_menu"));

    // ── Switch tabs closures ─────────────────────────────────
    let btn_gen_c = btn_general.clone();
    let btn_key_c = btn_keybinds.clone();
    let btn_con_c = btn_context.clone();
    let stack_c = stack.clone();
    btn_general.connect_clicked(move |_| {
        btn_gen_c.add_css_class("active-nav");
        btn_key_c.remove_css_class("active-nav");
        btn_con_c.remove_css_class("active-nav");
        stack_c.set_visible_child_name("general");
    });

    let btn_gen_c2 = btn_general.clone();
    let btn_key_c2 = btn_keybinds.clone();
    let btn_con_c2 = btn_context.clone();
    let stack_c2 = stack.clone();
    btn_keybinds.connect_clicked(move |_| {
        btn_key_c2.add_css_class("active-nav");
        btn_gen_c2.remove_css_class("active-nav");
        btn_con_c2.remove_css_class("active-nav");
        stack_c2.set_visible_child_name("keybinds");
    });

    let btn_gen_c3 = btn_general.clone();
    let btn_key_c3 = btn_keybinds.clone();
    let btn_con_c3 = btn_context.clone();
    let stack_c3 = stack.clone();
    btn_context.connect_clicked(move |_| {
        btn_con_c3.add_css_class("active-nav");
        btn_gen_c3.remove_css_class("active-nav");
        btn_key_c3.remove_css_class("active-nav");
        stack_c3.set_visible_child_name("context_menu");
    });

    // Also trigger on_change when window is destroyed/closed
    let on_change = std::rc::Rc::new(on_change_callback);
    let on_change_destroy = on_change.clone();
    window.connect_destroy(move |_| {
        on_change_destroy();
    });

    window.present();
}
