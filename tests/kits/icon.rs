//! Integration tests: icon resolving and zoom icon assets.

use babydra_ui_kit::ui::icon::{get_icon, get_icon_colored};

#[test]
fn zoom_icons_resolve_without_fallback() {
    let _ = gtk4::init();

    // Verify zoom icon retrieval for dark/light themes
    let zoom_in = get_icon("zoom-in", 16);
    assert_eq!(zoom_in.pixel_size(), 16);
    assert!(zoom_in.paintable().is_some());

    let zoom_out = get_icon("zoom-out", 16);
    assert_eq!(zoom_out.pixel_size(), 16);
    assert!(zoom_out.paintable().is_some());

    let zoom_fit = get_icon("zoom-fit", 16);
    assert_eq!(zoom_fit.pixel_size(), 16);
    assert!(zoom_fit.paintable().is_some());

    let zoom_orig_sym = get_icon("zoom-original-symbolic", 16);
    assert_eq!(zoom_orig_sym.pixel_size(), 16);
    assert!(zoom_orig_sym.paintable().is_some());

    let zoom_in_sym = get_icon("zoom-in-symbolic", 16);
    assert_eq!(zoom_in_sym.pixel_size(), 16);
    assert!(zoom_in_sym.paintable().is_some());

    let zoom_out_sym = get_icon("zoom-out-symbolic", 16);
    assert_eq!(zoom_out_sym.pixel_size(), 16);
    assert!(zoom_out_sym.paintable().is_some());

    // Verify colored variant
    let zoom_in_colored = get_icon_colored("zoom-in", 16, "#3b82f6");
    assert_eq!(zoom_in_colored.pixel_size(), 16);
    assert!(zoom_in_colored.paintable().is_some());
}
