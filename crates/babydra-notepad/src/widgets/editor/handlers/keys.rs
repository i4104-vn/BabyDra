//! Keyboard shortcuts and file operation handlers (Save, Save As, Open, New, Settings).

use crate::widgets::editor::render::{
    apply_editor_settings, show_toast, update_window_title, EditorUi,
};
use crate::widgets::editor::syntax;
use crate::widgets::editor::EditorState;
use crate::widgets::settings_dialog::show_settings_dialog;
use babydra_core::i18n::trans;
use babydra_core::models::notepad::load_notepad_cfg;
use gtk4::gdk::ModifierType;
use gtk4::gio;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, EventControllerKey, FileDialog, FileFilter};
use std::cell::RefCell;
use std::rc::Rc;

/// Opens the notepad settings dialog and updates editor styling when changes occur.
pub fn open_settings_dialog(ui: &EditorUi, state: &Rc<RefCell<EditorState>>) {
    let ui_c = ui.clone();
    let state_c = state.clone();

    show_settings_dialog(&ui.window, move |new_settings| {
        apply_editor_settings(&ui_c, &new_settings);

        // Re-highlight syntax with updated theme
        let is_dark = babydra_ui_kit::ui::theme::is_dark_mode();
        let p_opt = state_c.borrow().file_path.clone();
        let end_iter = ui_c
            .text_buffer
            .iter_at_line(1)
            .unwrap_or_else(|| ui_c.text_buffer.end_iter());
        let first_line = ui_c
            .text_buffer
            .text(&ui_c.text_buffer.start_iter(), &end_iter, false);
        let syntax = syntax::detect_syntax(p_opt.as_deref(), Some(first_line.as_str()));
        syntax::apply_highlighting_for_mode(
            &ui_c.text_buffer,
            syntax,
            is_dark,
            &new_settings.dark_theme,
            &new_settings.light_theme,
        );
    });
}

/// Formats text according to user preferences (whitespace trimming, final newline).
fn format_text_for_save(raw: &str) -> String {
    let settings = load_notepad_cfg();
    let mut text = if settings.trim_trailing_whitespace {
        let lines: Vec<&str> = raw.lines().map(|l| l.trim_end()).collect();
        lines.join("\n")
    } else {
        raw.to_string()
    };

    if settings.insert_final_newline && !text.is_empty() && !text.ends_with('\n') {
        text.push('\n');
    }

    text
}

/// Saves the currently open document to disk. If no path is assigned, prompts Save As.
pub fn save_document(state: &Rc<RefCell<EditorState>>, ui: &EditorUi) {
    let has_path = state.borrow().file_path.is_some();
    if has_path {
        let path = state.borrow().file_path.clone().unwrap();
        let raw_text = ui.text_buffer.text(
            &ui.text_buffer.start_iter(),
            &ui.text_buffer.end_iter(),
            true,
        );
        let formatted = format_text_for_save(raw_text.as_str());

        match std::fs::write(&path, formatted.as_str()) {
            Ok(()) => {
                let mut st = state.borrow_mut();
                st.is_modified = false;
                let file_name = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                update_window_title(&ui.window, &file_name, false);
                ui.status_modified_lbl.set_visible(false);
                let saved_msg = format!("{} {}", trans("notepad.save"), file_name);
                show_toast(ui, &saved_msg);
            }
            Err(e) => {
                let err_msg = format!("{}: {}", trans("notepad.failed_save"), e);
                show_toast(ui, &err_msg);
            }
        }
    } else {
        save_document_as(state, ui);
    }
}

/// Prompts the user with a file picker to save document to a new location.
pub fn save_document_as(state: &Rc<RefCell<EditorState>>, ui: &EditorUi) {
    let file_dialog = FileDialog::new();
    file_dialog.set_title(&trans("notepad.save_as"));

    let st = state.borrow();
    if let Some(ref path) = st.file_path {
        if let Some(parent) = path.parent() {
            let gfile = gio::File::for_path(parent);
            file_dialog.set_initial_folder(Some(&gfile));
        }
        if let Some(name) = path.file_name() {
            file_dialog.set_initial_name(Some(&name.to_string_lossy()));
        }
    }
    drop(st);

    let state_clone = state.clone();
    let window_clone = ui.window.clone();
    let text_buffer_clone = ui.text_buffer.clone();
    let status_modified_lbl_clone = ui.status_modified_lbl.clone();
    let status_file_lbl_clone = ui.status_file_lbl.clone();
    let status_lang_lbl_clone = ui.status_lang_lbl.clone();
    let toast_lbl_clone = ui.toast_lbl.clone();
    let toast_revealer_clone = ui.toast_revealer.clone();

    file_dialog.save(Some(&ui.window), None::<&gio::Cancellable>, move |res| {
        if let Ok(file) = res {
            if let Some(path) = file.path() {
                let raw_text = text_buffer_clone.text(
                    &text_buffer_clone.start_iter(),
                    &text_buffer_clone.end_iter(),
                    true,
                );
                let formatted = format_text_for_save(raw_text.as_str());

                match std::fs::write(&path, formatted.as_str()) {
                    Ok(()) => {
                        let mut st = state_clone.borrow_mut();
                        st.is_modified = false;
                        st.file_path = Some(path.clone());
                        let file_name = path
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string();

                        update_window_title(&window_clone, &file_name, false);
                        status_modified_lbl_clone.set_visible(false);
                        status_file_lbl_clone.set_text(&file_name);

                        // Detect and apply updated syntax
                        let syntax_ref = syntax::detect_syntax(Some(&path), None);
                        let lang = syntax::language_name(syntax_ref);
                        st.language = lang.clone();
                        status_lang_lbl_clone.set_text(&lang);

                        let settings = load_notepad_cfg();
                        syntax::apply_highlighting_for_mode(
                            &text_buffer_clone,
                            syntax_ref,
                            babydra_ui_kit::ui::theme::is_dark_mode(),
                            &settings.dark_theme,
                            &settings.light_theme,
                        );

                        let saved_msg = format!("{} {}", trans("notepad.save"), file_name);
                        toast_lbl_clone.set_text(&saved_msg);
                        toast_revealer_clone.set_reveal_child(true);
                        let rev = toast_revealer_clone.clone();
                        glib::timeout_add_local_once(
                            std::time::Duration::from_millis(2500),
                            move || {
                                rev.set_reveal_child(false);
                            },
                        );
                    }
                    Err(e) => {
                        let err_msg = format!("{}: {}", trans("notepad.failed_save"), e);
                        toast_lbl_clone.set_text(&err_msg);
                        toast_revealer_clone.set_reveal_child(true);
                        let rev = toast_revealer_clone.clone();
                        glib::timeout_add_local_once(
                            std::time::Duration::from_millis(3000),
                            move || {
                                rev.set_reveal_child(false);
                            },
                        );
                    }
                }
            }
        }
    });
}

