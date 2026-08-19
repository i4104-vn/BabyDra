//! Layout structure and widget builders for the desktop taskbar buttons and window previews.

use babydra_core::DesktopApp;
use gtk4::prelude::*;

/// Builds the base container box for the taskbar.
pub fn build_workspace_box() -> (gtk4::Box, gtk4::Box) {
    let parent_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 4);
    parent_box.add_css_class("taskbar-parent-box");
    parent_box.set_valign(gtk4::Align::Center);
    parent_box.set_halign(gtk4::Align::Center);

    let apps_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 4);
    apps_box.add_css_class("taskbar-apps-box");
    apps_box.set_valign(gtk4::Align::Center);
    parent_box.append(&apps_box);

    (parent_box, apps_box)
}

/// Creates a Popover widget anchored to the parent taskbar button.
pub fn build_popover_box(parent: &gtk4::Button) -> gtk4::Popover {
    let popover = babydra_ui_kit::components::create_popover(
        parent,
        gtk4::PositionType::Bottom,
        "taskbar-popover",
    );
    popover.set_autohide(true);
    popover
}

/// Constructs a taskbar item button with application icon, dot indicators for window count, and tooltip.
pub fn build_taskbar_btn(app: &DesktopApp, is_active: bool, window_count: usize) -> gtk4::Button {
    let btn = gtk4::Button::new();
    btn.add_css_class("taskbar-app-btn");
    btn.set_cursor_from_name(Some("pointer"));
    btn.set_widget_name(&app.app_id.clone().unwrap_or_else(|| app.name.clone()));
    btn.set_tooltip_text(Some(&app.name));
    btn.set_valign(gtk4::Align::Center);
    btn.set_halign(gtk4::Align::Center);

    if is_active {
        btn.add_css_class("active");
    }

    let container = gtk4::Box::new(gtk4::Orientation::Horizontal, 3);
    container.set_valign(gtk4::Align::Center);
    container.set_halign(gtk4::Align::Center);

    let dots_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    dots_box.add_css_class("taskbar-dots-container");
    dots_box.set_halign(gtk4::Align::Center);
    dots_box.set_valign(gtk4::Align::Center);

    let count = window_count.clamp(1, 3);
    for _ in 0..count {
        let dot = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
        dot.add_css_class("taskbar-dot");
        dot.set_size_request(3, 3);
        dots_box.append(&dot);
    }

    container.append(&dots_box);

    let icon = babydra_ui_kit::ui::icon::get_fallback_icon(
        app.icon.as_deref().unwrap_or(""),
        "application-x-executable",
    );
    icon.set_pixel_size(16);
    container.append(&icon);

    btn.set_child(Some(&container));

    btn
}

