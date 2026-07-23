use gtk4::prelude::*;
use gtk4::{Box, Orientation, Window, Stack};
use babydra_common::i18n::t;
use std::rc::Rc;

mod general;
mod context_menu;
mod keybinds;

/// Displays the main settings dialog with vertical icon-only pill navigation and card content stack.
pub fn show_settings_dialog(parent: &gtk4::Window, on_change_callback: impl Fn() + 'static) {
    let window = Window::builder()
        .title(&t("explore.settings"))
        .transient_for(parent)
        .modal(true)
        .resizable(false)
        .default_width(680)
        .default_height(460)
        .css_classes(vec!["explore-dialog".to_string()])
        .build();

    let main_hbox = Box::new(Orientation::Horizontal, 0);
    main_hbox.add_css_class("explore-dialog-box");
    window.set_child(Some(&main_hbox));

    // ── Left: Sidebar Navigation (Explore-style) ───────────────
    let sidebar = gtk4::ScrolledWindow::new();
    sidebar.set_hscrollbar_policy(gtk4::PolicyType::Never);
    sidebar.add_css_class("sidebar");
    sidebar.set_width_request(190);
    sidebar.set_hexpand(false);
    sidebar.set_vexpand(true);
    sidebar.set_margin_top(8);
    sidebar.set_margin_bottom(8);
    sidebar.set_margin_start(8);

    let nav_container = Box::new(Orientation::Vertical, 2);
    nav_container.set_margin_top(4);
    nav_container.set_margin_bottom(4);
    sidebar.set_child(Some(&nav_container));
    main_hbox.append(&sidebar);

    // 1. General (Settings Icon)
    let btn_general = babydra_utils::components::create_sidebar_item_button(&t("explore.settings_general"), "settings", "sidebar-item", || {});
    btn_general.add_css_class("active-nav");
    btn_general.set_cursor_from_name(Some("pointer"));

    // 2. Keybinds (Terminal/Command Icon)
    let btn_keybinds = babydra_utils::components::create_sidebar_item_button(&t("explore.settings_keybinds"), "terminal", "sidebar-item", || {});
    btn_keybinds.set_cursor_from_name(Some("pointer"));

    // 3. Context Menu (Folder/Menu Icon)
    let btn_context = babydra_utils::components::create_sidebar_item_button(&t("explore.settings_context_menu"), "folder", "sidebar-item", || {});
    btn_context.set_cursor_from_name(Some("pointer"));

    nav_container.append(&btn_general);
    nav_container.append(&btn_keybinds);
    nav_container.append(&btn_context);

    // ── Right: Content Stack ───────────────────────────────────
    let stack = Stack::new();
    stack.set_transition_type(gtk4::StackTransitionType::Crossfade);
    stack.set_transition_duration(300);
    stack.set_hexpand(true);
    stack.set_vexpand(true);

    let stack_container = Box::new(Orientation::Vertical, 0);
    stack_container.set_margin_top(16);
    stack_container.set_margin_start(16);
    stack_container.set_margin_end(16);
    stack_container.set_margin_bottom(16);
    stack_container.set_hexpand(true);
    stack_container.set_vexpand(true);
    stack_container.append(&stack);
    main_hbox.append(&stack_container);

    // Trigger on_change when window is destroyed/closed
    let on_change = std::rc::Rc::new(on_change_callback);

    // ── Build pages ──────────────────────────────────────────
    let tab_general = general::build_general_page();
    let on_change_k = on_change.clone();
    let tab_keybinds = keybinds::build_keybinds_page(&window, move || {
        on_change_k();
    });
    let tab_context = context_menu::build_context_menu_page(&window);

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

    // Close Request & Animation Setup
    let win_cancel = window.clone();
    let hbox_cancel = main_hbox.clone();
    let is_animating = Rc::new(std::cell::Cell::new(false));
    let is_animating_cancel = is_animating.clone();
    window.connect_close_request(move |_| {
        if is_animating_cancel.get() {
            return glib::Propagation::Stop;
        }
        is_animating_cancel.set(true);
        let win_cb = win_cancel.clone();
        babydra_utils::ui::animation::genie_out(
            hbox_cancel.upcast_ref(),
            680,
            460,
            300,
            move || {
                win_cb.destroy();
            }
        );
        glib::Propagation::Stop
    });

    let on_change_destroy = on_change.clone();
    window.connect_destroy(move |_| {
        on_change_destroy();
    });

    window.present();
    babydra_utils::ui::animation::genie_in(main_hbox.upcast_ref(), 680, 460, 300);
}
