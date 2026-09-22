//! BabyDra Notepad — Text editor with syntax highlighting.
//! Rust + GTK4 desktop text editor entry point.

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
fn open_file_picker(window: &ApplicationWindow, app: &Application, is_picking: &Rc<Cell<bool>>) {
    if is_picking.get() {
        return;
    }
    is_picking.set(true);

    let file_dialog = FileDialog::new();
    file_dialog.set_title(&trans("notepad.open_file"));

    let filter = FileFilter::new();
    filter.set_name(Some(&trans("notepad.text_filter")));
    filter.add_mime_type("text/plain");
    filter.add_mime_type("text/*");
    filter.add_mime_type("application/json");
    filter.add_mime_type("application/xml");
    filter.add_mime_type("application/javascript");
    filter.add_pattern("*.txt");
    filter.add_pattern("*.rs");
    filter.add_pattern("*.py");
    filter.add_pattern("*.js");
    filter.add_pattern("*.ts");
    filter.add_pattern("*.json");
    filter.add_pattern("*.toml");
    filter.add_pattern("*.yaml");
    filter.add_pattern("*.yml");
    filter.add_pattern("*.md");
    filter.add_pattern("*.c");
    filter.add_pattern("*.cpp");
    filter.add_pattern("*.h");
    filter.add_pattern("*.sh");
    filter.add_pattern("*.css");
    filter.add_pattern("*.html");

    let all_filter = FileFilter::new();
    all_filter.set_name(Some(&trans("notepad.all_files")));
    all_filter.add_pattern("*");

    let filters = gio::ListStore::new::<FileFilter>();
    filters.append(&filter);
    filters.append(&all_filter);
    file_dialog.set_filters(Some(&filters));
    file_dialog.set_default_filter(Some(&filter));

    let app_clone = app.clone();
    let win_clone = window.clone();
    let picking_flag = is_picking.clone();

    file_dialog.open(Some(window), None::<&gio::Cancellable>, move |res| {
        picking_flag.set(false);
        if let Ok(file) = res {
            if let Some(path) = file.path() {
                if path.exists() {
                    widgets::build_ui(&app_clone, path);
                    win_clone.close();
                }
            }
        }
    });
}

/// Builds an action button with text and a styled keycap badge.
fn create_flat_action_button(text: &str, shortcut: &str) -> (Button, Box) {
    let action_btn = Button::new();
    action_btn.add_css_class("preview-flat-action");
    action_btn.set_cursor_from_name(Some("pointer"));
    action_btn.set_halign(Align::Center);

    let row = Box::new(Orientation::Horizontal, 80);
    row.set_valign(Align::Center);

    let text_lbl = Label::new(Some(text));
    text_lbl.add_css_class("preview-flat-action-text");
    text_lbl.set_halign(Align::Start);
    text_lbl.set_hexpand(true);
    row.append(&text_lbl);

    let keycap_lbl = Label::new(Some(shortcut));
    keycap_lbl.add_css_class("preview-keycap");
    keycap_lbl.set_halign(Align::End);
    row.append(&keycap_lbl);

    action_btn.set_child(Some(&row));
    (action_btn, row)
}

/// Builds the flat, centered welcome window matching the BabyDra minimalist aesthetic.
fn build_welcome_window(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.set_title(Some(&trans("common.app_notepad_title")));
    window.set_icon_name(Some("babydra-notepad"));
    window.set_default_size(720, 480);
    window.add_css_class("viewer-window");

    let is_picking = Rc::new(Cell::new(false));

    // Outer container: centered horizontally and vertically
    let center_box = Box::new(Orientation::Vertical, 10);
    center_box.set_hexpand(true);
    center_box.set_vexpand(true);
    center_box.set_halign(Align::Center);
    center_box.set_valign(Align::Center);

    // 1. Center Icon Logo
    let icon = babydra_ui_kit::ui::icon::get_icon("edit", 64);
    icon.set_halign(Align::Center);
    center_box.append(&icon);

    // 2. Title Label ("BabyDra Notepad")
    let title_lbl = Label::new(Some("BabyDra Notepad"));
    title_lbl.add_css_class("preview-flat-title");
    title_lbl.set_halign(Align::Center);
    center_box.append(&title_lbl);

    // Spacer
    let spacer = Box::new(Orientation::Vertical, 0);
    spacer.set_margin_top(16);
    center_box.append(&spacer);

    // 3. Action Rows: Open File & New File
    let (open_btn, _) = create_flat_action_button(&trans("notepad.open_file"), "Ctrl + O");
    let app_open = app.clone();
    let win_open = window.clone();
    let pick_open = is_picking.clone();
    open_btn.connect_clicked(move |_| {
        open_file_picker(&win_open, &app_open, &pick_open);
    });
    center_box.append(&open_btn);

    let (new_btn, _) = create_flat_action_button(&trans("notepad.new_file"), "Ctrl + N");
    let app_new = app.clone();
    let win_new = window.clone();
    new_btn.connect_clicked(move |_| {
        widgets::build_ui_new(&app_new);
        win_new.close();
    });
    center_box.append(&new_btn);

    // Keyboard shortcuts (Ctrl + O, Ctrl + N)
    let key_controller = EventControllerKey::new();
    let win_key = window.clone();
    let app_key = app.clone();
    let pick_key = is_picking.clone();
    key_controller.connect_key_pressed(move |_, keyval, _, state| {
        if state.contains(ModifierType::CONTROL_MASK) {
            match keyval.name().as_deref() {
                Some("o") | Some("O") => {
                    open_file_picker(&win_key, &app_key, &pick_key);
                    return glib::Propagation::Stop;
                }
                Some("n") | Some("N") => {
                    widgets::build_ui_new(&app_key);
                    win_key.close();
                    return glib::Propagation::Stop;
                }
                _ => {}
            }
        }
        glib::Propagation::Proceed
    });
    window.add_controller(key_controller);

    // Drag and Drop target across the window
    let drop_target = DropTarget::new(glib::types::Type::INVALID, DragAction::COPY);
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
    babydra_core::services::logger::init_logger("babydra-notepad", "babydra-notepad.log");

    let app = Application::builder()
        .application_id("com.babydra.notepad")
        .flags(gio::ApplicationFlags::HANDLES_OPEN | gio::ApplicationFlags::NON_UNIQUE)
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

        build_welcome_window(app);
    });

    let exit_code = app.run().value();
    std::process::exit(exit_code);
}
