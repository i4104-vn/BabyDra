//! Common window configuration and aspect ratio geometry calculations.

use gdk4::Monitor;
use gtk4::prelude::*;
use gtk4::Application;
use gtk4::ApplicationWindow;

/// Calculates the greatest common divisor.
pub fn calculate_gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        calculate_gcd(b, a % b)
    }
}

/// Formats an aspect ratio as `W:H`.
pub fn format_aspect_ratio(w: u32, h: u32) -> String {
    let divisor = calculate_gcd(w, h);
    if let (Some(dw), Some(dh)) = (w.checked_div(divisor), h.checked_div(divisor)) {
        format!("{}:{}", dw, dh)
    } else {
        String::new()
    }
}

/// Calculates the initial window size matching the media aspect ratio and screen size.
pub fn calculate_initial_window_size(media_w: u32, media_h: u32) -> (i32, i32) {
    if media_w == 0 || media_h == 0 {
        return (800, 600);
    }

    let (screen_w, screen_h) = if let Some(display) = gdk4::Display::default() {
        let monitors = display.monitors();
        if let Some(monitor) = monitors
            .item(0)
            .and_then(|obj| obj.downcast::<Monitor>().ok())
        {
            let geom = monitor.geometry();
            (geom.width() as f64, geom.height() as f64)
        } else {
            (1920.0, 1080.0)
        }
    } else {
        (1920.0, 1080.0)
    };

    let max_w = (screen_w * 0.85).max(600.0);
    let max_h = (screen_h * 0.85).max(400.0);
    let min_w = 480.0;
    let min_h = 360.0;

    let media_w = media_w as f64;
    let media_h = media_h as f64;
    let aspect = media_w / media_h;

    let mut w = media_w;
    let mut h = media_h;

    if w > max_w {
        w = max_w;
        h = w / aspect;
    }
    if h > max_h {
        h = max_h;
        w = h * aspect;
    }

    if w < min_w {
        w = min_w;
        h = w / aspect;
    }
    if h < min_h {
        h = min_h;
        w = h * aspect;
    }

    if w > max_w {
        w = max_w;
        h = w / aspect;
    }
    if h > max_h {
        h = max_h;
        w = h * aspect;
    }

    (w.round().max(300.0) as i32, h.round().max(200.0) as i32)
}

/// Creates a standardized application window configured for media preview.
pub fn create_viewer_window(
    app: &Application,
    title: &str,
    media_w: u32,
    media_h: u32,
) -> (ApplicationWindow, (i32, i32)) {
    let window = ApplicationWindow::new(app);
    window.set_title(Some(title));
    window.set_icon_name(Some("babydra-preview"));

    let (win_w, win_h) = calculate_initial_window_size(media_w, media_h);
    window.set_default_size(win_w, win_h);
    window.add_css_class("viewer-window");

    (window, (win_w, win_h))
}
