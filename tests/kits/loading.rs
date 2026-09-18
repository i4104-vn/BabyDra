//! Integration tests for the custom loading component and loading GIF.

use babydra_ui_kit::components::{
    create_loading_card, create_loading_icon, create_loading_placeholder_row, create_placeholder,
    PlaceholderState, LOADING_GIF_BYTES,
};
use gtk4::gdk_pixbuf::prelude::*;
use gtk4::prelude::*;

#[test]
fn test_custom_loading_components_and_gif() {
    let _ = gtk4::init();

    // 1. Verify GIF bytes
    assert!(!LOADING_GIF_BYTES.is_empty(), "LOADING_GIF_BYTES must not be empty");
    assert!(
        LOADING_GIF_BYTES.starts_with(b"GIF89a") || LOADING_GIF_BYTES.starts_with(b"GIF87a"),
        "Must be a valid GIF header"
    );

    let stream = gtk4::gio::MemoryInputStream::from_bytes(&gtk4::glib::Bytes::from_static(LOADING_GIF_BYTES));
    let anim_res = gtk4::gdk_pixbuf::PixbufAnimation::from_stream(&stream, gtk4::gio::Cancellable::NONE);
    assert!(anim_res.is_ok(), "Loading GIF should parse cleanly via PixbufAnimation");

    let anim = anim_res.unwrap();
    assert!(!anim.is_static_image(), "Loading GIF must be an animated multi-frame sequence");
    assert_eq!(anim.width(), 256);
    assert_eq!(anim.height(), 256);

    // 2. Verify loading icon widget
    let icon = create_loading_icon(64);
    assert_eq!(icon.width_request(), 64);
    assert_eq!(icon.height_request(), 64);
    assert!(icon.has_css_class("loading-icon"));

    // 3. Verify loading card full width & height
    let card = create_loading_card(72);
    assert!(card.has_css_class("settings-card"));
    assert!(card.has_css_class("loading-card"));
    assert!(card.hexpands());
    assert!(card.vexpands());
    assert_eq!(card.halign(), gtk4::Align::Fill);
    assert_eq!(card.valign(), gtk4::Align::Fill);

    let child = card.first_child();
    assert!(child.is_some());
    let child_box = child.unwrap();
    assert_eq!(child_box.halign(), gtk4::Align::Center);
    assert_eq!(child_box.valign(), gtk4::Align::Center);

    // 4. Verify loading placeholder row
    let row = create_loading_placeholder_row(48);
    assert!(row.has_css_class("settings-card-row"));
    assert!(row.has_css_class("loading-card-row"));
    assert!(row.hexpands());
    assert!(row.vexpands());
    assert_eq!(row.halign(), gtk4::Align::Fill);
    assert_eq!(row.valign(), gtk4::Align::Fill);

    let placeholder_row = create_placeholder(PlaceholderState::Loading);
    assert!(placeholder_row.has_css_class("settings-card-row"));
    assert!(placeholder_row.vexpands());
}
