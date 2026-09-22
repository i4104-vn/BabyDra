//! Editor UI rendering and layout assembly for BabyDra Notepad.

use babydra_core::i18n::trans;
use babydra_core::models::notepad::{load_notepad_cfg, NotepadSettings};
use gtk4::prelude::*;
use gtk4::{
    Align, ApplicationWindow, Box, Button, CssProvider, DrawingArea, Label, Orientation, Overlay,
    Revealer, RevealerTransitionType, ScrolledWindow, Separator, TextBuffer, TextView, WrapMode,
};
use std::path::Path;

/// The full set of UI widgets for the editor window.
#[derive(Clone)]
#[allow(dead_code)]
pub struct EditorUi {
    pub window: ApplicationWindow,
    pub text_view: TextView,
    pub text_buffer: TextBuffer,
    pub line_numbers: DrawingArea,
    pub scrolled_window: ScrolledWindow,
    pub status_bar: Box,
    pub status_modified_lbl: Label,
    pub status_file_lbl: Label,
    pub status_cursor_lbl: Label,
    pub status_lines_lbl: Label,
    pub status_encoding_lbl: Label,
    pub status_lang_lbl: Label,
    pub btn_settings: Button,
    pub font_provider: CssProvider,
    pub toast_revealer: Revealer,
    pub toast_lbl: Label,
}

/// Updates the window title and modified indicator.
pub fn update_window_title(window: &ApplicationWindow, file_name: &str, is_modified: bool) {
    let title = if is_modified {
        format!("● {}", file_name)
    } else {
        file_name.to_string()
    };
    window.set_title(Some(&title));
}

/// Shows a brief toast message at the top center of the editor.
pub fn show_toast(ui: &EditorUi, message: &str) {
    ui.toast_lbl.set_text(message);
    ui.toast_revealer.set_reveal_child(true);

    let revealer = ui.toast_revealer.clone();
    glib::timeout_add_local_once(std::time::Duration::from_millis(2500), move || {
        revealer.set_reveal_child(false);
    });
}

/// Applies user preferences (font, size, line wrap, gutter, tab size) to the editor UI.
pub fn apply_editor_settings(ui: &EditorUi, settings: &NotepadSettings) {
    let css = format!(
        "textview.hosts-editor-text, textview.hosts-editor-text text {{ font-family: \"{}\", monospace; font-size: {}pt; }}",
        settings.font_family, settings.font_size
    );
    ui.font_provider.load_from_data(&css);

    ui.text_view.set_wrap_mode(if settings.word_wrap {
        WrapMode::Word
    } else {
        WrapMode::None
    });

    ui.line_numbers.set_visible(settings.show_line_numbers);

    let tab_chars = settings.tab_size as i32;
    let tab_px = tab_chars.max(1) * 8;
    let mut tabs = gtk4::pango::TabArray::new(1, false);
    tabs.set_tab(0, gtk4::pango::TabAlign::Left, tab_px * gtk4::pango::SCALE);
    ui.text_view.set_tabs(&tabs);
}

