/// Sets GNOME desktop color-scheme gsettings and synchronizes labwc titlebar and kitty theme.
pub fn set_color_scheme(dark: bool) -> std::io::Result<std::process::Output> {
    let _ = crate::services::system::theme::sync_labwc_titlebar_theme(dark);
    let _ = crate::services::system::theme::sync_kitty_theme(dark);
    let scheme = if dark { "prefer-dark" } else { "prefer-light" };
    std::process::Command::new("gsettings")
        .args(&["set", "org.gnome.desktop.interface", "color-scheme", scheme])
        .output()
}
