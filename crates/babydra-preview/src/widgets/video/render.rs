//! Video preview UI layout assembly and widget creation.

use babydra_core::i18n::trans;
use babydra_core::models::preview::VideoMetadata;
use babydra_core::services::preview::SPEED_PRESETS;
use crate::widgets::window::{create_viewer_window, format_aspect_ratio};
use std::path::PathBuf;
use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box, Button, ContentFit, Grid, Label,
    MediaFile, Orientation, Overlay, Picture, Popover, Scale,
};

/// Complete UI widgets structure for video playback preview.
pub struct VideoViewerUi {
    pub window: ApplicationWindow,
    pub picture: Picture,
    pub media_file: MediaFile,
    pub info_box: Box,
    pub controls_box: Box,
    pub play_pause_btn: Button,
    pub time_lbl: Label,
    pub timeline_scale: Scale,
    pub mute_btn: Button,
    pub volume_scale: Scale,
    pub speed_btn: Button,
    pub speed_popover: Popover,
    pub details_box: Box,
}

/// Formats seconds into a clean `MM:SS` or `HH:MM:SS` string.
pub fn format_duration(seconds: f64) -> String {
    let total_secs = seconds.round().max(0.0) as u64;
    let h = total_secs / 3600;
    let m = (total_secs % 3600) / 60;
    let s = total_secs % 60;

    if h > 0 {
        format!("{:02}:{:02}:{:02}", h, m, s)
    } else {
        format!("{:02}:{:02}", m, s)
    }
}

