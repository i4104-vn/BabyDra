//! Application Manager UI layout generator matching reference design Image 5.

use crate::widgets::apps::handler::PendingAction;
use crate::widgets::state::AppsWidget;
use babydra_core::models::settings::AppActionType;
use babydra_ui_kit::components::modals::PasswordDialog;
use gtk4::prelude::*;
use gtk4::{
    Box, Button, Entry, Label, ListBox, Orientation, Overlay, ProgressBar, ScrolledWindow, Stack,
    TextView,
};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct AppItemData {
    pub name: String,
    pub description: String,
    pub icon: Option<String>,
    pub pkg_name: String,
    pub can_downgrade: bool,
}

#[derive(Clone, Debug)]
pub struct PkgItemData {
    pub name: String,
    pub version: String,
    pub can_downgrade: bool,
}

/// Builds the base UI structure of the Apps page (empty list boxes initially).
pub fn build() -> (AppsWidget, PasswordDialog) {
    let root = Overlay::new();
    root.set_hexpand(true);

    let container = Box::new(Orientation::Vertical, 16);
    container.set_hexpand(true);
    container.set_vexpand(true);
    container.set_valign(gtk4::Align::Fill);

    // Title + Search Input Header Row
    let header_box = Box::new(Orientation::Horizontal, 12);
    header_box.set_margin_bottom(4);

    let title_label = Label::new(Some(&babydra_core::i18n::trans("settings.apps_title")));
    title_label.add_css_class("settings-page-title");
    title_label.set_hexpand(true);
    title_label.set_halign(gtk4::Align::Start);

    let search_entry = Entry::new();
    search_entry.set_placeholder_text(Some(&babydra_core::i18n::trans(
        "settings.apps_search_placeholder",
    )));
    search_entry.add_css_class("sidebar-search-entry");
    search_entry.set_width_request(220);
    search_entry.set_valign(gtk4::Align::Center);

    let refresh_btn = Button::new();
    refresh_btn.add_css_class("icon-btn");
    refresh_btn.add_css_class("circular");
    refresh_btn.set_cursor_from_name(Some("pointer"));
    refresh_btn.set_valign(gtk4::Align::Center);
    refresh_btn.set_size_request(34, 34);
    let refresh_icon = babydra_ui_kit::ui::icon::get_icon("refresh", 16);
    refresh_icon.set_pixel_size(16);
    refresh_btn.set_child(Some(&refresh_icon));
    refresh_btn.set_tooltip_text(Some("Refresh application list"));

    header_box.append(&title_label);
    header_box.append(&search_entry);
    header_box.append(&refresh_btn);
    container.append(&header_box);

    // Tabs Bar Row
    let tabs_box = Box::new(Orientation::Horizontal, 12);
    let tab_apps_btn = Button::with_label(&babydra_core::i18n::trans("settings.apps_tab_apps"));
    tab_apps_btn.add_css_class("app-tab-btn");
    tab_apps_btn.add_css_class("active");
    tab_apps_btn.set_cursor_from_name(Some("pointer"));

    let tab_packages_btn =
        Button::with_label(&babydra_core::i18n::trans("settings.apps_tab_packages"));
    tab_packages_btn.add_css_class("app-tab-btn");
    tab_packages_btn.set_cursor_from_name(Some("pointer"));

    tabs_box.append(&tab_apps_btn);
    tabs_box.append(&tab_packages_btn);
    container.append(&tabs_box);

    // Stack for Apps vs Packages view
    let stack = Stack::new();
    stack.set_hexpand(true);
    stack.set_transition_type(gtk4::StackTransitionType::Crossfade);
    stack.set_vexpand(true);
    stack.set_valign(gtk4::Align::Fill);

    // 1. Apps List (Glass Panel List Box)
    let apps_glass_card = Box::new(Orientation::Vertical, 0);
    apps_glass_card.add_css_class("glass-panel");
    apps_glass_card.set_hexpand(true);
    apps_glass_card.set_vexpand(true);
    apps_glass_card.set_valign(gtk4::Align::Fill);

    let apps_scrolled = ScrolledWindow::new();
    apps_scrolled.set_hexpand(true);
    apps_scrolled.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    apps_scrolled.set_vexpand(true);
    apps_scrolled.set_valign(gtk4::Align::Fill);

    let apps_list_box = ListBox::new();
    apps_list_box.set_selection_mode(gtk4::SelectionMode::None);

    apps_scrolled.set_child(Some(&apps_list_box));
    apps_glass_card.append(&apps_scrolled);
    stack.add_named(&apps_glass_card, Some("apps"));

    // 2. Packages List
    let pkgs_glass_card = Box::new(Orientation::Vertical, 0);
    pkgs_glass_card.add_css_class("glass-panel");
    pkgs_glass_card.set_hexpand(true);
    pkgs_glass_card.set_vexpand(true);
    pkgs_glass_card.set_valign(gtk4::Align::Fill);

    let pkgs_scrolled = ScrolledWindow::new();
    pkgs_scrolled.set_hexpand(true);
    pkgs_scrolled.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    pkgs_scrolled.set_vexpand(true);
    pkgs_scrolled.set_valign(gtk4::Align::Fill);

    let pkgs_list_box = ListBox::new();
    pkgs_list_box.set_selection_mode(gtk4::SelectionMode::None);

    pkgs_scrolled.set_child(Some(&pkgs_list_box));
    pkgs_glass_card.append(&pkgs_scrolled);
    stack.add_named(&pkgs_glass_card, Some("packages"));

    container.append(&stack);

    // Console Log Modal Dialog Overlay (auth-dialog-card style)
    let console_card = Box::new(Orientation::Vertical, 14);
    console_card.add_css_class("auth-dialog-card");
    console_card.set_halign(gtk4::Align::Center);
    console_card.set_valign(gtk4::Align::Center);
    console_card.set_width_request(480);
    console_card.set_visible(false);

    let console_header = Box::new(Orientation::Horizontal, 10);
    console_header.set_hexpand(true);

    let console_icon = babydra_ui_kit::ui::icon::get_icon("terminal", 18);
    console_icon.set_pixel_size(18);
    console_header.append(&console_icon);

    let console_title_lbl = Label::new(Some(&babydra_core::i18n::trans(
        "settings.apps_uninstall_log_title",
    )));
    console_title_lbl.add_css_class("settings-row-title");
    console_title_lbl.set_hexpand(true);
    console_title_lbl.set_halign(gtk4::Align::Start);
    console_header.append(&console_title_lbl);

    console_card.append(&console_header);

    let progress_bar = ProgressBar::new();
    progress_bar.add_css_class("console-progress");
    progress_bar.set_fraction(0.0);

    console_card.append(&progress_bar);

    let text_view = TextView::new();
    text_view.set_editable(false);
    text_view.set_cursor_visible(false);
    text_view.set_monospace(true);
    text_view.add_css_class("console-log-text");

    let text_buffer = text_view.buffer();

    let console_scroll = ScrolledWindow::new();
    console_scroll.set_policy(gtk4::PolicyType::Automatic, gtk4::PolicyType::Automatic);
    console_scroll.set_height_request(240);
    console_scroll.set_child(Some(&text_view));

    console_card.append(&console_scroll);

    // Footer Actions Row
    let actions_box = Box::new(Orientation::Horizontal, 8);
    actions_box.set_halign(gtk4::Align::End);

    let console_close_btn =
        Button::with_label(&babydra_core::i18n::trans("explore.settings_close"));
    console_close_btn.add_css_class("connect-pill-btn");
    console_close_btn.set_cursor_from_name(Some("pointer"));
    actions_box.append(&console_close_btn);

    console_card.append(&actions_box);

    root.set_child(Some(&container));

    // Reusable Password Dialog & Console Log Dialog Overlays
    let auth_dialog = PasswordDialog::new(
        "Uninstall Authentication",
        "Enter sudo password to confirm package removal:",
    );
    root.add_overlay(&auth_dialog.container);
    root.add_overlay(&console_card);

    let widget = AppsWidget {
        root,
        container,
        search_entry,
        refresh_btn,
        tab_apps_btn,
        tab_packages_btn,
        stack,
        apps_list_box,
        pkgs_list_box,
        console_card,
        console_title_lbl,
        console_close_btn,
        progress_bar,
        text_view,
        text_buffer,
        console_scroll,
    };

    (widget, auth_dialog)
}

