use gtk4::prelude::*;
use gtk4::{Box, Orientation, ScrolledWindow, FlowBox, Align, Label, Button, Image};
use std::path::PathBuf;
use babydra_common::FileEntry;

pub struct ContentView {
    container: ScrolledWindow,
    flowbox: FlowBox,
    nav_callback: std::boxed::Box<dyn Fn(PathBuf)>,
}

impl ContentView {
    pub fn new(nav_callback: impl Fn(PathBuf) + 'static) -> Self {
        let container = ScrolledWindow::new();
        container.set_hexpand(true);
        container.set_vexpand(true);
        container.set_css_classes(&["content-view"]);

        let flowbox = FlowBox::new();
        flowbox.set_valign(Align::Start);
        flowbox.set_max_children_per_line(15);
        flowbox.set_min_children_per_line(3);
        flowbox.set_selection_mode(gtk4::SelectionMode::Multiple);
        flowbox.set_margin_top(12);
        flowbox.set_margin_bottom(12);
        flowbox.set_margin_start(12);
        flowbox.set_margin_end(12);
        flowbox.set_column_spacing(10);
        flowbox.set_row_spacing(10);

        container.set_child(Some(&flowbox));

        Self {
            container,
            flowbox,
            nav_callback: std::boxed::Box::new(nav_callback),
        }
    }

    pub fn widget(&self) -> &ScrolledWindow {
        &self.container
    }

    pub fn update(&self, entries: &[FileEntry]) {
        // Clear flowbox
        while let Some(child) = self.flowbox.first_child() {
            self.flowbox.remove(&child);
        }

        for entry in entries {
            let item_box = Box::new(Orientation::Vertical, 6);
            item_box.set_size_request(100, 100);
            item_box.set_css_classes(&["file-item"]);

            let img = Image::from_icon_name(&entry.icon_name);
            img.set_pixel_size(48);

            let lbl = Label::builder()
                .label(&entry.display_name)
                .max_width_chars(12)
                .ellipsize(gtk4::pango::EllipsizeMode::End)
                .halign(Align::Center)
                .build();

            item_box.append(&img);
            item_box.append(&lbl);

            let btn = Button::builder()
                .child(&item_box)
                .css_classes(vec!["flat".to_string()])
                .build();

            let target_path = entry.path.clone();
            let is_dir = matches!(entry.file_type, babydra_common::FileType::Directory);
            let nav_cb = self.nav_callback.as_ref() as *const dyn Fn(PathBuf);
            let nav_cb = unsafe { &*nav_cb };

            btn.connect_clicked(move |_| {
                if is_dir {
                    nav_cb(target_path.clone());
                } else {
                    // Try to launch file
                    let uri = format!("file://{}", target_path.to_string_lossy());
                    let _ = gio::AppInfo::launch_default_for_uri(&uri, gio::AppLaunchContext::NONE);
                }
            });

            self.flowbox.append(&btn);
        }
    }
}
