//! Video preview UI layout assembly and widget creation.

use crate::widgets::window::format_aspect_ratio;
use babydra_core::i18n::trans;
use babydra_core::models::preview::VideoMetadata;
use babydra_core::services::preview::SPEED_PRESETS;
use gtk4::prelude::*;
use gtk4::{
    Align, ApplicationWindow, Box, Button, ContentFit, Grid, Label, MediaFile,
    Orientation, Overlay, Picture, Popover, Revealer, RevealerTransitionType, Scale, Separator,
    Spinner,
};
use std::path::PathBuf;

/// Complete UI widgets structure for video playback preview.
pub struct VideoViewerUi {
    pub window: ApplicationWindow,
    pub picture: Picture,
    pub media_file: MediaFile,
    pub info_box: Box,
    pub info_revealer: Revealer,
    pub meta_lbl: Label,
    pub controls_box: Box,
    pub controls_revealer: Revealer,
    pub play_pause_btn: Button,
    pub time_lbl: Label,
    pub total_time_lbl: Label,
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

/// Creates an animated centered loading placeholder.
pub fn create_loading_view() -> Box {
    let loading_box = Box::new(Orientation::Vertical, 12);
    loading_box.set_halign(Align::Center);
    loading_box.set_valign(Align::Center);
    loading_box.set_hexpand(true);
    loading_box.set_vexpand(true);

    let spinner = Spinner::new();
    spinner.set_spinning(true);
    spinner.set_size_request(36, 36);
    loading_box.append(&spinner);

    let label = Label::new(Some(&trans("common.pending")));
    label.add_css_class("dim-label");
    loading_box.append(&label);

    loading_box
}

/// Fills the video details box with an active loading indicator.
pub fn show_video_details_loading(details_box: &Box) {
    while let Some(child) = details_box.first_child() {
        details_box.remove(&child);
    }

    let details_title = Label::new(Some(&trans("preview.video_info")));
    details_title.add_css_class("exif-title");
    details_title.set_hexpand(false);
    details_box.append(&details_title);

    let spinner = Spinner::new();
    spinner.set_spinning(true);
    spinner.set_size_request(24, 24);
    spinner.set_margin_top(12);
    spinner.set_margin_bottom(12);
    details_box.append(&spinner);

    let loading_lbl = Label::new(Some(&trans("common.pending")));
    loading_lbl.add_css_class("dim-label");
    details_box.append(&loading_lbl);
}

/// Populates the video details box with parsed stream specs from ffprobe.
pub fn populate_video_details(details_box: &Box, meta: &VideoMetadata) {
    while let Some(child) = details_box.first_child() {
        details_box.remove(&child);
    }

    let details_title = Label::new(Some(&trans("preview.video_info")));
    details_title.add_css_class("exif-title");
    details_title.set_hexpand(false);
    details_box.append(&details_title);

    let grid = Grid::new();
    grid.set_hexpand(false);
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

    let res_aspect = format_aspect_ratio(meta.width, meta.height);
    let res_text = if !res_aspect.is_empty() {
        format!("{}x{} ({})", meta.width, meta.height, res_aspect)
    } else {
        format!("{}x{}", meta.width, meta.height)
    };

    add_spec_row(&trans("preview.resolution"), &res_text);
    add_spec_row(
        &trans("preview.duration"),
        &format_duration(meta.duration_secs),
    );
    add_spec_row(
        &trans("preview.file_size"),
        &babydra_ui_kit::components::explore::format_size(meta.file_size),
    );
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
            (Some(ch), Some(sr)) => {
                format!("{} ({} ch, {} Hz)", a.codec_name.to_uppercase(), ch, sr)
            }
            _ => a.codec_name.to_uppercase(),
        };
        add_spec_row(&trans("preview.audio"), &audio_disp);
    }

    details_box.append(&grid);
}


