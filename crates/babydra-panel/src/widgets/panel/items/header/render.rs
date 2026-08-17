use gtk4::prelude::*;
use std::rc::Rc;

/// Launch settings app.
fn launch_settings_app() {
    std::thread::spawn(|| {
        if std::process::Command::new("babydra-settings")
            .spawn()
            .is_ok()
        {
            return;
        }
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(parent) = exe_path.parent() {
                let local_settings = parent.join("babydra-settings");
                if local_settings.exists()
                    && std::process::Command::new(&local_settings).spawn().is_ok()
                {
                    return;
                }
            }
        }
        if let Ok(home) = std::env::var("HOME") {
            let user_bin = std::path::PathBuf::from(home).join(".local/bin/babydra-settings");
            let _ = std::process::Command::new(user_bin).spawn();
        }
    });
}

/// Creates a new `header row`.
pub fn create_header_row(_on_popover_toggled: Option<Rc<dyn Fn(bool) + 'static>>) -> gtk4::Box {
    let header_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    header_box.set_hexpand(true);

    let title = gtk4::Label::new(Some(&babydra_core::i18n::t("control.title")));
    title.add_css_class("control-center-title");
    title.set_xalign(0.0);
    title.set_hexpand(true);

    let btn_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    btn_box.set_halign(gtk4::Align::End);

    let is_dark = babydra_ui_kit::ui::theme::is_dark_mode();
    let theme_icon_name = if is_dark { "dark-mode" } else { "brightness" };
    let icon_color = if is_dark {
        "#ffffff"
    } else {
        "rgba(255,255,255,0.8)"
    };
    let theme_tooltip = babydra_core::i18n::t("control.dark_mode");

    // Theme toggle button
    let theme_btn = babydra_ui_kit::components::create_colored_icon_button(
        theme_icon_name,
        16,
        icon_color,
        &["circle-btn"],
        Some(&theme_tooltip),
        || {},
    );
    let theme_btn_clone_click = theme_btn.clone();
    theme_btn.connect_clicked(move |_| {
        let spinner = gtk4::Spinner::builder()
            .spinning(true)
            .halign(gtk4::Align::Center)
            .valign(gtk4::Align::Center)
            .build();
        theme_btn_clone_click.set_child(Some(&spinner));
        babydra_ui_kit::ui::theme::set_dark_mode(!babydra_ui_kit::ui::theme::is_dark_mode());
    });

    // Auto-update icon when theme changes
    if let Some(settings) = gtk4::Settings::default() {
        let btn_clone = theme_btn.clone();
        settings.connect_gtk_application_prefer_dark_theme_notify(move |_| {
            let dark = babydra_ui_kit::ui::theme::is_dark_mode();
            let name = if dark { "dark-mode" } else { "brightness" };
            let color = if dark {
                "#ffffff"
            } else {
                "rgba(255,255,255,0.8)"
            };
            let new_theme_icon = babydra_ui_kit::ui::icon::get_icon_colored(name, 16, color);
            btn_clone.set_child(Some(&new_theme_icon));
        });
    }

    let settings_btn = babydra_ui_kit::components::create_icon_button(
        "settings",
        16,
        &["circle-btn"],
        Some(&babydra_core::i18n::t("control.settings")),
        || {
            launch_settings_app();
        },
    );

    btn_box.append(&theme_btn);
    btn_box.append(&settings_btn);

    header_box.append(&title);
    header_box.append(&btn_box);

    header_box
}