/// Helper to wire action buttons (Uninstall / Downgrade) on each row.
fn wire_action_btn(
    btn: &Button,
    action_type: AppActionType,
    pkg_name: String,
    row_box: &Box,
    parent_list: &ListBox,
    auth_dialog_rc: &Rc<PasswordDialog>,
    pending_action: &Rc<RefCell<Option<PendingAction>>>,
) {
    let auth_dialog_c = auth_dialog_rc.clone();
    let pending_c = pending_action.clone();
    let row_box_c = row_box.clone();
    let parent_list_c = parent_list.clone();

    btn.connect_clicked(move |_| {
        if action_type == AppActionType::Downgrade
            && babydra_core::services::apps::pacman::find_cached_pkg(&pkg_name).is_none() {
                let msg = babydra_core::i18n::trans("settings.apps_downgrade_not_found")
                    .replace("{}", &pkg_name);
                babydra_core::send_settings_notif(
                    &babydra_core::i18n::trans("settings.apps_downgrade_log_title"),
                    &msg,
                );
                return;
            }

        *pending_c.borrow_mut() = Some(PendingAction {
            action_type: action_type.clone(),
            pkg_name: pkg_name.clone(),
            row_box: row_box_c.clone(),
            parent_list: parent_list_c.clone(),
        });

        let (title, prompt) = match action_type {
            AppActionType::Uninstall => (
                "Uninstall Authentication",
                format!("Enter sudo password to uninstall '{}':", pkg_name),
            ),
            AppActionType::Downgrade => (
                "Downgrade Authentication",
                format!(
                    "Enter sudo password to downgrade '{}' to cached version:",
                    pkg_name
                ),
            ),
        };

        auth_dialog_c.show_for(title, &prompt);
    });
}

