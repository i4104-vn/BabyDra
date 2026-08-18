/// Sets GNOME desktop color-scheme gsettings.
pub fn set_color_scheme(dark: bool) -> std::io::Result<std::process::Output> {
    let scheme = if dark { "prefer-dark" } else { "prefer-light" };
    std::process::Command::new("gsettings")
        .args(&["set", "org.gnome.desktop.interface", "color-scheme", scheme])
        .output()
}
