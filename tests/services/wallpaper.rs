//! Integration tests: wallpaper avatar cropping.
//!
//! Verifies the circular-mask pixbuf helper keeps corners transparent and
//! the center opaque. Skips when no avatar is configured on the machine.

use babydra_core::services::wallpaper::get_avatar_bytes;
use babydra_ui_kit::ui::image::crop_circle;

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
    let home = std::env::var("HOME").unwrap_or_default();
    let avatar_png = std::path::PathBuf::from(&home).join(".babydra/avatar.png");
    let orig_avatar = std::fs::read(&avatar_png).ok();

    let temp_img = std::env::temp_dir().join("babydra_test_wp_unit_test.png");
    let pix = gtk4::gdk_pixbuf::Pixbuf::new(gtk4::gdk_pixbuf::Colorspace::Rgb, true, 8, 100, 100).unwrap();
    pix.fill(0xff0000ff);
    let _ = pix.savev(&temp_img, "png", &[]);

    let prev_conf = babydra_core::config::load_babydra_config();

    let avatar_res = babydra_core::set_avatar(&temp_img);
    assert!(avatar_res.is_ok(), "Setting avatar should succeed");

    let wp_res = babydra_core::set_greeter_wp(&temp_img);
    assert!(wp_res.is_ok(), "Setting greeter wp should succeed");

    if !home.is_empty() {
        assert!(avatar_png.exists(), "avatar.png must exist");
    }

    // Both avatar and greeter wp should be retrievable as decoded bytes
    let av_bytes = babydra_core::get_avatar_bytes();
    assert!(av_bytes.is_some(), "Avatar bytes must be retrievable after set_avatar");

    let wp_bytes = babydra_core::get_greeter_wp_bytes();
    assert!(wp_bytes.is_some(), "Greeter wp bytes must be retrievable after set_greeter_wp");

    let wp_path = babydra_core::get_greeter_wp();
    assert!(wp_path.is_some(), "Greeter wp path must be retrievable after set_greeter_wp");
    assert!(wp_path.as_ref().unwrap().is_file(), "Greeter wp path must be a valid file");
    assert_eq!(wp_path.as_ref().unwrap().extension().and_then(|e| e.to_str()), Some("png"));

    let av_path = babydra_core::get_avatar_path();
    assert!(av_path.is_some(), "Avatar path must be retrievable after set_avatar");
    assert_eq!(av_path.as_ref().unwrap().extension().and_then(|e| e.to_str()), Some("png"));

    let _ = std::fs::remove_file(&temp_img);

    // Clean up test wallpaper from wallpaper dir if copied
    if !home.is_empty() {
        let test_copy = std::path::PathBuf::from(&home).join(".babydra/wallpaper/babydra_test_wp_unit_test.png");
        let _ = std::fs::remove_file(test_copy);
    }

    // Restore previous configuration and files
    babydra_core::config::save_babydra_config(&prev_conf);
    if let Some(content) = orig_avatar {
        let _ = std::fs::write(&avatar_png, content);
    } else {
        let _ = std::fs::remove_file(&avatar_png);
    }
}

#[test]
fn set_greeter_wp_rejects_video_and_gif() {
    let temp_mp4 = std::env::temp_dir().join("babydra_test_lock_wp.mp4");
    std::fs::write(&temp_mp4, b"fake video").unwrap();
    assert!(babydra_core::set_greeter_wp(&temp_mp4).is_err(), "MP4 must be rejected for lock/greeter");
    let _ = std::fs::remove_file(&temp_mp4);

    let temp_gif = std::env::temp_dir().join("babydra_test_lock_wp.gif");
    std::fs::write(&temp_gif, b"fake gif").unwrap();
    assert!(babydra_core::set_greeter_wp(&temp_gif).is_err(), "GIF must be rejected for lock/greeter");
    let _ = std::fs::remove_file(&temp_gif);
}

#[test]
fn read_image_metadata_extracts_dimensions_and_pixels() {
    let wp_path = std::path::PathBuf::from("wallpaper.png");
    if wp_path.exists() {
        let meta = babydra_core::read_image_metadata(&wp_path).expect("Should parse wallpaper.png");
        assert_eq!(meta.width, 3840);
        assert_eq!(meta.height, 2159);
        assert_eq!(meta.aspect_ratio, "16:9");
        assert!(meta.total_pixels > 8_000_000);
        assert_eq!(meta.format, "PNG");
        assert!(meta.color_space.is_some());
    }

    let jpg_path = std::path::PathBuf::from("crates/babydra-greeter/src/assets/wallpaper.jpg");
    if jpg_path.exists() {
        let meta = babydra_core::read_image_metadata(&jpg_path).expect("Should parse wallpaper.jpg");
        assert_eq!(meta.width, 1376);
        assert_eq!(meta.height, 768);
        assert_eq!(meta.aspect_ratio, "16:9");
        assert_eq!(meta.format, "JPEG");
    }
}

