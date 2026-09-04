//! Greeter window construction and overlay layout assembly.
//! Follows the `render.rs` convention used by babydra-panel / babydra-lock:
//! builds the window once and returns widget handles for signal wiring.

use babydra_core::i18n::trans;
use gtk4::prelude::*;
use gtk4::{Align, ApplicationWindow, Box as GtkBox, ContentFit, Orientation, Overlay};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer};
use tracing::info;

use crate::theme;
use crate::widgets;
use crate::widgets::login::LoginWidget;
use crate::widgets::splash::SplashWidget;
use crate::widgets::top_bar::TopBarWidget;

/// Handles to the greeter window and its top-level widget groups.
/// Passed to `handler::setup_handlers` so signal wiring stays out of the layout builder.
pub struct GreeterWidgets {
    pub window: ApplicationWindow,
    pub top_bar: TopBarWidget,
    pub splash: SplashWidget,
    pub login: LoginWidget,
}

/// Builds the greeter window layout with top bar, splash, and login widgets.
pub fn build_greeter_ui(app: &gtk4::Application) -> GreeterWidgets {
    info!(target: "babydra-greeter", "Building GTK Application Window UI");

    let window = ApplicationWindow::builder()
        .application(app)
        .title(&trans("common.app_greeter_title"))
        .decorated(false)
        .build();

    // Ensure window decorations are disabled completely
    window.set_decorated(false);

    // Layer shell: fullscreen overlay with exclusive keyboard on Wayland compositors
    info!(target: "babydra-greeter", "Configuring GTK Layer Shell (Overlay Layer, Exclusive Keyboard Mode)");
    babydra_ui_kit::ui::window::init_layer_window(
        &window,
        Layer::Overlay,
        KeyboardMode::Exclusive,
        -1,
        &[
            (Edge::Top, true),
            (Edge::Bottom, true),
            (Edge::Left, true),
            (Edge::Right, true),
        ],
        0,
        None,
    );

    // Fallback fullscreen
    info!(target: "babydra-greeter", "Applying fullscreen layout mode");
    window.fullscreen();

    info!(target: "babydra-greeter", "Triggering CSS theme loading");
    theme::load_css();

    // Background wallpaper embedded directly from build
    const GREETER_WALLPAPER_BYTES: &[u8] = include_bytes!("assets/wallpaper.jpg");
    let bg_picture = gtk4::Picture::new();
    let gbytes = gtk4::glib::Bytes::from_static(GREETER_WALLPAPER_BYTES);
    if let Ok(texture) = gtk4::gdk::Texture::from_bytes(&gbytes) {
        bg_picture.set_paintable(Some(&texture));
        info!(target: "babydra-greeter", "Asset loaded: embedded greeter wallpaper texture");
    } else {
        let stream = gtk4::gio::MemoryInputStream::from_bytes(&gbytes);
        if let Ok(pixbuf) =
            gtk4::gdk_pixbuf::Pixbuf::from_stream(&stream, gtk4::gio::Cancellable::NONE)
        {
            bg_picture.set_pixbuf(Some(&pixbuf));
            info!(target: "babydra-greeter", "Asset loaded: embedded greeter wallpaper pixbuf");
        } else {
            info!(target: "babydra-greeter", "Asset warning: failed to decode embedded greeter wallpaper");
        }
    }

    let overlay = Overlay::new();

    bg_picture.set_can_shrink(true);
    bg_picture.set_content_fit(ContentFit::Cover);
    bg_picture.set_hexpand(true);
    bg_picture.set_vexpand(true);
    overlay.set_child(Some(&bg_picture));

    // Dark tint overlay over wallpaper
    let tint = GtkBox::new(Orientation::Vertical, 0);
    tint.add_css_class("greeter-tint");
    overlay.add_overlay(&tint);

    info!(target: "babydra-greeter", "Building main layout overlay container");
    let top_bar = widgets::top_bar::build();
    let splash = widgets::splash::build();
    let login = widgets::login::build();

    overlay.add_overlay(&top_bar.container);

    // Center main container for splash and login
    let center_box = GtkBox::new(Orientation::Vertical, 0);
    center_box.set_valign(Align::Center);
    center_box.set_halign(Align::Center);
    center_box.append(&splash.container);
    center_box.append(&login.container);
    overlay.add_overlay(&center_box);

    window.set_child(Some(&overlay));

    GreeterWidgets {
        window,
        top_bar,
        splash,
        login,
    }
}
