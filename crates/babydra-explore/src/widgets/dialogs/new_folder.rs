use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label};
use std::path::PathBuf;
use std::rc::Rc;

pub fn show_new_folder_dialog(current_path: PathBuf, nav_callback: Rc<dyn Fn(PathBuf)>) {
    let dialog = gtk4::Dialog::builder()
        .title("Create New Folder")
        .use_header_bar(1)
        .build();
    
    let content_area = dialog.content_area();
    let vbox = Box::new(Orientation::Vertical, 8);
    vbox.set_margin_top(12);
    vbox.set_margin_bottom(12);
    vbox.set_margin_start(12);
    vbox.set_margin_end(12);
    content_area.append(&vbox);

    let lbl = Label::new(Some("Folder name:"));
    vbox.append(&lbl);

    let entry = gtk4::Entry::new();
    entry.set_text("New Folder");
    vbox.append(&entry);

    dialog.add_button("Cancel", gtk4::ResponseType::Cancel);
    dialog.add_button("Create", gtk4::ResponseType::Ok);

    let current_p_clone = current_path.clone();
    dialog.connect_response(move |dialog, response| {
        if response == gtk4::ResponseType::Ok {
            let name = entry.text().to_string();
            if !name.is_empty() {
                let folder_path = current_p_clone.join(name);
                let nav = nav_callback.clone();
                let current_p = current_p_clone.clone();
                glib::spawn_future_local(async move {
                    let _ = tokio::fs::create_dir_all(folder_path).await;
                    nav(current_p);
                });
            }
        }
        dialog.destroy();
    });

    dialog.show();
}