/// Builds the full viewer window UI for video files.
pub fn build_video_ui(
    app: &Application,
    path: &PathBuf,
    meta: &VideoMetadata,
) -> VideoViewerUi {
    let title = trans("preview.video_title").replace(
        "{}",
        &path.file_name().unwrap_or_default().to_string_lossy(),
    );
    let (window, _) = create_viewer_window(app, &title, meta.width, meta.height);

    let overlay = Overlay::new();

    let media_file = MediaFile::for_filename(path);

    let picture = Picture::new();
    picture.set_paintable(Some(&media_file));
    picture.set_content_fit(ContentFit::Contain);
    picture.set_hexpand(true);
    picture.set_vexpand(true);
    picture.add_css_class("viewer-drawing-area");
    overlay.set_child(Some(&picture));

    // --- Top-Left Info Box Overlay ---
    let info_box =
        babydra_ui_kit::components::create_css_card(Orientation::Vertical, 2, "info-card");
    info_box.set_halign(Align::Start);
    info_box.set_valign(Align::Start);
    info_box.set_margin_start(16);
    info_box.set_margin_top(16);

    let name_lbl = Label::new(Some(
        &path.file_name().unwrap_or_default().to_string_lossy(),
    ));
    name_lbl.add_css_class("info-item");
    name_lbl.set_halign(Align::Start);
    name_lbl.set_ellipsize(gtk4::pango::EllipsizeMode::Middle);
    name_lbl.set_max_width_chars(36);
    info_box.append(&name_lbl);

    let res_aspect = format_aspect_ratio(meta.width, meta.height);
    let res_text = if !res_aspect.is_empty() {
        format!("{}x{} ({})", meta.width, meta.height, res_aspect)
    } else {
        format!("{}x{}", meta.width, meta.height)
    };
    let meta_text = format!(
        "{} • {} • {}",
        res_text,
        format_duration(meta.duration_secs),
        babydra_ui_kit::components::explore::format_size(meta.file_size)
    );
    let meta_lbl = Label::new(Some(&meta_text));
    meta_lbl.add_css_class("info-item");
    meta_lbl.set_halign(Align::Start);
    info_box.append(&meta_lbl);

    overlay.add_overlay(&info_box);

    // --- Bottom-Center Video Playback Controls Pill ---
    let controls_box = Box::new(Orientation::Horizontal, 8);
    controls_box.add_css_class("controls-bar");
    controls_box.add_css_class("video-controls-bar");
    controls_box.set_halign(Align::Center);
    controls_box.set_valign(Align::End);
    controls_box.set_margin_bottom(16);

    // Play / Pause Button
    let play_pause_btn = babydra_ui_kit::components::create_icon_button(
        "media-playback-start-symbolic",
        16,
        &["control-btn", "video-play-btn"],
        None,
        || {},
    );
    play_pause_btn.set_cursor_from_name(Some("pointer"));
    controls_box.append(&play_pause_btn);

    // Time Label: 00:00 / 00:00
    let total_dur_str = format_duration(meta.duration_secs);
    let time_lbl = Label::new(Some(&format!("00:00 / {}", total_dur_str)));
    time_lbl.add_css_class("info-item");
    time_lbl.add_css_class("video-time-lbl");
    time_lbl.set_valign(Align::Center);
    time_lbl.set_margin_start(4);
    time_lbl.set_margin_end(4);
    controls_box.append(&time_lbl);

    // Timeline Scrubber Scale
    let max_dur = meta.duration_secs.max(1.0);
    let timeline_scale = Scale::with_range(Orientation::Horizontal, 0.0, max_dur, 0.1);
    timeline_scale.set_draw_value(false);
    timeline_scale.set_width_request(240);
    timeline_scale.add_css_class("video-timeline-scale");
    timeline_scale.set_valign(Align::Center);
    timeline_scale.set_cursor_from_name(Some("pointer"));
    controls_box.append(&timeline_scale);

    // Volume Button (Mute / Unmute Toggle)
    let mute_btn = babydra_ui_kit::components::create_icon_button(
        "audio-volume-high-symbolic",
        15,
        &["control-btn", "video-mute-btn"],
        None,
        || {},
    );
    mute_btn.set_cursor_from_name(Some("pointer"));
    controls_box.append(&mute_btn);

    // Compact Volume Scale
    let volume_scale = Scale::with_range(Orientation::Horizontal, 0.0, 1.0, 0.05);
    volume_scale.set_draw_value(false);
    volume_scale.set_value(1.0);
    volume_scale.set_width_request(70);
    volume_scale.add_css_class("video-volume-scale");
    volume_scale.set_valign(Align::Center);
    volume_scale.set_cursor_from_name(Some("pointer"));
    controls_box.append(&volume_scale);

    // Speed Selection Pill Button & Popover
    let speed_btn = Button::with_label("1.0x");
    speed_btn.add_css_class("control-btn");
    speed_btn.add_css_class("video-speed-btn");
    speed_btn.set_cursor_from_name(Some("pointer"));

    let speed_popover = Popover::new();
    let speed_list_box = Box::new(Orientation::Vertical, 4);
    speed_list_box.set_margin_top(6);
    speed_list_box.set_margin_bottom(6);
    speed_list_box.set_margin_start(6);
    speed_list_box.set_margin_end(6);

    for &spd in SPEED_PRESETS {
        let btn = Button::with_label(&format!("{:.2}x", spd).replace(".00", ".0").replace(".50", ".5").replace(".75", ".75").replace(".25", ".25"));
        btn.add_css_class("popover-btn");
        speed_list_box.append(&btn);
    }
    speed_popover.set_child(Some(&speed_list_box));
    speed_popover.set_parent(&speed_btn);

    controls_box.append(&speed_btn);

    overlay.add_overlay(&controls_box);

    // --- Centered Video Metadata Dialog (holding 'i') ---
    let details_box = Box::new(Orientation::Vertical, 12);
    details_box.add_css_class("exif-dialog");
    details_box.add_css_class("video-details-dialog");
    details_box.set_halign(Align::Center);
    details_box.set_valign(Align::Center);
    details_box.set_visible(false);

    let details_title = Label::new(Some(&trans("preview.video_info")));
    details_title.add_css_class("exif-title");
    details_box.append(&details_title);

    let grid = Grid::new();
    grid.set_column_spacing(24);
    grid.set_row_spacing(8);

    let mut row_idx = 0;
    let mut add_spec_row = |label: &str, value: &str| {
        let lbl = Label::new(Some(label));
        lbl.add_css_class("exif-label");
        lbl.set_halign(Align::Start);
        grid.attach(&lbl, 0, row_idx, 1, 1);

        let val = Label::new(Some(value));
        val.add_css_class("exif-value");
        val.set_halign(Align::End);
        grid.attach(&val, 1, row_idx, 1, 1);

        row_idx += 1;
    };

    add_spec_row(&trans("preview.resolution"), &res_text);
    add_spec_row(&trans("preview.duration"), &format_duration(meta.duration_secs));
    add_spec_row(&trans("preview.file_size"), &babydra_ui_kit::components::explore::format_size(meta.file_size));
    add_spec_row("Container", &meta.format_long_name);

    if let Some(ref v) = meta.video_stream {
        let codec_disp = v.codec_long_name.as_deref().unwrap_or(&v.codec_name);
        add_spec_row(&trans("preview.codec"), codec_disp);
        if let Some(fps) = v.fps {
            add_spec_row(&trans("preview.frame_rate"), &format!("{:.2} fps", fps));
        }
        if let Some(br) = v.bit_rate.or(meta.bit_rate) {
            let mbps = br as f64 / 1_000_000.0;
            add_spec_row(&trans("preview.bitrate"), &format!("{:.2} Mbps", mbps));
        }
        if let Some(ref pix) = v.pix_fmt {
            add_spec_row("Pixel Format", pix);
        }
    }

    if let Some(ref a) = meta.audio_stream {
        let audio_disp = match (&a.channels, &a.sample_rate) {
            (Some(ch), Some(sr)) => format!("{} ({} ch, {} Hz)", a.codec_name.to_uppercase(), ch, sr),
            _ => a.codec_name.to_uppercase(),
        };
        add_spec_row(&trans("preview.audio"), &audio_disp);
    }

    details_box.append(&grid);
    overlay.add_overlay(&details_box);

    window.set_child(Some(&overlay));

    VideoViewerUi {
        window,
        picture,
        media_file,
        info_box,
        controls_box,
        play_pause_btn,
        time_lbl,
        timeline_scale,
        mute_btn,
        volume_scale,
        speed_btn,
        speed_popover,
        details_box,
    }
}
