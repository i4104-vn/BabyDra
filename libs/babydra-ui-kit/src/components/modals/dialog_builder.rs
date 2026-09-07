//! Unified Modern Modal Dialog Builder
//! Eliminates boilerplate across all modal dialogs.

use std::boxed::Box as StdBox;

use gtk4::prelude::*;
use gtk4::{
    Align, Box, Button, CheckButton, Entry, GestureClick, Label, Orientation, PasswordEntry,
    ScrolledWindow, TextView, Widget,
};

use crate::ui::icon::get_icon;

/// Icon badge variant for dialog headers
#[derive(Clone, Copy, Debug)]
pub enum BadgeVariant {
    Primary,
    Danger,
    Success,
    Warning,
}

impl BadgeVariant {
    fn css_class(&self) -> &'static str {
        match self {
            BadgeVariant::Primary => "badge-primary",
            BadgeVariant::Danger => "badge-danger",
            BadgeVariant::Success => "badge-success",
            BadgeVariant::Warning => "badge-warning",
        }
    }
}

/// Button variant for action buttons
#[derive(Clone, Copy, Debug)]
pub enum ButtonVariant {
    Primary,
    Cancel,
    Danger,
}

impl ButtonVariant {
    fn css_class(&self) -> &'static str {
        match self {
            ButtonVariant::Primary => "modern-dialog-primary-btn",
            ButtonVariant::Cancel => "modern-dialog-cancel-btn",
            ButtonVariant::Danger => "modern-dialog-danger-btn",
        }
    }
}

/// Configuration for a dialog action button
pub struct ActionButton {
    pub label: String,
    pub variant: ButtonVariant,
    pub callback: StdBox<dyn Fn() + 'static>,
}

/// Builder for modern modal dialogs with consistent styling
pub struct ModernDialogBuilder {
    container: Box,
    card: Box,
    width: i32,
    badge_icon: Option<&'static str>,
    badge_variant: BadgeVariant,
    title: String,
    subtitle: Option<String>,
    header_spacing: i32,
    card_spacing: i32,
    card_margins: (i32, i32),
}

impl ModernDialogBuilder {
    /// Create a new dialog builder with default settings
    pub fn new(width: i32) -> Self {
        let container = Box::new(Orientation::Vertical, 0);
        container.add_css_class("modal-scrim-layer");
        container.set_halign(Align::Fill);
        container.set_valign(Align::Fill);
        container.set_hexpand(true);
        container.set_vexpand(true);
        container.set_visible(false);

        let click_blocker = GestureClick::new();
        click_blocker.connect_pressed(|_, _, _, _| {});
        container.add_controller(click_blocker);

        let card = Box::new(Orientation::Vertical, 16);
        card.add_css_class("modern-modal-card");
        card.set_halign(Align::Center);
        card.set_valign(Align::Center);
        card.set_width_request(width);
        card.set_margin_start(16);
        card.set_margin_end(16);

        Self {
            container,
            card,
            width,
            badge_icon: None,
            badge_variant: BadgeVariant::Primary,
            title: String::new(),
            subtitle: None,
            header_spacing: 14,
            card_spacing: 16,
            card_margins: (16, 16),
        }
    }

    /// Set the icon badge (icon name and variant)
    pub fn with_badge(mut self, icon_name: &'static str, variant: BadgeVariant) -> Self {
        self.badge_icon = Some(icon_name);
        self.badge_variant = variant;
        self
    }

    /// Set the dialog title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set the dialog subtitle
    pub fn with_subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// Set card spacing
    pub fn with_card_spacing(mut self, spacing: i32) -> Self {
        self.card_spacing = spacing;
        self.card.set_spacing(spacing);
        self
    }

    /// Set header spacing
    pub fn with_header_spacing(mut self, spacing: i32) -> Self {
        self.header_spacing = spacing;
        self
    }

    /// Set card margins
    pub fn with_card_margins(mut self, start: i32, end: i32) -> Self {
        self.card_margins = (start, end);
        self.card.set_margin_start(start);
        self.card.set_margin_end(end);
        self
    }

    /// Use terminal-style card instead of standard modal card
    pub fn as_terminal_dialog(mut self, width: i32, height: i32) -> Self {
        self.card.remove_css_class("modern-modal-card");
        self.card.add_css_class("modern-terminal-dialog");
        self.card.set_width_request(width);
        self.card.set_height_request(height);
        self.width = width;
        self
    }

