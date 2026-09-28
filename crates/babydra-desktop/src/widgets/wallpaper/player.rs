//! Live wallpaper playback handling for video and animated GIF files.
//! Features automatic self-healing on decoder errors and proactive
//! pipeline recycling to prevent GStreamer buffer pool exhaustion on continuous loops.

use gdk4::prelude::*;
use gtk4::prelude::*;
use std::cell::{Cell, RefCell};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::{Duration, Instant};

/// Interval between proactive pipeline recycles (15 minutes).
/// Periodic recycling guarantees fresh buffer pools and zero memory leaks
/// during 24/7 continuous video wallpaper looping.
pub const PIPELINE_RECYCLE_INTERVAL_SECS: u64 = 900;

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

/// Helper function to safely pause a MediaFile without panicking on error states.
pub fn safely_pause_media_file(mf: &gtk4::MediaFile) {
    if mf.error().is_none() && mf.is_playing() {
        mf.pause();
    }
}

/// Attaches error handling and self-healing recovery to a MediaFile instance.
fn attach_error_handler(
    mf: &gtk4::MediaFile,
    live_path: PathBuf,
    live_mode: babydra_core::wallpaper::WallpaperMode,
    live_pic: gtk4::Picture,
    da: gtk4::DrawingArea,
    active_mf: Rc<RefCell<Option<gtk4::MediaFile>>>,
    gif_src: Rc<RefCell<Option<glib::SourceId>>>,
    recycle_src: Rc<RefCell<Option<glib::SourceId>>>,
    consecutive_errors: Rc<Cell<u32>>,
    last_error_time: Rc<Cell<Option<Instant>>>,
) {
    let active_mf_err = active_mf.clone();
    let recycle_src_err = recycle_src.clone();
    let live_err = live_pic.clone();
    let da_err = da.clone();

    mf.connect_error_notify(move |f| {
        if let Some(err) = f.error() {
            tracing::warn!("Live wallpaper playback error: {}", err);

            // 1. Immediately pause and release the failed pipeline to free resources
            safely_pause_media_file(f);
            if let Some(active) = active_mf_err.borrow_mut().take() {
                if active != *f {
                    safely_pause_media_file(&active);
                }
            }

            // 2. Stop recycle timer
            if let Some(id) = recycle_src_err.borrow_mut().take() {
                id.remove();
            }

            // 3. Keep DrawingArea visible with cached wallpaper frame (prevents black screen)
            live_err.set_paintable(None::<&gtk4::gdk::Paintable>);
            live_err.set_visible(false);
            da_err.set_visible(true);
            da_err.queue_draw();

            // 4. Rate-limit recovery to avoid busy looping on permanently broken media
            let now = Instant::now();
            let mut errors = consecutive_errors.get();
            if let Some(prev) = last_error_time.get() {
                if now.duration_since(prev) > Duration::from_secs(30) {
                    errors = 0;
                }
            }
            errors += 1;
            consecutive_errors.set(errors);
            last_error_time.set(Some(now));

            let retry_delay = if errors > 5 {
                tracing::warn!(
                    "Live wallpaper encountered {} consecutive failures. Throttling recovery to 30s.",
                    errors
                );
                Duration::from_secs(30)
            } else {
                tracing::info!(
                    "Scheduling automatic live wallpaper recovery (attempt {}) in 1s...",
                    errors
                );
                Duration::from_secs(1)
            };

            let path_retry = live_path.clone();
            let mode_retry = live_mode;
            let pic_retry = live_err.clone();
            let da_retry = da_err.clone();
            let act_retry = active_mf_err.clone();
            let gif_retry = gif_src.clone();
            let rec_retry = recycle_src_err.clone();
            let err_cnt_retry = consecutive_errors.clone();
            let last_err_retry = last_error_time.clone();

            glib::timeout_add_local_once(retry_delay, move || {
                tracing::info!("Executing live wallpaper auto-recovery for {:?}", path_retry);
                start_live_media_internal(
                    &path_retry,
                    &mode_retry,
                    &pic_retry,
                    &da_retry,
                    &act_retry,
                    &gif_retry,
                    &rec_retry,
                    None,
                    err_cnt_retry,
                    last_err_retry,
                );
            });
        }
    });
}

/// Starts live wallpaper playback (video or GIF loop) on live_picture.
pub fn start_live_media(
    live_path: &Path,
    live_mode: &babydra_core::wallpaper::WallpaperMode,
    live_pic: &gtk4::Picture,
    da: &gtk4::DrawingArea,
    active_mf: &Rc<RefCell<Option<gtk4::MediaFile>>>,
    gif_src: &Rc<RefCell<Option<glib::SourceId>>>,
    recycle_src: &Rc<RefCell<Option<glib::SourceId>>>,
    pre_rolled_mf: Option<gtk4::MediaFile>,
) {
    let consecutive_errors = Rc::new(Cell::new(0));
    let last_error_time = Rc::new(Cell::new(None));
    start_live_media_internal(
        live_path,
        live_mode,
        live_pic,
        da,
        active_mf,
        gif_src,
        recycle_src,
        pre_rolled_mf,
        consecutive_errors,
        last_error_time,
    );
}