/// Renders installed applications into the apps ListBox.
pub fn render_apps_list(
    list_box: &ListBox,
    apps: &[AppItemData],
    is_loading: bool,
    auth_dialog_rc: &Rc<PasswordDialog>,
    pending_action: Rc<RefCell<Option<PendingAction>>>,
) {
    crate::widgets::helpers::clear_list_box(list_box);

    if is_loading && apps.is_empty() {
        list_box.append(&crate::widgets::helpers::create_placeholder(
            crate::widgets::helpers::PlaceholderState::Loading,
        ));
        return;
    }

    if apps.is_empty() {
        list_box.append(&crate::widgets::helpers::create_placeholder(
            crate::widgets::helpers::PlaceholderState::Empty {
                title_key: "settings.apps_no_apps",
                desc_key: None,
                icon_name: "application-x-executable",
            },
        ));
        return;
    }

    for app in apps {
        let row_box = Box::new(Orientation::Horizontal, 14);
        row_box.add_css_class("settings-card-row");
        row_box.set_margin_top(4);
        row_box.set_margin_bottom(4);
        row_box.set_margin_start(8);
        row_box.set_margin_end(8);

        // App Icon Box Container
        let icon_box = Box::new(Orientation::Vertical, 0);
        icon_box.add_css_class("blue-icon-badge-sm");
        icon_box.set_valign(gtk4::Align::Center);
        icon_box.set_halign(gtk4::Align::Start);

        let icon_name = app
            .icon
            .as_deref()
            .filter(|s| !s.is_empty())
            .unwrap_or("application-x-executable");
        let icon_img =
            babydra_ui_kit::ui::icon::get_fallback_icon(icon_name, "application-x-executable");
        icon_img.set_pixel_size(18);
        icon_img.set_valign(gtk4::Align::Center);
        icon_img.set_halign(gtk4::Align::Center);
        icon_img.set_vexpand(true);
        icon_box.append(&icon_img);
        row_box.append(&icon_box);

        // App Info Column (Title + Subtitle)
        let text_box = Box::new(Orientation::Vertical, 2);
        text_box.set_hexpand(true);
        text_box.set_valign(gtk4::Align::Center);

        let name_lbl = Label::new(Some(&app.name));
        name_lbl.add_css_class("settings-row-title");
        name_lbl.set_halign(gtk4::Align::Start);
        text_box.append(&name_lbl);

        let desc_lbl = Label::new(Some(&app.description));
        desc_lbl.add_css_class("settings-row-desc");
        desc_lbl.set_halign(gtk4::Align::Start);
        text_box.append(&desc_lbl);

        row_box.append(&text_box);

        if app.can_downgrade {
            let downgrade_btn = Button::new();
            downgrade_btn.add_css_class("icon-btn");
            downgrade_btn.add_css_class("circular");
            downgrade_btn.add_css_class("downgrade-btn");
            downgrade_btn.set_valign(gtk4::Align::Center);
            downgrade_btn.set_cursor_from_name(Some("pointer"));
            downgrade_btn.set_tooltip_text(Some("Downgrade package version"));

            let dl_icon = babydra_ui_kit::ui::icon::get_icon("folder-download", 16);
            dl_icon.set_pixel_size(16);
            downgrade_btn.set_child(Some(&dl_icon));

            wire_action_btn(
                &downgrade_btn,
                AppActionType::Downgrade,
                app.pkg_name.clone(),
                &row_box,
                list_box,
                auth_dialog_rc,
                &pending_action,
            );

            row_box.append(&downgrade_btn);
        }

        // X Uninstall Action Button
        let uninstall_btn = Button::new();
        uninstall_btn.add_css_class("icon-btn");
        uninstall_btn.add_css_class("circular");
        uninstall_btn.add_css_class("delete-btn");
        uninstall_btn.set_valign(gtk4::Align::Center);
        uninstall_btn.set_cursor_from_name(Some("pointer"));
        uninstall_btn.set_tooltip_text(Some("Uninstall package"));

        let x_icon = babydra_ui_kit::ui::icon::get_icon("edit-delete", 16);
        x_icon.set_pixel_size(16);
        uninstall_btn.set_child(Some(&x_icon));

        wire_action_btn(
            &uninstall_btn,
            AppActionType::Uninstall,
            app.pkg_name.clone(),
            &row_box,
            list_box,
            auth_dialog_rc,
            &pending_action,
        );

        row_box.append(&uninstall_btn);
        list_box.append(&row_box);
    }
}

