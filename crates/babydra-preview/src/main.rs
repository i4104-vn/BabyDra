//! Dynamic desktop image preview application.
//! Base Rust + GTK4 image viewer entry point.

use babydra_core::i18n::trans;
use gtk4::gdk::{DragAction, FileList, ModifierType};
use gtk4::gio;
use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box, Button, DropTarget, EventControllerKey, FileDialog,
    FileFilter, Label, Orientation,
};
use std::cell::Cell;
use std::path::PathBuf;
use std::rc::Rc;

mod widgets;

/// Launches the native GTK file picker dialog.
fn open_file_picker(
    window: &ApplicationWindow,
    app: &Application,
    is_picking: &Rc<Cell<bool>>,
) {
    if is_picking.get() {
        return;
    }
    is_picking.set(true);

    let file_dialog = FileDialog::new();
    file_dialog.set_title(&trans("common.open_image_file"));

    let filter = FileFilter::new();
    filter.set_name(Some(&trans("preview.media_filter")));
    filter.add_mime_type("image/png");
    filter.add_mime_type("image/jpeg");
    filter.add_mime_type("image/webp");
    filter.add_mime_type("video/mp4");
    filter.add_mime_type("video/x-matroska");
    filter.add_mime_type("video/webm");
    filter.add_mime_type("video/quicktime");
    filter.add_mime_type("video/x-msvideo");
    filter.add_pattern("*.mp4");
    filter.add_pattern("*.mkv");
    filter.add_pattern("*.webm");
    filter.add_pattern("*.mov");
    filter.add_pattern("*.avi");
    filter.add_pattern("*.png");
    filter.add_pattern("*.jpg");
    filter.add_pattern("*.jpeg");
    filter.add_pattern("*.webp");
    file_dialog.set_default_filter(Some(&filter));

    let app_clone = app.clone();
    let win_clone = window.clone();
    let picking_flag = is_picking.clone();

    file_dialog.open(
        Some(window),
        None::<&gio::Cancellable>,
        move |res| {
            picking_flag.set(false);
            if let Ok(file) = res {
                if let Some(path) = file.path() {
                    if path.exists() {
                        widgets::build_ui(&app_clone, path);
                        win_clone.close();
                    }
                }
            }
        },
    );
}

/// Builds the flat, centered welcome window matching the minimalist aesthetic.
fn build_welcome_window(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.set_title(Some(&trans("common.app_preview_title")));
    window.set_icon_name(Some("babydra-preview"));
    window.set_default_size(720, 480);
    window.add_css_class("viewer-window");

    let is_picking = Rc::new(Cell::new(false));

    // Outer container: centered horizontally and vertically
    let center_box = Box::new(Orientation::Vertical, 12);
    center_box.set_hexpand(true);
    center_box.set_vexpand(true);
    center_box.set_halign(Align::Center);
    center_box.set_valign(Align::Center);

    // 1. Center Icon Logo
    let icon = babydra_ui_kit::ui::icon::get_icon("babydra-preview", 64);
    icon.set_halign(Align::Center);
    center_box.append(&icon);

    // 2. Title Label ("BabyDra Preview")
    let title_lbl = Label::new(Some("BabyDra Preview"));
    title_lbl.add_css_class("preview-flat-title");
    title_lbl.set_halign(Align::Center);
    center_box.append(&title_lbl);

    // 3. Flat Action Row (Open File + Ctrl + O keycap)
    let action_btn = Button::new();
    action_btn.add_css_class("preview-flat-action");
    action_btn.set_cursor_from_name(Some("pointer"));
    action_btn.set_margin_top(28);
    action_btn.set_halign(Align::Center);

    let row = Box::new(Orientation::Horizontal, 110);
    row.set_valign(Align::Center);

    let text_lbl = Label::new(Some(&trans("preview.open_file")));
    text_lbl.add_css_class("preview-flat-action-text");
    text_lbl.set_halign(Align::Start);
    text_lbl.set_hexpand(true);
    row.append(&text_lbl);

    let keycap_lbl = Label::new(Some("Ctrl + O"));
    keycap_lbl.add_css_class("preview-keycap");
    keycap_lbl.set_halign(Align::End);
    row.append(&keycap_lbl);

    action_btn.set_child(Some(&row));

    let app_btn = app.clone();
    let win_btn = window.clone();
    let pick_btn = is_picking.clone();
    action_btn.connect_clicked(move |_| {
        open_file_picker(&win_btn, &app_btn, &pick_btn);
    });
    center_box.append(&action_btn);

    // Keyboard shortcut (Ctrl + O)
    let key_controller = EventControllerKey::new();
    let win_key = window.clone();
    let app_key = app.clone();
    let pick_key = is_picking.clone();
    key_controller.connect_key_pressed(move |_, keyval, _, state| {
        if state.contains(ModifierType::CONTROL_MASK) {
            if let Some("o") | Some("O") = keyval.name().as_deref() {
                open_file_picker(&win_key, &app_key, &pick_key);
                return gtk4::glib::Propagation::Stop;
            }
        }
        gtk4::glib::Propagation::Proceed
    });
    window.add_controller(key_controller);

    // Drag and Drop target across the window
    let drop_target = DropTarget::new(
        glib::types::Type::INVALID,
        DragAction::COPY,
    );
    drop_target.set_types(&[FileList::static_type(), gio::File::static_type()]);

    let app_drop = app.clone();
    let win_drop = window.clone();
    drop_target.connect_drop(move |_, value, _, _| {
        let path_opt = if let Ok(file_list) = value.get::<FileList>() {
            file_list.files().first().and_then(|f| f.path())
        } else if let Ok(file) = value.get::<gio::File>() {
            file.path()
        } else {
            None
        };

        if let Some(path) = path_opt {
            if path.exists() {
                widgets::build_ui(&app_drop, path);
                win_drop.close();
                return true;
            }
        }
        false
    });
    window.add_controller(drop_target);

    window.set_child(Some(&center_box));
    window.present();
}

/// Application entry point: `main`.
fn main() {
    babydra_core::services::logger::init_logger("babydra-preview", "babydra-preview.log");

    let app = Application::builder()
        .application_id("com.babydra.preview")
        .flags(gtk4::gio::ApplicationFlags::HANDLES_OPEN | gtk4::gio::ApplicationFlags::NON_UNIQUE)
        .build();

    app.connect_open(|app, files, _hint| {
        babydra_ui_kit::ui::theme::init_theme();
        for file in files {
            if let Some(path) = file.path() {
                if path.exists() {
                    widgets::build_ui(app, path);
                }
            }
        }
    });

    app.connect_activate(|app| {
        babydra_ui_kit::ui::theme::init_theme();

        let arg_path = std::env::args().nth(1);
        if let Some(p) = arg_path {
            let path = PathBuf::from(p);
            if path.exists() {
                widgets::build_ui(app, path);
                return;
            }
        }

        // Show the minimalist flat welcome page centered vertically and horizontally
        build_welcome_window(app);
    });

    let exit_code = app.run().value();
    std::process::exit(exit_code);
}
