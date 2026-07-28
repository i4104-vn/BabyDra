use gtk4::prelude::*;

pub mod password_dialog;
pub mod wifi_info_dialog;
pub mod wifi_password_dialog;
pub mod wifi_config_dialog;

pub use password_dialog::PasswordDialog;
pub use wifi_info_dialog::WifiInfoDialog;
pub use wifi_password_dialog::WifiPasswordDialog;
pub use wifi_config_dialog::WifiConfigDialog;

/// Creates a standardized dialog overlay box container.
pub fn create_dialog_box(title: &str, content: &impl IsA<gtk4::Widget>) -> gtk4::Box {
    let dialog_box = gtk4::Box::new(gtk4::Orientation::Vertical, 16);
    dialog_box.add_css_class("cheatsheet-overlay");
    dialog_box.set_halign(gtk4::Align::Center);
    dialog_box.set_valign(gtk4::Align::Center);

    let title_lbl = gtk4::Label::new(Some(title));
    title_lbl.add_css_class("cheatsheet-title");
    dialog_box.append(&title_lbl);
    dialog_box.append(content);

    dialog_box
}
