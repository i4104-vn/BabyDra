//! Video playback rate and GStreamer pipeline speed control.

use babydra_core::services::preview::set_media_file_speed_raw;
use glib::translate::ToGlibPtr;
use gtk4::MediaFile;
use std::ffi::c_void;

/// Sets the playback speed rate on a GTK4 MediaFile by reaching into its GstPlay pipeline.
pub fn set_playback_speed(media_file: &MediaFile, speed: f64) {
    let raw_ptr: *mut gtk4::ffi::GtkMediaFile = media_file.to_glib_none().0;
    set_media_file_speed_raw(raw_ptr as *mut c_void, speed);
}
