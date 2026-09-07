use gtk4::prelude::*;

use super::shell::DialogShell;

/// Presents a delete confirmation dialog. Calls `on_confirm` if the user clicks "Delete".
pub fn show_delete_confirm(
    title: &str,
    message: &str,
    on_confirm: impl Fn() + 'static,
    parent: Option<&impl IsA<gtk4::Window>>,
) {
    let shell = DialogShell::new(title, 380, 160, 12, parent);
    shell.add_header(
        "trash",
        super::shell::BadgeStyle::Danger,
        title,
        Some(message),
    );
    let bbox = shell.add_button_row();
    shell.cancel_button(&bbox);
    let btn_confirm = shell.danger_button(&bbox, &babydra_core::i18n::trans("explore.settings_delete"));

    let confirm_cb = std::rc::Rc::new(on_confirm);
    let win = shell.window.clone();
    btn_confirm.connect_clicked(move |_| {
        confirm_cb();
        win.close();
    });

    shell.window.present();
}
