use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Align, Image, ScrolledWindow, Frame, Stack};
use crate::widgets::preview_panel::create_preview_panel;
use babydra_common::InfoPanelWidgets;

/// Builds the InfoPanel UI hierarchy and returns a handle containing all components.
pub fn build_info_panel_ui() -> InfoPanelWidgets {
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
    let stack = Stack::new();
    stack.set_size_request(-1, 240);
    stack.set_transition_type(gtk4::StackTransitionType::Crossfade);

    let img_preview = Image::from_icon_name("text-x-generic");
    img_preview.set_pixel_size(96);
    img_preview.set_halign(Align::Center);
    img_preview.set_valign(Align::Center);
    stack.add_named(&img_preview, Some("image"));

    let (preview_container, preview_widgets) = create_preview_panel();
    stack.add_named(&preview_container, Some("text"));

    vbox.append(&stack);

    // Details Section
    let details_frame = Frame::new(Some("Details"));
    let details_box = Box::new(Orientation::Vertical, 8);
    details_box.set_margin_top(6);
    details_box.set_margin_bottom(6);
    details_box.set_margin_start(6);
    details_box.set_margin_end(6);
    details_frame.set_child(Some(&details_box));

    let lbl_name = create_detail_row(&details_box, "Name:");
    let lbl_type = create_detail_row(&details_box, "Type:");
    let lbl_size = create_detail_row(&details_box, "Size:");
    let lbl_modified = create_detail_row(&details_box, "Modified:");
    let lbl_owner = create_detail_row(&details_box, "Owner:");
    let lbl_permissions = create_detail_row(&details_box, "Permissions:");

    vbox.append(&details_frame);

    InfoPanelWidgets {
        container,
        img_preview,
        preview_widgets,
        stack,
        lbl_name,
        lbl_type,
        lbl_size,
        lbl_modified,
        lbl_owner,
        lbl_permissions,
    }
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
