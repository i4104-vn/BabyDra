//! Text modification handler with debounced syntax highlighting, autosave, and metadata updates.

use crate::widgets::editor::render::{update_window_title, EditorUi};
use crate::widgets::editor::syntax;
use crate::widgets::editor::EditorState;
use babydra_core::i18n::trans;
use babydra_core::models::notepad::load_notepad_cfg;
use gtk4::prelude::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

/// Sets up buffer change listener with debounced syntax highlighting, autosave, and metadata updates.
pub fn setup_text_change_handler(state: &Rc<RefCell<EditorState>>, ui: &EditorUi) {
    let change_counter = Rc::new(Cell::new(0u64));

    let window = ui.window.clone();
    let status_modified_lbl = ui.status_modified_lbl.clone();
    let status_lines_lbl = ui.status_lines_lbl.clone();
    let text_buffer = ui.text_buffer.clone();

    let state_clone = state.clone();
    let counter_clone = change_counter.clone();
    let ui_clone = ui.clone();

    ui.text_buffer.connect_changed(move |tb| {
        let mut st = state_clone.borrow_mut();
        if !st.is_modified {
            st.is_modified = true;
            let file_name = st
                .file_path
                .as_ref()
                .and_then(|p| p.file_name())
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| trans("notepad.untitled"));
            update_window_title(&window, &file_name, true);
            status_modified_lbl.set_visible(true);
        }

        let line_count = tb.line_count();
        st.line_count = line_count as usize;
        let lines_str = trans("notepad.lines_count").replace("{}", &line_count.to_string());
        status_lines_lbl.set_text(&lines_str);

        let size_bytes = tb.text(&tb.start_iter(), &tb.end_iter(), true).len() as u64;
        st.file_size = size_bytes;
        drop(st);

        // Bump debounce counter
        let current_id = counter_clone.get().wrapping_add(1);
        counter_clone.set(current_id);

        // 1. Debounced syntax highlighting (200ms)
        let counter_check_syntax = counter_clone.clone();
        let buf = text_buffer.clone();
        let st_ref = state_clone.clone();

        glib::timeout_add_local_once(std::time::Duration::from_millis(200), move || {
            if counter_check_syntax.get() == current_id {
                let is_dark = babydra_ui_kit::ui::theme::is_dark_mode();
                let settings = load_notepad_cfg();
                let p_opt = st_ref.borrow().file_path.clone();
                let end_iter = buf.iter_at_line(1).unwrap_or_else(|| buf.end_iter());
                let first_line = buf.text(&buf.start_iter(), &end_iter, false);
                let syntax = syntax::detect_syntax(p_opt.as_deref(), Some(first_line.as_str()));
                syntax::apply_highlighting_for_mode(
                    &buf,
                    syntax,
                    is_dark,
                    &settings.dark_theme,
                    &settings.light_theme,
                );
            }
        });

        // 2. Debounced auto-save (configurable seconds)
        let settings = load_notepad_cfg();
        if settings.auto_save {
            let delay_secs = settings.auto_save_delay_seconds.max(1) as u64;
            let counter_check_save = counter_clone.clone();
            let st_save = state_clone.clone();
            let ui_save = ui_clone.clone();

            glib::timeout_add_local_once(std::time::Duration::from_secs(delay_secs), move || {
                if counter_check_save.get() == current_id {
                    let current_settings = load_notepad_cfg();
                    if current_settings.auto_save {
                        let has_path = st_save.borrow().file_path.is_some();
                        let is_mod = st_save.borrow().is_modified;
                        if has_path && is_mod {
                            crate::widgets::editor::handlers::keys::save_document(
                                &st_save, &ui_save,
                            );
                        }
                    }
                }
            });
        }
    });
}
