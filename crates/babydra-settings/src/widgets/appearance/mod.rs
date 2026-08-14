//! Appearance and themes personalization panel.

mod handlers;
mod render;

pub fn create_appearance_widget() -> gtk4::Widget {
    let gtk_themes = babydra_common::services::system::theme::get_gtk_themes();
    let icon_themes = babydra_common::services::system::theme::get_icon_themes();
    let cursor_themes = babydra_common::services::system::theme::get_cursor_themes();
    let cursor_sizes = vec![16, 24, 32, 48, 64];

    let wp_path = babydra_common::get_current_wallpaper()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();
    let is_dark = babydra_utils::ui::theme::is_dark_mode();

    let (
        main_box,
        preview_pic,
        pick_btn,
        theme_toggle_btn,
        gtk_dropdown,
        icon_dropdown,
        cursor_dropdown,
        size_dropdown,
        target_dropdown,
        quick_select_box,
        avatar_pic,
        avatar_btn,
    ) = render::build_appearance_ui(
        &wp_path,
        "",
        is_dark,
        &gtk_themes,
        &icon_themes,
        &cursor_themes,
        &cursor_sizes,
    );

    handlers::setup_appearance_handlers(
        &main_box,
        &preview_pic,
        &pick_btn,
        &theme_toggle_btn,
        &gtk_dropdown,
        &icon_dropdown,
        &cursor_dropdown,
        &size_dropdown,
        &target_dropdown,
        &quick_select_box,
        &avatar_pic,
        &avatar_btn,
        gtk_themes,
        icon_themes,
        cursor_themes,
        cursor_sizes,
    );

    main_box.into()
}
