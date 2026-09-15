use babydra_core::i18n::trans;
use babydra_core::models::preview::ImageState;
use gdk_pixbuf::Pixbuf;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Label};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

pub mod handlers;
pub mod render;

/// Builds and presents the image viewer window for the given image path.
pub fn build_ui(app: &Application, path: PathBuf) {
    let pixbuf = match Pixbuf::from_file(&path) {
        Ok(pb) => pb,
        Err(_) => {
            let err_window = ApplicationWindow::new(app);
            err_window.set_default_size(600, 400);
            err_window.add_css_class("viewer-window");
            let err_label = Label::new(Some(&trans("preview.failed_load")));
            err_label.add_css_class("brand-text");
            err_window.set_child(Some(&err_label));
            err_window.present();
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

    let ui = render::build_image_ui(app, &path, img_w as u32, img_h as u32);

    handlers::setup_cairo_draw(&state, &ui.drawing_area);
    handlers::setup_image_handlers(&state, &ui);

    ui.window.present();
}