/// Launches a native file picker to open a text file.
pub fn open_file_dialog(app: &Application, window: &ApplicationWindow) {
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
    file_dialog.open(Some(window), None::<&gio::Cancellable>, move |res| {
        if let Ok(file) = res {
            if let Some(path) = file.path() {
                if path.exists() {
                    crate::widgets::build_ui(&app_clone, path);
                }
            }
        }
    });
}

/// Registers the keyboard event controller on the editor window.
pub fn setup_key_controller(state: &Rc<RefCell<EditorState>>, ui: &EditorUi, app: &Application) {
    // Connect status bar settings button click
    let ui_settings = ui.clone();
    let state_settings = state.clone();
    ui.btn_settings.connect_clicked(move |_| {
        open_settings_dialog(&ui_settings, &state_settings);
    });

    let key_controller = EventControllerKey::new();

    let state_save = state.clone();
    let state_save_as = state.clone();
    let state_settings_k = state.clone();
    let ui_save = ui.clone();
    let ui_save_as = ui.clone();
    let ui_settings_k = ui.clone();
    let app_clone = app.clone();
    let win_open = ui.window.clone();
    let text_buf = ui.text_buffer.clone();

    key_controller.connect_key_pressed(move |_, keyval, _, modifier| {
        let is_ctrl = modifier.contains(ModifierType::CONTROL_MASK);
        let is_shift = modifier.contains(ModifierType::SHIFT_MASK);

        if is_ctrl {
            match keyval.name().as_deref() {
                // Ctrl+Shift+S: Save As
                Some("s") | Some("S") if is_shift => {
                    save_document_as(&state_save_as, &ui_save_as);
                    return glib::Propagation::Stop;
                }
                // Ctrl+S: Save
                Some("s") | Some("S") => {
                    save_document(&state_save, &ui_save);
                    return glib::Propagation::Stop;
                }
                // Ctrl+O: Open
                Some("o") | Some("O") => {
                    open_file_dialog(&app_clone, &win_open);
                    return glib::Propagation::Stop;
                }
                // Ctrl+N: New Document
                Some("n") | Some("N") => {
                    crate::widgets::build_ui_new(&app_clone);
                    return glib::Propagation::Stop;
                }
                // Ctrl+,: Settings Dialog
                Some("comma") | Some(",") => {
                    open_settings_dialog(&ui_settings_k, &state_settings_k);
                    return glib::Propagation::Stop;
                }
                _ => {}
            }
        } else {
            let settings = load_notepad_cfg();

            // Tab key interception: indent with spaces
            if keyval.name().as_deref() == Some("Tab") && settings.indent_with_spaces {
                let spaces = " ".repeat(settings.tab_size.max(1) as usize);
                text_buf.insert_at_cursor(&spaces);
                return glib::Propagation::Stop;
            }

            // Return / Enter key interception: auto-indent
            if (keyval.name().as_deref() == Some("Return")
                || keyval.name().as_deref() == Some("KP_Enter"))
                && settings.auto_indent
            {
                let mark = text_buf.get_insert();
                let iter = text_buf.iter_at_mark(&mark);
                let mut line_start = iter;
                line_start.set_line_offset(0);
                let line_prefix = text_buf.text(&line_start, &iter, false);
                let indent: String = line_prefix
                    .chars()
                    .take_while(|c| *c == ' ' || *c == '\t')
                    .collect();

                if !indent.is_empty() {
                    let text_to_insert = format!("\n{}", indent);
                    text_buf.insert_at_cursor(&text_to_insert);
                    return glib::Propagation::Stop;
                }
            }
        }

        glib::Propagation::Proceed
    });

    ui.window.add_controller(key_controller);
}