/// Renders the list of active window previews inside the Popover dropdown.
/// Includes buttons to open a new app instance and to close all windows of this app.
pub fn render_previews(
    popover: &gtk4::Popover,
    windows: &[DesktopApp],
    app_id: &str,
) -> (
    Vec<(gtk4::Button, gtk4::Button, DesktopApp)>,
    Option<(gtk4::Button, String)>,
    Option<gtk4::Button>,
) {
    let previews_box = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
    previews_box.add_css_class("taskbar-previews-container");

    let header = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    header.add_css_class("taskbar-previews-header");
    header.set_valign(gtk4::Align::Center);

    if let Some(first_app) = windows.first() {
        let icon_name = first_app.icon.as_deref().unwrap_or(app_id);
        let header_icon =
            babydra_ui_kit::ui::icon::get_fallback_icon(icon_name, "application-x-executable");
        header_icon.set_pixel_size(14);
        header.append(&header_icon);

        let header_label = gtk4::Label::new(Some(&first_app.name));
        header_label.add_css_class("taskbar-previews-header-label");
        header_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        header.append(&header_label);
    } else {
        let header_label = gtk4::Label::new(Some(&babydra_core::i18n::trans("taskbar.tasks")));
        header_label.add_css_class("taskbar-previews-header-label");
        header.append(&header_label);
    }
    previews_box.append(&header);

    let header_sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    header_sep.add_css_class("taskbar-preview-separator");
    previews_box.append(&header_sep);

    let mut action_triggers = Vec::new();

    for app in windows {
        let item_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 4);
        item_box.add_css_class("taskbar-preview-list-item-box");
        item_box.set_hexpand(true);
        item_box.set_valign(gtk4::Align::Center);

        let preview_btn = gtk4::Button::new();
        preview_btn.add_css_class("taskbar-preview-list-btn");
        preview_btn.set_hexpand(true);

        let item_content = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
        item_content.set_valign(gtk4::Align::Center);

        let win_icon = babydra_ui_kit::ui::icon::get_icon("window-new-symbolic", 12);
        win_icon.add_css_class("taskbar-preview-item-icon");
        item_content.append(&win_icon);

        let title_lbl = gtk4::Label::new(None);
        title_lbl.add_css_class("taskbar-preview-title");
        title_lbl.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        title_lbl.set_max_width_chars(22);
        title_lbl.set_hexpand(true);
        title_lbl.set_halign(gtk4::Align::Start);

        let title_str = app.window_title.as_deref().unwrap_or("");
        let label_text = if title_str.is_empty() {
            app.name.clone()
        } else {
            title_str.to_string()
        };
        title_lbl.set_text(&label_text);
        item_content.append(&title_lbl);

        preview_btn.set_child(Some(&item_content));

        let kill_btn = gtk4::Button::builder()
            .child(&babydra_ui_kit::ui::icon::get_icon(
                "window-close-symbolic",
                14,
            ))
            .build();
        kill_btn.add_css_class("taskbar-preview-list-kill-btn");
        kill_btn.set_valign(gtk4::Align::Center);

        item_box.append(&preview_btn);
        item_box.append(&kill_btn);
        previews_box.append(&item_box);

        action_triggers.push((preview_btn, kill_btn, app.clone()));
    }

    let mut open_new_info = None;
    let mut close_all_btn_opt = None;

    if !windows.is_empty() {
        let (app_name, exec_cmd, icon_name) = if let Some(first_app) = windows.first() {
            (
                first_app.name.clone(),
                first_app.exec.clone(),
                first_app.icon.clone().unwrap_or_else(|| app_id.to_string()),
            )
        } else {
            (app_id.to_string(), app_id.to_string(), app_id.to_string())
        };

        let separator = gtk4::Separator::new(gtk4::Orientation::Horizontal);
        separator.add_css_class("taskbar-preview-separator");
        previews_box.append(&separator);

        let open_new_btn = gtk4::Button::new();
        open_new_btn.add_css_class("taskbar-preview-action-btn");
        open_new_btn.set_hexpand(true);

        let open_new_content = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        open_new_content.set_halign(gtk4::Align::Start);
        let open_new_icon =
            babydra_ui_kit::ui::icon::get_fallback_icon(&icon_name, "application-x-executable");
        open_new_icon.set_pixel_size(14);
        let open_new_label = gtk4::Label::new(Some(&format!("Open new {}", app_name)));
        open_new_label.add_css_class("taskbar-preview-action-label");
        open_new_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        open_new_label.set_max_width_chars(20);
        open_new_label.set_hexpand(true);
        open_new_label.set_halign(gtk4::Align::Start);
        open_new_content.append(&open_new_icon);
        open_new_content.append(&open_new_label);
        open_new_btn.set_child(Some(&open_new_content));
        previews_box.append(&open_new_btn);
        open_new_info = Some((open_new_btn, exec_cmd));

        let close_all_btn = gtk4::Button::new();
        close_all_btn.add_css_class("taskbar-preview-action-btn");
        close_all_btn.add_css_class("taskbar-preview-close-all");
        close_all_btn.set_hexpand(true);

        let close_all_content = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        close_all_content.set_halign(gtk4::Align::Start);

        let close_all_icon = babydra_ui_kit::ui::icon::get_icon("window-close-symbolic", 14);
        close_all_icon.set_pixel_size(14);
        close_all_icon.add_css_class("taskbar-preview-action-icon");

        let close_all_label =
            gtk4::Label::new(Some(&babydra_core::i18n::trans("taskbar.close_all")));
        close_all_label.add_css_class("taskbar-preview-action-label");
        close_all_label.add_css_class("close-all-text");
        close_all_content.append(&close_all_icon);
        close_all_content.append(&close_all_label);
        close_all_btn.set_child(Some(&close_all_content));
        previews_box.append(&close_all_btn);
        close_all_btn_opt = Some(close_all_btn);
    }

    popover.set_child(Some(&previews_box));

    (action_triggers, open_new_info, close_all_btn_opt)
}
