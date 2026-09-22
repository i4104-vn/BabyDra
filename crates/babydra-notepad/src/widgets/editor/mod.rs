//! Editor component module coordinator for BabyDra Notepad.

use babydra_core::i18n::trans;
use gtk4::prelude::*;
use gtk4::{Align, Application, ApplicationWindow, Box, Label, Orientation, Spinner};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

pub mod handlers;
pub mod render;
pub mod syntax;

pub use babydra_core::models::notepad::EditorState;

/// Creates a centered loading spinner placeholder while a file is read.
fn create_loading_view() -> Box {
    let loading_box = Box::new(Orientation::Vertical, 10);
    loading_box.set_halign(Align::Center);
    loading_box.set_valign(Align::Center);
    loading_box.set_hexpand(true);
    loading_box.set_vexpand(true);

    let spinner = Spinner::new();
    spinner.set_spinning(true);
    spinner.set_size_request(32, 32);
    loading_box.append(&spinner);

    let label = Label::new(Some(&trans("common.pending")));
    label.add_css_class("dim-label");
    loading_box.append(&label);
    loading_box
}

/// Shows an error message in the window when a file fails to load.
fn show_error(window: &ApplicationWindow) {
    let label = Label::new(Some(&trans("notepad.failed_load")));
    label.add_css_class("preview-error");
    label.set_halign(Align::Center);
    label.set_valign(Align::Center);
    label.set_hexpand(true);
    label.set_vexpand(true);
    window.set_child(Some(&label));
}

/// Builds and presents an editor window for an existing file on disk.
/// Reads the file content asynchronously on a background thread to prevent GUI lag.
pub fn build_ui(app: &Application, path: PathBuf) {
    let window = ApplicationWindow::new(app);
    let title = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| trans("notepad.untitled"));

    window.set_title(Some(&title));
    window.set_icon_name(Some("babydra-notepad"));
    window.set_default_size(960, 640);
    window.add_css_class("viewer-window");
    window.add_css_class("explore-window");

    let loading_view = create_loading_view();
    window.set_child(Some(&loading_view));
    window.present();

    let (tx, rx) = std::sync::mpsc::channel::<Result<Vec<u8>, std::io::Error>>();
    let p = path.clone();
    std::thread::spawn(move || {
        let res = std::fs::read(&p);
        let _ = tx.send(res);
    });

    let window_clone = window;
    let path_clone = path;
    let app_clone = app.clone();

    let mut rx_opt = Some(rx);
    glib::timeout_add_local(std::time::Duration::from_millis(16), move || {
        if let Some(ref rx_chan) = rx_opt {
            if let Ok(load_res) = rx_chan.try_recv() {
                rx_opt = None;
                match load_res {
                    Ok(bytes) => {
                        let (content, encoding) = match String::from_utf8(bytes) {
                            Ok(text) => (text, "UTF-8".to_string()),
                            Err(e) => {
                                let lossy = String::from_utf8_lossy(e.as_bytes()).to_string();
                                (lossy, "UTF-8 (Lossy)".to_string())
                            }
                        };

                        let first_line = content.lines().next().unwrap_or_default();
                        let syntax_ref = syntax::detect_syntax(Some(&path_clone), Some(first_line));
                        let lang = syntax::language_name(syntax_ref);

                        let ui = render::build_editor_layout(
                            &window_clone,
                            Some(&path_clone),
                            &content,
                            &lang,
                            &encoding,
                        );

                        // Apply syntax highlighting
                        let is_dark = babydra_ui_kit::ui::theme::is_dark_mode();
                        let settings = babydra_core::models::notepad::load_notepad_cfg();
                        syntax::apply_highlighting_for_mode(
                            &ui.text_buffer,
                            syntax_ref,
                            is_dark,
                            &settings.dark_theme,
                            &settings.light_theme,
                        );

                        let file_size = std::fs::metadata(&path_clone).map_or(0, |m| m.len());
                        let line_count = ui.text_buffer.line_count() as usize;

                        let state = Rc::new(RefCell::new(EditorState::new(
                            Some(path_clone.clone()),
                            lang,
                            encoding,
                            line_count,
                            file_size,
                        )));

                        handlers::setup_editor_handlers(&state, &ui, &app_clone);
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

/// Builds and presents an editor window with a fresh untitled document.
pub fn build_ui_new(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.set_icon_name(Some("babydra-notepad"));

    let lang = trans("notepad.plain_text");
    let encoding = "UTF-8".to_string();

    let ui = render::build_editor_layout(&window, None, "", &lang, &encoding);

    let state = Rc::new(RefCell::new(EditorState::new(None, lang, encoding, 1, 0)));

    handlers::setup_editor_handlers(&state, &ui, app);
    window.present();
}
