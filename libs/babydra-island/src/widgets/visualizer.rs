//! Animated wave bars audio visualizer indicator for the media player.

use gtk4::prelude::*;
use std::cell::Cell;
use std::rc::Rc;

/// Creates a horizontal box container containing 4 animated visualizer bars.
pub fn create_visualizer() -> (gtk4::Box, Vec<gtk4::Box>) {
    let visualizer_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 2);
    visualizer_box.add_css_class("notch-visualizer");
    visualizer_box.set_valign(gtk4::Align::Center);

    let mut bars = Vec::new();
    for _ in 0..4 {
        let bar = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        bar.add_css_class("visualizer-bar");
        bar.set_size_request(2, 2);
        bar.set_valign(gtk4::Align::End);
        visualizer_box.append(&bar);
        bars.push(bar);
    }
    (visualizer_box, bars)
}

/// Registers tick callbacks on visualizer bars to dynamically animate height changes using mixed sine/cosine waves when music is playing.
pub fn start_visualizer_animation(bars: Vec<gtk4::Box>, is_playing: Rc<Cell<bool>>) {
    if bars.is_empty() {
        return;
    }
    let start_time = std::cell::Cell::new(0i64);
    let first_bar = bars[0].clone();
    first_bar.add_tick_callback(move |_w, clock| {
        if is_playing.get() {
            let now = clock.frame_time();
            if start_time.get() == 0 {
                start_time.set(now);
            }
            let elapsed_sec = (now - start_time.get()) as f64 / 1_000_000.0;
            for (i, bar) in bars.iter().enumerate() {
                let freq1 = 6.0 + (i as f64 * 2.5);
                let freq2 = 4.0 - (i as f64 * 1.2);
                let val1 = (elapsed_sec * freq1).sin();
                let val2 = (elapsed_sec * freq2).cos();
                let mixed = (val1 + val2) / 2.0;
                let val = (mixed * 4.0 + 6.0) as i32;
                bar.set_size_request(2, val.max(2).min(10));
            }
        } else {
            start_time.set(0);
            for bar in &bars {
                bar.set_size_request(2, 2);
            }
        }
        glib::ControlFlow::Continue
    });
}

