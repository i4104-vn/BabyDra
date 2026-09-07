use gtk4::prelude::*;

use super::shell::DialogShell;

/// Presents a conflict dialog informing the user that a target file or folder already exists.
/// Offers options to Cancel or Override (Replace).
pub fn show_conflict_dialog(
    item_name: &str,
    on_override: impl FnOnce() + 'static,
    parent: Option<&impl IsA<gtk4::Window>>,
) {
    let title = babydra_core::i18n::trans("explore.dialog_conflict_title");
    let shell = DialogShell::new(&title, 420, 200, 14, parent);
    let msg = babydra_core::i18n::trans("explore.dialog_conflict_msg").replace("{}", item_name);
    shell.add_header(
        "info",
        super::shell::BadgeStyle::Warning,
        &title,
        Some(&msg),
    );

    let bbox = shell.add_button_row();
    shell.cancel_button(&bbox);
    let btn_override = shell.danger_button(&bbox, &babydra_core::i18n::trans("explore.dialog_override"));

    let override_cb = std::rc::Rc::new(std::cell::RefCell::new(Some(on_override)));
    let win = shell.window.clone();
    btn_override.connect_clicked(move |_| {
        if let Some(cb) = override_cb.borrow_mut().take() {
            cb();
        }
        win.close();
    });

    shell.window.present();
}
