use crate::widgets::preview_panel::create_preview_panel;
use crate::widgets::state::InfoPanelWidgets;
use gtk4::prelude::*;
use gtk4::{Align, Box, Frame, Image, Label, Orientation, ScrolledWindow, Stack};

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

    let img_preview_icon = Image::from_icon_name("text-x-generic");
    img_preview_icon.set_pixel_size(96);
    img_preview_icon.set_halign(Align::Center);
    img_preview_icon.set_valign(Align::Center);
    stack.add_named(&img_preview_icon, Some("image_icon"));

    let img_preview_picture = gtk4::Picture::new();
    img_preview_picture.set_can_shrink(true);
    img_preview_picture.set_content_fit(gtk4::ContentFit::Contain);
    img_preview_picture.set_halign(Align::Fill);
    img_preview_picture.set_valign(Align::Fill);
    stack.add_named(&img_preview_picture, Some("image_picture"));

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

    // Image Details Section
    let image_details_frame = Frame::new(Some(&babydra_core::i18n::trans("explore.prop_image_info")));
    let img_box = Box::new(Orientation::Vertical, 8);
    img_box.set_margin_top(6);
    img_box.set_margin_bottom(6);
    img_box.set_margin_start(6);
    img_box.set_margin_end(6);
    image_details_frame.set_child(Some(&img_box));

    let (_, lbl_img_dimensions) =
        create_detail_row_with_box(&img_box, &babydra_core::i18n::trans("explore.prop_dimensions"));
    let (_, lbl_img_pixels) =
        create_detail_row_with_box(&img_box, &babydra_core::i18n::trans("explore.prop_pixels"));
    let (_, lbl_img_dpi) =
        create_detail_row_with_box(&img_box, &babydra_core::i18n::trans("explore.prop_dpi"));
    let (row_img_color, lbl_img_color) =
        create_detail_row_with_box(&img_box, &babydra_core::i18n::trans("explore.prop_color_depth"));
    let (row_img_camera, lbl_img_camera) =
        create_detail_row_with_box(&img_box, &babydra_core::i18n::trans("explore.prop_camera"));
    let (row_img_exposure, lbl_img_exposure) =
        create_detail_row_with_box(&img_box, &babydra_core::i18n::trans("explore.prop_exposure"));

    image_details_frame.set_visible(false);
    vbox.append(&image_details_frame);

    InfoPanelWidgets {
        container,
        img_preview_icon,
        img_preview_picture,
        preview_widgets,
        stack,
        size_calc_generation: std::rc::Rc::new(std::cell::Cell::new(0)),
        lbl_name,
        lbl_type,
        lbl_size,
        lbl_modified,
        lbl_owner,
        lbl_permissions,
        image_details_frame,
        lbl_img_dimensions,
        lbl_img_pixels,
        lbl_img_dpi,
        row_img_color,
        lbl_img_color,
        row_img_camera,
        lbl_img_camera,
        row_img_exposure,
        lbl_img_exposure,
    }
}

/// Creates a new `detail row`.
fn create_detail_row(container: &Box, label: &str) -> Label {
    let (_, lbl) = create_detail_row_with_box(container, label);
    lbl
}

/// Creates a new `detail row` returning both the row container Box and value Label.
fn create_detail_row_with_box(container: &Box, label: &str) -> (Box, Label) {
    let hbox = Box::new(Orientation::Horizontal, 6);

    let lbl_title = Label::builder().label(label).halign(Align::Start).build();
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

    (hbox, lbl_val)
}
