use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Entry, Button, Align, Window};
use std::path::PathBuf;
use std::rc::Rc;

pub fn show_rename_dialog(path: &PathBuf, current_path: PathBuf, nav_callback: Rc<dyn Fn(PathBuf)>) {
    let window = Window::builder()
        .title("Rename File")
        .modal(true)
        .resizable(false)
        .default_width(320)
        .default_height(140)
        .css_classes(vec!["explore-dialog".to_string()])
        .build();

    let vbox = Box::new(Orientation::Vertical, 12);
    vbox.set_margin_top(16);
    vbox.set_margin_bottom(16);
    vbox.set_margin_start(16);
    vbox.set_margin_end(16);
    window.set_child(Some(&vbox));

    let lbl = Label::builder()
        .label("Enter new name:")
        .halign(Align::Start)
        .build();
    vbox.append(&lbl);

    let entry = Entry::new();
    entry.set_text(&path.file_name().unwrap().to_string_lossy());
    entry.set_hexpand(true);
    vbox.append(&entry);

    let bbox = Box::new(Orientation::Horizontal, 8);
    bbox.set_halign(Align::End);
    vbox.append(&bbox);

    let btn_cancel = Button::with_label("Cancel");
    let btn_rename = Button::builder()
        .label("Rename")
        .css_classes(vec!["suggested-action".to_string()])
        .build();

    bbox.append(&btn_cancel);
    bbox.append(&btn_rename);

    let win_cancel = window.clone();
    btn_cancel.connect_clicked(move |_| {
        win_cancel.close();
    });

    let win_rename = window.clone();
    let path = path.clone();
    let nav = nav_callback.clone();
    let current_p = current_path.clone();
    let entry_c = entry.clone();
    btn_rename.connect_clicked(move |_| {
        let new_name = entry_c.text().to_string();
        if !new_name.is_empty() {
            let path_c = path.clone();
            let nav_c = nav.clone();
            let cp_c = current_p.clone();
            glib::spawn_future_local(async move {
                if let Err(e) = babydra_common::rename_path(path_c, new_name).await {
                    eprintln!("Rename failed: {}", e);
                }
                nav_c(cp_c);
            });
        }
        win_rename.close();
    });

    // Make entry trigger rename on press enter
    let entry_trigger = entry.clone();
    entry.connect_activate(move |_| {
        btn_rename.emit_clicked();
    });

    window.present();
    entry_trigger.grab_focus();
}

pub fn show_new_folder_dialog(current_path: PathBuf, nav_callback: Rc<dyn Fn(PathBuf)>) {
    let window = Window::builder()
        .title("Create New Folder")
        .modal(true)
        .resizable(false)
        .default_width(320)
        .default_height(140)
        .css_classes(vec!["explore-dialog".to_string()])
        .build();

    let vbox = Box::new(Orientation::Vertical, 12);
    vbox.set_margin_top(16);
    vbox.set_margin_bottom(16);
    vbox.set_margin_start(16);
    vbox.set_margin_end(16);
    window.set_child(Some(&vbox));

    let lbl = Label::builder()
        .label("Folder name:")
        .halign(Align::Start)
        .build();
    vbox.append(&lbl);

    let entry = Entry::new();
    entry.set_text("New Folder");
    entry.set_hexpand(true);
    vbox.append(&entry);

    let bbox = Box::new(Orientation::Horizontal, 8);
    bbox.set_halign(Align::End);
    vbox.append(&bbox);

    let btn_cancel = Button::with_label("Cancel");
    let btn_create = Button::builder()
        .label("Create")
        .css_classes(vec!["suggested-action".to_string()])
        .build();

    bbox.append(&btn_cancel);
    bbox.append(&btn_create);

    let win_cancel = window.clone();
    btn_cancel.connect_clicked(move |_| {
        win_cancel.close();
    });

    let win_create = window.clone();
    let nav = nav_callback.clone();
    let current_p = current_path.clone();
    let entry_c = entry.clone();
    btn_create.connect_clicked(move |_| {
        let name = entry_c.text().to_string();
        if !name.is_empty() {
            let folder_path = current_p.join(name);
            let nav_c = nav.clone();
            let cp_c = current_p.clone();
            glib::spawn_future_local(async move {
                let _ = tokio::fs::create_dir_all(folder_path).await;
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
    entry_trigger.grab_focus();
}
