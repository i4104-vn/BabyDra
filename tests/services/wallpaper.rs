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