    /// Build the header section (badge + title + subtitle)
    fn build_header(&self) -> (Box, Label, Option<Label>) {
        let header_box = Box::new(Orientation::Horizontal, self.header_spacing);
        header_box.set_halign(Align::Start);
        header_box.set_hexpand(true);
        header_box.set_valign(Align::Center);

        if let Some(icon_name) = self.badge_icon {
            let badge = Box::new(Orientation::Vertical, 0);
            badge.add_css_class("modern-dialog-badge");
            badge.add_css_class(self.badge_variant.css_class());
            badge.set_size_request(42, 42);
            badge.set_halign(Align::Start);
            badge.set_valign(Align::Center);

            let icon = get_icon(icon_name, 22);
            icon.set_pixel_size(22);
            icon.set_vexpand(true);
            icon.set_hexpand(true);
            icon.set_valign(Align::Center);
            icon.set_halign(Align::Center);
            badge.append(&icon);
            header_box.append(&badge);
        }

        let title_box = Box::new(Orientation::Vertical, 3);
        title_box.set_valign(Align::Center);
        title_box.set_hexpand(true);

        let title_lbl = Label::new(Some(&self.title));
        title_lbl.add_css_class("modern-dialog-title");
        title_lbl.set_halign(Align::Start);
        title_lbl.set_wrap(true);
        title_box.append(&title_lbl);

        let subtitle_lbl = if let Some(sub) = &self.subtitle {
            let sub_lbl = Label::new(Some(sub));
            sub_lbl.add_css_class("modern-dialog-subtitle");
            sub_lbl.set_halign(Align::Start);
            sub_lbl.set_wrap(true);
            title_box.append(&sub_lbl);
            Some(sub_lbl)
        } else {
            None
        };

        header_box.append(&title_box);
        (header_box, title_lbl, subtitle_lbl)
    }

    

    /// Get the card widget for adding custom content
    pub fn card(&self) -> &Box {
        &self.card
    }

    /// Get the container widget (scrim layer)
    pub fn container(&self) -> &Box {
        &self.container
    }

    /// Finalize and return the container with header already added
    pub fn build(self) -> ModernDialog {
        // Build and add header, capturing title/subtitle labels
        let (header, title_lbl, subtitle_lbl) = self.build_header();
        self.card.append(&header);

        ModernDialog {
            container: self.container,
            card: self.card,
            title_lbl,
            subtitle_lbl,
        }
    }

    /// Build with custom header widget (for terminals, etc.)
    pub fn build_with_header(self, header: Box) -> ModernDialog {
        self.card.append(&header);
        ModernDialog {
            container: self.container,
            card: self.card,
            title_lbl: Label::new(None),
            subtitle_lbl: None,
        }
    }
}

/// Built dialog with container and card
pub struct ModernDialog {
    container: Box,
    card: Box,
    title_lbl: Label,
    subtitle_lbl: Option<Label>,
}

impl ModernDialog {
    pub fn container(&self) -> &Box {
        &self.container
    }

    pub fn card(&self) -> &Box {
        &self.card
    }

    /// Get the title label for dynamic updates
    pub fn title_label(&self) -> &Label {
        &self.title_lbl
    }

    /// Get the subtitle label for dynamic updates
    pub fn subtitle_label(&self) -> Option<&Label> {
        self.subtitle_lbl.as_ref()
    }

    /// Add a widget to the card
    pub fn add_child(&self, child: &impl IsA<Widget>) {
        self.card.append(child);
    }

    /// Add action buttons at the bottom
    pub fn add_actions(&self, buttons: Vec<ActionButton>) {
        let actions = Box::new(Orientation::Horizontal, 10);
        actions.set_halign(Align::End);
        actions.set_margin_top(6);

        for btn in buttons {
            let button = Button::with_label(&btn.label);
            button.add_css_class(btn.variant.css_class());
            button.set_cursor_from_name(Some("pointer"));
            let callback = btn.callback;
            button.connect_clicked(move |_| callback());
            actions.append(&button);
        }

        self.card.append(&actions);
    }

    /// Show the dialog
    pub fn show(&self) {
        self.container.set_visible(true);
    }

    /// Hide the dialog
    pub fn hide(&self) {
        self.container.set_visible(false);
    }

    /// Check if dialog is visible
    pub fn is_visible(&self) -> bool {
        self.container.is_visible()
    }
}

/// Helper to create a standard form entry with modern styling
pub fn create_modern_entry(placeholder: &str) -> Entry {
    let entry = Entry::new();
    entry.add_css_class("sidebar-search-entry");
    entry.add_css_class("modern-dialog-entry");
    entry.set_placeholder_text(Some(placeholder));
    entry
}

/// Helper to create a password entry with modern styling
pub fn create_modern_password_entry(placeholder: &str) -> PasswordEntry {
    let entry = PasswordEntry::new();
    entry.add_css_class("sidebar-search-entry");
    entry.add_css_class("modern-dialog-entry");
    entry.set_placeholder_text(Some(placeholder));
    entry
}

