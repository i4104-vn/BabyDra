//! Dynamic desktop image preview application.
//! Base Rust + GTK4 image viewer with zooming, panning, and EXIF key overlays.

use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::path::PathBuf;

struct ImageState {
    pixbuf: gdk_pixbuf::Pixbuf,
    scale: f64,
    offset_x: f64,
    offset_y: f64,
    min_scale: f64,
    img_w: f64,
    img_h: f64,
    // Drag gestures tracking
    drag_start_x: f64,
    drag_start_y: f64,
}

fn calculate_gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        calculate_gcd(b, a % b)
    }
}

fn format_aspect_ratio(w: u32, h: u32) -> String {
    let divisor = calculate_gcd(w, h);
    if divisor > 0 {
        format!("{}:{}", w / divisor, h / divisor)
    } else {
        String::new()
    }
}

fn format_file_size(bytes: u64) -> String {
    if bytes >= 1024 * 1024 {
        format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    }
}

fn clamp_position(st: &mut ImageState, area_w: f64, area_h: f64) {
    let scaled_w = st.img_w * st.scale;
    let scaled_h = st.img_h * st.scale;

    let limit_x = ((scaled_w - area_w) / 2.0).max(0.0);
    let limit_y = ((scaled_h - area_h) / 2.0).max(0.0);

    st.offset_x = st.offset_x.clamp(-limit_x, limit_x);
    st.offset_y = st.offset_y.clamp(-limit_y, limit_y);
}

fn update_zoom_display(st: &ImageState, lbl: &gtk4::Label) {
    lbl.set_text(&format!("{:.0}%", st.scale * 100.0));
}

fn fit_to_screen(st: &mut ImageState, area_w: f64, area_h: f64) {
    let scale_x = area_w / st.img_w;
    let scale_y = area_h / st.img_h;
    st.min_scale = scale_x.min(scale_y).min(1.0).max(0.05);
    st.scale = st.min_scale;
    st.offset_x = 0.0;
    st.offset_y = 0.0;
}

fn do_zoom(state: &Rc<RefCell<ImageState>>, area: &gtk4::DrawingArea, lbl: &gtk4::Label, delta: f64) {
    let mut st = state.borrow_mut();
    let area_w = area.width() as f64;
    let area_h = area.height() as f64;

    let next_scale = (st.scale + delta).clamp(st.min_scale, 5.0);
    st.scale = next_scale;

    if st.scale <= st.min_scale + 0.001 {
        st.offset_x = 0.0;
        st.offset_y = 0.0;
    } else {
        clamp_position(&mut *st, area_w, area_h);
    }

    update_zoom_display(&st, lbl);
    area.queue_draw();
}

fn main() {
    let app = gtk4::Application::new(
        Some("com.babydra.image-preview"),
        Default::default(),
    );

    app.connect_activate(|app| {
        // Load custom styling CSS rules from babydra-common
        babydra_common::init_theme();

        let arg_path = std::env::args().nth(1);
        if let Some(p) = arg_path {
            let path = PathBuf::from(p);
            if path.exists() {
                build_ui(app, path);
                return;
            }
        }

        // Fallback file selector if no path is given or if the path is invalid
        let fallback_window = gtk4::ApplicationWindow::new(app);
        fallback_window.set_title(Some("BabyDra Image Preview"));
        fallback_window.set_default_size(400, 200);

        let file_dialog = gtk4::FileDialog::new();
        file_dialog.set_title("Open Image File");
        
        let filter = gtk4::FileFilter::new();
        filter.set_name(Some("Images"));
        filter.add_mime_type("image/png");
        filter.add_mime_type("image/jpeg");
        filter.add_mime_type("image/webp");
        file_dialog.set_default_filter(Some(&filter));

        let app_clone = app.clone();
        let fallback_win_clone = fallback_window.clone();
        file_dialog.open(Some(&fallback_window), None::<&gio::Cancellable>, move |res| {
            if let Ok(file) = res {
                if let Some(path) = file.path() {
                    build_ui(&app_clone, path);
                    fallback_win_clone.close();
                } else {
                    fallback_win_clone.close();
                }
            } else {
                fallback_win_clone.close();
            }
        });

        fallback_window.present();
    });

    app.run();
}

