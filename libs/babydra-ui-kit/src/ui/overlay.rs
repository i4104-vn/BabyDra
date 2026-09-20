use gtk4::prelude::*;
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

pub struct OverlayWindowComponents {
    pub window: gtk4::ApplicationWindow,
    pub overlay_box: gtk4::Box,
    pub deck_container: gtk4::Box,
    pub cards_row: gtk4::Box,
    pub scrolled: gtk4::ScrolledWindow,
    pub focus_ring: gtk4::Box,
    pub meta_title: gtk4::Label,
    pub meta_subtitle: gtk4::Label,
    pub meta_bar: gtk4::Box,
    pub switcher_wrapper: gtk4::Box,
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
    window.set_margin(Edge::Top, 0);
    window.set_margin(Edge::Bottom, 0);
    window.set_margin(Edge::Left, 0);
    window.set_margin(Edge::Right, 0);
    window.add_css_class("switcher-window");

    let overlay_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    overlay_box.set_valign(gtk4::Align::Fill);
    overlay_box.set_halign(gtk4::Align::Fill);
    overlay_box.set_hexpand(true);
    overlay_box.set_vexpand(true);

    let switcher_wrapper = gtk4::Box::new(gtk4::Orientation::Horizontal, 16);
    switcher_wrapper.add_css_class("switcher-main-row");
    switcher_wrapper.set_halign(gtk4::Align::End);
    switcher_wrapper.set_valign(gtk4::Align::Center);
    switcher_wrapper.set_hexpand(true);
    switcher_wrapper.set_vexpand(true);
    switcher_wrapper.set_margin_end(0);

    let meta_bar = gtk4::Box::new(gtk4::Orientation::Vertical, 3);
    meta_bar.add_css_class("switcher-meta-bar");
    meta_bar.set_halign(gtk4::Align::End);
    meta_bar.set_valign(gtk4::Align::Center);

    let meta_title = gtk4::Label::new(None);
    meta_title.add_css_class("switcher-meta-title");
    meta_title.set_halign(gtk4::Align::Start);
    meta_title.set_xalign(0.0);
    meta_title.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    meta_title.set_max_width_chars(32);

    let meta_subtitle = gtk4::Label::new(None);
    meta_subtitle.add_css_class("switcher-meta-subtitle");
    meta_subtitle.set_halign(gtk4::Align::Start);
    meta_subtitle.set_xalign(0.0);
    meta_subtitle.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    meta_subtitle.set_max_width_chars(36);

    meta_bar.append(&meta_title);
    meta_bar.append(&meta_subtitle);

    let deck_container = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    deck_container.add_css_class("switcher-deck-container");
    deck_container.set_valign(gtk4::Align::Center);
    deck_container.set_halign(gtk4::Align::End);
    deck_container.set_size_request(74, 326);
    deck_container.set_vexpand(false);
    deck_container.set_hexpand(false);
    deck_container.set_margin_end(0);

    // Fixed height for 5 visible items: 5 * 54px + 4 * 8px = 302px
    let scrolled = gtk4::ScrolledWindow::new();
    scrolled.add_css_class("switcher-scrolled-window");
    scrolled.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::External);
    scrolled.set_kinetic_scrolling(false);
    scrolled.set_propagate_natural_height(false);
    scrolled.set_propagate_natural_width(false);
    scrolled.set_min_content_height(302);
    scrolled.set_max_content_height(302);
    scrolled.set_size_request(54, 302);
    scrolled.set_vexpand(false);
    scrolled.set_hexpand(false);

    let cards_row = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    cards_row.add_css_class("switcher-card-deck");
    cards_row.set_halign(gtk4::Align::Center);
    cards_row.set_valign(gtk4::Align::Start);
    scrolled.set_child(Some(&cards_row));

    // Overlay to hold the scrolling list AND the fixed center focus ring
    let deck_overlay = gtk4::Overlay::new();
    deck_overlay.set_child(Some(&scrolled));

    let focus_ring = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    focus_ring.add_css_class("switcher-focus-ring");
    focus_ring.set_can_target(false);
    focus_ring.set_halign(gtk4::Align::Center);
    focus_ring.set_valign(gtk4::Align::Center);
    focus_ring.set_size_request(54, 54);
    deck_overlay.add_overlay(&focus_ring);

    deck_container.append(&deck_overlay);

    switcher_wrapper.append(&meta_bar);
    switcher_wrapper.append(&deck_container);
    overlay_box.append(&switcher_wrapper);
    window.set_child(Some(&overlay_box));

    attach_dismiss_click(&overlay_box, &switcher_wrapper, &window);

    OverlayWindowComponents {
        window,
        overlay_box,
        deck_container,
        cards_row,
        scrolled,
        focus_ring,
        meta_title,
        meta_subtitle,
        meta_bar,
        switcher_wrapper,
    }
}

pub fn attach_dismiss_click(
    overlay_box: &gtk4::Box,
    wrapper: &gtk4::Box,
    window: &gtk4::ApplicationWindow,
) {
    let click_gesture = gtk4::GestureClick::new();
    let window_hide = window.clone();
    let wrapper_ref = wrapper.clone();
    click_gesture.connect_pressed(move |gesture, _, x, y| {
        let (wrap_w, wrap_h) = (wrapper_ref.allocated_width(), wrapper_ref.allocated_height());
        let (alloc_x, alloc_y) = (
            wrapper_ref.allocation().x() as f64,
            wrapper_ref.allocation().y() as f64,
        );
        let inside = x >= alloc_x
            && x <= alloc_x + wrap_w as f64
            && y >= alloc_y
            && y <= alloc_y + wrap_h as f64;
        if !inside {
            gesture.set_state(gtk4::EventSequenceState::Claimed);
            window_hide.set_visible(false);
        }
    });
    overlay_box.add_controller(click_gesture);
}