/// Helper to create a standard label with settings-row-desc class
pub fn create_form_label(text: &str) -> Label {
    let label = Label::new(Some(text));
    label.add_css_class("settings-row-desc");
    label.set_halign(Align::Start);
    label
}

/// Helper to create an error label (hidden by default)
pub fn create_error_label() -> Label {
    let label = Label::new(None);
    label.add_css_class("dialog-error-text");
    label.set_halign(Align::Start);
    label.set_visible(false);
    label
}

/// Helper to create a warning callout banner
pub fn create_warning_banner(icon_name: &str, text: &str) -> Box {
    let banner = Box::new(Orientation::Horizontal, 10);
    banner.add_css_class("modern-dialog-warning");
    banner.set_valign(Align::Center);

    let icon = get_icon(icon_name, 16);
    icon.set_pixel_size(16);
    icon.set_valign(Align::Center);
    banner.append(&icon);

    let label = Label::new(Some(text));
    label.add_css_class("modern-dialog-warning-text");
    label.set_valign(Align::Center);
    label.set_halign(Align::Start);
    label.set_wrap(true);
    label.set_hexpand(true);
    banner.append(&label);

    banner
}

/// Helper to create an info callout banner
pub fn create_info_banner(icon_name: &str, text: &str) -> Box {
    let banner = Box::new(Orientation::Horizontal, 10);
    banner.add_css_class("modern-dialog-info");
    banner.set_valign(Align::Center);

    let icon = get_icon(icon_name, 16);
    icon.set_pixel_size(16);
    icon.set_valign(Align::Center);
    banner.append(&icon);

    let label = Label::new(Some(text));
    label.add_css_class("modern-dialog-info-text");
    label.set_valign(Align::Center);
    label.set_halign(Align::Start);
    label.set_wrap(true);
    label.set_hexpand(true);
    banner.append(&label);

    banner
}

/// Helper to create an acknowledgment checkbox card
pub fn create_ack_card(text: &str) -> (Box, CheckButton) {
    use gtk4::CheckButton;

    let card = Box::new(Orientation::Horizontal, 10);
    card.add_css_class("modern-dialog-ack-card");
    card.set_valign(Align::Center);

    let check = CheckButton::new();
    check.set_valign(Align::Center);
    check.set_cursor_from_name(Some("pointer"));
    check.set_hexpand(true);

    let label = Label::new(Some(text));
    label.set_wrap(true);
    label.set_halign(Align::Start);
    label.set_valign(Align::Center);
    label.set_hexpand(true);
    label.add_css_class("modern-dialog-ack-label");
    check.set_child(Some(&label));
    check.set_active(false);

    card.append(&check);
    (card, check)
}

/// Helper to create a terminal-style console log view
pub fn create_terminal_console() -> (ScrolledWindow, TextView) {
    let text_view = TextView::new();
    text_view.set_editable(false);
    text_view.set_cursor_visible(false);
    text_view.set_monospace(true);
    text_view.add_css_class("console-log-text");

    let scroll = ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Automatic, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);
    scroll.set_child(Some(&text_view));

    (scroll, text_view)
}

/// Helper to create a terminal title bar with traffic lights
pub fn create_terminal_title_bar(title: &str) -> Box {
    let title_bar = Box::new(Orientation::Horizontal, 12);
    title_bar.add_css_class("terminal-title-bar");
    title_bar.set_valign(Align::Center);

    let traffic_dots = Box::new(Orientation::Horizontal, 6);
    traffic_dots.set_valign(Align::Center);

    let dot_close = Box::new(Orientation::Vertical, 0);
    dot_close.add_css_class("traffic-dot-close");
    traffic_dots.append(&dot_close);

    let dot_min = Box::new(Orientation::Vertical, 0);
    dot_min.add_css_class("traffic-dot-minimize");
    traffic_dots.append(&dot_min);

    let dot_max = Box::new(Orientation::Vertical, 0);
    dot_max.add_css_class("traffic-dot-maximize");
    traffic_dots.append(&dot_max);

    title_bar.append(&traffic_dots);

    let title_box = Box::new(Orientation::Horizontal, 8);
    title_box.set_hexpand(true);
    title_box.set_halign(Align::Center);

    let icon = get_icon("terminal", 16);
    icon.set_pixel_size(16);
    title_box.append(&icon);

    let title_lbl = Label::new(Some(title));
    title_lbl.add_css_class("terminal-window-title");
    title_box.append(&title_lbl);

    title_bar.append(&title_box);
    title_bar
}