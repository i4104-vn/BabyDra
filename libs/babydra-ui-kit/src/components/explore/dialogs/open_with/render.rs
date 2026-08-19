use babydra_core::i18n::trans;
use babydra_core::services::apps::DesktopApp;
use gtk4::prelude::*;
use gtk4::{
    Align, Box, Button, CheckButton, Label, ListBox, ListBoxRow, Orientation, ScrolledWindow,
    SearchEntry, Window,
};
use std::path::PathBuf;

pub struct OpenWithDialogWidgets {
    pub window: Window,
    pub _vbox: Box,
    pub search_entry: SearchEntry,
    pub listbox: ListBox,
    pub check_always: CheckButton,
    pub btn_cancel: Button,
    pub btn_open: Button,
    pub apps: Vec<DesktopApp>,
}

/// Builds the UI layout for the Open With (App Picker) dialog.
pub fn build_open_with_dialog(
    path: &PathBuf,
    parent: Option<&impl IsA<gtk4::Window>>,
) -> OpenWithDialogWidgets {
    let window = Window::builder()
        .title(&trans("explore.dialog_open_with_title"))
        .modal(true)
        .resizable(false)
        .default_width(420)
        .default_height(520)
        .css_classes(vec![
            "explore-dialog".to_string(),
            "open-with-dialog".to_string(),
        ])
        .build();

    if let Some(p) = parent {
        window.set_transient_for(Some(p));
    }

    let vbox = Box::new(Orientation::Vertical, 12);
    vbox.add_css_class("explore-dialog-box");
    vbox.set_margin_top(16);
    vbox.set_margin_bottom(16);
    vbox.set_margin_start(16);
    vbox.set_margin_end(16);
    window.set_child(Some(&vbox));

    // Header info with target file name
    let filename = path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let header_box = Box::new(Orientation::Vertical, 2);
    let lbl_title = Label::builder()
        .label(&trans("explore.dialog_open_with_title"))
        .halign(Align::Start)
        .css_classes(vec!["dialog-title".to_string()])
        .build();
    let lbl_subtitle = Label::builder()
        .label(&filename)
        .halign(Align::Start)
        .css_classes(vec!["dim-label".to_string()])
        .ellipsize(gtk4::pango::EllipsizeMode::Middle)
        .max_width_chars(40)
        .build();
    header_box.append(&lbl_title);
    header_box.append(&lbl_subtitle);
    vbox.append(&header_box);

    // Search bar
    let search_entry = SearchEntry::new();
    search_entry.set_placeholder_text(Some(&trans("explore.dialog_open_with_search")));
    search_entry.set_hexpand(true);
    vbox.append(&search_entry);

    // Apps list inside scrolled window
    let scrolled = ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .min_content_height(280)
        .hexpand(true)
        .vexpand(true)
        .css_classes(vec!["open-with-scrolled".to_string()])
        .build();

    let listbox = ListBox::new();
    listbox.set_selection_mode(gtk4::SelectionMode::Single);
    listbox.add_css_class("open-with-list");

    let apps = babydra_core::services::apps::scan_desktop_apps();

    for app in &apps {
        let row = ListBoxRow::new();
        row.add_css_class("open-with-row");

        let row_box = Box::new(Orientation::Horizontal, 12);
        row_box.set_margin_top(6);
        row_box.set_margin_bottom(6);
        row_box.set_margin_start(8);
        row_box.set_margin_end(8);

        let icon_name = app.icon.as_deref().unwrap_or("application-x-executable");
        let icon = crate::ui::icon::get_fallback_icon(icon_name, "application-x-executable");
        icon.set_pixel_size(32);
        icon.set_halign(Align::Center);
        icon.set_valign(Align::Center);
        row_box.append(&icon);

        let text_box = Box::new(Orientation::Vertical, 2);
        text_box.set_hexpand(true);
        text_box.set_valign(Align::Center);

        let name_lbl = Label::builder()
            .label(&app.name)
            .halign(Align::Start)
            .css_classes(vec!["app-name".to_string()])
            .ellipsize(gtk4::pango::EllipsizeMode::End)
            .build();

        let clean_exec = app
            .exec
            .split_whitespace()
            .filter(|w| !w.starts_with('%'))
            .collect::<Vec<&str>>()
            .join(" ");
        let exec_lbl = Label::builder()
            .label(&clean_exec)
            .halign(Align::Start)
            .css_classes(vec!["dim-label".to_string(), "app-exec".to_string()])
            .ellipsize(gtk4::pango::EllipsizeMode::End)
            .build();

        text_box.append(&name_lbl);
        text_box.append(&exec_lbl);
        row_box.append(&text_box);

        row.set_child(Some(&row_box));
        listbox.append(&row);
    }

    scrolled.set_child(Some(&listbox));
    vbox.append(&scrolled);

    // "Always use this app" checkbox
    let ext = path
        .extension()
        .map(|e| format!(".{}", e.to_string_lossy()))
        .unwrap_or_default();
    let always_label = if ext.is_empty() {
        trans("explore.dialog_open_with_always")
    } else {
        format!("{} ({})", trans("explore.dialog_open_with_always"), ext)
    };
    let check_always = CheckButton::with_label(&always_label);
    check_always.set_active(false);
    vbox.append(&check_always);

    // Bottom action buttons
    let bbox = Box::new(Orientation::Horizontal, 8);
    bbox.set_halign(Align::End);
    bbox.set_margin_top(4);
    vbox.append(&bbox);

    let btn_cancel = Button::with_label(&trans("explore.settings_cancel"));
    let btn_open = Button::builder()
        .label(&trans("explore.menu_open"))
        .css_classes(vec!["suggested-action".to_string()])
        .sensitive(false)
        .build();

    bbox.append(&btn_cancel);
    bbox.append(&btn_open);

    OpenWithDialogWidgets {
        window,
        _vbox: vbox,
        search_entry,
        listbox,
        check_always,
        btn_cancel,
        btn_open,
        apps,
    }
}
