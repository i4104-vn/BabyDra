pub mod render;

use gtk4::prelude::*;
use gtk4::Widget;

pub fn create_themes_widget() -> Widget {
    let gtk_themes = babydra_common::services::system::theme::get_gtk_themes();
    let cursor_themes = babydra_common::services::system::theme::get_cursor_themes();
    let cursor_sizes = vec![16, 24, 32, 48, 64];

    let widget = render::build(&gtk_themes, &cursor_themes, &cursor_sizes);

    let gtk_themes_clone = gtk_themes.clone();
    let cursor_themes_clone = cursor_themes.clone();
    let cursor_sizes_clone = cursor_sizes.clone();

    let gtk_dropdown = widget.gtk_theme_dropdown.clone();
    let cursor_dropdown = widget.cursor_theme_dropdown.clone();
    let size_dropdown = widget.cursor_size_dropdown.clone();

    widget.apply_btn.connect_clicked(move |_| {
        let gtk_idx = gtk_dropdown.selected() as usize;
        let cursor_idx = cursor_dropdown.selected() as usize;
        let size_idx = size_dropdown.selected() as usize;

        let selected_gtk = gtk_themes_clone.get(gtk_idx).cloned().unwrap_or_else(|| "Adwaita".to_string());
        let selected_cursor = cursor_themes_clone.get(cursor_idx).cloned().unwrap_or_else(|| "Adwaita".to_string());
        let selected_size = cursor_sizes_clone.get(size_idx).cloned().unwrap_or(24);

        let _ = babydra_common::services::system::theme::apply_appearance(&selected_gtk, &selected_cursor, selected_size);
    });

    widget.container.into()
}
