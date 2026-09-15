//! GStreamer pipeline playback speed and rate controls.

use std::ffi::c_void;

#[link(name = "gstplay-1.0")]
extern "C" {
    fn gst_play_set_rate(play: *mut c_void, rate: f64);
    #[allow(dead_code)]
    fn gst_play_get_rate(play: *mut c_void) -> f64;
}

/// Standard playback rate presets.
pub const SPEED_PRESETS: &[f64] = &[0.5, 0.75, 1.0, 1.25, 1.5, 2.0];

/// Sets playback speed on a GstPlay pipeline pointer.
pub fn set_gst_play_rate(play_ptr: *mut c_void, rate: f64) {
    if !play_ptr.is_null() {
        unsafe {
            gst_play_set_rate(play_ptr, rate);
        }
    }
}

/// Sets playback rate by reading the GstPlay pointer from a raw GtkMediaFile pointer (offset 24).
pub fn set_media_file_speed_raw(raw_ptr: *mut c_void, speed: f64) {
    if raw_ptr.is_null() {
        return;
    }

    unsafe {
        let play_field_ptr = (raw_ptr as *mut u8).add(24) as *mut *mut c_void;
        let play_ptr = *play_field_ptr;
        if !play_ptr.is_null() {
            gst_play_set_rate(play_ptr, speed);
        }
    }
}
