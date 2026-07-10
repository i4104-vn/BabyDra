use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Align, Image, ScrolledWindow, Frame};
use babydra_common::FileEntry;
use baby_utils::explore_helpers;

pub struct InfoPanel {
    container: ScrolledWindow,
    img_preview: Image,
    lbl_name: Label,
    lbl_type: Label,
    lbl_size: Label,
    lbl_modified: Label,
    lbl_owner: Label,
    lbl_permissions: Label,
}

impl InfoPanel {
    pub fn new() -> Self {
        let container = ScrolledWindow::new();
        container.set_hscrollbar_policy(gtk4::PolicyType::Never);
        container.set_css_classes(&["info-panel"]);
        container.set_size_request(250, -1);

        let vbox = Box::new(Orientation::Vertical, 12);
        vbox.set_margin_top(12);
        vbox.set_margin_bottom(12);
        vbox.set_margin_start(12);
        vbox.set_margin_end(12);
        container.set_child(Some(&vbox));

        // Preview Section
        let preview_frame = Frame::new(Some("Preview"));
        let preview_box = Box::new(Orientation::Vertical, 6);
        preview_box.set_margin_top(6);
        preview_box.set_margin_bottom(6);
        preview_box.set_margin_start(6);
        preview_box.set_margin_end(6);
        preview_frame.set_child(Some(&preview_box));

        let img_preview = Image::from_icon_name("text-x-generic");
        img_preview.set_pixel_size(96);
        img_preview.set_halign(Align::Center);
        preview_box.append(&img_preview);

        vbox.append(&preview_frame);

        // Details Section
        let details_frame = Frame::new(Some("Details"));
        let details_box = Box::new(Orientation::Vertical, 8);
        details_box.set_margin_top(6);
        details_box.set_margin_bottom(6);
        details_box.set_margin_start(6);
        details_box.set_margin_end(6);
        details_frame.set_child(Some(&details_box));

        let lbl_name = Self::create_detail_row(&details_box, "Name:");
        let lbl_type = Self::create_detail_row(&details_box, "Type:");
        let lbl_size = Self::create_detail_row(&details_box, "Size:");
        let lbl_modified = Self::create_detail_row(&details_box, "Modified:");
        let lbl_owner = Self::create_detail_row(&details_box, "Owner:");
        let lbl_permissions = Self::create_detail_row(&details_box, "Permissions:");

        vbox.append(&details_frame);

        Self {
            container,
            img_preview,
            lbl_name,
            lbl_type,
            lbl_size,
            lbl_modified,
            lbl_owner,
            lbl_permissions,
        }
    }

    pub fn widget(&self) -> &ScrolledWindow {
        &self.container
    }

    fn create_detail_row(container: &Box, label: &str) -> Label {
        let hbox = Box::new(Orientation::Horizontal, 6);
        
        let lbl_title = Label::builder()
            .label(label)
            .halign(Align::Start)
            .build();
        lbl_title.set_css_classes(&["dim-label"]);
        
        let lbl_val = Label::builder()
            .label("--")
            .halign(Align::End)
            .hexpand(true)
            .ellipsize(gtk4::pango::EllipsizeMode::End)
            .build();

        hbox.append(&lbl_title);
        hbox.append(&lbl_val);
        container.append(&hbox);

        lbl_val
    }

    pub fn update(&self, selection: &[FileEntry]) {
        if selection.is_empty() {
            self.clear();
            return;
        }

        if selection.len() > 1 {
            self.img_preview.set_icon_name(Some("dialog-information"));
            self.lbl_name.set_text(&format!("{} items selected", selection.len()));
            let total_size: u64 = selection.iter().map(|e| e.size).sum();
            self.lbl_size.set_text(&explore_helpers::format_size(total_size));
            self.lbl_type.set_text("Multiple types");
            self.lbl_modified.set_text("--");
            self.lbl_owner.set_text("--");
            self.lbl_permissions.set_text("--");
            return;
        }

        let entry = &selection[0];
        
        // Update thumbnail or icon preview
        if entry.mime_type.starts_with("image/") {
            self.img_preview.set_from_file(Some(&entry.path));
        } else {
            self.img_preview.set_icon_name(Some(&entry.icon_name));
        }

        self.lbl_name.set_text(&entry.display_name);
        self.lbl_type.set_text(&entry.mime_type);
        
        if matches!(entry.file_type, babydra_common::FileType::Directory) {
            self.lbl_size.set_text("Folder");
        } else {
            self.lbl_size.set_text(&explore_helpers::format_size(entry.size));
        }

        if let Some(mtime) = entry.modified {
            self.lbl_modified.set_text(&explore_helpers::format_date(mtime));
        } else {
            self.lbl_modified.set_text("--");
        }

        self.lbl_owner.set_text(&entry.owner);
        self.lbl_permissions.set_text(&format!("{:o}", entry.permissions & 0o777));
    }

    fn clear(&self) {
        self.img_preview.set_icon_name(Some("text-x-generic"));
        self.lbl_name.set_text("--");
        self.lbl_type.set_text("--");
        self.lbl_size.set_text("--");
        self.lbl_modified.set_text("--");
        self.lbl_owner.set_text("--");
        self.lbl_permissions.set_text("--");
    }
}
