//! Custom loading indicator and card components for BabyDra.
//!
//! Provides an animated loading widget based on `loading.gif` and unified
//! loading cards that occupy the full width and height of pending containers.

use gtk4::gdk_pixbuf::prelude::*;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Embedded bytes of the BabyDra animated loading GIF.
pub const LOADING_GIF_BYTES: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/ui/loading.gif"));

/// Creates an animated loading icon widget using the embedded BabyDra GIF.
///
/// Automatically manages timer lifecycle: advances frames smoothly at the GIF's
/// native rate, pauses when the widget is unmapped, and cleans up when destroyed.
pub fn create_loading_icon(size: i32) -> gtk4::Widget {
    let bytes = glib::Bytes::from_static(LOADING_GIF_BYTES);
    let stream = gio::MemoryInputStream::from_bytes(&bytes);

    let anim_opt = gtk4::gdk_pixbuf::PixbufAnimation::from_stream(&stream, gio::Cancellable::NONE).ok();

    let Some(anim) = anim_opt else {
        let spinner = gtk4::Spinner::new();
        spinner.set_size_request(size, size);
        spinner.set_halign(gtk4::Align::Center);
        spinner.set_valign(gtk4::Align::Center);
        spinner.start();
        return spinner.upcast();
    };

    if anim.is_static_image() {
        let iter = anim.iter(None);
        let pb = iter.pixbuf();
        let texture = gtk4::gdk::Texture::for_pixbuf(&pb);
        let pic = gtk4::Picture::for_paintable(&texture);
        pic.set_size_request(size, size);
        pic.set_can_shrink(true);
        pic.set_halign(gtk4::Align::Center);
        pic.set_valign(gtk4::Align::Center);
        return pic.upcast();
    }

    let pic = gtk4::Picture::new();
    pic.set_size_request(size, size);
    pic.set_can_shrink(true);
    pic.set_halign(gtk4::Align::Center);
    pic.set_valign(gtk4::Align::Center);
    pic.add_css_class("loading-icon");

    let anim_rc = Rc::new(anim);
    let active_source: Rc<RefCell<Option<glib::SourceId>>> = Rc::new(RefCell::new(None));

    // Paint initial frame
    let initial_iter = anim_rc.iter(None);
    let initial_pb = initial_iter.pixbuf();
    let initial_tex = gtk4::gdk::Texture::for_pixbuf(&initial_pb);
    pic.set_paintable(Some(&initial_tex));

    let start_animation = {
        let pic = pic.clone();
        let anim_rc = anim_rc.clone();
        let active_source = active_source.clone();

        Rc::new(move || {
            if active_source.borrow().is_some() {
                return;
            }

            let iter = anim_rc.iter(None);
            let pic_c = pic.clone();
            let delay = iter
                .delay_time()
                .unwrap_or(std::time::Duration::from_millis(33))
                .max(std::time::Duration::from_millis(20));

            let source_holder = active_source.clone();
            let s_id = glib::timeout_add_local(delay, move || {
                iter.advance(std::time::SystemTime::now());
                let pb = iter.pixbuf();
                let tex = gtk4::gdk::Texture::for_pixbuf(&pb);
                pic_c.set_paintable(Some(&tex));
                glib::ControlFlow::Continue
            });

            *source_holder.borrow_mut() = Some(s_id);
        })
    };

    let stop_animation = {
        let active_source = active_source.clone();
        Rc::new(move || {
            if let Some(s_id) = active_source.borrow_mut().take() {
                s_id.remove();
            }
        })
    };

    // Start immediately if already mapped or when mapped
    let start_c = start_animation.clone();
    let stop_c = stop_animation.clone();

    pic.connect_map(move |_| {
        start_c();
    });

    pic.connect_unmap(move |_| {
        stop_c();
    });

    // Cleanup when destroyed
    let stop_destroy = stop_animation.clone();
    pic.connect_destroy(move |_| {
        stop_destroy();
    });

    // Initial start
    start_animation();

    pic.upcast()
}

/// Creates a loading card that occupies the entire length and width of the
/// container waiting to load, with the animated loading icon positioned in the center.
pub fn create_loading_card(icon_size: i32) -> gtk4::Box {
    let card = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    card.add_css_class("settings-card");
    card.add_css_class("loading-card");
    card.set_hexpand(true);
    card.set_vexpand(true);
    card.set_halign(gtk4::Align::Fill);
    card.set_valign(gtk4::Align::Fill);

    let center_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    center_box.set_hexpand(true);
    center_box.set_vexpand(true);
    center_box.set_halign(gtk4::Align::Center);
    center_box.set_valign(gtk4::Align::Center);

    let icon = create_loading_icon(icon_size);
    center_box.append(&icon);

    card.append(&center_box);
    card
}

/// Creates a ListBoxRow placeholder loading card that occupies the full row area.
pub fn create_loading_placeholder_row(icon_size: i32) -> gtk4::ListBoxRow {
    let row = gtk4::ListBoxRow::new();
    row.add_css_class("settings-card-row");
    row.add_css_class("loading-card-row");
    row.set_selectable(false);
    row.set_activatable(false);
    row.set_hexpand(true);
    row.set_vexpand(true);
    row.set_valign(gtk4::Align::Fill);
    row.set_halign(gtk4::Align::Fill);

    let loading_card = create_loading_card(icon_size);
    row.set_child(Some(&loading_card));
    row
}
