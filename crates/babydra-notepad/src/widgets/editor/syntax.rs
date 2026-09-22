//! GTK4 TextBuffer syntax highlighting bridge powered by `babydra_core::services::syntax`.

#[allow(unused_imports)]
pub use babydra_core::services::syntax::{
    create_highlighter, detect_syntax, detect_syntax_by_extension, detect_syntax_by_name,
    get_theme, get_theme_by_name, language_name, list_available_themes, list_supported_languages,
    syntax_set, theme_set, Color, FontStyle, HighlightLines, LinesWithEndings, SyntaxReference,
    SyntaxSet,
};
use gtk4::prelude::*;

/// Retrieves or creates a cached `TextTag` for a specific color and style combination.
fn get_or_create_tag(
    tag_table: &gtk4::TextTagTable,
    color: Color,
    font_style: FontStyle,
) -> gtk4::TextTag {
    let style_flags = (if font_style.contains(FontStyle::BOLD) {
        1
    } else {
        0
    }) | (if font_style.contains(FontStyle::ITALIC) {
        2
    } else {
        0
    }) | (if font_style.contains(FontStyle::UNDERLINE) {
        4
    } else {
        0
    });

    let tag_name = format!(
        "syn_{:02x}{:02x}{:02x}_{:02x}_{}",
        color.r, color.g, color.b, color.a, style_flags
    );

    if let Some(tag) = tag_table.lookup(&tag_name) {
        return tag;
    }

    let tag = gtk4::TextTag::new(Some(&tag_name));
    let rgba = gtk4::gdk::RGBA::new(
        color.r as f32 / 255.0,
        color.g as f32 / 255.0,
        color.b as f32 / 255.0,
        color.a as f32 / 255.0,
    );
    tag.set_foreground_rgba(Some(&rgba));

    if font_style.contains(FontStyle::BOLD) {
        tag.set_weight(700);
    }
    if font_style.contains(FontStyle::ITALIC) {
        tag.set_style(gtk4::pango::Style::Italic);
    }
    if font_style.contains(FontStyle::UNDERLINE) {
        tag.set_underline(gtk4::pango::Underline::Single);
    }

    tag_table.add(&tag);
    tag
}

/// Applies syntax highlighting tags across the entire buffer using default theme for appearance.
#[allow(dead_code)]
pub fn apply_highlighting(buffer: &gtk4::TextBuffer, syntax: &SyntaxReference, is_dark: bool) {
    let theme = get_theme(is_dark);
    apply_highlighting_with_theme(buffer, syntax, theme);
}

/// Applies syntax highlighting tags across the buffer using custom dark/light theme preferences.
pub fn apply_highlighting_for_mode(
    buffer: &gtk4::TextBuffer,
    syntax: &SyntaxReference,
    is_dark: bool,
    dark_theme: &str,
    light_theme: &str,
) {
    let theme_name = if is_dark { dark_theme } else { light_theme };
    let theme = get_theme_by_name(theme_name).unwrap_or_else(|| get_theme(is_dark));
    apply_highlighting_with_theme(buffer, syntax, theme);
}

/// Applies syntax highlighting tags across the entire buffer using a specific theme.
pub fn apply_highlighting_with_theme(
    buffer: &gtk4::TextBuffer,
    syntax: &SyntaxReference,
    theme: &syntect::highlighting::Theme,
) {
    let ps = syntax_set();
    let mut highlighter = HighlightLines::new(syntax, theme);

    let tag_table = buffer.tag_table();
    let start_all = buffer.start_iter();
    let end_all = buffer.end_iter();

    buffer.remove_all_tags(&start_all, &end_all);

    let full_text = buffer.text(&start_all, &end_all, true);
    let mut curr_iter = buffer.start_iter();

    for line in LinesWithEndings::from(full_text.as_str()) {
        if let Ok(ranges) = highlighter.highlight_line(line, ps) {
            for (style, piece) in ranges {
                let char_count = piece.chars().count() as i32;
                if char_count > 0 {
                    let mut piece_end = curr_iter;
                    piece_end.forward_chars(char_count);

                    if style.foreground.a > 0 {
                        let tag = get_or_create_tag(&tag_table, style.foreground, style.font_style);
                        buffer.apply_tag(&tag, &curr_iter, &piece_end);
                    }
                    curr_iter = piece_end;
                }
            }
        } else {
            let char_count = line.chars().count() as i32;
            curr_iter.forward_chars(char_count);
        }
    }
}
