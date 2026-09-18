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
        Some(&trans("settings.device_account_subtitle")),
        Some("user"),
        false,
    );

    let content = card.content;

    // Get current hostname
    let hostname = babydra_core::services::system::account::get_system_hostname();
    let display_host = if !hostname.is_empty() && hostname != "localhost" {
        &hostname
    } else {
        "BabyDra Linux"
    };

    // Row 1: Hostname
    let host_row = GtkBox::new(gtk4::Orientation::Horizontal, 12);
    host_row.add_css_class("settings-card-row");
    host_row.set_margin_top(6);
    host_row.set_margin_bottom(6);
    host_row.set_margin_start(8);
    host_row.set_margin_end(8);

    let host_icon = babydra_ui_kit::ui::icon::get_icon("desktop", 20);
    host_icon.set_pixel_size(20);
    host_icon.set_valign(Align::Center);
    host_row.append(&host_icon);

    let host_text_col = GtkBox::new(gtk4::Orientation::Vertical, 2);
    host_text_col.set_valign(Align::Center);
    let host_title = Label::new(Some(&trans("settings.device_hostname")));
    host_title.add_css_class("settings-label");
    host_title.set_halign(Align::Start);
    host_text_col.append(&host_title);

    let host_row_lbl = Label::new(Some(display_host));
    host_row_lbl.add_css_class("settings-desc");
    host_row_lbl.set_halign(Align::Start);
    host_text_col.append(&host_row_lbl);
    host_row.append(&host_text_col);

    let spacer = GtkBox::new(gtk4::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    host_row.append(&spacer);

    let edit_host_btn = Button::with_label(&trans("settings.change_hostname"));
    edit_host_btn.add_css_class("connect-pill-btn");
    edit_host_btn.set_cursor_from_name(Some("pointer"));
    edit_host_btn.set_valign(Align::Center);
    host_row.append(&edit_host_btn);

    content.append(&host_row);

    // Row 2: Account Password
    let pwd_row = GtkBox::new(gtk4::Orientation::Horizontal, 12);
    pwd_row.add_css_class("settings-card-row");
    pwd_row.set_margin_top(6);
    pwd_row.set_margin_bottom(6);
    pwd_row.set_margin_start(8);
    pwd_row.set_margin_end(8);

    let pwd_icon = babydra_ui_kit::ui::icon::get_icon("lock", 20);
    pwd_icon.set_pixel_size(20);
    pwd_icon.set_valign(Align::Center);
    pwd_row.append(&pwd_icon);

    let pwd_text_col = GtkBox::new(gtk4::Orientation::Vertical, 2);
    pwd_text_col.set_valign(Align::Center);
    let pwd_title = Label::new(Some(&trans("settings.account_password")));
    pwd_title.add_css_class("settings-label");
    pwd_title.set_halign(Align::Start);
    pwd_text_col.append(&pwd_title);

    let pwd_sub = Label::new(Some(&trans("settings.password_protected")));
    pwd_sub.add_css_class("settings-desc");
    pwd_sub.set_halign(Align::Start);
    pwd_text_col.append(&pwd_sub);
    pwd_row.append(&pwd_text_col);

    let spacer2 = GtkBox::new(gtk4::Orientation::Horizontal, 0);
    spacer2.set_hexpand(true);
    pwd_row.append(&spacer2);

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
