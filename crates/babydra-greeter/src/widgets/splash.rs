use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Label, Orientation, Spinner};

pub struct SplashWidget {
    pub container: GtkBox,
}

/// Build.
pub fn build() -> SplashWidget {
    tracing::info!(target: "babydra-greeter", "Building SplashWidget container and initial loading spinner");
    let splash_container = GtkBox::new(Orientation::Vertical, 16);
    splash_container.set_valign(Align::Center);
    splash_container.set_halign(Align::Center);
    splash_container.add_css_class("splash-box");

    let splash_content = GtkBox::new(Orientation::Vertical, 12);
    splash_content.set_halign(Align::Center);

    let logo_wrapper = GtkBox::new(Orientation::Vertical, 0);
    logo_wrapper.add_css_class("splash-logo-wrapper");
    logo_wrapper.set_halign(Align::Center);

    let logo_splash = babydra_ui_kit::ui::icon::get_logo_png(110);
    logo_wrapper.append(&logo_splash);

    let splash_title = Label::new(Some(&babydra_core::i18n::t("greeter.os_name")));
    splash_title.add_css_class("splash-title");

    let splash_subtitle = Label::new(Some(&babydra_core::i18n::t("greeter.initializing")));
    splash_subtitle.add_css_class("splash-subtitle");

    let spinner = Spinner::new();
    spinner.add_css_class("splash-spinner");
    spinner.set_size_request(32, 32);
    spinner.start();

    splash_content.append(&logo_wrapper);
    splash_content.append(&splash_title);
    splash_content.append(&splash_subtitle);
    splash_content.append(&spinner);
    splash_container.append(&splash_content);

    SplashWidget {
        container: splash_container,
    }
}
