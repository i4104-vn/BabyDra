//! Image viewer UI builders (window, info card, zoom controls, EXIF box).

use babydra_core::i18n::trans;
use gtk4::prelude::*;
use std::path::PathBuf;

/// The full set of widgets built for the image viewer window.
pub struct ViewerUi {
    pub window: gtk4::ApplicationWindow,
    pub drawing_area: gtk4::DrawingArea,
    pub scale_lbl: gtk4::Label,
    pub info_box: gtk4::Box,
    pub controls_box: gtk4::Box,
    pub exif_box: gtk4::Box,
    pub zoom_out_btn: gtk4::Button,
    pub reset_btn: gtk4::Button,
    pub zoom_in_btn: gtk4::Button,
}

/// Calculates the greatest common divisor.
fn calculate_gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        calculate_gcd(b, a % b)
    }
}

/// Formats an aspect ratio as `W:H`.
fn format_aspect_ratio(w: u32, h: u32) -> String {
    let divisor = calculate_gcd(w, h);
    if divisor > 0 {
        format!("{}:{}", w / divisor, h / divisor)
    } else {
        String::new()
    }
}

/// Calculates the initial window size matching the image aspect ratio and screen size.
fn calculate_initial_window_size(img_w: u32, img_h: u32) -> (i32, i32) {
    if img_w == 0 || img_h == 0 {
        return (800, 600);
    }

    let (screen_w, screen_h) = if let Some(display) = gtk4::gdk::Display::default() {
        let monitors = display.monitors();
        if let Some(monitor) = monitors
            .item(0)
            .and_then(|obj| obj.downcast::<gtk4::gdk::Monitor>().ok())
        {
            let geom = monitor.geometry();
            (geom.width() as f64, geom.height() as f64)
        } else {
            (1920.0, 1080.0)
        }
    } else {
        (1920.0, 1080.0)
    };

    let max_w = (screen_w * 0.85).max(600.0);
    let max_h = (screen_h * 0.85).max(400.0);
    let min_w = 480.0;
    let min_h = 360.0;

    let img_w = img_w as f64;
    let img_h = img_h as f64;
    let aspect = img_w / img_h;

    let mut w = img_w;
    let mut h = img_h;

    if w > max_w {
        w = max_w;
        h = w / aspect;
    }
    if h > max_h {
        h = max_h;
        w = h * aspect;
    }

    if w < min_w {
        w = min_w;
        h = w / aspect;
    }
    if h < min_h {
        h = min_h;
        w = h * aspect;
    }

    if w > max_w {
        w = max_w;
        h = w / aspect;
    }
    if h > max_h {
        h = max_h;
        w = h * aspect;
    }

    (w.round().max(300.0) as i32, h.round().max(200.0) as i32)
}

/// Builds the full viewer window UI (image canvas, info card, zoom bar, EXIF box).
pub fn build_viewer_ui(
    app: &gtk4::Application,
    path: &PathBuf,
    img_w: u32,
    img_h: u32,
) -> ViewerUi {
    let window = gtk4::ApplicationWindow::new(app);
    window.set_title(Some(&trans("preview.title").replace(
        "{}",
        &path.file_name().unwrap_or_default().to_string_lossy(),
    )));
    window.set_icon_name(Some("babydra-preview"));

    let (win_w, win_h) = calculate_initial_window_size(img_w, img_h);
    window.set_default_size(win_w, win_h);
    window.add_css_class("viewer-window");

    let overlay = gtk4::Overlay::new();

    let drawing_area = gtk4::DrawingArea::new();
    drawing_area.set_hexpand(true);
    drawing_area.set_vexpand(true);
    overlay.set_child(Some(&drawing_area));

    // --- Top-Left Info Box Overlay ---
    let info_box =
        babydra_ui_kit::components::create_css_card(gtk4::Orientation::Vertical, 2, "info-card");
    info_box.set_halign(gtk4::Align::Start);
    info_box.set_valign(gtk4::Align::Start);
    info_box.set_margin_start(16);
    info_box.set_margin_top(16);

    let name_lbl = gtk4::Label::new(Some(
        &path.file_name().unwrap_or_default().to_string_lossy(),
    ));
    name_lbl.add_css_class("info-item");
    name_lbl.set_halign(gtk4::Align::Start);
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
    let meta_lbl = gtk4::Label::new(Some(&meta_text));
    meta_lbl.add_css_class("info-item");
    meta_lbl.set_halign(gtk4::Align::Start);
    info_box.append(&meta_lbl);

    overlay.add_overlay(&info_box);

    // --- Bottom-Center Zoom Controls Pill ---
    let controls_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    controls_box.add_css_class("controls-bar");
    controls_box.set_halign(gtk4::Align::Center);
    controls_box.set_valign(gtk4::Align::End);
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

    let scale_lbl = gtk4::Label::new(Some("100%"));
    scale_lbl.add_css_class("info-item");
    scale_lbl.set_valign(gtk4::Align::Center);
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
    let exif_box = gtk4::Box::new(gtk4::Orientation::Vertical, 12);
    exif_box.add_css_class("exif-dialog");
    exif_box.set_halign(gtk4::Align::Center);
    exif_box.set_valign(gtk4::Align::Center);
    exif_box.set_visible(false);

    let exif_title = gtk4::Label::new(Some(&trans("preview.camera_info")));
    exif_title.add_css_class("exif-title");
    exif_box.append(&exif_title);

    let grid = gtk4::Grid::new();
    grid.set_column_spacing(24);
    grid.set_row_spacing(8);

    let mut row_idx = 0;
    let mut add_exif_row = |label: &str, value: &str| {
        let lbl = gtk4::Label::new(Some(label));
        lbl.add_css_class("exif-label");
        lbl.set_halign(gtk4::Align::Start);
        grid.attach(&lbl, 0, row_idx, 1, 1);

        let val = gtk4::Label::new(Some(value));
        val.add_css_class("exif-value");
        val.set_halign(gtk4::Align::End);
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
        let no_exif_lbl = gtk4::Label::new(Some(&trans("preview.no_exif")));
        no_exif_lbl.add_css_class("exif-value");
        grid.attach(&no_exif_lbl, 0, 0, 2, 1);
    }

    exif_box.append(&grid);
    overlay.add_overlay(&exif_box);

    window.set_child(Some(&overlay));

    ViewerUi {
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
