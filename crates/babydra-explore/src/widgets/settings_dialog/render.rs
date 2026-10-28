//! Settings dialog UI construction: sidebar navigation, content stack, and page wiring.

use babydra_core::i18n::trans;
use gtk4::prelude::*;
use gtk4::{Align, Box, Orientation, Separator, Stack, Window};
use std::cell::RefCell;
use std::rc::Rc;

use super::context_menu;
use super::general;
use super::keybinds;

/// Displays the main settings dialog with vertical icon-only pill navigation and card content stack.
pub fn show_settings_dialog(parent: &gtk4::Window, on_change_callback: impl Fn() + 'static) {
    let window = Window::builder()
        .title(trans("explore.settings"))
        .icon_name("babydra-explore")
        .transient_for(parent)
        .modal(true)
        .resizable(false)
        .default_width(720)
        .default_height(500)
        .css_classes(vec![
            "explore-dialog".to_string(),
            "explore-settings-window".to_string(),
        ])
        .build();

    let main_hbox = Box::new(Orientation::Horizontal, 0);
    main_hbox.add_css_class("explore-dialog-box");
    window.set_child(Some(&main_hbox));

    // ── Left: Sidebar Navigation Dock (Matching Explore & Settings) ──
    let sidebar_box = Box::new(Orientation::Vertical, 6);
    sidebar_box.add_css_class("sidebar");
    sidebar_box.add_css_class("explore-sidebar");
    sidebar_box.add_css_class("explore-dialog-sidebar");
    sidebar_box.set_width_request(68);
    sidebar_box.set_hexpand(false);
    sidebar_box.set_vexpand(true);
    sidebar_box.set_margin_top(12);
    sidebar_box.set_margin_bottom(12);
    sidebar_box.set_margin_start(10);
    sidebar_box.set_margin_end(2);
    sidebar_box.set_halign(Align::Center);

    let active_popover: Rc<RefCell<Option<gtk4::Popover>>> = Rc::new(RefCell::new(None));

    // Top Header Icon Button
    let header_btn = gtk4::Button::new();
    header_btn.add_css_class("sidebar-header-btn");
    header_btn.add_css_class("flat");
    header_btn.set_cursor_from_name(Some("pointer"));
    header_btn.set_halign(Align::Center);
    header_btn.set_valign(Align::Center);
    header_btn.set_size_request(44, 44);

    let header_img = babydra_ui_kit::ui::icon::get_icon("settings", 24);
    header_img.set_pixel_size(24);
    header_img.set_valign(Align::Center);
    header_img.set_halign(Align::Center);
    header_btn.set_child(Some(&header_img));

    let (_pop, _) = babydra_ui_kit::components::attach_sidebar_popover(
        &header_btn,
        &trans("explore.settings"),
        &active_popover,
    );
    sidebar_box.append(&header_btn);

    // Separator
    let sep = Separator::new(Orientation::Horizontal);
    sep.add_css_class("sidebar-nav-separator");
    sep.set_margin_top(4);
    sep.set_margin_bottom(6);
    sidebar_box.append(&sep);

    // Nav container
    let nav_container = Box::new(Orientation::Vertical, 4);
    nav_container.set_halign(Align::Center);
    sidebar_box.append(&nav_container);
    main_hbox.append(&sidebar_box);

    // 1. General (Settings Icon)
    let icon_gen = babydra_ui_kit::ui::icon::get_icon("settings", 20);
    icon_gen.set_pixel_size(20);
    let (btn_general, _) = babydra_ui_kit::components::create_sidebar_dock_btn(
        &icon_gen,
        &trans("explore.settings_general"),
        true,
        &active_popover,
    );

    // 2. Keybinds (Terminal/Command Icon)
    let icon_key = babydra_ui_kit::ui::icon::get_icon("terminal", 20);
    icon_key.set_pixel_size(20);
    let (btn_keybinds, _) = babydra_ui_kit::components::create_sidebar_dock_btn(
        &icon_key,
        &trans("explore.settings_keybinds"),
        false,
        &active_popover,
    );

    // 3. Context Menu (Folder/Menu Icon)
    let icon_ctx = babydra_ui_kit::ui::icon::get_icon("folder", 20);
    icon_ctx.set_pixel_size(20);
    let (btn_context, _) = babydra_ui_kit::components::create_sidebar_dock_btn(
        &icon_ctx,
        &trans("explore.settings_context_menu"),
        false,
        &active_popover,
    );

    nav_container.append(&btn_general);
    nav_container.append(&btn_keybinds);
    nav_container.append(&btn_context);

    // ── Right: Elevated Content Card (Matching Explore Content View) ──
    let content_card = Box::new(Orientation::Vertical, 0);
    content_card.add_css_class("explore-dialog-content-card");
    content_card.set_hexpand(true);
    content_card.set_vexpand(true);
    content_card.set_margin_top(8);
    content_card.set_margin_bottom(8);
    content_card.set_margin_start(4);
    content_card.set_margin_end(12);

    let stack = Stack::new();
    stack.set_transition_type(gtk4::StackTransitionType::Crossfade);
    stack.set_transition_duration(250);
    stack.set_hexpand(true);
    stack.set_vexpand(true);
    content_card.append(&stack);
    main_hbox.append(&content_card);

    // Trigger on_change when window is destroyed/closed
    let on_change = Rc::new(on_change_callback);

    // ── Build pages ──────────────────────────────────────────
    let tab_general = general::build_general_page();
    let on_change_k = on_change.clone();
    let tab_keybinds = keybinds::build_keybinds_page(&window, move || {
        on_change_k();
    });
    let tab_context = context_menu::build_context_page(&window);

    stack.add_named(&tab_general, Some("general"));
    stack.add_named(&tab_keybinds, Some("keybinds"));
    stack.add_named(&tab_context, Some("context_menu"));

    // ── Switch tabs closures ─────────────────────────────────
    let act_pop1 = active_popover.clone();
    let btn_gen_c = btn_general.clone();
    let btn_key_c = btn_keybinds.clone();
    let btn_con_c = btn_context.clone();
    let stack_c = stack.clone();
    btn_general.connect_clicked(move |_| {
        if let Some(p) = act_pop1.borrow_mut().take() {
            p.popdown();
        }
        btn_gen_c.add_css_class("active-nav");
        btn_key_c.remove_css_class("active-nav");
        btn_con_c.remove_css_class("active-nav");
        stack_c.set_visible_child_name("general");
    });

    let act_pop2 = active_popover.clone();
    let btn_gen_c2 = btn_general.clone();
    let btn_key_c2 = btn_keybinds.clone();
    let btn_con_c2 = btn_context.clone();
    let stack_c2 = stack.clone();
    btn_keybinds.connect_clicked(move |_| {
        if let Some(p) = act_pop2.borrow_mut().take() {
            p.popdown();
        }
        btn_key_c2.add_css_class("active-nav");
        btn_gen_c2.remove_css_class("active-nav");
        btn_con_c2.remove_css_class("active-nav");
        stack_c2.set_visible_child_name("keybinds");
    });

    let act_pop3 = active_popover.clone();
    let btn_gen_c3 = btn_general.clone();
    let btn_key_c3 = btn_keybinds.clone();
    let btn_con_c3 = btn_context.clone();
    let stack_c3 = stack.clone();
    btn_context.connect_clicked(move |_| {
        if let Some(p) = act_pop3.borrow_mut().take() {
            p.popdown();
        }
        btn_con_c3.add_css_class("active-nav");
        btn_gen_c3.remove_css_class("active-nav");
        btn_key_c3.remove_css_class("active-nav");
        stack_c3.set_visible_child_name("context_menu");
    });

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
        babydra_ui_kit::ui::animation::genie_out(
            hbox_cancel.upcast_ref(),
            720,
            500,
            300,
            move || {
                win_cb.destroy();
            },
        );
        glib::Propagation::Stop
    });

    let on_change_destroy = on_change.clone();
    window.connect_destroy(move |_| {
        on_change_destroy();
    });

    window.present();
    babydra_ui_kit::ui::animation::genie_in(main_hbox.upcast_ref(), 720, 500, 300);
}
