use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Align};
use babydra_common::FileEntry;

/// Builds the visual GTK widgets layout for a file list row.
pub fn build_list_row_ui(entry: &FileEntry) -> Box {
    let item_box = Box::new(Orientation::Horizontal, 12);
    item_box.set_css_classes(&["list-row"]);
    item_box.set_margin_top(2);
    item_box.set_margin_bottom(2);
    item_box.set_margin_start(6);
    item_box.set_margin_end(6);

    let img = crate::ui::icon::get_system_or_file_icon(&entry.icon_name, "text-x-generic");
    img.set_pixel_size(24);
    item_box.append(&img);

    let lbl_name = Label::builder()
        .label(&entry.display_name)
        .halign(Align::Start)
        .hexpand(true)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .build();
    item_box.append(&lbl_name);

    // File size info
    let size_str = if matches!(entry.file_type, babydra_common::FileType::Directory) {
        "--".to_string()
    } else {
        crate::explore::format_size(entry.size)
    };
    let lbl_size = Label::new(Some(&size_str));
    lbl_size.set_css_classes(&["list-col-meta"]);
    lbl_size.set_size_request(80, -1);
    lbl_size.set_halign(Align::End);
    lbl_size.set_tooltip_text(Some("Size"));
    item_box.append(&lbl_size);

    // Permissions info
    let perm_str = format!("{:o}", entry.permissions & 0o777);
    let lbl_perm = Label::new(Some(&perm_str));
    lbl_perm.set_css_classes(&["list-col-meta"]);
    lbl_perm.set_size_request(80, -1);
    lbl_perm.set_halign(Align::End);
    lbl_perm.set_tooltip_text(Some("Permissions"));
    item_box.append(&lbl_perm);

    // Modified info
    let mod_str = if let Some(mtime) = entry.modified {
        crate::explore::format_date(mtime)
    } else {
        "--".to_string()
    };
    let lbl_date = Label::new(Some(&mod_str));
    lbl_date.set_css_classes(&["list-col-meta"]);
    lbl_date.set_size_request(140, -1);
    lbl_date.set_halign(Align::End);
    lbl_date.set_tooltip_text(Some("Modified Date"));
    item_box.append(&lbl_date);

    item_box
}
