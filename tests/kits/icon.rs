//! Integration tests: icon resolving and zoom icon assets.

use babydra_ui_kit::ui::icon::{get_icon, get_icon_colored};
use gtk4::prelude::*;

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

    // Verify theme toggle icons resolve properly
    let brightness_icon = get_icon("brightness", 16);
    assert_eq!(brightness_icon.pixel_size(), 16);
    assert!(brightness_icon.paintable().is_some());

    let dark_mode_icon = get_icon("dark-mode", 16);
    assert_eq!(dark_mode_icon.pixel_size(), 16);
    assert!(dark_mode_icon.paintable().is_some());

    // Verify colored icon handles rgba with and without spaces
    let brightness_colored_nospaces = get_icon_colored("brightness", 16, "rgba(255,255,255,0.8)");
    assert_eq!(brightness_colored_nospaces.pixel_size(), 16);
    assert!(brightness_colored_nospaces.paintable().is_some());

    let brightness_colored_spaces = get_icon_colored("brightness", 16, "rgba(255, 255, 255, 0.8)");
    assert_eq!(brightness_colored_spaces.pixel_size(), 16);
    assert!(brightness_colored_spaces.paintable().is_some());

    // Verify create_wp_thumb resolves
    let wp_thumb = babydra_ui_kit::components::create_wp_thumb(18);
    assert!(wp_thumb.has_css_class("sidebar-wallpaper-thumb"));
}

#[test]
fn test_ui_kit_image_cropping_and_rounding() {
    let pix = gtk4::gdk_pixbuf::Pixbuf::new(gtk4::gdk_pixbuf::Colorspace::Rgb, true, 8, 100, 50).unwrap();
    pix.fill(0xffffffff);
    let bytes = pix.save_to_bufferv("png", &[]).unwrap();

    // Test crop_square
    let sq = babydra_ui_kit::ui::image::crop_square(&bytes, 32);
    assert!(sq.is_some());
    let sq_pix = sq.unwrap();
    assert_eq!(sq_pix.width(), 32);
    assert_eq!(sq_pix.height(), 32);

    // Test crop_rounded
    let rounded = babydra_ui_kit::ui::image::crop_rounded(&bytes, 18, 4.0);
    assert!(rounded.is_some());
    let r_pix = rounded.unwrap();
    assert_eq!(r_pix.width(), 18);
    assert_eq!(r_pix.height(), 18);
    assert!(r_pix.has_alpha());

    // Test crop_circle
    let circle = babydra_ui_kit::ui::image::crop_circle(&bytes, 40);
    assert!(circle.is_some());
    let c_pix = circle.unwrap();
    assert_eq!(c_pix.width(), 40);
    assert_eq!(c_pix.height(), 40);
    assert!(c_pix.has_alpha());
}
