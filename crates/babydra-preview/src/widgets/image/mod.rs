use crate::widgets::utils::{create_loading_view, show_error};
use crate::widgets::window::{calculate_initial_window_size, create_viewer_window};
use babydra_core::models::preview::ImageState;
use gdk_pixbuf::Pixbuf;
use gtk4::prelude::*;
use gtk4::Application;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

pub mod handlers;
pub mod render;

/// Builds and presents the image viewer window immediately with a loading placeholder,
/// then reads the image file in a background thread and decodes the view seamlessly.
pub fn build_ui(app: &Application, path: PathBuf) {
    let title = path.file_name().unwrap_or_default().to_string_lossy();
    let (window, _) = create_viewer_window(app, &title, 800, 600);

    let loading_view = create_loading_view();
    window.set_child(Some(&loading_view));
    window.present();

    let (tx, rx) = std::sync::mpsc::channel::<Result<Vec<u8>, std::io::Error>>();
    let p = path.clone();
    std::thread::spawn(move || {
        let load_res = std::fs::read(&p);
        let _ = tx.send(load_res);
    });

    let window_clone = window;
    let path_clone = path;

    let mut rx_opt = Some(rx);
    glib::timeout_add_local(std::time::Duration::from_millis(16), move || {
        if let Some(ref rx_chan) = rx_opt {
            if let Ok(load_res) = rx_chan.try_recv() {
                rx_opt = None;
                match load_res {
                    Ok(bytes) => {
                        let glib_bytes = glib::Bytes::from(&bytes);
                        let stream = gio::MemoryInputStream::from_bytes(&glib_bytes);
                        let pixbuf_res = Pixbuf::from_stream(&stream, gio::Cancellable::NONE);

                        match pixbuf_res {
                            Ok(pixbuf) => {
                                let img_w = pixbuf.width() as f64;
                                let img_h = pixbuf.height() as f64;

                                // Resize window to conform to image dimensions and screen constraints
                                let (win_w, win_h) =
                                    calculate_initial_window_size(img_w as u32, img_h as u32);
                                window_clone.set_default_size(win_w, win_h);

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

                                let ui = render::build_image_content(
                                    &window_clone,
                                    &path_clone,
                                    img_w as u32,
                                    img_h as u32,
                                );

                                handlers::setup_cairo_draw(&state, &ui.drawing_area);
                                handlers::setup_image_handlers(&state, &ui, path_clone.clone());

                                ui.drawing_area.queue_draw();
                            }
                            Err(_) => {
                                show_error(&window_clone);
                            }
                        }
                    }
                    Err(_) => {
                        show_error(&window_clone);
                    }
                }
                return glib::ControlFlow::Break;
            }
        }
        glib::ControlFlow::Continue
    });
}
