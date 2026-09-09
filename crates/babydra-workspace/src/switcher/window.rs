use gtk4::prelude::*;
use gtk4_layer_shell::{Edge, KeyboardMode, Layer};

pub struct SwitcherWindowComponents {
    pub window: gtk4::ApplicationWindow,
    pub overlay_box: gtk4::Box,
    pub deck_container: gtk4::Box,
    pub cards_row: gtk4::Box,
}

/// Initializes the layer-shell window and base layout hierarchy.
pub fn create_switcher_window(app: &gtk4::Application) -> SwitcherWindowComponents {
    babydra_ui_kit::ui::theme::init_theme();

    let window = gtk4::ApplicationWindow::new(app);
    babydra_ui_kit::ui::theme::apply_theme_class(&window);
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
    window.add_css_class("switcher-window");
    window.add_css_class("workspace-switcher-window");

    // Fullscreen centered backdrop container
    let overlay_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    overlay_box.set_valign(gtk4::Align::Center);
    overlay_box.set_halign(gtk4::Align::Center);

    // Centered floating deck container
    let deck_container = gtk4::Box::new(gtk4::Orientation::Vertical, 16);
    deck_container.add_css_class("switcher-deck-container");
    deck_container.add_css_class("workspace-deck-container");
    deck_container.set_valign(gtk4::Align::Center);
    deck_container.set_halign(gtk4::Align::Center);

    // Header label
    let header_lbl = gtk4::Label::new(Some("Workspaces"));
    header_lbl.add_css_class("switcher-deck-header");
    deck_container.append(&header_lbl);

    // Cards row (horizontal card deck)
    let cards_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 16);
    cards_row.add_css_class("switcher-card-deck");
    cards_row.set_halign(gtk4::Align::Center);
    deck_container.append(&cards_row);

    // Hint label footer
    let hint_lbl = gtk4::Label::new(Some("Scroll or use Arrow keys to navigate • Enter to select"));
    hint_lbl.add_css_class("switcher-deck-hint");
    deck_container.append(&hint_lbl);

    overlay_box.append(&deck_container);
    window.set_child(Some(&overlay_box));

    SwitcherWindowComponents {
        window,
        overlay_box,
        deck_container,
        cards_row,
    }
}
