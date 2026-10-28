use babydra_core::i18n::trans;
use gtk4::prelude::*;
use gtk4::{Align, Box, Label, ListBox, Orientation, Overlay, ScrolledWindow, Window};

mod add_dialog;
pub mod row;

pub(super) const AVAILABLE_ICONS: &[&str] = &[
    "settings", "terminal", "folder", "text", "camera", "music", "user", "activity", "lock",
    "wifi", "refresh", "power", "search", "logo",
];

/// Builds the context-menu settings page and its add-option action.
pub fn build_context_page(parent_window: &Window) -> Overlay {
    let settings = babydra_core::load_explore_cfg();
    let overlay = Overlay::new();
    overlay.set_hexpand(true);
    overlay.set_vexpand(true);

    let scroll = ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .hexpand(true)
        .vexpand(true)
        .build();
    overlay.set_child(Some(&scroll));

    let content_box = Box::new(Orientation::Vertical, 12);
    content_box.set_margin_top(20);
    content_box.set_margin_bottom(20);
    content_box.set_margin_start(24);
    content_box.set_margin_end(24);
    content_box.set_vexpand(true);
    content_box.set_hexpand(true);

    // Header matching standard settings layout
    let header_box = Box::new(Orientation::Vertical, 4);
    header_box.set_margin_bottom(6);

    let title_lbl = Label::new(Some(&trans("explore.settings_context_menu")));
    title_lbl.add_css_class("settings-page-title");
    title_lbl.set_halign(Align::Start);
    header_box.append(&title_lbl);

    let subtitle_lbl = Label::new(Some(&trans("explore.settings_custom_options")));
    subtitle_lbl.add_css_class("settings-page-subtitle");
    subtitle_lbl.set_halign(Align::Start);
    header_box.append(&subtitle_lbl);

    content_box.append(&header_box);

    let context_listbox = ListBox::new();
    context_listbox.set_selection_mode(gtk4::SelectionMode::None);
    context_listbox.add_css_class("settings-card");
    content_box.append(&context_listbox);

    for item in settings.custom_context_items {
        row::render_option_row(&context_listbox, item);
    }

    scroll.set_child(Some(&content_box));

    let btn_fab = babydra_ui_kit::components::create_fab("plus");
    btn_fab.add_css_class("circular");
    btn_fab.set_margin_bottom(16);
    btn_fab.set_margin_end(16);
    btn_fab.set_tooltip_text(Some(&trans("explore.settings_add_option")));
    btn_fab.set_cursor_from_name(Some("pointer"));
    overlay.add_overlay(&btn_fab);

    let listbox = context_listbox.clone();
    let parent_window = parent_window.clone();
    btn_fab.connect_clicked(move |_| {
        add_dialog::show_add_option_dialog(&parent_window, &listbox);
    });

    overlay
}