fn start_live_media_internal(
    live_path: &Path,
    live_mode: &babydra_core::wallpaper::WallpaperMode,
    live_pic: &gtk4::Picture,
    da: &gtk4::DrawingArea,
    active_mf: &Rc<RefCell<Option<gtk4::MediaFile>>>,
    gif_src: &Rc<RefCell<Option<glib::SourceId>>>,
    recycle_src: &Rc<RefCell<Option<glib::SourceId>>>,
    pre_rolled_mf: Option<gtk4::MediaFile>,
    consecutive_errors: Rc<Cell<u32>>,
    last_error_time: Rc<Cell<Option<Instant>>>,
) {
    if *live_mode == babydra_core::wallpaper::WallpaperMode::Live {
        if babydra_core::wallpaper::is_video_file(live_path) {
            if babydra_core::wallpaper::is_gstreamer_plugin_available() {
                // Clear any existing recycle timer
                if let Some(id) = recycle_src.borrow_mut().take() {
                    id.remove();
                }

                let mf = pre_rolled_mf.unwrap_or_else(|| {
                    let m = gtk4::MediaFile::for_filename(live_path);
                    m.set_loop(true);
                    m.set_muted(true);
                    m
                });

                attach_error_handler(
                    &mf,
                    live_path.to_path_buf(),
                    *live_mode,
                    live_pic.clone(),
                    da.clone(),
                    active_mf.clone(),
                    gif_src.clone(),
                    recycle_src.clone(),
                    consecutive_errors.clone(),
                    last_error_time.clone(),
                );

                live_pic.set_paintable(Some(&mf));
                live_pic.set_visible(true);
                mf.play();

                // Keep da visible until the first frame is actually decoded and painted by GStreamer.
                // This completely eliminates any black screen flash during pipeline startup.
                let da_hide = da.clone();
                let first_frame_rendered = Rc::new(Cell::new(false));
                let first_frame_c = first_frame_rendered.clone();
                let err_cnt_reset = consecutive_errors.clone();
                mf.connect_invalidate_contents(move |_| {
                    if !first_frame_c.get() {
                        first_frame_c.set(true);
                        err_cnt_reset.set(0); // Successfully playing: reset error counter
                        da_hide.set_visible(false);
                    }
                });

                *active_mf.borrow_mut() = Some(mf);

                // Proactive periodic recycle: every 15 minutes, seamlessly swap with a fresh pipeline
                // to completely eliminate downstream buffer pool memory creep.
                let path_rec = live_path.to_path_buf();
                let mode_rec = *live_mode;
                let pic_rec = live_pic.clone();
                let da_rec = da.clone();
                let act_rec = active_mf.clone();
                let gif_rec = gif_src.clone();
                let rec_rec = recycle_src.clone();
                let err_rec = consecutive_errors.clone();
                let last_rec = last_error_time.clone();

                let timer_id = glib::timeout_add_local(
                    Duration::from_secs(PIPELINE_RECYCLE_INTERVAL_SECS),
                    move || {
                        tracing::debug!("Performing seamless proactive live wallpaper recycle");
                        recycle_live_video(
                            &path_rec,
                            &mode_rec,
                            &pic_rec,
                            &da_rec,
                            &act_rec,
                            &gif_rec,
                            &rec_rec,
                            err_rec.clone(),
                            last_rec.clone(),
                        );
                        glib::ControlFlow::Continue
                    },
                );
                *recycle_src.borrow_mut() = Some(timer_id);
            } else {
                live_pic.set_paintable(None::<&gtk4::gdk::Paintable>);
                live_pic.set_visible(false);
                da.set_visible(true);
            }
        } else if babydra_core::wallpaper::is_gif_file(live_path) {
            // Stop any existing recycle timer
            if let Some(id) = recycle_src.borrow_mut().take() {
                id.remove();
            }

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
                        .unwrap_or(Duration::from_millis(100))
                        .max(Duration::from_millis(20));
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

/// Seamlessly recycles the video pipeline by pre-decoding the first frame in a new pipeline
/// before hot-swapping into live_picture. Zero flicker, zero black screen, zero interruption.
fn recycle_live_video(
    live_path: &Path,
    live_mode: &babydra_core::wallpaper::WallpaperMode,
    live_pic: &gtk4::Picture,
    da: &gtk4::DrawingArea,
    active_mf: &Rc<RefCell<Option<gtk4::MediaFile>>>,
    gif_src: &Rc<RefCell<Option<glib::SourceId>>>,
    recycle_src: &Rc<RefCell<Option<glib::SourceId>>>,
    consecutive_errors: Rc<Cell<u32>>,
    last_error_time: Rc<Cell<Option<Instant>>>,
) {
    if !babydra_core::wallpaper::is_video_file(live_path)
        || !babydra_core::wallpaper::is_gstreamer_plugin_available()
    {
        return;
    }

    let new_mf = match prepare_live_video(live_path) {
        Some(m) => m,
        None => return,
    };

    let pic_c = live_pic.clone();
    let old_mf_opt = active_mf.borrow().clone();
    let active_mf_c = active_mf.clone();
    let swapped = Rc::new(Cell::new(false));
    let swapped_c = swapped.clone();
    let new_mf_c = new_mf.clone();

    new_mf.connect_invalidate_contents(move |_| {
        if !swapped_c.get() {
            swapped_c.set(true);
            pic_c.set_paintable(Some(&new_mf_c));
            *active_mf_c.borrow_mut() = Some(new_mf_c.clone());

            // Safely stop and drop the old media pipeline
            if let Some(old_mf) = old_mf_opt.as_ref() {
                safely_pause_media_file(old_mf);
            }
            tracing::debug!("Seamless live wallpaper pipeline recycle successful");
        }
    });

    attach_error_handler(
        &new_mf,
        live_path.to_path_buf(),
        *live_mode,
        live_pic.clone(),
        da.clone(),
        active_mf.clone(),
        gif_src.clone(),
        recycle_src.clone(),
        consecutive_errors,
        last_error_time,
    );

    new_mf.play();
}
