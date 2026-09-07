use gtk4::prelude::*;

use super::shell::DialogShell;

/// Presents a simple error/alert modal dialog with a Close/OK button.
pub fn show_alert_dialog(title: &str, message: &str, parent: Option<&impl IsA<gtk4::Window>>) {
    let shell = DialogShell::new(title, 380, 160, 12, parent);
    shell.add_header("info", super::shell::BadgeStyle::Primary, title, Some(message));
    let bbox = shell.add_button_row();
    let btn_ok = shell.action_button(&bbox, &babydra_core::i18n::trans("explore.settings_close"));

    // Genie close animation
    let is_animating = std::rc::Rc::new(std::cell::Cell::new(false));
    {
        let win_ok = shell.window.clone();
        btn_ok.connect_clicked(move |_| {
            win_ok.close();
        });
    }
    let win_cancel = shell.window.clone();
    let vbox_cancel = shell.vbox.clone();
    let is_animating_close = is_animating.clone();
    shell.window.connect_close_request(move |_| {
        if is_animating_close.get() {
            return glib::Propagation::Stop;
        }
        is_animating_close.set(true);
        let win_cb = win_cancel.clone();
        crate::ui::animation::genie_out(vbox_cancel.upcast_ref(), 340, 130, 300, move || {
            win_cb.destroy();
        });
        glib::Propagation::Stop
    });

    shell.window.present();
    crate::ui::animation::genie_in(shell.vbox.upcast_ref(), 340, 130, 300);
}
