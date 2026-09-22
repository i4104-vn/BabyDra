use babydra_core::i18n::trans;
use gtk4::prelude::*;
use gtk4::{ListBox, Overlay, Window};

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

    let scroll = gtk4::ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .hexpand(true)
        .vexpand(true)
        .build();
    overlay.set_child(Some(&scroll));

    let context_listbox = ListBox::new();
    context_listbox.set_selection_mode(gtk4::SelectionMode::None);
    context_listbox.add_css_class("settings-card");
    scroll.set_child(Some(&context_listbox));

    for item in settings.custom_context_items {
        row::render_option_row(&context_listbox, item);
    }

    let btn_fab = babydra_ui_kit::components::create_fab("plus");
    btn_fab.add_css_class("circular");
    btn_fab.set_margin_bottom(10);
    btn_fab.set_margin_end(10);
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
