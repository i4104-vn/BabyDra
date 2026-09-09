use gtk4::prelude::*;
use gtk4_layer_shell::{Edge, KeyboardMode, Layer};

pub struct OverlayWindowComponents {
    pub window: gtk4::ApplicationWindow,
    pub overlay_box: gtk4::Box,
    pub deck_container: gtk4::Box,
    pub cards_row: gtk4::Box,
    pub meta_title: gtk4::Label,
    pub meta_subtitle: gtk4::Label,
}

pub fn create_overlay_window(app: &gtk4::Application) -> OverlayWindowComponents {
    crate::ui::theme::init_theme();

    let window = gtk4::ApplicationWindow::new(app);
    crate::ui::theme::apply_theme_class(&window);
    crate::ui::window::init_layer_window(
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
    window.add_css_class("switcher-window");

    let overlay_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    overlay_box.set_valign(gtk4::Align::Center);
    overlay_box.set_halign(gtk4::Align::Center);

    let deck_container = gtk4::Box::new(gtk4::Orientation::Vertical, 10);
    deck_container.add_css_class("switcher-deck-container");
    deck_container.set_valign(gtk4::Align::Center);
    deck_container.set_halign(gtk4::Align::Center);

    let scrolled = gtk4::ScrolledWindow::new();
    scrolled.set_policy(gtk4::PolicyType::Automatic, gtk4::PolicyType::Never);
    scrolled.set_kinetic_scrolling(true);
    scrolled.set_vexpand(false);
    scrolled.set_hexpand(true);

    let cards_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    cards_row.set_halign(gtk4::Align::Center);
    scrolled.set_child(Some(&cards_row));
    deck_container.append(&scrolled);

    let meta_bar = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    meta_bar.add_css_class("switcher-meta-bar");
    meta_bar.set_halign(gtk4::Align::Center);
    meta_bar.set_valign(gtk4::Align::Center);

    let meta_title = gtk4::Label::new(None);
    meta_title.add_css_class("switcher-meta-title");
    meta_title.set_halign(gtk4::Align::Center);

    let meta_subtitle = gtk4::Label::new(None);
    meta_subtitle.add_css_class("switcher-meta-subtitle");
    meta_subtitle.set_halign(gtk4::Align::Center);
    meta_subtitle.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    meta_subtitle.set_max_width_chars(45);

    meta_bar.append(&meta_title);
    meta_bar.append(&meta_subtitle);
    deck_container.append(&meta_bar);

    overlay_box.append(&deck_container);
    window.set_child(Some(&overlay_box));

    attach_dismiss_click(&overlay_box, &deck_container, &window);

    OverlayWindowComponents {
        window,
        overlay_box,
        deck_container,
        cards_row,
        meta_title,
        meta_subtitle,
    }
}

pub fn attach_dismiss_click(
    overlay_box: &gtk4::Box,
    deck_container: &gtk4::Box,
    window: &gtk4::ApplicationWindow,
) {
    let click_gesture = gtk4::GestureClick::new();
    let window_hide = window.clone();
    let deck_ref = deck_container.clone();
    click_gesture.connect_pressed(move |gesture, _, x, y| {
        let (deck_w, deck_h) = (deck_ref.allocated_width(), deck_ref.allocated_height());
        let (alloc_x, alloc_y) = (
            deck_ref.allocation().x() as f64,
            deck_ref.allocation().y() as f64,
        );
        let inside = x >= alloc_x
            && x <= alloc_x + deck_w as f64
            && y >= alloc_y
            && y <= alloc_y + deck_h as f64;
        if !inside {
            gesture.set_state(gtk4::EventSequenceState::Claimed);
            window_hide.set_visible(false);
        }
    });
    overlay_box.add_controller(click_gesture);
}
