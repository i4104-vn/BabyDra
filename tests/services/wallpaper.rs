//! Integration tests: wallpaper avatar cropping.
//!
//! Verifies the circular-mask pixbuf helper keeps corners transparent and
//! the center opaque. Skips when no avatar is configured on the machine.

use babydra_core::services::wallpaper::{crop_circle, get_avatar_bytes};

fn read_alpha(pixbuf: &gtk4::gdk_pixbuf::Pixbuf, x: i32, y: i32) -> u8 {
    let rowstride = pixbuf.rowstride() as usize;
    let bytes = pixbuf.pixel_bytes().unwrap();
    bytes.as_ref()[y as usize * rowstride + x as usize * 4 + 3]
}

#[test]
fn circular_mask_makes_corners_transparent() {
    let Some(bytes) = get_avatar_bytes() else {
        eprintln!("SKIP: no avatar configured");
        return;
    };
    let Some(pixbuf) = crop_circle(&bytes, 80) else {
        panic!("failed to build circular pixbuf");
    };
    assert_eq!(pixbuf.width(), 80);
    assert_eq!(pixbuf.height(), 80);
    assert!(pixbuf.has_alpha());

    // Corners must be fully transparent (alpha == 0).
    for (x, y) in [(0, 0), (79, 0), (0, 79), (79, 79)] {
        assert_eq!(
            read_alpha(&pixbuf, x, y),
            0,
            "corner ({}, {}) should be transparent",
            x,
            y
        );
    }
    // Center must be fully opaque.
    assert_eq!(read_alpha(&pixbuf, 40, 40), 255);

    // Save a copy for visual inspection.
    let _ = pixbuf.savev("/tmp/avatar_circle_test.png", "png", &[]);
    eprintln!("saved /tmp/avatar_circle_test.png");
}

#[test]
fn set_greeter_wp_and_avatar_persist_cleanly() {
    let temp_img = std::env::temp_dir().join("babydra_test_wp.png");
    let pix = gtk4::gdk_pixbuf::Pixbuf::new(gtk4::gdk_pixbuf::Colorspace::Rgb, true, 8, 100, 100).unwrap();
    pix.fill(0xffffffff);
    let _ = pix.savev(&temp_img, "png", &[]);

    let avatar_res = babydra_core::set_avatar(&temp_img);
    assert!(avatar_res.is_ok(), "Setting avatar should succeed");

    let wp_res = babydra_core::set_greeter_wp(&temp_img);
    assert!(wp_res.is_ok(), "Setting greeter wp should succeed");

    // Both avatar and greeter wp should be retrievable
    let av_bytes = babydra_core::get_avatar_bytes();
    assert!(av_bytes.is_some(), "Avatar bytes must be retrievable after set_avatar");

    let wp_bytes = babydra_core::get_greeter_wp_bytes();
    assert!(wp_bytes.is_some(), "Greeter wp bytes must be retrievable after set_greeter_wp");

    let _ = std::fs::remove_file(&temp_img);
}
