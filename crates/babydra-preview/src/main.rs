//! Dynamic desktop image preview application.
//! Base Rust + GTK4 image viewer entry point.

use babydra_core::i18n::trans;
use gtk4::gdk::{DragAction, FileList};
use gtk4::gio;
use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box, Button, DropTarget, FileDialog, FileFilter,
    GestureClick, Label, Orientation,
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

/// Builds the black welcome window with centered logo, "Open File" button, and drag-and-drop target.
fn build_welcome_window(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.set_title(Some(&trans("common.app_preview_title")));
    window.set_icon_name(Some("babydra-preview"));
    window.set_default_size(720, 480);
    window.add_css_class("viewer-window");

    let is_picking = Rc::new(Cell::new(false));

    let root_box = Box::new(Orientation::Vertical, 0);
    root_box.set_hexpand(true);
    root_box.set_vexpand(true);
    root_box.set_halign(Align::Fill);
    root_box.set_valign(Align::Fill);

    let welcome_box = Box::new(Orientation::Vertical, 16);
    welcome_box.add_css_class("preview-welcome-box");
    welcome_box.set_halign(Align::Center);
    welcome_box.set_valign(Align::Center);

    // 1. Center Icon Logo
    let icon = babydra_ui_kit::ui::icon::get_icon("babydra-preview", 72);
    icon.add_css_class("preview-welcome-icon");
    welcome_box.append(&icon);

    // 2. "Open File" Button
    let open_btn = Button::with_label(&trans("preview.open_file"));
    open_btn.add_css_class("preview-open-btn");
    open_btn.set_cursor_from_name(Some("pointer"));
    open_btn.set_halign(Align::Center);

    let app_btn = app.clone();
    let win_btn = window.clone();
    let pick_btn = is_picking.clone();
    open_btn.connect_clicked(move |_| {
        open_file_picker(&win_btn, &app_btn, &pick_btn);
    });
    welcome_box.append(&open_btn);

    // 3. Drag and Drop Hint
    let hint_lbl = Label::new(Some(&trans("preview.drag_drop_hint")));
    hint_lbl.add_css_class("preview-hint-label");
    hint_lbl.set_halign(Align::Center);
    welcome_box.append(&hint_lbl);

    // Clicking anywhere on the welcome box also opens the file picker
    let click_gesture = GestureClick::new();
    let win_click = window.clone();
    let app_click = app.clone();
    let pick_click = is_picking.clone();
    click_gesture.connect_pressed(move |_, _, _, _| {
        open_file_picker(&win_click, &app_click, &pick_click);
    });
    welcome_box.add_controller(click_gesture);

    // Drag and Drop target on the entire window
    let drop_target = DropTarget::new(
        glib::types::Type::INVALID,
        DragAction::COPY,
    );
    drop_target.set_types(&[FileList::static_type(), gio::File::static_type()]);

    let box_enter = welcome_box.clone();
    drop_target.connect_enter(move |_, _, _| {
        box_enter.add_css_class("drag-over");
        DragAction::COPY
    });

    let box_leave = welcome_box.clone();
    drop_target.connect_leave(move |_| {
        box_leave.remove_css_class("drag-over");
    });

    let app_drop = app.clone();
    let win_drop = window.clone();
    let box_drop = welcome_box.clone();
    drop_target.connect_drop(move |_, value, _, _| {
        box_drop.remove_css_class("drag-over");

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

    root_box.append(&welcome_box);
    window.set_child(Some(&root_box));
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

        // Show the black welcome page with logo, open file button, and drag-and-drop target
        build_welcome_window(app);
    });

    let exit_code = app.run().value();
    std::process::exit(exit_code);
}
