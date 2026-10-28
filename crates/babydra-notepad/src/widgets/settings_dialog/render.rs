//! Settings dialog shell: navigation, page stack, and dialog lifecycle.

use super::pages::{build_editor_page, build_font_page, build_saving_page};
use babydra_core::i18n::trans;
use babydra_core::models::notepad::{load_notepad_cfg, NotepadSettings};
use babydra_ui_kit::components::{attach_sidebar_popover, create_sidebar_dock_btn};
use babydra_ui_kit::ui::icon::get_icon;
use gtk4::prelude::*;
use gtk4::{Align, Box, Button, Orientation, Separator, Stack, Window};
use std::cell::RefCell;
use std::rc::Rc;

pub fn show_settings_dialog(
    parent: &impl IsA<Window>,
    on_change_callback: impl Fn(NotepadSettings) + 'static,
) {
    let state = Rc::new(RefCell::new(load_notepad_cfg()));
    let changed: Rc<dyn Fn(NotepadSettings)> = Rc::new(on_change_callback);
    let window = Window::builder()
        .title(trans("notepad.settings"))
        .icon_name("babydra-notepad")
        .transient_for(parent.as_ref())
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

    // ── Left: Slim Sidebar Navigation Dock (Matching Explore & Settings) ──
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
    let header_btn = Button::new();
    header_btn.add_css_class("sidebar-header-btn");
    header_btn.add_css_class("flat");
    header_btn.set_cursor_from_name(Some("pointer"));
    header_btn.set_halign(Align::Center);
    header_btn.set_valign(Align::Center);
    header_btn.set_size_request(44, 44);

    let header_img = get_icon("settings", 24);
    header_img.set_pixel_size(24);
    header_img.set_valign(Align::Center);
    header_img.set_halign(Align::Center);
    header_btn.set_child(Some(&header_img));

    let (_pop, _) = attach_sidebar_popover(
        &header_btn,
        &trans("notepad.settings"),
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

    // 1. Font
    let icon_font = get_icon("format-text-size", 20);
    icon_font.set_pixel_size(20);
    let (btn_font, _) = create_sidebar_dock_btn(
        &icon_font,
        &trans("notepad.settings_font"),
        true,
        &active_popover,
    );

    // 2. Editor
    let icon_editor = get_icon("sliders", 20);
    icon_editor.set_pixel_size(20);
    let (btn_editor, _) = create_sidebar_dock_btn(
        &icon_editor,
        &trans("notepad.settings_editor"),
        false,
        &active_popover,
    );

    // 3. Saving / Formatting
    let icon_saving = get_icon("document-save", 20);
    icon_saving.set_pixel_size(20);
    let (btn_saving, _) = create_sidebar_dock_btn(
        &icon_saving,
        &trans("notepad.settings_formatting"),
        false,
        &active_popover,
    );

    nav_container.append(&btn_font);
    nav_container.append(&btn_editor);
    nav_container.append(&btn_saving);

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
    stack.add_named(&build_font_page(&state, &changed), Some("font"));
    stack.add_named(&build_editor_page(&state, &changed), Some("editor"));
    stack.add_named(&build_saving_page(&state, &changed), Some("saving"));
    content_card.append(&stack);

    let footer = Box::new(Orientation::Horizontal, 8);
    footer.set_halign(Align::End);
    footer.set_margin_top(10);
    footer.set_margin_bottom(8);
    footer.set_margin_end(8);
    let close = Button::builder()
        .label(trans("notepad.settings_save_btn"))
        .css_classes(vec![
            "suggested-action".to_string(),
            "modern-dialog-primary-btn".to_string(),
        ])
        .build();
    close.set_cursor_from_name(Some("pointer"));
    let window_c = window.clone();
    close.connect_clicked(move |_| window_c.close());
    footer.append(&close);
    content_card.append(&footer);

    main_hbox.append(&content_card);

    // ── Switch tabs closures ──
    let act_pop1 = active_popover.clone();
    let btn_f_c = btn_font.clone();
    let btn_e_c = btn_editor.clone();
    let btn_s_c = btn_saving.clone();
    let stack_c = stack.clone();
    btn_font.connect_clicked(move |_| {
        if let Some(p) = act_pop1.borrow_mut().take() {
            p.popdown();
        }
        btn_f_c.add_css_class("active-nav");
        btn_e_c.remove_css_class("active-nav");
        btn_s_c.remove_css_class("active-nav");
        stack_c.set_visible_child_name("font");
    });

    let act_pop2 = active_popover.clone();
    let btn_f_c2 = btn_font.clone();
    let btn_e_c2 = btn_editor.clone();
    let btn_s_c2 = btn_saving.clone();
    let stack_c2 = stack.clone();
    btn_editor.connect_clicked(move |_| {
        if let Some(p) = act_pop2.borrow_mut().take() {
            p.popdown();
        }
        btn_e_c2.add_css_class("active-nav");
        btn_f_c2.remove_css_class("active-nav");
        btn_s_c2.remove_css_class("active-nav");
        stack_c2.set_visible_child_name("editor");
    });

    let act_pop3 = active_popover.clone();
    let btn_f_c3 = btn_font.clone();
    let btn_e_c3 = btn_editor.clone();
    let btn_s_c3 = btn_saving.clone();
    let stack_c3 = stack.clone();
    btn_saving.connect_clicked(move |_| {
        if let Some(p) = act_pop3.borrow_mut().take() {
            p.popdown();
        }
        btn_s_c3.add_css_class("active-nav");
        btn_f_c3.remove_css_class("active-nav");
        btn_e_c3.remove_css_class("active-nav");
        stack_c3.set_visible_child_name("saving");
    });

    window.set_child(Some(&main_hbox));
    window.present();
}