fn build_ui(app: &gtk4::Application, path: PathBuf) {
    let window = gtk4::ApplicationWindow::new(app);
    window.set_title(Some(&format!("Image Preview - {}", path.file_name().unwrap_or_default().to_string_lossy())));
    window.set_default_size(800, 600);
    window.add_css_class("viewer-window");

    let pixbuf = match gdk_pixbuf::Pixbuf::from_file(&path) {
        Ok(pb) => pb,
        Err(_) => {
            let err_label = gtk4::Label::new(Some("Failed to load image file."));
            err_label.add_css_class("brand-text");
            window.set_child(Some(&err_label));
            window.present();
            return;
        }
    };

    let img_w = pixbuf.width() as f64;
    let img_h = pixbuf.height() as f64;

    let state = Rc::new(RefCell::new(ImageState {
        pixbuf,
        scale: 1.0,
        offset_x: 0.0,
        offset_y: 0.0,
        min_scale: 0.1,
        img_w,
        img_h,
        drag_start_x: 0.0,
        drag_start_y: 0.0,
    }));

    // Setup widget UI container overlay
    let overlay = gtk4::Overlay::new();

    let drawing_area = gtk4::DrawingArea::new();
    drawing_area.set_hexpand(true);
    drawing_area.set_vexpand(true);
    overlay.set_child(Some(&drawing_area));

    // Exif Metadata parsing
    let exif_data = babydra_common::read_exif(&path);

    // --- Bottom-Right Info Box Overlay ---
    let info_box = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
    info_box.add_css_class("info-card");
    info_box.set_halign(gtk4::Align::End);
    info_box.set_valign(gtk4::Align::End);
    info_box.set_margin_end(20);
    info_box.set_margin_bottom(20);

    let name_lbl = gtk4::Label::new(Some(&path.file_name().unwrap_or_default().to_string_lossy()));
    name_lbl.add_css_class("info-item");
    name_lbl.set_halign(gtk4::Align::Start);
    info_box.append(&name_lbl);

    let res_aspect = format_aspect_ratio(img_w as u32, img_h as u32);
    let res_text = if !res_aspect.is_empty() {
        format!("{:.0}x{:.0} ({})", img_w, img_h, res_aspect)
    } else {
        format!("{:.0}x{:.0}", img_w, img_h)
    };
    let resolution_lbl = gtk4::Label::new(Some(&res_text));
    resolution_lbl.add_css_class("info-item");
    resolution_lbl.set_halign(gtk4::Align::Start);
    info_box.append(&resolution_lbl);

    let size_bytes = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
    let size_lbl = gtk4::Label::new(Some(&format_file_size(size_bytes)));
    size_lbl.add_css_class("info-item");
    size_lbl.set_halign(gtk4::Align::Start);
    info_box.append(&size_lbl);

    let scale_lbl = gtk4::Label::new(Some("100%"));
    scale_lbl.add_css_class("info-item");
    scale_lbl.set_halign(gtk4::Align::Start);
    info_box.append(&scale_lbl);

    overlay.add_overlay(&info_box);

    // --- Bottom-Center Zoom Controls Pill ---
    let controls_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    controls_box.add_css_class("controls-bar");
    controls_box.set_halign(gtk4::Align::Center);
    controls_box.set_valign(gtk4::Align::End);
    controls_box.set_margin_bottom(20);

    let zoom_out_btn = gtk4::Button::from_icon_name("zoom-out-symbolic");
    zoom_out_btn.add_css_class("control-btn");
    zoom_out_btn.set_cursor_from_name(Some("pointer"));
    controls_box.append(&zoom_out_btn);

    let reset_btn = gtk4::Button::from_icon_name("zoom-original-symbolic");
    reset_btn.add_css_class("control-btn");
    reset_btn.set_cursor_from_name(Some("pointer"));
    controls_box.append(&reset_btn);

    let zoom_in_btn = gtk4::Button::from_icon_name("zoom-in-symbolic");
    zoom_in_btn.add_css_class("control-btn");
    zoom_in_btn.set_cursor_from_name(Some("pointer"));
    controls_box.append(&zoom_in_btn);

    overlay.add_overlay(&controls_box);

    // --- Centered EXIF Metadata Dialog ---
    let exif_box = gtk4::Box::new(gtk4::Orientation::Vertical, 12);
    exif_box.add_css_class("exif-dialog");
    exif_box.set_halign(gtk4::Align::Center);
    exif_box.set_valign(gtk4::Align::Center);
    exif_box.set_visible(false);

    let exif_title = gtk4::Label::new(Some("Camera Information"));
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

    if let Some(ref data) = exif_data {
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
        let no_exif_lbl = gtk4::Label::new(Some("No EXIF data available"));
        no_exif_lbl.add_css_class("exif-value");
        grid.attach(&no_exif_lbl, 0, 0, 2, 1);
    }

    exif_box.append(&grid);
    overlay.add_overlay(&exif_box);

    // --- Helpers / Closures ---
    let state_draw = state.clone();
    drawing_area.set_draw_func(move |_area, cr, width, height| {
        let st = state_draw.borrow();
        let w = width as f64;
        let h = height as f64;

        // Draw Dark Background
        cr.set_source_rgb(15.0 / 255.0, 15.0 / 255.0, 15.0 / 255.0);
        cr.paint().unwrap();

        // Calculate layout coordinates
        let draw_w = st.img_w * st.scale;
        let draw_h = st.img_h * st.scale;
        let start_x = (w - draw_w) / 2.0 + st.offset_x;
        let start_y = (h - draw_h) / 2.0 + st.offset_y;

        cr.save().unwrap();
        cr.translate(start_x, start_y);
        cr.scale(st.scale, st.scale);
        cr.set_source_pixbuf(&st.pixbuf, 0.0, 0.0);
        cr.paint().unwrap();
        cr.restore().unwrap();
    });

    // --- Event Handlers & Gestures ---
    let state_resize = state.clone();
    let area_clone = drawing_area.clone();
    let scale_lbl_clone = scale_lbl.clone();
    drawing_area.connect_resize(move |_, w, h| {
        let mut st = state_resize.borrow_mut();
        fit_to_screen(&mut st, w as f64, h as f64);
        update_zoom_display(&st, &scale_lbl_clone);
        area_clone.queue_draw();
    });

    let state_out = state.clone();
    let area_out = drawing_area.clone();
    let lbl_out = scale_lbl.clone();
    zoom_out_btn.connect_clicked(move |_| {
        do_zoom(&state_out, &area_out, &lbl_out, -0.1);
    });

    let state_in = state.clone();
    let area_in = drawing_area.clone();
    let lbl_in = scale_lbl.clone();
    zoom_in_btn.connect_clicked(move |_| {
        do_zoom(&state_in, &area_in, &lbl_in, 0.1);
    });

    let state_reset = state.clone();
    let area_reset = drawing_area.clone();
    let lbl_reset = scale_lbl.clone();
    reset_btn.connect_clicked(move |_| {
        let mut st = state_reset.borrow_mut();
        fit_to_screen(&mut st, area_reset.width() as f64, area_reset.height() as f64);
        update_zoom_display(&st, &lbl_reset);
        area_reset.queue_draw();
    });

    // Mouse Scroll Wheel Zoom
    let scroll_controller = gtk4::EventControllerScroll::new(gtk4::EventControllerScrollFlags::VERTICAL);
    let state_scroll = state.clone();
    let area_scroll = drawing_area.clone();
    let lbl_scroll = scale_lbl.clone();
    scroll_controller.connect_scroll(move |_, _, dy| {
        let delta = if dy > 0.0 { -0.1 } else { 0.1 };
        do_zoom(&state_scroll, &area_scroll, &lbl_scroll, delta);
        gtk4::glib::Propagation::Proceed
    });
    drawing_area.add_controller(scroll_controller);

    // Mouse Drag Panning
    let drag_gesture = gtk4::GestureDrag::new();
    let state_drag = state.clone();
    drag_gesture.connect_drag_begin(move |gesture, _x, _y| {
        let mut st = state_drag.borrow_mut();
        st.drag_start_x = st.offset_x;
        st.drag_start_y = st.offset_y;
        gesture.set_state(gtk4::EventSequenceState::Claimed);
    });

    let state_drag_update = state.clone();
    let area_drag_update = drawing_area.clone();
    drag_gesture.connect_drag_update(move |_, offset_x, offset_y| {
        let mut st = state_drag_update.borrow_mut();
        let area_w = area_drag_update.width() as f64;
        let area_h = area_drag_update.height() as f64;

        st.offset_x = st.drag_start_x + offset_x;
        st.offset_y = st.drag_start_y + offset_y;

        clamp_position(&mut st, area_w, area_h);
        area_drag_update.queue_draw();
    });
    drawing_area.add_controller(drag_gesture);

    // Key Events for Exif Box and Zoom Shortcuts
    let key_controller = gtk4::EventControllerKey::new();
    let state_key = state.clone();
    let area_key = drawing_area.clone();
    let lbl_key = scale_lbl.clone();
    let exif_box_clone = exif_box.clone();
    let info_box_clone = info_box.clone();
    let controls_box_clone = controls_box.clone();

    key_controller.connect_key_pressed(move |_, keyval, _, _| {
        match keyval.name().as_deref() {
            Some("i") | Some("I") => {
                exif_box_clone.set_visible(true);
                info_box_clone.set_visible(false);
                controls_box_clone.set_visible(false);
            }
            Some("plus") | Some("equal") => {
                do_zoom(&state_key, &area_key, &lbl_key, 0.1);
            }
            Some("minus") => {
                do_zoom(&state_key, &area_key, &lbl_key, -0.1);
            }
            Some("0") => {
                let mut st = state_key.borrow_mut();
                fit_to_screen(&mut st, area_key.width() as f64, area_key.height() as f64);
                update_zoom_display(&st, &lbl_key);
                area_key.queue_draw();
            }
            _ => {}
        }
        gtk4::glib::Propagation::Proceed
    });

    let exif_box_release = exif_box;
    let info_box_release = info_box;
    let controls_box_release = controls_box;
    key_controller.connect_key_released(move |_, keyval, _, _| {
        if let Some("i") | Some("I") = keyval.name().as_deref() {
            exif_box_release.set_visible(false);
            info_box_release.set_visible(true);
            controls_box_release.set_visible(true);
        }
    });

    window.add_controller(key_controller);

    window.set_child(Some(&overlay));
    window.present();
}