/// Builds the video viewer content onto an existing window.
pub fn build_video_content(window: &ApplicationWindow, path: &PathBuf) -> VideoViewerUi {
    let overlay = Overlay::new();

    let media_file = MediaFile::for_filename(path);

    let picture = Picture::new();
    picture.set_paintable(Some(&media_file));
    picture.set_content_fit(ContentFit::Contain);
    picture.set_hexpand(true);
    picture.set_vexpand(true);
    picture.add_css_class("viewer-drawing-area");
    overlay.set_child(Some(&picture));

    // --- Top-Right Info Box Overlay ---
    let info_box =
        babydra_ui_kit::components::create_css_card(Orientation::Vertical, 2, "info-card");

    let name_lbl = Label::new(Some(
        &path.file_name().unwrap_or_default().to_string_lossy(),
    ));
    name_lbl.add_css_class("info-item");
    name_lbl.set_halign(Align::End);
    name_lbl.set_ellipsize(gtk4::pango::EllipsizeMode::Middle);
    name_lbl.set_max_width_chars(36);
    info_box.append(&name_lbl);

    let size_bytes = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);
    let size_str = babydra_ui_kit::components::explore::format_size(size_bytes);
    let meta_lbl = Label::new(Some(&size_str));
    meta_lbl.add_css_class("info-item");
    meta_lbl.set_halign(Align::End);
    info_box.append(&meta_lbl);

    let info_revealer = Revealer::new();
    info_revealer.set_transition_type(RevealerTransitionType::SlideDown);
    info_revealer.set_transition_duration(300);
    info_revealer.set_halign(Align::End);
    info_revealer.set_valign(Align::Start);
    info_revealer.set_margin_end(16);
    info_revealer.set_margin_top(16);
    info_revealer.set_child(Some(&info_box));
    info_revealer.set_reveal_child(true);
    overlay.add_overlay(&info_revealer);

    // --- Bottom-Center Video Playback Controls Pill ---
    let controls_box = Box::new(Orientation::Horizontal, 6);
    controls_box.add_css_class("controls-bar");
    controls_box.add_css_class("video-controls-bar");

    // 1. Play / Pause Button
    let play_pause_btn = babydra_ui_kit::components::create_icon_button(
        "play",
        16,
        &["control-btn", "video-play-btn"],
        Some(&trans("preview.play")),
        || {},
    );
    play_pause_btn.set_cursor_from_name(Some("pointer"));
    controls_box.append(&play_pause_btn);

    // 2. Current Time Label (00:00)
    let time_lbl = Label::new(Some("00:00"));
    time_lbl.add_css_class("info-item");
    time_lbl.add_css_class("video-time-lbl");
    time_lbl.set_valign(Align::Center);
    time_lbl.set_margin_start(4);
    time_lbl.set_margin_end(2);
    controls_box.append(&time_lbl);

    // 3. Timeline Scrubber Scale
    let timeline_scale = Scale::with_range(Orientation::Horizontal, 0.0, 1.0, 0.1);
    timeline_scale.set_draw_value(false);
    timeline_scale.set_width_request(200);
    timeline_scale.add_css_class("video-timeline-scale");
    timeline_scale.set_valign(Align::Center);
    timeline_scale.set_cursor_from_name(Some("pointer"));
    controls_box.append(&timeline_scale);

    // 4. Total Duration Label
    let total_time_lbl = Label::new(Some("00:00"));
    total_time_lbl.add_css_class("info-item");
    total_time_lbl.add_css_class("video-time-lbl");
    total_time_lbl.set_valign(Align::Center);
    total_time_lbl.set_margin_start(2);
    total_time_lbl.set_margin_end(4);
    controls_box.append(&total_time_lbl);

    // Separator between timeline group and volume group
    let sep1 = Separator::new(Orientation::Vertical);
    sep1.add_css_class("control-separator");
    sep1.set_valign(Align::Center);
    sep1.set_size_request(1, 16);
    controls_box.append(&sep1);

    // 5. Volume Button
    let mute_btn = babydra_ui_kit::components::create_icon_button(
        "volume",
        16,
        &["control-btn", "video-mute-btn"],
        Some(&trans("preview.mute")),
        || {},
    );
    mute_btn.set_cursor_from_name(Some("pointer"));
    controls_box.append(&mute_btn);

    // 6. Compact Volume Scale
    let volume_scale = Scale::with_range(Orientation::Horizontal, 0.0, 1.0, 0.05);
    volume_scale.set_draw_value(false);
    volume_scale.set_value(1.0);
    volume_scale.set_width_request(58);
    volume_scale.add_css_class("video-volume-scale");
    volume_scale.set_valign(Align::Center);
    volume_scale.set_cursor_from_name(Some("pointer"));
    controls_box.append(&volume_scale);

    // Separator between volume group and speed group
    let sep2 = Separator::new(Orientation::Vertical);
    sep2.add_css_class("control-separator");
    sep2.set_valign(Align::Center);
    sep2.set_size_request(1, 16);
    controls_box.append(&sep2);

    // 7. Speed Selection Pill Button & Popover
    let speed_btn = Button::with_label("1.0x");
    speed_btn.add_css_class("control-btn");
    speed_btn.add_css_class("video-speed-btn");
    speed_btn.set_tooltip_text(Some(&trans("preview.speed")));
    speed_btn.set_cursor_from_name(Some("pointer"));

    let speed_popover = Popover::new();
    speed_popover.add_css_class("video-speed-popover");
    let speed_list_box = Box::new(Orientation::Vertical, 2);
    speed_list_box.set_margin_top(4);
    speed_list_box.set_margin_bottom(4);
    speed_list_box.set_margin_start(4);
    speed_list_box.set_margin_end(4);

    for &spd in SPEED_PRESETS {
        let label_text = format!("{:.2}x", spd)
            .replace(".00", ".0")
            .replace(".50", ".5")
            .replace(".75", ".75")
            .replace(".25", ".25");
        let btn = Button::with_label(&label_text);
        btn.add_css_class("video-speed-item");
        if (spd - 1.0).abs() < f64::EPSILON {
            btn.add_css_class("active");
        }
        speed_list_box.append(&btn);
    }
    speed_popover.set_child(Some(&speed_list_box));
    speed_popover.set_parent(&speed_btn);

    controls_box.append(&speed_btn);

    let controls_revealer = Revealer::new();
    controls_revealer.set_transition_type(RevealerTransitionType::SlideUp);
    controls_revealer.set_transition_duration(300);
    controls_revealer.set_halign(Align::Center);
    controls_revealer.set_valign(Align::End);
    controls_revealer.set_margin_bottom(16);
    controls_revealer.set_child(Some(&controls_box));
    controls_revealer.set_reveal_child(true);
    overlay.add_overlay(&controls_revealer);

    // --- Centered Video Metadata Dialog (holding 'i') ---
    let details_box = Box::new(Orientation::Vertical, 12);
    details_box.add_css_class("exif-dialog");
    details_box.add_css_class("video-details-dialog");
    details_box.set_halign(Align::Center);
    details_box.set_valign(Align::Center);
    details_box.set_visible(false);

    show_video_details_loading(&details_box);
    overlay.add_overlay(&details_box);

    window.set_child(Some(&overlay));

    VideoViewerUi {
        window: window.clone(),
        picture,
        media_file,
        info_box,
        info_revealer,
        meta_lbl,
        controls_box,
        controls_revealer,
        play_pause_btn,
        time_lbl,
        total_time_lbl,
        timeline_scale,
        mute_btn,
        volume_scale,
        speed_btn,
        speed_popover,
        details_box,
    }
}
