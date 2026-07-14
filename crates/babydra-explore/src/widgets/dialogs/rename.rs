use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label};
use std::path::PathBuf;
use std::rc::Rc;

pub fn show_rename_dialog(path: &PathBuf, current_path: PathBuf, nav_callback: Rc<dyn Fn(PathBuf)>) {
    let dialog = gtk4::Dialog::builder()
        .title("Rename File")
        .use_header_bar(1)
        .build();
    
    let content_area = dialog.content_area();
    let vbox = Box::new(Orientation::Vertical, 8);
    vbox.set_margin_top(12);
    vbox.set_margin_bottom(12);
    vbox.set_margin_start(12);
    vbox.set_margin_end(12);
    content_area.append(&vbox);

    let lbl = Label::new(Some("Enter new name:"));
    vbox.append(&lbl);

    let entry = gtk4::Entry::new();
    entry.set_text(&path.file_name().unwrap().to_string_lossy());
    vbox.append(&entry);

    dialog.add_button("Cancel", gtk4::ResponseType::Cancel);
    dialog.add_button("Rename", gtk4::ResponseType::Ok);

    let path = path.clone();
    dialog.connect_response(move |dialog, response| {
        if response == gtk4::ResponseType::Ok {
            let new_name = entry.text().to_string();
            if !new_name.is_empty() {
                let path = path.clone();
                let nav = nav_callback.clone();
                let current_p = current_path.clone();
                glib::spawn_future_local(async move {
                    if let Err(e) = babydra_common::rename_path(path, new_name).await {
                        eprintln!("Rename failed: {}", e);
                    }
                    nav(current_p);
                });
            }
        }
        dialog.destroy();
    });

    dialog.show();
}