/// Builds the complete UI layout for the Notepad editor window.
pub fn build_editor_layout(
    window: &ApplicationWindow,
    file_path: Option<&Path>,
    initial_text: &str,
    language: &str,
    encoding: &str,
) -> EditorUi {
    let file_name = file_path
        .and_then(|p| p.file_name())
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| trans("notepad.untitled"));

    update_window_title(window, &file_name, false);
    window.add_css_class("viewer-window");
    window.add_css_class("explore-window");
    window.set_default_size(960, 640);

    // Root vertical container: main editor area + bottom status bar
    let root_box = Box::new(Orientation::Vertical, 0);
    root_box.set_hexpand(true);
    root_box.set_vexpand(true);

    // Overlay allows floating info card and notifications on top of the text view
    let overlay = Overlay::new();
    overlay.set_hexpand(true);
    overlay.set_vexpand(true);

    // Editor content container (line numbers + scrolled text view)
    let editor_container = Box::new(Orientation::Horizontal, 0);
    editor_container.set_hexpand(true);
    editor_container.set_vexpand(true);

    // 1. Line numbers gutter (Cairo-drawn DrawingArea)
    let line_numbers = DrawingArea::new();
    line_numbers.add_css_class("viewer-drawing-area");
    line_numbers.set_size_request(48, -1);
    line_numbers.set_vexpand(true);
    editor_container.append(&line_numbers);

    // 2. Scrolled text editor
    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_hexpand(true);
    scrolled_window.set_vexpand(true);
    scrolled_window.set_policy(gtk4::PolicyType::Automatic, gtk4::PolicyType::Automatic);

    let text_buffer = TextBuffer::new(None);
    text_buffer.set_enable_undo(true);
    text_buffer.set_text(initial_text);

    let text_view = TextView::with_buffer(&text_buffer);
    text_view.add_css_class("hosts-editor-text");
    text_view.set_hexpand(true);
    text_view.set_vexpand(true);
    text_view.set_wrap_mode(WrapMode::None);
    text_view.set_monospace(true);
    text_view.set_left_margin(12);
    text_view.set_right_margin(12);
    text_view.set_top_margin(8);
    text_view.set_bottom_margin(8);

    let font_provider = CssProvider::new();
    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &font_provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION + 20,
        );
    }

    scrolled_window.set_child(Some(&text_view));
    editor_container.append(&scrolled_window);

    overlay.set_child(Some(&editor_container));

    // Top-Center Notification Toast
    let toast_box = Box::new(Orientation::Horizontal, 8);
    toast_box.add_css_class("controls-bar");

    let toast_lbl = Label::new(None);
    toast_lbl.add_css_class("info-item");
    toast_box.append(&toast_lbl);

    let toast_revealer = Revealer::new();
    toast_revealer.set_transition_type(RevealerTransitionType::SlideDown);
    toast_revealer.set_transition_duration(200);
    toast_revealer.set_halign(Align::Center);
    toast_revealer.set_valign(Align::Start);
    toast_revealer.set_margin_top(14);
    toast_revealer.set_child(Some(&toast_box));
    toast_revealer.set_reveal_child(false);
    overlay.add_overlay(&toast_revealer);

    root_box.append(&overlay);

    // 5. Bottom status bar
    let status_bar = Box::new(Orientation::Horizontal, 8);
    status_bar.add_css_class("status-bar");
    status_bar.set_valign(Align::End);

    // Modified indicator dot
    let status_modified_lbl = Label::new(Some("●"));
    status_modified_lbl.add_css_class("info-item");
    status_modified_lbl.set_visible(false);
    status_bar.append(&status_modified_lbl);

    // File name / path
    let status_file_lbl = Label::new(Some(&file_name));
    status_file_lbl.add_css_class("info-item");
    status_file_lbl.set_halign(Align::Start);
    status_file_lbl.set_ellipsize(gtk4::pango::EllipsizeMode::Middle);
    status_file_lbl.set_max_width_chars(36);
    status_bar.append(&status_file_lbl);

    // Spacer
    let spacer = Box::new(Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    status_bar.append(&spacer);

    // Cursor position (Ln X, Col Y)
    let status_cursor_lbl = Label::new(Some("Ln 1, Col 1"));
    status_cursor_lbl.add_css_class("info-item");
    status_bar.append(&status_cursor_lbl);

    let sep1 = Separator::new(Orientation::Vertical);
    sep1.add_css_class("status-bar-separator");
    status_bar.append(&sep1);

    // Line count
    let line_count = text_buffer.line_count();
    let lines_str = trans("notepad.lines_count").replace("{}", &line_count.to_string());
    let status_lines_lbl = Label::new(Some(&lines_str));
    status_lines_lbl.add_css_class("info-item");
    status_bar.append(&status_lines_lbl);

    let sep2 = Separator::new(Orientation::Vertical);
    sep2.add_css_class("status-bar-separator");
    status_bar.append(&sep2);

    // Encoding
    let status_encoding_lbl = Label::new(Some(encoding));
    status_encoding_lbl.add_css_class("info-item");
    status_bar.append(&status_encoding_lbl);

    let sep3 = Separator::new(Orientation::Vertical);
    sep3.add_css_class("status-bar-separator");
    status_bar.append(&sep3);

    // Language / Syntax
    let status_lang_lbl = Label::new(Some(language));
    status_lang_lbl.add_css_class("info-item");
    status_bar.append(&status_lang_lbl);

    // Separator and Settings button
    let sep4 = Separator::new(Orientation::Vertical);
    sep4.add_css_class("status-bar-separator");
    status_bar.append(&sep4);

    let btn_settings = babydra_ui_kit::components::create_icon_button(
        "settings",
        16,
        &["status-bar-item"],
        Some(&trans("notepad.settings")),
        || {},
    );
    status_bar.append(&btn_settings);

    root_box.append(&status_bar);
    window.set_child(Some(&root_box));

    let ui = EditorUi {
        window: window.clone(),
        text_view,
        text_buffer,
        line_numbers,
        scrolled_window,
        status_bar,
        status_modified_lbl,
        status_file_lbl,
        status_cursor_lbl,
        status_lines_lbl,
        status_encoding_lbl,
        status_lang_lbl,
        btn_settings,
        font_provider,
        toast_revealer,
        toast_lbl,
    };

    // Apply saved user settings immediately
    let initial_settings = load_notepad_cfg();
    apply_editor_settings(&ui, &initial_settings);

    ui
}
