//! Device & Account Settings card UI builder.

use babydra_core::i18n::trans;
use babydra_ui_kit::components::cards::create_collapsible_card;
use babydra_ui_kit::components::{ChangeHostnameDialog, ChangePasswordDialog};
use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Button, Label};

/// Holds widget references for the Device & Account card.
pub struct DeviceAccountWidgets {
    pub container: GtkBox,
    pub host_row_lbl: Label,
    pub edit_host_btn: Button,
    pub change_pwd_btn: Button,
    pub change_hostname_dialog: ChangeHostnameDialog,
    pub change_password_dialog: ChangePasswordDialog,
}

/// Renders the Device & Account Settings collapsible card.
pub fn render_device_account_card() -> DeviceAccountWidgets {
    let card = create_collapsible_card(
        &trans("settings.section_device_account"),
        None,
        Some("user"),
        false,
    );

    let content = card.content.clone();

    // Get current hostname
    let hostname = babydra_core::services::system::account::get_system_hostname();
    let display_host = if !hostname.is_empty() && hostname != "localhost" {
        &hostname
    } else {
        "BabyDra Linux"
    };

    // Row 1: Hostname
    let host_row = GtkBox::new(gtk4::Orientation::Horizontal, 14);
    host_row.set_margin_top(8);
    host_row.set_margin_bottom(8);
    host_row.set_margin_start(8);
    host_row.set_margin_end(8);

    let host_icon_badge = GtkBox::new(gtk4::Orientation::Vertical, 0);
    host_icon_badge.add_css_class("blue-icon-badge-sm");
    host_icon_badge.set_valign(Align::Center);
    host_icon_badge.set_halign(Align::Start);
    host_icon_badge.set_hexpand(false);
    host_icon_badge.set_vexpand(false);
    host_icon_badge.set_size_request(36, 36);

    let host_icon = babydra_ui_kit::ui::icon::get_icon("desktop", 18);
    host_icon.set_pixel_size(18);
    host_icon.set_valign(Align::Center);
    host_icon.set_halign(Align::Center);
    host_icon.set_vexpand(true);
    host_icon_badge.append(&host_icon);
    host_row.append(&host_icon_badge);

    let host_text_col = GtkBox::new(gtk4::Orientation::Vertical, 2);
    host_text_col.set_valign(Align::Center);
    host_text_col.set_halign(Align::Start);
    host_text_col.set_hexpand(true);

    let host_title = Label::new(Some(&trans("settings.device_hostname")));
    host_title.add_css_class("settings-row-title");
    host_title.set_halign(Align::Start);
    host_text_col.append(&host_title);

    let host_row_lbl = Label::new(Some(display_host));
    host_row_lbl.add_css_class("settings-row-desc");
    host_row_lbl.set_halign(Align::Start);
    host_text_col.append(&host_row_lbl);
    host_row.append(&host_text_col);

    let edit_host_btn = Button::with_label(&trans("settings.change_hostname"));
    edit_host_btn.add_css_class("connect-pill-btn");
    edit_host_btn.set_cursor_from_name(Some("pointer"));
    edit_host_btn.set_valign(Align::Center);
    host_row.append(&edit_host_btn);

    content.append(&host_row);

    // Row 2: Account Password
    let pwd_row = GtkBox::new(gtk4::Orientation::Horizontal, 14);
    pwd_row.set_margin_top(8);
    pwd_row.set_margin_bottom(8);
    pwd_row.set_margin_start(8);
    pwd_row.set_margin_end(8);

    let pwd_icon_badge = GtkBox::new(gtk4::Orientation::Vertical, 0);
    pwd_icon_badge.add_css_class("blue-icon-badge-sm");
    pwd_icon_badge.set_valign(Align::Center);
    pwd_icon_badge.set_halign(Align::Start);
    pwd_icon_badge.set_hexpand(false);
    pwd_icon_badge.set_vexpand(false);
    pwd_icon_badge.set_size_request(36, 36);

    let pwd_icon = babydra_ui_kit::ui::icon::get_icon("lock", 18);
    pwd_icon.set_pixel_size(18);
    pwd_icon.set_valign(Align::Center);
    pwd_icon.set_halign(Align::Center);
    pwd_icon.set_vexpand(true);
    pwd_icon_badge.append(&pwd_icon);
    pwd_row.append(&pwd_icon_badge);

    let pwd_text_col = GtkBox::new(gtk4::Orientation::Vertical, 2);
    pwd_text_col.set_valign(Align::Center);
    pwd_text_col.set_halign(Align::Start);
    pwd_text_col.set_hexpand(true);

    let pwd_title = Label::new(Some(&trans("settings.account_password")));
    pwd_title.add_css_class("settings-row-title");
    pwd_title.set_halign(Align::Start);
    pwd_text_col.append(&pwd_title);

    let pwd_sub = Label::new(Some(&trans("settings.password_protected")));
    pwd_sub.add_css_class("settings-row-desc");
    pwd_sub.set_halign(Align::Start);
    pwd_text_col.append(&pwd_sub);
    pwd_row.append(&pwd_text_col);

    let change_pwd_btn = Button::with_label(&trans("settings.change_password"));
    change_pwd_btn.add_css_class("connect-pill-btn");
    change_pwd_btn.set_cursor_from_name(Some("pointer"));
    change_pwd_btn.set_valign(Align::Center);
    pwd_row.append(&change_pwd_btn);

    content.append(&pwd_row);

    // Modal dialogs
    let change_hostname_dialog = ChangeHostnameDialog::new();
    let change_password_dialog = ChangePasswordDialog::new();

    DeviceAccountWidgets {
        container: card.container,
        host_row_lbl,
        edit_host_btn,
        change_pwd_btn,
        change_hostname_dialog,
        change_password_dialog,
    }
}
