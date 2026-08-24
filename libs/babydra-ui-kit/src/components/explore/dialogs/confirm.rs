use gtk4::prelude::*;

use super::shell::DialogShell;

/// Presents a delete confirmation dialog. Calls `on_confirm` if the user clicks "Delete".
pub fn show_delete_confirm(
    title: &str,
    message: &str,
    on_confirm: impl Fn() + 'static,
    parent: Option<&impl IsA<gtk4::Window>>,
) {
    let shell = DialogShell::new(title, 360, 120, 12, parent);
    shell.add_label(message);
    let bbox = shell.add_button_row();
    shell.cancel_button(&bbox);
    let btn_confirm = {
        let btn = gtk4::Button::builder()
            .label(babydra_core::i18n::trans("explore.settings_delete"))
            .css_classes(vec!["destructive-action".to_string()])
            .build();
        bbox.append(&btn);
        btn
    };

    let confirm_cb = std::rc::Rc::new(on_confirm);
    let win = shell.window.clone();
    btn_confirm.connect_clicked(move |_| {
        confirm_cb();
        win.close();
    });

    shell.window.present();
}
