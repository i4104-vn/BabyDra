use babydra_core::DesktopApp;
use gtk4::prelude::*;

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

pub fn build_popover_box(parent: &gtk4::Button) -> gtk4::Popover {
    let popover = babydra_ui_kit::components::create_popover(
        parent,
        gtk4::PositionType::Bottom,
        "taskbar-popover",
    );
    popover.set_has_arrow(false);
    popover.set_autohide(true);
    popover
}

pub fn build_taskbar_btn(app: &DesktopApp, is_active: bool, window_count: usize) -> gtk4::Button {
    let btn = gtk4::Button::new();
    btn.add_css_class("taskbar-app-btn");
    btn.set_cursor_from_name(Some("pointer"));
    btn.set_widget_name(&app.app_id.clone().unwrap_or_else(|| app.name.clone()));
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

pub fn render_previews(
    popover: &gtk4::Popover,
    windows: &[DesktopApp],
    app_id: &str,
) -> (
    Vec<(gtk4::Button, gtk4::Button, DesktopApp)>,
    Option<(gtk4::Button, String)>,
    Option<gtk4::Button>,
) {
    let previews_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    previews_box.add_css_class("taskbar-previews-container");
    previews_box.set_size_request(270, -1);

    // 1. Header Card
    let header = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    header.add_css_class("taskbar-previews-header");
    header.set_valign(gtk4::Align::Center);

    let first_app_opt = windows.first();
    let app_name = first_app_opt
        .map(|a| a.name.clone())
        .unwrap_or_else(|| babydra_core::i18n::trans("taskbar.tasks"));
    let icon_name = first_app_opt
        .and_then(|a| a.icon.clone())
        .unwrap_or_else(|| app_id.to_string());

    let header_icon_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    header_icon_box.add_css_class("taskbar-previews-header-icon-box");
    header_icon_box.set_valign(gtk4::Align::Center);
    header_icon_box.set_halign(gtk4::Align::Center);

    let header_icon =
        babydra_ui_kit::ui::icon::get_fallback_icon(&icon_name, "application-x-executable");
    header_icon.set_pixel_size(16);
    header_icon_box.append(&header_icon);
    header.append(&header_icon_box);

    let header_label = gtk4::Label::new(Some(&app_name));
    header_label.add_css_class("taskbar-previews-header-label");
    header_label.set_hexpand(true);
    header_label.set_halign(gtk4::Align::Start);
    header_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    header.append(&header_label);

    let count_pill = gtk4::Label::new(Some(&windows.len().to_string()));
    count_pill.add_css_class("taskbar-previews-header-count");
    count_pill.set_valign(gtk4::Align::Center);
    header.append(&count_pill);

    previews_box.append(&header);

    // Separator below header
    let header_sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    header_sep.add_css_class("taskbar-preview-separator");
    previews_box.append(&header_sep);

    // 2. Window Items List
    let items_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    items_box.add_css_class("taskbar-preview-items-list");

    let active_win = babydra_core::get_active_window();
    let active_title_opt = active_win.as_ref().map(|(_, t)| t.as_str());

    let mut action_triggers = Vec::new();

    let mut title_counts: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();
    for app in windows {
        let title_str = app.window_title.as_deref().unwrap_or("");
        let raw_text = if title_str.is_empty() {
            app.name.clone()
        } else {
            title_str.to_string()
        };
        *title_counts.entry(raw_text).or_insert(0) += 1;
    }

    let mut title_indices: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();

    for app in windows {
        let item_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 4);
        item_row.add_css_class("taskbar-preview-row");
        item_row.set_hexpand(true);
        item_row.set_valign(gtk4::Align::Center);

        let preview_btn = gtk4::Button::new();
        preview_btn.add_css_class("taskbar-preview-list-btn");
        preview_btn.set_hexpand(true);
        preview_btn.set_cursor_from_name(Some("pointer"));

        let item_content = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        item_content.set_valign(gtk4::Align::Center);
        item_content.set_hexpand(true);

        let title_str = app.window_title.as_deref().unwrap_or("");
        let raw_text = if title_str.is_empty() {
            app.name.clone()
        } else {
            title_str.to_string()
        };
        let is_dup = title_counts.get(&raw_text).copied().unwrap_or(0) > 1;

        if is_dup {
            let idx = title_indices.entry(raw_text.clone()).or_insert(0);
            *idx += 1;
            let badge = gtk4::Label::new(Some(&format!("{}", *idx)));
            badge.add_css_class("taskbar-preview-index-badge");
            badge.set_valign(gtk4::Align::Center);
            item_content.append(&badge);
        } else {
            let win_icon = babydra_ui_kit::ui::icon::get_icon("window-new-symbolic", 13);
            win_icon.add_css_class("taskbar-preview-item-icon");
            win_icon.set_valign(gtk4::Align::Center);
            item_content.append(&win_icon);
        }

        let title_lbl = gtk4::Label::new(Some(&raw_text));
        title_lbl.add_css_class("taskbar-preview-title");
        title_lbl.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        title_lbl.set_max_width_chars(28);
        title_lbl.set_hexpand(true);
        title_lbl.set_halign(gtk4::Align::Start);
        item_content.append(&title_lbl);

        let is_active = if let Some(ref act) = active_title_opt {
            !title_str.is_empty()
                && (title_str == *act || act.contains(title_str) || title_str.contains(*act))
        } else {
            false
        };

        if is_active {
            item_row.add_css_class("active-window");
            let active_dot = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
            active_dot.add_css_class("taskbar-preview-active-dot");
            active_dot.set_valign(gtk4::Align::Center);
            active_dot.set_tooltip_text(Some(&babydra_core::i18n::trans("taskbar.active")));
            item_content.append(&active_dot);
        }

        preview_btn.set_child(Some(&item_content));

        let kill_btn = gtk4::Button::builder()
            .child(&babydra_ui_kit::ui::icon::get_icon(
                "window-close-symbolic",
                12,
            ))
            .tooltip_text(&babydra_core::i18n::trans("taskbar.close"))
            .build();
        kill_btn.add_css_class("taskbar-preview-list-kill-btn");
        kill_btn.set_valign(gtk4::Align::Center);
        kill_btn.set_cursor_from_name(Some("pointer"));

        item_row.append(&preview_btn);
        item_row.append(&kill_btn);

        items_box.append(&item_row);
        action_triggers.push((preview_btn, kill_btn, app.clone()));
    }

    if windows.len() > 6 {
        let scroll = gtk4::ScrolledWindow::new();
        scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
        scroll.set_max_content_height(280);
        scroll.set_propagate_natural_height(true);
        scroll.set_child(Some(&items_box));
        previews_box.append(&scroll);
    } else {
        previews_box.append(&items_box);
    }

    // 3. Footer Actions
    let mut open_new_info = None;
    let mut close_all_btn_opt = None;

    if !windows.is_empty() {
        let (app_name, exec_cmd, _icon_name) = if let Some(first_app) = windows.first() {
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

        let actions_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
        actions_box.add_css_class("taskbar-preview-actions-box");

        let open_new_btn = gtk4::Button::new();
        open_new_btn.add_css_class("taskbar-preview-action-btn");
        open_new_btn.set_hexpand(true);
        open_new_btn.set_cursor_from_name(Some("pointer"));

        let open_new_content = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        open_new_content.set_halign(gtk4::Align::Start);
        open_new_content.set_valign(gtk4::Align::Center);

        let open_new_icon = babydra_ui_kit::ui::icon::get_icon("list-add-symbolic", 13);
        open_new_icon.add_css_class("taskbar-preview-add-icon");
        let open_label_text = format!(
            "{} {}",
            babydra_core::i18n::trans("taskbar.open_new"),
            app_name
        );
        let open_new_label = gtk4::Label::new(Some(&open_label_text));
        open_new_label.add_css_class("taskbar-preview-action-label");
        open_new_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        open_new_label.set_max_width_chars(26);
        open_new_label.set_hexpand(true);
        open_new_label.set_halign(gtk4::Align::Start);
        open_new_content.append(&open_new_icon);
        open_new_content.append(&open_new_label);
        open_new_btn.set_child(Some(&open_new_content));
        actions_box.append(&open_new_btn);
        open_new_info = Some((open_new_btn, exec_cmd));

        if windows.len() > 1 {
            let close_all_btn = gtk4::Button::new();
            close_all_btn.add_css_class("taskbar-preview-action-btn");
            close_all_btn.add_css_class("taskbar-preview-close-all");
            close_all_btn.set_hexpand(true);
            close_all_btn.set_cursor_from_name(Some("pointer"));

            let close_all_content = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
            close_all_content.set_halign(gtk4::Align::Start);
            close_all_content.set_valign(gtk4::Align::Center);

            let close_all_icon = babydra_ui_kit::ui::icon::get_icon("window-close-symbolic", 13);
            close_all_icon.add_css_class("taskbar-preview-action-icon");

            let close_all_label =
                gtk4::Label::new(Some(&babydra_core::i18n::trans("taskbar.close_all")));
            close_all_label.add_css_class("taskbar-preview-action-label");
            close_all_label.add_css_class("close-all-text");
            close_all_content.append(&close_all_icon);
            close_all_content.append(&close_all_label);
            close_all_btn.set_child(Some(&close_all_content));
            actions_box.append(&close_all_btn);
            close_all_btn_opt = Some(close_all_btn);
        }

        previews_box.append(&actions_box);
    }

    popover.set_child(Some(&previews_box));

    (action_triggers, open_new_info, close_all_btn_opt)
}
