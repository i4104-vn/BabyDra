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
    let shell = DialogShell::new(&title, 380, 140, 12, parent);
    let msg = babydra_core::i18n::trans("explore.dialog_conflict_msg").replace("{}", item_name);
    let lbl = shell.add_label(&msg);
    lbl.set_wrap(true);
    lbl.set_max_width_chars(45);

    let bbox = shell.add_button_row();
    shell.cancel_button(&bbox);
    let btn_override = {
        let btn = gtk4::Button::builder()
            .label(babydra_core::i18n::trans("explore.dialog_override"))
            .css_classes(vec![
                "suggested-action".to_string(),
                "destructive-action".to_string(),
            ])
            .build();
        bbox.append(&btn);
        btn
    };

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
