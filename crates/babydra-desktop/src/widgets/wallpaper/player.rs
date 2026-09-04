//! Live wallpaper playback handling for video and animated GIF files.

use gdk4::prelude::*;
use gtk4::prelude::*;
use std::cell::{Cell, RefCell};
use std::path::Path;
use std::rc::Rc;

/// Prepares and pre-rolls a live video wallpaper without starting playback.
pub fn prepare_live_video(live_path: &Path) -> Option<gtk4::MediaFile> {
    if babydra_core::wallpaper::is_video_file(live_path)
        && babydra_core::wallpaper::is_gstreamer_plugin_available()
    {
        let mf = gtk4::MediaFile::for_filename(live_path);
        mf.set_loop(true);
        mf.set_muted(true);
        Some(mf)
    } else {
        None
    }
}

/// Starts live wallpaper playback (video or GIF loop) on live_picture.
pub fn start_live_media(
    live_path: &Path,
    live_mode: &babydra_core::wallpaper::WallpaperMode,
    live_pic: &gtk4::Picture,
    da: &gtk4::DrawingArea,
    active_mf: &Rc<RefCell<Option<gtk4::MediaFile>>>,
    gif_src: &Rc<RefCell<Option<glib::SourceId>>>,
    pre_rolled_mf: Option<gtk4::MediaFile>,
) {
    if *live_mode == babydra_core::wallpaper::WallpaperMode::Live {
        if babydra_core::wallpaper::is_video_file(live_path) {
            if babydra_core::wallpaper::is_gstreamer_plugin_available() {
                let mf = pre_rolled_mf.unwrap_or_else(|| {
                    let m = gtk4::MediaFile::for_filename(live_path);
                    m.set_loop(true);
                    m.set_muted(true);
                    m
                });

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

                live_pic.set_paintable(Some(&mf));
                live_pic.set_visible(true);
                mf.play();

                // Keep da visible until the first frame is actually decoded and painted by GStreamer.
                // This completely eliminates any black screen flash during pipeline startup.
                let da_hide = da.clone();
                let first_frame_rendered = Rc::new(Cell::new(false));
                let first_frame_c = first_frame_rendered.clone();
                mf.connect_invalidate_contents(move |_| {
                    if !first_frame_c.get() {
                        first_frame_c.set(true);
                        da_hide.set_visible(false);
                    }
                });

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
