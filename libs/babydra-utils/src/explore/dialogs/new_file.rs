use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Entry, Button, Align, Window};
use std::path::PathBuf;
use std::rc::Rc;

use babydra_common::i18n::t;

/// Presents a dialog window to create a new empty file under a directory.
pub fn show_new_file_dialog(
    current_path: PathBuf,
    nav_callback: Rc<dyn Fn(PathBuf)>,
    parent: Option<&impl IsA<gtk4::Window>>,
) {
    let window = Window::builder()
        .title(&t("explore.dialog_new_file_title"))
        .modal(true)
        .resizable(false)
        .default_width(320)
        .default_height(140)
        .css_classes(vec!["explore-dialog".to_string()])
        .build();

    if let Some(p) = parent {
        window.set_transient_for(Some(p));
    }

    let vbox = Box::new(Orientation::Vertical, 12);
    vbox.add_css_class("explore-dialog-box");
    vbox.set_margin_top(16);
    vbox.set_margin_bottom(16);
    vbox.set_margin_start(16);
    vbox.set_margin_end(16);
    window.set_child(Some(&vbox));

    let lbl = Label::builder()
        .label(&t("explore.dialog_new_file_label"))
        .halign(Align::Start)
        .build();
    vbox.append(&lbl);

    let entry = Entry::new();
    entry.set_text(&t("explore.menu_new_file"));
    entry.set_hexpand(true);
    vbox.append(&entry);

    let bbox = Box::new(Orientation::Horizontal, 8);
    bbox.set_halign(Align::End);
    vbox.append(&bbox);

    let btn_cancel = Button::with_label(&t("explore.settings_cancel"));
    let btn_create = Button::builder()
        .label(&t("explore.settings_add"))
        .css_classes(vec!["suggested-action".to_string()])
        .build();

    bbox.append(&btn_cancel);
    bbox.append(&btn_create);

    let win_cancel_btn = window.clone();
    btn_cancel.connect_clicked(move |_| {
        win_cancel_btn.close();
    });

    let win_cancel = window.clone();
    let vbox_cancel = vbox.clone();
    let is_animating = Rc::new(std::cell::Cell::new(false));
    let is_animating_cancel = is_animating.clone();
    window.connect_close_request(move |_| {
        if is_animating_cancel.get() {
            return glib::Propagation::Stop;
        }
        is_animating_cancel.set(true);
        let win_cb = win_cancel.clone();
        crate::ui::animation::genie_out(
            vbox_cancel.upcast_ref(),
            320,
            140,
            300,
            move || {
                win_cb.destroy();
            }
        );
        glib::Propagation::Stop
    });

    let win_create = window.clone();
    let nav = nav_callback.clone();
    let current_p = current_path.clone();
    let entry_c = entry.clone();
    btn_create.connect_clicked(move |_| {
        let name = entry_c.text().to_string();
        if !name.is_empty() {
            let file_path = current_p.join(name);
            let nav_c = nav.clone();
            let cp_c = current_p.clone();
            glib::spawn_future_local(async move {
                let _ = tokio::fs::write(&file_path, "").await;
                nav_c(cp_c);
            });
        }
        win_create.close();
    });

    let entry_trigger = entry.clone();
    entry.connect_activate(move |_| {
        btn_create.emit_clicked();
    });

    window.present();
    crate::ui::animation::genie_in(vbox.upcast_ref(), 320, 140, 300);
    entry_trigger.grab_focus();
}
