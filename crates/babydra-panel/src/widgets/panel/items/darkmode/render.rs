use gtk4::prelude::*;

pub fn create_small_theme_toggle_tile() -> gtk4::Button {
    let is_dark_init = gtk4::Settings::default()
        .map(|s| s.is_gtk_application_prefer_dark_theme())
        .unwrap_or(true);

    babydra_utils::components::create_square_toggle_tile(
        "dark-mode",
        &babydra_common::i18n::t("control.dark_mode"),
        is_dark_init,
        |new_dark| {
            babydra_utils::ui::theme::set_dark_mode(new_dark);
        }
    )
}
