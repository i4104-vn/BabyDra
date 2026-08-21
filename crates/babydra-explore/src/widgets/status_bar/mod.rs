use babydra_core::i18n::trans;
use gtk4::prelude::*;
use gtk4::{Align, Box, Label, Orientation};

#[derive(Clone)]
pub struct StatusBarWidgets {
    pub container: Box,
    pub lbl_status: Label,
    pub btn_empty_trash: gtk4::Button,
    pub btn_toggle_preview: gtk4::Button,
    pub btn_view_icons: gtk4::Button,
    pub btn_view_list: gtk4::Button,
    pub dropdown_sort: gtk4::DropDown,
    pub btn_settings: gtk4::Button,
}

/// Creates the status bar widgets and returns a StatusBarWidgets struct.
pub fn create_status_bar() -> StatusBarWidgets {
    let container = Box::new(Orientation::Horizontal, 8);
    container.set_css_classes(&["status-bar"]);

    let lbl_status = Label::builder()
        .label(&format!("0 {}", trans("explore.items")))
        .halign(Align::Start)
        .hexpand(true)
        .build();
    container.append(&lbl_status);

    // Empty Trash button (visible only in Trash folder)
    let btn_empty_trash = gtk4::Button::builder()
        .valign(Align::Center)
        .css_classes(vec!["status-bar-trash-btn".to_string()])
        .visible(false)
        .tooltip_text(&trans("explore.empty_trash"))
        .build();
    btn_empty_trash.set_cursor_from_name(Some("pointer"));

    let trash_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    let trash_icon = babydra_ui_kit::ui::icon::get_icon("user-trash-full-symbolic", 14);
    let trash_lbl = gtk4::Label::new(Some(&trans("explore.empty_trash")));
    trash_lbl.add_css_class("status-bar-trash-lbl");
    trash_box.append(&trash_icon);
    trash_box.append(&trash_lbl);
    btn_empty_trash.set_child(Some(&trash_box));

    container.append(&btn_empty_trash);

    // DropDown for sorting (Auto, By date, By group)
    let sort_options = [
        trans("explore.sort_auto"),
        trans("explore.sort_date"),
        trans("explore.sort_group"),
    ];
    let sort_options_strs: Vec<&str> = sort_options.iter().map(|s| s.as_str()).collect();
    let dropdown_sort = gtk4::DropDown::from_strings(&sort_options_strs);
    dropdown_sort.set_css_classes(&["status-bar-dropdown"]);
    dropdown_sort.set_tooltip_text(Some(&trans("explore.sort_by")));
    container.append(&dropdown_sort);

    let btn_view_icons = babydra_ui_kit::components::create_icon_button(
        "view-grid",
        16,
        &["status-bar-btn"],
        Some(&trans("explore.view_grid")),
        || {},
    );
    container.append(&btn_view_icons);

    let btn_view_list = babydra_ui_kit::components::create_icon_button(
        "view-list",
        16,
        &["status-bar-btn"],
        Some(&trans("explore.view_list")),
        || {},
    );
    container.append(&btn_view_list);

    let btn_toggle_preview = babydra_ui_kit::components::create_icon_button(
        "sidebar",
        16,
        &["status-bar-btn", "status-bar-btn-active"],
        Some(&trans("explore.toggle_preview")),
        || {},
    );
    container.append(&btn_toggle_preview);

    // Separator before settings
    let sep = gtk4::Separator::new(Orientation::Vertical);
    sep.set_css_classes(&["status-bar-separator"]);
    container.append(&sep);

    let btn_settings = babydra_ui_kit::components::create_icon_button(
        "settings",
        16,
        &["status-bar-btn"],
        Some(&trans("explore.settings")),
        || {},
    );
    container.append(&btn_settings);

    StatusBarWidgets {
        container,
        lbl_status,
        btn_empty_trash,
        btn_toggle_preview,
        btn_view_icons,
        btn_view_list,
        dropdown_sort,
        btn_settings,
    }
}

/// Updates the status bar content and empty trash button visibility.
pub fn update_status_bar(
    widgets: &StatusBarWidgets,
    count: usize,
    total_size: u64,
    current_path: &std::path::Path,
) {
    let size_str = babydra_ui_kit::components::explore::format_size(total_size);
    widgets.lbl_status.set_text(&format!(
        "{} {} | {}: {}",
        count,
        trans("explore.items"),
        trans("explore.total_size"),
        size_str
    ));

    let is_trash = babydra_ui_kit::components::explore::is_in_trash(current_path);
    widgets.btn_empty_trash.set_visible(is_trash);
}
