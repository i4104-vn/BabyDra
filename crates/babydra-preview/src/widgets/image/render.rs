//! Image viewer UI rendering and layout assembly.

use crate::widgets::window::{create_viewer_window, format_aspect_ratio};
use babydra_core::i18n::trans;
use gtk4::prelude::*;
use gtk4::{Align, Application, ApplicationWindow, Box, Button, DrawingArea, Grid, Label, Orientation, Overlay};
use std::path::PathBuf;

/// The full set of widgets built for the image viewer window.
pub struct ImageViewerUi {
    pub window: ApplicationWindow,
    pub drawing_area: DrawingArea,
    pub scale_lbl: Label,
    pub info_box: Box,
    pub controls_box: Box,
    pub exif_box: Box,
    pub zoom_out_btn: Button,
    pub reset_btn: Button,
    pub zoom_in_btn: Button,
}

/// Builds the full viewer window UI for static images.
pub fn build_image_ui(
    app: &Application,
    path: &PathBuf,
    img_w: u32,
    img_h: u32,
) -> ImageViewerUi {
    let title = trans("preview.title").replace(
        "{}",
        &path.file_name().unwrap_or_default().to_string_lossy(),
    );
    let (window, _) = create_viewer_window(app, &title, img_w, img_h);

    let overlay = Overlay::new();

    let drawing_area = DrawingArea::new();
    drawing_area.set_hexpand(true);
    drawing_area.set_vexpand(true);
    drawing_area.add_css_class("viewer-drawing-area");
    overlay.set_child(Some(&drawing_area));

    // --- Top-Left Info Box Overlay ---
    let info_box =
        babydra_ui_kit::components::create_css_card(Orientation::Vertical, 2, "info-card");
    info_box.set_halign(Align::Start);
    info_box.set_valign(Align::Start);
    info_box.set_margin_start(16);
    info_box.set_margin_top(16);

    let name_lbl = Label::new(Some(
        &path.file_name().unwrap_or_default().to_string_lossy(),
    ));
    name_lbl.add_css_class("info-item");
    name_lbl.set_halign(Align::Start);
    name_lbl.set_ellipsize(gtk4::pango::EllipsizeMode::Middle);
    name_lbl.set_max_width_chars(32);
    info_box.append(&name_lbl);

    let res_aspect = format_aspect_ratio(img_w, img_h);
    let res_text = if !res_aspect.is_empty() {
        format!("{}x{} ({})", img_w, img_h, res_aspect)
    } else {
        format!("{}x{}", img_w, img_h)
    };
    let size_bytes = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);
    let meta_text = format!(
        "{} • {}",
        res_text,
        babydra_ui_kit::components::explore::format_size(size_bytes)
    );
    let meta_lbl = Label::new(Some(&meta_text));
    meta_lbl.add_css_class("info-item");
    meta_lbl.set_halign(Align::Start);
    info_box.append(&meta_lbl);

    overlay.add_overlay(&info_box);

    // --- Bottom-Center Zoom Controls Pill ---
    let controls_box = Box::new(Orientation::Horizontal, 6);
    controls_box.add_css_class("controls-bar");
    controls_box.set_halign(Align::Center);
    controls_box.set_valign(Align::End);
    controls_box.set_margin_bottom(16);

    let zoom_out_btn = babydra_ui_kit::components::create_icon_button(
        "zoom-out",
        16,
        &["control-btn"],
        None,
        || {},
    );
    zoom_out_btn.set_cursor_from_name(Some("pointer"));
    controls_box.append(&zoom_out_btn);

    let scale_lbl = Label::new(Some("100%"));
    scale_lbl.add_css_class("info-item");
    scale_lbl.set_valign(Align::Center);
    scale_lbl.set_margin_start(4);
    scale_lbl.set_margin_end(4);
    controls_box.append(&scale_lbl);

    let reset_btn = babydra_ui_kit::components::create_icon_button(
        "zoom-fit",
        16,
        &["control-btn"],
        None,
        || {},
    );
    reset_btn.set_cursor_from_name(Some("pointer"));
    controls_box.append(&reset_btn);

    let zoom_in_btn = babydra_ui_kit::components::create_icon_button(
        "zoom-in",
        16,
        &["control-btn"],
        None,
        || {},
    );
    zoom_in_btn.set_cursor_from_name(Some("pointer"));
    controls_box.append(&zoom_in_btn);

    overlay.add_overlay(&controls_box);

    // --- Centered EXIF Metadata Dialog ---
    let exif_box = Box::new(Orientation::Vertical, 12);
    exif_box.add_css_class("exif-dialog");
    exif_box.set_halign(Align::Center);
    exif_box.set_valign(Align::Center);
    exif_box.set_visible(false);

    let exif_title = Label::new(Some(&trans("preview.camera_info")));
    exif_title.add_css_class("exif-title");
    exif_box.append(&exif_title);

    let grid = Grid::new();
    grid.set_column_spacing(24);
    grid.set_row_spacing(8);

    let mut row_idx = 0;
    let mut add_exif_row = |label: &str, value: &str| {
        let lbl = Label::new(Some(label));
        lbl.add_css_class("exif-label");
        lbl.set_halign(Align::Start);
        grid.attach(&lbl, 0, row_idx, 1, 1);

        let val = Label::new(Some(value));
        val.add_css_class("exif-value");
        val.set_halign(Align::End);
        grid.attach(&val, 1, row_idx, 1, 1);

        row_idx += 1;
    };

    if let Some(ref data) = babydra_core::read_exif(path) {
        if let (Some(make), Some(model)) = (&data.make, &data.model) {
            add_exif_row("Device", &format!("{} {}", make.trim(), model.trim()));
        }
        if let Some(ref val) = data.aperture {
            add_exif_row("Aperture", val);
        }
        if let Some(ref val) = data.exposure_time {
            add_exif_row("Shutter Speed", val);
        }
        if let Some(ref val) = data.iso {
            add_exif_row("ISO Speed", val);
        }
        if let Some(ref val) = data.focal_length {
            add_exif_row("Focal Length", val);
        }
        if let Some(ref val) = data.lens_model {
            add_exif_row("Lens Model", val);
        }
        if let Some(ref val) = data.date_time {
            add_exif_row("Date Original", val);
        }
    } else {
        let no_exif_lbl = Label::new(Some(&trans("preview.no_exif")));
        no_exif_lbl.add_css_class("exif-value");
        grid.attach(&no_exif_lbl, 0, 0, 2, 1);
    }

    exif_box.append(&grid);
    overlay.add_overlay(&exif_box);

    window.set_child(Some(&overlay));

    ImageViewerUi {
        window,
        drawing_area,
        scale_lbl,
        info_box,
        controls_box,
        exif_box,
        zoom_out_btn,
        reset_btn,
        zoom_in_btn,
    }
}
