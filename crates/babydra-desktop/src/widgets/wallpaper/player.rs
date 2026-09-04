//! Live wallpaper playback handling for video and animated GIF files.

use gtk4::prelude::*;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

/// Starts live wallpaper playback (video or GIF loop) on live_picture.
pub fn start_live_media(
    live_path: &Path,
    live_mode: &babydra_core::wallpaper::WallpaperMode,
    live_pic: &gtk4::Picture,
    da: &gtk4::DrawingArea,
    active_mf: &Rc<RefCell<Option<gtk4::MediaFile>>>,
    gif_src: &Rc<RefCell<Option<glib::SourceId>>>,
) {
    if *live_mode == babydra_core::wallpaper::WallpaperMode::Live {
        if babydra_core::wallpaper::is_video_file(live_path) {
            if babydra_core::wallpaper::is_gstreamer_plugin_available() {
                let mf = gtk4::MediaFile::for_filename(live_path);
                mf.set_loop(true);
                mf.set_muted(true);
                let live_err = live_pic.clone();
                let da_err = da.clone();
                mf.connect_error_notify(move |f| {
                    if let Some(err) = f.error() {
                        tracing::warn!("Live wallpaper playback error: {}", err);
                        live_err.set_paintable(None::<&gtk4::gdk::Paintable>);
                        live_err.set_visible(false);
                        da_err.set_visible(true);
                    }
                });
                mf.play();
                live_pic.set_paintable(Some(&mf));
                live_pic.set_visible(true);
                da.set_visible(false);
                *active_mf.borrow_mut() = Some(mf);
            } else {
                live_pic.set_paintable(None::<&gtk4::gdk::Paintable>);
                live_pic.set_visible(false);
                da.set_visible(true);
            }
        } else if babydra_core::wallpaper::is_gif_file(live_path) {
            if let Ok(anim) = gdk_pixbuf::PixbufAnimation::from_file(live_path) {
                if anim.is_static_image() {
                    let file = gtk4::gio::File::for_path(live_path);
                    if let Ok(texture) = gtk4::gdk::Texture::from_file(&file) {
                        live_pic.set_paintable(Some(&texture));
                        live_pic.set_visible(true);
                        da.set_visible(false);
                    }
                } else {
                    let iter = anim.iter(None);
                    let pixbuf = iter.pixbuf();
                    let texture = gtk4::gdk::Texture::for_pixbuf(&pixbuf);
                    live_pic.set_paintable(Some(&texture));
                    live_pic.set_visible(true);
                    da.set_visible(false);

                    let pic_inner = live_pic.clone();
                    let delay = iter
                        .delay_time()
                        .unwrap_or(std::time::Duration::from_millis(100))
                        .max(std::time::Duration::from_millis(20));
                    let s_id = glib::timeout_add_local(delay, move || {
                        iter.advance(std::time::SystemTime::now());
                        let pb = iter.pixbuf();
                        let tex = gtk4::gdk::Texture::for_pixbuf(&pb);
                        pic_inner.set_paintable(Some(&tex));
                        glib::ControlFlow::Continue
                    });
                    *gif_src.borrow_mut() = Some(s_id);
                }
            }
        }
    }
}
