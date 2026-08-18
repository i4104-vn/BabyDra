//! Album artwork loading: bytes → scaled GTK widgets, the main-thread art
//! receiver, retry counting and fallback icons.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use gdk_pixbuf::prelude::*;
use gtk4::prelude::*;

/// (art_url, fallback_icon_name, load_result)
pub(crate) type ArtPayload = (String, String, Result<Vec<u8>, ()>);

/// Parses and scales raw image data from memory buffers to build cover art.
pub(crate) fn load_album_art_from_bytes(bytes: &[u8], size: i32) -> Option<gtk4::Widget> {
    let loader = gdk_pixbuf::PixbufLoader::new();
    loader.write(bytes).ok()?;
    loader.close().ok()?;
    let pb = loader.pixbuf()?;

    let w = pb.width();
    let h = pb.height();
    if w <= 0 || h <= 0 {
        return None;
    }

    let scale_w = size as f64 / w as f64;
    let scale_h = size as f64 / h as f64;
    let scale = scale_w.min(scale_h);

    let dest_w = (w as f64 * scale) as i32;
    let dest_h = (h as f64 * scale) as i32;

    let scaled_pb = pb.scale_simple(dest_w, dest_h, gdk_pixbuf::InterpType::Bilinear)?;

    let texture = gdk4::Texture::for_pixbuf(&scaled_pb);
    let picture = gtk4::Picture::for_paintable(&texture);
    picture.set_size_request(dest_w, dest_h);
    picture.set_content_fit(gtk4::ContentFit::Contain);
    picture.set_valign(gtk4::Align::Center);
    picture.set_halign(gtk4::Align::Center);
    Some(picture.upcast())
}

/// Spawns the main-thread task that applies fetched artwork to the notch and
/// popover art containers (with the retry/fallback logic).
pub(crate) fn spawn_art_receiver(
    mut rx: tokio::sync::mpsc::UnboundedReceiver<ArtPayload>,
    art_container: gtk4::Box,
    popover_art: gtk4::Box,
    last_attempted_url: Rc<RefCell<String>>,
    art_loaded: Rc<Cell<bool>>,
    fail_count: Rc<Cell<u32>>,
) {
    glib::MainContext::default().spawn_local(async move {
        while let Some((url, app_icon_name, result)) = rx.recv().await {
            if url != *last_attempted_url.borrow() {
                continue;
            }
            match result {
                Ok(bytes) => {
                    let small_art = load_album_art_from_bytes(&bytes, 16);
                    let large_art = load_album_art_from_bytes(&bytes, 240);
                    match (small_art, large_art) {
                        (Some(s_art), Some(l_art)) => {
                            art_loaded.set(true);
                            set_art(&art_container, &s_art, "notch-album-art");
                            set_art_expanded(&popover_art, &l_art);
                        }
                        _ => art_fail(
                            &last_attempted_url,
                            &art_loaded,
                            &fail_count,
                            &art_container,
                            &popover_art,
                            &app_icon_name,
                        ),
                    }
                }
                Err(_) => art_fail(
                    &last_attempted_url,
                    &art_loaded,
                    &fail_count,
                    &art_container,
                    &popover_art,
                    &app_icon_name,
                ),
            }
        }
    });
}

/// Replaces the child of `container` with `widget`.
fn set_art(container: &gtk4::Box, widget: &gtk4::Widget, css_class: &str) {
    if let Some(child) = container.first_child() {
        container.remove(&child);
    }
    widget.add_css_class(css_class);
    container.append(widget);
}

/// Replaces the popover art child with a stretched artwork widget.
fn set_art_expanded(container: &gtk4::Box, widget: &gtk4::Widget) {
    if let Some(child) = container.first_child() {
        container.remove(&child);
    }
    widget.add_css_class("media-popover-art");
    widget.set_hexpand(true);
    widget.set_vexpand(true);
    widget.set_halign(gtk4::Align::Center);
    widget.set_valign(gtk4::Align::Center);
    container.append(widget);
}

/// Artwork failure path: retries up to 3 times, then falls back to an icon.
fn art_fail(
    last_attempted_url: &Rc<RefCell<String>>,
    art_loaded: &Rc<Cell<bool>>,
    fail_count: &Rc<Cell<u32>>,
    art_container: &gtk4::Box,
    popover_art: &gtk4::Box,
    app_icon_name: &str,
) {
    let fails = fail_count.get() + 1;
    fail_count.set(fails);
    if fails >= 3 {
        art_loaded.set(true);
        set_art_fallback_icon(art_container, Some(popover_art), app_icon_name);
    } else {
        *last_attempted_url.borrow_mut() = String::new();
    }
}

/// Sets the album-art containers to a fallback icon (no artwork available).
pub(crate) fn set_art_fallback_icon(
    art_container: &gtk4::Box,
    popover_art: Option<&gtk4::Box>,
    icon_name: &str,
) {
    if let Some(child) = art_container.first_child() {
        art_container.remove(&child);
    }
    let music_icon_s = babydra_ui_kit::ui::icon::get_icon_colored(icon_name, 14, "#3b82f6");
    music_icon_s.add_css_class("notch-album-art");
    art_container.append(&music_icon_s);

    if let Some(popover_art) = popover_art {
        if let Some(child) = popover_art.first_child() {
            popover_art.remove(&child);
        }
        let fallback_card = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        fallback_card.add_css_class("fallback-art-box");
        fallback_card.set_size_request(200, 130);
        fallback_card.set_hexpand(true);
        fallback_card.set_halign(gtk4::Align::Center);
        fallback_card.set_valign(gtk4::Align::Center);

        let music_icon_l = babydra_ui_kit::ui::icon::get_icon_colored(icon_name, 56, "#3b82f6");
        music_icon_l.set_halign(gtk4::Align::Center);
        music_icon_l.set_valign(gtk4::Align::Center);
        music_icon_l.set_hexpand(true);
        music_icon_l.set_vexpand(true);
        fallback_card.append(&music_icon_l);
        popover_art.append(&fallback_card);
    }
}
