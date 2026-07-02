//! Struct mapping internal sub-widget references of the Dynamic Island layout.

/// References to sub-widgets within the Dynamic Island layout structure.
#[derive(Clone)]
pub struct IslandWidgets {
    /// Outer notch capsule layout container.
    pub notch_capsule: gtk4::Box,
    /// Default compact notch capsule view.
    pub default_view: gtk4::Box,
    /// Music player expanded notch capsule view.
    pub music_view: gtk4::Box,
    /// Label containing track information inside notch.
    pub track_label: gtk4::Label,
    /// Album art image container inside notch.
    pub art_container: gtk4::Box,
    /// Audio visualizer bar container inside notch.
    pub visualizer_box: gtk4::Box,
    /// Dropdown notification badge container.
    pub notification_badge: gtk4::Box,
    /// Title text label of the dropdown notification badge.
    pub badge_title: gtk4::Label,
    /// Body text description of the dropdown notification badge.
    pub badge_desc: gtk4::Label,
    /// Icon element container of the dropdown notification badge.
    pub badge_icon_container: gtk4::Box,
    /// Popover header text showing the track title.
    pub popover_title: gtk4::Label,
    /// Popover subtitle text showing the artist name.
    pub popover_artist: gtk4::Label,
    /// Album art image box container in the control popover.
    pub popover_art_container: gtk4::Box,
    /// Application name label in the control popover.
    pub popover_app_name: gtk4::Label,
    /// Icon element inside play/pause toggle button.
    pub play_btn_icon: gtk4::Image,
    /// Dynamic Island notification overlay view.
    pub notification_view: gtk4::Box,
    /// App icon container in the notification overlay.
    pub notif_art_container: gtk4::Box,
    /// Title text label in the notification overlay.
    pub notif_title_lbl: gtk4::Label,
    /// Message body label in the notification overlay.
    pub notif_body_lbl: gtk4::Label,
}