/// Renders installed packages into the packages ListBox.
pub fn render_pkgs_list(
    list_box: &ListBox,
    pkgs: &[PkgItemData],
    is_loading: bool,
    auth_dialog_rc: &Rc<PasswordDialog>,
    pending_action: Rc<RefCell<Option<PendingAction>>>,
) {
    crate::widgets::helpers::clear_list_box(list_box);

    if is_loading && pkgs.is_empty() {
        list_box.append(&crate::widgets::helpers::create_placeholder(
            crate::widgets::helpers::PlaceholderState::Loading,
        ));
        return;
    }

    if pkgs.is_empty() {
        list_box.append(&crate::widgets::helpers::create_placeholder(
            crate::widgets::helpers::PlaceholderState::Empty {
                title_key: "settings.apps_no_packages",
                desc_key: None,
                icon_name: "package-x-generic",
            },
        ));
        return;
    }

    for pkg in pkgs {
        let row_box = Box::new(Orientation::Horizontal, 14);
        row_box.add_css_class("settings-card-row");
        row_box.set_margin_top(4);
        row_box.set_margin_bottom(4);
        row_box.set_margin_start(8);
        row_box.set_margin_end(8);

        // Package Icon Box Container
        let icon_box = Box::new(Orientation::Vertical, 0);
        icon_box.add_css_class("blue-icon-badge-sm");
        icon_box.set_valign(gtk4::Align::Center);
        icon_box.set_halign(gtk4::Align::Start);

        let icon_img =
            babydra_ui_kit::ui::icon::get_fallback_icon(&pkg.name, "application-x-executable");
        icon_img.set_pixel_size(18);
        icon_img.set_valign(gtk4::Align::Center);
        icon_img.set_halign(gtk4::Align::Center);
        icon_img.set_vexpand(true);
        icon_box.append(&icon_img);
        row_box.append(&icon_box);

        // Package Info Column (Name + Version)
        let text_box = Box::new(Orientation::Vertical, 2);
        text_box.set_hexpand(true);
        text_box.set_valign(gtk4::Align::Center);

        let name_lbl = Label::new(Some(&pkg.name));
        name_lbl.add_css_class("settings-row-title");
        name_lbl.set_halign(gtk4::Align::Start);
        text_box.append(&name_lbl);

        let ver_lbl = Label::new(Some(&pkg.version));
        ver_lbl.add_css_class("settings-row-desc");
        ver_lbl.set_halign(gtk4::Align::Start);
        text_box.append(&ver_lbl);

        row_box.append(&text_box);

        if pkg.can_downgrade {
            let downgrade_btn = Button::new();
            downgrade_btn.add_css_class("icon-btn");
            downgrade_btn.add_css_class("circular");
            downgrade_btn.add_css_class("downgrade-btn");
            downgrade_btn.set_valign(gtk4::Align::Center);
            downgrade_btn.set_cursor_from_name(Some("pointer"));
            downgrade_btn.set_tooltip_text(Some("Downgrade package version"));

            let dl_icon = babydra_ui_kit::ui::icon::get_icon("folder-download", 16);
            dl_icon.set_pixel_size(16);
            downgrade_btn.set_child(Some(&dl_icon));

            wire_action_btn(
                &downgrade_btn,
                AppActionType::Downgrade,
                pkg.name.clone(),
                &row_box,
                list_box,
                auth_dialog_rc,
                &pending_action,
            );

            row_box.append(&downgrade_btn);
        }

        // X Uninstall Action Button
        let uninstall_btn = Button::new();
        uninstall_btn.add_css_class("icon-btn");
        uninstall_btn.add_css_class("circular");
        uninstall_btn.add_css_class("delete-btn");
        uninstall_btn.set_valign(gtk4::Align::Center);
        uninstall_btn.set_cursor_from_name(Some("pointer"));
        uninstall_btn.set_tooltip_text(Some("Uninstall package"));

        let x_icon = babydra_ui_kit::ui::icon::get_icon("edit-delete", 16);
        x_icon.set_pixel_size(16);
        uninstall_btn.set_child(Some(&x_icon));

        wire_action_btn(
            &uninstall_btn,
            AppActionType::Uninstall,
            pkg.name.clone(),
            &row_box,
            list_box,
            auth_dialog_rc,
            &pending_action,
        );

        row_box.append(&uninstall_btn);
        list_box.append(&row_box);
    }
}
